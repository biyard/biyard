// Biyard STO Launchpad — Data (verified, source-attributed only)
// 원칙:
// - 발행사(issuer)와 개별 STO(asset)를 분리해서 모델링
// - 자산 맵은 "발행된 STO 단위"로 표시 (발행사 홍보 페이지 아님)
// - 자체 인덱스·점수·랭킹 부여 안 함
// - 사용자 활동·시세·실시간 가격 표시 안 함
// - 출처(DART / SEC / 공식 자료 / 보도자료) 항상 표기

const CATEGORIES = [
  { key: "real_estate", label: "부동산", icon: "🏢" },
  { key: "music", label: "음악 IP", icon: "🎵" },
  { key: "art", label: "미술품", icon: "🎨" },
  { key: "livestock", label: "한우·축산", icon: "🐄" },
  { key: "luxury", label: "명품·수집품", icon: "💎" },
  { key: "infra", label: "인프라·신재생", icon: "⚡" },
  { key: "content", label: "콘텐츠 IP", icon: "🎬" },
];

const SOURCES = {
  DART: { key: "DART", label: "DART", desc: "전자공시시스템 (한국 금감원)", url: "https://dart.fss.or.kr/" },
  SEC: { key: "SEC", label: "SEC EDGAR", desc: "미국 SEC 공시", url: "https://www.sec.gov/edgar" },
  FSC: { key: "FSC", label: "금융위·금감원", desc: "공식 보도자료", url: "https://www.fsc.go.kr/" },
  SANDBOX: { key: "SANDBOX", label: "핀테크지원센터", desc: "혁신금융서비스 지정 현황", url: "https://sandbox.fintech.or.kr/" },
  OFFICIAL: { key: "OFFICIAL", label: "사업자 공식 자료", desc: "발행 사업자 공식 사이트·보도자료" },
  RWAXYZ: { key: "RWAXYZ", label: "rwa.xyz", desc: "글로벌 RWA 통계", url: "https://rwa.xyz/" },
  PRESS: { key: "PRESS", label: "언론 보도", desc: "검증된 언론 매체 인용" },
};

const STATUS_LABELS = {
  confirmed: { key: "confirmed", label: "공식 확인", color: "green" },
  press: { key: "press", label: "보도 인용", color: "blue" },
  undisclosed: { key: "undisclosed", label: "미공시", color: "gold" },
  unverifiable: { key: "unverifiable", label: "확인 불가", color: "red" },
};

