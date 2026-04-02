mod about;
mod cta;
mod data;
mod faq;
mod footer;
mod hero;
mod hero_cube;
mod showcase;
mod solution;
mod svgs;
mod why_biyard;

use dioxus::prelude::*;

use data::console_url;

#[component]
pub fn Home() -> Element {
    let console_href = console_url();
    rsx! {
        div {
            style: "min-height: 100vh; background: transparent; color: white; font-family: 'Noto Sans KR', sans-serif; overflow-x: hidden;",
            document::Link { rel: "stylesheet", href: asset!("/assets/scroll-animations.css") }
            document::Script { src: "https://cdnjs.cloudflare.com/ajax/libs/three.js/r128/three.min.js" }
            document::Script { src: asset!("/assets/three-cube.js") }
            document::Script { src: asset!("/assets/scroll-animations.js") }

            // Vignette overlay
            div { class: "vignette-overlay" }

            // Navigation
            nav {
                style: "position: fixed; top: 0; left: 0; right: 0; z-index: 100; background: rgba(2,4,8,0.7); backdrop-filter: blur(20px); -webkit-backdrop-filter: blur(20px); border-bottom: 1px solid rgba(255,255,255,0.04); padding: 16px 24px;",
                div {
                    class: "max-w-7xl mx-auto flex items-center justify-between",
                    a {
                        href: "#",
                        class: "flex items-center gap-2.5",
                        img {
                            src: asset!("/assets/biyard-logo.png"),
                            alt: "Biyard",
                            style: "width: 28px; height: 28px;",
                        }
                        span {
                            class: "text-xl font-extrabold italic tracking-tighter",
                            style: "color: #00dfc0;",
                            "BIYARD"
                        }
                    }
                    div {
                        class: "hidden md:flex items-center gap-8",
                        for (label, href) in [("About", "#about"), ("Solution", "#solution"), ("Showcase", "#showcase"), ("FAQ", "#faq")] {
                            a {
                                href: "{href}",
                                class: "text-xs font-bold uppercase tracking-widest",
                                style: "color: #475569; transition: color 0.2s;",
                                "{label}"
                            }
                        }
                    }
                    a {
                        href: "{console_href}",
                        class: "btn-hyper px-6 py-2.5 rounded-sm text-xs font-black uppercase tracking-widest interactive",
                        "Console"
                    }
                }
            }

            // Content wrapper (above Three.js canvas)
            div {
                class: "content-wrapper",
                hero::HeroSection {}
                about::AboutSection {}
                why_biyard::WhyBiyardSection {}
                solution::SolutionSection {}
                showcase::ShowcaseSection {}
                faq::FaqSection {}
                cta::CtaSection {}
                footer::Footer {}
            }
        }
    }
}
