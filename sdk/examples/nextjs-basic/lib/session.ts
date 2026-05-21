/**
 * STUB partner session. In a real integration this is replaced by the
 * partner's own auth (NextAuth, Clerk, cookies + DB, etc.).
 *
 * The only thing that matters for SDK security is: `meta_user_id` MUST come
 * from a trustworthy server-side source — NEVER from the request body or
 * query.
 */

import { headers } from "next/headers";

/**
 * Resolve the current user's `meta_user_id` from server-side state.
 *
 * This stub reads an `x-demo-user` header to make the example testable
 * without wiring up real auth. Replace with real session lookup in
 * production.
 *
 * Async because Next 15 made `headers()` async.
 */
export async function currentMetaUserId(): Promise<string> {
  const demoUser = (await headers()).get("x-demo-user");
  return demoUser ?? "1";
}
