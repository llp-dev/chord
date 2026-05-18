**ring > pbkdf2**

# Module: pbkdf2

## Contents

**Structs**

- [`Algorithm`](#algorithm) - A PBKDF2 algorithm.

**Functions**

- [`derive`](#derive) - Fills `out` with the key derived using PBKDF2 with the given inputs.
- [`verify`](#verify) - Verifies that a previously-derived (e.g., using `derive`) PBKDF2 value

**Statics**

- [`PBKDF2_HMAC_SHA1`](#pbkdf2_hmac_sha1) - PBKDF2 using HMAC-SHA1.
- [`PBKDF2_HMAC_SHA256`](#pbkdf2_hmac_sha256) - PBKDF2 using HMAC-SHA256.
- [`PBKDF2_HMAC_SHA384`](#pbkdf2_hmac_sha384) - PBKDF2 using HMAC-SHA384.
- [`PBKDF2_HMAC_SHA512`](#pbkdf2_hmac_sha512) - PBKDF2 using HMAC-SHA512.

---

## ring::pbkdf2::Algorithm

*Struct*

A PBKDF2 algorithm.

**Tuple Struct**: `()`

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Algorithm) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Algorithm`



## ring::pbkdf2::PBKDF2_HMAC_SHA1

*Static*

PBKDF2 using HMAC-SHA1.

```rust
static PBKDF2_HMAC_SHA1: Algorithm
```



## ring::pbkdf2::PBKDF2_HMAC_SHA256

*Static*

PBKDF2 using HMAC-SHA256.

```rust
static PBKDF2_HMAC_SHA256: Algorithm
```



## ring::pbkdf2::PBKDF2_HMAC_SHA384

*Static*

PBKDF2 using HMAC-SHA384.

```rust
static PBKDF2_HMAC_SHA384: Algorithm
```



## ring::pbkdf2::PBKDF2_HMAC_SHA512

*Static*

PBKDF2 using HMAC-SHA512.

```rust
static PBKDF2_HMAC_SHA512: Algorithm
```



## ring::pbkdf2::derive

*Function*

Fills `out` with the key derived using PBKDF2 with the given inputs.

Do not use `derive` as part of verifying a secret; use `verify` instead, to
minimize the effectiveness of timing attacks.

`out.len()` must be no larger than the digest length * (2**32 - 1), per the
PBKDF2 specification.

| Parameter   | RFC 2898 Section 5.2 Term
|-------------|-------------------------------------------
| digest_alg  | PRF (HMAC with the given digest algorithm)
| iterations  | c (iteration count)
| salt        | S (salt)
| secret      | P (password)
| out         | dk (derived key)
| out.len()   | dkLen (derived key length)

# Panics

`derive` panics if `out.len()` is larger than (2**32 - 1) * the digest
algorithm's output length, per the PBKDF2 specification.

```rust
fn derive(algorithm: Algorithm, iterations: core::num::NonZeroU32, salt: &[u8], secret: &[u8], out: & mut [u8])
```



## ring::pbkdf2::verify

*Function*

Verifies that a previously-derived (e.g., using `derive`) PBKDF2 value
matches the PBKDF2 value derived from the other inputs.

The comparison is done in constant time to prevent timing attacks. The
comparison will fail if `previously_derived` is empty (has a length of
zero).

| Parameter                  | RFC 2898 Section 5.2 Term
|----------------------------|--------------------------------------------
| digest_alg                 | PRF (HMAC with the given digest algorithm).
| `iterations`               | c (iteration count)
| `salt`                     | S (salt)
| `secret`                   | P (password)
| `previously_derived`       | dk (derived key)
| `previously_derived.len()` | dkLen (derived key length)

# Panics

`verify` panics if `out.len()` is larger than (2**32 - 1) * the digest
algorithm's output length, per the PBKDF2 specification.

```rust
fn verify(algorithm: Algorithm, iterations: core::num::NonZeroU32, salt: &[u8], secret: &[u8], previously_derived: &[u8]) -> Result<(), error::Unspecified>
```



