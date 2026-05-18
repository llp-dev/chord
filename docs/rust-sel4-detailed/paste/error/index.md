*[paste](../index.md) / [error](index.md)*

---

# Module `error`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Error`](#error) | struct |  |
| [`Result`](#result) | type |  |

## Structs

### `Error`

```rust
struct Error {
    begin: proc_macro::Span,
    end: proc_macro::Span,
    msg: String,
}
```

#### Implementations

- <span id="error-new"></span>`fn new(span: Span, msg: &str) -> Self`

- <span id="error-new2"></span>`fn new2(begin: Span, end: Span, msg: &str) -> Self`

- <span id="error-to-compile-error"></span>`fn to_compile_error(&self) -> TokenStream`

## Type Aliases

### `Result<T>`

```rust
type Result<T> = std::result::Result<T, Error>;
```

