*[base64ct](../index.md) / [errors](index.md)*

---

# Module `errors`

Error types

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`InvalidLengthError`](#invalidlengtherror) | struct | Insufficient output buffer length. |
| [`InvalidEncodingError`](#invalidencodingerror) | struct | Invalid encoding of provided Base64 string. |
| [`Error`](#error) | enum | Generic error, union of [`InvalidLengthError`] and [`InvalidEncodingError`]. |
| [`INVALID_ENCODING_MSG`](#invalid-encoding-msg) | const |  |
| [`INVALID_LENGTH_MSG`](#invalid-length-msg) | const |  |

## Structs

### `InvalidLengthError`

```rust
struct InvalidLengthError;
```

Insufficient output buffer length.

#### Trait Implementations

##### `impl Clone for InvalidLengthError`

- <span id="invalidlengtherror-clone"></span>`fn clone(&self) -> InvalidLengthError` — [`InvalidLengthError`](#invalidlengtherror)

##### `impl Copy for InvalidLengthError`

##### `impl Debug for InvalidLengthError`

- <span id="invalidlengtherror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for InvalidLengthError`

- <span id="invalidlengtherror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for InvalidLengthError`

##### `impl Error for InvalidLengthError`

##### `impl PartialEq for InvalidLengthError`

- <span id="invalidlengtherror-partialeq-eq"></span>`fn eq(&self, other: &InvalidLengthError) -> bool` — [`InvalidLengthError`](#invalidlengtherror)

##### `impl StructuralPartialEq for InvalidLengthError`

##### `impl ToString for InvalidLengthError`

- <span id="invalidlengtherror-tostring-to-string"></span>`fn to_string(&self) -> String`

### `InvalidEncodingError`

```rust
struct InvalidEncodingError;
```

Invalid encoding of provided Base64 string.

#### Trait Implementations

##### `impl Clone for InvalidEncodingError`

- <span id="invalidencodingerror-clone"></span>`fn clone(&self) -> InvalidEncodingError` — [`InvalidEncodingError`](#invalidencodingerror)

##### `impl Copy for InvalidEncodingError`

##### `impl Debug for InvalidEncodingError`

- <span id="invalidencodingerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for InvalidEncodingError`

- <span id="invalidencodingerror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for InvalidEncodingError`

##### `impl Error for InvalidEncodingError`

##### `impl PartialEq for InvalidEncodingError`

- <span id="invalidencodingerror-partialeq-eq"></span>`fn eq(&self, other: &InvalidEncodingError) -> bool` — [`InvalidEncodingError`](#invalidencodingerror)

##### `impl StructuralPartialEq for InvalidEncodingError`

##### `impl ToString for InvalidEncodingError`

- <span id="invalidencodingerror-tostring-to-string"></span>`fn to_string(&self) -> String`

## Enums

### `Error`

```rust
enum Error {
    InvalidEncoding,
    InvalidLength,
}
```

Generic error, union of [`InvalidLengthError`](#invalidlengtherror) and [`InvalidEncodingError`](#invalidencodingerror).

#### Variants

- **`InvalidEncoding`**

  Invalid encoding of provided Base64 string.

- **`InvalidLength`**

  Insufficient output buffer length.

#### Trait Implementations

##### `impl Clone for Error`

- <span id="error-clone"></span>`fn clone(&self) -> Error` — [`Error`](#error)

##### `impl Copy for Error`

##### `impl Debug for Error`

- <span id="error-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Error`

- <span id="error-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>`

##### `impl Eq for Error`

##### `impl Error for Error`

##### `impl PartialEq for Error`

- <span id="error-partialeq-eq"></span>`fn eq(&self, other: &Error) -> bool` — [`Error`](#error)

##### `impl StructuralPartialEq for Error`

##### `impl ToString for Error`

- <span id="error-tostring-to-string"></span>`fn to_string(&self) -> String`

## Constants

### `INVALID_ENCODING_MSG`
```rust
const INVALID_ENCODING_MSG: &str;
```

### `INVALID_LENGTH_MSG`
```rust
const INVALID_LENGTH_MSG: &str;
```

