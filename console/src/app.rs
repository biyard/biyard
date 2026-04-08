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

/// Newtype wrapper for the theme context signal so it doesn't collide
/// with any other `Signal<bool>` provider in the tree.
#[derive(Clone, Copy)]
pub struct ThemeIsDark(pub Signal<bool>);

#[cfg(not(feature = "server"))]
fn initial_theme_is_dark() -> bool {
    web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.document_element())
        .and_then(|el| el.get_attribute("data-theme"))
        .map(|t| t == "dark")
        .unwrap_or(true)
}

#[cfg(feature = "server")]
fn initial_theme_is_dark() -> bool {
    true
}

#[component]
pub fn App() -> Element {
    let _ = crate::features::accounts::context::Context::init()?;

    // Theme state lives at the root so it survives every child remount
    // (e.g. opening and closing the account menu drop-down). Putting the
    // signal inside `ThemeMenuAction` itself caused a "Dropped" panic the
    // moment the menu unmounted.
    use_context_provider(|| ThemeIsDark(Signal::new(initial_theme_is_dark())));

    rsx! {
        document::Link { rel: "icon", href: asset!("/assets/favicon.ico") }
        // Preconnect to Pretendard's CDN so the font stylesheet + WOFF2
        // files start resolving before CSS parsing reaches the @font-face
        // rules. Pretendard renders Korean + Latin in one family, so we
        // don't need a secondary web font.
        document::Link {
            rel: "preconnect",
            href: "https://cdn.jsdelivr.net",
            crossorigin: "anonymous",
        }
        document::Link {
            rel: "stylesheet",
            href: "https://cdn.jsdelivr.net/gh/orioncactus/pretendard@v1.3.9/dist/web/variable/pretendardvariable-dynamic-subset.min.css",
        }
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
