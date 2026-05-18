*[ring](../index.md) / [sealed](index.md)*

---

# Module `sealed`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Sealed`](#sealed) | trait | Traits that are designed to only be implemented internally in *ring*. |

## Traits

### `Sealed`

```rust
trait Sealed { ... }
```

Traits that are designed to only be implemented internally in *ring*.

#### Implementors

- [`EcdsaSigningAlgorithm`](../ec/suite_b/ecdsa/signing/index.md#ecdsasigningalgorithm)
- [`EcdsaVerificationAlgorithm`](../ec/suite_b/ecdsa/verification/index.md#ecdsaverificationalgorithm)
- [`EdDSAParameters`](../ec/curve25519/ed25519/verification/index.md#eddsaparameters)
- [`NonceRandom`](../ec/suite_b/ecdsa/signing/index.md#noncerandom)
- [`PKCS1`](../rsa/padding/pkcs1/index.md#pkcs1)
- [`PSS`](../rsa/padding/pss/index.md#pss)
- [`RsaParameters`](../rsa/index.md#rsaparameters)
- [`SystemRandom`](../rand/index.md#systemrandom)

