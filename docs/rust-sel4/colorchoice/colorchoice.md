**colorchoice**

# Module: colorchoice

## Contents

**Enums**

- [`ColorChoice`](#colorchoice) - Selection for overriding color output

---

## colorchoice::ColorChoice

*Enum*

Selection for overriding color output

**Variants:**
- `Auto` - Use colors if the output device appears to support them
- `AlwaysAnsi` - Like `Always`, except it never tries to use anything other than emitting ANSI
- `Always` - Try very hard to emit colors.
- `Never` - Never emit colors.

**Methods:**

- `fn global() -> Self` - Get the current [`ColorChoice`] state
- `fn write_global(self: Self)` - Override the detected [`ColorChoice`]

**Traits:** Copy, Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ColorChoice`
- **PartialEq**
  - `fn eq(self: &Self, other: &ColorChoice) -> bool`
- **Default**
  - `fn default() -> ColorChoice`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



