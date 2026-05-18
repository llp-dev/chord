**serde_json > error**

# Module: error

## Contents

**Structs**

- [`Error`](#error) - This type represents all possible errors that can occur when serializing or

**Enums**

- [`Category`](#category) - Categorizes the cause of a `serde_json::Error`.

**Type Aliases**

- [`Result`](#result) - Alias for a `Result` with the error type `serde_json::Error`.

---

## serde_json::error::Category

*Enum*

Categorizes the cause of a `serde_json::Error`.

**Variants:**
- `Io` - The error was caused by a failure to read or write bytes on an I/O
- `Syntax` - The error was caused by input that was not syntactically valid JSON.
- `Data` - The error was caused by input data that was semantically incorrect.
- `Eof` - The error was caused by prematurely reaching the end of the input data.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Category) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Category`



## serde_json::error::Error

*Struct*

This type represents all possible errors that can occur when serializing or
deserializing JSON data.

**Methods:**

- `fn line(self: &Self) -> usize` - One-based line number at which the error was detected.
- `fn column(self: &Self) -> usize` - One-based column number at which the error was detected.
- `fn classify(self: &Self) -> Category` - Categorizes the cause of this error.
- `fn is_io(self: &Self) -> bool` - Returns true if this error was caused by a failure to read or write
- `fn is_syntax(self: &Self) -> bool` - Returns true if this error was caused by input that was not
- `fn is_data(self: &Self) -> bool` - Returns true if this error was caused by input data that was
- `fn is_eof(self: &Self) -> bool` - Returns true if this error was caused by prematurely reaching the end of
- `fn io_error_kind(self: &Self) -> Option<ErrorKind>` - The kind reported by the underlying standard library I/O error, if this

**Trait Implementations:**

- **Error**
  - `fn custom<T>(msg: T) -> Error`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Error**
  - `fn source(self: &Self) -> Option<&dyn error::Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Error**
  - `fn custom<T>(msg: T) -> Error`
  - `fn invalid_type(unexp: de::Unexpected, exp: &dyn de::Expected) -> Self`
  - `fn invalid_value(unexp: de::Unexpected, exp: &dyn de::Expected) -> Self`



## serde_json::error::Result

*Type Alias*: `result::Result<T, Error>`

Alias for a `Result` with the error type `serde_json::Error`.



