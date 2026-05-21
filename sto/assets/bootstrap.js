// 페이지 첫 paint 전에 <html>의 data-theme와 lang 속성을 적용.
// 다음 방문(SSR)에서도 같은 값으로 렌더되도록 쿠키/localStorage에 영구화.
// 우선순위: cookie → localStorage → navigator.language → "dark"/"en".
(function () {
  try {
    function cookie(name) {
      var m = document.cookie.match(new RegExp("(?:^|; )" + name + "=([^;]+)"));
      return m && m[1];
    }

    var t = cookie("theme") || localStorage.getItem("theme") || "dark";
    document.documentElement.setAttribute("data-theme", t);

    var navLang = (navigator.language || "en").split("-")[0];
    var supported = navLang === "ko" ? "ko" : "en";
    var l = cookie("language") || localStorage.getItem("language") || supported;
    document.documentElement.setAttribute("lang", l);
    try { localStorage.setItem("language", l); } catch (e) {}
    if (!cookie("language")) {
      document.cookie = "language=" + l + "; path=/; max-age=" + (60 * 60 * 24 * 365);
    }
  } catch (e) {}
})();