// =====================================================================
// 발행사 (Issuer) — 어떤 사업자가 어떤 인가·체인·구조로 운영하는가
// =====================================================================
const ISSUERS = [
  // 한국
  {
    id: "kasa",
    name: "카사 (Kasa Korea)",
    region: "KR",
    country: "🇰🇷 한국",
    category: "real_estate",
    sandbox: "혁신금융서비스 정식 지정 (2019.12, 1호)",
    chain: "프라이빗 (자체 K-Ledger)",
    status: "신규 매입 중단 · 인가 신청 중",
    description:
      "상업용 부동산 (오피스·호텔·물류센터·상가)을 신탁수익증권 'DABS'로 발행하는 한국 부동산 STO 1호 사업자.",
    sources: [
      { src: "FSC", label: "금융위 혁신금융서비스 지정" },
      { src: "PRESS", label: "코인데스크코리아 (2020) 카사 출시" },
      { src: "PRESS", label: "뉴스1 (2025) 신규 매수 중단 보도" },
      { src: "OFFICIAL", label: "kasa.co.kr" },
    ],
  },
  {
    id: "lucentblock",
    name: "루센트블록 (소유)",
    region: "KR",
    country: "🇰🇷 한국",
    category: "real_estate",
    sandbox: "혁신금융서비스 지정 (2021.05)",
    chain: "프라이빗 (컨소시엄)",
    status: "유통 인가 신청 중",
    description:
      "지역 상업용 부동산을 신탁수익증권으로 발행. '소유' 브랜드 운영. 발행에서 유통 라이선스로 전환 진행.",
    sources: [
      { src: "FSC", label: "금융위 혁신금융서비스 지정" },
      { src: "PRESS", label: "이데일리 (2025) 11호 완판 보도" },
      { src: "PRESS", label: "더퍼블릭 (2026) 250억 자산 정리 보도" },
      { src: "OFFICIAL", label: "sou.place" },
    ],
  },
  {
    id: "funble",
    name: "펀블 (Funble)",
    region: "KR",
    country: "🇰🇷 한국",
    category: "real_estate",
    sandbox: "혁신금융서비스 지정 (2021)",
    chain: "프라이빗",
    status: "운영 중 · 키움증권 협업",
    description:
      "랜드마크 상업용 부동산 신탁수익증권. 키움증권과 협업하여 제도권 금융사 인프라 결합.",
    sources: [
      { src: "FSC", label: "금융위 혁신금융서비스 지정" },
      { src: "PRESS", label: "Rabby (2026) 부동산 조각투자 비교" },
    ],
  },
  {
    id: "musicow",
    name: "뮤직카우 (Musicow)",
    region: "KR",
    country: "🇰🇷 한국",
    category: "music",
    sandbox: "혁신금융서비스 지정 (2022.09 사후 정리)",
    chain: "프라이빗 + 미국 SEC 등록 (해외)",
    status: "운영 중 · 미국 진출 · NXT 컨소",
    description:
      "음악 저작권료를 무체재산권 신탁수익증권으로 발행. 누적 거래액 4,200억원. 2025 미국 SEC 등록.",
    sources: [
      { src: "FSC", label: "금융위 — 증권성 판단 (2022.04)" },
      { src: "FSC", label: "금융위 혁신금융서비스 지정 (2022.09)" },
      { src: "PRESS", label: "한국경제 (2025) 미국 진출 완판" },
      { src: "OFFICIAL", label: "musicow.com" },
    ],
  },
  {
    id: "stockeeper",
    name: "스탁키퍼 (뱅카우)",
    region: "KR",
    country: "🇰🇷 한국",
    category: "livestock",
    sandbox: "혁신금융서비스 지정 (2022.05)",
    chain: "프라이빗 (자체)",
    status: "운영 중 · 시리즈B 70억",
    description:
      "한우 사육·매각 수익권을 투자계약증권으로 발행. KB증권 협업. 누적 15건 발행 5건 청산.",
    sources: [
      { src: "FSC", label: "금융위 혁신금융서비스 지정" },
      { src: "PRESS", label: "한국일보 (2026.04) 70억 시리즈B" },
      { src: "PRESS", label: "머니투데이 (2022) KB증권 협업" },
    ],
  },
  {
    id: "yeolmae",
    name: "열매컴퍼니",
    region: "KR",
    country: "🇰🇷 한국",
    category: "art",
    sandbox: "투자계약증권 정식 발행",
    chain: "(공개 정보 부족)",
    status: "운영 중 · 1호 발행 후 확장",
    description:
      "미술품 투자계약증권 발행. 2023.12 1호 — 쿠사마 야요이 '호박' 금감원 승인. 미술금융회사로 도약 추진.",
    sources: [
      { src: "DART", label: "증권신고서" },
      { src: "PRESS", label: "더벨 (2024) 미술금융회사 도약" },
    ],
  },
  {
    id: "seoulauctionblue",
    name: "서울옥션블루",
    region: "KR",
    country: "🇰🇷 한국",
    category: "art",
    sandbox: "혁신금융 지정",
    chain: "(공개 정보 부족)",
    status: "운영 중 · NXT 컨소 MOU",
    description:
      "미술품 투자계약증권. 2023.11 증권신고서 제출. 신한투자증권 + NXT 컨소 MOU 협약 7개사 중 1곳.",
    sources: [
      { src: "DART", label: "증권신고서 (2023.11)" },
      { src: "PRESS", label: "한국경제 (2023) 증권신고서 제출" },
    ],
  },
  {
    id: "togetherart",
    name: "투게더아트",
    region: "KR",
    country: "🇰🇷 한국",
    category: "art",
    sandbox: "혁신금융 지정 (확인 필요)",
    chain: "(공개 정보 부족)",
    status: "운영 중 · NXT 컨소 MOU",
    description:
      "미술품 투자계약증권 발행. 신한투자증권 + NXT 컨소 MOU 협약 참여.",
    sources: [
      { src: "DART", label: "증권신고서 (확인 필요)" },
      { src: "PRESS", label: "한국경제 — NXT MOU 협약" },
    ],
  },
  {
    id: "tessa",
    name: "테사 (TESSA)",
    region: "KR",
    country: "🇰🇷 한국",
    category: "art",
    sandbox: "혁신금융 지정",
    chain: "(공개 정보 부족)",
    status: "위축 · 키움증권 결별 (2026)",
    description:
      "미술품 조각투자 플랫폼. 2026년 키움증권과 실명계좌 제휴 중단. 일부 청약 미달 사례.",
    sources: [
      { src: "PRESS", label: "비즈한국 (2026) 키움증권 결별" },
      { src: "OFFICIAL", label: "tessa.art" },
    ],
  },

  // 글로벌
  {
    id: "securitize",
    name: "Securitize",
    region: "GLOBAL",
    country: "🇺🇸 미국",
    category: "real_estate",
    sandbox: "SEC 등록 디지털 증권 발행 플랫폼",
    chain: "Ethereum 등 멀티체인",
    status: "운영 중",
    description:
      "기관·고액 자산의 토큰화 발행 인프라 제공. Aspen Coin 등 대형 STO 발행 사례 다수.",
    sources: [
      { src: "SEC", label: "SEC EDGAR" },
      { src: "OFFICIAL", label: "securitize.io" },
    ],
  },
  {
    id: "masterworks",
    name: "Masterworks",
    region: "GLOBAL",
    country: "🇺🇸 미국",
    category: "art",
    sandbox: "SEC Reg A+ (일반 투자자 청약 가능)",
    chain: "Off-chain (SEC 등록)",
    status: "운영 중 · 누적 100여 점 운영",
    description:
      "뱅크시·바스키아 등 블루칩 미술품을 LLC 단위로 토큰화. 한국 발행 구조와는 다르나 (한국은 투자계약증권), 동일 자산(블루칩 미술품) 카테고리의 글로벌 비교군으로 참고.",
    referenceOnly: true,
    referenceNote: "발행 구조(LLC 지분) 는 한국 미허용 — 자산 카테고리(미술품) 비교군으로만 사용",
    sources: [
      { src: "SEC", label: "SEC EDGAR Reg A+" },
      { src: "OFFICIAL", label: "masterworks.com" },
    ],
  },
  {
    id: "anote",
    name: "ANote Music",
    region: "GLOBAL",
    country: "🇪🇺 EU",
    category: "music",
    sandbox: "ESMA Prospectus (EU 등록)",
    chain: "Polygon",
    status: "운영 중",
    description:
      "음원 카탈로그 단위 토큰화 플랫폼. EU 기반. 카탈로그 단위 SPV로 분산 효과.",
    sources: [
      { src: "OFFICIAL", label: "ANote Music 공식 자료" },
      { src: "PRESS", label: "ESMA prospectus" },
    ],
  },
  {
    id: "royal",
    name: "Royal",
    region: "GLOBAL",
    country: "🇺🇸 미국",
    category: "music",
    sandbox: "SEC Reg D / 일부 비증권 NFT",
    chain: "Ethereum / Polygon",
    status: "운영 중",
    description:
      "아티스트의 음원 권리를 NFT로 발행. 팬이 음원 로열티의 일부를 보유.",
    sources: [
      { src: "OFFICIAL", label: "royal.io" },
      { src: "PRESS", label: "TechCrunch 등 글로벌 언론" },
    ],
  },
  {
    id: "konvi",
    name: "Konvi",
    region: "GLOBAL",
    country: "🇪🇺 유럽",
    category: "luxury",
    sandbox: "EU 규제 (개별 확인)",
    chain: "(공개 정보 부족)",
    status: "운영 중",
    description:
      "고급 시계·와인·자동차 등 명품 자산 분할 소유 플랫폼.",
    sources: [
      { src: "OFFICIAL", label: "Konvi 공식 자료" },
      { src: "PRESS", label: "유럽 언론 보도" },
    ],
  },
  {
    id: "wavist",
    name: "WAVIST × SBI Digital Markets",
    region: "GLOBAL",
    country: "🇸🇬 싱가포르 / 🇰🇷 한국",
    category: "music",
    sandbox: "싱가포르 MAS 라이선스",
    chain: "(공개 정보 부족)",
    status: "운영 중 · 1차 상품 완판",
    description:
      "K-pop 음원 IP STO. 싱가포르 SBI Digital Markets + 한국 교보생명 협력. 한국 직접 STO 시장 열리기 전 글로벌 우회 경로.",
    sources: [
      { src: "PRESS", label: "MS TODAY (2025) K콘텐츠 토큰화" },
      { src: "OFFICIAL", label: "SBI Digital Markets 공식 발표" },
    ],
  },
  {
    id: "story-seoul",
    name: "Story Protocol × 서울거래",
    region: "GLOBAL",
    country: "🇺🇸 미국 / 🇰🇷 한국",
    category: "content",
    sandbox: "준비 중 (MOU 단계)",
    chain: "Story Protocol",
    status: "준비 중 · 거래소 공동 설립 MOU",
    description:
      "K-콘텐츠 IP 토큰증권 + RWA 거래소 공동 설립 MOU. K-pop·웹툰·드라마 IP를 토큰화해 글로벌 팬 펀딩 모델.",
    sources: [
      { src: "PRESS", label: "한국경제 (2025.09)" },
      { src: "PRESS", label: "딜사이트 (2025) 블록체인 간담회" },
    ],
  },
];

