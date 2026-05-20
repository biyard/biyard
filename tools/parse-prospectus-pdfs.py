#!/usr/bin/env python3
"""
Parse STO 증권신고서 PDFs to extract investment-relevant fields.
- Lucentblock: text-based PDFs via pdfminer
- Kasa: image-based PDFs via OCR (pdf2image + tesseract)

Run from repo root:
  python3 tools/parse-prospectus-pdfs.py
"""

import json, re
from pathlib import Path

ROOT    = Path(__file__).parent.parent
PDF_DIR = ROOT / ".playwright-mcp"
OUT     = ROOT / "sto-mockup" / "prospectus-parsed.json"


def extract_text(pdf_path: Path) -> str:
    try:
        from pdfminer.high_level import extract_text as _ext
        return _ext(str(pdf_path))
    except Exception as e:
        print(f"  PDF parse error {pdf_path.name}: {e}")
        return ""


def extract_text_ocr(pdf_path: Path) -> list[str]:
    """Convert each page to image and run OCR. Returns list of page texts."""
    from pdf2image import convert_from_path
    import pytesseract
    pages = convert_from_path(str(pdf_path), dpi=200)
    return [pytesseract.image_to_string(p, lang='kor+eng') for p in pages]


def clean(text: str) -> str:
    return re.sub(r'\s+', ' ', text).strip()


def find_one(patterns: list[str], text: str) -> str | None:
    for pat in patterns:
        m = re.search(pat, text, re.DOTALL)
        if m:
            return clean(m.group(1))
    return None


def parse_property_table(text: str) -> dict:
    """
    pdfminer extracts tables with all labels in one block then all values.
    Pattern: "대지면적 \n연면적 \n건물규모 \n...\n내용 \n위치값 \n대상값 \n대지면적값 \n..."
    """
    result = {}

    # Find the label-then-value block starting with 대지면적/연면적 labels
    # followed by 내용 then the actual values
    m = re.search(
        r'(?:대지면적\s*\n+연면적|연면적\s*\n+대지면적)'
        r'.{0,300}?내용\s*\n+'
        r'(.+?)(?:나\.\s*부동산\s*현황|\Z)',
        text, re.DOTALL
    )
    if not m:
        return result

    values_block = m.group(1)

    # Extract land_area: first NN㎡ pattern (not floor area — floor area is second)
    areas = re.findall(r'([0-9,]+(?:\.[0-9]+)?)\s*㎡', values_block)
    if len(areas) >= 1:
        result["land_area"] = areas[0] + "㎡"
    if len(areas) >= 2:
        result["floor_area"] = areas[1] + "㎡"

    # floors: 지상/지하 description ("지하1층, 지상 4층" or "지상 1, 2층")
    floors_m = re.search(r'((?:지하[0-9]+층,?\s*)?지상\s*[0-9, ]+층[^\n]{0,20})', values_block)
    if floors_m:
        result["floors"] = clean(floors_m.group(1))

    # completion_date: year pattern
    date_m = re.search(r'([0-9]{4}년\s*[0-9]+월(?:\s*[0-9]+일)?[^\n]{0,40})', values_block)
    if date_m:
        result["completion_date"] = clean(date_m.group(1))

    return result


