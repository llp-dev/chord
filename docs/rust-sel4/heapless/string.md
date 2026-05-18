**heapless > string**

# Module: string

## Contents

**Structs**

- [`StringInner`](#stringinner) - Base struct for [`String`] and [`StringView`], generic over the [`StringStorage`].

**Enums**

- [`FromUtf16Error`](#fromutf16error) - A possible error value when converting a [`String`] from a UTF-16 byte slice.

**Type Aliases**

- [`OwnedStorage`](#ownedstorage) - Implementation of [`StringStorage`] that stores the data in an array whose size is known at compile time.
- [`String`](#string) - A fixed capacity [`String`](https://doc.rust-lang.org/std/string/struct.String.html).
- [`StringView`](#stringview) - A dynamic capacity [`String`](https://doc.rust-lang.org/std/string/struct.String.html).
- [`ViewStorage`](#viewstorage) - Implementation of [`StringStorage`] that stores the data in an unsized slice.

---

## heapless::string::FromUtf16Error

*Enum*

A possible error value when converting a [`String`] from a UTF-16 byte slice.

This type is the error type for the [`from_utf16`] method on [`String`].

[`from_utf16`]: String::from_utf16

**Variants:**
- `Capacity(crate::CapacityError)` - The capacity of the `String` is too small for the given operation.
- `DecodeUtf16(core::char::DecodeUtf16Error)` - Error decoding UTF-16.

**Traits:** Error

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **From**
  - `fn from(e: CapacityError) -> Self`



## heapless::string::OwnedStorage

*Type Alias*: `crate::vec::OwnedVecStorage<u8, N>`

Implementation of [`StringStorage`] that stores the data in an array whose size is known at compile time.



## heapless::string::String

*Type Alias*: `StringInner<LenT, OwnedStorage<N>>`

A fixed capacity [`String`](https://doc.rust-lang.org/std/string/struct.String.html).



## heapless::string::StringInner

*Struct*

Base struct for [`String`] and [`StringView`], generic over the [`StringStorage`].

In most cases you should use [`String`] or [`StringView`] directly. Only use this
struct if you want to write code that's generic over both.

**Generic Parameters:**
- LenT
- S

**Methods:**

- `fn new() -> Self` - Constructs a new, empty `String` with a fixed capacity of `N` bytes.
- `fn from_utf16(v: &[u16]) -> Result<Self, FromUtf16Error>` - Decodes a UTF-16–encoded slice `v` into a `String`, returning [`Err`]
- `fn from_utf8(vec: Vec<u8, N, LenT>) -> Result<Self, Utf8Error>` - Convert UTF-8 bytes into a `String`.
- `fn from_utf8_unchecked(vec: Vec<u8, N, LenT>) -> Self` - Convert UTF-8 bytes into a `String`, without checking that the string
- `fn into_bytes(self: Self) -> Vec<u8, N, LenT>` - Converts a `String` into a byte vector.
- `fn drain<R>(self: & mut Self, range: R) -> Drain<LenT>` - Removes the specified range from the string in bulk, returning all
- `fn as_view(self: &Self) -> &StringView<LenT>` - Get a reference to the `String`, erasing the `N` const-generic.
- `fn as_mut_view(self: & mut Self) -> & mut StringView<LenT>` - Get a mutable reference to the `String`, erasing the `N` const-generic.
- `fn as_str(self: &Self) -> &str` - Extracts a string slice containing the entire string.
- `fn as_mut_str(self: & mut Self) -> & mut str` - Converts a `String` into a mutable string slice.
- `fn as_mut_vec(self: & mut Self) -> & mut VecInner<u8, LenT, S>` - Returns a mutable reference to the contents of this `String`.
- `fn push_str(self: & mut Self, string: &str) -> Result<(), CapacityError>` - Appends a given string slice onto the end of this `String`.
- `fn capacity(self: &Self) -> usize` - Returns the maximum number of elements the String can hold.
- `fn push(self: & mut Self, c: char) -> Result<(), CapacityError>` - Appends the given [`char`] to the end of this `String`.
- `fn truncate(self: & mut Self, new_len: usize)` - Shortens this `String` to the specified length.
- `fn pop(self: & mut Self) -> Option<char>` - Removes the last character from the string buffer and returns it.
- `fn remove(self: & mut Self, index: usize) -> char` - Removes a [`char`] from this `String` at a byte position and returns it.
- `fn clear(self: & mut Self)` - Truncates this `String`, removing all contents.
- `fn insert(self: & mut Self, idx: usize, ch: char) -> Result<(), CapacityError>` - Inserts a character into this `String` at a byte position.
- `fn insert_str(self: & mut Self, idx: usize, string: &str) -> Result<(), CapacityError>` - Inserts a string slice into this `String` at a byte position.

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, rhs: &StringInner<LenT1, S1>) -> bool`
- **AsRef**
  - `fn as_ref(self: &Self) -> &str`
- **Borrow**
  - `fn borrow(self: &Self) -> &str`
- **Serialize**
  - `fn serialize<SER>(self: &Self, serializer: SER) -> Result<<SER as >::Ok, <SER as >::Error>`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &StringInner<LenT1, S1>) -> Option<Ordering>`
- **Deref**
  - `fn deref(self: &Self) -> &str`
- **Write**
  - `fn write_str(self: & mut Self, s: &str) -> Result<(), fmt::Error>`
  - `fn write_char(self: & mut Self, c: char) -> Result<(), fmt::Error>`
- **Hash**
  - `fn hash<H>(self: &Self, hasher: & mut H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &str) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **AsRef**
  - `fn as_ref(self: &Self) -> &[u8]`
- **BorrowMut**
  - `fn borrow_mut(self: & mut Self) -> & mut str`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> Ordering`
- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut str`
- **PartialEq**
  - `fn eq(self: &Self, other: &&str) -> bool`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## heapless::string::StringView

*Type Alias*: `StringInner<LenT, ViewStorage>`

A dynamic capacity [`String`](https://doc.rust-lang.org/std/string/struct.String.html).



## heapless::string::ViewStorage

*Type Alias*: `crate::vec::ViewVecStorage<u8>`

Implementation of [`StringStorage`] that stores the data in an unsized slice.



