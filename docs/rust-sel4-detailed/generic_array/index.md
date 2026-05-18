# Crate `generic_array`

This crate implements a structure that can be used as a generic array type.
Core Rust array types `[T; N]` can't be used generically with
respect to `N`, so for example this:

```rust{compile_fail}
struct Foo<T, N> {
    data: [T; N]
}
```

won't work.

**generic-array** exports a `GenericArray<T,N>` type, which lets
the above be implemented as:

```rust
use generic_array::{ArrayLength, GenericArray};

struct Foo<T, N: ArrayLength<T>> {
    data: GenericArray<T,N>
}
```

The `ArrayLength<T>` trait is implemented by default for
[unsigned integer types](../typenum/uint/index.html) from

# use generic_array::{ArrayLength, GenericArray};
use generic_array::typenum::U5;

struct Foo<N: ArrayLength<i32>> {
    data: GenericArray<i32, N>
}

# fn main() {
let foo = Foo::<U5>{data: GenericArray::default()};
# }
```rust

For example, `GenericArray<T, U5>` would work almost like `[T; 5]`:

```rust
# use generic_array::{ArrayLength, GenericArray};
use generic_array::typenum::U5;

struct Foo<T, N: ArrayLength<T>> {
    data: GenericArray<T, N>
}

# fn main() {
let foo = Foo::<i32, U5>{data: GenericArray::default()};
# }
```rust

For ease of use, an `arr!` macro is provided - example below:

```
# #[macro_use]
# extern crate generic_array;
# extern crate typenum;
# fn main() {
let array = arr![u32; 1, 2, 3];
assert_eq!(array[2], 3);
# }
```rust

## Contents

- [Modules](#modules)
  - [`hex`](#hex)
  - [`impls`](#impls)
  - [`arr`](#arr)
  - [`functional`](#functional)
  - [`iter`](#iter)
  - [`sequence`](#sequence)
- [Structs](#structs)
  - [`GenericArrayIter`](#genericarrayiter)
  - [`GenericArray`](#genericarray)
- [Traits](#traits)
  - [`ArrayLength`](#arraylength)
- [Functions](#functions)
  - [`from_iter_length_fail`](#from-iter-length-fail)
- [Macros](#macros)
  - [`arr!`](#arr)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`hex`](#hex) | mod | Generic array are commonly used as a return value for hash digests, so it's a good idea to allow to hexlify them easily. |
| [`impls`](#impls) | mod |  |
| [`arr`](#arr) | mod | Implementation for `arr!` macro. |
| [`functional`](#functional) | mod | Functional programming with generic sequences |
| [`iter`](#iter) | mod | `GenericArray` iterator implementation. |
| [`sequence`](#sequence) | mod | Useful traits for manipulating sequences of data stored in `GenericArray`s |
| [`GenericArrayIter`](#genericarrayiter) | struct |  |
| [`GenericArray`](#genericarray) | struct | Struct representing a generic array - `GenericArray<T, N>` works like [T; N] |
| [`ArrayLength`](#arraylength) | trait | Trait making `GenericArray` work, marking types to be used as length of an array |
| [`from_iter_length_fail`](#from-iter-length-fail) | fn |  |
| [`arr!`](#arr) | macro | Macro allowing for easy generation of Generic Arrays. |

## Modules

- [`hex`](hex/index.md) — Generic array are commonly used as a return value for hash digests, so
- [`impls`](impls/index.md)
- [`arr`](arr/index.md) — Implementation for `arr!` macro.
- [`functional`](functional/index.md) — Functional programming with generic sequences
- [`iter`](iter/index.md) — `GenericArray` iterator implementation.
- [`sequence`](sequence/index.md) — Useful traits for manipulating sequences of data stored in `GenericArray`s

## Structs

### `GenericArrayIter<T, N: ArrayLength<T>>`

```rust
struct GenericArrayIter<T, N: ArrayLength<T>> {
    array: core::mem::ManuallyDrop<super::GenericArray<T, N>>,
    index: usize,
    index_back: usize,
}
```

An iterator that moves out of a `GenericArray`

#### Implementations

- <span id="genericarrayiter-as-slice"></span>`fn as_slice(&self) -> &[T]`

  Returns the remaining items of this iterator as a slice

- <span id="genericarrayiter-as-mut-slice"></span>`fn as_mut_slice(&mut self) -> &mut [T]`

  Returns the remaining items of this iterator as a mutable slice

#### Trait Implementations

##### `impl<T: Clone, N> Clone for GenericArrayIter<T, N>`

- <span id="genericarrayiter-clone"></span>`fn clone(&self) -> Self`

##### `impl<T: fmt::Debug, N> Debug for GenericArrayIter<T, N>`

- <span id="genericarrayiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, N> DoubleEndedIterator for GenericArrayIter<T, N>`

- <span id="genericarrayiter-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<T>`

