**ring > digest**

# Module: digest

## Contents

**Structs**

- [`Algorithm`](#algorithm) - A digest algorithm.
- [`Context`](#context) - A context for multi-step (Init-Update-Finish) digest calculations.
- [`Digest`](#digest) - A calculated digest value.

**Functions**

- [`digest`](#digest) - Returns the digest of `data` using the given digest algorithm.

**Statics**

- [`SHA1_FOR_LEGACY_USE_ONLY`](#sha1_for_legacy_use_only) - SHA-1 as specified in [FIPS 180-4]. Deprecated.
- [`SHA256`](#sha256) - SHA-256 as specified in [FIPS 180-4].
- [`SHA384`](#sha384) - SHA-384 as specified in [FIPS 180-4].
- [`SHA512`](#sha512) - SHA-512 as specified in [FIPS 180-4].
- [`SHA512_256`](#sha512_256) - SHA-512/256 as specified in [FIPS 180-4].

**Constants**

- [`MAX_BLOCK_LEN`](#max_block_len) - The maximum block length ([`Algorithm::block_len()`]) of all the algorithms
- [`MAX_CHAINING_LEN`](#max_chaining_len) - The maximum chaining length ([`Algorithm::chaining_len()`]) of all the
- [`MAX_OUTPUT_LEN`](#max_output_len) - The maximum output length ([`Algorithm::output_len()`]) of all the
- [`SHA1_OUTPUT_LEN`](#sha1_output_len) - The length of the output of SHA-1, in bytes.
- [`SHA256_OUTPUT_LEN`](#sha256_output_len) - The length of the output of SHA-256, in bytes.
- [`SHA384_OUTPUT_LEN`](#sha384_output_len) - The length of the output of SHA-384, in bytes.
- [`SHA512_256_OUTPUT_LEN`](#sha512_256_output_len) - The length of the output of SHA-512/256, in bytes.
- [`SHA512_OUTPUT_LEN`](#sha512_output_len) - The length of the output of SHA-512, in bytes.

---

## ring::digest::Algorithm

*Struct*

A digest algorithm.

**Methods:**

- `fn block_len(self: &Self) -> usize` - The internal block length.
- `fn chaining_len(self: &Self) -> usize` - The size of the chaining value of the digest function, in bytes.
- `fn output_len(self: &Self) -> usize` - The length of a finalized digest.

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> Result<(), ::core::fmt::Error>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



## ring::digest::Context

*Struct*

A context for multi-step (Init-Update-Finish) digest calculations.

# Examples

```
use ring::digest;

let one_shot = digest::digest(&digest::SHA384, b"hello, world");

let mut ctx = digest::Context::new(&digest::SHA384);
ctx.update(b"hello");
ctx.update(b", ");
ctx.update(b"world");
let multi_part = ctx.finish();

assert_eq!(&one_shot.as_ref(), &multi_part.as_ref());
```

**Methods:**

- `fn new(algorithm: &'static Algorithm) -> Self` - Constructs a new context.
- `fn update(self: & mut Self, data: &[u8])` - Updates the digest with all the data in `data`.
- `fn finish(self: Self) -> Digest` - Finalizes the digest calculation and returns the digest value.
- `fn algorithm(self: &Self) -> &'static Algorithm` - The algorithm that this context is using.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Context`



## ring::digest::Digest

*Struct*

A calculated digest value.

Use [`Self::as_ref`] to get the value as a `&[u8]`.

**Methods:**

- `fn algorithm(self: &Self) -> &'static Algorithm` - The algorithm that was used to calculate the digest value.

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, fmt: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Digest`
- **AsRef**
  - `fn as_ref(self: &Self) -> &[u8]`



## ring::digest::MAX_BLOCK_LEN

*Constant*: `usize`

The maximum block length ([`Algorithm::block_len()`]) of all the algorithms
in this module.



## ring::digest::MAX_CHAINING_LEN

*Constant*: `usize`

The maximum chaining length ([`Algorithm::chaining_len()`]) of all the
algorithms in this module.



## ring::digest::MAX_OUTPUT_LEN

*Constant*: `usize`

The maximum output length ([`Algorithm::output_len()`]) of all the
algorithms in this module.



## ring::digest::SHA1_FOR_LEGACY_USE_ONLY

*Static*

SHA-1 as specified in [FIPS 180-4]. Deprecated.

[FIPS 180-4]: http://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf

```rust
static SHA1_FOR_LEGACY_USE_ONLY: Algorithm
```



## ring::digest::SHA1_OUTPUT_LEN

*Constant*: `usize`

The length of the output of SHA-1, in bytes.



## ring::digest::SHA256

*Static*

SHA-256 as specified in [FIPS 180-4].

[FIPS 180-4]: http://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf

```rust
static SHA256: Algorithm
```



## ring::digest::SHA256_OUTPUT_LEN

*Constant*: `usize`

The length of the output of SHA-256, in bytes.



## ring::digest::SHA384

*Static*

SHA-384 as specified in [FIPS 180-4].

[FIPS 180-4]: http://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf

```rust
static SHA384: Algorithm
```



## ring::digest::SHA384_OUTPUT_LEN

*Constant*: `usize`

The length of the output of SHA-384, in bytes.



## ring::digest::SHA512

*Static*

SHA-512 as specified in [FIPS 180-4].

[FIPS 180-4]: http://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf

```rust
static SHA512: Algorithm
```



## ring::digest::SHA512_256

*Static*

SHA-512/256 as specified in [FIPS 180-4].

This is *not* the same as just truncating the output of SHA-512, as
SHA-512/256 has its own initial state distinct from SHA-512's initial
state.

[FIPS 180-4]: http://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf

```rust
static SHA512_256: Algorithm
```



## ring::digest::SHA512_256_OUTPUT_LEN

*Constant*: `usize`

The length of the output of SHA-512/256, in bytes.



## ring::digest::SHA512_OUTPUT_LEN

*Constant*: `usize`

The length of the output of SHA-512, in bytes.



## ring::digest::digest

*Function*

Returns the digest of `data` using the given digest algorithm.

# Examples:

```
# #[cfg(feature = "alloc")]
# {
use ring::{digest, test};
let expected_hex = "09ca7e4eaa6e8ae9c7d261167129184883644d07dfba7cbfbc4c8a2e08360d5b";
let expected: Vec<u8> = test::from_hex(expected_hex).unwrap();
let actual = digest::digest(&digest::SHA256, b"hello, world");

assert_eq!(&expected, &actual.as_ref());
# }
```

```rust
fn digest(algorithm: &'static Algorithm, data: &[u8]) -> Digest
```



