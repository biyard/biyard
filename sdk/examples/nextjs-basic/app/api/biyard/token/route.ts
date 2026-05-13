import { NextResponse } from "next/server";

import { fetchTokenInfo } from "@/lib/biyard-server";

/**
 * Public token metadata. Not user-specific. Still proxied so the partner's
 * Biyard API key stays server-side.
 */
export async function GET() {
  try {
    const data = await fetchTokenInfo();
    return NextResponse.json(data);
  } catch (e) {
    return NextResponse.json(
      { error: e instanceof Error ? e.message : String(e) },
      { status: 502 },
    );
  }
}
