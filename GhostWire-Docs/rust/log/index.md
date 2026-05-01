Crate log Copy item path

<span class="sub-heading"><a href="../src/log/lib.rs.html#11-2010" class="src">Source</a>
</span>

Expand description

A lightweight logging facade.

The `log` crate provides a single logging API that abstracts over the
actual logging implementation. Libraries can use the logging API
provided by this crate, and the consumer of those libraries can choose
the logging implementation that is most suitable for its use case.

If no logging implementation is selected, the facade falls back to a
“noop” implementation that ignores all log messages. The overhead in
this case is very small - just an integer load, comparison and jump.

A log request consists of a *target*, a *level*, and a *body*. A target
is a string which defaults to the module path of the location of the log
request, though that default may be overridden. Logger implementations
typically use the target to filter requests based on some user
configuration.

# <a href="#usage" class="doc-anchor">§</a>Usage

The basic use of the log crate is through the five logging macros:
[`error!`](./macro.error.html), [`warn!`](./macro.warn.html),
[`info!`](./macro.info.html), [`debug!`](./macro.debug.html) and
[`trace!`](./macro.trace.html) where `error!` represents the
highest-priority log messages and `trace!` the lowest. The log messages
are filtered by configuring the log level to exclude messages with a
lower priority. Each of these macros accept format strings similarly to
[`println!`](https://doc.rust-lang.org/stable/std/macro.println.html).

Avoid writing expressions with side-effects in log statements. They may
not be evaluated.

## <a href="#in-libraries" class="doc-anchor">§</a>In libraries

Libraries should link only to the `log` crate, and use the provided
macros to log whatever information will be useful to downstream
consumers.

### <a href="#examples" class="doc-anchor">§</a>Examples

``` {.rust .rust-example-rendered}
use log::{info, warn};

pub fn shave_the_yak(yak: &mut Yak) {
    info!(target: "yak_events", "Commencing yak shaving for {yak:?}");

    loop {
        match find_a_razor() {
            Ok(razor) => {
                info!("Razor located: {razor}");
                yak.shave(razor);
                break;
            }
            Err(err) => {
                warn!("Unable to locate a razor: {err}, retrying");
            }
        }
    }
}
```

## <a href="#in-executables" class="doc-anchor">§</a>In executables

Executables should choose a logging implementation and initialize it
early in the runtime of the program. Logging implementations will
typically include a function to do this. Any log messages generated
before the implementation is initialized will be ignored.

The executable itself may use the `log` crate to log as well.

### <a href="#warning" class="doc-anchor">§</a>Warning

The logging system may only be initialized once.

## <a href="#structured-logging" class="doc-anchor">§</a>Structured logging

If you enable the `kv` feature you can associate structured values with
your log records. If we take the example from before, we can include
some additional context besides what’s in the formatted message:

``` {.rust .rust-example-rendered}
use log::{info, warn};

pub fn shave_the_yak(yak: &mut Yak) {
    info!(target: "yak_events", yak:serde; "Commencing yak shaving");

    loop {
        match find_a_razor() {
            Ok(razor) => {
                info!(razor; "Razor located");
                yak.shave(razor);
                break;
            }
            Err(e) => {
                warn!(e:err; "Unable to locate a razor, retrying");
            }
        }
    }
}
```

See the \[`kv`\] module documentation for more details.

# <a href="#available-logging-implementations" class="doc-anchor">§</a>Available logging implementations

In order to produce log output executables have to use a logger
implementation compatible with the facade. There are many available
implementations to choose from, here are some of the most popular ones:

-   Simple minimal loggers:
    -   [env\_logger](https://docs.rs/env_logger/*/env_logger/)
    -   [colog](https://docs.rs/colog/*/colog/)
    -   [simple\_logger](https://github.com/borntyping/rust-simple_logger)
    -   [simplelog](https://github.com/drakulix/simplelog.rs)
    -   [pretty\_env\_logger](https://docs.rs/pretty_env_logger/*/pretty_env_logger/)
    -   [stderrlog](https://docs.rs/stderrlog/*/stderrlog/)
    -   [flexi\_logger](https://docs.rs/flexi_logger/*/flexi_logger/)
    -   [call\_logger](https://docs.rs/call_logger/*/call_logger/)
    -   [std-logger](https://docs.rs/std-logger/*/std_logger/)
    -   [structured-logger](https://docs.rs/structured-logger/latest/structured_logger/)
    -   [clang\_log](https://docs.rs/clang_log/latest/clang_log)
    -   [ftail](https://docs.rs/ftail/latest/ftail)
-   Complex configurable frameworks:
    -   [log4rs](https://docs.rs/log4rs/*/log4rs/)
    -   [logforth](https://docs.rs/logforth/*/logforth/)
    -   [fern](https://docs.rs/fern/*/fern/)
    -   [spdlog-rs](https://docs.rs/spdlog-rs/*/spdlog/)
-   Adaptors for other facilities:
    -   [syslog](https://docs.rs/syslog/*/syslog/)
    -   [slog-stdlog](https://docs.rs/slog-stdlog/*/slog_stdlog/)
    -   [systemd-journal-logger](https://docs.rs/systemd-journal-logger/*/systemd_journal_logger/)
    -   [android\_log](https://docs.rs/android_log/*/android_log/)
    -   [win\_dbg\_logger](https://docs.rs/win_dbg_logger/*/win_dbg_logger/)
    -   [db\_logger](https://docs.rs/db_logger/*/db_logger/)
    -   [log-to-defmt](https://docs.rs/log-to-defmt/*/log_to_defmt/)
    -   [logcontrol-log](https://docs.rs/logcontrol-log/*/logcontrol_log/)
-   For WebAssembly binaries:
    -   [console\_log](https://docs.rs/console_log/*/console_log/)
-   For dynamic libraries:
    -   You may need to construct an FFI-safe wrapper over `log` to
        initialize in your libraries
-   Utilities:
    -   [log\_err](https://docs.rs/log_err/*/log_err/)
    -   [log-reload](https://docs.rs/log-reload/*/log_reload/)
    -   [alterable\_logger](https://docs.rs/alterable_logger/*/alterable_logger)

# <a href="#implementing-a-logger" class="doc-anchor">§</a>Implementing a Logger

Loggers implement the [`Log`](trait.Log.html) trait. Here’s a very basic
example that simply logs all messages at the [`Error`](enum.Level.html),
[`Warn`](enum.Level.html) or [`Info`](enum.Level.html) levels to stdout:

``` {.rust .rust-example-rendered}
use log::{Record, Level, Metadata};

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}
```

Loggers are installed by calling the [`set_logger`](fn.set_logger.html)
function. The maximum log level also needs to be adjusted via the
[`set_max_level`](fn.set_max_level.html) function. The logging facade
uses this as an optimization to improve performance of log messages at
levels that are disabled. It’s important to set it, as it defaults to
[`Off`](enum.LevelFilter.html), so no log messages will ever be
captured! In the case of our example logger, we’ll want to set the
maximum log level to [`Info`](enum.LevelFilter.html), since we ignore
any [`Debug`](enum.Level.html) or [`Trace`](enum.Level.html) level log
messages. A logging implementation should provide a function that wraps
a call to [`set_logger`](fn.set_logger.html) and
[`set_max_level`](fn.set_max_level.html), handling initialization of the
logger:

``` {.rust .rust-example-rendered}
use log::{SetLoggerError, LevelFilter};

static LOGGER: SimpleLogger = SimpleLogger;

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Info))
}
```

Implementations that adjust their configurations at runtime should take
care to adjust the maximum log level as well.

# <a href="#use-with-std" class="doc-anchor">§</a>Use with `std`

`set_logger` requires you to provide a `&'static Log`, which can be hard
to obtain if your logger depends on some runtime configuration. The
`set_boxed_logger` function is available with the `std` Cargo feature.
It is identical to `set_logger` except that it takes a `Box<Log>` rather
than a `&'static Log`:

``` {.rust .rust-example-rendered}
pub fn init() -> Result<(), SetLoggerError> {
    log::set_boxed_logger(Box::new(SimpleLogger))
        .map(|()| log::set_max_level(LevelFilter::Info))
}
```

# <a href="#compile-time-filters" class="doc-anchor">§</a>Compile time filters

Log levels can be statically disabled at compile time by enabling one of
these Cargo features:

-   `max_level_off`
-   `max_level_error`
-   `max_level_warn`
-   `max_level_info`
-   `max_level_debug`
-   `max_level_trace`

Log invocations at disabled levels will be skipped and will not even be
present in the resulting binary. These features control the value of the
`STATIC_MAX_LEVEL` constant. The logging macros check this value before
logging a message. By default, no levels are disabled.

It is possible to override this level for release builds only with the
following features:

-   `release_max_level_off`
-   `release_max_level_error`
-   `release_max_level_warn`
-   `release_max_level_info`
-   `release_max_level_debug`
-   `release_max_level_trace`

Libraries should avoid using the max level features because they’re
global and can’t be changed once they’re set.

For example, a crate can disable trace level logs in debug builds and
trace, debug, and info level logs in release builds with the following
configuration:

``` language-toml
[dependencies]
log = { version = "0.4", features = ["max_level_debug", "release_max_level_warn"] }
```

# <a href="#crate-feature-flags" class="doc-anchor">§</a>Crate Feature Flags

The following crate feature flags are available in addition to the
filters. They are configured in your `Cargo.toml`.

-   `std` allows use of `std` crate instead of the default `core`.
    Enables using `std::error` and `set_boxed_logger` functionality.
-   `serde` enables support for serialization and deserialization of
    `Level` and `LevelFilter`.

``` language-toml
[dependencies]
log = { version = "0.4", features = ["std", "serde"] }
```

# <a href="#version-compatibility" class="doc-anchor">§</a>Version compatibility

The 0.3 and 0.4 versions of the `log` crate are almost entirely
compatible. Log messages made using `log` 0.3 will forward transparently
to a logger implementation using `log` 0.4. Log messages made using
`log` 0.4 will forward to a logger implementation using `log` 0.3, but
the module path and file name information associated with the message
will unfortunately be lost.

# Macros<a href="#macros" class="anchor">§</a>

<a href="macro.debug.html" class="macro"
title="macro log::debug">debug</a>  
Logs a message at the debug level.

<a href="macro.error.html" class="macro"
title="macro log::error">error</a>  
Logs a message at the error level.

<a href="macro.info.html" class="macro" title="macro log::info">info</a>  
Logs a message at the info level.

<a href="macro.log.html" class="macro" title="macro log::log">log</a>  
The standard logging macro.

<a href="macro.log_enabled.html" class="macro"
title="macro log::log_enabled">log_enabled</a>  
Determines if a message logged at the specified level in that module
will be logged.

<a href="macro.trace.html" class="macro"
title="macro log::trace">trace</a>  
Logs a message at the trace level.

<a href="macro.warn.html" class="macro" title="macro log::warn">warn</a>  
Logs a message at the warn level.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Metadata.html" class="struct"
title="struct log::Metadata">Metadata</a>  
Metadata about a log message.

<a href="struct.MetadataBuilder.html" class="struct"
title="struct log::MetadataBuilder">MetadataBuilder</a>  
Builder for [`Metadata`](struct.Metadata.html).

<a href="struct.ParseLevelError.html" class="struct"
title="struct log::ParseLevelError">ParseLevelError</a>  
The type returned by
[`from_str`](https://doc.rust-lang.org/std/str/trait.FromStr.html#tymethod.from_str)
when the string doesn’t match any of the log levels.

<a href="struct.Record.html" class="struct"
title="struct log::Record">Record</a>  
The “payload” of a log message.

<a href="struct.RecordBuilder.html" class="struct"
title="struct log::RecordBuilder">RecordBuilder</a>  
Builder for [`Record`](struct.Record.html).

<a href="struct.SetLoggerError.html" class="struct"
title="struct log::SetLoggerError">SetLoggerError</a>  
The type returned by [`set_logger`](fn.set_logger.html) if
[`set_logger`](fn.set_logger.html) has already been called.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Level.html" class="enum" title="enum log::Level">Level</a>  
An enum representing the available verbosity levels of the logger.

<a href="enum.LevelFilter.html" class="enum"
title="enum log::LevelFilter">LevelFilter</a>  
An enum representing the available verbosity level filters of the
logger.

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.STATIC_MAX_LEVEL.html" class="constant"
title="constant log::STATIC_MAX_LEVEL">STATIC_MAX_LEVEL</a>  
The statically resolved maximum log level.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Log.html" class="trait" title="trait log::Log">Log</a>  
A trait encapsulating the operations required of a logger.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.logger.html" class="fn" title="fn log::logger">logger</a>  
Returns a reference to the logger.

<a href="fn.max_level.html" class="fn"
title="fn log::max_level">max_level</a>  
Returns the current maximum log level.

<a href="fn.set_boxed_logger.html" class="fn"
title="fn log::set_boxed_logger">set_boxed_logger</a>  
Sets the global logger to a `Box<Log>`.

<a href="fn.set_logger.html" class="fn"
title="fn log::set_logger">set_logger</a>  
Sets the global logger to a `&'static Log`.

<a href="fn.set_logger_racy.html" class="fn"
title="fn log::set_logger_racy">set_logger_racy</a><sup>⚠</sup>  
A thread-unsafe version of [`set_logger`](fn.set_logger.html).

<a href="fn.set_max_level.html" class="fn"
title="fn log::set_max_level">set_max_level</a>  
Sets the global maximum log level.

<a href="fn.set_max_level_racy.html" class="fn"
title="fn log::set_max_level_racy">set_max_level_racy</a><sup>⚠</sup>  
A thread-unsafe version of [`set_max_level`](fn.set_max_level.html).
