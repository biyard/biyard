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

    // 기본 한국어. 사용자가 cookie/localStorage 로 명시 변경한 경우만 반영.
    var l = cookie("language") || localStorage.getItem("language") || "ko";
    document.documentElement.setAttribute("lang", l);
    // localStorage 와 cookie 양쪽에 영구화 (서버 SSR + WASM 둘 다 같은 값 보도록).
    try { localStorage.setItem("language", l); } catch (e) {}
    if (!cookie("language")) {
      document.cookie = "language=" + l + "; path=/; max-age=" + (60 * 60 * 24 * 365);
    }
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

    // 1차 — 기본 언어를 한국어로 강제. 사용자가 토글 메뉴로 바꿀 수 있게 만드는 건 후속.
    // SSR + 첫 hydration 모두 ko 가 되도록 use_effect 로 첫 렌더 시 강제.
    use_effect(|| {
        use dioxus_translate::{Language, set_language};
        set_language(Language::Ko);
    });

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