// =====================================================================
// STO (개별 발행 자산) — 자산 맵의 핵심 단위
// =====================================================================
// 검증 가능한 발행 사례만 등재. 출처가 불명확한 사례는 제외.
const STOS = [
  // ===== 부동산 (한국) =====
  {
    id: "sto-kasa-1-londonbill",
    name: "역삼 런던빌 (1호)",
    issuerId: "kasa",
    category: "real_estate",
    icon: "🏢",
    region: "KR",
    country: "🇰🇷 한국",
    securityType: "부동산 신탁 디지털 수익증권 (DABS)",
    issuedAt: "2020.12.22",
    status: "상장·거래 중",
    underlying: "강남구 역삼동 런던빌 (지하 1층 / 지상 8층)",
    description:
      "카사 1호 DABS. 한국 부동산 토큰증권 정식 발행 첫 사례.",
    offering: {
      amount: 10156000000,
      currency: "KRW",
      unitPrice: 5000,
      totalUnits: 2031200,
      subscriptionStart: "2020-12-15",
      subscriptionEnd: "2020-12-22",
      subscriptionRate: 100,
    },
    realEstate: {
      propertyName: "런던빌",
      address: "서울 강남구 역삼동",
      propertyType: "office",
      completionYear: 2019,
      floorsAbove: 8,
      floorsBelow: 1,
      occupancyAtIssuance: "임대 완료 (신축)",
      nearestStation: "역삼역 (도보 5분)",
    },
    issuanceStructure: {
      issuer: "카사 코리아",
      trustee: "한국토지신탁",
      trusteeRole: "수익증권 발행·자산 관리",
    },
    narrative: {
      highlights: [
        "강남 핵심 권역 신축 빌딩",
        "임대 완료 상태에서 발행",
        "공모 총액 약 101억 원, 1DABS = 5,000원",
      ],
    },
    sources: [
      { src: "PRESS", label: "이코노믹리뷰 — 첫 상장 역삼 런던빌" },
      { src: "PRESS", label: "비즈한국 — 6개월 수익률 보도" },
      { src: "OFFICIAL", label: "카사 공식 (kasa.co.kr)" },
    ],
  },
  {
    id: "sto-kasa-2-jiwell",
    name: "서초 지웰타워 12층 (2호)",
    issuerId: "kasa",
    category: "real_estate",
    icon: "🏢",
    region: "KR",
    country: "🇰🇷 한국",
    securityType: "부동산 신탁 디지털 수익증권 (DABS)",
    issuedAt: "2021.07.30",
    status: "공모 당일 완판 · 거래 중",
    offering: {
      amount: 4000000000,
      currency: "KRW",
      unitPrice: 5000,
      totalUnits: 800000,
      subscriptionStart: "2021-07-30",
      subscriptionEnd: "2021-07-30",
      subscriptionRate: 100,
    },
    realEstate: {
      propertyName: "서초 지웰타워 12층",
      address: "서울 서초구",
      propertyType: "office",
    },
    underlying: "서초 지웰타워 12층",
    description:
      "카사 2호 DABS. 공모액 40억 원, 80만 DABS, 1DABS = 5,000원. 공모 당일 완판.",
    sources: [
      { src: "PRESS", label: "아주경제 — 2호 건물 당일 완판" },
      { src: "OFFICIAL", label: "카사 공식" },
    ],
  },
  {
    id: "sto-lucent-11",
    name: "대전 하나 스타트업파크 (소유 11호)",
    issuerId: "lucentblock",
    category: "real_estate",
    icon: "🏢",
    region: "KR",
    country: "🇰🇷 한국",
    securityType: "부동산관리처분신탁 수익증권",
    issuedAt: "2026",
    status: "공모 완판 · 첫 배당 2026.05.30",
    underlying: "대전 하나 스타트업파크",
    description:
      "루센트블록 소유 11호 상품. 청약률 100% 달성. 목표 배당 수익 연 9%. 첫 배당일 2026.05.30.",
    sources: [
      { src: "PRESS", label: "이데일리 (2025) 11호 완판" },
      { src: "OFFICIAL", label: "sou.place" },
    ],
  },
  {
    id: "sto-lucent-9",
    name: "소유 9호 부동산 자산",
    issuerId: "lucentblock",
    category: "real_estate",
    icon: "🏢",
    region: "KR",
    country: "🇰🇷 한국",
    securityType: "부동산관리처분신탁 수익증권",
    issuedAt: "2024",
    status: "조기 완판",
    underlying: "(공개 정보 — 사업자 페이지 참조)",
    description: "루센트블록 9호 부동산 조각투자. 조기 완판 보도.",
    sources: [
      { src: "PRESS", label: "이데일리 (마켓인) 9호 조기 완판" },
    ],
  },

  // ===== 음악 IP (한국) =====
  {
    id: "sto-musicow-1-nctdream",
    name: "NCT DREAM 'ANL' 음악수익증권 (1호)",
    issuerId: "musicow",
    category: "music",
    icon: "🎤",
    region: "KR",
    country: "🇰🇷 한국",
    securityType: "비금전 신탁수익증권",
    issuedAt: "2023.11 (신고) · 2023.12.08 청약",
    status: "발행 완료 (1호)",
    underlying: "NCT DREAM 'ANL' 저작권료",
    description:
      "뮤직카우 1호 비금전 신탁수익증권. 2022.04 증권성 판단 이후 약 1년 7개월 만의 정식 발행.",
    music: {
      songTitle: "ANL",
      artistName: "NCT DREAM",
      releaseYear: 2023,
      rightType: "composition",
      rightTypeLabel: "저작권료 참여청구권",
      auctionResult: {
        auctionDate: "2024-01-05",
        startPrice: 14000,
        endPrice: 18200,
        soldOutMinutes: 6.57,
      },
    },
    issuanceStructure: {
      issuer: "뮤직카우",
    },
    narrative: {
      highlights: [
        "감독당국 양식에 따른 첫 정식 증권신고서 (2023.11.15 제출)",
        "옥션 14,000원 → 18,200원, 6분 34초 완판",
        "K-pop 아이돌 음원 최초 STO 발행",
      ],
      riskFactors: [
        "음원 인기 변동 — 시간 경과에 따른 저작권료 감소",
        "분배 일정 — 음저협 정산 주기 의존",
        "유통 시장 유동성",
      ],
    },
    sources: [
      { src: "DART", label: "증권신고서 (2023.11.16 공시)" },
      { src: "PRESS", label: "머니투데이 (2023.11.15) 비정형 자산 최초" },
      { src: "PRESS", label: "ZDNet Korea (2023.11.17) 1호 증권신고서 승인" },
    ],
    notes:
      "DART 폴링 결과 — 뮤직카우 본사·자회사(뮤직카우에셋·포트폴리오인베스트먼트) 의 corp_code 검색 결과 감사보고서만 등록. 곡별 발행공시 부재. 1호 신고서는 별도 신탁사 명의로 등록되었거나 사모형 특례로 추정.",
  },
  {
    id: "sto-musicow-kelly",
    name: "Kelly Clarkson 'Mr. Know It All' 음악수익증권",
    issuerId: "musicow",
    category: "music",
    icon: "🎤",
    region: "GLOBAL",
    country: "🇺🇸 미국",
    securityType: "SEC 등록 음악수익증권",
    issuedAt: "2025",
    status: "발행 완료 · 일반 투자자 전량 판매",
    underlying: "Kelly Clarkson 'Mr. Know It All' 저작권료",
    description:
      "뮤직카우 미국 법인 (Musicow US)이 SEC 승인을 받아 발행한 1호 음악 수익증권. 총 382주 일반 투자자 전량 판매.",
    sources: [
      { src: "PRESS", label: "한국경제 (2025.08) 美 진출 완판" },
      { src: "SEC", label: "SEC 승인" },
    ],
  },

  // ===== 미술품 (한국) =====
  {
    id: "sto-yeolmae-1-pumpkin",
    name: "쿠사마 야요이 〈Pumpkin (호박)〉 (1호)",
    issuerId: "yeolmae",
    category: "art",
    icon: "🎨",
    region: "KR",
    country: "🇰🇷 한국",
    securityType: "투자계약증권",
    issuedAt: "2023.12.15",
    status: "운용 중",
    underlying: "쿠사마 야요이 〈Pumpkin〉 (호박 시리즈 작품)",
    description:
      "한국 투자계약증권 정식 발행 1호. 미술품 STO 첫 사례.",
    art: {
      artworkTitle: "Pumpkin (호박)",
      artistName: "쿠사마 야요이 (Yayoi Kusama, 草間 彌生)",
      artistNationality: "일본",
      artistBirthYear: 1929,
      artistBio:
        "1929년생 일본 현대미술 거장. 1950년대 미국에서 활동을 시작해 호박·물방울 무늬·인피니티 룸 등 강박과 환각을 모티프로 한 작업으로 세계적 인지도를 얻었다. 2014년 〈White No. 28〉이 크리스티 뉴욕 경매에서 약 710만 달러에 낙찰되며 일본 작가 경매가 최고 기록. 메트로폴리탄·MoMA·테이트 모던·도쿄 국립현대미술관 등 주요 미술관 소장.",
      artistHighlights: [
        "1957년 미국 이주, 1973년 일본 귀국",
        "1993년 베네치아 비엔날레 일본관 단독 전시",
        "2017년 도쿄 신주쿠에 쿠사마 야요이 미술관 개관",
        "Louis Vuitton·디올 등 글로벌 브랜드 협업",
        "2022년 한국에서 〈Pumpkin Forever〉 대형 회고전 개최",
      ],
      seriesContext:
        "호박 시리즈는 쿠사마의 가장 상징적인 모티프로 1960년대부터 현재까지 회화·조각·설치 형태로 지속 제작. 시그니처 노란색·검정 점 패턴과 결합. 도쿄 나오시마섬 〈Yellow Pumpkin〉(1994) 이 세계적 랜드마크.",
      relatedAuctions: [
        { work: "White No. 28", year: 2014, house: "크리스티 뉴욕", priceUsd: 7100000, note: "일본 작가 경매가 최고 기록 (당시)" },
        { work: "Infinity Nets [TWHOQ]", year: 2008, house: "크리스티 뉴욕", priceUsd: 5790000 },
        { work: "Pumpkin (1981)", year: 2019, house: "크리스티 홍콩", priceHkd: 70000000, note: "중대형 호박 회화 사례" },
      ],
      relatedAuctionsNote: "유사 작가·시리즈의 공개된 경매 낙찰 결과. 본 STO 기초자산 가치 평가는 별도이며 발행사 감정평가 결과 참조.",
    },
    issuanceStructure: {
      issuer: "열매컴퍼니",
    },
    narrative: {
      highlights: [
        "한국 투자계약증권 정식 발행 1호 (전체 미술 STO 첫 사례)",
        "쿠사마 야요이 — 한국에서 가장 인지도 높은 현대미술가 중 하나",
        "동일 작가 〈White No. 28〉 약 710만 달러 낙찰 (2014, 크리스티)",
      ],
      riskFactors: [
        "미술품 시세 변동성 — 운용기간 동안 시장 가격 변화",
        "유동성 — 매각 시점·가격이 경매·딜러 시장에 의존",
        "위조 위험 — 출처(provenance) 검증 의존",
        "보관 위험 — 화재·도난·훼손 (보험 의무 가입)",
        "유사작 비교의 한계 — 경매가는 작품별 컨디션·크기 등에 따라 차이",
      ],
      operationPlan: "수장고 보관·일정 기간 운용 후 매각·청산. 운용기간 중 전시·대여 시 별도 협의 (수익 분배 가능).",
      exitPlan: "발행 안내된 운용기간 만료 시 경매·딜러 시장 매각 후 매각순액 분배.",
    },
    sources: [
      { src: "DART", label: "증권신고서 (2023.12)" },
      { src: "PRESS", label: "한국경제TV (2023.12) 신고서 승인" },
      { src: "PRESS", label: "뉴스1 (2023.12) 1호 청약" },
      { src: "OFFICIAL", label: "Yayoi Kusama Foundation·쿠사마 야요이 미술관" },
      { src: "PRESS", label: "Christie's·Sotheby's 공개 경매 낙찰 결과" },
    ],
  },
  {
    id: "sto-yeolmae-2-leeufan",
    name: "이우환 〈Dialogue (다이얼로그)〉 (2호)",
    issuerId: "yeolmae",
    category: "art",
    icon: "🎨",
    region: "KR",
    country: "🇰🇷 한국",
    securityType: "투자계약증권",
    issuedAt: "2024.05 (청약)",
    status: "발행 완료",
    underlying: "이우환 〈Dialogue〉",
    description:
      "열매컴퍼니 2호 투자계약증권. 한국 미술계 거장 이우환 작품. 공모 12억 3천만 원, 1주 = 10만 원.",
    offering: {
      amount: 1230000000,
      currency: "KRW",
      unitPrice: 100000,
      totalUnits: 12300,
    },
    art: {
      artworkTitle: "Dialogue (다이얼로그)",
      artistName: "이우환 (Lee Ufan)",
    },
    sources: [
      { src: "DART", label: "증권신고서 (2024)" },
      { src: "PRESS", label: "글로벌이코노믹 (2025) 이우환 다이얼로그 청약" },
    ],
  },
  {
    id: "sto-seoulauction-1-warhol",
    name: "앤디 워홀 〈Dollar Sign〉 (1호)",
    issuerId: "seoulauctionblue",
    category: "art",
    icon: "🖼️",
    region: "KR",
    country: "🇰🇷 한국",
    securityType: "투자계약증권",
    issuedAt: "2023.11 (신고) · 2023.12.20~26 청약",
    status: "발행 완료",
    underlying: "앤디 워홀 〈Dollar Sign〉 (51.0×40.5cm, 8호)",
    description:
      "서울옥션블루(앱: 소투) 1호 투자계약증권. 한국 미술품 STO 정식 발행 첫 사례 중 하나.",
    offering: {
      amount: 700000000,
      currency: "KRW",
      unitPrice: 100000,
      totalUnits: 7000,
    },
    art: {
      artworkTitle: "Dollar Sign",
      artistName: "앤디 워홀 (Andy Warhol)",
      dimensions: "51.0 × 40.5 cm (8호)",
      acquisitionPrice: 626230000,
      acquisitionCurrency: "KRW",
      acquisitionVenue: "서울옥션 경매",
    },
    issuanceStructure: {
      issuer: "서울옥션블루",
      accountBank: "KB증권",
      accountBankRole: "청약증거금 분리계좌",
      protectionFundManager: "신한투자증권",
      protectionFundRole: "투자자보호기금 신탁",
    },
    sources: [
      { src: "DART", label: "증권신고서 (2023.11.28 제출)" },
      { src: "PRESS", label: "머니투데이 (2023.11) 1호 증권 앤디 워홀" },
      { src: "PRESS", label: "이투데이 (2023.11) 증권신고서 제출" },
      { src: "OFFICIAL", label: "서울옥션블루 공식 — soto" },
    ],
  },
  {
    id: "sto-togetherart-leeufan",
    name: "이우환 〈Dialogue〉 — 투게더아트",
    issuerId: "togetherart",
    category: "art",
    icon: "🎨",
    region: "KR",
    country: "🇰🇷 한국",
    securityType: "투자계약증권",
    issuedAt: "2025.01 (청약)",
    status: "발행 완료",
    underlying: "이우환 〈Dialogue〉",
    description:
      "투게더아트 미술품 투자계약증권. 이우환 작가의 〈Dialogue〉를 기초자산으로 발행.",
    sources: [
      { src: "DART", label: "증권신고서" },
      { src: "PRESS", label: "글로벌이코노믹 (2025.01) 이우환 다이얼로그 청약" },
    ],
  },

  // ===== 한우 (한국) — DART rcept 20260414002068 정정신고서 청산 결과표 =====
  {
    id: "sto-bancow-1-1",
    name: "가축투자계약증권 제1-1호 (계림농장)",
    issuerId: "stockeeper",
    category: "livestock",
    icon: "🐂",
    region: "KR",
    country: "🇰🇷 한국",
    securityType: "가축투자계약증권",
    issuedAt: "2024.07.05",
    status: "청산 완료",
    underlying: "계림농장 한우 거세우 50두",
    description:
      "스탁키퍼 가축투자계약증권 제1-1호. 532일 운용 후 청산. 매각순액 481,191,510원, 수익률 11.2% (연환산 7.7%).",
    dartRcptNo: "20260414002068",
    prospectusUrl:
      "https://dart.fss.or.kr/dsaf001/main.do?rcpNo=20260414002068",
    offering: {
      amount: 432600000,
      currency: "KRW",
      totalUnits: 50,
      unitPrice: 8652000,
      businessCost: 426160869,
      unusedAmount: 6439131,
    },
    livestock: {
      breed: "한우 거세우",
      farmName: "계림농장",
      cattleCount: 50,
      feedingPeriodDays: 532,
      feedingMethod: "약 24개월 비육 후 출하 — 곡물 사료 + 조사료 혼합 급여",
    },
    issuanceStructure: {
      issuer: "㈜스탁키퍼",
      issuerAddress: "서울특별시 강남구 테헤란로501, 브이플렉스208~210호",
      issuerCeo: "안재현",
      underwriter: "KB증권",
      accountBank: "KB증권",
      revenueShareNote: "운용기간 종료 후 매각순액 분배",
    },
    narrative: {
      highlights: [
        "스탁키퍼 첫 정식 발행 청산 완료 사례",
        "532일 운용 후 매각순액 481,191,510원",
        "공시 수익률 11.2% (연환산 7.7%)",
      ],
      riskFactors: [
        "가축전염병 (구제역 등 살처분 대상 감염병)",
        "사료비 변동 — 곡물 시세에 따른 운영비 증가",
        "도축 등급 미달 시 매각 가격 하락",
        "자연재해·화재·도난",
      ],
      operationPlan: "강원 계림농장에서 약 24개월 비육. 정기 수의 검진. 출하는 한우 경매시장 위탁.",
    },
    fees: {
      saleFee: 15837385,
      saleFeeNote: "경매수수료·도축수수료 등",
      liquidationFee: 1608200,
      liquidationFeeNote: "공동사업 운영자 청산수수료 + 등급장려금",
    },
    investorProtection: {
      fund1RatePercent: 1.5,
      fund1Note: "발행 전 모집예정금액의 1.5% 별도 계좌 적립 — 질병 폐사 보상",
      fund2Note: "청산수수료의 30% 적립, 최종 3.5억원 목표 — 화재·자연재해·도난 보상",
      coverageScope: "가축전염병 살처분 / 질병 폐사 / 도난 / 화재·자연재해",
    },
    liquidation: {
      liquidatedAt: "2025-12-19",
      salePrice: 492197964,
      currency: "KRW",
      netSalePrice: 481191510,
      reportedYieldPercent: 11.2,
      reportedYieldType: "total",
      annualizedYieldPercent: 7.7,
      reportedYieldSource: "DART rcept 20260414002068 — 정정신고서 (5건 청산 결과표)",
      reportedYieldSourceUrl: "https://dart.fss.or.kr/dsaf001/main.do?rcpNo=20260414002068",
    },
    sources: [
      { src: "DART", label: "정정신고서 (rcept 20260414002068) — 청산 결과표 + 투자자보호기금" },
      { src: "DART", label: "원 신고서 (rcept 20260326001272 — 8호 정정 시 1-1 데이터 포함)" },
      { src: "OFFICIAL", label: "스탁키퍼 stockeeper.co.kr" },
    ],
    verified: true,
  },
  {
    id: "sto-bancow-1-2",
    name: "가축투자계약증권 제1-2호 (계림농장)",
    issuerId: "stockeeper",
    category: "livestock",
    icon: "🐂",
    region: "KR",
    country: "🇰🇷 한국",
    securityType: "가축투자계약증권",
    issuedAt: "2024.07.22",
    status: "청산 완료",
    underlying: "계림농장 한우 거세우 50두",
    description:
      "519일 운용 후 청산. 매각순액 482,499,848원, 수익률 11.1% (연환산 7.8%).",
    dartRcptNo: "20260414002068",
    prospectusUrl:
      "https://dart.fss.or.kr/dsaf001/main.do?rcpNo=20260414002068",
    offering: {
      amount: 434200000, currency: "KRW", totalUnits: 50,
      businessCost: 428671493, unusedAmount: 5528507,
    },
    livestock: { breed: "한우 거세우", farmName: "계림농장", cattleCount: 50, feedingPeriodDays: 519 },
    issuanceStructure: {
      issuer: "㈜스탁키퍼",
      issuerCeo: "안재현",
      underwriter: "KB증권",
      accountBank: "KB증권",
    },
    fees: {
      saleFee: 15361022, saleFeeNote: "경매수수료·도축수수료 등",
      liquidationFee: 956000, liquidationFeeNote: "청산수수료 + 등급장려금",
    },
    investorProtection: {
      fund1RatePercent: 1.5,
      fund1Note: "모집액의 1.5% 적립 — 질병 폐사 보상",
      fund2Note: "청산수수료의 30%, 최종 3.5억원 목표",
    },
    liquidation: {
      liquidatedAt: "2025-12-23",
      salePrice: 493288363,
      netSalePrice: 482499848,
      currency: "KRW",
      reportedYieldPercent: 11.1,
      reportedYieldType: "total",
      annualizedYieldPercent: 7.8,
      reportedYieldSource: "DART rcept 20260414002068",
      reportedYieldSourceUrl: "https://dart.fss.or.kr/dsaf001/main.do?rcpNo=20260414002068",
    },
    sources: [{ src: "DART", label: "정정신고서 청산 결과표 (rcept 20260414002068)" }],
    verified: true,
  },
  {
    id: "sto-bancow-2-1",
    name: "가축투자계약증권 제2-1호 (충만농장)",
    issuerId: "stockeeper",
    category: "livestock",
    icon: "🐂",
    region: "KR",
    country: "🇰🇷 한국",
    securityType: "가축투자계약증권",
    issuedAt: "2024.10.25",
    status: "청산 완료",
    underlying: "충만농장 한우 거세우 44두",
    description:
      "398일 운용 후 청산. 매각순액 423,605,086원, 수익률 11.3% (연환산 10.4%).",
    dartRcptNo: "20260414002068",
    prospectusUrl:
      "https://dart.fss.or.kr/dsaf001/main.do?rcpNo=20260414002068",
    offering: {
      amount: 380540000, currency: "KRW", totalUnits: 44,
      businessCost: 368759342, unusedAmount: 11780658,
    },
    livestock: { breed: "한우 거세우", farmName: "충만농장", cattleCount: 44, feedingPeriodDays: 398 },
    issuanceStructure: { issuer: "㈜스탁키퍼", issuerCeo: "안재현", underwriter: "KB증권", accountBank: "KB증권" },
    fees: {
      saleFee: 16838014, saleFeeNote: "경매수수료·도축수수료 등",
      liquidationFee: 3912000, liquidationFeeNote: "청산수수료 + 등급장려금",
    },
    investorProtection: {
      fund1RatePercent: 1.5,
      fund1Note: "모집액의 1.5% 적립 — 질병 폐사 보상",
      fund2Note: "청산수수료의 30%, 최종 3.5억원 목표",
    },
    liquidation: {
      liquidatedAt: "2025-11-27",
      salePrice: 432574442,
      netSalePrice: 423605086,
      currency: "KRW",
      reportedYieldPercent: 11.3,
      annualizedYieldPercent: 10.4,
      reportedYieldSource: "DART rcept 20260414002068",
      reportedYieldSourceUrl: "https://dart.fss.or.kr/dsaf001/main.do?rcpNo=20260414002068",
    },
    sources: [{ src: "DART", label: "정정신고서 청산 결과표 (rcept 20260414002068)" }],
    verified: true,
  },
  {
    id: "sto-bancow-2-2",
    name: "가축투자계약증권 제2-2호 (충만농장)",
    issuerId: "stockeeper",
    category: "livestock",
    icon: "🐂",
    region: "KR",
    country: "🇰🇷 한국",
    securityType: "가축투자계약증권",
    issuedAt: "2024.11.07",
    status: "청산 완료",
    underlying: "충만농장 한우 거세우 44두",
    description:
      "385일 운용 후 청산. 매각순액 433,476,682원, 수익률 14.1% (연환산 13.3%).",
    dartRcptNo: "20260414002068",
    prospectusUrl:
      "https://dart.fss.or.kr/dsaf001/main.do?rcpNo=20260414002068",
    offering: {
      amount: 380000000, currency: "KRW", totalUnits: 44,
      businessCost: 366568197, unusedAmount: 13431803,
    },
    livestock: { breed: "한우 거세우", farmName: "충만농장", cattleCount: 44, feedingPeriodDays: 385 },
    issuanceStructure: { issuer: "㈜스탁키퍼", issuerCeo: "안재현", underwriter: "KB증권", accountBank: "KB증권" },
    fees: {
      saleFee: 16697570, saleFeeNote: "경매수수료·도축수수료 등",
      liquidationFee: 5888350, liquidationFeeNote: "청산수수료 + 등급장려금",
    },
    investorProtection: {
      fund1RatePercent: 1.5,
      fund1Note: "모집액의 1.5% 적립 — 질병 폐사 보상",
      fund2Note: "청산수수료의 30%, 최종 3.5억원 목표",
    },
    liquidation: {
      liquidatedAt: "2025-11-27",
      salePrice: 442630799,
      netSalePrice: 433476682,
      currency: "KRW",
      reportedYieldPercent: 14.1,
      annualizedYieldPercent: 13.3,
      reportedYieldSource: "DART rcept 20260414002068",
      reportedYieldSourceUrl: "https://dart.fss.or.kr/dsaf001/main.do?rcpNo=20260414002068",
    },
    sources: [{ src: "DART", label: "정정신고서 청산 결과표 (rcept 20260414002068)" }],
    verified: true,
  },
  {
    id: "sto-bancow-2-3",
    name: "가축투자계약증권 제2-3호 (충만농장)",
    issuerId: "stockeeper",
    category: "livestock",
    icon: "🐂",
    region: "KR",
    country: "🇰🇷 한국",
    securityType: "가축투자계약증권",
    issuedAt: "2024.11.22",
    status: "청산 완료",
    underlying: "충만농장 한우 거세우 43두",
    description:
      "371일 운용 후 청산. 매각순액 435,793,707원, 수익률 17.0% (연환산 16.7%) — 5건 중 최고.",
    dartRcptNo: "20260414002068",
    prospectusUrl:
      "https://dart.fss.or.kr/dsaf001/main.do?rcpNo=20260414002068",
    offering: {
      amount: 372440000, currency: "KRW", totalUnits: 43,
      businessCost: 362687505, unusedAmount: 11752495,
    },
    livestock: { breed: "한우 거세우", farmName: "충만농장", cattleCount: 43, feedingPeriodDays: 371 },
    issuanceStructure: { issuer: "㈜스탁키퍼", issuerCeo: "안재현", underwriter: "KB증권", accountBank: "KB증권" },
    fees: {
      saleFee: 17372228, saleFeeNote: "경매수수료·도축수수료 등",
      liquidationFee: 10973760, liquidationFeeNote: "청산수수료 + 등급장려금",
    },
    investorProtection: {
      fund1RatePercent: 1.5,
      fund1Note: "모집액의 1.5% 적립 — 질병 폐사 보상",
      fund2Note: "청산수수료의 30%, 최종 3.5억원 목표",
    },
    liquidation: {
      liquidatedAt: "2025-11-28",
      salePrice: 452387200,
      netSalePrice: 435793707,
      currency: "KRW",
      reportedYieldPercent: 17.0,
      annualizedYieldPercent: 16.7,
      reportedYieldSource: "DART rcept 20260414002068",
      reportedYieldSourceUrl: "https://dart.fss.or.kr/dsaf001/main.do?rcpNo=20260414002068",
    },
    sources: [{ src: "DART", label: "정정신고서 청산 결과표 (rcept 20260414002068)" }],
    verified: true,
  },
  {
    id: "sto-bancow-8",
    name: "뱅카우 8호 (제8-1·제8-2 합산)",
    issuerId: "stockeeper",
    category: "livestock",
    icon: "🐂",
    region: "KR",
    country: "🇰🇷 한국",
    securityType: "가축투자계약증권",
    issuedAt: "2026-03-26",
    status: "발행 진행",
    underlying: "한우 거세우 사육 (약 24개월)",
    description:
      "스탁키퍼 가축투자계약증권 제8-1호 30,367주 + 제8-2호 30,547주 합산 발행. 모집총액 ₩1,218,220,000. 신고서 2026.03.26 제출, 효력 2026.03.27 발생.",
    dartRcptNo: "20260326001272",
    prospectusUrl:
      "https://dart.fss.or.kr/dsaf001/main.do?rcpNo=20260326001272",
    offering: {
      amount: 1218220000,
      currency: "KRW",
      totalUnits: 60914,
    },
    livestock: { breed: "한우 거세우", feedingPeriodMonths: 24, lotName: "뱅카우 8호 (8-1 + 8-2)" },
    sources: [
      { src: "DART", label: "증권신고서(투자계약증권) rcept_no 20260326001272" },
    ],
    verified: true,
  },
  // ===== 부동산 (글로벌) =====
  // 주의: LLC 지분 토큰화(RealT 등) 는 한국에서 미허용 — 인텔리전스 가치 낮아 제외.
  // 단일 자산 SPV 형태(Aspen 등) 만 한국 부동산 STO 매핑 가능.
  {
    id: "sto-aspen",
    name: "St. Regis Aspen 호텔 지분 18.9% (Aspen Coin)",
    issuerId: "securitize",
    category: "real_estate",
    icon: "🏔️",
    region: "GLOBAL",
    country: "🇺🇸 미국",
    securityType: "Reg D 506(c)",
    issuedAt: "2018.10",
    status: "운영 중 · 2차 거래량 미미",
    underlying: "St. Regis Aspen 호텔 지분 18.9%",
    description:
      "콜로라도 아스펜 St. Regis 호텔 지분 18.9%를 토큰화. 발행 당시 $18M 모집. tZERO 거래소 상장.",
    sources: [
      { src: "SEC", label: "SEC EDGAR Reg D" },
      { src: "OFFICIAL", label: "Securitize 공식" },
    ],
    krMappingNote:
      "단일 호텔·랜드마크 부동산 토큰화 사례. 한국 호텔 STO 시 참고. 2차 유동성 한계 사례로도 의미.",
  },

  // ===== 미술품 (글로벌, 참고 비교군) =====
  // 한국은 투자계약증권 / 미국 Masterworks 는 LLC 지분. 발행 구조 다름.
  // 자산 카테고리 비교군으로만 사용.
  {
    id: "sto-masterworks-bluechip",
    name: "블루칩 미술품 컬렉션 (100여 점) — Masterworks",
    issuerId: "masterworks",
    category: "art",
    icon: "🖌️",
    region: "GLOBAL",
    country: "🇺🇸 미국",
    securityType: "Reg A+ (LLC 지분)",
    issuedAt: "2017.10~",
    status: "운영 중",
    underlying: "뱅크시·바스키아·KAWS 등 블루칩 미술품",
    description:
      "블루칩 미술품을 작품당 1개 LLC로 분할 소유. 재판매 시 시세 차익 분배. 한국은 투자계약증권 구조 — 발행 구조 차이 있음.",
    referenceOnly: true,
    referenceNote: "발행 구조(LLC 지분) 는 한국 미허용 — 미술품 카테고리 글로벌 비교군 용도",
    sources: [
      { src: "SEC", label: "SEC EDGAR Reg A+" },
      { src: "OFFICIAL", label: "masterworks.com" },
    ],
    krMappingNote:
      "서울옥션블루·열매컴퍼니·테사 모델의 자산 카테고리 비교군. 단 발행 구조는 한국과 다름 (LLC 지분 vs 투자계약증권).",
  },

  // ===== 음악 IP (글로벌) =====
  {
    id: "sto-anote-jpop",
    name: "J-POP 음원 카탈로그 25곡",
    issuerId: "anote",
    category: "music",
    icon: "🎶",
    region: "GLOBAL",
    country: "🇯🇵 일본 / 🇪🇺 EU",
    securityType: "ESMA Prospectus",
    issuedAt: "2022.06",
    status: "운영 중",
    underlying: "J-POP 음원 카탈로그 25곡",
    description:
      "J-POP 카탈로그 단위 SPV. 월 스트리밍 수익 자동 분배.",
    sources: [
      { src: "OFFICIAL", label: "ANote Music 공식" },
      { src: "PRESS", label: "ESMA prospectus" },
    ],
    krMappingNote:
      "뮤직카우 글로벌 사례. 카탈로그 단위 발행 (단곡 X, 묶음 O) — 분산 효과.",
  },
  {
    id: "sto-royal-nas",
    name: "Nas 'Ultra Black' 등 아티스트 음원 권리",
    issuerId: "royal",
    category: "music",
    icon: "👑",
    region: "GLOBAL",
    country: "🇺🇸 미국",
    securityType: "Reg D / 일부 비증권 NFT",
    issuedAt: "2021.10~",
    status: "운영 중",
    underlying: "Nas, Diplo, 3LAU 등 아티스트 음원 권리",
    description:
      "아티스트 음원 권리를 NFT로 발행. 팬이 음원 로열티의 일부를 보유.",
    sources: [
      { src: "OFFICIAL", label: "royal.io" },
      { src: "PRESS", label: "TechCrunch" },
    ],
    krMappingNote:
      "팬덤 직접 수익 공유 모델. K-pop 글로벌 팬덤 활용 가능성.",
  },
  {
    id: "sto-wavist-1",
    name: "K-pop 음원 IP STO 1차 (322만 달러)",
    issuerId: "wavist",
    category: "music",
    icon: "🎤",
    region: "GLOBAL",
    country: "🇸🇬 싱가포르 / 🇰🇷 한국",
    securityType: "MAS 라이선스 STO",
    issuedAt: "2025",
    status: "1차 상품 완판",
    underlying: "K-pop 음원 IP",
    description:
      "K-pop 음원 IP STO. 322만 달러 (한화 약 45억원) 규모로 완판. 한국 직접 STO 시장 열리기 전 글로벌 우회 경로 사례.",
    sources: [
      { src: "PRESS", label: "MS TODAY (2025)" },
      { src: "OFFICIAL", label: "SBI Digital Markets" },
    ],
    krMappingNote:
      "한국 K-pop IP가 해외 STO 인프라(싱가포르 MAS)를 활용한 우회 발행. 시행 후 한국 직접 발행으로 전환 가능성.",
  },

  // ===== 명품 (글로벌) =====
  {
    id: "sto-konvi-luxury",
    name: "명품 시계·와인·자동차 분할 소유 시리즈",
    issuerId: "konvi",
    category: "luxury",
    icon: "⌚",
    region: "GLOBAL",
    country: "🇪🇺 유럽",
    securityType: "EU 규제 (개별 확인)",
    issuedAt: "2021~",
    status: "운영 중",
    underlying: "롤렉스·파텍 필립·빈티지 와인·자동차",
    description:
      "고급 시계·와인·자동차 등 명품 자산 분할 소유.",
    sources: [
      { src: "OFFICIAL", label: "Konvi 공식" },
      { src: "PRESS", label: "유럽 언론" },
    ],
    krMappingNote:
      "한국 명품 조각투자 영역의 글로벌 사례. 한국 트레저러 등이 유사 모델.",
  },
];

