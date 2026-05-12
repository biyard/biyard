use dioxus::prelude::*;

use crate::Route;

/// 페이지 첫 paint 전에 <html> 의 data-theme 와 lang 속성을 적용.
/// 우선순위: cookie → localStorage → navigator.language → "dark"/"ko".
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

/// 테마 컨텍스트 — 컴포넌트에서 use_context::<ThemeIsDark>() 로 접근
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
    use_context_provider(|| ThemeIsDark(Signal::new(initial_theme_is_dark())));

    rsx! {
        document::Link { rel: "icon", r#type: "image/png", href: asset!("/assets/biyard-logo.png") }
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
        document::Script { {BOOTSTRAP_SCRIPT} }
        Router::<Route> {}
    }
}
