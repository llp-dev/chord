**rustls > versions**

# Module: versions

## Contents

**Structs**

- [`SupportedProtocolVersion`](#supportedprotocolversion) - A TLS protocol version supported by rustls.

**Statics**

- [`ALL_VERSIONS`](#all_versions) - A list of all the protocol versions supported by rustls.
- [`DEFAULT_VERSIONS`](#default_versions) - The version configuration that an application should use by default.
- [`TLS12`](#tls12) - TLS1.2
- [`TLS13`](#tls13) - TLS1.3

---

## rustls::versions::ALL_VERSIONS

*Static*

A list of all the protocol versions supported by rustls.

```rust
static ALL_VERSIONS: &[&SupportedProtocolVersion]
```



## rustls::versions::DEFAULT_VERSIONS

*Static*

The version configuration that an application should use by default.

This will be [`ALL_VERSIONS`] for now, but gives space in the future
to remove a version from here and require users to opt-in to older
versions.

```rust
static DEFAULT_VERSIONS: &[&SupportedProtocolVersion]
```



## rustls::versions::SupportedProtocolVersion

*Struct*

A TLS protocol version supported by rustls.

All possible instances of this class are provided by the library in
the [`ALL_VERSIONS`] array, as well as individually as [`TLS12`]
and [`TLS13`].

**Fields:**
- `version: crate::enums::ProtocolVersion` - The TLS enumeration naming this version.

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &SupportedProtocolVersion) -> bool`



## rustls::versions::TLS12

*Static*

TLS1.2

```rust
static TLS12: SupportedProtocolVersion
```



## rustls::versions::TLS13

*Static*

TLS1.3

```rust
static TLS13: SupportedProtocolVersion
```