// =====================================================================
// ISSUER_SUMMARIES — 시리즈/누적 집계 row (개별 STO 와 분리)
// 절대 STOS 와 같은 콜렉션에 두지 말 것 — 통계·필터·UI 모두 왜곡됨.
// 사용처: 발행사 페이지 / detail.html 의 발행사 카드 / 시리즈 위젯.
// =====================================================================
const ISSUER_SUMMARIES = [
  {
    id: "summary-stockeeper-bancow",
    issuerId: "stockeeper",
    seriesName: "뱅카우 시리즈",
    region: "KR",
    category: "livestock",
    totalIssued: 8,
    liquidatedCount: 5,
    cumulativeAmountKRW: 432600000 + 434200000 + 380540000 + 380000000 + 372440000 + 1218220000,
    note: "DART corp_code 01760118 폴링 — 2023.12.29~2026.04 누적 발행공시 49건 (증권신고서 + 정정 + 발행실적 + 투자설명서 사이클).",
    sources: [
      { src: "DART", label: "발행공시 49건 폴링" },
      { src: "PRESS", label: "한국일보 2026.04 시리즈B 70억" },
    ],
    verified: true,
  },
  {
    id: "summary-kasa-2024",
    issuerId: "kasa",
    seriesName: "카사 2024 신규 등록 자산",
    region: "KR",
    category: "real_estate",
    totalIssued: 3,
    items: ["마포구 상암 235빌딩", "서대문구 그레인바운더리빌딩", "종로구 북촌 월하재"],
    note: "2024년 신규 등록 3건. 정확한 발행 일자·DABS 단가는 DART 신고서 직접 확인 시 개별 row 로 분리 가능.",
    sources: [
      { src: "PRESS", label: "뉴스1 2025 카사 자산 보도" },
      { src: "DART", label: "증권신고서 (수기 확인 대기)" },
    ],
    verified: false,
  },
];

