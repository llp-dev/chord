**num_traits > ops > mul_add**

# Module: ops::mul_add

## Contents

**Traits**

- [`MulAdd`](#muladd) - Fused multiply-add. Computes `(self * a) + b` with only one rounding
- [`MulAddAssign`](#muladdassign) - The fused multiply-add assignment operation `*self = (*self * a) + b`

---

## num_traits::ops::mul_add::MulAdd

*Trait*

Fused multiply-add. Computes `(self * a) + b` with only one rounding
error, yielding a more accurate result than an unfused multiply-add.

Using `mul_add` can be more performant than an unfused multiply-add if
the target architecture has a dedicated `fma` CPU instruction.

Note that `A` and `B` are `Self` by default, but this is not mandatory.

# Example

```
use std::f32;

let m = 10.0_f32;
let x = 4.0_f32;
let b = 60.0_f32;

// 100.0
let abs_difference = (m.mul_add(x, b) - (m*x + b)).abs();

assert!(abs_difference <= 100.0 * f32::EPSILON);
```

**Methods:**

- `Output`: The resulting type after applying the fused multiply-add.
- `mul_add`: Performs the fused multiply-add operation `(self * a) + b`



## num_traits::ops::mul_add::MulAddAssign

*Trait*

The fused multiply-add assignment operation `*self = (*self * a) + b`

**Methods:**

- `mul_add_assign`: Performs the fused multiply-add assignment operation `*self = (*self * a) + b`



