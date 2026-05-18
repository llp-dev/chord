*[bytemuck](../index.md) / [no_uninit](index.md)*

---

# Module `no_uninit`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`NoUninit`](#nouninit) | trait | Marker trait for "plain old data" types with no uninit (or padding) bytes. |

## Traits

### `NoUninit`

```rust
trait NoUninit: Sized + Copy + 'static { ... }
```

Marker trait for "plain old data" types with no uninit (or padding) bytes.

The requirements for this is very similar to [`Pod`](../index.md),
except that it doesn't require that all bit patterns of the type are valid,
i.e. it does not require the type to be `Zeroable`.
This limits what you can do with a type of this kind, but also broadens the
included types to things like C-style enums. Notably, you can only cast from
*immutable* references to a [`NoUninit`](../index.md) type into *immutable* references of
any other type, no casting of mutable references or mutable references to
slices etc.

[`Pod`](../index.md) is a subset of [`NoUninit`](../index.md), meaning that any `T: Pod` is also
[`NoUninit`](../index.md) but any `T: NoUninit` is not necessarily [`Pod`](../index.md). If possible,
prefer implementing [`Pod`](../index.md) directly. To get more [`Pod`](../index.md)-like functionality
for a type that is only [`NoUninit`](../index.md), consider also implementing
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

The same as [`Pod`](../index.md) except we disregard the rule about it must
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

