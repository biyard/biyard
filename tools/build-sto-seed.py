#!/usr/bin/env python3
"""
STO 시드 데이터 생성기 — 캐시 파일들을 읽어 DynamoDB BatchWriteItem 포맷으로 변환.

입력:
  - /tmp/dart-cache/raw/parsed/_cycles.json          (DART 사이클 32건)
  - /tmp/dart-cache/raw/parsed/_filings_parsed.json  (DART 원본 공시)
  - sto-mockup/musicow-catalog.json                  (Musicow 472곡)
  - sto-mockup/musicow-pdf-details.json              (Musicow PDF 디테일 5건)
  - sto-mockup/dart-viewer-data.json                 (filings_raw 197건)

출력:
  - scripts/sto-seed-data.json
    형식: { "biyard-local-sto": [ {"PutRequest": {"Item": {...}}}, ... ] }

ID 부여:
  - sto_id, filing_id 는 (origin, external_id) 기반 deterministic UUID v7-style
  - 같은 입력이면 같은 ID 가 나옴 (재실행 멱등)
"""
import json
import hashlib
import time
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
CACHE = Path("/tmp/dart-cache/raw/parsed")
MOCK = REPO / "sto-mockup"
OUT = REPO / "scripts" / "sto-seed-data.json"
TABLE = "biyard-local-sto"

NOW_MS = int(time.time() * 1000)


def date_to_ms(date_str: str) -> int:
    """`YYYY-MM-DD` 또는 `YYYYMMDD` → UTC 자정 epoch ms. 비어있으면 0."""
    if not date_str:
        return 0
    s = date_str.strip()
    if len(s) == 8 and s.isdigit():  # YYYYMMDD
        s = f"{s[:4]}-{s[4:6]}-{s[6:]}"
    try:
        import datetime
        dt = datetime.datetime.strptime(s, "%Y-%m-%d").replace(tzinfo=datetime.timezone.utc)
        return int(dt.timestamp() * 1000)
    except Exception:
        return 0


def deterministic_id(*parts) -> str:
    """입력 튜플 기반 결정적 UUID 생성 (재실행 시 같은 ID)."""
    h = hashlib.sha256("::".join(str(p) for p in parts).encode()).hexdigest()
    # UUID v4 style (실제 v7 아님, 결정적이라는 게 핵심)
    return f"{h[0:8]}-{h[8:12]}-7{h[13:16]}-{h[16:20]}-{h[20:32]}"


# ---------- DynamoDB attribute value helpers ----------
def S(v):
    return {"S": str(v)}


def N(v):
    return {"N": str(v)}


def BOOL(v):
    return {"BOOL": bool(v)}


def NULL():
    return {"NULL": True}


def L(items):
    return {"L": items}


def M(d):
    return {"M": d}


def attr(v):
    """Python 값 → DynamoDB AttributeValue 자동 변환."""
    if v is None:
        return None  # omit
    if isinstance(v, bool):
        return BOOL(v)
    if isinstance(v, (int, float)):
        return N(v)
    if isinstance(v, str):
        return v != "" and S(v) or None  # 빈 문자열도 omit
    if isinstance(v, list):
        items = [attr(x) for x in v if attr(x) is not None]
        return L(items)
    if isinstance(v, dict):
        d = {k: attr(val) for k, val in v.items()}
        d = {k: val for k, val in d.items() if val is not None}
        return M(d) if d else None
    return S(str(v))


def put_item(item):
    """omit None 처리 후 PutRequest 로 감싸기."""
    cleaned = {}
    for k, v in item.items():
        if v is None:
            continue
        av = attr(v)
        if av is None:
            continue
        cleaned[k] = av
    return {"PutRequest": {"Item": cleaned}}


# ---------- 매핑 테이블 ----------
ISSUER_SLUG = {
    "스탁키퍼": "stockeeper",
    "데이터젠": "datagen",
    "투게더아트": "togetherart",
    "열매컴퍼니": "yeolmae",
    "서울옥션블루": "seoulauctionblue",
    "아티피오": "artipio",
    "카사": "kasa",
    "루센트블록": "lucentblock",
    "펀블": "funble",
    "뮤직카우": "musicow",
}

