*[heapless](../index.md) / [string](index.md)*

---

# Module `string`

A fixed capacity [`String`](https://doc.rust-lang.org/std/string/struct.String.html).

## Contents

- [Modules](#modules)
  - [`drain`](#drain)
  - [`storage`](#storage)
- [Structs](#structs)
  - [`Drain`](#drain)
  - [`StringInner`](#stringinner)
- [Enums](#enums)
  - [`FromUtf16Error`](#fromutf16error)
- [Traits](#traits)
  - [`StringStorage`](#stringstorage)
- [Type Aliases](#type-aliases)
  - [`OwnedStorage`](#ownedstorage)
  - [`ViewStorage`](#viewstorage)
  - [`String`](#string)
  - [`StringView`](#stringview)
- [Macros](#macros)
  - [`impl_try_from_num!`](#impl-try-from-num)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`drain`](#drain) | mod |  |
| [`storage`](#storage) | mod |  |
| [`Drain`](#drain) | struct |  |
| [`StringInner`](#stringinner) | struct | Base struct for [`String`] and [`StringView`], generic over the [`StringStorage`]. |
| [`FromUtf16Error`](#fromutf16error) | enum | A possible error value when converting a [`String`] from a UTF-16 byte slice. |
| [`StringStorage`](#stringstorage) | trait |  |
| [`OwnedStorage`](#ownedstorage) | type | Implementation of [`StringStorage`] that stores the data in an array whose size is known at compile time. |
| [`ViewStorage`](#viewstorage) | type | Implementation of [`StringStorage`] that stores the data in an unsized slice. |
| [`String`](#string) | type | A fixed capacity [`String`](https://doc.rust-lang.org/std/string/struct.String.html). |
| [`StringView`](#stringview) | type | A dynamic capacity [`String`](https://doc.rust-lang.org/std/string/struct.String.html). |
| [`impl_try_from_num!`](#impl-try-from-num) | macro |  |

## Modules

- [`drain`](drain/index.md)
- [`storage`](storage/index.md)

## Structs

### `Drain<'a, LenT: LenType>`

```rust
struct Drain<'a, LenT: LenType> {
    string: *mut super::StringView<LenT>,
    start: LenT,
    end: LenT,
    iter: core::str::Chars<'a>,
}
```

A draining iterator for `String`.

This struct is created by the [`drain`](drain/index.md) method on [`crate::String`](#cratestring). See its
documentation for more.


#### Fields

- **`string`**: `*mut super::StringView<LenT>`

  Will be used as &'a mut String in the destructor

- **`start`**: `LenT`

  Stast of part to remove

- **`end`**: `LenT`

  End of part to remove

- **`iter`**: `core::str::Chars<'a>`

  Current remaining range to remove

#### Implementations

- <span id="drain-as-str"></span>`fn as_str(&self) -> &str`

  Returns the remaining (sub)string of this iterator as a slice.

  

  # Examples

  

  ```rust

  use heapless::String;

  

  let mut s = String::<8>::try_from("abc").unwrap();

  let mut drain = s.drain(..);

  assert_eq!(drain.as_str(), "abc");

  let _ = drain.next().unwrap();

  assert_eq!(drain.as_str(), "bc");

  ```

#### Trait Implementations

##### `impl<LenT: LenType> AsRef for Drain<'_, LenT>`

- <span id="drain-asref-as-ref"></span>`fn as_ref(&self) -> &str`

##### `impl<LenT: LenType> Debug for Drain<'_, LenT>`

- <span id="drain-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<LenT: LenType> DoubleEndedIterator for Drain<'_, LenT>`

- <span id="drain-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<char>`

##### `impl<LenT: LenType> Drop for Drain<'_, LenT>`

- <span id="drain-drop"></span>`fn drop(&mut self)`

##### `impl<LenT: LenType> FusedIterator for Drain<'_, LenT>`

##### `impl IntoIterator for Drain<'a, LenT>`

- <span id="drain-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="drain-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="drain-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<LenT: LenType> Iterator for Drain<'_, LenT>`

- <span id="drain-iterator-type-item"></span>`type Item = char`

- <span id="drain-iterator-next"></span>`fn next(&mut self) -> Option<char>`

- <span id="drain-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="drain-iterator-last"></span>`fn last(self) -> Option<char>`

##### `impl<LenT: LenType> Send for Drain<'_, LenT>`

##### `impl<LenT: LenType> Sync for Drain<'_, LenT>`

### `StringInner<LenT: LenType, S: StringStorage + ?Sized>`

```rust
struct StringInner<LenT: LenType, S: StringStorage + ?Sized> {
    vec: crate::vec::VecInner<u8, LenT, S>,
}
```

Base struct for [`String`](#string) and [`StringView`](#stringview), generic over the [`StringStorage`](storage/index.md).

In most cases you should use [`String`](#string) or [`StringView`](#stringview) directly. Only use this
struct if you want to write code that's generic over both.

#### Implementations

- <span id="stringinner-new"></span>`const fn new() -> Self`

  Constructs a new, empty `String` with a fixed capacity of `N` bytes.

  

  # Examples

  

  Basic usage:

  

  ```rust

  use heapless::String;

  

  // allocate the string on the stack

  let mut s: String<4> = String::new();

  

  // allocate the string in a static variable

  static mut S: String<4> = String::new();

  ```

- <span id="stringinner-from-utf16"></span>`fn from_utf16(v: &[u16]) -> Result<Self, FromUtf16Error>` — [`FromUtf16Error`](#fromutf16error)

  Decodes a UTF-16–encoded slice `v` into a `String`, returning [`Err`](#err)

  if `v` contains any invalid data.

  

  # Examples

  

  Basic usage:

  

  ```rust

  use heapless::String;

  

  // 𝄞music

  let v = &[0xD834, 0xDD1E, 0x006d, 0x0075, 0x0073, 0x0069, 0x0063];

  let s: String<10> = String::from_utf16(v).unwrap();

  assert_eq!(s, "𝄞music");

  

  // 𝄞mu<invalid>ic

  let v = &[0xD834, 0xDD1E, 0x006d, 0x0075, 0xD800, 0x0069, 0x0063];

  assert!(String::<10>::from_utf16(v).is_err());

  ```

- <span id="stringinner-from-utf8"></span>`fn from_utf8(vec: Vec<u8, N, LenT>) -> Result<Self, Utf8Error>` — [`Vec`](../vec/index.md#vec)

  Convert UTF-8 bytes into a `String`.

  

  # Examples

  

  Basic usage:

  

  ```rust

  use heapless::{String, Vec};

  

  let mut sparkle_heart = Vec::<u8, 4>::new();

  sparkle_heart.extend_from_slice(&[240, 159, 146, 150]);

  

  let sparkle_heart: String<4> = String::from_utf8(sparkle_heart)?;

  assert_eq!("💖", sparkle_heart);

  Ok::<(), core::str::Utf8Error>(())

  ```

  

  Invalid UTF-8:

  

  ```rust

  use core::str::Utf8Error;

  use heapless::{String, Vec};

  

  let mut vec = Vec::<u8, 4>::new();

  vec.extend_from_slice(&[0, 159, 146, 150]);

  

  let e: Utf8Error = String::from_utf8(vec).unwrap_err();

  assert_eq!(e.valid_up_to(), 1);

  Ok::<(), core::str::Utf8Error>(())

  ```

- <span id="stringinner-from-utf8-unchecked"></span>`unsafe fn from_utf8_unchecked(vec: Vec<u8, N, LenT>) -> Self` — [`Vec`](../vec/index.md#vec)

  Convert UTF-8 bytes into a `String`, without checking that the string

  contains valid UTF-8.

  

  # Safety

  

  The bytes passed in must be valid UTF-8.

  

  # Examples

  

  Basic usage:

  

  ```rust

  use heapless::{String, Vec};

  

  let mut sparkle_heart = Vec::<u8, 4>::new();

  sparkle_heart.extend_from_slice(&[240, 159, 146, 150]);

  

  // Safety: `sparkle_heart` Vec is known to contain valid UTF-8

  let sparkle_heart: String<4> = unsafe { String::from_utf8_unchecked(sparkle_heart) };

  assert_eq!("💖", sparkle_heart);

  ```

- <span id="stringinner-into-bytes"></span>`fn into_bytes(self) -> Vec<u8, N, LenT>` — [`Vec`](../vec/index.md#vec)

  Converts a `String` into a byte vector.

  

  This consumes the `String`, so we do not need to copy its contents.

  

  # Examples

  

  Basic usage:

  

  ```rust

  use heapless::String;

  

  let s: String<4> = String::try_from("ab")?;

  let b = s.into_bytes();

  assert!(b.len() == 2);

  

  assert_eq!(&[b'a', b'b'], &b[..]);

  Ok::<(), heapless::CapacityError>(())

  ```

#### Trait Implementations

##### `impl<LenT: LenType, S: StringStorage + ?Sized> AsRef for StringInner<LenT, S>`

- <span id="stringinner-asref-as-ref"></span>`fn as_ref(&self) -> &str`

##### `impl<LenT: LenType, S: StringStorage + ?Sized> Debug for StringInner<LenT, S>`

- <span id="stringinner-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<LenT: LenType, S: StringStorage + ?Sized> Deref for StringInner<LenT, S>`

- <span id="stringinner-deref-type-target"></span>`type Target = str`

- <span id="stringinner-deref"></span>`fn deref(&self) -> &str`

##### `impl<LenT: LenType, S: StringStorage + ?Sized> DerefMut for StringInner<LenT, S>`

- <span id="stringinner-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut str`

##### `impl DeserializeOwned for StringInner<LenT, S>`

##### `impl<LenT: LenType, S: StringStorage + ?Sized> Display for StringInner<LenT, S>`

- <span id="stringinner-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<LenT: LenType, S: StringStorage + ?Sized> Eq for StringInner<LenT, S>`

##### `impl<LenT: LenType, S: StringStorage + ?Sized> Hash for StringInner<LenT, S>`

- <span id="stringinner-hash"></span>`fn hash<H: hash::Hasher>(&self, hasher: &mut H)`

##### `impl<LenT: LenType, S: StringStorage + ?Sized> Ord for StringInner<LenT, S>`

- <span id="stringinner-ord-cmp"></span>`fn cmp(&self, other: &Self) -> Ordering`

##### `impl<LenT1: LenType, LenT2: LenType, S1: StringStorage + ?Sized, S2: StringStorage + ?Sized> PartialEq for StringInner<LenT2, S2>`

- <span id="stringinner-partialeq-eq"></span>`fn eq(&self, rhs: &StringInner<LenT1, S1>) -> bool` — [`StringInner`](#stringinner)

##### `impl<LenT1: LenType, LenT2: LenType, S1: StringStorage + ?Sized, S2: StringStorage + ?Sized> PartialOrd for StringInner<LenT2, S2>`

- <span id="stringinner-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &StringInner<LenT1, S1>) -> Option<Ordering>` — [`StringInner`](#stringinner)

##### `impl Receiver for StringInner<LenT, S>`

- <span id="stringinner-receiver-type-target"></span>`type Target = T`

##### `impl<LenT: LenType, S: StringStorage + ?Sized> Serialize for crate::string::StringInner<LenT, S>`

- <span id="cratestringstringinner-serialize"></span>`fn serialize<SER>(&self, serializer: SER) -> Result<<SER as >::Ok, <SER as >::Error>`

##### `impl ToString for StringInner<LenT, S>`

- <span id="stringinner-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<LenT: LenType, S: StringStorage + ?Sized> Write for StringInner<LenT, S>`

- <span id="stringinner-write-write-str"></span>`fn write_str(&mut self, s: &str) -> Result<(), fmt::Error>`

- <span id="stringinner-write-write-char"></span>`fn write_char(&mut self, c: char) -> Result<(), fmt::Error>`

## Enums

### `FromUtf16Error`

```rust
enum FromUtf16Error {
    Capacity(crate::CapacityError),
    DecodeUtf16(core::char::DecodeUtf16Error),
}
```

A possible error value when converting a [`String`](#string) from a UTF-16 byte slice.

This type is the error type for the `from_utf16` method on [`String`](#string).


#### Variants

- **`Capacity`**

  The capacity of the `String` is too small for the given operation.

- **`DecodeUtf16`**

  Error decoding UTF-16.

#### Trait Implementations

##### `impl Debug for FromUtf16Error`

- <span id="fromutf16error-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for FromUtf16Error`

- <span id="fromutf16error-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for FromUtf16Error`

##### `impl ToString for FromUtf16Error`

- <span id="fromutf16error-tostring-to-string"></span>`fn to_string(&self) -> String`

## Traits

### `StringStorage`

```rust
trait StringStorage: StringStorageSealed { ... }
```

Trait defining how data for a String is stored.

There's two implementations available:

- [`OwnedStorage`](../linear_map/index.md): stores the data in an array whose size is known at compile time.
- [`ViewStorage`](#viewstorage): stores the data in an unsized slice

This allows [`String`](#string) to be generic over either sized or unsized storage. The [`string`](super)
module contains a [`StringInner`](#stringinner) struct that's generic on [`StringStorage`](storage/index.md),
and two type aliases for convenience:

- [`String<N>`](crate::string::String) = `StringInner<OwnedStorage<u8, N>>`
- [`StringView<T>`](crate::string::StringView) = `StringInner<ViewStorage<u8>>`

`String` can be unsized into `StrinsgView`, either by unsizing coercions such as `&mut String -> &mut StringView` or
`Box<String> -> Box<StringView>`, or explicitly with [`.as_view()`](crate::string::String::as_view) or [`.as_mut_view()`](crate::string::String::as_mut_view).

This trait is sealed, so you cannot implement it for your own types. You can only use
the implementations provided by this crate.





#### Implementors

- [`OwnedVecStorage`](../vec/storage/index.md#ownedvecstorage)
- [`ViewVecStorage`](../vec/storage/index.md#viewvecstorage)

## Type Aliases

### `OwnedStorage<const N: usize>`

```rust
type OwnedStorage<const N: usize> = crate::vec::OwnedVecStorage<u8, N>;
```

Implementation of [`StringStorage`](storage/index.md) that stores the data in an array whose size is known at compile time.

### `ViewStorage`

```rust
type ViewStorage = crate::vec::ViewVecStorage<u8>;
```

Implementation of [`StringStorage`](storage/index.md) that stores the data in an unsized slice.

### `String<const N: usize, LenT>`

```rust
type String<const N: usize, LenT> = StringInner<LenT, OwnedStorage<N>>;
```

A fixed capacity [`String`](https://doc.rust-lang.org/std/string/struct.String.html).

### `StringView<LenT>`

```rust
type StringView<LenT> = StringInner<LenT, ViewStorage>;
```

A dynamic capacity [`String`](https://doc.rust-lang.org/std/string/struct.String.html).

## Macros

### `impl_try_from_num!`

