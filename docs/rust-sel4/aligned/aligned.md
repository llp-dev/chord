**aligned**

# Module: aligned

## Contents

**Structs**

- [`A1`](#a1) - 1-byte alignment
- [`A16`](#a16) - 16-byte alignment
- [`A2`](#a2) - 2-byte alignment
- [`A32`](#a32) - 32-byte alignment
- [`A4`](#a4) - 4-byte alignment
- [`A64`](#a64) - 64-byte alignment
- [`A8`](#a8) - 8-byte alignment
- [`Aligned`](#aligned) - A newtype with alignment of at least `A` bytes

**Functions**

- [`Aligned`](#aligned) - Changes the alignment of `value` to be at least `A` bytes

**Traits**

- [`Alignment`](#alignment) - A marker trait for an alignment value.

---

## aligned::A1

*Struct*

1-byte alignment

**Unit Struct**

**Traits:** Alignment, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> A1`



## aligned::A16

*Struct*

16-byte alignment

**Unit Struct**

**Traits:** Alignment, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> A16`



## aligned::A2

*Struct*

2-byte alignment

**Unit Struct**

**Traits:** Copy, Alignment

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> A2`



## aligned::A32

*Struct*

32-byte alignment

**Unit Struct**

**Traits:** Copy, Alignment

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> A32`



## aligned::A4

*Struct*

4-byte alignment

**Unit Struct**

**Traits:** Copy, Alignment

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> A4`



## aligned::A64

*Struct*

64-byte alignment

**Unit Struct**

**Traits:** Alignment, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> A64`



## aligned::A8

*Struct*

8-byte alignment

**Unit Struct**

**Traits:** Alignment, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> A8`



## aligned::Aligned

*Struct*

A newtype with alignment of at least `A` bytes

**Generic Parameters:**
- A
- T

**Traits:** Eq, Copy

**Trait Implementations:**

- **Index**
  - `fn index(self: &Self, range: ops::RangeFull) -> &Aligned<A, [T]>`
- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut T`
- **BorrowMut**
  - `fn borrow_mut(self: & mut Self) -> & mut T`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **Deref**
  - `fn deref(self: &Self) -> &T`
- **Borrow**
  - `fn borrow(self: &Self) -> &T`
- **Index**
  - `fn index(self: &Self, range: ops::Range<usize>) -> &Aligned<A, [T]>`
- **AsMutSlice**
  - `fn as_mut_slice(self: & mut Self) -> & mut [<T as >::Element]`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Index**
  - `fn index(self: &Self, range: ops::RangeInclusive<usize>) -> &Aligned<A, [T]>`
- **AsSlice**
  - `fn as_slice(self: &Self) -> &[<T as >::Element]`
- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Index**
  - `fn index(self: &Self, range: ops::RangeToInclusive<usize>) -> &Aligned<A, [T]>`
- **IndexMut**
  - `fn index_mut(self: & mut Self, range: ops::RangeFull) -> & mut Aligned<A, [T]>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **IndexMut**
  - `fn index_mut(self: & mut Self, range: ops::Range<usize>) -> & mut Aligned<A, [T]>`
- **Index**
  - `fn index(self: &Self, range: ops::RangeTo<usize>) -> &Aligned<A, [T]>`
- **IndexMut**
  - `fn index_mut(self: & mut Self, range: ops::RangeInclusive<usize>) -> & mut Aligned<A, [T]>`
- **Default**
  - `fn default() -> Self`
- **IndexMut**
  - `fn index_mut(self: & mut Self, range: ops::RangeToInclusive<usize>) -> & mut Aligned<A, [T]>`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering>`
- **IndexMut**
  - `fn index_mut(self: & mut Self, range: ops::RangeTo<usize>) -> & mut Aligned<A, [T]>`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Index**
  - `fn index(self: &Self, range: ops::RangeFrom<usize>) -> &Aligned<A, [T]>`
- **BorrowMut**
  - `fn borrow_mut(self: & mut Self) -> & mut [<Aligned<A, T> as AsSlice>::Element]`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> Ordering`
- **IndexMut**
  - `fn index_mut(self: & mut Self, range: ops::RangeFrom<usize>) -> & mut Aligned<A, [T]>`
- **Borrow**
  - `fn borrow(self: &Self) -> &[<Aligned<A, T> as AsSlice>::Element]`



## aligned::Aligned

*Function*

Changes the alignment of `value` to be at least `A` bytes

```rust
fn Aligned<A, T>(value: T) -> Aligned<A, T>
```



## aligned::Alignment

*Trait*

A marker trait for an alignment value.

**Methods:**

- `ALIGN`: The alignment in bytes.



