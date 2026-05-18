*[ring](../index.md) / [cpu](index.md)*

---

# Module `cpu`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`intel`](#intel) | mod |  |
| [`Features`](#features) | struct | A witness indicating that CPU features have been detected and cached. |
| [`features`](#features) | fn |  |

## Modules

- [`intel`](intel/index.md)

## Structs

### `Features`

```rust
struct Features(());
```

A witness indicating that CPU features have been detected and cached.

TODO: Eventually all feature detection logic should be done through
functions that accept a `Features` parameter, to guarantee that nothing
tries to read the cached values before they are written.

This is a zero-sized type so that it can be "stored" wherever convenient.

#### Trait Implementations

##### `impl Clone for Features`

- <span id="features-clone"></span>`fn clone(&self) -> Features` — [`Features`](#features)

##### `impl Copy for Features`

## Functions

### `features`

```rust
fn features() -> Features
```

