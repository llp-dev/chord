*[libm](../../../index.md) / [math](../../index.md) / [support](../index.md) / [env](index.md)*

---

# Module `env`

Support for rounding directions and status flags as specified by IEEE 754.

Rust does not support the floating point environment so rounding mode is passed as an argument
and status flags are returned as part of the result. There is currently not much support for
this; most existing ports from musl use a form of `force_eval!` to raise exceptions, but this
has no side effects in Rust. Further, correct behavior relies on elementary operations making
use of the correct rounding and raising relevant exceptions, which is not the case for Rust.

This module exists so no functionality is lost when porting algorithms that respect floating
point environment, and so that some functionality may be tested (that which does not rely on
side effects from elementary operations). Full support would require wrappers around basic
operations, but there is no plan to add this at the current time.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FpResult`](#fpresult) | struct | A value combined with a floating point status. |
| [`Status`](#status) | struct | IEEE 754 exception status flags. |
| [`Round`](#round) | enum | IEEE 754 rounding mode, excluding the optional `roundTiesToAway` version of nearest. |

## Structs

### `FpResult<T>`

```rust
struct FpResult<T> {
    pub val: T,
    pub status: Status,
}
```

A value combined with a floating point status.

#### Implementations

- <span id="fpresult-new"></span>`fn new(val: T, status: Status) -> Self` — [`Status`](#status)

- <span id="fpresult-ok"></span>`fn ok(val: T) -> Self`

  Return `val` with `Status::OK`.

### `Status`

```rust
struct Status(u8);
```

IEEE 754 exception status flags.

#### Implementations

- <span id="status-const-ok"></span>`const OK: Self`

- <span id="status-const-invalid"></span>`const INVALID: Self`

- <span id="status-const-divide-by-zero"></span>`const DIVIDE_BY_ZERO: Self`

- <span id="status-const-overflow"></span>`const OVERFLOW: Self`

- <span id="status-const-underflow"></span>`const UNDERFLOW: Self`

- <span id="status-const-inexact"></span>`const INEXACT: Self`

- <span id="status-underflow"></span>`const fn underflow(self) -> bool`

  True if `UNDERFLOW` is set.

- <span id="status-overflow"></span>`const fn overflow(self) -> bool`

  True if `OVERFLOW` is set.

- <span id="status-set-underflow"></span>`fn set_underflow(&mut self, val: bool)`

- <span id="status-inexact"></span>`const fn inexact(self) -> bool`

  True if `INEXACT` is set.

- <span id="status-set-inexact"></span>`fn set_inexact(&mut self, val: bool)`

- <span id="status-set-flag"></span>`fn set_flag(&mut self, val: bool, mask: Self)`

- <span id="status-with"></span>`const fn with(self, rhs: Self) -> Self`

#### Trait Implementations

##### `impl Clone for Status`

- <span id="status-clone"></span>`fn clone(&self) -> Status` — [`Status`](#status)

##### `impl Copy for Status`

##### `impl Debug for Status`

- <span id="status-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Status`

##### `impl PartialEq for Status`

- <span id="status-partialeq-eq"></span>`fn eq(&self, other: &Status) -> bool` — [`Status`](#status)

##### `impl StructuralPartialEq for Status`

## Enums

### `Round`

```rust
enum Round {
    Nearest,
    Negative,
    Positive,
    Zero,
}
```

IEEE 754 rounding mode, excluding the optional `roundTiesToAway` version of nearest.

Integer representation comes from what CORE-MATH uses for indexing.

#### Variants

- **`Nearest`**

  IEEE 754 nearest, `roundTiesToEven`.

- **`Negative`**

  IEEE 754 `roundTowardNegative`.

- **`Positive`**

  IEEE 754 `roundTowardPositive`.

- **`Zero`**

  IEEE 754 `roundTowardZero`.

#### Trait Implementations

##### `impl Clone for Round`

- <span id="round-clone"></span>`fn clone(&self) -> Round` — [`Round`](#round)

##### `impl Copy for Round`

##### `impl Debug for Round`

- <span id="round-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl PartialEq for Round`

- <span id="round-partialeq-eq"></span>`fn eq(&self, other: &Round) -> bool` — [`Round`](#round)

##### `impl StructuralPartialEq for Round`

