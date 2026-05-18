**bytemuck > checked**

# Module: checked

## Contents

**Enums**

- [`CheckedCastError`](#checkedcasterror) - The things that can go wrong when casting between [`CheckedBitPattern`] data

**Functions**

- [`cast`](#cast) - Cast `A` into `B`
- [`cast_mut`](#cast_mut) - Cast `&mut A` into `&mut B`.
- [`cast_ref`](#cast_ref) - Cast `&A` into `&B`.
- [`cast_slice`](#cast_slice) - Cast `&[A]` into `&[B]`.
- [`cast_slice_mut`](#cast_slice_mut) - Cast `&mut [A]` into `&mut [B]`.
- [`from_bytes`](#from_bytes) - Re-interprets `&[u8]` as `&T`.
- [`from_bytes_mut`](#from_bytes_mut) - Re-interprets `&mut [u8]` as `&mut T`.
- [`pod_read_unaligned`](#pod_read_unaligned) - Reads the slice into a `T` value.
- [`try_cast`](#try_cast) - Try to cast `A` into `B`.
- [`try_cast_mut`](#try_cast_mut) - Try to convert a `&mut A` into `&mut B`.
- [`try_cast_ref`](#try_cast_ref) - Try to convert a `&A` into `&B`.
- [`try_cast_slice`](#try_cast_slice) - Try to convert `&[A]` into `&[B]` (possibly with a change in length).
- [`try_cast_slice_mut`](#try_cast_slice_mut) - Try to convert `&mut [A]` into `&mut [B]` (possibly with a change in
- [`try_from_bytes`](#try_from_bytes) - Re-interprets `&[u8]` as `&T`.
- [`try_from_bytes_mut`](#try_from_bytes_mut) - Re-interprets `&mut [u8]` as `&mut T`.
- [`try_pod_read_unaligned`](#try_pod_read_unaligned) - Reads from the bytes as if they were a `T`.

**Traits**

- [`CheckedBitPattern`](#checkedbitpattern) - A marker trait that allows types that have some invalid bit patterns to be

---

## bytemuck::checked::CheckedBitPattern

*Trait*

A marker trait that allows types that have some invalid bit patterns to be
used in places that otherwise require [`AnyBitPattern`] or [`Pod`] types by
performing a runtime check on a perticular set of bits. This is particularly
useful for types like fieldless ('C-style') enums, [`char`], bool, and
structs containing them.

To do this, we define a `Bits` type which is a type with equivalent layout
to `Self` other than the invalid bit patterns which disallow `Self` from
being [`AnyBitPattern`]. This `Bits` type must itself implement
[`AnyBitPattern`]. Then, we implement a function that checks whether a
certain instance of the `Bits` is also a valid bit pattern of `Self`. If
this check passes, then we can allow casting from the `Bits` to `Self` (and
therefore, any type which is able to be cast to `Bits` is also able to be
cast to `Self`).

[`AnyBitPattern`] is a subset of [`CheckedBitPattern`], meaning that any `T:
AnyBitPattern` is also [`CheckedBitPattern`]. This means you can also use
any [`AnyBitPattern`] type in the checked versions of casting functions in
this module. If it's possible, prefer implementing [`AnyBitPattern`] for
your type directly instead of [`CheckedBitPattern`] as it gives greater
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
# use bytemuck::{CheckedBitPattern, NoUninit};
# #[repr(u32)]
# #[derive(Copy, Clone, PartialEq, Eq, Debug)]
# enum MyEnum {
#     Variant0 = 0,
#     Variant1 = 1,
#     Variant2 = 2,
# }
# unsafe impl NoUninit for MyEnum {}
# unsafe impl CheckedBitPattern for MyEnum {
#     type Bits = u32;
#     fn is_valid_bit_pattern(bits: &u32) -> bool {
#         match *bits {
#             0 | 1 | 2 => true,
#             _ => false,
#         }
#     }
# }
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
  [`is_valid_bit_pattern`].
* This almost certainly means your type must be `#[repr(C)]` or a similar
  specified repr, but if you think you know better, you probably don't. If
  you still think you know better, be careful and have fun. And don't mess
  it up (I mean it).
* If [`is_valid_bit_pattern`] returns true, then the bit pattern contained
  in `bits` must also be valid for an instance of `Self`.
* Probably more, don't mess it up (I mean it 2.0)

[`is_valid_bit_pattern`]: CheckedBitPattern::is_valid_bit_pattern
[`Pod`]: crate::Pod

**Methods:**

- `Bits`: `Self` *must* have the same layout as the specified `Bits` except for
- `is_valid_bit_pattern`: If this function returns true, then it must be valid to reinterpret `bits`



## bytemuck::checked::CheckedCastError

*Enum*

The things that can go wrong when casting between [`CheckedBitPattern`] data
forms.

**Variants:**
- `PodCastError(crate::PodCastError)` - An error occurred during a true-[`Pod`] cast
- `InvalidBitPattern` - When casting to a [`CheckedBitPattern`] type, it is possible that the

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> CheckedCastError`
- **From**
  - `fn from(err: crate::PodCastError) -> CheckedCastError`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &CheckedCastError) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



## bytemuck::checked::cast

*Function*

Cast `A` into `B`

## Panics

* This is like [`try_cast`], but will panic on a size mismatch.

```rust
fn cast<A, B>(a: A) -> B
```



## bytemuck::checked::cast_mut

*Function*

Cast `&mut A` into `&mut B`.

## Panics

This is [`try_cast_mut`] but will panic on error.

```rust
fn cast_mut<A, B>(a: & mut A) -> & mut B
```



## bytemuck::checked::cast_ref

*Function*

Cast `&A` into `&B`.

## Panics

This is [`try_cast_ref`] but will panic on error.

```rust
fn cast_ref<A, B>(a: &A) -> &B
```



## bytemuck::checked::cast_slice

*Function*

Cast `&[A]` into `&[B]`.

## Panics

This is [`try_cast_slice`] but will panic on error.

```rust
fn cast_slice<A, B>(a: &[A]) -> &[B]
```



## bytemuck::checked::cast_slice_mut

*Function*

Cast `&mut [A]` into `&mut [B]`.

## Panics

This is [`try_cast_slice_mut`] but will panic on error.

```rust
fn cast_slice_mut<A, B>(a: & mut [A]) -> & mut [B]
```



## bytemuck::checked::from_bytes

*Function*

Re-interprets `&[u8]` as `&T`.

## Panics

This is [`try_from_bytes`] but will panic on error.

```rust
fn from_bytes<T>(s: &[u8]) -> &T
```



## bytemuck::checked::from_bytes_mut

*Function*

Re-interprets `&mut [u8]` as `&mut T`.

## Panics

This is [`try_from_bytes_mut`] but will panic on error.

```rust
fn from_bytes_mut<T>(s: & mut [u8]) -> & mut T
```



## bytemuck::checked::pod_read_unaligned

*Function*

Reads the slice into a `T` value.

## Panics
* This is like [`try_pod_read_unaligned`] but will panic on failure.

```rust
fn pod_read_unaligned<T>(bytes: &[u8]) -> T
```



## bytemuck::checked::try_cast

*Function*

Try to cast `A` into `B`.

Note that for this particular type of cast, alignment isn't a factor. The
input value is semantically copied into the function and then returned to a
new memory location which will have whatever the required alignment of the
output type is.

## Failure

* If the types don't have the same size this fails.
* If `a` contains an invalid bit pattern for `B` this fails.

```rust
fn try_cast<A, B>(a: A) -> Result<B, CheckedCastError>
```



## bytemuck::checked::try_cast_mut

*Function*

Try to convert a `&mut A` into `&mut B`.

As [`try_cast_ref`], but `mut`.

```rust
fn try_cast_mut<A, B>(a: & mut A) -> Result<& mut B, CheckedCastError>
```



## bytemuck::checked::try_cast_ref

*Function*

Try to convert a `&A` into `&B`.

## Failure

* If the reference isn't aligned in the new type
* If the source type and target type aren't the same size.
* If `a` contains an invalid bit pattern for `B` this fails.

```rust
fn try_cast_ref<A, B>(a: &A) -> Result<&B, CheckedCastError>
```



## bytemuck::checked::try_cast_slice

*Function*

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

```rust
fn try_cast_slice<A, B>(a: &[A]) -> Result<&[B], CheckedCastError>
```



## bytemuck::checked::try_cast_slice_mut

*Function*

Try to convert `&mut [A]` into `&mut [B]` (possibly with a change in
length).

As [`try_cast_slice`], but `&mut`.

```rust
fn try_cast_slice_mut<A, B>(a: & mut [A]) -> Result<& mut [B], CheckedCastError>
```



## bytemuck::checked::try_from_bytes

*Function*

Re-interprets `&[u8]` as `&T`.

## Failure

* If the slice isn't aligned for the new type
* If the slice's length isn’t exactly the size of the new type
* If the slice contains an invalid bit pattern for `T`

```rust
fn try_from_bytes<T>(s: &[u8]) -> Result<&T, CheckedCastError>
```



## bytemuck::checked::try_from_bytes_mut

*Function*

Re-interprets `&mut [u8]` as `&mut T`.

## Failure

* If the slice isn't aligned for the new type
* If the slice's length isn’t exactly the size of the new type
* If the slice contains an invalid bit pattern for `T`

```rust
fn try_from_bytes_mut<T>(s: & mut [u8]) -> Result<& mut T, CheckedCastError>
```



## bytemuck::checked::try_pod_read_unaligned

*Function*

Reads from the bytes as if they were a `T`.

## Failure
* If the `bytes` length is not equal to `size_of::<T>()`.
* If the slice contains an invalid bit pattern for `T`

```rust
fn try_pod_read_unaligned<T>(bytes: &[u8]) -> Result<T, CheckedCastError>
```



