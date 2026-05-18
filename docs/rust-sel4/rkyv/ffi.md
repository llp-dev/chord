**rkyv > ffi**

# Module: ffi

## Contents

**Structs**

- [`ArchivedCString`](#archivedcstring) - An archived [`CString`](crate::alloc::ffi::CString).
- [`CStringResolver`](#cstringresolver) - The resolver for `CString`.

---

## rkyv::ffi::ArchivedCString

*Struct*

An archived [`CString`](crate::alloc::ffi::CString).

Uses a [`RelPtr`] to a `CStr` under the hood.

**Methods:**

- `fn as_bytes(self: &Self) -> &[u8]` - Returns the contents of this CString as a slice of bytes.
- `fn as_bytes_with_nul(self: &Self) -> &[u8]` - Equivalent to [`as_bytes`][ArchivedCString::as_bytes()] except that the
- `fn as_c_str(self: &Self) -> &CStr` - Extracts a `CStr` slice containing the entire string.
- `fn resolve_from_c_str(c_str: &CStr, resolver: CStringResolver, out: Place<Self>)` - Resolves an archived C string from the given C string and parameters.
- `fn serialize_from_c_str<S>(c_str: &CStr, serializer: & mut S) -> Result<CStringResolver, <S as >::Error>` - Serializes a C string.

**Traits:** Portable, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<cmp::Ordering>`
- **PartialEq**
  - `fn eq(self: &Self, other: &CString) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> cmp::Ordering`
- **Deserialize**
  - `fn deserialize(self: &Self, deserializer: & mut D) -> Result<CString, <D as >::Error>`
- **Borrow**
  - `fn borrow(self: &Self) -> &CStr`
- **Index**
  - `fn index(self: &Self, _: RangeFull) -> &<Self as >::Output`
- **PartialEq**
  - `fn eq(self: &Self, other: &&CStr) -> bool`
- **Verify**
  - `fn verify(self: &Self, context: & mut C) -> Result<(), <C as >::Error>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`
- **AsRef**
  - `fn as_ref(self: &Self) -> &CStr`



## rkyv::ffi::CStringResolver

*Struct*

The resolver for `CString`.



