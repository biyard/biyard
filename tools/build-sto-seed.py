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

# 발행사 메타 — DB 컬럼은 모두 enum 영문 키 (`category`, `country`, `status`).
# 시리즈/인가 단계 등 자유 코멘트는 `status_note` 로 분리 (스키마 분리 원칙).
ISSUER_META = {
    "stockeeper": {"name": "스탁키퍼 (뱅카우)", "category": "LIVESTOCK", "country": "KR", "status": "OPERATING", "status_note": None, "description": "한우를 기초자산으로 한 가축투자계약증권을 발행합니다. 신한투자증권을 통해 계좌를 개설하면 청약할 수 있습니다."},
    "datagen": {"name": "데이터젠 (핀돈)", "category": "LIVESTOCK", "country": "KR", "status": "OPERATING", "status_note": None, "description": "한돈을 기초자산으로 하는 투자계약증권 발행사입니다. 계좌관리는 하나증권이 담당합니다."},
    "togetherart": {"name": "투게더아트", "category": "ART", "country": "KR", "status": "OPERATING", "status_note": None, "description": "유명 작가의 미술품을 투자계약증권 형태로 공모합니다."},
    "yeolmae": {"name": "열매컴퍼니 (아트앤가이드)", "category": "ART", "country": "KR", "status": "OPERATING", "status_note": None, "description": "미술품 공동구매와 투자계약증권 발행을 함께 운영합니다."},
    "seoulauctionblue": {"name": "서울옥션블루 (SOTWO)", "category": "ART", "country": "KR", "status": "OPERATING", "status_note": None, "description": "서울옥션 관계사로, 미술품 조각투자 플랫폼 ‘SOTWO’를 운영합니다."},
    "artipio": {"name": "아티피오 (Artipio)", "category": "ART", "country": "KR", "status": "OPERATING", "status_note": None, "description": "컨템포러리 작가의 작품을 기초자산으로 미술품 투자계약증권을 운영합니다."},
    "kasa": {"name": "카사 (Kasa Korea)", "category": "REAL_ESTATE", "country": "KR", "status": "OPERATING", "status_note": None, "description": "상업용 부동산을 디지털 수익증권(DABS)으로 발행합니다."},
    "lucentblock": {"name": "루센트블록 (소유)", "category": "REAL_ESTATE", "country": "KR", "status": "OPERATING", "status_note": None, "description": "지방·중소형 상업용 부동산을 신탁수익증권으로 공모합니다."},
    "funble": {"name": "펀블 (Funble)", "category": "REAL_ESTATE", "country": "KR", "status": "OPERATING", "status_note": None, "description": "랜드마크 상업용 부동산을 신탁수익증권으로 운영합니다."},
    "musicow": {"name": "뮤직카우 (Musicow)", "category": "MUSIC", "country": "KR", "status": "OPERATING", "status_note": None, "description": "음악 저작권을 기초자산으로 한 신탁수익증권을 운영합니다."},
}

# DART 사이클 상태 → STO_STATUS enum 키 매핑.
STATUS_NORMALIZE = {
    "발행완료": "ISSUED",
    "발행 완료": "ISSUED",
    "청산 완료": "LIQUIDATED",
    "신고중": "FILED",
    "증권신고서 제출": "FILED",
    "철회": "WITHDRAWN",
}


# ---------- 빌더 ----------
items = []   # 최종 PutRequest 리스트
sto_id_map = {}  # (origin, external_id) → sto_id 매핑 (재사용)
agg_count = {"MUSIC": 0, "ART": 0, "REAL_ESTATE": 0, "LIVESTOCK": 0}
agg_amount = {"MUSIC": 0, "ART": 0, "REAL_ESTATE": 0, "LIVESTOCK": 0}


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
        "country": meta["country"],
        "category": meta["category"],
        "description": meta["description"],
        "status": meta["status"],
        "status_note": meta.get("status_note"),
        "created_at": NOW_MS,
        "updated_at": NOW_MS,
        "gsi4_pk": "ISSUER",
        "gsi4_sk": f"TS#{NOW_MS}#{slug}",
    }
    items.append(put_item(item))


