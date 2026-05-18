*[slab](../index.md) / [builder](index.md)*

---

# Module `builder`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Builder`](#builder) | struct |  |

## Structs

### `Builder<T>`

```rust
struct Builder<T> {
    slab: crate::Slab<T>,
    vacant_list_broken: bool,
    first_vacant_index: Option<usize>,
}
```

#### Implementations

- <span id="builder-with-capacity"></span>`fn with_capacity(capacity: usize) -> Self`

- <span id="builder-pair"></span>`fn pair(&mut self, key: usize, value: T)`

- <span id="builder-build"></span>`fn build(self) -> Slab<T>` — [`Slab`](../index.md#slab)

