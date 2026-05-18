# Crate `zerocopy`

***<span style="font-size: 140%">Fast, safe, <span
style="color:red;">compile error</span>. Pick two.</span>***

Zerocopy makes zero-cost memory manipulation effortless. We write `unsafe`
so you don't have to.

*For an overview of what's changed from zerocopy 0.7, check out our [release
notes][release-notes], which include a step-by-step upgrading guide.*

*Have questions? Need more out of zerocopy? Submit a [customer request
issue][customer-request-issue] or ask the maintainers on
[GitHub][github-q-a] or [Discord][discord]!*




# Overview

##### Conversion Traits

Zerocopy provides four derivable traits for zero-cost conversions:
- [`TryFromBytes`](#tryfrombytes) indicates that a type may safely be converted from
  certain byte sequences (conditional on runtime checks)
- [`FromZeros`](#fromzeros) indicates that a sequence of zero bytes represents a valid
  instance of a type
- [`FromBytes`](#frombytes) indicates that a type may safely be converted from an
  arbitrary byte sequence
- [`IntoBytes`](#intobytes) indicates that a type may safely be converted *to* a byte
  sequence

These traits support sized types, slices, and [slice DSTs][slice-dsts].

##### Marker Traits

Zerocopy provides three derivable marker traits that do not provide any
functionality themselves, but are required to call certain methods provided
by the conversion traits:
- [`KnownLayout`](#knownlayout) indicates that zerocopy can reason about certain layout
  qualities of a type
- [`Immutable`](#immutable) indicates that a type is free from interior mutability,
  except by ownership or an exclusive (`&mut`) borrow
- [`Unaligned`](#unaligned) indicates that a type's alignment requirement is 1

You should generally derive these marker traits whenever possible.

##### Conversion Macros

Zerocopy provides six macros for safe casting between types:

- ([`try_`][try_transmute](#try-transmute))[`transmute`](#transmute) (conditionally) converts a value of
  one type to a value of another type of the same size
- ([`try_`][try_transmute_mut](#try-transmute-mut))[`transmute_mut`](#transmute-mut) (conditionally) converts a
  mutable reference of one type to a mutable reference of another type of
  the same size
- ([`try_`][try_transmute_ref](#try-transmute-ref))[`transmute_ref`](#transmute-ref) (conditionally) converts a
  mutable or immutable reference of one type to an immutable reference of
  another type of the same size

These macros perform *compile-time* size and alignment checks, meaning that
unconditional casts have zero cost at runtime. Conditional casts do not need
to validate size or alignment runtime, but do need to validate contents.

These macros cannot be used in generic contexts. For generic conversions,
use the methods defined by the [conversion traits](#conversion-traits).

##### Byteorder-Aware Numerics

Zerocopy provides byte-order aware integer types that support these
conversions; see the [`byteorder`](byteorder/index.md) module. These types are especially useful
for network parsing.

# Cargo Features

- **`alloc`**
  By default, `zerocopy` is `no_std`. When the `alloc` feature is enabled,
  the `alloc` crate is added as a dependency, and some allocation-related
  functionality is added.

- **`std`**
  By default, `zerocopy` is `no_std`. When the `std` feature is enabled, the
  `std` crate is added as a dependency (ie, `no_std` is disabled), and
  support for some `std` types is added. `std` implies `alloc`.

- **`derive`**
  Provides derives for the core marker traits via the `zerocopy-derive`
  crate. These derives are re-exported from `zerocopy`, so it is not
  necessary to depend on `zerocopy-derive` directly.

  However, you may experience better compile times if you instead directly
  depend on both `zerocopy` and `zerocopy-derive` in your `Cargo.toml`,
  since doing so will allow Rust to compile these crates in parallel. To do
  so, do *not* enable the `derive` feature, and list both dependencies in
  your `Cargo.toml` with the same leading non-zero version number; e.g:

  ```toml
  [dependencies]
  zerocopy = "0.X"
  zerocopy-derive = "0.X"
  ```

  To avoid the risk of [duplicate import errors][duplicate-import-errors] if
  one of your dependencies enables zerocopy's `derive` feature, import
  derives as `use zerocopy_derive::*` rather than by name (e.g., `use
  zerocopy_derive::FromBytes`).

- **`simd`**
  When the `simd` feature is enabled, `FromZeros`, `FromBytes`, and
  `IntoBytes` impls are emitted for all stable SIMD types which exist on the
  target platform. Note that the layout of SIMD types is not yet stabilized,
  so these impls may be removed in the future if layout changes make them
  invalid. For more information, see the Unsafe Code Guidelines Reference
  page on the [layout of packed SIMD vectors][simd-layout].

- **`simd-nightly`**
  Enables the `simd` feature and adds support for SIMD types which are only
  available on nightly. Since these types are unstable, support for any type
  may be removed at any point in the future.

- **`float-nightly`**
  Adds support for the unstable `f16` and `f128` types. These types are
  not yet fully implemented and may not be supported on all platforms.


# Security Ethos

Zerocopy is expressly designed for use in security-critical contexts. We
strive to ensure that that zerocopy code is sound under Rust's current
memory model, and *any future memory model*. We ensure this by:
- **...not 'guessing' about Rust's semantics.**
  We annotate `unsafe` code with a precise rationale for its soundness that
  cites a relevant section of Rust's official documentation. When Rust's
  documented semantics are unclear, we work with the Rust Operational
  Semantics Team to clarify Rust's documentation.
- **...rigorously testing our implementation.**
  We run tests using [Miri], ensuring that zerocopy is sound across a wide
  array of supported target platforms of varying endianness and pointer
  width, and across both current and experimental memory models of Rust.
- **...formally proving the correctness of our implementation.**
  We apply formal verification tools like [Kani][kani] to prove zerocopy's
  correctness.

For more information, see our full [soundness policy].



# Relationship to Project Safe Transmute

[Project Safe Transmute] is an official initiative of the Rust Project to
develop language-level support for safer transmutation. The Project consults
with crates like zerocopy to identify aspects of safer transmutation that
would benefit from compiler support, and has developed an [experimental,
compiler-supported analysis][mcp-transmutability] which determines whether,
for a given type, any value of that type may be soundly transmuted into
another type. Once this functionality is sufficiently mature, zerocopy
intends to replace its internal transmutability analysis (implemented by our
custom derives) with the compiler-supported one. This change will likely be
an implementation detail that is invisible to zerocopy's users.

Project Safe Transmute will not replace the need for most of zerocopy's
higher-level abstractions. The experimental compiler analysis is a tool for
checking the soundness of `unsafe` code, not a tool to avoid writing
`unsafe` code altogether. For the foreseeable future, crates like zerocopy
will still be required in order to provide higher-level abstractions on top
of the building block provided by Project Safe Transmute.


# MSRV

See our [MSRV policy].

# Changelog

Zerocopy uses [GitHub Releases].

# Thanks

Zerocopy is maintained by engineers at Google with help from [many wonderful
contributors][contributors]. Thank you to everyone who has lent a hand in
making Rust a little more secure!


## Contents

- [Modules](#modules)
  - [`byte_slice`](#byte-slice)
  - [`byteorder`](#byteorder)
  - [`deprecated`](#deprecated)
  - [`error`](#error)
  - [`impls`](#impls)
  - [`macros`](#macros)
  - [`ref`](#ref)
  - [`split_at`](#split-at)
  - [`wrappers`](#wrappers)
  - [`alloc_support`](#alloc-support)
  - [`private`](#private)
  - [`f32_ext`](#f32-ext)
  - [`f64_ext`](#f64-ext)
  - [`big_endian`](#big-endian)
  - [`little_endian`](#little-endian)
  - [`network_endian`](#network-endian)
  - [`native_endian`](#native-endian)
  - [`def`](#def)
  - [`read_only_def`](#read-only-def)
- [Structs](#structs)
  - [`Split`](#split)
  - [`KnownLayout`](#knownlayout)
  - [`U16`](#u16)
  - [`U32`](#u32)
  - [`U64`](#u64)
  - [`U128`](#u128)
  - [`Usize`](#usize)
  - [`I16`](#i16)
  - [`I32`](#i32)
  - [`I64`](#i64)
  - [`I128`](#i128)
  - [`Isize`](#isize)
  - [`F32`](#f32)
  - [`F64`](#f64)
  - [`AlignmentError`](#alignmenterror)
  - [`SizeError`](#sizeerror)
  - [`ValidityError`](#validityerror)
  - [`AllocError`](#allocerror)
  - [`Unalign`](#unalign)
- [Enums](#enums)
  - [`BigEndian`](#bigendian)
  - [`LittleEndian`](#littleendian)
  - [`ConvertError`](#converterror)
- [Traits](#traits)
  - [`SplitAt`](#splitat)
  - [`KnownLayout`](#knownlayout)
  - [`Immutable`](#immutable)
  - [`TryFromBytes`](#tryfrombytes)
  - [`FromZeros`](#fromzeros)
  - [`FromBytes`](#frombytes)
  - [`IntoBytes`](#intobytes)
  - [`Unaligned`](#unaligned)
  - [`ByteSlice`](#byteslice)
  - [`ByteSliceMut`](#byteslicemut)
  - [`CopyableByteSlice`](#copyablebyteslice)
  - [`CloneableByteSlice`](#cloneablebyteslice)
  - [`SplitByteSlice`](#splitbyteslice)
  - [`SplitByteSliceMut`](#splitbyteslicemut)
  - [`IntoByteSlice`](#intobyteslice)
  - [`IntoByteSliceMut`](#intobyteslicemut)
  - [`ByteOrder`](#byteorder)
- [Functions](#functions)
  - [`trailing_slice_layout`](#trailing-slice-layout)
  - [`FromZeros`](#fromzeros)
  - [`try_ref_from_prefix_suffix`](#try-ref-from-prefix-suffix)
  - [`try_mut_from_prefix_suffix`](#try-mut-from-prefix-suffix)
  - [`swap`](#swap)
  - [`try_read_from`](#try-read-from)
  - [`ref_from_prefix_suffix`](#ref-from-prefix-suffix)
  - [`mut_from_prefix_suffix`](#mut-from-prefix-suffix)
  - [`IntoBytes`](#intobytes)
  - [`ByteHash`](#bytehash)
  - [`cast_for_sized`](#cast-for-sized)
- [Type Aliases](#type-aliases)
  - [`NativeEndian`](#nativeendian)
  - [`NetworkEndian`](#networkendian)
  - [`BE`](#be)
  - [`LE`](#le)
  - [`CastError`](#casterror)
  - [`TryCastError`](#trycasterror)
  - [`TryReadError`](#tryreaderror)
  - [`AlignedTryCastError`](#alignedtrycasterror)
- [Macros](#macros)
  - [`transmute!`](#transmute)
  - [`transmute_ref!`](#transmute-ref)
  - [`transmute_mut!`](#transmute-mut)
  - [`try_transmute!`](#try-transmute)
  - [`try_transmute_ref!`](#try-transmute-ref)
  - [`try_transmute_mut!`](#try-transmute-mut)
  - [`include_value!`](#include-value)
  - [`impl_fmt_trait!`](#impl-fmt-trait)
  - [`impl_fmt_traits!`](#impl-fmt-traits)
  - [`impl_ops_traits!`](#impl-ops-traits)
  - [`doc_comment!`](#doc-comment)
  - [`define_max_value_constant!`](#define-max-value-constant)
  - [`define_type!`](#define-type)
  - [`define_float_conversion!`](#define-float-conversion)
  - [`module!`](#module)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`byte_slice`](#byte-slice) | mod | Traits for types that encapsulate a `[u8]`. |
| [`byteorder`](#byteorder) | mod | Byte order-aware numeric primitives. |
| [`deprecated`](#deprecated) | mod | Deprecated items. |
| [`error`](#error) | mod | Types related to error reporting. |
| [`impls`](#impls) | mod |  |
| [`macros`](#macros) | mod |  |
| [`ref`](#ref) | mod |  |
| [`split_at`](#split-at) | mod |  |
| [`wrappers`](#wrappers) | mod |  |
| [`alloc_support`](#alloc-support) | mod |  |
| [`private`](#private) | mod |  |
| [`f32_ext`](#f32-ext) | mod |  |
| [`f64_ext`](#f64-ext) | mod |  |
| [`big_endian`](#big-endian) | mod | Numeric primitives stored in big-endian byte order. |
| [`little_endian`](#little-endian) | mod | Numeric primitives stored in little-endian byte order. |
| [`network_endian`](#network-endian) | mod | Numeric primitives stored in network-endian byte order. |
| [`native_endian`](#native-endian) | mod | Numeric primitives stored in native-endian byte order. |
| [`def`](#def) | mod |  |
| [`read_only_def`](#read-only-def) | mod |  |
| [`Split`](#split) | struct |  |
| [`KnownLayout`](#knownlayout) | struct | Implements [`KnownLayout`]. |
| [`U16`](#u16) | struct | A 16-bit unsigned integer stored in a given byte order. |
| [`U32`](#u32) | struct | A 32-bit unsigned integer stored in a given byte order. |
| [`U64`](#u64) | struct | A 64-bit unsigned integer stored in a given byte order. |
| [`U128`](#u128) | struct | A 128-bit unsigned integer stored in a given byte order. |
| [`Usize`](#usize) | struct | A word-sized unsigned integer stored in a given byte order. |
| [`I16`](#i16) | struct | A 16-bit signed integer stored in a given byte order. |
| [`I32`](#i32) | struct | A 32-bit signed integer stored in a given byte order. |
| [`I64`](#i64) | struct | A 64-bit signed integer stored in a given byte order. |
| [`I128`](#i128) | struct | A 128-bit signed integer stored in a given byte order. |
| [`Isize`](#isize) | struct | A word-sized signed integer stored in a given byte order. |
| [`F32`](#f32) | struct | A 32-bit floating point number stored in a given byte order. |
| [`F64`](#f64) | struct | A 64-bit floating point number stored in a given byte order. |
| [`AlignmentError`](#alignmenterror) | struct | The error emitted if the conversion source is improperly aligned. |
| [`SizeError`](#sizeerror) | struct | The error emitted if the conversion source is of incorrect size. |
| [`ValidityError`](#validityerror) | struct | The error emitted if the conversion source contains invalid data. |
| [`AllocError`](#allocerror) | struct | The error type of a failed allocation. |
| [`Unalign`](#unalign) | struct | A type with no alignment requirement. |
| [`BigEndian`](#bigendian) | enum | Big-endian byte order. |
| [`LittleEndian`](#littleendian) | enum | Little-endian byte order. |
| [`ConvertError`](#converterror) | enum | Zerocopy's generic error type. |
| [`SplitAt`](#splitat) | trait |  |
| [`KnownLayout`](#knownlayout) | trait | Indicates that zerocopy can reason about certain aspects of a type's layout. |
| [`Immutable`](#immutable) | trait | Types which are free from interior mutability. |
| [`TryFromBytes`](#tryfrombytes) | trait | Types for which some bit patterns are valid. |
| [`FromZeros`](#fromzeros) | trait | Types for which a sequence of `0` bytes is a valid instance. |
| [`FromBytes`](#frombytes) | trait | Types for which any bit pattern is valid. |
| [`IntoBytes`](#intobytes) | trait | Types that can be converted to an immutable slice of initialized bytes. |
| [`Unaligned`](#unaligned) | trait | Types with no alignment requirement. |
| [`ByteSlice`](#byteslice) | trait | A mutable or immutable reference to a byte slice. |
| [`ByteSliceMut`](#byteslicemut) | trait | A mutable reference to a byte slice. |
| [`CopyableByteSlice`](#copyablebyteslice) | trait | A [`ByteSlice`] which can be copied without violating dereference stability. |
| [`CloneableByteSlice`](#cloneablebyteslice) | trait | A [`ByteSlice`] which can be cloned without violating dereference stability. |
| [`SplitByteSlice`](#splitbyteslice) | trait | A [`ByteSlice`] that can be split in two. |
| [`SplitByteSliceMut`](#splitbyteslicemut) | trait | A shorthand for [`SplitByteSlice`] and [`ByteSliceMut`]. |
| [`IntoByteSlice`](#intobyteslice) | trait | A [`ByteSlice`] that conveys no ownership, and so can be converted into a byte slice. |
| [`IntoByteSliceMut`](#intobyteslicemut) | trait | A [`ByteSliceMut`] that conveys no ownership, and so can be converted into a mutable byte slice. |
| [`ByteOrder`](#byteorder) | trait | A type-level representation of byte order. |
| [`trailing_slice_layout`](#trailing-slice-layout) | fn | Efficiently produces the [`TrailingSliceLayout`] of `T`. |
| [`FromZeros`](#fromzeros) | fn | Analyzes whether a type is [`FromZeros`]. |
| [`try_ref_from_prefix_suffix`](#try-ref-from-prefix-suffix) | fn |  |
| [`try_mut_from_prefix_suffix`](#try-mut-from-prefix-suffix) | fn |  |
| [`swap`](#swap) | fn |  |
| [`try_read_from`](#try-read-from) | fn | # Safety |
| [`ref_from_prefix_suffix`](#ref-from-prefix-suffix) | fn | Interprets the given affix of the given bytes as a `&Self`. |
| [`mut_from_prefix_suffix`](#mut-from-prefix-suffix) | fn | Interprets the given affix of the given bytes as a `&mut Self` without copying. |
| [`IntoBytes`](#intobytes) | fn | Analyzes whether a type is [`IntoBytes`]. |
| [`ByteHash`](#bytehash) | fn | Derives an optimized [`Hash`] implementation. |
| [`cast_for_sized`](#cast-for-sized) | fn | # Safety |
| [`NativeEndian`](#nativeendian) | type | The endianness used by this platform. |
| [`NetworkEndian`](#networkendian) | type | The endianness used in many network protocols. |
| [`BE`](#be) | type | A type alias for [`BigEndian`]. |
| [`LE`](#le) | type | A type alias for [`LittleEndian`]. |
| [`CastError`](#casterror) | type | The error type of reference conversions. |
| [`TryCastError`](#trycasterror) | type | The error type of fallible reference conversions. |
| [`TryReadError`](#tryreaderror) | type | The error type of fallible read-conversions. |
| [`AlignedTryCastError`](#alignedtrycasterror) | type | The error type of well-aligned, fallible casts. |
| [`transmute!`](#transmute) | macro | Safely transmutes a value of one type to a value of another type of the same size. |
| [`transmute_ref!`](#transmute-ref) | macro | Safely transmutes a mutable or immutable reference of one type to an immutable reference of another type of the same size and compatible alignment. |
| [`transmute_mut!`](#transmute-mut) | macro | Safely transmutes a mutable reference of one type to a mutable reference of another type of the same size and compatible alignment. |
| [`try_transmute!`](#try-transmute) | macro | Conditionally transmutes a value of one type to a value of another type of the same size. |
| [`try_transmute_ref!`](#try-transmute-ref) | macro | Conditionally transmutes a mutable or immutable reference of one type to an immutable reference of another type of the same size and compatible alignment. |
| [`try_transmute_mut!`](#try-transmute-mut) | macro | Conditionally transmutes a mutable reference of one type to a mutable reference of another type of the same size and compatible alignment. |
| [`include_value!`](#include-value) | macro | Includes a file and safely transmutes it to a value of an arbitrary type. |
| [`impl_fmt_trait!`](#impl-fmt-trait) | macro |  |
| [`impl_fmt_traits!`](#impl-fmt-traits) | macro |  |
| [`impl_ops_traits!`](#impl-ops-traits) | macro |  |
| [`doc_comment!`](#doc-comment) | macro |  |
| [`define_max_value_constant!`](#define-max-value-constant) | macro |  |
| [`define_type!`](#define-type) | macro |  |
| [`define_float_conversion!`](#define-float-conversion) | macro |  |
| [`module!`](#module) | macro |  |

## Modules

- [`byte_slice`](byte_slice/index.md) — Traits for types that encapsulate a `[u8]`.
- [`byteorder`](byteorder/index.md) — Byte order-aware numeric primitives.
- [`deprecated`](deprecated/index.md) — Deprecated items. These are kept separate so that they don't clutter up
- [`error`](error/index.md) — Types related to error reporting.
- [`impls`](impls/index.md)
- [`macros`](macros/index.md)
- [`ref`](ref/index.md)
- [`split_at`](split_at/index.md)
- [`wrappers`](wrappers/index.md)
- [`alloc_support`](alloc_support/index.md)
- [`private`](private/index.md)
- [`f32_ext`](f32_ext/index.md)
- [`f64_ext`](f64_ext/index.md)
- [`big_endian`](big_endian/index.md) — Numeric primitives stored in
- [`little_endian`](little_endian/index.md) — Numeric primitives stored in
- [`network_endian`](network_endian/index.md) — Numeric primitives stored in
- [`native_endian`](native_endian/index.md) — Numeric primitives stored in
- [`def`](def/index.md)
- [`read_only_def`](read_only_def/index.md)

## Structs

### `Split<T>`

```rust
struct Split<T> {
    source: T,
    l_len: usize,
}
```

A `T` that has been split into two possibly-overlapping parts.

For some dynamically sized types, the padding that appears after the
trailing slice field [is a dynamic function of the trailing slice
length](KnownLayout#slice-dst-layout). If `T` is split at a length that
requires trailing padding, the trailing padding of the left part of the
split `T` will overlap the right part. If `T` is a mutable reference or
permits interior mutation, you must ensure that the left and right parts do
not overlap. You can do this at zero-cost using using
`Self::via_immutable`, `Self::via_into_bytes`, or
`Self::via_unaligned`, or with a dynamic check by using
`Self::via_runtime_check`.

#### Fields

- **`source`**: `T`

  A pointer to the source slice DST.

- **`l_len`**: `usize`

  The length of the future left half of `source`.
  
  # Safety
  
  If `source` is a pointer to a slice DST, `l_len` is no greater than
  `source`'s length.

#### Implementations

- <span id="split-new"></span>`unsafe fn new(source: T, l_len: usize) -> Self`

  Produces a `Split` of `source` with `l_len`.

  

  # Safety

  

  `l_len` is no greater than `source`'s length.

#### Trait Implementations

##### `impl<T: fmt::Debug> Debug for Split<T>`

- <span id="split-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `KnownLayout<'a, K, V>`

```rust
struct KnownLayout<'a, K, V> {
    v: &'a mut FlatMap<K, V>,
    index: usize,
}
```

*Re-exported from `clap_builder`*

### `U16<O>`

```rust
struct U16<O>([u8; 2], PhantomData<O>);
```

A 16-bit unsigned integer stored in a given byte order.

`U16` is like the native `u16` type with
two major differences: First, it has no alignment requirement (its alignment is 1).
Second, the endianness of its memory layout is given by the type parameter `O`,
which can be any type which implements [`ByteOrder`](#byteorder). In particular, this refers
to [`BigEndian`](#bigendian), [`LittleEndian`](#littleendian), [`NativeEndian`](#nativeendian), and [`NetworkEndian`](#networkendian).

A `U16` can be constructed using
the `new` method, and its contained value can be obtained as a native
`u16` using the `get` method, or updated in place with
the `set` method. In all cases, if the endianness `O` is not the same as the
endianness of the current platform, an endianness swap will be performed in
order to uphold the invariants that a) the layout of `U16`
has endianness `O` and that, b) the layout of `u16` has
the platform's native endianness.

`U16` implements [`FromBytes`](#frombytes), [`IntoBytes`](#intobytes), and [`Unaligned`](#unaligned),
making it useful for parsing and serialization. See the module documentation for an
example of how it can be used for parsing UDP packets.







#### Implementations

- <span id="u16-const-zero"></span>`const ZERO: U16<O>`

- <span id="u16-const-max-value"></span>`const MAX_VALUE: U16<O>`

- <span id="u16-from-bytes"></span>`const fn from_bytes(bytes: [u8; 2]) -> U16<O>` — [`U16`](#u16)

  Constructs a new value from bytes which are already in `O` byte

  order.

- <span id="u16-to-bytes"></span>`const fn to_bytes(self) -> [u8; 2]`

  Extracts the bytes of `self` without swapping the byte order.

  

  The returned bytes will be in `O` byte order.

#### Trait Implementations

##### `impl<O: ByteOrder> Add for U16<O>`

- <span id="u16-add-type-output"></span>`type Output = U16<O>`

- <span id="u16-add"></span>`fn add(self, rhs: U16<O>) -> U16<O>` — [`U16`](#u16)

##### `impl<O: ByteOrder> AddAssign for U16<O>`

- <span id="u16-addassign-add-assign"></span>`fn add_assign(&mut self, rhs: U16<O>)` — [`U16`](#u16)

##### `impl<O> AsMut for U16<O>`

- <span id="u16-asmut-as-mut"></span>`fn as_mut(&mut self) -> &mut [u8; 2]`

##### `impl<O> AsRef for U16<O>`

- <span id="u16-asref-as-ref"></span>`fn as_ref(&self) -> &[u8; 2]`

##### `impl<O: ByteOrder> Binary for U16<O>`

- <span id="u16-binary-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> BitAnd for U16<O>`

- <span id="u16-bitand-type-output"></span>`type Output = U16<O>`

- <span id="u16-bitand"></span>`fn bitand(self, rhs: U16<O>) -> U16<O>` — [`U16`](#u16)

##### `impl<O: ByteOrder> BitAndAssign for U16<O>`

- <span id="u16-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, rhs: U16<O>)` — [`U16`](#u16)

##### `impl<O: ByteOrder> BitOr for U16<O>`

- <span id="u16-bitor-type-output"></span>`type Output = U16<O>`

- <span id="u16-bitor"></span>`fn bitor(self, rhs: U16<O>) -> U16<O>` — [`U16`](#u16)

##### `impl<O: ByteOrder> BitOrAssign for U16<O>`

- <span id="u16-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, rhs: U16<O>)` — [`U16`](#u16)

##### `impl<O: ByteOrder> BitXor for U16<O>`

- <span id="u16-bitxor-type-output"></span>`type Output = U16<O>`

- <span id="u16-bitxor"></span>`fn bitxor(self, rhs: U16<O>) -> U16<O>` — [`U16`](#u16)

##### `impl<O: ByteOrder> BitXorAssign for U16<O>`

- <span id="u16-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, rhs: U16<O>)` — [`U16`](#u16)

##### `impl<O: clone::Clone> Clone for U16<O>`

- <span id="u16-clone"></span>`fn clone(&self) -> U16<O>` — [`U16`](#u16)

##### `impl<O: marker::Copy> Copy for U16<O>`

##### `impl<O: ByteOrder> Debug for U16<O>`

- <span id="u16-debug-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O> Default for U16<O>`

- <span id="u16-default"></span>`fn default() -> U16<O>` — [`U16`](#u16)

##### `impl<O: ByteOrder> Display for U16<O>`

- <span id="u16-display-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Div for U16<O>`

- <span id="u16-div-type-output"></span>`type Output = U16<O>`

- <span id="u16-div"></span>`fn div(self, rhs: U16<O>) -> U16<O>` — [`U16`](#u16)

##### `impl<O: ByteOrder> DivAssign for U16<O>`

- <span id="u16-divassign-div-assign"></span>`fn div_assign(&mut self, rhs: U16<O>)` — [`U16`](#u16)

##### `impl<O: cmp::Eq> Eq for U16<O>`

##### `impl<O> FromBytes for U16<O>`

##### `impl<O> FromZeros for U16<O>`

##### `impl<O: hash::Hash> Hash for U16<O>`

- <span id="u16-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<O> Immutable for U16<O>`

##### `impl<O> IntoBytes for U16<O>`

##### `impl<O> KnownLayout for U16<O>`

- <span id="u16-knownlayout-type-pointermetadata"></span>`type PointerMetadata = ()`

##### `impl<O: ByteOrder> LowerHex for U16<O>`

- <span id="u16-lowerhex-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Mul for U16<O>`

- <span id="u16-mul-type-output"></span>`type Output = U16<O>`

- <span id="u16-mul"></span>`fn mul(self, rhs: U16<O>) -> U16<O>` — [`U16`](#u16)

##### `impl<O: ByteOrder> MulAssign for U16<O>`

- <span id="u16-mulassign-mul-assign"></span>`fn mul_assign(&mut self, rhs: U16<O>)` — [`U16`](#u16)

##### `impl<O> Not for U16<O>`

- <span id="u16-not-type-output"></span>`type Output = U16<O>`

- <span id="u16-not"></span>`fn not(self) -> U16<O>` — [`U16`](#u16)

##### `impl<O: ByteOrder> Octal for U16<O>`

- <span id="u16-octal-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Ord for U16<O>`

- <span id="u16-ord-cmp"></span>`fn cmp(&self, other: &Self) -> Ordering`

##### `impl<O: cmp::PartialEq> PartialEq for U16<O>`

- <span id="u16-partialeq-eq"></span>`fn eq(&self, other: &U16<O>) -> bool` — [`U16`](#u16)

##### `impl<O: ByteOrder> PartialOrd for U16<O>`

- <span id="u16-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

##### `impl<O: ByteOrder> Rem for U16<O>`

- <span id="u16-rem-type-output"></span>`type Output = U16<O>`

- <span id="u16-rem"></span>`fn rem(self, rhs: U16<O>) -> U16<O>` — [`U16`](#u16)

##### `impl<O: ByteOrder> RemAssign for U16<O>`

- <span id="u16-remassign-rem-assign"></span>`fn rem_assign(&mut self, rhs: U16<O>)` — [`U16`](#u16)

##### `impl<O: ByteOrder> Shl for U16<O>`

- <span id="u16-shl-type-output"></span>`type Output = U16<O>`

- <span id="u16-shl"></span>`fn shl(self, rhs: U16<O>) -> U16<O>` — [`U16`](#u16)

##### `impl<O: ByteOrder> ShlAssign for U16<O>`

- <span id="u16-shlassign-shl-assign"></span>`fn shl_assign(&mut self, rhs: U16<O>)` — [`U16`](#u16)

##### `impl<O: ByteOrder> Shr for U16<O>`

- <span id="u16-shr-type-output"></span>`type Output = U16<O>`

- <span id="u16-shr"></span>`fn shr(self, rhs: U16<O>) -> U16<O>` — [`U16`](#u16)

##### `impl<O: ByteOrder> ShrAssign for U16<O>`

- <span id="u16-shrassign-shr-assign"></span>`fn shr_assign(&mut self, rhs: U16<O>)` — [`U16`](#u16)

##### `impl<O> StructuralPartialEq for U16<O>`

##### `impl<O: ByteOrder> Sub for U16<O>`

- <span id="u16-sub-type-output"></span>`type Output = U16<O>`

- <span id="u16-sub"></span>`fn sub(self, rhs: U16<O>) -> U16<O>` — [`U16`](#u16)

##### `impl<O: ByteOrder> SubAssign for U16<O>`

- <span id="u16-subassign-sub-assign"></span>`fn sub_assign(&mut self, rhs: U16<O>)` — [`U16`](#u16)

##### `impl ToString for U16<O>`

- <span id="u16-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<O> TryFromBytes for U16<O>`

##### `impl<O> Unaligned for U16<O>`

##### `impl<O: ByteOrder> UpperHex for U16<O>`

- <span id="u16-upperhex-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

### `U32<O>`

```rust
struct U32<O>([u8; 4], PhantomData<O>);
```

A 32-bit unsigned integer stored in a given byte order.

`U32` is like the native `u32` type with
two major differences: First, it has no alignment requirement (its alignment is 1).
Second, the endianness of its memory layout is given by the type parameter `O`,
which can be any type which implements [`ByteOrder`](#byteorder). In particular, this refers
to [`BigEndian`](#bigendian), [`LittleEndian`](#littleendian), [`NativeEndian`](#nativeendian), and [`NetworkEndian`](#networkendian).

A `U32` can be constructed using
the `new` method, and its contained value can be obtained as a native
`u32` using the `get` method, or updated in place with
the `set` method. In all cases, if the endianness `O` is not the same as the
endianness of the current platform, an endianness swap will be performed in
order to uphold the invariants that a) the layout of `U32`
has endianness `O` and that, b) the layout of `u32` has
the platform's native endianness.

`U32` implements [`FromBytes`](#frombytes), [`IntoBytes`](#intobytes), and [`Unaligned`](#unaligned),
making it useful for parsing and serialization. See the module documentation for an
example of how it can be used for parsing UDP packets.







#### Implementations

- <span id="u32-const-zero"></span>`const ZERO: U32<O>`

- <span id="u32-const-max-value"></span>`const MAX_VALUE: U32<O>`

- <span id="u32-from-bytes"></span>`const fn from_bytes(bytes: [u8; 4]) -> U32<O>` — [`U32`](#u32)

  Constructs a new value from bytes which are already in `O` byte

  order.

- <span id="u32-to-bytes"></span>`const fn to_bytes(self) -> [u8; 4]`

  Extracts the bytes of `self` without swapping the byte order.

  

  The returned bytes will be in `O` byte order.

#### Trait Implementations

##### `impl<O: ByteOrder> Add for U32<O>`

- <span id="u32-add-type-output"></span>`type Output = U32<O>`

- <span id="u32-add"></span>`fn add(self, rhs: U32<O>) -> U32<O>` — [`U32`](#u32)

##### `impl<O: ByteOrder> AddAssign for U32<O>`

- <span id="u32-addassign-add-assign"></span>`fn add_assign(&mut self, rhs: U32<O>)` — [`U32`](#u32)

##### `impl<O> AsMut for U32<O>`

- <span id="u32-asmut-as-mut"></span>`fn as_mut(&mut self) -> &mut [u8; 4]`

##### `impl<O> AsRef for U32<O>`

- <span id="u32-asref-as-ref"></span>`fn as_ref(&self) -> &[u8; 4]`

##### `impl<O: ByteOrder> Binary for U32<O>`

- <span id="u32-binary-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> BitAnd for U32<O>`

- <span id="u32-bitand-type-output"></span>`type Output = U32<O>`

- <span id="u32-bitand"></span>`fn bitand(self, rhs: U32<O>) -> U32<O>` — [`U32`](#u32)

##### `impl<O: ByteOrder> BitAndAssign for U32<O>`

- <span id="u32-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, rhs: U32<O>)` — [`U32`](#u32)

##### `impl<O: ByteOrder> BitOr for U32<O>`

- <span id="u32-bitor-type-output"></span>`type Output = U32<O>`

- <span id="u32-bitor"></span>`fn bitor(self, rhs: U32<O>) -> U32<O>` — [`U32`](#u32)

##### `impl<O: ByteOrder> BitOrAssign for U32<O>`

- <span id="u32-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, rhs: U32<O>)` — [`U32`](#u32)

##### `impl<O: ByteOrder> BitXor for U32<O>`

- <span id="u32-bitxor-type-output"></span>`type Output = U32<O>`

- <span id="u32-bitxor"></span>`fn bitxor(self, rhs: U32<O>) -> U32<O>` — [`U32`](#u32)

##### `impl<O: ByteOrder> BitXorAssign for U32<O>`

- <span id="u32-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, rhs: U32<O>)` — [`U32`](#u32)

##### `impl<O: clone::Clone> Clone for U32<O>`

- <span id="u32-clone"></span>`fn clone(&self) -> U32<O>` — [`U32`](#u32)

##### `impl<O: marker::Copy> Copy for U32<O>`

##### `impl<O: ByteOrder> Debug for U32<O>`

- <span id="u32-debug-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O> Default for U32<O>`

- <span id="u32-default"></span>`fn default() -> U32<O>` — [`U32`](#u32)

##### `impl<O: ByteOrder> Display for U32<O>`

- <span id="u32-display-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Div for U32<O>`

- <span id="u32-div-type-output"></span>`type Output = U32<O>`

- <span id="u32-div"></span>`fn div(self, rhs: U32<O>) -> U32<O>` — [`U32`](#u32)

##### `impl<O: ByteOrder> DivAssign for U32<O>`

- <span id="u32-divassign-div-assign"></span>`fn div_assign(&mut self, rhs: U32<O>)` — [`U32`](#u32)

##### `impl<O: cmp::Eq> Eq for U32<O>`

##### `impl<O> FromBytes for U32<O>`

##### `impl<O> FromZeros for U32<O>`

##### `impl<O: hash::Hash> Hash for U32<O>`

- <span id="u32-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<O> Immutable for U32<O>`

##### `impl<O> IntoBytes for U32<O>`

##### `impl<O> KnownLayout for U32<O>`

- <span id="u32-knownlayout-type-pointermetadata"></span>`type PointerMetadata = ()`

##### `impl<O: ByteOrder> LowerHex for U32<O>`

- <span id="u32-lowerhex-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Mul for U32<O>`

- <span id="u32-mul-type-output"></span>`type Output = U32<O>`

- <span id="u32-mul"></span>`fn mul(self, rhs: U32<O>) -> U32<O>` — [`U32`](#u32)

##### `impl<O: ByteOrder> MulAssign for U32<O>`

- <span id="u32-mulassign-mul-assign"></span>`fn mul_assign(&mut self, rhs: U32<O>)` — [`U32`](#u32)

##### `impl<O> Not for U32<O>`

- <span id="u32-not-type-output"></span>`type Output = U32<O>`

- <span id="u32-not"></span>`fn not(self) -> U32<O>` — [`U32`](#u32)

##### `impl<O: ByteOrder> Octal for U32<O>`

- <span id="u32-octal-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Ord for U32<O>`

- <span id="u32-ord-cmp"></span>`fn cmp(&self, other: &Self) -> Ordering`

##### `impl<O: cmp::PartialEq> PartialEq for U32<O>`

- <span id="u32-partialeq-eq"></span>`fn eq(&self, other: &U32<O>) -> bool` — [`U32`](#u32)

##### `impl<O: ByteOrder> PartialOrd for U32<O>`

- <span id="u32-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

##### `impl<O: ByteOrder> Rem for U32<O>`

- <span id="u32-rem-type-output"></span>`type Output = U32<O>`

- <span id="u32-rem"></span>`fn rem(self, rhs: U32<O>) -> U32<O>` — [`U32`](#u32)

##### `impl<O: ByteOrder> RemAssign for U32<O>`

- <span id="u32-remassign-rem-assign"></span>`fn rem_assign(&mut self, rhs: U32<O>)` — [`U32`](#u32)

##### `impl<O: ByteOrder> Shl for U32<O>`

- <span id="u32-shl-type-output"></span>`type Output = U32<O>`

- <span id="u32-shl"></span>`fn shl(self, rhs: U32<O>) -> U32<O>` — [`U32`](#u32)

##### `impl<O: ByteOrder> ShlAssign for U32<O>`

- <span id="u32-shlassign-shl-assign"></span>`fn shl_assign(&mut self, rhs: U32<O>)` — [`U32`](#u32)

##### `impl<O: ByteOrder> Shr for U32<O>`

- <span id="u32-shr-type-output"></span>`type Output = U32<O>`

- <span id="u32-shr"></span>`fn shr(self, rhs: U32<O>) -> U32<O>` — [`U32`](#u32)

##### `impl<O: ByteOrder> ShrAssign for U32<O>`

- <span id="u32-shrassign-shr-assign"></span>`fn shr_assign(&mut self, rhs: U32<O>)` — [`U32`](#u32)

##### `impl<O> StructuralPartialEq for U32<O>`

##### `impl<O: ByteOrder> Sub for U32<O>`

- <span id="u32-sub-type-output"></span>`type Output = U32<O>`

- <span id="u32-sub"></span>`fn sub(self, rhs: U32<O>) -> U32<O>` — [`U32`](#u32)

##### `impl<O: ByteOrder> SubAssign for U32<O>`

- <span id="u32-subassign-sub-assign"></span>`fn sub_assign(&mut self, rhs: U32<O>)` — [`U32`](#u32)

##### `impl ToString for U32<O>`

- <span id="u32-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<O> TryFromBytes for U32<O>`

##### `impl<O> Unaligned for U32<O>`

##### `impl<O: ByteOrder> UpperHex for U32<O>`

- <span id="u32-upperhex-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

### `U64<O>`

```rust
struct U64<O>([u8; 8], PhantomData<O>);
```

A 64-bit unsigned integer stored in a given byte order.

`U64` is like the native `u64` type with
two major differences: First, it has no alignment requirement (its alignment is 1).
Second, the endianness of its memory layout is given by the type parameter `O`,
which can be any type which implements [`ByteOrder`](#byteorder). In particular, this refers
to [`BigEndian`](#bigendian), [`LittleEndian`](#littleendian), [`NativeEndian`](#nativeendian), and [`NetworkEndian`](#networkendian).

A `U64` can be constructed using
the `new` method, and its contained value can be obtained as a native
`u64` using the `get` method, or updated in place with
the `set` method. In all cases, if the endianness `O` is not the same as the
endianness of the current platform, an endianness swap will be performed in
order to uphold the invariants that a) the layout of `U64`
has endianness `O` and that, b) the layout of `u64` has
the platform's native endianness.

`U64` implements [`FromBytes`](#frombytes), [`IntoBytes`](#intobytes), and [`Unaligned`](#unaligned),
making it useful for parsing and serialization. See the module documentation for an
example of how it can be used for parsing UDP packets.







#### Implementations

- <span id="u64-const-zero"></span>`const ZERO: U64<O>`

- <span id="u64-const-max-value"></span>`const MAX_VALUE: U64<O>`

- <span id="u64-from-bytes"></span>`const fn from_bytes(bytes: [u8; 8]) -> U64<O>` — [`U64`](#u64)

  Constructs a new value from bytes which are already in `O` byte

  order.

- <span id="u64-to-bytes"></span>`const fn to_bytes(self) -> [u8; 8]`

  Extracts the bytes of `self` without swapping the byte order.

  

  The returned bytes will be in `O` byte order.

#### Trait Implementations

##### `impl<O: ByteOrder> Add for U64<O>`

- <span id="u64-add-type-output"></span>`type Output = U64<O>`

- <span id="u64-add"></span>`fn add(self, rhs: U64<O>) -> U64<O>` — [`U64`](#u64)

##### `impl<O: ByteOrder> AddAssign for U64<O>`

- <span id="u64-addassign-add-assign"></span>`fn add_assign(&mut self, rhs: U64<O>)` — [`U64`](#u64)

##### `impl<O> AsMut for U64<O>`

- <span id="u64-asmut-as-mut"></span>`fn as_mut(&mut self) -> &mut [u8; 8]`

##### `impl<O> AsRef for U64<O>`

- <span id="u64-asref-as-ref"></span>`fn as_ref(&self) -> &[u8; 8]`

##### `impl<O: ByteOrder> Binary for U64<O>`

- <span id="u64-binary-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> BitAnd for U64<O>`

- <span id="u64-bitand-type-output"></span>`type Output = U64<O>`

- <span id="u64-bitand"></span>`fn bitand(self, rhs: U64<O>) -> U64<O>` — [`U64`](#u64)

##### `impl<O: ByteOrder> BitAndAssign for U64<O>`

- <span id="u64-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, rhs: U64<O>)` — [`U64`](#u64)

##### `impl<O: ByteOrder> BitOr for U64<O>`

- <span id="u64-bitor-type-output"></span>`type Output = U64<O>`

- <span id="u64-bitor"></span>`fn bitor(self, rhs: U64<O>) -> U64<O>` — [`U64`](#u64)

##### `impl<O: ByteOrder> BitOrAssign for U64<O>`

- <span id="u64-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, rhs: U64<O>)` — [`U64`](#u64)

##### `impl<O: ByteOrder> BitXor for U64<O>`

- <span id="u64-bitxor-type-output"></span>`type Output = U64<O>`

- <span id="u64-bitxor"></span>`fn bitxor(self, rhs: U64<O>) -> U64<O>` — [`U64`](#u64)

##### `impl<O: ByteOrder> BitXorAssign for U64<O>`

- <span id="u64-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, rhs: U64<O>)` — [`U64`](#u64)

##### `impl<O: clone::Clone> Clone for U64<O>`

- <span id="u64-clone"></span>`fn clone(&self) -> U64<O>` — [`U64`](#u64)

##### `impl<O: marker::Copy> Copy for U64<O>`

##### `impl<O: ByteOrder> Debug for U64<O>`

- <span id="u64-debug-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O> Default for U64<O>`

- <span id="u64-default"></span>`fn default() -> U64<O>` — [`U64`](#u64)

##### `impl<O: ByteOrder> Display for U64<O>`

- <span id="u64-display-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Div for U64<O>`

- <span id="u64-div-type-output"></span>`type Output = U64<O>`

- <span id="u64-div"></span>`fn div(self, rhs: U64<O>) -> U64<O>` — [`U64`](#u64)

##### `impl<O: ByteOrder> DivAssign for U64<O>`

- <span id="u64-divassign-div-assign"></span>`fn div_assign(&mut self, rhs: U64<O>)` — [`U64`](#u64)

##### `impl<O: cmp::Eq> Eq for U64<O>`

##### `impl<O> FromBytes for U64<O>`

##### `impl<O> FromZeros for U64<O>`

##### `impl<O: hash::Hash> Hash for U64<O>`

- <span id="u64-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<O> Immutable for U64<O>`

##### `impl<O> IntoBytes for U64<O>`

##### `impl<O> KnownLayout for U64<O>`

- <span id="u64-knownlayout-type-pointermetadata"></span>`type PointerMetadata = ()`

##### `impl<O: ByteOrder> LowerHex for U64<O>`

- <span id="u64-lowerhex-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Mul for U64<O>`

- <span id="u64-mul-type-output"></span>`type Output = U64<O>`

- <span id="u64-mul"></span>`fn mul(self, rhs: U64<O>) -> U64<O>` — [`U64`](#u64)

##### `impl<O: ByteOrder> MulAssign for U64<O>`

- <span id="u64-mulassign-mul-assign"></span>`fn mul_assign(&mut self, rhs: U64<O>)` — [`U64`](#u64)

##### `impl<O> Not for U64<O>`

- <span id="u64-not-type-output"></span>`type Output = U64<O>`

- <span id="u64-not"></span>`fn not(self) -> U64<O>` — [`U64`](#u64)

##### `impl<O: ByteOrder> Octal for U64<O>`

- <span id="u64-octal-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Ord for U64<O>`

- <span id="u64-ord-cmp"></span>`fn cmp(&self, other: &Self) -> Ordering`

##### `impl<O: cmp::PartialEq> PartialEq for U64<O>`

- <span id="u64-partialeq-eq"></span>`fn eq(&self, other: &U64<O>) -> bool` — [`U64`](#u64)

##### `impl<O: ByteOrder> PartialOrd for U64<O>`

- <span id="u64-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

##### `impl<O: ByteOrder> Rem for U64<O>`

- <span id="u64-rem-type-output"></span>`type Output = U64<O>`

- <span id="u64-rem"></span>`fn rem(self, rhs: U64<O>) -> U64<O>` — [`U64`](#u64)

##### `impl<O: ByteOrder> RemAssign for U64<O>`

- <span id="u64-remassign-rem-assign"></span>`fn rem_assign(&mut self, rhs: U64<O>)` — [`U64`](#u64)

##### `impl<O: ByteOrder> Shl for U64<O>`

- <span id="u64-shl-type-output"></span>`type Output = U64<O>`

- <span id="u64-shl"></span>`fn shl(self, rhs: U64<O>) -> U64<O>` — [`U64`](#u64)

##### `impl<O: ByteOrder> ShlAssign for U64<O>`

- <span id="u64-shlassign-shl-assign"></span>`fn shl_assign(&mut self, rhs: U64<O>)` — [`U64`](#u64)

##### `impl<O: ByteOrder> Shr for U64<O>`

- <span id="u64-shr-type-output"></span>`type Output = U64<O>`

- <span id="u64-shr"></span>`fn shr(self, rhs: U64<O>) -> U64<O>` — [`U64`](#u64)

##### `impl<O: ByteOrder> ShrAssign for U64<O>`

- <span id="u64-shrassign-shr-assign"></span>`fn shr_assign(&mut self, rhs: U64<O>)` — [`U64`](#u64)

##### `impl<O> StructuralPartialEq for U64<O>`

##### `impl<O: ByteOrder> Sub for U64<O>`

- <span id="u64-sub-type-output"></span>`type Output = U64<O>`

- <span id="u64-sub"></span>`fn sub(self, rhs: U64<O>) -> U64<O>` — [`U64`](#u64)

##### `impl<O: ByteOrder> SubAssign for U64<O>`

- <span id="u64-subassign-sub-assign"></span>`fn sub_assign(&mut self, rhs: U64<O>)` — [`U64`](#u64)

##### `impl ToString for U64<O>`

- <span id="u64-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<O> TryFromBytes for U64<O>`

##### `impl<O> Unaligned for U64<O>`

##### `impl<O: ByteOrder> UpperHex for U64<O>`

- <span id="u64-upperhex-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

### `U128<O>`

```rust
struct U128<O>([u8; 16], PhantomData<O>);
```

A 128-bit unsigned integer stored in a given byte order.

`U128` is like the native `u128` type with
two major differences: First, it has no alignment requirement (its alignment is 1).
Second, the endianness of its memory layout is given by the type parameter `O`,
which can be any type which implements [`ByteOrder`](#byteorder). In particular, this refers
to [`BigEndian`](#bigendian), [`LittleEndian`](#littleendian), [`NativeEndian`](#nativeendian), and [`NetworkEndian`](#networkendian).

A `U128` can be constructed using
the `new` method, and its contained value can be obtained as a native
`u128` using the `get` method, or updated in place with
the `set` method. In all cases, if the endianness `O` is not the same as the
endianness of the current platform, an endianness swap will be performed in
order to uphold the invariants that a) the layout of `U128`
has endianness `O` and that, b) the layout of `u128` has
the platform's native endianness.

`U128` implements [`FromBytes`](#frombytes), [`IntoBytes`](#intobytes), and [`Unaligned`](#unaligned),
making it useful for parsing and serialization. See the module documentation for an
example of how it can be used for parsing UDP packets.







#### Implementations

- <span id="u128-const-zero"></span>`const ZERO: U128<O>`

- <span id="u128-const-max-value"></span>`const MAX_VALUE: U128<O>`

- <span id="u128-from-bytes"></span>`const fn from_bytes(bytes: [u8; 16]) -> U128<O>` — [`U128`](#u128)

  Constructs a new value from bytes which are already in `O` byte

  order.

- <span id="u128-to-bytes"></span>`const fn to_bytes(self) -> [u8; 16]`

  Extracts the bytes of `self` without swapping the byte order.

  

  The returned bytes will be in `O` byte order.

#### Trait Implementations

##### `impl<O: ByteOrder> Add for U128<O>`

- <span id="u128-add-type-output"></span>`type Output = U128<O>`

- <span id="u128-add"></span>`fn add(self, rhs: U128<O>) -> U128<O>` — [`U128`](#u128)

##### `impl<O: ByteOrder> AddAssign for U128<O>`

- <span id="u128-addassign-add-assign"></span>`fn add_assign(&mut self, rhs: U128<O>)` — [`U128`](#u128)

##### `impl<O> AsMut for U128<O>`

- <span id="u128-asmut-as-mut"></span>`fn as_mut(&mut self) -> &mut [u8; 16]`

##### `impl<O> AsRef for U128<O>`

- <span id="u128-asref-as-ref"></span>`fn as_ref(&self) -> &[u8; 16]`

##### `impl<O: ByteOrder> Binary for U128<O>`

- <span id="u128-binary-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> BitAnd for U128<O>`

- <span id="u128-bitand-type-output"></span>`type Output = U128<O>`

- <span id="u128-bitand"></span>`fn bitand(self, rhs: U128<O>) -> U128<O>` — [`U128`](#u128)

##### `impl<O: ByteOrder> BitAndAssign for U128<O>`

- <span id="u128-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, rhs: U128<O>)` — [`U128`](#u128)

##### `impl<O: ByteOrder> BitOr for U128<O>`

- <span id="u128-bitor-type-output"></span>`type Output = U128<O>`

- <span id="u128-bitor"></span>`fn bitor(self, rhs: U128<O>) -> U128<O>` — [`U128`](#u128)

##### `impl<O: ByteOrder> BitOrAssign for U128<O>`

- <span id="u128-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, rhs: U128<O>)` — [`U128`](#u128)

##### `impl<O: ByteOrder> BitXor for U128<O>`

- <span id="u128-bitxor-type-output"></span>`type Output = U128<O>`

- <span id="u128-bitxor"></span>`fn bitxor(self, rhs: U128<O>) -> U128<O>` — [`U128`](#u128)

##### `impl<O: ByteOrder> BitXorAssign for U128<O>`

- <span id="u128-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, rhs: U128<O>)` — [`U128`](#u128)

##### `impl<O: clone::Clone> Clone for U128<O>`

- <span id="u128-clone"></span>`fn clone(&self) -> U128<O>` — [`U128`](#u128)

##### `impl<O: marker::Copy> Copy for U128<O>`

##### `impl<O: ByteOrder> Debug for U128<O>`

- <span id="u128-debug-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O> Default for U128<O>`

- <span id="u128-default"></span>`fn default() -> U128<O>` — [`U128`](#u128)

##### `impl<O: ByteOrder> Display for U128<O>`

- <span id="u128-display-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Div for U128<O>`

- <span id="u128-div-type-output"></span>`type Output = U128<O>`

- <span id="u128-div"></span>`fn div(self, rhs: U128<O>) -> U128<O>` — [`U128`](#u128)

##### `impl<O: ByteOrder> DivAssign for U128<O>`

- <span id="u128-divassign-div-assign"></span>`fn div_assign(&mut self, rhs: U128<O>)` — [`U128`](#u128)

##### `impl<O: cmp::Eq> Eq for U128<O>`

##### `impl<O> FromBytes for U128<O>`

##### `impl<O> FromZeros for U128<O>`

##### `impl<O: hash::Hash> Hash for U128<O>`

- <span id="u128-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<O> Immutable for U128<O>`

##### `impl<O> IntoBytes for U128<O>`

##### `impl<O> KnownLayout for U128<O>`

- <span id="u128-knownlayout-type-pointermetadata"></span>`type PointerMetadata = ()`

##### `impl<O: ByteOrder> LowerHex for U128<O>`

- <span id="u128-lowerhex-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Mul for U128<O>`

- <span id="u128-mul-type-output"></span>`type Output = U128<O>`

- <span id="u128-mul"></span>`fn mul(self, rhs: U128<O>) -> U128<O>` — [`U128`](#u128)

##### `impl<O: ByteOrder> MulAssign for U128<O>`

- <span id="u128-mulassign-mul-assign"></span>`fn mul_assign(&mut self, rhs: U128<O>)` — [`U128`](#u128)

##### `impl<O> Not for U128<O>`

- <span id="u128-not-type-output"></span>`type Output = U128<O>`

- <span id="u128-not"></span>`fn not(self) -> U128<O>` — [`U128`](#u128)

##### `impl<O: ByteOrder> Octal for U128<O>`

- <span id="u128-octal-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Ord for U128<O>`

- <span id="u128-ord-cmp"></span>`fn cmp(&self, other: &Self) -> Ordering`

##### `impl<O: cmp::PartialEq> PartialEq for U128<O>`

- <span id="u128-partialeq-eq"></span>`fn eq(&self, other: &U128<O>) -> bool` — [`U128`](#u128)

##### `impl<O: ByteOrder> PartialOrd for U128<O>`

- <span id="u128-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

##### `impl<O: ByteOrder> Rem for U128<O>`

- <span id="u128-rem-type-output"></span>`type Output = U128<O>`

- <span id="u128-rem"></span>`fn rem(self, rhs: U128<O>) -> U128<O>` — [`U128`](#u128)

##### `impl<O: ByteOrder> RemAssign for U128<O>`

- <span id="u128-remassign-rem-assign"></span>`fn rem_assign(&mut self, rhs: U128<O>)` — [`U128`](#u128)

##### `impl<O: ByteOrder> Shl for U128<O>`

- <span id="u128-shl-type-output"></span>`type Output = U128<O>`

- <span id="u128-shl"></span>`fn shl(self, rhs: U128<O>) -> U128<O>` — [`U128`](#u128)

##### `impl<O: ByteOrder> ShlAssign for U128<O>`

- <span id="u128-shlassign-shl-assign"></span>`fn shl_assign(&mut self, rhs: U128<O>)` — [`U128`](#u128)

##### `impl<O: ByteOrder> Shr for U128<O>`

- <span id="u128-shr-type-output"></span>`type Output = U128<O>`

- <span id="u128-shr"></span>`fn shr(self, rhs: U128<O>) -> U128<O>` — [`U128`](#u128)

##### `impl<O: ByteOrder> ShrAssign for U128<O>`

- <span id="u128-shrassign-shr-assign"></span>`fn shr_assign(&mut self, rhs: U128<O>)` — [`U128`](#u128)

##### `impl<O> StructuralPartialEq for U128<O>`

##### `impl<O: ByteOrder> Sub for U128<O>`

- <span id="u128-sub-type-output"></span>`type Output = U128<O>`

- <span id="u128-sub"></span>`fn sub(self, rhs: U128<O>) -> U128<O>` — [`U128`](#u128)

##### `impl<O: ByteOrder> SubAssign for U128<O>`

- <span id="u128-subassign-sub-assign"></span>`fn sub_assign(&mut self, rhs: U128<O>)` — [`U128`](#u128)

##### `impl ToString for U128<O>`

- <span id="u128-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<O> TryFromBytes for U128<O>`

##### `impl<O> Unaligned for U128<O>`

##### `impl<O: ByteOrder> UpperHex for U128<O>`

- <span id="u128-upperhex-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

### `Usize<O>`

```rust
struct Usize<O>([u8; 8], PhantomData<O>);
```

A word-sized unsigned integer stored in a given byte order.

`Usize` is like the native `usize` type with
two major differences: First, it has no alignment requirement (its alignment is 1).
Second, the endianness of its memory layout is given by the type parameter `O`,
which can be any type which implements [`ByteOrder`](#byteorder). In particular, this refers
to [`BigEndian`](#bigendian), [`LittleEndian`](#littleendian), [`NativeEndian`](#nativeendian), and [`NetworkEndian`](#networkendian).

A `Usize` can be constructed using
the `new` method, and its contained value can be obtained as a native
`usize` using the `get` method, or updated in place with
the `set` method. In all cases, if the endianness `O` is not the same as the
endianness of the current platform, an endianness swap will be performed in
order to uphold the invariants that a) the layout of `Usize`
has endianness `O` and that, b) the layout of `usize` has
the platform's native endianness.

`Usize` implements [`FromBytes`](#frombytes), [`IntoBytes`](#intobytes), and [`Unaligned`](#unaligned),
making it useful for parsing and serialization. See the module documentation for an
example of how it can be used for parsing UDP packets.







#### Implementations

- <span id="usize-const-zero"></span>`const ZERO: Usize<O>`

- <span id="usize-const-max-value"></span>`const MAX_VALUE: Usize<O>`

- <span id="usize-from-bytes"></span>`const fn from_bytes(bytes: [u8; 8]) -> Usize<O>` — [`Usize`](#usize)

  Constructs a new value from bytes which are already in `O` byte

  order.

- <span id="usize-to-bytes"></span>`const fn to_bytes(self) -> [u8; 8]`

  Extracts the bytes of `self` without swapping the byte order.

  

  The returned bytes will be in `O` byte order.

#### Trait Implementations

##### `impl<O: ByteOrder> Add for Usize<O>`

- <span id="usize-add-type-output"></span>`type Output = Usize<O>`

- <span id="usize-add"></span>`fn add(self, rhs: Usize<O>) -> Usize<O>` — [`Usize`](#usize)

##### `impl<O: ByteOrder> AddAssign for Usize<O>`

- <span id="usize-addassign-add-assign"></span>`fn add_assign(&mut self, rhs: Usize<O>)` — [`Usize`](#usize)

##### `impl<O> AsMut for Usize<O>`

- <span id="usize-asmut-as-mut"></span>`fn as_mut(&mut self) -> &mut [u8; 8]`

##### `impl<O> AsRef for Usize<O>`

- <span id="usize-asref-as-ref"></span>`fn as_ref(&self) -> &[u8; 8]`

##### `impl<O: ByteOrder> Binary for Usize<O>`

- <span id="usize-binary-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> BitAnd for Usize<O>`

- <span id="usize-bitand-type-output"></span>`type Output = Usize<O>`

- <span id="usize-bitand"></span>`fn bitand(self, rhs: Usize<O>) -> Usize<O>` — [`Usize`](#usize)

##### `impl<O: ByteOrder> BitAndAssign for Usize<O>`

- <span id="usize-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, rhs: Usize<O>)` — [`Usize`](#usize)

##### `impl<O: ByteOrder> BitOr for Usize<O>`

- <span id="usize-bitor-type-output"></span>`type Output = Usize<O>`

- <span id="usize-bitor"></span>`fn bitor(self, rhs: Usize<O>) -> Usize<O>` — [`Usize`](#usize)

##### `impl<O: ByteOrder> BitOrAssign for Usize<O>`

- <span id="usize-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, rhs: Usize<O>)` — [`Usize`](#usize)

##### `impl<O: ByteOrder> BitXor for Usize<O>`

- <span id="usize-bitxor-type-output"></span>`type Output = Usize<O>`

- <span id="usize-bitxor"></span>`fn bitxor(self, rhs: Usize<O>) -> Usize<O>` — [`Usize`](#usize)

##### `impl<O: ByteOrder> BitXorAssign for Usize<O>`

- <span id="usize-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, rhs: Usize<O>)` — [`Usize`](#usize)

##### `impl<O: clone::Clone> Clone for Usize<O>`

- <span id="usize-clone"></span>`fn clone(&self) -> Usize<O>` — [`Usize`](#usize)

##### `impl<O: marker::Copy> Copy for Usize<O>`

##### `impl<O: ByteOrder> Debug for Usize<O>`

- <span id="usize-debug-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O> Default for Usize<O>`

- <span id="usize-default"></span>`fn default() -> Usize<O>` — [`Usize`](#usize)

##### `impl<O: ByteOrder> Display for Usize<O>`

- <span id="usize-display-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Div for Usize<O>`

- <span id="usize-div-type-output"></span>`type Output = Usize<O>`

- <span id="usize-div"></span>`fn div(self, rhs: Usize<O>) -> Usize<O>` — [`Usize`](#usize)

##### `impl<O: ByteOrder> DivAssign for Usize<O>`

- <span id="usize-divassign-div-assign"></span>`fn div_assign(&mut self, rhs: Usize<O>)` — [`Usize`](#usize)

##### `impl<O: cmp::Eq> Eq for Usize<O>`

##### `impl<O> FromBytes for Usize<O>`

##### `impl<O> FromZeros for Usize<O>`

##### `impl<O: hash::Hash> Hash for Usize<O>`

- <span id="usize-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<O> Immutable for Usize<O>`

##### `impl<O> IntoBytes for Usize<O>`

##### `impl<O> KnownLayout for Usize<O>`

- <span id="usize-knownlayout-type-pointermetadata"></span>`type PointerMetadata = ()`

##### `impl<O: ByteOrder> LowerHex for Usize<O>`

- <span id="usize-lowerhex-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Mul for Usize<O>`

- <span id="usize-mul-type-output"></span>`type Output = Usize<O>`

- <span id="usize-mul"></span>`fn mul(self, rhs: Usize<O>) -> Usize<O>` — [`Usize`](#usize)

##### `impl<O: ByteOrder> MulAssign for Usize<O>`

- <span id="usize-mulassign-mul-assign"></span>`fn mul_assign(&mut self, rhs: Usize<O>)` — [`Usize`](#usize)

##### `impl<O> Not for Usize<O>`

- <span id="usize-not-type-output"></span>`type Output = Usize<O>`

- <span id="usize-not"></span>`fn not(self) -> Usize<O>` — [`Usize`](#usize)

##### `impl<O: ByteOrder> Octal for Usize<O>`

- <span id="usize-octal-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Ord for Usize<O>`

- <span id="usize-ord-cmp"></span>`fn cmp(&self, other: &Self) -> Ordering`

##### `impl<O: cmp::PartialEq> PartialEq for Usize<O>`

- <span id="usize-partialeq-eq"></span>`fn eq(&self, other: &Usize<O>) -> bool` — [`Usize`](#usize)

##### `impl<O: ByteOrder> PartialOrd for Usize<O>`

- <span id="usize-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

##### `impl<O: ByteOrder> Rem for Usize<O>`

- <span id="usize-rem-type-output"></span>`type Output = Usize<O>`

- <span id="usize-rem"></span>`fn rem(self, rhs: Usize<O>) -> Usize<O>` — [`Usize`](#usize)

##### `impl<O: ByteOrder> RemAssign for Usize<O>`

- <span id="usize-remassign-rem-assign"></span>`fn rem_assign(&mut self, rhs: Usize<O>)` — [`Usize`](#usize)

##### `impl<O: ByteOrder> Shl for Usize<O>`

- <span id="usize-shl-type-output"></span>`type Output = Usize<O>`

- <span id="usize-shl"></span>`fn shl(self, rhs: Usize<O>) -> Usize<O>` — [`Usize`](#usize)

##### `impl<O: ByteOrder> ShlAssign for Usize<O>`

- <span id="usize-shlassign-shl-assign"></span>`fn shl_assign(&mut self, rhs: Usize<O>)` — [`Usize`](#usize)

##### `impl<O: ByteOrder> Shr for Usize<O>`

- <span id="usize-shr-type-output"></span>`type Output = Usize<O>`

- <span id="usize-shr"></span>`fn shr(self, rhs: Usize<O>) -> Usize<O>` — [`Usize`](#usize)

##### `impl<O: ByteOrder> ShrAssign for Usize<O>`

- <span id="usize-shrassign-shr-assign"></span>`fn shr_assign(&mut self, rhs: Usize<O>)` — [`Usize`](#usize)

##### `impl<O> StructuralPartialEq for Usize<O>`

##### `impl<O: ByteOrder> Sub for Usize<O>`

- <span id="usize-sub-type-output"></span>`type Output = Usize<O>`

- <span id="usize-sub"></span>`fn sub(self, rhs: Usize<O>) -> Usize<O>` — [`Usize`](#usize)

##### `impl<O: ByteOrder> SubAssign for Usize<O>`

- <span id="usize-subassign-sub-assign"></span>`fn sub_assign(&mut self, rhs: Usize<O>)` — [`Usize`](#usize)

##### `impl ToString for Usize<O>`

- <span id="usize-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<O> TryFromBytes for Usize<O>`

##### `impl<O> Unaligned for Usize<O>`

##### `impl<O: ByteOrder> UpperHex for Usize<O>`

- <span id="usize-upperhex-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

### `I16<O>`

```rust
struct I16<O>([u8; 2], PhantomData<O>);
```

A 16-bit signed integer stored in a given byte order.

`I16` is like the native `i16` type with
two major differences: First, it has no alignment requirement (its alignment is 1).
Second, the endianness of its memory layout is given by the type parameter `O`,
which can be any type which implements [`ByteOrder`](#byteorder). In particular, this refers
to [`BigEndian`](#bigendian), [`LittleEndian`](#littleendian), [`NativeEndian`](#nativeendian), and [`NetworkEndian`](#networkendian).

An `I16` can be constructed using
the `new` method, and its contained value can be obtained as a native
`i16` using the `get` method, or updated in place with
the `set` method. In all cases, if the endianness `O` is not the same as the
endianness of the current platform, an endianness swap will be performed in
order to uphold the invariants that a) the layout of `I16`
has endianness `O` and that, b) the layout of `i16` has
the platform's native endianness.

`I16` implements [`FromBytes`](#frombytes), [`IntoBytes`](#intobytes), and [`Unaligned`](#unaligned),
making it useful for parsing and serialization. See the module documentation for an
example of how it can be used for parsing UDP packets.







#### Implementations

- <span id="i16-const-zero"></span>`const ZERO: I16<O>`

- <span id="i16-from-bytes"></span>`const fn from_bytes(bytes: [u8; 2]) -> I16<O>` — [`I16`](#i16)

  Constructs a new value from bytes which are already in `O` byte

  order.

- <span id="i16-to-bytes"></span>`const fn to_bytes(self) -> [u8; 2]`

  Extracts the bytes of `self` without swapping the byte order.

  

  The returned bytes will be in `O` byte order.

#### Trait Implementations

##### `impl<O: ByteOrder> Add for I16<O>`

- <span id="i16-add-type-output"></span>`type Output = I16<O>`

- <span id="i16-add"></span>`fn add(self, rhs: I16<O>) -> I16<O>` — [`I16`](#i16)

##### `impl<O: ByteOrder> AddAssign for I16<O>`

- <span id="i16-addassign-add-assign"></span>`fn add_assign(&mut self, rhs: I16<O>)` — [`I16`](#i16)

##### `impl<O> AsMut for I16<O>`

- <span id="i16-asmut-as-mut"></span>`fn as_mut(&mut self) -> &mut [u8; 2]`

##### `impl<O> AsRef for I16<O>`

- <span id="i16-asref-as-ref"></span>`fn as_ref(&self) -> &[u8; 2]`

##### `impl<O: ByteOrder> Binary for I16<O>`

- <span id="i16-binary-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> BitAnd for I16<O>`

- <span id="i16-bitand-type-output"></span>`type Output = I16<O>`

- <span id="i16-bitand"></span>`fn bitand(self, rhs: I16<O>) -> I16<O>` — [`I16`](#i16)

##### `impl<O: ByteOrder> BitAndAssign for I16<O>`

- <span id="i16-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, rhs: I16<O>)` — [`I16`](#i16)

##### `impl<O: ByteOrder> BitOr for I16<O>`

- <span id="i16-bitor-type-output"></span>`type Output = I16<O>`

- <span id="i16-bitor"></span>`fn bitor(self, rhs: I16<O>) -> I16<O>` — [`I16`](#i16)

##### `impl<O: ByteOrder> BitOrAssign for I16<O>`

- <span id="i16-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, rhs: I16<O>)` — [`I16`](#i16)

##### `impl<O: ByteOrder> BitXor for I16<O>`

- <span id="i16-bitxor-type-output"></span>`type Output = I16<O>`

- <span id="i16-bitxor"></span>`fn bitxor(self, rhs: I16<O>) -> I16<O>` — [`I16`](#i16)

##### `impl<O: ByteOrder> BitXorAssign for I16<O>`

- <span id="i16-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, rhs: I16<O>)` — [`I16`](#i16)

##### `impl<O: clone::Clone> Clone for I16<O>`

- <span id="i16-clone"></span>`fn clone(&self) -> I16<O>` — [`I16`](#i16)

##### `impl<O: marker::Copy> Copy for I16<O>`

##### `impl<O: ByteOrder> Debug for I16<O>`

- <span id="i16-debug-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O> Default for I16<O>`

- <span id="i16-default"></span>`fn default() -> I16<O>` — [`I16`](#i16)

##### `impl<O: ByteOrder> Display for I16<O>`

- <span id="i16-display-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Div for I16<O>`

- <span id="i16-div-type-output"></span>`type Output = I16<O>`

- <span id="i16-div"></span>`fn div(self, rhs: I16<O>) -> I16<O>` — [`I16`](#i16)

##### `impl<O: ByteOrder> DivAssign for I16<O>`

- <span id="i16-divassign-div-assign"></span>`fn div_assign(&mut self, rhs: I16<O>)` — [`I16`](#i16)

##### `impl<O: cmp::Eq> Eq for I16<O>`

##### `impl<O> FromBytes for I16<O>`

##### `impl<O> FromZeros for I16<O>`

##### `impl<O: hash::Hash> Hash for I16<O>`

- <span id="i16-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<O> Immutable for I16<O>`

##### `impl<O> IntoBytes for I16<O>`

##### `impl<O> KnownLayout for I16<O>`

- <span id="i16-knownlayout-type-pointermetadata"></span>`type PointerMetadata = ()`

##### `impl<O: ByteOrder> LowerHex for I16<O>`

- <span id="i16-lowerhex-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Mul for I16<O>`

- <span id="i16-mul-type-output"></span>`type Output = I16<O>`

- <span id="i16-mul"></span>`fn mul(self, rhs: I16<O>) -> I16<O>` — [`I16`](#i16)

##### `impl<O: ByteOrder> MulAssign for I16<O>`

- <span id="i16-mulassign-mul-assign"></span>`fn mul_assign(&mut self, rhs: I16<O>)` — [`I16`](#i16)

##### `impl<O: ByteOrder> Neg for I16<O>`

- <span id="i16-neg-type-output"></span>`type Output = I16<O>`

- <span id="i16-neg"></span>`fn neg(self) -> I16<O>` — [`I16`](#i16)

##### `impl<O> Not for I16<O>`

- <span id="i16-not-type-output"></span>`type Output = I16<O>`

- <span id="i16-not"></span>`fn not(self) -> I16<O>` — [`I16`](#i16)

##### `impl<O: ByteOrder> Octal for I16<O>`

- <span id="i16-octal-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Ord for I16<O>`

- <span id="i16-ord-cmp"></span>`fn cmp(&self, other: &Self) -> Ordering`

##### `impl<O: cmp::PartialEq> PartialEq for I16<O>`

- <span id="i16-partialeq-eq"></span>`fn eq(&self, other: &I16<O>) -> bool` — [`I16`](#i16)

##### `impl<O: ByteOrder> PartialOrd for I16<O>`

- <span id="i16-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

##### `impl<O: ByteOrder> Rem for I16<O>`

- <span id="i16-rem-type-output"></span>`type Output = I16<O>`

- <span id="i16-rem"></span>`fn rem(self, rhs: I16<O>) -> I16<O>` — [`I16`](#i16)

##### `impl<O: ByteOrder> RemAssign for I16<O>`

- <span id="i16-remassign-rem-assign"></span>`fn rem_assign(&mut self, rhs: I16<O>)` — [`I16`](#i16)

##### `impl<O: ByteOrder> Shl for I16<O>`

- <span id="i16-shl-type-output"></span>`type Output = I16<O>`

- <span id="i16-shl"></span>`fn shl(self, rhs: I16<O>) -> I16<O>` — [`I16`](#i16)

##### `impl<O: ByteOrder> ShlAssign for I16<O>`

- <span id="i16-shlassign-shl-assign"></span>`fn shl_assign(&mut self, rhs: I16<O>)` — [`I16`](#i16)

##### `impl<O: ByteOrder> Shr for I16<O>`

- <span id="i16-shr-type-output"></span>`type Output = I16<O>`

- <span id="i16-shr"></span>`fn shr(self, rhs: I16<O>) -> I16<O>` — [`I16`](#i16)

##### `impl<O: ByteOrder> ShrAssign for I16<O>`

- <span id="i16-shrassign-shr-assign"></span>`fn shr_assign(&mut self, rhs: I16<O>)` — [`I16`](#i16)

##### `impl<O> StructuralPartialEq for I16<O>`

##### `impl<O: ByteOrder> Sub for I16<O>`

- <span id="i16-sub-type-output"></span>`type Output = I16<O>`

- <span id="i16-sub"></span>`fn sub(self, rhs: I16<O>) -> I16<O>` — [`I16`](#i16)

##### `impl<O: ByteOrder> SubAssign for I16<O>`

- <span id="i16-subassign-sub-assign"></span>`fn sub_assign(&mut self, rhs: I16<O>)` — [`I16`](#i16)

##### `impl ToString for I16<O>`

- <span id="i16-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<O> TryFromBytes for I16<O>`

##### `impl<O> Unaligned for I16<O>`

##### `impl<O: ByteOrder> UpperHex for I16<O>`

- <span id="i16-upperhex-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

### `I32<O>`

```rust
struct I32<O>([u8; 4], PhantomData<O>);
```

A 32-bit signed integer stored in a given byte order.

`I32` is like the native `i32` type with
two major differences: First, it has no alignment requirement (its alignment is 1).
Second, the endianness of its memory layout is given by the type parameter `O`,
which can be any type which implements [`ByteOrder`](#byteorder). In particular, this refers
to [`BigEndian`](#bigendian), [`LittleEndian`](#littleendian), [`NativeEndian`](#nativeendian), and [`NetworkEndian`](#networkendian).

An `I32` can be constructed using
the `new` method, and its contained value can be obtained as a native
`i32` using the `get` method, or updated in place with
the `set` method. In all cases, if the endianness `O` is not the same as the
endianness of the current platform, an endianness swap will be performed in
order to uphold the invariants that a) the layout of `I32`
has endianness `O` and that, b) the layout of `i32` has
the platform's native endianness.

`I32` implements [`FromBytes`](#frombytes), [`IntoBytes`](#intobytes), and [`Unaligned`](#unaligned),
making it useful for parsing and serialization. See the module documentation for an
example of how it can be used for parsing UDP packets.







#### Implementations

- <span id="i32-const-zero"></span>`const ZERO: I32<O>`

- <span id="i32-from-bytes"></span>`const fn from_bytes(bytes: [u8; 4]) -> I32<O>` — [`I32`](#i32)

  Constructs a new value from bytes which are already in `O` byte

  order.

- <span id="i32-to-bytes"></span>`const fn to_bytes(self) -> [u8; 4]`

  Extracts the bytes of `self` without swapping the byte order.

  

  The returned bytes will be in `O` byte order.

#### Trait Implementations

##### `impl<O: ByteOrder> Add for I32<O>`

- <span id="i32-add-type-output"></span>`type Output = I32<O>`

- <span id="i32-add"></span>`fn add(self, rhs: I32<O>) -> I32<O>` — [`I32`](#i32)

##### `impl<O: ByteOrder> AddAssign for I32<O>`

- <span id="i32-addassign-add-assign"></span>`fn add_assign(&mut self, rhs: I32<O>)` — [`I32`](#i32)

##### `impl<O> AsMut for I32<O>`

- <span id="i32-asmut-as-mut"></span>`fn as_mut(&mut self) -> &mut [u8; 4]`

##### `impl<O> AsRef for I32<O>`

- <span id="i32-asref-as-ref"></span>`fn as_ref(&self) -> &[u8; 4]`

##### `impl<O: ByteOrder> Binary for I32<O>`

- <span id="i32-binary-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> BitAnd for I32<O>`

- <span id="i32-bitand-type-output"></span>`type Output = I32<O>`

- <span id="i32-bitand"></span>`fn bitand(self, rhs: I32<O>) -> I32<O>` — [`I32`](#i32)

##### `impl<O: ByteOrder> BitAndAssign for I32<O>`

- <span id="i32-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, rhs: I32<O>)` — [`I32`](#i32)

##### `impl<O: ByteOrder> BitOr for I32<O>`

- <span id="i32-bitor-type-output"></span>`type Output = I32<O>`

- <span id="i32-bitor"></span>`fn bitor(self, rhs: I32<O>) -> I32<O>` — [`I32`](#i32)

##### `impl<O: ByteOrder> BitOrAssign for I32<O>`

- <span id="i32-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, rhs: I32<O>)` — [`I32`](#i32)

##### `impl<O: ByteOrder> BitXor for I32<O>`

- <span id="i32-bitxor-type-output"></span>`type Output = I32<O>`

- <span id="i32-bitxor"></span>`fn bitxor(self, rhs: I32<O>) -> I32<O>` — [`I32`](#i32)

##### `impl<O: ByteOrder> BitXorAssign for I32<O>`

- <span id="i32-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, rhs: I32<O>)` — [`I32`](#i32)

##### `impl<O: clone::Clone> Clone for I32<O>`

- <span id="i32-clone"></span>`fn clone(&self) -> I32<O>` — [`I32`](#i32)

##### `impl<O: marker::Copy> Copy for I32<O>`

##### `impl<O: ByteOrder> Debug for I32<O>`

- <span id="i32-debug-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O> Default for I32<O>`

- <span id="i32-default"></span>`fn default() -> I32<O>` — [`I32`](#i32)

##### `impl<O: ByteOrder> Display for I32<O>`

- <span id="i32-display-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Div for I32<O>`

- <span id="i32-div-type-output"></span>`type Output = I32<O>`

- <span id="i32-div"></span>`fn div(self, rhs: I32<O>) -> I32<O>` — [`I32`](#i32)

##### `impl<O: ByteOrder> DivAssign for I32<O>`

- <span id="i32-divassign-div-assign"></span>`fn div_assign(&mut self, rhs: I32<O>)` — [`I32`](#i32)

##### `impl<O: cmp::Eq> Eq for I32<O>`

##### `impl<O> FromBytes for I32<O>`

##### `impl<O> FromZeros for I32<O>`

##### `impl<O: hash::Hash> Hash for I32<O>`

- <span id="i32-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<O> Immutable for I32<O>`

##### `impl<O> IntoBytes for I32<O>`

##### `impl<O> KnownLayout for I32<O>`

- <span id="i32-knownlayout-type-pointermetadata"></span>`type PointerMetadata = ()`

##### `impl<O: ByteOrder> LowerHex for I32<O>`

- <span id="i32-lowerhex-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Mul for I32<O>`

- <span id="i32-mul-type-output"></span>`type Output = I32<O>`

- <span id="i32-mul"></span>`fn mul(self, rhs: I32<O>) -> I32<O>` — [`I32`](#i32)

##### `impl<O: ByteOrder> MulAssign for I32<O>`

- <span id="i32-mulassign-mul-assign"></span>`fn mul_assign(&mut self, rhs: I32<O>)` — [`I32`](#i32)

##### `impl<O: ByteOrder> Neg for I32<O>`

- <span id="i32-neg-type-output"></span>`type Output = I32<O>`

- <span id="i32-neg"></span>`fn neg(self) -> I32<O>` — [`I32`](#i32)

##### `impl<O> Not for I32<O>`

- <span id="i32-not-type-output"></span>`type Output = I32<O>`

- <span id="i32-not"></span>`fn not(self) -> I32<O>` — [`I32`](#i32)

##### `impl<O: ByteOrder> Octal for I32<O>`

- <span id="i32-octal-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Ord for I32<O>`

- <span id="i32-ord-cmp"></span>`fn cmp(&self, other: &Self) -> Ordering`

##### `impl<O: cmp::PartialEq> PartialEq for I32<O>`

- <span id="i32-partialeq-eq"></span>`fn eq(&self, other: &I32<O>) -> bool` — [`I32`](#i32)

##### `impl<O: ByteOrder> PartialOrd for I32<O>`

- <span id="i32-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

##### `impl<O: ByteOrder> Rem for I32<O>`

- <span id="i32-rem-type-output"></span>`type Output = I32<O>`

- <span id="i32-rem"></span>`fn rem(self, rhs: I32<O>) -> I32<O>` — [`I32`](#i32)

##### `impl<O: ByteOrder> RemAssign for I32<O>`

- <span id="i32-remassign-rem-assign"></span>`fn rem_assign(&mut self, rhs: I32<O>)` — [`I32`](#i32)

##### `impl<O: ByteOrder> Shl for I32<O>`

- <span id="i32-shl-type-output"></span>`type Output = I32<O>`

- <span id="i32-shl"></span>`fn shl(self, rhs: I32<O>) -> I32<O>` — [`I32`](#i32)

##### `impl<O: ByteOrder> ShlAssign for I32<O>`

- <span id="i32-shlassign-shl-assign"></span>`fn shl_assign(&mut self, rhs: I32<O>)` — [`I32`](#i32)

##### `impl<O: ByteOrder> Shr for I32<O>`

- <span id="i32-shr-type-output"></span>`type Output = I32<O>`

- <span id="i32-shr"></span>`fn shr(self, rhs: I32<O>) -> I32<O>` — [`I32`](#i32)

##### `impl<O: ByteOrder> ShrAssign for I32<O>`

- <span id="i32-shrassign-shr-assign"></span>`fn shr_assign(&mut self, rhs: I32<O>)` — [`I32`](#i32)

##### `impl<O> StructuralPartialEq for I32<O>`

##### `impl<O: ByteOrder> Sub for I32<O>`

- <span id="i32-sub-type-output"></span>`type Output = I32<O>`

- <span id="i32-sub"></span>`fn sub(self, rhs: I32<O>) -> I32<O>` — [`I32`](#i32)

##### `impl<O: ByteOrder> SubAssign for I32<O>`

- <span id="i32-subassign-sub-assign"></span>`fn sub_assign(&mut self, rhs: I32<O>)` — [`I32`](#i32)

##### `impl ToString for I32<O>`

- <span id="i32-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<O> TryFromBytes for I32<O>`

##### `impl<O> Unaligned for I32<O>`

##### `impl<O: ByteOrder> UpperHex for I32<O>`

- <span id="i32-upperhex-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

### `I64<O>`

```rust
struct I64<O>([u8; 8], PhantomData<O>);
```

A 64-bit signed integer stored in a given byte order.

`I64` is like the native `i64` type with
two major differences: First, it has no alignment requirement (its alignment is 1).
Second, the endianness of its memory layout is given by the type parameter `O`,
which can be any type which implements [`ByteOrder`](#byteorder). In particular, this refers
to [`BigEndian`](#bigendian), [`LittleEndian`](#littleendian), [`NativeEndian`](#nativeendian), and [`NetworkEndian`](#networkendian).

An `I64` can be constructed using
the `new` method, and its contained value can be obtained as a native
`i64` using the `get` method, or updated in place with
the `set` method. In all cases, if the endianness `O` is not the same as the
endianness of the current platform, an endianness swap will be performed in
order to uphold the invariants that a) the layout of `I64`
has endianness `O` and that, b) the layout of `i64` has
the platform's native endianness.

`I64` implements [`FromBytes`](#frombytes), [`IntoBytes`](#intobytes), and [`Unaligned`](#unaligned),
making it useful for parsing and serialization. See the module documentation for an
example of how it can be used for parsing UDP packets.







#### Implementations

- <span id="i64-const-zero"></span>`const ZERO: I64<O>`

- <span id="i64-from-bytes"></span>`const fn from_bytes(bytes: [u8; 8]) -> I64<O>` — [`I64`](#i64)

  Constructs a new value from bytes which are already in `O` byte

  order.

- <span id="i64-to-bytes"></span>`const fn to_bytes(self) -> [u8; 8]`

  Extracts the bytes of `self` without swapping the byte order.

  

  The returned bytes will be in `O` byte order.

#### Trait Implementations

##### `impl<O: ByteOrder> Add for I64<O>`

- <span id="i64-add-type-output"></span>`type Output = I64<O>`

- <span id="i64-add"></span>`fn add(self, rhs: I64<O>) -> I64<O>` — [`I64`](#i64)

##### `impl<O: ByteOrder> AddAssign for I64<O>`

- <span id="i64-addassign-add-assign"></span>`fn add_assign(&mut self, rhs: I64<O>)` — [`I64`](#i64)

##### `impl<O> AsMut for I64<O>`

- <span id="i64-asmut-as-mut"></span>`fn as_mut(&mut self) -> &mut [u8; 8]`

##### `impl<O> AsRef for I64<O>`

- <span id="i64-asref-as-ref"></span>`fn as_ref(&self) -> &[u8; 8]`

##### `impl<O: ByteOrder> Binary for I64<O>`

- <span id="i64-binary-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> BitAnd for I64<O>`

- <span id="i64-bitand-type-output"></span>`type Output = I64<O>`

- <span id="i64-bitand"></span>`fn bitand(self, rhs: I64<O>) -> I64<O>` — [`I64`](#i64)

##### `impl<O: ByteOrder> BitAndAssign for I64<O>`

- <span id="i64-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, rhs: I64<O>)` — [`I64`](#i64)

##### `impl<O: ByteOrder> BitOr for I64<O>`

- <span id="i64-bitor-type-output"></span>`type Output = I64<O>`

- <span id="i64-bitor"></span>`fn bitor(self, rhs: I64<O>) -> I64<O>` — [`I64`](#i64)

##### `impl<O: ByteOrder> BitOrAssign for I64<O>`

- <span id="i64-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, rhs: I64<O>)` — [`I64`](#i64)

##### `impl<O: ByteOrder> BitXor for I64<O>`

- <span id="i64-bitxor-type-output"></span>`type Output = I64<O>`

- <span id="i64-bitxor"></span>`fn bitxor(self, rhs: I64<O>) -> I64<O>` — [`I64`](#i64)

##### `impl<O: ByteOrder> BitXorAssign for I64<O>`

- <span id="i64-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, rhs: I64<O>)` — [`I64`](#i64)

##### `impl<O: clone::Clone> Clone for I64<O>`

- <span id="i64-clone"></span>`fn clone(&self) -> I64<O>` — [`I64`](#i64)

##### `impl<O: marker::Copy> Copy for I64<O>`

##### `impl<O: ByteOrder> Debug for I64<O>`

- <span id="i64-debug-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O> Default for I64<O>`

- <span id="i64-default"></span>`fn default() -> I64<O>` — [`I64`](#i64)

##### `impl<O: ByteOrder> Display for I64<O>`

- <span id="i64-display-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Div for I64<O>`

- <span id="i64-div-type-output"></span>`type Output = I64<O>`

- <span id="i64-div"></span>`fn div(self, rhs: I64<O>) -> I64<O>` — [`I64`](#i64)

##### `impl<O: ByteOrder> DivAssign for I64<O>`

- <span id="i64-divassign-div-assign"></span>`fn div_assign(&mut self, rhs: I64<O>)` — [`I64`](#i64)

##### `impl<O: cmp::Eq> Eq for I64<O>`

##### `impl<O> FromBytes for I64<O>`

##### `impl<O> FromZeros for I64<O>`

##### `impl<O: hash::Hash> Hash for I64<O>`

- <span id="i64-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<O> Immutable for I64<O>`

##### `impl<O> IntoBytes for I64<O>`

##### `impl<O> KnownLayout for I64<O>`

- <span id="i64-knownlayout-type-pointermetadata"></span>`type PointerMetadata = ()`

##### `impl<O: ByteOrder> LowerHex for I64<O>`

- <span id="i64-lowerhex-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Mul for I64<O>`

- <span id="i64-mul-type-output"></span>`type Output = I64<O>`

- <span id="i64-mul"></span>`fn mul(self, rhs: I64<O>) -> I64<O>` — [`I64`](#i64)

##### `impl<O: ByteOrder> MulAssign for I64<O>`

- <span id="i64-mulassign-mul-assign"></span>`fn mul_assign(&mut self, rhs: I64<O>)` — [`I64`](#i64)

##### `impl<O: ByteOrder> Neg for I64<O>`

- <span id="i64-neg-type-output"></span>`type Output = I64<O>`

- <span id="i64-neg"></span>`fn neg(self) -> I64<O>` — [`I64`](#i64)

##### `impl<O> Not for I64<O>`

- <span id="i64-not-type-output"></span>`type Output = I64<O>`

- <span id="i64-not"></span>`fn not(self) -> I64<O>` — [`I64`](#i64)

##### `impl<O: ByteOrder> Octal for I64<O>`

- <span id="i64-octal-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Ord for I64<O>`

- <span id="i64-ord-cmp"></span>`fn cmp(&self, other: &Self) -> Ordering`

##### `impl<O: cmp::PartialEq> PartialEq for I64<O>`

- <span id="i64-partialeq-eq"></span>`fn eq(&self, other: &I64<O>) -> bool` — [`I64`](#i64)

##### `impl<O: ByteOrder> PartialOrd for I64<O>`

- <span id="i64-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

##### `impl<O: ByteOrder> Rem for I64<O>`

- <span id="i64-rem-type-output"></span>`type Output = I64<O>`

- <span id="i64-rem"></span>`fn rem(self, rhs: I64<O>) -> I64<O>` — [`I64`](#i64)

##### `impl<O: ByteOrder> RemAssign for I64<O>`

- <span id="i64-remassign-rem-assign"></span>`fn rem_assign(&mut self, rhs: I64<O>)` — [`I64`](#i64)

##### `impl<O: ByteOrder> Shl for I64<O>`

- <span id="i64-shl-type-output"></span>`type Output = I64<O>`

- <span id="i64-shl"></span>`fn shl(self, rhs: I64<O>) -> I64<O>` — [`I64`](#i64)

##### `impl<O: ByteOrder> ShlAssign for I64<O>`

- <span id="i64-shlassign-shl-assign"></span>`fn shl_assign(&mut self, rhs: I64<O>)` — [`I64`](#i64)

##### `impl<O: ByteOrder> Shr for I64<O>`

- <span id="i64-shr-type-output"></span>`type Output = I64<O>`

- <span id="i64-shr"></span>`fn shr(self, rhs: I64<O>) -> I64<O>` — [`I64`](#i64)

##### `impl<O: ByteOrder> ShrAssign for I64<O>`

- <span id="i64-shrassign-shr-assign"></span>`fn shr_assign(&mut self, rhs: I64<O>)` — [`I64`](#i64)

##### `impl<O> StructuralPartialEq for I64<O>`

##### `impl<O: ByteOrder> Sub for I64<O>`

- <span id="i64-sub-type-output"></span>`type Output = I64<O>`

- <span id="i64-sub"></span>`fn sub(self, rhs: I64<O>) -> I64<O>` — [`I64`](#i64)

##### `impl<O: ByteOrder> SubAssign for I64<O>`

- <span id="i64-subassign-sub-assign"></span>`fn sub_assign(&mut self, rhs: I64<O>)` — [`I64`](#i64)

##### `impl ToString for I64<O>`

- <span id="i64-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<O> TryFromBytes for I64<O>`

##### `impl<O> Unaligned for I64<O>`

##### `impl<O: ByteOrder> UpperHex for I64<O>`

- <span id="i64-upperhex-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

### `I128<O>`

```rust
struct I128<O>([u8; 16], PhantomData<O>);
```

A 128-bit signed integer stored in a given byte order.

`I128` is like the native `i128` type with
two major differences: First, it has no alignment requirement (its alignment is 1).
Second, the endianness of its memory layout is given by the type parameter `O`,
which can be any type which implements [`ByteOrder`](#byteorder). In particular, this refers
to [`BigEndian`](#bigendian), [`LittleEndian`](#littleendian), [`NativeEndian`](#nativeendian), and [`NetworkEndian`](#networkendian).

An `I128` can be constructed using
the `new` method, and its contained value can be obtained as a native
`i128` using the `get` method, or updated in place with
the `set` method. In all cases, if the endianness `O` is not the same as the
endianness of the current platform, an endianness swap will be performed in
order to uphold the invariants that a) the layout of `I128`
has endianness `O` and that, b) the layout of `i128` has
the platform's native endianness.

`I128` implements [`FromBytes`](#frombytes), [`IntoBytes`](#intobytes), and [`Unaligned`](#unaligned),
making it useful for parsing and serialization. See the module documentation for an
example of how it can be used for parsing UDP packets.







#### Implementations

- <span id="i128-const-zero"></span>`const ZERO: I128<O>`

- <span id="i128-from-bytes"></span>`const fn from_bytes(bytes: [u8; 16]) -> I128<O>` — [`I128`](#i128)

  Constructs a new value from bytes which are already in `O` byte

  order.

- <span id="i128-to-bytes"></span>`const fn to_bytes(self) -> [u8; 16]`

  Extracts the bytes of `self` without swapping the byte order.

  

  The returned bytes will be in `O` byte order.

#### Trait Implementations

##### `impl<O: ByteOrder> Add for I128<O>`

- <span id="i128-add-type-output"></span>`type Output = I128<O>`

- <span id="i128-add"></span>`fn add(self, rhs: I128<O>) -> I128<O>` — [`I128`](#i128)

##### `impl<O: ByteOrder> AddAssign for I128<O>`

- <span id="i128-addassign-add-assign"></span>`fn add_assign(&mut self, rhs: I128<O>)` — [`I128`](#i128)

##### `impl<O> AsMut for I128<O>`

- <span id="i128-asmut-as-mut"></span>`fn as_mut(&mut self) -> &mut [u8; 16]`

##### `impl<O> AsRef for I128<O>`

- <span id="i128-asref-as-ref"></span>`fn as_ref(&self) -> &[u8; 16]`

##### `impl<O: ByteOrder> Binary for I128<O>`

- <span id="i128-binary-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> BitAnd for I128<O>`

- <span id="i128-bitand-type-output"></span>`type Output = I128<O>`

- <span id="i128-bitand"></span>`fn bitand(self, rhs: I128<O>) -> I128<O>` — [`I128`](#i128)

##### `impl<O: ByteOrder> BitAndAssign for I128<O>`

- <span id="i128-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, rhs: I128<O>)` — [`I128`](#i128)

##### `impl<O: ByteOrder> BitOr for I128<O>`

- <span id="i128-bitor-type-output"></span>`type Output = I128<O>`

- <span id="i128-bitor"></span>`fn bitor(self, rhs: I128<O>) -> I128<O>` — [`I128`](#i128)

##### `impl<O: ByteOrder> BitOrAssign for I128<O>`

- <span id="i128-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, rhs: I128<O>)` — [`I128`](#i128)

##### `impl<O: ByteOrder> BitXor for I128<O>`

- <span id="i128-bitxor-type-output"></span>`type Output = I128<O>`

- <span id="i128-bitxor"></span>`fn bitxor(self, rhs: I128<O>) -> I128<O>` — [`I128`](#i128)

##### `impl<O: ByteOrder> BitXorAssign for I128<O>`

- <span id="i128-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, rhs: I128<O>)` — [`I128`](#i128)

##### `impl<O: clone::Clone> Clone for I128<O>`

- <span id="i128-clone"></span>`fn clone(&self) -> I128<O>` — [`I128`](#i128)

##### `impl<O: marker::Copy> Copy for I128<O>`

##### `impl<O: ByteOrder> Debug for I128<O>`

- <span id="i128-debug-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O> Default for I128<O>`

- <span id="i128-default"></span>`fn default() -> I128<O>` — [`I128`](#i128)

##### `impl<O: ByteOrder> Display for I128<O>`

- <span id="i128-display-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Div for I128<O>`

- <span id="i128-div-type-output"></span>`type Output = I128<O>`

- <span id="i128-div"></span>`fn div(self, rhs: I128<O>) -> I128<O>` — [`I128`](#i128)

##### `impl<O: ByteOrder> DivAssign for I128<O>`

- <span id="i128-divassign-div-assign"></span>`fn div_assign(&mut self, rhs: I128<O>)` — [`I128`](#i128)

##### `impl<O: cmp::Eq> Eq for I128<O>`

##### `impl<O> FromBytes for I128<O>`

##### `impl<O> FromZeros for I128<O>`

##### `impl<O: hash::Hash> Hash for I128<O>`

- <span id="i128-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<O> Immutable for I128<O>`

##### `impl<O> IntoBytes for I128<O>`

##### `impl<O> KnownLayout for I128<O>`

- <span id="i128-knownlayout-type-pointermetadata"></span>`type PointerMetadata = ()`

##### `impl<O: ByteOrder> LowerHex for I128<O>`

- <span id="i128-lowerhex-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Mul for I128<O>`

- <span id="i128-mul-type-output"></span>`type Output = I128<O>`

- <span id="i128-mul"></span>`fn mul(self, rhs: I128<O>) -> I128<O>` — [`I128`](#i128)

##### `impl<O: ByteOrder> MulAssign for I128<O>`

- <span id="i128-mulassign-mul-assign"></span>`fn mul_assign(&mut self, rhs: I128<O>)` — [`I128`](#i128)

##### `impl<O: ByteOrder> Neg for I128<O>`

- <span id="i128-neg-type-output"></span>`type Output = I128<O>`

- <span id="i128-neg"></span>`fn neg(self) -> I128<O>` — [`I128`](#i128)

##### `impl<O> Not for I128<O>`

- <span id="i128-not-type-output"></span>`type Output = I128<O>`

- <span id="i128-not"></span>`fn not(self) -> I128<O>` — [`I128`](#i128)

##### `impl<O: ByteOrder> Octal for I128<O>`

- <span id="i128-octal-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Ord for I128<O>`

- <span id="i128-ord-cmp"></span>`fn cmp(&self, other: &Self) -> Ordering`

##### `impl<O: cmp::PartialEq> PartialEq for I128<O>`

- <span id="i128-partialeq-eq"></span>`fn eq(&self, other: &I128<O>) -> bool` — [`I128`](#i128)

##### `impl<O: ByteOrder> PartialOrd for I128<O>`

- <span id="i128-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

##### `impl<O: ByteOrder> Rem for I128<O>`

- <span id="i128-rem-type-output"></span>`type Output = I128<O>`

- <span id="i128-rem"></span>`fn rem(self, rhs: I128<O>) -> I128<O>` — [`I128`](#i128)

##### `impl<O: ByteOrder> RemAssign for I128<O>`

- <span id="i128-remassign-rem-assign"></span>`fn rem_assign(&mut self, rhs: I128<O>)` — [`I128`](#i128)

##### `impl<O: ByteOrder> Shl for I128<O>`

- <span id="i128-shl-type-output"></span>`type Output = I128<O>`

- <span id="i128-shl"></span>`fn shl(self, rhs: I128<O>) -> I128<O>` — [`I128`](#i128)

##### `impl<O: ByteOrder> ShlAssign for I128<O>`

- <span id="i128-shlassign-shl-assign"></span>`fn shl_assign(&mut self, rhs: I128<O>)` — [`I128`](#i128)

##### `impl<O: ByteOrder> Shr for I128<O>`

- <span id="i128-shr-type-output"></span>`type Output = I128<O>`

- <span id="i128-shr"></span>`fn shr(self, rhs: I128<O>) -> I128<O>` — [`I128`](#i128)

##### `impl<O: ByteOrder> ShrAssign for I128<O>`

- <span id="i128-shrassign-shr-assign"></span>`fn shr_assign(&mut self, rhs: I128<O>)` — [`I128`](#i128)

##### `impl<O> StructuralPartialEq for I128<O>`

##### `impl<O: ByteOrder> Sub for I128<O>`

- <span id="i128-sub-type-output"></span>`type Output = I128<O>`

- <span id="i128-sub"></span>`fn sub(self, rhs: I128<O>) -> I128<O>` — [`I128`](#i128)

##### `impl<O: ByteOrder> SubAssign for I128<O>`

- <span id="i128-subassign-sub-assign"></span>`fn sub_assign(&mut self, rhs: I128<O>)` — [`I128`](#i128)

##### `impl ToString for I128<O>`

- <span id="i128-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<O> TryFromBytes for I128<O>`

##### `impl<O> Unaligned for I128<O>`

##### `impl<O: ByteOrder> UpperHex for I128<O>`

- <span id="i128-upperhex-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

### `Isize<O>`

```rust
struct Isize<O>([u8; 8], PhantomData<O>);
```

A word-sized signed integer stored in a given byte order.

`Isize` is like the native `isize` type with
two major differences: First, it has no alignment requirement (its alignment is 1).
Second, the endianness of its memory layout is given by the type parameter `O`,
which can be any type which implements [`ByteOrder`](#byteorder). In particular, this refers
to [`BigEndian`](#bigendian), [`LittleEndian`](#littleendian), [`NativeEndian`](#nativeendian), and [`NetworkEndian`](#networkendian).

An `Isize` can be constructed using
the `new` method, and its contained value can be obtained as a native
`isize` using the `get` method, or updated in place with
the `set` method. In all cases, if the endianness `O` is not the same as the
endianness of the current platform, an endianness swap will be performed in
order to uphold the invariants that a) the layout of `Isize`
has endianness `O` and that, b) the layout of `isize` has
the platform's native endianness.

`Isize` implements [`FromBytes`](#frombytes), [`IntoBytes`](#intobytes), and [`Unaligned`](#unaligned),
making it useful for parsing and serialization. See the module documentation for an
example of how it can be used for parsing UDP packets.







#### Implementations

- <span id="isize-const-zero"></span>`const ZERO: Isize<O>`

- <span id="isize-from-bytes"></span>`const fn from_bytes(bytes: [u8; 8]) -> Isize<O>` — [`Isize`](#isize)

  Constructs a new value from bytes which are already in `O` byte

  order.

- <span id="isize-to-bytes"></span>`const fn to_bytes(self) -> [u8; 8]`

  Extracts the bytes of `self` without swapping the byte order.

  

  The returned bytes will be in `O` byte order.

#### Trait Implementations

##### `impl<O: ByteOrder> Add for Isize<O>`

- <span id="isize-add-type-output"></span>`type Output = Isize<O>`

- <span id="isize-add"></span>`fn add(self, rhs: Isize<O>) -> Isize<O>` — [`Isize`](#isize)

##### `impl<O: ByteOrder> AddAssign for Isize<O>`

- <span id="isize-addassign-add-assign"></span>`fn add_assign(&mut self, rhs: Isize<O>)` — [`Isize`](#isize)

##### `impl<O> AsMut for Isize<O>`

- <span id="isize-asmut-as-mut"></span>`fn as_mut(&mut self) -> &mut [u8; 8]`

##### `impl<O> AsRef for Isize<O>`

- <span id="isize-asref-as-ref"></span>`fn as_ref(&self) -> &[u8; 8]`

##### `impl<O: ByteOrder> Binary for Isize<O>`

- <span id="isize-binary-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> BitAnd for Isize<O>`

- <span id="isize-bitand-type-output"></span>`type Output = Isize<O>`

- <span id="isize-bitand"></span>`fn bitand(self, rhs: Isize<O>) -> Isize<O>` — [`Isize`](#isize)

##### `impl<O: ByteOrder> BitAndAssign for Isize<O>`

- <span id="isize-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, rhs: Isize<O>)` — [`Isize`](#isize)

##### `impl<O: ByteOrder> BitOr for Isize<O>`

- <span id="isize-bitor-type-output"></span>`type Output = Isize<O>`

- <span id="isize-bitor"></span>`fn bitor(self, rhs: Isize<O>) -> Isize<O>` — [`Isize`](#isize)

##### `impl<O: ByteOrder> BitOrAssign for Isize<O>`

- <span id="isize-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, rhs: Isize<O>)` — [`Isize`](#isize)

##### `impl<O: ByteOrder> BitXor for Isize<O>`

- <span id="isize-bitxor-type-output"></span>`type Output = Isize<O>`

- <span id="isize-bitxor"></span>`fn bitxor(self, rhs: Isize<O>) -> Isize<O>` — [`Isize`](#isize)

##### `impl<O: ByteOrder> BitXorAssign for Isize<O>`

- <span id="isize-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, rhs: Isize<O>)` — [`Isize`](#isize)

##### `impl<O: clone::Clone> Clone for Isize<O>`

- <span id="isize-clone"></span>`fn clone(&self) -> Isize<O>` — [`Isize`](#isize)

##### `impl<O: marker::Copy> Copy for Isize<O>`

##### `impl<O: ByteOrder> Debug for Isize<O>`

- <span id="isize-debug-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O> Default for Isize<O>`

- <span id="isize-default"></span>`fn default() -> Isize<O>` — [`Isize`](#isize)

##### `impl<O: ByteOrder> Display for Isize<O>`

- <span id="isize-display-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Div for Isize<O>`

- <span id="isize-div-type-output"></span>`type Output = Isize<O>`

- <span id="isize-div"></span>`fn div(self, rhs: Isize<O>) -> Isize<O>` — [`Isize`](#isize)

##### `impl<O: ByteOrder> DivAssign for Isize<O>`

- <span id="isize-divassign-div-assign"></span>`fn div_assign(&mut self, rhs: Isize<O>)` — [`Isize`](#isize)

##### `impl<O: cmp::Eq> Eq for Isize<O>`

##### `impl<O> FromBytes for Isize<O>`

##### `impl<O> FromZeros for Isize<O>`

##### `impl<O: hash::Hash> Hash for Isize<O>`

- <span id="isize-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<O> Immutable for Isize<O>`

##### `impl<O> IntoBytes for Isize<O>`

##### `impl<O> KnownLayout for Isize<O>`

- <span id="isize-knownlayout-type-pointermetadata"></span>`type PointerMetadata = ()`

##### `impl<O: ByteOrder> LowerHex for Isize<O>`

- <span id="isize-lowerhex-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Mul for Isize<O>`

- <span id="isize-mul-type-output"></span>`type Output = Isize<O>`

- <span id="isize-mul"></span>`fn mul(self, rhs: Isize<O>) -> Isize<O>` — [`Isize`](#isize)

##### `impl<O: ByteOrder> MulAssign for Isize<O>`

- <span id="isize-mulassign-mul-assign"></span>`fn mul_assign(&mut self, rhs: Isize<O>)` — [`Isize`](#isize)

##### `impl<O: ByteOrder> Neg for Isize<O>`

- <span id="isize-neg-type-output"></span>`type Output = Isize<O>`

- <span id="isize-neg"></span>`fn neg(self) -> Isize<O>` — [`Isize`](#isize)

##### `impl<O> Not for Isize<O>`

- <span id="isize-not-type-output"></span>`type Output = Isize<O>`

- <span id="isize-not"></span>`fn not(self) -> Isize<O>` — [`Isize`](#isize)

##### `impl<O: ByteOrder> Octal for Isize<O>`

- <span id="isize-octal-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Ord for Isize<O>`

- <span id="isize-ord-cmp"></span>`fn cmp(&self, other: &Self) -> Ordering`

##### `impl<O: cmp::PartialEq> PartialEq for Isize<O>`

- <span id="isize-partialeq-eq"></span>`fn eq(&self, other: &Isize<O>) -> bool` — [`Isize`](#isize)

##### `impl<O: ByteOrder> PartialOrd for Isize<O>`

- <span id="isize-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

##### `impl<O: ByteOrder> Rem for Isize<O>`

- <span id="isize-rem-type-output"></span>`type Output = Isize<O>`

- <span id="isize-rem"></span>`fn rem(self, rhs: Isize<O>) -> Isize<O>` — [`Isize`](#isize)

##### `impl<O: ByteOrder> RemAssign for Isize<O>`

- <span id="isize-remassign-rem-assign"></span>`fn rem_assign(&mut self, rhs: Isize<O>)` — [`Isize`](#isize)

##### `impl<O: ByteOrder> Shl for Isize<O>`

- <span id="isize-shl-type-output"></span>`type Output = Isize<O>`

- <span id="isize-shl"></span>`fn shl(self, rhs: Isize<O>) -> Isize<O>` — [`Isize`](#isize)

##### `impl<O: ByteOrder> ShlAssign for Isize<O>`

- <span id="isize-shlassign-shl-assign"></span>`fn shl_assign(&mut self, rhs: Isize<O>)` — [`Isize`](#isize)

##### `impl<O: ByteOrder> Shr for Isize<O>`

- <span id="isize-shr-type-output"></span>`type Output = Isize<O>`

- <span id="isize-shr"></span>`fn shr(self, rhs: Isize<O>) -> Isize<O>` — [`Isize`](#isize)

##### `impl<O: ByteOrder> ShrAssign for Isize<O>`

- <span id="isize-shrassign-shr-assign"></span>`fn shr_assign(&mut self, rhs: Isize<O>)` — [`Isize`](#isize)

##### `impl<O> StructuralPartialEq for Isize<O>`

##### `impl<O: ByteOrder> Sub for Isize<O>`

- <span id="isize-sub-type-output"></span>`type Output = Isize<O>`

- <span id="isize-sub"></span>`fn sub(self, rhs: Isize<O>) -> Isize<O>` — [`Isize`](#isize)

##### `impl<O: ByteOrder> SubAssign for Isize<O>`

- <span id="isize-subassign-sub-assign"></span>`fn sub_assign(&mut self, rhs: Isize<O>)` — [`Isize`](#isize)

##### `impl ToString for Isize<O>`

- <span id="isize-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<O> TryFromBytes for Isize<O>`

##### `impl<O> Unaligned for Isize<O>`

##### `impl<O: ByteOrder> UpperHex for Isize<O>`

- <span id="isize-upperhex-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

### `F32<O>`

```rust
struct F32<O>([u8; 4], PhantomData<O>);
```

A 32-bit floating point number stored in a given byte order.

`F32` is like the native `f32` type with
two major differences: First, it has no alignment requirement (its alignment is 1).
Second, the endianness of its memory layout is given by the type parameter `O`,
which can be any type which implements [`ByteOrder`](#byteorder). In particular, this refers
to [`BigEndian`](#bigendian), [`LittleEndian`](#littleendian), [`NativeEndian`](#nativeendian), and [`NetworkEndian`](#networkendian).

An `F32` can be constructed using
the `new` method, and its contained value can be obtained as a native
`f32` using the `get` method, or updated in place with
the `set` method. In all cases, if the endianness `O` is not the same as the
endianness of the current platform, an endianness swap will be performed in
order to uphold the invariants that a) the layout of `F32`
has endianness `O` and that, b) the layout of `f32` has
the platform's native endianness.

`F32` implements [`FromBytes`](#frombytes), [`IntoBytes`](#intobytes), and [`Unaligned`](#unaligned),
making it useful for parsing and serialization. See the module documentation for an
example of how it can be used for parsing UDP packets.







#### Implementations

- <span id="f32-const-zero"></span>`const ZERO: F32<O>`

- <span id="f32-from-bytes"></span>`const fn from_bytes(bytes: [u8; 4]) -> F32<O>` — [`F32`](#f32)

  Constructs a new value from bytes which are already in `O` byte

  order.

- <span id="f32-to-bytes"></span>`const fn to_bytes(self) -> [u8; 4]`

  Extracts the bytes of `self` without swapping the byte order.

  

  The returned bytes will be in `O` byte order.

#### Trait Implementations

##### `impl<O: ByteOrder> Add for F32<O>`

- <span id="f32-add-type-output"></span>`type Output = F32<O>`

- <span id="f32-add"></span>`fn add(self, rhs: F32<O>) -> F32<O>` — [`F32`](#f32)

##### `impl<O: ByteOrder> AddAssign for F32<O>`

- <span id="f32-addassign-add-assign"></span>`fn add_assign(&mut self, rhs: F32<O>)` — [`F32`](#f32)

##### `impl<O> AsMut for F32<O>`

- <span id="f32-asmut-as-mut"></span>`fn as_mut(&mut self) -> &mut [u8; 4]`

##### `impl<O> AsRef for F32<O>`

- <span id="f32-asref-as-ref"></span>`fn as_ref(&self) -> &[u8; 4]`

##### `impl<O: clone::Clone> Clone for F32<O>`

- <span id="f32-clone"></span>`fn clone(&self) -> F32<O>` — [`F32`](#f32)

##### `impl<O: marker::Copy> Copy for F32<O>`

##### `impl<O: ByteOrder> Debug for F32<O>`

- <span id="f32-debug-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O> Default for F32<O>`

- <span id="f32-default"></span>`fn default() -> F32<O>` — [`F32`](#f32)

##### `impl<O: ByteOrder> Display for F32<O>`

- <span id="f32-display-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Div for F32<O>`

- <span id="f32-div-type-output"></span>`type Output = F32<O>`

- <span id="f32-div"></span>`fn div(self, rhs: F32<O>) -> F32<O>` — [`F32`](#f32)

##### `impl<O: ByteOrder> DivAssign for F32<O>`

- <span id="f32-divassign-div-assign"></span>`fn div_assign(&mut self, rhs: F32<O>)` — [`F32`](#f32)

##### `impl<O: cmp::Eq> Eq for F32<O>`

##### `impl<O> FromBytes for F32<O>`

##### `impl<O> FromZeros for F32<O>`

##### `impl<O: hash::Hash> Hash for F32<O>`

- <span id="f32-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<O> Immutable for F32<O>`

##### `impl<O> IntoBytes for F32<O>`

##### `impl<O> KnownLayout for F32<O>`

- <span id="f32-knownlayout-type-pointermetadata"></span>`type PointerMetadata = ()`

##### `impl<O: ByteOrder> Mul for F32<O>`

- <span id="f32-mul-type-output"></span>`type Output = F32<O>`

- <span id="f32-mul"></span>`fn mul(self, rhs: F32<O>) -> F32<O>` — [`F32`](#f32)

##### `impl<O: ByteOrder> MulAssign for F32<O>`

- <span id="f32-mulassign-mul-assign"></span>`fn mul_assign(&mut self, rhs: F32<O>)` — [`F32`](#f32)

##### `impl<O: ByteOrder> Neg for F32<O>`

- <span id="f32-neg-type-output"></span>`type Output = F32<O>`

- <span id="f32-neg"></span>`fn neg(self) -> F32<O>` — [`F32`](#f32)

##### `impl<O: cmp::PartialEq> PartialEq for F32<O>`

- <span id="f32-partialeq-eq"></span>`fn eq(&self, other: &F32<O>) -> bool` — [`F32`](#f32)

##### `impl<O: ByteOrder> PartialOrd for F32<O>`

- <span id="f32-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

##### `impl<O: ByteOrder> Rem for F32<O>`

- <span id="f32-rem-type-output"></span>`type Output = F32<O>`

- <span id="f32-rem"></span>`fn rem(self, rhs: F32<O>) -> F32<O>` — [`F32`](#f32)

##### `impl<O: ByteOrder> RemAssign for F32<O>`

- <span id="f32-remassign-rem-assign"></span>`fn rem_assign(&mut self, rhs: F32<O>)` — [`F32`](#f32)

##### `impl<O> StructuralPartialEq for F32<O>`

##### `impl<O: ByteOrder> Sub for F32<O>`

- <span id="f32-sub-type-output"></span>`type Output = F32<O>`

- <span id="f32-sub"></span>`fn sub(self, rhs: F32<O>) -> F32<O>` — [`F32`](#f32)

##### `impl<O: ByteOrder> SubAssign for F32<O>`

- <span id="f32-subassign-sub-assign"></span>`fn sub_assign(&mut self, rhs: F32<O>)` — [`F32`](#f32)

##### `impl ToString for F32<O>`

- <span id="f32-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<O> TryFromBytes for F32<O>`

##### `impl<O> Unaligned for F32<O>`

### `F64<O>`

```rust
struct F64<O>([u8; 8], PhantomData<O>);
```

A 64-bit floating point number stored in a given byte order.

`F64` is like the native `f64` type with
two major differences: First, it has no alignment requirement (its alignment is 1).
Second, the endianness of its memory layout is given by the type parameter `O`,
which can be any type which implements [`ByteOrder`](#byteorder). In particular, this refers
to [`BigEndian`](#bigendian), [`LittleEndian`](#littleendian), [`NativeEndian`](#nativeendian), and [`NetworkEndian`](#networkendian).

An `F64` can be constructed using
the `new` method, and its contained value can be obtained as a native
`f64` using the `get` method, or updated in place with
the `set` method. In all cases, if the endianness `O` is not the same as the
endianness of the current platform, an endianness swap will be performed in
order to uphold the invariants that a) the layout of `F64`
has endianness `O` and that, b) the layout of `f64` has
the platform's native endianness.

`F64` implements [`FromBytes`](#frombytes), [`IntoBytes`](#intobytes), and [`Unaligned`](#unaligned),
making it useful for parsing and serialization. See the module documentation for an
example of how it can be used for parsing UDP packets.







#### Implementations

- <span id="f64-const-zero"></span>`const ZERO: F64<O>`

- <span id="f64-from-bytes"></span>`const fn from_bytes(bytes: [u8; 8]) -> F64<O>` — [`F64`](#f64)

  Constructs a new value from bytes which are already in `O` byte

  order.

- <span id="f64-to-bytes"></span>`const fn to_bytes(self) -> [u8; 8]`

  Extracts the bytes of `self` without swapping the byte order.

  

  The returned bytes will be in `O` byte order.

#### Trait Implementations

##### `impl<O: ByteOrder> Add for F64<O>`

- <span id="f64-add-type-output"></span>`type Output = F64<O>`

- <span id="f64-add"></span>`fn add(self, rhs: F64<O>) -> F64<O>` — [`F64`](#f64)

##### `impl<O: ByteOrder> AddAssign for F64<O>`

- <span id="f64-addassign-add-assign"></span>`fn add_assign(&mut self, rhs: F64<O>)` — [`F64`](#f64)

##### `impl<O> AsMut for F64<O>`

- <span id="f64-asmut-as-mut"></span>`fn as_mut(&mut self) -> &mut [u8; 8]`

##### `impl<O> AsRef for F64<O>`

- <span id="f64-asref-as-ref"></span>`fn as_ref(&self) -> &[u8; 8]`

##### `impl<O: clone::Clone> Clone for F64<O>`

- <span id="f64-clone"></span>`fn clone(&self) -> F64<O>` — [`F64`](#f64)

##### `impl<O: marker::Copy> Copy for F64<O>`

##### `impl<O: ByteOrder> Debug for F64<O>`

- <span id="f64-debug-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O> Default for F64<O>`

- <span id="f64-default"></span>`fn default() -> F64<O>` — [`F64`](#f64)

##### `impl<O: ByteOrder> Display for F64<O>`

- <span id="f64-display-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<O: ByteOrder> Div for F64<O>`

- <span id="f64-div-type-output"></span>`type Output = F64<O>`

- <span id="f64-div"></span>`fn div(self, rhs: F64<O>) -> F64<O>` — [`F64`](#f64)

##### `impl<O: ByteOrder> DivAssign for F64<O>`

- <span id="f64-divassign-div-assign"></span>`fn div_assign(&mut self, rhs: F64<O>)` — [`F64`](#f64)

##### `impl<O: cmp::Eq> Eq for F64<O>`

##### `impl<O> FromBytes for F64<O>`

##### `impl<O> FromZeros for F64<O>`

##### `impl<O: hash::Hash> Hash for F64<O>`

- <span id="f64-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<O> Immutable for F64<O>`

##### `impl<O> IntoBytes for F64<O>`

##### `impl<O> KnownLayout for F64<O>`

- <span id="f64-knownlayout-type-pointermetadata"></span>`type PointerMetadata = ()`

##### `impl<O: ByteOrder> Mul for F64<O>`

- <span id="f64-mul-type-output"></span>`type Output = F64<O>`

- <span id="f64-mul"></span>`fn mul(self, rhs: F64<O>) -> F64<O>` — [`F64`](#f64)

##### `impl<O: ByteOrder> MulAssign for F64<O>`

- <span id="f64-mulassign-mul-assign"></span>`fn mul_assign(&mut self, rhs: F64<O>)` — [`F64`](#f64)

##### `impl<O: ByteOrder> Neg for F64<O>`

- <span id="f64-neg-type-output"></span>`type Output = F64<O>`

- <span id="f64-neg"></span>`fn neg(self) -> F64<O>` — [`F64`](#f64)

##### `impl<O: cmp::PartialEq> PartialEq for F64<O>`

- <span id="f64-partialeq-eq"></span>`fn eq(&self, other: &F64<O>) -> bool` — [`F64`](#f64)

##### `impl<O: ByteOrder> PartialOrd for F64<O>`

- <span id="f64-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

##### `impl<O: ByteOrder> Rem for F64<O>`

- <span id="f64-rem-type-output"></span>`type Output = F64<O>`

- <span id="f64-rem"></span>`fn rem(self, rhs: F64<O>) -> F64<O>` — [`F64`](#f64)

##### `impl<O: ByteOrder> RemAssign for F64<O>`

- <span id="f64-remassign-rem-assign"></span>`fn rem_assign(&mut self, rhs: F64<O>)` — [`F64`](#f64)

##### `impl<O> StructuralPartialEq for F64<O>`

##### `impl<O: ByteOrder> Sub for F64<O>`

- <span id="f64-sub-type-output"></span>`type Output = F64<O>`

- <span id="f64-sub"></span>`fn sub(self, rhs: F64<O>) -> F64<O>` — [`F64`](#f64)

##### `impl<O: ByteOrder> SubAssign for F64<O>`

- <span id="f64-subassign-sub-assign"></span>`fn sub_assign(&mut self, rhs: F64<O>)` — [`F64`](#f64)

##### `impl ToString for F64<O>`

- <span id="f64-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<O> TryFromBytes for F64<O>`

##### `impl<O> Unaligned for F64<O>`

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

- <span id="alignmenterror-with-src"></span>`fn with_src<NewSrc>(self, new_src: NewSrc) -> AlignmentError<NewSrc, Dst>` — [`AlignmentError`](#alignmenterror)

- <span id="alignmenterror-map-src"></span>`fn map_src<NewSrc>(self, f: impl FnOnce(Src) -> NewSrc) -> AlignmentError<NewSrc, Dst>` — [`AlignmentError`](#alignmenterror)

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

- <span id="alignmenterror-into"></span>`fn into<S, V>(self) -> ConvertError<Self, S, V>` — [`ConvertError`](#converterror)

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

- <span id="sizeerror-with-src"></span>`fn with_src<NewSrc>(self, new_src: NewSrc) -> SizeError<NewSrc, Dst>` — [`SizeError`](#sizeerror)

  Sets the source value associated with the conversion error.

- <span id="sizeerror-map-src"></span>`fn map_src<NewSrc>(self, f: impl FnOnce(Src) -> NewSrc) -> SizeError<NewSrc, Dst>` — [`SizeError`](#sizeerror)

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

- <span id="sizeerror-with-dst"></span>`fn with_dst<NewDst: ?Sized>(self) -> SizeError<Src, NewDst>` — [`SizeError`](#sizeerror)

  Sets the destination type associated with the conversion error.

- <span id="sizeerror-into"></span>`fn into<A, V>(self) -> ConvertError<A, Self, V>` — [`ConvertError`](#converterror)

  Converts the error into a general [`ConvertError`](#converterror).

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

- <span id="validityerror-map-src"></span>`fn map_src<NewSrc>(self, f: impl FnOnce(Src) -> NewSrc) -> ValidityError<NewSrc, Dst>` — [`ValidityError`](#validityerror)

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

- <span id="validityerror-into"></span>`fn into<A, S>(self) -> ConvertError<A, S, Self>` — [`ConvertError`](#converterror)

  Converts the error into a general [`ConvertError`](#converterror).

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
[`AllocError`](#allocerror) type once it is stabilized. When that happens, this type will
be replaced by a type alias to the standard library type. We do not intend
to treat this as a breaking change; users who wish to avoid breakage should
avoid writing code which assumes that this is *not* such an alias. For
example, implementing the same trait for both types will result in an impl
conflict once this type is an alias.


#### Trait Implementations

##### `impl Clone for AllocError`

- <span id="allocerror-clone"></span>`fn clone(&self) -> AllocError` — [`AllocError`](#allocerror)

##### `impl Copy for AllocError`

##### `impl Debug for AllocError`

- <span id="allocerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for AllocError`

##### `impl PartialEq for AllocError`

- <span id="allocerror-partialeq-eq"></span>`fn eq(&self, other: &AllocError) -> bool` — [`AllocError`](#allocerror)

##### `impl StructuralPartialEq for AllocError`

### `Unalign<T>`

```rust
struct Unalign<T>(T);
```

A type with no alignment requirement.

An `Unalign` wraps a `T`, removing any alignment requirement. `Unalign<T>`
has the same size and bit validity as `T`, but not necessarily the same
alignment [or ABI]. This is useful if a type with an alignment requirement
needs to be read from a chunk of memory which provides no alignment
guarantees.

Since `Unalign` has no alignment requirement, the inner `T` may not be
properly aligned in memory. There are five ways to access the inner `T`:
- by value, using `get` or `into_inner`
- by reference inside of a callback, using `update`
- fallibly by reference, using `try_deref` or `try_deref_mut`; these can
  fail if the `Unalign` does not satisfy `T`'s alignment requirement at
  runtime
- unsafely by reference, using `deref_unchecked` or
  `deref_mut_unchecked`; it is the caller's responsibility to ensure that
  the `Unalign` satisfies `T`'s alignment requirement
- (where `T: Unaligned`) infallibly by reference, using `Deref::deref` or
  `DerefMut::deref_mut`








# Example

In this example, we need `EthernetFrame` to have no alignment requirement -
and thus implement [`Unaligned`](#unaligned). `EtherType` is `#[repr(u16)]` and so
cannot implement `Unaligned`. We use `Unalign` to relax `EtherType`'s
alignment requirement so that `EthernetFrame` has no alignment requirement
and can implement `Unaligned`.

```rust
use zerocopy::*;
use zerocopy_derive::*;
#[derive(FromBytes, KnownLayout, Immutable, Unaligned)] #[repr(C)] struct Mac([u8; 6]);

#[derive(PartialEq, Copy, Clone, Debug)]
#[derive(TryFromBytes, KnownLayout, Immutable)]
#[repr(u16)]
enum EtherType {
    Ipv4 = 0x0800u16.to_be(),
    Arp = 0x0806u16.to_be(),
    Ipv6 = 0x86DDu16.to_be(),
    /*
    ...
    */
}

#[derive(TryFromBytes, KnownLayout, Immutable, Unaligned)]
#[repr(C)]
struct EthernetFrame {
    src: Mac,
    dst: Mac,
    ethertype: Unalign<EtherType>,
    payload: [u8],
}

let bytes = &[
    0, 1, 2, 3, 4, 5,
    6, 7, 8, 9, 10, 11,
    /*
    ...
    */
    0x86, 0xDD,            // EtherType
    0xDE, 0xAD, 0xBE, 0xEF // Payload
][..];

// PANICS: Guaranteed not to panic because `bytes` is of the right
// length, has the right contents, and `EthernetFrame` has no
// alignment requirement.
let packet = EthernetFrame::try_ref_from_bytes(&bytes).unwrap();

assert_eq!(packet.ethertype.get(), EtherType::Ipv6);
assert_eq!(packet.payload, [0xDE, 0xAD, 0xBE, 0xEF]);
```

# Safety

`Unalign<T>` is guaranteed to have the same size and bit validity as `T`,
and to have `UnsafeCell`s covering the same byte ranges as `T`.
`Unalign<T>` is guaranteed to have alignment 1.

#### Implementations

- <span id="unalign-new"></span>`const fn new(val: T) -> Unalign<T>` — [`Unalign`](#unalign)

  Constructs a new `Unalign`.

- <span id="unalign-into-inner"></span>`const fn into_inner(self) -> T`

  Consumes `self`, returning the inner `T`.

- <span id="unalign-try-deref"></span>`fn try_deref(&self) -> Result<&T, AlignmentError<&Self, T>>` — [`AlignmentError`](#alignmenterror)

  Attempts to return a reference to the wrapped `T`, failing if `self` is

  not properly aligned.

  

  If `self` does not satisfy `align_of::<T>()`, then `try_deref` returns

  `Err`.

  

  If `T: Unaligned`, then `Unalign<T>` implements [`Deref`](../cpp_demangle/index.md), and callers

  may prefer `Deref::deref`, which is infallible.

- <span id="unalign-try-deref-mut"></span>`fn try_deref_mut(&mut self) -> Result<&mut T, AlignmentError<&mut Self, T>>` — [`AlignmentError`](#alignmenterror)

  Attempts to return a mutable reference to the wrapped `T`, failing if

  `self` is not properly aligned.

  

  If `self` does not satisfy `align_of::<T>()`, then `try_deref` returns

  `Err`.

  

  If `T: Unaligned`, then `Unalign<T>` implements `DerefMut`, and

  callers may prefer `DerefMut::deref_mut`, which is infallible.

- <span id="unalign-deref-unchecked"></span>`const unsafe fn deref_unchecked(&self) -> &T`

  Returns a reference to the wrapped `T` without checking alignment.

  

  If `T: Unaligned`, then `Unalign<T>` implements[ `Deref`], and callers

  may prefer `Deref::deref`, which is safe.

  

  # Safety

  

  The caller must guarantee that `self` satisfies `align_of::<T>()`.

- <span id="unalign-deref-mut-unchecked"></span>`unsafe fn deref_mut_unchecked(&mut self) -> &mut T`

  Returns a mutable reference to the wrapped `T` without checking

  alignment.

  

  If `T: Unaligned`, then `Unalign<T>` implements[ `DerefMut`], and

  callers may prefer `DerefMut::deref_mut`, which is safe.

  

  # Safety

  

  The caller must guarantee that `self` satisfies `align_of::<T>()`.

- <span id="unalign-get-ptr"></span>`const fn get_ptr(&self) -> *const T`

  Gets an unaligned raw pointer to the inner `T`.

  

  # Safety

  

  The returned raw pointer is not necessarily aligned to

  `align_of::<T>()`. Most functions which operate on raw pointers require

  those pointers to be aligned, so calling those functions with the result

  of `get_ptr` will result in undefined behavior if alignment is not

  guaranteed using some out-of-band mechanism. In general, the only

  functions which are safe to call with this pointer are those which are

  explicitly documented as being sound to use with an unaligned pointer,

  such as `read_unaligned`.

  

  Even if the caller is permitted to mutate `self` (e.g. they have

  ownership or a mutable borrow), it is not guaranteed to be sound to

  write through the returned pointer. If writing is required, prefer

  `get_mut_ptr` instead.

  

- <span id="unalign-get-mut-ptr"></span>`fn get_mut_ptr(&mut self) -> *mut T`

  Gets an unaligned mutable raw pointer to the inner `T`.

  

  # Safety

  

  The returned raw pointer is not necessarily aligned to

  `align_of::<T>()`. Most functions which operate on raw pointers require

  those pointers to be aligned, so calling those functions with the result

  of `get_ptr` will result in undefined behavior if alignment is not

  guaranteed using some out-of-band mechanism. In general, the only

  functions which are safe to call with this pointer are those which are

  explicitly documented as being sound to use with an unaligned pointer,

  such as `read_unaligned`.

- <span id="unalign-set"></span>`fn set(&mut self, t: T)`

  Sets the inner `T`, dropping the previous value.

- <span id="unalign-update"></span>`fn update<O, F: FnOnce(&mut T) -> O>(&mut self, f: F) -> O`

  Updates the inner `T` by calling a function on it.

  

  If `T: Unaligned`, then `Unalign<T>` implements `DerefMut`, and that

  impl should be preferred over this method when performing updates, as it

  will usually be faster and more ergonomic.

  

  For large types, this method may be expensive, as it requires copying

  `2 * size_of::<T>()` bytes. \[1\]

  

  \[1\] Since the inner `T` may not be aligned, it would not be sound to

  invoke `f` on it directly. Instead, `update` moves it into a

  properly-aligned location in the local stack frame, calls `f` on it, and

  then moves it back to its original location in `self`.

#### Trait Implementations

##### `impl<T: Copy> Clone for Unalign<T>`

- <span id="unalign-clone"></span>`fn clone(&self) -> Unalign<T>` — [`Unalign`](#unalign)

##### `impl<T: marker::Copy> Copy for Unalign<T>`

##### `impl<T: Unaligned + Debug> Debug for Unalign<T>`

- <span id="unalign-debug-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<T: default::Default> Default for Unalign<T>`

- <span id="unalign-default"></span>`fn default() -> Unalign<T>` — [`Unalign`](#unalign)

##### `impl<T: Unaligned> Deref for Unalign<T>`

- <span id="unalign-deref-type-target"></span>`type Target = T`

- <span id="unalign-deref"></span>`fn deref(&self) -> &T`

##### `impl<T: Unaligned> DerefMut for Unalign<T>`

- <span id="unalign-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut T`

##### `impl<T: Unaligned + Display> Display for Unalign<T>`

- <span id="unalign-display-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result`

##### `impl<T: Unaligned + Eq> Eq for Unalign<T>`

##### `impl<T> FromBytes for Unalign<T>`

##### `impl<T> FromZeros for Unalign<T>`

##### `impl<T: Unaligned + Hash> Hash for Unalign<T>`

- <span id="unalign-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl<T> Immutable for Unalign<T>`

##### `impl<T> IntoBytes for Unalign<T>`

##### `impl<T> KnownLayout for Unalign<T>`

- <span id="unalign-knownlayout-type-pointermetadata"></span>`type PointerMetadata = ()`

##### `impl<T: Unaligned + Ord> Ord for Unalign<T>`

- <span id="unalign-ord-cmp"></span>`fn cmp(&self, other: &Unalign<T>) -> Ordering` — [`Unalign`](#unalign)

##### `impl<T: Unaligned + PartialEq> PartialEq for Unalign<T>`

- <span id="unalign-partialeq-eq"></span>`fn eq(&self, other: &Unalign<T>) -> bool` — [`Unalign`](#unalign)

##### `impl<T: Unaligned + PartialOrd> PartialOrd for Unalign<T>`

- <span id="unalign-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Unalign<T>) -> Option<Ordering>` — [`Unalign`](#unalign)

##### `impl<T> Receiver for Unalign<T>`

- <span id="unalign-receiver-type-target"></span>`type Target = T`

##### `impl<T> ToString for Unalign<T>`

- <span id="unalign-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<T> TryFromBytes for Unalign<T>`

##### `impl<T> Unaligned for Unalign<T>`

## Enums

### `BigEndian`

```rust
enum BigEndian {
}
```

Big-endian byte order.

See [`ByteOrder`](#byteorder) for more details.

#### Trait Implementations

##### `impl ByteOrder for BigEndian`

##### `impl Clone for BigEndian`

- <span id="bigendian-clone"></span>`fn clone(&self) -> BigEndian` — [`BigEndian`](#bigendian)

##### `impl Copy for BigEndian`

##### `impl Debug for BigEndian`

- <span id="bigendian-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for BigEndian`

- <span id="bigendian-display-fmt"></span>`fn fmt(&self, _: &mut Formatter<'_>) -> fmt::Result`

##### `impl Eq for BigEndian`

##### `impl Hash for BigEndian`

- <span id="bigendian-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for BigEndian`

- <span id="bigendian-ord-cmp"></span>`fn cmp(&self, other: &BigEndian) -> cmp::Ordering` — [`BigEndian`](#bigendian)

##### `impl PartialEq for BigEndian`

- <span id="bigendian-partialeq-eq"></span>`fn eq(&self, other: &BigEndian) -> bool` — [`BigEndian`](#bigendian)

##### `impl PartialOrd for BigEndian`

- <span id="bigendian-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &BigEndian) -> option::Option<cmp::Ordering>` — [`BigEndian`](#bigendian)

##### `impl Sealed for super::BigEndian`

##### `impl StructuralPartialEq for BigEndian`

##### `impl ToString for BigEndian`

- <span id="bigendian-tostring-to-string"></span>`fn to_string(&self) -> String`

### `LittleEndian`

```rust
enum LittleEndian {
}
```

Little-endian byte order.

See [`ByteOrder`](#byteorder) for more details.

#### Trait Implementations

##### `impl ByteOrder for LittleEndian`

##### `impl Clone for LittleEndian`

- <span id="littleendian-clone"></span>`fn clone(&self) -> LittleEndian` — [`LittleEndian`](#littleendian)

##### `impl Copy for LittleEndian`

##### `impl Debug for LittleEndian`

- <span id="littleendian-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for LittleEndian`

- <span id="littleendian-display-fmt"></span>`fn fmt(&self, _: &mut Formatter<'_>) -> fmt::Result`

##### `impl Eq for LittleEndian`

##### `impl Hash for LittleEndian`

- <span id="littleendian-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for LittleEndian`

- <span id="littleendian-ord-cmp"></span>`fn cmp(&self, other: &LittleEndian) -> cmp::Ordering` — [`LittleEndian`](#littleendian)

##### `impl PartialEq for LittleEndian`

- <span id="littleendian-partialeq-eq"></span>`fn eq(&self, other: &LittleEndian) -> bool` — [`LittleEndian`](#littleendian)

##### `impl PartialOrd for LittleEndian`

- <span id="littleendian-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &LittleEndian) -> option::Option<cmp::Ordering>` — [`LittleEndian`](#littleendian)

##### `impl Sealed for super::LittleEndian`

##### `impl StructuralPartialEq for LittleEndian`

##### `impl ToString for LittleEndian`

- <span id="littleendian-tostring-to-string"></span>`fn to_string(&self) -> String`

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
- [`AlignmentError`](#alignmenterror): the conversion source was improperly aligned
- [`SizeError`](#sizeerror): the conversion source was of incorrect size
- [`ValidityError`](#validityerror): the conversion source contained invalid data

However, not all conversions produce all errors. For instance,
`FromBytes::ref_from_bytes` may fail due to alignment or size issues, but
not validity issues. This generic error type captures these
(im)possibilities via parameterization: `A` is parameterized with
[`AlignmentError`](#alignmenterror), `S` is parameterized with [`SizeError`](#sizeerror), and `V` is
parameterized with [`Infallible`](../hashbrown/index.md).

Zerocopy never uses this type directly in its API. Rather, we provide three
pre-parameterized aliases:
- [`CastError`](#casterror): the error type of reference conversions
- [`TryCastError`](#trycasterror): the error type of fallible reference conversions
- [`TryReadError`](#tryreaderror): the error type of fallible read conversions

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

- <span id="converterror-with-src"></span>`fn with_src<NewSrc>(self, new_src: NewSrc) -> CastError<NewSrc, Dst>` — [`CastError`](#casterror)

  Sets the source value associated with the conversion error.

- <span id="converterror-map-src"></span>`fn map_src<NewSrc>(self, f: impl FnOnce(Src) -> NewSrc) -> CastError<NewSrc, Dst>` — [`CastError`](#casterror)

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

- <span id="converterror-into"></span>`fn into(self) -> TryCastError<Src, Dst>` — [`TryCastError`](#trycasterror)

  Converts the error into a general [`ConvertError`](#converterror).

#### Trait Implementations

##### `impl<A: clone::Clone, S: clone::Clone, V: clone::Clone> Clone for ConvertError<A, S, V>`

- <span id="converterror-clone"></span>`fn clone(&self) -> ConvertError<A, S, V>` — [`ConvertError`](#converterror)

##### `impl<A: fmt::Debug, S: fmt::Debug, V: fmt::Debug> Debug for ConvertError<A, S, V>`

- <span id="converterror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<A: fmt::Display, S: fmt::Display, V: fmt::Display> Display for ConvertError<A, S, V>`

- <span id="converterror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<A: cmp::Eq, S: cmp::Eq, V: cmp::Eq> Eq for ConvertError<A, S, V>`

##### `impl<A, S, V> Error for ConvertError<A, S, V>`

##### `impl<A: cmp::PartialEq, S: cmp::PartialEq, V: cmp::PartialEq> PartialEq for ConvertError<A, S, V>`

- <span id="converterror-partialeq-eq"></span>`fn eq(&self, other: &ConvertError<A, S, V>) -> bool` — [`ConvertError`](#converterror)

##### `impl<A, S, V> StructuralPartialEq for ConvertError<A, S, V>`

##### `impl ToString for ConvertError<A, S, V>`

- <span id="converterror-tostring-to-string"></span>`fn to_string(&self) -> String`

## Traits

### `SplitAt`

```rust
trait SplitAt: KnownLayout<PointerMetadata = usize> { ... }
```

Types that can be split in two.

This trait generalizes Rust's existing support for splitting slices to
support slices and slice-based dynamically-sized types ("slice DSTs").

# Implementation

**Do not implement this trait yourself!** Instead, use
[`#[derive(SplitAt)]`][`derive`](../clap_builder/derive/index.md); e.g.:

```rust
use zerocopy_derive::{SplitAt, KnownLayout};
#[derive(SplitAt, KnownLayout)]
#[repr(C)]
struct MyStruct<T: ?Sized> {
/*
    ...,
*/
    // `SplitAt` types must have at least one field.
    field: T,
}
```

This derive performs a sophisticated, compile-time safety analysis to
determine whether a type is `SplitAt`.

# Safety

This trait does not convey any safety guarantees to code outside this crate.

You must not rely on the `#[doc(hidden)]` internals of `SplitAt`. Future
releases of zerocopy may make backwards-breaking changes to these items,
including changes that only affect soundness, which may cause code which
uses those items to silently become unsound.


#### Associated Types

- `type Elem`

#### Provided Methods

- `fn split_at_unchecked(&self, l_len: usize) -> Split<&Self>`

  Unsafely splits `self` in two.

- `fn split_at(&self, l_len: usize) -> Option<Split<&Self>>`

  Attempts to split `self` in two.

- `fn split_at_mut_unchecked(&mut self, l_len: usize) -> Split<&mut Self>`

  Unsafely splits `self` in two.

- `fn split_at_mut(&mut self, l_len: usize) -> Option<Split<&mut Self>>`

  Attempts to split `self` in two.

#### Implementors

- `[T]`

### `KnownLayout`

```rust
trait KnownLayout { ... }
```

Indicates that zerocopy can reason about certain aspects of a type's layout.

This trait is required by many of zerocopy's APIs. It supports sized types,
slices, and [slice DSTs](#dynamically-sized-types).

# Implementation

**Do not implement this trait yourself!** Instead, use
[`#[derive(KnownLayout)]`][`derive`](../clap_builder/derive/index.md); e.g.:

```rust
use zerocopy_derive::KnownLayout;
#[derive(KnownLayout)]
struct MyStruct {
/*
    ...
*/
}

#[derive(KnownLayout)]
enum MyEnum {
/*
    ...
*/
}

#[derive(KnownLayout)]
union MyUnion {
  variant: u8,
/*
    ...
*/
}
```

This derive performs a sophisticated analysis to deduce the layout
characteristics of types. You **must** implement this trait via the derive.

# Dynamically-sized types

`KnownLayout` supports slice-based dynamically sized types ("slice DSTs").

A slice DST is a type whose trailing field is either a slice or another
slice DST, rather than a type with fixed size. For example:

```rust
#[repr(C)]
struct PacketHeader {
/*
    ...
*/
}

#[repr(C)]
struct Packet {
    header: PacketHeader,
    body: [u8],
}
```

It can be useful to think of slice DSTs as a generalization of slices - in
other words, a normal slice is just the special case of a slice DST with
zero leading fields. In particular:
- Like slices, slice DSTs can have different lengths at runtime
- Like slices, slice DSTs cannot be passed by-value, but only by reference
  or via other indirection such as `Box`
- Like slices, a reference (or `Box`, or other pointer type) to a slice DST
  encodes the number of elements in the trailing slice field

## Slice DST layout

Just like other composite Rust types, the layout of a slice DST is not
well-defined unless it is specified using an explicit `#[repr(...)]`
attribute such as `#[repr(C)]`. [Other representations are
supported][reprs], but in this section, we'll use `#[repr(C)]` as our
example.

A `#[repr(C)]` slice DST is laid out [just like sized `#[repr(C)]`
types][repr-c-structs], but the presence of a variable-length field
introduces the possibility of *dynamic padding*. In particular, it may be
necessary to add trailing padding *after* the trailing slice field in order
to satisfy the outer type's alignment, and the amount of padding required
may be a function of the length of the trailing slice field. This is just a
natural consequence of the normal `#[repr(C)]` rules applied to slice DSTs,
but it can result in surprising behavior. For example, consider the
following type:

```rust
#[repr(C)]
struct Foo {
    a: u32,
    b: u8,
    z: [u16],
}
```

Assuming that `u32` has alignment 4 (this is not true on all platforms),
then `Foo` has alignment 4 as well. Here is the smallest possible value for
`Foo`:

```text
byte offset | 01234567
      field | aaaab---
                   ><
```

In this value, `z` has length 0. Abiding by `#[repr(C)]`, the lowest offset
that we can place `z` at is 5, but since `z` has alignment 2, we need to
round up to offset 6. This means that there is one byte of padding between
`b` and `z`, then 0 bytes of `z` itself (denoted `><` in this diagram), and
then two bytes of padding after `z` in order to satisfy the overall
alignment of `Foo`. The size of this instance is 8 bytes.

What about if `z` has length 1?

```text
byte offset | 01234567
      field | aaaab-zz
```

In this instance, `z` has length 1, and thus takes up 2 bytes. That means
that we no longer need padding after `z` in order to satisfy `Foo`'s
alignment. We've now seen two different values of `Foo` with two different
lengths of `z`, but they both have the same size - 8 bytes.

What about if `z` has length 2?

```text
byte offset | 012345678901
      field | aaaab-zzzz--
```

Now `z` has length 2, and thus takes up 4 bytes. This brings our un-padded
size to 10, and so we now need another 2 bytes of padding after `z` to
satisfy `Foo`'s alignment.

Again, all of this is just a logical consequence of the `#[repr(C)]` rules
applied to slice DSTs, but it can be surprising that the amount of trailing
padding becomes a function of the trailing slice field's length, and thus
can only be computed at runtime.


## What is a valid size?

There are two places in zerocopy's API that we refer to "a valid size" of a
type. In normal casts or conversions, where the source is a byte slice, we
need to know whether the source byte slice is a valid size of the
destination type. In prefix or suffix casts, we need to know whether *there
exists* a valid size of the destination type which fits in the source byte
slice and, if so, what the largest such size is.

As outlined above, a slice DST's size is defined by the number of elements
in its trailing slice field. However, there is not necessarily a 1-to-1
mapping between trailing slice field length and overall size. As we saw in
the previous section with the type `Foo`, instances with both 0 and 1
elements in the trailing `z` field result in a `Foo` whose size is 8 bytes.

When we say "x is a valid size of `T`", we mean one of two things:
- If `T: Sized`, then we mean that `x == size_of::<T>()`
- If `T` is a slice DST, then we mean that there exists a `len` such that the instance of
  `T` with `len` trailing slice elements has size `x`

When we say "largest possible size of `T` that fits in a byte slice", we
mean one of two things:
- If `T: Sized`, then we mean `size_of::<T>()` if the byte slice is at least
  `size_of::<T>()` bytes long
- If `T` is a slice DST, then we mean to consider all values, `len`, such
  that the instance of `T` with `len` trailing slice elements fits in the
  byte slice, and to choose the largest such `len`, if any


# Safety

This trait does not convey any safety guarantees to code outside this crate.

You must not rely on the `#[doc(hidden)]` internals of `KnownLayout`. Future
releases of zerocopy may make backwards-breaking changes to these items,
including changes that only affect soundness, which may cause code which
uses those items to silently become unsound.


#### Associated Types

- `type PointerMetadata: 1`

#### Provided Methods

- `fn size_for_metadata(meta: <Self as >::PointerMetadata) -> Option<usize>`

  Computes the size of an object of type `Self` with the given pointer

#### Implementors

- [`F32`](#f32)
- [`F64`](#f64)
- [`I128`](#i128)
- [`I16`](#i16)
- [`I32`](#i32)
- [`I64`](#i64)
- [`Isize`](#isize)
- [`ReadOnly`](wrappers/read_only_def/index.md#readonly)
- [`U128`](#u128)
- [`U16`](#u16)
- [`U32`](#u32)
- [`U64`](#u64)
- [`Unalign`](#unalign)
- [`Usize`](#usize)
- `&T`
- `&mut T`
- `()`
- `*const T`
- `*mut T`
- `AtomicBool`
- `AtomicI16`
- `AtomicI32`
- `AtomicI64`
- `AtomicI8`
- `AtomicIsize`
- `AtomicPtr<T>`
- `AtomicU16`
- `AtomicU32`
- `AtomicU64`
- `AtomicU8`
- `AtomicUsize`
- `Cell<T>`
- `CoreMaybeUninit<T>`
- `ManuallyDrop<T>`
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
- `Option<T>`
- `PhantomData<T>`
- `UnsafeCell<T>`
- `Wrapping<T>`
- `[T; N]`
- `[T]`
- `__m128`
- `__m128d`
- `__m128i`
- `__m256`
- `__m256d`
- `__m256i`
- `__m512`
- `__m512bh`
- `__m512d`
- `__m512i`
- `bool`
- `char`
- `f32`
- `f64`
- `i128`
- `i16`
- `i32`
- `i64`
- `i8`
- `isize`
- `str`
- `u128`
- `u16`
- `u32`
- `u64`
- `u8`
- `usize`

### `Immutable`

```rust
trait Immutable { ... }
```

Types which are free from interior mutability.

`T: Immutable` indicates that `T` does not permit interior mutation, except
by ownership or an exclusive (`&mut`) borrow.

# Implementation

**Do not implement this trait yourself!** Instead, use
[`#[derive(Immutable)]`][`derive`](../clap_builder/derive/index.md) (requires the `derive` Cargo feature);
e.g.:

```rust
use zerocopy_derive::Immutable;
#[derive(Immutable)]
struct MyStruct {
/*
    ...
*/
}

#[derive(Immutable)]
enum MyEnum {
/*
    ...
*/
}

#[derive(Immutable)]
union MyUnion {
  variant: u8,
/*
    ...
*/
}
```

This derive performs a sophisticated, compile-time safety analysis to
determine whether a type is `Immutable`.

# Safety

Unsafe code outside of this crate must not make any assumptions about `T`
based on `T: Immutable`. We reserve the right to relax the requirements for
`Immutable` in the future, and if unsafe code outside of this crate makes
assumptions based on `T: Immutable`, future relaxations may cause that code
to become unsound.



#### Implementors

- [`F32`](#f32)
- [`F64`](#f64)
- [`I128`](#i128)
- [`I16`](#i16)
- [`I32`](#i32)
- [`I64`](#i64)
- [`Isize`](#isize)
- [`ReadOnly`](wrappers/read_only_def/index.md#readonly)
- [`U128`](#u128)
- [`U16`](#u16)
- [`U32`](#u32)
- [`U64`](#u64)
- [`Unalign`](#unalign)
- [`Usize`](#usize)
- `&T`
- `&mut T`
- `()`
- `(A)`
- `(A, B)`
- `(A, B, C)`
- `(A, B, C, D)`
- `(A, B, C, D, E)`
- `(A, B, C, D, E, F)`
- `(A, B, C, D, E, F, G)`
- `(A, B, C, D, E, F, G, H)`
- `(A, B, C, D, E, F, G, H, I)`
- `(A, B, C, D, E, F, G, H, I, J)`
- `(A, B, C, D, E, F, G, H, I, J, K)`
- `(A, B, C, D, E, F, G, H, I, J, K, L)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z)`
- `*const T`
- `*mut T`
- `Box<T>`
- `CoreMaybeUninit<T>`
- `ManuallyDrop<T>`
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
- `Option<T>`
- `Option<fn() -> M>`
- `Option<fn(A, B, C, D, E, F, G, H, I, J, K, L) -> M>`
- `Option<fn(B, C, D, E, F, G, H, I, J, K, L) -> M>`
- `Option<fn(C, D, E, F, G, H, I, J, K, L) -> M>`
- `Option<fn(D, E, F, G, H, I, J, K, L) -> M>`
- `Option<fn(E, F, G, H, I, J, K, L) -> M>`
- `Option<fn(F, G, H, I, J, K, L) -> M>`
- `Option<fn(G, H, I, J, K, L) -> M>`
- `Option<fn(H, I, J, K, L) -> M>`
- `Option<fn(I, J, K, L) -> M>`
- `Option<fn(J, K, L) -> M>`
- `Option<fn(K, L) -> M>`
- `Option<fn(L) -> M>`
- `PhantomData<T>`
- `Wrapping<T>`
- `[T; N]`
- `[T]`
- `__m128`
- `__m128d`
- `__m128i`
- `__m256`
- `__m256d`
- `__m256i`
- `__m512`
- `__m512bh`
- `__m512d`
- `__m512i`
- `bool`
- `char`
- `f32`
- `f64`
- `i128`
- `i16`
- `i32`
- `i64`
- `i8`
- `isize`
- `str`
- `u128`
- `u16`
- `u32`
- `u64`
- `u8`
- `usize`

### `TryFromBytes`

```rust
trait TryFromBytes { ... }
```

Types for which some bit patterns are valid.

A memory region of the appropriate length which contains initialized bytes
can be viewed as a `TryFromBytes` type so long as the runtime value of those
bytes corresponds to a [*valid instance*] of that type. For example,
`bool` is `TryFromBytes`, so zerocopy can transmute a `u8` into a
`bool` so long as it first checks that the value of the `u8` is `0` or
`1`.

# Implementation

**Do not implement this trait yourself!** Instead, use
[`#[derive(TryFromBytes)]`][`derive`](../clap_builder/derive/index.md); e.g.:

```rust
use zerocopy_derive::{TryFromBytes, Immutable};
#[derive(TryFromBytes)]
struct MyStruct {
/*
    ...
*/
}

#[derive(TryFromBytes)]
#[repr(u8)]
enum MyEnum {
  V00,
/*
    ...
*/
}

#[derive(TryFromBytes, Immutable)]
union MyUnion {
  variant: u8,
/*
    ...
*/
}
```

This derive ensures that the runtime check of whether bytes correspond to a
valid instance is sound. You **must** implement this trait via the derive.

# What is a "valid instance"?

In Rust, each type has *bit validity*, which refers to the set of bit
patterns which may appear in an instance of that type. It is impossible for
safe Rust code to produce values which violate bit validity (ie, values
outside of the "valid" set of bit patterns). If `unsafe` code produces an
invalid value, this is considered [undefined behavior].

Rust's bit validity rules are currently being decided, which means that some
types have three classes of bit patterns: those which are definitely valid,
and whose validity is documented in the language; those which may or may not
be considered valid at some point in the future; and those which are
definitely invalid.

Zerocopy takes a conservative approach, and only considers a bit pattern to
be valid if its validity is a documented guarantee provided by the
language.

For most use cases, Rust's current guarantees align with programmers'
intuitions about what ought to be valid. As a result, zerocopy's
conservatism should not affect most users.

If you are negatively affected by lack of support for a particular type,
we encourage you to let us know by [filing an issue][github-repo].

# `TryFromBytes` is not symmetrical with [`IntoBytes`](#intobytes)

There are some types which implement both `TryFromBytes` and [`IntoBytes`](#intobytes),
but for which `TryFromBytes` is not guaranteed to accept all byte sequences
produced by `IntoBytes`. In other words, for some `T: TryFromBytes +
IntoBytes`, there exist values of `t: T` such that
`TryFromBytes::try_ref_from_bytes(t.as_bytes()) == None`. Code should not
generally assume that values produced by `IntoBytes` will necessarily be
accepted as valid by `TryFromBytes`.

# Safety

On its own, `T: TryFromBytes` does not make any guarantees about the layout
or representation of `T`. It merely provides the ability to perform a
validity check at runtime via methods like `try_ref_from_bytes`.

You must not rely on the `#[doc(hidden)]` internals of `TryFromBytes`.
Future releases of zerocopy may make backwards-breaking changes to these
items, including changes that only affect soundness, which may cause code
which uses those items to silently become unsound.






#### Provided Methods

- `fn try_ref_from_bytes(source: &[u8]) -> Result<&Self, TryCastError<&[u8], Self>>`

  Attempts to interpret the given `source` as a `&Self`.

- `fn try_ref_from_prefix(source: &[u8]) -> Result<(&Self, &[u8]), TryCastError<&[u8], Self>>`

  Attempts to interpret the prefix of the given `source` as a `&Self`.

- `fn try_ref_from_suffix(source: &[u8]) -> Result<(&[u8], &Self), TryCastError<&[u8], Self>>`

  Attempts to interpret the suffix of the given `source` as a `&Self`.

- `fn try_mut_from_bytes(bytes: &mut [u8]) -> Result<&mut Self, TryCastError<&mut [u8], Self>>`

  Attempts to interpret the given `source` as a `&mut Self` without

- `fn try_mut_from_prefix(source: &mut [u8]) -> Result<(&mut Self, &mut [u8]), TryCastError<&mut [u8], Self>>`

  Attempts to interpret the prefix of the given `source` as a `&mut

- `fn try_mut_from_suffix(source: &mut [u8]) -> Result<(&mut [u8], &mut Self), TryCastError<&mut [u8], Self>>`

  Attempts to interpret the suffix of the given `source` as a `&mut

- `fn try_ref_from_bytes_with_elems(source: &[u8], count: usize) -> Result<&Self, TryCastError<&[u8], Self>>`

  Attempts to interpret the given `source` as a `&Self` with a DST length

- `fn try_ref_from_prefix_with_elems(source: &[u8], count: usize) -> Result<(&Self, &[u8]), TryCastError<&[u8], Self>>`

  Attempts to interpret the prefix of the given `source` as a `&Self` with

- `fn try_ref_from_suffix_with_elems(source: &[u8], count: usize) -> Result<(&[u8], &Self), TryCastError<&[u8], Self>>`

  Attempts to interpret the suffix of the given `source` as a `&Self` with

- `fn try_mut_from_bytes_with_elems(source: &mut [u8], count: usize) -> Result<&mut Self, TryCastError<&mut [u8], Self>>`

  Attempts to interpret the given `source` as a `&mut Self` with a DST

- `fn try_mut_from_prefix_with_elems(source: &mut [u8], count: usize) -> Result<(&mut Self, &mut [u8]), TryCastError<&mut [u8], Self>>`

  Attempts to interpret the prefix of the given `source` as a `&mut Self`

- `fn try_mut_from_suffix_with_elems(source: &mut [u8], count: usize) -> Result<(&mut [u8], &mut Self), TryCastError<&mut [u8], Self>>`

  Attempts to interpret the suffix of the given `source` as a `&mut Self`

- `fn try_read_from_bytes(source: &[u8]) -> Result<Self, TryReadError<&[u8], Self>>`

  Attempts to read the given `source` as a `Self`.

- `fn try_read_from_prefix(source: &[u8]) -> Result<(Self, &[u8]), TryReadError<&[u8], Self>>`

  Attempts to read a `Self` from the prefix of the given `source`.

- `fn try_read_from_suffix(source: &[u8]) -> Result<(&[u8], Self), TryReadError<&[u8], Self>>`

  Attempts to read a `Self` from the suffix of the given `source`.

#### Implementors

- [`F32`](#f32)
- [`F64`](#f64)
- [`I128`](#i128)
- [`I16`](#i16)
- [`I32`](#i32)
- [`I64`](#i64)
- [`Isize`](#isize)
- [`ReadOnly`](wrappers/read_only_def/index.md#readonly)
- [`U128`](#u128)
- [`U16`](#u16)
- [`U32`](#u32)
- [`U64`](#u64)
- [`Unalign`](#unalign)
- [`Usize`](#usize)
- `()`
- `(A)`
- `(A, B)`
- `(A, B, C)`
- `(A, B, C, D)`
- `(A, B, C, D, E)`
- `(A, B, C, D, E, F)`
- `(A, B, C, D, E, F, G)`
- `(A, B, C, D, E, F, G, H)`
- `(A, B, C, D, E, F, G, H, I)`
- `(A, B, C, D, E, F, G, H, I, J)`
- `(A, B, C, D, E, F, G, H, I, J, K)`
- `(A, B, C, D, E, F, G, H, I, J, K, L)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z)`
- `*const T`
- `*mut T`
- `AtomicBool`
- `AtomicI16`
- `AtomicI32`
- `AtomicI64`
- `AtomicI8`
- `AtomicIsize`
- `AtomicPtr<T>`
- `AtomicU16`
- `AtomicU32`
- `AtomicU64`
- `AtomicU8`
- `AtomicUsize`
- `Cell<T>`
- `CoreMaybeUninit<T>`
- `ManuallyDrop<T>`
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
- `Option<&T>`
- `Option<&mut T>`
- `Option<Box<T>>`
- `Option<NonNull<T>>`
- `Option<NonZeroI128>`
- `Option<NonZeroI16>`
- `Option<NonZeroI32>`
- `Option<NonZeroI64>`
- `Option<NonZeroI8>`
- `Option<NonZeroIsize>`
- `Option<NonZeroU128>`
- `Option<NonZeroU16>`
- `Option<NonZeroU32>`
- `Option<NonZeroU64>`
- `Option<NonZeroU8>`
- `Option<NonZeroUsize>`
- `Option<fn() -> M>`
- `Option<fn(A, B, C, D, E, F, G, H, I, J, K, L) -> M>`
- `Option<fn(B, C, D, E, F, G, H, I, J, K, L) -> M>`
- `Option<fn(C, D, E, F, G, H, I, J, K, L) -> M>`
- `Option<fn(D, E, F, G, H, I, J, K, L) -> M>`
- `Option<fn(E, F, G, H, I, J, K, L) -> M>`
- `Option<fn(F, G, H, I, J, K, L) -> M>`
- `Option<fn(G, H, I, J, K, L) -> M>`
- `Option<fn(H, I, J, K, L) -> M>`
- `Option<fn(I, J, K, L) -> M>`
- `Option<fn(J, K, L) -> M>`
- `Option<fn(K, L) -> M>`
- `Option<fn(L) -> M>`
- `PhantomData<T>`
- `Wrapping<T>`
- `[T; N]`
- `[T]`
- `__m128`
- `__m128d`
- `__m128i`
- `__m256`
- `__m256d`
- `__m256i`
- `__m512`
- `__m512bh`
- `__m512d`
- `__m512i`
- `bool`
- `char`
- `core::cell::UnsafeCell<T>`
- `f32`
- `f64`
- `i128`
- `i16`
- `i32`
- `i64`
- `i8`
- `isize`
- `str`
- `u128`
- `u16`
- `u32`
- `u64`
- `u8`
- `usize`

### `FromZeros`

```rust
trait FromZeros: TryFromBytes { ... }
```

Types for which a sequence of `0` bytes is a valid instance.

Any memory region of the appropriate length which is guaranteed to contain
only zero bytes can be viewed as any `FromZeros` type with no runtime
overhead. This is useful whenever memory is known to be in a zeroed state,
such memory returned from some allocation routines.

# Warning: Padding bytes

Note that, when a value is moved or copied, only the non-padding bytes of
that value are guaranteed to be preserved. It is unsound to assume that
values written to padding bytes are preserved after a move or copy. For more
details, see the [`FromBytes` docs][frombytes-warning-padding-bytes].

# Implementation

**Do not implement this trait yourself!** Instead, use
[`#[derive(FromZeros)]`][`derive`](../clap_builder/derive/index.md); e.g.:

```rust
use zerocopy_derive::{FromZeros, Immutable};
#[derive(FromZeros)]
struct MyStruct {
/*
    ...
*/
}

#[derive(FromZeros)]
#[repr(u8)]
enum MyEnum {
  Variant0,
/*
    ...
*/
}

#[derive(FromZeros, Immutable)]
union MyUnion {
  variant: u8,
/*
    ...
*/
}
```

This derive performs a sophisticated, compile-time safety analysis to
determine whether a type is `FromZeros`.

# Safety

*This section describes what is required in order for `T: FromZeros`, and
what unsafe code may assume of such types. If you don't plan on implementing
`FromZeros` manually, and you don't plan on writing unsafe code that
operates on `FromZeros` types, then you don't need to read this section.*

If `T: FromZeros`, then unsafe code may assume that it is sound to produce a
`T` whose bytes are all initialized to zero. If a type is marked as
`FromZeros` which violates this contract, it may cause undefined behavior.

`#[derive(FromZeros)]` only permits [types which satisfy these
requirements][derive-analysis].



#### Provided Methods

- `fn zero(&mut self)`

  Overwrites `self` with zeros.

- `fn new_zeroed() -> Self`

  Creates an instance of `Self` from zeroed bytes.

- `fn new_box_zeroed() -> Result<Box<Self>, AllocError>`

  Creates a `Box<Self>` from zeroed bytes.

- `fn new_box_zeroed_with_elems(count: usize) -> Result<Box<Self>, AllocError>`

  Creates a `Box<[Self]>` (a boxed slice) from zeroed bytes.

- `fn new_vec_zeroed(len: usize) -> Result<Vec<Self>, AllocError>`

  Creates a `Vec<Self>` from zeroed bytes.

- `fn extend_vec_zeroed(v: &mut Vec<Self>, additional: usize) -> Result<(), AllocError>`

  Extends a `Vec<Self>` by pushing `additional` new items onto the end of

- `fn insert_vec_zeroed(v: &mut Vec<Self>, position: usize, additional: usize) -> Result<(), AllocError>`

  Inserts `additional` new items into `Vec<Self>` at `position`. The new

#### Implementors

- [`F32`](#f32)
- [`F64`](#f64)
- [`I128`](#i128)
- [`I16`](#i16)
- [`I32`](#i32)
- [`I64`](#i64)
- [`Isize`](#isize)
- [`ReadOnly`](wrappers/read_only_def/index.md#readonly)
- [`U128`](#u128)
- [`U16`](#u16)
- [`U32`](#u32)
- [`U64`](#u64)
- [`Unalign`](#unalign)
- [`Usize`](#usize)
- `()`
- `(A)`
- `(A, B)`
- `(A, B, C)`
- `(A, B, C, D)`
- `(A, B, C, D, E)`
- `(A, B, C, D, E, F)`
- `(A, B, C, D, E, F, G)`
- `(A, B, C, D, E, F, G, H)`
- `(A, B, C, D, E, F, G, H, I)`
- `(A, B, C, D, E, F, G, H, I, J)`
- `(A, B, C, D, E, F, G, H, I, J, K)`
- `(A, B, C, D, E, F, G, H, I, J, K, L)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z)`
- `*const T`
- `*mut T`
- `AtomicBool`
- `AtomicI16`
- `AtomicI32`
- `AtomicI64`
- `AtomicI8`
- `AtomicIsize`
- `AtomicPtr<T>`
- `AtomicU16`
- `AtomicU32`
- `AtomicU64`
- `AtomicU8`
- `AtomicUsize`
- `Cell<T>`
- `CoreMaybeUninit<T>`
- `ManuallyDrop<T>`
- `Option<&T>`
- `Option<&mut T>`
- `Option<Box<T>>`
- `Option<NonNull<T>>`
- `Option<NonZeroI128>`
- `Option<NonZeroI16>`
- `Option<NonZeroI32>`
- `Option<NonZeroI64>`
- `Option<NonZeroI8>`
- `Option<NonZeroIsize>`
- `Option<NonZeroU128>`
- `Option<NonZeroU16>`
- `Option<NonZeroU32>`
- `Option<NonZeroU64>`
- `Option<NonZeroU8>`
- `Option<NonZeroUsize>`
- `Option<fn() -> M>`
- `Option<fn(A, B, C, D, E, F, G, H, I, J, K, L) -> M>`
- `Option<fn(B, C, D, E, F, G, H, I, J, K, L) -> M>`
- `Option<fn(C, D, E, F, G, H, I, J, K, L) -> M>`
- `Option<fn(D, E, F, G, H, I, J, K, L) -> M>`
- `Option<fn(E, F, G, H, I, J, K, L) -> M>`
- `Option<fn(F, G, H, I, J, K, L) -> M>`
- `Option<fn(G, H, I, J, K, L) -> M>`
- `Option<fn(H, I, J, K, L) -> M>`
- `Option<fn(I, J, K, L) -> M>`
- `Option<fn(J, K, L) -> M>`
- `Option<fn(K, L) -> M>`
- `Option<fn(L) -> M>`
- `PhantomData<T>`
- `UnsafeCell<T>`
- `Wrapping<T>`
- `[T; N]`
- `[T]`
- `__m128`
- `__m128d`
- `__m128i`
- `__m256`
- `__m256d`
- `__m256i`
- `__m512`
- `__m512bh`
- `__m512d`
- `__m512i`
- `bool`
- `char`
- `f32`
- `f64`
- `i128`
- `i16`
- `i32`
- `i64`
- `i8`
- `isize`
- `str`
- `u128`
- `u16`
- `u32`
- `u64`
- `u8`
- `usize`

### `FromBytes`

```rust
trait FromBytes: FromZeros { ... }
```

Types for which any bit pattern is valid.

Any memory region of the appropriate length which contains initialized bytes
can be viewed as any `FromBytes` type with no runtime overhead. This is
useful for efficiently parsing bytes as structured data.

# Warning: Padding bytes

Note that, when a value is moved or copied, only the non-padding bytes of
that value are guaranteed to be preserved. It is unsound to assume that
values written to padding bytes are preserved after a move or copy. For
example, the following is unsound:

```rust,no_run
use core::mem::{size_of, transmute};
use zerocopy::FromZeros;
use zerocopy_derive::*;

// Assume `Foo` is a type with padding bytes.
#[derive(FromZeros, Default)]
struct Foo {
/*
    ...
*/
}

let mut foo: Foo = Foo::default();
FromZeros::zero(&mut foo);
// UNSOUND: Although `FromZeros::zero` writes zeros to all bytes of `foo`,
// those writes are not guaranteed to be preserved in padding bytes when
// `foo` is moved, so this may expose padding bytes as `u8`s.
let foo_bytes: [u8; size_of::<Foo>()] = unsafe { transmute(foo) };
```

# Implementation

**Do not implement this trait yourself!** Instead, use
[`#[derive(FromBytes)]`][`derive`](../clap_builder/derive/index.md); e.g.:

```rust
use zerocopy_derive::{FromBytes, Immutable};
#[derive(FromBytes)]
struct MyStruct {
/*
    ...
*/
}

#[derive(FromBytes)]
#[repr(u8)]
enum MyEnum {
  V00, V01, V02, V03, V04, V05, V06, V07, V08, V09, V0A, V0B, V0C, V0D, V0E,
  V0F, V10, V11, V12, V13, V14, V15, V16, V17, V18, V19, V1A, V1B, V1C, V1D,
  V1E, V1F, V20, V21, V22, V23, V24, V25, V26, V27, V28, V29, V2A, V2B, V2C,
  V2D, V2E, V2F, V30, V31, V32, V33, V34, V35, V36, V37, V38, V39, V3A, V3B,
  V3C, V3D, V3E, V3F, V40, V41, V42, V43, V44, V45, V46, V47, V48, V49, V4A,
  V4B, V4C, V4D, V4E, V4F, V50, V51, V52, V53, V54, V55, V56, V57, V58, V59,
  V5A, V5B, V5C, V5D, V5E, V5F, V60, V61, V62, V63, V64, V65, V66, V67, V68,
  V69, V6A, V6B, V6C, V6D, V6E, V6F, V70, V71, V72, V73, V74, V75, V76, V77,
  V78, V79, V7A, V7B, V7C, V7D, V7E, V7F, V80, V81, V82, V83, V84, V85, V86,
  V87, V88, V89, V8A, V8B, V8C, V8D, V8E, V8F, V90, V91, V92, V93, V94, V95,
  V96, V97, V98, V99, V9A, V9B, V9C, V9D, V9E, V9F, VA0, VA1, VA2, VA3, VA4,
  VA5, VA6, VA7, VA8, VA9, VAA, VAB, VAC, VAD, VAE, VAF, VB0, VB1, VB2, VB3,
  VB4, VB5, VB6, VB7, VB8, VB9, VBA, VBB, VBC, VBD, VBE, VBF, VC0, VC1, VC2,
  VC3, VC4, VC5, VC6, VC7, VC8, VC9, VCA, VCB, VCC, VCD, VCE, VCF, VD0, VD1,
  VD2, VD3, VD4, VD5, VD6, VD7, VD8, VD9, VDA, VDB, VDC, VDD, VDE, VDF, VE0,
  VE1, VE2, VE3, VE4, VE5, VE6, VE7, VE8, VE9, VEA, VEB, VEC, VED, VEE, VEF,
  VF0, VF1, VF2, VF3, VF4, VF5, VF6, VF7, VF8, VF9, VFA, VFB, VFC, VFD, VFE,
  VFF,
/*
    ...
*/
}

#[derive(FromBytes, Immutable)]
union MyUnion {
  variant: u8,
/*
    ...
*/
}
```

This derive performs a sophisticated, compile-time safety analysis to
determine whether a type is `FromBytes`.

# Safety

*This section describes what is required in order for `T: FromBytes`, and
what unsafe code may assume of such types. If you don't plan on implementing
`FromBytes` manually, and you don't plan on writing unsafe code that
operates on `FromBytes` types, then you don't need to read this section.*

If `T: FromBytes`, then unsafe code may assume that it is sound to produce a
`T` whose bytes are initialized to any sequence of valid `u8`s (in other
words, any byte value which is not uninitialized). If a type is marked as
`FromBytes` which violates this contract, it may cause undefined behavior.

`#[derive(FromBytes)]` only permits [types which satisfy these
requirements][derive-analysis].



#### Provided Methods

- `fn ref_from_bytes(source: &[u8]) -> Result<&Self, CastError<&[u8], Self>>`

  Interprets the given `source` as a `&Self`.

- `fn ref_from_prefix(source: &[u8]) -> Result<(&Self, &[u8]), CastError<&[u8], Self>>`

  Interprets the prefix of the given `source` as a `&Self` without

- `fn ref_from_suffix(source: &[u8]) -> Result<(&[u8], &Self), CastError<&[u8], Self>>`

  Interprets the suffix of the given bytes as a `&Self`.

- `fn mut_from_bytes(source: &mut [u8]) -> Result<&mut Self, CastError<&mut [u8], Self>>`

  Interprets the given `source` as a `&mut Self`.

- `fn mut_from_prefix(source: &mut [u8]) -> Result<(&mut Self, &mut [u8]), CastError<&mut [u8], Self>>`

  Interprets the prefix of the given `source` as a `&mut Self` without

- `fn mut_from_suffix(source: &mut [u8]) -> Result<(&mut [u8], &mut Self), CastError<&mut [u8], Self>>`

  Interprets the suffix of the given `source` as a `&mut Self` without

- `fn ref_from_bytes_with_elems(source: &[u8], count: usize) -> Result<&Self, CastError<&[u8], Self>>`

  Interprets the given `source` as a `&Self` with a DST length equal to

- `fn ref_from_prefix_with_elems(source: &[u8], count: usize) -> Result<(&Self, &[u8]), CastError<&[u8], Self>>`

  Interprets the prefix of the given `source` as a DST `&Self` with length

- `fn ref_from_suffix_with_elems(source: &[u8], count: usize) -> Result<(&[u8], &Self), CastError<&[u8], Self>>`

  Interprets the suffix of the given `source` as a DST `&Self` with length

- `fn mut_from_bytes_with_elems(source: &mut [u8], count: usize) -> Result<&mut Self, CastError<&mut [u8], Self>>`

  Interprets the given `source` as a `&mut Self` with a DST length equal

- `fn mut_from_prefix_with_elems(source: &mut [u8], count: usize) -> Result<(&mut Self, &mut [u8]), CastError<&mut [u8], Self>>`

  Interprets the prefix of the given `source` as a `&mut Self` with DST

- `fn mut_from_suffix_with_elems(source: &mut [u8], count: usize) -> Result<(&mut [u8], &mut Self), CastError<&mut [u8], Self>>`

  Interprets the suffix of the given `source` as a `&mut Self` with DST

- `fn read_from_bytes(source: &[u8]) -> Result<Self, SizeError<&[u8], Self>>`

  Reads a copy of `Self` from the given `source`.

- `fn read_from_prefix(source: &[u8]) -> Result<(Self, &[u8]), SizeError<&[u8], Self>>`

  Reads a copy of `Self` from the prefix of the given `source`.

- `fn read_from_suffix(source: &[u8]) -> Result<(&[u8], Self), SizeError<&[u8], Self>>`

  Reads a copy of `Self` from the suffix of the given `source`.

#### Implementors

- [`F32`](#f32)
- [`F64`](#f64)
- [`I128`](#i128)
- [`I16`](#i16)
- [`I32`](#i32)
- [`I64`](#i64)
- [`Isize`](#isize)
- [`ReadOnly`](wrappers/read_only_def/index.md#readonly)
- [`U128`](#u128)
- [`U16`](#u16)
- [`U32`](#u32)
- [`U64`](#u64)
- [`Unalign`](#unalign)
- [`Usize`](#usize)
- `()`
- `(A)`
- `(A, B)`
- `(A, B, C)`
- `(A, B, C, D)`
- `(A, B, C, D, E)`
- `(A, B, C, D, E, F)`
- `(A, B, C, D, E, F, G)`
- `(A, B, C, D, E, F, G, H)`
- `(A, B, C, D, E, F, G, H, I)`
- `(A, B, C, D, E, F, G, H, I, J)`
- `(A, B, C, D, E, F, G, H, I, J, K)`
- `(A, B, C, D, E, F, G, H, I, J, K, L)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y)`
- `(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z)`
- `AtomicI16`
- `AtomicI32`
- `AtomicI64`
- `AtomicI8`
- `AtomicIsize`
- `AtomicU16`
- `AtomicU32`
- `AtomicU64`
- `AtomicU8`
- `AtomicUsize`
- `Cell<T>`
- `CoreMaybeUninit<T>`
- `ManuallyDrop<T>`
- `Option<NonZeroI128>`
- `Option<NonZeroI16>`
- `Option<NonZeroI32>`
- `Option<NonZeroI64>`
- `Option<NonZeroI8>`
- `Option<NonZeroIsize>`
- `Option<NonZeroU128>`
- `Option<NonZeroU16>`
- `Option<NonZeroU32>`
- `Option<NonZeroU64>`
- `Option<NonZeroU8>`
- `Option<NonZeroUsize>`
- `PhantomData<T>`
- `UnsafeCell<T>`
- `Wrapping<T>`
- `[T; N]`
- `[T]`
- `__m128`
- `__m128d`
- `__m128i`
- `__m256`
- `__m256d`
- `__m256i`
- `__m512`
- `__m512bh`
- `__m512d`
- `__m512i`
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

### `IntoBytes`

```rust
trait IntoBytes { ... }
```

Types that can be converted to an immutable slice of initialized bytes.

Any `IntoBytes` type can be converted to a slice of initialized bytes of the
same size. This is useful for efficiently serializing structured data as raw
bytes.

# Implementation

**Do not implement this trait yourself!** Instead, use
[`#[derive(IntoBytes)]`][`derive`](../clap_builder/derive/index.md); e.g.:

```rust
use zerocopy_derive::IntoBytes;
#[derive(IntoBytes)]
#[repr(C)]
struct MyStruct {
/*
    ...
*/
}

#[derive(IntoBytes)]
#[repr(u8)]
enum MyEnum {
  Variant0,
/*
    ...
*/
}
```

This derive performs a sophisticated, compile-time safety analysis to
determine whether a type is `IntoBytes`. See the [derive
documentation][`derive`](../clap_builder/derive/index.md) for guidance on how to interpret error messages
produced by the derive's analysis.

# Safety

*This section describes what is required in order for `T: IntoBytes`, and
what unsafe code may assume of such types. If you don't plan on implementing
`IntoBytes` manually, and you don't plan on writing unsafe code that
operates on `IntoBytes` types, then you don't need to read this section.*

If `T: IntoBytes`, then unsafe code may assume that it is sound to treat any
`t: T` as an immutable `[u8]` of length `size_of_val(t)`. If a type is
marked as `IntoBytes` which violates this contract, it may cause undefined
behavior.

`#[derive(IntoBytes)]` only permits [types which satisfy these
requirements][derive-analysis].



#### Provided Methods

- `fn as_bytes(&self) -> &[u8]`

  Gets the bytes of this value.

- `fn as_mut_bytes(&mut self) -> &mut [u8]`

  Gets the bytes of this value mutably.

- `fn write_to(&self, dst: &mut [u8]) -> Result<(), SizeError<&Self, &mut [u8]>>`

  Writes a copy of `self` to `dst`.

- `fn write_to_prefix(&self, dst: &mut [u8]) -> Result<(), SizeError<&Self, &mut [u8]>>`

  Writes a copy of `self` to the prefix of `dst`.

- `fn write_to_suffix(&self, dst: &mut [u8]) -> Result<(), SizeError<&Self, &mut [u8]>>`

  Writes a copy of `self` to the suffix of `dst`.

#### Implementors

- [`F32`](#f32)
- [`F64`](#f64)
- [`I128`](#i128)
- [`I16`](#i16)
- [`I32`](#i32)
- [`I64`](#i64)
- [`Isize`](#isize)
- [`ReadOnly`](wrappers/read_only_def/index.md#readonly)
- [`U128`](#u128)
- [`U16`](#u16)
- [`U32`](#u32)
- [`U64`](#u64)
- [`Unalign`](#unalign)
- [`Usize`](#usize)
- `()`
- `AtomicBool`
- `AtomicI16`
- `AtomicI32`
- `AtomicI64`
- `AtomicI8`
- `AtomicIsize`
- `AtomicU16`
- `AtomicU32`
- `AtomicU64`
- `AtomicU8`
- `AtomicUsize`
- `Cell<T>`
- `ManuallyDrop<T>`
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
- `Option<NonZeroI128>`
- `Option<NonZeroI16>`
- `Option<NonZeroI32>`
- `Option<NonZeroI64>`
- `Option<NonZeroI8>`
- `Option<NonZeroIsize>`
- `Option<NonZeroU128>`
- `Option<NonZeroU16>`
- `Option<NonZeroU32>`
- `Option<NonZeroU64>`
- `Option<NonZeroU8>`
- `Option<NonZeroUsize>`
- `PhantomData<T>`
- `UnsafeCell<T>`
- `Wrapping<T>`
- `[T; N]`
- `[T]`
- `__m128`
- `__m128d`
- `__m128i`
- `__m256`
- `__m256d`
- `__m256i`
- `__m512`
- `__m512bh`
- `__m512d`
- `__m512i`
- `bool`
- `char`
- `f32`
- `f64`
- `i128`
- `i16`
- `i32`
- `i64`
- `i8`
- `isize`
- `str`
- `u128`
- `u16`
- `u32`
- `u64`
- `u8`
- `usize`

### `Unaligned`

```rust
trait Unaligned { ... }
```

Types with no alignment requirement.

If `T: Unaligned`, then `align_of::<T>() == 1`.

# Implementation

**Do not implement this trait yourself!** Instead, use
[`#[derive(Unaligned)]`][`derive`](../clap_builder/derive/index.md); e.g.:

```rust
use zerocopy_derive::Unaligned;
#[derive(Unaligned)]
#[repr(C)]
struct MyStruct {
/*
    ...
*/
}

#[derive(Unaligned)]
#[repr(u8)]
enum MyEnum {
  Variant0,
/*
    ...
*/
}

#[derive(Unaligned)]
#[repr(packed)]
union MyUnion {
  variant: u8,
/*
    ...
*/
}
```

This derive performs a sophisticated, compile-time safety analysis to
determine whether a type is `Unaligned`.

# Safety

*This section describes what is required in order for `T: Unaligned`, and
what unsafe code may assume of such types. If you don't plan on implementing
`Unaligned` manually, and you don't plan on writing unsafe code that
operates on `Unaligned` types, then you don't need to read this section.*

If `T: Unaligned`, then unsafe code may assume that it is sound to produce a
reference to `T` at any memory location regardless of alignment. If a type
is marked as `Unaligned` which violates this contract, it may cause
undefined behavior.

`#[derive(Unaligned)]` only permits [types which satisfy these
requirements][derive-analysis].



#### Implementors

- [`F32`](#f32)
- [`F64`](#f64)
- [`I128`](#i128)
- [`I16`](#i16)
- [`I32`](#i32)
- [`I64`](#i64)
- [`Isize`](#isize)
- [`ReadOnly`](wrappers/read_only_def/index.md#readonly)
- [`U128`](#u128)
- [`U16`](#u16)
- [`U32`](#u32)
- [`U64`](#u64)
- [`Unalign`](#unalign)
- [`Usize`](#usize)
- `()`
- `AtomicBool`
- `AtomicI8`
- `AtomicU8`
- `Cell<T>`
- `CoreMaybeUninit<T>`
- `ManuallyDrop<T>`
- `NonZeroI8`
- `NonZeroU8`
- `Option<NonZeroI8>`
- `Option<NonZeroU8>`
- `PhantomData<T>`
- `UnsafeCell<T>`
- `Wrapping<T>`
- `[T; N]`
- `[T]`
- `bool`
- `i8`
- `str`
- `u8`

### `ByteSlice`

```rust
trait ByteSlice: Deref<Target = [u8]> + Sized { ... }
```

A mutable or immutable reference to a byte slice.

`ByteSlice` abstracts over the mutability of a byte slice reference, and is
implemented for various special reference types such as
[`Ref<[u8]>`](core::cell::Ref) and [`RefMut<[u8]>`](core::cell::RefMut).

# Safety

Implementations of `ByteSlice` must promise that their implementations of
[`Deref`](../cpp_demangle/index.md) and `DerefMut` are "stable". In particular, given `B: ByteSlice`
and `b: B`, two calls, each to either `b.deref()` or `b.deref_mut()`, must
return a byte slice with the same address and length. This must hold even if
the two calls are separated by an arbitrary sequence of calls to methods on
`ByteSlice`, [`ByteSliceMut`](#byteslicemut), [`IntoByteSlice`](#intobyteslice), or [`IntoByteSliceMut`](#intobyteslicemut),
or on their super-traits. This does *not* need to hold if the two calls are
separated by any method calls, field accesses, or field modifications *other
than* those from these traits.

Note that this also implies that, given `b: B`, the address and length
cannot be modified via objects other than `b`, either on the same thread or
on another thread.

#### Implementors

- `&[u8]`
- `&mut [u8]`
- `cell::Ref<'_, [u8]>`
- `cell::RefMut<'_, [u8]>`

### `ByteSliceMut`

```rust
trait ByteSliceMut: ByteSlice + DerefMut { ... }
```

A mutable reference to a byte slice.

`ByteSliceMut` abstracts over various ways of storing a mutable reference to
a byte slice, and is implemented for various special reference types such as
`RefMut<[u8]>`.

`ByteSliceMut` is a shorthand for [`ByteSlice`](#byteslice) and `DerefMut`.

#### Implementors

- `B`

### `CopyableByteSlice`

```rust
trait CopyableByteSlice: ByteSlice + Copy + CloneableByteSlice { ... }
```

A [`ByteSlice`](#byteslice) which can be copied without violating dereference stability.

# Safety

If `B: CopyableByteSlice`, then the dereference stability properties
required by [`ByteSlice`](#byteslice) (see that trait's safety documentation) do not
only hold regarding two calls to `b.deref()` or `b.deref_mut()`, but also
hold regarding `c.deref()` or `c.deref_mut()`, where `c` is produced by
copying `b`.

#### Implementors

- `&[u8]`

### `CloneableByteSlice`

```rust
trait CloneableByteSlice: ByteSlice + Clone { ... }
```

A [`ByteSlice`](#byteslice) which can be cloned without violating dereference stability.

# Safety

If `B: CloneableByteSlice`, then the dereference stability properties
required by [`ByteSlice`](#byteslice) (see that trait's safety documentation) do not
only hold regarding two calls to `b.deref()` or `b.deref_mut()`, but also
hold regarding `c.deref()` or `c.deref_mut()`, where `c` is produced by
`b.clone()`, `b.clone().clone()`, etc.

#### Implementors

- `&[u8]`

### `SplitByteSlice`

```rust
trait SplitByteSlice: ByteSlice { ... }
```

A [`ByteSlice`](#byteslice) that can be split in two.

# Safety

Unsafe code may depend for its soundness on the assumption that `split_at`
and `split_at_unchecked` are implemented correctly. In particular, given `B:
SplitByteSlice` and `b: B`, if `b.deref()` returns a byte slice with address
`addr` and length `len`, then if `split <= len`, both of these
invocations:
- `b.split_at(split)`
- `b.split_at_unchecked(split)`

...will return `(first, second)` such that:
- `first`'s address is `addr` and its length is `split`
- `second`'s address is `addr + split` and its length is `len - split`

#### Required Methods

- `fn split_at_unchecked(self, mid: usize) -> (Self, Self)`

  Splits the slice at the midpoint, possibly omitting bounds checks.

#### Provided Methods

- `fn split_at(self, mid: usize) -> Result<(Self, Self), Self>`

  Attempts to split `self` at the midpoint.

#### Implementors

- `&[u8]`
- `&mut [u8]`
- `cell::Ref<'_, [u8]>`
- `cell::RefMut<'_, [u8]>`

### `SplitByteSliceMut`

```rust
trait SplitByteSliceMut: SplitByteSlice + ByteSliceMut { ... }
```

A shorthand for [`SplitByteSlice`](#splitbyteslice) and [`ByteSliceMut`](#byteslicemut).

#### Implementors

- `B`

### `IntoByteSlice<'a>`

```rust
trait IntoByteSlice<'a>: ByteSlice { ... }
```

A [`ByteSlice`](#byteslice) that conveys no ownership, and so can be converted into a
byte slice.

Some `ByteSlice` types (notably, the standard library's [`Ref`](ref/def/index.md) type) convey
ownership, and so they cannot soundly be moved by-value into a byte slice
type (`&[u8]`). Some methods in this crate's API (such as `Ref::into_ref`)
are only compatible with `ByteSlice` types without these ownership
semantics.


#### Required Methods

- `fn into_byte_slice(self) -> &'a [u8]`

  Coverts `self` into a `&[u8]`.

#### Implementors

- `&'a [u8]`
- `&'a mut [u8]`

### `IntoByteSliceMut<'a>`

```rust
trait IntoByteSliceMut<'a>: IntoByteSlice<'a> + ByteSliceMut { ... }
```

A [`ByteSliceMut`](#byteslicemut) that conveys no ownership, and so can be converted into a
mutable byte slice.

Some `ByteSliceMut` types (notably, the standard library's `RefMut` type)
convey ownership, and so they cannot soundly be moved by-value into a byte
slice type (`&mut [u8]`). Some methods in this crate's API (such as
`Ref::into_mut`) are only compatible with `ByteSliceMut` types without
these ownership semantics.


#### Required Methods

- `fn into_byte_slice_mut(self) -> &'a mut [u8]`

  Coverts `self` into a `&mut [u8]`.

#### Implementors

- `&'a mut [u8]`

### `ByteOrder`

```rust
trait ByteOrder: Copy + Clone + Debug + Display + Eq + PartialEq + Ord + PartialOrd + Hash + private::Sealed { ... }
```

A type-level representation of byte order.

This type is implemented by [`BigEndian`](#bigendian) and [`LittleEndian`](#littleendian), which
represent big-endian and little-endian byte order respectively. This module
also provides a number of useful aliases for those types: [`NativeEndian`](#nativeendian),
[`NetworkEndian`](#networkendian), [`BE`](#be), and [`LE`](#le).

`ByteOrder` types can be used to specify the byte order of the types in this
module - for example, `U32<BigEndian>` is a 32-bit integer stored in
big-endian byte order.


#### Implementors

- [`BigEndian`](#bigendian)
- [`LittleEndian`](#littleendian)

## Functions

### `trailing_slice_layout`

```rust
fn trailing_slice_layout<T>() -> TrailingSliceLayout
where
    T: ?Sized + KnownLayout<PointerMetadata = usize>
```

Efficiently produces the `TrailingSliceLayout` of `T`.

### `FromZeros`

```rust
fn FromZeros(&self) -> impl Iterator<Item = &T>
```

### `try_ref_from_prefix_suffix`

```rust
fn try_ref_from_prefix_suffix<T: TryFromBytes + KnownLayout + Immutable + ?Sized>(source: &[u8], cast_type: CastType, meta: Option<<T as >::PointerMetadata>) -> Result<(&T, &[u8]), TryCastError<&[u8], T>>
```

### `try_mut_from_prefix_suffix`

```rust
fn try_mut_from_prefix_suffix<T: IntoBytes + TryFromBytes + KnownLayout + ?Sized>(candidate: &mut [u8], cast_type: CastType, meta: Option<<T as >::PointerMetadata>) -> Result<(&mut T, &mut [u8]), TryCastError<&mut [u8], T>>
```

### `swap`

```rust
fn swap<T, U>((t, u): (T, U)) -> (U, T)
```

### `try_read_from`

```rust
unsafe fn try_read_from<S, T: TryFromBytes>(source: S, candidate: core::mem::MaybeUninit<T>) -> Result<T, TryReadError<S, T>>
```

# Safety

All bytes of `candidate` must be initialized.

### `ref_from_prefix_suffix`

```rust
fn ref_from_prefix_suffix<T: FromBytes + KnownLayout + Immutable + ?Sized>(source: &[u8], meta: Option<<T as >::PointerMetadata>, cast_type: CastType) -> Result<(&T, &[u8]), CastError<&[u8], T>>
```

Interprets the given affix of the given bytes as a `&Self`.

This method computes the largest possible size of `Self` that can fit in the
prefix or suffix bytes of `source`, then attempts to return both a reference
to those bytes interpreted as a `Self`, and a reference to the excess bytes.
If there are insufficient bytes, or if that affix of `source` is not
appropriately aligned, this returns `Err`.

### `mut_from_prefix_suffix`

```rust
fn mut_from_prefix_suffix<T: FromBytes + IntoBytes + KnownLayout + ?Sized>(source: &mut [u8], meta: Option<<T as >::PointerMetadata>, cast_type: CastType) -> Result<(&mut T, &mut [u8]), CastError<&mut [u8], T>>
```

Interprets the given affix of the given bytes as a `&mut Self` without
copying.

This method computes the largest possible size of `Self` that can fit in the
prefix or suffix bytes of `source`, then attempts to return both a reference
to those bytes interpreted as a `Self`, and a reference to the excess bytes.
If there are insufficient bytes, or if that affix of `source` is not
appropriately aligned, this returns `Err`.

### `IntoBytes`

```rust
fn IntoBytes(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

### `ByteHash`

```rust
fn ByteHash(&mut self) -> &mut St
```

Acquires a mutable reference to the underlying sink or stream that this
combinator is pulling from.

Note that care must be taken to avoid tampering with the state of the
sink or stream which may otherwise confuse this combinator.

### `cast_for_sized`

```rust
unsafe fn cast_for_sized<'a, T, A, R, S>(ptr: crate::pointer::Ptr<'a, [u8], (A, crate::pointer::invariant::Aligned, crate::pointer::invariant::Valid)>) -> crate::pointer::Ptr<'a, T, (A, crate::pointer::invariant::Unaligned, crate::pointer::invariant::Valid)>
where
    T: FromBytes + KnownLayout + ?Sized + TransmuteFromPtr<T, A, crate::pointer::invariant::Initialized, crate::pointer::invariant::Valid, crate::pointer::cast::IdCast, S>,
    A: crate::invariant::Aliasing,
    [u8]: MutationCompatible<T, A, crate::pointer::invariant::Initialized, crate::pointer::invariant::Initialized, R>
```

# Safety

`T: Sized` and `ptr`'s referent must have size `size_of::<T>()`.

## Type Aliases

### `NativeEndian`

```rust
type NativeEndian = LittleEndian;
```

The endianness used by this platform.

This is a type alias for [`BigEndian`](#bigendian) or [`LittleEndian`](#littleendian) depending on the
endianness of the target platform.

### `NetworkEndian`

```rust
type NetworkEndian = BigEndian;
```

The endianness used in many network protocols.

This is a type alias for [`BigEndian`](#bigendian).

### `BE`

```rust
type BE = BigEndian;
```

A type alias for [`BigEndian`](#bigendian).

### `LE`

```rust
type LE = LittleEndian;
```

A type alias for [`LittleEndian`](#littleendian).

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

This is like [`TryCastError`](#trycasterror), but for casts that are always well-aligned.
It is identical to `TryCastError`, except that its alignment error is
[`Infallible`](../hashbrown/index.md).

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

## Macros

### `transmute!`

Safely transmutes a value of one type to a value of another type of the same
size.

This macro behaves like an invocation of this function:

```ignore
const fn transmute<Src, Dst>(src: Src) -> Dst
where
    Src: IntoBytes,
    Dst: FromBytes,
    size_of::<Src>() == size_of::<Dst>(),
{
/*
    ...
*/
}
```

However, unlike a function, this macro can only be invoked when the types of
`Src` and `Dst` are completely concrete. The types `Src` and `Dst` are
inferred from the calling context; they cannot be explicitly specified in
the macro invocation.

Note that the `Src` produced by the expression `$e` will *not* be dropped.
Semantically, its bits will be copied into a new value of type `Dst`, the
original `Src` will be forgotten, and the value of type `Dst` will be
returned.

# `#![allow(shrink)]`

If `#![allow(shrink)]` is provided, `transmute!` additionally supports
transmutations that shrink the size of the value; e.g.:

```rust
use zerocopy::transmute;
let u: u32 = transmute!(#![allow(shrink)] 0u64);
assert_eq!(u, 0u32);
```

# Examples

```rust
use zerocopy::transmute;
let one_dimensional: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];

let two_dimensional: [[u8; 4]; 2] = transmute!(one_dimensional);

assert_eq!(two_dimensional, [[0, 1, 2, 3], [4, 5, 6, 7]]);
```

# Use in `const` contexts

This macro can be invoked in `const` contexts.

<h2 id='method.transmute.codegen'>
    <a class='doc-anchor' href='#method.transmute.codegen'>§</a>
    Code Generation
</h2>

 This abstraction is safe and cheap, but does not necessarily
 have zero runtime cost. The codegen you experience in practice
 will depend on optimization level, the layout of the destination
 type, and what the compiler can prove about the source.


<div class='codegen-tabs' style='--arity: 4'>
    <details name='tab-transmute' style='--n: 1'>
        <summary><h6>Format</h6></summary>
        <div>

 ```ignore
use zerocopy_derive::*;

// The only valid value of this type are the bytes `0xC0C0`.
#[derive(TryFromBytes, KnownLayout, Immutable, IntoBytes)]
#[repr(u16)]
pub enum C0C0 {
    _XC0C0 = 0xC0C0,
}

macro_rules! define_packet {
    ($name: ident, $trait: ident, $leading_field: ty) => {
        #[derive($trait, KnownLayout, Immutable, IntoBytes)]
        #[repr(C, align(2))]
        pub struct $name {
            magic_number: $leading_field,
            mug_size: u8,
            temperature: u8,
            marshmallows: [u8; 2],
        }
    };
}

/// Packet begins with bytes 0xC0C0.
define_packet!(CocoPacket, TryFromBytes, C0C0);

/// Packet begins with any two bytes.
define_packet!(LocoPacket, FromBytes, [u8; 2]);

 ```

</div>
    </details>
    <details name='tab-transmute' style='--n: 2'>
        <summary><h6>Benchmark</h6></summary>
        <div>

 ```ignore
use zerocopy::Unalign;
use zerocopy_derive::*;

#[path = "formats/coco_static_size.rs"]
mod format;

#[derive(IntoBytes, KnownLayout, Immutable)]
#[repr(C)]
struct MinimalViableSource {
    bytes: [u8; 6],
}

#[unsafe(no_mangle)]
fn bench_transmute(source: MinimalViableSource) -> Unalign<format::LocoPacket> {
    zerocopy::transmute!(source)
}

 ```

</div>
    </details>
    <details name='tab-transmute' style='--n: 3'open>
        <summary><h6>Assembly</h6></summary>
        <div>

 ```plain
bench_transmute:
	mov rax, rdi
	ret

 ```

</div>
    </details>
    <details name='tab-transmute' style='--n: 4'>
        <summary><h6>Machine Code Analysis</h6></summary>
        <div>

 ```plain
Iterations:        100
Instructions:      200
Total Cycles:      104
Total uOps:        200

Dispatch Width:    4
uOps Per Cycle:    1.92
IPC:               1.92
Block RThroughput: 1.0


Instruction Info:





[6]: HasSideEffects (U)

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.33                        mov	rax, rdi
 1      1     1.00                  U     ret


Resources:
[0]   - SBDivider
[1]   - SBFPDivider
[2]   - SBPort0
[3]   - SBPort1
[4]   - SBPort4
[5]   - SBPort5
[6.0] - SBPort23
[6.1] - SBPort23


Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6.0]  [6.1]  
 -      -     0.49   0.50    -     1.01    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6.0]  [6.1]  Instructions:
 -      -     0.49   0.50    -     0.01    -      -     mov	rax, rdi
 -      -      -      -      -     1.00    -      -     ret

 ```

</div>
    </details></div>

### `transmute_ref!`

Safely transmutes a mutable or immutable reference of one type to an
immutable reference of another type of the same size and compatible
alignment.

This macro behaves like an invocation of this function:

```ignore
fn transmute_ref<'src, 'dst, Src, Dst>(src: &'src Src) -> &'dst Dst
where
    'src: 'dst,
    Src: IntoBytes + Immutable + ?Sized,
    Dst: FromBytes + Immutable + ?Sized,
    align_of::<Src>() >= align_of::<Dst>(),
    size_compatible::<Src, Dst>(),
{
/*
    ...
*/
}
```

The types `Src` and `Dst` are inferred from the calling context; they cannot
be explicitly specified in the macro invocation.

# Size compatibility

`transmute_ref!` supports transmuting between `Sized` types, between unsized
(i.e., `?Sized`) types, and from a `Sized` type to an unsized type. It
supports any transmutation that preserves the number of bytes of the
referent, even if doing so requires updating the metadata stored in an
unsized "fat" reference:

```rust
use zerocopy::transmute_ref;
use core::mem::size_of_val; // Not in the prelude on our MSRV
let src: &[[u8; 2]] = &[[0, 1], [2, 3]][..];
let dst: &[u8] = transmute_ref!(src);

assert_eq!(src.len(), 2);
assert_eq!(dst.len(), 4);
assert_eq!(dst, [0, 1, 2, 3]);
assert_eq!(size_of_val(src), size_of_val(dst));
```

# Errors

Violations of the alignment and size compatibility checks are detected
*after* the compiler performs monomorphization. This has two important
consequences.

First, it means that generic code will *never* fail these conditions:

```rust
use zerocopy::{transmute_ref, FromBytes, IntoBytes, Immutable};
fn transmute_ref<Src, Dst>(src: &Src) -> &Dst
where
    Src: IntoBytes + Immutable,
    Dst: FromBytes + Immutable,
{
    transmute_ref!(src)
}
```

Instead, failures will only be detected once generic code is instantiated
with concrete types:

```compile_fail,E0080
use zerocopy::{transmute_ref, FromBytes, IntoBytes, Immutable};

fn transmute_ref<Src, Dst>(src: &Src) -> &Dst
where
    Src: IntoBytes + Immutable,
    Dst: FromBytes + Immutable,
{
    transmute_ref!(src)
}
let src: &u16 = &0;
let dst: &u8 = transmute_ref(src);
```

Second, the fact that violations are detected after monomorphization means
that `cargo check` will usually not detect errors, even when types are
concrete. Instead, `cargo build` must be used to detect such errors.

# Examples

Transmuting between `Sized` types:

```rust
use zerocopy::transmute_ref;
let one_dimensional: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];

let two_dimensional: &[[u8; 4]; 2] = transmute_ref!(&one_dimensional);

assert_eq!(two_dimensional, &[[0, 1, 2, 3], [4, 5, 6, 7]]);
```

Transmuting between unsized types:

```rust
use {zerocopy::*, zerocopy_derive::*};
type u16 = zerocopy::byteorder::native_endian::U16;
type u32 = zerocopy::byteorder::native_endian::U32;
#[derive(KnownLayout, FromBytes, IntoBytes, Immutable)]
#[repr(C)]
struct SliceDst<T, U> {
    t: T,
    u: [U],
}

type Src = SliceDst<u32, u16>;
type Dst = SliceDst<u16, u8>;

let src = Src::ref_from_bytes(&[0, 1, 2, 3, 4, 5, 6, 7]).unwrap();
let dst: &Dst = transmute_ref!(src);

assert_eq!(src.t.as_bytes(), [0, 1, 2, 3]);
assert_eq!(src.u.len(), 2);
assert_eq!(src.u.as_bytes(), [4, 5, 6, 7]);

assert_eq!(dst.t.as_bytes(), [0, 1]);
assert_eq!(dst.u, [2, 3, 4, 5, 6, 7]);
```

# Use in `const` contexts

This macro can be invoked in `const` contexts only when `Src: Sized` and
`Dst: Sized`.

<h2 id='method.transmute_ref.codegen'>
    <a class='doc-anchor' href='#method.transmute_ref.codegen'>§</a>
    Code Generation
</h2>

 This abstraction is safe and cheap, but does not necessarily
 have zero runtime cost. The codegen you experience in practice
 will depend on optimization level, the layout of the destination
 type, and what the compiler can prove about the source.


 The below examples illustrate typical codegen for
 increasingly complex types:


<div class='codegen-tabs' style='--arity: 2'>
    <details name='tab-transmute_ref' style='--n: 1'open>
        <summary><h6>Sized</h6></summary>
        <div>


<div class='codegen-tabs' style='--arity: 4'>
    <details name='tab-transmute_ref_static_size' style='--n: 1'>
        <summary><h6>Format</h6></summary>
        <div>

 ```ignore
use zerocopy_derive::*;

// The only valid value of this type are the bytes `0xC0C0`.
#[derive(TryFromBytes, KnownLayout, Immutable, IntoBytes)]
#[repr(u16)]
pub enum C0C0 {
    _XC0C0 = 0xC0C0,
}

macro_rules! define_packet {
    ($name: ident, $trait: ident, $leading_field: ty) => {
        #[derive($trait, KnownLayout, Immutable, IntoBytes)]
        #[repr(C, align(2))]
        pub struct $name {
            magic_number: $leading_field,
            mug_size: u8,
            temperature: u8,
            marshmallows: [u8; 2],
        }
    };
}

/// Packet begins with bytes 0xC0C0.
define_packet!(CocoPacket, TryFromBytes, C0C0);

/// Packet begins with any two bytes.
define_packet!(LocoPacket, FromBytes, [u8; 2]);

 ```

</div>
    </details>
    <details name='tab-transmute_ref_static_size' style='--n: 2'>
        <summary><h6>Benchmark</h6></summary>
        <div>

 ```ignore
use zerocopy_derive::*;

#[path = "formats/coco_static_size.rs"]
mod format;

#[derive(IntoBytes, KnownLayout, Immutable)]
#[repr(C, align(2))]
struct MinimalViableSource {
    bytes: [u8; 6],
}

#[unsafe(no_mangle)]
fn bench_transmute_ref_static_size(source: &MinimalViableSource) -> &format::LocoPacket {
    zerocopy::transmute_ref!(source)
}

 ```

</div>
    </details>
    <details name='tab-transmute_ref_static_size' style='--n: 3'open>
        <summary><h6>Assembly</h6></summary>
        <div>

 ```plain
bench_transmute_ref_static_size:
	mov rax, rdi
	ret

 ```

</div>
    </details>
    <details name='tab-transmute_ref_static_size' style='--n: 4'>
        <summary><h6>Machine Code Analysis</h6></summary>
        <div>

 ```plain
Iterations:        100
Instructions:      200
Total Cycles:      104
Total uOps:        200

Dispatch Width:    4
uOps Per Cycle:    1.92
IPC:               1.92
Block RThroughput: 1.0


Instruction Info:





[6]: HasSideEffects (U)

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.33                        mov	rax, rdi
 1      1     1.00                  U     ret


Resources:
[0]   - SBDivider
[1]   - SBFPDivider
[2]   - SBPort0
[3]   - SBPort1
[4]   - SBPort4
[5]   - SBPort5
[6.0] - SBPort23
[6.1] - SBPort23


Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6.0]  [6.1]  
 -      -     0.49   0.50    -     1.01    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6.0]  [6.1]  Instructions:
 -      -     0.49   0.50    -     0.01    -      -     mov	rax, rdi
 -      -      -      -      -     1.00    -      -     ret

 ```

</div>
    </details></div>

</div>
    </details>
    <details name='tab-transmute_ref' style='--n: 2'>
        <summary><h6>Unsized</h6></summary>
        <div>


<div class='codegen-tabs' style='--arity: 4'>
    <details name='tab-transmute_ref_dynamic_size' style='--n: 1'>
        <summary><h6>Format</h6></summary>
        <div>

 ```ignore
use zerocopy_derive::*;

// The only valid value of this type are the bytes `0xC0C0`.
#[derive(TryFromBytes, KnownLayout, Immutable, IntoBytes)]
#[repr(u16)]
pub enum C0C0 {
    _XC0C0 = 0xC0C0,
}

macro_rules! define_packet {
    ($name: ident, $trait: ident, $leading_field: ty) => {
        #[derive($trait, KnownLayout, Immutable, IntoBytes, SplitAt)]
        #[repr(C, align(2))]
        pub struct $name {
            magic_number: $leading_field,
            mug_size: u8,
            temperature: u8,
            marshmallows: [[u8; 2]],
        }
    };
}

/// Packet begins with bytes 0xC0C0.
define_packet!(CocoPacket, TryFromBytes, C0C0);

/// Packet begins with any two bytes.
define_packet!(LocoPacket, FromBytes, [u8; 2]);

 ```

</div>
    </details>
    <details name='tab-transmute_ref_dynamic_size' style='--n: 2'>
        <summary><h6>Benchmark</h6></summary>
        <div>

 ```ignore
use zerocopy_derive::*;

#[path = "formats/coco_dynamic_size.rs"]
mod format;

#[derive(IntoBytes, KnownLayout, Immutable)]
#[repr(C, align(2))]
struct MinimalViableSource {
    header: [u8; 6],
    trailer: [[u8; 2]],
}

#[unsafe(no_mangle)]
fn bench_transmute_ref_dynamic_size(source: &MinimalViableSource) -> &format::LocoPacket {
    zerocopy::transmute_ref!(source)
}

 ```

</div>
    </details>
    <details name='tab-transmute_ref_dynamic_size' style='--n: 3'open>
        <summary><h6>Assembly</h6></summary>
        <div>

 ```plain
bench_transmute_ref_dynamic_size:
	mov rax, rdi
	lea rdx, [rsi + 1]
	ret

 ```

</div>
    </details>
    <details name='tab-transmute_ref_dynamic_size' style='--n: 4'>
        <summary><h6>Machine Code Analysis</h6></summary>
        <div>

 ```plain
Iterations:        100
Instructions:      300
Total Cycles:      104
Total uOps:        300

Dispatch Width:    4
uOps Per Cycle:    2.88
IPC:               2.88
Block RThroughput: 1.0


Instruction Info:





[6]: HasSideEffects (U)

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.33                        mov	rax, rdi
 1      1     0.50                        lea	rdx, [rsi + 1]
 1      1     1.00                  U     ret


Resources:
[0]   - SBDivider
[1]   - SBFPDivider
[2]   - SBPort0
[3]   - SBPort1
[4]   - SBPort4
[5]   - SBPort5
[6.0] - SBPort23
[6.1] - SBPort23


Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6.0]  [6.1]  
 -      -     0.99   1.00    -     1.01    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6.0]  [6.1]  Instructions:
 -      -     0.99    -      -     0.01    -      -     mov	rax, rdi
 -      -      -     1.00    -      -      -      -     lea	rdx, [rsi + 1]
 -      -      -      -      -     1.00    -      -     ret

 ```

</div>
    </details></div>

</div>
    </details></div>

### `transmute_mut!`

Safely transmutes a mutable reference of one type to a mutable reference of
another type of the same size and compatible alignment.

This macro behaves like an invocation of this function:

```ignore
const fn transmute_mut<'src, 'dst, Src, Dst>(src: &'src mut Src) -> &'dst mut Dst
where
    'src: 'dst,
    Src: FromBytes + IntoBytes + ?Sized,
    Dst: FromBytes + IntoBytes + ?Sized,
    align_of::<Src>() >= align_of::<Dst>(),
    size_compatible::<Src, Dst>(),
{
/*
    ...
*/
}
```

The types `Src` and `Dst` are inferred from the calling context; they cannot
be explicitly specified in the macro invocation.

# Size compatibility

`transmute_mut!` supports transmuting between `Sized` types, between unsized
(i.e., `?Sized`) types, and from a `Sized` type to an unsized type. It
supports any transmutation that preserves the number of bytes of the
referent, even if doing so requires updating the metadata stored in an
unsized "fat" reference:

```rust
use zerocopy::transmute_mut;
use core::mem::size_of_val; // Not in the prelude on our MSRV
let src: &mut [[u8; 2]] = &mut [[0, 1], [2, 3]][..];
let dst: &mut [u8] = transmute_mut!(src);

assert_eq!(dst.len(), 4);
assert_eq!(dst, [0, 1, 2, 3]);
let dst_size = size_of_val(dst);
assert_eq!(src.len(), 2);
assert_eq!(size_of_val(src), dst_size);
```

# Errors

Violations of the alignment and size compatibility checks are detected
*after* the compiler performs monomorphization. This has two important
consequences.

First, it means that generic code will *never* fail these conditions:

```rust
use zerocopy::{transmute_mut, FromBytes, IntoBytes, Immutable};
fn transmute_mut<Src, Dst>(src: &mut Src) -> &mut Dst
where
    Src: FromBytes + IntoBytes,
    Dst: FromBytes + IntoBytes,
{
    transmute_mut!(src)
}
```

Instead, failures will only be detected once generic code is instantiated
with concrete types:

```compile_fail,E0080
use zerocopy::{transmute_mut, FromBytes, IntoBytes, Immutable};

fn transmute_mut<Src, Dst>(src: &mut Src) -> &mut Dst
where
    Src: FromBytes + IntoBytes,
    Dst: FromBytes + IntoBytes,
{
    transmute_mut!(src)
}
let src: &mut u16 = &mut 0;
let dst: &mut u8 = transmute_mut(src);
```

Second, the fact that violations are detected after monomorphization means
that `cargo check` will usually not detect errors, even when types are
concrete. Instead, `cargo build` must be used to detect such errors.


# Examples

Transmuting between `Sized` types:

```rust
use zerocopy::transmute_mut;
let mut one_dimensional: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];

let two_dimensional: &mut [[u8; 4]; 2] = transmute_mut!(&mut one_dimensional);

assert_eq!(two_dimensional, &[[0, 1, 2, 3], [4, 5, 6, 7]]);

two_dimensional.reverse();

assert_eq!(one_dimensional, [4, 5, 6, 7, 0, 1, 2, 3]);
```

Transmuting between unsized types:

```rust
use {zerocopy::*, zerocopy_derive::*};
type u16 = zerocopy::byteorder::native_endian::U16;
type u32 = zerocopy::byteorder::native_endian::U32;
#[derive(KnownLayout, FromBytes, IntoBytes, Immutable)]
#[repr(C)]
struct SliceDst<T, U> {
    t: T,
    u: [U],
}

type Src = SliceDst<u32, u16>;
type Dst = SliceDst<u16, u8>;

let mut bytes = [0, 1, 2, 3, 4, 5, 6, 7];
let src = Src::mut_from_bytes(&mut bytes[..]).unwrap();
let dst: &mut Dst = transmute_mut!(src);

assert_eq!(dst.t.as_bytes(), [0, 1]);
assert_eq!(dst.u, [2, 3, 4, 5, 6, 7]);

assert_eq!(src.t.as_bytes(), [0, 1, 2, 3]);
assert_eq!(src.u.len(), 2);
assert_eq!(src.u.as_bytes(), [4, 5, 6, 7]);
```

### `try_transmute!`

Conditionally transmutes a value of one type to a value of another type of
the same size.

This macro behaves like an invocation of this function:

```ignore
fn try_transmute<Src, Dst>(src: Src) -> Result<Dst, ValidityError<Src, Dst>>
where
    Src: IntoBytes,
    Dst: TryFromBytes,
    size_of::<Src>() == size_of::<Dst>(),
{
/*
    ...
*/
}
```

However, unlike a function, this macro can only be invoked when the types of
`Src` and `Dst` are completely concrete. The types `Src` and `Dst` are
inferred from the calling context; they cannot be explicitly specified in
the macro invocation.

Note that the `Src` produced by the expression `$e` will *not* be dropped.
Semantically, its bits will be copied into a new value of type `Dst`, the
original `Src` will be forgotten, and the value of type `Dst` will be
returned.

# Examples

```rust
use zerocopy::*;
// 0u8 → bool = false
assert_eq!(try_transmute!(0u8), Ok(false));

// 1u8 → bool = true
 assert_eq!(try_transmute!(1u8), Ok(true));

// 2u8 → bool = error
assert!(matches!(
    try_transmute!(2u8),
    Result::<bool, _>::Err(ValidityError { .. })
));
```

<h2 id='method.try_transmute.codegen'>
    <a class='doc-anchor' href='#method.try_transmute.codegen'>§</a>
    Code Generation
</h2>

 This abstraction is safe and cheap, but does not necessarily
 have zero runtime cost. The codegen you experience in practice
 will depend on optimization level, the layout of the destination
 type, and what the compiler can prove about the source.


<div class='codegen-tabs' style='--arity: 4'>
    <details name='tab-try_transmute' style='--n: 1'>
        <summary><h6>Format</h6></summary>
        <div>

 ```ignore
use zerocopy_derive::*;

// The only valid value of this type are the bytes `0xC0C0`.
#[derive(TryFromBytes, KnownLayout, Immutable, IntoBytes)]
#[repr(u16)]
pub enum C0C0 {
    _XC0C0 = 0xC0C0,
}

macro_rules! define_packet {
    ($name: ident, $trait: ident, $leading_field: ty) => {
        #[derive($trait, KnownLayout, Immutable, IntoBytes)]
        #[repr(C, align(2))]
        pub struct $name {
            magic_number: $leading_field,
            mug_size: u8,
            temperature: u8,
            marshmallows: [u8; 2],
        }
    };
}

/// Packet begins with bytes 0xC0C0.
define_packet!(CocoPacket, TryFromBytes, C0C0);

/// Packet begins with any two bytes.
define_packet!(LocoPacket, FromBytes, [u8; 2]);

 ```

</div>
    </details>
    <details name='tab-try_transmute' style='--n: 2'>
        <summary><h6>Benchmark</h6></summary>
        <div>

 ```ignore
use zerocopy::Unalign;
use zerocopy_derive::*;

#[path = "formats/coco_static_size.rs"]
mod format;

#[derive(IntoBytes, KnownLayout, Immutable)]
#[repr(C)]
struct MinimalViableSource {
    bytes: [u8; 6],
}

#[unsafe(no_mangle)]
fn bench_try_transmute(source: MinimalViableSource) -> Option<Unalign<format::CocoPacket>> {
    zerocopy::try_transmute!(source).ok()
}

 ```

</div>
    </details>
    <details name='tab-try_transmute' style='--n: 3'open>
        <summary><h6>Assembly</h6></summary>
        <div>

 ```plain
bench_try_transmute:
	movzx ecx, di
	xor eax, eax
	cmp ecx, 49344
	sete al
	and rdi, -65536
	xor rax, 49345
	or rax, rdi
	ret

 ```

</div>
    </details>
    <details name='tab-try_transmute' style='--n: 4'>
        <summary><h6>Machine Code Analysis</h6></summary>
        <div>

 ```plain
Iterations:        100
Instructions:      800
Total Cycles:      238
Total uOps:        800

Dispatch Width:    4
uOps Per Cycle:    3.36
IPC:               3.36
Block RThroughput: 2.0


Instruction Info:





[6]: HasSideEffects (U)

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.33                        movzx	ecx, di
 1      0     0.25                        xor	eax, eax
 1      1     0.33                        cmp	ecx, 49344
 1      1     0.50                        sete	al
 1      1     0.33                        and	rdi, -65536
 1      1     0.33                        xor	rax, 49345
 1      1     0.33                        or	rax, rdi
 1      1     1.00                  U     ret


Resources:
[0]   - SBDivider
[1]   - SBFPDivider
[2]   - SBPort0
[3]   - SBPort1
[4]   - SBPort4
[5]   - SBPort5
[6.0] - SBPort23
[6.1] - SBPort23


Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6.0]  [6.1]  
 -      -     2.33   2.33    -     2.34    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6.0]  [6.1]  Instructions:
 -      -     0.32   0.67    -     0.01    -      -     movzx	ecx, di
 -      -      -      -      -      -      -      -     xor	eax, eax
 -      -     0.33   0.67    -      -      -      -     cmp	ecx, 49344
 -      -     1.00    -      -      -      -      -     sete	al
 -      -     0.67   0.33    -      -      -      -     and	rdi, -65536
 -      -      -     0.66    -     0.34    -      -     xor	rax, 49345
 -      -     0.01    -      -     0.99    -      -     or	rax, rdi
 -      -      -      -      -     1.00    -      -     ret

 ```

</div>
    </details></div>

### `try_transmute_ref!`

Conditionally transmutes a mutable or immutable reference of one type to an
immutable reference of another type of the same size and compatible
alignment.

*Note that while the **value** of the referent is checked for validity at
runtime, the **size** and **alignment** are checked at compile time. For
conversions which are fallible with respect to size and alignment, see the
methods on [`TryFromBytes`](#tryfrombytes).*

This macro behaves like an invocation of this function:

```ignore
fn try_transmute_ref<Src, Dst>(src: &Src) -> Result<&Dst, ValidityError<&Src, Dst>>
where
    Src: IntoBytes + Immutable + ?Sized,
    Dst: TryFromBytes + Immutable + ?Sized,
    align_of::<Src>() >= align_of::<Dst>(),
    size_compatible::<Src, Dst>(),
{
/*
    ...
*/
}
```

The types `Src` and `Dst` are inferred from the calling context; they cannot
be explicitly specified in the macro invocation.

# Size compatibility

`try_transmute_ref!` supports transmuting between `Sized` types, between
unsized (i.e., `?Sized`) types, and from a `Sized` type to an unsized type.
It supports any transmutation that preserves the number of bytes of the
referent, even if doing so requires updating the metadata stored in an
unsized "fat" reference:

```rust
use zerocopy::try_transmute_ref;
use core::mem::size_of_val; // Not in the prelude on our MSRV
let src: &[[u8; 2]] = &[[0, 1], [2, 3]][..];
let dst: &[u8] = try_transmute_ref!(src).unwrap();

assert_eq!(src.len(), 2);
assert_eq!(dst.len(), 4);
assert_eq!(dst, [0, 1, 2, 3]);
assert_eq!(size_of_val(src), size_of_val(dst));
```

# Examples

Transmuting between `Sized` types:

```rust
use zerocopy::*;
// 0u8 → bool = false
assert_eq!(try_transmute_ref!(&0u8), Ok(&false));

// 1u8 → bool = true
 assert_eq!(try_transmute_ref!(&1u8), Ok(&true));

// 2u8 → bool = error
assert!(matches!(
    try_transmute_ref!(&2u8),
    Result::<&bool, _>::Err(ValidityError { .. })
));
```

Transmuting between unsized types:

```rust
use {zerocopy::*, zerocopy_derive::*};
type u16 = zerocopy::byteorder::native_endian::U16;
type u32 = zerocopy::byteorder::native_endian::U32;
#[derive(KnownLayout, FromBytes, IntoBytes, Immutable)]
#[repr(C)]
struct SliceDst<T, U> {
    t: T,
    u: [U],
}

type Src = SliceDst<u32, u16>;
type Dst = SliceDst<u16, bool>;

let src = Src::ref_from_bytes(&[0, 1, 0, 1, 0, 1, 0, 1]).unwrap();
let dst: &Dst = try_transmute_ref!(src).unwrap();

assert_eq!(src.t.as_bytes(), [0, 1, 0, 1]);
assert_eq!(src.u.len(), 2);
assert_eq!(src.u.as_bytes(), [0, 1, 0, 1]);

assert_eq!(dst.t.as_bytes(), [0, 1]);
assert_eq!(dst.u, [false, true, false, true, false, true]);
```

<h2 id='method.try_transmute_ref.codegen'>
    <a class='doc-anchor' href='#method.try_transmute_ref.codegen'>§</a>
    Code Generation
</h2>

 This abstraction is safe and cheap, but does not necessarily
 have zero runtime cost. The codegen you experience in practice
 will depend on optimization level, the layout of the destination
 type, and what the compiler can prove about the source.


 The below examples illustrate typical codegen for
 increasingly complex types:


<div class='codegen-tabs' style='--arity: 2'>
    <details name='tab-try_transmute_ref' style='--n: 1'open>
        <summary><h6>Sized</h6></summary>
        <div>


<div class='codegen-tabs' style='--arity: 4'>
    <details name='tab-try_transmute_ref_static_size' style='--n: 1'>
        <summary><h6>Format</h6></summary>
        <div>

 ```ignore
use zerocopy_derive::*;

// The only valid value of this type are the bytes `0xC0C0`.
#[derive(TryFromBytes, KnownLayout, Immutable, IntoBytes)]
#[repr(u16)]
pub enum C0C0 {
    _XC0C0 = 0xC0C0,
}

macro_rules! define_packet {
    ($name: ident, $trait: ident, $leading_field: ty) => {
        #[derive($trait, KnownLayout, Immutable, IntoBytes)]
        #[repr(C, align(2))]
        pub struct $name {
            magic_number: $leading_field,
            mug_size: u8,
            temperature: u8,
            marshmallows: [u8; 2],
        }
    };
}

/// Packet begins with bytes 0xC0C0.
define_packet!(CocoPacket, TryFromBytes, C0C0);

/// Packet begins with any two bytes.
define_packet!(LocoPacket, FromBytes, [u8; 2]);

 ```

</div>
    </details>
    <details name='tab-try_transmute_ref_static_size' style='--n: 2'>
        <summary><h6>Benchmark</h6></summary>
        <div>

 ```ignore
use zerocopy_derive::*;

#[path = "formats/coco_static_size.rs"]
mod format;

#[derive(IntoBytes, KnownLayout, Immutable)]
#[repr(C, align(2))]
struct MinimalViableSource {
    bytes: [u8; 6],
}

#[unsafe(no_mangle)]
fn bench_try_transmute_ref_static_size(
    source: &MinimalViableSource,
) -> Option<&format::CocoPacket> {
    zerocopy::try_transmute_ref!(source).ok()
}

 ```

</div>
    </details>
    <details name='tab-try_transmute_ref_static_size' style='--n: 3'open>
        <summary><h6>Assembly</h6></summary>
        <div>

 ```plain
bench_try_transmute_ref_static_size:
	xor eax, eax
	cmp word ptr [rdi], -16192
	cmove rax, rdi
	ret

 ```

</div>
    </details>
    <details name='tab-try_transmute_ref_static_size' style='--n: 4'>
        <summary><h6>Machine Code Analysis</h6></summary>
        <div>

 ```plain
Iterations:        100
Instructions:      400
Total Cycles:      160
Total uOps:        600

Dispatch Width:    4
uOps Per Cycle:    3.75
IPC:               2.50
Block RThroughput: 1.5


Instruction Info:





[6]: HasSideEffects (U)

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.25                        xor	eax, eax
 2      6     0.50    *                   cmp	word ptr [rdi], -16192
 2      2     0.67                        cmove	rax, rdi
 1      1     1.00                  U     ret


Resources:
[0]   - SBDivider
[1]   - SBFPDivider
[2]   - SBPort0
[3]   - SBPort1
[4]   - SBPort4
[5]   - SBPort5
[6.0] - SBPort23
[6.1] - SBPort23


Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6.0]  [6.1]  
 -      -     1.02   1.48    -     1.50   0.50   0.50   

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6.0]  [6.1]  Instructions:
 -      -      -      -      -      -      -      -     xor	eax, eax
 -      -     0.02   0.49    -     0.49   0.50   0.50   cmp	word ptr [rdi], -16192
 -      -     1.00   0.99    -     0.01    -      -     cmove	rax, rdi
 -      -      -      -      -     1.00    -      -     ret

 ```

</div>
    </details></div>

</div>
    </details>
    <details name='tab-try_transmute_ref' style='--n: 2'>
        <summary><h6>Unsized</h6></summary>
        <div>


<div class='codegen-tabs' style='--arity: 4'>
    <details name='tab-try_transmute_ref_dynamic_size' style='--n: 1'>
        <summary><h6>Format</h6></summary>
        <div>

 ```ignore
use zerocopy_derive::*;

// The only valid value of this type are the bytes `0xC0C0`.
#[derive(TryFromBytes, KnownLayout, Immutable, IntoBytes)]
#[repr(u16)]
pub enum C0C0 {
    _XC0C0 = 0xC0C0,
}

macro_rules! define_packet {
    ($name: ident, $trait: ident, $leading_field: ty) => {
        #[derive($trait, KnownLayout, Immutable, IntoBytes, SplitAt)]
        #[repr(C, align(2))]
        pub struct $name {
            magic_number: $leading_field,
            mug_size: u8,
            temperature: u8,
            marshmallows: [[u8; 2]],
        }
    };
}

/// Packet begins with bytes 0xC0C0.
define_packet!(CocoPacket, TryFromBytes, C0C0);

/// Packet begins with any two bytes.
define_packet!(LocoPacket, FromBytes, [u8; 2]);

 ```

</div>
    </details>
    <details name='tab-try_transmute_ref_dynamic_size' style='--n: 2'>
        <summary><h6>Benchmark</h6></summary>
        <div>

 ```ignore
use zerocopy_derive::*;

#[path = "formats/coco_dynamic_size.rs"]
mod format;

#[derive(IntoBytes, KnownLayout, Immutable)]
#[repr(C, align(2))]
struct MinimalViableSource {
    header: [u8; 6],
    trailer: [[u8; 2]],
}

#[unsafe(no_mangle)]
fn bench_try_transmute_ref_dynamic_size(
    source: &MinimalViableSource,
) -> Option<&format::CocoPacket> {
    zerocopy::try_transmute_ref!(source).ok()
}

 ```

</div>
    </details>
    <details name='tab-try_transmute_ref_dynamic_size' style='--n: 3'open>
        <summary><h6>Assembly</h6></summary>
        <div>

 ```plain
bench_try_transmute_ref_dynamic_size:
	lea rdx, [rsi + 1]
	xor eax, eax
	cmp word ptr [rdi], -16192
	cmove rax, rdi
	ret

 ```

</div>
    </details>
    <details name='tab-try_transmute_ref_dynamic_size' style='--n: 4'>
        <summary><h6>Machine Code Analysis</h6></summary>
        <div>

 ```plain
Iterations:        100
Instructions:      500
Total Cycles:      209
Total uOps:        700

Dispatch Width:    4
uOps Per Cycle:    3.35
IPC:               2.39
Block RThroughput: 1.8


Instruction Info:





[6]: HasSideEffects (U)

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        lea	rdx, [rsi + 1]
 1      0     0.25                        xor	eax, eax
 2      6     0.50    *                   cmp	word ptr [rdi], -16192
 2      2     0.67                        cmove	rax, rdi
 1      1     1.00                  U     ret


Resources:
[0]   - SBDivider
[1]   - SBFPDivider
[2]   - SBPort0
[3]   - SBPort1
[4]   - SBPort4
[5]   - SBPort5
[6.0] - SBPort23
[6.1] - SBPort23


Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6.0]  [6.1]  
 -      -     1.50   1.51    -     1.99   0.50   0.50   

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6.0]  [6.1]  Instructions:
 -      -     0.51   0.49    -      -      -      -     lea	rdx, [rsi + 1]
 -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -     0.02    -     0.98   0.50   0.50   cmp	word ptr [rdi], -16192
 -      -     0.99   1.00    -     0.01    -      -     cmove	rax, rdi
 -      -      -      -      -     1.00    -      -     ret

 ```

</div>
    </details></div>

</div>
    </details></div>

### `try_transmute_mut!`

Conditionally transmutes a mutable reference of one type to a mutable
reference of another type of the same size and compatible alignment.

*Note that while the **value** of the referent is checked for validity at
runtime, the **size** and **alignment** are checked at compile time. For
conversions which are fallible with respect to size and alignment, see the
methods on [`TryFromBytes`](#tryfrombytes).*

This macro behaves like an invocation of this function:

```ignore
fn try_transmute_mut<Src, Dst>(src: &mut Src) -> Result<&mut Dst, ValidityError<&mut Src, Dst>>
where
    Src: FromBytes + IntoBytes + ?Sized,
    Dst: TryFromBytes + IntoBytes + ?Sized,
    align_of::<Src>() >= align_of::<Dst>(),
    size_compatible::<Src, Dst>(),
{
/*
    ...
*/
}
```

The types `Src` and `Dst` are inferred from the calling context; they cannot
be explicitly specified in the macro invocation.

# Size compatibility

`try_transmute_mut!` supports transmuting between `Sized` types, between
unsized (i.e., `?Sized`) types, and from a `Sized` type to an unsized type.
It supports any transmutation that preserves the number of bytes of the
referent, even if doing so requires updating the metadata stored in an
unsized "fat" reference:

```rust
use zerocopy::try_transmute_mut;
use core::mem::size_of_val; // Not in the prelude on our MSRV
let src: &mut [[u8; 2]] = &mut [[0, 1], [2, 3]][..];
let dst: &mut [u8] = try_transmute_mut!(src).unwrap();

assert_eq!(dst.len(), 4);
assert_eq!(dst, [0, 1, 2, 3]);
let dst_size = size_of_val(dst);
assert_eq!(src.len(), 2);
assert_eq!(size_of_val(src), dst_size);
```

# Examples

Transmuting between `Sized` types:

```rust
use zerocopy::*;
// 0u8 → bool = false
let src = &mut 0u8;
assert_eq!(try_transmute_mut!(src), Ok(&mut false));

// 1u8 → bool = true
let src = &mut 1u8;
 assert_eq!(try_transmute_mut!(src), Ok(&mut true));

// 2u8 → bool = error
let src = &mut 2u8;
assert!(matches!(
    try_transmute_mut!(src),
    Result::<&mut bool, _>::Err(ValidityError { .. })
));
```

Transmuting between unsized types:

```rust
use {zerocopy::*, zerocopy_derive::*};
type u16 = zerocopy::byteorder::native_endian::U16;
type u32 = zerocopy::byteorder::native_endian::U32;
#[derive(KnownLayout, FromBytes, IntoBytes, Immutable)]
#[repr(C)]
struct SliceDst<T, U> {
    t: T,
    u: [U],
}

type Src = SliceDst<u32, u16>;
type Dst = SliceDst<u16, bool>;

let mut bytes = [0, 1, 0, 1, 0, 1, 0, 1];
let src = Src::mut_from_bytes(&mut bytes).unwrap();

assert_eq!(src.t.as_bytes(), [0, 1, 0, 1]);
assert_eq!(src.u.len(), 2);
assert_eq!(src.u.as_bytes(), [0, 1, 0, 1]);

let dst: &Dst = try_transmute_mut!(src).unwrap();

assert_eq!(dst.t.as_bytes(), [0, 1]);
assert_eq!(dst.u, [false, true, false, true, false, true]);
```

### `include_value!`

Includes a file and safely transmutes it to a value of an arbitrary type.

The file will be included as a byte array, `[u8; N]`, which will be
transmuted to another type, `T`. `T` is inferred from the calling context,
and must implement [`FromBytes`](#frombytes).

The file is located relative to the current file (similarly to how modules
are found). The provided path is interpreted in a platform-specific way at
compile time. So, for instance, an invocation with a Windows path containing
backslashes `\` would not compile correctly on Unix.

`include_value!` is ignorant of byte order. For byte order-aware types, see
the [`byteorder`](byteorder/index.md) module.


# Examples

Assume there are two files in the same directory with the following
contents:

File `data` (no trailing newline):

```text
abcd
```

File `main.rs`:

```rust
use zerocopy::include_value;
macro_rules! include_value {
($file:expr) => { zerocopy::include_value!(concat!("../testdata/include_value/", $file)) };
}

fn main() {
    let as_u32: u32 = include_value!("data");
    assert_eq!(as_u32, u32::from_ne_bytes([b'a', b'b', b'c', b'd']));
    let as_i32: i32 = include_value!("data");
    assert_eq!(as_i32, i32::from_ne_bytes([b'a', b'b', b'c', b'd']));
}
```

# Use in `const` contexts

This macro can be invoked in `const` contexts.

### `impl_fmt_trait!`

### `impl_fmt_traits!`

### `impl_ops_traits!`

### `doc_comment!`

### `define_max_value_constant!`

### `define_type!`

### `define_float_conversion!`

### `module!`

