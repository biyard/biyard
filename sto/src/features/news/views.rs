use dioxus::prelude::*;

use crate::features::catalog::views::Topbar;

#[component]
pub fn NewsView() -> Element {
    rsx! {
        Topbar { active: "news".to_string() }
        main { class: "max-w-7xl mx-auto px-6 py-8",
            h1 { class: "text-xl font-bold mb-1", "공시·뉴스" }
            p { class: "text-foreground-muted text-sm mb-6",
                "발행사가 새로 등록한 공시와 관련 보도를 모아 보여드립니다."
            }
            section { class: "bg-panel border border-dashed border-border rounded-2xl p-12 text-center text-foreground-muted",
                "공시·보도 피드를 준비 중입니다. 곧 금감원 전자공시(DART)와 주요 매체 보도를 자동으로 모아 보여드릴 예정입니다."
            }
        }
    }
}
