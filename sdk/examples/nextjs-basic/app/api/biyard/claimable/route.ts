import { NextResponse } from "next/server";

import { fetchClaimable } from "@/lib/biyard-server";
import { currentMetaUserId } from "@/lib/session";

export async function GET() {
  // SECURITY: meta_user_id comes from the partner session — never from the
  // request body, query, or headers controllable by the browser.
  const metaUserId = currentMetaUserId();

  try {
    const data = await fetchClaimable(metaUserId);
    return NextResponse.json(data);
  } catch (e) {
    return NextResponse.json(
      { error: e instanceof Error ? e.message : String(e) },
      { status: 502 },
    );
  }
}
