*[ring](../index.md) / [pkcs8](index.md)*

---

# Module `pkcs8`

PKCS#8 is specified in [RFC 5958].


## Contents

- [Structs](#structs)
  - [`PublicKeyOptions`](#publickeyoptions)
  - [`Template`](#template)
  - [`Document`](#document)
- [Enums](#enums)
  - [`Version`](#version)
- [Functions](#functions)
  - [`unwrap_key`](#unwrap-key)
  - [`unwrap_key_`](#unwrap-key)
  - [`unwrap_key__`](#unwrap-key)
  - [`wrap_key`](#wrap-key)
  - [`wrap_key_`](#wrap-key)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`PublicKeyOptions`](#publickeyoptions) | struct |  |
| [`Template`](#template) | struct | A template for constructing PKCS#8 documents. |
| [`Document`](#document) | struct | A generated PKCS#8 document. |
| [`Version`](#version) | enum |  |
| [`unwrap_key`](#unwrap-key) | fn | Parses an unencrypted PKCS#8 private key, verifies that it is the right type of key, and returns the key value. |
| [`unwrap_key_`](#unwrap-key) | fn | Parses an unencrypted PKCS#8 private key, verifies that it is the right type of key, and returns the key value. |
| [`unwrap_key__`](#unwrap-key) | fn |  |
| [`wrap_key`](#wrap-key) | fn |  |
| [`wrap_key_`](#wrap-key) | fn | Formats a private key "prefix\|\|private_key\|\|middle\|\|public_key" where `template` is "prefix\|\|middle" split at position `private_key_index`. |

## Structs

### `PublicKeyOptions`

```rust
struct PublicKeyOptions {
    pub accept_legacy_ed25519_public_key_tag: bool,
}
```

#### Fields

- **`accept_legacy_ed25519_public_key_tag`**: `bool`

  Should the wrong public key ASN.1 tagging used by early implementations
  of PKCS#8 v2 (including earlier versions of *ring*) be accepted?

### `Template`

```rust
struct Template {
    pub bytes: &'static [u8],
    pub alg_id_range: core::ops::Range<usize>,
    pub curve_id_index: usize,
    pub private_key_index: usize,
}
```

A template for constructing PKCS#8 documents.

Note that this only works for ECC.

#### Implementations

- <span id="template-alg-id-value"></span>`fn alg_id_value(&self) -> untrusted::Input<'_>`

- <span id="template-alg-id-value"></span>`fn alg_id_value_(&self) -> &[u8]`

- <span id="template-curve-oid"></span>`fn curve_oid(&self) -> untrusted::Input<'_>`

### `Document`

```rust
struct Document {
    bytes: [u8; 185],
    len: usize,
}
```

A generated PKCS#8 document.

#### Trait Implementations

##### `impl AsRef for Document`

- <span id="document-asref-as-ref"></span>`fn as_ref(&self) -> &[u8]`

## Enums

### `Version`

```rust
enum Version {
    V1Only,
    V1OrV2(PublicKeyOptions),
    V2Only(PublicKeyOptions),
}
```

## Functions

### `unwrap_key`

```rust
fn unwrap_key<'a>(template: &Template, version: Version, input: untrusted::Input<'a>) -> Result<(untrusted::Input<'a>, Option<untrusted::Input<'a>>), error::KeyRejected>
```

Parses an unencrypted PKCS#8 private key, verifies that it is the right type
of key, and returns the key value.

PKCS#8 is specified in [RFC 5958].


### `unwrap_key_`

```rust
fn unwrap_key_<'a>(alg_id: untrusted::Input<'_>, version: Version, input: untrusted::Input<'a>) -> Result<(untrusted::Input<'a>, Option<untrusted::Input<'a>>), error::KeyRejected>
```

Parses an unencrypted PKCS#8 private key, verifies that it is the right type
of key, and returns the key value.

`alg_id` must be the encoded value (not including the outermost `SEQUENCE`
tag and length) of the `AlgorithmIdentifier` that identifies the key type.
The result will be an encoded `RSAPrivateKey` or `ECPrivateKey` or similar.

PKCS#8 is specified in [RFC 5958].


### `unwrap_key__`

```rust
fn unwrap_key__<'a>(alg_id: untrusted::Input<'_>, version: Version, input: &mut untrusted::Reader<'a>) -> Result<(untrusted::Input<'a>, Option<untrusted::Input<'a>>), error::KeyRejected>
```

### `wrap_key`

```rust
fn wrap_key(template: &Template, private_key: &[u8], public_key: &[u8]) -> Document
```

### `wrap_key_`

```rust
fn wrap_key_(template: &Template, private_key: &[u8], public_key: &[u8], bytes: &mut [u8])
```

Formats a private key "prefix||private_key||middle||public_key" where
`template` is "prefix||middle" split at position `private_key_index`.

