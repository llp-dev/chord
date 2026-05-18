**allocator_api2 > stable > boxed**

# Module: stable::boxed

## Contents

**Structs**

- [`Box`](#box) - A pointer type for heap allocation.

---

## allocator_api2::stable::boxed::Box

*Struct*

A pointer type for heap allocation.

See the [module-level documentation](../../std/boxed/index.html) for more.

**Generic Parameters:**
- T
- A

**Tuple Struct**: `()`

**Methods:**

- `fn new_uninit_slice(len: usize) -> Box<[mem::MaybeUninit<T>]>` - Constructs a new boxed slice with uninitialized contents.
- `fn new_zeroed_slice(len: usize) -> Box<[mem::MaybeUninit<T>]>` - Constructs a new boxed slice with uninitialized contents, with the memory
- `fn try_new_uninit_slice(len: usize) -> Result<Box<[mem::MaybeUninit<T>]>, AllocError>` - Constructs a new boxed slice with uninitialized contents. Returns an error if
- `fn try_new_zeroed_slice(len: usize) -> Result<Box<[mem::MaybeUninit<T>]>, AllocError>` - Constructs a new boxed slice with uninitialized contents, with the memory
- `fn from_raw_in(raw: *mut T, alloc: A) -> Self` - Constructs a box from a raw pointer in the given allocator.
- `fn into_raw(b: Self) -> *mut T` - Consumes the `Box`, returning a wrapped raw pointer.
- `fn into_raw_with_allocator(b: Self) -> (*mut T, A)` - Consumes the `Box`, returning a wrapped raw pointer and the allocator.
- `fn into_non_null(b: Self) -> (NonNull<T>, A)`
- `fn allocator(b: &Self) -> &A` - Returns a reference to the underlying allocator.
- `fn leak<'a>(b: Self) -> &'a  mut T` - Consumes and leaks the `Box`, returning a mutable reference,
- `fn into_pin(boxed: Self) -> Pin<Self>` - Converts a `Box<T>` into a `Pin<Box<T>>`. If `T` does not implement [`Unpin`], then
- `fn from_raw(raw: *mut T) -> Self` - Constructs a box from a raw pointer.
- `fn assume_init(self: Self) -> Box<[T], A>` - Converts to `Box<[T], A>`.
- `fn assume_init(self: Self) -> Box<T, A>` - Converts to `Box<T, A>`.
- `fn write(boxed: Self, value: T) -> Box<T, A>` - Writes the value and converts to `Box<T, A>`.
- `fn new_uninit_slice_in(len: usize, alloc: A) -> Box<[mem::MaybeUninit<T>], A>` - Constructs a new boxed slice with uninitialized contents in the provided allocator.
- `fn new_zeroed_slice_in(len: usize, alloc: A) -> Box<[mem::MaybeUninit<T>], A>` - Constructs a new boxed slice with uninitialized contents in the provided allocator,
- `fn try_new_uninit_slice_in(len: usize, alloc: A) -> Result<Box<[MaybeUninit<T>], A>, AllocError>` - Constructs a new boxed slice with uninitialized contents in the provided allocator. Returns an error if
- `fn try_new_zeroed_slice_in(len: usize, alloc: A) -> Result<Box<[mem::MaybeUninit<T>], A>, AllocError>` - Constructs a new boxed slice with uninitialized contents in the provided allocator, with the memory
- `fn into_vec(self: Self) -> Vec<T, A>` - Converts `self` into a vector without clones or allocation.
- `fn downcast<T>(self: Self) -> Result<Box<T, A>, Self>` - Attempt to downcast the box to a concrete type.
- `fn downcast_unchecked<T>(self: Self) -> Box<T, A>` - Downcasts the box to a concrete type.
- `fn slice(b: Self) -> Box<[T], A>`
- `fn into_vec(self: Self) -> Vec<T, A>`
- `fn new_in(x: T, alloc: A) -> Self` - Allocates memory in the given allocator then places `x` into it.
- `fn try_new_in(x: T, alloc: A) -> Result<Self, AllocError>` - Allocates memory in the given allocator then places `x` into it,
- `fn new_uninit_in(alloc: A) -> Box<mem::MaybeUninit<T>, A>` - Constructs a new box with uninitialized contents in the provided allocator.
- `fn try_new_uninit_in(alloc: A) -> Result<Box<mem::MaybeUninit<T>, A>, AllocError>` - Constructs a new box with uninitialized contents in the provided allocator,
- `fn new_zeroed_in(alloc: A) -> Box<mem::MaybeUninit<T>, A>` - Constructs a new `Box` with uninitialized contents, with the memory
- `fn try_new_zeroed_in(alloc: A) -> Result<Box<mem::MaybeUninit<T>, A>, AllocError>` - Constructs a new `Box` with uninitialized contents, with the memory
- `fn pin_in(x: T, alloc: A) -> Pin<Self>` - Constructs a new `Pin<Box<T, A>>`. If `T` does not implement [`Unpin`], then
- `fn into_boxed_slice(boxed: Self) -> Box<[T], A>` - Converts a `Box<T>` into a `Box<[T]>`
- `fn into_inner(boxed: Self) -> T` - Consumes the `Box`, returning the wrapped value.
- `fn new(x: T) -> Self` - Allocates memory on the heap and then places `x` into it.
- `fn new_uninit() -> Box<mem::MaybeUninit<T>>` - Constructs a new box with uninitialized contents.
- `fn new_zeroed() -> Box<mem::MaybeUninit<T>>` - Constructs a new `Box` with uninitialized contents, with the memory
- `fn pin(x: T) -> Pin<Box<T>>` - Constructs a new `Pin<Box<T>>`. If `T` does not implement [`Unpin`], then
- `fn try_new(x: T) -> Result<Self, AllocError>` - Allocates memory on the heap then places `x` into it,
- `fn try_new_uninit() -> Result<Box<mem::MaybeUninit<T>>, AllocError>` - Constructs a new box with uninitialized contents on the heap,
- `fn try_new_zeroed() -> Result<Box<mem::MaybeUninit<T>>, AllocError>` - Constructs a new `Box` with uninitialized contents, with the memory
- `fn downcast<T>(self: Self) -> Result<Box<T, A>, Self>` - Attempt to downcast the box to a concrete type.
- `fn downcast_unchecked<T>(self: Self) -> Box<T, A>` - Downcasts the box to a concrete type.
- `fn downcast<T>(self: Self) -> Result<Box<T, A>, Self>` - Attempt to downcast the box to a concrete type.
- `fn downcast_unchecked<T>(self: Self) -> Box<T, A>` - Downcasts the box to a concrete type.

**Traits:** Sync, Eq, Send, FusedIterator

**Trait Implementations:**

- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<I as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
  - `fn nth(self: & mut Self, n: usize) -> Option<<I as >::Item>`
  - `fn last(self: Self) -> Option<<I as >::Item>`
- **AsRef**
  - `fn as_ref(self: &Self) -> &T`
- **Drop**
  - `fn drop(self: & mut Self)`
- **BorrowMut**
  - `fn borrow_mut(self: & mut Self) -> & mut T`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering>`
  - `fn lt(self: &Self, other: &Self) -> bool`
  - `fn le(self: &Self, other: &Self) -> bool`
  - `fn ge(self: &Self, other: &Self) -> bool`
  - `fn gt(self: &Self, other: &Self) -> bool`
- **Borrow**
  - `fn borrow(self: &Self) -> &T`
- **Clone**
  - `fn clone(self: &Self) -> Self`
  - `fn clone_from(self: & mut Self, other: &Self)`
- **From**
  - `fn from(v: Vec<T, A>) -> Self` - Convert a vector into a boxed slice.
- **FromIterator**
  - `fn from_iter<T>(iter: T) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
  - `fn ne(self: &Self, other: &Self) -> bool`
- **From**
  - `fn from(array: [T; N]) -> Box<[T]>` - Converts a `[T; N]` into a `Box<[T]>`
- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<<I as >::Item>`
  - `fn nth_back(self: & mut Self, n: usize) -> Option<<I as >::Item>`
- **Default**
  - `fn default() -> Self`
- **From**
  - `fn from(s: Box<str, A>) -> Self` - Converts a `Box<str>` into a `Box<[u8]>`
- **Default**
  - `fn default() -> Self`
- **From**
  - `fn from(s: &str) -> Box<str, A>` - Converts a `&str` into a `Box<str>`
- **AsMut**
  - `fn as_mut(self: & mut Self) -> & mut T`
- **Default**
  - `fn default() -> Self` - Creates a `Box<T>`, with the `Default` value for T.
- **From**
  - `fn from(slice: &[T]) -> Box<[T], A>` - Converts a `&[T]` into a `Box<[T]>`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> Ordering`
- **From**
  - `fn from(t: T) -> Self` - Converts a `T` into a `Box<T>`
- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut T`
- **Hasher**
  - `fn finish(self: &Self) -> u64`
  - `fn write(self: & mut Self, bytes: &[u8])`
  - `fn write_u8(self: & mut Self, i: u8)`
  - `fn write_u16(self: & mut Self, i: u16)`
  - `fn write_u32(self: & mut Self, i: u32)`
  - `fn write_u64(self: & mut Self, i: u64)`
  - `fn write_u128(self: & mut Self, i: u128)`
  - `fn write_usize(self: & mut Self, i: usize)`
  - `fn write_i8(self: & mut Self, i: i8)`
  - `fn write_i16(self: & mut Self, i: i16)`
  - `fn write_i32(self: & mut Self, i: i32)`
  - `fn write_i64(self: & mut Self, i: i64)`
  - `fn write_i128(self: & mut Self, i: i128)`
  - `fn write_isize(self: & mut Self, i: isize)`
- **Deref**
  - `fn deref(self: &Self) -> &T`
- **Pointer**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **TryFrom**
  - `fn try_from(boxed_slice: Box<[T], A>) -> Result<Self, <Self as >::Error>` - Attempts to convert a `Box<[T]>` into a `Box<[T; N]>`.
- **ExactSizeIterator**
  - `fn len(self: &Self) -> usize`
- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut Context) -> Poll<<Self as >::Output>`
- **Clone**
  - `fn clone(self: &Self) -> Self` - Returns a new box with a `clone()` of this box's contents.
  - `fn clone_from(self: & mut Self, source: &Self)` - Copies `source`'s contents into `self` without creating a new allocation.