// 한국 STO 시장 타임라인
const TIMELINE = [
  { date: "2019.12", label: "카사 — 혁신금융서비스 지정 1호 (부동산)", state: "done", source: "FSC" },
  { date: "2022.04", label: "금융위 — 뮤직카우 증권성 판단 (투자계약증권)", state: "done", source: "FSC" },
  { date: "2023.04", label: "금융위 — 토큰증권 발행·유통 규율체계 정비방안 발표", state: "done", source: "FSC" },
  { date: "2025.12.03", label: "자본시장법·전자증권법 개정안 법사위 의결", state: "done", source: "FSC" },
  { date: "2026.01.15", label: "국회 본회의 최종 통과", state: "done", source: "FSC" },
  { date: "2026.02", label: "장외거래소 예비인가 (KDX, NXT 조건부)", state: "done", source: "FSC" },
  { date: "2026.03", label: "민관 합동 토큰증권 협의체 출범", state: "done", source: "FSC" },
  { date: "2026.05", label: "Biyard — 글로벌+한국 통합 정보 플랫폼 1차 MVP", state: "now" },
  { date: "2027.02.04", label: "개정법 시행 예정", state: "pending", source: "FSC" },
];

// 헬퍼: 발행사 ID로 발행사 조회
function findIssuer(id) {
  return ISSUERS.find((i) => i.id === id);
}

