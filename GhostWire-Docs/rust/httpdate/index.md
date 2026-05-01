Crate httpdate Copy item path

<span class="sub-heading"><a href="../src/httpdate/lib.rs.html#1-160" class="src">Source</a>
</span>

Expand description

Date and time utils for HTTP.

Multiple HTTP header fields store timestamps. For example a response
created on May 15, 2015 may contain the header
`Date: Fri, 15 May 2015 15:34:21 GMT`. Since the timestamp does not
contain any timezone or leap second information it is equvivalent to
writing 1431696861 Unix time. Rust’s `SystemTime` is used to store these
timestamps.

This crate provides two public functions:

-   `parse_http_date` to parse a HTTP datetime string to a system time
-   `fmt_http_date` to format a system time to a IMF-fixdate

In addition it exposes the `HttpDate` type that can be used to parse and
format timestamps. Convert a sytem time to `HttpDate` and vice versa.
The `HttpDate` (8 bytes) is smaller than `SystemTime` (16 bytes) and
using the display impl avoids a temporary allocation.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Error.html" class="struct"
title="struct httpdate::Error">Error</a>  
An opaque error type for all parsing errors.

<a href="struct.HttpDate.html" class="struct"
title="struct httpdate::HttpDate">HttpDate</a>  
HTTP timestamp type.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.fmt_http_date.html" class="fn"
title="fn httpdate::fmt_http_date">fmt_http_date</a>  
Format a date to be used in a HTTP header field.

<a href="fn.parse_http_date.html" class="fn"
title="fn httpdate::parse_http_date">parse_http_date</a>  
Parse a date from an HTTP header field.
