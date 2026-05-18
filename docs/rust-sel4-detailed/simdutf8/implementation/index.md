*[simdutf8](../index.md) / [implementation](index.md)*

---

# Module `implementation`

Contains UTF-8 validation implementations.

## Contents

- [Modules](#modules)
  - [`algorithm`](#algorithm)
  - [`helpers`](#helpers)
  - [`x86`](#x86)
- [Functions](#functions)
  - [`validate_utf8_basic`](#validate-utf8-basic)
  - [`validate_utf8_compat`](#validate-utf8-compat)
  - [`validate_utf8_basic_fallback`](#validate-utf8-basic-fallback)
  - [`validate_utf8_compat_fallback`](#validate-utf8-compat-fallback)
- [Type Aliases](#type-aliases)
  - [`Utf8ErrorCompat`](#utf8errorcompat)
  - [`Utf8ErrorBasic`](#utf8errorbasic)
  - [`ValidateUtf8Fn`](#validateutf8fn)
  - [`ValidateUtf8CompatFn`](#validateutf8compatfn)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`algorithm`](#algorithm) | mod |  |
| [`helpers`](#helpers) | mod |  |
| [`x86`](#x86) | mod |  |
| [`validate_utf8_basic`](#validate-utf8-basic) | fn | Fn needed instead of re-import, otherwise not inlined in non-std case |
| [`validate_utf8_compat`](#validate-utf8-compat) | fn | Fn needed instead of re-import, otherwise not inlined in non-std case |
| [`validate_utf8_basic_fallback`](#validate-utf8-basic-fallback) | fn |  |
| [`validate_utf8_compat_fallback`](#validate-utf8-compat-fallback) | fn |  |
| [`Utf8ErrorCompat`](#utf8errorcompat) | type |  |
| [`Utf8ErrorBasic`](#utf8errorbasic) | type |  |
| [`ValidateUtf8Fn`](#validateutf8fn) | type |  |
| [`ValidateUtf8CompatFn`](#validateutf8compatfn) | type |  |

## Modules

- [`algorithm`](algorithm/index.md)
- [`helpers`](helpers/index.md)
- [`x86`](x86/index.md)

## Functions

### `validate_utf8_basic`

```rust
unsafe fn validate_utf8_basic(input: &[u8]) -> Result<(), crate::basic::Utf8Error>
```

Fn needed instead of re-import, otherwise not inlined in non-std case

### `validate_utf8_compat`

```rust
unsafe fn validate_utf8_compat(input: &[u8]) -> Result<(), crate::compat::Utf8Error>
```

Fn needed instead of re-import, otherwise not inlined in non-std case

### `validate_utf8_basic_fallback`

```rust
fn validate_utf8_basic_fallback(input: &[u8]) -> Result<(), crate::basic::Utf8Error>
```

### `validate_utf8_compat_fallback`

```rust
fn validate_utf8_compat_fallback(input: &[u8]) -> Result<(), crate::compat::Utf8Error>
```

## Type Aliases

### `Utf8ErrorCompat`

```rust
type Utf8ErrorCompat = crate::compat::Utf8Error;
```

### `Utf8ErrorBasic`

```rust
type Utf8ErrorBasic = crate::basic::Utf8Error;
```

### `ValidateUtf8Fn`

```rust
type ValidateUtf8Fn = fn(&[u8]) -> Result<(), crate::basic::Utf8Error>;
```

### `ValidateUtf8CompatFn`

```rust
type ValidateUtf8CompatFn = fn(&[u8]) -> Result<(), crate::compat::Utf8Error>;
```

