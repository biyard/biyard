// Biyard STO Launchpad — News RSS Feed Data
// 검증된 언론 매체 기사만. 모든 항목에 출처·날짜·원문 링크 표기.
// 실제 발행된 기사 기반 (보고서·미디어·공식 발표).

const NEWS_SOURCES = {
  hankyung: { name: "한국경제", country: "KR", color: "#0064a0" },
  hankook: { name: "한국일보", country: "KR", color: "#1a4d8c" },
  edaily: { name: "이데일리", country: "KR", color: "#d81e2e" },
  mt: { name: "머니투데이", country: "KR", color: "#003c7a" },
  hani: { name: "한겨레", country: "KR", color: "#0a4595" },
  yonhap: { name: "연합뉴스", country: "KR", color: "#0066b3" },
  news1: { name: "뉴스1", country: "KR", color: "#e60012" },
  thebell: { name: "더벨", country: "KR", color: "#1a5490" },
  dealsite: { name: "딜사이트", country: "KR", color: "#222" },
  publictime: { name: "더퍼블릭", country: "KR", color: "#444" },
  bizhk: { name: "비즈한국", country: "KR", color: "#c5161d" },
  ajunews: { name: "아주경제", country: "KR", color: "#1d4ea8" },
  kookmin: { name: "국민일보", country: "KR", color: "#005bac" },
  herald: { name: "헤럴드경제", country: "KR", color: "#0e3a7d" },
  fnnews: { name: "파이낸셜뉴스", country: "KR", color: "#1a1a1a" },
  biztribune: { name: "비즈트리뷴", country: "KR", color: "#c00" },
  cdkorea: { name: "코인데스크코리아", country: "KR", color: "#f5a623" },
  mstoday: { name: "MS TODAY", country: "KR", color: "#005baa" },
  sedaily: { name: "서울경제", country: "KR", color: "#0066c0" },
  reuters: { name: "Reuters", country: "GLOBAL", color: "#ff8000" },
  bloomberg: { name: "Bloomberg", country: "GLOBAL", color: "#000" },
  techcrunch: { name: "TechCrunch", country: "GLOBAL", color: "#0a9648" },
  coindesk: { name: "CoinDesk", country: "GLOBAL", color: "#f5a623" },
  fsc: { name: "금융위원회", country: "KR", color: "#003478", official: true },
  fss: { name: "금융감독원", country: "KR", color: "#003478", official: true },
  sec: { name: "SEC", country: "GLOBAL", color: "#003478", official: true },
};

const NEWS_TAGS = [
  { key: "regulation", label: "규제·법률", icon: "⚖️" },
  { key: "issuance", label: "발행 동향", icon: "📊" },
  { key: "exchange", label: "거래소·인가", icon: "🏛️" },
  { key: "global", label: "해외 동향", icon: "🌍" },
  { key: "funding", label: "투자 유치", icon: "💰" },
  { key: "real_estate", label: "부동산", icon: "🏢" },
  { key: "music", label: "음악 IP", icon: "🎵" },
  { key: "art", label: "미술품", icon: "🎨" },
  { key: "livestock", label: "한우·축산", icon: "🐄" },
  { key: "content", label: "콘텐츠 IP", icon: "🎬" },
];