// 헬퍼: 발행사 ID로 해당 발행사의 STO 목록 조회
function stosByIssuer(id) {
  return STOS.filter((s) => s.issuerId === id);
}

// 카테고리별 STO·발행사 통계
function categoryStats() {
  return CATEGORIES.map((c) => {
    const krStos = STOS.filter((s) => s.category === c.key && s.region === "KR");
    const globalStos = STOS.filter((s) => s.category === c.key && s.region === "GLOBAL");
    const krIssuers = ISSUERS.filter((i) => i.category === c.key && i.region === "KR");
    const globalIssuers = ISSUERS.filter((i) => i.category === c.key && i.region === "GLOBAL");
    return {
      ...c,
      krStosCount: krStos.length,
      globalStosCount: globalStos.length,
      krIssuersCount: krIssuers.length,
      globalIssuersCount: globalIssuers.length,
    };
  });
}

// 헬퍼: 발행사 ID 로 시리즈 요약 조회
function summariesByIssuer(id) {
  return ISSUER_SUMMARIES.filter((x) => x.issuerId === id);
}

// DART OpenAPI 공식 백필 STO 머지 (data-dart-backfill.js 가 먼저 로드되어 있다면)
// 중복 매핑 — 기존 보도자료/공식 자료 기반 STO 와 DART 공시 회차가 동일한 케이스를 머지.
//   기존 ID 가 우선, DART 메타(rceptNo·filingUrl·filingsCount·offering)는 기존 항목에 병합.
// DART 사이클 ID → 기존 보도자료 기반 STO ID 매핑.
//   동일 회차로 판단되면 기존 항목에 DART 메타·소스 배지를 합치고,
//   매핑이 없는 DART 사이클은 새 항목으로 추가됨.
const DART_DUP_MAP = {
  "sto-yeolmae-cyc-1": "sto-yeolmae-1-pumpkin",       // 야요이 쿠사마 Pumpkin (열매 1호)
  "sto-yeolmae-cyc-2": "sto-yeolmae-2-leeufan",       // 이우환 Dialogue (열매 2호)
  "sto-seoulauctionblue-cyc-1": "sto-seoulauction-1-warhol", // 워홀 Dollar Sign
};

