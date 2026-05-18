**num_integer > roots**

# Module: roots

## Contents

**Functions**

- [`cbrt`](#cbrt) - Returns the truncated principal cube root of an integer --
- [`nth_root`](#nth_root) - Returns the truncated principal `n`th root of an integer --
- [`sqrt`](#sqrt) - Returns the truncated principal square root of an integer --

**Traits**

- [`Roots`](#roots) - Provides methods to compute an integer's square root, cube root,

---

## num_integer::roots::Roots

*Trait*

Provides methods to compute an integer's square root, cube root,
and arbitrary `n`th root.

**Methods:**

- `nth_root`: Returns the truncated principal `n`th root of an integer
- `sqrt`: Returns the truncated principal square root of an integer -- `⌊√x⌋`
- `cbrt`: Returns the truncated principal cube root of an integer --



## num_integer::roots::cbrt

*Function*

Returns the truncated principal cube root of an integer --
see [Roots::cbrt](trait.Roots.html#method.cbrt).

```rust
fn cbrt<T>(x: T) -> T
```



## num_integer::roots::nth_root

*Function*

Returns the truncated principal `n`th root of an integer --
see [Roots::nth_root](trait.Roots.html#tymethod.nth_root).

```rust
fn nth_root<T>(x: T, n: u32) -> T
```



## num_integer::roots::sqrt

*Function*

Returns the truncated principal square root of an integer --
see [Roots::sqrt](trait.Roots.html#method.sqrt).

```rust
fn sqrt<T>(x: T) -> T
```



