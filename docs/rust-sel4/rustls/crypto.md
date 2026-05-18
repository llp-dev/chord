**rustls > crypto**

# Module: crypto

## Contents

**Modules**

- [`cipher`](#cipher) - TLS message encryption/decryption interfaces.
- [`hash`](#hash) - Hashing interfaces.
- [`hmac`](#hmac) - HMAC interfaces.
- [`hpke`](#hpke) - Hybrid public key encryption (RFC 9180).
- [`ring`](#ring) - *ring* based CryptoProvider.
- [`tls12`](#tls12) - Cryptography specific to TLS1.2.
- [`tls13`](#tls13) - Cryptography specific to TLS1.3.

**Structs**

- [`CompletedKeyExchange`](#completedkeyexchange) - The result from [`SupportedKxGroup::start_and_complete()`].
- [`CryptoProvider`](#cryptoprovider) - Controls core cryptography used by rustls.
- [`SharedSecret`](#sharedsecret) - The result from [`ActiveKeyExchange::complete`] or [`ActiveKeyExchange::complete_hybrid_component`].

**Traits**

- [`ActiveKeyExchange`](#activekeyexchange) - An in-progress key exchange originating from a [`SupportedKxGroup`].
- [`KeyProvider`](#keyprovider) - A mechanism for loading private [`SigningKey`]s from [`PrivateKeyDer`].
- [`SecureRandom`](#securerandom) - A source of cryptographically secure randomness.
- [`SupportedKxGroup`](#supportedkxgroup) - A supported key exchange group.

---

## rustls::crypto::ActiveKeyExchange

*Trait*

An in-progress key exchange originating from a [`SupportedKxGroup`].

**Methods:**

- `complete`: Completes the key exchange, given the peer's public key.
- `complete_for_tls_version`: Completes the key exchange for the given TLS version, given the peer's public key.
- `hybrid_component`: For hybrid key exchanges, returns the [`NamedGroup`] and key share
- `complete_hybrid_component`: Completes the classical component of the key exchange, given the peer's public key.
- `pub_key`: Return the public key being used.
- `ffdhe_group`: FFDHE group the `ActiveKeyExchange` is operating in.
- `group`: Return the group being used.



## rustls::crypto::CompletedKeyExchange

*Struct*

The result from [`SupportedKxGroup::start_and_complete()`].

**Fields:**
- `group: crate::NamedGroup` - Which group was used.
- `pub_key: alloc::vec::Vec<u8>` - Our key share (sometimes a public key).
- `secret: SharedSecret` - The computed shared secret.



## rustls::crypto::CryptoProvider

*Struct*

Controls core cryptography used by rustls.

This crate comes with two built-in options, provided as
`CryptoProvider` structures:

- [`crypto::aws_lc_rs::default_provider`]: (behind the `aws_lc_rs` crate feature,
  which is enabled by default).  This provider uses the [aws-lc-rs](https://github.com/aws/aws-lc-rs)
  crate.  The `fips` crate feature makes this option use FIPS140-3-approved cryptography.
- [`crypto::ring::default_provider`]: (behind the `ring` crate feature, which
  is optional).  This provider uses the [*ring*](https://github.com/briansmith/ring)
  crate.

This structure provides defaults. Everything in it can be overridden at
runtime by replacing field values as needed.

# Using the per-process default `CryptoProvider`

There is the concept of an implicit default provider, configured at run-time once in
a given process.

It is used for functions like [`ClientConfig::builder()`] and [`ServerConfig::builder()`].

The intention is that an application can specify the [`CryptoProvider`] they wish to use
once, and have that apply to the variety of places where their application does TLS
(which may be wrapped inside other libraries).
They should do this by calling [`CryptoProvider::install_default()`] early on.

To achieve this goal:

- _libraries_ should use [`ClientConfig::builder()`]/[`ServerConfig::builder()`]
  or otherwise rely on the [`CryptoProvider::get_default()`] provider.
- _applications_ should call [`CryptoProvider::install_default()`] early
  in their `fn main()`. If _applications_ uses a custom provider based on the one built-in,
  they can activate the `custom-provider` feature to ensure its usage.

# Using a specific `CryptoProvider`

Supply the provider when constructing your [`ClientConfig`] or [`ServerConfig`]:

- [`ClientConfig::builder_with_provider()`]
- [`ServerConfig::builder_with_provider()`]

When creating and configuring a webpki-backed client or server certificate verifier, a choice of
provider is also needed to start the configuration process:

- [`client::WebPkiServerVerifier::builder_with_provider()`]
- [`server::WebPkiClientVerifier::builder_with_provider()`]

If you install a custom provider and want to avoid any accidental use of a built-in provider, the feature
`custom-provider` can be activated to ensure your custom provider is used everywhere
and not a built-in one. This will disable any implicit use of a built-in provider.

# Making a custom `CryptoProvider`

Your goal will be to populate an instance of this `CryptoProvider` struct.

## Which elements are required?

There is no requirement that the individual elements ([`SupportedCipherSuite`], [`SupportedKxGroup`],
[`SigningKey`], etc.) come from the same crate.  It is allowed and expected that uninteresting
elements would be delegated back to one of the default providers (statically) or a parent
provider (dynamically).

For example, if we want to make a provider that just overrides key loading in the config builder
API (with [`ConfigBuilder::with_single_cert`], etc.), it might look like this:

```
# #[cfg(feature = "aws_lc_rs")] {
# use std::sync::Arc;
# mod fictious_hsm_api { pub fn load_private_key(key_der: pki_types::PrivateKeyDer<'static>) -> ! { unreachable!(); } }
use rustls::crypto::aws_lc_rs;

pub fn provider() -> rustls::crypto::CryptoProvider {
  rustls::crypto::CryptoProvider{
    key_provider: &HsmKeyLoader,
    ..aws_lc_rs::default_provider()
  }
}

#[derive(Debug)]
struct HsmKeyLoader;

impl rustls::crypto::KeyProvider for HsmKeyLoader {
    fn load_private_key(&self, key_der: pki_types::PrivateKeyDer<'static>) -> Result<Arc<dyn rustls::sign::SigningKey>, rustls::Error> {
         fictious_hsm_api::load_private_key(key_der)
    }
}
# }
```

## References to the individual elements

The elements are documented separately:

- **Random** - see [`crypto::SecureRandom::fill()`].
- **Cipher suites** - see [`SupportedCipherSuite`], [`Tls12CipherSuite`], and
  [`Tls13CipherSuite`].
- **Key exchange groups** - see [`crypto::SupportedKxGroup`].
- **Signature verification algorithms** - see [`crypto::WebPkiSupportedAlgorithms`].
- **Authentication key loading** - see [`crypto::KeyProvider::load_private_key()`] and
  [`sign::SigningKey`].

# Example code

See custom [`provider-example/`] for a full client and server example that uses
cryptography from the [`RustCrypto`] and [`dalek-cryptography`] projects.

```shell
$ cargo run --example client | head -3
Current ciphersuite: TLS13_CHACHA20_POLY1305_SHA256
HTTP/1.1 200 OK
Content-Type: text/html; charset=utf-8
Content-Length: 19899
```

[`provider-example/`]: https://github.com/rustls/rustls/tree/main/provider-example/
[`RustCrypto`]: https://github.com/RustCrypto
[`dalek-cryptography`]: https://github.com/dalek-cryptography

# FIPS-approved cryptography
The `fips` crate feature enables use of the `aws-lc-rs` crate in FIPS mode.

You can verify the configuration at runtime by checking
[`ServerConfig::fips()`]/[`ClientConfig::fips()`] return `true`.

**Fields:**
- `cipher_suites: alloc::vec::Vec<suites::SupportedCipherSuite>` - List of supported ciphersuites, in preference order -- the first element
- `kx_groups: alloc::vec::Vec<&'static dyn SupportedKxGroup>` - List of supported key exchange groups, in preference order -- the
- `signature_verification_algorithms: WebPkiSupportedAlgorithms` - List of signature verification algorithms for use with webpki.
- `secure_random: &'static dyn SecureRandom` - Source of cryptographically secure random numbers.
- `key_provider: &'static dyn KeyProvider` - Provider for loading private [`SigningKey`]s from [`PrivateKeyDer`].

**Methods:**

- `fn install_default(self: Self) -> Result<(), alloc::sync::Arc<Self>>` - Sets this `CryptoProvider` as the default for this process.
- `fn get_default() -> Option<&'static alloc::sync::Arc<Self>>` - Returns the default `CryptoProvider` for this process.
- `fn fips(self: &Self) -> bool` - Returns `true` if this `CryptoProvider` is operating in FIPS mode.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> CryptoProvider`



## rustls::crypto::KeyProvider

*Trait*

A mechanism for loading private [`SigningKey`]s from [`PrivateKeyDer`].

This trait is intended to be used with private key material that is sourced from DER,
such as a private-key that may be present on-disk. It is not intended to be used with
keys held in hardware security modules (HSMs) or physical tokens. For these use-cases
see the Rustls manual section on [customizing private key usage].

[customizing private key usage]: <https://docs.rs/rustls/latest/rustls/manual/_03_howto/index.html#customising-private-key-usage>

**Methods:**

- `load_private_key`: Decode and validate a private signing key from `key_der`.
- `fips`: Return `true` if this is backed by a FIPS-approved implementation.



## rustls::crypto::SecureRandom

*Trait*

A source of cryptographically secure randomness.

**Methods:**

- `fill`: Fill the given buffer with random bytes.
- `fips`: Return `true` if this is backed by a FIPS-approved implementation.



## rustls::crypto::SharedSecret

*Struct*

The result from [`ActiveKeyExchange::complete`] or [`ActiveKeyExchange::complete_hybrid_component`].

**Methods:**

- `fn secret_bytes(self: &Self) -> &[u8]` - Returns the shared secret as a slice of bytes.

**Trait Implementations:**

- **From**
  - `fn from(source: &[u8]) -> Self`
- **From**
  - `fn from(buf: Vec<u8>) -> Self`
- **Drop**
  - `fn drop(self: & mut Self)`



## rustls::crypto::SupportedKxGroup

*Trait*

A supported key exchange group.

This type carries both configuration and implementation. Specifically,
it has a TLS-level name expressed using the [`NamedGroup`] enum, and
a function which produces a [`ActiveKeyExchange`].

Compare with [`NamedGroup`], which carries solely a protocol identifier.

**Methods:**

- `start`: Start a key exchange.
- `start_and_complete`: Start and complete a key exchange, in one operation.
- `ffdhe_group`: FFDHE group the `SupportedKxGroup` operates in.
- `name`: Named group the SupportedKxGroup operates in.
- `fips`: Return `true` if this is backed by a FIPS-approved implementation.
- `usable_for_version`: Return `true` if this should be offered/selected with the given version.



## Module: cipher

TLS message encryption/decryption interfaces.



## Module: hash

Hashing interfaces.



## Module: hmac

HMAC interfaces.



## Module: hpke

Hybrid public key encryption (RFC 9180).



## Module: ring

*ring* based CryptoProvider.



## Module: tls12

Cryptography specific to TLS1.2.



## Module: tls13

Cryptography specific to TLS1.3.



