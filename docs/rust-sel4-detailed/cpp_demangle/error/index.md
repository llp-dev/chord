*[cpp_demangle](../index.md) / [error](index.md)*

---

# Module `error`

Custom `Error` and `Result` types for the `cpp_demangle` crate.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Error`](#error) | enum | Errors that can occur while demangling a symbol. |
| [`Result`](#result) | type | A demangling result of `T` or a `cpp_demangle::error::Error`. |

## Enums

### `Error`

```rust
enum Error {
    UnexpectedEnd,
    UnexpectedText,
    BadBackReference,
    BadTemplateArgReference,
    ForwardTemplateArgReference,
    BadFunctionArgReference,
    BadLeafNameReference,
    Overflow,
    TooMuchRecursion,
}
```

Errors that can occur while demangling a symbol.

#### Variants

- **`UnexpectedEnd`**

  The mangled symbol ends abruptly.

- **`UnexpectedText`**

  The mangled symbol is not well-formed.

- **`BadBackReference`**

  Found a back reference that is out-of-bounds of the substitution
  table.

- **`BadTemplateArgReference`**

  Found a reference to a template arg that is either out-of-bounds, or in
  a context without template args.

- **`ForwardTemplateArgReference`**

  Found a reference to a template arg from within the arg itself (or from
  within an earlier arg).

- **`BadFunctionArgReference`**

  Found a reference to a function arg that is either out-of-bounds, or in
  a context without function args.

- **`BadLeafNameReference`**

  Found a reference to a leaf name in a context where there is no current
  leaf name.

- **`Overflow`**

  An overflow or underflow would occur when parsing an integer in a
  mangled symbol.

- **`TooMuchRecursion`**

  Encountered too much recursion when demangling symbol.

#### Trait Implementations

##### `impl Clone for Error`

- <span id="error-clone"></span>`fn clone(&self) -> Error` — [`Error`](#error)

##### `impl Copy for Error`

##### `impl Debug for Error`

- <span id="error-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Error`

- <span id="error-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Error`

##### `impl Hash for Error`

- <span id="error-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Error`

- <span id="error-partialeq-eq"></span>`fn eq(&self, other: &Error) -> bool` — [`Error`](#error)

##### `impl StructuralPartialEq for Error`

##### `impl ToString for Error`

- <span id="error-tostring-to-string"></span>`fn to_string(&self) -> String`

## Type Aliases

### `Result<T>`

```rust
type Result<T> = ::core::result::Result<T, Error>;
```

A demangling result of `T` or a `cpp_demangle::error::Error`.

