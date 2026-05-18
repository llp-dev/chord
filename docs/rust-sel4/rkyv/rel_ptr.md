**rkyv > rel_ptr**

# Module: rel_ptr

## Contents

**Structs**

- [`RawRelPtr`](#rawrelptr) - An untyped pointer which resolves relative to its position in memory.
- [`RelPtr`](#relptr) - A pointer which resolves to relative to its position in memory.

**Functions**

- [`signed_offset`](#signed_offset) - Calculates the offset between two positions as an `isize`.

**Traits**

- [`Offset`](#offset) - A offset that can be used with [`RawRelPtr`].

**Type Aliases**

- [`RawRelPtrI16`](#rawrelptri16) - A raw relative pointer that uses an archived `i16` as the underlying offset.
- [`RawRelPtrI32`](#rawrelptri32) - A raw relative pointer that uses an archived `i32` as the underlying offset.
- [`RawRelPtrI64`](#rawrelptri64) - A raw relative pointer that uses an archived `i64` as the underlying offset.
- [`RawRelPtrI8`](#rawrelptri8) - A raw relative pointer that uses an archived `i8` as the underlying offset.
- [`RawRelPtrU16`](#rawrelptru16) - A raw relative pointer that uses an archived `u16` as the underlying offset.
- [`RawRelPtrU32`](#rawrelptru32) - A raw relative pointer that uses an archived `u32` as the underlying offset.
- [`RawRelPtrU64`](#rawrelptru64) - A raw relative pointer that uses an archived `u64` as the underlying offset.
- [`RawRelPtrU8`](#rawrelptru8) - A raw relative pointer that uses an archived `u8` as the underlying offset.

---

## rkyv::rel_ptr::Offset

*Trait*

A offset that can be used with [`RawRelPtr`].

**Methods:**

- `from_isize`: Creates a new offset between a `from` position and a `to` position.
- `to_isize`: Gets the offset as an `isize`.



## rkyv::rel_ptr::RawRelPtr

*Struct*

An untyped pointer which resolves relative to its position in memory.

This is the most fundamental building block in rkyv. It allows the
construction and use of pointers that can be safely relocated as long as the
source and target are moved together. This is what allows memory to be moved
from disk into memory and accessed without decoding.

Regular pointers are *absolute*, meaning that the pointer can be moved
without being invalidated. However, the pointee **cannot** be moved,
otherwise the pointer is invalidated.

Relative pointers are *relative*, meaning that the **pointer** can be moved
with the **pointee** without invalidating the pointer. However, if either
the **pointer** or the **pointee** move independently, the pointer will be
invalidated.

**Generic Parameters:**
- O

**Methods:**

- `fn try_emplace_invalid<E>(out: Place<Self>) -> Result<(), E>` - Attempts to create an invalid `RawRelPtr` in-place.
- `fn emplace_invalid(out: Place<Self>)` - Creates an invalid `RawRelPtr` in-place.
- `fn try_emplace<E>(to: usize, out: Place<Self>) -> Result<(), E>` - Attempts to create a new `RawRelPtr` in-place between the given `from`
- `fn emplace(to: usize, out: Place<Self>)` - Creates a new `RawRelPtr` in-place between the given `from` and `to`
- `fn base_raw(this: *mut Self) -> *mut u8` - Gets the base pointer for the pointed-to relative pointer.
- `fn offset_raw(this: *mut Self) -> isize` - Gets the offset of the pointed-to relative pointer from its base.
- `fn as_ptr_raw(this: *mut Self) -> *mut ()` - Calculates the memory address being pointed to by the pointed-to
- `fn as_ptr_wrapping_raw(this: *mut Self) -> *mut ()` - Calculates the memory address being pointed to by the pointed-to
- `fn is_invalid_raw(this: *mut Self) -> bool` - Gets whether the offset of the pointed-to relative pointer is invalid.
- `fn base(self: &Self) -> *const u8` - Gets the base pointer for the relative pointer.
- `fn base_mut(this: Seal<Self>) -> *mut u8` - Gets the mutable base pointer for the relative pointer.
- `fn offset(self: &Self) -> isize` - Gets the offset of the relative pointer from its base.
- `fn is_invalid(self: &Self) -> bool` - Gets whether the offset of the relative pointer is invalid.
- `fn as_ptr(self: &Self) -> *const ()` - Calculates the memory address being pointed to by this relative pointer.
- `fn as_mut_ptr(this: Seal<Self>) -> *mut ()` - Calculates the mutable memory address being pointed to by this relative
- `fn as_ptr_wrapping(self: &Self) -> *const ()` - Calculates the memory address being pointed to by this relative pointer
- `fn as_mut_ptr_wrapping(this: Seal<Self>) -> *mut ()` - Calculates the mutable memory address being pointed to by this relative

**Traits:** Portable

**Trait Implementations:**

- **Pointer**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`



## rkyv::rel_ptr::RawRelPtrI16

*Type Alias*: `RawRelPtr<crate::primitive::ArchivedI16>`

A raw relative pointer that uses an archived `i16` as the underlying offset.



## rkyv::rel_ptr::RawRelPtrI32

*Type Alias*: `RawRelPtr<crate::primitive::ArchivedI32>`

A raw relative pointer that uses an archived `i32` as the underlying offset.



## rkyv::rel_ptr::RawRelPtrI64

*Type Alias*: `RawRelPtr<crate::primitive::ArchivedI64>`

A raw relative pointer that uses an archived `i64` as the underlying offset.



## rkyv::rel_ptr::RawRelPtrI8

*Type Alias*: `RawRelPtr<i8>`

A raw relative pointer that uses an archived `i8` as the underlying offset.



## rkyv::rel_ptr::RawRelPtrU16

*Type Alias*: `RawRelPtr<crate::primitive::ArchivedU16>`

A raw relative pointer that uses an archived `u16` as the underlying offset.



## rkyv::rel_ptr::RawRelPtrU32

*Type Alias*: `RawRelPtr<crate::primitive::ArchivedU32>`

A raw relative pointer that uses an archived `u32` as the underlying offset.



## rkyv::rel_ptr::RawRelPtrU64

*Type Alias*: `RawRelPtr<crate::primitive::ArchivedU64>`

A raw relative pointer that uses an archived `u64` as the underlying offset.



## rkyv::rel_ptr::RawRelPtrU8

*Type Alias*: `RawRelPtr<u8>`

A raw relative pointer that uses an archived `u8` as the underlying offset.



## rkyv::rel_ptr::RelPtr

*Struct*

A pointer which resolves to relative to its position in memory.

This is a strongly-typed version of [`RawRelPtr`].

See [`Archive`](crate::Archive) for an example of creating one.

**Generic Parameters:**
- T
- O

**Methods:**

- `fn try_emplace<E>(to: usize, out: Place<Self>) -> Result<(), E>` - Attempts to create a relative pointer from one position to another.
- `fn emplace(to: usize, out: Place<Self>)` - Creates a relative pointer from one position to another.
- `fn try_emplace_invalid<E>(out: Place<Self>) -> Result<(), E>` - Attempts to create an invalid relative pointer with default metadata.
- `fn emplace_invalid(out: Place<Self>)` - Creates an invalid relative pointer with default metadata.
- `fn try_emplace_unsized<E>(to: usize, metadata: <T as >::ArchivedMetadata, out: Place<Self>) -> Result<(), E>` - Attempts to create a relative pointer from one position to another.
- `fn emplace_unsized(to: usize, metadata: <T as >::ArchivedMetadata, out: Place<Self>)` - Creates a relative pointer from one position to another.
- `fn base_raw(this: *mut Self) -> *mut u8` - Gets the base pointer for the pointed-to relative pointer.
- `fn offset_raw(this: *mut Self) -> isize` - Gets the offset of the pointed-to relative pointer from its base.
- `fn as_ptr_raw(this: *mut Self) -> *mut T` - Calculates the memory address being pointed to by the pointed-to
- `fn as_ptr_wrapping_raw(this: *mut Self) -> *mut T` - Calculates the memory address being pointed to by the pointed-to
- `fn is_invalid_raw(this: *mut Self) -> bool` - Gets whether the offset of the pointed-to relative pointer is invalid.
- `fn base(self: &Self) -> *const u8` - Gets the base pointer for the relative pointer.
- `fn base_mut(this: Seal<Self>) -> *mut u8` - Gets the mutable base pointer for this relative pointer.
- `fn offset(self: &Self) -> isize` - Gets the offset of the relative pointer from its base.
- `fn is_invalid(self: &Self) -> bool` - Gets whether the offset of the relative pointer is 0.
- `fn metadata(self: &Self) -> &<T as >::ArchivedMetadata` - Gets the metadata of the relative pointer.
- `fn as_ptr(self: &Self) -> *const T` - Calculates the memory address being pointed to by this relative pointer.
- `fn as_mut_ptr(this: Seal<Self>) -> *mut T` - Calculates the mutable memory address being pointed to by this relative
- `fn as_ptr_wrapping(self: &Self) -> *const T` - Calculates the memory address being pointed to by this relative pointer
- `fn as_mut_ptr_wrapping(this: Seal<Self>) -> *mut T` - Calculates the mutable memory address being pointed to by this relative

**Traits:** Portable

**Trait Implementations:**

- **Pointer**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## rkyv::rel_ptr::signed_offset

*Function*

Calculates the offset between two positions as an `isize`.

This function exists solely to get the distance between two `usizes` as an
`isize` with a full range of values.

# Examples

```
# use rkyv::rel_ptr::signed_offset;
# use rancor::Error;
assert!(signed_offset::<Error>(0, 1).is_ok_and(|x| x == 1));
assert!(signed_offset::<Error>(1, 0).is_ok_and(|x| x == -1));
assert!(signed_offset::<Error>(0, isize::MAX as usize)
    .is_ok_and(|x| x == isize::MAX));
assert!(signed_offset::<Error>(isize::MAX as usize, 0)
    .is_ok_and(|x| x == -isize::MAX));
assert!(signed_offset::<Error>(0, isize::MAX as usize + 1).is_err());
assert!(signed_offset::<Error>(isize::MAX as usize + 1, 0)
    .is_ok_and(|x| x == isize::MIN));
assert!(signed_offset::<Error>(0, isize::MAX as usize + 2).is_err());
assert!(signed_offset::<Error>(isize::MAX as usize + 2, 0).is_err());
```

```rust
fn signed_offset<E>(from: usize, to: usize) -> Result<isize, E>
```



