import { NextResponse } from "next/server";

import { fetchTransactions } from "@/lib/biyard-server";
import { currentMetaUserId } from "@/lib/session";

export async function GET(req: Request) {
  // SECURITY: meta_user_id from session, never from caller.
  const metaUserId = currentMetaUserId();
  const url = new URL(req.url);
  const limit = url.searchParams.get("limit");
  const bookmark = url.searchParams.get("bookmark");
  const month = url.searchParams.get("month");
  try {
    const data = await fetchTransactions(metaUserId, {
      limit: limit ? Number(limit) : undefined,
      bookmark: bookmark ?? undefined,
      month: month ?? undefined,
    });
    return NextResponse.json(data);
  } catch (e) {
    return NextResponse.json(
      { error: e instanceof Error ? e.message : String(e) },
      { status: 502 },
    );
  }
}
