**clap_builder**

# Module: clap_builder

## Contents

**Modules**

- [`builder`](#builder) - Define [`Command`] line [arguments][`Arg`]
- [`error`](#error) - Error reporting
- [`parser`](#parser) - [`Command`][crate::Command] line argument parser

**Macros**

- [`arg`](#arg) - Create an [`Arg`] from a usage string.
- [`command`](#command) - Requires `cargo` feature flag to be enabled.
- [`value_parser`](#value_parser) - Select a [`ValueParser`] implementation from the intended type

**Type Aliases**

- [`Error`](#error) - Command Line Argument Parser Error

---

## clap_builder::Error

*Type Alias*: `error::Error<error::DefaultFormatter>`

Command Line Argument Parser Error

See [`Command::error`] to create an error.

[`Command::error`]: crate::Command::error



## clap_builder::arg

*Declarative Macro*

Create an [`Arg`] from a usage string.

Allows creation of basic settings for the [`Arg`].

<div class="warning">

**NOTE**: Not all settings may be set using the usage string method. Some properties are
only available via the builder pattern.

</div>

# Syntax

Usage strings typically following the form:

```notrust
[explicit name] [short] [long] [value names] [...] [help string]
```

### Explicit Name

The name may be either a bare-word or a string, followed by a `:`, like `name:` or
`"name":`.

*Note:* This is an optional field, if it's omitted the argument will use one of the additional
fields as the name using the following priority order:

 1. Explicit Name
 2. Long
 3. Value Name

See [`Arg::id`][crate::Arg::id].

### Short

A short flag is a `-` followed by either a bare-character or quoted character, like `-f` or
`-'f'`.

See [`Arg::short`][crate::Arg::short].

### Long

A long flag is a `--` followed by either a bare-word or a string, like `--foo` or
`--"foo"`.

<div class="warning">

**NOTE:** Dashes in the long name (e.g. `--foo-bar`) is not supported and quoting is required
(e.g. `--"foo-bar"`).

</div>

See [`Arg::long`][crate::Arg::long].

### Values (Value Notation)

This is set by placing bare-word between:
- `[]` like `[FOO]`
  - Positional argument: optional
  - Named argument: optional value
- `<>` like `<FOO>`: required

See [`Arg::value_name`][crate::Arg::value_name].

### `...`

`...` (three consecutive dots/periods) specifies that this argument may occur multiple
times (not to be confused with multiple values per occurrence).

See [`ArgAction::Count`][crate::ArgAction::Count] and [`ArgAction::Append`][crate::ArgAction::Append].

### Help String

The help string is denoted between a pair of double quotes `""` and may contain any
characters.

# Examples

```rust
# use clap_builder as clap;
# use clap::{Command, Arg, arg};
let cmd = Command::new("prog")
    .args(&[
        arg!(--config <FILE> "a required file for the configuration and no short"),
        arg!(-d --debug ... "turns on debugging information and allows multiples"),
        arg!([input] "an optional input file to use")
    ]);

let m = cmd.try_get_matches_from(["prog", "--config", "file.toml"]).unwrap();
assert_eq!(m.get_one::<String>("config").unwrap(), "file.toml");
assert_eq!(*m.get_one::<u8>("debug").unwrap(), 0);
assert_eq!(m.get_one::<String>("input"), None);
```
[`Arg`]: crate::Arg

```rust
macro_rules! arg {
    ( -$($tail:tt)+ ) => { ... };
    ( $name:ident: $($tail:tt)+ ) => { ... };
    ( $name:literal: $($tail:tt)+ ) => { ... };
    ( $($tail:tt)+ ) => { ... };
}
```



## Module: builder

Define [`Command`] line [arguments][`Arg`]



## clap_builder::command

*Declarative Macro*

Requires `cargo` feature flag to be enabled.

```rust
macro_rules! command {
    () => { ... };
    ($name:expr) => { ... };
}
```



## Module: error

Error reporting



## Module: parser

[`Command`][crate::Command] line argument parser



## clap_builder::value_parser

*Declarative Macro*

Select a [`ValueParser`] implementation from the intended type

Supported types
- [`ValueParserFactory` types][ValueParserFactory], including
  - [Native types][ValueParser]: `bool`, `String`, `OsString`, `PathBuf`
  - [Ranged numeric types][RangedI64ValueParser]: `u8`, `i8`, `u16`, `i16`, `u32`, `i32`, `u64`, `i64`
- [`ValueEnum` types][crate::ValueEnum]
- [`From<OsString>` types][std::convert::From] and [`From<&OsStr>` types][std::convert::From]
- [`From<String>` types][std::convert::From] and [`From<&str>` types][std::convert::From]
- [`FromStr` types][std::str::FromStr], including usize, isize

# Example

Usage:
```rust
# use clap_builder as clap;
# use std::path::PathBuf;
# use std::path::Path;
let mut cmd = clap::Command::new("raw")
    .arg(
        clap::Arg::new("output")
            .value_parser(clap::value_parser!(PathBuf))
            .required(true)
    );

let m = cmd.try_get_matches_from_mut(["cmd", "file.txt"]).unwrap();
let port: &PathBuf = m.get_one("output")
    .expect("required");
assert_eq!(port, Path::new("file.txt"));
```

Example mappings:
```rust
# use clap_builder as clap;
# use clap::ColorChoice;
// Built-in types
let parser = clap::value_parser!(String);
assert_eq!(format!("{parser:?}"), "ValueParser::string");
let parser = clap::value_parser!(std::ffi::OsString);
assert_eq!(format!("{parser:?}"), "ValueParser::os_string");
let parser = clap::value_parser!(std::path::PathBuf);
assert_eq!(format!("{parser:?}"), "ValueParser::path_buf");
clap::value_parser!(u16).range(3000..);
clap::value_parser!(u64).range(3000..);

// FromStr types
let parser = clap::value_parser!(usize);
assert_eq!(format!("{parser:?}"), "_AnonymousValueParser(ValueParser::other(usize))");

// ValueEnum types
clap::value_parser!(ColorChoice);
```

```rust
macro_rules! value_parser {
    ($name:ty) => { ... };
}
```



