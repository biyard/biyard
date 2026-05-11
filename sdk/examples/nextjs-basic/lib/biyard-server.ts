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
