*[indexmap](../index.md) / [util](index.md)*

---

# Module `util`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`third`](#third) | fn |  |
| [`simplify_range`](#simplify-range) | fn |  |
| [`try_simplify_range`](#try-simplify-range) | fn |  |
| [`slice_eq`](#slice-eq) | fn |  |

## Functions

### `third`

```rust
fn third<A, B, C>(t: (A, B, C)) -> C
```

### `simplify_range`

```rust
fn simplify_range<R>(range: R, len: usize) -> core::ops::Range<usize>
where
    R: RangeBounds<usize>
```

### `try_simplify_range`

```rust
fn try_simplify_range<R>(range: R, len: usize) -> Option<core::ops::Range<usize>>
where
    R: RangeBounds<usize>
```

### `slice_eq`

```rust
fn slice_eq<T, U>(left: &[T], right: &[U], eq: impl Fn(&T, &U) -> bool) -> bool
```

