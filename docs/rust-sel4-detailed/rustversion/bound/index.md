*[rustversion](../index.md) / [bound](index.md)*

---

# Module `bound`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Bound`](#bound) | enum |  |
| [`parse`](#parse) | fn |  |

## Enums

### `Bound`

```rust
enum Bound {
    Nightly(crate::date::Date),
    Stable(crate::release::Release),
}
```

#### Trait Implementations

##### `impl PartialEq for crate::version::Version`

- <span id="crateversionversion-partialeq-eq"></span>`fn eq(&self, rhs: &Bound) -> bool` — [`Bound`](#bound)

##### `impl PartialOrd for crate::version::Version`

- <span id="crateversionversion-partialord-partial-cmp"></span>`fn partial_cmp(&self, rhs: &Bound) -> Option<Ordering>` — [`Bound`](#bound)

## Functions

### `parse`

```rust
fn parse(paren: proc_macro::Group, iter: &'_ mut IterImpl) -> std::result::Result<Bound, Error>
```

