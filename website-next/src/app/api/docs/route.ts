import { NextRequest, NextResponse } from "next/server";

const REPO = "PHANTOMOJO/GhostWire-secure-mesh-communication";
const CODEBERG_RAW = `https://codeberg.org/${REPO}/raw/branch/main`;

export async function GET(request: NextRequest) {
  const { searchParams } = new URL(request.url);
  const docPath = searchParams.get("path");

  if (!docPath || docPath.includes("..")) {
    return NextResponse.json({ error: "Invalid path" }, { status: 400 });
  }

  try {
    const res = await fetch(`${CODEBERG_RAW}/${docPath}`, {
      headers: { Accept: "text/plain" },
    });

    if (!res.ok) {
      return NextResponse.json(
        { error: `Codeberg returned ${res.status}` },
        { status: res.status }
      );
    }

    const text = await res.text();

    return new NextResponse(text, {
      headers: {
        "Content-Type": "text/plain; charset=utf-8",
        "Cache-Control": "public, s-maxage=300, stale-while-revalidate=60",
      },
    });
  } catch (err) {
    console.error("Docs proxy error:", err);
    return NextResponse.json(
      { error: "Failed to fetch from Codeberg" },
      { status: 502 }
    );
  }
}