- <span id="genericarrayiter-doubleendediterator-rfold"></span>`fn rfold<B, F>(self, init: B, f: F) -> B`

##### `impl<T, N> Drop for GenericArrayIter<T, N>`

- <span id="genericarrayiter-drop"></span>`fn drop(&mut self)`

##### `impl<T, N> ExactSizeIterator for GenericArrayIter<T, N>`

- <span id="genericarrayiter-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<T, N> FusedIterator for GenericArrayIter<T, N>`

##### `impl IntoIterator for GenericArrayIter<T, N>`

- <span id="genericarrayiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="genericarrayiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="genericarrayiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T, N> Iterator for GenericArrayIter<T, N>`

- <span id="genericarrayiter-iterator-type-item"></span>`type Item = T`

- <span id="genericarrayiter-iterator-next"></span>`fn next(&mut self) -> Option<T>`

- <span id="genericarrayiter-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

- <span id="genericarrayiter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="genericarrayiter-iterator-count"></span>`fn count(self) -> usize`

- <span id="genericarrayiter-iterator-nth"></span>`fn nth(&mut self, n: usize) -> Option<T>`

- <span id="genericarrayiter-iterator-last"></span>`fn last(self) -> Option<T>`

##### `impl<T> Same for GenericArrayIter<T, N>`

- <span id="genericarrayiter-same-type-output"></span>`type Output = T`

### `GenericArray<T, U: ArrayLength<T>>`

```rust
struct GenericArray<T, U: ArrayLength<T>> {
    data: <U as >::ArrayType,
}
```

Struct representing a generic array - `GenericArray<T, N>` works like [T; N]

#### Implementations

- <span id="genericarray-as-slice"></span>`fn as_slice(&self) -> &[T]`

  Extracts a slice containing the entire array.

- <span id="genericarray-as-mut-slice"></span>`fn as_mut_slice(&mut self) -> &mut [T]`

  Extracts a mutable slice containing the entire array.

