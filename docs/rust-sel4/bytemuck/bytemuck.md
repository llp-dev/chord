**bytemuck**

# Module: bytemuck

## Contents

**Modules**

- [`checked`](#checked) - Checked versions of the casting functions exposed in crate root

**Macros**

- [`offset_of`](#offset_of) - Find the offset in bytes of the given `$field` of `$Type`. Requires an

**Enums**

- [`PodCastError`](#podcasterror) - The things that can go wrong when casting between [`Pod`] data forms.

**Functions**

- [`bytes_of`](#bytes_of) - Re-interprets `&T` as `&[u8]`.
- [`bytes_of_mut`](#bytes_of_mut) - Re-interprets `&mut T` as `&mut [u8]`.
- [`cast`](#cast) - Cast `A` into `B`
- [`cast_mut`](#cast_mut) - Cast `&mut A` into `&mut B`.
- [`cast_ref`](#cast_ref) - Cast `&A` into `&B`.
- [`cast_slice`](#cast_slice) - Cast `&[A]` into `&[B]`.
- [`cast_slice_mut`](#cast_slice_mut) - Cast `&mut [A]` into `&mut [B]`.
- [`fill_zeroes`](#fill_zeroes) - Fill all bytes of `slice` with zeroes (see [`Zeroable`]).
- [`from_bytes`](#from_bytes) - Re-interprets `&[u8]` as `&T`.
- [`from_bytes_mut`](#from_bytes_mut) - Re-interprets `&mut [u8]` as `&mut T`.
- [`pod_align_to`](#pod_align_to) - As [`align_to`](https://doc.rust-lang.org/std/primitive.slice.html#method.align_to),
- [`pod_align_to_mut`](#pod_align_to_mut) - As [`align_to_mut`](https://doc.rust-lang.org/std/primitive.slice.html#method.align_to_mut),
- [`pod_read_unaligned`](#pod_read_unaligned) - Reads the slice into a `T` value.
- [`try_cast`](#try_cast) - Try to cast `A` into `B`.
- [`try_cast_mut`](#try_cast_mut) - Try to convert a `&mut A` into `&mut B`.
- [`try_cast_ref`](#try_cast_ref) - Try to convert a `&A` into `&B`.
- [`try_cast_slice`](#try_cast_slice) - Try to convert `&[A]` into `&[B]` (possibly with a change in length).
- [`try_cast_slice_mut`](#try_cast_slice_mut) - Try to convert `&mut [A]` into `&mut [B]` (possibly with a change in
- [`try_from_bytes`](#try_from_bytes) - Re-interprets `&[u8]` as `&T`.
- [`try_from_bytes_mut`](#try_from_bytes_mut) - Re-interprets `&mut [u8]` as `&mut T`.
- [`try_pod_read_unaligned`](#try_pod_read_unaligned) - Reads from the bytes as if they were a `T`.
- [`write_zeroes`](#write_zeroes) - Fill all bytes of `target` with zeroes (see [`Zeroable`]).

---

## bytemuck::PodCastError

*Enum*

The things that can go wrong when casting between [`Pod`] data forms.

**Variants:**
- `TargetAlignmentGreaterAndInputNotAligned` - You tried to cast a reference into a reference to a type with a higher
- `OutputSliceWouldHaveSlop` - If the element size of a slice changes, then the output slice changes
- `SizeMismatch` - When casting an individual `T`, `&T`, or `&mut T` value the
- `AlignmentMismatch` - For this type of cast the alignments must be exactly the same and they

**Traits:** Eq, Copy

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &PodCastError) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> PodCastError`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## bytemuck::bytes_of

*Function*

Re-interprets `&T` as `&[u8]`.

Any ZST becomes an empty slice, and in that case the pointer value of that
empty slice might not match the pointer value of the input reference.

```rust
fn bytes_of<T>(t: &T) -> &[u8]
```



## bytemuck::bytes_of_mut

*Function*

Re-interprets `&mut T` as `&mut [u8]`.

Any ZST becomes an empty slice, and in that case the pointer value of that
empty slice might not match the pointer value of the input reference.

```rust
fn bytes_of_mut<T>(t: & mut T) -> & mut [u8]
```



## bytemuck::cast

*Function*

Cast `A` into `B`

## Panics

* This is like [`try_cast`], but will panic on a size mismatch.

```rust
fn cast<A, B>(a: A) -> B
```



## bytemuck::cast_mut

*Function*

Cast `&mut A` into `&mut B`.

## Panics

This is [`try_cast_mut`] but will panic on error.

```rust
fn cast_mut<A, B>(a: & mut A) -> & mut B
```



## bytemuck::cast_ref

*Function*

Cast `&A` into `&B`.

## Panics

This is [`try_cast_ref`] but will panic on error.

```rust
fn cast_ref<A, B>(a: &A) -> &B
```



## bytemuck::cast_slice

*Function*

Cast `&[A]` into `&[B]`.

## Panics

This is [`try_cast_slice`] but will panic on error.

```rust
fn cast_slice<A, B>(a: &[A]) -> &[B]
```



## bytemuck::cast_slice_mut

*Function*

Cast `&mut [A]` into `&mut [B]`.

## Panics

This is [`try_cast_slice_mut`] but will panic on error.

```rust
fn cast_slice_mut<A, B>(a: & mut [A]) -> & mut [B]
```



## Module: checked

Checked versions of the casting functions exposed in crate root
that support [`CheckedBitPattern`] types.



## bytemuck::fill_zeroes

*Function*

Fill all bytes of `slice` with zeroes (see [`Zeroable`]).

This is similar to `slice.fill(Zeroable::zeroed())`, but guarantees that any
padding bytes in `slice` are zeroed as well.

See also [`write_zeroes`], which zeroes all bytes of a single value rather
than a slice.

```rust
fn fill_zeroes<T>(slice: & mut [T])
```



## bytemuck::from_bytes

*Function*

Re-interprets `&[u8]` as `&T`.

## Panics

This is like [`try_from_bytes`] but will panic on error.

```rust
fn from_bytes<T>(s: &[u8]) -> &T
```



## bytemuck::from_bytes_mut

*Function*

Re-interprets `&mut [u8]` as `&mut T`.

## Panics

This is like [`try_from_bytes_mut`] but will panic on error.

```rust
fn from_bytes_mut<T>(s: & mut [u8]) -> & mut T
```



## bytemuck::offset_of

*Declarative Macro*

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
# use bytemuck::offset_of;
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
# use bytemuck::offset_of;
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

[rust-lang/rust#27060]: https://github.com/rust-lang/rust/issues/27060

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
# #[repr(C, packed)] #[derive(Default)] struct Example { field: u32 }
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

```rust
macro_rules! offset_of {
    ($instance:expr, $Type:path, $field:tt) => { ... };
    ($Type:path, $field:tt) => { ... };
}
```



## bytemuck::pod_align_to

*Function*

As [`align_to`](https://doc.rust-lang.org/std/primitive.slice.html#method.align_to),
but safe because of the [`Pod`] bound.

```rust
fn pod_align_to<T, U>(vals: &[T]) -> (&[T], &[U], &[T])
```



## bytemuck::pod_align_to_mut

*Function*

As [`align_to_mut`](https://doc.rust-lang.org/std/primitive.slice.html#method.align_to_mut),
but safe because of the [`Pod`] bound.

```rust
fn pod_align_to_mut<T, U>(vals: & mut [T]) -> (& mut [T], & mut [U], & mut [T])
```



## bytemuck::pod_read_unaligned

*Function*

Reads the slice into a `T` value.

Unlike [`from_bytes`], the slice doesn't need to respect alignment of `T`,
only sizes must match.

## Panics
* This is like `try_pod_read_unaligned` but will panic on failure.

```rust
fn pod_read_unaligned<T>(bytes: &[u8]) -> T
```



## bytemuck::try_cast

*Function*

Try to cast `A` into `B`.

Note that for this particular type of cast, alignment isn't a factor. The
input value is semantically copied into the function and then returned to a
new memory location which will have whatever the required alignment of the
output type is.

## Failure

* If the types don't have the same size this fails.

```rust
fn try_cast<A, B>(a: A) -> Result<B, PodCastError>
```



## bytemuck::try_cast_mut

*Function*

Try to convert a `&mut A` into `&mut B`.

As [`try_cast_ref`], but `mut`.

```rust
fn try_cast_mut<A, B>(a: & mut A) -> Result<& mut B, PodCastError>
```



## bytemuck::try_cast_ref

*Function*

Try to convert a `&A` into `&B`.

## Failure

* If the reference isn't aligned in the new type
* If the source type and target type aren't the same size.

```rust
fn try_cast_ref<A, B>(a: &A) -> Result<&B, PodCastError>
```



## bytemuck::try_cast_slice

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
* Similarly, you can't convert between a [ZST](https://doc.rust-lang.org/nomicon/exotic-sizes.html#zero-sized-types-zsts)
  and a non-ZST.

```rust
fn try_cast_slice<A, B>(a: &[A]) -> Result<&[B], PodCastError>
```



## bytemuck::try_cast_slice_mut

*Function*

Try to convert `&mut [A]` into `&mut [B]` (possibly with a change in
length).

As [`try_cast_slice`], but `&mut`.

```rust
fn try_cast_slice_mut<A, B>(a: & mut [A]) -> Result<& mut [B], PodCastError>
```



## bytemuck::try_from_bytes

*Function*

Re-interprets `&[u8]` as `&T`.

## Failure

* If the slice isn't aligned for the new type
* If the slice's length isn’t exactly the size of the new type

```rust
fn try_from_bytes<T>(s: &[u8]) -> Result<&T, PodCastError>
```



## bytemuck::try_from_bytes_mut

*Function*

Re-interprets `&mut [u8]` as `&mut T`.

## Failure

* If the slice isn't aligned for the new type
* If the slice's length isn’t exactly the size of the new type

```rust
fn try_from_bytes_mut<T>(s: & mut [u8]) -> Result<& mut T, PodCastError>
```



## bytemuck::try_pod_read_unaligned

*Function*

Reads from the bytes as if they were a `T`.

Unlike [`from_bytes`], the slice doesn't need to respect alignment of `T`,
only sizes must match.

## Failure
* If the `bytes` length is not equal to `size_of::<T>()`.

```rust
fn try_pod_read_unaligned<T>(bytes: &[u8]) -> Result<T, PodCastError>
```



## bytemuck::write_zeroes

*Function*

Fill all bytes of `target` with zeroes (see [`Zeroable`]).

This is similar to `*target = Zeroable::zeroed()`, but guarantees that any
padding bytes in `target` are zeroed as well.

See also [`fill_zeroes`], if you have a slice rather than a single value.

```rust
fn write_zeroes<T>(target: & mut T)
```



