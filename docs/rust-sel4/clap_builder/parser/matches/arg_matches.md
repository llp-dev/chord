**clap_builder > parser > matches > arg_matches**

# Module: parser::matches::arg_matches

## Contents

**Structs**

- [`ArgMatches`](#argmatches) - Container for parse results.
- [`IdsRef`](#idsref) - Iterate over [`Arg`][crate::Arg] and [`ArgGroup`][crate::ArgGroup] [`Id`]s via [`ArgMatches::ids`].
- [`Indices`](#indices) - Iterate over indices for where an argument appeared when parsing, via [`ArgMatches::indices_of`]
- [`OccurrenceValues`](#occurrencevalues)
- [`OccurrenceValuesRef`](#occurrencevaluesref)
- [`Occurrences`](#occurrences)
- [`OccurrencesRef`](#occurrencesref)
- [`RawOccurrenceValues`](#rawoccurrencevalues)
- [`RawOccurrences`](#rawoccurrences)
- [`RawValues`](#rawvalues) - Iterate over raw argument values via [`ArgMatches::get_raw`].
- [`Values`](#values) - Iterate over multiple values for an argument via [`ArgMatches::remove_many`].
- [`ValuesRef`](#valuesref) - Iterate over multiple values for an argument via [`ArgMatches::get_many`].

---

## clap_builder::parser::matches::arg_matches::ArgMatches

*Struct*

Container for parse results.

Used to get information about the arguments that were supplied to the program at runtime by
the user. New instances of this struct are obtained by using the [`Command::get_matches`] family of
methods.

# Examples

```no_run
# use clap_builder as clap;
# use clap::{Command, Arg, ArgAction};
# use clap::parser::ValueSource;
let matches = Command::new("MyApp")
    .arg(Arg::new("out")
        .long("output")
        .required(true)
        .action(ArgAction::Set)
        .default_value("-"))
    .arg(Arg::new("cfg")
        .short('c')
        .action(ArgAction::Set))
    .get_matches(); // builds the instance of ArgMatches

// to get information about the "cfg" argument we created, such as the value supplied we use
// various ArgMatches methods, such as [ArgMatches::get_one]
if let Some(c) = matches.get_one::<String>("cfg") {
    println!("Value for -c: {c}");
}

// The ArgMatches::get_one method returns an Option because the user may not have supplied
// that argument at runtime. But if we specified that the argument was "required" as we did
// with the "out" argument, we can safely unwrap because `clap` verifies that was actually
// used at runtime.
println!("Value for --output: {}", matches.get_one::<String>("out").unwrap());

// You can check the presence of an argument's values
if matches.contains_id("out") {
    // However, if you want to know where the value came from
    if matches.value_source("out").expect("checked contains_id") == ValueSource::CommandLine {
        println!("`out` set by user");
    } else {
        println!("`out` is defaulted");
    }
}
```
[`Command::get_matches`]: crate::Command::get_matches()

**Methods:**

- `fn try_get_one<T>(self: &Self, id: &str) -> Result<Option<&T>, MatchesError>` - Non-panicking version of [`ArgMatches::get_one`]
- `fn try_get_many<T>(self: &Self, id: &str) -> Result<Option<ValuesRef<T>>, MatchesError>` - Non-panicking version of [`ArgMatches::get_many`]
- `fn try_get_occurrences<T>(self: &Self, id: &str) -> Result<Option<OccurrencesRef<T>>, MatchesError>` - Non-panicking version of [`ArgMatches::get_occurrences`]
- `fn try_get_raw(self: &Self, id: &str) -> Result<Option<RawValues>, MatchesError>` - Non-panicking version of [`ArgMatches::get_raw`]
- `fn try_get_raw_occurrences(self: &Self, id: &str) -> Result<Option<RawOccurrences>, MatchesError>` - Non-panicking version of [`ArgMatches::get_raw_occurrences`]
- `fn try_remove_one<T>(self: & mut Self, id: &str) -> Result<Option<T>, MatchesError>` - Non-panicking version of [`ArgMatches::remove_one`]
- `fn try_remove_many<T>(self: & mut Self, id: &str) -> Result<Option<Values<T>>, MatchesError>` - Non-panicking version of [`ArgMatches::remove_many`]
- `fn try_remove_occurrences<T>(self: & mut Self, id: &str) -> Result<Option<Occurrences<T>>, MatchesError>` - Non-panicking version of [`ArgMatches::remove_occurrences`]
- `fn try_contains_id(self: &Self, id: &str) -> Result<bool, MatchesError>` - Non-panicking version of [`ArgMatches::contains_id`]
- `fn try_clear_id(self: & mut Self, id: &str) -> Result<bool, MatchesError>` - Clears the values for the given `id`
- `fn get_one<T>(self: &Self, id: &str) -> Option<&T>` - Gets the value of a specific option or positional argument.
- `fn get_count(self: &Self, id: &str) -> u8` - Gets the value of a specific [`ArgAction::Count`][crate::ArgAction::Count] flag
- `fn get_flag(self: &Self, id: &str) -> bool` - Gets the value of a specific [`ArgAction::SetTrue`][crate::ArgAction::SetTrue] or [`ArgAction::SetFalse`][crate::ArgAction::SetFalse] flag
- `fn get_many<T>(self: &Self, id: &str) -> Option<ValuesRef<T>>` - Iterate over values of a specific option or positional argument.
- `fn get_occurrences<T>(self: &Self, id: &str) -> Option<OccurrencesRef<T>>` - Iterate over the values passed to each occurrence of an option.
- `fn get_raw(self: &Self, id: &str) -> Option<RawValues>` - Iterate over the original argument values.
- `fn get_raw_occurrences(self: &Self, id: &str) -> Option<RawOccurrences>` - Iterate over the original values for each occurrence of an option.
- `fn remove_one<T>(self: & mut Self, id: &str) -> Option<T>` - Returns the value of a specific option or positional argument.
- `fn remove_many<T>(self: & mut Self, id: &str) -> Option<Values<T>>` - Return values of a specific option or positional argument.
- `fn remove_occurrences<T>(self: & mut Self, id: &str) -> Option<Occurrences<T>>` - Return values for each occurrence of an option.
- `fn contains_id(self: &Self, id: &str) -> bool` - Check if values are present for the argument or group id
- `fn ids(self: &Self) -> IdsRef` - Iterate over [`Arg`][crate::Arg] and [`ArgGroup`][crate::ArgGroup] [`Id`]s via [`ArgMatches::ids`].
- `fn args_present(self: &Self) -> bool` - Check if any [`Arg`][crate::Arg]s were present on the command line
- `fn value_source(self: &Self, id: &str) -> Option<ValueSource>` - Report where argument value came from
- `fn index_of(self: &Self, id: &str) -> Option<usize>` - The first index of that an argument showed up.
- `fn indices_of(self: &Self, id: &str) -> Option<Indices>` - All indices an argument appeared at when parsing.
- `fn subcommand(self: &Self) -> Option<(&str, &ArgMatches)>` - The name and `ArgMatches` of the current [subcommand].
- `fn remove_subcommand(self: & mut Self) -> Option<(String, ArgMatches)>` - Return the name and `ArgMatches` of the current [subcommand].
- `fn subcommand_matches(self: &Self, name: &str) -> Option<&ArgMatches>` - The `ArgMatches` for the current [subcommand].
- `fn subcommand_name(self: &Self) -> Option<&str>` - The name of the current [subcommand].

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> ArgMatches`
- **PartialEq**
  - `fn eq(self: &Self, other: &ArgMatches) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> ArgMatches`



## clap_builder::parser::matches::arg_matches::IdsRef

*Struct*

Iterate over [`Arg`][crate::Arg] and [`ArgGroup`][crate::ArgGroup] [`Id`]s via [`ArgMatches::ids`].

# Examples

```rust
# use clap_builder as clap;
# use clap::{Command, arg, value_parser};

let m = Command::new("myprog")
    .arg(arg!(--color <when>)
        .value_parser(["auto", "always", "never"]))
    .arg(arg!(--config <path>)
        .value_parser(value_parser!(std::path::PathBuf)))
    .get_matches_from(["myprog", "--config=config.toml", "--color=auto"]);
assert_eq!(
    m.ids()
        .map(|id| id.as_str())
        .collect::<Vec<_>>(),
    ["config", "color"]
);
```

**Generic Parameters:**
- 'a

**Traits:** ExactSizeIterator

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<&'a Id>`
- **Clone**
  - `fn clone(self: &Self) -> IdsRef<'a>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<&'a Id>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`



## clap_builder::parser::matches::arg_matches::Indices

*Struct*

Iterate over indices for where an argument appeared when parsing, via [`ArgMatches::indices_of`]

# Examples

```rust
# use clap_builder as clap;
# use clap::{Command, Arg, ArgAction};
let m = Command::new("myapp")
    .arg(Arg::new("output")
        .short('o')
        .num_args(1..)
        .action(ArgAction::Set))
    .get_matches_from(vec!["myapp", "-o", "val1", "val2"]);

let mut indices = m.indices_of("output").unwrap();

assert_eq!(indices.next(), Some(2));
assert_eq!(indices.next(), Some(3));
assert_eq!(indices.next(), None);
```
[`ArgMatches::indices_of`]: ArgMatches::indices_of()

**Generic Parameters:**
- 'a

**Traits:** ExactSizeIterator

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<usize>`
- **Clone**
  - `fn clone(self: &Self) -> Indices<'a>`
- **Default**
  - `fn default() -> Self`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<usize>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`



## clap_builder::parser::matches::arg_matches::OccurrenceValues

*Struct*

**Generic Parameters:**
- T



## clap_builder::parser::matches::arg_matches::OccurrenceValuesRef

*Struct*

**Generic Parameters:**
- 'a
- T



## clap_builder::parser::matches::arg_matches::Occurrences

*Struct*

**Generic Parameters:**
- T



## clap_builder::parser::matches::arg_matches::OccurrencesRef

*Struct*

**Generic Parameters:**
- 'a
- T



## clap_builder::parser::matches::arg_matches::RawOccurrenceValues

*Struct*

**Generic Parameters:**
- 'a



## clap_builder::parser::matches::arg_matches::RawOccurrences

*Struct*

**Generic Parameters:**
- 'a



## clap_builder::parser::matches::arg_matches::RawValues

*Struct*

Iterate over raw argument values via [`ArgMatches::get_raw`].

# Examples

```rust
# #[cfg(unix)] {
# use clap_builder as clap;
# use clap::{Command, arg, value_parser};
use std::ffi::OsString;
use std::os::unix::ffi::{OsStrExt,OsStringExt};

let m = Command::new("utf8")
    .arg(arg!(<arg> "some arg")
        .value_parser(value_parser!(OsString)))
    .get_matches_from(vec![OsString::from("myprog"),
                            // "Hi {0xe9}!"
                            OsString::from_vec(vec![b'H', b'i', b' ', 0xe9, b'!'])]);
assert_eq!(
    &*m.get_raw("arg")
        .unwrap()
        .next().unwrap()
        .as_bytes(),
    [b'H', b'i', b' ', 0xe9, b'!']
);
# }
```

**Generic Parameters:**
- 'a

**Traits:** ExactSizeIterator

**Trait Implementations:**

- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<&'a OsStr>`
- **Clone**
  - `fn clone(self: &Self) -> RawValues<'a>`
- **Default**
  - `fn default() -> Self`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<&'a OsStr>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## clap_builder::parser::matches::arg_matches::Values

*Struct*

Iterate over multiple values for an argument via [`ArgMatches::remove_many`].

# Examples

```rust
# use clap_builder as clap;
# use clap::{Command, Arg, ArgAction};
let mut m = Command::new("myapp")
    .arg(Arg::new("output")
        .short('o')
        .action(ArgAction::Append))
    .get_matches_from(vec!["myapp", "-o", "val1", "-o", "val2"]);

let mut values = m.remove_many::<String>("output")
    .unwrap();

assert_eq!(values.next(), Some(String::from("val1")));
assert_eq!(values.next(), Some(String::from("val2")));
assert_eq!(values.next(), None);
```

**Generic Parameters:**
- T

**Traits:** ExactSizeIterator

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<<Self as >::Item>`
- **Clone**
  - `fn clone(self: &Self) -> Values<T>`
- **Default**
  - `fn default() -> Self`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`



## clap_builder::parser::matches::arg_matches::ValuesRef

*Struct*

Iterate over multiple values for an argument via [`ArgMatches::get_many`].

# Examples

```rust
# use clap_builder as clap;
# use clap::{Command, Arg, ArgAction};
let m = Command::new("myapp")
    .arg(Arg::new("output")
        .short('o')
        .action(ArgAction::Append))
    .get_matches_from(vec!["myapp", "-o", "val1", "-o", "val2"]);

let mut values = m.get_many::<String>("output")
    .unwrap()
    .map(|s| s.as_str());

assert_eq!(values.next(), Some("val1"));
assert_eq!(values.next(), Some("val2"));
assert_eq!(values.next(), None);
```

**Generic Parameters:**
- 'a
- T

**Traits:** ExactSizeIterator

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<<Self as >::Item>`
- **Clone**
  - `fn clone(self: &Self) -> ValuesRef<'a, T>`
- **Default**
  - `fn default() -> Self`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`