- <span id="genericarray-from-slice"></span>`fn from_slice(slice: &[T]) -> &GenericArray<T, N>` — [`GenericArray`](#genericarray)

  Converts slice to a generic array reference with inferred length;

  

  # Panics

  

  Panics if the slice is not equal to the length of the array.

- <span id="genericarray-from-mut-slice"></span>`fn from_mut_slice(slice: &mut [T]) -> &mut GenericArray<T, N>` — [`GenericArray`](#genericarray)

  Converts mutable slice to a mutable generic array reference

  

  # Panics

  

  Panics if the slice is not equal to the length of the array.

#### Trait Implementations

##### `impl<T, N> AsMut for super::GenericArray<T, N>`

- <span id="supergenericarray-asmut-as-mut"></span>`fn as_mut(&mut self) -> &mut [T]`

##### `impl<T, N> AsRef for super::GenericArray<T, N>`

- <span id="supergenericarray-asref-as-ref"></span>`fn as_ref(&self) -> &[T]`

##### `impl<T: Clone, N> Clone for super::GenericArray<T, N>`

- <span id="supergenericarray-clone"></span>`fn clone(&self) -> GenericArray<T, N>` — [`GenericArray`](#genericarray)

##### `impl<T, N, M> Concat for GenericArray<T, N>`

- <span id="genericarray-concat-type-rest"></span>`type Rest = GenericArray<T, M>`

- <span id="genericarray-concat-type-output"></span>`type Output = GenericArray<T, <N as Add>::Output>`

- <span id="genericarray-concat"></span>`fn concat(self, rest: <Self as >::Rest) -> <Self as >::Output` — [`Concat`](sequence/index.md#concat)

##### `impl<T: Copy, N> Copy for super::GenericArray<T, N>`

##### `impl<T: Debug, N> Debug for super::GenericArray<T, N>`

- <span id="supergenericarray-debug-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Default, N> Default for super::GenericArray<T, N>`

- <span id="supergenericarray-default"></span>`fn default() -> Self`

##### `impl<T, N> Deref for GenericArray<T, N>`

- <span id="genericarray-deref-type-target"></span>`type Target = [T]`

- <span id="genericarray-deref"></span>`fn deref(&self) -> &[T]`

##### `impl<T, N> DerefMut for GenericArray<T, N>`

- <span id="genericarray-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut [T]`

##### `impl<T: Eq, N> Eq for super::GenericArray<T, N>`

##### `impl<T, N> FromIterator for GenericArray<T, N>`

- <span id="genericarray-fromiterator-from-iter"></span>`fn from_iter<I>(iter: I) -> GenericArray<T, N>` — [`GenericArray`](#genericarray)

##### `impl<T, N> FunctionalSequence for GenericArray<T, N>`

- <span id="genericarray-functionalsequence-map"></span>`fn map<U, F>(self, f: F) -> MappedSequence<Self, T, U>` — [`MappedSequence`](functional/index.md#mappedsequence)

- <span id="genericarray-functionalsequence-zip"></span>`fn zip<B, Rhs, U, F>(self, rhs: Rhs, f: F) -> MappedSequence<Self, T, U>` — [`MappedSequence`](functional/index.md#mappedsequence)

- <span id="genericarray-functionalsequence-fold"></span>`fn fold<U, F>(self, init: U, f: F) -> U`

##### `impl<T, N> GenericSequence for GenericArray<T, N>`

- <span id="genericarray-genericsequence-type-length"></span>`type Length = N`

- <span id="genericarray-genericsequence-type-sequence"></span>`type Sequence = GenericArray<T, N>`

- <span id="genericarray-genericsequence-generate"></span>`fn generate<F>(f: F) -> GenericArray<T, N>` — [`GenericArray`](#genericarray)

##### `impl<T: Hash, N> Hash for super::GenericArray<T, N>`

- <span id="supergenericarray-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<T, N> IntoIterator for super::GenericArray<T, N>`

- <span id="supergenericarray-intoiterator-type-item"></span>`type Item = T`

- <span id="supergenericarray-intoiterator-type-intoiter"></span>`type IntoIter = GenericArrayIter<T, N>`

- <span id="supergenericarray-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl<T, N> Lengthen for GenericArray<T, N>`

- <span id="genericarray-lengthen-type-longer"></span>`type Longer = GenericArray<T, <N as Add>::Output>`

- <span id="genericarray-lengthen-append"></span>`fn append(self, last: T) -> <Self as >::Longer` — [`Lengthen`](sequence/index.md#lengthen)

- <span id="genericarray-lengthen-prepend"></span>`fn prepend(self, first: T) -> <Self as >::Longer` — [`Lengthen`](sequence/index.md#lengthen)

##### `impl<T> LowerHex for crate::GenericArray<u8, T>`

- <span id="crategenericarray-lowerhex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, U, N> MappedGenericSequence for GenericArray<T, N>`

- <span id="genericarray-mappedgenericsequence-type-mapped"></span>`type Mapped = GenericArray<U, N>`

##### `impl<T: Ord, N> Ord for super::GenericArray<T, N>`

- <span id="supergenericarray-ord-cmp"></span>`fn cmp(&self, other: &GenericArray<T, N>) -> Ordering` — [`GenericArray`](#genericarray)

##### `impl<T: PartialEq, N> PartialEq for super::GenericArray<T, N>`

- <span id="supergenericarray-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T: PartialOrd, N> PartialOrd for super::GenericArray<T, N>`

- <span id="supergenericarray-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &GenericArray<T, N>) -> Option<Ordering>` — [`GenericArray`](#genericarray)

##### `impl<T> Receiver for GenericArray<T, U>`

- <span id="genericarray-receiver-type-target"></span>`type Target = T`

##### `impl<T> Same for GenericArray<T, U>`

- <span id="genericarray-same-type-output"></span>`type Output = T`

##### `impl<T: Send, N: ArrayLength<T>> Send for GenericArray<T, N>`

##### `impl<T, N> Shorten for GenericArray<T, N>`

- <span id="genericarray-shorten-type-shorter"></span>`type Shorter = GenericArray<T, <N as Sub>::Output>`

- <span id="genericarray-shorten-pop-back"></span>`fn pop_back(self) -> (<Self as >::Shorter, T)` — [`Shorten`](sequence/index.md#shorten)

- <span id="genericarray-shorten-pop-front"></span>`fn pop_front(self) -> (T, <Self as >::Shorter)` — [`Shorten`](sequence/index.md#shorten)

##### `impl<T, N, K> Split for GenericArray<T, N>`

- <span id="genericarray-split-type-first"></span>`type First = GenericArray<T, K>`

- <span id="genericarray-split-type-second"></span>`type Second = GenericArray<T, <N as Sub>::Output>`

- <span id="genericarray-split"></span>`fn split(self) -> (<Self as >::First, <Self as >::Second)` — [`Split`](sequence/index.md#split)

##### `impl<T: Sync, N: ArrayLength<T>> Sync for GenericArray<T, N>`

##### `impl<T> UpperHex for crate::GenericArray<u8, T>`

- <span id="crategenericarray-upperhex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Traits

### `ArrayLength<T>`

```rust
trait ArrayLength<T>: Unsigned { ... }
```

Trait making `GenericArray` work, marking types to be used as length of an array

#### Associated Types

- `type ArrayType`

#### Implementors

- `typenum::uint::UInt<N, typenum::bit::B0>`
- `typenum::uint::UInt<N, typenum::bit::B1>`
- `typenum::uint::UTerm`

## Functions

### `from_iter_length_fail`

```rust
fn from_iter_length_fail(length: usize, expected: usize) -> never
```

## Macros

### `arr!`

Macro allowing for easy generation of Generic Arrays.
Example: `let test = arr![u32; 1, 2, 3];`

