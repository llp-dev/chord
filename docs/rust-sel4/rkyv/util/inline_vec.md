**rkyv > util > inline_vec**

# Module: util::inline_vec

## Contents

**Structs**

- [`Drain`](#drain) - A draining iterator for `InlineVec<T>`.
- [`InlineVec`](#inlinevec) - A vector that uses inline-allocated memory.

---

## rkyv::util::inline_vec::Drain

*Struct*

A draining iterator for `InlineVec<T>`.

This `struct` is created by [`InlineVec::drain`]. See its documentation for
more.

**Generic Parameters:**
- 'a
- T
- const N



## rkyv::util::inline_vec::InlineVec

*Struct*

A vector that uses inline-allocated memory.

**Generic Parameters:**
- T
- const N

**Methods:**

- `fn assume_init(self: Self) -> InlineVec<T, N>` - Assuming that all the elements are initialized, removes the
- `fn new() -> Self` - Constructs a new, empty `InlineVec`.
- `fn clear(self: & mut Self)` - Clears the vector, removing all values.
- `fn as_mut_ptr(self: & mut Self) -> *mut T` - Returns an unsafe mutable pointer to the vector's buffer.
- `fn as_mut_slice(self: & mut Self) -> & mut [T]` - Extracts a mutable slice of the entire vector.
- `fn as_ptr(self: &Self) -> *const T` - Returns a raw pointer to the vector's buffer.
- `fn as_slice(self: &Self) -> &[T]` - Extracts a slice containing the entire vector.
- `fn capacity(self: &Self) -> usize` - Returns the number of elements the vector can hole without reallocating.
- `fn reserve(self: & mut Self, additional: usize)` - Ensures that there is capacity for at least `additional` more elements
- `fn is_empty(self: &Self) -> bool` - Returns `true` if the vector contains no elements.
- `fn len(self: &Self) -> usize` - Returns the number of elements in the vector, also referred to as its
- `fn extend_from_slice(self: & mut Self, other: &[T])` - Copies and appends all elements in a slice to the `ScratchVec`.
- `fn pop(self: & mut Self) -> Option<T>` - Removes the last element from a vector and returns it, or `None` if it
- `fn push_unchecked(self: & mut Self, value: T)` - Appends an element to the back of a collection without performing bounds
- `fn push(self: & mut Self, value: T)` - Appends an element to the back of a collection.
- `fn reserve_exact(self: & mut Self, additional: usize)` - Reserves the minimum capacity for exactly `additional` more elements to
- `fn set_len(self: & mut Self, new_len: usize)` - Forces the length of the vector to `new_len`.
- `fn drain(self: & mut Self) -> Drain<T, N>` - Creates a draining iterator that removes all of the elements from the

**Traits:** Send, Sync

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **IndexMut**
  - `fn index_mut(self: & mut Self, index: I) -> & mut <Self as >::Output`
- **AsMut**
  - `fn as_mut(self: & mut Self) -> & mut [T]`
- **BorrowMut**
  - `fn borrow_mut(self: & mut Self) -> & mut [T]`
- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut <Self as >::Target`
- **Index**
  - `fn index(self: &Self, index: I) -> &<Self as >::Output`
- **Borrow**
  - `fn borrow(self: &Self) -> &[T]`
- **Default**
  - `fn default() -> Self`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **Drop**
  - `fn drop(self: & mut Self)`
- **AsRef**
  - `fn as_ref(self: &Self) -> &[T]`



