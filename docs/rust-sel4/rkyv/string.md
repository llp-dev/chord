**rkyv > string**

# Module: string

## Contents

**Modules**

- [`repr`](#repr) - An archived string representation that supports inlining short strings.

**Structs**

- [`ArchivedString`](#archivedstring) - An archived [`String`].
- [`StringResolver`](#stringresolver) - The resolver for `String`.

---

## rkyv::string::ArchivedString

*Struct*

An archived [`String`].

This has inline and out-of-line representations. Short strings will use the
available space inside the structure to store the string, and long strings
will store a [`RelPtr`](crate::RelPtr) to a `str` instead.

**Methods:**

- `fn as_str(self: &Self) -> &str` - Extracts a string slice containing the entire `ArchivedString`.
- `fn as_str_seal(this: Seal<Self>) -> Seal<str>` - Extracts a sealed mutable string slice containing the entire
- `fn resolve_from_str(value: &str, resolver: StringResolver, out: Place<Self>)` - Resolves an archived string from a given `str`.
- `fn serialize_from_str<S>(value: &str, serializer: & mut S) -> Result<StringResolver, <S as >::Error>` - Serializes an archived string from a given `str`.

**Traits:** Eq, Portable

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &&str) -> bool`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **Index**
  - `fn index(self: &Self, index: Range<usize>) -> &<Self as >::Output`
- **PartialEq**
  - `fn eq(self: &Self, other: &String) -> bool`
- **Index**
  - `fn index(self: &Self, index: RangeFrom<usize>) -> &<Self as >::Output`
- **Index**
  - `fn index(self: &Self, index: RangeFull) -> &<Self as >::Output`
- **Index**
  - `fn index(self: &Self, index: RangeInclusive<usize>) -> &<Self as >::Output`
- **Index**
  - `fn index(self: &Self, index: RangeTo<usize>) -> &<Self as >::Output`
- **Index**
  - `fn index(self: &Self, index: RangeToInclusive<usize>) -> &<Self as >::Output`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<cmp::Ordering>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &str) -> Option<cmp::Ordering>`
- **Deserialize**
  - `fn deserialize(self: &Self, _: & mut D) -> Result<String, <D as >::Error>`
- **Verify**
  - `fn verify(self: &Self, context: & mut C) -> Result<(), <C as >::Error>`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &String) -> Option<Ordering>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Borrow**
  - `fn borrow(self: &Self) -> &str`
- **PartialEq**
  - `fn eq(self: &Self, other: &str) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &&str) -> Option<cmp::Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> cmp::Ordering`
- **AsRef**
  - `fn as_ref(self: &Self) -> &str`



## rkyv::string::StringResolver

*Struct*

The resolver for `String`.



## Module: repr

An archived string representation that supports inlining short strings.



