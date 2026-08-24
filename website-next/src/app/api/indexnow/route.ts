import { NextRequest, NextResponse } from "next/server";
import {
  checkRateLimit,
  getClientKey,
  rateLimitResponse,
} from "@/lib/rate-limit";

const INDEXNOW_KEY = "9ca14d3b003f23829a385dd23045ce11abd16923b13866863e5accb89f5991f5";
const INDEXNOW_ENDPOINT = "https://api.indexnow.org/indexnow";

function isAuthorized(req: NextRequest): boolean {
  const token = process.env.INDEXNOW_ADMIN_TOKEN;
  if (!token) return false;
  return req.headers.get("authorization") === `Bearer ${token}`;
}

function isAllowedUrl(value: unknown): value is string {
  if (typeof value !== "string") return false;
  try {
    const url = new URL(value);
    return url.protocol === "https:" && url.hostname === "www.ghostwire.cc";
  } catch {
    return false;
  }
}

export async function POST(req: NextRequest) {
  if (!isAuthorized(req)) {
    return NextResponse.json({ ok: false, error: "Not found" }, { status: 404 });
  }
  const rateLimit = checkRateLimit(
    `indexnow:${getClientKey(req)}`,
    10,
    10 * 60 * 1000
  );
  if (!rateLimit.ok) return rateLimitResponse(rateLimit.retryAfter);

  let body: unknown;
  try {
    body = await req.json();
  } catch {
    return NextResponse.json({ error: "Invalid JSON body" }, { status: 400 });
  }
  const { urls } = typeof body === "object" && body !== null ? body as { urls?: unknown } : {};

  if (
    !Array.isArray(urls) ||
    urls.length === 0 ||
    urls.length > 100 ||
    !urls.every(isAllowedUrl)
  ) {
    return NextResponse.json(
      { error: "up to 100 same-origin HTTPS URLs are required" },
      { status: 400 }
    );
  }

  const payload = {
    host: "www.ghostwire.cc",
    key: INDEXNOW_KEY,
    keyLocation: `https://www.ghostwire.cc/${INDEXNOW_KEY}.txt`,
    urlList: urls,
  };

  const res = await fetch(INDEXNOW_ENDPOINT, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(payload),
  });

  return NextResponse.json({
    submitted: urls.length,
    status: res.status,
    ok: res.ok,
  });
}

export async function GET() {
  return NextResponse.json({ ok: false, error: "Not found" }, { status: 404 });
}
