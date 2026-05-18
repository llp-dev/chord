*[const_oid](../index.md) / [error](index.md)*

---

# Module `error`

Error types

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Error`](#error) | enum | OID errors. |
| [`Result`](#result) | type | Result type |

## Enums

### `Error`

```rust
enum Error {
    ArcInvalid {
        arc: crate::Arc,
    },
    ArcTooBig,
    Base128,
    DigitExpected {
        actual: u8,
    },
    Empty,
    Length,
    NotEnoughArcs,
    TrailingDot,
}
```

OID errors.

#### Variants

- **`ArcInvalid`**

  Arc exceeds allowed range (i.e. for first or second OID)

- **`ArcTooBig`**

  Arc is too big (exceeds 32-bit limits of this library).
  
  Technically the size of an arc is not constrained by X.660, however
  this library has elected to use `u32` as the arc representation as
  sufficient for PKIX/PKCS usages.

- **`Base128`**

  Base 128 encoding error (used in BER/DER serialization of arcs).

- **`DigitExpected`**

  Expected a digit, but was provided something else.

- **`Empty`**

  Input data is empty.

- **`Length`**

  OID length is invalid (too short or too long).

- **`NotEnoughArcs`**

  Minimum 3 arcs required.

- **`TrailingDot`**

  Trailing `.` character at end of input.

#### Implementations

- <span id="error-panic"></span>`const fn panic(self) -> never`

  Escalate this error into a panic.

  

  This is a workaround until `Result::unwrap` is allowed in `const fn`.

#### Trait Implementations

##### `impl Clone for Error`

- <span id="error-clone"></span>`fn clone(&self) -> Error` — [`Error`](#error)

##### `impl Copy for Error`

##### `impl Debug for Error`

- <span id="error-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Error`

- <span id="error-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Error`

##### `impl Ord for Error`

- <span id="error-ord-cmp"></span>`fn cmp(&self, other: &Error) -> cmp::Ordering` — [`Error`](#error)

##### `impl PartialEq for Error`

- <span id="error-partialeq-eq"></span>`fn eq(&self, other: &Error) -> bool` — [`Error`](#error)

##### `impl PartialOrd for Error`

- <span id="error-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Error) -> option::Option<cmp::Ordering>` — [`Error`](#error)

##### `impl StructuralPartialEq for Error`

## Type Aliases

### `Result<T>`

```rust
type Result<T> = core::result::Result<T, Error>;
```

Result type

