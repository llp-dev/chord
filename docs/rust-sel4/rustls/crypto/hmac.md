**rustls > crypto > hmac**

# Module: crypto::hmac

## Contents

**Structs**

- [`Tag`](#tag) - A HMAC tag, stored as a value.

**Traits**

- [`Hmac`](#hmac) - A concrete HMAC implementation, for a single cryptographic hash function.
- [`Key`](#key) - A HMAC key that is ready for use.

---

## rustls::crypto::hmac::Hmac

*Trait*

A concrete HMAC implementation, for a single cryptographic hash function.

You should have one object that implements this trait for HMAC-SHA256, another
for HMAC-SHA384, etc.

**Methods:**

- `with_key`: Prepare to use `key` as a HMAC key.
- `hash_output_len`: Give the length of the underlying hash function.  In RFC2104 terminology this is `L`.
- `fips`: Return `true` if this is backed by a FIPS-approved implementation.



## rustls::crypto::hmac::Key

*Trait*

A HMAC key that is ready for use.

The algorithm used is implicit in the `Hmac` object that produced the key.

**Methods:**

- `sign`: Calculates a tag over `data` -- a slice of byte slices.
- `sign_concat`: Calculates a tag over the concatenation of `first`, the items in `middle`, and `last`.
- `tag_len`: Returns the length of the tag returned by a computation using



## rustls::crypto::hmac::Tag

*Struct*

A HMAC tag, stored as a value.

**Methods:**

- `fn new(bytes: &[u8]) -> Self` - Build a tag by copying a byte slice.

**Trait Implementations:**

- **Drop**
  - `fn drop(self: & mut Self)`
- **AsRef**
  - `fn as_ref(self: &Self) -> &[u8]`
- **Clone**
  - `fn clone(self: &Self) -> Tag`



