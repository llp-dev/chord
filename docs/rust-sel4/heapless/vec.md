**heapless > vec**

# Module: vec

## Contents

**Structs**

- [`IntoIter`](#intoiter) - An iterator that moves out of an [`Vec`][`Vec`].
- [`VecInner`](#vecinner) - Base struct for [`Vec`] and [`VecView`], generic over the [`VecStorage`].

**Type Aliases**

- [`Vec`](#vec) - A fixed capacity [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html).
- [`VecView`](#vecview) - A [`Vec`] with dynamic capacity

---

## heapless::vec::IntoIter

*Struct*

An iterator that moves out of an [`Vec`][`Vec`].

This struct is created by calling the `into_iter` method on [`Vec`][`Vec`].

**Generic Parameters:**
- T
- const N
- LenT

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
- **Drop**
  - `fn drop(self: & mut Self)`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## heapless::vec::Vec

*Type Alias*: `VecInner<T, LenT, OwnedVecStorage<T, N>>`

A fixed capacity [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html).

# Examples

```
use heapless::Vec;

// A vector with a fixed capacity of 8 elements allocated on the stack
let mut vec = Vec::<_, 8>::new();
vec.push(1).unwrap();
vec.push(2).unwrap();

assert_eq!(vec.len(), 2);
assert_eq!(vec[0], 1);

assert_eq!(vec.pop(), Some(2));
assert_eq!(vec.len(), 1);

vec[0] = 7;
assert_eq!(vec[0], 7);

vec.extend([1, 2, 3].iter().cloned());

for x in &vec {
    println!("{}", x);
}
assert_eq!(*vec, [7, 1, 2, 3]);
```

In some cases, the const-generic might be cumbersome. `Vec` can coerce into a [`VecView`] to remove the need for the const-generic:

```rust
use heapless::{Vec, VecView};

let vec: Vec<u8, 10> = Vec::from_slice(&[1, 2, 3, 4]).unwrap();
let view: &VecView<_, _> = &vec;
```

For uncommmon capacity values, or in generic scenarios, you may have to provide the `LenT` generic yourself.

This should be the smallest unsigned integer type that your capacity fits in, or `usize` if you don't want to consider this.



## heapless::vec::VecInner

*Struct*

Base struct for [`Vec`] and [`VecView`], generic over the [`VecStorage`].

In most cases you should use [`Vec`] or [`VecView`] directly. Only use this
struct if you want to write code that's generic over both.

**Generic Parameters:**
- T
- LenT
- S

**Methods:**

- `fn drain<R>(self: & mut Self, range: R) -> Drain<T, LenT>` - Removes the specified range from the vector in bulk, returning all
- `fn as_view(self: &Self) -> &VecView<T, LenT>` - Get a reference to the `Vec`, erasing the `N` const-generic.
- `fn as_mut_view(self: & mut Self) -> & mut VecView<T, LenT>` - Get a mutable reference to the `Vec`, erasing the `N` const-generic.
- `fn as_ptr(self: &Self) -> *const T` - Returns a raw pointer to the vector’s buffer.
- `fn as_mut_ptr(self: & mut Self) -> *mut T` - Returns a raw pointer to the vector’s buffer, which may be mutated through.
- `fn as_slice(self: &Self) -> &[T]` - Extracts a slice containing the entire vector.
- `fn as_mut_slice(self: & mut Self) -> & mut [T]` - Extracts a mutable slice containing the entire vector.
- `fn capacity(self: &Self) -> usize` - Returns the maximum number of elements the vector can hold.
- `fn clear(self: & mut Self)` - Clears the vector, removing all values.
- `fn extend<I>(self: & mut Self, iter: I)` - Extends the vec from an iterator.
- `fn extend_from_slice(self: & mut Self, other: &[T]) -> Result<(), CapacityError>` - Clones and appends all elements in a slice to the `Vec`.
- `fn pop(self: & mut Self) -> Option<T>` - Removes the last element from a vector and returns it, or `None` if it's empty
- `fn push(self: & mut Self, item: T) -> Result<(), T>` - Appends an `item` to the back of the collection
- `fn pop_unchecked(self: & mut Self) -> T` - Removes the last element from a vector and returns it
- `fn push_unchecked(self: & mut Self, item: T)` - Appends an `item` to the back of the collection
- `fn truncate(self: & mut Self, len: usize)` - Shortens the vector, keeping the first `len` elements and dropping the rest.
- `fn resize(self: & mut Self, new_len: usize, value: T) -> Result<(), CapacityError>` - Resizes the Vec in-place so that len is equal to `new_len`.
- `fn resize_default(self: & mut Self, new_len: usize) -> Result<(), CapacityError>` - Resizes the `Vec` in-place so that `len` is equal to `new_len`.
- `fn set_len(self: & mut Self, new_len: usize)` - Forces the length of the vector to `new_len`.
- `fn swap_remove(self: & mut Self, index: usize) -> T` - Removes an element from the vector and returns it.
- `fn swap_remove_unchecked(self: & mut Self, index: usize) -> T` - Removes an element from the vector and returns it.
- `fn is_full(self: &Self) -> bool` - Returns true if the vec is full
- `fn is_empty(self: &Self) -> bool` - Returns true if the vec is empty
- `fn starts_with(self: &Self, needle: &[T]) -> bool` - Returns `true` if `needle` is a prefix of the Vec.
- `fn ends_with(self: &Self, needle: &[T]) -> bool` - Returns `true` if `needle` is a suffix of the Vec.
- `fn insert(self: & mut Self, index: usize, element: T) -> Result<(), T>` - Inserts an element at position `index` within the vector, shifting all
- `fn remove(self: & mut Self, index: usize) -> T` - Removes and returns the element at position `index` within the vector,
- `fn retain<F>(self: & mut Self, f: F)` - Retains only the elements specified by the predicate.
- `fn retain_mut<F>(self: & mut Self, f: F)` - Retains only the elements specified by the predicate, passing a mutable reference to it.
- `fn spare_capacity_mut(self: & mut Self) -> & mut [MaybeUninit<T>]` - Returns the remaining spare capacity of the vector as a slice of `MaybeUninit<T>`.
- `fn new() -> Self` - Constructs a new, empty vector with a fixed capacity of `N`
- `fn from_slice(other: &[T]) -> Result<Self, CapacityError>` - Constructs a new vector with a fixed capacity of `N` and fills it
- `fn from_array<const M>(src: [T; M]) -> Self` - Constructs a new vector with a fixed capacity of `N`, initializing
- `fn into_array<const M>(self: Self) -> Result<[T; M], Self>` - Returns the contents of the vector as an array of length `M` if the length
- `fn cast_len_type<NewLenT>(self: Self) -> Vec<T, N, NewLenT>` - Casts the `LenT` type to a new type, preserving everything else about the vector.

**Traits:** Eq

**Trait Implementations:**

- **Serialize**
  - `fn serialize<S>(self: &Self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`
- **PartialEq**
  - `fn eq(self: &Self, other: &&[B]) -> bool`
- **AsRef**
  - `fn as_ref(self: &Self) -> &[T]`
- **PartialEq**
  - `fn eq(self: &Self, other: &&[B; N]) -> bool`
- **Ord**
  - `fn cmp(self: &Self, other: &Self) -> Ordering`
- **Extend**
  - `fn extend<I>(self: & mut Self, iter: I)`
- **AsRef**
  - `fn as_ref(self: &Self) -> &Self`
- **Drop**
  - `fn drop(self: & mut Self)`
- **Borrow**
  - `fn borrow(self: &Self) -> &[T]`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Deref**
  - `fn deref(self: &Self) -> &<Self as >::Target`
- **PartialEq**
  - `fn eq(self: &Self, other: && mut [B]) -> bool`
- **AsMut**
  - `fn as_mut(self: & mut Self) -> & mut [T]`
- **PartialEq**
  - `fn eq(self: &Self, other: &[B]) -> bool`
- **PartialEq**
  - `fn eq(self: &Self, other: &[B; N]) -> bool`
- **PartialEq**
  - `fn eq(self: &Self, other: &VecInner<B, LenTB, SB>) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &VecInner<T, LenTA, SA>) -> Option<Ordering>`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`
- **AsMut**
  - `fn as_mut(self: & mut Self) -> & mut Self`
- **Extend**
  - `fn extend<I>(self: & mut Self, iter: I)`
- **BorrowMut**
  - `fn borrow_mut(self: & mut Self) -> & mut [T]`
- **Write**
  - `fn write_str(self: & mut Self, s: &str) -> fmt::Result`
- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut <Self as >::Target`



## heapless::vec::VecView

*Type Alias*: `VecInner<T, LenT, ViewVecStorage<T>>`

A [`Vec`] with dynamic capacity

[`Vec`] coerces to `VecView`. `VecView` is `!Sized`, meaning it can only ever be used by reference.

Unlike [`Vec`], `VecView` does not have an `N` const-generic parameter.
This has the ergonomic advantage of making it possible to use functions without needing to know at
compile-time the size of the buffers used, for example for use in `dyn` traits.

`VecView<T>` is to `Vec<T, N>` what `[T]` is to `[T; N]`.

```rust
use heapless::{Vec, VecView};

let mut vec: Vec<u8, 10> = Vec::from_slice(&[1, 2, 3, 4]).unwrap();
let view: &VecView<_, _> = &vec;
assert_eq!(view, &[1, 2, 3, 4]);

let mut_view: &mut VecView<_, _> = &mut vec;
mut_view.push(5);
assert_eq!(vec, [1, 2, 3, 4, 5]);
```



