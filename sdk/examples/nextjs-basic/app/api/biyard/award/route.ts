import { NextResponse } from "next/server";

import { awardPoints } from "@/lib/biyard-server";
import { currentMetaUserId } from "@/lib/session";

interface Body {
  month: string;
  amount: number;
  description?: string;
}

/**
 * Award points to the currently authenticated user.
 *
 * In a real partner integration this endpoint is called from the partner's
 * own business events (a purchase, a completed action, a referral, etc.) —
 * not from a browser button. It's exposed here so the example UI can
 * top up the demo user's balance for repeated claim testing.
 *
 * SECURITY: meta_user_id is derived from the partner session — never from
 * the request body. Same rule as /claim-signature.
 */
export async function POST(req: Request) {
  const metaUserId = await currentMetaUserId();

  let body: Body;
  try {
    body = (await req.json()) as Body;
  } catch {
    return NextResponse.json({ error: "Invalid JSON body" }, { status: 400 });
  }

  if (!body.month || !Number.isFinite(body.amount) || body.amount <= 0) {
    return NextResponse.json(
      { error: "month and positive amount are required" },
      { status: 400 },
    );
  }

  try {
    const data = await awardPoints(
      metaUserId,
      body.month,
      body.amount,
      body.description,
    );
    return NextResponse.json(data);
  } catch (e) {
    return NextResponse.json(
      { error: e instanceof Error ? e.message : String(e) },
      { status: 502 },
    );
  }
}
