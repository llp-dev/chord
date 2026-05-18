*[ring](../../index.md) / [rsa](../index.md) / [keypair](index.md)*

---

# Module `keypair`

## Contents

- [Structs](#structs)
  - [`KeyPair`](#keypair)
  - [`PrivatePrime`](#privateprime)
  - [`PrivateCrtPrime`](#privatecrtprime)
- [Enums](#enums)
  - [`P`](#p)
  - [`Q`](#q)
  - [`D`](#d)
- [Functions](#functions)
  - [`elem_exp_consttime`](#elem-exp-consttime)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`KeyPair`](#keypair) | struct | An RSA key pair, used for signing. |
| [`PrivatePrime`](#privateprime) | struct |  |
| [`PrivateCrtPrime`](#privatecrtprime) | struct |  |
| [`P`](#p) | enum |  |
| [`Q`](#q) | enum |  |
| [`D`](#d) | enum |  |
| [`elem_exp_consttime`](#elem-exp-consttime) | fn |  |

## Structs

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

- <span id="keypair-from-pkcs8"></span>`fn from_pkcs8(pkcs8: &[u8]) -> Result<Self, KeyRejected>` — [`KeyRejected`](../../error/index.md#keyrejected)

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

- <span id="keypair-from-der"></span>`fn from_der(input: &[u8]) -> Result<Self, KeyRejected>` — [`KeyRejected`](../../error/index.md#keyrejected)

  Parses an RSA private key that is not inside a PKCS#8 wrapper.

  

  The private key must be encoded as a binary DER-encoded ASN.1

  `RSAPrivateKey` as described in [RFC 3447 Appendix A.1.2]). In all other

  respects, this is just like `from_pkcs8()`. See the documentation for

  `from_pkcs8()` for more details.

  

  It is recommended to use `from_pkcs8()` (with a PKCS#8-encoded key)

  instead.

  

  See `Self::from_components()` for more details on how the input is

  validated.

- <span id="keypair-from-der-reader"></span>`fn from_der_reader(input: &mut untrusted::Reader<'_>) -> Result<Self, KeyRejected>` — [`KeyRejected`](../../error/index.md#keyrejected)

- <span id="keypair-from-components"></span>`fn from_components<Public, Private>(components: &KeyPairComponents<Public, Private>) -> Result<Self, KeyRejected>` — [`KeyPairComponents`](../keypair_components/index.md#keypaircomponents), [`KeyRejected`](../../error/index.md#keyrejected)

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

- <span id="keypair-from-components"></span>`fn from_components_(_: &KeyPairComponents<&[u8]>, cpu_features: cpu::Features) -> Result<Self, KeyRejected>` — [`KeyPairComponents`](../keypair_components/index.md#keypaircomponents), [`Features`](../../cpu/index.md#features), [`KeyRejected`](../../error/index.md#keyrejected)

- <span id="keypair-public"></span>`fn public(&self) -> &PublicKey` — [`PublicKey`](../public_key/index.md#publickey)

  Returns a reference to the public key.

- <span id="keypair-public-modulus-len"></span>`fn public_modulus_len(&self) -> usize`

  Returns the length in bytes of the key pair's public modulus.

  

  A signature has the same length as the public modulus.

#### Trait Implementations

##### `impl Debug for KeyPair`

- <span id="keypair-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> Result<(), ::core::fmt::Error>`

##### `impl KeyPair for KeyPair`

- <span id="keypair-keypair-type-publickey"></span>`type PublicKey = PublicKey`

- <span id="keypair-keypair-public-key"></span>`fn public_key(&self) -> &<Self as >::PublicKey` — [`KeyPair`](../../signature/index.md#keypair)

### `PrivatePrime<M>`

```rust
struct PrivatePrime<M> {
    modulus: self::modulus::OwnedModulus<M>,
    oneRR: bigint::One<M, crate::arithmetic::montgomery::RR>,
}
```

#### Implementations

- <span id="privateprime-new"></span>`fn new(p: untrusted::Input<'_>, n_bits: BitLength, cpu_features: cpu::Features) -> Result<Self, KeyRejected>` — [`BitLength`](../../bits/index.md#bitlength), [`Features`](../../cpu/index.md#features), [`KeyRejected`](../../error/index.md#keyrejected)

### `PrivateCrtPrime<M>`

```rust
struct PrivateCrtPrime<M> {
    modulus: self::modulus::OwnedModulus<M>,
    oneRRR: bigint::One<M, crate::arithmetic::montgomery::RRR>,
    exponent: self::private_exponent::PrivateExponent,
}
```

#### Implementations

- <span id="privatecrtprime-new"></span>`fn new(p: PrivatePrime<M>, dP: untrusted::Input<'_>, cpu_features: cpu::Features) -> Result<Self, KeyRejected>` — [`PrivatePrime`](#privateprime), [`Features`](../../cpu/index.md#features), [`KeyRejected`](../../error/index.md#keyrejected)

  Constructs a `PrivateCrtPrime` from the private prime `p` and `dP` where

  dP == d % (p - 1).

## Enums

### `P`

```rust
enum P {
}
```

### `Q`

```rust
enum Q {
}
```

### `D`

```rust
enum D {
}
```

## Functions

### `elem_exp_consttime`

```rust
fn elem_exp_consttime<M>(c: &bigint::Elem<super::N>, p: &PrivateCrtPrime<M>, other_prime_len_bits: crate::bits::BitLength, cpu_features: cpu::Features) -> Result<bigint::Elem<M>, error::Unspecified>
```

