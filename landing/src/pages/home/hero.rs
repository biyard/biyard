use dioxus::prelude::*;

use super::data::console_url;
use super::svgs::{
    CoffeeIcon, FashionIcon, FloorPriceChart, HoldersChart, ShoeIcon, TreasuryBars,
    REVENUE_DONUT_SVG,
};

#[component]
pub(super) fn HeroSection() -> Element {
    let console_href = console_url();
    rsx! {
        // Hero
        section {
            class: "relative overflow-hidden flex flex-col items-center justify-center px-4",
            style: "background: linear-gradient(135deg, #0c1018 0%, #0d1a24 50%, #0c1018 100%); min-height: 100vh; padding-top: 64px;",
            // Background glows (large, dramatic)
            div {
                class: "absolute inset-0 pointer-events-none",
                div {
                    class: "absolute rounded-full",
                    style: "top: 15%; left: 35%; width: 500px; height: 500px; background: #6366f1; filter: blur(120px); opacity: 0.12;",
                }
                div {
                    class: "absolute rounded-full",
                    style: "top: 25%; left: 45%; width: 400px; height: 400px; background: #00d4aa; filter: blur(100px); opacity: 0.08;",
                }
                div {
                    class: "absolute rounded-full",
                    style: "bottom: 20%; right: 30%; width: 450px; height: 450px; background: #38bdf8; filter: blur(100px); opacity: 0.08;",
                }
            }

            // Main centered content
            div {
                class: "relative z-10 flex flex-col items-center pt-16 pb-12 w-full max-w-6xl mx-auto",

                // Multi-cube container
                div {
                    class: "relative mb-20 w-full lg:w-[750px] h-[320px] lg:h-[500px] mx-auto",
                    style: "max-width: 100%;",

                    // Left cube (Fashion/Retail - overlapping, tilted)
                    div {
                        class: "absolute hidden lg:block",
                        style: "left: 10px; bottom: 5%; perspective: 900px; z-index: 2;",
                        // Glow
                        div { style: "position: absolute; top: 50%; left: 50%; transform: translate(-50%,-50%); width: 160px; height: 160px; background: #60a5fa; filter: blur(60px); opacity: 0.12; border-radius: 50%;" }
                        div {
                            style: "width: 200px; height: 200px; position: relative; transform-style: preserve-3d; animation: spinCubeLeft 22s linear infinite;",
                            // Front: Shoe SVG
                            div {
                                style: "transform: translateZ(100px); background: rgba(96,165,250,0.08); border: 1px solid rgba(96,165,250,0.25); position: absolute; inset: 0; border-radius: 14px; backdrop-filter: blur(4px); display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 10px;",
                                div {
                                    style: "color: #60a5fa; width: 80px; height: 50px;",
                                    ShoeIcon {}
                                }
                                span { style: "font-size: 9px; font-weight: 700; color: #60a5fa; margin-top: 4px;", "Shoe Brand" }
                            }
                            // Back: Coffee SVG
                            div {
                                style: "transform: rotateY(180deg) translateZ(100px); background: rgba(244,114,182,0.08); border: 1px solid rgba(244,114,182,0.25); position: absolute; inset: 0; border-radius: 14px; backdrop-filter: blur(4px); display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 10px;",
                                div {
                                    style: "color: #f472b6; width: 70px; height: 50px;",
                                    CoffeeIcon {}
                                }
                                span { style: "font-size: 9px; font-weight: 700; color: #f472b6; margin-top: 4px;", "Coffee Brand" }
                            }
                            // Left side
                            div {
                                style: "transform: rotateY(-90deg) translateZ(100px); background: rgba(52,211,153,0.08); border: 1px solid rgba(52,211,153,0.2); position: absolute; inset: 0; border-radius: 14px; backdrop-filter: blur(4px); display: flex; flex-direction: column; align-items: center; justify-content: center;",
                                div {
                                    style: "color: #34d399; width: 60px; height: 50px;",
                                    FashionIcon {}
                                }
                                span { style: "font-size: 9px; font-weight: 700; color: #34d399; margin-top: 2px;", "Fashion Brand" }
                            }
                            // Right side
                            div {
                                style: "transform: rotateY(90deg) translateZ(100px); background: rgba(251,191,36,0.06); border: 1px solid rgba(251,191,36,0.2); position: absolute; inset: 0; border-radius: 14px; backdrop-filter: blur(4px); display: flex; flex-direction: column; align-items: center; justify-content: center;",
                                span { style: "font-size: 24px; font-weight: 800; color: #fbbf24; opacity: 0.6;", "+" }
                                span { style: "font-size: 8px; color: #7a8ba6; margin-top: 2px;", "More Brands" }
                            }
                        }
                    }

                    // Right cube (Stats/Metrics - overlapping, tilted)
                    div {
                        class: "absolute hidden lg:block",
                        style: "right: 10px; bottom: 0%; perspective: 900px; z-index: 2;",
                        // Glow
                        div { style: "position: absolute; top: 50%; left: 50%; transform: translate(-50%,-50%); width: 160px; height: 160px; background: #a78bfa; filter: blur(60px); opacity: 0.12; border-radius: 50%;" }
                        div {
                            style: "width: 200px; height: 200px; position: relative; transform-style: preserve-3d; animation: spinCubeRight 19s linear infinite;",
                            // Face 1: Rising line chart + Floor Price
                            div {
                                style: "transform: translateZ(100px); background: rgba(52,211,153,0.06); border: 1px solid rgba(52,211,153,0.2); position: absolute; inset: 0; border-radius: 14px; backdrop-filter: blur(4px); display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 15px;",
                                div {
                                    style: "width: 100%; height: 60px; color: #34d399;",
                                    FloorPriceChart {}
                                }
                                span { style: "font-size: 8px; color: #34d399; letter-spacing: 1px; text-transform: uppercase; margin-top: 6px;", "Floor Price" }
                            }
                            // Face 2: Stair-step bars + Treasury
                            div {
                                style: "transform: rotateY(180deg) translateZ(100px); background: rgba(96,165,250,0.06); border: 1px solid rgba(96,165,250,0.2); position: absolute; inset: 0; border-radius: 14px; backdrop-filter: blur(4px); display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 15px;",
                                div {
                                    style: "width: 100%; height: 60px; color: #60a5fa;",
                                    TreasuryBars {}
                                }
                                span { style: "font-size: 8px; color: #60a5fa; letter-spacing: 1px; text-transform: uppercase; margin-top: 6px;", "Treasury" }
                            }
                            // Face 3: Growing user icons + Holders
                            div {
                                style: "transform: rotateY(90deg) translateZ(100px); background: rgba(167,139,250,0.06); border: 1px solid rgba(167,139,250,0.2); position: absolute; inset: 0; border-radius: 14px; backdrop-filter: blur(4px); display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 15px;",
                                div {
                                    style: "width: 100%; height: 60px; color: #a78bfa;",
                                    HoldersChart {}
                                }
                                span { style: "font-size: 8px; color: #a78bfa; letter-spacing: 1px; text-transform: uppercase; margin-top: 6px;", "Holders" }
                            }
                            // Face 4: Pie/donut chart + Revenue
                            div {
                                style: "transform: rotateY(-90deg) translateZ(100px); background: rgba(244,114,182,0.06); border: 1px solid rgba(244,114,182,0.2); position: absolute; inset: 0; border-radius: 14px; backdrop-filter: blur(4px); display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 15px;",
                                div {
                                    style: "width: 100%; height: 60px; color: #f472b6;",
                                    dangerous_inner_html: REVENUE_DONUT_SVG,
                                }
                                span { style: "font-size: 8px; color: #f472b6; letter-spacing: 1px; text-transform: uppercase; margin-top: 6px;", "Revenue" }
                            }
                        }
                    }

                    // Main centered cube (original)
                    div {
                        class: "absolute w-[260px] h-[260px] lg:w-[380px] lg:h-[380px]",
                        style: "left: 50%; top: 40%; transform: translate(-50%, -50%);",
                    // Outer glow ring
                    div {
                        class: "absolute rounded-full w-[200px] h-[200px] lg:w-[300px] lg:h-[300px]",
                        style: "top: 50%; left: 50%; transform: translate(-50%, -50%); border: 1px solid #00d4aa; opacity: 0.2; box-shadow: 0 0 60px rgba(0,212,170,0.3), inset 0 0 60px rgba(0,212,170,0.1);",
                    }
                    // Dashed ring
                    div {
                        class: "absolute rounded-full hidden lg:block",
                        style: "top: 50%; left: 50%; transform: translate(-50%, -50%); width: 420px; height: 420px; border: 1px dashed rgba(96,165,250,0.3); opacity: 0.15; animation: spinCube 30s linear infinite;",
                    }
                    // Center glow
                    div {
                        class: "absolute rounded-full w-[160px] h-[160px] lg:w-[240px] lg:h-[240px]",
                        style: "top: 50%; left: 50%; transform: translate(-50%, -50%); background: #6366f1; filter: blur(90px); opacity: 0.3;",
                    }
                    // Rotating cube (big: 200x200, translateZ 100px)
                    div {
                        class: "absolute",
                        style: "top: 50%; left: 50%; transform: translate(-50%, -50%); perspective: 1200px;",
                        div {
                            style: "width: 200px; height: 200px; position: relative; transform-style: preserve-3d; animation: spinCubeMain 15s linear infinite;",
                            // Cube faces
                            for (transform, icon, value, label, bg) in [
                                ("translateZ(100px)", "\u{1F4C8}", "$0.0245", "Floor Price", "rgba(96,165,250,0.12)"),
                                ("rotateY(180deg) translateZ(100px)", "\u{1F512}", "$72,600", "Treasury", "rgba(167,139,250,0.12)"),
                                ("rotateY(90deg) translateZ(100px)", "\u{267E}\u{FE0F}", "AUTO", "Buyback", "rgba(99,102,241,0.12)"),
                                ("rotateY(-90deg) translateZ(100px)", "\u{1F525}", "DEFLATION", "Burn", "rgba(244,114,182,0.12)"),
                            ] {
                                {
                                    let face_style = format!(
                                        "transform: {}; background: {}; border: 1px solid rgba(148,163,250,0.2); box-shadow: 0 0 30px rgba(99,102,241,0.12), inset 0 0 30px rgba(99,102,241,0.05); position: absolute; inset: 0; border-radius: 20px; backdrop-filter: blur(4px); display: flex; flex-direction: column; align-items: center; justify-content: center;",
                                        transform, bg
                                    );
                                    rsx! {
                                        div {
                                            style: "{face_style}",
                                            span {
                                                class: "text-3xl mb-1",
                                                style: "filter: drop-shadow(0 0 8px rgba(99,102,241,0.5));",
                                                "{icon}"
                                            }
                                            span {
                                                class: "text-base font-extrabold",
                                                style: "color: #a5b4fc; letter-spacing: -0.025em;",
                                                "{value}"
                                            }
                                            span {
                                                class: "text-xs font-semibold mt-1",
                                                style: "color: #7a8ba6; letter-spacing: 0.1em; text-transform: uppercase; font-size: 9px;",
                                                "{label}"
                                            }
                                        }
                                    }
                                }
                            }
                            // Top and bottom faces (Biyard logo)
                            for transform in ["rotateX(90deg) translateZ(100px)", "rotateX(-90deg) translateZ(100px)"] {
                                {
                                    let face_style = format!(
                                        "transform: {}; background: rgba(0,212,170,0.15); border: 1px solid rgba(0,212,170,0.35); position: absolute; inset: 0; border-radius: 20px; display: flex; align-items: center; justify-content: center;",
                                        transform
                                    );
                                    rsx! {
                                        div {
                                            style: "{face_style}",
                                            span {
                                                class: "text-xl font-black tracking-wider",
                                                style: "color: #00d4aa; filter: drop-shadow(0 0 12px rgba(0,212,170,0.6));",
                                                "Biyard"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    // Orbiting labels around the cube (hidden on mobile)
                    for (text, top, left, delay) in [
                        ("Treasury \u{2191}", "2%", "60%", "0s"),
                        ("Burn \u{1F525}", "82%", "70%", "1.2s"),
                        ("Buyback \u{267E}\u{FE0F}", "80%", "5%", "0.6s"),
                        ("Floor \u{2191}", "5%", "2%", "1.8s"),
                    ] {
                        {
                            let label_style = format!(
                                "top: {}; left: {}; position: absolute; background: rgba(99,102,241,0.08); border: 1px solid rgba(148,163,250,0.2); border-radius: 9999px; padding: 6px 14px; font-size: 11px; font-weight: 600; color: #a5b4fc; letter-spacing: 0.5px; animation: floatParticle 3.5s ease-in-out {} infinite alternate; box-shadow: 0 0 16px rgba(99,102,241,0.1);",
                                top, left, delay
                            );
                            rsx! {
                                div { class: "hidden lg:block", style: "{label_style}", "{text}" }
                            }
                        }
                    }
                    // Sparkle dots (hidden on mobile)
                    for (top, left, dur) in [("3%", "42%", "2.5s"), ("50%", "2%", "3.2s"), ("92%", "48%", "4s"), ("30%", "95%", "2.8s"), ("65%", "95%", "3.5s"), ("15%", "85%", "2.2s")] {
                        {
                            let dot_style = format!(
                                "position: absolute; top: {}; left: {}; width: 5px; height: 5px; border-radius: 9999px; background: #00d4aa; box-shadow: 0 0 8px #00d4aa, 0 0 16px rgba(0,212,170,0.4); animation: floatParticle {} ease-in-out infinite alternate;",
                                top, left, dur
                            );
                            rsx! {
                                div { class: "hidden lg:block", style: "{dot_style}" }
                            }
                        }
                    }
                    } // end main cube absolute
                } // end multi-cube container

                // Hero text below cube
                div {
                    class: "text-center max-w-2xl",
                    p {
                        class: "font-medium mb-4 uppercase text-sm",
                        style: "color: #00d4aa; letter-spacing: 0.1em;",
                        "Revenue-Backed Token Platform"
                    }
                    h1 {
                        class: "text-3xl md:text-5xl leading-tight",
                        style: "color: #e8eefc; font-weight: 300; font-family: 'Outfit', 'Noto Sans KR', sans-serif; letter-spacing: -0.02em;",
                        "매출이 곧 브랜드의"
                        br {}
                        span {
                            style: "background-image: linear-gradient(to right, #60a5fa, #a78bfa); -webkit-background-clip: text; background-clip: text; color: transparent; font-weight: 600; font-family: 'Outfit', 'Noto Sans KR', sans-serif;",
                            "가치가 됩니다"
                        }
                    }
                    p {
                        class: "mt-6 text-base md:text-xl max-w-2xl mx-auto leading-relaxed",
                        style: "color: #7a8ba6;",
                        "실제 매출에 연동된 토큰 이코노미로, 고객은 주주가 되고 브랜드는 함께 성장합니다."
                    }
                    div {
                        class: "mt-10 flex items-center justify-center gap-4 flex-wrap",
                        a {
                            class: "inline-flex items-center px-8 py-4 rounded-xl font-bold text-lg",
                            style: "background: #00d4aa; color: #0c1018; box-shadow: 0 10px 25px rgba(0,212,170,0.3);",
                            href: "{console_href}",
                            "Console로 이동하기 →"
                        }
                    }
                    // Company logos flowing between cubes
                    div {
                        class: "mt-12 relative hidden lg:block",
                        style: "height: 40px; width: 100%; max-width: 800px; overflow: hidden;",
                        // Flowing logo track
                        div {
                            class: "flex items-center gap-10 absolute",
                            style: "animation: flowLogos 25s linear infinite; white-space: nowrap;",
                            for (name, color) in [
                                ("StyleKorea", "#60a5fa"), ("FreshMart", "#34d399"), ("FitLife", "#f472b6"),
                                ("BrewHaus", "#fbbf24"), ("TechGym", "#a78bfa"), ("MediCare", "#38bdf8"),
                                ("GreenWalk", "#34d399"), ("LuxBrand", "#f472b6"), ("RunClub", "#60a5fa"),
                                ("CafeNova", "#fbbf24"),
                                ("StyleKorea", "#60a5fa"), ("FreshMart", "#34d399"), ("FitLife", "#f472b6"),
                                ("BrewHaus", "#fbbf24"), ("TechGym", "#a78bfa"), ("MediCare", "#38bdf8"),
                                ("GreenWalk", "#34d399"), ("LuxBrand", "#f472b6"), ("RunClub", "#60a5fa"),
                                ("CafeNova", "#fbbf24"),
                            ] {
                                div {
                                    class: "flex items-center gap-2 px-4 py-1.5 rounded-full flex-shrink-0",
                                    style: "background: rgba(20,28,43,0.6); border: 1px solid rgba(255,255,255,0.06); backdrop-filter: blur(4px);",
                                    // Glow dot
                                    div {
                                        style: "width: 6px; height: 6px; border-radius: 50%; background: {color}; box-shadow: 0 0 6px {color};",
                                    }
                                    span {
                                        style: "font-size: 12px; font-weight: 600; color: {color}; letter-spacing: 0.5px; font-family: 'Outfit', sans-serif;",
                                        "{name}"
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
