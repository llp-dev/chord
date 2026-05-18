*[bytemuck](../index.md) / [internal](index.md)*

---

# Module `internal`

Internal implementation of casting functions not bound by marker traits
and therefore marked as unsafe. This is used so that we don't need to
duplicate the business logic contained in these functions between the
versions exported in the crate root, `checked`, and `relaxed` modules.

## Contents

- [Functions](#functions)
  - [`something_went_wrong`](#something-went-wrong)
  - [`bytes_of`](#bytes-of)
  - [`bytes_of_mut`](#bytes-of-mut)
  - [`from_bytes`](#from-bytes)
  - [`from_bytes_mut`](#from-bytes-mut)
  - [`try_pod_read_unaligned`](#try-pod-read-unaligned)
  - [`pod_read_unaligned`](#pod-read-unaligned)
  - [`is_aligned_to`](#is-aligned-to)
  - [`try_from_bytes`](#try-from-bytes)
  - [`try_from_bytes_mut`](#try-from-bytes-mut)
  - [`cast`](#cast)
  - [`cast_mut`](#cast-mut)
  - [`cast_ref`](#cast-ref)
  - [`cast_slice`](#cast-slice)
  - [`cast_slice_mut`](#cast-slice-mut)
  - [`try_cast`](#try-cast)
  - [`try_cast_ref`](#try-cast-ref)
  - [`try_cast_mut`](#try-cast-mut)
  - [`try_cast_slice`](#try-cast-slice)
  - [`try_cast_slice_mut`](#try-cast-slice-mut)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`something_went_wrong`](#something-went-wrong) | fn | Immediately panics. |
| [`bytes_of`](#bytes-of) | fn | Re-interprets `&T` as `&[u8]`. |
| [`bytes_of_mut`](#bytes-of-mut) | fn | Re-interprets `&mut T` as `&mut [u8]`. |
| [`from_bytes`](#from-bytes) | fn | Re-interprets `&[u8]` as `&T`. |
| [`from_bytes_mut`](#from-bytes-mut) | fn | Re-interprets `&mut [u8]` as `&mut T`. |
| [`try_pod_read_unaligned`](#try-pod-read-unaligned) | fn | Reads from the bytes as if they were a `T`. |
| [`pod_read_unaligned`](#pod-read-unaligned) | fn | Reads the slice into a `T` value. |
| [`is_aligned_to`](#is-aligned-to) | fn | Checks if `ptr` is aligned to an `align` memory boundary. |
| [`try_from_bytes`](#try-from-bytes) | fn | Re-interprets `&[u8]` as `&T`. |
| [`try_from_bytes_mut`](#try-from-bytes-mut) | fn | Re-interprets `&mut [u8]` as `&mut T`. |
| [`cast`](#cast) | fn | Cast `A` into `B` |
| [`cast_mut`](#cast-mut) | fn | Cast `&mut A` into `&mut B`. |
| [`cast_ref`](#cast-ref) | fn | Cast `&A` into `&B`. |
| [`cast_slice`](#cast-slice) | fn | Cast `&[A]` into `&[B]`. |
| [`cast_slice_mut`](#cast-slice-mut) | fn | Cast `&mut [A]` into `&mut [B]`. |
| [`try_cast`](#try-cast) | fn | Try to cast `A` into `B`. |
| [`try_cast_ref`](#try-cast-ref) | fn | Try to convert a `&A` into `&B`. |
| [`try_cast_mut`](#try-cast-mut) | fn | Try to convert a `&mut A` into `&mut B`. |
| [`try_cast_slice`](#try-cast-slice) | fn | Try to convert `&[A]` into `&[B]` (possibly with a change in length). |
| [`try_cast_slice_mut`](#try-cast-slice-mut) | fn | Try to convert `&mut [A]` into `&mut [B]` (possibly with a change in length). |

## Functions

### `something_went_wrong`

```rust
fn something_went_wrong<D: core::fmt::Display>(_src: &str, _err: D) -> never
```

Immediately panics.

### `bytes_of`

```rust
unsafe fn bytes_of<T: Copy>(t: &T) -> &[u8]
```

Re-interprets `&T` as `&[u8]`.

Any ZST becomes an empty slice, and in that case the pointer value of that
empty slice might not match the pointer value of the input reference.

### `bytes_of_mut`

```rust
unsafe fn bytes_of_mut<T: Copy>(t: &mut T) -> &mut [u8]
```

Re-interprets `&mut T` as `&mut [u8]`.

Any ZST becomes an empty slice, and in that case the pointer value of that
empty slice might not match the pointer value of the input reference.

### `from_bytes`

```rust
unsafe fn from_bytes<T: Copy>(s: &[u8]) -> &T
```

Re-interprets `&[u8]` as `&T`.

## Panics

This is [`try_from_bytes`](#try-from-bytes) but will panic on error.

### `from_bytes_mut`

```rust
unsafe fn from_bytes_mut<T: Copy>(s: &mut [u8]) -> &mut T
```

Re-interprets `&mut [u8]` as `&mut T`.

## Panics

This is [`try_from_bytes_mut`](#try-from-bytes-mut) but will panic on error.

### `try_pod_read_unaligned`

```rust
unsafe fn try_pod_read_unaligned<T: Copy>(bytes: &[u8]) -> Result<T, crate::PodCastError>
```

Reads from the bytes as if they were a `T`.

## Failure
* If the `bytes` length is not equal to `size_of::<T>()`.

### `pod_read_unaligned`

```rust
unsafe fn pod_read_unaligned<T: Copy>(bytes: &[u8]) -> T
```

Reads the slice into a `T` value.

## Panics
* This is like `try_pod_read_unaligned` but will panic on failure.

### `is_aligned_to`

```rust
fn is_aligned_to(ptr: *const (), align: usize) -> bool
```

Checks if `ptr` is aligned to an `align` memory boundary.

## Panics
* If `align` is not a power of two. This includes when `align` is zero.

### `try_from_bytes`

```rust
unsafe fn try_from_bytes<T: Copy>(s: &[u8]) -> Result<&T, crate::PodCastError>
```

Re-interprets `&[u8]` as `&T`.

## Failure

* If the slice isn't aligned for the new type
* If the slice's length isn’t exactly the size of the new type

### `try_from_bytes_mut`

```rust
unsafe fn try_from_bytes_mut<T: Copy>(s: &mut [u8]) -> Result<&mut T, crate::PodCastError>
```

Re-interprets `&mut [u8]` as `&mut T`.

## Failure

* If the slice isn't aligned for the new type
* If the slice's length isn’t exactly the size of the new type

### `cast`

```rust
unsafe fn cast<A: Copy, B: Copy>(a: A) -> B
```

Cast `A` into `B`

## Panics

* This is like [`try_cast`](try_cast), but will panic on a size mismatch.

### `cast_mut`

```rust
unsafe fn cast_mut<A: Copy, B: Copy>(a: &mut A) -> &mut B
```

Cast `&mut A` into `&mut B`.

## Panics

This is [`try_cast_mut`](#try-cast-mut) but will panic on error.

### `cast_ref`

```rust
unsafe fn cast_ref<A: Copy, B: Copy>(a: &A) -> &B
```

Cast `&A` into `&B`.

## Panics

This is [`try_cast_ref`](#try-cast-ref) but will panic on error.

### `cast_slice`

```rust
unsafe fn cast_slice<A: Copy, B: Copy>(a: &[A]) -> &[B]
```

Cast `&[A]` into `&[B]`.

## Panics

This is [`try_cast_slice`](#try-cast-slice) but will panic on error.

### `cast_slice_mut`

```rust
unsafe fn cast_slice_mut<A: Copy, B: Copy>(a: &mut [A]) -> &mut [B]
```

Cast `&mut [A]` into `&mut [B]`.

## Panics

This is [`try_cast_slice_mut`](#try-cast-slice-mut) but will panic on error.

### `try_cast`

```rust
unsafe fn try_cast<A: Copy, B: Copy>(a: A) -> Result<B, crate::PodCastError>
```

Try to cast `A` into `B`.

Note that for this particular type of cast, alignment isn't a factor. The
input value is semantically copied into the function and then returned to a
new memory location which will have whatever the required alignment of the
output type is.

## Failure

* If the types don't have the same size this fails.

### `try_cast_ref`

```rust
unsafe fn try_cast_ref<A: Copy, B: Copy>(a: &A) -> Result<&B, crate::PodCastError>
```

Try to convert a `&A` into `&B`.

## Failure

* If the reference isn't aligned in the new type
* If the source type and target type aren't the same size.

### `try_cast_mut`

```rust
unsafe fn try_cast_mut<A: Copy, B: Copy>(a: &mut A) -> Result<&mut B, crate::PodCastError>
```

Try to convert a `&mut A` into `&mut B`.

As [`try_cast_ref`](#try-cast-ref), but `mut`.

### `try_cast_slice`

```rust
unsafe fn try_cast_slice<A: Copy, B: Copy>(a: &[A]) -> Result<&[B], crate::PodCastError>
```

Try to convert `&[A]` into `&[B]` (possibly with a change in length).

* `input.as_ptr() as usize == output.as_ptr() as usize`
* `input.len() * size_of::<A>() == output.len() * size_of::<B>()`

## Failure

* If the target type has a greater alignment requirement and the input slice
  isn't aligned.
* If the target element type is a different size from the current element
  type, and the output slice wouldn't be a whole number of elements when
  accounting for the size change (eg: 3 `u16` values is 1.5 `u32` values, so
  that's a failure).

### `try_cast_slice_mut`

```rust
unsafe fn try_cast_slice_mut<A: Copy, B: Copy>(a: &mut [A]) -> Result<&mut [B], crate::PodCastError>
```

Try to convert `&mut [A]` into `&mut [B]` (possibly with a change in
length).

As [`try_cast_slice`](#try-cast-slice), but `&mut`.

