**rustls > crypto > hpke**

# Module: crypto::hpke

## Contents

**Structs**

- [`EncapsulatedSecret`](#encapsulatedsecret) - An encapsulated secret returned from setting up a sender or receiver context.
- [`HpkeKeyPair`](#hpkekeypair) - An HPKE key pair, made of a matching public and private key.
- [`HpkePrivateKey`](#hpkeprivatekey) - An HPKE private key.
- [`HpkePublicKey`](#hpkepublickey) - An HPKE public key.
- [`HpkeSuite`](#hpkesuite) - An HPKE suite, specifying a key encapsulation mechanism and a symmetric cipher suite.

**Traits**

- [`Hpke`](#hpke) - An HPKE instance that can be used for base-mode single-shot encryption and decryption.
- [`HpkeOpener`](#hpkeopener) - An HPKE opener context.
- [`HpkeSealer`](#hpkesealer) - An HPKE sealer context.

---

## rustls::crypto::hpke::EncapsulatedSecret

*Struct*

An encapsulated secret returned from setting up a sender or receiver context.

**Tuple Struct**: `(alloc::vec::Vec<u8>)`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## rustls::crypto::hpke::Hpke

*Trait*

An HPKE instance that can be used for base-mode single-shot encryption and decryption.

**Methods:**

- `seal`: Seal the provided `plaintext` to the recipient public key `pub_key` with application supplied
- `setup_sealer`: Set up a sealer context for the receiver public key `pub_key` with application supplied `info`.
- `open`: Open the provided `ciphertext` using the encapsulated secret `enc`, with application
- `setup_opener`: Set up an opener context for the secret key `secret_key` with application supplied `info`.
- `generate_key_pair`: Generate a new public key and private key pair compatible with this HPKE instance.
- `fips`: Return whether the HPKE instance is FIPS compatible.
- `suite`: Return the [HpkeSuite] that this HPKE instance supports.



## rustls::crypto::hpke::HpkeKeyPair

*Struct*

An HPKE key pair, made of a matching public and private key.

**Fields:**
- `public_key: HpkePublicKey` - A HPKE public key.
- `private_key: HpkePrivateKey` - A HPKE private key.



## rustls::crypto::hpke::HpkeOpener

*Trait*

An HPKE opener context.

This is a stateful object that can be used to open sealed messages sealed
by a sender.

**Methods:**

- `open`: Open the provided `ciphertext` with additional data `aad`, returning plaintext.



## rustls::crypto::hpke::HpkePrivateKey

*Struct*

An HPKE private key.

**Tuple Struct**: `()`

**Methods:**

- `fn secret_bytes(self: &Self) -> &[u8]` - Return the private key bytes.

**Trait Implementations:**

- **Drop**
  - `fn drop(self: & mut Self)`
- **From**
  - `fn from(bytes: Vec<u8>) -> Self`



## rustls::crypto::hpke::HpkePublicKey

*Struct*

An HPKE public key.

**Tuple Struct**: `(alloc::vec::Vec<u8>)`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> HpkePublicKey`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## rustls::crypto::hpke::HpkeSealer

*Trait*

An HPKE sealer context.

This is a stateful object that can be used to seal messages for receipt by
a receiver.

**Methods:**

- `seal`: Seal the provided `plaintext` with additional data `aad`, returning



## rustls::crypto::hpke::HpkeSuite

*Struct*

An HPKE suite, specifying a key encapsulation mechanism and a symmetric cipher suite.

**Fields:**
- `kem: crate::msgs::enums::HpkeKem` - The choice of HPKE key encapsulation mechanism.
- `sym: crate::msgs::handshake::HpkeSymmetricCipherSuite` - The choice of HPKE symmetric cipher suite.

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &HpkeSuite) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> HpkeSuite`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



