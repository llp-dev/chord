*[ring](../../index.md) / [rsa](../index.md) / [public_key_components](index.md)*

---

# Module `public_key_components`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`PublicKeyComponents`](#publickeycomponents) | struct | RSA public key components. |

## Structs

### `PublicKeyComponents<B>`

```rust
struct PublicKeyComponents<B> {
    pub n: B,
    pub e: B,
}
```

RSA public key components.

`B` must implement `AsRef<[u8]>` like `&[u8]` or `Vec<u8>`.

#### Fields

- **`n`**: `B`

  The public modulus, encoded in big-endian bytes without leading zeros.

- **`e`**: `B`

  The public exponent, encoded in big-endian bytes without leading zeros.

#### Implementations

- <span id="superpublickeycomponents-verify"></span>`fn verify(&self, params: &RsaParameters, message: &[u8], signature: &[u8]) -> Result<(), error::Unspecified>` — [`RsaParameters`](../index.md#rsaparameters), [`Unspecified`](../../error/index.md#unspecified)

  Verifies that `signature` is a valid signature of `message` using `self`

  as the public key. `params` determine what algorithm parameters

  (padding, digest algorithm, key length range, etc.) are used in the

  verification.

  

  When the public key is in DER-encoded PKCS#1 ASN.1 format, it is

  recommended to use `ring::signature::verify()` with

  `ring::signature::RSA_PKCS1_*`, because `ring::signature::verify()`

  will handle the parsing in that case. Otherwise, this function can be used

  to pass in the raw bytes for the public key components as

  `untrusted::Input` arguments.

#### Trait Implementations

##### `impl<B: clone::Clone> Clone for PublicKeyComponents<B>`

- <span id="publickeycomponents-clone"></span>`fn clone(&self) -> PublicKeyComponents<B>` — [`PublicKeyComponents`](#publickeycomponents)

##### `impl<B: marker::Copy> Copy for PublicKeyComponents<B>`

##### `impl<B> Debug for PublicKeyComponents<B>`

- <span id="publickeycomponents-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error>`

