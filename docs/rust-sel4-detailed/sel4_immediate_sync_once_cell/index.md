# Crate `sel4_immediate_sync_once_cell`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ImmediateSyncOnceCell`](#immediatesynconcecell) | struct |  |

## Structs

### `ImmediateSyncOnceCell<T>`

```rust
struct ImmediateSyncOnceCell<T> {
    init_started: core::sync::atomic::AtomicBool,
    init_completed: core::sync::atomic::AtomicBool,
    inner: core::cell::UnsafeCell<Option<T>>,
}
```

#### Implementations

- <span id="immediatesynconcecell-new"></span>`const fn new() -> Self`

- <span id="immediatesynconcecell-get"></span>`fn get(&self) -> Option<&T>`

- <span id="immediatesynconcecell-set"></span>`fn set(&self, value: T) -> Result<(), T>`

#### Trait Implementations

##### `impl<T> Default for ImmediateSyncOnceCell<T>`

- <span id="immediatesynconcecell-default"></span>`fn default() -> Self`

##### `impl<T> Sync for ImmediateSyncOnceCell<T>`

