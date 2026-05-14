"use client";

import { defineBiyardWidgets } from "@biyard/widget";
import { useEffect, useState, type CSSProperties, type ReactNode } from "react";

declare global {
  // eslint-disable-next-line @typescript-eslint/no-namespace
  namespace JSX {
    interface IntrinsicElements {
      "biyard-claim": React.DetailedHTMLProps<
        React.HTMLAttributes<HTMLElement> & {
          "base-url": string;
          month: string;
          "chain-id"?: string | number;
          mode?: "modal" | "inline";
          theme?: "auto" | "light" | "dark";
          branding?: "default" | "minimal" | "none";
          lang?: string;
          label?: string;
          title?: string;
          subtitle?: string;
        },
        HTMLElement
      >;
      "biyard-balance": React.DetailedHTMLProps<
        React.HTMLAttributes<HTMLElement> & {
          "base-url": string;
          month?: string;
          "chain-id"?: string | number;
          theme?: "auto" | "light" | "dark";
          branding?: "default" | "minimal" | "none";
          lang?: string;
          title?: string;
        },
        HTMLElement
      >;
      "biyard-transactions": React.DetailedHTMLProps<
        React.HTMLAttributes<HTMLElement> & {
          "base-url": string;
          limit?: string | number;
          month?: string;
          theme?: "auto" | "light" | "dark";
          branding?: "default" | "minimal" | "none";
          lang?: string;
          title?: string;
        },
        HTMLElement
      >;
      "biyard-monthly-summary": React.DetailedHTMLProps<
        React.HTMLAttributes<HTMLElement> & {
          "base-url": string;
          theme?: "auto" | "light" | "dark";
          branding?: "default" | "minimal" | "none";
          lang?: string;
          title?: string;
        },
        HTMLElement
      >;
    }
  }
}

export default function ClientWidgets() {
  const [ready, setReady] = useState(false);
  useEffect(() => {
    defineBiyardWidgets();
    setReady(true);
  }, []);

  if (!ready) return <p style={{ color: "#888" }}>Loading widgets…</p>;

  return (
    <>
      <Section
        title="Claim — modal trigger"
        description="Wallet connect + on-chain claim, opened via a partner-styled button."
      >
        <biyard-claim base-url="/api/biyard" month="2026-01" />
      </Section>

      <Section
        title="Balance"
        description="Points (proxy) + on-chain token balance (wallet RPC)."
      >
        <biyard-balance base-url="/api/biyard" month="2026-01" />
      </Section>

      <Section
        title="Monthly summary"
        description="Per-month earned / spent / balance + claimed flag."
      >
        <biyard-monthly-summary base-url="/api/biyard" />
      </Section>

      <Section
        title="Transactions"
        description="Paginated activity list with load-more."
      >
        <biyard-transactions base-url="/api/biyard" limit={6} />
      </Section>

      <Section
        title="Theming — indigo accent, minimal branding"
        description="Same widgets, custom CSS variables, 'via Biyard' instead of 'Secured by Biyard'."
      >
        <div
          style={
            {
              ["--biyard-color-accent" as string]: "#6366f1",
              ["--biyard-color-accent-foreground" as string]: "#ffffff",
              ["--biyard-radius" as string]: "16px",
              display: "grid",
              gap: 12,
            } as CSSProperties
          }
        >
          <biyard-balance
            base-url="/api/biyard"
            month="2026-01"
            branding="minimal"
          />
          <biyard-claim
            base-url="/api/biyard"
            month="2026-01"
            branding="minimal"
            title="Acme rewards"
            subtitle="January 2026"
          />
        </div>
      </Section>

      <Section
        title="Dark theme + Korean"
        description='theme="dark" + lang="ko" applied to a balance card.'
      >
        <biyard-balance
          base-url="/api/biyard"
          month="2026-01"
          theme="dark"
          lang="ko"
        />
      </Section>
    </>
  );
}

function Section({
  title,
  description,
  children,
}: {
  title: string;
  description?: string;
  children: ReactNode;
}) {
  return (
    <section style={{ marginTop: 32 }}>
      <h2
        style={{
          fontSize: 14,
          fontWeight: 600,
          color: "#374151",
          margin: "0 0 4px",
        }}
      >
        {title}
      </h2>
      {description && (
        <p style={{ color: "#6b7280", fontSize: 13, margin: "0 0 12px" }}>
          {description}
        </p>
      )}
      <div>{children}</div>
    </section>
  );
}
