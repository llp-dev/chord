*[object](../../index.md) / [build](../index.md) / [error](index.md)*

---

# Module `error`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Error`](#error) | struct | The error type used within the build module. |
| [`Result`](#result) | type | The result type used within the build module. |

## Structs

### `Error`

```rust
struct Error(alloc::string::String);
```

The error type used within the build module.

#### Implementations

- <span id="error-new"></span>`fn new(message: impl Into<String>) -> Self`

#### Trait Implementations

##### `impl Clone for Error`

- <span id="error-clone"></span>`fn clone(&self) -> Error` — [`Error`](#error)

##### `impl Debug for Error`

- <span id="error-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Error`

- <span id="error-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Error`

##### `impl<K> Equivalent for Error`

- <span id="error-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Error for Error`

##### `impl PartialEq for Error`

- <span id="error-partialeq-eq"></span>`fn eq(&self, other: &Error) -> bool` — [`Error`](#error)

##### `impl StructuralPartialEq for Error`

##### `impl ToString for Error`

- <span id="error-tostring-to-string"></span>`fn to_string(&self) -> String`

## Type Aliases

### `Result<T>`

```rust
type Result<T> = result::Result<T, Error>;
```

The result type used within the build module.

