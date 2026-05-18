**rustls > crypto > ring**

# Module: crypto::ring

## Contents

**Modules**

- [`cipher_suite`](#cipher_suite) - All defined cipher suites supported by *ring* appear in this module.
- [`kx_group`](#kx_group) - All defined key exchange groups supported by *ring* appear in this module.
- [`sign`](#sign) - Using software keys for authentication.

**Functions**

- [`default_provider`](#default_provider) - A `CryptoProvider` backed by the [*ring*] crate.

**Statics**

- [`ALL_CIPHER_SUITES`](#all_cipher_suites) - A list of all the cipher suites supported by the rustls *ring* provider.
- [`ALL_KX_GROUPS`](#all_kx_groups) - A list of all the key exchange groups supported by this provider.
- [`DEFAULT_CIPHER_SUITES`](#default_cipher_suites) - The cipher suite configuration that an application should use by default.
- [`DEFAULT_KX_GROUPS`](#default_kx_groups) - A list of the default key exchange groups supported by this provider.

---

## rustls::crypto::ring::ALL_CIPHER_SUITES

*Static*

A list of all the cipher suites supported by the rustls *ring* provider.

```rust
static ALL_CIPHER_SUITES: &[crate::suites::SupportedCipherSuite]
```



## rustls::crypto::ring::ALL_KX_GROUPS

*Static*

A list of all the key exchange groups supported by this provider.

```rust
static ALL_KX_GROUPS: &[&dyn SupportedKxGroup]
```



## rustls::crypto::ring::DEFAULT_CIPHER_SUITES

*Static*

The cipher suite configuration that an application should use by default.

This will be [`ALL_CIPHER_SUITES`] sans any supported cipher suites that
shouldn't be enabled by most applications.

```rust
static DEFAULT_CIPHER_SUITES: &[crate::suites::SupportedCipherSuite]
```



## rustls::crypto::ring::DEFAULT_KX_GROUPS

*Static*

A list of the default key exchange groups supported by this provider.

```rust
static DEFAULT_KX_GROUPS: &[&dyn SupportedKxGroup]
```



## Module: cipher_suite

All defined cipher suites supported by *ring* appear in this module.



## rustls::crypto::ring::default_provider

*Function*

A `CryptoProvider` backed by the [*ring*] crate.

[*ring*]: https://github.com/briansmith/ring

```rust
fn default_provider() -> crate::crypto::CryptoProvider
```



## Module: kx_group

All defined key exchange groups supported by *ring* appear in this module.

[`ALL_KX_GROUPS`] is provided as an array of all of these values.
[`DEFAULT_KX_GROUPS`] is provided as an array of this provider's defaults.



## Module: sign

Using software keys for authentication.