def parse_lease_table(text: str) -> dict:
    """
    Lease table patterns observed across PDFs:

    A (수원행궁): 임차인\n\n㈜어반플레이\n\n임대차목적물\n\n...\n임대기간\n\n보증금\n\n...\n\n2023년...까지
    B (안국):     임차인\n\n임대차 조건\n주식회사 다운타우너\n\n임대차목적물\n\n...\n임대기간\n\n...\n신탁등기...
    C (이태원):   임차인\n\n임대차목적물\n\n임대기간\n\n보증금\n\n임대료\n\n관리비\n\n임대차 조건\n㈜글로우서울\n\n...\n\n2022년...까지
    D (전주):     임차인\n\n임대차 조건\n\n㈜에이지엠티\n\n임대차목적물...
    """
    result = {}

    # --- Tenant ---
    tenant = find_one([
        # Pattern A: value directly after 임차인 label
        r'임차인\s*\n+\n*((?:주식회사|㈜)[^\n]{2,30})\s*\n',
        # Pattern C: labels block then "임대차 조건 \n<tenant>"
        r'임대차\s*조건\s*\n+\s*((?:주식회사|㈜)[^\n]{2,30})',
        # Pattern B/D: "임차인 \n\n임대차 조건 \n<tenant>"
        r'임차인\s*\n+임대차\s*조건\s*\n+\n*((?:주식회사|㈜)[^\n]{2,30})',
    ], text)
    if tenant:
        result["tenant"] = tenant

    # --- Lease term ---
    lease_term = find_one([
        # Pattern A: direct after 임대기간 label
        r'임대기간\s*\n+\n*([0-9]{4}년[^\n]{5,80})',
        # Pattern C (이태원): labels block → 임대차 조건 → tenant → 목적물 상세 → lease date
        # Search for date in values block that follows label group
        r'임대차\s*조건\s*\n.{10,600}?([0-9]{4}년\s*[0-9]+월\s*[0-9]+일부터[^\n]{5,60})',
        # 신탁등기 완료일 pattern (pattern B, 안국)
        r'(신탁\s*등기\s*완료일로부터\s*[0-9]+\s*개월[^\n]{0,30})',
        # 계약기간 with 신탁
        r'계약기간\s*\n+\n*(신탁[^\n]{5,40})',
    ], text)
    if lease_term:
        # Trim trailing incomplete parenthesis if any
        lt = lease_term
        if lt.count('(') > lt.count(')'):
            lt += ')'
        result["lease_term"] = lt

    return result


