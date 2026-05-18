**num_traits > ops > euclid**

# Module: ops::euclid

## Contents

**Traits**

- [`CheckedEuclid`](#checkedeuclid)
- [`Euclid`](#euclid)

---

## num_traits::ops::euclid::CheckedEuclid

*Trait*

**Methods:**

- `checked_div_euclid`: Performs euclid division that returns `None` instead of panicking on division by zero
- `checked_rem_euclid`: Finds the euclid remainder of dividing two numbers, checking for underflow, overflow and
- `checked_div_rem_euclid`: Returns both the quotient and remainder from checked Euclidean division.



## num_traits::ops::euclid::Euclid

*Trait*

**Methods:**

- `div_euclid`: Calculates Euclidean division, the matching method for `rem_euclid`.
- `rem_euclid`: Calculates the least nonnegative remainder of `self (mod v)`.
- `div_rem_euclid`: Returns both the quotient and remainder from Euclidean division.



