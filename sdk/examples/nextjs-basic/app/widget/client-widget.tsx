"use client";

import { defineBiyardClaim } from "@biyard/widget";
import { useEffect, useRef, useState, type CSSProperties } from "react";

declare global {
  // eslint-disable-next-line @typescript-eslint/no-namespace
  namespace JSX {
    interface IntrinsicElements {
      "biyard-claim": React.DetailedHTMLProps<
        React.HTMLAttributes<HTMLElement> & {
          "base-url": string;
          "chain-id"?: string | number;
          month: string;
          mode?: "modal" | "inline";
          theme?: "auto" | "light" | "dark";
          branding?: "default" | "minimal" | "none";
          lang?: string;
          label?: string;
          title?: string;
          subtitle?: string;
          amount?: string;
          symbol?: string;
          decimals?: string | number;
        },
        HTMLElement
      >;
    }
  }
}

export interface ClientWidgetProps {
  month: string;
  mode?: "modal" | "inline";
  theme?: "auto" | "light" | "dark";
  branding?: "default" | "minimal" | "none";
  lang?: string;
  label?: string;
  title?: string;
  subtitle?: string;
  style?: CSSProperties;
}

export default function ClientWidget({
  month,
  mode,
  theme,
  branding,
  lang,
  label,
  title,
  subtitle,
  style,
}: ClientWidgetProps) {
  const elRef = useRef<HTMLElement | null>(null);
  const [lastTx, setLastTx] = useState<string | null>(null);

  useEffect(() => {
    defineBiyardClaim();
  }, []);

  useEffect(() => {
    const el = elRef.current;
    if (!el) return;
    const onSuccess = (e: Event) => {
      const detail = (e as CustomEvent<{ tx_hash: string }>).detail;
      setLastTx(detail.tx_hash);
    };
    el.addEventListener("biyard-claim-success", onSuccess);
    return () => el.removeEventListener("biyard-claim-success", onSuccess);
  }, []);

  return (
    <>
      <biyard-claim
        ref={elRef as React.RefObject<HTMLElement>}
        base-url="/api/biyard"
        month={month}
        {...(mode ? { mode } : {})}
        {...(theme ? { theme } : {})}
        {...(branding ? { branding } : {})}
        {...(lang ? { lang } : {})}
        {...(label ? { label } : {})}
        {...(title ? { title } : {})}
        {...(subtitle ? { subtitle } : {})}
        style={style}
      />
      {lastTx && (
        <p style={{ marginTop: 8, color: "#059669", fontSize: 13 }}>
          Last tx: <code>{lastTx}</code>
        </p>
      )}
    </>
  );
}
