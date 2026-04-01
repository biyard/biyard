use dioxus::prelude::*;

#[component]
pub(super) fn SolutionSection() -> Element {
    rsx! {
        // How It Works - Timeline style
        section {
            id: "solution",
            class: "py-20 px-4 relative overflow-hidden",
            style: "background: #141c2b;",
            div { class: "absolute", style: "top: 50%; left: -150px; transform: translateY(-50%); width: 500px; height: 500px; background: radial-gradient(circle, rgba(167,139,250,0.1) 0%, transparent 70%); pointer-events: none;" }
            div { class: "absolute", style: "bottom: -100px; right: -50px; width: 400px; height: 400px; background: radial-gradient(circle, rgba(96,165,250,0.06) 0%, transparent 70%); pointer-events: none;" }
            div {
                class: "max-w-5xl mx-auto text-center mb-16 reveal-fade",
                h2 {
                    class: "text-3xl md:text-4xl font-bold mb-4 reveal-type",
                    style: "color: #e8eefc; font-family: 'Outfit', 'Noto Sans KR', sans-serif;",
                    "어떻게 작동하나요?"
                }
                p {
                    class: "text-lg max-w-2xl mx-auto",
                    style: "color: #7a8ba6;",
                    "고객이 제품을 구매하면 매출의 일부가 트레저리에 적립되고, 제품을 사용하며 활동할수록 추가 리워드를 받습니다."
                }
            }
            // Timeline
            div {
                class: "max-w-5xl mx-auto relative",
                // Horizontal connecting line (desktop only)
                div {
                    class: "hidden md:block absolute",
                    style: "top: 60px; left: 10%; right: 10%; height: 2px; background: linear-gradient(to right, #60a5fa, #a78bfa, #fbbf24, #34d399); opacity: 0.3;",
                }
                // Animated particles on the line
                div {
                    class: "hidden md:block absolute",
                    style: "top: 57px; left: 10%; width: 8px; height: 8px; border-radius: 50%; background: #60a5fa; box-shadow: 0 0 12px #60a5fa; animation: flowLogos 6s linear infinite;",
                }
                div {
                    class: "grid grid-cols-1 md:grid-cols-4 gap-0",
                    for (si, (step, title, desc, accent, svg_content, node_size)) in [
                        ("01", "고객이 구매", "구매 금액의 2~4%가 자동 적립", "#60a5fa",
                         // Small: single coin
                         r#"<svg viewBox='0 0 80 80' style='width:100%;height:100%;'><circle cx='40' cy='40' r='18' fill='none' stroke='currentColor' stroke-width='1.5' opacity='0.5'/><circle cx='40' cy='40' r='10' fill='currentColor' opacity='0.1'/><text x='40' y='45' text-anchor='middle' fill='currentColor' font-size='14' font-weight='bold' opacity='0.6'>$</text></svg>"#,
                         "70px"),
                        ("02", "트레저리 적립", "온체인 트레저리에 누적", "#a78bfa",
                         // Medium: vault with stacking coins
                         r#"<svg viewBox='0 0 100 100' style='width:100%;height:100%;'><rect x='20' y='25' width='60' height='50' rx='8' fill='none' stroke='currentColor' stroke-width='1.5' opacity='0.5'/><circle cx='50' cy='48' r='12' fill='none' stroke='currentColor' stroke-width='1.5' opacity='0.4'/><circle cx='50' cy='48' r='5' fill='currentColor' opacity='0.15'/><ellipse cx='50' cy='82' rx='16' ry='4' fill='currentColor' opacity='0.08' stroke='currentColor' stroke-width='0.8' opacity='0.2'/><ellipse cx='50' cy='78' rx='16' ry='4' fill='currentColor' opacity='0.1' stroke='currentColor' stroke-width='0.8' opacity='0.25'/><ellipse cx='50' cy='74' rx='16' ry='4' fill='currentColor' opacity='0.12' stroke='currentColor' stroke-width='0.8' opacity='0.3'/></svg>"#,
                         "90px"),
                        ("03", "활동 인증", "고객 활동 인증 시 추가 리워드", "#fbbf24",
                         // Larger: network of verified nodes
                         r#"<svg viewBox='0 0 120 120' style='width:100%;height:100%;'><circle cx='60' cy='35' r='10' fill='none' stroke='currentColor' stroke-width='1.2' opacity='0.4'/><path d='M57,35 L59,37 L64,32' fill='none' stroke='currentColor' stroke-width='1.5' opacity='0.6'/><circle cx='30' cy='70' r='10' fill='none' stroke='currentColor' stroke-width='1.2' opacity='0.4'/><path d='M27,70 L29,72 L34,67' fill='none' stroke='currentColor' stroke-width='1.5' opacity='0.6'/><circle cx='90' cy='70' r='10' fill='none' stroke='currentColor' stroke-width='1.2' opacity='0.4'/><path d='M87,70 L89,72 L94,67' fill='none' stroke='currentColor' stroke-width='1.5' opacity='0.6'/><circle cx='45' cy='100' r='10' fill='none' stroke='currentColor' stroke-width='1.2' opacity='0.3'/><path d='M42,100 L44,102 L49,97' fill='none' stroke='currentColor' stroke-width='1.5' opacity='0.5'/><circle cx='75' cy='100' r='10' fill='none' stroke='currentColor' stroke-width='1.2' opacity='0.3'/><path d='M72,100 L74,102 L79,97' fill='none' stroke='currentColor' stroke-width='1.5' opacity='0.5'/><line x1='60' y1='45' x2='35' y2='62' stroke='currentColor' stroke-width='0.8' opacity='0.2' stroke-dasharray='3,3'/><line x1='60' y1='45' x2='85' y2='62' stroke='currentColor' stroke-width='0.8' opacity='0.2' stroke-dasharray='3,3'/><line x1='35' y1='80' x2='45' y2='92' stroke='currentColor' stroke-width='0.8' opacity='0.15' stroke-dasharray='3,3'/><line x1='85' y1='80' x2='75' y2='92' stroke='currentColor' stroke-width='0.8' opacity='0.15' stroke-dasharray='3,3'/></svg>"#,
                         "110px"),
                        ("04", "함께 성장", "모든 토큰 홀더의 가치 상승", "#34d399",
                         // Biggest: massive ascending chart with ecosystem
                         r#"<svg viewBox='0 0 150 140' style='width:100%;height:100%;'><line x1='15' y1='120' x2='15' y2='10' stroke='currentColor' stroke-width='0.8' opacity='0.15'/><line x1='15' y1='120' x2='140' y2='120' stroke='currentColor' stroke-width='0.8' opacity='0.15'/><path d='M20,110 Q40,105 55,90 Q70,75 85,55 Q100,35 115,22 L135,12' fill='none' stroke='currentColor' stroke-width='2.5' opacity='0.7' stroke-linecap='round'/><path d='M20,110 Q40,105 55,90 Q70,75 85,55 Q100,35 115,22 L135,12 L140,12 L140,120 L20,120 Z' fill='currentColor' opacity='0.06'/><circle cx='55' cy='90' r='3' fill='currentColor' opacity='0.3'/><circle cx='85' cy='55' r='4' fill='currentColor' opacity='0.4'/><circle cx='115' cy='22' r='4' fill='currentColor' opacity='0.5'/><circle cx='135' cy='12' r='6' fill='currentColor' opacity='0.6' style='animation: pulseGreen 2s ease-in-out infinite;'/><polygon points='132,8 140,14 134,14' fill='currentColor' opacity='0.6'/><circle cx='45' cy='130' r='4' fill='currentColor' opacity='0.08'/><path d='M40,138 Q40,134 45,133 Q50,134 50,138' fill='currentColor' opacity='0.06'/><circle cx='75' cy='130' r='5' fill='currentColor' opacity='0.12'/><path d='M69,138 Q69,133 75,132 Q81,133 81,138' fill='currentColor' opacity='0.08'/><circle cx='105' cy='128' r='6' fill='currentColor' opacity='0.16'/><path d='M98,138 Q98,132 105,131 Q112,132 112,138' fill='currentColor' opacity='0.1'/></svg>"#,
                         "140px"),
                    ].iter().enumerate() {
                        {
                            let step_color = format!("color: {};", accent);
                            let _dummy: [&str; 4] = [
                                // Card 1: Credit card connected by dotted lines to cart, data particles flowing
                                r#"<rect x="30" y="50" width="100" height="65" rx="8" fill="none" stroke="currentColor" stroke-width="1.2" opacity="0.5"/>
<line x1="42" y1="68" x2="82" y2="68" stroke="currentColor" stroke-width="1" opacity="0.3"/>
<rect x="42" y="78" width="50" height="6" rx="2" fill="none" stroke="currentColor" stroke-width="0.8" opacity="0.25"/>
<rect x="42" y="90" width="30" height="6" rx="2" fill="none" stroke="currentColor" stroke-width="0.8" opacity="0.2"/>
<circle cx="115" cy="100" r="3" fill="currentColor" opacity="0.15"/>
<line x1="132" y1="82" x2="180" y2="82" stroke="currentColor" stroke-width="1" opacity="0.3" stroke-dasharray="4,4"/>
<circle cx="140" cy="82" r="2.5" fill="currentColor" opacity="0.6" style="animation: floatParticle 2s ease-in-out infinite alternate;"/>
<circle cx="155" cy="82" r="2" fill="currentColor" opacity="0.4" style="animation: floatParticle 2.5s ease-in-out 0.3s infinite alternate;"/>
<circle cx="168" cy="82" r="1.5" fill="currentColor" opacity="0.3" style="animation: floatParticle 2s ease-in-out 0.6s infinite alternate;"/>
<rect x="190" y="55" width="55" height="60" rx="6" fill="none" stroke="currentColor" stroke-width="1.2" opacity="0.4"/>
<line x1="200" y1="75" x2="235" y2="75" stroke="currentColor" stroke-width="0.8" opacity="0.2"/>
<circle cx="217" cy="90" r="8" fill="none" stroke="currentColor" stroke-width="1" opacity="0.3"/>
<line x1="213" y1="90" x2="221" y2="90" stroke="currentColor" stroke-width="0.8" opacity="0.3"/>
<line x1="217" y1="86" x2="217" y2="94" stroke="currentColor" stroke-width="0.8" opacity="0.3"/>
<circle cx="80" cy="140" r="3" fill="currentColor" opacity="0.1"/>
<line x1="80" y1="118" x2="80" y2="136" stroke="currentColor" stroke-width="0.8" opacity="0.15" stroke-dasharray="3,3"/>
<circle cx="217" cy="120" r="3" fill="currentColor" opacity="0.1"/>
<line x1="217" y1="118" x2="217" y2="136" stroke="currentColor" stroke-width="0.8" opacity="0.15" stroke-dasharray="3,3"/>"#,
                                // Card 2: Vault/safe with stacking coins, arrows showing inflow
                                r#"<rect x="95" y="45" width="90" height="75" rx="10" fill="none" stroke="currentColor" stroke-width="1.2" opacity="0.4"/>
<rect x="105" y="37" width="70" height="14" rx="5" fill="none" stroke="currentColor" stroke-width="1" opacity="0.3"/>
<circle cx="140" cy="80" r="14" fill="none" stroke="currentColor" stroke-width="1.5" opacity="0.5"/>
<circle cx="140" cy="80" r="5" fill="currentColor" opacity="0.15"/>
<rect x="138" y="80" width="4" height="12" rx="1.5" fill="currentColor" opacity="0.25"/>
<line x1="30" y1="60" x2="90" y2="70" stroke="currentColor" stroke-width="1" opacity="0.25" stroke-dasharray="4,4"/>
<polygon points="88,68 93,72 88,74" fill="currentColor" opacity="0.3"/>
<line x1="30" y1="90" x2="90" y2="85" stroke="currentColor" stroke-width="1" opacity="0.2" stroke-dasharray="4,4"/>
<polygon points="88,83 93,86 88,88" fill="currentColor" opacity="0.25"/>
<line x1="30" y1="120" x2="90" y2="100" stroke="currentColor" stroke-width="1" opacity="0.15" stroke-dasharray="4,4"/>
<polygon points="88,98 93,101 88,103" fill="currentColor" opacity="0.2"/>
<circle cx="25" cy="58" r="4" fill="currentColor" opacity="0.12"/>
<circle cx="22" cy="90" r="3.5" fill="currentColor" opacity="0.1"/>
<circle cx="25" cy="122" r="3" fill="currentColor" opacity="0.08"/>
<ellipse cx="220" cy="100" rx="18" ry="5" fill="none" stroke="currentColor" stroke-width="1" opacity="0.2"/>
<ellipse cx="220" cy="95" rx="18" ry="5" fill="none" stroke="currentColor" stroke-width="1" opacity="0.25"/>
<ellipse cx="220" cy="90" rx="18" ry="5" fill="none" stroke="currentColor" stroke-width="1" opacity="0.3"/>
<ellipse cx="220" cy="85" rx="18" ry="5" fill="none" stroke="currentColor" stroke-width="1" opacity="0.35"/>
<line x1="188" y1="82" x2="200" y2="88" stroke="currentColor" stroke-width="0.8" opacity="0.2" stroke-dasharray="3,3"/>"#,
                                // Card 3: Path/route with checkpoint nodes, verification flow
                                r#"<circle cx="35" cy="130" r="6" fill="none" stroke="currentColor" stroke-width="1.2" opacity="0.3"/>
<circle cx="35" cy="130" r="2" fill="currentColor" opacity="0.2"/>
<path d="M42,125 Q65,100 80,105 Q95,110 105,85 Q115,60 140,65 Q165,70 175,45 Q185,20 220,30" fill="none" stroke="currentColor" stroke-width="1.2" opacity="0.4" stroke-linecap="round"/>
<circle cx="80" cy="105" r="10" fill="none" stroke="currentColor" stroke-width="1" opacity="0.25"/>
<path d="M75,105 L79,109 L86,101" fill="none" stroke="currentColor" stroke-width="1.2" opacity="0.5"/>
<circle cx="105" cy="85" r="10" fill="none" stroke="currentColor" stroke-width="1" opacity="0.3"/>
<path d="M100,85 L104,89 L111,81" fill="none" stroke="currentColor" stroke-width="1.2" opacity="0.55"/>
<circle cx="140" cy="65" r="10" fill="none" stroke="currentColor" stroke-width="1" opacity="0.35"/>
<path d="M135,65 L139,69 L146,61" fill="none" stroke="currentColor" stroke-width="1.2" opacity="0.65"/>
<circle cx="175" cy="45" r="10" fill="none" stroke="currentColor" stroke-width="1" opacity="0.4"/>
<path d="M170,45 L174,49 L181,41" fill="none" stroke="currentColor" stroke-width="1.2" opacity="0.75"/>
<circle cx="220" cy="30" r="12" fill="none" stroke="currentColor" stroke-width="1.5" opacity="0.5"/>
<path d="M214,30 L219,35 L228,25" fill="none" stroke="currentColor" stroke-width="1.5" opacity="0.8"/>
<line x1="55" y1="140" x2="250" y2="140" stroke="currentColor" stroke-width="0.8" opacity="0.1"/>
<circle cx="80" cy="140" r="2" fill="currentColor" opacity="0.08"/>
<circle cx="140" cy="140" r="2" fill="currentColor" opacity="0.08"/>
<circle cx="200" cy="140" r="2" fill="currentColor" opacity="0.08"/>"#,
                                // Card 4: Ascending chart with nodes, connected to user icons
                                r#"<line x1="30" y1="140" x2="30" y2="20" stroke="currentColor" stroke-width="0.8" opacity="0.15"/>
<line x1="30" y1="140" x2="250" y2="140" stroke="currentColor" stroke-width="0.8" opacity="0.15"/>
<line x1="30" y1="110" x2="250" y2="110" stroke="currentColor" stroke-width="0.5" opacity="0.06"/>
<line x1="30" y1="80" x2="250" y2="80" stroke="currentColor" stroke-width="0.5" opacity="0.06"/>
<line x1="30" y1="50" x2="250" y2="50" stroke="currentColor" stroke-width="0.5" opacity="0.06"/>
<path d="M40,125 L80,110 L120,90 L160,65 L200,42 L240,25" fill="none" stroke="currentColor" stroke-width="1.5" opacity="0.6" stroke-linecap="round"/>
<circle cx="40" cy="125" r="4" fill="currentColor" opacity="0.15" stroke="currentColor" stroke-width="1" opacity="0.3"/>
<circle cx="80" cy="110" r="4" fill="currentColor" opacity="0.2" stroke="currentColor" stroke-width="1" opacity="0.35"/>
<circle cx="120" cy="90" r="5" fill="currentColor" opacity="0.25" stroke="currentColor" stroke-width="1" opacity="0.4"/>
<circle cx="160" cy="65" r="5" fill="currentColor" opacity="0.3" stroke="currentColor" stroke-width="1" opacity="0.45"/>
<circle cx="200" cy="42" r="5" fill="currentColor" opacity="0.35" stroke="currentColor" stroke-width="1" opacity="0.5"/>
<circle cx="240" cy="25" r="6" fill="currentColor" opacity="0.4" stroke="currentColor" stroke-width="1.2" opacity="0.6"/>
<polygon points="240,20 248,28 243,28" fill="currentColor" opacity="0.5"/>
<circle cx="80" cy="155" r="5" fill="none" stroke="currentColor" stroke-width="0.8" opacity="0.2"/>
<circle cx="80" cy="152" r="2" fill="none" stroke="currentColor" stroke-width="0.8" opacity="0.2"/>
<circle cx="140" cy="155" r="5" fill="none" stroke="currentColor" stroke-width="0.8" opacity="0.25"/>
<circle cx="140" cy="152" r="2" fill="none" stroke="currentColor" stroke-width="0.8" opacity="0.25"/>
<circle cx="200" cy="155" r="5" fill="none" stroke="currentColor" stroke-width="0.8" opacity="0.3"/>
<circle cx="200" cy="152" r="2" fill="none" stroke="currentColor" stroke-width="0.8" opacity="0.3"/>
<line x1="80" y1="148" x2="80" y2="140" stroke="currentColor" stroke-width="0.8" opacity="0.15" stroke-dasharray="2,2"/>
<line x1="140" y1="148" x2="140" y2="140" stroke="currentColor" stroke-width="0.8" opacity="0.15" stroke-dasharray="2,2"/>
<line x1="200" y1="148" x2="200" y2="140" stroke="currentColor" stroke-width="0.8" opacity="0.15" stroke-dasharray="2,2"/>"#,
                            ];
                            let svg_str = svg_content.to_string();
                            let size_style = format!("width: {}; height: {};", node_size, node_size);
                            let glow_bg = format!("background: {}; filter: blur(30px); opacity: {};", accent, 0.08 + si as f64 * 0.04);
                            rsx! {
                                div {
                                    class: "text-center px-3 py-6 flex flex-col items-center reveal-bounce",
                                    // SVG diagram (grows with each step)
                                    div {
                                        class: "relative mx-auto mb-5",
                                        style: "{size_style}",
                                        // Glow behind (bigger each step)
                                        div {
                                            class: "absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 rounded-full",
                                            style: "{glow_bg} width: 100%; height: 100%;",
                                        }
                                        div {
                                            style: "color: {accent}; width: 100%; height: 100%;",
                                            dangerous_inner_html: "{svg_str}",
                                        }
                                    }
                                    p {
                                        class: "text-xs font-bold mb-1 tracking-widest",
                                        style: "{step_color}",
                                        "STEP {step}"
                                    }
                                    h3 {
                                        class: "text-base font-bold mb-2",
                                        style: "color: #e8eefc;",
                                        "{title}"
                                    }
                                    p {
                                        class: "text-xs leading-relaxed",
                                        style: "color: #7a8ba6;",
                                        "{desc}"
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
