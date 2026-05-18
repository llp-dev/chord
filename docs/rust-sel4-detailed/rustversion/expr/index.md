*[rustversion](../index.md) / [expr](index.md)*

---

# Module `expr`

## Contents

- [Enums](#enums)
  - [`Expr`](#expr)
- [Functions](#functions)
  - [`parse`](#parse)
  - [`parse_nightly`](#parse-nightly)
  - [`parse_stable`](#parse-stable)
  - [`parse_since`](#parse-since)
  - [`parse_before`](#parse-before)
  - [`parse_not`](#parse-not)
  - [`parse_any`](#parse-any)
  - [`parse_all`](#parse-all)
  - [`parse_comma_separated`](#parse-comma-separated)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Expr`](#expr) | enum |  |
| [`parse`](#parse) | fn |  |
| [`parse_nightly`](#parse-nightly) | fn |  |
| [`parse_stable`](#parse-stable) | fn |  |
| [`parse_since`](#parse-since) | fn |  |
| [`parse_before`](#parse-before) | fn |  |
| [`parse_not`](#parse-not) | fn |  |
| [`parse_any`](#parse-any) | fn |  |
| [`parse_all`](#parse-all) | fn |  |
| [`parse_comma_separated`](#parse-comma-separated) | fn |  |

## Enums

### `Expr`

```rust
enum Expr {
    Stable,
    Beta,
    Nightly,
    Date(crate::date::Date),
    Since(crate::bound::Bound),
    Before(crate::bound::Bound),
    Release(crate::release::Release),
    Not(Box<Expr>),
    Any(Vec<Expr>),
    All(Vec<Expr>),
}
```

#### Implementations

- <span id="expr-eval"></span>`fn eval(&self, rustc: Version) -> bool` — [`Version`](../version/index.md#version)

## Functions

### `parse`

```rust
fn parse(iter: &'_ mut IterImpl) -> std::result::Result<Expr, Error>
```

### `parse_nightly`

```rust
fn parse_nightly(iter: &'_ mut IterImpl) -> std::result::Result<Expr, Error>
```

### `parse_stable`

```rust
fn parse_stable(iter: &'_ mut IterImpl) -> std::result::Result<Expr, Error>
```

### `parse_since`

```rust
fn parse_since(introducer: &proc_macro::Ident, iter: &'_ mut IterImpl) -> std::result::Result<Expr, Error>
```

### `parse_before`

```rust
fn parse_before(introducer: &proc_macro::Ident, iter: &'_ mut IterImpl) -> std::result::Result<Expr, Error>
```

### `parse_not`

```rust
fn parse_not(introducer: &proc_macro::Ident, iter: &'_ mut IterImpl) -> std::result::Result<Expr, Error>
```

### `parse_any`

```rust
fn parse_any(introducer: &proc_macro::Ident, iter: &'_ mut IterImpl) -> std::result::Result<Expr, Error>
```

### `parse_all`

```rust
fn parse_all(introducer: &proc_macro::Ident, iter: &'_ mut IterImpl) -> std::result::Result<Expr, Error>
```

### `parse_comma_separated`

```rust
fn parse_comma_separated(iter: &'_ mut IterImpl) -> std::result::Result<Vec<Expr>, Error>
```

