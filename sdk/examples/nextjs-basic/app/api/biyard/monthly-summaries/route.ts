import { NextResponse } from "next/server";

import { fetchMonthlySummaries } from "@/lib/biyard-server";
import { currentMetaUserId } from "@/lib/session";

export async function GET() {
  // SECURITY: meta_user_id from session, never from caller.
  const metaUserId = currentMetaUserId();
  try {
    const data = await fetchMonthlySummaries(metaUserId);
    return NextResponse.json(data);
  } catch (e) {
    return NextResponse.json(
      { error: e instanceof Error ? e.message : String(e) },
      { status: 502 },
    );
  }
}
