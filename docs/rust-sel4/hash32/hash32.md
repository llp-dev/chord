**hash32**

# Module: hash32

## Contents

**Structs**

- [`BuildHasherDefault`](#buildhasherdefault) - A copy of [`core::hash::BuildHasherDefault`][0], but with a const constructor.

**Traits**

- [`Hasher`](#hasher) - An extension of [core::hash::Hasher][0] for hashers which use 32 bits.

---

## hash32::BuildHasherDefault

*Struct*

A copy of [`core::hash::BuildHasherDefault`][0], but with a const constructor.

This will eventually be deprecated once the version in `core` becomes const-constructible
(presumably using `const Default`).

[0]: https://doc.rust-lang.org/core/hash/struct.BuildHasherDefault.html

**Generic Parameters:**
- H

**Methods:**

- `fn new() -> Self` - `const` constructor

**Traits:** Eq

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, _other: &BuildHasherDefault<H>) -> bool`
- **BuildHasher**
  - `fn build_hasher(self: &Self) -> <Self as >::Hasher`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## hash32::Hasher

*Trait*

An extension of [core::hash::Hasher][0] for hashers which use 32 bits.

For hashers which implement this trait, the standard `finish` method should just return a
zero-extended version of the result of `finish32`.

[0]: https://doc.rust-lang.org/core/hash/trait.Hasher.html

# Contract

Implementers of this trait must *not* perform any 64-bit (or 128-bit) operation while computing
the hash.

**Methods:**

- `finish32`: The equivalent of [`core::hash::Hasher.finish`][0] for 32-bit hashers.



