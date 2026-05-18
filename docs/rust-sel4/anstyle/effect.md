**anstyle > effect**

# Module: effect

## Contents

**Structs**

- [`EffectIter`](#effectiter) - Enumerate each enabled value in [`Effects`]
- [`Effects`](#effects) - A set of text effects

---

## anstyle::effect::EffectIter

*Struct*

Enumerate each enabled value in [`Effects`]

**Traits:** Eq

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
- **PartialEq**
  - `fn eq(self: &Self, other: &EffectIter) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> EffectIter`



## anstyle::effect::Effects

*Struct*

A set of text effects

# Examples

```rust
let effects = anstyle::Effects::BOLD | anstyle::Effects::UNDERLINE;
```

**Tuple Struct**: `()`

**Methods:**

- `fn new() -> Self` - No effects enabled
- `fn is_plain(self: Self) -> bool` - Check if no effects are enabled
- `fn contains(self: Self, other: Effects) -> bool` - Returns `true` if all of the effects in `other` are contained within `self`.
- `fn insert(self: Self, other: Effects) -> Self` - Inserts the specified effects in-place.
- `fn remove(self: Self, other: Effects) -> Self` - Removes the specified effects in-place.
- `fn clear(self: Self) -> Self` - Reset all effects in-place
- `fn set(self: Self, other: Self, enable: bool) -> Self` - Enable or disable the specified effects depending on the passed value.
- `fn iter(self: Self) -> EffectIter` - Iterate over enabled effects
- `fn render(self: Self) -> impl Trait` - Render the ANSI code

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Effects) -> $crate::option::Option<$crate::cmp::Ordering>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Effects) -> bool`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: Self)`
- **Default**
  - `fn default() -> Effects`
- **Clone**
  - `fn clone(self: &Self) -> Effects`
- **Sub**
  - `fn sub(self: Self, other: Self) -> Self`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: Self)`
- **BitOr**
  - `fn bitor(self: Self, rhs: Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Ord**
  - `fn cmp(self: &Self, other: &Effects) -> $crate::cmp::Ordering`



