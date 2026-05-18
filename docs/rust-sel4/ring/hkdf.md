**ring > hkdf**

# Module: hkdf

## Contents

**Structs**

- [`Algorithm`](#algorithm) - An HKDF algorithm.
- [`Okm`](#okm) - An HKDF OKM (Output Keying Material)
- [`Prk`](#prk) - A HKDF PRK (pseudorandom key).
- [`Salt`](#salt) - A salt for HKDF operations.

**Statics**

- [`HKDF_SHA1_FOR_LEGACY_USE_ONLY`](#hkdf_sha1_for_legacy_use_only) - HKDF using HMAC-SHA-1. Obsolete.
- [`HKDF_SHA256`](#hkdf_sha256) - HKDF using HMAC-SHA-256.
- [`HKDF_SHA384`](#hkdf_sha384) - HKDF using HMAC-SHA-384.
- [`HKDF_SHA512`](#hkdf_sha512) - HKDF using HMAC-SHA-512.

**Traits**

- [`KeyType`](#keytype) - The length of the OKM (Output Keying Material) for a `Prk::expand()` call.

---

## ring::hkdf::Algorithm

*Struct*

An HKDF algorithm.

**Tuple Struct**: `()`

**Methods:**

- `fn hmac_algorithm(self: &Self) -> hmac::Algorithm` - The underlying HMAC algorithm.

**Traits:** Copy, Eq

**Trait Implementations:**

- **KeyType**
  - `fn len(self: &Self) -> usize`
- **Clone**
  - `fn clone(self: &Self) -> Algorithm`
- **PartialEq**
  - `fn eq(self: &Self, other: &Algorithm) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ring::hkdf::HKDF_SHA1_FOR_LEGACY_USE_ONLY

*Static*

HKDF using HMAC-SHA-1. Obsolete.

```rust
static HKDF_SHA1_FOR_LEGACY_USE_ONLY: Algorithm
```



## ring::hkdf::HKDF_SHA256

*Static*

HKDF using HMAC-SHA-256.

```rust
static HKDF_SHA256: Algorithm
```



## ring::hkdf::HKDF_SHA384

*Static*

HKDF using HMAC-SHA-384.

```rust
static HKDF_SHA384: Algorithm
```



## ring::hkdf::HKDF_SHA512

*Static*

HKDF using HMAC-SHA-512.

```rust
static HKDF_SHA512: Algorithm
```



## ring::hkdf::KeyType

*Trait*

The length of the OKM (Output Keying Material) for a `Prk::expand()` call.

**Methods:**

- `len`: The length that `Prk::expand()` should expand its input to.



## ring::hkdf::Okm

*Struct*

An HKDF OKM (Output Keying Material)

Intentionally not `Clone` or `Copy` as an OKM is generally only safe to
use once.

**Generic Parameters:**
- 'a
- L

**Methods:**

- `fn len(self: &Self) -> &L` - The `OkmLength` given to `Prk::expand()`.
- `fn fill(self: Self, out: & mut [u8]) -> Result<(), error::Unspecified>` - Fills `out` with the output of the HKDF-Expand operation for the given

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ring::hkdf::Prk

*Struct*

A HKDF PRK (pseudorandom key).

**Tuple Struct**: `()`

**Methods:**

- `fn new_less_safe(algorithm: Algorithm, value: &[u8]) -> Self` - Construct a new `Prk` directly with the given value.
- `fn expand<'a, L>(self: &'a Self, info: &'a [&'a [u8]], len: L) -> Result<Okm<'a, L>, error::Unspecified>` - The [HKDF-Expand] operation.

**Trait Implementations:**

- **From**
  - `fn from(okm: Okm<Algorithm>) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Prk`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ring::hkdf::Salt

*Struct*

A salt for HKDF operations.

**Tuple Struct**: `()`

**Methods:**

- `fn new(algorithm: Algorithm, value: &[u8]) -> Self` - Constructs a new `Salt` with the given value based on the given digest
- `fn extract(self: &Self, secret: &[u8]) -> Prk` - The [HKDF-Extract] operation.
- `fn algorithm(self: &Self) -> Algorithm` - The algorithm used to derive this salt.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(okm: Okm<Algorithm>) -> Self`



