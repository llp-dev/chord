*[getrandom](../index.md) / [lazy](index.md)*

---

# Module `lazy`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`LazyUsize`](#lazyusize) | struct |  |
| [`LazyBool`](#lazybool) | struct |  |

## Structs

### `LazyUsize`

```rust
struct LazyUsize(core::sync::atomic::AtomicUsize);
```

#### Implementations

- <span id="lazyusize-new"></span>`const fn new() -> Self`

- <span id="lazyusize-const-uninit"></span>`const UNINIT: usize`

- <span id="lazyusize-unsync-init"></span>`fn unsync_init(&self, init: impl FnOnce() -> usize) -> usize`

### `LazyBool`

```rust
struct LazyBool(LazyUsize);
```

#### Implementations

- <span id="lazybool-new"></span>`const fn new() -> Self`

- <span id="lazybool-unsync-init"></span>`fn unsync_init(&self, init: impl FnOnce() -> bool) -> bool`

