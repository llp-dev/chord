**sel4 > arch > imp > arch > imp > user_context**

# Module: arch::imp::arch::imp::user_context

## Contents

**Structs**

- [`UserContext`](#usercontext)

---

## sel4::arch::imp::arch::imp::user_context::UserContext

*Struct*

**Tuple Struct**: `()`

**Methods:**

- `fn from_inner(inner: sys::seL4_UserContext) -> Self`
- `fn into_inner(self: Self) -> sys::seL4_UserContext`
- `fn inner(self: &Self) -> &sys::seL4_UserContext`
- `fn inner_mut(self: & mut Self) -> & mut sys::seL4_UserContext`
- `fn pc(self: &Self) -> &Word`
- `fn pc_mut(self: & mut Self) -> & mut Word`
- `fn sp(self: &Self) -> &Word`
- `fn sp_mut(self: & mut Self) -> & mut Word`
- `fn c_param(self: &Self, ix: usize) -> &Word`
- `fn c_param_mut(self: & mut Self, ix: usize) -> & mut Word`

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> UserContext`
- **PartialEq**
  - `fn eq(self: &Self, other: &UserContext) -> bool`
- **Default**
  - `fn default() -> UserContext`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



