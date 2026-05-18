*[ring](../../index.md) / [aead](../index.md) / [quic](index.md)*

---

# Module `quic`

QUIC Header Protection.

See draft-ietf-quic-tls.

## Contents

- [Structs](#structs)
  - [`HeaderProtectionKey`](#headerprotectionkey)
  - [`Algorithm`](#algorithm)
- [Enums](#enums)
  - [`KeyInner`](#keyinner)
  - [`AlgorithmID`](#algorithmid)
- [Functions](#functions)
  - [`aes_init_128`](#aes-init-128)
  - [`aes_init_256`](#aes-init-256)
  - [`aes_new_mask`](#aes-new-mask)
  - [`chacha20_init`](#chacha20-init)
  - [`chacha20_new_mask`](#chacha20-new-mask)
- [Type Aliases](#type-aliases)
  - [`Sample`](#sample)
- [Constants](#constants)
  - [`SAMPLE_LEN`](#sample-len)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`HeaderProtectionKey`](#headerprotectionkey) | struct | A key for generating QUIC Header Protection masks. |
| [`Algorithm`](#algorithm) | struct | A QUIC Header Protection Algorithm. |
| [`KeyInner`](#keyinner) | enum |  |
| [`AlgorithmID`](#algorithmid) | enum |  |
| [`aes_init_128`](#aes-init-128) | fn |  |
| [`aes_init_256`](#aes-init-256) | fn |  |
| [`aes_new_mask`](#aes-new-mask) | fn |  |
| [`chacha20_init`](#chacha20-init) | fn |  |
| [`chacha20_new_mask`](#chacha20-new-mask) | fn |  |
| [`Sample`](#sample) | type | QUIC sample for new key masks |
| [`SAMPLE_LEN`](#sample-len) | const |  |

## Structs

### `HeaderProtectionKey`

```rust
struct HeaderProtectionKey {
    inner: KeyInner,
    algorithm: &'static Algorithm,
}
```

A key for generating QUIC Header Protection masks.

#### Implementations

- <span id="headerprotectionkey-new"></span>`fn new(algorithm: &'static Algorithm, key_bytes: &[u8]) -> Result<Self, error::Unspecified>` — [`Algorithm`](#algorithm), [`Unspecified`](../../error/index.md#unspecified)

  Create a new header protection key.

  

  `key_bytes` must be exactly `algorithm.key_len` bytes long.

- <span id="headerprotectionkey-new-mask"></span>`fn new_mask(&self, sample: &[u8]) -> Result<[u8; 5], error::Unspecified>` — [`Unspecified`](../../error/index.md#unspecified)

  Generate a new QUIC Header Protection mask.

  

  `sample` must be exactly `self.algorithm().sample_len()` bytes long.

- <span id="headerprotectionkey-algorithm"></span>`fn algorithm(&self) -> &'static Algorithm` — [`Algorithm`](#algorithm)

  The key's algorithm.

### `Algorithm`

```rust
struct Algorithm {
    init: fn(&[u8], cpu::Features) -> Result<KeyInner, error::Unspecified>,
    new_mask: fn(&KeyInner, Sample) -> [u8; 5],
    key_len: usize,
    id: AlgorithmID,
}
```

A QUIC Header Protection Algorithm.

#### Implementations

- <span id="algorithm-key-len"></span>`fn key_len(&self) -> usize`

  The length of the key.

- <span id="algorithm-sample-len"></span>`fn sample_len(&self) -> usize`

  The required sample length.

#### Trait Implementations

##### `impl Debug for Algorithm`

- <span id="algorithm-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> Result<(), ::core::fmt::Error>`

##### `impl Eq for Algorithm`

##### `impl KeyType for &'static Algorithm`

- <span id="static-algorithm-keytype-len"></span>`fn len(&self) -> usize`

##### `impl PartialEq for Algorithm`

- <span id="algorithm-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

## Enums

### `KeyInner`

```rust
enum KeyInner {
    Aes(aes::Key),
    ChaCha20(chacha::Key),
}
```

### `AlgorithmID`

```rust
enum AlgorithmID {
    AES_128,
    AES_256,
    CHACHA20,
}
```

#### Trait Implementations

##### `impl Debug for AlgorithmID`

- <span id="algorithmid-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for AlgorithmID`

##### `impl PartialEq for AlgorithmID`

- <span id="algorithmid-partialeq-eq"></span>`fn eq(&self, other: &AlgorithmID) -> bool` — [`AlgorithmID`](#algorithmid)

##### `impl StructuralPartialEq for AlgorithmID`

## Functions

### `aes_init_128`

```rust
fn aes_init_128(key: &[u8], cpu_features: cpu::Features) -> Result<KeyInner, error::Unspecified>
```

### `aes_init_256`

```rust
fn aes_init_256(key: &[u8], cpu_features: cpu::Features) -> Result<KeyInner, error::Unspecified>
```

### `aes_new_mask`

```rust
fn aes_new_mask(key: &KeyInner, sample: Sample) -> [u8; 5]
```

### `chacha20_init`

```rust
fn chacha20_init(key: &[u8], _cpu_features: cpu::Features) -> Result<KeyInner, error::Unspecified>
```

### `chacha20_new_mask`

```rust
fn chacha20_new_mask(key: &KeyInner, sample: Sample) -> [u8; 5]
```

## Type Aliases

### `Sample`

```rust
type Sample = [u8; 16];
```

QUIC sample for new key masks

## Constants

### `SAMPLE_LEN`
```rust
const SAMPLE_LEN: usize = 16usize;
```

