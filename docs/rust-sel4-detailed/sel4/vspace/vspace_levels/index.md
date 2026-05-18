*[sel4](../../index.md) / [vspace](../index.md) / [vspace_levels](index.md)*

---

# Module `vspace_levels`

Items describing the layout of address translation structures for this kernel configuration.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`span_bits`](#span-bits) | fn | The number of address bits spanned by the given translation table level. |
| [`step_bits`](#step-bits) | fn | The number of address bits spanned by entries of the given translation table level. |
| [`NUM_LEVELS`](#num-levels) | const | The maximum number of levels of translation tables for this kernel configuration. |
| [`HIGHEST_LEVEL_WITH_PAGE_ENTRIES`](#highest-level-with-page-entries) | const | Highest level of translation table whose entries can be pages rather than lower-level translation tables. |

## Functions

### `span_bits`

```rust
fn span_bits(level: usize) -> usize
```

The number of address bits spanned by the given translation table level.

### `step_bits`

```rust
fn step_bits(level: usize) -> usize
```

The number of address bits spanned by entries of the given translation table level.

## Constants

### `NUM_LEVELS`
```rust
const NUM_LEVELS: usize = 4usize;
```

### `HIGHEST_LEVEL_WITH_PAGE_ENTRIES`
```rust
const HIGHEST_LEVEL_WITH_PAGE_ENTRIES: usize = 1usize;
```

