use dioxus::prelude::*;

// ── Native RSX SVG Components ──

#[component]
pub(super) fn ShoeIcon() -> Element {
    rsx! {
        svg {
            view_box: "0 0 120 60",
            path {
                d: "M15,45 Q15,30 25,25 Q35,20 50,18 L75,16 Q85,15 95,18 Q105,22 110,30 L112,38 L112,42 Q112,48 105,48 L20,48 Q15,48 15,45 Z",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "1.8",
                opacity: "0.7",
            }
            path {
                d: "M12,48 L115,48 Q115,55 108,55 L18,55 Q12,55 12,48 Z",
                fill: "currentColor",
                opacity: "0.12",
                stroke: "currentColor",
                stroke_width: "1.5",
            }
            path {
                d: "M50,20 L58,17 M60,22 L68,19 M70,23 L78,20",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "1",
                opacity: "0.4",
            }
        }
    }
}

#[component]
pub(super) fn CoffeeIcon() -> Element {
    rsx! {
        svg {
            view_box: "0 0 120 60",
            path {
                d: "M35,15 Q35,10 45,10 L75,10 Q85,10 85,15 L82,45 Q82,50 72,50 L48,50 Q38,50 38,45 Z",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "1.8",
                opacity: "0.7",
            }
            path {
                d: "M85,20 Q100,20 100,32 Q100,42 85,42",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "1.5",
                opacity: "0.5",
            }
            path {
                d: "M50,5 Q48,0 52,-5",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "1",
                opacity: "0.3",
            }
            path {
                d: "M60,3 Q58,-3 62,-8",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "1",
                opacity: "0.25",
            }
            path {
                d: "M70,5 Q68,0 72,-5",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "1",
                opacity: "0.3",
            }
        }
    }
}

#[component]
pub(super) fn FashionIcon() -> Element {
    rsx! {
        svg {
            view_box: "0 0 120 80",
            circle { cx: "58", cy: "12", r: "7", fill: "currentColor", opacity: "0.6" }
            path {
                d: "M52,20 L52,28 Q52,32 48,36 L40,44 L40,48 Q46,46 50,42 L56,36 L56,48 L50,62 L46,68 L50,70 L56,56 L60,48 L64,56 L70,70 L74,68 L70,62 L64,48 L64,36 L68,40 Q72,44 76,42 L80,38 L78,34 Q74,36 70,34 L64,28 L64,20 Z",
                fill: "currentColor",
                opacity: "0.5",
            }
        }
    }
}

#[component]
pub(super) fn FloorPriceChart() -> Element {
    rsx! {
        svg {
            view_box: "0 0 100 50",
            fill: "none",
            style: "width:100%;height:100%;",
            path {
                d: "M5,42 L20,38 L35,35 L50,28 L65,22 L80,15 L95,8",
                stroke: "currentColor",
                stroke_width: "2",
                opacity: "0.7",
                stroke_linecap: "round",
            }
            path {
                d: "M5,42 L20,38 L35,35 L50,28 L65,22 L80,15 L95,8 L95,48 L5,48 Z",
                fill: "currentColor",
                opacity: "0.08",
            }
            circle { cx: "50", cy: "28", r: "2.5", fill: "currentColor", opacity: "0.5" }
            circle { cx: "80", cy: "15", r: "2.5", fill: "currentColor", opacity: "0.6" }
            circle { cx: "95", cy: "8", r: "3", fill: "currentColor", opacity: "0.8" }
        }
    }
}

#[component]
pub(super) fn TreasuryBars() -> Element {
    rsx! {
        svg {
            view_box: "0 0 100 50",
            fill: "none",
            style: "width:100%;height:100%;",
            rect { x: "8", y: "38", width: "10", height: "10", rx: "2", fill: "currentColor", opacity: "0.15" }
            rect { x: "22", y: "32", width: "10", height: "16", rx: "2", fill: "currentColor", opacity: "0.2" }
            rect { x: "36", y: "26", width: "10", height: "22", rx: "2", fill: "currentColor", opacity: "0.25" }
            rect { x: "50", y: "20", width: "10", height: "28", rx: "2", fill: "currentColor", opacity: "0.3" }
            rect { x: "64", y: "14", width: "10", height: "34", rx: "2", fill: "currentColor", opacity: "0.4" }
            rect { x: "78", y: "6", width: "10", height: "42", rx: "2", fill: "currentColor", opacity: "0.5" }
        }
    }
}

