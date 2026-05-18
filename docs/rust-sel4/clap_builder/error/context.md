**clap_builder > error > context**

# Module: error::context

## Contents

**Enums**

- [`ContextKind`](#contextkind) - Semantics for a piece of error information
- [`ContextValue`](#contextvalue) - A piece of error information

---

## clap_builder::error::context::ContextKind

*Enum*

Semantics for a piece of error information

**Variants:**
- `InvalidSubcommand` - The cause of the error
- `InvalidArg` - The cause of the error
- `PriorArg` - Existing arguments
- `ValidSubcommand` - Accepted subcommands
- `ValidValue` - Accepted values
- `InvalidValue` - Rejected values
- `ActualNumValues` - Number of values present
- `ExpectedNumValues` - Number of allowed values
- `MinValues` - Minimum number of allowed values
- `SuggestedCommand` - Potential fix for the user
- `SuggestedSubcommand` - Potential fix for the user
- `SuggestedArg` - Potential fix for the user
- `SuggestedValue` - Potential fix for the user
- `TrailingArg` - Trailing argument
- `Suggested` - Potential fix for the user
- `Usage` - A usage string
- `Custom` - An opaque message to the user

**Methods:**

- `fn as_str(self: Self) -> Option<&'static str>` - End-user description of the error case, where relevant

**Traits:** Eq, Copy

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut std::fmt::Formatter) -> std::fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &ContextKind) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> ContextKind`



## clap_builder::error::context::ContextValue

*Enum*

A piece of error information

**Variants:**
- `None` - [`ContextKind`] is self-sufficient, no additional information needed
- `Bool(bool)` - A single value
- `String(String)` - A single value
- `Strings(Vec<String>)` - Many values
- `StyledStr(crate::builder::StyledStr)` - A single value
- `StyledStrs(Vec<crate::builder::StyledStr>)` - many value
- `Number(isize)` - A single value

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &ContextValue) -> bool`
- **Display**
  - `fn fmt(self: &Self, f: & mut std::fmt::Formatter) -> std::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ContextValue`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



