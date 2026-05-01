Crate chrono Copy item path

<span class="sub-heading"><a href="../src/chrono/lib.rs.html#1-743" class="src">Source</a>
</span>

Expand description

# <a href="#chrono-date-and-time-for-rust" class="doc-anchor">§</a>Chrono: Date and Time for Rust

Chrono aims to provide all functionality needed to do correct operations
on dates and times in the [proleptic Gregorian
calendar](https://en.wikipedia.org/wiki/Proleptic_Gregorian_calendar):

-   The [`DateTime`](struct.DateTime.html "struct chrono::DateTime")
    type is timezone-aware by default, with separate timezone-naive
    types.
-   Operations that may produce an invalid or ambiguous date and time
    return `Option` or
    [`MappedLocalTime`](offset/type.MappedLocalTime.html "type chrono::offset::MappedLocalTime").
-   Configurable parsing and formatting with a `strftime` inspired date
    and time formatting syntax.
-   The [`Local`](struct.Local.html "struct chrono::Local") timezone
    works with the current timezone of the OS.
-   Types and operations are implemented to be reasonably efficient.

Timezone data is not shipped with chrono by default to limit binary
sizes. Use the companion crate
[Chrono-TZ](https://crates.io/crates/chrono-tz) or
[`tzfile`](https://crates.io/crates/tzfile) for full timezone support.

### <a href="#features" class="doc-anchor">§</a>Features

Chrono supports various runtime environments and operating systems, and
has several features that may be enabled or disabled.

Default features:

-   `alloc`: Enable features that depend on allocation (primarily string
    formatting).
-   `std`: Enables functionality that depends on the standard library.
    This is a superset of `alloc` and adds interoperation with standard
    library types and traits.
-   `clock`: Enables reading the local timezone (`Local`). This is a
    superset of `now`.
-   `now`: Enables reading the system time (`now`).
-   `wasmbind`: Interface with the JS Date API for the `wasm32` target.

Optional features:

-   `serde`: Enable serialization/deserialization via
    [serde](https://github.com/serde-rs/serde).
-   `rkyv`: Deprecated, use the `rkyv-*` features.
-   `rkyv-16`: Enable serialization/deserialization via
    [rkyv](https://github.com/rkyv/rkyv), using 16-bit integers for
    integral `*size` types.
-   `rkyv-32`: Enable serialization/deserialization via
    [rkyv](https://github.com/rkyv/rkyv), using 32-bit integers for
    integral `*size` types.
-   `rkyv-64`: Enable serialization/deserialization via
    [rkyv](https://github.com/rkyv/rkyv), using 64-bit integers for
    integral `*size` types.
-   `rkyv-validation`: Enable rkyv validation support using `bytecheck`.
-   `arbitrary`: Construct arbitrary instances of a type with the
    Arbitrary crate.
-   `unstable-locales`: Enable localization. This adds various methods
    with a `_localized` suffix. The implementation and API may change or
    even be removed in a patch release. Feedback welcome.
-   `oldtime`: This feature no longer has any effect; it used to offer
    compatibility with the `time` 0.1 crate.

Note: The `rkyv{,-16,-32,-64}` features are mutually exclusive.

See the [cargo
docs](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#choosing-features)
for examples of specifying features.

## <a href="#overview" class="doc-anchor">§</a>Overview

### <a href="#time-delta--duration" class="doc-anchor">§</a>Time delta / Duration

Chrono has a
[`TimeDelta`](struct.TimeDelta.html "struct chrono::TimeDelta") type to
represent the magnitude of a time span. This is an “accurate” duration
represented as seconds and nanoseconds, and does not represent “nominal”
components such as days or months.

The [`TimeDelta`](struct.TimeDelta.html "struct chrono::TimeDelta") type
was previously named `Duration` (and is still available as a type alias
with that name). A notable difference with the similar
[`core::time::Duration`](https://doc.rust-lang.org/1.93.1/core/time/struct.Duration.html "struct core::time::Duration")
is that it is a signed value instead of unsigned.

Chrono currently only supports a small number of operations with
[`core::time::Duration`](https://doc.rust-lang.org/1.93.1/core/time/struct.Duration.html "struct core::time::Duration").
You can convert between both types with the
[`TimeDelta::from_std`](struct.TimeDelta.html#method.from_std "associated function chrono::TimeDelta::from_std")
and
[`TimeDelta::to_std`](struct.TimeDelta.html#method.to_std "method chrono::TimeDelta::to_std")
methods.

### <a href="#date-and-time" class="doc-anchor">§</a>Date and Time

Chrono provides a
[`DateTime`](struct.DateTime.html "struct chrono::DateTime") type to
represent a date and a time in a timezone.

For more abstract moment-in-time tracking such as internal timekeeping
that is unconcerned with timezones, consider
[`std::time::SystemTime`](https://doc.rust-lang.org/1.93.1/std/time/struct.SystemTime.html "struct std::time::SystemTime"),
which tracks your system clock, or
[`std::time::Instant`](https://doc.rust-lang.org/1.93.1/std/time/struct.Instant.html "struct std::time::Instant"),
which is an opaque but monotonically-increasing representation of a
moment in time.

[`DateTime`](struct.DateTime.html "struct chrono::DateTime") is
timezone-aware and must be constructed from a
[`TimeZone`](trait.TimeZone.html "trait chrono::TimeZone") object, which
defines how the local date is converted to and back from the UTC date.
There are three well-known
[`TimeZone`](trait.TimeZone.html "trait chrono::TimeZone")
implementations:

-   [`Utc`](struct.Utc.html "struct chrono::Utc") specifies the UTC time
    zone. It is most efficient.

-   [`Local`](struct.Local.html "struct chrono::Local") specifies the
    system local time zone.

-   [`FixedOffset`](struct.FixedOffset.html "struct chrono::FixedOffset")
    specifies an arbitrary, fixed time zone such as UTC+09:00 or
    UTC-10:30. This often results from the parsed textual date and time.
    Since it stores the most information and does not depend on the
    system environment, you would want to normalize other `TimeZone`s
    into this type.

[`DateTime`](struct.DateTime.html "struct chrono::DateTime")s with
different [`TimeZone`](trait.TimeZone.html "trait chrono::TimeZone")
types are distinct and do not mix, but can be converted to each other
using the
[`DateTime::with_timezone`](struct.DateTime.html#method.with_timezone "method chrono::DateTime::with_timezone")
method.

You can get the current date and time in the UTC time zone
([`Utc::now()`](struct.Utc.html#method.now "associated function chrono::Utc::now"))
or in the local time zone
([`Local::now()`](struct.Local.html#method.now "associated function chrono::Local::now")).

``` {.rust .rust-example-rendered}
use chrono::prelude::*;

let utc: DateTime<Utc> = Utc::now(); // e.g. `2014-11-28T12:45:59.324310806Z`
```

``` {.rust .rust-example-rendered}
use chrono::prelude::*;

let local: DateTime<Local> = Local::now(); // e.g. `2014-11-28T21:45:59.324310806+09:00`
```

Alternatively, you can create your own date and time. This is a bit
verbose due to Rust’s lack of function and method overloading, but in
turn we get a rich combination of initialization methods.

``` {.rust .rust-example-rendered}
use chrono::offset::MappedLocalTime;
use chrono::prelude::*;


let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
assert_eq!(
    dt,
    NaiveDate::from_ymd_opt(2014, 7, 8)?
        .and_hms_opt(9, 10, 11)?
        .and_utc()
);

// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, NaiveDate::from_yo_opt(2014, 189)?.and_hms_opt(9, 10, 11)?.and_utc());
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(
    dt,
    NaiveDate::from_isoywd_opt(2014, 28, Weekday::Tue)?.and_hms_opt(9, 10, 11)?.and_utc()
);

let dt = NaiveDate::from_ymd_opt(2014, 7, 8)?
    .and_hms_milli_opt(9, 10, 11, 12)?
    .and_utc(); // `2014-07-08T09:10:11.012Z`
assert_eq!(
    dt,
    NaiveDate::from_ymd_opt(2014, 7, 8)?
        .and_hms_micro_opt(9, 10, 11, 12_000)?
        .and_utc()
);
assert_eq!(
    dt,
    NaiveDate::from_ymd_opt(2014, 7, 8)?
        .and_hms_nano_opt(9, 10, 11, 12_000_000)?
        .and_utc()
);

// dynamic verification
assert_eq!(
    Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33),
    MappedLocalTime::Single(
        NaiveDate::from_ymd_opt(2014, 7, 8)?.and_hms_opt(21, 15, 33)?.and_utc()
    )
);
assert_eq!(Utc.with_ymd_and_hms(2014, 7, 8, 80, 15, 33), MappedLocalTime::None);
assert_eq!(Utc.with_ymd_and_hms(2014, 7, 38, 21, 15, 33), MappedLocalTime::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local
    .from_local_datetime(
        &NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap(),
    )
    .unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600)
    .unwrap()
    .from_local_datetime(
        &NaiveDate::from_ymd_opt(2014, 7, 8)
            .unwrap()
            .and_hms_milli_opt(18, 10, 11, 12)
            .unwrap(),
    )
    .unwrap();
assert_eq!(dt, fixed_dt);
```

Various properties are available to the date and time, and can be
altered individually. Most of them are defined in the traits
[`Datelike`](trait.Datelike.html "trait chrono::Datelike") and
[`Timelike`](trait.Timelike.html "trait chrono::Timelike") which you
should `use` before. Addition and subtraction is also supported. The
following illustrates most supported operations to the date and time:

``` {.rust .rust-example-rendered}
use chrono::prelude::*;
use chrono::TimeDelta;

// assume this returned `2014-11-28T21:45:59.324310806+09:00`:
let dt = FixedOffset::east_opt(9 * 3600)
    .unwrap()
    .from_local_datetime(
        &NaiveDate::from_ymd_opt(2014, 11, 28)
            .unwrap()
            .and_hms_nano_opt(21, 45, 59, 324310806)
            .unwrap(),
    )
    .unwrap();

// property accessors
assert_eq!((dt.year(), dt.month(), dt.day()), (2014, 11, 28));
assert_eq!((dt.month0(), dt.day0()), (10, 27)); // for unfortunate souls
assert_eq!((dt.hour(), dt.minute(), dt.second()), (21, 45, 59));
assert_eq!(dt.weekday(), Weekday::Fri);
assert_eq!(dt.weekday().number_from_monday(), 5); // Mon=1, ..., Sun=7
assert_eq!(dt.ordinal(), 332); // the day of year
assert_eq!(dt.num_days_from_ce(), 735565); // the number of days from and including Jan 1, 1

// time zone accessor and manipulation
assert_eq!(dt.offset().fix().local_minus_utc(), 9 * 3600);
assert_eq!(dt.timezone(), FixedOffset::east_opt(9 * 3600).unwrap());
assert_eq!(
    dt.with_timezone(&Utc),
    NaiveDate::from_ymd_opt(2014, 11, 28)
        .unwrap()
        .and_hms_nano_opt(12, 45, 59, 324310806)
        .unwrap()
        .and_utc()
);

// a sample of property manipulations (validates dynamically)
assert_eq!(dt.with_day(29).unwrap().weekday(), Weekday::Sat); // 2014-11-29 is Saturday
assert_eq!(dt.with_day(32), None);
assert_eq!(dt.with_year(-300).unwrap().num_days_from_ce(), -109606); // November 29, 301 BCE

// arithmetic operations
let dt1 = Utc.with_ymd_and_hms(2014, 11, 14, 8, 9, 10).unwrap();
let dt2 = Utc.with_ymd_and_hms(2014, 11, 14, 10, 9, 8).unwrap();
assert_eq!(dt1.signed_duration_since(dt2), TimeDelta::try_seconds(-2 * 3600 + 2).unwrap());
assert_eq!(dt2.signed_duration_since(dt1), TimeDelta::try_seconds(2 * 3600 - 2).unwrap());
assert_eq!(
    Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 0).unwrap()
        + TimeDelta::try_seconds(1_000_000_000).unwrap(),
    Utc.with_ymd_and_hms(2001, 9, 9, 1, 46, 40).unwrap()
);
assert_eq!(
    Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 0).unwrap()
        - TimeDelta::try_seconds(1_000_000_000).unwrap(),
    Utc.with_ymd_and_hms(1938, 4, 24, 22, 13, 20).unwrap()
);
```

### <a href="#formatting-and-parsing" class="doc-anchor">§</a>Formatting and Parsing

Formatting is done via the
[`format`](struct.DateTime.html#method.format "method chrono::DateTime::format")
method, which format is equivalent to the familiar `strftime` format.

See
[`format::strftime`](format/strftime/index.html#specifiers "mod chrono::format::strftime")
documentation for full syntax and list of specifiers.

The default `to_string` method and `{:?}` specifier also give a
reasonable representation. Chrono also provides
[`to_rfc2822`](struct.DateTime.html#method.to_rfc2822 "method chrono::DateTime::to_rfc2822")
and
[`to_rfc3339`](struct.DateTime.html#method.to_rfc3339 "method chrono::DateTime::to_rfc3339")
methods for well-known formats.

Chrono now also provides date formatting in almost any language without
the help of an additional C library. This functionality is under the
feature `unstable-locales`:

``` language-toml
chrono = { version = "0.4", features = ["unstable-locales"] }
```

The `unstable-locales` feature requires and implies at least the `alloc`
feature.

``` {.rust .rust-example-rendered}
use chrono::prelude::*;

let dt = Utc.with_ymd_and_hms(2014, 11, 28, 12, 0, 9).unwrap();
assert_eq!(dt.format("%Y-%m-%d %H:%M:%S").to_string(), "2014-11-28 12:00:09");
assert_eq!(dt.format("%a %b %e %T %Y").to_string(), "Fri Nov 28 12:00:09 2014");
assert_eq!(
    dt.format_localized("%A %e %B %Y, %T", Locale::fr_BE).to_string(),
    "vendredi 28 novembre 2014, 12:00:09"
);

assert_eq!(dt.format("%a %b %e %T %Y").to_string(), dt.format("%c").to_string());
assert_eq!(dt.to_string(), "2014-11-28 12:00:09 UTC");
assert_eq!(dt.to_rfc2822(), "Fri, 28 Nov 2014 12:00:09 +0000");
assert_eq!(dt.to_rfc3339(), "2014-11-28T12:00:09+00:00");
assert_eq!(format!("{:?}", dt), "2014-11-28T12:00:09Z");

// Note that milli/nanoseconds are only printed if they are non-zero
let dt_nano = NaiveDate::from_ymd_opt(2014, 11, 28)
    .unwrap()
    .and_hms_nano_opt(12, 0, 9, 1)
    .unwrap()
    .and_utc();
assert_eq!(format!("{:?}", dt_nano), "2014-11-28T12:00:09.000000001Z");
```

Parsing can be done with two methods:

1.  The standard
    [`FromStr`](https://doc.rust-lang.org/1.93.1/core/str/traits/trait.FromStr.html "trait core::str::traits::FromStr")
    trait (and
    [`parse`](https://doc.rust-lang.org/1.93.1/std/primitive.str.html#method.parse "method str::parse")
    method on a string) can be used for parsing `DateTime<FixedOffset>`,
    `DateTime<Utc>` and `DateTime<Local>` values. This parses what the
    `{:?}`
    ([`std::fmt::Debug`](https://doc.rust-lang.org/1.93.1/core/fmt/trait.Debug.html "trait core::fmt::Debug")
    format specifier prints, and requires the offset to be present.

2.  [`DateTime::parse_from_str`](struct.DateTime.html#method.parse_from_str "associated function chrono::DateTime::parse_from_str")
    parses a date and time with offsets and returns
    `DateTime<FixedOffset>`. This should be used when the offset is a
    part of input and the caller cannot guess that. It *cannot* be used
    when the offset can be missing.
    [`DateTime::parse_from_rfc2822`](struct.DateTime.html#method.parse_from_rfc2822 "associated function chrono::DateTime::parse_from_rfc2822")
    and
    [`DateTime::parse_from_rfc3339`](struct.DateTime.html#method.parse_from_rfc3339 "associated function chrono::DateTime::parse_from_rfc3339")
    are similar but for well-known formats.

More detailed control over the parsing process is available via
[`format`](format/index.html "mod chrono::format") module.

``` {.rust .rust-example-rendered}
use chrono::prelude::*;

let dt = Utc.with_ymd_and_hms(2014, 11, 28, 12, 0, 9).unwrap();
let fixed_dt = dt.with_timezone(&FixedOffset::east_opt(9 * 3600).unwrap());

// method 1
assert_eq!("2014-11-28T12:00:09Z".parse::<DateTime<Utc>>(), Ok(dt.clone()));
assert_eq!("2014-11-28T21:00:09+09:00".parse::<DateTime<Utc>>(), Ok(dt.clone()));
assert_eq!("2014-11-28T21:00:09+09:00".parse::<DateTime<FixedOffset>>(), Ok(fixed_dt.clone()));

// method 2
assert_eq!(
    DateTime::parse_from_str("2014-11-28 21:00:09 +09:00", "%Y-%m-%d %H:%M:%S %z"),
    Ok(fixed_dt.clone())
);
assert_eq!(
    DateTime::parse_from_rfc2822("Fri, 28 Nov 2014 21:00:09 +0900"),
    Ok(fixed_dt.clone())
);
assert_eq!(DateTime::parse_from_rfc3339("2014-11-28T21:00:09+09:00"), Ok(fixed_dt.clone()));

// oops, the year is missing!
assert!(DateTime::parse_from_str("Fri Nov 28 12:00:09", "%a %b %e %T %Y").is_err());
// oops, the format string does not include the year at all!
assert!(DateTime::parse_from_str("Fri Nov 28 12:00:09", "%a %b %e %T").is_err());
// oops, the weekday is incorrect!
assert!(DateTime::parse_from_str("Sat Nov 28 12:00:09 2014", "%a %b %e %T %Y").is_err());
```

Again: See
[`format::strftime`](format/strftime/index.html#specifiers "mod chrono::format::strftime")
documentation for full syntax and list of specifiers.

### <a href="#conversion-from-and-to-epoch-timestamps"
class="doc-anchor">§</a>Conversion from and to EPOCH timestamps

Use
[`DateTime::from_timestamp(seconds, nanoseconds)`](struct.DateTime.html#method.from_timestamp "associated function chrono::DateTime::from_timestamp")
to construct a
[`DateTime<Utc>`](struct.DateTime.html "struct chrono::DateTime") from a
UNIX timestamp (seconds, nanoseconds that passed since January 1st
1970).

Use
[`DateTime.timestamp`](struct.DateTime.html#method.timestamp "method chrono::DateTime::timestamp")
to get the timestamp (in seconds) from a
[`DateTime`](struct.DateTime.html "struct chrono::DateTime").
Additionally, you can use
[`DateTime.timestamp_subsec_nanos`](struct.DateTime.html#method.timestamp_subsec_nanos "method chrono::DateTime::timestamp_subsec_nanos")
to get the number of additional number of nanoseconds.

``` {.rust .rust-example-rendered}
// We need the trait in scope to use Utc::timestamp().
use chrono::{DateTime, Utc};

// Construct a datetime from epoch:
let dt: DateTime<Utc> = DateTime::from_timestamp_secs(1_500_000_000).unwrap();
assert_eq!(dt.to_rfc2822(), "Fri, 14 Jul 2017 02:40:00 +0000");

// Get epoch value from a datetime:
let dt = DateTime::parse_from_rfc2822("Fri, 14 Jul 2017 02:40:00 +0000").unwrap();
assert_eq!(dt.timestamp(), 1_500_000_000);
```

### <a href="#naive-date-and-time" class="doc-anchor">§</a>Naive date and time

Chrono provides naive counterparts to `Date`, (non-existent) `Time` and
`DateTime` as
[`NaiveDate`](struct.NaiveDate.html "struct chrono::NaiveDate"),
[`NaiveTime`](struct.NaiveTime.html "struct chrono::NaiveTime") and
[`NaiveDateTime`](struct.NaiveDateTime.html "struct chrono::NaiveDateTime")
respectively.

They have almost equivalent interfaces as their timezone-aware twins,
but are not associated to time zones obviously and can be quite
low-level. They are mostly useful for building blocks for higher-level
types.

Timezone-aware `DateTime` and `Date` types have two methods returning
naive versions:
[`naive_local`](struct.DateTime.html#method.naive_local "method chrono::DateTime::naive_local")
returns a view to the naive local time, and
[`naive_utc`](struct.DateTime.html#method.naive_utc "method chrono::DateTime::naive_utc")
returns a view to the naive UTC time.

## <a href="#limitations" class="doc-anchor">§</a>Limitations

-   Only the proleptic Gregorian calendar (i.e. extended to support
    older dates) is supported.
-   Date types are limited to about +/- 262,000 years from the common
    epoch.
-   Time types are limited to nanosecond accuracy.
-   Leap seconds can be represented, but Chrono does not fully support
    them. See [Leap Second
    Handling](struct.NaiveTime.html#leap-second-handling "struct chrono::NaiveTime").

## <a href="#rust-version-requirements" class="doc-anchor">§</a>Rust version requirements

The Minimum Supported Rust Version (MSRV) is currently **Rust 1.61.0**.

The MSRV is explicitly tested in CI. It may be bumped in minor releases,
but this is not done lightly.

## <a href="#relation-between-chrono-and-time-01" class="doc-anchor">§</a>Relation between chrono and time 0.1

Rust first had a `time` module added to `std` in its 0.7 release. It
later moved to `libextra`, and then to a `libtime` library shipped
alongside the standard library. In 2014 work on chrono started in order
to provide a full-featured date and time library in Rust. Some
improvements from chrono made it into the standard library; notably,
`chrono::Duration` was included as `std::time::Duration`
([rust#15934](https://github.com/rust-lang/rust/pull/15934)) in 2014.

In preparation of Rust 1.0 at the end of 2014 `libtime` was moved out of
the Rust distro and into the `time` crate to eventually be redesigned
([rust#18832](https://github.com/rust-lang/rust/pull/18832#issuecomment-62448221),
[rust#18858](https://github.com/rust-lang/rust/pull/18858)), like the
`num` and `rand` crates. Of course chrono kept its dependency on this
`time` crate. `time` started re-exporting `std::time::Duration` during
this period. Later, the standard library was changed to have a more
limited unsigned `Duration` type
([rust#24920](https://github.com/rust-lang/rust/pull/24920), [RFC
1040](https://rust-lang.github.io/rfcs/1040-duration-reform.html)),
while the `time` crate kept the full functionality with
`time::Duration`. `time::Duration` had been a part of chrono’s public
API.

By 2016 `time` 0.1 lived under the `rust-lang-deprecated` organisation
and was not actively maintained
([time#136](https://github.com/time-rs/time/issues/136)). chrono
absorbed the platform functionality and `Duration` type of the `time`
crate in [chrono#478](https://github.com/chronotope/chrono/pull/478)
(the work started in
[chrono#286](https://github.com/chronotope/chrono/pull/286)). In order
to preserve compatibility with downstream crates depending on `time` and
`chrono` sharing a `Duration` type, chrono kept depending on time 0.1.
chrono offered the option to opt out of the `time` dependency by
disabling the `oldtime` feature (swapping it out for an effectively
similar chrono type). In 2019, @jhpratt took over maintenance on the
`time` crate and released what amounts to a new crate as `time` 0.2.

## <a href="#security-advisories" class="doc-anchor">§</a>Security advisories

In November of 2020
[CVE-2020-26235](https://nvd.nist.gov/vuln/detail/CVE-2020-26235) and
[RUSTSEC-2020-0071](https://rustsec.org/advisories/RUSTSEC-2020-0071)
were opened against the `time` crate. @quininer had found that calls to
`localtime_r` may be unsound
([chrono#499](https://github.com/chronotope/chrono/pull/499)).
Eventually, almost a year later, this was also made into a security
advisory against chrono as
[RUSTSEC-2020-0159](https://rustsec.org/advisories/RUSTSEC-2020-0159.html),
which had platform code similar to `time`.

On Unix-like systems a process is given a timezone id or description via
the `TZ` environment variable. We need this timezone data to calculate
the current local time from a value that is in UTC, such as the time
from the system clock. `time` 0.1 and chrono used the POSIX function
`localtime_r` to do the conversion to local time, which reads the `TZ`
variable.

Rust assumes the environment to be writable and uses locks to access it
from multiple threads. Some other programming languages and libraries
use similar locking strategies, but these are typically not shared
across languages. More importantly, POSIX declares modifying the
environment in a multi-threaded process as unsafe, and `getenv` in libc
can’t be changed to take a lock because it returns a pointer to the data
(see [rust#27970](https://github.com/rust-lang/rust/issues/27970) for
more discussion).

Since version 4.20 chrono no longer uses `localtime_r`, instead using
Rust code to query the timezone (from the `TZ` variable or via
`iana-time-zone` as a fallback) and work with data from the system
timezone database directly. The code for this was forked from the [tz-rs
crate](https://crates.io/crates/tz-rs) by @x-hgg-x. As such, chrono now
respects the Rust lock when reading the `TZ` environment variable. In
general, code should avoid modifying the environment.

## <a href="#removing-time-01" class="doc-anchor">§</a>Removing time 0.1

Because time 0.1 has been unmaintained for years, however, the security
advisory mentioned above has not been addressed. While chrono
maintainers were careful not to break backwards compatibility with the
`time::Duration` type, there has been a long stream of issues from users
inquiring about the time 0.1 dependency with the vulnerability. We
investigated the potential breakage of removing the time 0.1 dependency
in [chrono#1095](https://github.com/chronotope/chrono/pull/1095) using a
crater-like experiment and determined that the potential for breaking
(public) dependencies is very low. We reached out to those few crates
that did still depend on compatibility with time 0.1.

As such, for chrono 0.4.30 we have decided to swap out the time 0.1
`Duration` implementation for a local one that will offer a strict
superset of the existing API going forward. This will prevent most
downstream users from being affected by the security vulnerability in
time 0.1 while minimizing the ecosystem impact of semver-incompatible
version churn.

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use time_delta::OutOfRangeError;`

`pub use date::MAX_DATE;`<span class="stab deprecated"
title="">Deprecated</span>

`pub use date::MIN_DATE;`<span class="stab deprecated"
title="">Deprecated</span>

`pub use datetime::MAX_DATETIME;`<span class="stab deprecated"
title="">Deprecated</span>

`pub use datetime::MIN_DATETIME;`<span class="stab deprecated"
title="">Deprecated</span>

`pub use format::`<a href="format/struct.ParseError.html" class="struct"
title="struct chrono::format::ParseError"><code>ParseError</code></a>`;`

`pub use format::`<a href="format/type.ParseResult.html" class="type"
title="type chrono::format::ParseResult"><code>ParseResult</code></a>`;`

`pub use format::`<a href="format/enum.SecondsFormat.html" class="enum"
title="enum chrono::format::SecondsFormat"><code>SecondsFormat</code></a>`;`

`pub use naive::`<a href="naive/struct.IsoWeek.html" class="struct"
title="struct chrono::naive::IsoWeek"><code>IsoWeek</code></a>`;`

`pub use naive::`<a href="naive/struct.NaiveWeek.html" class="struct"
title="struct chrono::naive::NaiveWeek"><code>NaiveWeek</code></a>`;`

`pub use offset::`<a href="offset/type.MappedLocalTime.html" class="type"
title="type chrono::offset::MappedLocalTime"><code>MappedLocalTime</code></a>`;`

`pub use round::`<a href="round/trait.DurationRound.html" class="trait"
title="trait chrono::round::DurationRound"><code>DurationRound</code></a>`;`

`pub use round::`<a href="round/enum.RoundingError.html" class="enum"
title="enum chrono::round::RoundingError"><code>RoundingError</code></a>`;`

`pub use round::`<a href="round/trait.SubsecRound.html" class="trait"
title="trait chrono::round::SubsecRound"><code>SubsecRound</code></a>`;`

`pub use weekday::ParseWeekdayError;`

`pub use month::ParseMonthError;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="format/index.html" class="mod"
title="mod chrono::format">format</a>  
Formatting (and parsing) utilities for date and time.

<a href="naive/index.html" class="mod"
title="mod chrono::naive">naive</a>  
Date and time types unconcerned with timezones.

<a href="offset/index.html" class="mod"
title="mod chrono::offset">offset</a>  
The time zone, which calculates offsets from the local time to UTC.

<a href="prelude/index.html" class="mod"
title="mod chrono::prelude">prelude</a>  
A convenience module appropriate for glob imports
(`use chrono::prelude::*;`).

<a href="round/index.html" class="mod"
title="mod chrono::round">round</a>  
Functionality for rounding or truncating a `DateTime` by a `TimeDelta`.

<a href="serde/index.html" class="mod"
title="mod chrono::serde">serde</a>  
Serialization/Deserialization with serde

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Date.html" class="struct"
title="struct chrono::Date">Date</a><span class="stab deprecated" title="">Deprecated</span>  
ISO 8601 calendar date with time zone.

<a href="struct.DateTime.html" class="struct"
title="struct chrono::DateTime">DateTime</a>  
ISO 8601 combined date and time with time zone.

<a href="struct.Days.html" class="struct"
title="struct chrono::Days">Days</a>  
A duration in calendar days.

<a href="struct.FixedOffset.html" class="struct"
title="struct chrono::FixedOffset">FixedOffset</a>  
The time zone with fixed offset, from UTC-23:59:59 to UTC+23:59:59.

<a href="struct.Local.html" class="struct"
title="struct chrono::Local">Local</a>  
The local timescale.

<a href="struct.Months.html" class="struct"
title="struct chrono::Months">Months</a>  
A duration in calendar months

<a href="struct.NaiveDate.html" class="struct"
title="struct chrono::NaiveDate">NaiveDate</a>  
ISO 8601 calendar date without timezone. Allows for every [proleptic
Gregorian
date](struct.NaiveDate.html#calendar-date "struct chrono::NaiveDate")
from Jan 1, 262145 BCE to Dec 31, 262143 CE. Also supports the
conversion from ISO 8601 ordinal and week date.

<a href="struct.NaiveDateTime.html" class="struct"
title="struct chrono::NaiveDateTime">NaiveDateTime</a>  
ISO 8601 combined date and time without timezone.

<a href="struct.NaiveTime.html" class="struct"
title="struct chrono::NaiveTime">NaiveTime</a>  
ISO 8601 time without timezone. Allows for the nanosecond precision and
optional leap second representation.

<a href="struct.OutOfRange.html" class="struct"
title="struct chrono::OutOfRange">OutOfRange</a>  
Out of range error type used in various converting APIs

<a href="struct.TimeDelta.html" class="struct"
title="struct chrono::TimeDelta">TimeDelta</a>  
Time duration with nanosecond precision.

<a href="struct.Utc.html" class="struct"
title="struct chrono::Utc">Utc</a>  
The UTC time zone. This is the most efficient time zone when you don’t
need the local time. It is also used as an offset (which is also a dummy
type).

<a href="struct.WeekdaySet.html" class="struct"
title="struct chrono::WeekdaySet">WeekdaySet</a>  
A collection of [`Weekday`](enum.Weekday.html "enum chrono::Weekday")s
stored as a single byte.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Month.html" class="enum"
title="enum chrono::Month">Month</a>  
The month of the year.

<a href="enum.Weekday.html" class="enum"
title="enum chrono::Weekday">Weekday</a>  
The day of week.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Datelike.html" class="trait"
title="trait chrono::Datelike">Datelike</a>  
The common set of methods for date component.

<a href="trait.Offset.html" class="trait"
title="trait chrono::Offset">Offset</a>  
The offset from the local time to UTC.

<a href="trait.TimeZone.html" class="trait"
title="trait chrono::TimeZone">TimeZone</a>  
The time zone.

<a href="trait.Timelike.html" class="trait"
title="trait chrono::Timelike">Timelike</a>  
The common set of methods for time component.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Duration.html" class="type"
title="type chrono::Duration">Duration</a>  
Alias of
[`TimeDelta`](struct.TimeDelta.html "struct chrono::TimeDelta").
