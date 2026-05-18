# Crate `log`

A lightweight logging facade.

The `log` crate provides a single logging API that abstracts over the
actual logging implementation. Libraries can use the logging API provided
by this crate, and the consumer of those libraries can choose the logging
implementation that is most suitable for its use case.

If no logging implementation is selected, the facade falls back to a "noop"
implementation that ignores all log messages. The overhead in this case
is very small - just an integer load, comparison and jump.

A log request consists of a _target_, a _level_, and a _body_. A target is a
string which defaults to the module path of the location of the log request,
though that default may be overridden. Logger implementations typically use
the target to filter requests based on some user configuration.

# Usage

The basic use of the log crate is through the five logging macros: `error!`,
`warn!`, `info!`, `debug!` and `trace!`
where `error!` represents the highest-priority log messages
and `trace!` the lowest. The log messages are filtered by configuring
the log level to exclude messages with a lower priority.
Each of these macros accept format strings similarly to `println!`.






Avoid writing expressions with side-effects in log statements. They may not be evaluated.

## In libraries

Libraries should link only to the `log` crate, and use the provided
macros to log whatever information will be useful to downstream consumers.

### Examples

```rust
#[derive(Debug)] pub struct Yak(String);
impl Yak { fn shave(&mut self, _: u32) {} }
fn find_a_razor() -> Result<u32, u32> { Ok(1) }
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
fn main() {}
```

## In executables

Executables should choose a logging implementation and initialize it early in the
runtime of the program. Logging implementations will typically include a
function to do this. Any log messages generated before
the implementation is initialized will be ignored.

The executable itself may use the `log` crate to log as well.

### Warning

The logging system may only be initialized once.

## Structured logging

If you enable the `kv` feature you can associate structured values
with your log records. If we take the example from before, we can include
some additional context besides what's in the formatted message:

```rust
use serde::Serialize;
#[derive(Debug, Serialize)] pub struct Yak(String);
impl Yak { fn shave(&mut self, _: u32) {} }
fn find_a_razor() -> Result<u32, std::io::Error> { Ok(1) }
#[cfg(feature = "kv_serde")]
fn main() {
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
}
#[cfg(not(feature = "kv_serde"))]
fn main() {}
```

See the `kv` module documentation for more details.

# Available logging implementations

In order to produce log output executables have to use
a logger implementation compatible with the facade.
There are many available implementations to choose from,
here are some of the most popular ones:

* Simple minimal loggers:
    * [env_logger]
    * [colog]
    * [simple_logger]
    * [simplelog]
    * [pretty_env_logger]
    * [stderrlog]
    * [flexi_logger]
    * [call_logger]
    * [std-logger]
    * [structured-logger]
    * [clang_log]
    * [ftail]
* Complex configurable frameworks:
    * [log4rs]
    * [logforth]
    * [fern]
    * [spdlog-rs]
* Adaptors for other facilities:
    * [`syslog`](../libc/index.md)
    * [slog-stdlog]
    * [systemd-journal-logger]
    * [android_log]
    * [win_dbg_logger]
    * [db_logger]
    * [log-to-defmt]
    * [logcontrol-log]
* For WebAssembly binaries:
    * [console_log]
* For dynamic libraries:
    * You may need to construct an FFI-safe wrapper over `log` to initialize in your libraries
* Utilities:
    * [log_err]
    * [log-reload]
    * [alterable_logger]

# Implementing a Logger

