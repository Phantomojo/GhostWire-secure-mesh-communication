[ort](../index.html)

Module logging Copy item path

<span class="sub-heading"><a href="../../src/ort/logging.rs.html#1-250" class="src">Source</a>
</span>

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Logger.html" class="struct"
title="struct ort::logging::Logger">Logger</a>  
A reference to a session’s logger, typically obtained in custom operator
contexts.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.LogLevel.html" class="enum"
title="enum ort::logging::LogLevel">LogLevel</a>

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.LoggerFunction.html" class="type"
title="type ort::logging::LoggerFunction">LoggerFunction</a>  
`LoggerFunction` accepts the message’s
[`LogLevel`](enum.LogLevel.html "enum ort::logging::LogLevel"), its
category, log ID, code location, and the message itself.
