*[heapless](../index.md) / [slice](index.md)*

---

# Module `slice`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`range`](#range) | fn |  |

## Functions

### `range`

```rust
fn range<R>(range: R, bounds: ops::RangeTo<usize>) -> ops::Range<usize>
where
    R: ops::RangeBounds<usize>
```

