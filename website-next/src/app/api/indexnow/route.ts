import { NextRequest, NextResponse } from "next/server";

const INDEXNOW_KEY = "9ca14d3b003f23829a385dd23045ce11abd16923b13866863e5accb89f5991f5";
const INDEXNOW_ENDPOINT = "https://api.indexnow.org/indexnow";

export async function POST(req: NextRequest) {
  const { urls } = await req.json();

  if (!Array.isArray(urls) || urls.length === 0) {
    return NextResponse.json({ error: "urls array required" }, { status: 400 });
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
  const baseUrl = "https://www.ghostwire.cc";
  const sections = [
    "#about", "#security", "#features", "#architecture",
    "#roadmap", "#faq", "#community", "#quickstart",
  ];

  const urls = [baseUrl, ...sections.map((s) => `${baseUrl}/${s}`)];

  const payload = {
    host: "www.ghostwire.cc",
    key: INDEXNOW_KEY,
    keyLocation: `${baseUrl}/${INDEXNOW_KEY}.txt`,
    urlList: urls,
  };

  const res = await fetch(INDEXNOW_ENDPOINT, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(payload),
  });

  return NextResponse.json({
    submitted: urls.length,
    urls,
    status: res.status,
    ok: res.ok,
  });
}
