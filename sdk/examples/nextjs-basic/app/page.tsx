"use client";

import {
  BiyardClaim,
  type ClaimableResponse,
  type TokenInfo,
} from "@biyard/sdk";
import Link from "next/link";
import { useCallback, useEffect, useMemo, useState } from "react";

// Partner-defined branding. The SDK and widget don't ship any branding text
// of their own — the integrator decides what to show around the flow.
const APP_TITLE = "Acme Rewards";
const APP_SUBTITLE = "Token claim demo built on @biyard/sdk";

function formatRaw(raw: string, decimals = 18, frac = 4): string {
  try {
    const base = 10n ** BigInt(decimals);
    const n = BigInt(raw);
    const whole = n / base;
    const fr = n % base;
    const wholeStr = whole.toString().replace(/\B(?=(\d{3})+(?!\d))/g, ",");
    if (fr === 0n) return wholeStr;
    const trunc = fr.toString().padStart(decimals, "0").slice(0, frac).replace(/0+$/, "");
    return trunc ? `${wholeStr}.${trunc}` : wholeStr;
  } catch {
    return raw;
  }
}

export default function Page() {
  const [chainId, setChainId] = useState<number | null>(null);

  // Bootstrap: fetch token metadata once to discover the chain id. We use a
  // throwaway fetch-only BiyardClaim (chainId 0) for that — the proxy call
  // doesn't touch RPC. Then create the real client with the discovered chain.
  useEffect(() => {
    const bootstrap = new BiyardClaim({ baseUrl: "/api/biyard", chainId: 0 });
    bootstrap.getTokenInfo().then((t) => {
      if (t.chain_id) setChainId(t.chain_id);
    });
  }, []);

  const biyard = useMemo(() => {
    if (!chainId) return null;
    return new BiyardClaim({ baseUrl: "/api/biyard", chainId });
  }, [chainId]);

  return (
    <main style={{ maxWidth: 720, margin: "48px auto", padding: "0 24px" }}>
      <header style={{ marginBottom: 24 }}>
        <h1 style={{ marginBottom: 4 }}>{APP_TITLE}</h1>
        <p style={{ color: "#666", margin: 0 }}>{APP_SUBTITLE}</p>
      </header>

      <p style={{ marginTop: 16 }}>
        <Link href="/widget">→ See the drop-in &lt;biyard-claim&gt; widget</Link>
      </p>

      {biyard ? (
        <>
          <TokenSummary biyard={biyard} />
          <Wallet biyard={biyard} />
          <Claimable biyard={biyard} />
        </>
      ) : (
        <p style={{ color: "#888" }}>Initializing…</p>
      )}
    </main>
  );
}

function TokenSummary({ biyard }: { biyard: BiyardClaim }) {
  const [token, setToken] = useState<TokenInfo | null>(null);
  const [error, setError] = useState<Error | null>(null);

  useEffect(() => {
    biyard
      .getTokenInfo()
      .then(setToken)
      .catch((e) => setError(e instanceof Error ? e : new Error(String(e))));
  }, [biyard]);

  if (error) return <p style={{ color: "crimson" }}>{error.message}</p>;
  if (!token) return <p style={{ color: "#888" }}>Loading token info…</p>;

  return (
    <section
      style={{
        marginTop: 16,
        padding: 16,
        border: "1px solid #e5e5e5",
        borderRadius: 8,
        background: "#fafafa",
      }}
    >
      <div style={{ display: "flex", justifyContent: "space-between" }}>
        <div>
          <div style={{ fontWeight: 600 }}>
            {token.name} ({token.symbol})
          </div>
          <div style={{ color: "#666", fontSize: 13, marginTop: 4 }}>
            Chain {token.chain_id ?? "?"} · decimals {token.decimals}
          </div>
        </div>
        {token.contract_address && (
          <code style={{ fontSize: 12, color: "#888" }}>
            {token.contract_address.slice(0, 8)}…{token.contract_address.slice(-6)}
          </code>
        )}
      </div>
    </section>
  );
}

