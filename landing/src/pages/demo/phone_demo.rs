use dioxus::prelude::*;

use super::biyard_screens::*;
use super::screens::*;

/// Step types — each one is a scenario frame
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Step {
    LmHome,
    LmProduct,
    LmCheckout,
    ByReward,
    ByDashboard,
    ByFeed,
    ByDao,
    ByActivity,
}

/// Rich info per step — owner, description, Biyard tech, revenue, competitive edge, code
pub(super) struct StepInfo {
    pub owner: &'static str,
    pub description: &'static str,
    pub tech: &'static str,
    pub revenue: &'static str,
    pub competitive: &'static str,
    pub code: &'static str,
}

impl Step {
    fn label(&self) -> &'static str {
        match self {
            Step::LmHome => "1. 홈",
            Step::LmProduct => "2. 상품",
            Step::LmCheckout => "3. 결제",
            Step::ByReward => "4. 팬덤 레벨업",
            Step::ByDashboard => "5. 셰퍼드 대시보드",
            Step::ByFeed => "6. 팬덤 피드",
            Step::ByDao => "7. DAO 투표",
            Step::ByActivity => "8. 걷기 인증",
        }
    }
    fn is_biyard(&self) -> bool {
        matches!(
            self,
            Step::ByReward | Step::ByDashboard | Step::ByFeed | Step::ByDao | Step::ByActivity
        )
    }
    pub fn info(&self) -> StepInfo {
        match self {
            Step::LmHome => StepInfo {
                owner: "르무통 자체 앱",
                description: "기존 상품 리스트. Biyard는 건드리지 않음.",
                tech: "",
                revenue: "",
                competitive: "",
                code: "",
            },
            Step::LmProduct => StepInfo {
                owner: "르무통 자체 앱",
                description: "기존 상품 상세. Biyard는 '적립금' 표기만 '팬덤 지분'으로 바꾸도록 권고.",
                tech: "",
                revenue: "",
                competitive: "",
                code: "",
            },
            Step::LmCheckout => StepInfo {
                owner: "르무통 자체 앱",
                description: "기존 결제 플로우. Biyard는 결제 완료 이벤트만 webhook으로 수신.",
                tech: "",
                revenue: "",
                competitive: "",
                code: "",
            },
            Step::ByReward => StepInfo {
                owner: "Biyard Launchpad",
                description: "결제 완료 직후 Biyard SDK가 훅을 잡아 팬덤 레벨업 모달을 트리거. 기존 포인트 UX를 그대로 두되 프레이밍만 팬덤화.",
                tech: "\u{2022} Payment Webhook Listener (기존 결제 시스템 무관)\n\u{2022} RB-AMM Treasury 자동 적립 스마트 컨트랙트\n\u{2022} Fandom Level Engine (레벨/배지 규칙 엔진)\n\u{2022} 모바일 모달 컴포넌트 (RN/Native)",
                revenue: "\u{2022} 런칭 수수료 $5,000 (1회, 토큰 발행 + 셋업)\n\u{2022} 거래당 0.5~1% 수수료 (매출 규모에 따라 할인)\n\u{2022} 르무통 예상 연 거래 수수료: ~$12,000 (연 매출 20억 × 0.6%)",
                competitive: "\u{2022} vs. 일반 포인트 SaaS: 매출 연동 검증 불가 → 우리만 가능\n\u{2022} vs. NFT 마켓플레이스: 결제 연동이 없음 → 우리는 직접 훅 제공\n\u{2022} vs. Web3 로열티 스타트업: 기업 앱 수정 요구 → 우리는 webhook 한 줄",
                code: "await Biyard.recordPurchase({\n  userId,\n  amount: 129000,\n  brandId: 'lemouton',\n})\n// \u{2192} 트레저리 자동 적립 (RB-AMM)\n// \u{2192} 팬덤 지분 발행 (Floor=T\u{00F7}S 기반)\n// \u{2192} 레벨업 이벤트 브로드캐스트",
            },
            Step::ByDashboard => StepInfo {
                owner: "Biyard Launchpad",
                description: "셰퍼드 대시보드 — 나의 지분/레벨/랭킹, 팬덤 전체 가치. 마이페이지 영역에 iframe/컴포넌트로 임베드. 커스터마이징 가능.",
                tech: "\u{2022} Pre-built Wallet Dashboard 컴포넌트\n\u{2022} Real-time Treasury API (Floor = Treasury \u{00F7} Supply)\n\u{2022} Leveling & Ranking Engine (서버)\n\u{2022} Chart SDK (\u{200B}브랜드 테마 적용 가능)",
                revenue: "\u{2022} Business 플랜 월 $399에 포함 \u{2192} 구독료의 핵심 가치\n\u{2022} 테마 커스터마이징/커스텀 필드 별도 옵션 +$100~300/월\n\u{2022} AUM 수수료 연 0.3~0.5% (트레저리 자산 규모 × 운용료)",
                competitive: "\u{2022} vs. 자체 개발: 구축 6개월+ \u{2192} 우리는 즉시 임베드\n\u{2022} vs. Weverse: 엔터 전용 + 폐쇄형 \u{2192} 우리는 범용 + 오픈\n\u{2022} vs. 일반 로열티 UI: 스토리/자산 프레이밍 없음 \u{2192} 우리는 Web3 자산 프레이밍",
                code: "<BiyardDashboard\n  userId={user.id}\n  brand=\"lemouton\"\n  fandomName=\"Shepherd\"\n  theme={lemoutonTheme}\n  showRanking\n  showDAOBadges\n/>",
            },
            Step::ByFeed => StepInfo {
                owner: "Biyard Launchpad",
                description: "팬덤 커뮤니티 피드. 네이버 카페 + 위버스 구조를 완성된 앱에 모듈로 삽입. 포스팅/댓글/좋아요/모더레이션 전부 포함.",
                tech: "\u{2022} Social Feed Module (포스트/댓글/좋아요)\n\u{2022} AI 모더레이션 + 신고 시스템\n\u{2022} Push/Email Notification Engine\n\u{2022} Activity-based Reward Distribution (포스팅 50, 좋아요 5 등)",
                revenue: "\u{2022} Community Module 프리미엄 애드온 +$149/월\n\u{2022} 프리미엄 관리자 도구(AI 모더레이션 대시보드) +$99/월\n\u{2022} 커스텀 이모지/뱃지 발행 건당 $X",
                competitive: "\u{2022} vs. 네이버 카페: 자산화/온체인 없음 \u{2192} 우리는 기여도=지분\n\u{2022} vs. Discord 서버: 브랜드 앱 외부 \u{2192} 우리는 앱 내 임베드\n\u{2022} vs. Weverse: 엔터만 입점 \u{2192} 우리는 D2C 모든 업종",
                code: "<BiyardFandomFeed\n  brand=\"lemouton\"\n  moderationLevel=\"standard\"\n  rewardRules={{\n    post: 50,\n    like: 5,\n    comment: 20,\n  }}\n/>",
            },
            Step::ByDao => StepInfo {
                owner: "Biyard Launchpad",
                description: "DAO 투표 — 팬덤이 신상 컬러/친환경 정책 등을 직접 결정. 기업은 공짜 인사이트, 팬은 소속감. 참여 시 팬덤 지분 자동 지급.",
                tech: "\u{2022} On-chain Voting Smart Contract (가스비 최적화)\n\u{2022} Proposal Creation UI (브랜드 관리자)\n\u{2022} Gating Logic (Lv.N 이상만 참여)\n\u{2022} Result Aggregation + 실시간 집계",
                revenue: "\u{2022} DAO Module 프리미엄 애드온 +$199/월\n\u{2022} 투표 1건당 가스비 차익 (배치 처리 최적화로 마진 확보)\n\u{2022} 대형 브랜드 전용 Custom Voting 룰 제작비 $3K~10K",
                competitive: "\u{2022} vs. Snapshot(일반 DAO툴): 기업 UX 미고려 \u{2192} 우리는 B2B 패키지\n\u{2022} vs. Aragon: 개발자용 \u{2192} 우리는 노코드 관리자 UI\n\u{2022} vs. 투표 SaaS: 온체인 아님 \u{2192} 우리는 검증 가능한 온체인",
                code: "<BiyardDAO\n  brand=\"lemouton\"\n  minLevel={2}\n  rewardForVoting={20}\n/>\n\n// Admin side:\nawait Biyard.createProposal({\n  title: '2025 F/W 컬러',\n  options: ['샌드', '오크', '올리브'],\n  gating: {{ minLevel: 2 }},\n})",
            },
            Step::ByActivity => StepInfo {
                owner: "Biyard Launchpad",
                description: "활동 인증 — 신발을 신고 걸을수록 팬덤 지분이 자람. HealthKit/Google Fit 연동 + 이상 패턴 감지. 르무통 특유의 '걷기 브랜드' 서사 강화.",
                tech: "\u{2022} Activity Verification Engine\n\u{2022} HealthKit / Google Fit 브릿지\n\u{2022} Anti-fraud Detection (비정상 걸음 패턴 ML 감지)\n\u{2022} Rule Engine (걷기/방문/공유 등 커스텀)",
                revenue: "\u{2022} Activity Module 프리미엄 애드온 +$99/월\n\u{2022} Fraud Detection 고급 옵션 +$49/월\n\u{2022} 오프라인 위치 인증 (매장 방문) +$79/월",
                competitive: "\u{2022} vs. 삼성헬스/캐시워크: 자체 앱만 동작 \u{2192} 우리는 SDK\n\u{2022} vs. 브랜드 자체 구현: 부정 감지 어려움 \u{2192} 우리는 ML 탑재\n\u{2022} vs. 걷기 토큰 앱(StepN 등): 브랜드 무관 \u{2192} 우리는 브랜드별",
                code: "await Biyard.verifyActivity({\n  userId,\n  type: 'walking',\n  steps: 8412,\n  source: 'healthkit',\n})\n// \u{2192} Fraud Check (ML)\n// \u{2192} Rule Engine 의 리워드 계산\n// \u{2192} 팬덤 지분 자동 지급",
            },
        }
    }
}