ISSUER_META = {
    "stockeeper": {"name": "스탁키퍼 (뱅카우)", "category": "livestock", "region": "KR", "country": "🇰🇷 한국", "status": "운영 중", "description": "한우를 기초자산으로 한 가축투자계약증권을 발행합니다. 신한투자증권을 통해 계좌를 개설하면 청약할 수 있습니다."},
    "datagen": {"name": "데이터젠 (핀돈)", "category": "livestock", "region": "KR", "country": "🇰🇷 한국", "status": "운영 중", "description": "한돈을 기초자산으로 하는 투자계약증권 발행사입니다. 계좌관리는 하나증권이 담당합니다."},
    "togetherart": {"name": "투게더아트", "category": "art", "region": "KR", "country": "🇰🇷 한국", "status": "운영 중", "description": "유명 작가의 미술품을 투자계약증권 형태로 공모합니다."},
    "yeolmae": {"name": "열매컴퍼니 (아트앤가이드)", "category": "art", "region": "KR", "country": "🇰🇷 한국", "status": "운영 중 · IPO 준비", "description": "미술품 공동구매와 투자계약증권 발행을 함께 운영합니다. 누적 184건 공모, 그중 140건은 이미 매각이 완료됐습니다."},
    "seoulauctionblue": {"name": "서울옥션블루 (SOTWO)", "category": "art", "region": "KR", "country": "🇰🇷 한국", "status": "신규 발행 일시 중단", "description": "서울옥션 관계사로, 미술품 조각투자 플랫폼 ‘SOTWO’를 운영합니다. 신규 공모는 잠시 중단된 상태입니다."},
    "artipio": {"name": "아티피오 (Artipio)", "category": "art", "region": "KR", "country": "🇰🇷 한국", "status": "공모 준비 중", "description": "호크니, 록카쿠 등 컨템포러리 작가의 작품을 기초자산으로 미술품 투자계약증권을 준비하고 있습니다."},
    "kasa": {"name": "카사 (Kasa Korea)", "category": "real_estate", "region": "KR", "country": "🇰🇷 한국", "status": "운영 중 · 대신증권 협업", "description": "상업용 부동산을 디지털 수익증권(DABS)으로 발행합니다. 대신프라퍼티가 지분 90%를 보유 중입니다."},
    "lucentblock": {"name": "루센트블록 (소유)", "category": "real_estate", "region": "KR", "country": "🇰🇷 한국", "status": "거래소 인가 탈락 · 사업 재편 검토", "description": "지방·중소형 상업용 부동산을 신탁수익증권으로 공모해 왔으나, 거래소 인가에서 탈락해 약 250억 원 규모 자산 정리가 거론되고 있습니다."},
    "funble": {"name": "펀블 (Funble)", "category": "real_estate", "region": "KR", "country": "🇰🇷 한국", "status": "사업 종료 · 연내 청산", "description": "랜드마크 상업용 부동산을 신탁수익증권으로 운영해 왔으나, 수익증권 투자중개업 인가를 취득하지 못해 연내 청산 절차에 들어갔습니다."},
    "musicow": {"name": "뮤직카우 (Musicow)", "category": "music", "region": "KR", "country": "🇰🇷 한국", "status": "운영 중 · NXT 컨소시엄 참여", "description": "음악 저작권을 기초자산으로 한 신탁수익증권을 운영합니다. 누적 거래액 4,200억 원 이상이며, 보관기관은 키움증권㈜입니다."},
}

STATUS_NORMALIZE = {
    "발행완료": "발행완료",
    "발행 완료": "발행완료",
    "신고중": "신고중",
    "증권신고서 제출": "신고중",
    "철회": "철회",
}


# ---------- 빌더 ----------
items = []   # 최종 PutRequest 리스트
sto_id_map = {}  # (origin, external_id) → sto_id 매핑 (재사용)


def make_sto_id(origin, external_id):
    key = (origin, external_id)
    if key not in sto_id_map:
        sto_id_map[key] = deterministic_id("STO", origin, external_id)
    return sto_id_map[key]


