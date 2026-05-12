use dioxus::prelude::*;

use crate::features::catalog::views::Topbar;

#[component]
pub fn NewsView() -> Element {
    rsx! {
        Topbar { active: "news".to_string() }
        main { class: "max-w-7xl mx-auto px-6 py-8",
            h1 { class: "text-xl font-bold mb-4", "시장 헤드라인" }
            p { class: "text-foreground-muted text-sm mb-6", "공시·시장 동향. (현재는 placeholder — 향후 DART 신규 공시 + 보도자료 자동 큐레이션)" }
            section { class: "bg-panel border border-dashed border-border rounded-2xl p-12 text-center text-foreground-muted",
                "뉴스 피드 준비 중. DART 공시 + 보도자료 자동 큐레이션이 들어옵니다."
            }
        }
    }
}
