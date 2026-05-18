**sel4_async_network_rustls > error**

# Module: error

## Contents

**Enums**

- [`Error`](#error)

---

## sel4_async_network_rustls::error::Error

*Enum*

**Generic Parameters:**
- E

**Variants:**
- `TransitError(E)`
- `ConnectionAborted`
- `TlsError(rustls::Error)`
- `EncodeError(rustls::unbuffered::EncodeError)`
- `EncryptError(rustls::unbuffered::EncryptError)`

**Traits:** Error

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(err: EncryptError) -> Self`
- **From**
  - `fn from(err: EncodeError) -> Self`
- **From**
  - `fn from(err: TlsError) -> Self`
- **Display**
  - `fn fmt(self: &Self, __formatter: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **Error**
  - `fn kind(self: &Self) -> ErrorKind`



