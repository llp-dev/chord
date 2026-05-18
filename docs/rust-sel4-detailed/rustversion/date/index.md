*[rustversion](../index.md) / [date](index.md)*

---

# Module `date`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Date`](#date) | struct |  |
| [`parse`](#parse) | fn |  |
| [`try_parse`](#try-parse) | fn |  |

## Structs

### `Date`

```rust
struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}
```

#### Trait Implementations

##### `impl Clone for Date`

- <span id="date-clone"></span>`fn clone(&self) -> Date` — [`Date`](#date)

##### `impl Copy for Date`

##### `impl Debug for Date`

- <span id="date-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Date`

- <span id="date-display-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Date`

##### `impl Ord for Date`

- <span id="date-ord-cmp"></span>`fn cmp(&self, other: &Date) -> cmp::Ordering` — [`Date`](#date)

##### `impl PartialEq for Date`

- <span id="date-partialeq-eq"></span>`fn eq(&self, other: &Date) -> bool` — [`Date`](#date)

##### `impl PartialOrd for Date`

- <span id="date-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Date) -> option::Option<cmp::Ordering>` — [`Date`](#date)

##### `impl StructuralPartialEq for Date`

##### `impl ToString for Date`

- <span id="date-tostring-to-string"></span>`fn to_string(&self) -> String`

## Functions

### `parse`

```rust
fn parse(paren: proc_macro::Group, iter: &'_ mut IterImpl) -> std::result::Result<Date, Error>
```

### `try_parse`

```rust
fn try_parse(iter: &'_ mut IterImpl) -> std::result::Result<Date, ()>
```

