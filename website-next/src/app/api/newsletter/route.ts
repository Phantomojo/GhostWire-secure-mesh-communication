import { NextResponse } from "next/server";
import { getDb } from "@/lib/db";

function isValidEmail(email: string): boolean {
  const re = /^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$/;
  return re.test(email) && email.length <= 254;
}

export async function POST(request: Request) {
  let body: unknown;
  try {
    body = await request.json();
  } catch {
    return NextResponse.json(
      { ok: false, error: "Invalid JSON body" },
      { status: 400 }
    );
  }

  const email =
    typeof body === "object" && body !== null && "email" in body
      ? String((body as { email: unknown }).email).trim().toLowerCase()
      : "";

  if (!email) {
    return NextResponse.json(
      { ok: false, error: "Email is required" },
      { status: 400 }
    );
  }
  if (!isValidEmail(email)) {
    return NextResponse.json(
      { ok: false, error: "Please provide a valid email address" },
      { status: 400 }
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
        return NextResponse.json(
          {
            ok: true,
            message: "You are already subscribed. Welcome back to the mesh.",
          },
          { status: 200 }
        );
      }
      await db
        .prepare(
          "UPDATE newsletter_subscribers SET active = 1, source = 'website-footer', updated_at = datetime('now') WHERE email = ?"
        )
        .bind(email)
        .run();
      return NextResponse.json(
        {
          ok: true,
          message: "Your subscription has been reactivated.",
        },
        { status: 200 }
      );
    }

    await db
      .prepare(
        "INSERT INTO newsletter_subscribers (email, source, active, created_at, updated_at) VALUES (?, 'website-footer', 1, datetime('now'), datetime('now'))"
      )
      .bind(email)
      .run();

    return NextResponse.json(
      {
        ok: true,
        message: "Subscribed. Welcome to the mesh.",
      },
      { status: 201 }
    );
  } catch (err) {
    console.error("Newsletter subscription error:", err);
    return NextResponse.json(
      { ok: false, error: "Subscription failed. Please try again later." },
      { status: 500 }
    );
  }
}

export async function GET() {
  try {
    const db = getDb();
    const result = await db
      .prepare("SELECT COUNT(*) as count FROM newsletter_subscribers WHERE active = 1")
      .first<{ count: number }>();
    return NextResponse.json({ ok: true, count: result?.count ?? 0 });
  } catch (err) {
    console.error("Newsletter count error:", err);
    return NextResponse.json(
      { ok: false, error: "Failed to fetch subscriber count" },
      { status: 500 }
    );
  }
}
