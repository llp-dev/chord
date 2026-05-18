# Crate `hash32`

32-bit hashing algorithms

# Why?

Because 32-bit architectures are a thing (e.g. ARM Cortex-M) and you don't want your hashing
function to pull in a bunch of slow 64-bit compiler intrinsics (software implementations of
64-bit operations).

# Relationship to `core::hash`

This crate extends `core::hash` with a 32-bit version of `Hasher`, which extends
`core::hash::Hasher`. It requires that the hasher only performs 32-bit operations when computing
the hash, and adds `finish32` to get the hasher's result as a `u32`. The standard `finish`
method should just zero-extend this result.

Since it extends `core::hash::Hasher`, `Hasher` can be used with any type which implements the
standard `Hash` trait.

This crate also adds a version of `BuildHasherDefault` with a const constructor, to work around
the `core` version's lack of one.


# Hashers

This crate provides implementations of the following 32-bit hashing algorithms:

- [Fowler-Noll-Vo](#fnvhasher)
- [MurmurHash3](#murmur3hasher)

# Generic code

In generic code, the trait bound `H: core::hash::Hasher` accepts *both* 64-bit hashers like
`std::collections::hash_map::DefaultHasher`; and 32-bit hashers like the ones defined in this
crate (`hash32::FnvHasher` and `hash32::Murmur3Hasher`)

The trait bound `H: hash32::Hasher` is *more* restrictive as it only accepts 32-bit hashers.

The `BuildHasherDefault<H>` type implements the `core::hash::BuildHasher` trait so it can
construct both 32-bit and 64-bit hashers. To constrain the type to only produce 32-bit hasher
you can add the trait bound `H::Hasher: hash32::Hasher`

# MSRV

This crate is guaranteed to compile on latest stable Rust. It *might* compile on older
versions but that may change in any new patch release.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`fnv`](#fnv) | mod |  |
| [`murmur3`](#murmur3) | mod |  |
| [`FnvHasher`](#fnvhasher) | struct |  |
| [`Murmur3Hasher`](#murmur3hasher) | struct |  |
| [`BuildHasherDefault`](#buildhasherdefault) | struct | A copy of [`core::hash::BuildHasherDefault`][0], but with a const constructor. |
| [`Hasher`](#hasher) | trait | An extension of [core::hash::Hasher][0] for hashers which use 32 bits. |

## Modules

- [`fnv`](fnv/index.md)
- [`murmur3`](murmur3/index.md)

## Structs

### `FnvHasher`

```rust
struct FnvHasher {
    state: u32,
}
```

32-bit Fowler-Noll-Vo hasher

#### Trait Implementations

##### `impl Default for Hasher`

- <span id="hasher-default"></span>`fn default() -> Self`

##### `impl Hasher for Hasher`

- <span id="hasher-hasher-finish32"></span>`fn finish32(&self) -> u32`

### `Murmur3Hasher`

```rust
struct Murmur3Hasher {
    buf: Buffer,
    index: Index,
    processed: u32,
    state: State,
}
```

32-bit MurmurHash3 hasher

#### Implementations

- <span id="hasher-push"></span>`fn push(&mut self, buf: &[u8])`

#### Trait Implementations

##### `impl Default for Hasher`

- <span id="hasher-default"></span>`fn default() -> Self`

##### `impl Hasher for Hasher`

- <span id="hasher-hasher-finish32"></span>`fn finish32(&self) -> u32`

### `BuildHasherDefault<H>`

```rust
struct BuildHasherDefault<H> {
    _marker: core::marker::PhantomData<H>,
}
```

A copy of [`core::hash::BuildHasherDefault`][0], but with a const constructor.

This will eventually be deprecated once the version in `core` becomes const-constructible
(presumably using `const Default`).


#### Implementations

- <span id="buildhasherdefault-new"></span>`const fn new() -> Self`

  `const` constructor

#### Trait Implementations

##### `impl<H> BuildHasher for BuildHasherDefault<H>`

- <span id="buildhasherdefault-buildhasher-type-hasher"></span>`type Hasher = H`

- <span id="buildhasherdefault-buildhasher-build-hasher"></span>`fn build_hasher(&self) -> <Self as >::Hasher`

##### `impl<H> Clone for BuildHasherDefault<H>`

- <span id="buildhasherdefault-clone"></span>`fn clone(&self) -> Self`

##### `impl<H> Debug for BuildHasherDefault<H>`

- <span id="buildhasherdefault-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<H> Default for BuildHasherDefault<H>`

- <span id="buildhasherdefault-default"></span>`fn default() -> Self`

##### `impl<H> Eq for BuildHasherDefault<H>`

##### `impl<H> PartialEq for BuildHasherDefault<H>`

- <span id="buildhasherdefault-partialeq-eq"></span>`fn eq(&self, _other: &BuildHasherDefault<H>) -> bool` — [`BuildHasherDefault`](#buildhasherdefault)

## Traits

### `Hasher`

```rust
trait Hasher: core::hash::Hasher { ... }
```

An extension of [core::hash::Hasher][0] for hashers which use 32 bits.

For hashers which implement this trait, the standard `finish` method should just return a
zero-extended version of the result of `finish32`.

# Contract

Implementers of this trait must *not* perform any 64-bit (or 128-bit) operation while computing
the hash.

#### Required Methods

- `fn finish32(&self) -> u32`

  The equivalent of [`core::hash::Hasher.finish`][0] for 32-bit hashers.

#### Implementors

- [`Hasher`](fnv/index.md#hasher)
- [`Hasher`](murmur3/index.md#hasher)