def add_issuer(slug):
    if slug not in ISSUER_META:
        return
    meta = ISSUER_META[slug]
    item = {
        "pk": f"ISSUER#{slug}",
        "sk": "ISSUER",
        "issuer_id": slug,
        "name": meta["name"],
        "region": meta["region"],
        "country": meta["country"],
        "category": meta["category"],
        "description": meta["description"],
        "status": meta["status"],
        "created_at": NOW_MS,
        "updated_at": NOW_MS,
    }
    items.append(put_item(item))


CATEGORY_META_KIND = {
    "music": "Music",
    "art": "Art",
    "real_estate": "RealEstate",
    "livestock": "Livestock",
}


def build_category_meta(sto):
    """카테고리별 부가 메타를 `{kind, ...}` 형태로 반환. 부가 메타가 전혀 없으면 None."""
    cat = sto["category"]
    if cat == "music":
        meta = {
            "kind": "Music",
            "artist": sto.get("artist"),
            "rights_category": sto.get("rights_category"),
            "trust_no": sto.get("trust_no"),
            "year": sto.get("year"),
        }
    elif cat == "art":
        meta = {
            "kind": "Art",
            "artwork_year": sto.get("year"),
            "medium": sto.get("medium"),
            "dimensions": sto.get("dimensions"),
        }
    elif cat == "real_estate":
        meta = {
            "kind": "RealEstate",
            "address": sto.get("address"),
            "building_type": sto.get("building_type"),
            "floor_area": sto.get("floor_area"),
        }
    elif cat == "livestock":
        meta = {
            "kind": "Livestock",
            "farm_name": sto.get("farm_name"),
            "breed": sto.get("breed"),
            "head_count": sto.get("head_count"),
        }
    else:
        return None
    # 모든 부가 필드가 비어있으면 row 만들지 않음
    if all(meta.get(k) in (None, "") for k in meta if k != "kind"):
        return None
    return meta


def add_sto(sto):
    """sto: dict — required: name, category, region, status, issued_at, origin, external_id.
    `issued_at` 은 `YYYY-MM-DD` (또는 빈 문자열) 로 받고, 내부에서 epoch ms 로 변환.
    """
    sto_id = sto["sto_id"]
    issued_at_ms = date_to_ms(sto.get("issued_at", ""))
    item = {
        "pk": f"STO#{sto_id}",
        "sk": "STO",
        "sto_id": sto_id,
        "name": sto["name"],
        "underlying": sto.get("underlying"),
        "category": sto["category"],
        "region": sto["region"],
        "country": sto.get("country", "🇰🇷 한국" if sto["region"] == "KR" else "🌍 해외"),
        "issuer_id": sto.get("issuer_id"),
        "security_type": sto.get("security_type"),
        "classification": sto.get("classification"),
        "status": sto["status"],
        "issued_at": issued_at_ms,
        "origin": sto["origin"],
        "external_id": sto.get("external_id"),
        "external_url": sto.get("external_url"),
        "offering": sto.get("offering"),
        "issuance_structure": sto.get("issuance_structure"),
        "sources": sto.get("sources", []),
        "created_at": NOW_MS,
        "updated_at": NOW_MS,
        # GSI fields
        "gsi1_pk": f"STATUS#{sto['status']}",
        "gsi1_sk": f"TS#{issued_at_ms}#{sto_id}",
        "gsi2_pk": f"CAT#{sto['region']}#{sto['category']}",
        "gsi2_sk": f"TS#{issued_at_ms}#{sto_id}",
    }
    if sto.get("issuer_id"):
        item["gsi3_pk"] = f"ISSUER#{sto['issuer_id']}"
        item["gsi3_sk"] = f"TS#{issued_at_ms}#{sto_id}"
    items.append(put_item(item))

    meta = build_category_meta(sto)
    if meta:
        kind = meta["kind"]
        meta_item = {
            "pk": f"STO#{sto_id}",
            "sk": f"STO_META#{CATEGORY_META_KIND_TO_SK.get(sto['category'], kind.upper())}",
            "meta": meta,
            "created_at": NOW_MS,
            "updated_at": NOW_MS,
        }
        items.append(put_item(meta_item))


