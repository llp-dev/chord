**rkyv > util > alloc > aligned_vec**

# Module: util::alloc::aligned_vec

## Contents

**Structs**

- [`AlignedVec`](#alignedvec) - A vector of bytes that aligns its memory to the specified alignment.

---

## rkyv::util::alloc::aligned_vec::AlignedVec

*Struct*

A vector of bytes that aligns its memory to the specified alignment.

```
# use rkyv::util::AlignedVec;
let bytes = AlignedVec::<4096>::with_capacity(1);
assert_eq!(bytes.as_ptr() as usize % 4096, 0);
```

**Generic Parameters:**
- const ALIGNMENT

**Methods:**

- `fn new() -> Self` - Constructs a new, empty `AlignedVec`.
- `fn with_capacity(capacity: usize) -> Self` - Constructs a new, empty `AlignedVec` with the specified capacity.
- `fn clear(self: & mut Self)` - Clears the vector, removing all values.
- `fn change_capacity(self: & mut Self, new_cap: usize)` - Change capacity of vector.
- `fn shrink_to_fit(self: & mut Self)` - Shrinks the capacity of the vector as much as possible.
- `fn as_mut_ptr(self: & mut Self) -> *mut u8` - Returns an unsafe mutable pointer to the vector's buffer.
- `fn as_mut_slice(self: & mut Self) -> & mut [u8]` - Extracts a mutable slice of the entire vector.
- `fn as_ptr(self: &Self) -> *const u8` - Returns a raw pointer to the vector's buffer.
- `fn as_slice(self: &Self) -> &[u8]` - Extracts a slice containing the entire vector.
- `fn capacity(self: &Self) -> usize` - Returns the number of elements the vector can hold without reallocating.
- `fn reserve(self: & mut Self, additional: usize)` - Reserves capacity for at least `additional` more bytes to be inserted
- `fn grow_capacity_to(self: & mut Self, new_cap: usize)` - Grows total capacity of vector to `new_cap` or more.
- `fn resize(self: & mut Self, new_len: usize, value: u8)` - Resizes the Vec in-place so that len is equal to new_len.
- `fn is_empty(self: &Self) -> bool` - Returns `true` if the vector contains no elements.
- `fn len(self: &Self) -> usize` - Returns the number of elements in the vector, also referred to as its
- `fn extend_from_slice(self: & mut Self, other: &[u8])` - Copies and appends all bytes in a slice to the `AlignedVec`.
- `fn pop(self: & mut Self) -> Option<u8>` - Removes the last element from a vector and returns it, or `None` if it
- `fn push(self: & mut Self, value: u8)` - Appends an element to the back of a collection.
- `fn reserve_exact(self: & mut Self, additional: usize)` - Reserves the minimum capacity for exactly `additional` more elements to
- `fn set_len(self: & mut Self, new_len: usize)` - Forces the length of the vector to `new_len`.
- `fn into_boxed_slice(self: Self) -> Box<[u8]>` - Converts the vector into `Box<[u8]>`. The returned slice is 1-aligned.
- `fn into_vec(self: Self) -> Vec<u8>` - Converts the vector into `Vec<u8>`.
- `fn extend_from_reader<R>(self: & mut Self, r: & mut R) -> io::Result<usize>` - Reads all bytes until EOF from `r` and appends them to this

**Traits:** Send, Sync

**Trait Implementations:**

- **BorrowMut**
  - `fn borrow_mut(self: & mut Self) -> & mut [u8]`
- **Default**
  - `fn default() -> Self`
- **Writer**
  - `fn write(self: & mut Self, bytes: &[u8]) -> Result<(), E>`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **Drop**
  - `fn drop(self: & mut Self)`
- **Write**
  - `fn write(self: & mut Self, buf: &[u8]) -> io::Result<usize>`
  - `fn write_vectored(self: & mut Self, bufs: &[io::IoSlice]) -> io::Result<usize>`
  - `fn write_all(self: & mut Self, buf: &[u8]) -> io::Result<()>`
  - `fn flush(self: & mut Self) -> io::Result<()>`
- **Positional**
  - `fn pos(self: &Self) -> usize`
- **Borrow**
  - `fn borrow(self: &Self) -> &[u8]`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **IndexMut**
  - `fn index_mut(self: & mut Self, index: I) -> & mut <Self as >::Output`
- **AsMut**
  - `fn as_mut(self: & mut Self) -> & mut [u8]`
- **AsRef**
  - `fn as_ref(self: &Self) -> &[u8]`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut <Self as >::Target`
- **Index**
  - `fn index(self: &Self, index: I) -> &<Self as >::Output`



