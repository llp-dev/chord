*[zerocopy](../index.md) / [error](index.md)*

---

# Module `error`

Types related to error reporting.

## Single failure mode errors

Generally speaking, zerocopy's conversions may fail for one of up to three
reasons:
- [`AlignmentError`](../index.md): the conversion source was improperly aligned
- [`SizeError`](../index.md): the conversion source was of incorrect size
- [`ValidityError`](../index.md): the conversion source contained invalid data

Methods that only have one failure mode, like
`FromBytes::read_from_bytes`, return that mode's corresponding error type
directly.

## Compound errors

Conversion methods that have either two or three possible failure modes
return one of these error types:
- [`CastError`](../index.md): the error type of reference conversions
- [`TryCastError`](../index.md): the error type of fallible reference conversions
- [`TryReadError`](../index.md): the error type of fallible read conversions

## [`Unaligned`](../index.md) destination types

For [`Unaligned`](../index.md) destination types, alignment errors are impossible. All
compound error types support infallibly discarding the alignment error via
[`From`](../../thiserror_impl/attr/index.md) so long as `Dst: Unaligned`. For example, see [`<SizeError as
From<ConvertError>>::from`][size-error-from].

## Accessing the conversion source

All error types provide an `into_src` method that converts the error into
the source value underlying the failed conversion.

## Display formatting

All error types provide a `Display` implementation that produces a
human-readable error message. When `debug_assertions` are enabled, these
error messages are verbose and may include potentially sensitive
information, including:

- the names of the involved types
- the sizes of the involved types
- the addresses of the involved types
- the contents of the involved types

When `debug_assertions` are disabled (as is default for `release` builds),
such potentially sensitive information is excluded.

In the future, we may support manually configuring this behavior. If you are
interested in this feature, [let us know on GitHub][issue-1457] so we know
to prioritize it.

## Validation order

Our conversion methods typically check alignment, then size, then bit
validity. However, we do not guarantee that this is always the case, and
this behavior may change between releases.

## `Send`, `Sync`, and `'static`

Our error types are `Send`, `Sync`, and `'static` when their `Src` parameter
is `Send`, `Sync`, or `'static`, respectively. This can cause issues when an
error is sent or synchronized across threads; e.g.:

```compile_fail,E0515
use zerocopy::*;

let result: SizeError<&[u8], u32> = std::thread::spawn(|| {
    let source = &mut [0u8, 1, 2][..];
    // Try (and fail) to read a `u32` from `source`.
    u32::read_from_bytes(source).unwrap_err()
}).join().unwrap();
```

To work around this, use `map_src` to convert the
source parameter to an unproblematic type; e.g.:

```rust
use zerocopy::*;

let result: SizeError<(), u32> = std::thread::spawn(|| {
    let source = &mut [0u8, 1, 2][..];
    // Try (and fail) to read a `u32` from `source`.
    u32::read_from_bytes(source).unwrap_err()
        // Erase the error source.
        .map_src(drop)
}).join().unwrap();
```

Alternatively, use `.to_string()` to eagerly convert the error into a
human-readable message; e.g.:

```rust
use zerocopy::*;

let result: Result<u32, String> = std::thread::spawn(|| {
    let source = &mut [0u8, 1, 2][..];
    // Try (and fail) to read a `u32` from `source`.
    u32::read_from_bytes(source)
        // Eagerly render the error message.
        .map_err(|err| err.to_string())
}).join().unwrap();
```

## Contents