function Wallet({ biyard }: { biyard: BiyardClaim }) {
  const [address, setAddress] = useState<string | null>(null);
  const [connecting, setConnecting] = useState(false);
  const [error, setError] = useState<Error | null>(null);

  useEffect(() => {
    void biyard.getWalletAddress().then(setAddress);
  }, [biyard]);

  const connect = useCallback(async () => {
    setConnecting(true);
    setError(null);
    try {
      setAddress(await biyard.connectWallet());
    } catch (e) {
      setError(e instanceof Error ? e : new Error(String(e)));
    } finally {
      setConnecting(false);
    }
  }, [biyard]);

  return (
    <section style={{ marginTop: 24 }}>
      <h2>Wallet</h2>
      {address ? (
        <p>
          Connected: <code>{address}</code>
        </p>
      ) : (
        <button type="button" onClick={connect} disabled={connecting}>
          {connecting ? "Connecting..." : "Connect wallet"}
        </button>
      )}
      {error && <p style={{ color: "crimson" }}>{error.message}</p>}
    </section>
  );
}

function Claimable({ biyard }: { biyard: BiyardClaim }) {
  const [data, setData] = useState<ClaimableResponse | null>(null);
  const [token, setToken] = useState<TokenInfo | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);
  const [claimingMonth, setClaimingMonth] = useState<string | null>(null);

  const refresh = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const [c, t] = await Promise.all([
        biyard.getClaimable(),
        biyard.getTokenInfo(),
      ]);
      setData(c);
      setToken(t);
    } catch (e) {
      setError(e instanceof Error ? e : new Error(String(e)));
    } finally {
      setLoading(false);
    }
  }, [biyard]);

  useEffect(() => {
    void refresh();
  }, [refresh]);

  const onClaim = useCallback(
    async (month: string) => {
      setClaimingMonth(month);
      try {
        const result = await biyard.claim(month);
        alert(`Claimed! tx=${result.tx_hash}`);
        await refresh();
      } catch (e) {
        alert(`Error: ${e instanceof Error ? e.message : String(e)}`);
      } finally {
        setClaimingMonth(null);
      }
    },
    [biyard, refresh],
  );

  if (loading) return <p>Loading...</p>;
  if (error) {
    return (
      <section style={{ marginTop: 24 }}>
        <h2>Claimable months</h2>
        <p style={{ color: "crimson" }}>{error.message}</p>
        <button type="button" onClick={refresh}>
          Retry
        </button>
      </section>
    );
  }

  const months = data?.months ?? [];
  const symbol = token?.symbol ?? "";
  const decimals = token?.decimals ?? 18;

  return (
    <section style={{ marginTop: 24 }}>
      <h2>Claimable months</h2>
      {months.length === 0 ? (
        <p>No claimable months yet.</p>
      ) : (
        <ul style={{ paddingLeft: 0, listStyle: "none" }}>
          {months.map((m) => (
            <li
              key={m.month}
              style={{
                border: "1px solid #e5e5e5",
                borderRadius: 8,
                padding: 16,
                marginBottom: 12,
                display: "flex",
                justifyContent: "space-between",
                alignItems: "center",
              }}
            >
              <div>
                <strong>{m.month}</strong>
                <div style={{ color: "#666", fontSize: 14, marginTop: 4 }}>
                  {formatRaw(m.remaining, decimals)} {symbol} remaining
                </div>
              </div>
              <button
                type="button"
                disabled={claimingMonth === m.month}
                onClick={() => onClaim(m.month)}
                style={{
                  padding: "8px 16px",
                  borderRadius: 6,
                  border: "1px solid #111",
                  background: "#111",
                  color: "white",
                  cursor: "pointer",
                }}
              >
                {claimingMonth === m.month ? "Claiming..." : `Claim ${symbol}`}
              </button>
            </li>
          ))}
        </ul>
      )}
    </section>
  );
}
