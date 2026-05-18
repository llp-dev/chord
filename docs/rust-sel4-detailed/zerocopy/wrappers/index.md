*[zerocopy](../index.md) / [wrappers](index.md)*

---

# Module `wrappers`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`read_only_def`](#read-only-def) | mod |  |
| [`Unalign`](#unalign) | struct | A type with no alignment requirement. |

## Modules

- [`read_only_def`](read_only_def/index.md)

## Structs

### `Unalign<T>`

```rust
struct Unalign<T>(T);
```

A type with no alignment requirement.

An `Unalign` wraps a `T`, removing any alignment requirement. `Unalign<T>`
has the same size and bit validity as `T`, but not necessarily the same
alignment [or ABI]. This is useful if a type with an alignment requirement
needs to be read from a chunk of memory which provides no alignment
guarantees.

Since `Unalign` has no alignment requirement, the inner `T` may not be
properly aligned in memory. There are five ways to access the inner `T`:
- by value, using `get` or `into_inner`
- by reference inside of a callback, using `update`
- fallibly by reference, using `try_deref` or `try_deref_mut`; these can
  fail if the `Unalign` does not satisfy `T`'s alignment requirement at
  runtime
- unsafely by reference, using `deref_unchecked` or
  `deref_mut_unchecked`; it is the caller's responsibility to ensure that
  the `Unalign` satisfies `T`'s alignment requirement
- (where `T: Unaligned`) infallibly by reference, using `Deref::deref` or
  `DerefMut::deref_mut`








# Example

In this example, we need `EthernetFrame` to have no alignment requirement -
and thus implement [`Unaligned`](../index.md). `EtherType` is `#[repr(u16)]` and so
cannot implement `Unaligned`. We use `Unalign` to relax `EtherType`'s
alignment requirement so that `EthernetFrame` has no alignment requirement
and can implement `Unaligned`.

```rust
use zerocopy::*;
use zerocopy_derive::*;
#[derive(FromBytes, KnownLayout, Immutable, Unaligned)] #[repr(C)] struct Mac([u8; 6]);

#[derive(PartialEq, Copy, Clone, Debug)]
#[derive(TryFromBytes, KnownLayout, Immutable)]
#[repr(u16)]
enum EtherType {
    Ipv4 = 0x0800u16.to_be(),
    Arp = 0x0806u16.to_be(),
    Ipv6 = 0x86DDu16.to_be(),
    /*
    ...
    */
}

#[derive(TryFromBytes, KnownLayout, Immutable, Unaligned)]
#[repr(C)]
struct EthernetFrame {
    src: Mac,
    dst: Mac,
    ethertype: Unalign<EtherType>,
    payload: [u8],
}

let bytes = &[
    0, 1, 2, 3, 4, 5,
    6, 7, 8, 9, 10, 11,
    /*
    ...
    */
    0x86, 0xDD,            // EtherType
    0xDE, 0xAD, 0xBE, 0xEF // Payload
][..];

// PANICS: Guaranteed not to panic because `bytes` is of the right
// length, has the right contents, and `EthernetFrame` has no
// alignment requirement.
let packet = EthernetFrame::try_ref_from_bytes(&bytes).unwrap();

assert_eq!(packet.ethertype.get(), EtherType::Ipv6);
assert_eq!(packet.payload, [0xDE, 0xAD, 0xBE, 0xEF]);
```

# Safety

`Unalign<T>` is guaranteed to have the same size and bit validity as `T`,
and to have `UnsafeCell`s covering the same byte ranges as `T`.
`Unalign<T>` is guaranteed to have alignment 1.

#### Implementations

- <span id="unalign-new"></span>`const fn new(val: T) -> Unalign<T>` — [`Unalign`](../index.md#unalign)

  Constructs a new `Unalign`.

- <span id="unalign-into-inner"></span>`const fn into_inner(self) -> T`

  Consumes `self`, returning the inner `T`.

- <span id="unalign-try-deref"></span>`fn try_deref(&self) -> Result<&T, AlignmentError<&Self, T>>` — [`AlignmentError`](../index.md#alignmenterror)

  Attempts to return a reference to the wrapped `T`, failing if `self` is

  not properly aligned.

  

  If `self` does not satisfy `align_of::<T>()`, then `try_deref` returns

  `Err`.

  

  If `T: Unaligned`, then `Unalign<T>` implements [`Deref`](../../cpp_demangle/index.md), and callers

  may prefer `Deref::deref`, which is infallible.

- <span id="unalign-try-deref-mut"></span>`fn try_deref_mut(&mut self) -> Result<&mut T, AlignmentError<&mut Self, T>>` — [`AlignmentError`](../index.md#alignmenterror)

  Attempts to return a mutable reference to the wrapped `T`, failing if

  `self` is not properly aligned.

  

  If `self` does not satisfy `align_of::<T>()`, then `try_deref` returns

  `Err`.

  

  If `T: Unaligned`, then `Unalign<T>` implements `DerefMut`, and

  callers may prefer `DerefMut::deref_mut`, which is infallible.

- <span id="unalign-deref-unchecked"></span>`const unsafe fn deref_unchecked(&self) -> &T`

  Returns a reference to the wrapped `T` without checking alignment.

  

  If `T: Unaligned`, then `Unalign<T>` implements[ `Deref`], and callers

  may prefer `Deref::deref`, which is safe.

  

  # Safety

  

  The caller must guarantee that `self` satisfies `align_of::<T>()`.

- <span id="unalign-deref-mut-unchecked"></span>`unsafe fn deref_mut_unchecked(&mut self) -> &mut T`

  Returns a mutable reference to the wrapped `T` without checking

  alignment.

  

  If `T: Unaligned`, then `Unalign<T>` implements[ `DerefMut`], and

  callers may prefer `DerefMut::deref_mut`, which is safe.

  

  # Safety

  

  The caller must guarantee that `self` satisfies `align_of::<T>()`.

- <span id="unalign-get-ptr"></span>`const fn get_ptr(&self) -> *const T`

  Gets an unaligned raw pointer to the inner `T`.

  

  # Safety

  

  The returned raw pointer is not necessarily aligned to

  `align_of::<T>()`. Most functions which operate on raw pointers require

  those pointers to be aligned, so calling those functions with the result

  of `get_ptr` will result in undefined behavior if alignment is not

  guaranteed using some out-of-band mechanism. In general, the only

  functions which are safe to call with this pointer are those which are

  explicitly documented as being sound to use with an unaligned pointer,

  such as `read_unaligned`.

  

  Even if the caller is permitted to mutate `self` (e.g. they have

  ownership or a mutable borrow), it is not guaranteed to be sound to

  write through the returned pointer. If writing is required, prefer

  `get_mut_ptr` instead.

  

- <span id="unalign-get-mut-ptr"></span>`fn get_mut_ptr(&mut self) -> *mut T`

  Gets an unaligned mutable raw pointer to the inner `T`.

  

  # Safety

  

  The returned raw pointer is not necessarily aligned to

  `align_of::<T>()`. Most functions which operate on raw pointers require

  those pointers to be aligned, so calling those functions with the result

  of `get_ptr` will result in undefined behavior if alignment is not

  guaranteed using some out-of-band mechanism. In general, the only

  functions which are safe to call with this pointer are those which are

  explicitly documented as being sound to use with an unaligned pointer,

  such as `read_unaligned`.

- <span id="unalign-set"></span>`fn set(&mut self, t: T)`

  Sets the inner `T`, dropping the previous value.

- <span id="unalign-update"></span>`fn update<O, F: FnOnce(&mut T) -> O>(&mut self, f: F) -> O`

  Updates the inner `T` by calling a function on it.

  

  If `T: Unaligned`, then `Unalign<T>` implements `DerefMut`, and that

  impl should be preferred over this method when performing updates, as it

  will usually be faster and more ergonomic.

  

  For large types, this method may be expensive, as it requires copying

  `2 * size_of::<T>()` bytes. \[1\]

  

  \[1\] Since the inner `T` may not be aligned, it would not be sound to

  invoke `f` on it directly. Instead, `update` moves it into a

  properly-aligned location in the local stack frame, calls `f` on it, and

  then moves it back to its original location in `self`.

#### Trait Implementations

##### `impl<T: Copy> Clone for Unalign<T>`

- <span id="unalign-clone"></span>`fn clone(&self) -> Unalign<T>` — [`Unalign`](../index.md#unalign)

##### `impl<T: marker::Copy> Copy for Unalign<T>`

##### `impl<T: Unaligned + Debug> Debug for Unalign<T>`

- <span id="unalign-debug-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<T: default::Default> Default for Unalign<T>`

- <span id="unalign-default"></span>`fn default() -> Unalign<T>` — [`Unalign`](../index.md#unalign)

##### `impl<T: Unaligned> Deref for Unalign<T>`

- <span id="unalign-deref-type-target"></span>`type Target = T`

- <span id="unalign-deref"></span>`fn deref(&self) -> &T`

##### `impl<T: Unaligned> DerefMut for Unalign<T>`

- <span id="unalign-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut T`

##### `impl<T: Unaligned + Display> Display for Unalign<T>`

- <span id="unalign-display-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<T: Unaligned + Eq> Eq for Unalign<T>`

##### `impl<T> FromBytes for Unalign<T>`

##### `impl<T> FromZeros for Unalign<T>`

##### `impl<T: Unaligned + Hash> Hash for Unalign<T>`

- <span id="unalign-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<T> Immutable for Unalign<T>`

##### `impl<T> IntoBytes for Unalign<T>`

##### `impl<T> KnownLayout for Unalign<T>`

- <span id="unalign-knownlayout-type-pointermetadata"></span>`type PointerMetadata = ()`

##### `impl<T: Unaligned + Ord> Ord for Unalign<T>`

- <span id="unalign-ord-cmp"></span>`fn cmp(&self, other: &Unalign<T>) -> Ordering` — [`Unalign`](../index.md#unalign)

##### `impl<T: Unaligned + PartialEq> PartialEq for Unalign<T>`

- <span id="unalign-partialeq-eq"></span>`fn eq(&self, other: &Unalign<T>) -> bool` — [`Unalign`](../index.md#unalign)

##### `impl<T: Unaligned + PartialOrd> PartialOrd for Unalign<T>`

- <span id="unalign-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Unalign<T>) -> Option<Ordering>` — [`Unalign`](../index.md#unalign)

##### `impl<T> Receiver for Unalign<T>`

- <span id="unalign-receiver-type-target"></span>`type Target = T`

##### `impl<T> ToString for Unalign<T>`

- <span id="unalign-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<T> TryFromBytes for Unalign<T>`

##### `impl<T> Unaligned for Unalign<T>`