Loggers implement the [`Log`](#log) trait. Here's a very basic example that simply
logs all messages at the [`Error`][level_link], [`Warn`][level_link] or
[`Info`][level_link] levels to stdout:

```rust
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

fn main() {}
```

Loggers are installed by calling the [`set_logger`](#set-logger) function. The maximum
log level also needs to be adjusted via the [`set_max_level`](#set-max-level) function. The
logging facade uses this as an optimization to improve performance of log
messages at levels that are disabled. It's important to set it, as it
defaults to [`Off`][filter_link], so no log messages will ever be captured!
In the case of our example logger, we'll want to set the maximum log level
to [`Info`][filter_link], since we ignore any [`Debug`][level_link] or
[`Trace`][level_link] level log messages. A logging implementation should
provide a function that wraps a call to [`set_logger`](#set-logger) and
[`set_max_level`](#set-max-level), handling initialization of the logger:

```rust
use log::{Level, Metadata};
struct SimpleLogger;
impl log::Log for SimpleLogger {
  fn enabled(&self, _: &Metadata) -> bool { false }
  fn log(&self, _: &log::Record) {}
  fn flush(&self) {}
}
fn main() {}
use log::{SetLoggerError, LevelFilter};

static LOGGER: SimpleLogger = SimpleLogger;

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Info))
}
```

Implementations that adjust their configurations at runtime should take care
to adjust the maximum log level as well.

# Use with `std`

`set_logger` requires you to provide a `&'static Log`, which can be hard to
obtain if your logger depends on some runtime configuration. The
`set_boxed_logger` function is available with the `std` Cargo feature. It is
identical to `set_logger` except that it takes a `Box<Log>` rather than a
`&'static Log`:

```rust
use log::{Level, LevelFilter, Log, SetLoggerError, Metadata};
struct SimpleLogger;
impl log::Log for SimpleLogger {
  fn enabled(&self, _: &Metadata) -> bool { false }
  fn log(&self, _: &log::Record) {}
  fn flush(&self) {}
}
fn main() {}
#[cfg(feature = "std")]
pub fn init() -> Result<(), SetLoggerError> {
    log::set_boxed_logger(Box::new(SimpleLogger))
        .map(|()| log::set_max_level(LevelFilter::Info))
}
```

# Compile time filters

Log levels can be statically disabled at compile time by enabling one of these Cargo features:

* `max_level_off`
* `max_level_error`
* `max_level_warn`
* `max_level_info`
* `max_level_debug`
* `max_level_trace`

Log invocations at disabled levels will be skipped and will not even be present in the
resulting binary. These features control the value of the `STATIC_MAX_LEVEL` constant. The
logging macros check this value before logging a message. By default, no levels are disabled.

It is possible to override this level for release builds only with the following features:

* `release_max_level_off`
* `release_max_level_error`
* `release_max_level_warn`
* `release_max_level_info`
* `release_max_level_debug`
* `release_max_level_trace`

Libraries should avoid using the max level features because they're global and can't be changed
once they're set.

For example, a crate can disable trace level logs in debug builds and trace, debug, and info
level logs in release builds with the following configuration:

```toml
[dependencies]
log = { version = "0.4", features = ["max_level_debug", "release_max_level_warn"] }
```
# Crate Feature Flags

The following crate feature flags are available in addition to the filters. They are
configured in your `Cargo.toml`.

* `std` allows use of `std` crate instead of the default `core`. Enables using `std::error` and
  `set_boxed_logger` functionality.
* `serde` enables support for serialization and deserialization of `Level` and `LevelFilter`.

```toml
[dependencies]
log = { version = "0.4", features = ["std", "serde"] }
```

# Version compatibility

The 0.3 and 0.4 versions of the `log` crate are almost entirely compatible. Log messages
made using `log` 0.3 will forward transparently to a logger implementation using `log` 0.4. Log
messages made using `log` 0.4 will forward to a logger implementation using `log` 0.3, but the
module path and file name information associated with the message will unfortunately be lost.




































## Contents

- [Modules](#modules)
  - [`macros`](#macros)
- [Structs](#structs)
  - [`Record`](#record)
  - [`RecordBuilder`](#recordbuilder)
  - [`Metadata`](#metadata)
  - [`MetadataBuilder`](#metadatabuilder)
  - [`NopLogger`](#noplogger)
  - [`SetLoggerError`](#setloggererror)
  - [`ParseLevelError`](#parselevelerror)
- [Enums](#enums)
  - [`Level`](#level)
  - [`LevelFilter`](#levelfilter)
  - [`MaybeStaticStr`](#maybestaticstr)
- [Traits](#traits)
  - [`Log`](#log)
- [Functions](#functions)
  - [`set_max_level`](#set-max-level)
  - [`set_max_level_racy`](#set-max-level-racy)
  - [`max_level`](#max-level)
  - [`set_logger`](#set-logger)
  - [`set_logger_inner`](#set-logger-inner)
  - [`set_logger_racy`](#set-logger-racy)
  - [`logger`](#logger)
- [Constants](#constants)
  - [`UNINITIALIZED`](#uninitialized)
  - [`INITIALIZING`](#initializing)
  - [`INITIALIZED`](#initialized)
  - [`STATIC_MAX_LEVEL`](#static-max-level)
- [Macros](#macros)
  - [`log_enabled!`](#log-enabled)
  - [`log!`](#log)
  - [`error!`](#error)
  - [`warn!`](#warn)
  - [`info!`](#info)
  - [`debug!`](#debug)
  - [`trace!`](#trace)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`macros`](#macros) | mod |  |
| [`Record`](#record) | struct | The "payload" of a log message. |
| [`RecordBuilder`](#recordbuilder) | struct | Builder for [`Record`](struct.Record.html). |
| [`Metadata`](#metadata) | struct | Metadata about a log message. |
| [`MetadataBuilder`](#metadatabuilder) | struct | Builder for [`Metadata`](struct.Metadata.html). |
| [`NopLogger`](#noplogger) | struct | A dummy initial value for LOGGER. |
| [`SetLoggerError`](#setloggererror) | struct | The type returned by [`set_logger`] if [`set_logger`] has already been called. |
| [`ParseLevelError`](#parselevelerror) | struct | The type returned by [`from_str`] when the string doesn't match any of the log levels. |
| [`Level`](#level) | enum | An enum representing the available verbosity levels of the logger. |
| [`LevelFilter`](#levelfilter) | enum | An enum representing the available verbosity level filters of the logger. |
| [`MaybeStaticStr`](#maybestaticstr) | enum |  |
| [`Log`](#log) | trait | A trait encapsulating the operations required of a logger. |
| [`set_max_level`](#set-max-level) | fn | Sets the global maximum log level. |
| [`set_max_level_racy`](#set-max-level-racy) | fn | A thread-unsafe version of [`set_max_level`]. |
| [`max_level`](#max-level) | fn | Returns the current maximum log level. |
| [`set_logger`](#set-logger) | fn | Sets the global logger to a `&'static Log`. |
| [`set_logger_inner`](#set-logger-inner) | fn |  |
| [`set_logger_racy`](#set-logger-racy) | fn | A thread-unsafe version of [`set_logger`]. |
| [`logger`](#logger) | fn | Returns a reference to the logger. |
| [`UNINITIALIZED`](#uninitialized) | const |  |
| [`INITIALIZING`](#initializing) | const |  |
| [`INITIALIZED`](#initialized) | const |  |
| [`STATIC_MAX_LEVEL`](#static-max-level) | const | The statically resolved maximum log level. |
| [`log_enabled!`](#log-enabled) | macro | Determines if a message logged at the specified level in that module will be logged. |
| [`log!`](#log) | macro | The standard logging macro. |
| [`error!`](#error) | macro | Logs a message at the error level. |
| [`warn!`](#warn) | macro | Logs a message at the warn level. |
| [`info!`](#info) | macro | Logs a message at the info level. |
| [`debug!`](#debug) | macro | Logs a message at the debug level. |
| [`trace!`](#trace) | macro | Logs a message at the trace level. |

## Modules

- [`macros`](macros/index.md)

## Structs

### `Record<'a>`

```rust
struct Record<'a> {
    metadata: Metadata<'a>,
    args: fmt::Arguments<'a>,
    module_path: Option<MaybeStaticStr<'a>>,
    file: Option<MaybeStaticStr<'a>>,
    line: Option<u32>,
}
```

The "payload" of a log message.

# Use

`Record` structures are passed as parameters to the [`log`][method.log]
method of the [`Log`](#log) trait. Logger implementors manipulate these
structures in order to display log messages. `Record`s are automatically
created by the `log!` macro and so are not seen by log users.

Note that the `level()` and `target()` accessors are equivalent to
`self.metadata().level()` and `self.metadata().target()` respectively.
These methods are provided as a convenience for users of this structure.

# Example

The following example shows a simple logger that displays the level,
module path, and message of any `Record` that is passed to it.

```rust
struct SimpleLogger;

impl log::Log for SimpleLogger {
   fn enabled(&self, _metadata: &log::Metadata) -> bool {
       true
   }

   fn log(&self, record: &log::Record) {
       if !self.enabled(record.metadata()) {
           return;
       }

       println!("{}:{} -- {}",
                record.level(),
                record.target(),
                record.args());
   }
   fn flush(&self) {}
}
```






#### Implementations

- <span id="record-builder"></span>`fn builder() -> RecordBuilder<'a>` — [`RecordBuilder`](#recordbuilder)

  Returns a new builder.

- <span id="record-args"></span>`fn args(&self) -> &fmt::Arguments<'a>`

  The message body.

- <span id="record-metadata"></span>`fn metadata(&self) -> &Metadata<'a>` — [`Metadata`](#metadata)

  Metadata about the log directive.

- <span id="record-level"></span>`fn level(&self) -> Level` — [`Level`](#level)

  The verbosity level of the message.

- <span id="record-target"></span>`fn target(&self) -> &'a str`

  The name of the target of the directive.

- <span id="record-module-path"></span>`fn module_path(&self) -> Option<&'a str>`

  The module path of the message.

- <span id="record-module-path-static"></span>`fn module_path_static(&self) -> Option<&'static str>`

  The module path of the message, if it is a `'static` string.

- <span id="record-file"></span>`fn file(&self) -> Option<&'a str>`

  The source file containing the message.

- <span id="record-file-static"></span>`fn file_static(&self) -> Option<&'static str>`

  The source file containing the message, if it is a `'static` string.

- <span id="record-line"></span>`fn line(&self) -> Option<u32>`

  The line containing the message.

#### Trait Implementations

##### `impl Clone for Record<'a>`

- <span id="record-clone"></span>`fn clone(&self) -> Record<'a>` — [`Record`](#record)

##### `impl Debug for Record<'a>`

- <span id="record-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `RecordBuilder<'a>`

```rust
struct RecordBuilder<'a> {
    record: Record<'a>,
}
```

Builder for [`Record`](#record).

Typically should only be used by log library creators or for testing and "shim loggers".
The `RecordBuilder` can set the different parameters of `Record` object, and returns
the created object when `build` is called.

# Examples

```rust
use log::{Level, Record};

let record = Record::builder()
                .args(format_args!("Error!"))
                .level(Level::Error)
                .target("myApp")
                .file(Some("server.rs"))
                .line(Some(144))
                .module_path(Some("server"))
                .build();
```

Alternatively, use [`MetadataBuilder`](#metadatabuilder):

```rust
use log::{Record, Level, MetadataBuilder};

let error_metadata = MetadataBuilder::new()
                        .target("myApp")
                        .level(Level::Error)
                        .build();

let record = Record::builder()
                .metadata(error_metadata)
                .args(format_args!("Error!"))
                .line(Some(433))
                .file(Some("app.rs"))
                .module_path(Some("server"))
                .build();
```

#### Implementations

- <span id="recordbuilder-new"></span>`fn new() -> RecordBuilder<'a>` — [`RecordBuilder`](#recordbuilder)

  Construct new `RecordBuilder`.

  

  The default options are:

  

  - `args`: `format_args!("")`

  - `metadata`: `Metadata::builder().build()`

  - `module_path`: `None`

  - `file`: `None`

  - `line`: `None`

  

- <span id="recordbuilder-args"></span>`fn args(&mut self, args: fmt::Arguments<'a>) -> &mut RecordBuilder<'a>` — [`RecordBuilder`](#recordbuilder)

  Set [`args`](#record-args).

- <span id="recordbuilder-metadata"></span>`fn metadata(&mut self, metadata: Metadata<'a>) -> &mut RecordBuilder<'a>` — [`Metadata`](#metadata), [`RecordBuilder`](#recordbuilder)

  Set [`metadata`](#record-metadata). Construct a `Metadata` object with [`MetadataBuilder`](#metadatabuilder).

- <span id="recordbuilder-level"></span>`fn level(&mut self, level: Level) -> &mut RecordBuilder<'a>` — [`Level`](#level), [`RecordBuilder`](#recordbuilder)

  Set [`Metadata::level`](#metadata-level).

- <span id="recordbuilder-target"></span>`fn target(&mut self, target: &'a str) -> &mut RecordBuilder<'a>` — [`RecordBuilder`](#recordbuilder)

  Set [`Metadata::target`](#metadata-target)

- <span id="recordbuilder-module-path"></span>`fn module_path(&mut self, path: Option<&'a str>) -> &mut RecordBuilder<'a>` — [`RecordBuilder`](#recordbuilder)

  Set [`module_path`](#record-module-path)

- <span id="recordbuilder-module-path-static"></span>`fn module_path_static(&mut self, path: Option<&'static str>) -> &mut RecordBuilder<'a>` — [`RecordBuilder`](#recordbuilder)

  Set [`module_path`](#record-module-path) to a `'static` string

- <span id="recordbuilder-file"></span>`fn file(&mut self, file: Option<&'a str>) -> &mut RecordBuilder<'a>` — [`RecordBuilder`](#recordbuilder)

  Set [`file`](#record-file)

- <span id="recordbuilder-file-static"></span>`fn file_static(&mut self, file: Option<&'static str>) -> &mut RecordBuilder<'a>` — [`RecordBuilder`](#recordbuilder)

  Set [`file`](#record-file) to a `'static` string.

- <span id="recordbuilder-line"></span>`fn line(&mut self, line: Option<u32>) -> &mut RecordBuilder<'a>` — [`RecordBuilder`](#recordbuilder)

  Set [`line`](#record-line)

- <span id="recordbuilder-build"></span>`fn build(&self) -> Record<'a>` — [`Record`](#record)

  Invoke the builder and return a `Record`

#### Trait Implementations

##### `impl Debug for RecordBuilder<'a>`

- <span id="recordbuilder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for RecordBuilder<'_>`

- <span id="recordbuilder-default"></span>`fn default() -> Self`

### `Metadata<'a>`

```rust
struct Metadata<'a> {
    level: Level,
    target: &'a str,
}
```

Metadata about a log message.

# Use

`Metadata` structs are created when users of the library use
logging macros.

They are consumed by implementations of the `Log` trait in the
`enabled` method.

`Record`s use `Metadata` to determine the log message's severity
and target.

Users should use the `log_enabled!` macro in their code to avoid
constructing expensive log messages.

# Examples

```rust
use log::{Record, Level, Metadata};

struct MyLogger;

impl log::Log for MyLogger {
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

fn main(){}
```

#### Implementations

- <span id="metadata-builder"></span>`fn builder() -> MetadataBuilder<'a>` — [`MetadataBuilder`](#metadatabuilder)

  Returns a new builder.

- <span id="metadata-level"></span>`fn level(&self) -> Level` — [`Level`](#level)

  The verbosity level of the message.

- <span id="metadata-target"></span>`fn target(&self) -> &'a str`

  The name of the target of the directive.

#### Trait Implementations

##### `impl Clone for Metadata<'a>`

- <span id="metadata-clone"></span>`fn clone(&self) -> Metadata<'a>` — [`Metadata`](#metadata)

##### `impl Debug for Metadata<'a>`

- <span id="metadata-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Metadata<'a>`

##### `impl Hash for Metadata<'a>`

- <span id="metadata-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Metadata<'a>`

- <span id="metadata-ord-cmp"></span>`fn cmp(&self, other: &Metadata<'a>) -> cmp::Ordering` — [`Metadata`](#metadata)

##### `impl PartialEq for Metadata<'a>`

- <span id="metadata-partialeq-eq"></span>`fn eq(&self, other: &Metadata<'a>) -> bool` — [`Metadata`](#metadata)

##### `impl PartialOrd for Metadata<'a>`

- <span id="metadata-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Metadata<'a>) -> option::Option<cmp::Ordering>` — [`Metadata`](#metadata)

##### `impl StructuralPartialEq for Metadata<'a>`

### `MetadataBuilder<'a>`

```rust
struct MetadataBuilder<'a> {
    metadata: Metadata<'a>,
}
```

Builder for [`Metadata`](#metadata).

Typically should only be used by log library creators or for testing and "shim loggers".
The `MetadataBuilder` can set the different parameters of a `Metadata` object, and returns
the created object when `build` is called.

# Example

```rust
let target = "myApp";
use log::{Level, MetadataBuilder};
let metadata = MetadataBuilder::new()
                    .level(Level::Debug)
                    .target(target)
                    .build();
```

#### Implementations

- <span id="metadatabuilder-new"></span>`fn new() -> MetadataBuilder<'a>` — [`MetadataBuilder`](#metadatabuilder)

  Construct a new `MetadataBuilder`.

  

  The default options are:

  

  - `level`: `Level::Info`

  - `target`: `""`

- <span id="metadatabuilder-level"></span>`fn level(&mut self, arg: Level) -> &mut MetadataBuilder<'a>` — [`Level`](#level), [`MetadataBuilder`](#metadatabuilder)

  Setter for [`level`](#metadata-level).

- <span id="metadatabuilder-target"></span>`fn target(&mut self, target: &'a str) -> &mut MetadataBuilder<'a>` — [`MetadataBuilder`](#metadatabuilder)

  Setter for [`target`](#metadata-target).

- <span id="metadatabuilder-build"></span>`fn build(&self) -> Metadata<'a>` — [`Metadata`](#metadata)

  Returns a `Metadata` object.

#### Trait Implementations

##### `impl Debug for MetadataBuilder<'a>`

- <span id="metadatabuilder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for MetadataBuilder<'_>`

- <span id="metadatabuilder-default"></span>`fn default() -> Self`

##### `impl Eq for MetadataBuilder<'a>`

##### `impl Hash for MetadataBuilder<'a>`

- <span id="metadatabuilder-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for MetadataBuilder<'a>`

- <span id="metadatabuilder-ord-cmp"></span>`fn cmp(&self, other: &MetadataBuilder<'a>) -> cmp::Ordering` — [`MetadataBuilder`](#metadatabuilder)

##### `impl PartialEq for MetadataBuilder<'a>`

- <span id="metadatabuilder-partialeq-eq"></span>`fn eq(&self, other: &MetadataBuilder<'a>) -> bool` — [`MetadataBuilder`](#metadatabuilder)

##### `impl PartialOrd for MetadataBuilder<'a>`

- <span id="metadatabuilder-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &MetadataBuilder<'a>) -> option::Option<cmp::Ordering>` — [`MetadataBuilder`](#metadatabuilder)

##### `impl StructuralPartialEq for MetadataBuilder<'a>`

### `NopLogger`

```rust
struct NopLogger;
```

A dummy initial value for LOGGER.

#### Trait Implementations

##### `impl Log for NopLogger`

- <span id="noplogger-log-enabled"></span>`fn enabled(&self, _: &Metadata<'_>) -> bool` — [`Metadata`](#metadata)

- <span id="noplogger-log"></span>`fn log(&self, _: &Record<'_>)` — [`Record`](#record)

- <span id="noplogger-log-flush"></span>`fn flush(&self)`

### `SetLoggerError`

```rust
struct SetLoggerError(());
```

The type returned by [`set_logger`](#set-logger) if [`set_logger`](#set-logger) has already been called.


#### Trait Implementations

##### `impl Debug for SetLoggerError`

- <span id="setloggererror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for SetLoggerError`

- <span id="setloggererror-display-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ParseLevelError`

```rust
struct ParseLevelError(());
```

The type returned by [`from_str`](#from-str) when the string doesn't match any of the log levels.


#### Trait Implementations

##### `impl Debug for ParseLevelError`

- <span id="parselevelerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ParseLevelError`

- <span id="parselevelerror-display-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ParseLevelError`

##### `impl PartialEq for ParseLevelError`

- <span id="parselevelerror-partialeq-eq"></span>`fn eq(&self, other: &ParseLevelError) -> bool` — [`ParseLevelError`](#parselevelerror)

##### `impl StructuralPartialEq for ParseLevelError`

## Enums

### `Level`

```rust
enum Level {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}
```

An enum representing the available verbosity levels of the logger.

Typical usage includes: checking if a certain `Level` is enabled with
[`log_enabled!`](#log_enabled), specifying the `Level` of
[`log!`](#log), and comparing a `Level` directly to a
[`LevelFilter`](#levelfilter).

#### Variants

- **`Error`**

  The "error" level.
  
  Designates very serious errors.

- **`Warn`**

  The "warn" level.
  
  Designates hazardous situations.

- **`Info`**

  The "info" level.
  
  Designates useful information.

- **`Debug`**

  The "debug" level.
  
  Designates lower priority information.

- **`Trace`**

  The "trace" level.
  
  Designates very low priority, often extremely verbose, information.

#### Implementations

- <span id="level-from-usize"></span>`fn from_usize(u: usize) -> Option<Level>` — [`Level`](#level)

- <span id="level-max"></span>`fn max() -> Level` — [`Level`](#level)

  Returns the most verbose logging level.

- <span id="level-to-level-filter"></span>`fn to_level_filter(&self) -> LevelFilter` — [`LevelFilter`](#levelfilter)

  Converts the `Level` to the equivalent `LevelFilter`.

- <span id="level-as-str"></span>`fn as_str(&self) -> &'static str`

  Returns the string representation of the `Level`.

  

  This returns the same string as the `fmt::Display` implementation.

- <span id="level-iter"></span>`fn iter() -> impl Iterator<Item = Self>`

  Iterate through all supported logging levels.

  

  The order of iteration is from more severe to less severe log messages.

  

  # Examples

  

  ```rust

  use log::Level;

  

  let mut levels = Level::iter();

  

  assert_eq!(Some(Level::Error), levels.next());

  assert_eq!(Some(Level::Trace), levels.last());

  ```

- <span id="level-increment-severity"></span>`fn increment_severity(&self) -> Self`

  Get the next-highest `Level` from this one.

  

  If the current `Level` is at the highest level, the returned `Level` will be the same as the

  current one.

  

  # Examples

  

  ```rust

  use log::Level;

  

  let level = Level::Info;

  

  assert_eq!(Level::Debug, level.increment_severity());

  assert_eq!(Level::Trace, level.increment_severity().increment_severity());

  assert_eq!(Level::Trace, level.increment_severity().increment_severity().increment_severity()); // max level

  ```

- <span id="level-decrement-severity"></span>`fn decrement_severity(&self) -> Self`

  Get the next-lowest `Level` from this one.

  

  If the current `Level` is at the lowest level, the returned `Level` will be the same as the

  current one.

  

  # Examples

  

  ```rust

  use log::Level;

  

  let level = Level::Info;

  

  assert_eq!(Level::Warn, level.decrement_severity());

  assert_eq!(Level::Error, level.decrement_severity().decrement_severity());

  assert_eq!(Level::Error, level.decrement_severity().decrement_severity().decrement_severity()); // min level

  ```

#### Trait Implementations

##### `impl Clone for Level`

- <span id="level-clone"></span>`fn clone(&self) -> Level` — [`Level`](#level)

##### `impl Copy for Level`

##### `impl Debug for Level`

- <span id="level-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Level`

- <span id="level-display-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Level`

##### `impl FromStr for Level`

- <span id="level-fromstr-type-err"></span>`type Err = ParseLevelError`

- <span id="level-fromstr-from-str"></span>`fn from_str(level: &str) -> Result<Level, <Self as >::Err>` — [`Level`](#level)

##### `impl Hash for Level`

- <span id="level-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Level`

- <span id="level-ord-cmp"></span>`fn cmp(&self, other: &Level) -> cmp::Ordering` — [`Level`](#level)

##### `impl PartialEq for Level`

- <span id="level-partialeq-eq"></span>`fn eq(&self, other: &Level) -> bool` — [`Level`](#level)

##### `impl PartialOrd for Level`

- <span id="level-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Level) -> option::Option<cmp::Ordering>` — [`Level`](#level)

##### `impl StructuralPartialEq for Level`

### `LevelFilter`

```rust
enum LevelFilter {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}
```

An enum representing the available verbosity level filters of the logger.

A `LevelFilter` may be compared directly to a [`Level`](#level). Use this type
to get and set the maximum log level with `max_level()` and [`set_max_level`](#set-max-level).




#### Variants

- **`Off`**

  A level lower than all log levels.

- **`Error`**

  Corresponds to the `Error` log level.

- **`Warn`**

  Corresponds to the `Warn` log level.

- **`Info`**

  Corresponds to the `Info` log level.

- **`Debug`**

  Corresponds to the `Debug` log level.

- **`Trace`**

  Corresponds to the `Trace` log level.

#### Implementations

- <span id="levelfilter-from-usize"></span>`fn from_usize(u: usize) -> Option<LevelFilter>` — [`LevelFilter`](#levelfilter)

- <span id="levelfilter-max"></span>`fn max() -> LevelFilter` — [`LevelFilter`](#levelfilter)

  Returns the most verbose logging level filter.

- <span id="levelfilter-to-level"></span>`fn to_level(&self) -> Option<Level>` — [`Level`](#level)

  Converts `self` to the equivalent `Level`.

  

  Returns `None` if `self` is `LevelFilter::Off`.

- <span id="levelfilter-as-str"></span>`fn as_str(&self) -> &'static str`

  Returns the string representation of the `LevelFilter`.

  

  This returns the same string as the `fmt::Display` implementation.

- <span id="levelfilter-iter"></span>`fn iter() -> impl Iterator<Item = Self>`

  Iterate through all supported filtering levels.

  

  The order of iteration is from less to more verbose filtering.

  

  # Examples

  

  ```rust

  use log::LevelFilter;

  

  let mut levels = LevelFilter::iter();

  

  assert_eq!(Some(LevelFilter::Off), levels.next());

  assert_eq!(Some(LevelFilter::Trace), levels.last());

  ```

- <span id="levelfilter-increment-severity"></span>`fn increment_severity(&self) -> Self`

  Get the next-highest `LevelFilter` from this one.

  

  If the current `LevelFilter` is at the highest level, the returned `LevelFilter` will be the

  same as the current one.

  

  # Examples

  

  ```rust

  use log::LevelFilter;

  

  let level_filter = LevelFilter::Info;

  

  assert_eq!(LevelFilter::Debug, level_filter.increment_severity());

  assert_eq!(LevelFilter::Trace, level_filter.increment_severity().increment_severity());

  assert_eq!(LevelFilter::Trace, level_filter.increment_severity().increment_severity().increment_severity()); // max level

  ```

- <span id="levelfilter-decrement-severity"></span>`fn decrement_severity(&self) -> Self`

  Get the next-lowest `LevelFilter` from this one.

  

  If the current `LevelFilter` is at the lowest level, the returned `LevelFilter` will be the

  same as the current one.

  

  # Examples

  

  ```rust

  use log::LevelFilter;

  

  let level_filter = LevelFilter::Info;

  

  assert_eq!(LevelFilter::Warn, level_filter.decrement_severity());

  assert_eq!(LevelFilter::Error, level_filter.decrement_severity().decrement_severity());

  assert_eq!(LevelFilter::Off, level_filter.decrement_severity().decrement_severity().decrement_severity());

  assert_eq!(LevelFilter::Off, level_filter.decrement_severity().decrement_severity().decrement_severity().decrement_severity()); // min level

  ```

#### Trait Implementations

##### `impl Clone for LevelFilter`

- <span id="levelfilter-clone"></span>`fn clone(&self) -> LevelFilter` — [`LevelFilter`](#levelfilter)

##### `impl Copy for LevelFilter`

##### `impl Debug for LevelFilter`

- <span id="levelfilter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for LevelFilter`

- <span id="levelfilter-display-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for LevelFilter`

##### `impl FromStr for LevelFilter`

- <span id="levelfilter-fromstr-type-err"></span>`type Err = ParseLevelError`

- <span id="levelfilter-fromstr-from-str"></span>`fn from_str(level: &str) -> Result<LevelFilter, <Self as >::Err>` — [`LevelFilter`](#levelfilter)

##### `impl Hash for LevelFilter`

- <span id="levelfilter-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for LevelFilter`

- <span id="levelfilter-ord-cmp"></span>`fn cmp(&self, other: &LevelFilter) -> cmp::Ordering` — [`LevelFilter`](#levelfilter)

##### `impl PartialEq for Level`

- <span id="level-partialeq-eq"></span>`fn eq(&self, other: &LevelFilter) -> bool` — [`LevelFilter`](#levelfilter)

##### `impl PartialOrd for Level`

- <span id="level-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &LevelFilter) -> Option<cmp::Ordering>` — [`LevelFilter`](#levelfilter)

##### `impl StructuralPartialEq for LevelFilter`

### `MaybeStaticStr<'a>`

```rust
enum MaybeStaticStr<'a> {
    Static(&'static str),
    Borrowed(&'a str),
}
```

#### Implementations

- <span id="maybestaticstr-get"></span>`fn get(&self) -> &'a str`

#### Trait Implementations

##### `impl Clone for MaybeStaticStr<'a>`

- <span id="maybestaticstr-clone"></span>`fn clone(&self) -> MaybeStaticStr<'a>` — [`MaybeStaticStr`](#maybestaticstr)

##### `impl Copy for MaybeStaticStr<'a>`

##### `impl Debug for MaybeStaticStr<'a>`

- <span id="maybestaticstr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for MaybeStaticStr<'a>`

##### `impl Hash for MaybeStaticStr<'a>`

- <span id="maybestaticstr-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for MaybeStaticStr<'a>`

- <span id="maybestaticstr-ord-cmp"></span>`fn cmp(&self, other: &MaybeStaticStr<'a>) -> cmp::Ordering` — [`MaybeStaticStr`](#maybestaticstr)

##### `impl PartialEq for MaybeStaticStr<'a>`

- <span id="maybestaticstr-partialeq-eq"></span>`fn eq(&self, other: &MaybeStaticStr<'a>) -> bool` — [`MaybeStaticStr`](#maybestaticstr)

##### `impl PartialOrd for MaybeStaticStr<'a>`

- <span id="maybestaticstr-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &MaybeStaticStr<'a>) -> option::Option<cmp::Ordering>` — [`MaybeStaticStr`](#maybestaticstr)

##### `impl StructuralPartialEq for MaybeStaticStr<'a>`

## Traits

### `Log`

```rust
trait Log: Sync + Send { ... }
```

A trait encapsulating the operations required of a logger.

#### Required Methods

- `fn enabled(&self, metadata: &Metadata<'_>) -> bool`

  Determines if a log message with the specified metadata would be

- `fn log(&self, record: &Record<'_>)`

  Logs the `Record`.

- `fn flush(&self)`

  Flushes any buffered records.

#### Implementors

- [`NopLogger`](#noplogger)
- `&T`

## Functions

### `set_max_level`

```rust
fn set_max_level(level: LevelFilter)
```

Sets the global maximum log level.

Generally, this should only be called by the active logging implementation.

Note that `Trace` is the maximum level, because it provides the maximum amount of detail in the emitted logs.

### `set_max_level_racy`

```rust
unsafe fn set_max_level_racy(level: LevelFilter)
```

A thread-unsafe version of [`set_max_level`](#set-max-level).

This function is available on all platforms, even those that do not have
support for atomics that is needed by [`set_max_level`](#set-max-level).

In almost all cases, [`set_max_level`](#set-max-level) should be preferred.

# Safety

This function is only safe to call when it cannot race with any other
calls to `set_max_level` or `set_max_level_racy`.

This can be upheld by (for example) making sure that **there are no other
threads**, and (on embedded) that **interrupts are disabled**.

It is safe to use all other logging functions while this function runs
(including all logging macros).


### `max_level`

```rust
fn max_level() -> LevelFilter
```

Returns the current maximum log level.

The `log!`, `error!`, `warn!`, `info!`, `debug!`, and `trace!` macros check
this value and discard any message logged at a higher level. The maximum
log level is set by the [`set_max_level`](#set-max-level) function.








### `set_logger`

```rust
fn set_logger(logger: &'static dyn Log) -> Result<(), SetLoggerError>
```

Sets the global logger to a `&'static Log`.

This function may only be called once in the lifetime of a program. Any log
events that occur before the call to `set_logger` completes will be ignored.

This function does not typically need to be called manually. Logger
implementations should provide an initialization method that installs the
logger internally.

# Availability

This method is available even when the `std` feature is disabled. However,
it is currently unavailable on `thumbv6` targets, which lack support for
some atomic operations which are used by this function. Even on those
targets, [`set_logger_racy`](#set-logger-racy) will be available.

# Errors

An error is returned if a logger has already been set.

# Examples

```rust
use log::{error, info, warn, Record, Level, Metadata, LevelFilter};

static MY_LOGGER: MyLogger = MyLogger;

struct MyLogger;

impl log::Log for MyLogger {
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

fn main(){
log::set_logger(&MY_LOGGER).unwrap();
log::set_max_level(LevelFilter::Info);

info!("hello log");
warn!("warning");
error!("oops");
}
```


### `set_logger_inner`

```rust
fn set_logger_inner<F>(make_logger: F) -> Result<(), SetLoggerError>
where
    F: FnOnce() -> &'static dyn Log
```

### `set_logger_racy`

```rust
unsafe fn set_logger_racy(logger: &'static dyn Log) -> Result<(), SetLoggerError>
```

A thread-unsafe version of [`set_logger`](#set-logger).

This function is available on all platforms, even those that do not have
support for atomics that is needed by [`set_logger`](#set-logger).

In almost all cases, [`set_logger`](#set-logger) should be preferred.

# Safety

This function is only safe to call when it cannot race with any other
calls to `set_logger` or `set_logger_racy`.

This can be upheld by (for example) making sure that **there are no other
threads**, and (on embedded) that **interrupts are disabled**.

It is safe to use other logging functions while this function runs
(including all logging macros).


### `logger`

```rust
fn logger() -> &'static dyn Log
```

Returns a reference to the logger.

If a logger has not been set, a no-op implementation is returned.

## Constants

### `UNINITIALIZED`
```rust
const UNINITIALIZED: usize = 0usize;
```

### `INITIALIZING`
```rust
const INITIALIZING: usize = 1usize;
```

### `INITIALIZED`
```rust
const INITIALIZED: usize = 2usize;
```

### `STATIC_MAX_LEVEL`
```rust
const STATIC_MAX_LEVEL: LevelFilter;
```

The statically resolved maximum log level.

See the crate level documentation for information on how to configure this.

This value is checked by the log macros, but not by the `Log`ger returned by
the [`logger`](#logger) function. Code that manually calls functions on that value
should compare the level against this value.


## Macros

### `log_enabled!`

Determines if a message logged at the specified level in that module will
be logged.

This can be used to avoid expensive computation of log message arguments if
the message would be ignored anyway.

# Examples

```rust
use log::{debug, log_enabled, Level};

struct Data { x: u32, y: u32 }
fn expensive_call() -> Data { Data { x: 0, y: 0 } }
let my_logger = log::__private_api::GlobalLogger;
if log_enabled!(Level::Debug) {
    let data = expensive_call();
    debug!("expensive debug data: {} {}", data.x, data.y);
}

if log_enabled!(target: "Global", Level::Debug) {
   let data = expensive_call();
   debug!(target: "Global", "expensive debug data: {} {}", data.x, data.y);
}

if log_enabled!(logger: my_logger, Level::Debug) {
   let data = expensive_call();
   debug!(target: "Global", "expensive debug data: {} {}", data.x, data.y);
}
```

This macro accepts the same `target` and `logger` arguments as [`macro@log`](#macrolog).

### `log!`

The standard logging macro.

This macro will generically log with the specified `Level` and `format!`
based argument list.

```rust
use log::{log, Level};

let data = (42, "Forty-two");
let private_data = "private";

log!(Level::Error, "Received errors: {}, {}", data.0, data.1);
```

Optionally, you can specify a `target` argument to attach a specific target
to the log record. By default, the target is the module path of the caller.

```rust
use log::{log, Level};

let data = (42, "Forty-two");
let private_data = "private";

log!(
    target: "app_events",
    Level::Error,
    "Received errors: {}, {}",
    data.0, data.1
);
```

And optionally, you can specify a `logger` argument to use a specific logger
instead of the default global logger.

```rust
struct MyLogger {}
impl Log for MyLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        false
    }
    fn log(&self, _record: &log::Record) {}
    fn flush(&self) {}
}
use log::{log, Level, Log};

let data = (42, "Forty-two");
let private_data = "private";

let my_logger = MyLogger {};
log!(
    logger: my_logger,
    Level::Error,
    "Received errors: {}, {}",
    data.0, data.1
);
```

The `logger` argument accepts a value that implements the `Log` trait. The value
will be borrowed within the macro.

Note that the global level set via Cargo features, or through `set_max_level` will
still apply, even when a custom logger is supplied with the `logger` argument.

### `error!`

Logs a message at the error level.

# Examples

```rust
use log::error;

let my_logger = log::__private_api::GlobalLogger;
let (err_info, port) = ("No connection", 22);

error!("Error: {err_info} on port {port}");
error!(target: "app_events", "App Error: {err_info}, Port: {port}");
error!(logger: my_logger, "App Error: {err_info}, Port: {port}");
```

### `warn!`

Logs a message at the warn level.

# Examples

```rust
use log::warn;

let my_logger = log::__private_api::GlobalLogger;
let warn_description = "Invalid Input";

warn!("Warning! {warn_description}!");
warn!(target: "input_events", "App received warning: {warn_description}");
warn!(logger: my_logger, "App received warning: {warn_description}");
```

### `info!`

Logs a message at the info level.

# Examples

```rust
use log::info;

let my_logger = log::__private_api::GlobalLogger;
struct Connection { port: u32, speed: f32 }
let conn_info = Connection { port: 40, speed: 3.20 };

info!("Connected to port {} at {} Mb/s", conn_info.port, conn_info.speed);
info!(
    target: "connection_events",
    "Successful connection, port: {}, speed: {}",
    conn_info.port, conn_info.speed
);
info!(
    logger: my_logger,
    "Successful connection, port: {}, speed: {}",
    conn_info.port, conn_info.speed
);
```

### `debug!`

Logs a message at the debug level.

# Examples

```rust
use log::debug;

let my_logger = log::__private_api::GlobalLogger;
struct Position { x: f32, y: f32 }
let pos = Position { x: 3.234, y: -1.223 };

debug!("New position: x: {}, y: {}", pos.x, pos.y);
debug!(target: "app_events", "New position: x: {}, y: {}", pos.x, pos.y);
debug!(logger: my_logger, "New position: x: {}, y: {}", pos.x, pos.y);
```

### `trace!`

Logs a message at the trace level.

# Examples

```rust
use log::trace;

let my_logger = log::__private_api::GlobalLogger;
struct Position { x: f32, y: f32 }
let pos = Position { x: 3.234, y: -1.223 };

trace!("Position is: x: {}, y: {}", pos.x, pos.y);
trace!(target: "app_events", "x is {} and y is {}",
       if pos.x >= 0.0 { "positive" } else { "negative" },
       if pos.y >= 0.0 { "positive" } else { "negative" });
trace!(logger: my_logger, "x is {} and y is {}",
       if pos.x >= 0.0 { "positive" } else { "negative" },
       if pos.y >= 0.0 { "positive" } else { "negative" });
```

