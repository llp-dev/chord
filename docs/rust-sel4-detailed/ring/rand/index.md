*[ring](../index.md) / [rand](index.md)*

---

# Module `rand`

Cryptographic pseudo-random number generation.

*ring* functions that generate random bytes take a `&dyn SecureRandom`
parameter to make it clear which functions are non-deterministic.

## Contents

- [Modules](#modules)
  - [`sealed`](#sealed)
- [Structs](#structs)
  - [`Random`](#random)
  - [`SystemRandom`](#systemrandom)
- [Traits](#traits)
  - [`SecureRandom`](#securerandom)
  - [`RandomlyConstructable`](#randomlyconstructable)
- [Functions](#functions)
  - [`generate`](#generate)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`sealed`](#sealed) | mod |  |
| [`Random`](#random) | struct | A random value constructed from a `SecureRandom` that hasn't been exposed through any safe Rust interface. |
| [`SystemRandom`](#systemrandom) | struct | A secure random number generator where the random values come directly from the operating system. |
| [`SecureRandom`](#securerandom) | trait | A secure random number generator. |
| [`RandomlyConstructable`](#randomlyconstructable) | trait | A type that can be returned by `ring::rand::generate()`. |
| [`generate`](#generate) | fn | Generate the new random value using `rng`. |

## Modules

- [`sealed`](sealed/index.md)

## Structs

### `Random<T: RandomlyConstructable>`

```rust
struct Random<T: RandomlyConstructable>(T);
```

A random value constructed from a `SecureRandom` that hasn't been exposed
through any safe Rust interface.

Intentionally does not implement any traits other than `Sized`.

#### Implementations

- <span id="random-expose"></span>`fn expose(self) -> T`

  Expose the random value.

### `SystemRandom`

```rust
struct SystemRandom(());
```

A secure random number generator where the random values come directly
from the operating system.

"Directly from the operating system" here presently means "whatever the
`getrandom` crate does" but that may change in the future. That roughly
means calling libc's `getrandom` function or whatever is analogous to that;
see the `getrandom` crate's documentation for more info.

A single `SystemRandom` may be shared across multiple threads safely.

`new()` is guaranteed to always succeed and to have low latency; it won't
try to open or read from a file or do similar things. The first call to
`fill()` may block a substantial amount of time since any and all
initialization is deferred to it. Therefore, it may be a good idea to call
`fill()` once at a non-latency-sensitive time to minimize latency for
future calls.

#### Implementations

- <span id="systemrandom-new"></span>`fn new() -> Self`

  Constructs a new `SystemRandom`.

#### Trait Implementations

##### `impl Clone for SystemRandom`

- <span id="systemrandom-clone"></span>`fn clone(&self) -> SystemRandom` — [`SystemRandom`](#systemrandom)

##### `impl Debug for SystemRandom`

- <span id="systemrandom-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Sealed for SystemRandom`

##### `impl SecureRandom for SystemRandom`

- <span id="systemrandom-securerandom-fill"></span>`fn fill(&self, dest: &mut [u8]) -> Result<(), Unspecified>` — [`Unspecified`](../error/index.md#unspecified)

## Traits

### `SecureRandom`

```rust
trait SecureRandom: sealed::SecureRandom { ... }
```

A secure random number generator.

#### Required Methods

- `fn fill(&self, dest: &mut [u8]) -> Result<(), error::Unspecified>`

  Fills `dest` with random bytes.

#### Implementors

- `T`

### `RandomlyConstructable`

```rust
trait RandomlyConstructable: self::sealed::RandomlyConstructable { ... }
```

A type that can be returned by `ring::rand::generate()`.

#### Implementors

- `T`

## Functions

### `generate`

```rust
fn generate<T>(rng: &dyn SecureRandom) -> Result<Random<T>, error::Unspecified>
where
    T: RandomlyConstructable
```

Generate the new random value using `rng`.

