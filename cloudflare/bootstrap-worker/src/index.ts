interface Env {
  DB: D1Database;
  PEER_TTL_SECONDS: string;
  MAX_PEERS: string;
  RATE_LIMIT_MAX: string;
  RATE_LIMIT_WINDOW: string;
  ALLOWED_ORIGINS: string;
  REGISTRATION_SECRET: string;
}

interface PeerRecord {
  peer_id: string;
  multiaddrs: string;
  last_seen: number;
  node_id: string;
  reticulum_address: string;
}

// ponytail: in-memory rate limit — single-instance CF Worker
const ipBuckets = new Map<string, number[]>();

function checkRateLimit(ip: string, env: Env): boolean {
  const maxReqs = parseInt(env.RATE_LIMIT_MAX || "10");
  const windowMs = parseInt(env.RATE_LIMIT_WINDOW || "60000"); // 60s default
  const now = Date.now();
  const timestamps = ipBuckets.get(ip) || [];
  const recent = timestamps.filter((t) => now - t < windowMs);
  if (recent.length >= maxReqs) return false;
  recent.push(now);
  ipBuckets.set(ip, recent);
  return true;
}

function getClientIp(request: Request): string {
  return request.headers.get("cf-connecting-ip") || "unknown";
}

export default {
  async fetch(request: Request, env: Env): Promise<Response> {
    const url = new URL(request.url);
    let path = url.pathname;
    const method = request.method;

    if (path.startsWith("/bootstrap")) {
      path = path.slice("/bootstrap".length) || "/";
    }

    if (method === "OPTIONS") {
      return corsHeaders(null, 204, env);
    }

    const clientIp = getClientIp(request);

    if (path === "/api/register" && method === "POST") {
      if (!checkRateLimit(clientIp, env)) {
        return corsHeaders(
          Response.json({ error: "rate limit exceeded" }, { status: 429 }),
          undefined,
          env
        );
      }
      return handleRegister(request, env);
    }

    if (path === "/api/peers" && method === "GET") {
      return handleGetPeers(env);
    }

    if (path.startsWith("/api/peers/") && method === "DELETE") {
      if (!checkRateLimit(clientIp, env)) {
        return corsHeaders(
          Response.json({ error: "rate limit exceeded" }, { status: 429 }),
          undefined,
          env
        );
      }
      const peerId = path.split("/api/peers/")[1];
      if (env.REGISTRATION_SECRET) {
        const valid = await verifyRegistrationAuth(request, env, peerId);
        if (!valid) {
          return corsHeaders(
            Response.json({ error: "unauthorized" }, { status: 401 }),
            undefined,
            env
          );
        }
      }
      return handleDeregister(peerId, env);
    }

    if (path === "/api/health") {
      return corsHeaders(
        Response.json({ status: "ok", version: "1.1.0" }),
        undefined,
        env
      );
    }

    return corsHeaders(
      Response.json({ error: "not found" }, { status: 404 }),
      undefined,
      env
    );
  },
};

async function verifyRegistrationAuth(
  request: Request,
  env: Env,
  peerId: string
): Promise<boolean> {
  const signature = request.headers.get("X-GW-Signature");
  const timestamp = request.headers.get("X-GW-Timestamp");

  if (!signature || !timestamp || !env.REGISTRATION_SECRET) return false;

  const ts = parseInt(timestamp);
  if (isNaN(ts)) return false;
  if (Math.abs(Date.now() / 1000 - ts) > 60) return false;

  const encoder = new TextEncoder();
  const keyData = encoder.encode(env.REGISTRATION_SECRET);
  const message = encoder.encode(peerId + timestamp);

  const cryptoKey = await crypto.subtle.importKey(
    "raw",
    keyData,
    { name: "HMAC", hash: "SHA-256" },
    false,
    ["sign"]
  );

  const signatureBuffer = await crypto.subtle.sign("HMAC", cryptoKey, message);
  const expectedSig = Array.from(new Uint8Array(signatureBuffer))
    .map((b) => b.toString(16).padStart(2, "0"))
    .join("");

  return timingSafeEqual(signature, expectedSig);
}

function timingSafeEqual(a: string, b: string): boolean {
  if (a.length !== b.length) return false;
  let result = 0;
  for (let i = 0; i < a.length; i++) {
    result |= a.charCodeAt(i) ^ b.charCodeAt(i);
  }
  return result === 0;
}

function validateMultiaddrs(addrs: string[]): boolean {
  for (const addr of addrs) {
    if (
      !addr.startsWith("/ip4/") &&
      !addr.startsWith("/ip6/") &&
      !addr.startsWith("/dns4/") &&
      !addr.startsWith("/dns6/") &&
      !addr.startsWith("/onion/")
    ) {
      return false;
    }
    if (
      !addr.includes("/tcp/") &&
      !addr.includes("/udp/") &&
      !addr.includes("/ws") &&
      !addr.includes("/quic")
    ) {
      return false;
    }
  }
  return true;
}

