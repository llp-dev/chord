*[ring](../../index.md) / [ec](../index.md) / [keys](index.md)*

---

# Module `keys`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`KeyPair`](#keypair) | struct |  |
| [`Seed`](#seed) | struct |  |
| [`PublicKey`](#publickey) | struct |  |
| [`PUBLIC_KEY_MAX_LEN`](#public-key-max-len) | const | The maximum length, in bytes, of an encoded public key. |

## Structs

### `KeyPair`

```rust
struct KeyPair {
    seed: Seed,
    public_key: PublicKey,
}
```

#### Implementations

- <span id="keypair-derive"></span>`fn derive(seed: Seed) -> Result<Self, error::Unspecified>` — [`Seed`](#seed), [`Unspecified`](../../error/index.md#unspecified)

- <span id="keypair-public-key"></span>`fn public_key(&self) -> &PublicKey` — [`PublicKey`](#publickey)

- <span id="keypair-split"></span>`fn split(self) -> (Seed, PublicKey)` — [`Seed`](#seed), [`PublicKey`](#publickey)

### `Seed`

```rust
struct Seed {
    bytes: [u8; 48],
    curve: &'static super::Curve,
    cpu_features: cpu::Features,
}
```

#### Implementations

- <span id="seed-generate"></span>`fn generate(curve: &'static Curve, rng: &dyn rand::SecureRandom, cpu_features: cpu::Features) -> Result<Self, error::Unspecified>` — [`Curve`](../index.md#curve), [`SecureRandom`](../../rand/index.md#securerandom), [`Features`](../../cpu/index.md#features), [`Unspecified`](../../error/index.md#unspecified)

- <span id="seed-from-bytes"></span>`fn from_bytes(curve: &'static Curve, bytes: untrusted::Input<'_>, cpu_features: cpu::Features) -> Result<Self, error::Unspecified>` — [`Curve`](../index.md#curve), [`Features`](../../cpu/index.md#features), [`Unspecified`](../../error/index.md#unspecified)

- <span id="seed-bytes-less-safe"></span>`fn bytes_less_safe(&self) -> &[u8]`

- <span id="seed-compute-public-key"></span>`fn compute_public_key(&self) -> Result<PublicKey, error::Unspecified>` — [`PublicKey`](#publickey), [`Unspecified`](../../error/index.md#unspecified)

### `PublicKey`

```rust
struct PublicKey {
    bytes: [u8; 97],
    len: usize,
}
```

#### Trait Implementations

##### `impl AsRef for PublicKey`

- <span id="publickey-asref-as-ref"></span>`fn as_ref(&self) -> &[u8]`

##### `impl Clone for PublicKey`

- <span id="publickey-clone"></span>`fn clone(&self) -> PublicKey` — [`PublicKey`](#publickey)

##### `impl Copy for PublicKey`

## Constants

### `PUBLIC_KEY_MAX_LEN`
```rust
const PUBLIC_KEY_MAX_LEN: usize = 97usize;
```

The maximum length, in bytes, of an encoded public key.

