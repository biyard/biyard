use dioxus::prelude::*;
use crate::Route;

#[component]
pub(super) fn Footer() -> Element {
    rsx! {
        footer {
            class: "py-16 px-6 md:px-24",
            style: "border-top: 1px solid rgba(255,255,255,0.05); position: relative; z-index: 10;",
            div {
                class: "max-w-6xl mx-auto flex flex-col md:flex-row justify-between items-center gap-10",
                div {
                    class: "flex items-center gap-2.5",
                    img { src: asset!("/assets/biyard-logo.png"), alt: "Biyard", style: "width: 24px; height: 24px;" }
                    span { class: "text-2xl font-black italic tracking-tighter", style: "color: #00dfc0;", "BIYARD" }
                }
                div {
                    class: "flex gap-10 uppercase tracking-widest",
                    style: "font-size: 10px; font-weight: 900; color: #64748b;",
                    Link { to: Route::Team {}, class: "hover:text-white transition-colors", "Team" }
                    a { href: "https://github.com/biyard", target: "_blank", class: "hover:text-white transition-colors", "GitHub" }
                    a { href: "https://www.linkedin.com/company/75498162", target: "_blank", class: "hover:text-white transition-colors", "LinkedIn" }
                    a { href: "mailto:finance@biyard.co", class: "hover:text-white transition-colors", "Contact" }
                }
                div { class: "font-mono", style: "font-size: 10px; color: #334155;", "\u{00A9} 2026 BIYARD CORP." }
            }
        }
    }
}