async function handleRegister(request: Request, env: Env): Promise<Response> {
  let body: { peer_id: string; multiaddrs: string[]; node_id?: string; reticulum_address?: string };
  try {
    body = await request.json();
  } catch {
    return corsHeaders(
      Response.json({ error: "invalid json" }, { status: 400 }),
      undefined,
      env
    );
  }

  if (!body.peer_id || !body.multiaddrs?.length) {
    return corsHeaders(
      Response.json({ error: "peer_id and multiaddrs required" }, { status: 400 }),
      undefined,
      env
    );
  }

  if (body.peer_id.length < 44 || body.peer_id.length > 88) {
    return corsHeaders(
      Response.json({ error: "invalid peer_id format" }, { status: 400 }),
      undefined,
      env
    );
  }

  if (!validateMultiaddrs(body.multiaddrs)) {
    return corsHeaders(
      Response.json({ error: "invalid multiaddrs format" }, { status: 400 }),
      undefined,
      env
    );
  }

  const reticulumAddr = body.reticulum_address || "";
  if (reticulumAddr && !/^[0-9a-f]{64}$/i.test(reticulumAddr)) {
    return corsHeaders(
      Response.json({ error: "invalid reticulum_address format (expected 64 hex chars)" }, { status: 400 }),
      undefined,
      env
    );
  }

  if (env.REGISTRATION_SECRET) {
    const valid = await verifyRegistrationAuth(request, env, body.peer_id);
    if (!valid) {
      return corsHeaders(
        Response.json({ error: "invalid or missing registration signature" }, { status: 401 }),
        undefined,
        env
      );
    }
  }

  const maxPeers = parseInt(env.MAX_PEERS || "100");
  const existing = await env.DB.prepare(
    "SELECT COUNT(*) as cnt FROM peers WHERE peer_id = ?"
  )
    .bind(body.peer_id)
    .first<{ cnt: number }>();

  const now = Math.floor(Date.now() / 1000);

  if (existing && existing.cnt > 0) {
    await env.DB.prepare(
      "UPDATE peers SET multiaddrs = ?, last_seen = ?, node_id = ?, reticulum_address = ? WHERE peer_id = ?"
    )
      .bind(JSON.stringify(body.multiaddrs), now, body.node_id || "", reticulumAddr, body.peer_id)
      .run();
  } else {
    const count = await env.DB.prepare("SELECT COUNT(*) as cnt FROM peers").first<{
      cnt: number;
    }>();
    if (count && count.cnt >= maxPeers) {
      await env.DB.prepare(
        "DELETE FROM peers WHERE id = (SELECT id FROM peers ORDER BY last_seen ASC LIMIT 1)"
      ).run();
    }

    await env.DB.prepare(
      "INSERT INTO peers (peer_id, multiaddrs, last_seen, node_id, reticulum_address) VALUES (?, ?, ?, ?, ?)"
    )
      .bind(body.peer_id, JSON.stringify(body.multiaddrs), now, body.node_id || "", reticulumAddr)
      .run();
  }

  return corsHeaders(
    Response.json({ ok: true, peer_id: body.peer_id }),
    undefined,
    env
  );
}

async function handleGetPeers(env: Env): Promise<Response> {
  const ttl = parseInt(env.PEER_TTL_SECONDS || "3600");
  const cutoff = Math.floor(Date.now() / 1000) - ttl;

  await env.DB.prepare("DELETE FROM peers WHERE last_seen < ?").bind(cutoff).run();

  const { results } = await env.DB.prepare(
    "SELECT peer_id, multiaddrs, node_id, reticulum_address FROM peers ORDER BY last_seen DESC LIMIT 50"
  ).all<{ peer_id: string; multiaddrs: string; node_id: string; reticulum_address: string }>();

  const peers = results.map((r) => ({
    peer_id: r.peer_id,
    multiaddrs: JSON.parse(r.multiaddrs) as string[],
    node_id: r.node_id,
    reticulum_address: r.reticulum_address || "",
  }));

  return corsHeaders(Response.json({ peers }), undefined, env);
}

async function handleDeregister(peerId: string, env: Env): Promise<Response> {
  await env.DB.prepare("DELETE FROM peers WHERE peer_id = ?").bind(peerId).run();
  return corsHeaders(Response.json({ ok: true }), undefined, env);
}

function corsHeaders(body: Response | null, status?: number, env?: Env): Response {
  // ponytail: strict CORS — only allow configured origins, never *
  const allowedOrigins = (env?.ALLOWED_ORIGINS || "https://ghostwire.cc,http://localhost:5173")
    .split(",")
    .map((s) => s.trim());

  const headers: Record<string, string> = {
    "Access-Control-Allow-Methods": "GET, POST, DELETE, OPTIONS",
    "Access-Control-Allow-Headers": "Content-Type, X-GW-Signature, X-GW-Timestamp",
    "Cache-Control": "no-store",
  };

  if (!body) return new Response(null, { status: status || 204, headers });
  return new Response(body.body, { status: body.status, headers });
}
