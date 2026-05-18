**ring > rand**

# Module: rand

## Contents

**Structs**

- [`Random`](#random) - A random value constructed from a `SecureRandom` that hasn't been exposed
- [`SystemRandom`](#systemrandom) - A secure random number generator where the random values come directly

**Functions**

- [`generate`](#generate) - Generate the new random value using `rng`.

**Traits**

- [`RandomlyConstructable`](#randomlyconstructable) - A type that can be returned by `ring::rand::generate()`.
- [`SecureRandom`](#securerandom) - A secure random number generator.

---

## ring::rand::Random

*Struct*

A random value constructed from a `SecureRandom` that hasn't been exposed
through any safe Rust interface.

Intentionally does not implement any traits other than `Sized`.

**Generic Parameters:**
- T

**Tuple Struct**: `()`

**Methods:**

- `fn expose(self: Self) -> T` - Expose the random value.



## ring::rand::RandomlyConstructable

*Trait*

A type that can be returned by `ring::rand::generate()`.



## ring::rand::SecureRandom

*Trait*

A secure random number generator.

**Methods:**

- `fill`: Fills `dest` with random bytes.



## ring::rand::SystemRandom

*Struct*

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

**Tuple Struct**: `()`

**Methods:**

- `fn new() -> Self` - Constructs a new `SystemRandom`.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> SystemRandom`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ring::rand::generate

*Function*

Generate the new random value using `rng`.

```rust
fn generate<T>(rng: &dyn SecureRandom) -> Result<Random<T>, error::Unspecified>
```



