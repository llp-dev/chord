**clap_builder > parser > error**

# Module: parser::error

## Contents

**Enums**

- [`MatchesError`](#matcheserror) - Violation of [`ArgMatches`][crate::ArgMatches] assumptions

---

## clap_builder::parser::error::MatchesError

*Enum*

Violation of [`ArgMatches`][crate::ArgMatches] assumptions

**Variants:**
- `Downcast{ actual: self::any_value::AnyValueId, expected: self::any_value::AnyValueId }` - Failed to downcast `AnyValue` to the specified type
- `UnknownArgument{  }` - Argument not defined in [`Command`][crate::Command]

**Traits:** Error

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut std::fmt::Formatter) -> std::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> MatchesError`



