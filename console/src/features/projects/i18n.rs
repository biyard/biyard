use dioxus_translate::{Translator, translate};

translate! {
    ProjectsTranslate;

    title: {
        en: "Brands",
        ko: "브랜드",
    },
    create_project: {
        en: "Create Brand",
        ko: "브랜드 생성",
    },
    create_new: {
        en: "Create New Brand",
        ko: "새 브랜드 생성",
    },
    create: {
        en: "Create",
        ko: "생성",
    },
    cancel: {
        en: "Cancel",
        ko: "취소",
    },
    name: {
        en: "Name",
        ko: "이름",
    },
    name_placeholder: {
        en: "Enter brand name",
        ko: "브랜드 이름 입력",
    },
    description: {
        en: "Description",
        ko: "설명",
    },
    description_placeholder: {
        en: "Enter description (optional)",
        ko: "설명 입력 (선택사항)",
    },
    monthly_supply: {
        en: "Monthly Token Supply",
        ko: "월별 토큰 공급량",
    },
    monthly_supply_placeholder: {
        en: "Enter monthly token supply (0 for manual provisioning)",
        ko: "월별 토큰 공급량 입력 (수동 프로비저닝은 0)",
    },
    symbol: {
        en: "Symbol",
        ko: "심볼",
    },
    symbol_placeholder: {
        en: "Enter token symbol (e.g., BTC, ETH)",
        ko: "토큰 심볼 입력 (예: BTC, ETH)",
    },
    decimals: {
        en: "Decimals",
        ko: "소수점",
    },
    decimals_placeholder: {
        en: "Enter decimals (0-18, default: 0)",
        ko: "소수점 입력 (0-18, 기본값: 0)",
    },
    status: {
        en: "Status",
        ko: "상태",
    },
    created_at: {
        en: "Created At",
        ko: "생성일",
    },
    updated_at: {
        en: "Updated At",
        ko: "수정일",
    },
    no_projects: {
        en: "No brands yet",
        ko: "브랜드가 없습니다",
    },
    no_projects_desc: {
        en: "Create your first brand to get started",
        ko: "첫 번째 브랜드를 생성하여 시작하세요",
    },
    loading: {
        en: "Loading...",
        ko: "로딩 중...",
    },
    loading_project: {
        en: "Loading brand...",
        ko: "브랜드 로딩 중...",
    },
    loading_transactions: {
        en: "Loading transactions...",
        ko: "트랜잭션 로딩 중...",
    },
    delete: {
        en: "Delete",
        ko: "삭제",
    },
    delete_confirm: {
        en: "Are you sure you want to delete this brand?",
        ko: "이 브랜드를 삭제하시겠습니까?",
    },
    delete_project: {
        en: "Delete Brand",
        ko: "브랜드 삭제",
    },
    back: {
        en: "Back",
        ko: "뒤로",
    },
    back_to_projects: {
        en: "Back to Brands",
        ko: "브랜드 목록",
    },
    overview: {
        en: "Overview",
        ko: "개요",
    },
    tokens: {
        en: "Token Transactions",
        ko: "토큰 트랜잭션",
    },
    points: {
        en: "Point History",
        ko: "포인트 히스토리",
    },
    settings_tab: {
        en: "Settings",
        ko: "설정",
    },
    project_info: {
        en: "Brand Information",
        ko: "브랜드 정보",
    },
    project_id: {
        en: "Brand ID",
        ko: "브랜드 ID",
    },
    token_info: {
        en: "Token",
        ko: "토큰",
    },
    point_info: {
        en: "Points",
        ko: "포인트",
    },
    point_aggregation: {
        en: "Point Aggregation",
        ko: "포인트 집계",
    },
    total_supply: {
        en: "Total Supply",
        ko: "총 공급량",
    },
    circulating_supply: {
        en: "Circulating Supply",
        ko: "유통 공급량",
    },
    supplied_points: {
        en: "Supplied Points",
        ko: "공급 포인트",
    },
    traded_points: {
        en: "Traded Points",
        ko: "거래 포인트",
    },
    awarded_points: {
        en: "Awarded Points",
        ko: "지급 포인트",
    },
    deducted_points: {
        en: "Deducted Points",
        ko: "차감 포인트",
    },
    exchanged_points: {
        en: "Exchanged Points",
        ko: "교환 포인트",
    },
    total_awarded: {
        en: "Total Awarded",
        ko: "총 지급량",
    },
    total_deducted: {
        en: "Total Deducted",
        ko: "총 차감량",
    },
    date: {
        en: "Date",
        ko: "날짜",
    },
    transactions: {
        en: "Point Transactions",
        ko: "포인트 거래 내역",
    },
    no_transactions: {
        en: "No transactions yet",
        ko: "트랜잭션이 없습니다",
    },
    no_transactions_desc: {
        en: "Point transactions will appear here.",
        ko: "포인트 트랜잭션이 여기에 표시됩니다.",
    },
    user_id: {
        en: "User",
        ko: "사용자",
    },
    r#type: {
        en: "Type",
        ko: "유형",
    },
    transaction_type: {
        en: "Type",
        ko: "유형",
    },
    amount: {
        en: "Amount",
        ko: "수량",
    },
    month: {
        en: "Month",
        ko: "월",
    },
    no_token: {
        en: "No tokens yet",
        ko: "토큰이 없습니다",
    },
    no_token_desc: {
        en: "Create your first token for this brand.",
        ko: "이 브랜드의 첫 번째 토큰을 생성하세요.",
    },
    create_token: {
        en: "Create Token",
        ko: "토큰 생성",
    },
    no_token_transactions: {
        en: "No token transactions yet",
        ko: "토큰 트랜잭션이 없습니다",
    },
    no_token_transactions_desc: {
        en: "Token transactions will appear here.",
        ko: "토큰 트랜잭션이 여기에 표시됩니다.",
    },
    project_name: {
        en: "Brand Name",
        ko: "브랜드 이름",
    },
    actions: {
        en: "Actions",
        ko: "작업",
    },
    save: {
        en: "Save",
        ko: "저장",
    },
    saving: {
        en: "Saving...",
        ko: "저장 중...",
    },
    creating: {
        en: "Creating...",
        ko: "생성 중...",
    },
    active: {
        en: "Active",
        ko: "활성",
    },
    inactive: {
        en: "Inactive",
        ko: "비활성",
    },
    not_found: {
        en: "Brand not found",
        ko: "브랜드를 찾을 수 없습니다",
    },
    not_found_desc: {
        en: "The brand you're looking for doesn't exist.",
        ko: "요청하신 브랜드가 존재하지 않습니다.",
    },
    no_points_yet: {
        en: "No points yet",
        ko: "포인트가 없습니다",
    },
    no_points_desc: {
        en: "Point transactions will appear here when created.",
        ko: "포인트 트랜잭션이 생성되면 여기에 표시됩니다.",
    },
    exchange_ratio: {
        en: "Exchange Ratio",
        ko: "교환 비율",
    },
    token_value: {
        en: "Token Value",
        ko: "토큰 가치",
    },
    settings_placeholder: {
        en: "Brand settings will be available here.",
        ko: "브랜드 설정이 여기에 제공될 예정입니다.",
    },
    danger_zone: {
        en: "Danger Zone",
        ko: "위험 영역",
    },
    token_symbol: {
        en: "Token Symbol",
        ko: "토큰 심볼",
    },
    token_decimals: {
        en: "Token Decimals",
        ko: "토큰 소수점",
    },
    token_name: {
        en: "Token Name",
        ko: "토큰 이름",
    },
    token_mint: {
        en: "Mint Token",
        ko: "토큰 발행",
    },
    target_user_id: {
        en: "Target User ID",
        ko: "대상 사용자 ID",
    },
    mint_amount: {
        en: "Mint Amount",
        ko: "발행 수량",
    },
    mint_description_placeholder: {
        en: "e.g. Meeting demo mint",
        ko: "예: 미팅 데모 발행",
    },
    validation_error: {
        en: "Please check the target user ID and mint amount.",
        ko: "대상 사용자 ID와 발행 수량을 확인해주세요.",
    },
    mint_success: {
        en: "Token minting completed.",
        ko: "토큰 발행이 완료되었습니다.",
    },
    mint_failure: {
        en: "Token minting failed: ",
        ko: "토큰 발행 실패: ",
    },
    minting: {
        en: "Minting...",
        ko: "발행 중...",
    },
    brand: {
        en: "Brand",
        ko: "브랜드",
    },
    brand_name: {
        en: "Brand Name",
        ko: "브랜드 이름",
    },
    brand_display_name: {
        en: "Display Name",
        ko: "브랜드명",
    },
    brand_display_name_desc: {
        en: "The display name shown in the service.",
        ko: "브랜드명은 서비스에 표시되는 이름입니다.",
    },
    brand_logo_url: {
        en: "Brand Logo URL",
        ko: "브랜드 로고 URL",
    },
    brand_settings: {
        en: "Brand Settings",
        ko: "브랜드 설정",
    },
    token_info_immutable: {
        en: "Token Info (Immutable)",
        ko: "토큰 정보 (불변)",
    },
    token_immutable_desc: {
        en: "Token name/symbol cannot be changed after creation. Total supply can only increase by minting.",
        ko: "토큰 이름/심볼은 생성 후 변경할 수 없습니다. 총 발행량은 토큰 발행으로만 증가합니다.",
    },
    treasury_reserve_rate: {
        en: "Treasury Reserve Rate",
        ko: "트레저리 적립률",
    },
    treasury_reserve_rate_desc: {
        en: "0.1 = 10% of revenue reserved in treasury",
        ko: "0.1 = 매출의 10%를 트레저리에 적립",
    },
    save_settings: {
        en: "Save Settings",
        ko: "설정 저장",
    },
    settings_saved: {
        en: "Settings saved.",
        ko: "설정이 저장되었습니다.",
    },
    save_failure: {
        en: "Save failed: ",
        ko: "저장 실패: ",
    },
    treasury_simulation: {
        en: "Treasury (Simulation)",
        ko: "트레저리(시뮬레이션)",
    },
    treasury_balance: {
        en: "Treasury Balance",
        ko: "트레저리 잔고",
    },
    simulated_sales_total: {
        en: "Cumulative Sales (Simulation)",
        ko: "누적 매출(시뮬레이션)",
    },
    estimated_floor_price: {
        en: "Floor Price (Est.)",
        ko: "하한가(예상)",
    },
    floor_price_formula: {
        en: "Floor Price (Est.) = Treasury Balance / Total Supply (simulation basis)",
        ko: "하한가(예상) = 트레저리 잔고 / 총 발행량 (시뮬레이션 기준)",
    },
    revenue_to_treasury_simulation: {
        en: "Revenue -> Treasury Simulation",
        ko: "매출 -> 트레저리 시뮬레이션",
    },
    revenue_input: {
        en: "Revenue Input",
        ko: "매출 입력",
    },
    revenue_input_placeholder: {
        en: "e.g. 10000000",
        ko: "예: 10000000",
    },
    apply_revenue: {
        en: "Apply Revenue",
        ko: "매출 반영",
    },
    applying: {
        en: "Applying...",
        ko: "반영 중...",
    },
    simulation_success: {
        en: "Revenue simulation applied.",
        ko: "매출 시뮬레이션이 반영되었습니다.",
    },
    simulation_failure: {
        en: "Simulation failed: ",
        ko: "시뮬레이션 실패: ",
    },
    current_treasury_balance: {
        en: "Current Treasury Balance: ",
        ko: "현재 트레저리 잔고: ",
    },
    cumulative_sales_label: {
        en: " | Cumulative Sales (Simulation): ",
        ko: " | 누적 매출(시뮬레이션): ",
    },
    floor_price_overview_note: {
        en: "Floor Price (Est.) is auto-calculated based on total supply in the Overview tab.",
        ko: "하한가(예상)는 Overview 탭에서 총 발행량 기준으로 자동 계산됩니다.",
    },
    initial_total_supply: {
        en: "Initial Total Supply",
        ko: "초기 총 발행량",
    },
    token_name_placeholder: {
        en: "e.g. LeMouton Token",
        ko: "예: LeMouton Token",
    },
    mint_confirm_title: {
        en: "Confirm Token Minting",
        ko: "토큰 발행 확인",
    },
    mint_confirm_message: {
        en: "Are you sure you want to mint tokens? This action cannot be undone.",
        ko: "토큰을 발행하시겠습니까? 이 작업은 되돌릴 수 없습니다.",
    },
    mint_confirm_target: {
        en: "Target User",
        ko: "대상 사용자",
    },
    mint_confirm_amount: {
        en: "Mint Amount",
        ko: "발행 수량",
    },
    confirm: {
        en: "Confirm",
        ko: "확인",
    },
    tx_hash: {
        en: "Tx Hash",
        ko: "트랜잭션 해시",
    },
    on_chain: {
        en: "On-chain",
        ko: "온체인",
    },
    deploy_token_on_chain: {
        en: "Deploy On-chain",
        ko: "온체인 배포",
    },
    deploying: {
        en: "Deploying...",
        ko: "배포 중...",
    },
    deployed: {
        en: "Deployed",
        ko: "배포됨",
    },
    not_deployed: {
        en: "Not Deployed",
        ko: "미배포",
    },
    contract_address: {
        en: "Contract Address",
        ko: "컨트랙트 주소",
    },
    chain: {
        en: "Chain",
        ko: "체인",
    },
    select_chain: {
        en: "Select Chain",
        ko: "체인 선택",
    },
    deploy_success: {
        en: "Token successfully deployed on-chain.",
        ko: "온체인에 토큰이 배포되었습니다.",
    },
    deploy_failure: {
        en: "Deployment failed: ",
        ko: "배포 실패: ",
    },
}
