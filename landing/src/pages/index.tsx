import { IndexPage as RootPage } from "@/features/index/components/index-page";

export function IndexPage() {
  return (
    <>
      <link href="/logos/favicon.ico" rel="icon" />
      <link
        rel="apple-touch-icon"
        sizes="180x180"
        href="/logos/apple-touch-icon.png"
      />
      <link rel="preconnect" href="https://fonts.googleapis.com" />
      <link
        rel="preconnect"
        href="https://fonts.gstatic.com"
        crossOrigin="anonymous"
      />
      <link
        href="https://fonts.googleapis.com/css2?family=Noto+Sans+KR:wght@100..900&family=Outfit:wght@100..900&display=swap"
        rel="stylesheet"
      />
      <script src="https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4"></script>
      <script
        type="module"
        src="https://unpkg.com/@dotlottie/player-component@2.7.12/dist/dotlottie-player.mjs"
      ></script>
      <script src="/js/intro-animation.js"></script>
      <RootPage />
    </>
  );
}
