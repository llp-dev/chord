*[ring](../../../../index.md) / [ec](../../../index.md) / [suite_b](../../index.md) / [ecdsa](../index.md) / [signing](index.md)*

---

# Module `signing`

ECDSA Signatures using the P-256 and P-384 curves.

## Contents

- [Structs](#structs)
  - [`EcdsaSigningAlgorithm`](#ecdsasigningalgorithm)
  - [`EcdsaKeyPair`](#ecdsakeypair)
  - [`NonceRandom`](#noncerandom)
  - [`NonceRandomKey`](#noncerandomkey)
  - [`PublicKey`](#publickey)
- [Enums](#enums)
  - [`AlgorithmID`](#algorithmid)
- [Functions](#functions)
  - [`format_rs_fixed`](#format-rs-fixed)
  - [`format_rs_asn1`](#format-rs-asn1)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`EcdsaSigningAlgorithm`](#ecdsasigningalgorithm) | struct | An ECDSA signing algorithm. |
| [`EcdsaKeyPair`](#ecdsakeypair) | struct | An ECDSA key pair, used for signing. |
| [`NonceRandom`](#noncerandom) | struct | Generates an ECDSA nonce in a way that attempts to protect against a faulty `SecureRandom`. |
| [`NonceRandomKey`](#noncerandomkey) | struct |  |
| [`PublicKey`](#publickey) | struct |  |
| [`AlgorithmID`](#algorithmid) | enum |  |
| [`format_rs_fixed`](#format-rs-fixed) | fn |  |
| [`format_rs_asn1`](#format-rs-asn1) | fn |  |

## Structs

### `EcdsaSigningAlgorithm`

```rust
struct EcdsaSigningAlgorithm {
    curve: &'static ec::Curve,
    private_scalar_ops: &'static PrivateScalarOps,
    private_key_ops: &'static PrivateKeyOps,
    digest_alg: &'static digest::Algorithm,
    pkcs8_template: &'static pkcs8::Template,
    format_rs: fn(&'static ScalarOps, &elem::Elem<N, Unencoded>, &elem::Elem<N, Unencoded>, &mut [u8]) -> usize,
    id: AlgorithmID,
}
```

An ECDSA signing algorithm.

#### Trait Implementations

##### `impl Debug for EcdsaSigningAlgorithm`

- <span id="ecdsasigningalgorithm-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> Result<(), ::core::fmt::Error>`

##### `impl Eq for EcdsaSigningAlgorithm`

##### `impl PartialEq for EcdsaSigningAlgorithm`

- <span id="ecdsasigningalgorithm-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Sealed for EcdsaSigningAlgorithm`

### `EcdsaKeyPair`

```rust
struct EcdsaKeyPair {
    d: elem::Elem<N, R>,
    nonce_key: NonceRandomKey,
    alg: &'static EcdsaSigningAlgorithm,
    public_key: PublicKey,
}
```

An ECDSA key pair, used for signing.

#### Implementations

- <span id="ecdsakeypair-generate-pkcs8"></span>`fn generate_pkcs8(alg: &'static EcdsaSigningAlgorithm, rng: &dyn rand::SecureRandom) -> Result<pkcs8::Document, error::Unspecified>` — [`EcdsaSigningAlgorithm`](#ecdsasigningalgorithm), [`SecureRandom`](../../../../rand/index.md#securerandom), [`Document`](../../../../pkcs8/index.md#document), [`Unspecified`](../../../../error/index.md#unspecified)

  Generates a new key pair and returns the key pair serialized as a

  PKCS#8 document.

  

  The PKCS#8 document will be a v1 `OneAsymmetricKey` with the public key

  included in the `ECPrivateKey` structure, as described in

  [RFC 5958 Section 2] and [RFC 5915]. The `ECPrivateKey` structure will

  not have a `parameters` field so the generated key is compatible with

  PKCS#11.

  

- <span id="ecdsakeypair-from-pkcs8"></span>`fn from_pkcs8(alg: &'static EcdsaSigningAlgorithm, pkcs8: &[u8], rng: &dyn rand::SecureRandom) -> Result<Self, error::KeyRejected>` — [`EcdsaSigningAlgorithm`](#ecdsasigningalgorithm), [`SecureRandom`](../../../../rand/index.md#securerandom), [`KeyRejected`](../../../../error/index.md#keyrejected)

  Constructs an ECDSA key pair by parsing an unencrypted PKCS#8 v1

  id-ecPublicKey `ECPrivateKey` key.

  

  The input must be in PKCS#8 v1 format. It must contain the public key in

  the `ECPrivateKey` structure; `from_pkcs8()` will verify that the public

  key and the private key are consistent with each other. The algorithm

  identifier must identify the curve by name; it must not use an

  "explicit" encoding of the curve. The `parameters` field of the

  `ECPrivateKey`, if present, must be the same named curve that is in the

  algorithm identifier in the PKCS#8 header.

- <span id="ecdsakeypair-from-private-key-and-public-key"></span>`fn from_private_key_and_public_key(alg: &'static EcdsaSigningAlgorithm, private_key: &[u8], public_key: &[u8], rng: &dyn rand::SecureRandom) -> Result<Self, error::KeyRejected>` — [`EcdsaSigningAlgorithm`](#ecdsasigningalgorithm), [`SecureRandom`](../../../../rand/index.md#securerandom), [`KeyRejected`](../../../../error/index.md#keyrejected)

  Constructs an ECDSA key pair from the private key and public key bytes

  

  The private key must encoded as a big-endian fixed-length integer. For

  example, a P-256 private key must be 32 bytes prefixed with leading

  zeros as needed.

  

  The public key is encoding in uncompressed form using the

  Octet-String-to-Elliptic-Curve-Point algorithm in

  [SEC 1: Elliptic Curve Cryptography, Version 2.0].

  

  This is intended for use by code that deserializes key pairs. It is

  recommended to use `EcdsaKeyPair::from_pkcs8()` (with a PKCS#8-encoded

  key) instead.

- <span id="ecdsakeypair-new"></span>`fn new(alg: &'static EcdsaSigningAlgorithm, key_pair: ec::KeyPair, rng: &dyn rand::SecureRandom) -> Result<Self, error::KeyRejected>` — [`EcdsaSigningAlgorithm`](#ecdsasigningalgorithm), [`KeyPair`](../../../keys/index.md#keypair), [`SecureRandom`](../../../../rand/index.md#securerandom), [`KeyRejected`](../../../../error/index.md#keyrejected)

- <span id="ecdsakeypair-sign"></span>`fn sign(&self, rng: &dyn rand::SecureRandom, message: &[u8]) -> Result<signature::Signature, error::Unspecified>` — [`SecureRandom`](../../../../rand/index.md#securerandom), [`Signature`](../../../../signature/index.md#signature), [`Unspecified`](../../../../error/index.md#unspecified)

  Returns the signature of the `message` using a random nonce generated by `rng`.

- <span id="ecdsakeypair-sign-digest"></span>`fn sign_digest(&self, h: digest::Digest, rng: &dyn rand::SecureRandom) -> Result<signature::Signature, error::Unspecified>` — [`Digest`](../../../../digest/index.md#digest), [`SecureRandom`](../../../../rand/index.md#securerandom), [`Signature`](../../../../signature/index.md#signature), [`Unspecified`](../../../../error/index.md#unspecified)

  Returns the signature of message digest `h` using a "random" nonce

  generated by `rng`.

#### Trait Implementations

##### `impl Debug for EcdsaKeyPair`

- <span id="ecdsakeypair-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> Result<(), ::core::fmt::Error>`

##### `impl KeyPair for EcdsaKeyPair`

- <span id="ecdsakeypair-keypair-type-publickey"></span>`type PublicKey = PublicKey`

- <span id="ecdsakeypair-keypair-public-key"></span>`fn public_key(&self) -> &<Self as >::PublicKey` — [`KeyPair`](../../../../signature/index.md#keypair)

### `NonceRandom<'a>`

```rust
struct NonceRandom<'a> {
    key: &'a NonceRandomKey,
    message_digest: &'a digest::Digest,
    rng: &'a dyn rand::SecureRandom,
}
```

Generates an ECDSA nonce in a way that attempts to protect against a faulty
`SecureRandom`.

#### Trait Implementations

##### `impl Debug for NonceRandom<'_>`

- <span id="noncerandom-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Sealed for NonceRandom<'a>`

##### `impl SecureRandom for NonceRandom<'a>`

- <span id="noncerandom-securerandom-fill"></span>`fn fill(&self, dest: &mut [u8]) -> Result<(), Unspecified>` — [`Unspecified`](../../../../error/index.md#unspecified)

### `NonceRandomKey`

```rust
struct NonceRandomKey(digest::Digest);
```

#### Implementations

- <span id="noncerandomkey-new"></span>`fn new(alg: &EcdsaSigningAlgorithm, seed: &ec::Seed, rng: &dyn rand::SecureRandom) -> Result<Self, error::KeyRejected>` — [`EcdsaSigningAlgorithm`](#ecdsasigningalgorithm), [`Seed`](../../../keys/index.md#seed), [`SecureRandom`](../../../../rand/index.md#securerandom), [`KeyRejected`](../../../../error/index.md#keyrejected)

### `PublicKey`

```rust
struct PublicKey(ec::PublicKey);
```

#### Trait Implementations

##### `impl AsRef for PublicKey`

- <span id="publickey-asref-as-ref"></span>`fn as_ref(&self) -> &[u8]`

##### `impl Clone for PublicKey`

- <span id="publickey-clone"></span>`fn clone(&self) -> PublicKey` — [`PublicKey`](#publickey)

##### `impl Copy for PublicKey`

##### `impl Debug for PublicKey`

- <span id="publickey-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> Result<(), ::core::fmt::Error>`

## Enums

### `AlgorithmID`

```rust
enum AlgorithmID {
    ECDSA_P256_SHA256_FIXED_SIGNING,
    ECDSA_P384_SHA384_FIXED_SIGNING,
    ECDSA_P256_SHA256_ASN1_SIGNING,
    ECDSA_P384_SHA384_ASN1_SIGNING,
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

### `format_rs_fixed`

```rust
fn format_rs_fixed(ops: &'static ScalarOps, r: &elem::Elem<N, Unencoded>, s: &elem::Elem<N, Unencoded>, out: &mut [u8]) -> usize
```

### `format_rs_asn1`

```rust
fn format_rs_asn1(ops: &'static ScalarOps, r: &elem::Elem<N, Unencoded>, s: &elem::Elem<N, Unencoded>, out: &mut [u8]) -> usize
```

