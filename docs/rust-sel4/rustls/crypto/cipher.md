**rustls > crypto > cipher**

# Module: crypto::cipher

## Contents

**Structs**

- [`AeadKey`](#aeadkey) - A key for an AEAD algorithm.
- [`Iv`](#iv) - A write or read IV.
- [`KeyBlockShape`](#keyblockshape) - How a TLS1.2 `key_block` is partitioned.
- [`Nonce`](#nonce) - A nonce.  This is unique for all messages on a connection.
- [`UnsupportedOperationError`](#unsupportedoperationerror) - An error indicating that the AEAD algorithm does not support the requested operation.

**Functions**

- [`make_tls12_aad`](#make_tls12_aad) - Returns a TLS1.2 `additional_data` encoding.
- [`make_tls13_aad`](#make_tls13_aad) - Returns a TLS1.3 `additional_data` encoding.

**Traits**

- [`MessageDecrypter`](#messagedecrypter) - Objects with this trait can decrypt TLS messages.
- [`MessageEncrypter`](#messageencrypter) - Objects with this trait can encrypt TLS messages.
- [`Tls12AeadAlgorithm`](#tls12aeadalgorithm) - Factory trait for building `MessageEncrypter` and `MessageDecrypter` for a TLS1.2 cipher suite.
- [`Tls13AeadAlgorithm`](#tls13aeadalgorithm) - Factory trait for building `MessageEncrypter` and `MessageDecrypter` for a TLS1.3 cipher suite.

**Constants**

- [`NONCE_LEN`](#nonce_len) - Size of TLS nonces (incorrectly termed "IV" in standard) for all supported ciphersuites

---

## rustls::crypto::cipher::AeadKey

*Struct*

A key for an AEAD algorithm.

This is a value type for a byte string up to `AeadKey::MAX_LEN` bytes in length.

**Trait Implementations:**

- **From**
  - `fn from(bytes: [u8; 32]) -> Self`
- **Drop**
  - `fn drop(self: & mut Self)`
- **AsRef**
  - `fn as_ref(self: &Self) -> &[u8]`



## rustls::crypto::cipher::Iv

*Struct*

A write or read IV.

**Tuple Struct**: `()`

**Methods:**

- `fn new(value: [u8; 12]) -> Self` - Create a new `Iv` from a byte array, of precisely `NONCE_LEN` bytes.
- `fn copy(value: &[u8]) -> Self` - Create a new `Iv` from a byte slice, of precisely `NONCE_LEN` bytes.

**Trait Implementations:**

- **AsRef**
  - `fn as_ref(self: &Self) -> &[u8]`
- **Default**
  - `fn default() -> Iv`
- **From**
  - `fn from(bytes: [u8; 12]) -> Self`



## rustls::crypto::cipher::KeyBlockShape

*Struct*

How a TLS1.2 `key_block` is partitioned.

Note: ciphersuites with non-zero `mac_key_length` are  not currently supported.

**Fields:**
- `enc_key_len: usize` - How long keys are.
- `fixed_iv_len: usize` - How long the fixed part of the 'IV' is.
- `explicit_nonce_len: usize` - This is a non-standard extension which extends the



## rustls::crypto::cipher::MessageDecrypter

*Trait*

Objects with this trait can decrypt TLS messages.

**Methods:**

- `decrypt`: Decrypt the given TLS message `msg`, using the sequence number



## rustls::crypto::cipher::MessageEncrypter

*Trait*

Objects with this trait can encrypt TLS messages.

**Methods:**

- `encrypt`: Encrypt the given TLS message `msg`, using the sequence number
- `encrypted_payload_len`: Return the length of the ciphertext that results from encrypting plaintext of



## rustls::crypto::cipher::NONCE_LEN

*Constant*: `usize`

Size of TLS nonces (incorrectly termed "IV" in standard) for all supported ciphersuites
(AES-GCM, Chacha20Poly1305)



## rustls::crypto::cipher::Nonce

*Struct*

A nonce.  This is unique for all messages on a connection.

**Tuple Struct**: `([u8; 12])`

**Methods:**

- `fn new(iv: &Iv, seq: u64) -> Self` - Combine an `Iv` and sequence number to produce a unique nonce.
- `fn for_path(path_id: u32, iv: &Iv, pn: u64) -> Self` - Creates a unique nonce based on the `iv`, the packet number `pn` and multipath `path_id`.



## rustls::crypto::cipher::Tls12AeadAlgorithm

*Trait*

Factory trait for building `MessageEncrypter` and `MessageDecrypter` for a TLS1.2 cipher suite.

**Methods:**

- `encrypter`: Build a `MessageEncrypter` for the given key/iv and extra key block (which can be used for
- `decrypter`: Build a `MessageDecrypter` for the given key/iv.
- `key_block_shape`: Return a `KeyBlockShape` that defines how large the `key_block` is and how it
- `extract_keys`: Convert the key material from `key`/`iv`, into a `ConnectionTrafficSecrets` item.
- `fips`: Return `true` if this is backed by a FIPS-approved implementation.



## rustls::crypto::cipher::Tls13AeadAlgorithm

*Trait*

Factory trait for building `MessageEncrypter` and `MessageDecrypter` for a TLS1.3 cipher suite.

**Methods:**

- `encrypter`: Build a `MessageEncrypter` for the given key/iv.
- `decrypter`: Build a `MessageDecrypter` for the given key/iv.
- `key_len`: The length of key in bytes required by `encrypter()` and `decrypter()`.
- `extract_keys`: Convert the key material from `key`/`iv`, into a `ConnectionTrafficSecrets` item.
- `fips`: Return `true` if this is backed by a FIPS-approved implementation.



## rustls::crypto::cipher::UnsupportedOperationError

*Struct*

An error indicating that the AEAD algorithm does not support the requested operation.

**Unit Struct**

**Traits:** Eq, Copy

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &UnsupportedOperationError) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> UnsupportedOperationError`



## rustls::crypto::cipher::make_tls12_aad

*Function*

Returns a TLS1.2 `additional_data` encoding.

See RFC5246 s6.2.3.3 for the `additional_data` definition.

```rust
fn make_tls12_aad(seq: u64, typ: crate::enums::ContentType, vers: crate::enums::ProtocolVersion, len: usize) -> [u8; 13]
```



## rustls::crypto::cipher::make_tls13_aad

*Function*

Returns a TLS1.3 `additional_data` encoding.

See RFC8446 s5.2 for the `additional_data` definition.

```rust
fn make_tls13_aad(payload_len: usize) -> [u8; 5]
```



