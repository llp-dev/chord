*[ring](../index.md) / [rsa](index.md)*

---

# Module `rsa`

RSA.

## Contents

- [Modules](#modules)
  - [`padding`](#padding)
  - [`keypair`](#keypair)
  - [`keypair_components`](#keypair-components)
  - [`public_exponent`](#public-exponent)
  - [`public_key`](#public-key)
  - [`public_key_components`](#public-key-components)
  - [`public_modulus`](#public-modulus)
  - [`verification`](#verification)
- [Structs](#structs)
  - [`RsaParameters`](#rsaparameters)
  - [`KeyPair`](#keypair)
  - [`KeyPairComponents`](#keypaircomponents)
  - [`PublicKey`](#publickey)
  - [`PublicKeyComponents`](#publickeycomponents)
- [Enums](#enums)
  - [`N`](#n)
- [Functions](#functions)
  - [`parse_public_key`](#parse-public-key)
- [Constants](#constants)
  - [`PUBLIC_KEY_PUBLIC_MODULUS_MAX_LEN`](#public-key-public-modulus-max-len)
  - [`PRIVATE_KEY_PUBLIC_MODULUS_MAX_BITS`](#private-key-public-modulus-max-bits)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`padding`](#padding) | mod |  |
| [`keypair`](#keypair) | mod |  |
| [`keypair_components`](#keypair-components) | mod |  |
| [`public_exponent`](#public-exponent) | mod |  |
| [`public_key`](#public-key) | mod |  |
| [`public_key_components`](#public-key-components) | mod |  |
| [`public_modulus`](#public-modulus) | mod |  |
| [`verification`](#verification) | mod | Verification of RSA signatures. |
| [`RsaParameters`](#rsaparameters) | struct | Parameters for RSA verification. |
| [`KeyPair`](#keypair) | struct |  |
| [`KeyPairComponents`](#keypaircomponents) | struct |  |
| [`PublicKey`](#publickey) | struct |  |
| [`PublicKeyComponents`](#publickeycomponents) | struct |  |
| [`N`](#n) | enum |  |
| [`parse_public_key`](#parse-public-key) | fn |  |
| [`PUBLIC_KEY_PUBLIC_MODULUS_MAX_LEN`](#public-key-public-modulus-max-len) | const |  |
| [`PRIVATE_KEY_PUBLIC_MODULUS_MAX_BITS`](#private-key-public-modulus-max-bits) | const |  |

## Modules

- [`padding`](padding/index.md)
- [`keypair`](keypair/index.md)
- [`keypair_components`](keypair_components/index.md)
- [`public_exponent`](public_exponent/index.md)
- [`public_key`](public_key/index.md)
- [`public_key_components`](public_key_components/index.md)
- [`public_modulus`](public_modulus/index.md)
- [`verification`](verification/index.md) — Verification of RSA signatures.

## Structs

### `RsaParameters`

```rust
struct RsaParameters {
    padding_alg: &'static dyn padding::Verification,
    min_bits: bits::BitLength,
}
```

Parameters for RSA verification.

#### Trait Implementations

##### `impl Debug for RsaParameters`

- <span id="rsaparameters-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Sealed for super::RsaParameters`

##### `impl VerificationAlgorithm for super::RsaParameters`

- <span id="superrsaparameters-verificationalgorithm-verify"></span>`fn verify(&self, public_key: untrusted::Input<'_>, msg: untrusted::Input<'_>, signature: untrusted::Input<'_>) -> Result<(), error::Unspecified>` — [`Unspecified`](../error/index.md#unspecified)

### `KeyPair`

```rust
struct KeyPair {
    p: PrivateCrtPrime<P>,
    q: PrivateCrtPrime<Q>,
    qInv: bigint::Elem<P, crate::arithmetic::montgomery::R>,
    public: super::PublicKey,
}
```

An RSA key pair, used for signing.

#### Implementations

- <span id="keypair-from-pkcs8"></span>`fn from_pkcs8(pkcs8: &[u8]) -> Result<Self, KeyRejected>` — [`KeyRejected`](../error/index.md#keyrejected)

  Parses an unencrypted PKCS#8-encoded RSA private key.

  

  This will generate a 2048-bit RSA private key of the correct form using

  OpenSSL's command line tool:

  

  ```sh

     openssl genpkey -algorithm RSA \

         -pkeyopt rsa_keygen_bits:2048 \

         -pkeyopt rsa_keygen_pubexp:65537 | \

       openssl pkcs8 -topk8 -nocrypt -outform der > rsa-2048-private-key.pk8

  ```

  

  This will generate a 3072-bit RSA private key of the correct form:

  

  ```sh

     openssl genpkey -algorithm RSA \

         -pkeyopt rsa_keygen_bits:3072 \

         -pkeyopt rsa_keygen_pubexp:65537 | \

       openssl pkcs8 -topk8 -nocrypt -outform der > rsa-3072-private-key.pk8

  ```

  

  Often, keys generated for use in OpenSSL-based software are stored in

  the Base64 “PEM” format without the PKCS#8 wrapper. Such keys can be

  converted to binary PKCS#8 form using the OpenSSL command line tool like

  this:

  

  ```sh

  openssl pkcs8 -topk8 -nocrypt -outform der \

      -in rsa-2048-private-key.pem > rsa-2048-private-key.pk8

  ```

  

  Base64 (“PEM”) PKCS#8-encoded keys can be converted to the binary PKCS#8

  form like this:

  

  ```sh

  openssl pkcs8 -nocrypt -outform der \

      -in rsa-2048-private-key.pem > rsa-2048-private-key.pk8

  ```

  

  See `Self::from_components` for more details on how the input is

  validated.

  

  See [RFC 5958] and [RFC 3447 Appendix A.1.2] for more details of the

  encoding of the key.

- <span id="keypair-from-der"></span>`fn from_der(input: &[u8]) -> Result<Self, KeyRejected>` — [`KeyRejected`](../error/index.md#keyrejected)

  Parses an RSA private key that is not inside a PKCS#8 wrapper.

  

  The private key must be encoded as a binary DER-encoded ASN.1

  `RSAPrivateKey` as described in [RFC 3447 Appendix A.1.2]). In all other

  respects, this is just like `from_pkcs8()`. See the documentation for

  `from_pkcs8()` for more details.

  

  It is recommended to use `from_pkcs8()` (with a PKCS#8-encoded key)

  instead.

  

  See `Self::from_components()` for more details on how the input is

  validated.

- <span id="keypair-from-der-reader"></span>`fn from_der_reader(input: &mut untrusted::Reader<'_>) -> Result<Self, KeyRejected>` — [`KeyRejected`](../error/index.md#keyrejected)

- <span id="keypair-from-components"></span>`fn from_components<Public, Private>(components: &KeyPairComponents<Public, Private>) -> Result<Self, KeyRejected>` — [`KeyPairComponents`](keypair_components/index.md#keypaircomponents), [`KeyRejected`](../error/index.md#keyrejected)

  Constructs an RSA private key from its big-endian-encoded components.

  

  Only two-prime (not multi-prime) keys are supported. The public modulus

  (n) must be at least 2047 bits. The public modulus must be no larger

  than 4096 bits. It is recommended that the public modulus be exactly

  2048 or 3072 bits. The public exponent must be at least 65537 and must

  be no more than 33 bits long.

  

  The private key is validated according to [NIST SP-800-56B rev. 1]

  section 6.4.1.4.3, crt_pkv (Intended Exponent-Creation Method Unknown),

  with the following exceptions:

  

  * Section 6.4.1.2.1, Step 1: Neither a target security level nor an

    expected modulus length is provided as a parameter, so checks

    regarding these expectations are not done.

  * Section 6.4.1.2.1, Step 3: Since neither the public key nor the

    expected modulus length is provided as a parameter, the consistency

    check between these values and the private key's value of n isn't

    done.

  * Section 6.4.1.2.1, Step 5: No primality tests are done, both for

    performance reasons and to avoid any side channels that such tests

    would provide.

  * Section 6.4.1.2.1, Step 6, and 6.4.1.4.3, Step 7:

      * *ring* has a slightly looser lower bound for the values of `p`

      and `q` than what the NIST document specifies. This looser lower

      bound matches what most other crypto libraries do. The check might

      be tightened to meet NIST's requirements in the future. Similarly,

      the check that `p` and `q` are not too close together is skipped

      currently, but may be added in the future.

      - The validity of the mathematical relationship of `dP`, `dQ`, `e`

      and `n` is verified only during signing. Some size checks of `d`,

      `dP` and `dQ` are performed at construction, but some NIST checks

      are skipped because they would be expensive and/or they would leak

      information through side channels. If a preemptive check of the

      consistency of `dP`, `dQ`, `e` and `n` with each other is

      necessary, that can be done by signing any message with the key

      pair.

  

      * `d` is not fully validated, neither at construction nor during

      signing. This is OK as far as *ring*'s usage of the key is

      concerned because *ring* never uses the value of `d` (*ring* always

      uses `p`, `q`, `dP` and `dQ` via the Chinese Remainder Theorem,

      instead). However, *ring*'s checks would not be sufficient for

      validating a key pair for use by some other system; that other

      system must check the value of `d` itself if `d` is to be used.

- <span id="keypair-from-components"></span>`fn from_components_(_: &KeyPairComponents<&[u8]>, cpu_features: cpu::Features) -> Result<Self, KeyRejected>` — [`KeyPairComponents`](keypair_components/index.md#keypaircomponents), [`Features`](../cpu/index.md#features), [`KeyRejected`](../error/index.md#keyrejected)

- <span id="keypair-public"></span>`fn public(&self) -> &PublicKey` — [`PublicKey`](public_key/index.md#publickey)

  Returns a reference to the public key.

- <span id="keypair-public-modulus-len"></span>`fn public_modulus_len(&self) -> usize`

  Returns the length in bytes of the key pair's public modulus.

  

  A signature has the same length as the public modulus.

#### Trait Implementations

##### `impl Debug for KeyPair`

- <span id="keypair-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> Result<(), ::core::fmt::Error>`

##### `impl KeyPair for KeyPair`

- <span id="keypair-keypair-type-publickey"></span>`type PublicKey = PublicKey`

- <span id="keypair-keypair-public-key"></span>`fn public_key(&self) -> &<Self as >::PublicKey` — [`KeyPair`](../signature/index.md#keypair)

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

- <span id="keypaircomponents-clone"></span>`fn clone(&self) -> KeyPairComponents<Public, Private>` — [`KeyPairComponents`](keypair_components/index.md#keypaircomponents)

##### `impl<Public: marker::Copy, Private: marker::Copy> Copy for KeyPairComponents<Public, Private>`

##### `impl<Public, Private> Debug for KeyPairComponents<Public, Private>`

- <span id="keypaircomponents-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error>`

### `PublicKey`

```rust
struct PublicKey {
    inner: Inner,
    serialized: alloc::boxed::Box<[u8]>,
}
```

An RSA Public Key.

#### Implementations

- <span id="publickey-from-modulus-and-exponent"></span>`fn from_modulus_and_exponent(n: untrusted::Input<'_>, e: untrusted::Input<'_>, n_min_bits: bits::BitLength, n_max_bits: bits::BitLength, e_min_value: PublicExponent, cpu_features: cpu::Features) -> Result<Self, error::KeyRejected>` — [`BitLength`](../bits/index.md#bitlength), [`PublicExponent`](public_exponent/index.md#publicexponent), [`Features`](../cpu/index.md#features), [`KeyRejected`](../error/index.md#keyrejected)

- <span id="publickey-modulus-len"></span>`fn modulus_len(&self) -> usize`

  The length, in bytes, of the public modulus.

  

  The modulus length is rounded up to a whole number of bytes if its

  bit length isn't a multiple of 8.

- <span id="publickey-inner"></span>`fn inner(&self) -> &Inner` — [`Inner`](public_key/index.md#inner)

#### Trait Implementations

##### `impl AsRef for PublicKey`

- <span id="publickey-asref-as-ref"></span>`fn as_ref(&self) -> &[u8]`

##### `impl Clone for PublicKey`

- <span id="publickey-clone"></span>`fn clone(&self) -> PublicKey` — [`PublicKey`](public_key/index.md#publickey)

##### `impl Debug for PublicKey`

- <span id="publickey-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> Result<(), ::core::fmt::Error>`

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

- <span id="superpublickeycomponents-verify"></span>`fn verify(&self, params: &RsaParameters, message: &[u8], signature: &[u8]) -> Result<(), error::Unspecified>` — [`RsaParameters`](#rsaparameters), [`Unspecified`](../error/index.md#unspecified)

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

- <span id="publickeycomponents-clone"></span>`fn clone(&self) -> PublicKeyComponents<B>` — [`PublicKeyComponents`](public_key_components/index.md#publickeycomponents)

##### `impl<B: marker::Copy> Copy for PublicKeyComponents<B>`

##### `impl<B> Debug for PublicKeyComponents<B>`

- <span id="publickeycomponents-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error>`

## Enums

### `N`

```rust
enum N {
}
```

#### Trait Implementations

##### `impl PublicModulus for N`

## Functions

### `parse_public_key`

```rust
fn parse_public_key(input: untrusted::Input<'_>) -> Result<(io::Positive<'_>, io::Positive<'_>), error::Unspecified>
```

## Constants

### `PUBLIC_KEY_PUBLIC_MODULUS_MAX_LEN`
```rust
const PUBLIC_KEY_PUBLIC_MODULUS_MAX_LEN: usize = 1_024usize;
```

### `PRIVATE_KEY_PUBLIC_MODULUS_MAX_BITS`
```rust
const PRIVATE_KEY_PUBLIC_MODULUS_MAX_BITS: bits::BitLength;
```

