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
      <script src="/js/intro-animation.js"></script>

      <title>Biyard</title>
      <meta
        name="keywords"
        content="blockchain, ai, security, cryptography, web3, metaverse, digital twin, digital asset, NFT, tokenization, digital identity, digital wallet, digital currency, Physical AI, DID, Cryptocurrency, DAO, Deeptech"
      />
      <meta name="author" content="Biyard Corp." />
      <meta content="Biyard" property="og:site_name" />
      <meta content="en_US" property="og:locale" />
      <meta
        name="description"
        content="Biyard leverages cutting-edge technologies like blockchain and AI to tackle complex societal challenges. We focus on areas burdened by high costs of distrust or where technology can offer significant contributions. Our work includes civic tech initiatives enhancing participation and governance, but also extends into diverse fields like Art and Sustainability, driving innovation. Based in South Korea with a global team, we collaborate across sectors to build trust-based infrastructure and solutions for the future."
      />
      <meta
        content="Biyard leverages cutting-edge technologies like blockchain and AI to tackle complex societal challenges. We focus on areas burdened by high costs of distrust or where technology can offer significant contributions. Our work includes civic tech initiatives enhancing participation and governance, but also extends into diverse fields like Art and Sustainability, driving innovation. Based in South Korea with a global team, we collaborate across sectors to build trust-based infrastructure and solutions for the future."
        property="og:description"
      />
      <meta content="Biyard" property="og:title" />
      <meta content="website" property="og:type" />
      <meta content="summary_large_image" property="twitter:card" />
      <meta content="/images/meet_biyard.png" property="og:image" />
      <meta content="/images/meet_biyard.png" property="twitter:image" />

      <RootPage />
    </>
  );
}
