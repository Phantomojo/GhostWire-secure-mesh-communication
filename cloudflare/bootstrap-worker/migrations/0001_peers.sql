CREATE TABLE IF NOT EXISTS peers (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  peer_id TEXT NOT NULL UNIQUE,
  multiaddrs TEXT NOT NULL,
  last_seen INTEGER NOT NULL,
  node_id TEXT DEFAULT ''
);

CREATE INDEX IF NOT EXISTS idx_peers_last_seen ON peers(last_seen);
CREATE INDEX IF NOT EXISTS idx_peers_peer_id ON peers(peer_id);