# SK suffix 는 category 식별자(영문) 로 통일
CATEGORY_META_KIND_TO_SK = {
    "music": "MUSIC",
    "art": "ART",
    "real_estate": "REAL_ESTATE",
    "livestock": "LIVESTOCK",
}


def add_filing(sto_id, filing):
    """filing: dict — required: filing_id, filing_source, title, filed_at (YYYY-MM-DD)"""
    item = {
        "pk": f"STO#{sto_id}",
        "sk": f"FILING#{filing['filing_id']}",
        "filing_id": filing["filing_id"],
        "filing_source": filing["filing_source"],
        "filing_type": filing.get("filing_type"),
        "title": filing["title"],
        "filed_at": date_to_ms(filing["filed_at"]),
        "url": filing.get("url"),
        "attachments": filing.get("attachments", []),
        "rcept_no": filing.get("rcept_no"),
        "created_at": NOW_MS,
    }
    items.append(put_item(item))


# ---------- 1. Issuer 16개 시드 ----------
for slug in ISSUER_META:
    add_issuer(slug)
print(f"✓ Issuer: {len(ISSUER_META)}건")


# ---------- 2. DART 사이클 → Sto + Filings ----------
cycles_path = CACHE / "_cycles.json"
filings_path = CACHE / "_filings_parsed.json"

cycles = []
if cycles_path.exists():
    cycles = json.load(open(cycles_path, encoding="utf-8"))
print(f"DART 사이클 입력: {len(cycles)}건")

# rcept_no → 사이클 매핑은 _groups.json 에서. 우리는 사이클당 initial_rcept_no 만 있음
# 모든 공시는 filings_raw 에서 다시 읽고, 사이클 매핑은 발행사+initial_rcept_no 기준
sto_count_dart = 0
for cyc in cycles:
    issuer = cyc["issuer"]
    slug = ISSUER_SLUG.get(issuer)
    if not slug:
        continue
    cat_meta = ISSUER_META[slug]["category"]
    sto_id = make_sto_id("DART", cyc["initial_rcept_no"])
    status_raw = cyc.get("status", "신고중")
    status = STATUS_NORMALIZE.get(status_raw, status_raw)

    offering = None
    if cyc.get("amount") or cyc.get("units"):
        offering = {
            "amount": cyc.get("amount"),
            "currency": "KRW",
            "total_units": cyc.get("units"),
        }
        if cyc.get("amount") and cyc.get("units"):
            offering["unit_price"] = cyc["amount"] // cyc["units"]

    add_sto({
        "sto_id": sto_id,
        "name": cyc.get("asset_name") or f"{issuer} 사이클 #{cyc['cycle_no']}",
        "underlying": cyc.get("asset_name"),
        "category": cat_meta,
        "region": "KR",
        "country": "🇰🇷 한국",
        "issuer_id": slug,
        "security_type": "투자계약증권",
        "classification": "정식 투자계약증권 (DART)",
        "status": status,
        "issued_at": f"{cyc['initial_filing_date'][:4]}-{cyc['initial_filing_date'][4:6]}-{cyc['initial_filing_date'][6:]}",
        "origin": "DART",
        "external_id": cyc["initial_rcept_no"],
        "external_url": f"https://dart.fss.or.kr/dsaf001/main.do?rcpNo={cyc['initial_rcept_no']}",
        "offering": offering,
        "issuance_structure": {"issuer": issuer},
        "artist": cyc.get("artist"),
        "year": cyc.get("year"),
        "sources": [{"src": "DART", "label": f"DART 증권신고서 ({cyc['initial_rcept_no']})"}],
    })
    sto_count_dart += 1
print(f"✓ DART Sto: {sto_count_dart}건")