def build_category_meta(sto):
    """카테고리별 부가 메타를 평탄 dict 로 반환. 부가 필드가 전혀 없으면 None.
    카테고리는 enum 영문 키 (`MUSIC`, `ART`, `REAL_ESTATE`, `LIVESTOCK`).
    """
    cat = sto["category"]
    if cat == "MUSIC":
        meta = {
            "artist": sto.get("artist"),
            "rights_category": sto.get("rights_category"),
            "trust_no": sto.get("trust_no"),
            "year": sto.get("year"),
        }
    elif cat == "ART":
        meta = {
            "artist": sto.get("artist"),
            "artwork_year": sto.get("year"),
            "medium": sto.get("medium"),
            "dimensions": sto.get("dimensions"),
        }
    elif cat == "REAL_ESTATE":
        meta = {
            "address": sto.get("address"),
            "building_type": sto.get("building_type"),
            "floor_area": sto.get("floor_area"),
            "land_area": sto.get("land_area"),
            "floors": sto.get("floors"),
            "completion_date": sto.get("completion_date"),
            "trustee": sto.get("issuance_structure", {}).get("trustee"),
            "tenant": sto.get("tenant"),
            "lease_term": sto.get("lease_term"),
            "total_offering": sto.get("total_offering"),
            "total_units": sto.get("total_units_str"),
            "unit_price": sto.get("unit_price_str"),
            "upfront_fee": sto.get("upfront_fee"),
            "dividend_frequency": sto.get("dividend_frequency"),
            "appraisal_values": sto.get("appraisal_values"),
        }
    elif cat == "LIVESTOCK":
        meta = {
            "farm_name": sto.get("farm_name"),
            "breed": sto.get("breed"),
            "head_count": sto.get("head_count"),
        }
    else:
        return None
    # 모든 부가 필드가 비어있으면 row 만들지 않음
    if all(v in (None, "") for v in meta.values()):
        return None
    return meta


def add_sto(sto):
    """sto: dict — required: name, category, country, status, issued_at, origin, external_id.
    모든 enum 키 (`category`, `country`, `status`, `origin`) 는 영문 UPPER_SNAKE 로 들어와야 한다.
    `issued_at` 은 `YYYY-MM-DD` (또는 빈 문자열) 로 받고, 내부에서 epoch ms 로 변환.
    """
    sto_id = sto["sto_id"]
    issued_at_ms = date_to_ms(sto.get("issued_at", ""))
    issuer_id_val = sto.get("issuer_id")
    issuer_name_val = ISSUER_META.get(issuer_id_val, {}).get("name") if issuer_id_val else None
    item = {
        "pk": f"STO#{sto_id}",
        "sk": "STO",
        "sto_id": sto_id,
        "name": sto["name"],
        "underlying": sto.get("underlying"),
        "category": sto["category"],
        "country": sto["country"],
        "issuer_id": issuer_id_val,
        "issuer_name": issuer_name_val,
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
        # GSI fields — enum 키 그대로 사용
        "gsi1_pk": f"STATUS#{sto['status']}",
        "gsi1_sk": f"TS#{issued_at_ms}#{sto_id}",
        "gsi2_pk": f"CAT#{sto['country']}#{sto['category']}",
        "gsi2_sk": f"TS#{issued_at_ms}#{sto_id}",
        "country_category": f"{sto['country']}#{sto['category']}",
        "gsi4_pk": "STO",
        "gsi4_sk": f"TS#{issued_at_ms}#{sto_id}",
    }
    if sto.get("issuer_id"):
        item["gsi3_pk"] = f"ISSUER#{sto['issuer_id']}"
        item["gsi3_sk"] = f"TS#{issued_at_ms}#{sto_id}"
    items.append(put_item(item))

    # Aggregator 누적
    cat = sto["category"]
    if cat in agg_count:
        agg_count[cat] += 1
        if sto.get("offering") and sto["offering"].get("amount"):
            agg_amount[cat] += int(sto["offering"]["amount"])

    meta = build_category_meta(sto)
    if meta:
        # 카테고리 메타 row 는 nested map 으로 박지 않고 attribute 자체에 평탄 컬럼으로 적재.
        # sk 는 카테고리 enum 영문 키.
        meta_item = {
            "pk": f"STO#{sto_id}",
            "sk": f"STO_META#{sto['category']}",
            "created_at": NOW_MS,
            "updated_at": NOW_MS,
            **meta,
        }
        items.append(put_item(meta_item))


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


