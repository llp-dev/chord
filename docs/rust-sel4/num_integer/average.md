**num_integer > average**

# Module: average

## Contents

**Functions**

- [`average_ceil`](#average_ceil) - Returns the ceiling value of the average of `x` and `y` --
- [`average_floor`](#average_floor) - Returns the floor value of the average of `x` and `y` --

**Traits**

- [`Average`](#average) - Provides methods to compute the average of two integers, without overflows.

---

## num_integer::average::Average

*Trait*

Provides methods to compute the average of two integers, without overflows.

**Methods:**

- `average_ceil`: Returns the ceiling value of the average of `self` and `other`.
- `average_floor`: Returns the floor value of the average of `self` and `other`.



## num_integer::average::average_ceil

*Function*

Returns the ceiling value of the average of `x` and `y` --
see [Average::average_ceil](trait.Average.html#tymethod.average_ceil).

```rust
fn average_ceil<T>(x: T, y: T) -> T
```



## num_integer::average::average_floor

*Function*

Returns the floor value of the average of `x` and `y` --
see [Average::average_floor](trait.Average.html#tymethod.average_floor).

```rust
fn average_floor<T>(x: T, y: T) -> T
```



