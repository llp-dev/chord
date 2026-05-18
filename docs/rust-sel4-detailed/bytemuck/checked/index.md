*[bytemuck](../index.md) / [checked](index.md)*

---

# Module `checked`

Checked versions of the casting functions exposed in crate root
that support [`CheckedBitPattern`](#checkedbitpattern) types.

## Contents

- [Enums](#enums)
  - [`CheckedCastError`](#checkedcasterror)
- [Traits](#traits)
  - [`CheckedBitPattern`](#checkedbitpattern)
- [Functions](#functions)
  - [`try_from_bytes`](#try-from-bytes)
  - [`try_from_bytes_mut`](#try-from-bytes-mut)
  - [`try_pod_read_unaligned`](#try-pod-read-unaligned)
  - [`try_cast`](#try-cast)
  - [`try_cast_ref`](#try-cast-ref)
  - [`try_cast_mut`](#try-cast-mut)
  - [`try_cast_slice`](#try-cast-slice)
  - [`try_cast_slice_mut`](#try-cast-slice-mut)
  - [`from_bytes`](#from-bytes)
  - [`from_bytes_mut`](#from-bytes-mut)
  - [`pod_read_unaligned`](#pod-read-unaligned)
  - [`cast`](#cast)
  - [`cast_mut`](#cast-mut)
  - [`cast_ref`](#cast-ref)
  - [`cast_slice`](#cast-slice)
  - [`cast_slice_mut`](#cast-slice-mut)
- [Macros](#macros)
  - [`impl_checked_for_nonzero!`](#impl-checked-for-nonzero)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`CheckedCastError`](#checkedcasterror) | enum | The things that can go wrong when casting between [`CheckedBitPattern`] data forms. |
| [`CheckedBitPattern`](#checkedbitpattern) | trait | A marker trait that allows types that have some invalid bit patterns to be used in places that otherwise require [`AnyBitPattern`] or [`Pod`] types by performing a runtime check on a perticular set of bits. |
| [`try_from_bytes`](#try-from-bytes) | fn | Re-interprets `&[u8]` as `&T`. |
| [`try_from_bytes_mut`](#try-from-bytes-mut) | fn | Re-interprets `&mut [u8]` as `&mut T`. |
| [`try_pod_read_unaligned`](#try-pod-read-unaligned) | fn | Reads from the bytes as if they were a `T`. |
| [`try_cast`](#try-cast) | fn | Try to cast `A` into `B`. |
| [`try_cast_ref`](#try-cast-ref) | fn | Try to convert a `&A` into `&B`. |
| [`try_cast_mut`](#try-cast-mut) | fn | Try to convert a `&mut A` into `&mut B`. |
| [`try_cast_slice`](#try-cast-slice) | fn | Try to convert `&[A]` into `&[B]` (possibly with a change in length). |
| [`try_cast_slice_mut`](#try-cast-slice-mut) | fn | Try to convert `&mut [A]` into `&mut [B]` (possibly with a change in length). |
| [`from_bytes`](#from-bytes) | fn | Re-interprets `&[u8]` as `&T`. |
| [`from_bytes_mut`](#from-bytes-mut) | fn | Re-interprets `&mut [u8]` as `&mut T`. |
| [`pod_read_unaligned`](#pod-read-unaligned) | fn | Reads the slice into a `T` value. |
| [`cast`](#cast) | fn | Cast `A` into `B` |
| [`cast_mut`](#cast-mut) | fn | Cast `&mut A` into `&mut B`. |
| [`cast_ref`](#cast-ref) | fn | Cast `&A` into `&B`. |
| [`cast_slice`](#cast-slice) | fn | Cast `&[A]` into `&[B]`. |
| [`cast_slice_mut`](#cast-slice-mut) | fn | Cast `&mut [A]` into `&mut [B]`. |
| [`impl_checked_for_nonzero!`](#impl-checked-for-nonzero) | macro |  |

## Enums

### `CheckedCastError`

```rust
enum CheckedCastError {
    PodCastError(crate::PodCastError),
    InvalidBitPattern,
}
```

The things that can go wrong when casting between [`CheckedBitPattern`](#checkedbitpattern) data
forms.

#### Variants

- **`PodCastError`**

  An error occurred during a true-[`Pod`](../index.md) cast
  

- **`InvalidBitPattern`**

  When casting to a [`CheckedBitPattern`](#checkedbitpattern) type, it is possible that the
  original data contains an invalid bit pattern. If so, the cast will
  fail and this error will be returned. Will never happen on casts
  between [`Pod`](../index.md) types.
  

#### Trait Implementations

##### `impl Clone for CheckedCastError`

- <span id="checkedcasterror-clone"></span>`fn clone(&self) -> CheckedCastError` — [`CheckedCastError`](#checkedcasterror)

##### `impl Copy for CheckedCastError`

##### `impl Debug for CheckedCastError`

- <span id="checkedcasterror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for CheckedCastError`

- <span id="checkedcasterror-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for CheckedCastError`

##### `impl Hash for CheckedCastError`

- <span id="checkedcasterror-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for CheckedCastError`

- <span id="checkedcasterror-partialeq-eq"></span>`fn eq(&self, other: &CheckedCastError) -> bool` — [`CheckedCastError`](#checkedcasterror)

##### `impl StructuralPartialEq for CheckedCastError`

## Traits

### `CheckedBitPattern`

```rust
trait CheckedBitPattern: Copy { ... }
```

A marker trait that allows types that have some invalid bit patterns to be
used in places that otherwise require [`AnyBitPattern`](../index.md) or [`Pod`](../index.md) types by
performing a runtime check on a perticular set of bits. This is particularly
useful for types like fieldless ('C-style') enums, [`char`](../../chrono/format/scan/index.md), bool, and
structs containing them.

To do this, we define a `Bits` type which is a type with equivalent layout
to `Self` other than the invalid bit patterns which disallow `Self` from
being [`AnyBitPattern`](../index.md). This `Bits` type must itself implement
[`AnyBitPattern`](../index.md). Then, we implement a function that checks whether a
certain instance of the `Bits` is also a valid bit pattern of `Self`. If
this check passes, then we can allow casting from the `Bits` to `Self` (and
therefore, any type which is able to be cast to `Bits` is also able to be
cast to `Self`).

[`AnyBitPattern`](../index.md) is a subset of [`CheckedBitPattern`](#checkedbitpattern), meaning that any `T:
AnyBitPattern` is also [`CheckedBitPattern`](#checkedbitpattern). This means you can also use
any [`AnyBitPattern`](../index.md) type in the checked versions of casting functions in
this module. If it's possible, prefer implementing [`AnyBitPattern`](../index.md) for
your type directly instead of [`CheckedBitPattern`](#checkedbitpattern) as it gives greater
flexibility.

# Derive

A `#[derive(CheckedBitPattern)]` macro is provided under the `derive`
feature flag which will automatically validate the requirements of this
trait and implement the trait for you for both enums and structs. This is
the recommended method for implementing the trait, however it's also
possible to do manually.

# Example

If manually implementing the trait, we can do something like so:

```rust
use bytemuck::{CheckedBitPattern, NoUninit};

#[repr(u32)]
#[derive(Copy, Clone)]
enum MyEnum {
    Variant0 = 0,
    Variant1 = 1,
    Variant2 = 2,
}

unsafe impl CheckedBitPattern for MyEnum {
    type Bits = u32;

    fn is_valid_bit_pattern(bits: &u32) -> bool {
        match *bits {
            0 | 1 | 2 => true,
            _ => false,
        }
    }
}

// It is often useful to also implement `NoUninit` on our `CheckedBitPattern` types.
// This will allow us to do casting of mutable references (and mutable slices).
// It is not always possible to do so, but in this case we have no padding so it is.
unsafe impl NoUninit for MyEnum {}
```

We can now use relevant casting functions. For example,

```rust
use bytemuck::{CheckedBitPattern, NoUninit};
#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum MyEnum {
    Variant0 = 0,
    Variant1 = 1,
    Variant2 = 2,
}
unsafe impl NoUninit for MyEnum {}
unsafe impl CheckedBitPattern for MyEnum {
    type Bits = u32;
    fn is_valid_bit_pattern(bits: &u32) -> bool {
        match *bits {
            0 | 1 | 2 => true,
            _ => false,
        }
    }
}
use bytemuck::{bytes_of, bytes_of_mut};
use bytemuck::checked;

let bytes = bytes_of(&2u32);
let result = checked::try_from_bytes::<MyEnum>(bytes);
assert_eq!(result, Ok(&MyEnum::Variant2));

// Fails for invalid discriminant
let bytes = bytes_of(&100u32);
let result = checked::try_from_bytes::<MyEnum>(bytes);
assert!(result.is_err());

// Since we implemented NoUninit, we can also cast mutably from an original type
// that is `NoUninit + AnyBitPattern`:
let mut my_u32 = 2u32;
{
  let as_enum_mut = checked::cast_mut::<_, MyEnum>(&mut my_u32);
  assert_eq!(as_enum_mut, &mut MyEnum::Variant2);
  *as_enum_mut = MyEnum::Variant0;
}
assert_eq!(my_u32, 0u32);
```

# Safety

* `Self` *must* have the same layout as the specified `Bits` except for the
  possible invalid bit patterns being checked during
  `is_valid_bit_pattern`.
* This almost certainly means your type must be `#[repr(C)]` or a similar
  specified repr, but if you think you know better, you probably don't. If
  you still think you know better, be careful and have fun. And don't mess
  it up (I mean it).
* If `is_valid_bit_pattern` returns true, then the bit pattern contained
  in `bits` must also be valid for an instance of `Self`.
* Probably more, don't mess it up (I mean it 2.0)



#### Associated Types

- `type Bits: 1`

#### Required Methods

- `fn is_valid_bit_pattern(bits: &<Self as >::Bits) -> bool`

  If this function returns true, then it must be valid to reinterpret `bits`

#### Implementors

- `T`
- `bool`
- `char`
- `core::num::NonZeroI128`
- `core::num::NonZeroI16`
- `core::num::NonZeroI32`
- `core::num::NonZeroI64`
- `core::num::NonZeroI8`
- `core::num::NonZeroIsize`
- `core::num::NonZeroU128`
- `core::num::NonZeroU16`
- `core::num::NonZeroU32`
- `core::num::NonZeroU64`
- `core::num::NonZeroU8`
- `core::num::NonZeroUsize`

## Functions

### `try_from_bytes`

```rust
fn try_from_bytes<T: CheckedBitPattern>(s: &[u8]) -> Result<&T, CheckedCastError>
```

Re-interprets `&[u8]` as `&T`.

## Failure

* If the slice isn't aligned for the new type
* If the slice's length isn’t exactly the size of the new type
* If the slice contains an invalid bit pattern for `T`

### `try_from_bytes_mut`

```rust
fn try_from_bytes_mut<T: CheckedBitPattern + NoUninit>(s: &mut [u8]) -> Result<&mut T, CheckedCastError>
```

Re-interprets `&mut [u8]` as `&mut T`.

## Failure

* If the slice isn't aligned for the new type
* If the slice's length isn’t exactly the size of the new type
* If the slice contains an invalid bit pattern for `T`

### `try_pod_read_unaligned`

```rust
fn try_pod_read_unaligned<T: CheckedBitPattern>(bytes: &[u8]) -> Result<T, CheckedCastError>
```

Reads from the bytes as if they were a `T`.

## Failure
* If the `bytes` length is not equal to `size_of::<T>()`.
* If the slice contains an invalid bit pattern for `T`

### `try_cast`

```rust
fn try_cast<A: NoUninit, B: CheckedBitPattern>(a: A) -> Result<B, CheckedCastError>
```

Try to cast `A` into `B`.

Note that for this particular type of cast, alignment isn't a factor. The
input value is semantically copied into the function and then returned to a
new memory location which will have whatever the required alignment of the
output type is.

## Failure

* If the types don't have the same size this fails.
* If `a` contains an invalid bit pattern for `B` this fails.

### `try_cast_ref`

```rust
fn try_cast_ref<A: NoUninit, B: CheckedBitPattern>(a: &A) -> Result<&B, CheckedCastError>
```

Try to convert a `&A` into `&B`.

## Failure

* If the reference isn't aligned in the new type
* If the source type and target type aren't the same size.
* If `a` contains an invalid bit pattern for `B` this fails.

### `try_cast_mut`

```rust
fn try_cast_mut<A: NoUninit + AnyBitPattern, B: CheckedBitPattern + NoUninit>(a: &mut A) -> Result<&mut B, CheckedCastError>
```

Try to convert a `&mut A` into `&mut B`.

As [`try_cast_ref`](#try-cast-ref), but `mut`.

### `try_cast_slice`

```rust
fn try_cast_slice<A: NoUninit, B: CheckedBitPattern>(a: &[A]) -> Result<&[B], CheckedCastError>
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
* If any element of the converted slice would contain an invalid bit pattern
  for `B` this fails.

### `try_cast_slice_mut`

```rust
fn try_cast_slice_mut<A: NoUninit + AnyBitPattern, B: CheckedBitPattern + NoUninit>(a: &mut [A]) -> Result<&mut [B], CheckedCastError>
```

Try to convert `&mut [A]` into `&mut [B]` (possibly with a change in
length).

As [`try_cast_slice`](#try-cast-slice), but `&mut`.

### `from_bytes`

```rust
fn from_bytes<T: CheckedBitPattern>(s: &[u8]) -> &T
```

Re-interprets `&[u8]` as `&T`.

## Panics

This is [`try_from_bytes`](#try-from-bytes) but will panic on error.

### `from_bytes_mut`

```rust
fn from_bytes_mut<T: NoUninit + CheckedBitPattern>(s: &mut [u8]) -> &mut T
```

Re-interprets `&mut [u8]` as `&mut T`.

## Panics

This is [`try_from_bytes_mut`](#try-from-bytes-mut) but will panic on error.

### `pod_read_unaligned`

```rust
fn pod_read_unaligned<T: CheckedBitPattern>(bytes: &[u8]) -> T
```

Reads the slice into a `T` value.

## Panics
* This is like [`try_pod_read_unaligned`](#try-pod-read-unaligned) but will panic on failure.

### `cast`

```rust
fn cast<A: NoUninit, B: CheckedBitPattern>(a: A) -> B
```

Cast `A` into `B`

## Panics

* This is like [`try_cast`](#try-cast), but will panic on a size mismatch.

### `cast_mut`

```rust
fn cast_mut<A: NoUninit + AnyBitPattern, B: NoUninit + CheckedBitPattern>(a: &mut A) -> &mut B
```

Cast `&mut A` into `&mut B`.

## Panics

This is [`try_cast_mut`](#try-cast-mut) but will panic on error.

### `cast_ref`

```rust
fn cast_ref<A: NoUninit, B: CheckedBitPattern>(a: &A) -> &B
```

Cast `&A` into `&B`.

## Panics

This is [`try_cast_ref`](#try-cast-ref) but will panic on error.

### `cast_slice`

```rust
fn cast_slice<A: NoUninit, B: CheckedBitPattern>(a: &[A]) -> &[B]
```

Cast `&[A]` into `&[B]`.

## Panics

This is [`try_cast_slice`](#try-cast-slice) but will panic on error.

### `cast_slice_mut`

```rust
fn cast_slice_mut<A: NoUninit + AnyBitPattern, B: NoUninit + CheckedBitPattern>(a: &mut [A]) -> &mut [B]
```

Cast `&mut [A]` into `&mut [B]`.

## Panics

This is [`try_cast_slice_mut`](#try-cast-slice-mut) but will panic on error.

## Macros

### `impl_checked_for_nonzero!`

