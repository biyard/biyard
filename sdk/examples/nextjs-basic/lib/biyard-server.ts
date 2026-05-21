/**
 * Server-side helper to call the Biyard API with the partner's API key.
 *
 * Lives on the server only. The API key is read from `BIYARD_API_KEY` and
 * must never reach the browser.
 */

function env(name: string): string {
  const v = process.env[name];
  if (!v) throw new Error(`Missing required env var: ${name}`);
  return v;
}

function apiBase() {
  return process.env.BIYARD_API_URL ?? "https://api.biyard.co";
}

async function biyardFetch<T>(
  path: string,
  init: RequestInit & { method: "GET" | "POST" },
): Promise<T> {
  const apiKey = env("BIYARD_API_KEY");
  const res = await fetch(`${apiBase()}${path}`, {
    ...init,
    headers: {
      ...init.headers,
      authorization: `Bearer ${apiKey}`,
      "content-type": "application/json",
    },
    cache: "no-store",
  });
  if (!res.ok) {
    const body = await res.text().catch(() => "");
    throw new Error(`Biyard ${path} ${res.status}: ${body}`);
  }
  return (await res.json()) as T;
}

export async function fetchTokenInfo() {
  const projectId = env("BIYARD_PROJECT_ID");
  return biyardFetch<unknown>(`/v1/projects/${projectId}/tokens`, {
    method: "GET",
  });
}

export async function fetchClaimable(metaUserId: string) {
  const projectId = env("BIYARD_PROJECT_ID");
  const params = new URLSearchParams({ meta_user_id: metaUserId });
  return biyardFetch<unknown>(
    `/v1/projects/${projectId}/tokens/claimable?${params.toString()}`,
    { method: "GET" },
  );
}

export async function fetchUserBalance(metaUserId: string, month?: string) {
  const projectId = env("BIYARD_PROJECT_ID");
  const q = month ? `?month=${encodeURIComponent(month)}` : "";
  return biyardFetch<unknown>(
    `/v1/projects/${projectId}/points/${encodeURIComponent(metaUserId)}${q}`,
    { method: "GET" },
  );
}

export async function fetchMonthlySummaries(metaUserId: string) {
  const projectId = env("BIYARD_PROJECT_ID");
  return biyardFetch<unknown>(
    `/v1/projects/${projectId}/points/${encodeURIComponent(metaUserId)}/monthly-summaries`,
    { method: "GET" },
  );
}

export async function fetchTransactions(
  metaUserId: string,
  opts: { limit?: number; bookmark?: string | null; month?: string } = {},
) {
  const projectId = env("BIYARD_PROJECT_ID");
  const params = new URLSearchParams();
  if (opts.limit != null) params.set("limit", String(opts.limit));
  if (opts.bookmark) params.set("bookmark", opts.bookmark);
  if (opts.month) params.set("month", opts.month);
  const q = params.toString();
  return biyardFetch<unknown>(
    `/v1/projects/${projectId}/points/${encodeURIComponent(metaUserId)}/transactions${q ? `?${q}` : ""}`,
    { method: "GET" },
  );
}

export async function fetchClaimSignature(
  metaUserId: string,
  month: string,
  walletAddress: string,
) {
  const projectId = env("BIYARD_PROJECT_ID");
  return biyardFetch<unknown>(
    `/v1/projects/${projectId}/tokens/claim-signature`,
    {
      method: "POST",
      body: JSON.stringify({
        meta_user_id: metaUserId,
        month,
        wallet_address: walletAddress,
      }),
    },
  );
}

/**
 * Award points to a user. Used by the example's `/api/dev/refill-points`
 * endpoint so that we can keep clicking "Claim" during demos without
 * manually re-running curl every time the balance hits zero.
 *
 * Not part of the SDK contract — partners don't need this in production.
 */
export async function awardPoints(
  metaUserId: string,
  month: string,
  amount: number,
  description?: string,
) {
  const projectId = env("BIYARD_PROJECT_ID");
  return biyardFetch<unknown>(`/v1/projects/${projectId}/points`, {
    method: "POST",
    body: JSON.stringify({
      transactions: [
        {
          month,
          description: description ?? "SDK demo refill",
          tx_type: "Award",
          to: metaUserId,
          amount,
        },
      ],
    }),
  });
}
