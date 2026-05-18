**clap_builder > derive**

# Module: derive

## Contents

**Traits**

- [`Args`](#args) - Parse a set of arguments into a user-defined container.
- [`CommandFactory`](#commandfactory) - Create a [`Command`] relevant for a user-defined container.
- [`FromArgMatches`](#fromargmatches) - Converts an instance of [`ArgMatches`] to a user-defined container.
- [`Parser`](#parser) - Parse command-line arguments into `Self`.
- [`Subcommand`](#subcommand) - Parse a sub-command into a user-defined enum.
- [`ValueEnum`](#valueenum) - Parse arguments into enums.

---

## clap_builder::derive::Args

*Trait*

Parse a set of arguments into a user-defined container.

Implementing this trait lets a parent container delegate argument parsing behavior to `Self`.
with:
- `#[command(flatten)] args: ChildArgs`: Attribute can only be used with struct fields that impl
  `Args`.
- `Variant(ChildArgs)`: No attribute is used with enum variants that impl `Args`.

<div class="warning">

**NOTE:** Deriving requires the `derive` feature flag

</div>

**Methods:**

- `group_id`: Report the [`ArgGroup::id`][crate::ArgGroup::id] for this set of arguments
- `augment_args`: Append to [`Command`] so it can instantiate `Self` via
- `augment_args_for_update`: Append to [`Command`] so it can instantiate `self` via



## clap_builder::derive::CommandFactory

*Trait*

Create a [`Command`] relevant for a user-defined container.

Derived as part of [`Parser`].

**Methods:**

- `command`: Build a [`Command`] that can instantiate `Self`.
- `command_for_update`: Build a [`Command`] that can update `self`.



## clap_builder::derive::FromArgMatches

*Trait*

Converts an instance of [`ArgMatches`] to a user-defined container.

Derived as part of [`Parser`], [`Args`], and [`Subcommand`].

**Methods:**

- `from_arg_matches`: Instantiate `Self` from [`ArgMatches`], parsing the arguments as needed.
- `from_arg_matches_mut`: Instantiate `Self` from [`ArgMatches`], parsing the arguments as needed.
- `update_from_arg_matches`: Assign values from `ArgMatches` to `self`.
- `update_from_arg_matches_mut`: Assign values from `ArgMatches` to `self`.



## clap_builder::derive::Parser

*Trait*

Parse command-line arguments into `Self`.

The primary one-stop-shop trait used to create an instance of a `clap`
[`Command`], conduct the parsing, and turn the resulting [`ArgMatches`] back
into concrete instance of the user struct.

This trait is primarily a convenience on top of [`FromArgMatches`] +
[`CommandFactory`] which uses those two underlying traits to build the two
fundamental functions `parse` which uses the `std::env::args_os` iterator,
and `parse_from` which allows the consumer to supply the iterator (along
with fallible options for each).

See also [`Subcommand`] and [`Args`].

<div class="warning">

**NOTE:** Deriving requires the `derive` feature flag

</div>

**Methods:**

- `parse`: Parse from `std::env::args_os()`, [exit][Error::exit] on error.
- `try_parse`: Parse from `std::env::args_os()`, return Err on error.
- `parse_from`: Parse from iterator, [exit][Error::exit] on error.
- `try_parse_from`: Parse from iterator, return Err on error.
- `update_from`: Update from iterator, [exit][Error::exit] on error.
- `try_update_from`: Update from iterator, return Err on error.



## clap_builder::derive::Subcommand

*Trait*

Parse a sub-command into a user-defined enum.

Implementing this trait lets a parent container delegate subcommand behavior to `Self`.
with:
- `#[command(subcommand)] field: SubCmd`: Attribute can be used with either struct fields or enum
  variants that impl `Subcommand`.
- `#[command(flatten)] Variant(SubCmd)`: Attribute can only be used with enum variants that impl
  `Subcommand`.

<div class="warning">

**NOTE:** Deriving requires the `derive` feature flag

</div>

**Methods:**

- `augment_subcommands`: Append to [`Command`] so it can instantiate `Self` via
- `augment_subcommands_for_update`: Append to [`Command`] so it can instantiate `self` via
- `has_subcommand`: Test whether `Self` can parse a specific subcommand



## clap_builder::derive::ValueEnum

*Trait*

Parse arguments into enums.

When deriving [`Parser`], a field whose type implements `ValueEnum` can have the attribute
`#[arg(value_enum)]` which will
- Call [`EnumValueParser`][crate::builder::EnumValueParser]
- Allowing using the `#[arg(default_value_t)]` attribute without implementing `Display`.

<div class="warning">

**NOTE:** Deriving requires the `derive` feature flag

</div>

**Methods:**

- `value_variants`: All possible argument values, in display order.
- `from_str`: Parse an argument into `Self`.
- `to_possible_value`: The canonical argument value.



