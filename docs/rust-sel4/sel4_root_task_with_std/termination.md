**sel4_root_task_with_std > termination**

# Module: termination

## Contents

**Enums**

- [`Never`](#never) - Stable alternative to `!`.

**Traits**

- [`Termination`](#termination) - Trait for the return type of [`#[root_task]`](crate::root_task) main functions.

---

## sel4_root_task_with_std::termination::Never

*Enum*

Stable alternative to `!`.

This type in uninhabited like `!`, but does not require the unstable `#[feature(never_type)]`.
It implements [`Termination`], so it is useful in return types for
[`#[root_task]`](crate::root_task) main functions.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &Never) -> $crate::cmp::Ordering`
- **Termination**
  - `fn report(self: Self) -> <Self as >::Error`
- **Clone**
  - `fn clone(self: &Self) -> Never`
- **PartialEq**
  - `fn eq(self: &Self, other: &Never) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Never) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, _f: & mut fmt::Formatter) -> fmt::Result`



## sel4_root_task_with_std::termination::Termination

*Trait*

Trait for the return type of [`#[root_task]`](crate::root_task) main functions.

**Methods:**

- `Error`
- `report`



