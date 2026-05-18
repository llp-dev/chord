*[sel4_abstract_ptr](../index.md) / [core_ext](index.md)*

---

# Module `core_ext`

## Contents

- [Traits](#traits)
  - [`AbstractPtrSliceIndex`](#abstractptrsliceindex)
  - [`AbstractPtrSliceIndexInternal`](#abstractptrsliceindexinternal)
- [Functions](#functions)
  - [`non_null_slice_len`](#non-null-slice-len)
  - [`mut_ptr_slice_len`](#mut-ptr-slice-len)
  - [`non_null_slice_as_mut_ptr`](#non-null-slice-as-mut-ptr)
  - [`mut_ptr_slice_as_mut_ptr`](#mut-ptr-slice-as-mut-ptr)
  - [`non_null_index`](#non-null-index)
  - [`range`](#range)
  - [`bounds_check`](#bounds-check)
- [Macros](#macros)
  - [`slice_index_impl!`](#slice-index-impl)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`AbstractPtrSliceIndex`](#abstractptrsliceindex) | trait |  |
| [`AbstractPtrSliceIndexInternal`](#abstractptrsliceindexinternal) | trait |  |
| [`non_null_slice_len`](#non-null-slice-len) | fn |  |
| [`mut_ptr_slice_len`](#mut-ptr-slice-len) | fn |  |
| [`non_null_slice_as_mut_ptr`](#non-null-slice-as-mut-ptr) | fn |  |
| [`mut_ptr_slice_as_mut_ptr`](#mut-ptr-slice-as-mut-ptr) | fn |  |
| [`non_null_index`](#non-null-index) | fn |  |
| [`range`](#range) | fn |  |
| [`bounds_check`](#bounds-check) | fn |  |
| [`slice_index_impl!`](#slice-index-impl) | macro |  |

## Traits

### `AbstractPtrSliceIndex<T: ?Sized>`

```rust
trait AbstractPtrSliceIndex<T: ?Sized>: SliceIndex<T> + AbstractPtrSliceIndexInternal<T> { ... }
```

#### Implementors

- `(core::ops::Bound<usize>, core::ops::Bound<usize>)`
- `core::ops::Range<usize>`
- `ops::RangeFrom<usize>`
- `ops::RangeFull`
- `ops::RangeInclusive<usize>`
- `ops::RangeTo<usize>`
- `ops::RangeToInclusive<usize>`
- `usize`

### `AbstractPtrSliceIndexInternal<T: ?Sized>`

```rust
trait AbstractPtrSliceIndexInternal<T: ?Sized>: SliceIndex<T> { ... }
```

#### Required Methods

- `fn abstract_ptr_slice_index(self, slice: *mut T) -> *mut <Self as >::Output`

#### Implementors

- `(core::ops::Bound<usize>, core::ops::Bound<usize>)`
- `core::ops::Range<usize>`
- `ops::RangeFrom<usize>`
- `ops::RangeFull`
- `ops::RangeInclusive<usize>`
- `ops::RangeTo<usize>`
- `ops::RangeToInclusive<usize>`
- `usize`

## Functions

### `non_null_slice_len`

```rust
fn non_null_slice_len<T>(p: core::ptr::NonNull<[T]>) -> usize
```

### `mut_ptr_slice_len`

```rust
fn mut_ptr_slice_len<T>(p: *mut [T]) -> usize
```

### `non_null_slice_as_mut_ptr`

```rust
fn non_null_slice_as_mut_ptr<T>(p: core::ptr::NonNull<[T]>) -> *mut T
```

### `mut_ptr_slice_as_mut_ptr`

```rust
fn mut_ptr_slice_as_mut_ptr<T>(p: *mut [T]) -> *mut T
```

### `non_null_index`

```rust
fn non_null_index<I, T>(p: core::ptr::NonNull<[T]>, index: I) -> core::ptr::NonNull<<I as SliceIndex>::Output>
where
    I: AbstractPtrSliceIndex<[T]> + SliceIndex<[()]> + Clone
```

### `range`

```rust
fn range<R>(range: R, bounds: core::ops::RangeTo<usize>) -> core::ops::Range<usize>
where
    R: RangeBounds<usize>
```

### `bounds_check`

```rust
fn bounds_check(len: usize, index: impl SliceIndex<[()]>)
```

## Macros

### `slice_index_impl!`

