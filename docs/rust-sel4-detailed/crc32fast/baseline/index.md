*[crc32fast](../index.md) / [baseline](index.md)*

---

# Module `baseline`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`State`](#state) | struct |  |
| [`update_fast_16`](#update-fast-16) | fn |  |
| [`update_slow`](#update-slow) | fn |  |

## Structs

### `State`

```rust
struct State {
    state: u32,
}
```

#### Implementations

- <span id="state-new"></span>`fn new(state: u32) -> Self`

- <span id="state-update"></span>`fn update(&mut self, buf: &[u8])`

- <span id="state-finalize"></span>`fn finalize(self) -> u32`

- <span id="state-reset"></span>`fn reset(&mut self)`

- <span id="state-combine"></span>`fn combine(&mut self, other: u32, amount: u64)`

#### Trait Implementations

##### `impl Clone for State`

- <span id="state-clone"></span>`fn clone(&self) -> State` — [`State`](#state)

## Functions

### `update_fast_16`

```rust
fn update_fast_16(prev: u32, buf: &[u8]) -> u32
```

### `update_slow`

```rust
fn update_slow(prev: u32, buf: &[u8]) -> u32
```

