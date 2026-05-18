*[ring](../../../../index.md) / [ec](../../../index.md) / [suite_b](../../index.md) / [ecdsa](../index.md) / [verification](index.md)*

---

# Module `verification`

ECDSA Signatures using the P-256 and P-384 curves.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`EcdsaVerificationAlgorithm`](#ecdsaverificationalgorithm) | struct | An ECDSA verification algorithm. |
| [`AlgorithmID`](#algorithmid) | enum |  |
| [`split_rs_fixed`](#split-rs-fixed) | fn |  |
| [`split_rs_asn1`](#split-rs-asn1) | fn |  |

## Structs

### `EcdsaVerificationAlgorithm`

```rust
struct EcdsaVerificationAlgorithm {
    ops: &'static PublicScalarOps,
    digest_alg: &'static digest::Algorithm,
    split_rs: fn(&'static ScalarOps, &mut untrusted::Reader<'a>) -> Result<(untrusted::Input<'a>, untrusted::Input<'a>), error::Unspecified>,
    id: AlgorithmID,
}
```

An ECDSA verification algorithm.

#### Implementations

- <span id="ecdsaverificationalgorithm-verify-digest"></span>`fn verify_digest(&self, public_key: untrusted::Input<'_>, e: elem::Elem<N, Unencoded>, signature: untrusted::Input<'_>) -> Result<(), error::Unspecified>` — [`Elem`](../../ops/index.md#elem), [`N`](../../ops/index.md#n), [`Unencoded`](../../../../arithmetic/montgomery/index.md#unencoded), [`Unspecified`](../../../../error/index.md#unspecified)

  This is intentionally not public.

#### Trait Implementations

##### `impl Debug for EcdsaVerificationAlgorithm`

- <span id="ecdsaverificationalgorithm-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> Result<(), ::core::fmt::Error>`

##### `impl Sealed for EcdsaVerificationAlgorithm`

##### `impl VerificationAlgorithm for EcdsaVerificationAlgorithm`

- <span id="ecdsaverificationalgorithm-verificationalgorithm-verify"></span>`fn verify(&self, public_key: untrusted::Input<'_>, msg: untrusted::Input<'_>, signature: untrusted::Input<'_>) -> Result<(), error::Unspecified>` — [`Unspecified`](../../../../error/index.md#unspecified)

## Enums

### `AlgorithmID`

```rust
enum AlgorithmID {
    ECDSA_P256_SHA256_ASN1,
    ECDSA_P256_SHA256_FIXED,
    ECDSA_P256_SHA384_ASN1,
    ECDSA_P384_SHA256_ASN1,
    ECDSA_P384_SHA384_ASN1,
    ECDSA_P384_SHA384_FIXED,
}
```

#### Trait Implementations

##### `impl Debug for AlgorithmID`

- <span id="algorithmid-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

### `split_rs_fixed`

```rust
fn split_rs_fixed<'a>(ops: &'static ScalarOps, input: &mut untrusted::Reader<'a>) -> Result<(untrusted::Input<'a>, untrusted::Input<'a>), error::Unspecified>
```

### `split_rs_asn1`

```rust
fn split_rs_asn1<'a>(_ops: &'static ScalarOps, input: &mut untrusted::Reader<'a>) -> Result<(untrusted::Input<'a>, untrusted::Input<'a>), error::Unspecified>
```