import re

# 한우/한돈처럼 기초자산이 동질적이라 회차 식별이 유일한 자산명 단서인 경우만
# "사이클 #N" → "제N호" 표기로 정규화. 미술품·부동산·음악처럼 자산 자체가 고유한
# 정체성을 갖는 카테고리는 원본 자산명을 그대로 사용한다.
_CYCLE_RE = re.compile(r"^(.*?)\s*사이클\s*#?\s*(\d+)\s*$")


def normalize_fungible_name(name: str | None, issuer: str, cycle_no: int) -> str:
    """동질 기초자산 (livestock) STO 의 회차 표기 정규화.
    - "뱅카우 한우 사이클 #1" → "뱅카우 한우 제1호"
    - 자산명이 비어있거나 상태 라벨만 있음 (예: "(철회)") → "{issuer} 제{cycle_no}호"
      (상태 자체는 `status` 필드에 별도 저장; 자산명에 박지 않는다)
    """
    if not name or not name.strip():
        return f"{issuer} 제{cycle_no}호"
    s = name.strip()
    if s.startswith("(") and s.endswith(")"):
        return f"{issuer} 제{cycle_no}호"
    m = _CYCLE_RE.match(s)
    if m:
        prefix = m.group(1).strip()
        n = int(m.group(2))
        return f"{prefix or issuer} 제{n}호"
    return s


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
else:
    # /tmp 캐시가 휘발됐을 때 sto-mockup/dart-viewer-data.json 에 백업된 cycles 사용
    viewer_path = MOCK / "dart-viewer-data.json"
    if viewer_path.exists():
        viewer = json.load(open(viewer_path, encoding="utf-8"))
        cycles = viewer.get("cycles", [])
        print(f"  (fallback) dart-viewer-data.json 에서 cycles 로드")
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

    raw_name = cyc.get("asset_name")
    # 카테고리별 자산명 결정:
    #   livestock — 회차 표기 (한우/한돈은 자산이 동질적)
    #   art/real_estate/music — 자산 자체가 고유하므로 원본 작품명/건물명/곡명 그대로
    # 상태 라벨만 들어온 경우 ("(철회)") 는 카테고리 무관 발행사 + 호수로 fallback.
    cycle_no = cyc.get("cycle_no") or 0
    if cat_meta == "LIVESTOCK":
        sto_name = normalize_fungible_name(raw_name, issuer, cycle_no)
    else:
        if not raw_name or not raw_name.strip() or (
            raw_name.strip().startswith("(") and raw_name.strip().endswith(")")
        ):
            sto_name = f"{issuer} 제{cycle_no}호"
        else:
            sto_name = raw_name.strip()

    # underlying 은 표시명과 별개의 "기초자산 설명" — 한우는 회차 자체가 기초자산이라 동일,
    # 그 외 카테고리는 원본 asset_name (작품명/건물명/곡명) 이 기초자산 그 자체.
    underlying = sto_name if cat_meta == "LIVESTOCK" else (raw_name or sto_name)

    add_sto({
        "sto_id": sto_id,
        "name": sto_name,
        "underlying": underlying,
        "category": cat_meta,
        "country": "KR",
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

# DART 공시 (filings_raw) 를 cycle 별로 매핑해 적재.
#  - 매핑 키: _groups.json 의 그룹 안에 있는 filing.rcept_no 중 하나가 cycle.initial_rcept_no 와 일치하면 그 cycle
#  - 그룹 단위로 모든 filing 을 같은 sto_id 에 적재 (정정/투자설명서/발행실적보고서/철회 등)
#  - cycle 과 매칭되지 않는 그룹은 누락 (rare — 30개 그룹 중 cycle 매칭 가능 그룹만 사용)
def normalize_filing_type(report_nm: str) -> str:
    """DART report_nm → FilingType enum 키."""
    s = report_nm or ""
    if "철회" in s:
        return "WITHDRAWN"  # FilingType 에 WITHDRAWN 이 없으면 OTHER 로 강등 (아래에서 보정)
    if "발행실적보고서" in s:
        return "ISSUANCE_REPORT"
    if "투자설명서" in s:
        return "PROSPECTUS"
    if "정정" in s and "증권신고서" in s:
        return "CORRECTED"
    if "증권신고서" in s:
        return "SECURITIES_REGISTRATION"
    return "OTHER"


# FilingType enum 에 정의된 값만 허용 (그 외는 OTHER 로 강등)
ALLOWED_FILING_TYPES = {"SECURITIES_REGISTRATION", "CORRECTED", "PROSPECTUS",
                        "ISSUANCE_REPORT", "PERIODIC", "MATERIAL", "OTHER", "UNKNOWN"}

# cycle 의 initial_rcept_no → sto_id 인덱스 (위에서 sto_id_map 에 이미 만들어둠)
groups_path = CACHE / "_groups.json"
groups = {}
if groups_path.exists():
    groups = json.load(open(groups_path, encoding="utf-8"))
else:
    # /tmp 캐시가 휘발됐을 때 filings_raw 의 각 filing 을 단독 그룹으로 만들어
    # resolve_sto_for_group 의 시간근접 fallback 으로 cycle 에 매핑한다.
    viewer_path = MOCK / "dart-viewer-data.json"
    if viewer_path.exists():
        viewer = json.load(open(viewer_path, encoding="utf-8"))
        for f in viewer.get("filings_raw", []):
            gkey = f"{f['_issuer']}#{f['rcept_no']}"
            groups[gkey] = [{
                "rcept_no": f["rcept_no"],
                "rcept_dt": f["rcept_dt"],
                "report_nm": f.get("report_nm", ""),
            }]
        print(f"  (fallback) dart-viewer-data.json 에서 filings_raw → groups 재구성")
print(f"DART filing 그룹 입력: {len(groups)}개")

# 보조 인덱스: (issuer, cycle_no) → sto_id (Primary serial → cycle_no fallback 용)
issuer_cycle_index: dict[tuple[str, int], str] = {}
issuer_cycle_list: dict[str, list[dict]] = {}
for cyc in cycles:
    slug = ISSUER_SLUG.get(cyc["issuer"])
    if not slug:
        continue
    sid = sto_id_map.get(("DART", cyc["initial_rcept_no"]))
    if not sid:
        continue
    issuer_cycle_index[(cyc["issuer"], cyc["cycle_no"])] = sid
    issuer_cycle_list.setdefault(cyc["issuer"], []).append({
        "cycle_no": cyc["cycle_no"],
        "initial_filing_date": cyc["initial_filing_date"],
        "sto_id": sid,
    })


def resolve_sto_for_group(group_key: str, glist: list[dict]) -> str | None:
    """그룹의 cycle 매핑 — 3단계 fallback.
    1) 그룹 내 어떤 filing.rcept_no 가 cycle.initial_rcept_no 와 일치
    2) group_key 의 serial 첫 숫자가 cycle_no 와 일치 (같은 issuer 안에서)
    3) 그룹 first filing 의 rcept_dt 직전에 시작된 같은 issuer cycle 중 가장 늦은 것
    """
    # 1) rcept_no 직접 매칭
    for f in glist:
        sid = sto_id_map.get(("DART", f["rcept_no"]))
        if sid:
            return sid
    # 2) serial 첫 숫자 → cycle_no
    if "#" in group_key:
        issuer_part, serial = group_key.split("#", 1)
        head = serial.split("-")[0]
        if head.isdigit():
            sid = issuer_cycle_index.get((issuer_part, int(head)))
            if sid:
                return sid
    # 3) 시간 근접: group 의 가장 이른 rcept_dt 직전 cycle
    if "#" in group_key:
        issuer_part, _ = group_key.split("#", 1)
        cycles_for_issuer = issuer_cycle_list.get(issuer_part, [])
        if cycles_for_issuer:
            earliest_filing_dt = min(f["rcept_dt"] for f in glist)
            # 같거나 이전인 cycle 중 가장 늦은 것
            candidates = [c for c in cycles_for_issuer if c["initial_filing_date"] <= earliest_filing_dt]
            if candidates:
                best = max(candidates, key=lambda c: c["initial_filing_date"])
                return best["sto_id"]
    return None


filing_count_dart = 0
unmapped_groups = 0
for gkey, glist in groups.items():
    sto_id = resolve_sto_for_group(gkey, glist)
    if not sto_id:
        unmapped_groups += 1
        continue
    for f in glist:
        ft = normalize_filing_type(f.get("report_nm", ""))
        if ft not in ALLOWED_FILING_TYPES:
            ft = "OTHER"
        add_filing(sto_id, {
            "filing_id": f["rcept_no"],
            "filing_source": "DART",
            "filing_type": ft,
            "title": f.get("report_nm") or "(제목 없음)",
            "filed_at": f"{f['rcept_dt'][:4]}-{f['rcept_dt'][4:6]}-{f['rcept_dt'][6:]}",
            "url": f"https://dart.fss.or.kr/dsaf001/main.do?rcpNo={f['rcept_no']}",
            "rcept_no": f["rcept_no"],
        })
        filing_count_dart += 1
print(f"✓ DART Filing: {filing_count_dart}건 (매핑된 그룹 {len(groups) - unmapped_groups}/{len(groups)})")


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
        sto_id = make_sto_id("COMPANY", sid_external)

        offering = None
        disclosure = song.get("disclosure")
        pdf_details = disclosure.get("pdfDetails") if disclosure else None
        if pdf_details and song.get("offering"):
            offering = song["offering"]

        add_sto({
            "sto_id": sto_id,
            "name": song["name"],
            "underlying": f"{song.get('artist','?')} - {song['name']}",
            "category": "MUSIC",
            "country": "KR",
            "issuer_id": "musicow",
            "security_type": "음악 수익증권",
            "classification": "혁신금융서비스 기반 음악 IP 수익증권",
            "status": "ISSUED",
            "issued_at": "2023-09-19",
            "origin": "COMPANY",
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
                "filing_source": "COMPANY",
                "filing_type": "OTHER",
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


# ---------- 3.5. 부동산 STO (스크래핑 결과) ----------
# tools/scrape-realestate.py 실행 결과가 있으면 자동 포함.
# 없으면 조용히 skip.

# 증권신고서 파싱 결과 — 이름으로 매칭해 부가 필드 merge
_prospectus_path = MOCK / "prospectus-parsed.json"
_prospectus_by_name: dict[str, dict] = {}
if _prospectus_path.exists():
    for entry in json.load(open(_prospectus_path, encoding="utf-8")):
        if entry.get("parse_status") == "ok":
            # 공백 제거 정규화 키로도 등록 (scraped name ↔ parsed name 불일치 대응)
            _prospectus_by_name[entry["name"]] = entry
            _prospectus_by_name[entry["name"].replace(" ", "")] = entry

def _merge_prospectus(row: dict) -> dict:
    """스크래핑 row에 증권신고서 파싱 결과를 merge.
    부동산 상세 필드는 파싱값 우선 (PDF가 더 정확); 이름/상태/날짜/URL은 스크래핑값 유지.
    """
    name = row.get("name", "")
    p = _prospectus_by_name.get(name) or _prospectus_by_name.get(name.replace(" ", ""), {})
    if not p:
        return row
    # 스크래핑 기본 필드는 유지, 파싱으로 보강할 상세 필드만 override
    PROSPECTUS_WINS = {
        "address", "floor_area", "land_area", "floors", "completion_date",
        "trustee", "tenant", "lease_term", "total_offering", "total_units",
        "unit_price", "upfront_fee", "dividend_frequency", "appraisal_values",
    }
    merged = dict(row)
    for k in PROSPECTUS_WINS:
        if p.get(k) is not None:
            merged[k] = p[k]
    return merged

SCRAPED_SOURCES = [
    (MOCK / "kasa-scraped.json",        "kasa",        "DABS (신탁수익증권)"),
    (MOCK / "lucentblock-scraped.json", "lucentblock", "신탁수익증권"),
    (MOCK / "funble-scraped.json",      "funble",      "신탁수익증권"),
]

realestate_count = 0
for scraped_path, issuer_slug, sec_type in SCRAPED_SOURCES:
    if not scraped_path.exists():
        continue
    scraped = json.load(open(scraped_path, encoding="utf-8"))
    for raw_row in scraped:
        name = raw_row.get("name")
        if not name:
            continue
        row = _merge_prospectus(raw_row)

        external_url = row.get("external_url", "")
        sto_id = deterministic_id("COMPANY", issuer_slug, name, external_url)
        offering = None
        # 모집총액: 스크래핑 amount(int) 또는 파싱 total_offering(str "X,XXX원") 활용
        amt = row.get("amount")
        up_str = row.get("unit_price")  # "5,000원"
        tu_str = row.get("total_units")  # "578,000"
        sub_end = row.get("subscription_end")
        sub_start = row.get("subscription_start")
        # unit_price/total_units: 숫자 정규화
        up_int = int(re.sub(r'[^0-9]', '', str(up_str))) if up_str else None
        tu_int = int(re.sub(r'[^0-9]', '', str(tu_str))) if tu_str else None
        if any(v for v in [amt, up_int, tu_int, sub_end]):
            sub_range = None
            if sub_start and sub_end:
                sub_range = f"{sub_start} ~ {sub_end}"
            elif sub_end:
                sub_range = f"~ {sub_end}"
            offering = {k: v for k, v in {
                "amount": amt,
                "unit_price": up_int,
                "total_units": tu_int,
                "subscription": sub_range,
            }.items() if v is not None}

        # trustee: 파싱 결과 우선, 스크래핑 fallback
        trustee = row.get("trustee")
        issuance = {"issuer": row.get("issuer", issuer_slug)}
        if trustee:
            issuance["trustee"] = trustee
        if row.get("underwriter"):
            issuance["underwriter"] = row["underwriter"]

        add_sto({
            "sto_id": sto_id,
            "name": name,
            "underlying": row.get("underlying") or name,
            "category": "REAL_ESTATE",
            "country": "KR",
            "issuer_id": issuer_slug,
            "security_type": sec_type,
            "classification": "DABS 기반 부동산 수익증권 (혁신금융서비스)",
            "status": row.get("status", "ISSUED"),
            "issued_at": row.get("issued_at") or "",
            "origin": "COMPANY",
            "external_id": external_url,
            "external_url": external_url,
            "offering": offering if offering else None,
            "issuance_structure": issuance,
            # 부동산 메타 — 스크래핑 + 파싱 merge
            "address": row.get("address"),
            "building_type": row.get("building_type"),
            "floor_area": row.get("floor_area"),
            "land_area": row.get("land_area"),
            "floors": row.get("floors"),
            "completion_date": row.get("completion_date"),
            "tenant": row.get("tenant"),
            "lease_term": row.get("lease_term"),
            "total_offering": row.get("total_offering"),
            "total_units_str": tu_str,
            "unit_price_str": up_str,
            "upfront_fee": row.get("upfront_fee"),
            "dividend_frequency": row.get("dividend_frequency"),
            "appraisal_values": row.get("appraisal_values"),
            "sources": row.get("sources", [{"src": "COMPANY", "label": f"{issuer_slug} 상품 페이지"}]),
        })
        realestate_count += 1
if realestate_count:
    print(f"✓ 부동산 STO (스크래핑): {realestate_count}건")


# ---------- 3.6. PlannedSto mock 3건 ----------
# 발행사·증권사가 등록한 "예상 공모" 카드 — DART 신고 이전 단계.
# 실제 운영 시에는 발행사 포털에서 등록되는 데이터.
PLANNED_MOCK = [
    {
        "planned_id": "planned-togetherart-yuyoungkuk",
        "name": "유영국 — Work (1984)",
        "category": "ART",
        "country": "KR",
        "issuer_id": "togetherart",
        "broker": "가람증권",
        "broker_role": "인수",
        "expected_amount": 660000000,
        "expected_window": "~ 05.18",
    },
    {
        "planned_id": "planned-stockeeper-2-1",
        "name": "뱅카우 한우 제2-1호",
        "category": "LIVESTOCK",
        "country": "KR",
        "issuer_id": "stockeeper",
        "broker": "해솔투자증권",
        "broker_role": "계좌관리",
        "expected_amount": 480000000,
        "expected_window": "~ 05.22",
    },
    {
        "planned_id": "planned-musicow-nctdream",
        "name": "NCT DREAM — ANL 후속",
        "category": "MUSIC",
        "country": "KR",
        "issuer_id": "musicow",
        "broker": "다온증권",
        "broker_role": "중개",
        "expected_amount": 320000000,
        "expected_window": "~ 05.25",
    },
]
for p in PLANNED_MOCK:
    iid = p["issuer_id"]
    item = {
        "pk": "PLANNED",
        "sk": f"PLANNED#{p['planned_id']}",
        "planned_id": p["planned_id"],
        "name": p["name"],
        "category": p["category"],
        "country": p["country"],
        "issuer_id": iid,
        "issuer_name": ISSUER_META.get(iid, {}).get("name"),
        "broker": p.get("broker"),
        "broker_role": p.get("broker_role"),
        "expected_amount": p.get("expected_amount"),
        "expected_window": p.get("expected_window"),
        "registered_at": NOW_MS,
        "registered_by": iid,
        "created_at": NOW_MS,
        "updated_at": NOW_MS,
        "gsi4_pk": "PLANNED",
        "gsi4_sk": f"TS#{NOW_MS}#{p['planned_id']}",
    }
    items.append(put_item(item))
print(f"✓ PlannedSto mock: {len(PLANNED_MOCK)}건")


# ---------- 4. Aggregator row ----------
total_count = sum(agg_count.values())
total_amount = sum(agg_amount.values())
agg_item = {
    "pk": "AGGREGATE",
    "sk": "AGGREGATE#CATEGORY_SCALE",
    "music_count": agg_count["MUSIC"],
    "music_amount": agg_amount["MUSIC"],
    "art_count": agg_count["ART"],
    "art_amount": agg_amount["ART"],
    "real_estate_count": agg_count["REAL_ESTATE"],
    "real_estate_amount": agg_amount["REAL_ESTATE"],
    "livestock_count": agg_count["LIVESTOCK"],
    "livestock_amount": agg_amount["LIVESTOCK"],
    "total_count": total_count,
    "total_amount": total_amount,
    "updated_at": NOW_MS,
}
items.append(put_item(agg_item))
print(f"✓ Aggregator: 1건 (총 {total_count} STO, 모집액 {total_amount:,} 원)")


# ---------- 5. 출력 ----------
OUT.parent.mkdir(parents=True, exist_ok=True)
with open(OUT, "w", encoding="utf-8") as f:
    json.dump({TABLE: items}, f, ensure_ascii=False, indent=2)

print(f"\n총 {len(items)} items → {OUT}")
print(f"({len(items)/25:.1f}개 chunk for BatchWriteItem)")
