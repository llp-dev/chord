*[ring](../index.md) / [agreement](index.md)*

---

# Module `agreement`

Key Agreement: ECDH, including X25519.

# Example

Note that this example uses X25519, but ECDH using NIST P-256/P-384 is done
exactly the same way, just substituting
`agreement::ECDH_P256`/`agreement::ECDH_P384` for `agreement::X25519`.

```rust
use ring::{agreement, rand};

let rng = rand::SystemRandom::new();

let my_private_key = agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng)?;

// Make `my_public_key` a byte slice containing my public key. In a real
// application, this would be sent to the peer in an encoded protocol
// message.
let my_public_key = my_private_key.compute_public_key()?;

let peer_public_key_bytes = {
    // In a real application, the peer public key would be parsed out of a
    // protocol message. Here we just generate one.
    let peer_private_key =
        agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng)?;
    peer_private_key.compute_public_key()?
};

let peer_public_key = agreement::UnparsedPublicKey::new(
    &agreement::X25519,
    peer_public_key_bytes);

agreement::agree_ephemeral(
    my_private_key,
    &peer_public_key,
    |_key_material| {
        // In a real application, we'd apply a KDF to the key material and the
        // public keys (as recommended in RFC 7748) and then derive session
        // keys from the result. We omit all that here.
    },
)?;

Ok::<(), ring::error::Unspecified>(())
```

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Algorithm`](#algorithm) | struct | A key agreement algorithm. |
| [`EphemeralPrivateKey`](#ephemeralprivatekey) | struct | An ephemeral private key for use (only) with `agree_ephemeral`. |
| [`PublicKey`](#publickey) | struct | A public key for key agreement. |
| [`UnparsedPublicKey`](#unparsedpublickey) | struct | An unparsed, possibly malformed, public key for key agreement. |
| [`agree_ephemeral`](#agree-ephemeral) | fn | Performs a key agreement with an ephemeral private key and the given public key. |
| [`agree_ephemeral_`](#agree-ephemeral) | fn |  |

## Structs

### `Algorithm`

```rust
struct Algorithm {
    curve: &'static ec::Curve,
    ecdh: fn(&mut [u8], &ec::Seed, untrusted::Input<'_>) -> Result<(), error::Unspecified>,
}
```

A key agreement algorithm.

#### Trait Implementations

##### `impl Debug for Algorithm`

- <span id="algorithm-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> Result<(), ::core::fmt::Error>`

##### `impl Eq for Algorithm`

##### `impl PartialEq for Algorithm`

- <span id="algorithm-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

### `EphemeralPrivateKey`

```rust
struct EphemeralPrivateKey {
    private_key: ec::Seed,
    algorithm: &'static Algorithm,
}
```

An ephemeral private key for use (only) with `agree_ephemeral`. The
signature of `agree_ephemeral` ensures that an `EphemeralPrivateKey` can be
used for at most one key agreement.

#### Implementations

- <span id="ephemeralprivatekey-generate"></span>`fn generate(alg: &'static Algorithm, rng: &dyn rand::SecureRandom) -> Result<Self, error::Unspecified>` — [`Algorithm`](#algorithm), [`SecureRandom`](../rand/index.md#securerandom), [`Unspecified`](../error/index.md#unspecified)

  Generate a new ephemeral private key for the given algorithm.

- <span id="ephemeralprivatekey-compute-public-key"></span>`fn compute_public_key(&self) -> Result<PublicKey, error::Unspecified>` — [`PublicKey`](#publickey), [`Unspecified`](../error/index.md#unspecified)

  Computes the public key from the private key.

- <span id="ephemeralprivatekey-algorithm"></span>`fn algorithm(&self) -> &'static Algorithm` — [`Algorithm`](#algorithm)

  The algorithm for the private key.

#### Trait Implementations

##### `impl Debug for EphemeralPrivateKey`

- <span id="ephemeralprivatekey-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> Result<(), ::core::fmt::Error>`

### `PublicKey`

```rust
struct PublicKey {
    algorithm: &'static Algorithm,
    bytes: ec::PublicKey,
}
```

A public key for key agreement.

#### Implementations

- <span id="publickey-algorithm"></span>`fn algorithm(&self) -> &'static Algorithm` — [`Algorithm`](#algorithm)

  The algorithm for the public key.

#### Trait Implementations

##### `impl AsRef for PublicKey`

- <span id="publickey-asref-as-ref"></span>`fn as_ref(&self) -> &[u8]`

##### `impl Clone for PublicKey`

- <span id="publickey-clone"></span>`fn clone(&self) -> PublicKey` — [`PublicKey`](#publickey)

##### `impl Debug for PublicKey`

- <span id="publickey-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error>`

### `UnparsedPublicKey<B>`

```rust
struct UnparsedPublicKey<B> {
    algorithm: &'static Algorithm,
    bytes: B,
}
```

An unparsed, possibly malformed, public key for key agreement.

#### Implementations

- <span id="unparsedpublickey-new"></span>`fn new(algorithm: &'static Algorithm, bytes: B) -> Self` — [`Algorithm`](#algorithm)

  Constructs a new `UnparsedPublicKey`.

- <span id="unparsedpublickey-algorithm"></span>`fn algorithm(&self) -> &'static Algorithm` — [`Algorithm`](#algorithm)

  The algorithm for the public key.

- <span id="unparsedpublickey-bytes"></span>`fn bytes(&self) -> &B`

  TODO: doc

#### Trait Implementations

##### `impl<B> AsRef for UnparsedPublicKey<B>`

- <span id="unparsedpublickey-asref-as-ref"></span>`fn as_ref(&self) -> &[u8]`

##### `impl<B: clone::Clone> Clone for UnparsedPublicKey<B>`

- <span id="unparsedpublickey-clone"></span>`fn clone(&self) -> UnparsedPublicKey<B>` — [`UnparsedPublicKey`](#unparsedpublickey)

##### `impl<B: marker::Copy> Copy for UnparsedPublicKey<B>`

##### `impl<B> Debug for UnparsedPublicKey<B>`

- <span id="unparsedpublickey-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error>`

## Functions

### `agree_ephemeral`

```rust
fn agree_ephemeral<B: AsRef<[u8]>, R>(my_private_key: EphemeralPrivateKey, peer_public_key: &UnparsedPublicKey<B>, kdf: impl FnOnce(&[u8]) -> R) -> Result<R, error::Unspecified>
```

Performs a key agreement with an ephemeral private key and the given public
key.

`my_private_key` is the ephemeral private key to use. Since it is moved, it
will not be usable after calling `agree_ephemeral`, thus guaranteeing that
the key is used for only one key agreement.

`peer_public_key` is the peer's public key. `agree_ephemeral` will return
`Err(error_value)` if it does not match `my_private_key's` algorithm/curve.
`agree_ephemeral` verifies that it is encoded in the standard form for the
algorithm and that the key is *valid*; see the algorithm's documentation for
details on how keys are to be encoded and what constitutes a valid key for
that algorithm.

After the key agreement is done, `agree_ephemeral` calls `kdf` with the raw
key material from the key agreement operation and then returns what `kdf`
returns.

### `agree_ephemeral_`

```rust
fn agree_ephemeral_<R>(my_private_key: EphemeralPrivateKey, peer_public_key: UnparsedPublicKey<&[u8]>, kdf: impl FnOnce(&[u8]) -> R) -> Result<R, error::Unspecified>
```

