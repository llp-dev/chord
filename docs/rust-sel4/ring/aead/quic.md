**ring > aead > quic**

# Module: aead::quic

## Contents

**Structs**

- [`Algorithm`](#algorithm) - A QUIC Header Protection Algorithm.
- [`HeaderProtectionKey`](#headerprotectionkey) - A key for generating QUIC Header Protection masks.

**Statics**

- [`AES_128`](#aes_128) - AES-128.
- [`AES_256`](#aes_256) - AES-256.
- [`CHACHA20`](#chacha20) - ChaCha20.

**Type Aliases**

- [`Sample`](#sample) - QUIC sample for new key masks

---

## ring::aead::quic::AES_128

*Static*

AES-128.

```rust
static AES_128: Algorithm
```



## ring::aead::quic::AES_256

*Static*

AES-256.

```rust
static AES_256: Algorithm
```



## ring::aead::quic::Algorithm

*Struct*

A QUIC Header Protection Algorithm.

**Methods:**

- `fn key_len(self: &Self) -> usize` - The length of the key.
- `fn sample_len(self: &Self) -> usize` - The required sample length.

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> Result<(), ::core::fmt::Error>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



## ring::aead::quic::CHACHA20

*Static*

ChaCha20.

```rust
static CHACHA20: Algorithm
```



## ring::aead::quic::HeaderProtectionKey

*Struct*

A key for generating QUIC Header Protection masks.

**Methods:**

- `fn new(algorithm: &'static Algorithm, key_bytes: &[u8]) -> Result<Self, error::Unspecified>` - Create a new header protection key.
- `fn new_mask(self: &Self, sample: &[u8]) -> Result<[u8; 5], error::Unspecified>` - Generate a new QUIC Header Protection mask.
- `fn algorithm(self: &Self) -> &'static Algorithm` - The key's algorithm.

**Trait Implementations:**

- **From**
  - `fn from(okm: hkdf::Okm<&'static Algorithm>) -> Self`



## ring::aead::quic::Sample

*Type Alias*: `[u8; 16]`

QUIC sample for new key masks



