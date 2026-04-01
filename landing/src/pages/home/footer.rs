use dioxus::prelude::*;
use crate::Route;

#[component]
pub(super) fn Footer() -> Element {
    rsx! {
        footer {
            style: "background: #0c1018; border-top: 1px solid rgba(0,212,170,0.08);",
            div {
                class: "max-w-6xl mx-auto px-4 py-12",
                div {
                    class: "grid grid-cols-1 md:grid-cols-3 gap-8 mb-8",
                    div {
                        div {
                            class: "flex items-center gap-2 mb-4",
                            img {
                                src: asset!("/assets/biyard-logo.png"),
                                alt: "Biyard",
                                style: "width: 24px; height: 24px;",
                            }
                            span {
                                class: "text-lg font-bold",
                                style: "color: #00d4aa;",
                                "Biyard"
                            }
                        }
                        p {
                            class: "text-sm leading-relaxed",
                            style: "color: #7a8ba6;",
                            "Revenue-backed token infrastructure for brands. Building trust through transparency."
                        }
                    }
                    div {
                        p {
                            class: "text-sm font-semibold mb-3",
                            style: "color: #e8eefc;",
                            "Links"
                        }
                        div {
                            class: "space-y-2",
                            Link {
                                to: Route::Team {},
                                class: "block text-sm hover:text-white transition-colors",
                                style: "color: #7a8ba6;",
                                "Team"
                            }
                            a {
                                href: "https://github.com/biyard",
                                target: "_blank",
                                class: "block text-sm",
                                style: "color: #7a8ba6;",
                                "GitHub"
                            }
                            a {
                                href: "https://www.linkedin.com/company/75498162",
                                target: "_blank",
                                class: "block text-sm",
                                style: "color: #7a8ba6;",
                                "LinkedIn"
                            }
                        }
                    }
                    div {
                        p {
                            class: "text-sm font-semibold mb-3",
                            style: "color: #e8eefc;",
                            "Contact"
                        }
                        div {
                            class: "space-y-2",
                            a {
                                href: "mailto:finance@biyard.co",
                                class: "block text-sm",
                                style: "color: #7a8ba6;",
                                "finance@biyard.co"
                            }
                        }
                    }
                }
                div {
                    class: "pt-6 flex items-center justify-between",
                    style: "border-top: 1px solid rgba(255,255,255,0.06);",
                    p {
                        class: "text-xs",
                        style: "color: #4a5568;",
                        "\u{00A9} 2026 Biyard Corp. All rights reserved."
                    }
                    div {
                        class: "flex gap-4",
                        a {
                            href: "https://github.com/biyard",
                            target: "_blank",
                            class: "text-xs",
                            style: "color: #4a5568;",
                            "GitHub"
                        }
                        a {
                            href: "https://www.linkedin.com/company/75498162",
                            target: "_blank",
                            class: "text-xs",
                            style: "color: #4a5568;",
                            "LinkedIn"
                        }
                    }
                }
            }
        }
    }
}
