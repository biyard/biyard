use dioxus::prelude::*;

/// Two rows of scrolling brand logos:
/// Row 1: Partner/collaborating companies (left→right)
/// Row 2: Onboarded platform brands (right→left)
#[component]
pub(super) fn PartnersSection() -> Element {
    rsx! {
        section {
            class: "py-20 overflow-hidden relative",
            style: "z-index: 10;",

            // Row 1: Partners
            div {
                class: "mb-6",
                div {
                    class: "text-center mb-6",
                    span {
                        style: "color: #64748b; font-size: 10px; font-weight: 900; letter-spacing: 0.4em; text-transform: uppercase;",
                        "Partners & Collaborators"
                    }
                }
                div {
                    style: "height: 44px; overflow: hidden; position: relative;",
                    div {
                        class: "flex items-center gap-10 absolute",
                        style: "animation: flowLeft 30s linear infinite; white-space: nowrap;",
                        for _round in 0..2u8 {
                            for (name, color) in [
                                ("Samsung Securities", "#60a5fa"),
                                ("KB Securities", "#34d399"),
                                ("Simsan Ventures", "#a78bfa"),
                                ("Hashed", "#f472b6"),
                                ("Kakao Ventures", "#fbbf24"),
                                ("SBI Investment", "#38bdf8"),
                                ("Ethereum Foundation", "#60a5fa"),
                                ("Polygon Labs", "#a78bfa"),
                                ("AWS Korea", "#fbbf24"),
                                ("Google Cloud", "#34d399"),
                            ] {
                                div {
                                    class: "flex items-center gap-2 px-5 py-2 rounded-full flex-shrink-0",
                                    style: "background: rgba(255,255,255,0.02); border: 1px solid rgba(255,255,255,0.05);",
                                    div { style: "width: 6px; height: 6px; border-radius: 50%; background: {color}; box-shadow: 0 0 6px {color};" }
                                    span { style: "font-size: 12px; font-weight: 700; color: {color}; letter-spacing: 0.3px;", "{name}" }
                                }
                            }
                        }
                    }
                }
            }

            // Row 2: Onboarded platforms
            div {
                div {
                    class: "text-center mb-6",
                    span {
                        style: "color: #64748b; font-size: 10px; font-weight: 900; letter-spacing: 0.4em; text-transform: uppercase;",
                        "Onboarded Platforms"
                    }
                }
                div {
                    style: "height: 44px; overflow: hidden; position: relative;",
                    div {
                        class: "flex items-center gap-10 absolute",
                        style: "animation: flowRight 25s linear infinite; white-space: nowrap;",
                        for _round in 0..2u8 {
                            for (name, icon, color) in [
                                ("Shoe Brand", "\u{1F45F}", "#60a5fa"),
                                ("Coffee Brand", "\u{2615}", "#f472b6"),
                                ("Fashion Brand", "\u{1F454}", "#34d399"),
                                ("FreshMart", "\u{1F34E}", "#fbbf24"),
                                ("FitLife", "\u{1F3CB}\u{FE0F}", "#a78bfa"),
                                ("MediCare", "\u{1FA7A}", "#38bdf8"),
                                ("BrewHaus", "\u{1F37A}", "#34d399"),
                                ("TechGym", "\u{1F4BB}", "#f472b6"),
                                ("RunClub", "\u{1F3C3}", "#60a5fa"),
                                ("CafeNova", "\u{2615}", "#fbbf24"),
                            ] {
                                div {
                                    class: "flex items-center gap-2 px-5 py-2 rounded-full flex-shrink-0",
                                    style: "background: rgba(255,255,255,0.02); border: 1px solid rgba(255,255,255,0.05);",
                                    span { style: "font-size: 14px;", "{icon}" }
                                    span { style: "font-size: 12px; font-weight: 700; color: {color}; letter-spacing: 0.3px;", "{name}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
