**clap_builder > builder > arg**

# Module: builder::arg

## Contents

**Structs**

- [`Arg`](#arg) - The abstract representation of a command line argument. Used to set all the options and

---

## clap_builder::builder::arg::Arg

*Struct*

The abstract representation of a command line argument. Used to set all the options and
relationships that define a valid argument for the program.

There are two methods for constructing [`Arg`]s, using the builder pattern and setting options
manually, or using a usage string which is far less verbose but has fewer options. You can also
use a combination of the two methods to achieve the best of both worlds.

- [Basic API][crate::Arg#basic-api]
- [Value Handling][crate::Arg#value-handling]
- [Help][crate::Arg#help-1]
- [Advanced Argument Relations][crate::Arg#advanced-argument-relations]
- [Reflection][crate::Arg#reflection]

# Examples

```rust
# use clap_builder as clap;
# use clap::{Arg, arg, ArgAction};
// Using the traditional builder pattern and setting each option manually
let cfg = Arg::new("config")
      .short('c')
      .long("config")
      .action(ArgAction::Set)
      .value_name("FILE")
      .help("Provides a config file to myprog");
// Using a usage string (setting a similar argument to the one above)
let input = arg!(-i --input <FILE> "Provides an input file to the program");
```

**Methods:**

- `fn group<impl IntoResettable<Id>>(self: Self, group_id: impl Trait) -> Self` - The name of the [`ArgGroup`] the argument belongs to.
- `fn groups<impl Into<Id>, impl IntoIterator<Item = impl Into<Id>>>(self: Self, group_ids: impl Trait) -> Self` - The names of [`ArgGroup`]'s the argument belongs to.
- `fn default_value_if<impl Into<Id>, impl Into<ArgPredicate>, impl IntoResettable<OsStr>>(self: Self, arg_id: impl Trait, predicate: impl Trait, default: impl Trait) -> Self` - Specifies the value of the argument if `arg` has been used at runtime.
- `fn default_values_if<impl Into<Id>, impl Into<ArgPredicate>, impl Into<OsStr>, impl IntoIterator<Item = impl Into<OsStr>>>(self: Self, arg_id: impl Trait, predicate: impl Trait, defaults: impl Trait) -> Self` - Specifies the values of the argument if `arg` has been used at runtime.
- `fn default_value_ifs<impl Into<Id>, impl Into<ArgPredicate>, impl IntoResettable<OsStr>, impl IntoIterator<Item = (impl Into<Id>, impl Into<ArgPredicate>, impl IntoResettable<OsStr>)>>(self: Self, ifs: impl Trait) -> Self` - Specifies multiple values and conditions in the same manner as [`Arg::default_value_if`].
- `fn default_values_ifs<impl Into<Id>, impl Into<ArgPredicate>, impl Into<OsStr>, impl IntoIterator<Item = impl Into<OsStr>>, impl IntoIterator<Item = (impl Into<Id>, impl Into<ArgPredicate>, impl IntoIterator<Item = impl Into<OsStr>>)>>(self: Self, ifs: impl Trait) -> Self` - Specifies multiple values and conditions in the same manner as [`Arg::default_values_if`].
- `fn required_unless_present<impl IntoResettable<Id>>(self: Self, arg_id: impl Trait) -> Self` - Set this arg as [required] as long as the specified argument is not present at runtime.
- `fn required_unless_present_all<impl Into<Id>, impl IntoIterator<Item = impl Into<Id>>>(self: Self, names: impl Trait) -> Self` - Sets this arg as [required] unless *all* of the specified arguments are present at runtime.
- `fn required_unless_present_any<impl Into<Id>, impl IntoIterator<Item = impl Into<Id>>>(self: Self, names: impl Trait) -> Self` - Sets this arg as [required] unless *any* of the specified arguments are present at runtime.
- `fn required_if_eq<impl Into<Id>, impl Into<OsStr>>(self: Self, arg_id: impl Trait, val: impl Trait) -> Self` - This argument is [required] only if the specified `arg` is present at runtime and its value
- `fn required_if_eq_any<impl Into<Id>, impl Into<OsStr>, impl IntoIterator<Item = (impl Into<Id>, impl Into<OsStr>)>>(self: Self, ifs: impl Trait) -> Self` - Specify this argument is [required] based on multiple conditions.
- `fn required_if_eq_all<impl Into<Id>, impl Into<OsStr>, impl IntoIterator<Item = (impl Into<Id>, impl Into<OsStr>)>>(self: Self, ifs: impl Trait) -> Self` - Specify this argument is [required] based on multiple conditions.
- `fn requires_if<impl Into<ArgPredicate>, impl Into<Id>>(self: Self, val: impl Trait, arg_id: impl Trait) -> Self` - Require another argument if this arg matches the [`ArgPredicate`]
- `fn requires_ifs<impl Into<ArgPredicate>, impl Into<Id>, impl IntoIterator<Item = (impl Into<ArgPredicate>, impl Into<Id>)>>(self: Self, ifs: impl Trait) -> Self` - Allows multiple conditional requirements.
- `fn conflicts_with<impl IntoResettable<Id>>(self: Self, arg_id: impl Trait) -> Self` - This argument is mutually exclusive with the specified argument.
- `fn conflicts_with_all<impl Into<Id>, impl IntoIterator<Item = impl Into<Id>>>(self: Self, names: impl Trait) -> Self` - This argument is mutually exclusive with the specified arguments.
- `fn overrides_with<impl IntoResettable<Id>>(self: Self, arg_id: impl Trait) -> Self` - Sets an overridable argument.
- `fn overrides_with_all<impl Into<Id>, impl IntoIterator<Item = impl Into<Id>>>(self: Self, names: impl Trait) -> Self` - Sets multiple mutually overridable arguments by name.
- `fn help<impl IntoResettable<StyledStr>>(self: Self, h: impl Trait) -> Self` - Sets the description of the argument for short help (`-h`).
- `fn long_help<impl IntoResettable<StyledStr>>(self: Self, h: impl Trait) -> Self` - Sets the description of the argument for long help (`--help`).
- `fn display_order<impl IntoResettable<usize>>(self: Self, ord: impl Trait) -> Self` - Allows custom ordering of args within the help message.
- `fn help_heading<impl IntoResettable<Str>>(self: Self, heading: impl Trait) -> Self` - Override the `--help` section this appears in.
- `fn next_line_help(self: Self, yes: bool) -> Self` - Render the [help][Arg::help] on the line after the argument.
- `fn hide(self: Self, yes: bool) -> Self` - Do not display the argument in help message.
- `fn hide_possible_values(self: Self, yes: bool) -> Self` - Do not display the [possible values][crate::builder::ValueParser::possible_values] in the help message.
- `fn hide_default_value(self: Self, yes: bool) -> Self` - Do not display the default value of the argument in the help message.
- `fn hide_short_help(self: Self, yes: bool) -> Self` - Hides an argument from short help (`-h`).
- `fn hide_long_help(self: Self, yes: bool) -> Self` - Hides an argument from long help (`--help`).
- `fn action<impl IntoResettable<ArgAction>>(self: Self, action: impl Trait) -> Self` - Specify how to react to an argument when parsing it.
- `fn value_parser<impl IntoResettable<super::ValueParser>>(self: Self, parser: impl Trait) -> Self` - Specify the typed behavior of the argument.
- `fn num_args<impl IntoResettable<ValueRange>>(self: Self, qty: impl Trait) -> Self` - Specifies the number of arguments parsed per occurrence
- `fn value_name<impl IntoResettable<Str>>(self: Self, name: impl Trait) -> Self` - Placeholder for the argument's value in the help message / usage.
- `fn value_names<impl Into<Str>, impl IntoIterator<Item = impl Into<Str>>>(self: Self, names: impl Trait) -> Self` - Placeholders for the argument's values in the help message / usage.
- `fn value_hint<impl IntoResettable<ValueHint>>(self: Self, value_hint: impl Trait) -> Self` - Provide the shell a hint about how to complete this argument.
- `fn ignore_case(self: Self, yes: bool) -> Self` - Match values against [`PossibleValuesParser`][crate::builder::PossibleValuesParser] without matching case.
- `fn allow_hyphen_values(self: Self, yes: bool) -> Self` - Allows values which start with a leading hyphen (`-`)
- `fn allow_negative_numbers(self: Self, yes: bool) -> Self` - Allows negative numbers to pass as values.
- `fn require_equals(self: Self, yes: bool) -> Self` - Requires that options use the `--option=val` syntax
- `fn value_delimiter<impl IntoResettable<char>>(self: Self, d: impl Trait) -> Self` - Allow grouping of multiple values via a delimiter.
- `fn value_terminator<impl IntoResettable<Str>>(self: Self, term: impl Trait) -> Self` - Sentinel to **stop** parsing multiple values of a given argument.
- `fn raw(self: Self, yes: bool) -> Self` - Consume all following arguments.
- `fn default_value<impl IntoResettable<OsStr>>(self: Self, val: impl Trait) -> Self` - Value for the argument when not present.
- `fn default_values<impl Into<OsStr>, impl IntoIterator<Item = impl Into<OsStr>>>(self: Self, vals: impl Trait) -> Self` - Value for the argument when not present.
- `fn default_missing_value<impl IntoResettable<OsStr>>(self: Self, val: impl Trait) -> Self` - Value for the argument when the flag is present but no value is specified.
- `fn default_missing_value_os<impl Into<OsStr>>(self: Self, val: impl Trait) -> Self` - Value for the argument when the flag is present but no value is specified.
- `fn default_missing_values<impl Into<OsStr>, impl IntoIterator<Item = impl Into<OsStr>>>(self: Self, vals: impl Trait) -> Self` - Value for the argument when the flag is present but no value is specified.
- `fn default_missing_values_os<impl Into<OsStr>, impl IntoIterator<Item = impl Into<OsStr>>>(self: Self, vals: impl Trait) -> Self` - Value for the argument when the flag is present but no value is specified.
- `fn new<impl Into<Id>>(id: impl Trait) -> Self` - Create a new [`Arg`] with a unique name.
- `fn id<impl Into<Id>>(self: Self, id: impl Trait) -> Self` - Set the identifier used for referencing this argument in the clap API.
- `fn short<impl IntoResettable<char>>(self: Self, s: impl Trait) -> Self` - Sets the short version of the argument without the preceding `-`.
- `fn long<impl IntoResettable<Str>>(self: Self, l: impl Trait) -> Self` - Sets the long version of the argument without the preceding `--`.
- `fn alias<impl IntoResettable<Str>>(self: Self, name: impl Trait) -> Self` - Add an alias, which functions as a hidden long flag.
- `fn short_alias<impl IntoResettable<char>>(self: Self, name: impl Trait) -> Self` - Add an alias, which functions as a hidden short flag.
- `fn aliases<impl Into<Str>, impl IntoIterator<Item = impl Into<Str>>>(self: Self, names: impl Trait) -> Self` - Add aliases, which function as hidden long flags.
- `fn short_aliases<impl IntoIterator<Item = char>>(self: Self, names: impl Trait) -> Self` - Add aliases, which functions as a hidden short flag.
- `fn visible_alias<impl IntoResettable<Str>>(self: Self, name: impl Trait) -> Self` - Add an alias, which functions as a visible long flag.
- `fn visible_short_alias<impl IntoResettable<char>>(self: Self, name: impl Trait) -> Self` - Add an alias, which functions as a visible short flag.
- `fn visible_aliases<impl Into<Str>, impl IntoIterator<Item = impl Into<Str>>>(self: Self, names: impl Trait) -> Self` - Add aliases, which function as visible long flags.
- `fn visible_short_aliases<impl IntoIterator<Item = char>>(self: Self, names: impl Trait) -> Self` - Add aliases, which function as visible short flags.
- `fn index<impl IntoResettable<usize>>(self: Self, idx: impl Trait) -> Self` - Specifies the index of a positional argument **starting at** 1.
- `fn trailing_var_arg(self: Self, yes: bool) -> Self` - This is a "var arg" and everything that follows should be captured by it, as if the user had
- `fn last(self: Self, yes: bool) -> Self` - This arg is the last, or final, positional argument (i.e. has the highest
- `fn required(self: Self, yes: bool) -> Self` - Specifies that the argument must be present.
- `fn requires<impl IntoResettable<Id>>(self: Self, arg_id: impl Trait) -> Self` - Sets an argument that is required when this one is present
- `fn exclusive(self: Self, yes: bool) -> Self` - This argument must be passed alone; it conflicts with all other arguments.
- `fn global(self: Self, yes: bool) -> Self` - Specifies that an argument can be matched to all child [`Subcommand`]s.
- `fn get_id(self: &Self) -> &Id` - Get the name of the argument
- `fn get_help(self: &Self) -> Option<&StyledStr>` - Get the help specified for this argument, if any
- `fn get_long_help(self: &Self) -> Option<&StyledStr>` - Get the long help specified for this argument, if any
- `fn get_display_order(self: &Self) -> usize` - Get the placement within help
- `fn get_help_heading(self: &Self) -> Option<&str>` - Get the help heading specified for this argument, if any
- `fn get_short(self: &Self) -> Option<char>` - Get the short option name for this argument, if any
- `fn get_visible_short_aliases(self: &Self) -> Option<Vec<char>>` - Get visible short aliases for this argument, if any
- `fn get_all_short_aliases(self: &Self) -> Option<Vec<char>>` - Get *all* short aliases for this argument, if any, both visible and hidden.
- `fn get_short_and_visible_aliases(self: &Self) -> Option<Vec<char>>` - Get the short option name and its visible aliases, if any
- `fn get_long(self: &Self) -> Option<&str>` - Get the long option name for this argument, if any
- `fn get_visible_aliases(self: &Self) -> Option<Vec<&str>>` - Get visible aliases for this argument, if any
- `fn get_all_aliases(self: &Self) -> Option<Vec<&str>>` - Get *all* aliases for this argument, if any, both visible and hidden.
- `fn get_long_and_visible_aliases(self: &Self) -> Option<Vec<&str>>` - Get the long option name and its visible aliases, if any
- `fn get_aliases(self: &Self) -> Option<Vec<&str>>` - Get hidden aliases for this argument, if any
- `fn get_possible_values(self: &Self) -> Vec<PossibleValue>` - Get the names of possible values for this argument. Only useful for user
- `fn get_value_names(self: &Self) -> Option<&[Str]>` - Get the names of values for this argument.
- `fn get_num_args(self: &Self) -> Option<ValueRange>` - Get the number of values for this argument.
- `fn get_value_delimiter(self: &Self) -> Option<char>` - Get the delimiter between multiple values
- `fn get_value_terminator(self: &Self) -> Option<&Str>` - Get the value terminator for this argument. The `value_terminator` is a value
- `fn get_index(self: &Self) -> Option<usize>` - Get the index of this argument, if any
- `fn get_value_hint(self: &Self) -> ValueHint` - Get the value hint of this argument
- `fn get_default_values(self: &Self) -> &[OsStr]` - Get the default values specified for this argument, if any
- `fn is_positional(self: &Self) -> bool` - Checks whether this argument is a positional or not.
- `fn is_required_set(self: &Self) -> bool` - Reports whether [`Arg::required`] is set
- `fn is_allow_hyphen_values_set(self: &Self) -> bool` - Report whether [`Arg::allow_hyphen_values`] is set
- `fn is_allow_negative_numbers_set(self: &Self) -> bool` - Report whether [`Arg::allow_negative_numbers`] is set
- `fn get_action(self: &Self) -> &ArgAction` - Behavior when parsing the argument
- `fn get_value_parser(self: &Self) -> &super::ValueParser` - Configured parser for argument values
- `fn is_global_set(self: &Self) -> bool` - Report whether [`Arg::global`] is set
- `fn is_next_line_help_set(self: &Self) -> bool` - Report whether [`Arg::next_line_help`] is set
- `fn is_hide_set(self: &Self) -> bool` - Report whether [`Arg::hide`] is set
- `fn is_hide_default_value_set(self: &Self) -> bool` - Report whether [`Arg::hide_default_value`] is set
- `fn is_hide_possible_values_set(self: &Self) -> bool` - Report whether [`Arg::hide_possible_values`] is set
- `fn is_hide_short_help_set(self: &Self) -> bool` - Report whether [`Arg::hide_short_help`] is set
- `fn is_hide_long_help_set(self: &Self) -> bool` - Report whether [`Arg::hide_long_help`] is set
- `fn is_require_equals_set(self: &Self) -> bool` - Report whether [`Arg::require_equals`] is set
- `fn is_exclusive_set(self: &Self) -> bool` - Reports whether [`Arg::exclusive`] is set
- `fn is_trailing_var_arg_set(self: &Self) -> bool` - Report whether [`Arg::trailing_var_arg`] is set
- `fn is_last_set(self: &Self) -> bool` - Reports whether [`Arg::last`] is set
- `fn is_ignore_case_set(self: &Self) -> bool` - Reports whether [`Arg::ignore_case`] is set

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Arg) -> bool`
- **From**
  - `fn from(a: &Arg) -> Self`
- **Default**
  - `fn default() -> Arg`
- **Display**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> Arg`
- **Debug**
  - `fn fmt(self: &Self, f: & mut Formatter) -> Result<(), fmt::Error>`
- **Ord**
  - `fn cmp(self: &Self, other: &Arg) -> Ordering`