- [Structs](#structs)
  - [`AlignmentError`](#alignmenterror)
  - [`SizeError`](#sizeerror)
  - [`ValidityError`](#validityerror)
  - [`AllocError`](#allocerror)
- [Enums](#enums)
  - [`ConvertError`](#converterror)
- [Type Aliases](#type-aliases)
  - [`CastError`](#casterror)
  - [`TryCastError`](#trycasterror)
  - [`TryReadError`](#tryreaderror)
  - [`AlignedTryCastError`](#alignedtrycasterror)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`AlignmentError`](#alignmenterror) | struct | The error emitted if the conversion source is improperly aligned. |
| [`SizeError`](#sizeerror) | struct | The error emitted if the conversion source is of incorrect size. |
| [`ValidityError`](#validityerror) | struct | The error emitted if the conversion source contains invalid data. |
| [`AllocError`](#allocerror) | struct | The error type of a failed allocation. |
| [`ConvertError`](#converterror) | enum | Zerocopy's generic error type. |
| [`CastError`](#casterror) | type | The error type of reference conversions. |
| [`TryCastError`](#trycasterror) | type | The error type of fallible reference conversions. |
| [`TryReadError`](#tryreaderror) | type | The error type of fallible read-conversions. |
| [`AlignedTryCastError`](#alignedtrycasterror) | type | The error type of well-aligned, fallible casts. |

## Structs

### `AlignmentError<Src, Dst: ?Sized>`

```rust
struct AlignmentError<Src, Dst: ?Sized> {
    src: Src,
    _dst: crate::util::SendSyncPhantomData<Dst>,
}
```

The error emitted if the conversion source is improperly aligned.

#### Fields

- **`src`**: `Src`

  The source value involved in the conversion.

- **`_dst`**: `crate::util::SendSyncPhantomData<Dst>`

  The inner destination type involved in the conversion.
  
  INVARIANT: An `AlignmentError` may only be constructed if `Dst`'s
  alignment requirement is greater than one.

#### Implementations

- <span id="alignmenterror-new-unchecked"></span>`unsafe fn new_unchecked(src: Src) -> Self`

  # Safety

  

  The caller must ensure that `Dst`'s alignment requirement is greater

  than one.

- <span id="alignmenterror-into-src"></span>`fn into_src(self) -> Src`

  Produces the source underlying the failed conversion.

- <span id="alignmenterror-with-src"></span>`fn with_src<NewSrc>(self, new_src: NewSrc) -> AlignmentError<NewSrc, Dst>` — [`AlignmentError`](../index.md#alignmenterror)

- <span id="alignmenterror-map-src"></span>`fn map_src<NewSrc>(self, f: impl FnOnce(Src) -> NewSrc) -> AlignmentError<NewSrc, Dst>` — [`AlignmentError`](../index.md#alignmenterror)

  Maps the source value associated with the conversion error.

  

  This can help mitigate [issues with `Send`, `Sync` and `'static`

  bounds][self#send-sync-and-static].

  

  # Examples

  

  ```rust

  use zerocopy::*;

  

  let unaligned = Unalign::new(0u16);

  

  // Attempt to deref `unaligned`. This might fail with an alignment error.

  let maybe_n: Result<&u16, AlignmentError<&Unalign<u16>, u16>> = unaligned.try_deref();

  

  // Map the error's source to its address as a usize.

  let maybe_n: Result<&u16, AlignmentError<usize, u16>> = maybe_n.map_err(|err| {

      err.map_src(|src| src as *const _ as usize)

  });

  ```

- <span id="alignmenterror-into"></span>`fn into<S, V>(self) -> ConvertError<Self, S, V>` — [`ConvertError`](../index.md#converterror)

- <span id="alignmenterror-display-verbose-extras"></span>`fn display_verbose_extras(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

  Format extra details for a verbose, human-readable error message.

  

  This formatting may include potentially sensitive information.

#### Trait Implementations

##### `impl<Src: Clone, Dst: ?Sized> Clone for AlignmentError<Src, Dst>`

- <span id="alignmenterror-clone"></span>`fn clone(&self) -> Self`

##### `impl<Src, Dst: ?Sized> Debug for AlignmentError<Src, Dst>`

- <span id="alignmenterror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Src, Dst> Display for AlignmentError<Src, Dst>`

- <span id="alignmenterror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Src: Eq, Dst: ?Sized> Eq for AlignmentError<Src, Dst>`

##### `impl<Src, Dst> Error for AlignmentError<Src, Dst>`

##### `impl<Src: PartialEq, Dst: ?Sized> PartialEq for AlignmentError<Src, Dst>`

- <span id="alignmenterror-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl ToString for AlignmentError<Src, Dst>`

- <span id="alignmenterror-tostring-to-string"></span>`fn to_string(&self) -> String`

### `SizeError<Src, Dst: ?Sized>`

```rust
struct SizeError<Src, Dst: ?Sized> {
    src: Src,
    _dst: crate::util::SendSyncPhantomData<Dst>,
}
```

The error emitted if the conversion source is of incorrect size.

#### Fields

- **`src`**: `Src`

  The source value involved in the conversion.

- **`_dst`**: `crate::util::SendSyncPhantomData<Dst>`

  The inner destination type involved in the conversion.

#### Implementations

- <span id="sizeerror-new"></span>`fn new(src: Src) -> Self`

- <span id="sizeerror-into-src"></span>`fn into_src(self) -> Src`

  Produces the source underlying the failed conversion.

- <span id="sizeerror-with-src"></span>`fn with_src<NewSrc>(self, new_src: NewSrc) -> SizeError<NewSrc, Dst>` — [`SizeError`](../index.md#sizeerror)

  Sets the source value associated with the conversion error.

- <span id="sizeerror-map-src"></span>`fn map_src<NewSrc>(self, f: impl FnOnce(Src) -> NewSrc) -> SizeError<NewSrc, Dst>` — [`SizeError`](../index.md#sizeerror)

  Maps the source value associated with the conversion error.

  

  This can help mitigate [issues with `Send`, `Sync` and `'static`

  bounds][self#send-sync-and-static].

  

  # Examples

  

  ```rust

  use zerocopy::*;

  

  let source: [u8; 3] = [0, 1, 2];

  

  // Try to read a `u32` from `source`. This will fail because there are insufficient

  // bytes in `source`.

  let maybe_u32: Result<u32, SizeError<&[u8], u32>> = u32::read_from_bytes(&source[..]);

  

  // Map the error's source to its size.

  let maybe_u32: Result<u32, SizeError<usize, u32>> = maybe_u32.map_err(|err| {

      err.map_src(|src| src.len())

  });

  ```

- <span id="sizeerror-with-dst"></span>`fn with_dst<NewDst: ?Sized>(self) -> SizeError<Src, NewDst>` — [`SizeError`](../index.md#sizeerror)

  Sets the destination type associated with the conversion error.

- <span id="sizeerror-into"></span>`fn into<A, V>(self) -> ConvertError<A, Self, V>` — [`ConvertError`](../index.md#converterror)

  Converts the error into a general [`ConvertError`](../index.md).

- <span id="sizeerror-display-verbose-extras"></span>`fn display_verbose_extras(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

  Format extra details for a verbose, human-readable error message.

  

  This formatting may include potentially sensitive information.

#### Trait Implementations

##### `impl<Src: Clone, Dst: ?Sized> Clone for SizeError<Src, Dst>`

- <span id="sizeerror-clone"></span>`fn clone(&self) -> Self`

##### `impl<Src, Dst: ?Sized> Debug for SizeError<Src, Dst>`

- <span id="sizeerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Src, Dst> Display for SizeError<Src, Dst>`

- <span id="sizeerror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Src: Eq, Dst: ?Sized> Eq for SizeError<Src, Dst>`

##### `impl<Src, Dst> Error for SizeError<Src, Dst>`

##### `impl<Src: PartialEq, Dst: ?Sized> PartialEq for SizeError<Src, Dst>`

- <span id="sizeerror-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl ToString for SizeError<Src, Dst>`

- <span id="sizeerror-tostring-to-string"></span>`fn to_string(&self) -> String`

### `ValidityError<Src, Dst: ?Sized + TryFromBytes>`

```rust
struct ValidityError<Src, Dst: ?Sized + TryFromBytes> {
    src: Src,
    _dst: crate::util::SendSyncPhantomData<Dst>,
}
```

The error emitted if the conversion source contains invalid data.

#### Fields

- **`src`**: `Src`

  The source value involved in the conversion.

- **`_dst`**: `crate::util::SendSyncPhantomData<Dst>`

  The inner destination type involved in the conversion.

#### Implementations

- <span id="validityerror-new"></span>`fn new(src: Src) -> Self`

- <span id="validityerror-into-src"></span>`fn into_src(self) -> Src`

  Produces the source underlying the failed conversion.

- <span id="validityerror-map-src"></span>`fn map_src<NewSrc>(self, f: impl FnOnce(Src) -> NewSrc) -> ValidityError<NewSrc, Dst>` — [`ValidityError`](../index.md#validityerror)

  Maps the source value associated with the conversion error.

  

  This can help mitigate [issues with `Send`, `Sync` and `'static`

  bounds][self#send-sync-and-static].

  

  # Examples

  

  ```rust

  use zerocopy::*;

  

  let source: u8 = 42;

  

  // Try to transmute the `source` to a `bool`. This will fail.

  let maybe_bool: Result<bool, ValidityError<u8, bool>> = try_transmute!(source);

  

  // Drop the error's source.

  let maybe_bool: Result<bool, ValidityError<(), bool>> = maybe_bool.map_err(|err| {

      err.map_src(drop)

  });

  ```

- <span id="validityerror-into"></span>`fn into<A, S>(self) -> ConvertError<A, S, Self>` — [`ConvertError`](../index.md#converterror)

  Converts the error into a general [`ConvertError`](../index.md).

- <span id="validityerror-display-verbose-extras"></span>`fn display_verbose_extras(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

  Format extra details for a verbose, human-readable error message.

  

  This formatting may include potentially sensitive information.

#### Trait Implementations

##### `impl<Src: Clone, Dst: ?Sized + TryFromBytes> Clone for ValidityError<Src, Dst>`

- <span id="validityerror-clone"></span>`fn clone(&self) -> Self`

##### `impl<Src, Dst: ?Sized + TryFromBytes> Debug for ValidityError<Src, Dst>`

- <span id="validityerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Src, Dst> Display for ValidityError<Src, Dst>`

- <span id="validityerror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Src: Eq, Dst: ?Sized + TryFromBytes> Eq for ValidityError<Src, Dst>`

##### `impl<Src, Dst> Error for ValidityError<Src, Dst>`

##### `impl<Src: PartialEq, Dst: ?Sized + TryFromBytes> PartialEq for ValidityError<Src, Dst>`

- <span id="validityerror-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl ToString for ValidityError<Src, Dst>`

- <span id="validityerror-tostring-to-string"></span>`fn to_string(&self) -> String`

### `AllocError`

```rust
struct AllocError;
```

The error type of a failed allocation.

This type is intended to be deprecated in favor of the standard library's
[`AllocError`](../index.md) type once it is stabilized. When that happens, this type will
be replaced by a type alias to the standard library type. We do not intend
to treat this as a breaking change; users who wish to avoid breakage should
avoid writing code which assumes that this is *not* such an alias. For
example, implementing the same trait for both types will result in an impl
conflict once this type is an alias.


#### Trait Implementations

##### `impl Clone for AllocError`

- <span id="allocerror-clone"></span>`fn clone(&self) -> AllocError` — [`AllocError`](../index.md#allocerror)

##### `impl Copy for AllocError`

##### `impl Debug for AllocError`

- <span id="allocerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for AllocError`

##### `impl PartialEq for AllocError`

- <span id="allocerror-partialeq-eq"></span>`fn eq(&self, other: &AllocError) -> bool` — [`AllocError`](../index.md#allocerror)

##### `impl StructuralPartialEq for AllocError`

## Enums

### `ConvertError<A, S, V>`

```rust
enum ConvertError<A, S, V> {
    Alignment(A),
    Size(S),
    Validity(V),
}
```

Zerocopy's generic error type.

Generally speaking, zerocopy's conversions may fail for one of up to three
reasons:
- [`AlignmentError`](../index.md): the conversion source was improperly aligned
- [`SizeError`](../index.md): the conversion source was of incorrect size
- [`ValidityError`](../index.md): the conversion source contained invalid data

However, not all conversions produce all errors. For instance,
`FromBytes::ref_from_bytes` may fail due to alignment or size issues, but
not validity issues. This generic error type captures these
(im)possibilities via parameterization: `A` is parameterized with
[`AlignmentError`](../index.md), `S` is parameterized with [`SizeError`](../index.md), and `V` is
parameterized with [`Infallible`](../../hashbrown/index.md).

Zerocopy never uses this type directly in its API. Rather, we provide three
pre-parameterized aliases:
- [`CastError`](../index.md): the error type of reference conversions
- [`TryCastError`](../index.md): the error type of fallible reference conversions
- [`TryReadError`](../index.md): the error type of fallible read conversions

#### Variants

- **`Alignment`**

  The conversion source was improperly aligned.

- **`Size`**

  The conversion source was of incorrect size.

- **`Validity`**

  The conversion source contained invalid data.

#### Implementations

- <span id="converterror-into-src"></span>`fn into_src(self) -> Src`

  Produces the source underlying the failed conversion.

- <span id="converterror-with-src"></span>`fn with_src<NewSrc>(self, new_src: NewSrc) -> CastError<NewSrc, Dst>` — [`CastError`](../index.md#casterror)

  Sets the source value associated with the conversion error.

- <span id="converterror-map-src"></span>`fn map_src<NewSrc>(self, f: impl FnOnce(Src) -> NewSrc) -> CastError<NewSrc, Dst>` — [`CastError`](../index.md#casterror)

  Maps the source value associated with the conversion error.

  

  This can help mitigate [issues with `Send`, `Sync` and `'static`

  bounds][self#send-sync-and-static].

  

  # Examples

  

  ```rust

  use zerocopy::*;

  

  let source: [u8; 3] = [0, 1, 2];

  

  // Try to read a `u32` from `source`. This will fail because there are insufficient

  // bytes in `source`.

  let maybe_u32: Result<&u32, CastError<&[u8], u32>> = u32::ref_from_bytes(&source[..]);

  

  // Map the error's source to its size and address.

  let maybe_u32: Result<&u32, CastError<(usize, usize), u32>> = maybe_u32.map_err(|err| {

      err.map_src(|src| (src.len(), src.as_ptr() as usize))

  });

  ```

- <span id="converterror-into"></span>`fn into(self) -> TryCastError<Src, Dst>` — [`TryCastError`](../index.md#trycasterror)

  Converts the error into a general [`ConvertError`](../index.md).

#### Trait Implementations

##### `impl<A: clone::Clone, S: clone::Clone, V: clone::Clone> Clone for ConvertError<A, S, V>`

- <span id="converterror-clone"></span>`fn clone(&self) -> ConvertError<A, S, V>` — [`ConvertError`](../index.md#converterror)

##### `impl<A: fmt::Debug, S: fmt::Debug, V: fmt::Debug> Debug for ConvertError<A, S, V>`

- <span id="converterror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<A: fmt::Display, S: fmt::Display, V: fmt::Display> Display for ConvertError<A, S, V>`

- <span id="converterror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<A: cmp::Eq, S: cmp::Eq, V: cmp::Eq> Eq for ConvertError<A, S, V>`

##### `impl<A, S, V> Error for ConvertError<A, S, V>`

##### `impl<A: cmp::PartialEq, S: cmp::PartialEq, V: cmp::PartialEq> PartialEq for ConvertError<A, S, V>`

- <span id="converterror-partialeq-eq"></span>`fn eq(&self, other: &ConvertError<A, S, V>) -> bool` — [`ConvertError`](../index.md#converterror)

##### `impl<A, S, V> StructuralPartialEq for ConvertError<A, S, V>`

##### `impl ToString for ConvertError<A, S, V>`

- <span id="converterror-tostring-to-string"></span>`fn to_string(&self) -> String`

## Type Aliases

### `CastError<Src, Dst: ?Sized>`

```rust
type CastError<Src, Dst: ?Sized> = ConvertError<AlignmentError<Src, Dst>, SizeError<Src, Dst>, core::convert::Infallible>;
```

The error type of reference conversions.

Reference conversions, like `FromBytes::ref_from_bytes` may emit
[alignment](AlignmentError) and [size](SizeError) errors.

### `TryCastError<Src, Dst: ?Sized + TryFromBytes>`

```rust
type TryCastError<Src, Dst: ?Sized + TryFromBytes> = ConvertError<AlignmentError<Src, Dst>, SizeError<Src, Dst>, ValidityError<Src, Dst>>;
```

The error type of fallible reference conversions.

Fallible reference conversions, like `TryFromBytes::try_ref_from_bytes`
may emit [alignment](AlignmentError), [size](SizeError), and
[validity](ValidityError) errors.

### `TryReadError<Src, Dst: ?Sized + TryFromBytes>`

```rust
type TryReadError<Src, Dst: ?Sized + TryFromBytes> = ConvertError<core::convert::Infallible, SizeError<Src, Dst>, ValidityError<Src, Dst>>;
```

The error type of fallible read-conversions.

Fallible read-conversions, like `TryFromBytes::try_read_from_bytes` may
emit [size](SizeError) and [validity](ValidityError) errors, but not
alignment errors.

### `AlignedTryCastError<Src, Dst: ?Sized + TryFromBytes>`

```rust
type AlignedTryCastError<Src, Dst: ?Sized + TryFromBytes> = ConvertError<core::convert::Infallible, SizeError<Src, Dst>, ValidityError<Src, Dst>>;
```

The error type of well-aligned, fallible casts.

This is like [`TryCastError`](../index.md), but for casts that are always well-aligned.
It is identical to `TryCastError`, except that its alignment error is
[`Infallible`](../../hashbrown/index.md).

As of this writing, none of zerocopy's API produces this error directly.
However, it is useful since it permits users to infallibly discard alignment
errors when they can prove statically that alignment errors are impossible.

# Examples

```rust
use core::convert::Infallible;
use zerocopy::*;
use zerocopy_derive::*;

#[derive(TryFromBytes, KnownLayout, Unaligned, Immutable)]
#[repr(C, packed)]
struct Bools {
    one: bool,
    two: bool,
    many: [bool],
}

impl Bools {
    fn parse(bytes: &[u8]) -> Result<&Bools, AlignedTryCastError<&[u8], Bools>> {
        // Since `Bools: Unaligned`, we can infallibly discard
        // the alignment error.
        Bools::try_ref_from_bytes(bytes).map_err(Into::into)
    }
}
```

