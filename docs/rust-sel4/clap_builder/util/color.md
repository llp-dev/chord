**clap_builder > util > color**

# Module: util::color

## Contents

**Enums**

- [`ColorChoice`](#colorchoice) - Represents the color preferences for program output

---

## clap_builder::util::color::ColorChoice

*Enum*

Represents the color preferences for program output

**Variants:**
- `Auto` - Enables colored output only when the output is going to a terminal or TTY.
- `Always` - Enables colored output regardless of whether or not the output is going to a terminal/TTY.
- `Never` - Disables colored output no matter if the output is going to a terminal/TTY, or not.

**Methods:**

- `fn possible_values() -> impl Trait` - Report all `possible_values`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut std::fmt::Formatter) -> std::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ColorChoice`
- **FromStr**
  - `fn from_str(s: &str) -> Result<Self, <Self as >::Err>`
- **ValueEnum**
  - `fn value_variants<'a>() -> &'a [Self]`
  - `fn to_possible_value(self: &Self) -> Option<PossibleValue>`
- **Default**
  - `fn default() -> ColorChoice`
- **PartialEq**
  - `fn eq(self: &Self, other: &ColorChoice) -> bool`



