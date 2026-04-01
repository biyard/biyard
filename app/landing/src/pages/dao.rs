use dioxus::prelude::*;
use crate::components::UserNav;

#[derive(Clone)]
struct Proposal {
    id: &'static str,
    brand: &'static str,
    title: &'static str,
    desc: &'static str,
    yes: i32,
    no: i32,
    deadline: &'static str,
}

const PROPOSALS: &[Proposal] = &[
    Proposal { id: "1", brand: "Le Mouton", title: "리워드 배수 2배 증가", desc: "걷기 챌린지 리워드를 기존 대비 2배로 상향하는 제안입니다.", yes: 1250, no: 340, deadline: "2026-04-15" },
    Proposal { id: "2", brand: "Cafe Blossom", title: "신메뉴 출시 기념 보너스", desc: "신메뉴 구매 시 추가 50 포인트 지급 제안입니다.", yes: 890, no: 120, deadline: "2026-04-20" },
    Proposal { id: "3", brand: "RunPulse", title: "마라톤 이벤트 토큰 배분", desc: "서울 마라톤 참가자에게 500 RPT 보너스 지급 제안입니다.", yes: 2100, no: 180, deadline: "2026-04-10" },
];

#[component]
pub fn Dao() -> Element {
    let mut votes = use_signal(|| {
        let mut m = std::collections::HashMap::<String, String>::new();
        m.insert("3".to_string(), "yes".to_string());
        m
    });

    rsx! {
        div {
            style: "background: #0a0e17; color: #e8eefc; min-height: 100vh;",
            UserNav {}
            div {
                class: "max-w-4xl mx-auto px-4 py-10",
                div {
                    class: "mb-8",
                    h1 { class: "text-3xl font-bold", "DAO Governance" }
                    p { class: "text-gray-400 mt-1", "토큰 보유자로서 브랜드의 미래를 결정하세요" }
                }
                div {
                    class: "space-y-6",
                    for proposal in PROPOSALS.iter() {
                        {
                            let total = proposal.yes + proposal.no;
                            let yes_pct = if total > 0 { (proposal.yes as f64 / total as f64 * 100.0) as i32 } else { 0 };
                            let no_pct = 100 - yes_pct;
                            let pid = proposal.id.to_string();
                            let user_vote = votes.read().get(&pid).cloned();

                            rsx! {
                                div {
                                    class: "rounded-2xl p-6",
                                    style: "background: #141c2b; border: 1px solid rgba(0,212,170,0.12);",
                                    // Brand badge + deadline
                                    div {
                                        class: "flex items-center gap-2 mb-3",
                                        span {
                                            class: "text-xs px-2 py-0.5 rounded-full font-medium",
                                            style: "background: rgba(0,212,170,0.15); color: #00d4aa;",
                                            "{proposal.brand}"
                                        }
                                        span { class: "text-xs text-gray-500", "마감: {proposal.deadline}" }
                                    }
                                    h3 { class: "text-lg font-bold mb-2", "{proposal.title}" }
                                    p { class: "text-sm text-gray-400 mb-5", "{proposal.desc}" }
                                    // Vote bars
                                    div {
                                        class: "space-y-3 mb-5",
                                        VoteBar { label: "찬성", count: proposal.yes, pct: yes_pct, color: "#00d4aa" }
                                        VoteBar { label: "반대", count: proposal.no, pct: no_pct, color: "#ef4444" }
                                    }
                                    // Vote buttons
                                    if let Some(vote) = user_vote {
                                        p {
                                            class: "text-sm text-gray-500",
                                            "투표 완료: "
                                            span {
                                                style: if vote == "yes" { "color: #00d4aa;" } else { "color: #ef4444;" },
                                                if vote == "yes" { "찬성" } else { "반대" }
                                            }
                                        }
                                    } else {
                                        div {
                                            class: "flex gap-3",
                                            {
                                                let pid_yes = pid.clone();
                                                let pid_no = pid.clone();
                                                rsx! {
                                                    button {
                                                        class: "flex-1 py-2.5 rounded-lg text-sm font-semibold",
                                                        style: "background: #00d4aa; color: #0a0e17;",
                                                        onclick: move |_| {
                                                            votes.write().insert(pid_yes.clone(), "yes".to_string());
                                                        },
                                                        "찬성"
                                                    }
                                                    button {
                                                        class: "flex-1 py-2.5 rounded-lg text-sm font-semibold",
                                                        style: "border: 1px solid rgba(239,68,68,0.5); color: #ef4444;",
                                                        onclick: move |_| {
                                                            votes.write().insert(pid_no.clone(), "no".to_string());
                                                        },
                                                        "반대"
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
            }
        }
    }
}

#[component]
fn VoteBar(label: &'static str, count: i32, pct: i32, color: &'static str) -> Element {
    rsx! {
        div {
            div {
                class: "flex justify-between text-sm mb-1",
                span { "{label}" }
                span { class: "text-gray-400", "{count} votes" }
            }
            div {
                class: "w-full h-7 rounded-full overflow-hidden",
                style: "background: #1a2435;",
                div {
                    class: "h-full rounded-full flex items-center justify-end pr-2 transition-all",
                    style: "width: {pct}%; background: {color};",
                    span {
                        class: "text-xs font-bold",
                        style: if color == "#00d4aa" { "color: #0a0e17;" } else { "color: white;" },
                        "{pct}%"
                    }
                }
            }
        }
    }
}
