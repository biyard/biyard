#!/usr/bin/env python3
"""
Parse cached Kasa/sou PDFs to extract real estate metadata,
then patch kasa-scraped.json and lucentblock-scraped.json.

Requires: pip install pdfminer.six

Run after scrape-kasa-details.py and scrape-sou-details.py:
  python3 tools/parse-realestate-pdfs.py
"""

import json, re
from pathlib import Path

ROOT = Path(__file__).parent.parent
MOCK = ROOT / "sto-mockup"

KASA_SCRAPED    = MOCK / "kasa-scraped.json"
LUCENT_SCRAPED  = MOCK / "lucentblock-scraped.json"
KASA_DETAILS    = MOCK / "kasa-building-details.json"
KASA_PDFS       = MOCK / "kasa-pdfs"
SOU_PDFS        = MOCK / "sou-pdfs"


def extract_text(pdf_path: Path) -> str:
    try:
        from pdfminer.high_level import extract_text as _extract
        return _extract(str(pdf_path))
    except Exception as e:
        print(f"  PDF parse error {pdf_path.name}: {e}")
        return ""


def find_address(text: str) -> str | None:
    # 소재지 패턴: "서울 강남구 ..." or "경기도 ..." etc.
    patterns = [
        r"소\s*재\s*지\s*[：:]\s*([^\n]{5,60})",
        r"부동산의\s*소재지\s*[：:]\s*([^\n]{5,60})",
        r"물건지\s*[：:]\s*([^\n]{5,60})",
        r"(서울|경기|인천|부산|대구|광주|대전|울산|세종|강원|충북|충남|전북|전남|경북|경남|제주)[^\n]{5,50}",
    ]
    for pat in patterns:
        m = re.search(pat, text)
        if m:
            return m.group(1).strip() if m.lastindex else m.group(0).strip()
    return None


def find_floor_area(text: str) -> str | None:
    patterns = [
        r"연\s*면\s*적\s*[：:]\s*([\d,\.]+\s*㎡)",
        r"총\s*면\s*적\s*[：:]\s*([\d,\.]+\s*㎡)",
        r"건\s*물\s*면\s*적\s*[：:]\s*([\d,\.]+\s*㎡)",
        r"([\d,\.]+)\s*㎡\s*\(",
    ]
    for pat in patterns:
        m = re.search(pat, text)
        if m:
            return m.group(1).strip()
    return None


def find_building_type(text: str) -> str | None:
    patterns = [
        r"용\s*도\s*[：:]\s*([^\n]{3,30})",
        r"건물\s*용도\s*[：:]\s*([^\n]{3,30})",
        r"주\s*용\s*도\s*[：:]\s*([^\n]{3,30})",
    ]
    for pat in patterns:
        m = re.search(pat, text)
        if m:
            return m.group(1).strip()
    return None


# ── Kasa: patch from __NEXT_DATA__ cache (more reliable than PDF parsing) ──────

def patch_kasa_from_details():
    if not KASA_DETAILS.exists():
        print("kasa-building-details.json not found, run scrape-kasa-details.py first")
        return

    details = json.loads(KASA_DETAILS.read_text())
    # name → detail map
    by_name = {d["name"]: d for d in details}

    scraped = json.loads(KASA_SCRAPED.read_text())
    changed = 0
    for item in scraped:
        name = item.get("name", "")
        detail = by_name.get(name)
        if not detail:
            print(f"  [kasa] No detail match for: {name}")
            continue

        # address: prefer detail over existing
        if detail.get("address_street"):
            item["address"] = detail["address_street"]
        if detail.get("usage_purpose"):
            item["building_type"] = detail["usage_purpose"]
        if detail.get("floors"):
            item["floors"] = detail["floors"]
        if detail.get("gross_floor_area"):
            item["floor_area"] = f"{detail['gross_floor_area']}㎡"
        if detail.get("lat") and detail.get("lng"):
            item["lat"] = detail["lat"]
            item["lng"] = detail["lng"]
        if detail.get("completion_date"):
            item["completion_date"] = detail["completion_date"]
        changed += 1

    KASA_SCRAPED.write_text(json.dumps(scraped, ensure_ascii=False, indent=2))
    print(f"  [kasa] Patched {changed} items → {KASA_SCRAPED}")


# ── sou.place: patch from PDFs ──────────────────────────────────────────────────

def patch_sou_from_pdfs():
    if not SOU_PDFS.exists() or not any(SOU_PDFS.iterdir()):
        print("sou-pdfs/ empty, run scrape-sou-details.py first")
        return

    scraped = json.loads(LUCENT_SCRAPED.read_text())
    changed = 0

    for item in scraped:
        name = item.get("name", "")
        # Find matching PDF (증권신고서 or 투자설명서 preferred)
        safe_name = re.sub(r'[\\/:*?"<>|]', '_', name)
        candidates = list(SOU_PDFS.glob(f"{safe_name}*증권신고서*.pdf")) + \
                     list(SOU_PDFS.glob(f"{safe_name}*투자설명서*.pdf")) + \
                     list(SOU_PDFS.glob(f"{safe_name}*.pdf"))

        if not candidates:
            print(f"  [sou] No PDF for: {name}")
            continue

        pdf_path = candidates[0]
        print(f"  [sou] Parsing {pdf_path.name} ...")
        text = extract_text(pdf_path)
        if not text:
            continue

        addr = find_address(text)
        area = find_floor_area(text)
        btype = find_building_type(text)

        if addr and not item.get("address"):
            item["address"] = addr
            changed += 1
        if area:
            item["floor_area"] = area
            changed += 1
        if btype and not item.get("building_type"):
            item["building_type"] = btype
            changed += 1

        print(f"    addr={addr}, area={area}, type={btype}")

    LUCENT_SCRAPED.write_text(json.dumps(scraped, ensure_ascii=False, indent=2))
    print(f"  [sou] Patched {changed} fields → {LUCENT_SCRAPED}")


def main():
    print("=== Patching Kasa from __NEXT_DATA__ cache ===")
    patch_kasa_from_details()
    print()
    print("=== Patching sou.place from PDFs ===")
    patch_sou_from_pdfs()
    print("\nDone. Re-run build-sto-seed.py to regenerate seed data.")


if __name__ == "__main__":
    main()
