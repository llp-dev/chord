# Crate `sel4_immutable_cell`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ImmutableCell`](#immutablecell) | struct |  |

## Structs

### `ImmutableCell<T: ?Sized>`

```rust
struct ImmutableCell<T: ?Sized> {
    value: core::cell::UnsafeCell<T>,
}
```

#### Implementations

- <span id="immutablecell-new"></span>`const fn new(value: T) -> Self`

#### Trait Implementations

##### `impl<T: Default> Default for ImmutableCell<T>`

- <span id="immutablecell-default"></span>`fn default() -> Self`

##### `impl<T> Sync for ImmutableCell<T>`