pub(super) const ALL_STEPS: [Step; 8] = [
    Step::LmHome,
    Step::LmProduct,
    Step::LmCheckout,
    Step::ByReward,
    Step::ByDashboard,
    Step::ByFeed,
    Step::ByDao,
    Step::ByActivity,
];

#[component]
pub(super) fn PhoneDemo() -> Element {
    let mut current = use_signal(|| Step::LmHome);

    rsx! {
        section {
            class: "demo-section",
            style: "padding-top: 60px; padding-bottom: 60px;",
            div {
                class: "demo-container",

                // Minimal header
                div { style: "margin-bottom: 20px;",
                    div { style: "display: flex; gap: 10px; flex-wrap: wrap; margin-bottom: 10px;",
                        span { class: "legend-pill legend-lm", "\u{1F90E} 르무통 자체 개발 (이미 완료)" }
                        span { class: "legend-pill legend-by", "\u{2B21} Biyard Launchpad 제공" }
                    }
                    p { style: "color: #64748b; font-size: 12px;",
                        "각 스텝 클릭 \u{2192} 우측에 제공 기술 · 수익 · 경쟁력 · 통합 코드 표시"
                    }
                }

                // Step controller
                div { class: "step-controller",
                    for idx in 0..ALL_STEPS.len() {
                        {
                            let step = ALL_STEPS[idx];
                            let active = *current.read() == step;
                            let lm_class = if step.is_biyard() { "" } else { "lm-step " };
                            let active_class = if active { "active" } else { "" };
                            rsx! {
                                button {
                                    key: "{idx}",
                                    class: "step-btn {lm_class}{active_class}",
                                    onclick: move |_| current.set(ALL_STEPS[idx]),
                                    "{step.label()}"
                                }
                            }
                        }
                    }
                }

                // Phone + annotation split
                div { style: "display: grid; grid-template-columns: auto 1fr; gap: 40px; align-items: start; margin-top: 16px;",

                    // Phone
                    div { style: "display: flex; justify-content: center;",
                        div { class: "phone-frame",
                            div { class: "phone-notch" }
                            div { class: "phone-screen",
                                {
                                    match *current.read() {
                                        Step::LmHome => rsx! { LmHomeScreen {} },
                                        Step::LmProduct => rsx! { LmProductScreen {} },
                                        Step::LmCheckout => rsx! { LmCheckoutScreen {} },
                                        Step::ByReward => rsx! { BiyardFandomReward {} },
                                        Step::ByDashboard => rsx! { BiyardDashboard {} },
                                        Step::ByFeed => rsx! { BiyardFandomFeed {} },
                                        Step::ByDao => rsx! { BiyardDao {} },
                                        Step::ByActivity => rsx! { BiyardActivity {} },
                                    }
                                }
                            }
                        }
                    }

                    // Annotation panel
                    {
                        let step = *current.read();
                        let info = step.info();
                        let is_by = step.is_biyard();
                        let owner_color = if is_by { "#00dfc0" } else { "#D4C5B0" };
                        let owner_bg = if is_by { "rgba(0,223,192,0.06)" } else { "rgba(212,197,176,0.06)" };
                        let owner_border = if is_by { "rgba(0,223,192,0.2)" } else { "rgba(212,197,176,0.2)" };
                        rsx! {
                            div { style: "padding: 24px; background: rgba(255,255,255,0.02); border: 1px solid rgba(255,255,255,0.06); border-radius: 18px; position: sticky; top: 100px;",
                                div { style: "padding: 6px 12px; display: inline-block; background: {owner_bg}; border: 1px solid {owner_border}; color: {owner_color}; font-size: 10px; font-weight: 900; letter-spacing: 0.3em; text-transform: uppercase; border-radius: 999px; margin-bottom: 14px;",
                                    "{info.owner}"
                                }
                                h3 { style: "font-size: 22px; font-weight: 900; color: #fff; margin-bottom: 12px;", "{step.label()}" }
                                p { style: "color: #94a3b8; font-size: 13px; line-height: 1.7; margin-bottom: 18px;", "{info.description}" }

                                if is_by {
                                    // Tech stack
                                    div { style: "padding: 14px 16px; background: rgba(0,223,192,0.04); border: 1px solid rgba(0,223,192,0.2); border-radius: 12px; margin-bottom: 10px;",
                                        p { style: "font-size: 10px; color: #00dfc0; font-weight: 900; letter-spacing: 0.2em; margin-bottom: 10px;", "\u{2699}\u{FE0F} 제공 기술" }
                                        pre { style: "font-size: 12px; color: #cbd5e1; line-height: 1.8; white-space: pre-wrap; word-break: break-word; margin: 0; font-family: 'Noto Sans KR', sans-serif;", "{info.tech}" }
                                    }
                                    // Revenue
                                    div { style: "padding: 14px 16px; background: rgba(251,191,36,0.06); border: 1px solid rgba(251,191,36,0.25); border-radius: 12px; margin-bottom: 10px;",
                                        p { style: "font-size: 10px; color: #fbbf24; font-weight: 900; letter-spacing: 0.2em; margin-bottom: 10px;", "\u{1F4B0} 수익 포인트" }
                                        pre { style: "font-size: 12px; color: #e2e8f0; line-height: 1.8; white-space: pre-wrap; word-break: break-word; margin: 0; font-family: 'Noto Sans KR', sans-serif; font-weight: 600;", "{info.revenue}" }
                                    }
                                    // Competitive
                                    div { style: "padding: 14px 16px; background: rgba(167,139,250,0.06); border: 1px solid rgba(167,139,250,0.25); border-radius: 12px; margin-bottom: 10px;",
                                        p { style: "font-size: 10px; color: #a78bfa; font-weight: 900; letter-spacing: 0.2em; margin-bottom: 10px;", "\u{1F3AF} 경쟁력" }
                                        pre { style: "font-size: 12px; color: #cbd5e1; line-height: 1.8; white-space: pre-wrap; word-break: break-word; margin: 0; font-family: 'Noto Sans KR', sans-serif;", "{info.competitive}" }
                                    }
                                    // Code
                                    div { style: "padding: 12px 14px; background: rgba(0,0,0,0.5); border: 1px solid rgba(255,255,255,0.06); border-radius: 10px;",
                                        p { style: "font-size: 10px; color: #64748b; font-weight: 900; letter-spacing: 0.2em; margin-bottom: 8px;", "\u{2261} 통합 코드" }
                                        pre { style: "font-family: 'SF Mono', Menlo, monospace; font-size: 11px; color: #cbd5e1; line-height: 1.6; white-space: pre-wrap; word-break: break-word; margin: 0;",
                                            "{info.code}"
                                        }
                                    }
                                } else {
                                    div { style: "padding: 14px 16px; background: rgba(212,197,176,0.04); border: 1px solid rgba(212,197,176,0.15); border-radius: 12px;",
                                        p { style: "font-size: 12px; color: #D4C5B0; line-height: 1.8;",
                                            "\u{2139}\u{FE0F} 르무통이 이미 개발한 영역. Biyard는 수정 없음, 기업 부담 없음."
                                        }
                                    }
                                }

                                // Nav
                                div { style: "display: flex; gap: 8px; margin-top: 16px;",
                                    button {
                                        style: "flex: 1; padding: 10px; background: rgba(255,255,255,0.04); border: 1px solid rgba(255,255,255,0.1); border-radius: 8px; color: #94a3b8; font-size: 12px; cursor: pointer;",
                                        onclick: move |_| {
                                            let cur = *current.read();
                                            let idx = ALL_STEPS.iter().position(|s| *s == cur).unwrap_or(0);
                                            if idx > 0 {
                                                current.set(ALL_STEPS[idx - 1]);
                                            }
                                        },
                                        "\u{2190} 이전"
                                    }
                                    button {
                                        style: "flex: 1; padding: 10px; background: #00dfc0; color: #020408; border: none; border-radius: 8px; font-size: 12px; font-weight: 900; cursor: pointer;",
                                        onclick: move |_| {
                                            let cur = *current.read();
                                            let idx = ALL_STEPS.iter().position(|s| *s == cur).unwrap_or(0);
                                            if idx < ALL_STEPS.len() - 1 {
                                                current.set(ALL_STEPS[idx + 1]);
                                            }
                                        },
                                        "다음 \u{2192}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
