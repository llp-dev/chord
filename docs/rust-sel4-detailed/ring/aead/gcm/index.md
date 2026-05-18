*[ring](../../index.md) / [aead](../index.md) / [gcm](index.md)*

---

# Module `gcm`

## Contents

- [Modules](#modules)
  - [`gcm_nohw`](#gcm-nohw)
- [Structs](#structs)
  - [`Key`](#key)
  - [`Context`](#context)
  - [`HTable`](#htable)
  - [`u128`](#u128)
  - [`Xi`](#xi)
  - [`ContextInner`](#contextinner)
- [Enums](#enums)
  - [`Implementation`](#implementation)
- [Functions](#functions)
  - [`detect_implementation`](#detect-implementation)
  - [`has_avx_movbe`](#has-avx-movbe)
- [Constants](#constants)
  - [`HTABLE_LEN`](#htable-len)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`gcm_nohw`](#gcm-nohw) | mod |  |
| [`Key`](#key) | struct |  |
| [`Context`](#context) | struct |  |
| [`HTable`](#htable) | struct |  |
| [`u128`](#u128) | struct |  |
| [`Xi`](#xi) | struct |  |
| [`ContextInner`](#contextinner) | struct |  |
| [`Implementation`](#implementation) | enum |  |
| [`detect_implementation`](#detect-implementation) | fn |  |
| [`has_avx_movbe`](#has-avx-movbe) | fn |  |
| [`HTABLE_LEN`](#htable-len) | const |  |

## Modules

- [`gcm_nohw`](gcm_nohw/index.md)

## Structs

### `Key`

```rust
struct Key {
    h_table: HTable,
}
```

#### Implementations

- <span id="key-new"></span>`fn new(h_be: Block, cpu_features: cpu::Features) -> Self` — [`Block`](../block/index.md#block), [`Features`](../../cpu/index.md#features)

#### Trait Implementations

##### `impl Clone for Key`

- <span id="key-clone"></span>`fn clone(&self) -> Key` — [`Key`](#key)

### `Context`

```rust
struct Context {
    inner: ContextInner,
    aad_len: crate::bits::BitLength<u64>,
    in_out_len: crate::bits::BitLength<u64>,
    cpu_features: cpu::Features,
}
```

#### Implementations

- <span id="context-new"></span>`fn new(key: &Key, aad: Aad<&[u8]>, in_out_len: usize, cpu_features: cpu::Features) -> Result<Self, error::Unspecified>` — [`Key`](#key), [`Aad`](../index.md#aad), [`Features`](../../cpu/index.md#features), [`Unspecified`](../../error/index.md#unspecified)

- <span id="context-inner"></span>`fn inner(&mut self) -> (&HTable, &mut Xi)` — [`HTable`](#htable), [`Xi`](#xi)

  Access to `inner` for the integrated AES-GCM implementations only.

- <span id="context-update-blocks"></span>`fn update_blocks(&mut self, input: &[u8])`

- <span id="context-update-block"></span>`fn update_block(&mut self, a: Block)` — [`Block`](../block/index.md#block)

- <span id="context-pre-finish"></span>`fn pre_finish<F>(self, f: F) -> super::Tag` — [`Tag`](../index.md#tag)

- <span id="context-is-avx"></span>`fn is_avx(&self) -> bool`

### `HTable`

```rust
struct HTable {
    Htable: [u128; 16],
}
```

#### Trait Implementations

##### `impl Clone for HTable`

- <span id="htable-clone"></span>`fn clone(&self) -> HTable` — [`HTable`](#htable)

### `u128`

```rust
struct u128 {
    hi: u64,
    lo: u64,
}
```

#### Trait Implementations

##### `impl Clone for u128`

- <span id="u128-clone"></span>`fn clone(&self) -> u128` — [`u128`](#u128)

##### `impl Copy for u128`

### `Xi`

```rust
struct Xi(super::block::Block);
```

#### Trait Implementations

##### `impl BitXorAssign for Xi`

- <span id="xi-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, a: Block)` — [`Block`](../block/index.md#block)

### `ContextInner`

```rust
struct ContextInner {
    Xi: Xi,
    Htable: HTable,
}
```

## Enums

### `Implementation`

```rust
enum Implementation {
    CLMUL,
    Fallback,
}
```

## Functions

### `detect_implementation`

```rust
fn detect_implementation(cpu_features: cpu::Features) -> Implementation
```

### `has_avx_movbe`

```rust
fn has_avx_movbe(cpu_features: cpu::Features) -> bool
```

## Constants

### `HTABLE_LEN`
```rust
const HTABLE_LEN: usize = 16usize;
```

