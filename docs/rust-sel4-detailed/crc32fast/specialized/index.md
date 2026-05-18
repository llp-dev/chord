*[crc32fast](../index.md) / [specialized](index.md)*

---

# Module `specialized`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`pclmulqdq`](#pclmulqdq) | mod | Specialized checksum code for the x86 CPU architecture, based on the efficient algorithm described in the following whitepaper |
| [`State`](#state) | struct |  |

## Modules

- [`pclmulqdq`](pclmulqdq/index.md) — Specialized checksum code for the x86 CPU architecture, based on the efficient algorithm described

## Structs

### `State`

```rust
struct State {
    state: u32,
}
```

#### Implementations

- <span id="state-new"></span>`fn new(state: u32) -> Option<Self>`

- <span id="state-update"></span>`fn update(&mut self, buf: &[u8])`

- <span id="state-finalize"></span>`fn finalize(self) -> u32`

- <span id="state-reset"></span>`fn reset(&mut self)`

- <span id="state-combine"></span>`fn combine(&mut self, other: u32, amount: u64)`

#### Trait Implementations

##### `impl Clone for State`

- <span id="state-clone"></span>`fn clone(&self) -> State` — [`State`](pclmulqdq/index.md#state)

