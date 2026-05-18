**num_traits > pow**

# Module: pow

## Contents

**Functions**

- [`checked_pow`](#checked_pow) - Raises a value to the power of exp, returning `None` if an overflow occurred.
- [`pow`](#pow) - Raises a value to the power of exp, using exponentiation by squaring.

**Traits**

- [`Pow`](#pow) - Binary operator for raising a value to a power.

---

## num_traits::pow::Pow

*Trait*

Binary operator for raising a value to a power.

**Methods:**

- `Output`: The result after applying the operator.
- `pow`: Returns `self` to the power `rhs`.



## num_traits::pow::checked_pow

*Function*

Raises a value to the power of exp, returning `None` if an overflow occurred.

Note that `0⁰` (`checked_pow(0, 0)`) returns `Some(1)`. Mathematically this is undefined.

Otherwise same as the `pow` function.

# Example

```rust
use num_traits::checked_pow;

assert_eq!(checked_pow(2i8, 4), Some(16));
assert_eq!(checked_pow(7i8, 8), None);
assert_eq!(checked_pow(7u32, 8), Some(5_764_801));
assert_eq!(checked_pow(0u32, 0), Some(1)); // Be aware if this case affect you
```

```rust
fn checked_pow<T>(base: T, exp: usize) -> Option<T>
```



## num_traits::pow::pow

*Function*

Raises a value to the power of exp, using exponentiation by squaring.

Note that `0⁰` (`pow(0, 0)`) returns `1`. Mathematically this is undefined.

# Example

```rust
use num_traits::pow;

assert_eq!(pow(2i8, 4), 16);
assert_eq!(pow(6u8, 3), 216);
assert_eq!(pow(0u8, 0), 1); // Be aware if this case affects you
```

```rust
fn pow<T>(base: T, exp: usize) -> T
```