def parse_lucentblock(text: str) -> dict:
    data = {}

    addr_regions = r'(?:서울|경기|인천|부산|대구|광주|대전|울산|세종|강원|충북|충남|전북|전남|전라북도|전라남도|경북|경남|제주)'

    # Address — look inside 내용 block for the location line (first value in property table)
    raw_addr = find_one([
        r'신탁부동산은\s+(' + addr_regions + r'[^\n]{5,80})\s+(?:토지|소재)',
        r'소\s*재\s*지\s*[：:]\s*(' + addr_regions + r'[^\n]{5,80})',
        r'건물\s*소\s*재\s*지\s*[：:]\s*(' + addr_regions + r'[^\n]{5,80})',
        r'부동산\s*위치\s*\n+(' + addr_regions + r'[^\n]{5,80})',
        r'위치\s*\n+(' + addr_regions + r'[^\n]{5,80})',
        r'위치\s*\n\n(' + addr_regions + r'[^\n]{5,80})',
    ], text)
    if raw_addr:
        # Strip trailing particles (에, 에서, 소재, 토지...)
        data["address"] = re.sub(r'\s*(?:에서?|소재|토지|및|건물)\s*$', '', raw_addr).strip()

    # Extract from property table (대지면적/연면적/floors/completion_date)
    prop = parse_property_table(text)
    data.update(prop)

    # Completion date — also try standalone patterns if table parse missed it
    if "completion_date" not in data:
        data["completion_date"] = find_one([
            r'건물\s*준공일\s*\n+\n*([0-9]{4}년[^\n]{0,20})',
            r'준\s*공\s*일\s*\n+([0-9]{4}년[^\n]{0,20})',
            r'사용승인일\s*\n+([0-9]{4}년[^\n]{0,20})',
            r'(?:준공|리모델링)\s*([0-9]{4}년\s*[0-9]+월)',
        ], text)

    # Usage
    data["usage"] = find_one([
        r'주\s*용\s*도\s*\n+\n*(근린생활시설[^\n]{0,60}|업무시설[^\n]{0,40}|숙박시설[^\n]{0,40}|판매시설[^\n]{0,40})',
        r'주\s*용\s*도\s+(근린생활시설[^\n]{0,60}|업무시설[^\n]{0,40}|숙박시설[^\n]{0,40}|판매시설[^\n]{0,40})',
    ], text)

    # Appraisal values — "49.2억원 (㈜감정평가법인정명)"
    appraisals = re.findall(r'([0-9]+(?:\.[0-9]+)?)\s*억원\s*\(([^)]{3,30})\)', text)
    seen = set()
    appraisal_list = []
    for amount, appraiser in appraisals:
        appraiser_norm = re.sub(r'\s+', '', appraiser)
        amount_norm = float(amount)
        key = f"{amount_norm:.2f}_{appraiser_norm}"
        if key not in seen:
            seen.add(key)
            appraiser_display = re.sub(r'\s+', ' ', appraiser).strip()
            appraisal_list.append({"amount_억원": amount_norm, "appraiser": appraiser_display})
    if appraisal_list:
        data["appraisal_values"] = appraisal_list[:4]

    # Trustee
    data["trustee"] = find_one([
        r'신탁업자\s*[：:（(]\s*([^,，\n\)）]{5,50})',
        r'발\s*행\s*인\s*[：:]\s*([^,，\n]{5,60})',
        r'수탁자\s+([^\n,]{5,50}(?:신탁|자산)[^\n,]{0,20})',
    ], text)

    # Platform operator
    data["platform_operator"] = find_one([
        r'혁신금융사업자\s*[）\)]?\s*[：:，,]\s*(주식회사[^\n,]{3,30}|㈜[^\n,]{3,30})',
        r'플랫폼운영수익자\s*[：:]\s*(주식회사[^\n,]{3,30}|㈜[^\n,]{3,30})',
        r'(㈜루센트블록)',
    ], text) or "㈜루센트블록"

    # Total offering amount
    data["total_offering"] = find_one([
        r'모집\(매출\)\s*총액\s*[：:]\s*([0-9,]+원)',
        r'총\s*모\s*집\s*금\s*액\s*[：:]\s*([0-9,]+원)',
    ], text)

    # Total units (좌수) — handle spaces in numbers like "1,360 ,000 SOU"
    units_raw = find_one([
        r'수량\s+([0-9,\s]+)\s*좌',
        r'수량\s+([0-9,\s]+)\s*SOU',
        r'수량\s*[：:]\s*([0-9,\s]+)\s*(?:SOU|좌)',
        r'(?:수량|발행량)\s*([0-9,\s]+)\s*(?:SOU|좌|구)',
    ], text)
    if units_raw:
        data["total_units"] = re.sub(r'\s', '', units_raw)  # remove spaces within number

    # Unit price
    data["unit_price"] = find_one([
        r'1좌당\s*(?:공모)?가액\s*[：:]\s*금?\s*([0-9,]+원)',
        r'공모가액\s*(?:금\s*)?([0-9,]+원)',
    ], text)

    # Tenant and lease term from lease table
    lease = parse_lease_table(text)
    data.update(lease)

    # Expected yield
    data["expected_yield"] = find_one([
        r'예상\s*운용수익률\s*\(연환산\)[^\d]*(\d+\.\d+%)',
        r'연간\s*예상\s*수익률[^\d]*(\d+\.\d+%)',
    ], text)

    # Dividend frequency (months between payments)
    data["dividend_frequency"] = find_one([
        r'매\s*(1|2|3|6|12)개월\s*(?:마다|당|에\s*1회)',
        r'배당\s*주기\s*[：:]\s*([^\n]{3,30})',
    ], text)

    # Operation period
    data["operation_period"] = find_one([
        r'운용\s*기간\s*[：:]\s*([^\n]{5,40})',
        r'신탁\s*기간\s*[：:]\s*([^\n]{5,40})',
    ], text)

    # Upfront fee
    data["upfront_fee"] = find_one([
        r'총수수료\s+공모금액의\s+([0-9\.]+%)',
        r'선취수수료\s+.*?공모금액의\s+([0-9\.]+%)',
    ], text)

    return {k: v for k, v in data.items() if v}


