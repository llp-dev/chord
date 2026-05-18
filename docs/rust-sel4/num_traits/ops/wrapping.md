**num_traits > ops > wrapping**

# Module: ops::wrapping

## Contents

**Traits**

- [`WrappingAdd`](#wrappingadd) - Performs addition that wraps around on overflow.
- [`WrappingMul`](#wrappingmul) - Performs multiplication that wraps around on overflow.
- [`WrappingNeg`](#wrappingneg) - Performs a negation that does not panic.
- [`WrappingShl`](#wrappingshl) - Performs a left shift that does not panic.
- [`WrappingShr`](#wrappingshr) - Performs a right shift that does not panic.
- [`WrappingSub`](#wrappingsub) - Performs subtraction that wraps around on overflow.

---

## num_traits::ops::wrapping::WrappingAdd

*Trait*

Performs addition that wraps around on overflow.

**Methods:**

- `wrapping_add`: Wrapping (modular) addition. Computes `self + other`, wrapping around at the boundary of



## num_traits::ops::wrapping::WrappingMul

*Trait*

Performs multiplication that wraps around on overflow.

**Methods:**

- `wrapping_mul`: Wrapping (modular) multiplication. Computes `self * other`, wrapping around at the boundary



## num_traits::ops::wrapping::WrappingNeg

*Trait*

Performs a negation that does not panic.

**Methods:**

- `wrapping_neg`: Wrapping (modular) negation. Computes `-self`,



## num_traits::ops::wrapping::WrappingShl

*Trait*

Performs a left shift that does not panic.

**Methods:**

- `wrapping_shl`: Panic-free bitwise shift-left; yields `self << mask(rhs)`,



## num_traits::ops::wrapping::WrappingShr

*Trait*

Performs a right shift that does not panic.

**Methods:**

- `wrapping_shr`: Panic-free bitwise shift-right; yields `self >> mask(rhs)`,



## num_traits::ops::wrapping::WrappingSub

*Trait*

Performs subtraction that wraps around on overflow.

**Methods:**

- `wrapping_sub`: Wrapping (modular) subtraction. Computes `self - other`, wrapping around at the boundary



