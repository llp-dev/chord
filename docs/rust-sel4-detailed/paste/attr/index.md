*[paste](../index.md) / [attr](index.md)*

---

# Module `attr`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`expand_attr`](#expand-attr) | fn |  |
| [`do_paste_name_value_attr`](#do-paste-name-value-attr) | fn |  |
| [`is_stringlike`](#is-stringlike) | fn |  |

## Functions

### `expand_attr`

```rust
fn expand_attr(attr: proc_macro::TokenStream, span: proc_macro::Span, contains_paste: &mut bool) -> std::result::Result<proc_macro::TokenStream, Error>
```

### `do_paste_name_value_attr`

```rust
fn do_paste_name_value_attr(attr: proc_macro::TokenStream, span: proc_macro::Span, leading: usize) -> std::result::Result<proc_macro::TokenStream, Error>
```

### `is_stringlike`

```rust
fn is_stringlike(token: &proc_macro::TokenTree) -> bool
```

