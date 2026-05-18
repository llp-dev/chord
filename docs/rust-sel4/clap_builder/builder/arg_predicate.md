**clap_builder > builder > arg_predicate**

# Module: builder::arg_predicate

## Contents

**Enums**

- [`ArgPredicate`](#argpredicate) - Operations to perform on argument values

---

## clap_builder::builder::arg_predicate::ArgPredicate

*Enum*

Operations to perform on argument values

These do not apply to [`ValueSource::DefaultValue`][crate::parser::ValueSource::DefaultValue]

**Variants:**
- `IsPresent` - Is the argument present?
- `Equals(crate::builder::OsStr)` - Does the argument match the specified value?

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &ArgPredicate) -> bool`
- **From**
  - `fn from(other: S) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> ArgPredicate`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



