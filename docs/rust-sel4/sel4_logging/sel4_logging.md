**sel4_logging**

# Module: sel4_logging

## Contents

**Structs**

- [`Logger`](#logger)
- [`LoggerBuilder`](#loggerbuilder)

**Functions**

- [`fmt_with_line`](#fmt_with_line)
- [`fmt_with_module`](#fmt_with_module)

**Constants**

- [`FMT_RECORD_DEFAULT`](#fmt_record_default)

**Type Aliases**

- [`FmtRecordFn`](#fmtrecordfn)

---

## sel4_logging::FMT_RECORD_DEFAULT

*Constant*: `FmtRecordFn`



## sel4_logging::FmtRecordFn

*Type Alias*: `fn(...)`



## sel4_logging::Logger

*Struct*

**Fields:**
- `level_filter: LevelFilter`
- `filter: fn(...)`
- `fmt: FmtRecordFn`
- `write: fn(...)`
- `flush: fn(...)`

**Methods:**

- `fn const_default() -> Self`
- `fn level_filter(self: &Self) -> LevelFilter`
- `fn set_max_level(self: &Self)`
- `fn set(self: &'static Self) -> Result<(), SetLoggerError>`

**Trait Implementations:**

- **Log**
  - `fn enabled(self: &Self, metadata: &Metadata) -> bool`
  - `fn log(self: &Self, record: &Record)`
  - `fn flush(self: &Self)`



## sel4_logging::LoggerBuilder

*Struct*

**Tuple Struct**: `()`

**Methods:**

- `fn const_default() -> Self`
- `fn build(self: Self) -> Logger`
- `fn level_filter(self: Self, level_filter: LevelFilter) -> Self`
- `fn filter(self: Self, filter: fn(...)) -> Self`
- `fn fmt(self: Self, fmt: FmtRecordFn) -> Self`
- `fn write(self: Self, write: fn(...)) -> Self`
- `fn flush(self: Self, flush: fn(...)) -> Self`

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`



## sel4_logging::fmt_with_line

*Function*

```rust
fn fmt_with_line(record: &log::Record, f: & mut fmt::Formatter) -> fmt::Result
```



## sel4_logging::fmt_with_module

*Function*

```rust
fn fmt_with_module(record: &log::Record, f: & mut fmt::Formatter) -> fmt::Result
```



