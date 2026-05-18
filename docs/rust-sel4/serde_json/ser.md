**serde_json > ser**

# Module: ser

## Contents

**Structs**

- [`CompactFormatter`](#compactformatter) - This structure compacts a JSON value with no extra whitespace.
- [`PrettyFormatter`](#prettyformatter) - This structure pretty prints a JSON value to make it human readable.
- [`Serializer`](#serializer) - A structure for serializing Rust values into JSON.

**Enums**

- [`CharEscape`](#charescape) - Represents a character escape code in a type-safe manner.

**Functions**

- [`to_string`](#to_string) - Serialize the given data structure as a String of JSON.
- [`to_string_pretty`](#to_string_pretty) - Serialize the given data structure as a pretty-printed String of JSON.
- [`to_vec`](#to_vec) - Serialize the given data structure as a JSON byte vector.
- [`to_vec_pretty`](#to_vec_pretty) - Serialize the given data structure as a pretty-printed JSON byte vector.
- [`to_writer`](#to_writer) - Serialize the given data structure as JSON into the I/O stream.
- [`to_writer_pretty`](#to_writer_pretty) - Serialize the given data structure as pretty-printed JSON into the I/O

**Traits**

- [`Formatter`](#formatter) - This trait abstracts away serializing the JSON control characters, which allows the user to

---

## serde_json::ser::CharEscape

*Enum*

Represents a character escape code in a type-safe manner.

**Variants:**
- `Quote` - An escaped quote `"`
- `ReverseSolidus` - An escaped reverse solidus `\`
- `Solidus` - An escaped solidus `/`
- `Backspace` - An escaped backspace character (usually escaped as `\b`)
- `FormFeed` - An escaped form feed character (usually escaped as `\f`)
- `LineFeed` - An escaped line feed character (usually escaped as `\n`)
- `CarriageReturn` - An escaped carriage return character (usually escaped as `\r`)
- `Tab` - An escaped tab character (usually escaped as `\t`)
- `AsciiControl(u8)` - An escaped ASCII plane control character (usually escaped as



## serde_json::ser::CompactFormatter

*Struct*

This structure compacts a JSON value with no extra whitespace.

**Unit Struct**

**Traits:** Formatter

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> CompactFormatter`
- **Clone**
  - `fn clone(self: &Self) -> CompactFormatter`



## serde_json::ser::Formatter

*Trait*

This trait abstracts away serializing the JSON control characters, which allows the user to
optionally pretty print the JSON output.

**Methods:**

- `write_null`: Writes a `null` value to the specified writer.
- `write_bool`: Writes a `true` or `false` value to the specified writer.
- `write_i8`: Writes an integer value like `-123` to the specified writer.
- `write_i16`: Writes an integer value like `-123` to the specified writer.
- `write_i32`: Writes an integer value like `-123` to the specified writer.
- `write_i64`: Writes an integer value like `-123` to the specified writer.
- `write_i128`: Writes an integer value like `-123` to the specified writer.
- `write_u8`: Writes an integer value like `123` to the specified writer.
- `write_u16`: Writes an integer value like `123` to the specified writer.
- `write_u32`: Writes an integer value like `123` to the specified writer.
- `write_u64`: Writes an integer value like `123` to the specified writer.
- `write_u128`: Writes an integer value like `123` to the specified writer.
- `write_f32`: Writes a floating point value like `-31.26e+12` to the specified writer.
- `write_f64`: Writes a floating point value like `-31.26e+12` to the specified writer.
- `write_number_str`: Writes a number that has already been rendered to a string.
- `begin_string`: Called before each series of `write_string_fragment` and
- `end_string`: Called after each series of `write_string_fragment` and
- `write_string_fragment`: Writes a string fragment that doesn't need any escaping to the
- `write_char_escape`: Writes a character escape code to the specified writer.
- `write_byte_array`: Writes the representation of a byte array. Formatters can choose whether
- `begin_array`: Called before every array.  Writes a `[` to the specified
- `end_array`: Called after every array.  Writes a `]` to the specified
- `begin_array_value`: Called before every array value.  Writes a `,` if needed to
- `end_array_value`: Called after every array value.
- `begin_object`: Called before every object.  Writes a `{` to the specified
- `end_object`: Called after every object.  Writes a `}` to the specified
- `begin_object_key`: Called before every object key.
- `end_object_key`: Called after every object key.  A `:` should be written to the
- `begin_object_value`: Called before every object value.  A `:` should be written to
- `end_object_value`: Called after every object value.
- `write_raw_fragment`: Writes a raw JSON fragment that doesn't need any escaping to the



## serde_json::ser::PrettyFormatter

*Struct*

This structure pretty prints a JSON value to make it human readable.

**Generic Parameters:**
- 'a

**Methods:**

- `fn new() -> Self` - Construct a pretty printer formatter that defaults to using two spaces for indentation.
- `fn with_indent(indent: &'a [u8]) -> Self` - Construct a pretty printer formatter that uses the `indent` string for indentation.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Formatter**
  - `fn begin_array<W>(self: & mut Self, writer: & mut W) -> io::Result<()>`
  - `fn end_array<W>(self: & mut Self, writer: & mut W) -> io::Result<()>`
  - `fn begin_array_value<W>(self: & mut Self, writer: & mut W, first: bool) -> io::Result<()>`
  - `fn end_array_value<W>(self: & mut Self, _writer: & mut W) -> io::Result<()>`
  - `fn begin_object<W>(self: & mut Self, writer: & mut W) -> io::Result<()>`
  - `fn end_object<W>(self: & mut Self, writer: & mut W) -> io::Result<()>`
  - `fn begin_object_key<W>(self: & mut Self, writer: & mut W, first: bool) -> io::Result<()>`
  - `fn begin_object_value<W>(self: & mut Self, writer: & mut W) -> io::Result<()>`
  - `fn end_object_value<W>(self: & mut Self, _writer: & mut W) -> io::Result<()>`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> PrettyFormatter<'a>`



## serde_json::ser::Serializer

*Struct*

A structure for serializing Rust values into JSON.

**Generic Parameters:**
- W
- F

**Methods:**

- `fn new(writer: W) -> Self` - Creates a new JSON serializer.
- `fn with_formatter(writer: W, formatter: F) -> Self` - Creates a new JSON visitor whose output will be written to the writer
- `fn into_inner(self: Self) -> W` - Unwrap the `Writer` from the `Serializer`.
- `fn pretty(writer: W) -> Self` - Creates a new JSON pretty print serializer.



## serde_json::ser::to_string

*Function*

Serialize the given data structure as a String of JSON.

# Errors

Serialization can fail if `T`'s implementation of `Serialize` decides to
fail, or if `T` contains a map with non-string keys.

```rust
fn to_string<T>(value: &T) -> crate::error::Result<alloc::string::String>
```



## serde_json::ser::to_string_pretty

*Function*

Serialize the given data structure as a pretty-printed String of JSON.

# Errors

Serialization can fail if `T`'s implementation of `Serialize` decides to
fail, or if `T` contains a map with non-string keys.

```rust
fn to_string_pretty<T>(value: &T) -> crate::error::Result<alloc::string::String>
```



## serde_json::ser::to_vec

*Function*

Serialize the given data structure as a JSON byte vector.

# Errors

Serialization can fail if `T`'s implementation of `Serialize` decides to
fail, or if `T` contains a map with non-string keys.

```rust
fn to_vec<T>(value: &T) -> crate::error::Result<alloc::vec::Vec<u8>>
```



## serde_json::ser::to_vec_pretty

*Function*

Serialize the given data structure as a pretty-printed JSON byte vector.

# Errors

Serialization can fail if `T`'s implementation of `Serialize` decides to
fail, or if `T` contains a map with non-string keys.

```rust
fn to_vec_pretty<T>(value: &T) -> crate::error::Result<alloc::vec::Vec<u8>>
```



## serde_json::ser::to_writer

*Function*

Serialize the given data structure as JSON into the I/O stream.

Serialization guarantees it only feeds valid UTF-8 sequences to the writer.

# Errors

Serialization can fail if `T`'s implementation of `Serialize` decides to
fail, or if `T` contains a map with non-string keys.

```rust
fn to_writer<W, T>(writer: W, value: &T) -> crate::error::Result<()>
```



## serde_json::ser::to_writer_pretty

*Function*

Serialize the given data structure as pretty-printed JSON into the I/O
stream.

Serialization guarantees it only feeds valid UTF-8 sequences to the writer.

# Errors

Serialization can fail if `T`'s implementation of `Serialize` decides to
fail, or if `T` contains a map with non-string keys.

```rust
fn to_writer_pretty<W, T>(writer: W, value: &T) -> crate::error::Result<()>
```



