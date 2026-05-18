*[ring](../../../../index.md) / [ec](../../../index.md) / [curve25519](../../index.md) / [ed25519](../index.md) / [signing](index.md)*

---

# Module `signing`

EdDSA Signatures.

## Contents

- [Structs](#structs)
  - [`Ed25519KeyPair`](#ed25519keypair)
  - [`PublicKey`](#publickey)
- [Functions](#functions)
  - [`unwrap_pkcs8`](#unwrap-pkcs8)
- [Type Aliases](#type-aliases)
  - [`Prefix`](#prefix)
  - [`Seed`](#seed)
- [Constants](#constants)
  - [`PREFIX_LEN`](#prefix-len)
  - [`SIGNATURE_LEN`](#signature-len)
  - [`SEED_LEN`](#seed-len)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Ed25519KeyPair`](#ed25519keypair) | struct | An Ed25519 key pair, for signing. |
| [`PublicKey`](#publickey) | struct |  |
| [`unwrap_pkcs8`](#unwrap-pkcs8) | fn |  |
| [`Prefix`](#prefix) | type |  |
| [`Seed`](#seed) | type |  |
| [`PREFIX_LEN`](#prefix-len) | const |  |
| [`SIGNATURE_LEN`](#signature-len) | const |  |
| [`SEED_LEN`](#seed-len) | const |  |

## Structs

### `Ed25519KeyPair`

```rust
struct Ed25519KeyPair {
    private_scalar: Scalar,
    private_prefix: [u8; 32],
    public_key: PublicKey,
}
```

An Ed25519 key pair, for signing.

#### Implementations

- <span id="ed25519keypair-generate-pkcs8"></span>`fn generate_pkcs8(rng: &dyn rand::SecureRandom) -> Result<pkcs8::Document, error::Unspecified>` — [`SecureRandom`](../../../../rand/index.md#securerandom), [`Document`](../../../../pkcs8/index.md#document), [`Unspecified`](../../../../error/index.md#unspecified)

  Generates a new key pair and returns the key pair serialized as a

  PKCS#8 document.

  

  The PKCS#8 document will be a v2 `OneAsymmetricKey` with the public key,

  as described in [RFC 5958 Section 2]; see [RFC 8410 Section 10.3] for an

  example.

  

- <span id="ed25519keypair-from-pkcs8"></span>`fn from_pkcs8(pkcs8: &[u8]) -> Result<Self, error::KeyRejected>` — [`KeyRejected`](../../../../error/index.md#keyrejected)

  Constructs an Ed25519 key pair by parsing an unencrypted PKCS#8 v2

  Ed25519 private key.

  

  `openssl genpkey -algorithm ED25519` generates PKCS# v1 keys, which

  require the use of `Ed25519KeyPair::from_pkcs8_maybe_unchecked()`

  instead of `Ed25519KeyPair::from_pkcs8()`.

  

  The input must be in PKCS#8 v2 format, and in particular it must contain

  the public key in addition to the private key. `from_pkcs8()` will

  verify that the public key and the private key are consistent with each

  other.

  

  Some early implementations of PKCS#8 v2, including earlier versions of

  *ring* and other implementations, wrapped the public key in the wrong

  ASN.1 tags. Both that incorrect form and the standardized form are

  accepted.

  

  If you need to parse PKCS#8 v1 files (without the public key) then use

  `Ed25519KeyPair::from_pkcs8_maybe_unchecked()` instead.

- <span id="ed25519keypair-from-pkcs8-maybe-unchecked"></span>`fn from_pkcs8_maybe_unchecked(pkcs8: &[u8]) -> Result<Self, error::KeyRejected>` — [`KeyRejected`](../../../../error/index.md#keyrejected)

  Constructs an Ed25519 key pair by parsing an unencrypted PKCS#8 v1 or v2

  Ed25519 private key.

  

  `openssl genpkey -algorithm ED25519` generates PKCS# v1 keys.

  

  It is recommended to use `Ed25519KeyPair::from_pkcs8()`, which accepts

  only PKCS#8 v2 files that contain the public key.

  `from_pkcs8_maybe_unchecked()` parses PKCS#2 files exactly like

  `from_pkcs8()`. It also accepts v1 files. PKCS#8 v1 files do not contain

  the public key, so when a v1 file is parsed the public key will be

  computed from the private key, and there will be no consistency check

  between the public key and the private key.

  

  Some early implementations of PKCS#8 v2, including earlier versions of

  *ring* and other implementations, wrapped the public key in the wrong

  ASN.1 tags. Both that incorrect form and the standardized form are

  accepted.

  

  PKCS#8 v2 files are parsed exactly like `Ed25519KeyPair::from_pkcs8()`.

- <span id="ed25519keypair-from-seed-and-public-key"></span>`fn from_seed_and_public_key(seed: &[u8], public_key: &[u8]) -> Result<Self, error::KeyRejected>` — [`KeyRejected`](../../../../error/index.md#keyrejected)

  Constructs an Ed25519 key pair from the private key seed `seed` and its

  public key `public_key`.

  

  It is recommended to use `Ed25519KeyPair::from_pkcs8()` instead.

  

  The private and public keys will be verified to be consistent with each

  other. This helps avoid misuse of the key (e.g. accidentally swapping

  the private key and public key, or using the wrong private key for the

  public key). This also detects any corruption of the public or private

  key.

- <span id="ed25519keypair-from-seed-unchecked"></span>`fn from_seed_unchecked(seed: &[u8]) -> Result<Self, error::KeyRejected>` — [`KeyRejected`](../../../../error/index.md#keyrejected)

  Constructs a Ed25519 key pair from the private key seed `seed`.

  

  It is recommended to use `Ed25519KeyPair::from_pkcs8()` instead. When

  that is not practical, it is recommended to use

  `Ed25519KeyPair::from_seed_and_public_key()` instead.

  

  Since the public key is not given, the public key will be computed from

  the private key. It is not possible to detect misuse or corruption of

  the private key since the public key isn't given as input.

- <span id="ed25519keypair-from-seed"></span>`fn from_seed_(seed: &[u8; 32]) -> Self`

- <span id="ed25519keypair-sign"></span>`fn sign(&self, msg: &[u8]) -> signature::Signature` — [`Signature`](../../../../signature/index.md#signature)

  Returns the signature of the message `msg`.

#### Trait Implementations

##### `impl Debug for Ed25519KeyPair`

- <span id="ed25519keypair-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> Result<(), ::core::fmt::Error>`

##### `impl KeyPair for Ed25519KeyPair`

- <span id="ed25519keypair-keypair-type-publickey"></span>`type PublicKey = PublicKey`

- <span id="ed25519keypair-keypair-public-key"></span>`fn public_key(&self) -> &<Self as >::PublicKey` — [`KeyPair`](../../../../signature/index.md#keypair)

### `PublicKey`

```rust
struct PublicKey([u8; 32]);
```

#### Trait Implementations

##### `impl AsRef for PublicKey`

- <span id="publickey-asref-as-ref"></span>`fn as_ref(&self) -> &[u8]`

##### `impl Clone for PublicKey`

- <span id="publickey-clone"></span>`fn clone(&self) -> PublicKey` — [`PublicKey`](#publickey)

##### `impl Copy for PublicKey`

##### `impl Debug for PublicKey`

- <span id="publickey-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> Result<(), ::core::fmt::Error>`

## Functions

### `unwrap_pkcs8`

```rust
fn unwrap_pkcs8(version: pkcs8::Version, input: untrusted::Input<'_>) -> Result<(untrusted::Input<'_>, Option<untrusted::Input<'_>>), error::KeyRejected>
```

## Type Aliases

### `Prefix`

```rust
type Prefix = [u8; 32];
```

### `Seed`

```rust
type Seed = [u8; 32];
```

## Constants

### `PREFIX_LEN`
```rust
const PREFIX_LEN: usize = 32usize;
```

### `SIGNATURE_LEN`
```rust
const SIGNATURE_LEN: usize = 64usize;
```

### `SEED_LEN`
```rust
const SEED_LEN: usize = 32usize;
```

