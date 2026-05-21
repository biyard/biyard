#!/usr/bin/env python3
"""
Kasa building detail scraper.

Steps:
  1. Visit kasa.co.kr/building → extract __NEXT_DATA__ → cache as kasa-building-details.json
  2. For each building, visit its disclosure page → click 증권신고서 button →
     capture the signed PDF URL from the new tab → download PDF to sto-mockup/kasa-pdfs/

Run:
  cd /path/to/biyard
  pip install playwright requests
  playwright install chromium
  python3 tools/scrape-kasa-details.py

Re-running is safe: skips already-downloaded PDFs.
"""

import json, time, re, os, sys
from pathlib import Path
import requests
from playwright.sync_api import sync_playwright

ROOT    = Path(__file__).parent.parent
MOCK    = ROOT / "sto-mockup"
PDF_DIR = MOCK / "kasa-pdfs"
CACHE   = MOCK / "kasa-building-details.json"

PDF_DIR.mkdir(parents=True, exist_ok=True)


def scrape_building_details(page) -> list[dict]:
    print("→ Loading kasa.co.kr/building ...")
    page.goto("https://www.kasa.co.kr/building", wait_until="networkidle", timeout=30000)

    dabses = page.evaluate("""() => {
        const dabses = window.__NEXT_DATA__?.props?.pageProps?.dabses || [];
        return dabses.map(d => ({
            code: d.code,
            name: d.name,
            status: d.uiStatus,
            address_street: d.building?.address?.street || null,
            address_postal: d.building?.address?.postalCode || null,
            floors: d.building?.numberOfStories || null,
            gross_floor_area: d.building?.grossFloorArea || null,
            site_area: d.building?.siteArea || null,
            usage_purpose: d.building?.usagePurposeDisplay || null,
            completion_date: d.building?.completionDate || null,
            lat: d.building?.latLng?.latitude || null,
            lng: d.building?.latLng?.longitude || null,
            offering_price: d.offering?.price || null,
            offering_units: d.offering?.units || null,
            ksd_name_kr: d.ksdDabsNameKr || null,
        }));
    }""")
    print(f"  Found {len(dabses)} buildings")
    return dabses


def scrape_disclosure_pdf_urls(page, context, buildings: list[dict]) -> dict[str, str]:
    """
    Visit each building's disclosure page, click 증권신고서, capture signed PDF URL.
    Returns { code: pdf_url }.
    Already-downloaded codes are skipped.
    """
    pdf_urls = {}

    for b in buildings:
        code = b["code"]
        name = b["name"]
        pdf_path = PDF_DIR / f"{code}-증권신고서.pdf"

        if pdf_path.exists():
            print(f"  [{name}] PDF already cached, skipping")
            continue

        print(f"  [{name}] visiting disclosure page ...")
        disc_url = f"https://www.kasa.co.kr/disclosure/{code}"
        page.goto(disc_url, wait_until="networkidle", timeout=30000)
        time.sleep(1)

        # Find the first 증권신고서 disclosure link (a clickable row)
        # The page lists disclosures; we click the most recent 증권신고서 entry
        body_text = page.evaluate("() => document.body.innerText")

        # Look for a row containing 증권신고서 and click it
        rows = page.locator("text=증권신고서").all()
        if not rows:
            print(f"    No 증권신고서 found on disclosure list for {name}")
            continue

        # Click first match — opens detail page
        try:
            rows[0].click()
            page.wait_for_load_state("networkidle", timeout=15000)
        except Exception as e:
            print(f"    Click failed: {e}")
            continue

        time.sleep(0.5)

        # On the detail page, click 증권신고서 attachment button → opens new tab with PDF
        attach_btns = page.locator("button:has-text('증권신고서')").all()
        if not attach_btns:
            print(f"    No attachment button found for {name}")
            continue

        with context.expect_page() as new_page_info:
            try:
                attach_btns[0].click()
            except Exception as e:
                print(f"    Attachment click failed: {e}")
                continue

        new_tab = new_page_info.value
        new_tab.wait_for_load_state("domcontentloaded", timeout=15000)
        pdf_url = new_tab.url
        new_tab.close()

        if not pdf_url or "media.kr.kasa.exchange" not in pdf_url:
            print(f"    Unexpected PDF URL: {pdf_url}")
            continue

        print(f"    PDF URL captured: {pdf_url[:80]}...")
        pdf_urls[code] = pdf_url

    return pdf_urls


def download_pdfs(pdf_urls: dict[str, str]):
    for code, url in pdf_urls.items():
        pdf_path = PDF_DIR / f"{code}-증권신고서.pdf"
        if pdf_path.exists():
            print(f"  [{code}] already downloaded")
            continue
        print(f"  [{code}] downloading PDF ...")
        try:
            r = requests.get(url, timeout=60)
            r.raise_for_status()
            pdf_path.write_bytes(r.content)
            print(f"    Saved {len(r.content)//1024} KB → {pdf_path.name}")
        except Exception as e:
            print(f"    Download failed: {e}")


def main():
    with sync_playwright() as pw:
        browser = pw.chromium.launch(headless=True)
        context = browser.new_context()
        page = context.new_page()

        # Step 1: Extract building details from __NEXT_DATA__
        buildings = scrape_building_details(page)
        CACHE.write_text(json.dumps(buildings, ensure_ascii=False, indent=2))
        print(f"  Cached building details → {CACHE}")

        # Step 2: Visit each building's disclosure page, capture PDF URLs
        pdf_urls = scrape_disclosure_pdf_urls(page, context, buildings)
        print(f"\n  Captured {len(pdf_urls)} PDF URLs")

        # Step 3: Download PDFs
        print("\n→ Downloading PDFs ...")
        download_pdfs(pdf_urls)

        browser.close()

    print("\nDone.")
    print(f"Building details: {CACHE}")
    print(f"PDFs: {PDF_DIR}/")


if __name__ == "__main__":
    main()
