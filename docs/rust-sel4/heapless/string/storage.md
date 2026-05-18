**heapless > string > storage**

# Module: string::storage

## Contents

**Traits**

- [`StringStorage`](#stringstorage) - Trait defining how data for a String is stored.
- [`StringStorageSealed`](#stringstoragesealed)

---

## heapless::string::storage::StringStorage

*Trait*

Trait defining how data for a String is stored.

There's two implementations available:

- [`OwnedStorage`]: stores the data in an array whose size is known at compile time.
- [`ViewStorage`]: stores the data in an unsized slice

This allows [`String`] to be generic over either sized or unsized storage. The [`string`](super)
module contains a [`StringInner`] struct that's generic on [`StringStorage`],
and two type aliases for convenience:

- [`String<N>`](crate::string::String) = `StringInner<OwnedStorage<u8, N>>`
- [`StringView<T>`](crate::string::StringView) = `StringInner<ViewStorage<u8>>`

`String` can be unsized into `StrinsgView`, either by unsizing coercions such as `&mut String -> &mut StringView` or
`Box<String> -> Box<StringView>`, or explicitly with [`.as_view()`](crate::string::String::as_view) or [`.as_mut_view()`](crate::string::String::as_mut_view).

This trait is sealed, so you cannot implement it for your own types. You can only use
the implementations provided by this crate.

[`StringInner`]: super::StringInner
[`String`]: super::String
[`OwnedStorage`]: super::OwnedStorage
[`ViewStorage`]: super::ViewStorage



## heapless::string::storage::StringStorageSealed

*Trait*

**Methods:**

- `as_string_view`
- `as_string_mut_view`



