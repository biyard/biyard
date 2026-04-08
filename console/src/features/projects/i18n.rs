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
    brand_profile: {
        en: "Brand profile",
        ko: "브랜드 프로필",
    },
    viewer_readonly_notice: {
        en: "You have view-only access. Ask an admin to make changes on your behalf.",
        ko: "읽기 전용 권한입니다. 변경이 필요하면 관리자에게 요청하세요.",
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
        en: "e.g. 1,000,000 — set to 0 to provision manually",
        ko: "예: 1,000,000 — 수동 프로비저닝은 0",
    },
    monthly_supply_help: {
        en: "Per-month budget for rewarding users with points convertible to this brand's token. Independent of the token's on-chain total supply (set on the Token tab).",
        ko: "이 브랜드의 토큰으로 전환 가능한 포인트를 사용자에게 지급할 월별 예산입니다. 토큰의 온체인 총 공급량(토큰 탭에서 설정)과는 별개입니다.",
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
    delete_brand_warning: {
        en: "This action cannot be undone. All tokens, points, and history tied to this brand will be permanently removed.",
        ko: "이 작업은 되돌릴 수 없습니다. 이 브랜드에 연결된 모든 토큰, 포인트, 기록이 영구적으로 삭제됩니다.",
    },
    delete_brand_confirm_prompt: {
        en: "To confirm, type the brand name below.",
        ko: "삭제를 확인하려면 아래에 브랜드 이름을 입력하세요.",
    },
    delete_brand_confirm_placeholder: {
        en: "Type brand name to confirm",
        ko: "확인을 위해 브랜드 이름 입력",
    },
    delete_brand_mismatch: {
        en: "Brand name does not match.",
        ko: "브랜드 이름이 일치하지 않습니다.",
    },
    delete_brand_button: {
        en: "Permanently delete brand",
        ko: "브랜드 영구 삭제",
    },
    deleting: {
        en: "Deleting...",
        ko: "삭제 중...",
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
        en: "Token",
        ko: "토큰",
    },
    points: {
        en: "Point",
        ko: "포인트",
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
    transactions_subtitle: {
        en: "Recent point history across awards, deductions, exchanges, and transfers.",
        ko: "최근 포인트 지급 / 차감 / 교환 / 이체 내역입니다.",
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
    token_load_error: {
        en: "Could not load token information. Please refresh and try again.",
        ko: "토큰 정보를 불러오지 못했습니다. 새로고침 후 다시 시도해주세요.",
    },
    point_load_error: {
        en: "Could not load point activity. Please refresh and try again.",
        ko: "포인트 활동을 불러오지 못했습니다. 새로고침 후 다시 시도해주세요.",
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
    target_user_id_placeholder: {
        en: "User ID or wallet address",
        ko: "사용자 ID 또는 지갑 주소",
    },
    mint_requires_deploy: {
        en: "Deploy the token contract first to mint tokens to addresses.",
        ko: "토큰 컨트랙트를 먼저 배포해야 토큰을 발행할 수 있습니다.",
    },
    deploy_token_confirm_title: {
        en: "Deploy token to chain?",
        ko: "토큰을 체인에 배포할까요?",
    },
    deploy_treasury_confirm_title: {
        en: "Deploy treasury contract?",
        ko: "트레저리 컨트랙트를 배포할까요?",
    },
    deploy_confirm_message: {
        en: "This will publish the contract to the selected blockchain network.",
        ko: "선택한 블록체인 네트워크에 컨트랙트를 게시합니다.",
    },
    deploy_confirm_irreversible_title: {
        en: "This action cannot be undone",
        ko: "이 작업은 되돌릴 수 없습니다",
    },
    deploy_confirm_irreversible_body: {
        en: "Once deployed, the token name, symbol, decimals, and initial supply are locked on-chain. Gas fees apply.",
        ko: "한번 배포되면 토큰 이름, 심볼, decimals, 초기 공급량은 온체인에서 잠깁니다. 가스비가 발생합니다.",
    },
    deploy_confirm_acknowledge: {
        en: "I understand this deploys to the blockchain and cannot be undone.",
        ko: "이 작업이 블록체인에 배포되고 되돌릴 수 없음을 이해합니다.",
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
    brand_logo: {
        en: "Brand Logo",
        ko: "브랜드 로고",
    },
    brand_logo_upload_cta: {
        en: "Drop an image or click to upload",
        ko: "이미지를 끌어다 놓거나 클릭하여 업로드",
    },
    brand_logo_upload_hint: {
        en: "PNG or JPG, recommended 512×512",
        ko: "PNG 또는 JPG, 권장 크기 512×512",
    },
    brand_logo_change_cta: {
        en: "Click to replace the current logo",
        ko: "클릭하여 로고 교체",
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
        en: "Share of revenue routed to treasury reserves to back token value. Higher rates improve token stability at the cost of operating cash.",
        ko: "매출의 일부를 트레저리에 적립하여 토큰 가치를 뒷받침합니다. 값이 높을수록 토큰 안정성이 커지지만 운영 자금은 줄어듭니다.",
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
    treasury_overview: {
        en: "Treasury Overview",
        ko: "트레저리 개요",
    },
    treasury_page_placeholder: {
        en: "Treasury data is being rebuilt around on-chain balances and a real sales ledger. Live status and manual entry tools land next.",
        ko: "트레저리 데이터는 온체인 잔고와 실제 매출 로그 기반으로 재설계 중입니다. 실시간 상태와 수동 입력 도구가 곧 추가됩니다.",
    },
    treasury_not_deployed: {
        en: "Treasury contracts are not deployed yet. Deploy the brand token from the Tokens tab to see the live on-chain treasury.",
        ko: "트레저리 컨트랙트가 아직 배포되지 않았습니다. 토큰 탭에서 브랜드 토큰을 배포하면 온체인 트레저리 상태를 확인할 수 있습니다.",
    },
    treasury_onchain_title: {
        en: "On-Chain Treasury Status",
        ko: "온체인 트레저리 현황",
    },
    treasury_onchain_balance: {
        en: "Treasury Balance",
        ko: "트레저리 잔고",
    },
    treasury_onchain_floor: {
        en: "Floor Price",
        ko: "하한가",
    },
    treasury_onchain_circulating: {
        en: "Circulating Supply",
        ko: "유통 공급량",
    },
    treasury_onchain_total_supply: {
        en: "Total Supply",
        ko: "총 발행량",
    },
    treasury_contract_address_label: {
        en: "Treasury contract:",
        ko: "트레저리 컨트랙트:",
    },
    open_simulator: {
        en: "Open Floor Price Simulator",
        ko: "하한가 시뮬레이터 열기",
    },
    open_sales_log: {
        en: "Open Sales Log",
        ko: "매출 로그 열기",
    },
    close: {
        en: "Close",
        ko: "닫기",
    },
    // --- Floor Price Simulator (Dialog A) ---
    simulator_title: {
        en: "Floor Price Simulator",
        ko: "하한가 시뮬레이터",
    },
    simulator_subtitle: {
        en: "What-if tool. Nothing here is saved to the database or the blockchain — it only shows how the floor price mechanism reacts to sales, reward mints, and redemptions.",
        ko: "가상 시나리오 도구입니다. 입력값은 DB나 블록체인에 저장되지 않으며, 매출·리워드 발행·환매에 따라 하한가가 어떻게 변하는지 보여줄 뿐입니다.",
    },
    simulator_config_title: {
        en: "Configuration",
        ko: "설정",
    },
    simulator_reserve_rate: {
        en: "Reserve Rate",
        ko: "트레저리 적립률",
    },
    simulator_action_sale_title: {
        en: "Record Sale",
        ko: "매출 입력",
    },
    simulator_action_sale_hint: {
        en: "Adds amount × reserve rate to treasury.",
        ko: "매출 × 적립률만큼 트레저리가 증가합니다.",
    },
    simulator_apply_sale: {
        en: "Add Sale",
        ko: "매출 반영",
    },
    simulator_action_mint_title: {
        en: "Reward Mint",
        ko: "리워드 발행",
    },
    simulator_action_mint_hint: {
        en: "Mints tokens without adding treasury. Dilutes floor.",
        ko: "트레저리 증가 없이 토큰만 발행합니다. 하한가가 희석됩니다.",
    },
    simulator_apply_mint: {
        en: "Mint",
        ko: "발행",
    },
    simulator_action_redeem_title: {
        en: "Redeem (Buyback)",
        ko: "환매",
    },
    simulator_action_redeem_hint: {
        en: "Burns tokens at the floor price. Floor price is preserved.",
        ko: "하한가로 토큰을 소각합니다. 하한가는 유지됩니다.",
    },
    simulator_apply_redeem: {
        en: "Redeem",
        ko: "환매",
    },
    simulator_log_title: {
        en: "Action Log",
        ko: "액션 로그",
    },
    simulator_log_empty: {
        en: "No actions yet. Try recording a sale above.",
        ko: "아직 액션이 없습니다. 위에서 매출을 입력해보세요.",
    },
    simulator_log_col_action: {
        en: "Action",
        ko: "액션",
    },
    simulator_log_col_amount: {
        en: "Input",
        ko: "입력",
    },
    simulator_log_col_delta: {
        en: "Δ Treasury",
        ko: "Δ 트레저리",
    },
    simulator_log_col_treasury: {
        en: "Treasury",
        ko: "트레저리",
    },
    simulator_log_col_supply: {
        en: "Supply",
        ko: "발행량",
    },
    simulator_log_col_floor: {
        en: "Floor",
        ko: "하한가",
    },
    simulator_log_kind_sale: {
        en: "Sale",
        ko: "매출",
    },
    simulator_log_kind_mint: {
        en: "Mint",
        ko: "발행",
    },
    simulator_log_kind_redeem: {
        en: "Redeem",
        ko: "환매",
    },
    simulator_reset: {
        en: "Reset",
        ko: "초기화",
    },
    // --- Sales Log Dialog (Dialog B) ---
    sales_log_title: {
        en: "Sales Log",
        ko: "매출 로그",
    },
    sales_log_subtitle: {
        en: "Real sales ledger. Each row is a recorded sale and is persisted to the database. Normal production flow is the brand's POS calling the sales log API directly — this dialog is for manual onboarding and demos.",
        ko: "실제 매출 이력입니다. 모든 행이 DB에 저장됩니다. 정상 운영에서는 브랜드 POS가 API를 직접 호출하며, 이 다이얼로그는 수동 입력이나 시연용입니다.",
    },
    sales_log_count_label: {
        en: "Rows",
        ko: "건수",
    },
    sales_log_total_label: {
        en: "Total Amount",
        ko: "합계",
    },
    sales_log_add_title: {
        en: "Add Sale",
        ko: "매출 추가",
    },
    sales_log_amount_label: {
        en: "Amount",
        ko: "금액",
    },
    sales_log_amount_placeholder: {
        en: "e.g. 129000",
        ko: "예: 129000",
    },
    sales_log_amount_invalid: {
        en: "Enter a positive amount.",
        ko: "양의 금액을 입력해주세요.",
    },
    sales_log_memo_label: {
        en: "Memo (optional)",
        ko: "메모 (선택)",
    },
    sales_log_memo_placeholder: {
        en: "e.g. Order #123",
        ko: "예: 주문 #123",
    },
    sales_log_add_button: {
        en: "Add",
        ko: "추가",
    },
    sales_log_submitting: {
        en: "Adding...",
        ko: "추가 중...",
    },
    sales_log_add_success: {
        en: "Sale added.",
        ko: "매출이 추가되었습니다.",
    },
    sales_log_add_failure: {
        en: "Failed to add sale: ",
        ko: "매출 추가 실패: ",
    },
    sales_log_list_title: {
        en: "Recent Sales",
        ko: "최근 매출",
    },
    sales_log_empty: {
        en: "No sales recorded yet.",
        ko: "아직 기록된 매출이 없습니다.",
    },
    sales_log_col_created_at: {
        en: "Time",
        ko: "시각",
    },
    sales_log_col_amount: {
        en: "Amount",
        ko: "금액",
    },
    sales_log_col_memo: {
        en: "Memo",
        ko: "메모",
    },
    brands_breadcrumb: {
        en: "Brands",
        ko: "브랜드",
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
        en: "Revenue → Treasury Simulation",
        ko: "매출 → 트레저리 시뮬레이션",
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
    token_name_placeholder: {
        en: "e.g. My Token",
        ko: "예: 마이 토큰",
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
        en: "Deploy Contract",
        ko: "컨트랙트 배포",
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
    token_only: {
        en: "Token Only",
        ko: "토큰만 배포됨",
    },
    contract_address: {
        en: "Contract Address",
        ko: "컨트랙트 주소",
    },
    treasury_contract_address: {
        en: "Treasury Contract",
        ko: "트레저리 컨트랙트",
    },
    stable_token_address: {
        en: "Stable Token",
        ko: "스테이블 토큰",
    },
    treasury_deployment_tx_hash: {
        en: "Treasury Tx Hash",
        ko: "트레저리 트랜잭션 해시",
    },
    chain: {
        en: "Chain",
        ko: "체인",
    },
    select_chain: {
        en: "Select Chain",
        ko: "블록체인 선택",
    },
    deploy_stack_note: {
        en: "Deploy the token contract and floor-price treasury together, wired to a stable reserve on the selected chain.",
        ko: "선택한 체인에 토큰 컨트랙트와 하한가 트레저리를 함께 배포하고 스테이블 준비금을 연결합니다.",
    },
    complete_treasury_note: {
        en: "This brand already has a token contract. Complete the on-chain setup by deploying the missing floor-price treasury on the same chain.",
        ko: "이 브랜드에는 이미 토큰 컨트랙트가 있습니다. 같은 체인에 누락된 하한가 트레저리를 배포해 온체인 구성을 마무리하세요.",
    },
    deploy_treasury_on_chain: {
        en: "Deploy Treasury",
        ko: "트레저리 배포",
    },
    deploy_success: {
        en: "Token and floor-price treasury successfully deployed on-chain.",
        ko: "온체인에 토큰과 하한가 트레저리가 함께 배포되었습니다.",
    },
    treasury_deploy_success: {
        en: "Floor-price treasury successfully deployed and linked to the existing token.",
        ko: "기존 토큰에 연결된 하한가 트레저리가 배포되었습니다.",
    },
    deploy_failure: {
        en: "Deployment failed: ",
        ko: "배포 실패: ",
    },
    edit_brand: {
        en: "Edit Brand",
        ko: "브랜드 편집",
    },
    edit_token: {
        en: "Edit Token",
        ko: "토큰 편집",
    },
    save_token: {
        en: "Save Token",
        ko: "토큰 저장",
    },
    token_saved: {
        en: "Token saved.",
        ko: "토큰이 저장되었습니다.",
    },
    token_required_fields: {
        en: "Please fill in token name, symbol, decimals, and initial supply.",
        ko: "토큰 이름, 심볼, 소수점 자릿수, 초기 발행량을 모두 입력해주세요.",
    },
    create_project_subtitle: {
        en: "Define the brand identity and treasury defaults. You can configure the token in the next step.",
        ko: "브랜드 아이덴티티와 트레저리 기본값을 정의하세요. 토큰은 다음 단계에서 설정할 수 있습니다.",
    },
    edit_project_subtitle: {
        en: "Update your brand identity and treasury defaults.",
        ko: "브랜드 아이덴티티와 트레저리 기본값을 수정합니다.",
    },
    create_brand_subtitle_in: {
        en: "Create a new brand inside {enterprise}.",
        ko: "{enterprise} 에 새 브랜드를 생성합니다.",
    },
    edit_brand_subtitle_in: {
        en: "Update brand settings inside {enterprise}.",
        ko: "{enterprise} 의 브랜드 설정을 수정합니다.",
    },
    brands_page_subtitle_in: {
        en: "Monitor brand health, treasury posture, and launch readiness across {enterprise}.",
        ko: "{enterprise} 전반의 브랜드 상태, 트레저리, 출시 준비 현황을 확인하세요.",
    },
    create_token_subtitle: {
        en: "Configure the token that will represent value for this brand. You can come back later if you skip this step.",
        ko: "이 브랜드의 가치를 나타낼 토큰을 설정하세요. 이 단계를 건너뛰어도 나중에 다시 설정할 수 있습니다.",
    },
    edit_token_subtitle: {
        en: "Token metadata can be edited until the token is deployed on-chain.",
        ko: "토큰 메타데이터는 온체인 배포 전까지 수정할 수 있습니다.",
    },
    token_create_helper: {
        en: "Token name, symbol, and supply can be changed until the token is deployed on-chain.",
        ko: "토큰 이름, 심볼, 발행량은 온체인 배포 전까지 변경할 수 있습니다.",
    },
    token_edit_helper: {
        en: "These values are locked once the token is deployed on-chain.",
        ko: "온체인에 배포되면 이 값들은 잠깁니다.",
    },
    brand_create_helper: {
        en: "Launch with the brand profile and treasury defaults already configured.",
        ko: "브랜드 프로필과 트레저리 기본값을 한 번에 설정합니다.",
    },
    brand_edit_helper: {
        en: "Update your brand identity and treasury defaults from this page.",
        ko: "이 페이지에서 브랜드 아이덴티티와 트레저리 기본값을 수정합니다.",
    },
    brand_preview_description_placeholder: {
        en: "Add a short description to define the brand story and operating context.",
        ko: "브랜드 스토리와 운영 컨텍스트를 정의하는 짧은 설명을 추가하세요.",
    },
    next_create_token: {
        en: "Create Brand & Continue",
        ko: "브랜드 생성 후 계속",
    },
    skip_for_now: {
        en: "Skip for Now",
        ko: "나중에 설정",
    },
    back_to_brand: {
        en: "Back to Brand",
        ko: "브랜드로 돌아가기",
    },
    token_already_exists: {
        en: "Token Already Exists",
        ko: "토큰이 이미 존재합니다",
    },
    token_already_exists_subtitle: {
        en: "This brand already has a token configured. You can edit it until it is deployed.",
        ko: "이 브랜드에는 이미 토큰이 설정되어 있습니다. 배포 전까지 수정할 수 있습니다.",
    },
    token_locked_title: {
        en: "Token Locked",
        ko: "토큰 잠김",
    },
    token_locked_subtitle: {
        en: "This token has been deployed on-chain and can no longer be edited.",
        ko: "이 토큰은 온체인에 배포되어 더 이상 수정할 수 없습니다.",
    },
    live_preview: {
        en: "Live preview",
        ko: "실시간 미리보기",
    },
    operating_defaults: {
        en: "Operating defaults",
        ko: "운영 기본값",
    },
    sort_newest_first: {
        en: "Newest first",
        ko: "최신순",
    },
    sort_oldest_first: {
        en: "Oldest first",
        ko: "오래된순",
    },
    load_more: {
        en: "Load more",
        ko: "더 보기",
    },
    loading_more: {
        en: "Loading more...",
        ko: "더 불러오는 중...",
    },
}
