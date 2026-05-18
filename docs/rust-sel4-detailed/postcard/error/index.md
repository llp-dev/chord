*[postcard](../index.md) / [error](index.md)*

---

# Module `error`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Error`](#error) | enum | This is the error type used by Postcard |
| [`Result`](#result) | type | This is the Result type used by Postcard. |

## Enums

### `Error`

```rust
enum Error {
    WontImplement,
    NotYetImplemented,
    SerializeBufferFull,
    SerializeSeqLengthUnknown,
    DeserializeUnexpectedEnd,
    DeserializeBadVarint,
    DeserializeBadBool,
    DeserializeBadChar,
    DeserializeBadUtf8,
    DeserializeBadOption,
    DeserializeBadEnum,
    DeserializeBadEncoding,
    DeserializeBadCrc,
    SerdeSerCustom,
    SerdeDeCustom,
    CollectStrError,
}
```

This is the error type used by Postcard

#### Variants

- **`WontImplement`**

  This is a feature that postcard will never implement

- **`NotYetImplemented`**

  This is a feature that postcard intends to support, but does not yet

- **`SerializeBufferFull`**

  The serialize buffer is full

- **`SerializeSeqLengthUnknown`**

  The length of a sequence must be known

- **`DeserializeUnexpectedEnd`**

  Hit the end of buffer, expected more data

- **`DeserializeBadVarint`**

  Found a varint that didn't terminate. Is the usize too big for this platform?

- **`DeserializeBadBool`**

  Found a bool that wasn't 0 or 1

- **`DeserializeBadChar`**

  Found an invalid unicode char

- **`DeserializeBadUtf8`**

  Tried to parse invalid utf-8

- **`DeserializeBadOption`**

  Found an Option discriminant that wasn't 0 or 1

- **`DeserializeBadEnum`**

  Found an enum discriminant that was > `u32::MAX`

- **`DeserializeBadEncoding`**

  The original data was not well encoded

- **`DeserializeBadCrc`**

  Bad CRC while deserializing

- **`SerdeSerCustom`**

  Serde Serialization Error

- **`SerdeDeCustom`**

  Serde Deserialization Error

- **`CollectStrError`**

  Error while processing `collect_str` during serialization

#### Trait Implementations

##### `impl Clone for Error`

- <span id="error-clone"></span>`fn clone(&self) -> Error` — [`Error`](#error)

##### `impl Debug for Error`

- <span id="error-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for Error`

- <span id="error-deserialize"></span>`fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

##### `impl DeserializeOwned for Error`

##### `impl Display for Error`

- <span id="error-display-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for Error`

##### `impl Error for Error`

- <span id="error-error-custom"></span>`fn custom<T>(_msg: T) -> Self`

##### `impl PartialEq for Error`

- <span id="error-partialeq-eq"></span>`fn eq(&self, other: &Error) -> bool` — [`Error`](#error)

##### `impl Serialize for Error`

- <span id="error-serialize"></span>`fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

##### `impl StructuralPartialEq for Error`

##### `impl ToString for Error`

- <span id="error-tostring-to-string"></span>`fn to_string(&self) -> String`

## Type Aliases

### `Result<T>`

```rust
type Result<T> = ::core::result::Result<T, Error>;
```

This is the Result type used by Postcard.

