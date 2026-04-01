use dioxus::prelude::*;
use crate::Route;

struct TeamMember {
    name: &'static str,
    role: &'static str,
    bio: &'static str,
    email: &'static str,
    color: &'static str,
}

const TEAM: &[TeamMember] = &[
    TeamMember {
        name: "Summer",
        role: "Founder & CEO",
        bio: "Korean Web3 industry educator, specializing in industry convergence and DAO technology. Venture Partner at Simsan Ventures, a London-based VC focused on early stage and deep tech.",
        email: "summer@biyard.co",
        color: "#60a5fa",
    },
    TeamMember {
        name: "Miner",
        role: "CEO & CTO",
        bio: "PhD in cryptography and computer security. Led development of messenger for a game platform with 75M concurrent users. Development leader of an EVM-based blockchain cloud platform.",
        email: "miner@biyard.co",
        color: "#a78bfa",
    },
    TeamMember {
        name: "Rosa",
        role: "Senior Researcher",
        bio: "Senior Researcher with over 10 years of experience in developing and managing projects. Bachelor's and Master's from Korea University, Master's in Technology Management from KAIST.",
        email: "rosa@biyard.co",
        color: "#f472b6",
    },
    TeamMember {
        name: "Ria",
        role: "UI/UX Designer",
        bio: "Talented UI/UX designer focused on creating intuitive and beautiful user experiences for Web3 products.",
        email: "ria@biyard.co",
        color: "#fbbf24",
    },
    TeamMember {
        name: "Victor",
        role: "Software Engineer",
        bio: "Computer science-trained developer specializing in web/mobile frontend and blockchain smart contracts. Leads development of Biyard's flagship services.",
        email: "victor@biyard.co",
        color: "#34d399",
    },
    TeamMember {
        name: "Ryan",
        role: "Software Engineer",
        bio: "Developer who majored in Computer Science, primarily develops smart contracts and web applications on blockchain.",
        email: "ryan@biyard.co",
        color: "#38bdf8",
    },
    TeamMember {
        name: "JH",
        role: "Project Manager",
        bio: "Responsible for managing project timelines and cross-functional coordination. Also handles company investments and partnerships for strategic growth.",
        email: "jhpark@biyard.co",
        color: "#a78bfa",
    },
];

#[component]
pub fn Team() -> Element {
    rsx! {
        div {
            style: "min-height: 100vh; background: #0c1018; color: #e8eefc; font-family: 'Outfit', 'Noto Sans KR', sans-serif;",

            // Header
            div {
                style: "background: #0c1018; border-bottom: 1px solid rgba(0,212,170,0.08); padding: 16px 24px;",
                div {
                    class: "max-w-6xl mx-auto flex items-center justify-between",
                    Link {
                        to: Route::Home {},
                        class: "flex items-center gap-2",
                        img {
                            src: asset!("/assets/biyard-logo.png"),
                            alt: "Biyard",
                            style: "width: 28px; height: 28px;",
                        }
                        span {
                            class: "text-xl font-extrabold",
                            style: "color: #00d4aa;",
                            "Biyard"
                        }
                    }
                    Link {
                        to: Route::Home {},
                        class: "text-sm",
                        style: "color: #7a8ba6;",
                        "\u{2190} Back"
                    }
                }
            }

            // Team content
            div {
                class: "max-w-5xl mx-auto px-4 py-20",
                div {
                    class: "text-center mb-16",
                    p {
                        class: "text-sm font-semibold tracking-widest uppercase mb-3",
                        style: "color: #00d4aa;",
                        "OUR TEAM"
                    }
                    h1 {
                        class: "text-4xl md:text-5xl mb-4",
                        style: "color: #e8eefc; font-weight: 300; letter-spacing: -0.02em;",
                        "Building the Future of "
                        span {
                            style: "background-image: linear-gradient(to right, #60a5fa, #a78bfa); -webkit-background-clip: text; background-clip: text; color: transparent; font-weight: 600;",
                            "Token Economy"
                        }
                    }
                    p {
                        class: "text-lg max-w-2xl mx-auto",
                        style: "color: #7a8ba6;",
                        "Blockchain, cryptography, and Web3 experts building a trustworthy revenue-backed token platform."
                    }
                }

                // Team grid
                div {
                    class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                    for member in TEAM.iter() {
                        div {
                            class: "rounded-2xl p-6 relative overflow-hidden",
                            style: "background: rgba(10,16,26,0.5); backdrop-filter: blur(16px); -webkit-backdrop-filter: blur(16px); border: 1px solid rgba(255,255,255,0.06); box-shadow: 0 8px 32px rgba(0,0,0,0.2);",
                            // Top glow
                            div {
                                class: "absolute top-0 left-[10%] right-[10%] h-[1px]",
                                style: "background: linear-gradient(90deg, transparent, {member.color}, transparent); opacity: 0.3;",
                            }
                            div {
                                class: "flex items-start gap-4",
                                // Avatar
                                div {
                                    class: "w-16 h-16 rounded-full flex items-center justify-center text-2xl font-bold flex-shrink-0",
                                    style: "background: rgba(255,255,255,0.04); color: {member.color}; border: 1px solid rgba(255,255,255,0.08);",
                                    "{&member.name[..1]}"
                                }
                                div {
                                    class: "flex-1",
                                    h3 {
                                        class: "text-lg font-bold",
                                        style: "color: #e8eefc;",
                                        "{member.name}"
                                    }
                                    p {
                                        class: "text-sm font-medium mb-3",
                                        style: "color: {member.color};",
                                        "{member.role}"
                                    }
                                    p {
                                        class: "text-sm leading-relaxed mb-3",
                                        style: "color: #7a8ba6;",
                                        "{member.bio}"
                                    }
                                    a {
                                        href: "mailto:{member.email}",
                                        class: "text-xs",
                                        style: "color: #4a5568;",
                                        "{member.email}"
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
