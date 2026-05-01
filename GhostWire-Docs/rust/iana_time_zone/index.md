Crate iana\_time\_zone Copy item path

<span class="sub-heading"><a href="../src/iana_time_zone/lib.rs.html#1-119" class="src">Source</a>
</span>

Expand description

get the IANA time zone for the current system

This small utility crate provides the
[`get_timezone()`](fn.get_timezone.html) function.

``` {.rust .rust-example-rendered}
// Get the current time zone as a string.
let tz_str = iana_time_zone::get_timezone()?;
println!("The current time zone is: {}", tz_str);
```

The resulting string can be parsed to a
[`chrono-tz::Tz`](https://docs.rs/chrono-tz/latest/chrono_tz/enum.Tz.html)
variant like this:

``` {.rust .rust-example-rendered}
let tz_str = iana_time_zone::get_timezone()?;
let tz: chrono_tz::Tz = tz_str.parse()?;
```

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.GetTimezoneError.html" class="enum"
title="enum iana_time_zone::GetTimezoneError">GetTimezoneError</a>  
Error types

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.get_timezone.html" class="fn"
title="fn iana_time_zone::get_timezone">get_timezone</a>  
Get the current IANA time zone as a string.
