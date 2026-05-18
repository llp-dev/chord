*[hash32](../index.md) / [fnv](index.md)*

---

# Module `fnv`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Hasher`](#hasher) | struct | 32-bit Fowler-Noll-Vo hasher |
| [`BASIS`](#basis) | const |  |
| [`PRIME`](#prime) | const |  |

## Structs

### `Hasher`

```rust
struct Hasher {
    state: u32,
}
```

32-bit Fowler-Noll-Vo hasher

#### Trait Implementations

##### `impl Default for Hasher`

- <span id="hasher-default"></span>`fn default() -> Self`

##### `impl Hasher for Hasher`

- <span id="hasher-hasher-finish32"></span>`fn finish32(&self) -> u32`

## Constants

### `BASIS`
```rust
const BASIS: u32 = 2_166_136_261u32;
```

### `PRIME`
```rust
const PRIME: u32 = 16_777_619u32;
```

