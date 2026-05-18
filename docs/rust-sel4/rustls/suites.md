**rustls > suites**

# Module: suites

## Contents

**Structs**

- [`CipherSuiteCommon`](#ciphersuitecommon) - Common state for cipher suites (both for TLS 1.2 and TLS 1.3)
- [`ExtractedSecrets`](#extractedsecrets) - Secrets for transmitting/receiving data over a TLS session.

**Enums**

- [`ConnectionTrafficSecrets`](#connectiontrafficsecrets) - Secrets used to encrypt/decrypt data in a TLS session.
- [`SupportedCipherSuite`](#supportedciphersuite) - A cipher suite supported by rustls.

---

## rustls::suites::CipherSuiteCommon

*Struct*

Common state for cipher suites (both for TLS 1.2 and TLS 1.3)

**Fields:**
- `suite: crate::enums::CipherSuite` - The TLS enumeration naming this cipher suite.
- `hash_provider: &'static dyn crypto::hash::Hash` - Which hash function the suite uses.
- `confidentiality_limit: u64` - Number of TCP-TLS messages that can be safely encrypted with a single key of this type

**Methods:**

- `fn fips(self: &Self) -> bool` - Return `true` if this is backed by a FIPS-approved implementation.



## rustls::suites::ConnectionTrafficSecrets

*Enum*

Secrets used to encrypt/decrypt data in a TLS session.

These can be used to configure kTLS for a socket in one direction.
The only other piece of information needed is the sequence number,
which is in [ExtractedSecrets].

**Variants:**
- `Aes128Gcm{ key: crate::crypto::cipher::AeadKey, iv: crate::crypto::cipher::Iv }` - Secrets for the AES_128_GCM AEAD algorithm
- `Aes256Gcm{ key: crate::crypto::cipher::AeadKey, iv: crate::crypto::cipher::Iv }` - Secrets for the AES_256_GCM AEAD algorithm
- `Chacha20Poly1305{ key: crate::crypto::cipher::AeadKey, iv: crate::crypto::cipher::Iv }` - Secrets for the CHACHA20_POLY1305 AEAD algorithm



## rustls::suites::ExtractedSecrets

*Struct*

Secrets for transmitting/receiving data over a TLS session.

After performing a handshake with rustls, these secrets can be extracted
to configure kTLS for a socket, and have the kernel take over encryption
and/or decryption.

**Fields:**
- `tx: (u64, ConnectionTrafficSecrets)` - sequence number and secrets for the "tx" (transmit) direction
- `rx: (u64, ConnectionTrafficSecrets)` - sequence number and secrets for the "rx" (receive) direction



## rustls::suites::SupportedCipherSuite

*Enum*

A cipher suite supported by rustls.

This type carries both configuration and implementation. Compare with
[`CipherSuite`], which carries solely a cipher suite identifier.

**Variants:**
- `Tls12(&'static crate::tls12::Tls12CipherSuite)` - A TLS 1.2 cipher suite
- `Tls13(&'static crate::tls13::Tls13CipherSuite)` - A TLS 1.3 cipher suite

**Methods:**

- `fn suite(self: &Self) -> CipherSuite` - The cipher suite's identifier
- `fn tls13(self: &Self) -> Option<&'static Tls13CipherSuite>` - Return the inner `Tls13CipherSuite` for this suite, if it is a TLS1.3 suite.
- `fn version(self: &Self) -> &'static SupportedProtocolVersion` - Return supported protocol version for the cipher suite.
- `fn usable_for_signature_algorithm(self: &Self, _sig_alg: SignatureAlgorithm) -> bool` - Return true if this suite is usable for a key only offering `sig_alg`
- `fn fips(self: &Self) -> bool` - Return `true` if this is backed by a FIPS-approved implementation.

**Traits:** Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &SupportedCipherSuite) -> bool`
- **From**
  - `fn from(s: &'static Tls13CipherSuite) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> SupportedCipherSuite`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **From**
  - `fn from(s: &'static Tls12CipherSuite) -> Self`



