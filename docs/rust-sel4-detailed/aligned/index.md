# Crate `aligned`

A newtype with alignment of at least `A` bytes

# Examples

```rust
use std::mem;

use aligned::{Aligned, A2, A4, A16};

// Array aligned to a 2 byte boundary
static X: Aligned<A2, [u8; 3]> = Aligned([0; 3]);

// Array aligned to a 4 byte boundary
static Y: Aligned<A4, [u8; 3]> = Aligned([0; 3]);

// Unaligned array
static Z: [u8; 3] = [0; 3];

// You can allocate the aligned arrays on the stack too
let w: Aligned<A16, _> = Aligned([0u8; 3]);

assert_eq!(mem::align_of_val(&X), 2);
assert_eq!(mem::align_of_val(&Y), 4);
assert_eq!(mem::align_of_val(&Z), 1);
assert_eq!(mem::align_of_val(&w), 16);
```

## Contents

- [Modules](#modules)
  - [`sealed`](#sealed)
- [Structs](#structs)
  - [`A1`](#a1)
  - [`A2`](#a2)
  - [`A4`](#a4)
  - [`A8`](#a8)
  - [`A16`](#a16)
  - [`A32`](#a32)
  - [`A64`](#a64)
  - [`Aligned`](#aligned)
- [Traits](#traits)
  - [`Alignment`](#alignment)
- [Functions](#functions)
  - [`Aligned`](#aligned)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`sealed`](#sealed) | mod |  |
| [`A1`](#a1) | struct | 1-byte alignment |
| [`A2`](#a2) | struct | 2-byte alignment |
| [`A4`](#a4) | struct | 4-byte alignment |
| [`A8`](#a8) | struct | 8-byte alignment |
| [`A16`](#a16) | struct | 16-byte alignment |
| [`A32`](#a32) | struct | 32-byte alignment |
| [`A64`](#a64) | struct | 64-byte alignment |
| [`Aligned`](#aligned) | struct | A newtype with alignment of at least `A` bytes |
| [`Alignment`](#alignment) | trait | A marker trait for an alignment value. |
| [`Aligned`](#aligned) | fn | Changes the alignment of `value` to be at least `A` bytes |

## Modules

- [`sealed`](sealed/index.md)

## Structs

### `A1`

```rust
struct A1;
```

1-byte alignment

#### Trait Implementations

##### `impl Alignment for A1`

- <span id="a1-alignment-const-align"></span>`const ALIGN: usize`

##### `impl Clone for A1`

- <span id="a1-clone"></span>`fn clone(&self) -> A1` — [`A1`](#a1)

##### `impl Copy for A1`

##### `impl Sealed for super::A1`

### `A2`

```rust
struct A2;
```

2-byte alignment

#### Trait Implementations

##### `impl Alignment for A2`

- <span id="a2-alignment-const-align"></span>`const ALIGN: usize`

##### `impl Clone for A2`

- <span id="a2-clone"></span>`fn clone(&self) -> A2` — [`A2`](#a2)

##### `impl Copy for A2`

##### `impl Sealed for super::A2`

### `A4`

```rust
struct A4;
```

4-byte alignment

#### Trait Implementations

##### `impl Alignment for A4`

- <span id="a4-alignment-const-align"></span>`const ALIGN: usize`

##### `impl Clone for A4`

- <span id="a4-clone"></span>`fn clone(&self) -> A4` — [`A4`](#a4)

##### `impl Copy for A4`

##### `impl Sealed for super::A4`

### `A8`

```rust
struct A8;
```

8-byte alignment

#### Trait Implementations

##### `impl Alignment for A8`

- <span id="a8-alignment-const-align"></span>`const ALIGN: usize`

##### `impl Clone for A8`

- <span id="a8-clone"></span>`fn clone(&self) -> A8` — [`A8`](#a8)

##### `impl Copy for A8`

##### `impl Sealed for super::A8`

### `A16`

```rust
struct A16;
```

16-byte alignment

#### Trait Implementations

##### `impl Alignment for A16`

- <span id="a16-alignment-const-align"></span>`const ALIGN: usize`

##### `impl Clone for A16`

- <span id="a16-clone"></span>`fn clone(&self) -> A16` — [`A16`](#a16)

##### `impl Copy for A16`

##### `impl Sealed for super::A16`

### `A32`

```rust
struct A32;
```

32-byte alignment

#### Trait Implementations

##### `impl Alignment for A32`

- <span id="a32-alignment-const-align"></span>`const ALIGN: usize`

##### `impl Clone for A32`

- <span id="a32-clone"></span>`fn clone(&self) -> A32` — [`A32`](#a32)

##### `impl Copy for A32`

##### `impl Sealed for super::A32`

### `A64`

```rust
struct A64;
```

64-byte alignment

#### Trait Implementations

##### `impl Alignment for A64`

- <span id="a64-alignment-const-align"></span>`const ALIGN: usize`

##### `impl Clone for A64`

- <span id="a64-clone"></span>`fn clone(&self) -> A64` — [`A64`](#a64)

##### `impl Copy for A64`

##### `impl Sealed for super::A64`

### `Aligned<A, T>`

```rust
struct Aligned<A, T>
where
    T: ?Sized {
    _alignment: [A; 0],
    value: T,
}
```

A newtype with alignment of at least `A` bytes

#### Implementations

- <span id="aligned-is-index-aligned"></span>`fn is_index_aligned(index: usize) -> bool`

- <span id="aligned-check-start-index"></span>`fn check_start_index(index: usize)`

#### Trait Implementations

##### `impl<A, T> AsMutSlice for Aligned<A, T>`

- <span id="aligned-asmutslice-as-mut-slice"></span>`fn as_mut_slice(&mut self) -> &mut [<T as >::Element]`

##### `impl<A, T> AsSlice for Aligned<A, T>`

- <span id="aligned-asslice-type-element"></span>`type Element = <T as AsSlice>::Element`

- <span id="aligned-asslice-as-slice"></span>`fn as_slice(&self) -> &[<T as >::Element]`

##### `impl<A, T> Clone for Aligned<A, T>`

- <span id="aligned-clone"></span>`fn clone(&self) -> Self`

##### `impl<A, T> Copy for Aligned<A, T>`

##### `impl<A, T> Debug for Aligned<A, T>`

- <span id="aligned-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<A, T> Default for Aligned<A, T>`

- <span id="aligned-default"></span>`fn default() -> Self`

##### `impl<A, T> Deref for Aligned<A, T>`

- <span id="aligned-deref-type-target"></span>`type Target = T`

- <span id="aligned-deref"></span>`fn deref(&self) -> &T`

##### `impl<A, T> DerefMut for Aligned<A, T>`

- <span id="aligned-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut T`

##### `impl<A, T> Display for Aligned<A, T>`

- <span id="aligned-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<A, T> Eq for Aligned<A, T>`

##### `impl<A, T> Hash for Aligned<A, T>`

- <span id="aligned-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl<A, T> Index for Aligned<A, [T]>`

- <span id="aligned-index-type-output"></span>`type Output = Aligned<A, [T]>`

- <span id="aligned-index"></span>`fn index(&self, range: ops::RangeFrom<usize>) -> &Aligned<A, [T]>` — [`Aligned`](#aligned)

##### `impl<A, T> IndexMut for Aligned<A, [T]>`

- <span id="aligned-indexmut-index-mut"></span>`fn index_mut(&mut self, range: ops::RangeFrom<usize>) -> &mut Aligned<A, [T]>` — [`Aligned`](#aligned)

##### `impl<A, T> Ord for Aligned<A, T>`

- <span id="aligned-ord-cmp"></span>`fn cmp(&self, other: &Self) -> Ordering`

##### `impl<A, T> PartialEq for Aligned<A, T>`

- <span id="aligned-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<A, T> PartialOrd for Aligned<A, T>`

- <span id="aligned-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

##### `impl<T> Receiver for Aligned<A, T>`

- <span id="aligned-receiver-type-target"></span>`type Target = T`

##### `impl<T> ToString for Aligned<A, T>`

- <span id="aligned-tostring-to-string"></span>`fn to_string(&self) -> String`

## Traits

### `Alignment`

```rust
trait Alignment: Copy + sealed::Sealed { ... }
```

A marker trait for an alignment value.

#### Associated Constants

- `const ALIGN: usize`

#### Implementors

- [`A16`](#a16)
- [`A1`](#a1)
- [`A2`](#a2)
- [`A32`](#a32)
- [`A4`](#a4)
- [`A64`](#a64)
- [`A8`](#a8)

## Functions

### `Aligned`

```rust
const fn Aligned<A, T>(value: T) -> Aligned<A, T>
```

Changes the alignment of `value` to be at least `A` bytes

