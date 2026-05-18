**object > build > bytes**

# Module: build::bytes

## Contents

**Structs**

- [`ByteString`](#bytestring) - A byte slice that is a string of an unknown encoding.
- [`Bytes`](#bytes) - A byte slice.

---

## object::build::bytes::ByteString

*Struct*

A byte slice that is a string of an unknown encoding.

Uses copy-on-write to avoid unnecessary allocations. The bytes can be
accessed as a slice using the `Deref` trait, or as a mutable `Vec` using the
`to_mut` method.

Provides a `Debug` implementation that interprets the bytes as UTF-8.

**Generic Parameters:**
- 'a

**Tuple Struct**: `()`

**Methods:**

- `fn to_mut(self: & mut Self) -> & mut Vec<u8>` - Acquire a mutable reference to the bytes.
- `fn as_slice(self: &Self) -> &[u8]` - Get the bytes as a slice.

**Traits:** Eq

**Trait Implementations:**

- **Borrow**
  - `fn borrow(self: &Self) -> &[u8]`
- **Deref**
  - `fn deref(self: &Self) -> &[u8]`
- **From**
  - `fn from(s: &'a str) -> Self`
- **Default**
  - `fn default() -> ByteString<'a>`
- **From**
  - `fn from(bytes: &'a [u8]) -> Self`
- **Debug**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ByteString<'a>`
- **PartialEq**
  - `fn eq(self: &Self, other: &ByteString<'a>) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **From**
  - `fn from(bytes: Vec<u8>) -> Self`
- **Display**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`



## object::build::bytes::Bytes

*Struct*

A byte slice.

Uses copy-on-write to avoid unnecessary allocations. The bytes can be
accessed as a slice using the `Deref` trait, or as a mutable `Vec` using the
`to_mut` method.

Provides a `Debug` implementation that shows the first 8 bytes and the length.

**Generic Parameters:**
- 'a

**Tuple Struct**: `()`

**Methods:**

- `fn to_mut(self: & mut Self) -> & mut Vec<u8>` - Acquire a mutable reference to the bytes.
- `fn as_slice(self: &Self) -> &[u8]` - Get the bytes as a slice.

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Bytes<'a>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Bytes<'a>) -> bool`
- **From**
  - `fn from(bytes: &'a [u8]) -> Self`
- **From**
  - `fn from(bytes: Vec<u8>) -> Self`
- **Default**
  - `fn default() -> Bytes<'a>`
- **Deref**
  - `fn deref(self: &Self) -> &[u8]`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



