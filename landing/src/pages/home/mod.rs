mod about;
mod cta;
mod data;
mod faq;
mod footer;
mod hero;
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
            style: "min-height: 100vh; background: #0c1018; color: #e8eefc; font-family: 'Outfit', 'Noto Sans KR', sans-serif;",
            document::Link { rel: "stylesheet", href: asset!("/assets/animations.css") }
            document::Script { src: asset!("/assets/scroll-reveal.js") }

            // Sticky Navigation Bar
            nav {
                style: "position: fixed; top: 0; left: 0; right: 0; z-index: 50; background: rgba(12,16,24,0.85); backdrop-filter: blur(16px); -webkit-backdrop-filter: blur(16px); border-bottom: 1px solid rgba(0,212,170,0.08); padding: 14px 24px;",
                div {
                    class: "max-w-6xl mx-auto flex items-center justify-between",
                    a {
                        href: "#",
                        class: "flex items-center gap-2",
                        img {
                            src: asset!("/assets/biyard-logo.png"),
                            alt: "Biyard",
                            style: "width: 28px; height: 28px; flex-shrink: 0;",
                        }
                        span {
                            class: "text-xl font-extrabold",
                            style: "color: #00d4aa;",
                            "Biyard"
                        }
                    }
                    div {
                        class: "hidden md:flex items-center gap-8",
                        a {
                            href: "#about",
                            class: "text-sm font-medium transition-colors",
                            style: "color: #7a8ba6;",
                            "About"
                        }
                        a {
                            href: "#solution",
                            class: "text-sm font-medium transition-colors",
                            style: "color: #7a8ba6;",
                            "Solution"
                        }
                        a {
                            href: "#showcase",
                            class: "text-sm font-medium transition-colors",
                            style: "color: #7a8ba6;",
                            "Showcase"
                        }
                        a {
                            href: "#faq",
                            class: "text-sm font-medium transition-colors",
                            style: "color: #7a8ba6;",
                            "FAQ"
                        }
                    }
                    a {
                        href: "{console_href}",
                        class: "inline-flex items-center px-4 py-2 rounded-lg text-sm font-bold",
                        style: "background: #00d4aa; color: #0c1018;",
                        "Console"
                    }
                }
            }

            // Content
            hero::HeroSection {}
            about::AboutSection {}
            solution::SolutionSection {}
            showcase::ShowcaseSection {}
            why_biyard::WhyBiyardSection {}
            faq::FaqSection {}
            cta::CtaSection {}
            footer::Footer {}
        }
    }
}
