**log**

# Module: log

## Contents

**Macros**

- [`debug`](#debug) - Logs a message at the debug level.
- [`error`](#error) - Logs a message at the error level.
- [`info`](#info) - Logs a message at the info level.
- [`log`](#log) - The standard logging macro.
- [`log_enabled`](#log_enabled) - Determines if a message logged at the specified level in that module will
- [`trace`](#trace) - Logs a message at the trace level.
- [`warn`](#warn) - Logs a message at the warn level.

**Structs**

- [`Metadata`](#metadata) - Metadata about a log message.
- [`MetadataBuilder`](#metadatabuilder) - Builder for [`Metadata`](struct.Metadata.html).
- [`ParseLevelError`](#parselevelerror) - The type returned by [`from_str`] when the string doesn't match any of the log levels.
- [`Record`](#record) - The "payload" of a log message.
- [`RecordBuilder`](#recordbuilder) - Builder for [`Record`](struct.Record.html).
- [`SetLoggerError`](#setloggererror) - The type returned by [`set_logger`] if [`set_logger`] has already been called.

**Enums**

- [`Level`](#level) - An enum representing the available verbosity levels of the logger.
- [`LevelFilter`](#levelfilter) - An enum representing the available verbosity level filters of the logger.

**Functions**

- [`logger`](#logger) - Returns a reference to the logger.
- [`max_level`](#max_level) - Returns the current maximum log level.
- [`set_boxed_logger`](#set_boxed_logger) - Sets the global logger to a `Box<Log>`.
- [`set_logger`](#set_logger) - Sets the global logger to a `&'static Log`.
- [`set_logger_racy`](#set_logger_racy) - A thread-unsafe version of [`set_logger`].
- [`set_max_level`](#set_max_level) - Sets the global maximum log level.
- [`set_max_level_racy`](#set_max_level_racy) - A thread-unsafe version of [`set_max_level`].

**Traits**

- [`Log`](#log) - A trait encapsulating the operations required of a logger.

**Constants**

- [`STATIC_MAX_LEVEL`](#static_max_level) - The statically resolved maximum log level.

---

## log::Level

*Enum*

An enum representing the available verbosity levels of the logger.

Typical usage includes: checking if a certain `Level` is enabled with
[`log_enabled!`](macro.log_enabled.html), specifying the `Level` of
[`log!`](macro.log.html), and comparing a `Level` directly to a
[`LevelFilter`](enum.LevelFilter.html).

**Variants:**
- `Error` - The "error" level.
- `Warn` - The "warn" level.
- `Info` - The "info" level.
- `Debug` - The "debug" level.
- `Trace` - The "trace" level.

**Methods:**

- `fn max() -> Level` - Returns the most verbose logging level.
- `fn to_level_filter(self: &Self) -> LevelFilter` - Converts the `Level` to the equivalent `LevelFilter`.
- `fn as_str(self: &Self) -> &'static str` - Returns the string representation of the `Level`.
- `fn iter() -> impl Trait` - Iterate through all supported logging levels.
- `fn increment_severity(self: &Self) -> Self` - Get the next-highest `Level` from this one.
- `fn decrement_severity(self: &Self) -> Self` - Get the next-lowest `Level` from this one.

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &LevelFilter) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Level`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Ord**
  - `fn cmp(self: &Self, other: &Level) -> $crate::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Level) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Display**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`
- **FromStr**
  - `fn from_str(level: &str) -> Result<Level, <Self as >::Err>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Level) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &LevelFilter) -> Option<cmp::Ordering>`



## log::LevelFilter

*Enum*

An enum representing the available verbosity level filters of the logger.

A `LevelFilter` may be compared directly to a [`Level`]. Use this type
to get and set the maximum log level with [`max_level()`] and [`set_max_level`].

[`Level`]: enum.Level.html
[`max_level()`]: fn.max_level.html
[`set_max_level`]: fn.set_max_level.html

**Variants:**
- `Off` - A level lower than all log levels.
- `Error` - Corresponds to the `Error` log level.
- `Warn` - Corresponds to the `Warn` log level.
- `Info` - Corresponds to the `Info` log level.
- `Debug` - Corresponds to the `Debug` log level.
- `Trace` - Corresponds to the `Trace` log level.

**Methods:**

- `fn max() -> LevelFilter` - Returns the most verbose logging level filter.
- `fn to_level(self: &Self) -> Option<Level>` - Converts `self` to the equivalent `Level`.
- `fn as_str(self: &Self) -> &'static str` - Returns the string representation of the `LevelFilter`.
- `fn iter() -> impl Trait` - Iterate through all supported filtering levels.
- `fn increment_severity(self: &Self) -> Self` - Get the next-highest `LevelFilter` from this one.
- `fn decrement_severity(self: &Self) -> Self` - Get the next-lowest `LevelFilter` from this one.

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &LevelFilter) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> LevelFilter`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Level) -> Option<cmp::Ordering>`
- **Display**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Level) -> bool`
- **FromStr**
  - `fn from_str(level: &str) -> Result<LevelFilter, <Self as >::Err>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Ord**
  - `fn cmp(self: &Self, other: &LevelFilter) -> $crate::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &LevelFilter) -> $crate::option::Option<$crate::cmp::Ordering>`



## log::Log

*Trait*

A trait encapsulating the operations required of a logger.

**Methods:**

- `enabled`: Determines if a log message with the specified metadata would be
- `log`: Logs the `Record`.
- `flush`: Flushes any buffered records.



## log::Metadata

*Struct*

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

```
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

# fn main(){}
```

**Generic Parameters:**
- 'a

**Methods:**

- `fn builder() -> MetadataBuilder<'a>` - Returns a new builder.
- `fn level(self: &Self) -> Level` - The verbosity level of the message.
- `fn target(self: &Self) -> &'a str` - The name of the target of the directive.

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Metadata<'a>) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &Metadata<'a>) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &Metadata<'a>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Metadata<'a>`



## log::MetadataBuilder

*Struct*

Builder for [`Metadata`](struct.Metadata.html).

Typically should only be used by log library creators or for testing and "shim loggers".
The `MetadataBuilder` can set the different parameters of a `Metadata` object, and returns
the created object when `build` is called.

# Example

```
let target = "myApp";
use log::{Level, MetadataBuilder};
let metadata = MetadataBuilder::new()
                    .level(Level::Debug)
                    .target(target)
                    .build();
```

**Generic Parameters:**
- 'a

**Methods:**

- `fn new() -> MetadataBuilder<'a>` - Construct a new `MetadataBuilder`.
- `fn level(self: & mut Self, arg: Level) -> & mut MetadataBuilder<'a>` - Setter for [`level`](struct.Metadata.html#method.level).
- `fn target(self: & mut Self, target: &'a str) -> & mut MetadataBuilder<'a>` - Setter for [`target`](struct.Metadata.html#method.target).
- `fn build(self: &Self) -> Metadata<'a>` - Returns a `Metadata` object.

**Traits:** Eq

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &MetadataBuilder<'a>) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &MetadataBuilder<'a>) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &MetadataBuilder<'a>) -> bool`
- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## log::ParseLevelError

*Struct*

The type returned by [`from_str`] when the string doesn't match any of the log levels.

[`from_str`]: https://doc.rust-lang.org/std/str/trait.FromStr.html#tymethod.from_str

**Tuple Struct**: `()`

**Traits:** Eq, Error

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &ParseLevelError) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## log::Record

*Struct*

The "payload" of a log message.

# Use

`Record` structures are passed as parameters to the [`log`][method.log]
method of the [`Log`] trait. Logger implementors manipulate these
structures in order to display log messages. `Record`s are automatically
created by the [`log!`] macro and so are not seen by log users.

Note that the [`level()`] and [`target()`] accessors are equivalent to
`self.metadata().level()` and `self.metadata().target()` respectively.
These methods are provided as a convenience for users of this structure.

# Example

The following example shows a simple logger that displays the level,
module path, and message of any `Record` that is passed to it.

```
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

[method.log]: trait.Log.html#tymethod.log
[`Log`]: trait.Log.html
[`log!`]: macro.log.html
[`level()`]: struct.Record.html#method.level
[`target()`]: struct.Record.html#method.target

**Generic Parameters:**
- 'a

**Methods:**

- `fn builder() -> RecordBuilder<'a>` - Returns a new builder.
- `fn args(self: &Self) -> &fmt::Arguments<'a>` - The message body.
- `fn metadata(self: &Self) -> &Metadata<'a>` - Metadata about the log directive.
- `fn level(self: &Self) -> Level` - The verbosity level of the message.
- `fn target(self: &Self) -> &'a str` - The name of the target of the directive.
- `fn module_path(self: &Self) -> Option<&'a str>` - The module path of the message.
- `fn module_path_static(self: &Self) -> Option<&'static str>` - The module path of the message, if it is a `'static` string.
- `fn file(self: &Self) -> Option<&'a str>` - The source file containing the message.
- `fn file_static(self: &Self) -> Option<&'static str>` - The source file containing the message, if it is a `'static` string.
- `fn line(self: &Self) -> Option<u32>` - The line containing the message.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Record<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## log::RecordBuilder

*Struct*

Builder for [`Record`](struct.Record.html).

Typically should only be used by log library creators or for testing and "shim loggers".
The `RecordBuilder` can set the different parameters of `Record` object, and returns
the created object when `build` is called.

# Examples

```
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

Alternatively, use [`MetadataBuilder`](struct.MetadataBuilder.html):

```
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

**Generic Parameters:**
- 'a

**Methods:**

- `fn new() -> RecordBuilder<'a>` - Construct new `RecordBuilder`.
- `fn args(self: & mut Self, args: fmt::Arguments<'a>) -> & mut RecordBuilder<'a>` - Set [`args`](struct.Record.html#method.args).
- `fn metadata(self: & mut Self, metadata: Metadata<'a>) -> & mut RecordBuilder<'a>` - Set [`metadata`](struct.Record.html#method.metadata). Construct a `Metadata` object with [`MetadataBuilder`](struct.MetadataBuilder.html).
- `fn level(self: & mut Self, level: Level) -> & mut RecordBuilder<'a>` - Set [`Metadata::level`](struct.Metadata.html#method.level).
- `fn target(self: & mut Self, target: &'a str) -> & mut RecordBuilder<'a>` - Set [`Metadata::target`](struct.Metadata.html#method.target)
- `fn module_path(self: & mut Self, path: Option<&'a str>) -> & mut RecordBuilder<'a>` - Set [`module_path`](struct.Record.html#method.module_path)
- `fn module_path_static(self: & mut Self, path: Option<&'static str>) -> & mut RecordBuilder<'a>` - Set [`module_path`](struct.Record.html#method.module_path) to a `'static` string
- `fn file(self: & mut Self, file: Option<&'a str>) -> & mut RecordBuilder<'a>` - Set [`file`](struct.Record.html#method.file)
- `fn file_static(self: & mut Self, file: Option<&'static str>) -> & mut RecordBuilder<'a>` - Set [`file`](struct.Record.html#method.file) to a `'static` string.
- `fn line(self: & mut Self, line: Option<u32>) -> & mut RecordBuilder<'a>` - Set [`line`](struct.Record.html#method.line)
- `fn build(self: &Self) -> Record<'a>` - Invoke the builder and return a `Record`

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## log::STATIC_MAX_LEVEL

*Constant*: `LevelFilter`

The statically resolved maximum log level.

See the crate level documentation for information on how to configure this.

This value is checked by the log macros, but not by the `Log`ger returned by
the [`logger`] function. Code that manually calls functions on that value
should compare the level against this value.

[`logger`]: fn.logger.html



## log::SetLoggerError

*Struct*

The type returned by [`set_logger`] if [`set_logger`] has already been called.

[`set_logger`]: fn.set_logger.html

**Tuple Struct**: `()`

**Traits:** Error

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## log::debug

*Declarative Macro*

Logs a message at the debug level.

# Examples

```
use log::debug;

# let my_logger = log::__private_api::GlobalLogger;
# struct Position { x: f32, y: f32 }
let pos = Position { x: 3.234, y: -1.223 };

debug!("New position: x: {}, y: {}", pos.x, pos.y);
debug!(target: "app_events", "New position: x: {}, y: {}", pos.x, pos.y);
debug!(logger: my_logger, "New position: x: {}, y: {}", pos.x, pos.y);
```

```rust
macro_rules! debug {
    (logger: $logger:expr, target: $target:expr, $($arg:tt)+) => { ... };
    (logger: $logger:expr, $($arg:tt)+) => { ... };
    (target: $target:expr, $($arg:tt)+) => { ... };
    ($($arg:tt)+) => { ... };
}
```



## log::error

*Declarative Macro*

Logs a message at the error level.

# Examples

```
use log::error;

# let my_logger = log::__private_api::GlobalLogger;
let (err_info, port) = ("No connection", 22);

error!("Error: {err_info} on port {port}");
error!(target: "app_events", "App Error: {err_info}, Port: {port}");
error!(logger: my_logger, "App Error: {err_info}, Port: {port}");
```

```rust
macro_rules! error {
    (logger: $logger:expr, target: $target:expr, $($arg:tt)+) => { ... };
    (logger: $logger:expr, $($arg:tt)+) => { ... };
    (target: $target:expr, $($arg:tt)+) => { ... };
    ($($arg:tt)+) => { ... };
}
```



## log::info

*Declarative Macro*

Logs a message at the info level.

# Examples

```
use log::info;

# let my_logger = log::__private_api::GlobalLogger;
# struct Connection { port: u32, speed: f32 }
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

```rust
macro_rules! info {
    (logger: $logger:expr, target: $target:expr, $($arg:tt)+) => { ... };
    (logger: $logger:expr, $($arg:tt)+) => { ... };
    (target: $target:expr, $($arg:tt)+) => { ... };
    ($($arg:tt)+) => { ... };
}
```



## log::log

*Declarative Macro*

The standard logging macro.

This macro will generically log with the specified `Level` and `format!`
based argument list.

```
use log::{log, Level};

let data = (42, "Forty-two");
let private_data = "private";

log!(Level::Error, "Received errors: {}, {}", data.0, data.1);
```

Optionally, you can specify a `target` argument to attach a specific target
to the log record. By default, the target is the module path of the caller.

```
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

```
# struct MyLogger {}
# impl Log for MyLogger {
#     fn enabled(&self, _metadata: &log::Metadata) -> bool {
#         false
#     }
#     fn log(&self, _record: &log::Record) {}
#     fn flush(&self) {}
# }
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

```rust
macro_rules! log {
    (logger: $logger:expr, target: $target:expr, $lvl:expr, $($arg:tt)+) => { ... };
    (logger: $logger:expr, $lvl:expr, $($arg:tt)+) => { ... };
    (target: $target:expr, $lvl:expr, $($arg:tt)+) => { ... };
    ($lvl:expr, $($arg:tt)+) => { ... };
}
```



## log::log_enabled

*Declarative Macro*

Determines if a message logged at the specified level in that module will
be logged.

This can be used to avoid expensive computation of log message arguments if
the message would be ignored anyway.

# Examples

```
use log::{debug, log_enabled, Level};

# struct Data { x: u32, y: u32 }
# fn expensive_call() -> Data { Data { x: 0, y: 0 } }
# let my_logger = log::__private_api::GlobalLogger;
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

This macro accepts the same `target` and `logger` arguments as [`macro@log`].

```rust
macro_rules! log_enabled {
    (logger: $logger:expr, target: $target:expr, $lvl:expr) => { ... };
    (logger: $logger:expr, $lvl:expr) => { ... };
    (target: $target:expr, $lvl:expr) => { ... };
    ($lvl:expr) => { ... };
}
```



## log::logger

*Function*

Returns a reference to the logger.

If a logger has not been set, a no-op implementation is returned.

```rust
fn logger() -> &'static dyn Log
```



## log::max_level

*Function*

Returns the current maximum log level.

The [`log!`], [`error!`], [`warn!`], [`info!`], [`debug!`], and [`trace!`] macros check
this value and discard any message logged at a higher level. The maximum
log level is set by the [`set_max_level`] function.

[`log!`]: macro.log.html
[`error!`]: macro.error.html
[`warn!`]: macro.warn.html
[`info!`]: macro.info.html
[`debug!`]: macro.debug.html
[`trace!`]: macro.trace.html
[`set_max_level`]: fn.set_max_level.html

```rust
fn max_level() -> LevelFilter
```



## log::set_boxed_logger

*Function*

Sets the global logger to a `Box<Log>`.

This is a simple convenience wrapper over `set_logger`, which takes a
`Box<Log>` rather than a `&'static Log`. See the documentation for
[`set_logger`] for more details.

Requires the `std` feature.

# Errors

An error is returned if a logger has already been set.

[`set_logger`]: fn.set_logger.html

```rust
fn set_boxed_logger(logger: Box<dyn Log>) -> Result<(), SetLoggerError>
```



## log::set_logger

*Function*

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
targets, [`set_logger_racy`] will be available.

# Errors

An error is returned if a logger has already been set.

# Examples

```
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

# fn main(){
log::set_logger(&MY_LOGGER).unwrap();
log::set_max_level(LevelFilter::Info);

info!("hello log");
warn!("warning");
error!("oops");
# }
```

[`set_logger_racy`]: fn.set_logger_racy.html

```rust
fn set_logger(logger: &'static dyn Log) -> Result<(), SetLoggerError>
```



## log::set_logger_racy

*Function*

A thread-unsafe version of [`set_logger`].

This function is available on all platforms, even those that do not have
support for atomics that is needed by [`set_logger`].

In almost all cases, [`set_logger`] should be preferred.

# Safety

This function is only safe to call when it cannot race with any other
calls to `set_logger` or `set_logger_racy`.

This can be upheld by (for example) making sure that **there are no other
threads**, and (on embedded) that **interrupts are disabled**.

It is safe to use other logging functions while this function runs
(including all logging macros).

[`set_logger`]: fn.set_logger.html

```rust
fn set_logger_racy(logger: &'static dyn Log) -> Result<(), SetLoggerError>
```



## log::set_max_level

*Function*

Sets the global maximum log level.

Generally, this should only be called by the active logging implementation.

Note that `Trace` is the maximum level, because it provides the maximum amount of detail in the emitted logs.

```rust
fn set_max_level(level: LevelFilter)
```



## log::set_max_level_racy

*Function*

A thread-unsafe version of [`set_max_level`].

This function is available on all platforms, even those that do not have
support for atomics that is needed by [`set_max_level`].

In almost all cases, [`set_max_level`] should be preferred.

# Safety

This function is only safe to call when it cannot race with any other
calls to `set_max_level` or `set_max_level_racy`.

This can be upheld by (for example) making sure that **there are no other
threads**, and (on embedded) that **interrupts are disabled**.

It is safe to use all other logging functions while this function runs
(including all logging macros).

[`set_max_level`]: fn.set_max_level.html

```rust
fn set_max_level_racy(level: LevelFilter)
```



## log::trace

*Declarative Macro*

Logs a message at the trace level.

# Examples

```
use log::trace;

# let my_logger = log::__private_api::GlobalLogger;
# struct Position { x: f32, y: f32 }
let pos = Position { x: 3.234, y: -1.223 };

trace!("Position is: x: {}, y: {}", pos.x, pos.y);
trace!(target: "app_events", "x is {} and y is {}",
       if pos.x >= 0.0 { "positive" } else { "negative" },
       if pos.y >= 0.0 { "positive" } else { "negative" });
trace!(logger: my_logger, "x is {} and y is {}",
       if pos.x >= 0.0 { "positive" } else { "negative" },
       if pos.y >= 0.0 { "positive" } else { "negative" });
```

```rust
macro_rules! trace {
    (logger: $logger:expr, target: $target:expr, $($arg:tt)+) => { ... };
    (logger: $logger:expr, $($arg:tt)+) => { ... };
    (target: $target:expr, $($arg:tt)+) => { ... };
    ($($arg:tt)+) => { ... };
}
```



## log::warn

*Declarative Macro*

Logs a message at the warn level.

# Examples

```
use log::warn;

# let my_logger = log::__private_api::GlobalLogger;
let warn_description = "Invalid Input";

warn!("Warning! {warn_description}!");
warn!(target: "input_events", "App received warning: {warn_description}");
warn!(logger: my_logger, "App received warning: {warn_description}");
```

```rust
macro_rules! warn {
    (logger: $logger:expr, target: $target:expr, $($arg:tt)+) => { ... };
    (logger: $logger:expr, $($arg:tt)+) => { ... };
    (target: $target:expr, $($arg:tt)+) => { ... };
    ($($arg:tt)+) => { ... };
}
```



