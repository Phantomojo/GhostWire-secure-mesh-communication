[ghostwire](../index.html)

Module tui Copy item path

<span class="sub-heading"><a href="../../src/ghostwire/tui/mod.rs.html#1-23"
class="src">Source</a> </span>

Expand description

TUI — Terminal User Interface for GhostWire

Modern chat-style terminal interface that connects to the GhostWire
daemon via HTTP API. The daemon owns the database and mesh connections;
the TUI is a thin client that displays data and sends commands.

Architecture: ghostwire start → daemon (background, owns sled DB + mesh)
ghostwire tui → TUI client → HTTP API → daemon ghostwire stop → stops
the daemon

Modules:

-   `theme` — color palette, styles, logo rendering, trust bars
-   `first_run` — guided setup wizard for new users
-   `daemon_client` — HTTP client that talks to the daemon API
-   `app` — main application state, event loop, screen rendering

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.run_tui.html" class="fn"
title="fn ghostwire::tui::run_tui">run_tui</a>  
Run the TUI, connecting to the daemon on the given port