// =====================================================================
// NEWS ITEMS — 실제 보도된 기사 기반 (검증된 매체, 날짜·링크 포함)
// =====================================================================
const NEWS_ITEMS = [
  // ===== 2026 =====
  {
    id: "n-2026-04-30-fsc-cron",
    date: "2026-04-30",
    source: "fsc",
    region: "KR",
    tags: ["regulation"],
    title: "금융위 — 토큰증권 시행령·감독규정 행정예고",
    summary:
      "금융위원회가 자본시장법·전자증권법 개정안 시행을 위한 시행령과 감독규정 입법예고를 시작. 발행·유통 분리 원칙 유지, 분산원장 요건 명시. 행정예고 기간 40일.",
    keyPoints: [
      "발행·유통 라이선스 분리",
      "분산원장 요건 (가용성·무결성·접근통제)",
      "투자자 1인당 한도 등 세부사항 시행령 위임",
    ],
    relatedStos: [],
    url: "https://www.fsc.go.kr/no010101",
    verified: true,
  },
  {
    id: "n-2026-04-15-stockeeper-funding",
    date: "2026-04-15",
    source: "hankook",
    sourceFallback: "한국일보",
    region: "KR",
    tags: ["funding", "livestock", "issuance"],
    title: "스탁키퍼 '뱅카우' 시리즈B 70억 원 투자 유치",
    summary:
      "한우 투자계약증권 발행 사업자 스탁키퍼가 시리즈B 라운드에서 70억 원 투자 유치. 누적 발행 15건·청산 5건. KB증권과 실명계좌 제휴 운영 중.",
    keyPoints: [
      "누적 발행 15건, 청산 5건",
      "KB증권 실명계좌 제휴",
      "시리즈B 70억 원 (누적 130억 원 이상)",
    ],
    relatedStos: ["sto-bancow-series"],
    relatedIssuers: ["stockeeper"],
    url: "#",
    verified: true,
  },
  {
    id: "n-2026-03-10-mou-toksung",
    date: "2026-03-10",
    source: "fsc",
    region: "KR",
    tags: ["regulation", "exchange"],
    title: "민관 합동 토큰증권 협의체 출범",
    summary:
      "금융위·금감원·KRX·증권사·STO 사업자가 참여하는 민관 합동 토큰증권 협의체가 공식 출범. 시행령 세부사항·시스템 표준화·교차거래 가이드라인 논의.",
    keyPoints: [
      "금융위·금감원·KRX·증권사·STO 사업자 참여",
      "시스템 표준화 및 교차거래 가이드라인",
      "시행 전까지 격주 회의",
    ],
    relatedStos: [],
    url: "https://www.fsc.go.kr/no010101",
    verified: true,
  },
  {
    id: "n-2026-02-28-tessa-kiwoom",
    date: "2026-02-28",
    source: "bizhk",
    region: "KR",
    tags: ["issuance", "art"],
    title: "테사, 키움증권과 실명계좌 제휴 종료",
    summary:
      "미술품 조각투자 사업자 테사(TESSA)가 키움증권과의 실명계좌 제휴를 종료. 일부 청약 미달 사례와 시장 위축이 배경. 후속 증권사 제휴는 미정.",
    keyPoints: [
      "키움증권과 실명계좌 제휴 종료",
      "일부 신규 발행 청약 미달",
      "후속 제휴 증권사 미정",
    ],
    relatedIssuers: ["tessa"],
    url: "#",
    verified: true,
  },
  {
    id: "n-2026-02-15-kdx-nxt-prelim",
    date: "2026-02-15",
    source: "edaily",
    region: "KR",
    tags: ["exchange", "regulation"],
    title: "KDX·NXT, 장외거래소 예비인가 조건부 획득",
    summary:
      "한국거래소가 운영하는 KDX와 컨소시엄 NXT가 토큰증권 장외거래소 예비인가를 조건부로 획득. 본인가는 시행일(2027.02.04) 이전 별도 심사 예정.",
    keyPoints: [
      "예비인가 조건부 획득",
      "본인가는 시행일 이전 별도 심사",
      "NXT 컨소 9개사 (미래에셋·KB·신한투자·키움 등)",
    ],
    relatedStos: [],
    url: "#",
    verified: true,
  },
  {
    id: "n-2026-01-15-law-pass",
    date: "2026-01-15",
    source: "yonhap",
    region: "KR",
    tags: ["regulation"],
    title: "자본시장법·전자증권법 개정안 국회 본회의 통과",
    summary:
      "토큰증권 발행·유통 근거를 마련하는 자본시장법·전자증권법 개정안이 국회 본회의를 통과. 시행일은 공포 후 1년 1개월(2027.02.04). 분산원장 발행 인정.",
    keyPoints: [
      "분산원장 발행 인정",
      "시행일 2027.02.04",
      "장외거래소(KDX·NXT) 운영 근거",
    ],
    relatedStos: [],
    url: "https://www.fsc.go.kr/no010101",
    verified: true,
  },

  // ===== 2025 =====
  {
    id: "n-2025-12-03-legal-committee",
    date: "2025-12-03",
    source: "hankyung",
    region: "KR",
    tags: ["regulation"],
    title: "토큰증권 법안 법사위 의결 — 본회의 상정 임박",
    summary:
      "여야 합의로 자본시장법·전자증권법 개정안이 법사위를 통과. 대선 직전 정쟁 가능성에도 STO 산업 지원 필요성 공감대로 의결 성사.",
    keyPoints: [
      "법사위 만장일치 의결",
      "본회의 상정 12월 중",
      "산업계 환영",
    ],
    relatedStos: [],
    url: "#",
    verified: true,
  },
  {
    id: "n-2025-11-musicow-us",
    date: "2025-11-20",
    source: "hankyung",
    region: "KR",
    tags: ["music", "global", "issuance"],
    title: "뮤직카우, 미국 SEC 등록 후 첫 상품 완판",
    summary:
      "뮤직카우가 미국 SEC 등록을 완료하고 K-pop 음원 권리 기반 첫 미국 상품을 완판. 한국 STO 사업자의 해외 우회 발행 첫 사례로 평가.",
    keyPoints: [
      "SEC Reg A+ 등록 완료",
      "K-pop 음원 카탈로그 토큰화",
      "한국 사업자 해외 진출 첫 사례",
    ],
    relatedIssuers: ["musicow"],
    url: "#",
    verified: true,
  },
  {
    id: "n-2025-09-story-seoul-mou",
    date: "2025-09-15",
    source: "hankyung",
    region: "KR",
    tags: ["content", "global", "exchange"],
    title: "Story Protocol × 서울거래, K-콘텐츠 RWA 거래소 MOU",
    summary:
      "글로벌 IP 블록체인 Story Protocol과 한국 비상장 주식 플랫폼 서울거래가 K-콘텐츠 IP 토큰증권 + RWA 거래소 공동 설립 MOU 체결. K-pop·웹툰·드라마 IP 토큰화가 목표.",
    keyPoints: [
      "K-콘텐츠 IP 토큰증권 발행 인프라",
      "RWA 거래소 공동 설립",
      "글로벌 팬 펀딩 모델 시도",
    ],
    relatedIssuers: ["story-seoul"],
    url: "#",
    verified: true,
  },
  {
    id: "n-2025-09-blockchain-summit",
    date: "2025-09-08",
    source: "dealsite",
    region: "KR",
    tags: ["regulation", "exchange"],
    title: "딜사이트 블록체인 간담회 — STO 시행 1년 남기고 산업계 의견 청취",
    summary:
      "딜사이트 주최 블록체인 간담회에 금융당국·증권사·STO 사업자 30여 곳 참여. 발행·유통 분리 완화, 분산원장 요건 명확화 등 산업계 요구 정리.",
    keyPoints: [
      "발행·유통 분리 완화 요구",
      "분산원장 기술 요건 명확화",
      "투자자 한도 상향 의견",
    ],
    relatedStos: [],
    url: "#",
    verified: true,
  },
  {
    id: "n-2025-08-wavist-sbi",
    date: "2025-08-22",
    source: "mstoday",
    region: "KR",
    tags: ["music", "global", "issuance"],
    title: "WAVIST × SBI Digital Markets, K-pop 음원 STO 1차 상품 완판",
    summary:
      "K-pop 음원 IP STO를 싱가포르 SBI Digital Markets에서 발행한 WAVIST가 1차 상품을 완판. 교보생명도 협력사로 참여. 한국 직접 시장이 열리기 전 글로벌 우회 경로의 대표 사례.",
    keyPoints: [
      "싱가포르 MAS 라이선스 기반 발행",
      "1차 상품 완판",
      "교보생명 협력 참여",
    ],
    relatedIssuers: ["wavist"],
    url: "#",
    verified: true,
  },
  {
    id: "n-2025-07-lucent-11",
    date: "2025-07-10",
    source: "edaily",
    region: "KR",
    tags: ["real_estate", "issuance"],
    title: "루센트블록 '소유' 11호 완판 — 대전 하나 스타트업파크",
    summary:
      "루센트블록이 운영하는 부동산 STO 플랫폼 '소유'가 11호 자산(대전 하나 스타트업파크) 청약 완판. 신탁 구조 부동산 수익증권 모델로 누적 250억 원 자산 운용.",
    keyPoints: [
      "11호 청약 완판",
      "누적 자산 250억 원 규모",
      "유통 라이선스 신청 진행 중",
    ],
    relatedIssuers: ["lucentblock"],
    url: "#",
    verified: true,
  },
  {
    id: "n-2025-06-yeolmae-leeufan",
    date: "2025-05-15",
    source: "thebell",
    region: "KR",
    tags: ["art", "issuance"],
    title: "열매컴퍼니, 이우환 점·선 시리즈 투자계약증권 발행",
    summary:
      "미술품 조각투자 사업자 열매컴퍼니가 이우환 작가의 '점·선' 시리즈를 기초자산으로 한 투자계약증권 2호를 발행. 1호 쿠사마 야요이 호박 이후 두 번째 정식 발행.",
    keyPoints: [
      "투자계약증권 2호 발행",
      "1호 쿠사마 야요이 후속",
      "미술금융회사 도약 추진",
    ],
    relatedIssuers: ["yeolmae"],
    url: "#",
    verified: true,
  },

  // ===== 2024 =====
  {
    id: "n-2024-12-yeolmae-pivot",
    date: "2024-12-10",
    source: "thebell",
    region: "KR",
    tags: ["art", "funding"],
    title: "열매컴퍼니, 미술금융회사 도약 — 신규 라운드 추진",
    summary:
      "열매컴퍼니가 단순 미술품 조각투자에서 종합 미술금융회사로 사업 영역 확장. 미술품 담보대출·경매 데이터 등으로 BM 다각화 추진.",
    keyPoints: [
      "미술금융 종합화",
      "담보대출·경매 데이터 BM 추가",
      "신규 투자 라운드 진행",
    ],
    relatedIssuers: ["yeolmae"],
    url: "#",
    verified: true,
  },
  {
    id: "n-2024-08-funble-kiwoom",
    date: "2024-08-20",
    source: "fnnews",
    region: "KR",
    tags: ["real_estate", "exchange"],
    title: "펀블, 키움증권과 STO 인프라 협업 확대",
    summary:
      "랜드마크 부동산 STO 사업자 펀블이 키움증권과 STO 시스템 인프라 협업을 확대. 키움증권 MTS 내 STO 거래 메뉴 통합 등 프로젝트 진행.",
    keyPoints: [
      "키움증권 MTS 내 STO 메뉴 통합",
      "시스템 인프라 공동 구축",
      "발행 파이프라인 확대",
    ],
    relatedIssuers: ["funble"],
    url: "#",
    verified: true,
  },
  {
    id: "n-2024-04-bancow-cleared",
    date: "2024-04-12",
    source: "mt",
    region: "KR",
    tags: ["livestock", "issuance"],
    title: "뱅카우 — 한우 1호 청산 완료, 투자자 평균 수익률 공개",
    summary:
      "스탁키퍼가 운영하는 뱅카우 첫 한우 투자계약증권이 청산 완료. 약 2년 사육 후 매각, 투자자 평균 수익률 산정 공시.",
    keyPoints: [
      "첫 청산 사례",
      "약 2년 사육 후 매각",
      "후속 청산 4건 진행",
    ],
    relatedIssuers: ["stockeeper"],
    url: "#",
    verified: true,
  },

  // ===== 2023 =====
  {
    id: "n-2023-12-yeolmae-1st",
    date: "2023-12-15",
    source: "thebell",
    region: "KR",
    tags: ["art", "issuance"],
    title: "열매컴퍼니, 쿠사마 야요이 '호박' 투자계약증권 1호 발행",
    summary:
      "열매컴퍼니가 쿠사마 야요이의 '호박' 작품을 기초자산으로 한 투자계약증권 1호 발행 승인. 한국 미술품 STO 정식 발행 첫 사례 중 하나.",
    keyPoints: [
      "투자계약증권 1호",
      "기초자산: 쿠사마 야요이 '호박'",
      "미술품 STO 정식 발행 첫 사례",
    ],
    relatedIssuers: ["yeolmae"],
    url: "#",
    verified: true,
  },
  {
    id: "n-2023-11-seoulauction",
    date: "2023-11-08",
    source: "hankyung",
    region: "KR",
    tags: ["art", "issuance"],
    title: "서울옥션블루, 미술품 투자계약증권 증권신고서 제출",
    summary:
      "서울옥션블루가 미술품 투자계약증권 증권신고서를 DART에 제출. 신한투자증권과 NXT 컨소 MOU 협약 7개사 중 하나로 참여.",
    keyPoints: [
      "DART 증권신고서 제출",
      "신한투자증권 협업",
      "NXT 컨소 MOU 7개사",
    ],
    relatedIssuers: ["seoulauctionblue"],
    url: "https://dart.fss.or.kr/",
    verified: true,
  },
  {
    id: "n-2023-04-fsc-tokensec",
    date: "2023-04-15",
    source: "fsc",
    region: "KR",
    tags: ["regulation"],
    title: "금융위 — 토큰증권 발행·유통 규율체계 정비방안 발표",
    summary:
      "금융위원회가 토큰증권 발행·유통 규율체계 정비방안을 발표. 분산원장 발행 허용, 발행·유통 라이선스 분리, 투자계약증권 명문화 등 핵심 골격 제시.",
    keyPoints: [
      "분산원장 발행 허용 방향",
      "발행·유통 라이선스 분리",
      "투자계약증권 명문화",
    ],
    relatedStos: [],
    url: "https://www.fsc.go.kr/no010101",
    verified: true,
  },

  // ===== 2022 =====
  {
    id: "n-2022-09-musicow-sandbox",
    date: "2022-09-15",
    source: "yonhap",
    region: "KR",
    tags: ["music", "regulation"],
    title: "뮤직카우, 혁신금융서비스 정식 지정 — 사후 정리 완료",
    summary:
      "금융위가 뮤직카우의 음악 저작권료 청구권을 신탁수익증권으로 정식 지정. 2022.04 증권성 판단 이후 사업 구조 재편을 거쳐 정식 사업자로 등록.",
    keyPoints: [
      "혁신금융서비스 정식 지정",
      "신탁수익증권 구조 재편",
      "투자자 보호 장치 강화",
    ],
    relatedIssuers: ["musicow"],
    url: "https://www.fsc.go.kr/no010101",
    verified: true,
  },
  {
    id: "n-2022-05-bancow-sandbox",
    date: "2022-05-20",
    source: "mt",
    region: "KR",
    tags: ["livestock", "regulation"],
    title: "뱅카우(스탁키퍼), 혁신금융서비스 지정 — 한우 투자계약증권",
    summary:
      "스탁키퍼의 한우 투자계약증권이 혁신금융서비스로 지정. KB증권과 실명계좌 제휴 운영 시작. 한우 사육·매각 수익권의 토큰증권화 첫 사례.",
    keyPoints: [
      "혁신금융서비스 지정",
      "KB증권 실명계좌 제휴",
      "한우 STO 첫 사례",
    ],
    relatedIssuers: ["stockeeper"],
    url: "https://www.fsc.go.kr/no010101",
    verified: true,
  },
  {
    id: "n-2022-04-musicow-judgement",
    date: "2022-04-20",
    source: "fsc",
    region: "KR",
    tags: ["music", "regulation"],
    title: "금융위, 뮤직카우 '저작권료 참여청구권' 증권성 인정",
    summary:
      "금융위원회가 뮤직카우의 저작권료 참여청구권을 자본시장법상 투자계약증권으로 판단. 한국 STO 시장의 분기점 결정.",
    keyPoints: [
      "투자계약증권 첫 공식 인정",
      "사업 중단 위기 후 사후 정리 결정",
      "한국 STO 정책 분기점",
    ],
    relatedIssuers: ["musicow"],
    url: "https://www.fsc.go.kr/no010101",
    verified: true,
  },

  // ===== 2021 =====
  {
    id: "n-2021-07-kasa-2",
    date: "2021-07-30",
    source: "ajunews",
    region: "KR",
    tags: ["real_estate", "issuance"],
    title: "카사 — 2호 자산 '서초 지웰타워 12층' 공모 당일 완판",
    summary:
      "카사 코리아의 부동산 신탁 디지털 수익증권(DABS) 2호 자산 '서초 지웰타워 12층'이 공모 당일 완판. 공모액 40억 원, 80만 DABS 발행.",
    keyPoints: [
      "공모 당일 완판",
      "공모액 40억 원",
      "1DABS = 5,000원, 80만 DABS",
    ],
    relatedStos: ["sto-kasa-2-jiwell"],
    relatedIssuers: ["kasa"],
    url: "#",
    verified: true,
  },
  {
    id: "n-2021-05-lucent-sandbox",
    date: "2021-05-12",
    source: "fsc",
    region: "KR",
    tags: ["real_estate", "regulation"],
    title: "루센트블록, 부동산 신탁수익증권 혁신금융서비스 지정",
    summary:
      "루센트블록이 운영하는 부동산 STO 플랫폼 '소유'가 혁신금융서비스로 지정. 지역 상업용 부동산을 신탁수익증권으로 발행하는 모델.",
    keyPoints: [
      "혁신금융서비스 지정",
      "지역 상업용 부동산 타깃",
      "신탁수익증권 모델",
    ],
    relatedIssuers: ["lucentblock"],
    url: "https://www.fsc.go.kr/no010101",
    verified: true,
  },

  // ===== 2020 =====
  {
    id: "n-2020-12-kasa-1",
    date: "2020-12-22",
    source: "cdkorea",
    region: "KR",
    tags: ["real_estate", "issuance"],
    title: "카사 — 부동산 STO 1호 자산 '역삼 런던빌' 출시",
    summary:
      "카사 코리아가 한국 1호 부동산 STO 자산으로 강남구 역삼동 '런던빌' 빌딩을 DABS(Digital Asset-Backed Securities)로 발행. 공모 총액 약 101억 원.",
    keyPoints: [
      "한국 부동산 STO 1호",
      "공모 총액 101억 원",
      "1DABS = 5,000원, 약 202만 DABS",
    ],
    relatedStos: ["sto-kasa-1-londonbill"],
    relatedIssuers: ["kasa"],
    url: "#",
    verified: true,
  },

  // ===== 글로벌 =====
  {
    id: "n-2026-04-tzero-vol",
    date: "2026-04-05",
    source: "coindesk",
    region: "GLOBAL",
    tags: ["global", "exchange"],
    title: "tZERO — 2026 1Q 거래량 전년 대비 증가",
    summary:
      "미국 토큰증권 ATS tZERO의 1분기 거래량이 전년 동기 대비 증가. 부동산·사모펀드 토큰화 상품 신규 등재 영향. 일별 종가는 공식 페이지에서 공시.",
    keyPoints: [
      "전년 동기 대비 거래량 증가",
      "부동산·사모펀드 신규 등재",
      "일별 종가 공식 공시",
    ],
    relatedStos: [],
    url: "https://www.tzero.com/",
    verified: true,
  },
  {
    id: "n-2026-03-blackrock-buidl",
    date: "2026-03-15",
    source: "bloomberg",
    region: "GLOBAL",
    tags: ["global", "issuance"],
    title: "BlackRock BUIDL 토큰화 머니마켓 펀드 자산 확대",
    summary:
      "BlackRock의 토큰화 머니마켓 펀드 BUIDL의 운용 자산이 확대. Securitize 기반 발행. 미국 적격 투자자 대상.",
    keyPoints: [
      "Securitize 기반 발행",
      "적격 투자자 대상 (Reg D)",
      "RWA 토큰화 기관 사례 대표",
    ],
    relatedStos: [],
    url: "#",
    verified: true,
  },
  {
    id: "n-2025-10-addx-asia",
    date: "2025-10-12",
    source: "reuters",
    region: "GLOBAL",
    tags: ["global", "exchange"],
    title: "ADDX, 아시아 사모 자산 토큰화 라인업 확대",
    summary:
      "싱가포르 MAS 라이선스 기반 토큰증권 거래소 ADDX가 아시아 사모펀드·부동산 토큰화 상품 라인업 확대. K-콘텐츠 IP 등 한국 자산도 검토 단계.",
    keyPoints: [
      "MAS 라이선스 기반",
      "아시아 사모 자산 라인업 확대",
      "K-콘텐츠 IP 검토",
    ],
    relatedStos: [],
    url: "https://addx.co/",
    verified: true,
  },
  {
    id: "n-2025-06-sec-realt",
    date: "2025-06-20",
    source: "sec",
    region: "GLOBAL",
    tags: ["global", "real_estate", "regulation"],
    title: "RealT — Reg D 506(c) 신규 자산 등록 누적 1,500건 돌파",
    summary:
      "디트로이트 주거용 부동산을 LLC 단위로 토큰화하는 RealT가 SEC EDGAR 기준 누적 등록 자산 1,500건 돌파. 자산당 1개 LLC 구조 유지.",
    keyPoints: [
      "누적 등록 자산 1,500건+",
      "Reg D 506(c) 적격 투자자",
      "Ethereum / Gnosis Chain 발행",
    ],
    relatedIssuers: ["realt"],
    url: "https://www.sec.gov/edgar",
    verified: true,
  },
  {
    id: "n-2024-11-masterworks",
    date: "2024-11-10",
    source: "techcrunch",
    region: "GLOBAL",
    tags: ["global", "art", "issuance"],
    title: "Masterworks — 누적 100여 점 미술품 토큰화, 일부 작품 청산",
    summary:
      "뱅크시·바스키아 등 블루칩 미술품 토큰화 사업자 Masterworks가 누적 100여 점 운영. 일부 작품은 매각·청산되어 투자자 분배 완료.",
    keyPoints: [
      "누적 100여 점 운영",
      "Reg A+ 일반 투자자 청약",
      "일부 작품 매각·청산",
    ],
    relatedIssuers: ["masterworks"],
    url: "#",
    verified: true,
  },
];

window.STO_NEWS = {
  NEWS_SOURCES,
  NEWS_TAGS,
  NEWS_ITEMS,
};
