**clap_builder > builder > possible_value**

# Module: builder::possible_value

## Contents

**Structs**

- [`PossibleValue`](#possiblevalue) - A possible value of an argument.

---

## clap_builder::builder::possible_value::PossibleValue

*Struct*

A possible value of an argument.

This is used for specifying [possible values] of [Args].

See also [`PossibleValuesParser`][crate::builder::PossibleValuesParser]

<div class="warning">

**NOTE:** Most likely you can use strings, rather than `PossibleValue` as it is only required
to [hide] single values from help messages and shell completions or to attach [help] to
possible values.

</div>

# Examples

```rust
# use clap_builder as clap;
# use clap::{Arg, builder::PossibleValue, ArgAction};
let cfg = Arg::new("config")
    .action(ArgAction::Set)
    .value_name("FILE")
    .value_parser([
        PossibleValue::new("fast"),
        PossibleValue::new("slow").help("slower than fast"),
        PossibleValue::new("secret speed").hide(true)
    ]);
```

[Args]: crate::Arg
[possible values]: crate::builder::ValueParser::possible_values
[hide]: PossibleValue::hide()
[help]: PossibleValue::help()

**Methods:**

- `fn new<impl Into<Str>>(name: impl Trait) -> Self` - Create a [`PossibleValue`] with its name.
- `fn help<impl IntoResettable<StyledStr>>(self: Self, help: impl Trait) -> Self` - Sets the help description of the value.
- `fn hide(self: Self, yes: bool) -> Self` - Hides this value from help and shell completions.
- `fn alias<impl IntoResettable<Str>>(self: Self, name: impl Trait) -> Self` - Sets a *hidden* alias for this argument value.
- `fn aliases<impl Into<Str>, impl IntoIterator<Item = impl Into<Str>>>(self: Self, names: impl Trait) -> Self` - Sets multiple *hidden* aliases for this argument value.
- `fn get_name(self: &Self) -> &str` - Get the name of the argument value
- `fn get_help(self: &Self) -> Option<&StyledStr>` - Get the help specified for this argument, if any
- `fn is_hide_set(self: &Self) -> bool` - Report if [`PossibleValue::hide`] is set
- `fn get_name_and_aliases(self: &Self) -> impl Trait` - Returns all valid values of the argument value.
- `fn matches(self: &Self, value: &str, ignore_case: bool) -> bool` - Tests if the value is valid for this argument value

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> PossibleValue`
- **PartialEq**
  - `fn eq(self: &Self, other: &PossibleValue) -> bool`
- **From**
  - `fn from(s: S) -> Self`
- **Default**
  - `fn default() -> PossibleValue`



