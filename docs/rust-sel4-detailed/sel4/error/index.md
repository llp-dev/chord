*[sel4](../index.md) / [error](index.md)*

---

# Module `error`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Error`](#error) | enum | Corresponds to `seL4_Error`. |
| [`Result`](#result) | type | Alias for `Result<_, Error>`. |

## Enums

### `Error`

```rust
enum Error {
    InvalidArgument,
    InvalidCapability,
    IllegalOperation,
    RangeError,
    AlignmentError,
    FailedLookup,
    TruncatedMessage,
    DeleteFirst,
    RevokeFirst,
    NotEnoughMemory,
}
```

Corresponds to `seL4_Error`.

#### Implementations

- <span id="error-into-sys"></span>`const fn into_sys(self) -> sys::seL4_Error::Type`

- <span id="error-from-sys"></span>`fn from_sys(err: sys::seL4_Error::Type) -> Option<Self>`

- <span id="error-wrap"></span>`fn wrap(err: sys::seL4_Error::Type) -> Result<()>` — [`Result`](#result)

- <span id="error-or"></span>`fn or<T>(err: sys::seL4_Error::Type, value: T) -> Result<T>` — [`Result`](#result)

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

## Type Aliases

### `Result<T>`

```rust
type Result<T> = result::Result<T, Error>;
```

Alias for `Result<_, Error>`.