def _parse_appraisal_from_table(pages: list[str]) -> list[dict]:
    """
    감정평가표 페이지에서 금액과 기관명 추출.
    페이지 헤더: "[감정평가표 N: 기관명]"
    금액: "일십억칠천..." or "\1,071,695,200.-" 형태
    """
    results = []
    seen = set()
    for page in pages:
        # 헤더에서 기관명 추출: "[감정평가표 1: 감정평가법인가치앤같이]"
        hdr_m = re.search(r'감정평가표\s*[0-9]+\s*[：:]\s*([^\]）\n]{3,40})', page)
        if not hdr_m:
            continue
        appraiser_raw = clean(hdr_m.group(1).rstrip(']）)'))

        # 금액: 한글 금액 "일십억..." or 숫자 "\1,063,295,200.-"
        amount_won = None
        # 숫자 형식: \1,063,295,200.- or (￦1,071,695,200,-)
        num_m = re.search(r'[\\￦₩\\(（]\s*1\s*[,.]?\s*([0-9]{3})[,.]([0-9]{3})[,.]([0-9]{3})', page)
        if num_m:
            amount_won = int(f"1{num_m.group(1)}{num_m.group(2)}{num_m.group(3)}")
        if amount_won is None:
            # 합계 행에서 숫자 찾기: "\ 1,071,695,200 ,-"
            sum_m = re.search(r'힙\s*om.*?([0-9]{1,2},[0-9]{3},[0-9]{3},[0-9]{3})', page, re.DOTALL)
            if sum_m:
                amount_won = int(sum_m.group(1).replace(',', ''))

        if amount_won and amount_won > 0:
            amount_억 = round(amount_won / 100_000_000, 4)
            key = f"{amount_억:.2f}_{re.sub(r'\\s', '', appraiser_raw)}"
            if key not in seen:
                seen.add(key)
                results.append({"amount_억원": amount_억, "appraiser": appraiser_raw})
    return results


