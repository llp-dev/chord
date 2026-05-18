*[zerocopy](../../index.md) / [impls](../index.md) / [tuples](index.md)*

---

# Module `tuples`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`impl_tuple!`](#impl-tuple) | macro | Generates various trait implementations for tuples. |

## Macros

### `impl_tuple!`

Generates various trait implementations for tuples.

# Safety

`impl_tuple!` should be provided name-number pairs, where each number is
the ordinal of the preceding type name.

