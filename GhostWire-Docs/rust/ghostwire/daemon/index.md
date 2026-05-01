[ghostwire](../index.html)

Module daemon Copy item path

<span class="sub-heading"><a href="../../src/ghostwire/daemon.rs.html#1-220"
class="src">Source</a> </span>

Expand description

Daemon management — start, stop, status, PID file handling

The GhostWire daemon runs the Core engine (mesh networking, encryption,
message routing) as a background process. Clients (TUI, CLI) connect to
it via HTTP/WebSocket — no database lock conflicts.

Usage: ghostwire start — start the daemon in background ghostwire stop —
stop the running daemon ghostwire status — check if daemon is running
ghostwire tui — launch TUI (auto-starts daemon if needed)

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.DEFAULT_DAEMON_PORT.html" class="constant"
title="constant ghostwire::daemon::DEFAULT_DAEMON_PORT">DEFAULT_DAEMON_PORT</a>  
Default daemon port

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.daemon_log_path.html" class="fn"
title="fn ghostwire::daemon::daemon_log_path">daemon_log_path</a>  
Path to the daemon log file

<a href="fn.daemon_status.html" class="fn"
title="fn ghostwire::daemon::daemon_status">daemon_status</a>  
Get daemon status information

<a href="fn.ensure_daemon.html" class="fn"
title="fn ghostwire::daemon::ensure_daemon">ensure_daemon</a>  
Ensure the daemon is running — start it if not

<a href="fn.get_daemon_pid.html" class="fn"
title="fn ghostwire::daemon::get_daemon_pid">get_daemon_pid</a>  
Get the running daemon’s PID, if any

<a href="fn.is_daemon_running.html" class="fn"
title="fn ghostwire::daemon::is_daemon_running">is_daemon_running</a>  
Check if the daemon is currently running

<a href="fn.pid_file_path.html" class="fn"
title="fn ghostwire::daemon::pid_file_path">pid_file_path</a>  
Path to the daemon PID file

<a href="fn.start_daemon.html" class="fn"
title="fn ghostwire::daemon::start_daemon">start_daemon</a>  
Start the daemon in the background

<a href="fn.stop_daemon.html" class="fn"
title="fn ghostwire::daemon::stop_daemon">stop_daemon</a>  
Stop the running daemon
