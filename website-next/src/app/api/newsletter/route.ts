import { NextResponse } from "next/server";
import { getDb } from "@/lib/db";
import {
  checkRateLimit,
  getClientKey,
  rateLimitResponse,
} from "@/lib/rate-limit";

function jsonResponse(body: unknown, status = 200) {
  return NextResponse.json(body, {
    status,
    headers: { "Cache-Control": "no-store" },
  });
}
function isValidEmail(email: string): boolean {
  const re = /^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$/;
  return re.test(email) && email.length <= 254;
}
export async function POST(request: Request) {
  const rateLimit = checkRateLimit(
    `newsletter:${getClientKey(request)}`,
    5,
    10 * 60 * 1000
  );
  if (!rateLimit.ok) return rateLimitResponse(rateLimit.retryAfter);

  let body: unknown;
  try {
    body = await request.json();
  } catch {
    return jsonResponse({ ok: false, error: "Invalid JSON body" }, 400);
  }
  const email =
    typeof body === "object" && body !== null && "email" in body
      ? String((body as { email: unknown }).email).trim().toLowerCase()
      : "";

  if (!email) {
    return jsonResponse({ ok: false, error: "Email is required" }, 400);
  }
  if (!isValidEmail(email)) {
    return jsonResponse(
      { ok: false, error: "Please provide a valid email address" },
      400
    );
  }
  try {
    const db = getDb();

    const existing = await db
      .prepare("SELECT id, active FROM newsletter_subscribers WHERE email = ?")
      .bind(email)
      .first<{ id: string; active: number }>();

    if (existing) {
      if (existing.active) {
        return jsonResponse({
          ok: true,
          message: "You are already subscribed. Welcome back to the mesh.",
        });
      }
      await db
        .prepare(
          "UPDATE newsletter_subscribers SET active = 1, source = 'website-footer', updated_at = datetime('now') WHERE email = ?"
        )
        .bind(email)
        .run();
      return jsonResponse({
        ok: true,
        message: "Your subscription has been reactivated.",
      });
    }
    await db
      .prepare(
        "INSERT INTO newsletter_subscribers (email, source, active, created_at, updated_at) VALUES (?, 'website-footer', 1, datetime('now'), datetime('now'))"
      )
      .bind(email)
      .run();

    return jsonResponse(
      { ok: true, message: "Subscribed. Welcome to the mesh." },
      201
    );
  } catch (err) {
    console.error("Newsletter subscription error:", err);
    return jsonResponse(
      { ok: false, error: "Subscription failed. Please try again later." },
      500
    );
  }
}
