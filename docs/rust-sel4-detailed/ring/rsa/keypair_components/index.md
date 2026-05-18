*[ring](../../index.md) / [rsa](../index.md) / [keypair_components](index.md)*

---

# Module `keypair_components`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`KeyPairComponents`](#keypaircomponents) | struct | RSA key pair components. |

## Structs

### `KeyPairComponents<Public, Private>`

```rust
struct KeyPairComponents<Public, Private> {
    pub public_key: super::PublicKeyComponents<Public>,
    pub d: Private,
    pub p: Private,
    pub q: Private,
    pub dP: Private,
    pub dQ: Private,
    pub qInv: Private,
}
```

RSA key pair components.

#### Fields

- **`public_key`**: `super::PublicKeyComponents<Public>`

  The public key components.

- **`d`**: `Private`

  The private exponent.

- **`p`**: `Private`

  The first prime factor of `d`.

- **`q`**: `Private`

  The second prime factor of `d`.

- **`dP`**: `Private`

  `p`'s public Chinese Remainder Theorem exponent.

- **`dQ`**: `Private`

  `q`'s public Chinese Remainder Theorem exponent.

- **`qInv`**: `Private`

  `q**-1 mod p`.

#### Trait Implementations

##### `impl<Public: clone::Clone, Private: clone::Clone> Clone for KeyPairComponents<Public, Private>`

- <span id="keypaircomponents-clone"></span>`fn clone(&self) -> KeyPairComponents<Public, Private>` — [`KeyPairComponents`](#keypaircomponents)

##### `impl<Public: marker::Copy, Private: marker::Copy> Copy for KeyPairComponents<Public, Private>`

##### `impl<Public, Private> Debug for KeyPairComponents<Public, Private>`

- <span id="keypaircomponents-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error>`