function mergeDartBackfill(base, backfill) {
  if (!Array.isArray(backfill)) return base;
  const baseById = new Map(base.map((s) => [s.id, s]));
  const merged = [...base];
  for (const dartSto of backfill) {
    const targetId = DART_DUP_MAP[dartSto.id];
    if (targetId && baseById.has(targetId)) {
      const target = baseById.get(targetId);
      // DART 메타만 병합. 기존 name/description/narrative 는 유지.
      target.dart = dartSto.dart;
      if (!target.offering) target.offering = {};
      for (const k of ["amount", "currency", "unitPrice", "totalUnits"]) {
        if (dartSto.offering && dartSto.offering[k] != null && target.offering[k] == null) {
          target.offering[k] = dartSto.offering[k];
        }
      }
      // DART 소스 배지 추가
      const hasDartSrc = (target.sources || []).some((s) => s.src === "DART");
      if (!hasDartSrc) {
        target.sources = [
          ...(target.sources || []),
          { src: "DART", label: `DART 증권신고서 (${dartSto.dart.rceptNo})` },
        ];
      }
    } else {
      merged.push(dartSto);
    }
  }
  return merged;
}

const STOS_WITH_DART =
  typeof window !== "undefined" && Array.isArray(window.DART_BACKFILL_STOS)
    ? mergeDartBackfill(STOS, window.DART_BACKFILL_STOS)
    : STOS;

