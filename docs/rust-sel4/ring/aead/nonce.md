**ring > aead > nonce**

# Module: aead::nonce

## Contents

**Structs**

- [`Nonce`](#nonce) - A nonce for a single AEAD opening or sealing operation.

**Constants**

- [`NONCE_LEN`](#nonce_len) - All the AEADs we support use 96-bit nonces.

---

## ring::aead::nonce::NONCE_LEN

*Constant*: `usize`

All the AEADs we support use 96-bit nonces.



## ring::aead::nonce::Nonce

*Struct*

A nonce for a single AEAD opening or sealing operation.

The user must ensure, for a particular key, that each nonce is unique.

`Nonce` intentionally doesn't implement `Clone` to ensure that each one is
consumed at most once.

**Tuple Struct**: `()`

**Methods:**

- `fn try_assume_unique_for_key(value: &[u8]) -> Result<Self, error::Unspecified>` - Constructs a `Nonce` with the given value, assuming that the value is
- `fn assume_unique_for_key(value: [u8; 12]) -> Self` - Constructs a `Nonce` with the given value, assuming that the value is

**Trait Implementations:**

- **AsRef**
  - `fn as_ref(self: &Self) -> &[u8; 12]`



