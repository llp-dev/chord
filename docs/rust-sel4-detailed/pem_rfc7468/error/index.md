*[pem_rfc7468](../index.md) / [error](index.md)*

---

# Module `error`

Error types

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Error`](#error) | enum | PEM errors. |
| [`Result`](#result) | type | Result type with the `pem-rfc7468` crate's [`Error`] type. |

## Enums

### `Error`

```rust
enum Error {
    Base64(base64ct::Error),
    CharacterEncoding,
    EncapsulatedText,
    HeaderDisallowed,
    Label,
    Length,
    Preamble,
    PreEncapsulationBoundary,
    PostEncapsulationBoundary,
    UnexpectedTypeLabel {
        expected: &'static str,
    },
}
```

PEM errors.

#### Variants

- **`Base64`**

  Base64-related errors.

- **`CharacterEncoding`**

  Character encoding-related errors.

- **`EncapsulatedText`**

  Errors in the encapsulated text (which aren't specifically Base64-related).

- **`HeaderDisallowed`**

  Header detected in the encapsulated text.

- **`Label`**

  Invalid label.

- **`Length`**

  Invalid length.

- **`Preamble`**

  "Preamble" (text before pre-encapsulation boundary) contains invalid data.

- **`PreEncapsulationBoundary`**

  Errors in the pre-encapsulation boundary.

- **`PostEncapsulationBoundary`**

  Errors in the post-encapsulation boundary.

- **`UnexpectedTypeLabel`**

  Unexpected PEM type label.

#### Trait Implementations

##### `impl Clone for Error`

- <span id="error-clone"></span>`fn clone(&self) -> Error` — [`Error`](#error)

##### `impl Copy for Error`

##### `impl Debug for Error`

- <span id="error-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Error`

- <span id="error-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Error`

##### `impl PartialEq for Error`

- <span id="error-partialeq-eq"></span>`fn eq(&self, other: &Error) -> bool` — [`Error`](#error)

##### `impl StructuralPartialEq for Error`

##### `impl ToString for Error`

- <span id="error-tostring-to-string"></span>`fn to_string(&self) -> String`

## Type Aliases

### `Result<T>`

```rust
type Result<T> = core::result::Result<T, Error>;
```

Result type with the `pem-rfc7468` crate's [`Error`](#error) type.

