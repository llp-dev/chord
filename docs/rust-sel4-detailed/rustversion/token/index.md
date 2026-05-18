*[rustversion](../index.md) / [token](index.md)*

---

# Module `token`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`parse_punct`](#parse-punct) | fn |  |
| [`parse_optional_punct`](#parse-optional-punct) | fn |  |
| [`parse_optional_keyword`](#parse-optional-keyword) | fn |  |
| [`parse_literal`](#parse-literal) | fn |  |
| [`parse_paren`](#parse-paren) | fn |  |
| [`parse_optional_paren`](#parse-optional-paren) | fn |  |
| [`parse_end`](#parse-end) | fn |  |

## Functions

### `parse_punct`

```rust
fn parse_punct(iter: &'_ mut IterImpl, ch: char) -> std::result::Result<(), Error>
```

### `parse_optional_punct`

```rust
fn parse_optional_punct(iter: &'_ mut IterImpl, ch: char) -> Option<()>
```

### `parse_optional_keyword`

```rust
fn parse_optional_keyword(iter: &'_ mut IterImpl, keyword: &str) -> Option<proc_macro::Span>
```

### `parse_literal`

```rust
fn parse_literal(iter: &'_ mut IterImpl) -> std::result::Result<proc_macro::Literal, Error>
```

### `parse_paren`

```rust
fn parse_paren(introducer: &proc_macro::Ident, iter: &'_ mut IterImpl) -> std::result::Result<proc_macro::Group, Error>
```

### `parse_optional_paren`

```rust
fn parse_optional_paren(iter: &'_ mut IterImpl) -> Option<proc_macro::Group>
```

### `parse_end`

```rust
fn parse_end(iter: &'_ mut IterImpl) -> std::result::Result<(), Error>
```

