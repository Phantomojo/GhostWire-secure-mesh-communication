interface Env {
  DB: D1Database;
  PEER_TTL_SECONDS: string;
  MAX_PEERS: string;
}

interface PeerRecord {
  peer_id: string;
  multiaddrs: string;
  last_seen: number;
  node_id: string;
}

export default {
  async fetch(request: Request, env: Env): Promise<Response> {
    const url = new URL(request.url);
    const path = url.pathname;
    const method = request.method;

    if (method === "OPTIONS") {
      return corsHeaders(null, 204);
    }

    if (path === "/api/register" && method === "POST") {
      return handleRegister(request, env);
    }

    if (path === "/api/peers" && method === "GET") {
      return handleGetPeers(env);
    }

    if (path.startsWith("/api/peers/") && method === "DELETE") {
      const peerId = path.split("/api/peers/")[1];
      return handleDeregister(peerId, env);
    }

    if (path === "/api/health") {
      return corsHeaders(Response.json({ status: "ok", version: "1.0.0" }));
    }

    return corsHeaders(Response.json({ error: "not found" }, { status: 404 }));
  },
};

async function handleRegister(request: Request, env: Env): Promise<Response> {
  let body: { peer_id: string; multiaddrs: string[]; node_id?: string };
  try {
    body = await request.json();
  } catch {
    return corsHeaders(Response.json({ error: "invalid json" }, { status: 400 }));
  }

  if (!body.peer_id || !body.multiaddrs?.length) {
    return corsHeaders(
      Response.json({ error: "peer_id and multiaddrs required" }, { status: 400 })
    );
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
      "UPDATE peers SET multiaddrs = ?, last_seen = ?, node_id = ? WHERE peer_id = ?"
    )
      .bind(JSON.stringify(body.multiaddrs), now, body.node_id || "", body.peer_id)
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
      "INSERT INTO peers (peer_id, multiaddrs, last_seen, node_id) VALUES (?, ?, ?, ?)"
    )
      .bind(body.peer_id, JSON.stringify(body.multiaddrs), now, body.node_id || "")
      .run();
  }

  return corsHeaders(
    Response.json({ ok: true, peer_id: body.peer_id })
  );
}

async function handleGetPeers(env: Env): Promise<Response> {
  const ttl = parseInt(env.PEER_TTL_SECONDS || "3600");
  const cutoff = Math.floor(Date.now() / 1000) - ttl;

  await env.DB.prepare("DELETE FROM peers WHERE last_seen < ?").bind(cutoff).run();

  const { results } = await env.DB.prepare(
    "SELECT peer_id, multiaddrs, node_id FROM peers ORDER BY last_seen DESC LIMIT 50"
  ).all<{ peer_id: string; multiaddrs: string; node_id: string }>();

  const peers = results.map((r) => ({
    peer_id: r.peer_id,
    multiaddrs: JSON.parse(r.multiaddrs) as string[],
    node_id: r.node_id,
  }));

  return corsHeaders(Response.json({ peers }));
}

async function handleDeregister(peerId: string, env: Env): Promise<Response> {
  await env.DB.prepare("DELETE FROM peers WHERE peer_id = ?").bind(peerId).run();
  return corsHeaders(Response.json({ ok: true }));
}

function corsHeaders(body: Response | null, status?: number): Response {
  const headers = {
    "Access-Control-Allow-Origin": "*",
    "Access-Control-Allow-Methods": "GET, POST, DELETE, OPTIONS",
    "Access-Control-Allow-Headers": "Content-Type",
    "Cache-Control": "no-store",
  };
  if (!body) return new Response(null, { status: status || 204, headers });
  return new Response(body.body, { status: body.status, headers });
}
