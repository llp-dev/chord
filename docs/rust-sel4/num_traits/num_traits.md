**num_traits**

# Module: num_traits

## Contents

**Modules**

- [`bounds`](#bounds)
- [`cast`](#cast)
- [`float`](#float)
- [`identities`](#identities)
- [`int`](#int)
- [`ops`](#ops)
- [`pow`](#pow)
- [`real`](#real)
- [`sign`](#sign)

**Structs**

- [`ParseFloatError`](#parsefloaterror)

**Enums**

- [`FloatErrorKind`](#floaterrorkind)

**Functions**

- [`clamp`](#clamp) - A value bounded by a minimum and a maximum
- [`clamp_max`](#clamp_max) - A value bounded by a maximum value
- [`clamp_min`](#clamp_min) - A value bounded by a minimum value

**Traits**

- [`Num`](#num) - The base trait for numeric types, covering `0` and `1` values,
- [`NumAssign`](#numassign) - The trait for `Num` types which also implement assignment operators.
- [`NumAssignOps`](#numassignops) - Generic trait for types implementing numeric assignment operators (like `+=`).
- [`NumAssignRef`](#numassignref) - The trait for `NumAssign` types which also implement assignment operations
- [`NumOps`](#numops) - Generic trait for types implementing basic numeric operations
- [`NumRef`](#numref) - The trait for `Num` types which also implement numeric operations taking
- [`RefNum`](#refnum) - The trait for `Num` references which implement numeric operations, taking the

---

## num_traits::FloatErrorKind

*Enum*

**Variants:**
- `Empty`
- `Invalid`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## num_traits::Num

*Trait*

The base trait for numeric types, covering `0` and `1` values,
comparisons, basic numeric operations, and string conversion.

**Methods:**

- `FromStrRadixErr`
- `from_str_radix`: Convert from a string and radix (typically `2..=36`).



## num_traits::NumAssign

*Trait*

The trait for `Num` types which also implement assignment operators.

This is automatically implemented for types which implement the operators.



## num_traits::NumAssignOps

*Trait*

Generic trait for types implementing numeric assignment operators (like `+=`).

This is automatically implemented for types which implement the operators.



## num_traits::NumAssignRef

*Trait*

The trait for `NumAssign` types which also implement assignment operations
taking the second operand by reference.

This is automatically implemented for types which implement the operators.



## num_traits::NumOps

*Trait*

Generic trait for types implementing basic numeric operations

This is automatically implemented for types which implement the operators.



## num_traits::NumRef

*Trait*

The trait for `Num` types which also implement numeric operations taking
the second operand by reference.

This is automatically implemented for types which implement the operators.



## num_traits::ParseFloatError

*Struct*

**Fields:**
- `kind: FloatErrorKind`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## num_traits::RefNum

*Trait*

The trait for `Num` references which implement numeric operations, taking the
second operand either by value or by reference.

This is automatically implemented for all types which implement the operators. It covers
every type implementing the operations though, regardless of it being a reference or
related to `Num`.



## Module: bounds



## Module: cast



## num_traits::clamp

*Function*

A value bounded by a minimum and a maximum

 If input is less than min then this returns min.
 If input is greater than max then this returns max.
 Otherwise this returns input.

**Panics** in debug mode if `!(min <= max)`.

```rust
fn clamp<T>(input: T, min: T, max: T) -> T
```



## num_traits::clamp_max

*Function*

A value bounded by a maximum value

 If input is greater than max then this returns max.
 Otherwise this returns input.
 `clamp_max(std::f32::NAN, 1.0)` preserves `NAN` different from `f32::max(std::f32::NAN, 1.0)`.

**Panics** in debug mode if `!(max == max)`. (This occurs if `max` is `NAN`.)

```rust
fn clamp_max<T>(input: T, max: T) -> T
```



## num_traits::clamp_min

*Function*

A value bounded by a minimum value

 If input is less than min then this returns min.
 Otherwise this returns input.
 `clamp_min(std::f32::NAN, 1.0)` preserves `NAN` different from `f32::min(std::f32::NAN, 1.0)`.

**Panics** in debug mode if `!(min == min)`. (This occurs if `min` is `NAN`.)

```rust
fn clamp_min<T>(input: T, min: T) -> T
```



## Module: float



## Module: identities



## Module: int



## Module: ops



## Module: pow



## Module: real



## Module: sign



