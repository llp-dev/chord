**rustls > crypto > tls13**

# Module: crypto::tls13

## Contents

**Structs**

- [`HkdfExpanderUsingHmac`](#hkdfexpanderusinghmac) - Implementation of `HkdfExpander` via `hmac::Key`.
- [`HkdfUsingHmac`](#hkdfusinghmac) - Implementation of `Hkdf` (and thence `HkdfExpander`) via `hmac::Hmac`.
- [`OkmBlock`](#okmblock) - Output key material from HKDF, as a value type.
- [`OutputLengthError`](#outputlengtherror) - An error type used for `HkdfExpander::expand_slice` when

**Functions**

- [`expand`](#expand) - `HKDF-Expand(PRK, info, L)` to construct any type from a byte array.

**Traits**

- [`Hkdf`](#hkdf) - A HKDF implementation oriented to the needs of TLS1.3.
- [`HkdfExpander`](#hkdfexpander) - Implementation of `HKDF-Expand` with an implicitly stored and immutable `PRK`.

---

## rustls::crypto::tls13::Hkdf

*Trait*

A HKDF implementation oriented to the needs of TLS1.3.

See [RFC5869](https://datatracker.ietf.org/doc/html/rfc5869) for the terminology
used in this definition.

You can use [`HkdfUsingHmac`] which implements this trait on top of an implementation
of [`hmac::Hmac`].

**Methods:**

- `extract_from_zero_ikm`: `HKDF-Extract(salt, 0_HashLen)`
- `extract_from_secret`: `HKDF-Extract(salt, secret)`
- `extract_from_kx_shared_secret`: `HKDF-Extract(salt, shared_secret)` where `shared_secret` is the result of a key exchange.
- `expander_for_okm`: Build a `HkdfExpander` using `okm` as the secret PRK.
- `hmac_sign`: Signs `message` using `key` viewed as a HMAC key.
- `fips`: Return `true` if this is backed by a FIPS-approved implementation.



## rustls::crypto::tls13::HkdfExpander

*Trait*

Implementation of `HKDF-Expand` with an implicitly stored and immutable `PRK`.

**Methods:**

- `expand_slice`: `HKDF-Expand(PRK, info, L)` into a slice.
- `expand_block`: `HKDF-Expand(PRK, info, L=HashLen)` returned as a value.
- `hash_len`: Return what `HashLen` is for this instance.



## rustls::crypto::tls13::HkdfExpanderUsingHmac

*Struct*

Implementation of `HkdfExpander` via `hmac::Key`.

**Tuple Struct**: `()`

**Trait Implementations:**

- **HkdfExpander**
  - `fn expand_slice(self: &Self, info: &[&[u8]], output: & mut [u8]) -> Result<(), OutputLengthError>`
  - `fn expand_block(self: &Self, info: &[&[u8]]) -> OkmBlock`
  - `fn hash_len(self: &Self) -> usize`



## rustls::crypto::tls13::HkdfUsingHmac

*Struct*

Implementation of `Hkdf` (and thence `HkdfExpander`) via `hmac::Hmac`.

**Generic Parameters:**
- 'a

**Tuple Struct**: `(&'a dyn hmac::Hmac)`

**Trait Implementations:**

- **Hkdf**
  - `fn extract_from_zero_ikm(self: &Self, salt: Option<&[u8]>) -> Box<dyn HkdfExpander>`
  - `fn extract_from_secret(self: &Self, salt: Option<&[u8]>, secret: &[u8]) -> Box<dyn HkdfExpander>`
  - `fn expander_for_okm(self: &Self, okm: &OkmBlock) -> Box<dyn HkdfExpander>`
  - `fn hmac_sign(self: &Self, key: &OkmBlock, message: &[u8]) -> hmac::Tag`



## rustls::crypto::tls13::OkmBlock

*Struct*

Output key material from HKDF, as a value type.

**Methods:**

- `fn new(bytes: &[u8]) -> Self` - Build a single OKM block by copying a byte slice.

**Trait Implementations:**

- **AsRef**
  - `fn as_ref(self: &Self) -> &[u8]`
- **Clone**
  - `fn clone(self: &Self) -> OkmBlock`
- **Drop**
  - `fn drop(self: & mut Self)`



## rustls::crypto::tls13::OutputLengthError

*Struct*

An error type used for `HkdfExpander::expand_slice` when
the slice exceeds the maximum HKDF output length.

**Unit Struct**

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## rustls::crypto::tls13::expand

*Function*

`HKDF-Expand(PRK, info, L)` to construct any type from a byte array.

- `PRK` is the implicit key material represented by this instance.
- `L := N`; N is the size of the byte array.
- `info` is a slice of byte slices, which should be processed sequentially
  (or concatenated if that is not possible).

This is infallible, because the set of types (and therefore their length) is known
at compile time.

```rust
fn expand<T, const N>(expander: &dyn HkdfExpander, info: &[&[u8]]) -> T
```



