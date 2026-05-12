use dioxus::prelude::*;

use crate::features::catalog::controllers::list_stos;
use crate::features::catalog::StoSummary;

#[component]
pub fn HomeView() -> Element {
    let data = use_server_future(|| async { list_stos().await })?;
    let resp = data.read();
    let resp_ref = resp.as_ref();

    rsx! {
        Topbar {}
        main { class: "max-w-7xl mx-auto px-6 py-8",
            h1 { class: "text-2xl font-bold mb-4", "Biyard STO" }
            p { class: "text-ink-soft mb-8",
                "국내 STO·조각투자 정보 플랫폼. DART 공시 + 뮤직카우 카탈로그 통합."
            }
            section {
                h2 { class: "text-base font-bold mb-3 pb-2 border-b border-line",
                    "최근 발행 STO"
                }
                match resp_ref {
                    Some(Ok(r)) => rsx! {
                        div { class: "text-xs text-muted mb-3",
                            "총 " span { class: "text-ink font-bold", "{r.total}" } " 건"
                        }
                        StoTable { items: r.items.iter().take(20).cloned().collect::<Vec<_>>() }
                    },
                    Some(Err(e)) => rsx! {
                        div { class: "text-danger", "로드 실패: {e}" }
                    },
                    None => rsx! {
                        div { class: "text-muted", "로딩 중..." }
                    },
                }
            }
        }
    }
}

#[component]
pub fn Topbar() -> Element {
    rsx! {
        header { class: "border-b border-line bg-panel",
            div { class: "max-w-7xl mx-auto px-6 py-3 flex items-center gap-6",
                a { href: "/", class: "font-bold text-brand", "Biyard STO" }
                nav { class: "flex gap-4 text-sm text-ink-soft",
                    a { href: "/", class: "hover:text-ink", "홈" }
                    a { href: "/assets", class: "hover:text-ink", "STO 시장" }
                }
            }
        }
    }
}

#[component]
pub fn StoTable(items: Vec<StoSummary>) -> Element {
    rsx! {
        div { class: "overflow-x-auto",
            table { class: "w-full text-sm",
                thead {
                    tr { class: "text-left text-xs text-muted uppercase",
                        th { class: "px-3 py-2 bg-panel-2 border-b border-line", "" }
                        th { class: "px-3 py-2 bg-panel-2 border-b border-line", "자산명" }
                        th { class: "px-3 py-2 bg-panel-2 border-b border-line", "카테고리" }
                        th { class: "px-3 py-2 bg-panel-2 border-b border-line", "발행사" }
                        th { class: "px-3 py-2 bg-panel-2 border-b border-line", "상태" }
                        th { class: "px-3 py-2 bg-panel-2 border-b border-line text-right", "발행일" }
                    }
                }
                tbody {
                    for s in items.iter() {
                        tr { class: "border-b border-line/50 hover:bg-panel-2",
                            td { class: "px-3 py-2 w-7", { flag_for(&s.region) } }
                            td { class: "px-3 py-2",
                                div { class: "font-semibold", {s.name.clone()} }
                                if let Some(artist) = &s.artist {
                                    div { class: "text-xs text-muted", {artist.clone()} }
                                }
                            }
                            td { class: "px-3 py-2 w-32",
                                span { class: "px-2 py-0.5 text-xs rounded bg-panel-2 text-ink-soft",
                                    { category_label(&s.category) }
                                }
                            }
                            td { class: "px-3 py-2 w-32 text-ink-soft", {s.issuer_id.clone().unwrap_or_default()} }
                            td { class: "px-3 py-2 w-24 text-xs", {s.status.clone()} }
                            td { class: "px-3 py-2 w-28 text-xs font-mono text-muted text-right", {s.issued_at.clone()} }
                        }
                    }
                }
            }
        }
    }
}

fn category_label(c: &str) -> &'static str {
    match c {
        "real_estate" => "🏢 부동산",
        "art" => "🎨 미술품",
        "music" => "🎵 음악 IP",
        "livestock" => "🐄 한우·축산",
        "luxury" => "💎 명품",
        "infra" => "⚡ 인프라",
        "content" => "🎬 콘텐츠",
        _ => "기타",
    }
}

fn flag_for(region: &str) -> &'static str {
    match region {
        "KR" => "🇰🇷",
        _ => "🌍",
    }
}
