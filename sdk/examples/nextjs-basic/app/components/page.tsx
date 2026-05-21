"use client";

import dynamic from "next/dynamic";

const ClientComponents = dynamic(() => import("./client-components"), {
  ssr: false,
});

export default function ComponentsPage() {
  return (
    <main
      style={{
        maxWidth: 1100,
        margin: "48px auto",
        padding: "0 24px",
        color: "#111827",
      }}
    >
      <header style={{ marginBottom: 32 }}>
        <h1 style={{ margin: "0 0 6px", fontSize: 28, fontWeight: 700 }}>
          Components
        </h1>
        <p
          style={{
            color: "#6b7280",
            margin: 0,
            maxWidth: 720,
            fontSize: 14,
            lineHeight: 1.5,
          }}
        >
          Each widget on its own, with the snippet you&apos;d paste into your
          page. Try light / dark, English / Korean, and resize your browser
          to see how each widget responds to container width.
        </p>
        <PageNav active="components" />
      </header>

      <ClientComponents />
    </main>
  );
}

function PageNav({ active }: { active: "preview" | "components" }) {
  return (
    <nav
      style={{
        display: "flex",
        gap: 8,
        marginTop: 20,
        borderBottom: "1px solid #e5e7eb",
      }}
    >
      <NavLink href="/widget" label="Live preview" active={active === "preview"} />
      <NavLink
        href="/components"
        label="Components"
        active={active === "components"}
      />
    </nav>
  );
}

function NavLink({
  href,
  label,
  active,
}: {
  href: string;
  label: string;
  active: boolean;
}) {
  return (
    <a
      href={href}
      style={{
        padding: "10px 16px",
        fontSize: 14,
        fontWeight: active ? 600 : 500,
        color: active ? "#111827" : "#6b7280",
        textDecoration: "none",
        borderBottom: active ? "2px solid #111827" : "2px solid transparent",
        marginBottom: -1,
      }}
    >
      {label}
    </a>
  );
}
