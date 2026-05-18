*[ring](../../../../index.md) / [ec](../../../index.md) / [curve25519](../../index.md) / [ed25519](../index.md) / [verification](index.md)*

---

# Module `verification`

EdDSA Signatures.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`EdDSAParameters`](#eddsaparameters) | struct | Parameters for EdDSA signing and verification. |
| [`x25519_ge_double_scalarmult_vartime`](#x25519-ge-double-scalarmult-vartime) | fn |  |

## Structs

### `EdDSAParameters`

```rust
struct EdDSAParameters;
```

Parameters for EdDSA signing and verification.

#### Trait Implementations

##### `impl Debug for EdDSAParameters`

- <span id="eddsaparameters-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error>`

##### `impl Sealed for EdDSAParameters`

##### `impl VerificationAlgorithm for EdDSAParameters`

- <span id="eddsaparameters-verificationalgorithm-verify"></span>`fn verify(&self, public_key: untrusted::Input<'_>, msg: untrusted::Input<'_>, signature: untrusted::Input<'_>) -> Result<(), error::Unspecified>` — [`Unspecified`](../../../../error/index.md#unspecified)

## Functions

### `x25519_ge_double_scalarmult_vartime`

```rust
unsafe fn x25519_ge_double_scalarmult_vartime(r: &mut Point, a_coeff: &Scalar, a: &ExtPoint, b_coeff: &Scalar)
```

