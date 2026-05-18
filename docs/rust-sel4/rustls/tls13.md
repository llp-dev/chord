**rustls > tls13**

# Module: tls13

## Contents

**Structs**

- [`Tls13CipherSuite`](#tls13ciphersuite) - A TLS 1.3 cipher suite supported by rustls.

---

## rustls::tls13::Tls13CipherSuite

*Struct*

A TLS 1.3 cipher suite supported by rustls.

**Fields:**
- `common: crate::suites::CipherSuiteCommon` - Common cipher suite fields.
- `hkdf_provider: &'static dyn crypto::tls13::Hkdf` - How to complete HKDF with the suite's hash function.
- `aead_alg: &'static dyn crypto::cipher::Tls13AeadAlgorithm` - How to produce a [MessageDecrypter] or [MessageEncrypter]
- `quic: Option<&'static dyn crate::quic::Algorithm>` - How to create QUIC header and record protection algorithms

**Methods:**

- `fn can_resume_from(self: &Self, prev: &'static Self) -> Option<&'static Self>` - Can a session using suite self resume from suite prev?
- `fn fips(self: &Self) -> bool` - Return `true` if this is backed by a FIPS-approved implementation.
- `fn quic_suite(self: &'static Self) -> Option<crate::quic::Suite>` - Returns a `quic::Suite` for the ciphersuite, if supported.

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



