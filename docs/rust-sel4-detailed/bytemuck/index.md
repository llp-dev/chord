# Crate `bytemuck`

This crate gives small utilities for casting between plain data types.

## Basics

Data comes in five basic forms in Rust, so we have five basic casting
functions:

* `T` uses [`cast`](#cast)
* `&T` uses [`cast_ref`](#cast-ref)
* `&mut T` uses [`cast_mut`](#cast-mut)
* `&[T]` uses [`cast_slice`](#cast-slice)
* `&mut [T]` uses [`cast_slice_mut`](#cast-slice-mut)

Depending on the function, the [`NoUninit`](#nouninit) and/or [`AnyBitPattern`](#anybitpattern) traits
are used to maintain memory safety.

**Historical Note:** When the crate first started the [`Pod`](#pod) trait was used
instead, and so you may hear people refer to that, but it has the strongest
requirements and people eventually wanted the more fine-grained system, so
here we are. All types that impl `Pod` have a blanket impl to also support
`NoUninit` and `AnyBitPattern`. The traits unfortunately do not have a
perfectly clean hierarchy for semver reasons.

## Failures

Some casts will never fail, and other casts might fail.

* `cast::<u32, f32>` always works (and `f32::from_bits`).
* `cast_ref::<[u8; 4], u32>` might fail if the specific array reference
  given at runtime doesn't have alignment 4.

In addition to the "normal" forms of each function, which will panic on
invalid input, there's also `try_` versions which will return a `Result`.

If you would like to statically ensure that a cast will work at runtime you
can use the `must_cast` crate feature and the `must_` casting functions. A
"must cast" that can't be statically known to be valid will cause a
compilation error (and sometimes a very hard to read compilation error).

## Using Your Own Types

All the functions listed above are guarded by the [`Pod`](#pod) trait, which is a
sub-trait of the [`Zeroable`](#zeroable) trait.

If you enable the crate's `derive` feature then these traits can be derived
on your own types. The derive macros will perform the necessary checks on
your type declaration, and trigger an error if your type does not qualify.

The derive macros might not cover all edge cases, and sometimes they will
error when actually everything is fine. As a last resort you can impl these
traits manually. However, these traits are `unsafe`, and you should
carefully read the requirements before using a manual implementation.

## Cargo Features

The crate supports Rust 1.34 when no features are enabled, and so there's
cargo features for thing that you might consider "obvious".

The cargo features **do not** promise any particular MSRV, and they may
increase their MSRV in new versions.

* `derive`: Provide derive macros for the various traits.
* `extern_crate_alloc`: Provide utilities for `alloc` related types such as
  Box and Vec.
* `zeroable_maybe_uninit` and `zeroable_atomics`: Provide more [`Zeroable`](#zeroable)
  impls.
* `pod_saturating`: Provide more [`Pod`](#pod) and [`Zeroable`](#zeroable) impls.
* `wasm_simd` and `aarch64_simd`: Support more SIMD types.
* `min_const_generics`: Provides appropriate impls for arrays of all lengths
  instead of just for a select list of array lengths.
* `must_cast`: Provides the `must_` functions, which will compile error if
  the requested cast can't be statically verified.
* `const_zeroed`: Provides a const version of the `zeroed` function.

## Related Crates

- [`pack1`](https://docs.rs/pack1), which contains `bytemuck`-compatible
  packed little-endian, big-endian and native-endian integer and floating
  point number types.

## Contents

- [Modules](#modules)
  - [`anybitpattern`](#anybitpattern)
  - [`checked`](#checked)
  - [`internal`](#internal)
  - [`zeroable`](#zeroable)
  - [`zeroable_in_option`](#zeroable-in-option)
  - [`pod`](#pod)
  - [`pod_in_option`](#pod-in-option)
  - [`no_uninit`](#no-uninit)
  - [`contiguous`](#contiguous)
  - [`offset_of`](#offset-of)
  - [`transparent`](#transparent)
- [Enums](#enums)
  - [`PodCastError`](#podcasterror)
- [Traits](#traits)
  - [`CheckedBitPattern`](#checkedbitpattern)
  - [`AnyBitPattern`](#anybitpattern)
  - [`Zeroable`](#zeroable)
  - [`ZeroableInOption`](#zeroableinoption)
  - [`Pod`](#pod)
  - [`PodInOption`](#podinoption)
  - [`NoUninit`](#nouninit)
  - [`Contiguous`](#contiguous)
  - [`TransparentWrapper`](#transparentwrapper)
- [Functions](#functions)
  - [`bytes_of`](#bytes-of)
  - [`bytes_of_mut`](#bytes-of-mut)
  - [`from_bytes`](#from-bytes)
  - [`from_bytes_mut`](#from-bytes-mut)
  - [`try_pod_read_unaligned`](#try-pod-read-unaligned)
  - [`pod_read_unaligned`](#pod-read-unaligned)
  - [`try_from_bytes`](#try-from-bytes)
  - [`try_from_bytes_mut`](#try-from-bytes-mut)
  - [`cast`](#cast)
  - [`cast_mut`](#cast-mut)
  - [`cast_ref`](#cast-ref)
  - [`cast_slice`](#cast-slice)
  - [`cast_slice_mut`](#cast-slice-mut)
  - [`pod_align_to`](#pod-align-to)
  - [`pod_align_to_mut`](#pod-align-to-mut)
  - [`try_cast`](#try-cast)
  - [`try_cast_ref`](#try-cast-ref)
  - [`try_cast_mut`](#try-cast-mut)
  - [`try_cast_slice`](#try-cast-slice)
  - [`try_cast_slice_mut`](#try-cast-slice-mut)
  - [`write_zeroes`](#write-zeroes)
  - [`fill_zeroes`](#fill-zeroes)
- [Macros](#macros)
  - [`impl_unsafe_marker_for_array!`](#impl-unsafe-marker-for-array)
  - [`transmute!`](#transmute)
  - [`impl_unsafe_marker_for_simd!`](#impl-unsafe-marker-for-simd)
  - [`maybe_const_fn!`](#maybe-const-fn)
  - [`offset_of!`](#offset-of)
  - [`impl_for_fn!`](#impl-for-fn)
  - [`impl_contiguous!`](#impl-contiguous)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`anybitpattern`](#anybitpattern) | mod |  |
| [`checked`](#checked) | mod | Checked versions of the casting functions exposed in crate root that support [`CheckedBitPattern`] types. |
| [`internal`](#internal) | mod | Internal implementation of casting functions not bound by marker traits and therefore marked as unsafe. |
| [`zeroable`](#zeroable) | mod |  |
| [`zeroable_in_option`](#zeroable-in-option) | mod |  |
| [`pod`](#pod) | mod |  |
| [`pod_in_option`](#pod-in-option) | mod |  |
| [`no_uninit`](#no-uninit) | mod |  |
| [`contiguous`](#contiguous) | mod |  |
| [`offset_of`](#offset-of) | mod |  |
| [`transparent`](#transparent) | mod |  |
| [`PodCastError`](#podcasterror) | enum | The things that can go wrong when casting between [`Pod`] data forms. |
| [`CheckedBitPattern`](#checkedbitpattern) | trait |  |
| [`AnyBitPattern`](#anybitpattern) | trait | Marker trait for "plain old data" types that are valid for any bit pattern. |
| [`Zeroable`](#zeroable) | trait | Trait for types that can be safely created with [`zeroed`](core::mem::zeroed). |
| [`ZeroableInOption`](#zeroableinoption) | trait | Trait for types which are [Zeroable](Zeroable) when wrapped in [Option](core::option::Option). |
| [`Pod`](#pod) | trait | Marker trait for "plain old data". |
| [`PodInOption`](#podinoption) | trait | Trait for types which are [Pod](Pod) when wrapped in [Option](core::option::Option). |
| [`NoUninit`](#nouninit) | trait | Marker trait for "plain old data" types with no uninit (or padding) bytes. |
| [`Contiguous`](#contiguous) | trait | A trait indicating that |
| [`TransparentWrapper`](#transparentwrapper) | trait | A trait which indicates that a type is a `#[repr(transparent)]` wrapper around the `Inner` value. |
| [`bytes_of`](#bytes-of) | fn | Re-interprets `&T` as `&[u8]`. |
| [`bytes_of_mut`](#bytes-of-mut) | fn | Re-interprets `&mut T` as `&mut [u8]`. |
| [`from_bytes`](#from-bytes) | fn | Re-interprets `&[u8]` as `&T`. |
| [`from_bytes_mut`](#from-bytes-mut) | fn | Re-interprets `&mut [u8]` as `&mut T`. |
| [`try_pod_read_unaligned`](#try-pod-read-unaligned) | fn | Reads from the bytes as if they were a `T`. |
| [`pod_read_unaligned`](#pod-read-unaligned) | fn | Reads the slice into a `T` value. |
| [`try_from_bytes`](#try-from-bytes) | fn | Re-interprets `&[u8]` as `&T`. |
| [`try_from_bytes_mut`](#try-from-bytes-mut) | fn | Re-interprets `&mut [u8]` as `&mut T`. |
| [`cast`](#cast) | fn | Cast `A` into `B` |
| [`cast_mut`](#cast-mut) | fn | Cast `&mut A` into `&mut B`. |
| [`cast_ref`](#cast-ref) | fn | Cast `&A` into `&B`. |
| [`cast_slice`](#cast-slice) | fn | Cast `&[A]` into `&[B]`. |
| [`cast_slice_mut`](#cast-slice-mut) | fn | Cast `&mut [A]` into `&mut [B]`. |
| [`pod_align_to`](#pod-align-to) | fn | As [`align_to`](https://doc.rust-lang.org/std/primitive.slice.html#method.align_to), but safe because of the [`Pod`] bound. |
| [`pod_align_to_mut`](#pod-align-to-mut) | fn | As [`align_to_mut`](https://doc.rust-lang.org/std/primitive.slice.html#method.align_to_mut), but safe because of the [`Pod`] bound. |
| [`try_cast`](#try-cast) | fn | Try to cast `A` into `B`. |
| [`try_cast_ref`](#try-cast-ref) | fn | Try to convert a `&A` into `&B`. |
| [`try_cast_mut`](#try-cast-mut) | fn | Try to convert a `&mut A` into `&mut B`. |
| [`try_cast_slice`](#try-cast-slice) | fn | Try to convert `&[A]` into `&[B]` (possibly with a change in length). |
| [`try_cast_slice_mut`](#try-cast-slice-mut) | fn | Try to convert `&mut [A]` into `&mut [B]` (possibly with a change in length). |
| [`write_zeroes`](#write-zeroes) | fn | Fill all bytes of `target` with zeroes (see [`Zeroable`]). |
| [`fill_zeroes`](#fill-zeroes) | fn | Fill all bytes of `slice` with zeroes (see [`Zeroable`]). |
| [`impl_unsafe_marker_for_array!`](#impl-unsafe-marker-for-array) | macro |  |
| [`transmute!`](#transmute) | macro | A macro to transmute between two types without requiring knowing size statically. |
| [`impl_unsafe_marker_for_simd!`](#impl-unsafe-marker-for-simd) | macro | A macro to implement marker traits for various simd types. |
| [`maybe_const_fn!`](#maybe-const-fn) | macro | A macro for conditionally const-ifying a function. |
| [`offset_of!`](#offset-of) | macro | Find the offset in bytes of the given `$field` of `$Type`. |
| [`impl_for_fn!`](#impl-for-fn) | macro |  |
| [`impl_contiguous!`](#impl-contiguous) | macro |  |

## Modules

- [`anybitpattern`](anybitpattern/index.md)
- [`checked`](checked/index.md) — Checked versions of the casting functions exposed in crate root
- [`internal`](internal/index.md) — Internal implementation of casting functions not bound by marker traits
- [`zeroable`](zeroable/index.md)
- [`zeroable_in_option`](zeroable_in_option/index.md)
- [`pod`](pod/index.md)
- [`pod_in_option`](pod_in_option/index.md)
- [`no_uninit`](no_uninit/index.md)
- [`contiguous`](contiguous/index.md)
- [`offset_of`](offset_of/index.md)
- [`transparent`](transparent/index.md)

## Enums

### `PodCastError`

```rust
enum PodCastError {
    TargetAlignmentGreaterAndInputNotAligned,
    OutputSliceWouldHaveSlop,
    SizeMismatch,
    AlignmentMismatch,
}
```

The things that can go wrong when casting between [`Pod`](#pod) data forms.

#### Variants

- **`TargetAlignmentGreaterAndInputNotAligned`**

  You tried to cast a reference into a reference to a type with a higher
  alignment requirement but the input reference wasn't aligned.

- **`OutputSliceWouldHaveSlop`**

  If the element size of a slice changes, then the output slice changes
  length accordingly. If the output slice wouldn't be a whole number of
  elements, then the conversion fails.

- **`SizeMismatch`**

  When casting an individual `T`, `&T`, or `&mut T` value the
  source size and destination size must be an exact match.

- **`AlignmentMismatch`**

  For this type of cast the alignments must be exactly the same and they
  were not so now you're sad.
  
  This error is generated **only** by operations that cast allocated types
  (such as `Box` and `Vec`), because in that case the alignment must stay
  exact.

#### Trait Implementations

##### `impl Clone for PodCastError`

- <span id="podcasterror-clone"></span>`fn clone(&self) -> PodCastError` — [`PodCastError`](#podcasterror)

##### `impl Copy for PodCastError`

##### `impl Debug for PodCastError`

- <span id="podcasterror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for PodCastError`

- <span id="podcasterror-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for PodCastError`

##### `impl Hash for PodCastError`

- <span id="podcasterror-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for PodCastError`

- <span id="podcasterror-partialeq-eq"></span>`fn eq(&self, other: &PodCastError) -> bool` — [`PodCastError`](#podcasterror)

##### `impl StructuralPartialEq for PodCastError`

## Traits

### `CheckedBitPattern`

```rust
trait CheckedBitPattern: Copy { ... }
```

A marker trait that allows types that have some invalid bit patterns to be
used in places that otherwise require [`AnyBitPattern`](#anybitpattern) or [`Pod`](#pod) types by
performing a runtime check on a perticular set of bits. This is particularly
useful for types like fieldless ('C-style') enums, [`char`](../chrono/format/scan/index.md), bool, and
structs containing them.

To do this, we define a `Bits` type which is a type with equivalent layout
to `Self` other than the invalid bit patterns which disallow `Self` from
being [`AnyBitPattern`](#anybitpattern). This `Bits` type must itself implement
[`AnyBitPattern`](#anybitpattern). Then, we implement a function that checks whether a
certain instance of the `Bits` is also a valid bit pattern of `Self`. If
this check passes, then we can allow casting from the `Bits` to `Self` (and
therefore, any type which is able to be cast to `Bits` is also able to be
cast to `Self`).

[`AnyBitPattern`](#anybitpattern) is a subset of [`CheckedBitPattern`](checked/index.md), meaning that any `T:
AnyBitPattern` is also [`CheckedBitPattern`](checked/index.md). This means you can also use
any [`AnyBitPattern`](#anybitpattern) type in the checked versions of casting functions in
this module. If it's possible, prefer implementing [`AnyBitPattern`](#anybitpattern) for
your type directly instead of [`CheckedBitPattern`](checked/index.md) as it gives greater
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

### `AnyBitPattern`

```rust
trait AnyBitPattern: Zeroable + Sized + Copy + 'static { ... }
```

Marker trait for "plain old data" types that are valid for any bit pattern.

The requirements for this is very similar to [`Pod`](#pod), except that the type
can allow uninit (or padding) bytes. This limits what you can do with a type
of this kind, but also broadens the included types to `repr(C)` `struct`s
that contain padding as well as `union`s. Notably, you can only cast
*immutable* references and *owned* values into [`AnyBitPattern`](#anybitpattern) types, not
*mutable* references.

[`Pod`](#pod) is a subset of [`AnyBitPattern`](#anybitpattern), meaning that any `T: Pod` is also
[`AnyBitPattern`](#anybitpattern) but any `T: AnyBitPattern` is not necessarily [`Pod`](#pod).

[`AnyBitPattern`](#anybitpattern) is a subset of [`Zeroable`](#zeroable), meaning that any `T:
AnyBitPattern` is also [`Zeroable`](#zeroable), but any `T: Zeroable` is not
necessarily [`AnyBitPattern`](#anybitpattern)

# Derive

A `#[derive(AnyBitPattern)]` macro is provided under the `derive` feature
flag which will automatically validate the requirements of this trait and
implement the trait for you for both structs and enums. This is the
recommended method for implementing the trait, however it's also possible to
do manually. If you implement it manually, you *must* carefully follow the
below safety rules.

* *NOTE: even `C-style`, fieldless enums are intentionally **excluded** from
  this trait, since it is **unsound** for an enum to have a discriminant
  value that is not one of its defined variants.

# Safety

Similar to [`Pod`](#pod) except we disregard the rule about it must not contain
uninit bytes. Still, this is a quite strong guarantee about a type, so *be
careful* when implementing it manually.

* The type must be inhabited (eg: no
  [Infallible](core::convert::Infallible)).
* The type must be valid for any bit pattern of its backing memory.
* Structs need to have all fields also be `AnyBitPattern`.
* It is disallowed for types to contain pointer types, `Cell`, `UnsafeCell`,
  atomics, and any other forms of interior mutability.
* More precisely: A shared reference to the type must allow reads, and
  *only* reads. RustBelt's separation logic is based on the notion that a
  type is allowed to define a sharing predicate, its own invariant that must
  hold for shared references, and this predicate is the reasoning that allow
  it to deal with atomic and cells etc. We require the sharing predicate to
  be trivial and permit only read-only access.
* There's probably more, don't mess it up (I mean it).

#### Implementors

- `T`

### `Zeroable`

```rust
trait Zeroable: Sized { ... }
```

Trait for types that can be safely created with
[`zeroed`](core::mem::zeroed).

An all-zeroes value may or may not be the same value as the
[Default](core::default::Default) value of the type.

## Safety

* Your type must be inhabited (eg: no
  [Infallible](core::convert::Infallible)).
* Your type must be allowed to be an "all zeroes" bit pattern (eg: no
  [`NonNull<T>`](core::ptr::NonNull)).

## Features

Some `impl`s are feature gated due to the MSRV policy:

* `MaybeUninit<T>` was not available in 1.34.0, but is available under the
  `zeroable_maybe_uninit` feature flag.
* `Atomic*` types require Rust 1.60.0 or later to work on certain platforms,
  but is available under the `zeroable_atomics` feature flag.
* `[T; N]` for arbitrary `N` requires the `min_const_generics` feature flag.

#### Provided Methods

- `fn zeroed() -> Self`

  Calls [`zeroed`](core::mem::zeroed).

#### Implementors

- `()`
- `(A)`
- `(A, B)`
- `(A, B, C)`
- `(A, B, C, D)`
- `(A, B, C, D, E)`
- `(A, B, C, D, E, F)`
- `(A, B, C, D, E, F, G)`
- `(A, B, C, D, E, F, G, H)`
- `*const T`
- `*const [T]`
- `*const str`
- `*mut T`
- `*mut [T]`
- `*mut str`
- `Option<T>`
- `PhantomData<T>`
- `PhantomPinned`
- `Wrapping<T>`
- `[T; 0]`
- `[T; 1024]`
- `[T; 10]`
- `[T; 11]`
- `[T; 128]`
- `[T; 12]`
- `[T; 13]`
- `[T; 14]`
- `[T; 15]`
- `[T; 16]`
- `[T; 17]`
- `[T; 18]`
- `[T; 19]`
- `[T; 1]`
- `[T; 2048]`
- `[T; 20]`
- `[T; 21]`
- `[T; 22]`
- `[T; 23]`
- `[T; 24]`
- `[T; 256]`
- `[T; 25]`
- `[T; 26]`
- `[T; 27]`
- `[T; 28]`
- `[T; 29]`
- `[T; 2]`
- `[T; 30]`
- `[T; 31]`
- `[T; 32]`
- `[T; 3]`
- `[T; 4096]`
- `[T; 48]`
- `[T; 4]`
- `[T; 512]`
- `[T; 5]`
- `[T; 64]`
- `[T; 6]`
- `[T; 7]`
- `[T; 8]`
- `[T; 96]`
- `[T; 9]`
- `bool`
- `char`
- `core::cell::Cell<T>`
- `core::cell::UnsafeCell<T>`
- `core::cmp::Reverse<T>`
- `core::mem::ManuallyDrop<T>`
- `f32`
- `f64`
- `i128`
- `i16`
- `i32`
- `i64`
- `i8`
- `isize`
- `u128`
- `u16`
- `u32`
- `u64`
- `u8`
- `usize`
- `x86_64::__m128`
- `x86_64::__m128d`
- `x86_64::__m128i`
- `x86_64::__m256`
- `x86_64::__m256d`
- `x86_64::__m256i`

### `ZeroableInOption`

```rust
trait ZeroableInOption: Sized { ... }
```

Trait for types which are [Zeroable](Zeroable) when wrapped in
[Option](core::option::Option).

## Safety

* `Option<YourType>` must uphold the same invariants as
  [Zeroable](Zeroable).

#### Implementors

- `&T`
- `&mut T`
- `NonNull<T>`
- `NonZeroI128`
- `NonZeroI16`
- `NonZeroI32`
- `NonZeroI64`
- `NonZeroI8`
- `NonZeroIsize`
- `NonZeroU128`
- `NonZeroU16`
- `NonZeroU32`
- `NonZeroU64`
- `NonZeroU8`
- `NonZeroUsize`
- `fn() -> R`
- `fn(A) -> R`
- `fn(A, B) -> R`
- `fn(A, B, C) -> R`
- `fn(A, B, C, D) -> R`
- `fn(A, B, C, D, E) -> R`
- `fn(A, B, C, D, E, F) -> R`
- `fn(A, B, C, D, E, F, G) -> R`
- `fn(A, B, C, D, E, F, G, H) -> R`
- `fn(A, B, C, D, E, F, G, H, I) -> R`
- `fn(A, B, C, D, E, F, G, H, I, J) -> R`
- `fn(A, B, C, D, E, F, G, H, I, J, K) -> R`
- `fn(A, B, C, D, E, F, G, H, I, J, K, L) -> R`
- `fn(A, B, C, D, E, F, G, H, I, J, K, L, M) -> R`

### `Pod`

```rust
trait Pod: Zeroable + Copy + 'static { ... }
```

Marker trait for "plain old data".

The point of this trait is that once something is marked "plain old data"
you can really go to town with the bit fiddling and bit casting. Therefore,
it's a relatively strong claim to make about a type. Do not add this to your
type casually.

**Reminder:** The results of casting around bytes between data types are
_endian dependant_. Little-endian machines are the most common, but
big-endian machines do exist (and big-endian is also used for "network
order" bytes).

## Safety

* The type must be inhabited (eg: no
  [Infallible](core::convert::Infallible)).
* The type must allow any bit pattern (eg: no `bool` or `char`, which have
  illegal bit patterns).
* The type must not contain any uninit (or padding) bytes, either in the
  middle or on the end (eg: no `#[repr(C)] struct Foo(u8, u16)`, which has
  padding in the middle, and also no `#[repr(C)] struct Foo(u16, u8)`, which
  has padding on the end).
* The type needs to have all fields also be `Pod`.
* The type needs to be `repr(C)` or `repr(transparent)`. In the case of
  `repr(C)`, the `packed` and `align` repr modifiers can be used as long as
  all other rules end up being followed.
* It is disallowed for types to contain pointer types, `Cell`, `UnsafeCell`,
  atomics, and any other forms of interior mutability.
* More precisely: A shared reference to the type must allow reads, and
  *only* reads. RustBelt's separation logic is based on the notion that a
  type is allowed to define a sharing predicate, its own invariant that must
  hold for shared references, and this predicate is the reasoning that allow
  it to deal with atomic and cells etc. We require the sharing predicate to
  be trivial and permit only read-only access.

#### Implementors

- `()`
- `Option<T>`
- `PhantomData<T>`
- `PhantomPinned`
- `Wrapping<T>`
- `[T; 0]`
- `[T; 1024]`
- `[T; 10]`
- `[T; 11]`
- `[T; 128]`
- `[T; 12]`
- `[T; 13]`
- `[T; 14]`
- `[T; 15]`
- `[T; 16]`
- `[T; 17]`
- `[T; 18]`
- `[T; 19]`
- `[T; 1]`
- `[T; 2048]`
- `[T; 20]`
- `[T; 21]`
- `[T; 22]`
- `[T; 23]`
- `[T; 24]`
- `[T; 256]`
- `[T; 25]`
- `[T; 26]`
- `[T; 27]`
- `[T; 28]`
- `[T; 29]`
- `[T; 2]`
- `[T; 30]`
- `[T; 31]`
- `[T; 32]`
- `[T; 3]`
- `[T; 4096]`
- `[T; 48]`
- `[T; 4]`
- `[T; 512]`
- `[T; 5]`
- `[T; 64]`
- `[T; 6]`
- `[T; 7]`
- `[T; 8]`
- `[T; 96]`
- `[T; 9]`
- `core::mem::ManuallyDrop<T>`
- `f32`
- `f64`
- `i128`
- `i16`
- `i32`
- `i64`
- `i8`
- `isize`
- `u128`
- `u16`
- `u32`
- `u64`
- `u8`
- `usize`
- `x86_64::__m128`
- `x86_64::__m128d`
- `x86_64::__m128i`
- `x86_64::__m256`
- `x86_64::__m256d`
- `x86_64::__m256i`

### `PodInOption`

```rust
trait PodInOption: ZeroableInOption + Copy + 'static { ... }
```

Trait for types which are [Pod](Pod) when wrapped in
[Option](core::option::Option).

## Safety

* `Option<T>` must uphold the same invariants as [Pod](Pod).
* **Reminder:** pointers are **not** pod! **Do not** mix this trait with a
  newtype over [NonNull](core::ptr::NonNull).

#### Implementors

- `NonZeroI128`
- `NonZeroI16`
- `NonZeroI32`
- `NonZeroI64`
- `NonZeroI8`
- `NonZeroIsize`
- `NonZeroU128`
- `NonZeroU16`
- `NonZeroU32`
- `NonZeroU64`
- `NonZeroU8`
- `NonZeroUsize`

### `NoUninit`

```rust
trait NoUninit: Sized + Copy + 'static { ... }
```

Marker trait for "plain old data" types with no uninit (or padding) bytes.

The requirements for this is very similar to [`Pod`](#pod),
except that it doesn't require that all bit patterns of the type are valid,
i.e. it does not require the type to be `Zeroable`.
This limits what you can do with a type of this kind, but also broadens the
included types to things like C-style enums. Notably, you can only cast from
*immutable* references to a [`NoUninit`](#nouninit) type into *immutable* references of
any other type, no casting of mutable references or mutable references to
slices etc.

[`Pod`](#pod) is a subset of [`NoUninit`](#nouninit), meaning that any `T: Pod` is also
[`NoUninit`](#nouninit) but any `T: NoUninit` is not necessarily [`Pod`](#pod). If possible,
prefer implementing [`Pod`](#pod) directly. To get more [`Pod`](#pod)-like functionality
for a type that is only [`NoUninit`](#nouninit), consider also implementing
`CheckedBitPattern`.

The rules for padding for various types and representations are documented
in the Rust reference section on [type layout].

# Derive

A `#[derive(NoUninit)]` macro is provided under the `derive` feature flag
which will automatically validate the requirements of this trait and
implement the trait for you for both enums and structs. This is the
recommended method for implementing the trait, however it's also possible to
do manually. If you implement it manually, you *must* carefully follow the
below safety rules.

# Safety

The same as [`Pod`](#pod) except we disregard the rule about it must
allow any bit pattern (i.e. it does not need to be
`Zeroable`). Still, this is a quite strong guarantee
about a type, so *be careful* whem implementing it manually.

* The type must be inhabited (eg: no
  [Infallible](core::convert::Infallible)).
* The type must not contain any uninit (or padding) bytes, either in the
  middle or on the end (eg: no `#[repr(C)] struct Foo(u8, u16)`, which has
  padding in the middle, and also no `#[repr(C)] struct Foo(u16, u8)`, which
  has padding on the end).
* Structs need to have all fields also be `NoUninit`.
* Structs need to be `repr(C)` or `repr(transparent)`. In the case of
  `repr(C)`, the `packed` and `align` repr modifiers can be used as long as
  all other rules end up being followed.
* Enums need to be `#[repr(Int)]`, `#[repr(C)]`, or both.
* Enums may have fields. If the enum has fields,
    * Each variant's fields must individually follow the same rules as a struct
    * All variants must be the same size, and require no padding-to-alignment
    * There must be no padding needed between the discriminant type and the
      "fields struct" of any variant
* It is disallowed for types to contain pointer types, `Cell`, `UnsafeCell`,
  atomics, and any other forms of interior mutability.
* More precisely: A shared reference to the type must allow reads, and
  *only* reads. RustBelt's separation logic is based on the notion that a
  type is allowed to define a sharing predicate, its own invariant that must
  hold for shared references, and this predicate is the reasoning that allow
  it to deal with atomic and cells etc. We require the sharing predicate to
  be trivial and permit only read-only access.
* There's probably more, don't mess it up (I mean it).


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

### `Contiguous`

```rust
trait Contiguous: Copy + 'static { ... }
```

A trait indicating that:

1. A type has an equivalent representation to some known integral type.
2. All instances of this type fall in a fixed range of values.
3. Within that range, there are no gaps.

This is generally useful for fieldless enums (aka "c-style" enums), however
it's important that it only be used for those with an explicit `#[repr]`, as
`#[repr(Rust)]` fieldess enums have an unspecified layout.

Additionally, you shouldn't assume that all implementations are enums. Any
type which meets the requirements above while following the rules under
"Safety" below is valid.

# Example

```rust
use bytemuck::Contiguous;
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum Foo {
  A = 0,
  B = 1,
  C = 2,
  D = 3,
  E = 4,
}
unsafe impl Contiguous for Foo {
  type Int = u8;
  const MIN_VALUE: u8 = Foo::A as u8;
  const MAX_VALUE: u8 = Foo::E as u8;
}
assert_eq!(Foo::from_integer(3).unwrap(), Foo::D);
assert_eq!(Foo::from_integer(8), None);
assert_eq!(Foo::C.into_integer(), 2);
```
# Safety

This is an unsafe trait, and incorrectly implementing it is undefined
behavior.

Informally, by implementing it, you're asserting that `C` is identical to
the integral type `C::Int`, and that every `C` falls between `C::MIN_VALUE`
and `C::MAX_VALUE` exactly once, without any gaps.

Precisely, the guarantees you must uphold when implementing `Contiguous` for
some type `C` are:

1. The size of `C` and `C::Int` must be the same, and neither may be a ZST.
   (Note: alignment is explicitly allowed to differ)

2. `C::Int` must be a primitive integer, and not a wrapper type. In the
   future, this may be lifted to include cases where the behavior is
   identical for a relevant set of traits (Ord, arithmetic, ...).

3. All `C::Int`s which are in the *inclusive* range between `C::MIN_VALUE`
   and `C::MAX_VALUE` are bitwise identical to unique valid instances of
   `C`.

4. There exist no instances of `C` such that their bitpatterns, when
   interpreted as instances of `C::Int`, fall outside of the `MAX_VALUE` /
   `MIN_VALUE` range -- It is legal for unsafe code to assume that if it
   gets a `C` that implements `Contiguous`, it is in the appropriate range.

5. Finally, you promise not to provide overridden implementations of
   `Contiguous::from_integer` and `Contiguous::into_integer`.

For clarity, the following rules could be derived from the above, but are
listed explicitly:

- `C::MAX_VALUE` must be greater or equal to `C::MIN_VALUE` (therefore, `C`
  must be an inhabited type).

- There exist no two values between `MIN_VALUE` and `MAX_VALUE` such that
  when interpreted as a `C` they are considered identical (by, say, match).

#### Associated Types

- `type Int: 2`

#### Associated Constants

- `const MAX_VALUE: <Self as >::Int`

- `const MIN_VALUE: <Self as >::Int`

#### Provided Methods

- `fn from_integer(value: <Self as >::Int) -> Option<Self>`

  If `value` is within the range for valid instances of this type,

- `fn into_integer(self) -> <Self as >::Int`

  Perform the conversion from `C` into the underlying integral type. This

#### Implementors

- `NonZeroU128`
- `NonZeroU16`
- `NonZeroU32`
- `NonZeroU64`
- `NonZeroU8`
- `NonZeroUsize`
- `bool`
- `i128`
- `i16`
- `i32`
- `i64`
- `i8`
- `isize`
- `u128`
- `u16`
- `u32`
- `u64`
- `u8`
- `usize`

### `TransparentWrapper<Inner: ?Sized>`

```rust
trait TransparentWrapper<Inner: ?Sized> { ... }
```

A trait which indicates that a type is a `#[repr(transparent)]` wrapper
around the `Inner` value.

This allows safely copy transmuting between the `Inner` type and the
`TransparentWrapper` type. Functions like `wrap_{}` convert from the inner
type to the wrapper type and `peel_{}` functions do the inverse conversion
from the wrapper type to the inner type. We deliberately do not call the
wrapper-removing methods "unwrap" because at this point that word is too
strongly tied to the Option/ Result methods.

# Safety

The safety contract of `TransparentWrapper` is relatively simple:

For a given `Wrapper` which implements `TransparentWrapper<Inner>`:

1. `Wrapper` must be a wrapper around `Inner` with an identical data
   representations. This    either means that it must be a
   `#[repr(transparent)]` struct which    contains a either a field of type
   `Inner` (or a field of some other    transparent wrapper for `Inner`) as
   the only non-ZST field.

2. Any fields *other* than the `Inner` field must be trivially constructable
   ZSTs, for example `PhantomData`, `PhantomPinned`, etc. (When deriving
   `TransparentWrapper` on a type with ZST fields, the ZST fields must be
   [`Zeroable`](#zeroable)).

3. The `Wrapper` may not impose additional alignment requirements over
   `Inner`.
    - Note: this is currently guaranteed by `repr(transparent)`, but there
      have been discussions of lifting it, so it's stated here explicitly.

4. All functions on `TransparentWrapper` **may not** be overridden.

## Caveats

If the wrapper imposes additional constraints upon the inner type which are
required for safety, it's responsible for ensuring those still hold -- this
generally requires preventing access to instances of the inner type, as
implementing `TransparentWrapper<U> for T` means anybody can call
`T::cast_ref(any_instance_of_u)`.

For example, it would be invalid to implement TransparentWrapper for `str`
to implement `TransparentWrapper` around `[u8]` because of this.

# Examples

## Basic

```rust
use bytemuck::TransparentWrapper;
#[derive(Default)]
struct SomeStruct(u32);

#[repr(transparent)]
struct MyWrapper(SomeStruct);

unsafe impl TransparentWrapper<SomeStruct> for MyWrapper {}

// interpret a reference to &SomeStruct as a &MyWrapper
let thing = SomeStruct::default();
let inner_ref: &MyWrapper = MyWrapper::wrap_ref(&thing);

// Works with &mut too.
let mut mut_thing = SomeStruct::default();
let inner_mut: &mut MyWrapper = MyWrapper::wrap_mut(&mut mut_thing);

let _ = (inner_ref, inner_mut); // silence warnings
```

## Use with dynamically sized types

```rust
use bytemuck::TransparentWrapper;

#[repr(transparent)]
struct Slice<T>([T]);

unsafe impl<T> TransparentWrapper<[T]> for Slice<T> {}

let s = Slice::wrap_ref(&[1u32, 2, 3]);
assert_eq!(&s.0, &[1, 2, 3]);

let mut buf = [1, 2, 3u8];
let sm = Slice::wrap_mut(&mut buf);
```

## Deriving

When deriving, the non-wrapped fields must uphold all the normal
requirements, and must also be `Zeroable`.
```rust
use bytemuck::TransparentWrapper;
use std::marker::PhantomData;

#[derive(TransparentWrapper)]
#[repr(transparent)]
#[transparent(usize)]
struct Wrapper<T: ?Sized>(usize, PhantomData<T>); // PhantomData<T> implements Zeroable for all T
```

Here, an error will occur, because `MyZst` does not implement `Zeroable`.
```compile_fail
use bytemuck::TransparentWrapper;
struct MyZst;

#[derive(TransparentWrapper)]
#[repr(transparent)]
#[transparent(usize)]
struct Wrapper(usize, MyZst); // MyZst does not implement Zeroable
```

#### Provided Methods

- `fn wrap(s: Inner) -> Self`

  Convert the inner type into the wrapper type.

- `fn wrap_ref(s: &Inner) -> &Self`

  Convert a reference to the inner type into a reference to the wrapper

- `fn wrap_mut(s: &mut Inner) -> &mut Self`

  Convert a mutable reference to the inner type into a mutable reference to

- `fn wrap_slice(s: &[Inner]) -> &[Self]`

  Convert a slice to the inner type into a slice to the wrapper type.

- `fn wrap_slice_mut(s: &mut [Inner]) -> &mut [Self]`

  Convert a mutable slice to the inner type into a mutable slice to the

- `fn peel(s: Self) -> Inner`

  Convert the wrapper type into the inner type.

- `fn peel_ref(s: &Self) -> &Inner`

  Convert a reference to the wrapper type into a reference to the inner

- `fn peel_mut(s: &mut Self) -> &mut Inner`

  Convert a mutable reference to the wrapper type into a mutable reference

- `fn peel_slice(s: &[Self]) -> &[Inner]`

  Convert a slice to the wrapped type into a slice to the inner type.

- `fn peel_slice_mut(s: &mut [Self]) -> &mut [Inner]`

  Convert a mutable slice to the wrapped type into a mutable slice to the

#### Implementors

- `core::num::Wrapping<T>`

## Functions

### `bytes_of`

```rust
fn bytes_of<T: NoUninit>(t: &T) -> &[u8]
```

Re-interprets `&T` as `&[u8]`.

Any ZST becomes an empty slice, and in that case the pointer value of that
empty slice might not match the pointer value of the input reference.

### `bytes_of_mut`

```rust
fn bytes_of_mut<T: NoUninit + AnyBitPattern>(t: &mut T) -> &mut [u8]
```

Re-interprets `&mut T` as `&mut [u8]`.

Any ZST becomes an empty slice, and in that case the pointer value of that
empty slice might not match the pointer value of the input reference.

### `from_bytes`

```rust
fn from_bytes<T: AnyBitPattern>(s: &[u8]) -> &T
```

Re-interprets `&[u8]` as `&T`.

## Panics

This is like [`try_from_bytes`](#try-from-bytes) but will panic on error.

### `from_bytes_mut`

```rust
fn from_bytes_mut<T: NoUninit + AnyBitPattern>(s: &mut [u8]) -> &mut T
```

Re-interprets `&mut [u8]` as `&mut T`.

## Panics

This is like [`try_from_bytes_mut`](#try-from-bytes-mut) but will panic on error.

### `try_pod_read_unaligned`

```rust
fn try_pod_read_unaligned<T: AnyBitPattern>(bytes: &[u8]) -> Result<T, PodCastError>
```

Reads from the bytes as if they were a `T`.

Unlike [`from_bytes`](#from-bytes), the slice doesn't need to respect alignment of `T`,
only sizes must match.

## Failure
* If the `bytes` length is not equal to `size_of::<T>()`.

### `pod_read_unaligned`

```rust
fn pod_read_unaligned<T: AnyBitPattern>(bytes: &[u8]) -> T
```

Reads the slice into a `T` value.

Unlike [`from_bytes`](#from-bytes), the slice doesn't need to respect alignment of `T`,
only sizes must match.

## Panics
* This is like `try_pod_read_unaligned` but will panic on failure.

### `try_from_bytes`

```rust
fn try_from_bytes<T: AnyBitPattern>(s: &[u8]) -> Result<&T, PodCastError>
```

Re-interprets `&[u8]` as `&T`.

## Failure

* If the slice isn't aligned for the new type
* If the slice's length isn’t exactly the size of the new type

### `try_from_bytes_mut`

```rust
fn try_from_bytes_mut<T: NoUninit + AnyBitPattern>(s: &mut [u8]) -> Result<&mut T, PodCastError>
```

Re-interprets `&mut [u8]` as `&mut T`.

## Failure

* If the slice isn't aligned for the new type
* If the slice's length isn’t exactly the size of the new type

### `cast`

```rust
fn cast<A: NoUninit, B: AnyBitPattern>(a: A) -> B
```

Cast `A` into `B`

## Panics

* This is like [`try_cast`](#try-cast), but will panic on a size mismatch.

### `cast_mut`

```rust
fn cast_mut<A: NoUninit + AnyBitPattern, B: NoUninit + AnyBitPattern>(a: &mut A) -> &mut B
```

Cast `&mut A` into `&mut B`.

## Panics

This is [`try_cast_mut`](#try-cast-mut) but will panic on error.

### `cast_ref`

```rust
fn cast_ref<A: NoUninit, B: AnyBitPattern>(a: &A) -> &B
```

Cast `&A` into `&B`.

## Panics

This is [`try_cast_ref`](#try-cast-ref) but will panic on error.

### `cast_slice`

```rust
fn cast_slice<A: NoUninit, B: AnyBitPattern>(a: &[A]) -> &[B]
```

Cast `&[A]` into `&[B]`.

## Panics

This is [`try_cast_slice`](#try-cast-slice) but will panic on error.

### `cast_slice_mut`

```rust
fn cast_slice_mut<A: NoUninit + AnyBitPattern, B: NoUninit + AnyBitPattern>(a: &mut [A]) -> &mut [B]
```

Cast `&mut [A]` into `&mut [B]`.

## Panics

This is [`try_cast_slice_mut`](#try-cast-slice-mut) but will panic on error.

### `pod_align_to`

```rust
fn pod_align_to<T: NoUninit, U: AnyBitPattern>(vals: &[T]) -> (&[T], &[U], &[T])
```

As [`align_to`](https://doc.rust-lang.org/std/primitive.slice.html#method.align_to),
but safe because of the [`Pod`](#pod) bound.

### `pod_align_to_mut`

```rust
fn pod_align_to_mut<T: NoUninit + AnyBitPattern, U: NoUninit + AnyBitPattern>(vals: &mut [T]) -> (&mut [T], &mut [U], &mut [T])
```

As [`align_to_mut`](https://doc.rust-lang.org/std/primitive.slice.html#method.align_to_mut),
but safe because of the [`Pod`](#pod) bound.

### `try_cast`

```rust
fn try_cast<A: NoUninit, B: AnyBitPattern>(a: A) -> Result<B, PodCastError>
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
fn try_cast_ref<A: NoUninit, B: AnyBitPattern>(a: &A) -> Result<&B, PodCastError>
```

Try to convert a `&A` into `&B`.

## Failure

* If the reference isn't aligned in the new type
* If the source type and target type aren't the same size.

### `try_cast_mut`

```rust
fn try_cast_mut<A: NoUninit + AnyBitPattern, B: NoUninit + AnyBitPattern>(a: &mut A) -> Result<&mut B, PodCastError>
```

Try to convert a `&mut A` into `&mut B`.

As [`try_cast_ref`](#try-cast-ref), but `mut`.

### `try_cast_slice`

```rust
fn try_cast_slice<A: NoUninit, B: AnyBitPattern>(a: &[A]) -> Result<&[B], PodCastError>
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
* Similarly, you can't convert between a [ZST](https://doc.rust-lang.org/nomicon/exotic-sizes.html#zero-sized-types-zsts)
  and a non-ZST.

### `try_cast_slice_mut`

```rust
fn try_cast_slice_mut<A: NoUninit + AnyBitPattern, B: NoUninit + AnyBitPattern>(a: &mut [A]) -> Result<&mut [B], PodCastError>
```

Try to convert `&mut [A]` into `&mut [B]` (possibly with a change in
length).

As [`try_cast_slice`](#try-cast-slice), but `&mut`.

### `write_zeroes`

```rust
fn write_zeroes<T: Zeroable>(target: &mut T)
```

Fill all bytes of `target` with zeroes (see [`Zeroable`](#zeroable)).

This is similar to `*target = Zeroable::zeroed()`, but guarantees that any
padding bytes in `target` are zeroed as well.

See also [`fill_zeroes`](#fill-zeroes), if you have a slice rather than a single value.

### `fill_zeroes`

```rust
fn fill_zeroes<T: Zeroable>(slice: &mut [T])
```

Fill all bytes of `slice` with zeroes (see [`Zeroable`](#zeroable)).

This is similar to `slice.fill(Zeroable::zeroed())`, but guarantees that any
padding bytes in `slice` are zeroed as well.

See also [`write_zeroes`](#write-zeroes), which zeroes all bytes of a single value rather
than a slice.

## Macros

### `impl_unsafe_marker_for_array!`

### `transmute!`

A macro to transmute between two types without requiring knowing size
statically.

### `impl_unsafe_marker_for_simd!`

A macro to implement marker traits for various simd types.
#[allow(unused)] because the impls are only compiled on relevant platforms
with relevant cargo features enabled.

### `maybe_const_fn!`

A macro for conditionally const-ifying a function.
#[allow(unused)] because currently it is only used with the `must_cast` feature.

### `offset_of!`

Find the offset in bytes of the given `$field` of `$Type`. Requires an
already initialized `$instance` value to work with.

This is similar to the macro from [`memoffset`](https://docs.rs/memoffset),
however it uses no `unsafe` code.

This macro has a 3-argument and 2-argument version.
* In the 3-arg version you specify an instance of the type, the type itself,
  and the field name.
* In the 2-arg version the macro will call the [`default`](Default::default)
  method to make a temporary instance of the type for you.

The output of this macro is the byte offset of the field (as a `usize`). The
calculations of the macro are fixed across the entire program, but if the
type used is `repr(Rust)` then they're *not* fixed across compilations or
compilers.

## Examples

### 3-arg Usage

```rust
use bytemuck::offset_of;
// enums can't derive default, and for this example we don't pick one
enum MyExampleEnum {
  A,
  B,
  C,
}

// so now our struct here doesn't have Default
#[repr(C)]
struct MyNotDefaultType {
  pub counter: i32,
  pub some_field: MyExampleEnum,
}

// but we provide an instance of the type and it's all good.
let val = MyNotDefaultType { counter: 5, some_field: MyExampleEnum::A };
assert_eq!(offset_of!(val, MyNotDefaultType, some_field), 4);
```

### 2-arg Usage

```rust
use bytemuck::offset_of;
#[derive(Default)]
#[repr(C)]
struct Vertex {
  pub loc: [f32; 3],
  pub color: [f32; 3],
}
// if the type impls Default the macro can make its own default instance.
assert_eq!(offset_of!(Vertex, loc), 0);
assert_eq!(offset_of!(Vertex, color), 12);
```

# Usage with `#[repr(packed)]` structs

Attempting to compute the offset of a `#[repr(packed)]` struct with
`bytemuck::offset_of!` requires an `unsafe` block. We hope to relax this in
the future, but currently it is required to work around a soundness hole in
Rust (See [rust-lang/rust#27060]).

<p style="background:rgba(255,181,77,0.16);padding:0.75em;">
<strong>Warning:</strong> This is only true for versions of bytemuck >
1.4.0. Previous versions of
<code style="background:rgba(41,24,0,0.1);">bytemuck::offset_of!</code>
will only emit a warning when used on the field of a packed struct in safe
code, which can lead to unsoundness.
</p>

For example, the following will fail to compile:

```compile_fail
#[repr(C, packed)]
#[derive(Default)]
struct Example {
  field: u32,
}
// Doesn't compile:
let _offset = bytemuck::offset_of!(Example, field);
```

While the error message this generates will mention the
`safe_packed_borrows` lint, the macro will still fail to compile even if
that lint is `#[allow]`ed:

```compile_fail
#[repr(C, packed)] #[derive(Default)] struct Example { field: u32 }
// Still doesn't compile:
#[allow(safe_packed_borrows)]
{
  let _offset = bytemuck::offset_of!(Example, field);
}
```

This *can* be worked around by using `unsafe`, but it is only sound to do so
if you can guarantee that taking a reference to the field is sound.

In practice, this means it only works for fields of align(1) types, or if
you know the field's offset in advance (defeating the point of `offset_of`)
and can prove that the struct's alignment and the field's offset are enough
to prove the field's alignment.

Once the `raw_ref` macros are available, a future version of this crate will
use them to lift the limitations of packed structs. For the duration of the
`1.x` version of this crate that will be behind an on-by-default cargo
feature (to maintain minimum rust version support).

### `impl_for_fn!`

### `impl_contiguous!`

