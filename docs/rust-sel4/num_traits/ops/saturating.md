**num_traits > ops > saturating**

# Module: ops::saturating

## Contents

**Traits**

- [`Saturating`](#saturating) - Saturating math operations. Deprecated, use `SaturatingAdd`, `SaturatingSub` and
- [`SaturatingAdd`](#saturatingadd) - Performs addition that saturates at the numeric bounds instead of overflowing.
- [`SaturatingMul`](#saturatingmul) - Performs multiplication that saturates at the numeric bounds instead of overflowing.
- [`SaturatingSub`](#saturatingsub) - Performs subtraction that saturates at the numeric bounds instead of overflowing.

---

## num_traits::ops::saturating::Saturating

*Trait*

Saturating math operations. Deprecated, use `SaturatingAdd`, `SaturatingSub` and
`SaturatingMul` instead.

**Methods:**

- `saturating_add`: Saturating addition operator.
- `saturating_sub`: Saturating subtraction operator.



## num_traits::ops::saturating::SaturatingAdd

*Trait*

Performs addition that saturates at the numeric bounds instead of overflowing.

**Methods:**

- `saturating_add`: Saturating addition. Computes `self + other`, saturating at the relevant high or low boundary of



## num_traits::ops::saturating::SaturatingMul

*Trait*

Performs multiplication that saturates at the numeric bounds instead of overflowing.

**Methods:**

- `saturating_mul`: Saturating multiplication. Computes `self * other`, saturating at the relevant high or low boundary of



## num_traits::ops::saturating::SaturatingSub

*Trait*

Performs subtraction that saturates at the numeric bounds instead of overflowing.

**Methods:**

- `saturating_sub`: Saturating subtraction. Computes `self - other`, saturating at the relevant high or low boundary of