def parse_kasa_ocr(pages: list[str]) -> dict:
    """
    Parse kasa 증권신고서 (image-based PDF, OCR text per page).
    Key pages by structure:
    - 개요 page (~p12): 모집총액, 수량, 액면가
    - 신탁회사 page (~p21): 수탁자
    - 부동산 개요 page (~p30): 위치, 대지면적, 연면적, 감정평가 요약
    - 층별 현황 page (~p32): 건물규모
    - 임대현황 page (~p33): 임차인, 임대기간
    - 감정평가표 pages (~p34-35): 감정평가 금액+기관명
    """
    data = {}
    full = "\n".join(pages)

    # --- 모집총액 / 수량 / 1좌당 가액 ---
    # 표 행: "194,000 | 5,000  5,000 | 970,000,000"
    offering_m = re.search(
        r'([0-9][0-9,]+)\s+[|｜]?\s*([0-9,]{4,})\s+([0-9,]{4,})\s+[|｜]?\s*([0-9,]{7,})',
        full
    )
    if offering_m:
        units_str = offering_m.group(1).replace(',', '').replace(' ', '')
        face_str  = offering_m.group(2).replace(',', '').replace(' ', '')
        total_str = offering_m.group(4).replace(',', '').replace(' ', '')
        if units_str.isdigit() and face_str.isdigit() and total_str.isdigit():
            data["total_units"] = f"{int(units_str):,}"
            data["unit_price"]  = f"{int(face_str):,}원"
            total_억 = int(total_str) / 100_000_000
            data["total_offering"] = f"{total_억:.0f}억원"

    # --- 수탁자: "신탁회사는 XXX으로" 문장에서 추출 ---
    trustee_m = re.search(r'신탁회사는\s+([^\s,으]{3,20}(?:신탁|자산신탁))\s*(?:으로|이)', full)
    if trustee_m:
        data["trustee"] = clean(trustee_m.group(1))

    # --- 부동산 개요 섹션 ---
    for page in pages:
        if '부동산 개요' not in page and '부동산개요' not in page:
            continue

        # 위치 / 주소
        addr_m = re.search(
            r'위\s*치\s+([^\n]{5,60}(?:구|시|군|동|읍|면|리)[^\n]{0,30})',
            page
        )
        if addr_m and 'address' not in data:
            addr = clean(addr_m.group(1))
            addr = re.sub(r'\s*(소재|토지|건물|및|부속).*$', '', addr).strip()
            # OCR 표 테두리 "| " prefix 제거
            addr = re.sub(r'^[|｜\s]+', '', addr).strip()
            if len(addr) >= 8:
                data["address"] = addr

        # 대지면적: OCR에서 ㎡ 기호 누락 + 소수점 누락 발생
        # "데지면적 420" → 실제 42.0㎡, "대지면적 42.0 nf" → 42.0㎡
        # 감정평가표 페이지에서 토지면적 "42.00" 더 신뢰도 높음 — 우선순위 낮게 설정
        land_m = re.search(
            r'(?:데|대)지\s*면\s*적\s+([0-9]+(?:\.[0-9]+)?)\s*(?:nf|㎡|m2)?',
            page
        )
        if land_m and 'land_area' not in data:
            data["_land_area_draft"] = land_m.group(1)

        # 연면적: "nf" / "마" = ㎡ OCR 오인식
        floor_m = re.search(
            r'연\s*면\s*적\s+[\[|]?\s*([0-9]+(?:\.[0-9]+)?)\s*(?:nf|㎡|m2|마)',
            page
        )
        if floor_m and 'floor_area' not in data:
            data["floor_area"] = floor_m.group(1) + "㎡"

        # 감정평가 — 부동산 개요 페이지에서 직접: "10.3억 원 (기관명)" 형태
        # OCR에서 "어" "역" 등으로 오인식되지만 숫자 패턴은 비교적 안정적
        if 'appraisal_values' not in data:
            appr_lines = re.findall(
                r'[ㆍ·\-]\s*([0-9]+(?:\.[0-9]+)?)\s*(?:억|역|어)\s*원\s*[（(（]([^)\n）]{3,40})',
                page
            )
            if appr_lines:
                seen_a = set()
                lst_a = []
                for amount_s, appraiser_raw in appr_lines:
                    amount = float(amount_s)
                    appraiser = re.sub(r'^.{1,3}?(?=감정평가|주식회사|㈜|\(주\))', '', clean(appraiser_raw)).strip()
                    key = f"{amount:.2f}_{re.sub(r'\\s','',appraiser)}"
                    if key not in seen_a:
                        seen_a.add(key)
                        lst_a.append({"amount_억원": amount, "appraiser": appraiser})
                if lst_a:
                    data["appraisal_values"] = lst_a

    # --- 층별 현황 페이지에서 건물규모 ---
    # 건물규모 테이블이 아닌 층별 현황 합계로부터 층수 파악
    for page in pages:
        if '층별' not in page and '건물규모' not in page:
            continue
        # 지상N층 형태 찾기
        floors_m = re.search(
            r'((?:지하\s*[0-9]+층[,\s]*)?지상\s*[0-9]+층)',
            page
        )
        if floors_m and 'floors' not in data:
            data["floors"] = clean(floors_m.group(1))
            break

    # --- 감정평가 금액+기관명 (감정평가표 페이지) ---
    appraisal_list = _parse_appraisal_from_table(pages)
    if appraisal_list:
        cleaned = []
        for entry in appraisal_list:
            appraiser = entry["appraiser"]
            appraiser = re.sub(r'^.{1,3}?(?=감정평가|주식회사|㈜|\(주\))', '', appraiser).strip()
            cleaned.append({"amount_억원": entry["amount_억원"], "appraiser": appraiser})
        data["appraisal_values"] = cleaned

    # 감정평가표에서 토지면적 추출 — appraisal 추출 여부와 무관하게 항상 시도
    for page in pages:
        has_appr = '감정평가표' in page or ('감정평가' in page and ('단가' in page or '감정평가사' in page))
        has_land = bool(re.search(r'(?:토\s*치|노\s*지|토\s*지)\s+[0-9]+\.[0-9]', page))
        if not (has_appr or has_land):
            continue
        land_m = re.search(
            r'(?:토\s*치|노\s*지|토\s*지)\s+[0-9]+\s+(?:토\s*치|노\s*지|토\s*지)\s+([0-9]+\.[0-9]+)',
            page
        )
        if not land_m:
            land_m = re.search(
                r'(?:토\s*치|노\s*지|토\s*지)\s+([0-9]+\.[0-9]{1,2})(?:\s|$)',
                page
            )
        if land_m:
            data["land_area"] = land_m.group(1) + "㎡"
            data.pop("_land_area_draft", None)
            break

    # 감정평가표에서 못 잡은 경우 draft 값 사용
    if 'land_area' not in data and '_land_area_draft' in data:
        val = data["_land_area_draft"]
        # OCR 소수점 누락 보정: "420" → "42.0", "6610" → "66.10" 등
        # 감정평가표의 토지면적과 일치하도록: 정수 3-4자리이면서 ×10 오인식 가능성
        # 여기서는 그냥 숫자 그대로 사용 (더 안전)
        data["land_area"] = val + "㎡"
    data.pop("_land_area_draft", None)

    # --- 임대현황 페이지: 임차인 + 임대기간 ---
    for page in pages:
        if '임대현황' not in page and ('임차인' not in page or '임대기간' not in page):
            continue

        # 임차인: "임차인  김유찬" 또는 "임차인  주식회사XXX"
        if 'tenant' not in data:
            # 임대차 계약 주요 조건 표에서 "임차인 | 값" 형태
            # 우선: "임차인  [넓은공백]  회사명" — 테이블 값 행 (공백 4칸 이상)
            tenant_m = re.search(
                r'^임\s*차\s*인\s{4,}(?:\|\s*)?((?:주식회사|㈜|\(주\))[가-힣]+|[가-힣]{2,20})',
                page, re.MULTILINE
            )
            if not tenant_m:
                # 줄바꿈 이후
                tenant_m = re.search(
                    r'임\s*차\s*인\s*\n+(?:\|\s*)?((?:주식회사|㈜|\(주\))[가-힣]+|[가-힣]{2,20})',
                    page
                )
            if not tenant_m:
                # fallback: 공백 1칸 이상
                tenant_m = re.search(
                    r'임\s*차\s*인\s+((?:주식회사|㈜|\(주\))[가-힣]+|[가-힣]{2,20})',
                    page
                )
            if tenant_m:
                candidate = clean(tenant_m.group(1))
                _bad = {'임대면', '임대차', '임대기', '계약기', '보증금', '실비정산',
                        '면적', '임대면적', '추가', '관리비', '부담', '신용도', '지급능력',
                        '유치', '모집', '미확정', '해당없음'}
                if candidate not in _bad and len(candidate) >= 2:
                    data["tenant"] = candidate

        # 임대기간: YYYY.MM.DD-\nYYYY.MM.DD 또는 한 줄 또는 "계약기간" 행
        if 'lease_term' not in data:
            # "2024.06.05-\n2026.06.04" 또는 "2024.06.05- 2026.06.04"
            lt_m = re.search(
                r'([0-9]{4}[.\-][0-9]{2}[.\-][0-9]{2})\s*[-~\n]+\s*([0-9]{4}[.\-][0-9]{2}[.\-][0-9]{2})',
                page
            )
            if lt_m:
                data["lease_term"] = f"{lt_m.group(1)} ~ {lt_m.group(2)}"
            else:
                # "계약기간 2024.06.05, ~ 2026.06.04" 형태
                lt_m2 = re.search(
                    r'(?:계약기간|임대기간|aa)\s+([0-9]{4}[.\-][0-9]{2}[.\-][0-9]{2})[,\s]*[-~]+\s*([0-9]{4}[.\-][0-9]{2}[.\-][0-9]{2})',
                    page
                )
                if lt_m2:
                    data["lease_term"] = f"{lt_m2.group(1)} ~ {lt_m2.group(2)}"

    # --- 배당 주기 ---
    div_m = re.search(r'매\s*([1-9][0-9]?)\s*개월\s*(?:마다|당|에\s*1회)', full)
    if div_m:
        n = int(div_m.group(1))
        data["dividend_frequency"] = {1: "월배당", 3: "분기배당", 6: "반기배당", 12: "연배당"}.get(n, f"매{n}개월배당")

    # --- OCR 오타 후처리 ---
    if "address" in data:
        data["address"] = re.sub(r'서울특[변번]서', '서울특별시', data["address"])
    if "tenant" in data:
        # "XXX주" → "XXX" (㈜ OCR 오인식이 뒤에 붙는 경우)
        data["tenant"] = re.sub(r'주$', '', data["tenant"]).strip()
        if len(data["tenant"]) < 2:
            del data["tenant"]

    return {k: v for k, v in data.items() if v}


