#!/usr/bin/env python3
"""
부동산 STO 스크래퍼 — kasa.co.kr (DABS) + sou.place (루센트블록/소유)

사용법:
  pip install playwright
  playwright install chromium
  python3 tools/scrape-realestate.py

출력:
  sto-mockup/kasa-scraped.json        — Kasa DABS 자산 목록 (kasa.co.kr/building)
  sto-mockup/lucentblock-scraped.json — 루센트블록 자산 목록 (sou.place 공시자료)

각 파일 형식:
  [ { "name", "underlying", "address", "building_type", "status",
      "issued_at", "amount", "unit_price", "total_units",
      "subscription_start", "subscription_end",
      "issuer_id", "issuer", "trustee", "underwriter",
      "external_url", "sources" }, ... ]

build-sto-seed.py 가 이 파일을 읽어 add_sto() 로 투입.
파일이 없으면 seed 빌더가 조용히 skip 하므로, 스크래핑 실패 시 기존 수동 파일 유지.
"""

import json
import time
import re
from pathlib import Path

try:
    from playwright.sync_api import sync_playwright
except ImportError:
    print("playwright not installed. Run: pip install playwright && playwright install chromium")
    raise

REPO = Path(__file__).resolve().parent.parent
OUT_KASA = REPO / "sto-mockup" / "kasa-scraped.json"
OUT_LUCENT = REPO / "sto-mockup" / "lucentblock-scraped.json"

KASA_BASE = "https://www.kasa.co.kr"
LUCENT_BASE = "https://sou.place"


def clean(s: str | None) -> str | None:
    if not s:
        return None
    return " ".join(s.split()).strip() or None


def parse_amount_krw(s: str | None) -> int | None:
    """'6,600,000,000원' / '66억' / '21.3억' → int (원)."""
    if not s:
        return None
    s = s.replace(",", "").replace(" ", "").replace("원", "").strip()
    if "억" in s:
        parts = s.split("억")
        try:
            v = float(parts[0]) * 1_0000_0000
            if len(parts) > 1 and parts[1]:
                tail = parts[1].replace("만", "")
                if tail:
                    v += float(tail) * (1_0000 if "만" in parts[1] else 1)
            return int(v)
        except ValueError:
            return None
    try:
        return int(float(s))
    except ValueError:
        return None


def parse_date_loose(s: str | None) -> str | None:
    """'2024. 05. 07' / '2024.05.07' / '2024년 5월 7일' → 'YYYY-MM-DD'."""
    if not s:
        return None
    m = re.search(r"(\d{4})[.\s년]+(\d{1,2})[.\s월]+(\d{1,2})", s)
    if m:
        y, mo, d = m.groups()
        return f"{y}-{int(mo):02d}-{int(d):02d}"
    return None


# ─────────────────────────────────────────────────
# KASA — kasa.co.kr/building
# ─────────────────────────────────────────────────

# 카드 파싱: 목록 페이지 텍스트를 빌딩 단위로 분리.
# 카드 구조 (텍스트 순서):
#   상태 라벨 (거래 가능 / 매각 완료 / 투표 종료)
#   빌딩명
#   주소
#   공모기간  [날짜 ~ 날짜]  (없을 수도 있음)
#   공모금액  [N억 원]        (없을 수도 있음)
#   매각금액 / 상장일 / 거래가능일 …
STATUS_LABELS = {"거래 가능", "매각 완료", "투표 종료"}


def parse_kasa_listing(body: str, links: list[str]) -> list[dict]:
    """목록 페이지 body 텍스트 + 링크 배열 → 빌딩 dict 리스트."""
    lines = [l.strip() for l in body.splitlines() if l.strip()]

    # 빌딩 카드 시작: 상태 라벨 바로 다음 줄이 빌딩명, 그 다음이 주소
    buildings = []
    i = 0
    link_idx = 0

    nav_trash = {
        "법인 투자", "카사 소식", "빌딩 정보", "공시", "이벤트", "채용",
        "자주 묻는 질문", "카사 앱 다운로드", "빌딩 정보", "거래 방법 알아보기",
        "DABS 거래는", "왜 카사에서", "대표문의 contact@kasa.co.kr",
    }

    while i < len(lines):
        line = lines[i]
        if line in STATUS_LABELS:
            status_label = line
            if i + 2 >= len(lines):
                i += 1
                continue
            name = lines[i + 1]
            address = lines[i + 2]

            # Skip nav trash
            if name in nav_trash or address in nav_trash:
                i += 1
                continue

            # Map status
            status = {
                "거래 가능": "ISSUED",
                "투표 종료": "ISSUED",
                "매각 완료": "LIQUIDATED",
            }.get(status_label, "ISSUED")

            # Consume subsequent lines looking for dates/amounts
            j = i + 3
            subscription_start = None
            subscription_end = None
            amount = None
            issued_at = None
            building_url = None

            # Grab link for this building
            if link_idx < len(links):
                building_url = links[link_idx]
                link_idx += 1

            while j < len(lines) and lines[j] not in STATUS_LABELS:
                l = lines[j]
                if l == "공모기간" and j + 1 < len(lines):
                    period = lines[j + 1]
                    parts = period.replace("~", " ~ ").split("~")
                    subscription_start = parse_date_loose(parts[0].strip())
                    issued_at = subscription_start
                    if len(parts) > 1:
                        subscription_end = parse_date_loose(parts[1].strip())
                    j += 2
                    continue
                if l == "공모금액" and j + 1 < len(lines):
                    amount = parse_amount_krw(lines[j + 1])
                    j += 2
                    continue
                if l == "상장일" and j + 1 < len(lines) and not issued_at:
                    issued_at = parse_date_loose(lines[j + 1])
                    j += 2
                    continue
                if l == "거래가능일" and j + 1 < len(lines) and not issued_at:
                    issued_at = parse_date_loose(lines[j + 1])
                    j += 2
                    continue
                j += 1

            buildings.append({
                "name": name,
                "underlying": name,
                "address": address,
                "building_type": "상업용 빌딩",
                "status": status,
                "issued_at": issued_at,
                "amount": amount,
                "unit_price": None,
                "total_units": None,
                "subscription_start": subscription_start,
                "subscription_end": subscription_end,
                "issuer_id": "kasa",
                "issuer": "카사코리아",
                "trustee": None,
                "underwriter": None,
                "external_url": building_url,
                "sources": [{"src": "COMPANY", "label": "Kasa 빌딩 정보"}],
            })
            i = j
        else:
            i += 1

    return buildings


