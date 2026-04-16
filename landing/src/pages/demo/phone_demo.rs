use dioxus::prelude::*;

use super::biyard_screens::*;
use super::screens::*;

/// Step types — each one is a scenario frame
#[derive(Clone, Copy, PartialEq)]
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
    fn description(&self) -> (&'static str, &'static str) {
        match self {
            Step::LmHome => ("르무통 자체 앱", "기존 상품 리스트와 브랜드 경험. 변경 없음."),
            Step::LmProduct => ("르무통 자체 앱", "기존 상품 상세 페이지. 가격, 적립금 안내 등."),
            Step::LmCheckout => ("르무통 자체 앱", "기존 결제 플로우. 여기까지는 르무통이 이미 개발 완료."),
            Step::ByReward => (
                "Biyard Launchpad",
                "결제 완료 후, 우리가 제공하는 팬덤 레벨업 모달이 트리거됩니다. 기존 '적립금 3,870원' 대신 '팬덤 지분 +3,870' 으로 프레이밍.",
            ),
            Step::ByDashboard => (
                "Biyard Launchpad",
                "셰퍼드 대시보드 — 나의 팬덤 지분, 레벨, 팬덤 전체 가치, 랭킹. '르무통 셰퍼드'는 이 앱에서 공식 팬덤 명칭.",
            ),
            Step::ByFeed => (
                "Biyard Launchpad",
                "팬덤 커뮤니티 피드. 네이버 카페 + BTS 위버스처럼 팬들이 구매/활동을 공유하고 서로 리워드받는 공간.",
            ),
            Step::ByDao => (
                "Biyard Launchpad",
                "DAO 투표 — 팬덤이 르무통의 다음 신상 컬러, 친환경 정책 등을 직접 결정. 참여하면 팬덤 지분 추가 획득.",
            ),
            Step::ByActivity => (
                "Biyard Launchpad",
                "활동 인증 — 르무통 신발을 신고 걸을수록 팬덤 지분이 자랍니다. 건강과 자산이 동시에 성장.",
            ),
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
            style: "background: rgba(0,0,0,0.3); border-top: 1px solid rgba(255,255,255,0.04); border-bottom: 1px solid rgba(255,255,255,0.04);",
            div {
                class: "demo-container",

                div { style: "text-align: center; margin-bottom: 16px;",
                    p { style: "color: #00dfc0; font-size: 11px; font-weight: 900; letter-spacing: 0.4em; text-transform: uppercase;", "Interactive Scenario" }
                    h2 { style: "font-size: clamp(1.8rem, 3.5vw, 2.6rem); font-weight: 900; color: #fff; margin-top: 12px;",
                        "르무통 앱에 "
                        span { class: "gradient-text-mint", "Biyard Launchpad" }
                        "를 붙이면?"
                    }
                    p { style: "color: #94a3b8; font-size: 14px; margin-top: 8px;",
                        "르무통이 이미 개발한 앱은 그대로. 팬덤 경험만 Biyard가 제공합니다."
                    }
                }

                // Legend
                div { style: "display: flex; gap: 12px; justify-content: center; margin: 20px 0 24px;",
                    span { class: "legend-pill legend-lm", "\u{1F90E} 르무통 자체 개발 (이미 완료)" }
                    span { class: "legend-pill legend-by", "\u{2B21} Biyard Launchpad 제공" }
                }

                // Step controller
                div { class: "step-controller",
                    for step in ALL_STEPS {
                        {
                            let active = *current.read() == step;
                            let lm_class = if step.is_biyard() { "" } else { "lm-step " };
                            let active_class = if active { "active" } else { "" };
                            rsx! {
                                button {
                                    class: "step-btn {lm_class}{active_class}",
                                    onclick: move |_| current.set(step),
                                    "{step.label()}"
                                }
                            }
                        }
                    }
                }

                // Phone + annotation split
                div { style: "display: grid; grid-template-columns: auto 1fr; gap: 48px; align-items: start; margin-top: 24px;",

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
                        let (owner, desc) = step.description();
                        let is_by = step.is_biyard();
                        let owner_color = if is_by { "#00dfc0" } else { "#D4C5B0" };
                        let owner_bg = if is_by { "rgba(0,223,192,0.06)" } else { "rgba(212,197,176,0.06)" };
                        let owner_border = if is_by { "rgba(0,223,192,0.2)" } else { "rgba(212,197,176,0.2)" };
                        rsx! {
                            div { style: "padding: 28px; background: rgba(255,255,255,0.02); border: 1px solid rgba(255,255,255,0.06); border-radius: 18px; position: sticky; top: 100px;",
                                div { style: "padding: 6px 12px; display: inline-block; background: {owner_bg}; border: 1px solid {owner_border}; color: {owner_color}; font-size: 10px; font-weight: 900; letter-spacing: 0.3em; text-transform: uppercase; border-radius: 999px; margin-bottom: 16px;",
                                    "{owner}"
                                }
                                h3 { style: "font-size: 22px; font-weight: 900; color: #fff; margin-bottom: 14px;", "{step.label()}" }
                                p { style: "color: #94a3b8; font-size: 14px; line-height: 1.8; margin-bottom: 20px;", "{desc}" }

                                // Integration detail for Biyard steps
                                if is_by {
                                    div { style: "padding: 16px; background: rgba(0,223,192,0.04); border: 1px solid rgba(0,223,192,0.15); border-radius: 12px;",
                                        p { style: "font-size: 10px; color: #00dfc0; font-weight: 900; letter-spacing: 0.2em; margin-bottom: 8px;", "\u{2261} INTEGRATION" }
                                        p { style: "font-family: monospace; font-size: 11px; color: #cbd5e1; line-height: 1.6;",
                                            {
                                                let code = match step {
                                                    Step::ByReward => "await Biyard.recordPurchase({\n  userId, \n  amount: 129000,\n  brandId: 'lemouton'\n})",
                                                    Step::ByDashboard => "<BiyardDashboard\n  userId={user.id}\n  brand=\"lemouton\"\n  fandomName=\"Shepherd\"\n/>",
                                                    Step::ByFeed => "<BiyardFandomFeed brand=\"lemouton\" />",
                                                    Step::ByDao => "<BiyardDAO\n  brand=\"lemouton\"\n  minLevel={2}\n/>",
                                                    Step::ByActivity => "await Biyard.verifyActivity({\n  userId, \n  type: 'walking',\n  steps: 8412\n})",
                                                    _ => "",
                                                };
                                                rsx! { "{code}" }
                                            }
                                        }
                                    }
                                } else {
                                    div { style: "padding: 16px; background: rgba(212,197,176,0.04); border: 1px solid rgba(212,197,176,0.15); border-radius: 12px;",
                                        p { style: "font-size: 11px; color: #D4C5B0; line-height: 1.6;",
                                            "이 화면은 르무통이 이미 개발 완료한 부분입니다. Biyard는 건드리지 않습니다."
                                        }
                                    }
                                }

                                // Navigation
                                div { style: "display: flex; gap: 8px; margin-top: 20px;",
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
