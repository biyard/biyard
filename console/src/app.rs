use dioxus::prelude::*;

use crate::common::ui::Spinner;

/// Inline script injected into <head> to apply the theme and language
/// **before** the first paint. Reading order:
///   - theme:    cookie → localStorage → "dark" (default)
///   - language: cookie → localStorage → navigator.language
///
/// Dark is the primary mode (matches the Biyard landing identity). Users can
/// switch to light explicitly via the sidebar toggle.
///
/// This runs synchronously in the browser parser, so the body never flashes
/// the wrong theme/language during SSR hydration.
const BOOTSTRAP_SCRIPT: &str = r#"
(function () {
  try {
    function cookie(name) {
      var m = document.cookie.match(new RegExp("(?:^|; )" + name + "=([^;]+)"));
      return m && m[1];
    }

    var t = cookie("theme") || localStorage.getItem("theme") || "dark";
    document.documentElement.setAttribute("data-theme", t);

    var l = cookie("language") || localStorage.getItem("language");
    if (!l) {
      var nav = (navigator.language || "en").split("-")[0];
      l = nav === "ko" ? "ko" : "en";
    }
    document.documentElement.setAttribute("lang", l);
  } catch (e) {}
})();
"#;

#[component]
pub fn App() -> Element {
    let _ = crate::features::accounts::context::Context::init()?;

    rsx! {
        document::Link { rel: "icon", href: asset!("/assets/favicon.ico") }
        document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }
        document::Link {
            rel: "stylesheet",
            href: asset!("/assets/dx-components-theme.css"),
        }
        document::Script { {BOOTSTRAP_SCRIPT} }
        SuspenseBoundary {
            fallback: move |_| rsx! {
                div { class: "flex min-h-screen items-center justify-center bg-background px-6",
                    div { class: "flex w-full max-w-sm items-center gap-4 rounded-[28px] border border-border bg-panel px-6 py-5 shadow-[0_18px_40px_rgba(15,23,42,0.06)]",
                        Spinner { class: "h-7 w-7 animate-spin" }
                        div {
                            p { class: "font-display text-lg font-bold text-foreground",
                                "Loading console"
                            }
                            p { class: "text-sm text-foreground-muted",
                                "Preparing your workspace"
                            }
                        }
                    }
                }
            },
            Router::<crate::Route> {}
        }
    }
}