def scrape_kasa(page) -> list[dict]:
    page.goto(f"{KASA_BASE}/building", wait_until="networkidle", timeout=30_000)
    time.sleep(3)

    links = []
    for a in page.query_selector_all("a[href*='/building/']"):
        href = a.get_attribute("href") or ""
        if href and href != "/building":
            links.append(KASA_BASE + href if href.startswith("/") else href)

    body = page.inner_text("body")
    results = parse_kasa_listing(body, links)
    print(f"[kasa] {len(results)}건 파싱")
    return results


# ─────────────────────────────────────────────────
# SOY.PLACE (루센트블록) — 공시자료 탭
# ─────────────────────────────────────────────────

LOCATION_KEYWORDS = {
    "서울", "부산", "대전", "인천", "광주", "대구", "울산", "세종",
    "경기", "강원", "충북", "충남", "전북", "전남", "경북", "경남", "제주",
    "강남", "강북", "마포", "종로", "중구", "서초", "성동", "구로",
    "신도림", "성수", "문래", "이태원", "안국", "여의도", "압구정",
    "수원", "전주", "타워", "빌딩", "센터", "파크", "뮤지엄",
}


def _looks_like_building(name: str) -> bool:
    return any(kw in name for kw in LOCATION_KEYWORDS)


def scrape_lucentblock(page) -> list[dict]:
    page.goto(LUCENT_BASE, wait_until="networkidle", timeout=30_000)
    time.sleep(5)

    body = page.inner_text("body")

    # 공시자료 섹션은 SPA 렌더링 후 body 하단에 추가됨.
    # 탭 이름 목록은 "공시자료" 헤더 직후, 배당 공시 이전 줄들.
    idx = body.rfind("공시자료")
    if idx == -1:
        print("[lucentblock] 공시자료 섹션 못 찾음")
        return []

    disclosure_section = body[idx:]
    tabs_raw = disclosure_section.split("\n")
    tab_names = []
    for line in tabs_raw[1:]:
        line = line.strip()
        if not line:
            continue
        if re.match(r"\d+기 배당", line) or "발행실적" in line or "청약" in line:
            break
        if _looks_like_building(line):
            tab_names.append(line)

    if not tab_names:
        print("[lucentblock] 빌딩 탭명을 찾지 못함 (섹션 미렌더링 가능성) — 기존 파일 유지")
        return []

    print(f"[lucentblock] 빌딩 탭 {len(tab_names)}개: {tab_names}")

    results = []
    for i, name in enumerate(tab_names):
        results.append({
            "name": name,
            "underlying": name,
            "address": None,
            "building_type": "상업용 빌딩",
            "status": "ISSUED",
            "issued_at": None,
            "amount": None,
            "unit_price": None,
            "total_units": None,
            "subscription_start": None,
            "subscription_end": None,
            "issuer_id": "lucentblock",
            "issuer": "루센트블록",
            "trustee": None,
            "underwriter": None,
            "external_url": LUCENT_BASE,
            "sources": [{"src": "COMPANY", "label": f"소유(sou.place) 공시자료 — {i+1}호"}],
        })

    return results


# ─────────────────────────────────────────────────
# 메인
# ─────────────────────────────────────────────────

def main():
    OUT_KASA.parent.mkdir(parents=True, exist_ok=True)

    with sync_playwright() as pw:
        browser = pw.chromium.launch(
            headless=True,
            args=["--no-sandbox", "--disable-dev-shm-usage"],
        )
        ctx = browser.new_context(
            user_agent=(
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) "
                "AppleWebKit/537.36 (KHTML, like Gecko) "
                "Chrome/124.0.0.0 Safari/537.36"
            ),
            locale="ko-KR",
        )
        page = ctx.new_page()

        print("=== Kasa 스크래핑 ===")
        try:
            kasa_data = scrape_kasa(page)
        except Exception as e:
            print(f"Kasa 실패: {e}")
            kasa_data = []
        print(f"Kasa: {len(kasa_data)}건")
        if kasa_data:
            OUT_KASA.write_text(json.dumps(kasa_data, ensure_ascii=False, indent=2))
            print(f"→ {OUT_KASA}")

        print("\n=== 루센트블록 (sou.place) 스크래핑 ===")
        try:
            lucent_data = scrape_lucentblock(page)
        except Exception as e:
            print(f"루센트블록 실패: {e}")
            lucent_data = []
        print(f"루센트블록: {len(lucent_data)}건")
        if lucent_data:
            OUT_LUCENT.write_text(json.dumps(lucent_data, ensure_ascii=False, indent=2))
            print(f"→ {OUT_LUCENT}")

        browser.close()

    print("\n완료. build-sto-seed.py 를 실행해 시드 데이터를 재생성하세요.")
    print("  python3 tools/build-sto-seed.py")


if __name__ == "__main__":
    main()
