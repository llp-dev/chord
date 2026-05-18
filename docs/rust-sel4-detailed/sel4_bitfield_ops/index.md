# Crate `sel4_bitfield_ops`

## Contents

- [Modules](#modules)
  - [`sealing`](#sealing)
- [Structs](#structs)
  - [`Bitfield`](#bitfield)
- [Traits](#traits)
  - [`UnsignedPrimInt`](#unsignedprimint)
  - [`PrimInt`](#primint)
  - [`UnsignedPrimIntExt`](#unsignedprimintext)
- [Functions](#functions)
  - [`get_bit`](#get-bit)
  - [`get_bits`](#get-bits)
  - [`set_bits`](#set-bits)
  - [`check_range`](#check-range)
  - [`checked_cast`](#checked-cast)
  - [`set_bits_from_slice`](#set-bits-from-slice)
  - [`set_bits_from_slice_via`](#set-bits-from-slice-via)
  - [`get`](#get)
  - [`set`](#set)
- [Macros](#macros)
  - [`impl_prim_int!`](#impl-prim-int)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`sealing`](#sealing) | mod |  |
| [`Bitfield`](#bitfield) | struct |  |
| [`UnsignedPrimInt`](#unsignedprimint) | trait |  |
| [`PrimInt`](#primint) | trait |  |
| [`UnsignedPrimIntExt`](#unsignedprimintext) | trait |  |
| [`get_bit`](#get-bit) | fn |  |
| [`get_bits`](#get-bits) | fn |  |
| [`set_bits`](#set-bits) | fn |  |
| [`check_range`](#check-range) | fn |  |
| [`checked_cast`](#checked-cast) | fn |  |
| [`set_bits_from_slice`](#set-bits-from-slice) | fn |  |
| [`set_bits_from_slice_via`](#set-bits-from-slice-via) | fn |  |
| [`get`](#get) | fn |  |
| [`set`](#set) | fn |  |
| [`impl_prim_int!`](#impl-prim-int) | macro |  |

## Modules

- [`sealing`](sealing/index.md)

## Structs

### `Bitfield<T, U>`

```rust
struct Bitfield<T, U> {
    inner: T,
    _phantom: core::marker::PhantomData<U>,
}
```

#### Implementations

- <span id="bitfield-new"></span>`fn new(inner: T) -> Self`

- <span id="bitfield-into-inner"></span>`fn into_inner(self) -> T`

- <span id="bitfield-inner"></span>`fn inner(&self) -> &T`

- <span id="bitfield-inner-mut"></span>`fn inner_mut(&mut self) -> &mut T`

#### Trait Implementations

##### `impl<T: clone::Clone, U: clone::Clone> Clone for Bitfield<T, U>`

- <span id="bitfield-clone"></span>`fn clone(&self) -> Bitfield<T, U>` — [`Bitfield`](#bitfield)

##### `impl<T: marker::Copy, U: marker::Copy> Copy for Bitfield<T, U>`

##### `impl<T: fmt::Debug, U: fmt::Debug> Debug for Bitfield<T, U>`

- <span id="bitfield-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq, U: cmp::Eq> Eq for Bitfield<T, U>`

##### `impl<T: hash::Hash, U: hash::Hash> Hash for Bitfield<T, U>`

- <span id="bitfield-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T: cmp::Ord, U: cmp::Ord> Ord for Bitfield<T, U>`

- <span id="bitfield-ord-cmp"></span>`fn cmp(&self, other: &Bitfield<T, U>) -> cmp::Ordering` — [`Bitfield`](#bitfield)

##### `impl<T: cmp::PartialEq, U: cmp::PartialEq> PartialEq for Bitfield<T, U>`

- <span id="bitfield-partialeq-eq"></span>`fn eq(&self, other: &Bitfield<T, U>) -> bool` — [`Bitfield`](#bitfield)

##### `impl<T: cmp::PartialOrd, U: cmp::PartialOrd> PartialOrd for Bitfield<T, U>`

- <span id="bitfield-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Bitfield<T, U>) -> option::Option<cmp::Ordering>` — [`Bitfield`](#bitfield)

##### `impl<T, U> StructuralPartialEq for Bitfield<T, U>`

## Traits

### `UnsignedPrimInt`

```rust
trait UnsignedPrimInt: UnsignedPrimIntSealed + Copy + Eq + Not<Output = Self> + BitAnd<Output = Self> + BitOr<Output = Self> + BitAndAssign + BitOrAssign + Shl<usize, Output = Self> + Shr<usize, Output = Self> + From<bool> { ... }
```

#### Associated Constants

- `const NUM_BITS: usize`

#### Provided Methods

- `fn zero() -> Self`

- `fn one() -> Self`

#### Implementors

- `u128`
- `u16`
- `u32`
- `u64`
- `u8`
- `usize`

### `PrimInt`

```rust
trait PrimInt: PrimIntSealed { ... }
```

#### Associated Types

- `type Unsigned: 1`

#### Required Methods

- `fn cast_from_unsigned(val: <Self as >::Unsigned) -> Self`

- `fn cast_to_unsigned(val: Self) -> <Self as >::Unsigned`

#### Implementors

- `T`
- `i128`
- `i16`
- `i32`
- `i64`
- `i8`
- `isize`

### `UnsignedPrimIntExt`

```rust
trait UnsignedPrimIntExt: UnsignedPrimInt { ... }
```

#### Provided Methods

- `fn mask(range: Range<usize>) -> Self`

- `fn take(self, num_bits: usize) -> Self`

#### Implementors

- `T`

## Functions

### `get_bit`

```rust
fn get_bit<T: UnsignedPrimInt>(src: &[T], i: usize) -> bool
```

### `get_bits`

```rust
fn get_bits<T: UnsignedPrimInt, U: UnsignedPrimInt + TryFrom<T>>(src: &[T], src_range: core::ops::Range<usize>) -> U
```

### `set_bits`

```rust
fn set_bits<T: UnsignedPrimInt, U: UnsignedPrimInt + TryInto<T>>(dst: &mut [T], dst_range: core::ops::Range<usize>, src: U)
```

### `check_range`

```rust
fn check_range<T: UnsignedPrimInt, U: UnsignedPrimInt>(arr: &[T], range: &core::ops::Range<usize>)
```

### `checked_cast`

```rust
fn checked_cast<T: TryInto<U>, U>(val: T) -> U
```

### `set_bits_from_slice`

```rust
fn set_bits_from_slice<T, U>(dst: &mut [T], dst_range: core::ops::Range<usize>, src: &[U], src_start: usize)
where
    T: UnsignedPrimInt + TryFrom<usize>,
    U: UnsignedPrimInt,
    usize: TryFrom<U>
```

### `set_bits_from_slice_via`

```rust
fn set_bits_from_slice_via<T, U, V>(dst: &mut [T], dst_range: core::ops::Range<usize>, src: &[U], src_start: usize)
where
    T: UnsignedPrimInt + TryFrom<V>,
    U: UnsignedPrimInt,
    V: UnsignedPrimInt + TryFrom<U>
```

### `get`

```rust
fn get<T: UnsignedPrimInt, U: PrimInt>(src: &[T], src_start_bit: usize) -> U
where
    <U as >::Unsigned: TryFrom<T>
```

### `set`

```rust
fn set<T: UnsignedPrimInt, U: PrimInt>(dst: &mut [T], dst_start_bit: usize, src: U)
where
    <U as >::Unsigned: TryInto<T>
```

## Macros

### `impl_prim_int!`

