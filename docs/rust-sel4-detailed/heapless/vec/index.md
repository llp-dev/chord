*[heapless](../index.md) / [vec](index.md)*

---

# Module `vec`

A fixed capacity [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html).

## Contents

- [Modules](#modules)
  - [`drain`](#drain)
  - [`storage`](#storage)
- [Structs](#structs)
  - [`Drain`](#drain)
  - [`VecInner`](#vecinner)
  - [`IntoIter`](#intoiter)
- [Traits](#traits)
  - [`VecStorage`](#vecstorage)
- [Type Aliases](#type-aliases)
  - [`OwnedVecStorage`](#ownedvecstorage)
  - [`ViewVecStorage`](#viewvecstorage)
  - [`Vec`](#vec)
  - [`VecView`](#vecview)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`drain`](#drain) | mod |  |
| [`storage`](#storage) | mod |  |
| [`Drain`](#drain) | struct |  |
| [`VecInner`](#vecinner) | struct | Base struct for [`Vec`] and [`VecView`], generic over the [`VecStorage`]. |
| [`IntoIter`](#intoiter) | struct | An iterator that moves out of an [`Vec`][`Vec`]. |
| [`VecStorage`](#vecstorage) | trait |  |
| [`OwnedVecStorage`](#ownedvecstorage) | type |  |
| [`ViewVecStorage`](#viewvecstorage) | type |  |
| [`Vec`](#vec) | type | A fixed capacity [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html). |
| [`VecView`](#vecview) | type | A [`Vec`] with dynamic capacity |

## Modules

- [`drain`](drain/index.md)
- [`storage`](storage/index.md)

## Structs

### `Drain<'a, T: 'a, LenT: LenType>`

```rust
struct Drain<'a, T: 'a, LenT: LenType> {
    tail_start: LenT,
    tail_len: LenT,
    iter: slice::Iter<'a, T>,
    vec: core::ptr::NonNull<super::VecView<T, LenT>>,
}
```

A draining iterator for [`Vec`](super::Vec).

This `struct` is created by [`Vec::drain`](super::Vec::drain).
See its documentation for more.

#### Fields

- **`tail_start`**: `LenT`

  Index of tail to preserve

- **`tail_len`**: `LenT`

  Length of tail

- **`iter`**: `slice::Iter<'a, T>`

  Current remaining range to remove

#### Implementations

- <span id="drain-as-slice"></span>`fn as_slice(&self) -> &[T]`

  Returns the remaining items of this iterator as a slice.

  

  # Examples

  

  ```rust

  use heapless::{vec, Vec};

  

  let mut vec = Vec::<_, 3>::from_array(['a', 'b', 'c']);

  let mut drain = vec.drain(..);

  assert_eq!(drain.as_slice(), &['a', 'b', 'c']);

  let _ = drain.next().unwrap();

  assert_eq!(drain.as_slice(), &['b', 'c']);

  ```

#### Trait Implementations

##### `impl<T, LenT: LenType> AsRef for Drain<'_, T, LenT>`

- <span id="drain-asref-as-ref"></span>`fn as_ref(&self) -> &[T]`

##### `impl<T: fmt::Debug, LenT: LenType> Debug for Drain<'_, T, LenT>`

- <span id="drain-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, LenT: LenType> DoubleEndedIterator for Drain<'_, T, LenT>`

- <span id="drain-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<T>`

##### `impl<T, LenT: LenType> Drop for Drain<'_, T, LenT>`

- <span id="drain-drop"></span>`fn drop(&mut self)`

##### `impl<T, LenT: LenType> ExactSizeIterator for Drain<'_, T, LenT>`

##### `impl<T, LenT: LenType> FusedIterator for Drain<'_, T, LenT>`

##### `impl IntoIterator for Drain<'a, T, LenT>`

- <span id="drain-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="drain-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="drain-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T, LenT: LenType> Iterator for Drain<'_, T, LenT>`

- <span id="drain-iterator-type-item"></span>`type Item = T`

- <span id="drain-iterator-next"></span>`fn next(&mut self) -> Option<T>`

- <span id="drain-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<T: Send, LenT: LenType> Send for Drain<'_, T, LenT>`

##### `impl<T: Sync, LenT: LenType> Sync for Drain<'_, T, LenT>`

### `VecInner<T, LenT: LenType, S: VecStorage<T> + ?Sized>`

```rust
struct VecInner<T, LenT: LenType, S: VecStorage<T> + ?Sized> {
    phantom: core::marker::PhantomData<T>,
    len: LenT,
    buffer: S,
}
```

Base struct for [`Vec`](#vec) and [`VecView`](#vecview), generic over the [`VecStorage`](storage/index.md).

In most cases you should use [`Vec`](#vec) or [`VecView`](#vecview) directly. Only use this
struct if you want to write code that's generic over both.

#### Implementations

- <span id="vecinner-const-elem"></span>`const ELEM: MaybeUninit<T>`

- <span id="vecinner-const-init"></span>`const INIT: [MaybeUninit<T>; N]`

- <span id="vecinner-new"></span>`const fn new() -> Self`

  Constructs a new, empty vector with a fixed capacity of `N`

  

  # Examples

  

  ```rust

  use heapless::Vec;

  

  // allocate the vector on the stack

  let mut x: Vec<u8, 16> = Vec::new();

  

  // allocate the vector in a static variable

  static mut X: Vec<u8, 16> = Vec::new();

  ```

- <span id="vecinner-from-slice"></span>`fn from_slice(other: &[T]) -> Result<Self, CapacityError>` — [`CapacityError`](../index.md#capacityerror)

  Constructs a new vector with a fixed capacity of `N` and fills it

  with the provided slice.

  

  This is equivalent to the following code:

  

  ```rust

  use heapless::Vec;

  

  let mut v: Vec<u8, 16> = Vec::new();

  v.extend_from_slice(&[1, 2, 3]).unwrap();

  ```

- <span id="vecinner-from-array"></span>`fn from_array<const M: usize>(src: [T; M]) -> Self`

  Constructs a new vector with a fixed capacity of `N`, initializing

  it with the provided array.

  

  The length of the provided array, `M` may be equal to _or_ less than

  the capacity of the vector, `N`.

  

  If the length of the provided array is greater than the capacity of the

  vector a compile-time error will be produced.

- <span id="vecinner-into-array"></span>`fn into_array<const M: usize>(self) -> Result<[T; M], Self>`

  Returns the contents of the vector as an array of length `M` if the length

  of the vector is exactly `M`, otherwise returns `Err(self)`.

  

  # Examples

  

  ```rust

  use heapless::Vec;

  let buffer: Vec<u8, 42> = Vec::from_slice(&[1, 2, 3, 5, 8]).unwrap();

  let array: [u8; 5] = buffer.into_array().unwrap();

  assert_eq!(array, [1, 2, 3, 5, 8]);

  ```

- <span id="vecinner-clone"></span>`fn clone(&self) -> Self`

  Clones a vec into a new vec

- <span id="vecinner-cast-len-type"></span>`fn cast_len_type<NewLenT: LenType>(self) -> Vec<T, N, NewLenT>` — [`Vec`](#vec)

  Casts the `LenT` type to a new type, preserving everything else about the vector.

  

  This can be useful if you need to pass a `Vec<T, N, u8>` into a `Vec<T, N, usize>` for example.

  

  This will check at compile time if the `N` value will fit into `NewLenT`, and error if not.

#### Trait Implementations

##### `impl<T, LenT: LenType, S: VecStorage<T> + ?Sized> AsMut for VecInner<T, LenT, S>`

- <span id="vecinner-asmut-as-mut"></span>`fn as_mut(&mut self) -> &mut Self`

##### `impl<T, LenT: LenType, S: VecStorage<T> + ?Sized> AsRef for VecInner<T, LenT, S>`

- <span id="vecinner-asref-as-ref"></span>`fn as_ref(&self) -> &Self`

##### `impl<T, LenT: LenType, S: VecStorage<T> + ?Sized> Debug for VecInner<T, LenT, S>`

- <span id="vecinner-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, LenT: LenType, S: VecStorage<T> + ?Sized> Deref for VecInner<T, LenT, S>`

- <span id="vecinner-deref-type-target"></span>`type Target = [T]`

- <span id="vecinner-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<T, LenT: LenType, S: VecStorage<T> + ?Sized> DerefMut for VecInner<T, LenT, S>`

- <span id="vecinner-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl<T> DeserializeOwned for VecInner<T, LenT, S>`

##### `impl<T, LenT: LenType, S: VecStorage<T> + ?Sized> Drop for VecInner<T, LenT, S>`

- <span id="vecinner-drop"></span>`fn drop(&mut self)`

##### `impl<T, LenT: LenType, S: VecStorage<T> + ?Sized> Eq for VecInner<T, LenT, S>`

##### `impl<T, LenT: LenType, S: VecStorage<T> + ?Sized> Extend for VecInner<T, LenT, S>`

- <span id="vecinner-extend"></span>`fn extend<I>(&mut self, iter: I)`

##### `impl<T, LenT: LenType, S: VecStorage<T> + ?Sized> Hash for VecInner<T, LenT, S>`

- <span id="vecinner-hash"></span>`fn hash<H: hash::Hasher>(&self, state: &mut H)`

##### `impl<T, LenT: LenType, S: VecStorage<T> + ?Sized> IntoIterator for &'a VecInner<T, LenT, S>`

- <span id="a-vecinner-intoiterator-type-item"></span>`type Item = &'a T`

- <span id="a-vecinner-intoiterator-type-intoiter"></span>`type IntoIter = Iter<'a, T>`

- <span id="a-vecinner-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl<T, LenT: LenType, S: VecStorage<T> + ?Sized> Ord for VecInner<T, LenT, S>`

- <span id="vecinner-ord-cmp"></span>`fn cmp(&self, other: &Self) -> Ordering`

##### `impl<A, B, LenTA, LenTB, SA, SB> PartialEq for VecInner<A, LenTA, SA>`

- <span id="vecinner-partialeq-eq"></span>`fn eq(&self, other: &VecInner<B, LenTB, SB>) -> bool` — [`VecInner`](#vecinner)

##### `impl<T, LenTA: LenType, LenTB: LenType, SA: VecStorage<T> + ?Sized, SB: VecStorage<T> + ?Sized> PartialOrd for VecInner<T, LenTB, SB>`

- <span id="vecinner-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &VecInner<T, LenTA, SA>) -> Option<Ordering>` — [`VecInner`](#vecinner)

##### `impl<T> Receiver for VecInner<T, LenT, S>`

- <span id="vecinner-receiver-type-target"></span>`type Target = T`

##### `impl<T, LenT: LenType, St: VecStorage<T>> Serialize for crate::vec::VecInner<T, LenT, St>`

- <span id="cratevecvecinner-serialize"></span>`fn serialize<S>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`

##### `impl<LenT: LenType, S: VecStorage<u8> + ?Sized> Write for VecInner<u8, LenT, S>`

- <span id="vecinner-write-write-str"></span>`fn write_str(&mut self, s: &str) -> fmt::Result`

### `IntoIter<T, const N: usize, LenT: LenType>`

```rust
struct IntoIter<T, const N: usize, LenT: LenType> {
    vec: Vec<T, N, LenT>,
    next: LenT,
}
```

An iterator that moves out of an [`Vec`][`Vec`](#vec).

This struct is created by calling the `into_iter` method on [`Vec`][`Vec`](#vec).

#### Trait Implementations

##### `impl<T, LenT: LenType> Clone for IntoIter<T, N, LenT>`

- <span id="intoiter-clone"></span>`fn clone(&self) -> Self`

##### `impl<T, LenT: LenType> Debug for IntoIter<T, N, LenT>`

- <span id="intoiter-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T, LenT: LenType> Drop for IntoIter<T, N, LenT>`

- <span id="intoiter-drop"></span>`fn drop(&mut self)`

##### `impl IntoIterator for IntoIter<T, N, LenT>`

- <span id="intoiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="intoiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="intoiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T, LenT: LenType> Iterator for IntoIter<T, N, LenT>`

- <span id="intoiter-iterator-type-item"></span>`type Item = T`

- <span id="intoiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

## Traits

### `VecStorage<T>`

```rust
trait VecStorage<T>: VecSealedStorage<T> { ... }
```

Trait defining how data for a container is stored.

There's two implementations available:

- [`OwnedVecStorage`](storage/index.md): stores the data in an array `[T; N]` whose size is known at compile time.
- [`ViewVecStorage`](storage/index.md): stores the data in an unsized `[T]`.

This allows [`Vec`](#vec) to be generic over either sized or unsized storage. The [`vec`](super)
module contains a [`VecInner`](#vecinner) struct that's generic on [`VecStorage`](storage/index.md),
and two type aliases for convenience:

- [`Vec<T, N>`](crate::vec::Vec) = `VecInner<T, OwnedStorage<T, N>>`
- [`VecView<T>`](crate::vec::VecView) = `VecInner<T, ViewStorage<T>>`

`Vec` can be unsized into `VecView`, either by unsizing coercions such as `&mut Vec -> &mut VecView` or
`Box<Vec> -> Box<VecView>`, or explicitly with [`.as_view()`](crate::vec::Vec::as_view) or [`.as_mut_view()`](crate::vec::Vec::as_mut_view).

This trait is sealed, so you cannot implement it for your own types. You can only use
the implementations provided by this crate.




#### Implementors

- [`OwnedVecStorage`](storage/index.md#ownedvecstorage)
- [`ViewVecStorage`](storage/index.md#viewvecstorage)

## Type Aliases

### `OwnedVecStorage<T, const N: usize>`

```rust
type OwnedVecStorage<T, const N: usize> = VecStorageInner<[core::mem::MaybeUninit<T>; N]>;
```

Implementation of [`VecStorage`](storage/index.md) that stores the data in an array `[T; N]` whose size is known at compile time.

### `ViewVecStorage<T>`

```rust
type ViewVecStorage<T> = VecStorageInner<[core::mem::MaybeUninit<T>]>;
```

Implementation of [`VecStorage`](storage/index.md) that stores the data in an unsized `[T]`.

### `Vec<T, const N: usize, LenT>`

```rust
type Vec<T, const N: usize, LenT> = VecInner<T, LenT, OwnedVecStorage<T, N>>;
```

A fixed capacity [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html).

# Examples

```rust
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

In some cases, the const-generic might be cumbersome. `Vec` can coerce into a [`VecView`](#vecview) to remove the need for the const-generic:

```rust
use heapless::{Vec, VecView};

let vec: Vec<u8, 10> = Vec::from_slice(&[1, 2, 3, 4]).unwrap();
let view: &VecView<_, _> = &vec;
```

For uncommmon capacity values, or in generic scenarios, you may have to provide the `LenT` generic yourself.

This should be the smallest unsigned integer type that your capacity fits in, or `usize` if you don't want to consider this.

### `VecView<T, LenT>`

```rust
type VecView<T, LenT> = VecInner<T, LenT, ViewVecStorage<T>>;
```

A [`Vec`](#vec) with dynamic capacity

[`Vec`](#vec) coerces to `VecView`. `VecView` is `!Sized`, meaning it can only ever be used by reference.

Unlike [`Vec`](#vec), `VecView` does not have an `N` const-generic parameter.
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

