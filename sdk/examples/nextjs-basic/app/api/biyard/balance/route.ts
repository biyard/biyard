import { NextResponse } from "next/server";

import { fetchUserBalance } from "@/lib/biyard-server";
import { currentMetaUserId } from "@/lib/session";

export async function GET(req: Request) {
  // SECURITY: meta_user_id from session, never from caller.
  const metaUserId = await currentMetaUserId();
  const month = new URL(req.url).searchParams.get("month") ?? undefined;
  try {
    const data = await fetchUserBalance(metaUserId, month);
    return NextResponse.json(data);
  } catch (e) {
    return NextResponse.json(
      { error: e instanceof Error ? e.message : String(e) },
      { status: 502 },
    );
  }
}