// 뮤직카우 카탈로그 머지 (data-musicow.js — 시장 데이터 제외, STO 식별 메타만)
const STOS_WITH_MUSICOW =
  typeof window !== "undefined" && Array.isArray(window.MUSICOW_STOS)
    ? [...STOS_WITH_DART, ...window.MUSICOW_STOS]
    : STOS_WITH_DART;

// 아티피오 발행사 (DART 백필에서 발견 — 미술품 투자계약증권 발행사)
const ARTIPIO = {
  id: "artipio",
  name: "아티피오 (Artipio)",
  region: "KR",
  country: "🇰🇷 한국",
  category: "art",
  sandbox: "—",
  chain: "—",
  status: "발행 시도 중",
  description:
    "미술품 투자계약증권 발행사. 데이비드 호크니 디지털 페인팅 등 컨템포러리 미술품을 기초자산으로 발행. 2024.12 DART 정식 공시 시작.",
  sources: [
    { src: "DART", label: "DART 증권신고서 (corp_code 01708233)" },
    { src: "OFFICIAL", label: "artipio.com" },
  ],
};
const ISSUERS_FINAL = ISSUERS.some((i) => i.id === ARTIPIO.id)
  ? ISSUERS
  : [...ISSUERS, ARTIPIO];

window.STO_DATA = {
  CATEGORIES,
  SOURCES,
  STATUS_LABELS,
  ISSUERS: ISSUERS_FINAL,
  STOS: STOS_WITH_MUSICOW,
  ISSUER_SUMMARIES,
  TIMELINE,
  summariesByIssuer,
  findIssuer,
  stosByIssuer,
  categoryStats,
};
