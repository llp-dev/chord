# Crate `sel4_logging`

## Contents

- [Modules](#modules)
  - [`synchronized`](#synchronized)
- [Structs](#structs)
  - [`SynchronizedLogger`](#synchronizedlogger)
  - [`Logger`](#logger)
  - [`WriteWrapper`](#writewrapper)
  - [`DisplayWrapper`](#displaywrapper)
  - [`LoggerBuilder`](#loggerbuilder)
- [Functions](#functions)
  - [`fmt_with_module`](#fmt-with-module)
  - [`fmt_with_line`](#fmt-with-line)
- [Type Aliases](#type-aliases)
  - [`FmtRecordFn`](#fmtrecordfn)
- [Constants](#constants)
  - [`FMT_RECORD_DEFAULT`](#fmt-record-default)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`synchronized`](#synchronized) | mod |  |
| [`SynchronizedLogger`](#synchronizedlogger) | struct |  |
| [`Logger`](#logger) | struct |  |
| [`WriteWrapper`](#writewrapper) | struct |  |
| [`DisplayWrapper`](#displaywrapper) | struct |  |
| [`LoggerBuilder`](#loggerbuilder) | struct |  |
| [`fmt_with_module`](#fmt-with-module) | fn |  |
| [`fmt_with_line`](#fmt-with-line) | fn |  |
| [`FmtRecordFn`](#fmtrecordfn) | type |  |
| [`FMT_RECORD_DEFAULT`](#fmt-record-default) | const |  |

## Modules

- [`synchronized`](synchronized/index.md)

## Structs

### `SynchronizedLogger<R, T>`

```rust
struct SynchronizedLogger<R, T>(lock_api::Mutex<R, T>);
```

#### Implementations

- <span id="synchronizedlogger-new"></span>`const fn new(inner: T) -> Self`

#### Trait Implementations

##### `impl<R: RawMutex + Send + Sync, T: Log> Log for SynchronizedLogger<R, T>`

- <span id="synchronizedlogger-log-enabled"></span>`fn enabled(&self, metadata: &Metadata<'_>) -> bool`

- <span id="synchronizedlogger-log"></span>`fn log(&self, record: &Record<'_>)`

- <span id="synchronizedlogger-log-flush"></span>`fn flush(&self)`

### `Logger`

```rust
struct Logger {
    pub level_filter: LevelFilter,
    pub filter: fn(&log::Metadata<'_>) -> bool,
    pub fmt: FmtRecordFn,
    pub write: fn(&str),
    pub flush: fn(),
}
```

#### Implementations

- <span id="logger-const-default"></span>`const fn const_default() -> Self`

- <span id="logger-level-filter"></span>`fn level_filter(&self) -> LevelFilter` — [`LevelFilter`](#levelfilter)

- <span id="logger-set-max-level"></span>`fn set_max_level(&self)`

- <span id="logger-set"></span>`fn set(self: &'static Self) -> Result<(), SetLoggerError>`

#### Trait Implementations

##### `impl Log for Logger`

- <span id="logger-log-enabled"></span>`fn enabled(&self, metadata: &Metadata<'_>) -> bool`

- <span id="logger-log"></span>`fn log(&self, record: &Record<'_>)`

- <span id="logger-log-flush"></span>`fn flush(&self)`

### `WriteWrapper`

```rust
struct WriteWrapper(fn(&str));
```

#### Trait Implementations

##### `impl Write for WriteWrapper`

- <span id="writewrapper-write-write-str"></span>`fn write_str(&mut self, s: &str) -> fmt::Result`

### `DisplayWrapper<'a>`

```rust
struct DisplayWrapper<'a> {
    fmt: FmtRecordFn,
    record: &'a log::Record<'a>,
}
```

#### Trait Implementations

##### `impl Display for DisplayWrapper<'_>`

- <span id="displaywrapper-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `LoggerBuilder`

```rust
struct LoggerBuilder(Logger);
```

#### Implementations

- <span id="loggerbuilder-const-default"></span>`const fn const_default() -> Self`

- <span id="loggerbuilder-build"></span>`const fn build(self) -> Logger` — [`Logger`](#logger)

- <span id="loggerbuilder-level-filter"></span>`const fn level_filter(self, level_filter: LevelFilter) -> Self` — [`LevelFilter`](#levelfilter)

- <span id="loggerbuilder-filter"></span>`const fn filter(self, filter: fn(&Metadata<'_>) -> bool) -> Self`

- <span id="loggerbuilder-fmt"></span>`const fn fmt(self, fmt: FmtRecordFn) -> Self` — [`FmtRecordFn`](#fmtrecordfn)

- <span id="loggerbuilder-write"></span>`const fn write(self, write: fn(&str)) -> Self`

- <span id="loggerbuilder-flush"></span>`const fn flush(self, flush: fn()) -> Self`

#### Trait Implementations

##### `impl Default for LoggerBuilder`

- <span id="loggerbuilder-default"></span>`fn default() -> Self`

## Functions

### `fmt_with_module`

```rust
fn fmt_with_module(record: &log::Record<'_>, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

### `fmt_with_line`

```rust
fn fmt_with_line(record: &log::Record<'_>, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

## Type Aliases

### `FmtRecordFn`

```rust
type FmtRecordFn = fn(&log::Record<'_>, &mut fmt::Formatter<'_>) -> fmt::Result;
```

## Constants

### `FMT_RECORD_DEFAULT`
```rust
const FMT_RECORD_DEFAULT: FmtRecordFn = {fmt_with_module as for<'a, 'b, 'c, 'd> fn(&'a log::Record<'b>, &'c mut core::fmt::Formatter<'d>) -> core::result::Result<(), core::fmt::Error>};
```