#[component]
pub(super) fn HoldersChart() -> Element {
    rsx! {
        svg {
            view_box: "0 0 100 50",
            fill: "none",
            style: "width:100%;height:100%;",
            circle { cx: "20", cy: "15", r: "5", fill: "currentColor", opacity: "0.2" }
            path { d: "M12,30 Q12,24 20,22 Q28,24 28,30", fill: "currentColor", opacity: "0.15" }
            circle { cx: "50", cy: "13", r: "6", fill: "currentColor", opacity: "0.3" }
            path { d: "M41,30 Q41,23 50,21 Q59,23 59,30", fill: "currentColor", opacity: "0.2" }
            circle { cx: "80", cy: "10", r: "7", fill: "currentColor", opacity: "0.45" }
            path { d: "M70,30 Q70,22 80,20 Q90,22 90,30", fill: "currentColor", opacity: "0.3" }
            path {
                d: "M15,38 L30,36 L50,33 L70,30 L90,26",
                stroke: "currentColor",
                stroke_width: "1",
                opacity: "0.3",
                stroke_dasharray: "3,3",
            }
            polygon { points: "88,24 93,27 88,29", fill: "currentColor", opacity: "0.3" }
        }
    }
}

#[component]
pub(super) fn TransformArrow() -> Element {
    rsx! {
        svg {
            view_box: "0 0 80 80",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            style: "width:72px;height:72px;",
            circle {
                cx: "40",
                cy: "40",
                r: "38",
                fill: "rgba(0,212,170,0.12)",
                stroke: "#00d4aa",
                stroke_width: "2.5",
                style: "animation: glowPulse 2s ease-in-out infinite;",
            }
            circle {
                cx: "40",
                cy: "40",
                r: "32",
                fill: "none",
                stroke: "#00d4aa",
                stroke_width: "0.8",
                opacity: "0.25",
                stroke_dasharray: "4,3",
            }
            path {
                d: "M22 40 H54 M46 30 L56 40 L46 50",
                stroke: "#00d4aa",
                stroke_width: "3",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}

// ── SVG String Constants (complex SVGs with <text>, <defs>, <marker>) ──

pub(super) const REVENUE_DONUT_SVG: &str = r#"<svg viewBox='0 0 100 50' fill='none' style='width:100%;height:100%;'><circle cx='50' cy='25' r='18' fill='none' stroke='currentColor' stroke-width='3' opacity='0.12'/><circle cx='50' cy='25' r='18' fill='none' stroke='currentColor' stroke-width='3' opacity='0.5' stroke-dasharray='85,113' stroke-dashoffset='0' stroke-linecap='round'/><circle cx='50' cy='25' r='12' fill='none' stroke='currentColor' stroke-width='2' opacity='0.08'/><text x='50' y='28' text-anchor='middle' fill='currentColor' font-size='8' font-weight='bold' opacity='0.6'>75%</text></svg>"#;

pub(super) const BEFORE_DIAGRAM_SVG: &str = r##"<svg viewBox="0 0 280 260" fill="none" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:280px;height:auto;">
  <polygon points="140,10 170,25 170,55 140,70 110,55 110,25" fill="rgba(239,68,68,0.12)" stroke="#ef4444" stroke-width="1.5" opacity="0.7" style="animation: pulseRed 3s ease-in-out infinite;transform-origin:140px 40px;"/>
  <text x="140" y="44" text-anchor="middle" fill="#ef4444" font-size="11" font-weight="bold">프로젝트</text>
  <text x="140" y="82" text-anchor="middle" fill="#ef4444" font-size="9" opacity="0.6">실적 없는 프로젝트</text>
  <line x1="120" y1="60" x2="65" y2="100" stroke="#ef4444" stroke-width="1" stroke-dasharray="4,4" opacity="0.4" style="animation: dashFlow 2s linear infinite;"/>
  <line x1="140" y1="70" x2="140" y2="100" stroke="#ef4444" stroke-width="1" stroke-dasharray="4,4" opacity="0.35" style="animation: dashFlow 2.5s linear infinite;"/>
  <line x1="160" y1="60" x2="215" y2="100" stroke="#ef4444" stroke-width="1" stroke-dasharray="4,4" opacity="0.3" style="animation: dashFlow 3s linear infinite;"/>
  <line x1="90" y1="82" x2="100" y2="88" stroke="#ef4444" stroke-width="0.8" opacity="0.15" stroke-dasharray="2,3"/>
  <line x1="180" y1="78" x2="192" y2="85" stroke="#ef4444" stroke-width="0.8" opacity="0.12" stroke-dasharray="2,3"/>
  <g style="animation: pulseRed 2.5s ease-in-out infinite;transform-origin:60px 115px;">
    <circle cx="60" cy="108" r="7" fill="none" stroke="#ef4444" stroke-width="1" opacity="0.4"/>
    <path d="M48,130 Q48,120 60,118 Q72,120 72,130" fill="none" stroke="#ef4444" stroke-width="1" opacity="0.3"/>
  </g>
  <g style="animation: pulseRed 3s ease-in-out 0.5s infinite;transform-origin:140px 115px;">
    <circle cx="140" cy="108" r="7" fill="none" stroke="#ef4444" stroke-width="1" opacity="0.3"/>
    <path d="M128,130 Q128,120 140,118 Q152,120 152,130" fill="none" stroke="#ef4444" stroke-width="1" opacity="0.2"/>
  </g>
  <g style="animation: pulseRed 3.5s ease-in-out 1s infinite;transform-origin:220px 115px;">
    <circle cx="220" cy="108" r="7" fill="none" stroke="#ef4444" stroke-width="1" opacity="0.2"/>
    <path d="M208,130 Q208,120 220,118 Q232,120 232,130" fill="none" stroke="#ef4444" stroke-width="1" opacity="0.12"/>
  </g>
  <text x="60" y="148" text-anchor="middle" fill="#ef4444" font-size="8" opacity="0.5">피해 반복</text>
  <line x1="30" y1="160" x2="30" y2="245" stroke="rgba(239,68,68,0.3)" stroke-width="1"/>
  <line x1="30" y1="245" x2="255" y2="245" stroke="rgba(239,68,68,0.3)" stroke-width="1"/>
  <line x1="30" y1="185" x2="255" y2="185" stroke="rgba(239,68,68,0.06)" stroke-width="0.5"/>
  <line x1="30" y1="210" x2="255" y2="210" stroke="rgba(239,68,68,0.06)" stroke-width="0.5"/>
  <path d="M30,170 Q70,172 100,182 Q140,196 180,212 Q220,228 255,240 L255,245 L30,245 Z" fill="rgba(239,68,68,0.06)"/>
  <path d="M30,170 Q70,172 100,182 Q140,196 180,212 Q220,228 255,240" stroke="#ef4444" stroke-width="2" fill="none" stroke-linecap="round" stroke-dasharray="300" style="animation: lineGrow 4s ease-out infinite;"/>
  <path d="M245,225 L250,233 L240,233 Z" fill="#ef4444" opacity="0.4"/>
  <text x="245" y="220" text-anchor="end" fill="#ef4444" font-size="9" opacity="0.5">가치 하락</text>
</svg>"##;

pub(super) const AFTER_DIAGRAM_SVG: &str = r##"<svg viewBox="0 0 280 260" fill="none" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:280px;height:auto;">
  <defs>
    <marker id="arrowG" markerWidth="6" markerHeight="4" refX="5" refY="2" orient="auto">
      <path d="M0,0 L6,2 L0,4" fill="#00d4aa" opacity="0.5"/>
    </marker>
  </defs>
  <g style="animation: nodeFloat 3s ease-in-out infinite;">
  <path d="M120,8 L160,8 L165,13 L165,48 L158,52 L152,48 L146,52 L140,48 L134,52 L128,48 L122,52 L115,48 L115,13 Z" fill="rgba(0,212,170,0.12)" stroke="#00d4aa" stroke-width="1.5" opacity="0.7"/>
  <line x1="125" y1="20" x2="155" y2="20" stroke="#00d4aa" stroke-width="0.8" opacity="0.3"/>
  <line x1="125" y1="27" x2="150" y2="27" stroke="#00d4aa" stroke-width="0.8" opacity="0.25"/>
  <line x1="125" y1="34" x2="145" y2="34" stroke="#00d4aa" stroke-width="0.8" opacity="0.2"/>
  <text x="140" y="46" text-anchor="middle" fill="#00d4aa" font-size="9" font-weight="bold" opacity="0.8">매출</text>
  <text x="172" y="30" text-anchor="start" fill="#00d4aa" font-size="8" opacity="0.55">실제 매출</text>
  </g>
  <line x1="140" y1="53" x2="140" y2="68" stroke="#00d4aa" stroke-width="1.5" opacity="0.6" marker-end="url(#arrowG)"/>
  <path d="M105,70 L175,70 L175,100 Q175,115 140,120 Q105,115 105,100 Z" fill="rgba(0,212,170,0.1)" stroke="#00d4aa" stroke-width="2" opacity="0.7" style="animation: fadeInOut 4s ease-in-out infinite;filter:drop-shadow(0 0 8px rgba(0,212,170,0.3));"/>
  <path d="M120,85 L135,85 L135,100 L120,100 Z" fill="none" stroke="#00d4aa" stroke-width="1" opacity="0.35"/>
  <path d="M145,85 L160,85 L160,100 L145,100 Z" fill="none" stroke="#00d4aa" stroke-width="1" opacity="0.35"/>
  <circle cx="140" cy="92" r="5" fill="none" stroke="#00d4aa" stroke-width="1" opacity="0.4"/>
  <circle cx="140" cy="92" r="2" fill="#00d4aa" opacity="0.25"/>
  <text x="140" y="114" text-anchor="middle" fill="#00d4aa" font-size="9" font-weight="bold" opacity="0.7">트레저리</text>
  <text x="182" y="92" text-anchor="start" fill="#00d4aa" font-size="8" opacity="0.5">온체인 트레저리</text>
  <line x1="115" y1="115" x2="62" y2="142" stroke="#00d4aa" stroke-width="1.2" opacity="0.5" marker-end="url(#arrowG)" stroke-dasharray="6,4" style="animation: dashFlow 2s linear infinite;"/>
  <line x1="140" y1="120" x2="140" y2="142" stroke="#00d4aa" stroke-width="1.2" opacity="0.5" marker-end="url(#arrowG)" stroke-dasharray="6,4" style="animation: dashFlow 2.5s linear infinite;"/>
  <line x1="165" y1="115" x2="218" y2="142" stroke="#00d4aa" stroke-width="1.2" opacity="0.5" marker-end="url(#arrowG)" stroke-dasharray="6,4" style="animation: dashFlow 3s linear infinite;"/>
  <g style="animation: pulseGreen 3s ease-in-out infinite;transform-origin:55px 160px;">
    <circle cx="55" cy="150" r="6" fill="none" stroke="#00d4aa" stroke-width="1.2" opacity="0.6"/>
    <path d="M44,168 Q44,160 55,158 Q66,160 66,168" fill="rgba(0,212,170,0.12)" stroke="#00d4aa" stroke-width="1" opacity="0.5"/>
    <circle cx="55" cy="150" r="10" fill="none" stroke="#00d4aa" stroke-width="0.5" opacity="0.15"/>
    <text x="55" y="180" text-anchor="middle" fill="#00d4aa" font-size="7" opacity="0.5">토큰 홀더</text>
  </g>
  <g style="animation: pulseGreen 3s ease-in-out 0.5s infinite;transform-origin:140px 158px;">
    <circle cx="140" cy="148" r="7" fill="none" stroke="#00d4aa" stroke-width="1.2" opacity="0.7"/>
    <path d="M128,168 Q128,159 140,157 Q152,159 152,168" fill="rgba(0,212,170,0.15)" stroke="#00d4aa" stroke-width="1" opacity="0.6"/>
  <circle cx="140" cy="148" r="12" fill="none" stroke="#00d4aa" stroke-width="0.5" opacity="0.18"/>
  <text x="140" y="180" text-anchor="middle" fill="#00d4aa" font-size="7" opacity="0.5">토큰 홀더</text>
  </g>
  <g style="animation: pulseGreen 3s ease-in-out 1s infinite;transform-origin:225px 157px;">
    <circle cx="225" cy="147" r="8" fill="none" stroke="#00d4aa" stroke-width="1.2" opacity="0.8"/>
    <path d="M212,168 Q212,158 225,156 Q238,158 238,168" fill="rgba(0,212,170,0.18)" stroke="#00d4aa" stroke-width="1" opacity="0.7"/>
    <circle cx="225" cy="147" r="14" fill="none" stroke="#00d4aa" stroke-width="0.5" opacity="0.2"/>
    <text x="225" y="180" text-anchor="middle" fill="#00d4aa" font-size="7" opacity="0.5">토큰 홀더</text>
  </g>
  <text x="140" y="192" text-anchor="middle" fill="#00d4aa" font-size="8" opacity="0.55">가치 상승</text>
  <line x1="30" y1="205" x2="30" y2="248" stroke="rgba(0,212,170,0.3)" stroke-width="1"/>
  <line x1="30" y1="248" x2="255" y2="248" stroke="rgba(0,212,170,0.3)" stroke-width="1"/>
  <line x1="30" y1="235" x2="255" y2="235" stroke="rgba(0,212,170,0.05)" stroke-width="0.5"/>
  <line x1="30" y1="222" x2="255" y2="222" stroke="rgba(0,212,170,0.05)" stroke-width="0.5"/>
  <path d="M30,246 Q70,244 110,238 Q150,230 190,222 Q230,216 255,210 L255,248 L30,248 Z" fill="rgba(0,212,170,0.08)"/>
  <path d="M30,246 Q70,244 110,238 Q150,230 190,222 Q230,216 255,210" stroke="#00d4aa" stroke-width="2" fill="none" stroke-linecap="round" stroke-dasharray="300" style="animation: lineGrow 4s ease-out infinite;"/>
  <path d="M248,215 L253,207 L258,215 Z" fill="#00d4aa" opacity="0.5"/>
  <text x="245" y="205" text-anchor="end" fill="#00d4aa" font-size="9" opacity="0.55">우상향</text>
</svg>"##;

pub(super) const BEFORE_CYCLE_SVG: &str = r#"<svg viewBox='0 0 280 280' fill='none' style='width:100%;height:100%;'>
                                    <!-- Circular arrow path -->
                                    <circle cx='140' cy='140' r='100' fill='none' stroke='currentColor' stroke-width='1' opacity='0.15' stroke-dasharray='6,6'/>
                                    <!-- Step 1: top -->
                                    <circle cx='140' cy='40' r='28' fill='rgba(239,68,68,0.08)' stroke='currentColor' stroke-width='1.5' opacity='0.5'/>
                                    <text x='140' y='37' text-anchor='middle' fill='currentColor' font-size='8' font-weight='bold' opacity='0.7'>광고비</text>
                                    <text x='140' y='49' text-anchor='middle' fill='currentColor' font-size='8' opacity='0.5'>1억 지출</text>
                                    <!-- Arrow 1→2 -->
                                    <path d='M165,55 Q200,70 215,105' fill='none' stroke='currentColor' stroke-width='1.2' opacity='0.3' stroke-dasharray='4,3' style='animation: dashFlow 2s linear infinite;'/>
                                    <polygon points='213,100 218,108 210,106' fill='currentColor' opacity='0.3'/>
                                    <!-- Step 2: right -->
                                    <circle cx='230' cy='140' r='28' fill='rgba(239,68,68,0.08)' stroke='currentColor' stroke-width='1.5' opacity='0.5'/>
                                    <text x='230' y='137' text-anchor='middle' fill='currentColor' font-size='8' font-weight='bold' opacity='0.7'>고객 유치</text>
                                    <text x='230' y='149' text-anchor='middle' fill='currentColor' font-size='8' opacity='0.5'>일시적</text>
                                    <!-- Arrow 2→3 -->
                                    <path d='M215,168 Q200,205 165,225' fill='none' stroke='currentColor' stroke-width='1.2' opacity='0.3' stroke-dasharray='4,3' style='animation: dashFlow 2.5s linear infinite;'/>
                                    <polygon points='168,222 163,228 160,220' fill='currentColor' opacity='0.3'/>
                                    <!-- Step 3: bottom -->
                                    <circle cx='140' cy='240' r='28' fill='rgba(239,68,68,0.08)' stroke='currentColor' stroke-width='1.5' opacity='0.5'/>
                                    <text x='140' y='237' text-anchor='middle' fill='currentColor' font-size='8' font-weight='bold' opacity='0.7'>고객 이탈</text>
                                    <text x='140' y='249' text-anchor='middle' fill='currentColor' font-size='8' opacity='0.5'>재방문 X</text>
                                    <!-- Arrow 3→4 -->
                                    <path d='M115,225 Q80,205 65,168' fill='none' stroke='currentColor' stroke-width='1.2' opacity='0.3' stroke-dasharray='4,3' style='animation: dashFlow 3s linear infinite;'/>
                                    <polygon points='67,172 62,165 70,167' fill='currentColor' opacity='0.3'/>
                                    <!-- Step 4: left -->
                                    <circle cx='50' cy='140' r='28' fill='rgba(239,68,68,0.08)' stroke='currentColor' stroke-width='1.5' opacity='0.5'/>
                                    <text x='50' y='137' text-anchor='middle' fill='currentColor' font-size='8' font-weight='bold' opacity='0.7'>또 광고</text>
                                    <text x='50' y='149' text-anchor='middle' fill='currentColor' font-size='8' opacity='0.5'>반복 지출</text>
                                    <!-- Arrow 4→1 -->
                                    <path d='M65,112 Q80,75 115,55' fill='none' stroke='currentColor' stroke-width='1.2' opacity='0.3' stroke-dasharray='4,3' style='animation: dashFlow 2s linear infinite;'/>
                                    <polygon points='112,58 117,52 118,60' fill='currentColor' opacity='0.3'/>
                                    <!-- Center: ring -->
                                    <circle cx='140' cy='140' r='35' fill='none' stroke='rgba(239,68,68,0.2)' stroke-width='1.5'/>
                                    <circle cx='140' cy='140' r='30' fill='none' stroke='rgba(239,68,68,0.08)' stroke-width='0.5' stroke-dasharray='4,3'/>
                                    <text x='140' y='136' text-anchor='middle' fill='currentColor' font-size='10' font-weight='bold' opacity='0.5'>돈이</text>
                                    <text x='140' y='150' text-anchor='middle' fill='currentColor' font-size='10' font-weight='bold' opacity='0.5'>빠져나감</text>
                                </svg>"#;

pub(super) const AFTER_CYCLE_SVG: &str = r#"<svg viewBox='0 0 280 280' fill='none' style='width:100%;height:100%;'>
                                    <!-- Circular arrow path -->
                                    <circle cx='140' cy='140' r='100' fill='none' stroke='currentColor' stroke-width='1.5' opacity='0.2'/>
                                    <!-- Step 1: top -->
                                    <circle cx='140' cy='40' r='28' fill='rgba(0,212,170,0.1)' stroke='currentColor' stroke-width='1.5' opacity='0.6'/>
                                    <text x='140' y='37' text-anchor='middle' fill='currentColor' font-size='8' font-weight='bold' opacity='0.8'>매출 발생</text>
                                    <text x='140' y='49' text-anchor='middle' fill='currentColor' font-size='8' opacity='0.6'>2~4% 적립</text>
                                    <!-- Arrow 1→2 (glowing) -->
                                    <path d='M165,55 Q200,70 215,105' fill='none' stroke='currentColor' stroke-width='1.5' opacity='0.5' style='animation: dashFlow 2s linear infinite;'/>
                                    <polygon points='213,100 218,108 210,106' fill='currentColor' opacity='0.5'/>
                                    <!-- Step 2: right -->
                                    <circle cx='230' cy='140' r='28' fill='rgba(0,212,170,0.1)' stroke='currentColor' stroke-width='1.5' opacity='0.6'/>
                                    <text x='230' y='137' text-anchor='middle' fill='currentColor' font-size='8' font-weight='bold' opacity='0.8'>토큰 가치</text>
                                    <text x='230' y='149' text-anchor='middle' fill='currentColor' font-size='8' opacity='0.6'>자동 상승</text>
                                    <!-- Arrow 2→3 -->
                                    <path d='M215,168 Q200,205 165,225' fill='none' stroke='currentColor' stroke-width='1.5' opacity='0.5' style='animation: dashFlow 2.5s linear infinite;'/>
                                    <polygon points='168,222 163,228 160,220' fill='currentColor' opacity='0.5'/>
                                    <!-- Step 3: bottom -->
                                    <circle cx='140' cy='240' r='28' fill='rgba(0,212,170,0.1)' stroke='currentColor' stroke-width='1.5' opacity='0.6'/>
                                    <text x='140' y='237' text-anchor='middle' fill='currentColor' font-size='8' font-weight='bold' opacity='0.8'>고객 홍보</text>
                                    <text x='140' y='249' text-anchor='middle' fill='currentColor' font-size='8' opacity='0.6'>입소문 확산</text>
                                    <!-- Arrow 3→4 -->
                                    <path d='M115,225 Q80,205 65,168' fill='none' stroke='currentColor' stroke-width='1.5' opacity='0.5' style='animation: dashFlow 3s linear infinite;'/>
                                    <polygon points='67,172 62,165 70,167' fill='currentColor' opacity='0.5'/>
                                    <!-- Step 4: left -->
                                    <circle cx='50' cy='140' r='28' fill='rgba(0,212,170,0.1)' stroke='currentColor' stroke-width='1.5' opacity='0.6'/>
                                    <text x='50' y='137' text-anchor='middle' fill='currentColor' font-size='8' font-weight='bold' opacity='0.8'>신규 고객</text>
                                    <text x='50' y='149' text-anchor='middle' fill='currentColor' font-size='8' opacity='0.6'>자연 유입</text>
                                    <!-- Arrow 4→1 -->
                                    <path d='M65,112 Q80,75 115,55' fill='none' stroke='currentColor' stroke-width='1.5' opacity='0.5' style='animation: dashFlow 2s linear infinite;'/>
                                    <polygon points='112,58 117,52 118,60' fill='currentColor' opacity='0.5'/>
                                    <!-- Center: ring -->
                                    <circle cx='140' cy='140' r='35' fill='none' stroke='rgba(0,212,170,0.2)' stroke-width='1.5'/>
                                    <circle cx='140' cy='140' r='30' fill='none' stroke='rgba(0,212,170,0.08)' stroke-width='0.5' stroke-dasharray='4,3'/>
                                    <text x='140' y='132' text-anchor='middle' fill='currentColor' font-size='11' font-weight='bold' opacity='0.7'>가치가</text>
                                    <text x='140' y='148' text-anchor='middle' fill='currentColor' font-size='11' font-weight='bold' opacity='0.7'>순환한다</text>
                                </svg>"#;
