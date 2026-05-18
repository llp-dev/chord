**num_traits > ops > checked**

# Module: ops::checked

## Contents

**Traits**

- [`CheckedAdd`](#checkedadd) - Performs addition that returns `None` instead of wrapping around on
- [`CheckedDiv`](#checkeddiv) - Performs division that returns `None` instead of panicking on division by zero and instead of
- [`CheckedMul`](#checkedmul) - Performs multiplication that returns `None` instead of wrapping around on underflow or
- [`CheckedNeg`](#checkedneg) - Performs negation that returns `None` if the result can't be represented.
- [`CheckedRem`](#checkedrem) - Performs an integral remainder that returns `None` instead of panicking on division by zero and
- [`CheckedShl`](#checkedshl) - Performs a left shift that returns `None` on shifts larger than
- [`CheckedShr`](#checkedshr) - Performs a right shift that returns `None` on shifts larger than
- [`CheckedSub`](#checkedsub) - Performs subtraction that returns `None` instead of wrapping around on underflow.

---

## num_traits::ops::checked::CheckedAdd

*Trait*

Performs addition that returns `None` instead of wrapping around on
overflow.

**Methods:**

- `checked_add`: Adds two numbers, checking for overflow. If overflow happens, `None` is



## num_traits::ops::checked::CheckedDiv

*Trait*

Performs division that returns `None` instead of panicking on division by zero and instead of
wrapping around on underflow and overflow.

**Methods:**

- `checked_div`: Divides two numbers, checking for underflow, overflow and division by



## num_traits::ops::checked::CheckedMul

*Trait*

Performs multiplication that returns `None` instead of wrapping around on underflow or
overflow.

**Methods:**

- `checked_mul`: Multiplies two numbers, checking for underflow or overflow. If underflow



## num_traits::ops::checked::CheckedNeg

*Trait*

Performs negation that returns `None` if the result can't be represented.

**Methods:**

- `checked_neg`: Negates a number, returning `None` for results that can't be represented, like signed `MIN`



## num_traits::ops::checked::CheckedRem

*Trait*

Performs an integral remainder that returns `None` instead of panicking on division by zero and
instead of wrapping around on underflow and overflow.

**Methods:**

- `checked_rem`: Finds the remainder of dividing two numbers, checking for underflow, overflow and division



## num_traits::ops::checked::CheckedShl

*Trait*

Performs a left shift that returns `None` on shifts larger than
or equal to the type width.

**Methods:**

- `checked_shl`: Checked shift left. Computes `self << rhs`, returning `None`



## num_traits::ops::checked::CheckedShr

*Trait*

Performs a right shift that returns `None` on shifts larger than
or equal to the type width.

**Methods:**

- `checked_shr`: Checked shift right. Computes `self >> rhs`, returning `None`



## num_traits::ops::checked::CheckedSub

*Trait*

Performs subtraction that returns `None` instead of wrapping around on underflow.

**Methods:**

- `checked_sub`: Subtracts two numbers, checking for underflow. If underflow happens,



