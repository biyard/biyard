import { NextResponse } from "next/server";

import { fetchClaimSignature } from "@/lib/biyard-server";
import { currentMetaUserId } from "@/lib/session";

interface Body {
  month: string;
  wallet_address: string;
}

export async function POST(req: Request) {
  // SECURITY: meta_user_id comes from the partner session — never from the
  // request body. Even if the browser sends `meta_user_id` in the body, we
  // ignore it.
  const metaUserId = currentMetaUserId();

  let body: Body;
  try {
    body = (await req.json()) as Body;
  } catch {
    return NextResponse.json({ error: "Invalid JSON body" }, { status: 400 });
  }

  if (!body.month || !body.wallet_address) {
    return NextResponse.json(
      { error: "month and wallet_address are required" },
      { status: 400 },
    );
  }

  try {
    const data = await fetchClaimSignature(
      metaUserId,
      body.month,
      body.wallet_address,
    );
    return NextResponse.json(data);
  } catch (e) {
    return NextResponse.json(
      { error: e instanceof Error ? e.message : String(e) },
      { status: 502 },
    );
  }
}
