*[zerocopy](../index.md) / [split_at](index.md)*

---

# Module `split_at`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Split`](#split) | struct | A `T` that has been split into two possibly-overlapping parts. |
| [`SplitAt`](#splitat) | trait | Types that can be split in two. |

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
[`#[derive(SplitAt)]`][`derive`](../../clap_builder/derive/index.md); e.g.:

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

