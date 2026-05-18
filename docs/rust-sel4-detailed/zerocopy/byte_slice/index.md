*[zerocopy](../index.md) / [byte_slice](index.md)*

---

# Module `byte_slice`

Traits for types that encapsulate a `[u8]`.

These traits are used to bound the `B` parameter of [`Ref`](../ref/def/index.md).

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ByteSlice`](#byteslice) | trait | A mutable or immutable reference to a byte slice. |
| [`ByteSliceMut`](#byteslicemut) | trait | A mutable reference to a byte slice. |
| [`CopyableByteSlice`](#copyablebyteslice) | trait | A [`ByteSlice`] which can be copied without violating dereference stability. |
| [`CloneableByteSlice`](#cloneablebyteslice) | trait | A [`ByteSlice`] which can be cloned without violating dereference stability. |
| [`SplitByteSlice`](#splitbyteslice) | trait | A [`ByteSlice`] that can be split in two. |
| [`SplitByteSliceMut`](#splitbyteslicemut) | trait | A shorthand for [`SplitByteSlice`] and [`ByteSliceMut`]. |
| [`IntoByteSlice`](#intobyteslice) | trait | A [`ByteSlice`] that conveys no ownership, and so can be converted into a byte slice. |
| [`IntoByteSliceMut`](#intobyteslicemut) | trait | A [`ByteSliceMut`] that conveys no ownership, and so can be converted into a mutable byte slice. |

## Traits

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
[`Deref`](../../cpp_demangle/index.md) and `DerefMut` are "stable". In particular, given `B: ByteSlice`
and `b: B`, two calls, each to either `b.deref()` or `b.deref_mut()`, must
return a byte slice with the same address and length. This must hold even if
the two calls are separated by an arbitrary sequence of calls to methods on
`ByteSlice`, [`ByteSliceMut`](../index.md), [`IntoByteSlice`](../index.md), or [`IntoByteSliceMut`](../index.md),
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

`ByteSliceMut` is a shorthand for [`ByteSlice`](../index.md) and `DerefMut`.

#### Implementors

- `B`

### `CopyableByteSlice`

```rust
trait CopyableByteSlice: ByteSlice + Copy + CloneableByteSlice { ... }
```

A [`ByteSlice`](../index.md) which can be copied without violating dereference stability.

# Safety

If `B: CopyableByteSlice`, then the dereference stability properties
required by [`ByteSlice`](../index.md) (see that trait's safety documentation) do not
only hold regarding two calls to `b.deref()` or `b.deref_mut()`, but also
hold regarding `c.deref()` or `c.deref_mut()`, where `c` is produced by
copying `b`.

#### Implementors

- `&[u8]`

### `CloneableByteSlice`

```rust
trait CloneableByteSlice: ByteSlice + Clone { ... }
```

A [`ByteSlice`](../index.md) which can be cloned without violating dereference stability.

# Safety

If `B: CloneableByteSlice`, then the dereference stability properties
required by [`ByteSlice`](../index.md) (see that trait's safety documentation) do not
only hold regarding two calls to `b.deref()` or `b.deref_mut()`, but also
hold regarding `c.deref()` or `c.deref_mut()`, where `c` is produced by
`b.clone()`, `b.clone().clone()`, etc.

#### Implementors

- `&[u8]`

### `SplitByteSlice`

```rust
trait SplitByteSlice: ByteSlice { ... }
```

A [`ByteSlice`](../index.md) that can be split in two.

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

A shorthand for [`SplitByteSlice`](../index.md) and [`ByteSliceMut`](../index.md).

#### Implementors

- `B`

### `IntoByteSlice<'a>`

```rust
trait IntoByteSlice<'a>: ByteSlice { ... }
```

A [`ByteSlice`](../index.md) that conveys no ownership, and so can be converted into a
byte slice.

Some `ByteSlice` types (notably, the standard library's [`Ref`](../ref/def/index.md) type) convey
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

A [`ByteSliceMut`](../index.md) that conveys no ownership, and so can be converted into a
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

