**rustls > tls12**

# Module: tls12

## Contents

**Structs**

- [`Tls12CipherSuite`](#tls12ciphersuite) - A TLS 1.2 cipher suite supported by rustls.

---

## rustls::tls12::Tls12CipherSuite

*Struct*

A TLS 1.2 cipher suite supported by rustls.

**Fields:**
- `common: crate::suites::CipherSuiteCommon` - Common cipher suite fields.
- `prf_provider: &'static dyn crypto::tls12::Prf` - How to compute the TLS1.2 PRF for the suite's hash function.
- `kx: crate::msgs::handshake::KeyExchangeAlgorithm` - How to exchange/agree keys.
- `sign: &'static [crate::enums::SignatureScheme]` - How to sign messages for authentication.
- `aead_alg: &'static dyn Tls12AeadAlgorithm` - How to produce a [`MessageDecrypter`] or [`MessageEncrypter`]

**Methods:**

- `fn resolve_sig_schemes(self: &Self, offered: &[SignatureScheme]) -> Vec<SignatureScheme>` - Resolve the set of supported [`SignatureScheme`]s from the
- `fn fips(self: &Self) -> bool` - Return `true` if this is backed by a FIPS-approved implementation.

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



