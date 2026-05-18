**clap_builder > parser > matches > value_source**

# Module: parser::matches::value_source

## Contents

**Enums**

- [`ValueSource`](#valuesource) - Origin of the argument's value

---

## clap_builder::parser::matches::value_source::ValueSource

*Enum*

Origin of the argument's value

**Variants:**
- `DefaultValue` - Value came [`Arg::default_value`][crate::Arg::default_value]
- `EnvVariable` - Value came [`Arg::env`][crate::Arg::env]
- `CommandLine` - Value was passed in on the command-line

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ValueSource`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Ord**
  - `fn cmp(self: &Self, other: &ValueSource) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &ValueSource) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &ValueSource) -> $crate::option::Option<$crate::cmp::Ordering>`



