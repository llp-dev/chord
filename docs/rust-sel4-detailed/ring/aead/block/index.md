*[ring](../../index.md) / [aead](../index.md) / [block](index.md)*

---

# Module `block`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Block`](#block) | struct |  |
| [`BLOCK_LEN`](#block-len) | const |  |

## Structs

### `Block`

```rust
struct Block([u8; 16]);
```

#### Implementations

- <span id="block-zero"></span>`fn zero() -> Self`

- <span id="block-overwrite-part-at"></span>`fn overwrite_part_at(&mut self, index: usize, a: &[u8])`

- <span id="block-zero-from"></span>`fn zero_from(&mut self, index: usize)`

#### Trait Implementations

##### `impl AsRef for Block`

- <span id="block-asref-as-ref"></span>`fn as_ref(&self) -> &[u8; 16]`

##### `impl BitXor for Block`

- <span id="block-bitxor-type-output"></span>`type Output = Block`

- <span id="block-bitxor"></span>`fn bitxor(self, a: Self) -> Self`

##### `impl BitXorAssign for Block`

- <span id="block-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, a: Self)`

##### `impl Clone for Block`

- <span id="block-clone"></span>`fn clone(&self) -> Block` — [`Block`](#block)

##### `impl Copy for Block`

## Constants

### `BLOCK_LEN`
```rust
const BLOCK_LEN: usize = 16usize;
```

