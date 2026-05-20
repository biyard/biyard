#!/usr/bin/env python3
"""
sou.place (루센트블록) disclosure PDF scraper.

Steps:
  1. Visit sou.place → click 공시자료 button → panel opens
  2. For each building tab in the panel, click it → see disclosure list
  3. For each disclosure that has PDF attachments, click the attachment item →
     intercept the network response (the blob is fetched from a CDN) → save locally

Run:
  cd /path/to/biyard
  pip install playwright requests
  playwright install chromium
  python3 tools/scrape-sou-details.py

Re-running is safe: skips already-downloaded PDFs.
"""

import json, time, re, os
from pathlib import Path
import requests
from playwright.sync_api import sync_playwright, Route

ROOT    = Path(__file__).parent.parent
MOCK    = ROOT / "sto-mockup"
PDF_DIR = MOCK / "sou-pdfs"
CACHE   = MOCK / "sou-building-details.json"

PDF_DIR.mkdir(parents=True, exist_ok=True)

# Keywords that identify a 증권신고서 or 투자설명서 (most useful for building details)
PRIORITY_DOCS = ["증권신고서", "투자설명서", "공모청약안내문"]


def _safe_filename(s: str) -> str:
    return re.sub(r'[\\/:*?"<>|]', '_', s)


def scrape_sou(page, context):
    captured: dict[str, bytes] = {}  # filename → pdf bytes

    # Intercept all PDF responses from sou CDN
    def handle_route(route: Route):
        response = route.fetch()
        url = route.request.url
        if response.status == 200 and ".pdf" in url.lower():
            fname = url.split("/")[-1].split("?")[0]
            captured[fname] = response.body()
            print(f"    Intercepted: {fname} ({len(response.body())//1024} KB)")
        route.fulfill(response=response)

    context.route("**/*.pdf*", handle_route)

    print("→ Loading sou.place ...")
    page.goto("https://sou.place", wait_until="networkidle", timeout=30000)
    time.sleep(1)

    # Click 공시자료 button
    page.locator("button:has-text('공시자료')").first.click()
    time.sleep(1.5)

    # Get building tab names
    body = page.evaluate("() => document.body.innerText")

    # Extract building list from the panel — they appear as clickable items
    # The panel structure: building names are listed as buttons/divs
    building_names_raw = page.evaluate("""() => {
        // Find the sidebar list of buildings in the 공시자료 panel
        const allText = document.body.innerText;
        // After '공시자료' header, building names appear before the disclosure list
        const lines = allText.split('\\n').map(l => l.trim()).filter(Boolean);
        const startIdx = lines.indexOf('공시자료');
        if (startIdx === -1) return [];
        // Collect lines that look like building names (until we hit a disclosure title)
        const names = [];
        for (let i = startIdx + 1; i < lines.length; i++) {
            const line = lines[i];
            if (line === '채용') break;
            // Building names don't contain dates or "배당" etc.
            if (!line.includes('기 배당') && !line.includes('20') && line.length < 30) {
                names.push(line);
            } else {
                break;
            }
        }
        return names;
    }""")
    print(f"  Found buildings: {building_names_raw}")

    results = []

    for bname in building_names_raw:
        print(f"\n  [{bname}] clicking tab ...")
        try:
            page.locator(f"text={bname}").first.click()
            time.sleep(1.5)
        except Exception as e:
            print(f"    Tab click failed: {e}")
            continue

        # Get list of disclosures for this building
        disclosures_text = page.evaluate("""() => {
            return document.body.innerText;
        }""")

        # Find PDF attachment items visible on screen
        pdf_items = page.locator("span").filter(has_text=re.compile(r"\.pdf$")).all()
        print(f"    Found {len(pdf_items)} PDF attachment items")

        # Click priority docs first (증권신고서, 투자설명서), then others
        clicked_files = set()
        for pdf_span in pdf_items:
            fname = pdf_span.text_content().strip() if pdf_span.text_content() else ""
            if not fname.endswith(".pdf"):
                continue

            # Only download priority documents to avoid flooding
            is_priority = any(kw in fname for kw in PRIORITY_DOCS)
            if not is_priority:
                continue

            if fname in clicked_files:
                continue
            clicked_files.add(fname)

            safe = _safe_filename(bname)
            dest = PDF_DIR / f"{safe}-{_safe_filename(fname)}"
            if dest.exists():
                print(f"    Already have: {dest.name}")
                continue

            print(f"    Clicking: {fname}")
            try:
                pdf_span.click()
                time.sleep(2)  # wait for blob fetch + intercept
            except Exception as e:
                print(f"    Click failed: {e}")

        # Save any intercepted PDFs for this building
        for fname, data in list(captured.items()):
            safe = _safe_filename(bname)
            dest = PDF_DIR / f"{safe}-{_safe_filename(fname)}"
            if not dest.exists():
                dest.write_bytes(data)
                print(f"    Saved: {dest.name} ({len(data)//1024} KB)")
        captured.clear()

        results.append({"building": bname, "pdfs": list(clicked_files)})

    return results


def main():
    with sync_playwright() as pw:
        browser = pw.chromium.launch(headless=True)
        context = browser.new_context()
        page = context.new_page()

        results = scrape_sou(page, context)

        CACHE.write_text(json.dumps(results, ensure_ascii=False, indent=2))
        print(f"\n  Cached scrape results → {CACHE}")

        browser.close()

    print("\nDone.")
    print(f"PDFs: {PDF_DIR}/")
    print(f"Results: {CACHE}")


if __name__ == "__main__":
    main()
