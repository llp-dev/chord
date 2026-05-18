**ring > error**

# Module: error

## Contents

**Structs**

- [`KeyRejected`](#keyrejected) - An error parsing or validating a key.
- [`Unspecified`](#unspecified) - An error with absolutely no details.

---

## ring::error::KeyRejected

*Struct*

An error parsing or validating a key.

The `Display` implementation will return a string that will help you better
understand why a key was rejected change which errors are reported in which
situations while minimizing the likelihood that any applications will be
broken.

Here is an incomplete list of reasons a key may be unsupported:

* Invalid or Inconsistent Components: A component of the key has an invalid
  value, or the mathematical relationship between two (or more) components
  required for a valid key does not hold.

* The encoding of the key is invalid. Perhaps the key isn't in the correct
  format; e.g. it may be Base64 ("PEM") encoded, in which case   the Base64
  encoding needs to be undone first.

* The encoding includes a versioning mechanism and that mechanism indicates
  that the key is encoded in a version of the encoding that isn't supported.
  This might happen for multi-prime RSA keys (keys with more than two
  private   prime factors), which aren't supported, for example.

* Too small or too Large: One of the primary components of the key is too
  small or two large. Too-small keys are rejected for security reasons. Some
  unnecessarily large keys are rejected for performance reasons.

 * Wrong algorithm: The key is not valid for the algorithm in which it was
   being used.

 * Unexpected errors: Report this as a bug.

**Tuple Struct**: `()`

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> KeyRejected`



## ring::error::Unspecified

*Struct*

An error with absolutely no details.

*ring* uses this unit type as the error type in most of its results
because (a) usually the specific reasons for a failure are obvious or are
not useful to know, and/or (b) providing more details about a failure might
provide a dangerous side channel, and/or (c) it greatly simplifies the
error handling logic.

`Result<T, ring::error::Unspecified>` is mostly equivalent to
`Result<T, ()>`. However, `ring::error::Unspecified` implements
[`std::error::Error`] and users of *ring* can implement
`From<ring::error::Unspecified>` to map this to their own error types, as
described in [“Error Handling” in the Rust Book]:

```
use ring::rand::{self, SecureRandom};

enum Error {
    CryptoError,

#  #[cfg(feature = "alloc")]
    IOError(std::io::Error),
    // [...]
}

impl From<ring::error::Unspecified> for Error {
    fn from(_: ring::error::Unspecified) -> Self { Error::CryptoError }
}

fn eight_random_bytes() -> Result<[u8; 8], Error> {
    let rng = rand::SystemRandom::new();
    let mut bytes = [0; 8];

    // The `From<ring::error::Unspecified>` implementation above makes this
    // equivalent to
    // `rng.fill(&mut bytes).map_err(|_| Error::CryptoError)?`.
    rng.fill(&mut bytes)?;

    Ok(bytes)
}

assert!(eight_random_bytes().is_ok());
```

Experience with using and implementing other crypto libraries like has
shown that sophisticated error reporting facilities often cause significant
bugs themselves, both within the crypto library and within users of the
crypto library. This approach attempts to minimize complexity in the hopes
of avoiding such problems. In some cases, this approach may be too extreme,
and it may be important for an operation to provide some details about the
cause of a failure. Users of *ring* are encouraged to report such cases so
that they can be addressed individually.

[`std::error::Error`]: https://doc.rust-lang.org/std/error/trait.Error.html
[“Error Handling” in the Rust Book]:
    https://doc.rust-lang.org/book/first-edition/error-handling.html#the-from-trait

**Unit Struct**

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(_: KeyRejected) -> Self`
- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **From**
  - `fn from(_: untrusted::EndOfInput) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Unspecified`
- **From**
  - `fn from(_: core::array::TryFromSliceError) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Unspecified) -> bool`