def main():
    results = []

    lucentblock_pdfs = sorted(PDF_DIR.glob("증권신고서-*.pdf"))
    lucentblock_pdfs = [p for p in lucentblock_pdfs if "kasa" not in p.name]

    kasa_pdfs = sorted(PDF_DIR.glob("증권신고서-kasa-*.pdf"))

    print(f"루센트블록 텍스트 PDF: {len(lucentblock_pdfs)}개")
    print(f"카사 이미지 PDF: {len(kasa_pdfs)}개 (OCR)")

    for pdf in lucentblock_pdfs:
        name_raw = pdf.stem.replace("증권신고서-", "").replace("-", " ")
        print(f"\n  파싱: {pdf.name}")
        text = extract_text(pdf)
        clean_text = text.replace('\x0c', '\n')
        if len(clean_text.replace('\n', '').strip()) < 100:
            print(f"    → 이미지 기반 PDF (텍스트 없음)")
            results.append({"name": name_raw, "source": "lucentblock", "parse_status": "image_only"})
            continue

        parsed = parse_lucentblock(clean_text)
        parsed["name"] = name_raw
        parsed["source"] = "lucentblock"
        parsed["parse_status"] = "ok"
        results.append(parsed)
        print(f"    → 추출 필드: {list(parsed.keys())}")

    for pdf in kasa_pdfs:
        name_raw = pdf.stem.replace("증권신고서-kasa-", "")
        print(f"\n  OCR 파싱: {pdf.name}")
        try:
            pages = extract_text_ocr(pdf)
            parsed = parse_kasa_ocr(pages)
            parsed["name"] = name_raw
            parsed["source"] = "kasa"
            parsed["parse_status"] = "ok"
            results.append(parsed)
            print(f"    → 추출 필드: {[k for k in parsed if k not in ('name','source','parse_status')]}")
        except Exception as e:
            print(f"    → OCR 실패: {e}")
            results.append({"name": name_raw, "source": "kasa", "parse_status": "ocr_error", "error": str(e)})

    OUT.write_text(json.dumps(results, ensure_ascii=False, indent=2))
    print(f"\n결과 저장: {OUT}")


if __name__ == "__main__":
    main()