# DART filings_raw — 197건. 사이클당 initial_rcept_no 매칭이 어려우므로
# 각 filing 의 발행사(corp_name)·rcept_dt 로 동일 발행사의 모든 사이클 STO 에 묶기엔 모호
# → 첫 단계는 *raw 공시 전체를 별도 PK STO#dart-raw 가 아니라 발행사 기준 그룹*
# 일단 1회성 시드 단계에서 *filings_raw 는 적재 생략* 하고, 사이클 매핑된 사이클의 initial 만 Filing 으로
# (raw 공시 일괄 적재는 다음 작업 분할)

# 사이클별 initial 공시를 Filing 으로
for cyc in cycles:
    slug = ISSUER_SLUG.get(cyc["issuer"])
    if not slug:
        continue
    sto_id = sto_id_map.get(("DART", cyc["initial_rcept_no"]))
    if not sto_id:
        continue
    add_filing(sto_id, {
        "filing_id": cyc["initial_rcept_no"],
        "filing_source": "DART",
        "filing_type": "증권신고서",
        "title": cyc.get("asset_name") or f"{cyc['issuer']} 증권신고서",
        "filed_at": f"{cyc['initial_filing_date'][:4]}-{cyc['initial_filing_date'][4:6]}-{cyc['initial_filing_date'][6:]}",
        "url": f"https://dart.fss.or.kr/dsaf001/main.do?rcpNo={cyc['initial_rcept_no']}",
        "rcept_no": cyc["initial_rcept_no"],
    })
print(f"✓ DART Filing: {len(cycles)}건")


# ---------- 3. Musicow → Sto + Filings ----------
catalog_path = MOCK / "musicow-catalog.json"
if catalog_path.exists():
    catalog = json.load(open(catalog_path, encoding="utf-8"))
    songs = catalog.get("songs", [])
    print(f"Musicow 카탈로그 입력: {len(songs)}곡")

    sto_count_music = 0
    filing_count_music = 0
    for song in songs:
        sid_external = song["id"].replace("musicow-", "")
        sto_id = make_sto_id("MUSICOW", sid_external)

        offering = None
        disclosure = song.get("disclosure")
        pdf_details = disclosure.get("pdfDetails") if disclosure else None
        if pdf_details and song.get("offering"):
            offering = song["offering"]

        add_sto({
            "sto_id": sto_id,
            "name": song["name"],
            "underlying": f"{song.get('artist','?')} - {song['name']}",
            "category": "music",
            "region": "KR",
            "country": "🇰🇷 한국",
            "issuer_id": "musicow",
            "security_type": "음악 수익증권",
            "classification": "혁신금융서비스 기반 음악 IP 수익증권",
            "status": "발행완료",
            "issued_at": "2023-09-19",
            "origin": "MUSICOW",
            "external_id": sid_external,
            "external_url": song.get("externalUrl"),
            "offering": offering,
            "issuance_structure": {
                "issuer": "㈜뮤직카우인베스트",
                "trustee": "키움증권㈜",
                "trustee_role": "보관기관",
            },
            "artist": song.get("artist"),
            "rights_category": song.get("rightsCategory"),
            "trust_no": pdf_details.get("trustContractNo") if pdf_details else None,
            "sources": song.get("sources", []),
        })
        sto_count_music += 1

        # Disclosure 있는 곡만 Filing 추가
        if disclosure:
            add_filing(sto_id, {
                "filing_id": disclosure["noticeId"],
                "filing_source": "MUSICOW",
                "filing_type": "발행안내",
                "title": disclosure["title"],
                "filed_at": disclosure["publishedAt"],
                "url": disclosure["noticePageUrl"],
                "attachments": [
                    {"name": f["name"], "url": f["url"], "size_bytes": f.get("size")}
                    for f in disclosure.get("files", [])
                ],
            })
            filing_count_music += 1
    print(f"✓ Musicow Sto: {sto_count_music}건 / Filing: {filing_count_music}건")


# ---------- 4. 출력 ----------
OUT.parent.mkdir(parents=True, exist_ok=True)
with open(OUT, "w", encoding="utf-8") as f:
    json.dump({TABLE: items}, f, ensure_ascii=False, indent=2)

print(f"\n총 {len(items)} items → {OUT}")
print(f"({len(items)/25:.1f}개 chunk for BatchWriteItem)")
