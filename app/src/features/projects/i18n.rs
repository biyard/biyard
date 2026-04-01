use dioxus_translate::{Translator, translate};

translate! {
    ProjectsTranslate;

    title: {
        en: "Projects",
        ko: "프로젝트",
    },
    create_project: {
        en: "Create Project",
        ko: "프로젝트 생성",
    },
    create_new: {
        en: "Create New Project",
        ko: "새 프로젝트 생성",
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
        en: "Enter project name",
        ko: "프로젝트 이름 입력",
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
        en: "No projects yet",
        ko: "프로젝트가 없습니다",
    },
    no_projects_desc: {
        en: "Create your first project to get started",
        ko: "첫 번째 프로젝트를 생성하여 시작하세요",
    },
    loading: {
        en: "Loading...",
        ko: "로딩 중...",
    },
    loading_project: {
        en: "Loading project...",
        ko: "프로젝트 로딩 중...",
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
        en: "Are you sure you want to delete this project?",
        ko: "이 프로젝트를 삭제하시겠습니까?",
    },
    delete_project: {
        en: "Delete Project",
        ko: "프로젝트 삭제",
    },
    back: {
        en: "Back",
        ko: "뒤로",
    },
    back_to_projects: {
        en: "Back to Projects",
        ko: "프로젝트 목록",
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
        en: "Project Information",
        ko: "프로젝트 정보",
    },
    project_id: {
        en: "Project ID",
        ko: "프로젝트 ID",
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
        en: "Create your first token for this project.",
        ko: "이 프로젝트의 첫 번째 토큰을 생성하세요.",
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
        en: "Project Name",
        ko: "프로젝트 이름",
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
        en: "Project not found",
        ko: "프로젝트를 찾을 수 없습니다",
    },
    not_found_desc: {
        en: "The project you're looking for doesn't exist.",
        ko: "요청하신 프로젝트가 존재하지 않습니다.",
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
        en: "Project settings will be available here.",
        ko: "프로젝트 설정이 여기에 제공될 예정입니다.",
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
}
