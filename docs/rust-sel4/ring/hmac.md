**ring > hmac**

# Module: hmac

## Contents

**Structs**

- [`Algorithm`](#algorithm) - An HMAC algorithm.
- [`Context`](#context) - A context for multi-step (Init-Update-Finish) HMAC signing.
- [`Key`](#key) - A key to use for HMAC signing.
- [`Tag`](#tag) - An HMAC tag.

**Functions**

- [`sign`](#sign) - Calculates the HMAC of `data` using the key `key` in one step.
- [`verify`](#verify) - Calculates the HMAC of `data` using the signing key `key`, and verifies

**Statics**

- [`HMAC_SHA1_FOR_LEGACY_USE_ONLY`](#hmac_sha1_for_legacy_use_only) - HMAC using SHA-1. Obsolete.
- [`HMAC_SHA256`](#hmac_sha256) - HMAC using SHA-256.
- [`HMAC_SHA384`](#hmac_sha384) - HMAC using SHA-384.
- [`HMAC_SHA512`](#hmac_sha512) - HMAC using SHA-512.

---

## ring::hmac::Algorithm

*Struct*

An HMAC algorithm.

**Tuple Struct**: `()`

**Methods:**

- `fn digest_algorithm(self: &Self) -> &'static digest::Algorithm` - The digest algorithm this HMAC algorithm is based on.

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Algorithm) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Algorithm`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **KeyType**
  - `fn len(self: &Self) -> usize`



## ring::hmac::Context

*Struct*

A context for multi-step (Init-Update-Finish) HMAC signing.

Use `sign` for single-step HMAC signing.

**Methods:**

- `fn with_key(signing_key: &Key) -> Self` - Constructs a new HMAC signing context using the given digest algorithm
- `fn update(self: & mut Self, data: &[u8])` - Updates the HMAC with all the data in `data`. `update` may be called
- `fn sign(self: Self) -> Tag` - Finalizes the HMAC calculation and returns the HMAC value. `sign`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Context`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> Result<(), core::fmt::Error>`



## ring::hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY

*Static*

HMAC using SHA-1. Obsolete.

```rust
static HMAC_SHA1_FOR_LEGACY_USE_ONLY: Algorithm
```



## ring::hmac::HMAC_SHA256

*Static*

HMAC using SHA-256.

```rust
static HMAC_SHA256: Algorithm
```



## ring::hmac::HMAC_SHA384

*Static*

HMAC using SHA-384.

```rust
static HMAC_SHA384: Algorithm
```



## ring::hmac::HMAC_SHA512

*Static*

HMAC using SHA-512.

```rust
static HMAC_SHA512: Algorithm
```



## ring::hmac::Key

*Struct*

A key to use for HMAC signing.

**Methods:**

- `fn generate(algorithm: Algorithm, rng: &dyn rand::SecureRandom) -> Result<Self, error::Unspecified>` - Generate an HMAC signing key using the given digest algorithm with a
- `fn new(algorithm: Algorithm, key_value: &[u8]) -> Self` - Construct an HMAC signing key using the given digest algorithm and key
- `fn algorithm(self: &Self) -> Algorithm` - The digest algorithm for the key.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Key`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> Result<(), core::fmt::Error>`
- **From**
  - `fn from(okm: hkdf::Okm<Algorithm>) -> Self`



## ring::hmac::Tag

*Struct*

An HMAC tag.

For a given tag `t`, use `t.as_ref()` to get the tag value as a byte slice.

**Tuple Struct**: `()`

**Traits:** Copy

**Trait Implementations:**

- **AsRef**
  - `fn as_ref(self: &Self) -> &[u8]`
- **Clone**
  - `fn clone(self: &Self) -> Tag`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ring::hmac::sign

*Function*

Calculates the HMAC of `data` using the key `key` in one step.

Use `Context` to calculate HMACs where the input is in multiple parts.

It is generally not safe to implement HMAC verification by comparing the
return value of `sign` to a tag. Use `verify` for verification instead.

```rust
fn sign(key: &Key, data: &[u8]) -> Tag
```



## ring::hmac::verify

*Function*

Calculates the HMAC of `data` using the signing key `key`, and verifies
whether the resultant value equals `tag`, in one step.

This is logically equivalent to, but more efficient than, constructing a
`Key` with the same value as `key` and then using `verify`.

The verification will be done in constant time to prevent timing attacks.

```rust
fn verify(key: &Key, data: &[u8], tag: &[u8]) -> Result<(), error::Unspecified>
```



