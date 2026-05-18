**clap_builder > util > id**

# Module: util::id

## Contents

**Structs**

- [`Id`](#id) - [`Arg`][crate::Arg] or [`ArgGroup`][crate::ArgGroup] identifier

---

## clap_builder::util::id::Id

*Struct*

[`Arg`][crate::Arg] or [`ArgGroup`][crate::ArgGroup] identifier

This is used for accessing the value in [`ArgMatches`][crate::ArgMatches] or defining
relationships between `Arg`s and `ArgGroup`s with functions like
[`Arg::conflicts_with`][crate::Arg::conflicts_with].

**Tuple Struct**: `()`

**Methods:**

- `fn as_str(self: &Self) -> &str` - Get the raw string of the `Id`

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut std::fmt::Formatter) -> std::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Str) -> bool`
- **From**
  - `fn from(name: &'static str) -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Id) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Default**
  - `fn default() -> Id`
- **PartialEq**
  - `fn eq(self: &Self, other: &String) -> bool`
- **PartialEq**
  - `fn eq(self: &Self, other: &str) -> bool`
- **Display**
  - `fn fmt(self: &Self, f: & mut std::fmt::Formatter) -> std::fmt::Result`
- **From**
  - `fn from(id: &Id) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Id) -> bool`
- **Borrow**
  - `fn borrow(self: &Self) -> &str`
- **From**
  - `fn from(name: &Str) -> Self`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &&str) -> bool`
- **AsRef**
  - `fn as_ref(self: &Self) -> &str`
- **From**
  - `fn from(name: &&'static str) -> Self`
- **From**
  - `fn from(name: Str) -> Self`
- **Ord**
  - `fn cmp(self: &Self, other: &Id) -> $crate::cmp::Ordering`
- **Clone**
  - `fn clone(self: &Self) -> Id`



