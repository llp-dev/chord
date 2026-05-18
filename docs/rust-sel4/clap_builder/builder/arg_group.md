**clap_builder > builder > arg_group**

# Module: builder::arg_group

## Contents

**Structs**

- [`ArgGroup`](#arggroup) - Specifies a logical group of [arguments]

---

## clap_builder::builder::arg_group::ArgGroup

*Struct*

Specifies a logical group of [arguments]

You can use this for
- applying validation to an entire group, like [`ArgGroup::multiple`]
- validate relationships between an argument and a group, like [conflicts] or [requirements]
- check which argument in a group was specified on the command-line

For visually grouping arguments in help, see instead
[`Arg::help_heading`][crate::Arg::help_heading].

# Examples

The following example demonstrates using an `ArgGroup` to ensure that one, and only one, of
the arguments from the specified group is present at runtime.

```rust
# use clap_builder as clap;
# use clap::{Command, arg, ArgGroup, error::ErrorKind};
let result = Command::new("cmd")
    .arg(arg!(--"set-ver" <ver> "set the version manually"))
    .arg(arg!(--major           "auto increase major"))
    .arg(arg!(--minor           "auto increase minor"))
    .arg(arg!(--patch           "auto increase patch"))
    .group(ArgGroup::new("vers")
         .args(["set-ver", "major", "minor", "patch"])
         .required(true))
    .try_get_matches_from(vec!["cmd", "--major", "--patch"]);
// Because we used two args in the group it's an error
assert!(result.is_err());
let err = result.unwrap_err();
assert_eq!(err.kind(), ErrorKind::ArgumentConflict);
```

This next example shows a passing parse of the same scenario
```rust
# use clap_builder as clap;
# use clap::{Command, arg, ArgGroup, Id};
let result = Command::new("cmd")
    .arg(arg!(--"set-ver" <ver> "set the version manually"))
    .arg(arg!(--major           "auto increase major"))
    .arg(arg!(--minor           "auto increase minor"))
    .arg(arg!(--patch           "auto increase patch"))
    .group(ArgGroup::new("vers")
         .args(["set-ver", "major", "minor","patch"])
         .required(true))
    .try_get_matches_from(vec!["cmd", "--major"]);
assert!(result.is_ok());
let matches = result.unwrap();
// We may not know which of the args was used, so we can test for the group...
assert!(matches.contains_id("vers"));
// We can also ask the group which arg was used
assert_eq!(matches
    .get_one::<Id>("vers")
    .expect("`vers` is required")
    .as_str(),
    "major"
);
// we could also alternatively check each arg individually (not shown here)
```
[arguments]: crate::Arg
[conflicts]: crate::Arg::conflicts_with()
[requirements]: crate::Arg::requires()

**Methods:**

- `fn new<impl Into<Id>>(id: impl Trait) -> Self` - Create a `ArgGroup` using a unique name.
- `fn id<impl Into<Id>>(self: Self, id: impl Trait) -> Self` - Sets the group name.
- `fn arg<impl IntoResettable<Id>>(self: Self, arg_id: impl Trait) -> Self` - Adds an [argument] to this group by name
- `fn args<impl Into<Id>, impl IntoIterator<Item = impl Into<Id>>>(self: Self, ns: impl Trait) -> Self` - Adds multiple [arguments] to this group by name
- `fn get_args(self: &Self) -> impl Trait` - Getters for all args. It will return a vector of `Id`
- `fn multiple(self: Self, yes: bool) -> Self` - Allows more than one of the [`Arg`]s in this group to be used. (Default: `false`)
- `fn is_multiple(self: & mut Self) -> bool` - Return true if the group allows more than one of the arguments
- `fn required(self: Self, yes: bool) -> Self` - Require an argument from the group to be present when parsing.
- `fn requires<impl IntoResettable<Id>>(self: Self, id: impl Trait) -> Self` - Specify an argument or group that must be present when this group is.
- `fn requires_all<impl Into<Id>, impl IntoIterator<Item = impl Into<Id>>>(self: Self, ns: impl Trait) -> Self` - Specify arguments or groups that must be present when this group is.
- `fn conflicts_with<impl IntoResettable<Id>>(self: Self, id: impl Trait) -> Self` - Specify an argument or group that must **not** be present when this group is.
- `fn conflicts_with_all<impl Into<Id>, impl IntoIterator<Item = impl Into<Id>>>(self: Self, ns: impl Trait) -> Self` - Specify arguments or groups that must **not** be present when this group is.
- `fn get_id(self: &Self) -> &Id` - Get the name of the group
- `fn is_required_set(self: &Self) -> bool` - Reports whether [`ArgGroup::required`] is set

**Traits:** Eq

**Trait Implementations:**

- **Default**
  - `fn default() -> ArgGroup`
- **From**
  - `fn from(g: &ArgGroup) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ArgGroup`
- **PartialEq**
  - `fn eq(self: &Self, other: &ArgGroup) -> bool`



