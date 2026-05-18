**clap_builder > builder > value_parser**

# Module: builder::value_parser

## Contents

**Structs**

- [`BoolValueParser`](#boolvalueparser) - Implementation for [`ValueParser::bool`]
- [`BoolishValueParser`](#boolishvalueparser) - Parse bool-like string values
- [`EnumValueParser`](#enumvalueparser) - Parse an [`ValueEnum`][crate::ValueEnum] value.
- [`FalseyValueParser`](#falseyvalueparser) - Parse false-like string values, everything else is `true`
- [`MapValueParser`](#mapvalueparser) - Adapt a `TypedValueParser` from one value to another
- [`NonEmptyStringValueParser`](#nonemptystringvalueparser) - Parse non-empty string values
- [`OsStringValueParser`](#osstringvalueparser) - Implementation for [`ValueParser::os_string`]
- [`PathBufValueParser`](#pathbufvalueparser) - Implementation for [`ValueParser::path_buf`]
- [`PossibleValuesParser`](#possiblevaluesparser) - Verify the value is from an enumerated set of [`PossibleValue`][crate::builder::PossibleValue].
- [`RangedI64ValueParser`](#rangedi64valueparser) - Parse number that fall within a range of values
- [`RangedU64ValueParser`](#rangedu64valueparser) - Parse number that fall within a range of values
- [`StringValueParser`](#stringvalueparser) - Implementation for [`ValueParser::string`]
- [`TryMapValueParser`](#trymapvalueparser) - Adapt a `TypedValueParser` from one value to another
- [`UnknownArgumentValueParser`](#unknownargumentvalueparser) - When encountered, report [`ErrorKind::UnknownArgument`][crate::error::ErrorKind::UnknownArgument]
- [`ValueParser`](#valueparser) - Parse/validate argument values

**Traits**

- [`TypedValueParser`](#typedvalueparser) - Parse/validate argument values
- [`ValueParserFactory`](#valueparserfactory) - Register a type with [`value_parser!`][crate::value_parser!]

---

## clap_builder::builder::value_parser::BoolValueParser

*Struct*

Implementation for [`ValueParser::bool`]

Useful for composing new [`TypedValueParser`]s

**Methods:**

- `fn new() -> Self` - Implementation for [`ValueParser::bool`]

**Traits:** Copy

**Trait Implementations:**

- **TypedValueParser**
  - `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>`
  - `fn possible_values(self: &Self) -> Option<Box<dyn Iterator>>`
- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> BoolValueParser`



## clap_builder::builder::value_parser::BoolishValueParser

*Struct*

Parse bool-like string values

See also:
- [`ValueParser::bool`] for different human readable bool representations
- [`FalseyValueParser`] for assuming non-false is true

# Example

Usage:
```rust
# use clap_builder as clap;
let mut cmd = clap::Command::new("raw")
    .arg(
        clap::Arg::new("append")
            .value_parser(clap::builder::BoolishValueParser::new())
            .required(true)
    );

let m = cmd.try_get_matches_from_mut(["cmd", "true"]).unwrap();
let port: bool = *m.get_one("append")
    .expect("required");
assert_eq!(port, true);
```

Semantics:
```rust
# use clap_builder as clap;
# use std::ffi::OsStr;
# use clap::builder::TypedValueParser;
# let cmd = clap::Command::new("test");
# let arg = None;
let value_parser = clap::builder::BoolishValueParser::new();
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("random")).is_err());
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("")).is_err());
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("100")).is_err());
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("true")).unwrap(), true);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("Yes")).unwrap(), true);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("oN")).unwrap(), true);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("1")).unwrap(), true);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("false")).unwrap(), false);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("No")).unwrap(), false);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("oFF")).unwrap(), false);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("0")).unwrap(), false);
```

**Methods:**

- `fn new() -> Self` - Parse bool-like string values

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> BoolishValueParser`
- **TypedValueParser**
  - `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>`
  - `fn possible_values(self: &Self) -> Option<Box<dyn Iterator>>`
- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## clap_builder::builder::value_parser::EnumValueParser

*Struct*

Parse an [`ValueEnum`][crate::ValueEnum] value.

See also:
- [`PossibleValuesParser`]

# Example

```rust
# use clap_builder as clap;
# use std::ffi::OsStr;
# use clap::ColorChoice;
# use clap::builder::TypedValueParser;
# let cmd = clap::Command::new("test");
# let arg = None;

// Usage
let mut cmd = clap::Command::new("raw")
    .arg(
        clap::Arg::new("color")
            .value_parser(clap::builder::EnumValueParser::<ColorChoice>::new())
            .required(true)
    );

let m = cmd.try_get_matches_from_mut(["cmd", "always"]).unwrap();
let port: ColorChoice = *m.get_one("color")
    .expect("required");
assert_eq!(port, ColorChoice::Always);

// Semantics
let value_parser = clap::builder::EnumValueParser::<ColorChoice>::new();
// or
let value_parser = clap::value_parser!(ColorChoice);
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("random")).is_err());
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("")).is_err());
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("always")).unwrap(), ColorChoice::Always);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("auto")).unwrap(), ColorChoice::Auto);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("never")).unwrap(), ColorChoice::Never);
```

**Generic Parameters:**
- E

**Tuple Struct**: `()`

**Methods:**

- `fn new() -> Self` - Parse an [`ValueEnum`][crate::ValueEnum]

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **TypedValueParser**
  - `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>`
  - `fn possible_values(self: &Self) -> Option<Box<dyn Iterator>>`
- **Clone**
  - `fn clone(self: &Self) -> EnumValueParser<E>`



## clap_builder::builder::value_parser::FalseyValueParser

*Struct*

Parse false-like string values, everything else is `true`

See also:
- [`ValueParser::bool`] for assuming non-false is true
- [`BoolishValueParser`] for different human readable bool representations

# Example

Usage:
```rust
# use clap_builder as clap;
let mut cmd = clap::Command::new("raw")
    .arg(
        clap::Arg::new("append")
            .value_parser(clap::builder::FalseyValueParser::new())
            .required(true)
    );

let m = cmd.try_get_matches_from_mut(["cmd", "true"]).unwrap();
let port: bool = *m.get_one("append")
    .expect("required");
assert_eq!(port, true);
```

Semantics:
```rust
# use clap_builder as clap;
# use std::ffi::OsStr;
# use clap::builder::TypedValueParser;
# let cmd = clap::Command::new("test");
# let arg = None;
let value_parser = clap::builder::FalseyValueParser::new();
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("random")).unwrap(), true);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("100")).unwrap(), true);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("")).unwrap(), false);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("false")).unwrap(), false);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("No")).unwrap(), false);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("oFF")).unwrap(), false);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("0")).unwrap(), false);
```

**Methods:**

- `fn new() -> Self` - Parse false-like string values, everything else is `true`

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> FalseyValueParser`
- **TypedValueParser**
  - `fn parse_ref(self: &Self, cmd: &crate::Command, _arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>`
  - `fn possible_values(self: &Self) -> Option<Box<dyn Iterator>>`
- **Default**
  - `fn default() -> Self`



## clap_builder::builder::value_parser::MapValueParser

*Struct*

Adapt a `TypedValueParser` from one value to another

See [`TypedValueParser::map`]

**Generic Parameters:**
- P
- F

**Trait Implementations:**

- **TypedValueParser**
  - `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>`
  - `fn parse(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: std::ffi::OsString) -> Result<<Self as >::Value, crate::Error>`
  - `fn possible_values(self: &Self) -> Option<Box<dyn Iterator>>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> MapValueParser<P, F>`



## clap_builder::builder::value_parser::NonEmptyStringValueParser

*Struct*

Parse non-empty string values

See also:
- [`ValueParser::string`]

# Example

Usage:
```rust
# use clap_builder as clap;
let mut cmd = clap::Command::new("raw")
    .arg(
        clap::Arg::new("append")
            .value_parser(clap::builder::NonEmptyStringValueParser::new())
            .required(true)
    );

let m = cmd.try_get_matches_from_mut(["cmd", "true"]).unwrap();
let port: &String = m.get_one("append")
    .expect("required");
assert_eq!(port, "true");
```

Semantics:
```rust
# use clap_builder as clap;
# use std::ffi::OsStr;
# use clap::builder::TypedValueParser;
# let cmd = clap::Command::new("test");
# let arg = None;
let value_parser = clap::builder::NonEmptyStringValueParser::new();
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("random")).unwrap(), "random");
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("")).is_err());
```

**Methods:**

- `fn new() -> Self` - Parse non-empty string values

**Traits:** Copy

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **TypedValueParser**
  - `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>`
- **Clone**
  - `fn clone(self: &Self) -> NonEmptyStringValueParser`



## clap_builder::builder::value_parser::OsStringValueParser

*Struct*

Implementation for [`ValueParser::os_string`]

Useful for composing new [`TypedValueParser`]s

**Methods:**

- `fn new() -> Self` - Implementation for [`ValueParser::os_string`]

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> OsStringValueParser`
- **TypedValueParser**
  - `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>`
  - `fn parse(self: &Self, _cmd: &crate::Command, _arg: Option<&crate::Arg>, value: std::ffi::OsString) -> Result<<Self as >::Value, crate::Error>`
- **Default**
  - `fn default() -> Self`



## clap_builder::builder::value_parser::PathBufValueParser

*Struct*

Implementation for [`ValueParser::path_buf`]

Useful for composing new [`TypedValueParser`]s

**Methods:**

- `fn new() -> Self` - Implementation for [`ValueParser::path_buf`]

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> PathBufValueParser`
- **Default**
  - `fn default() -> Self`
- **TypedValueParser**
  - `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>`
  - `fn parse(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: std::ffi::OsString) -> Result<<Self as >::Value, crate::Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## clap_builder::builder::value_parser::PossibleValuesParser

*Struct*

Verify the value is from an enumerated set of [`PossibleValue`][crate::builder::PossibleValue].

See also:
- [`EnumValueParser`] for directly supporting [`ValueEnum`][crate::ValueEnum] types
- [`TypedValueParser::map`] for adapting values to a more specialized type, like an external
  enums that can't implement [`ValueEnum`][crate::ValueEnum]

# Example

Usage:
```rust
# use clap_builder as clap;
let mut cmd = clap::Command::new("raw")
    .arg(
        clap::Arg::new("color")
            .value_parser(clap::builder::PossibleValuesParser::new(["always", "auto", "never"]))
            .required(true)
    );

let m = cmd.try_get_matches_from_mut(["cmd", "always"]).unwrap();
let port: &String = m.get_one("color")
    .expect("required");
assert_eq!(port, "always");
```

Semantics:
```rust
# use clap_builder as clap;
# use std::ffi::OsStr;
# use clap::builder::TypedValueParser;
# let cmd = clap::Command::new("test");
# let arg = None;
let value_parser = clap::builder::PossibleValuesParser::new(["always", "auto", "never"]);
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("random")).is_err());
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("")).is_err());
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("always")).unwrap(), "always");
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("auto")).unwrap(), "auto");
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("never")).unwrap(), "never");
```

**Tuple Struct**: `()`

**Methods:**

- `fn new<impl Into<PossibleValuesParser>>(values: impl Trait) -> Self` - Verify the value is from an enumerated set of [`PossibleValue`][crate::builder::PossibleValue].

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> PossibleValuesParser`
- **TypedValueParser**
  - `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>`
  - `fn parse(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: std::ffi::OsString) -> Result<String, crate::Error>`
  - `fn possible_values(self: &Self) -> Option<Box<dyn Iterator>>`
- **From**
  - `fn from(values: I) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## clap_builder::builder::value_parser::RangedI64ValueParser

*Struct*

Parse number that fall within a range of values

<div class="warning">

**NOTE:** To capture negative values, you will also need to set
[`Arg::allow_negative_numbers`][crate::Arg::allow_negative_numbers] or
[`Arg::allow_hyphen_values`][crate::Arg::allow_hyphen_values].

</div>

# Example

Usage:
```rust
# use clap_builder as clap;
let mut cmd = clap::Command::new("raw")
    .arg(
        clap::Arg::new("port")
            .long("port")
            .value_parser(clap::value_parser!(u16).range(3000..))
            .action(clap::ArgAction::Set)
            .required(true)
    );

let m = cmd.try_get_matches_from_mut(["cmd", "--port", "3001"]).unwrap();
let port: u16 = *m.get_one("port")
    .expect("required");
assert_eq!(port, 3001);
```

Semantics:
```rust
# use clap_builder as clap;
# use std::ffi::OsStr;
# use clap::builder::TypedValueParser;
# let cmd = clap::Command::new("test");
# let arg = None;
let value_parser = clap::builder::RangedI64ValueParser::<i32>::new().range(-1..200);
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("random")).is_err());
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("")).is_err());
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("-200")).is_err());
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("300")).is_err());
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("-1")).unwrap(), -1);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("0")).unwrap(), 0);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("50")).unwrap(), 50);
```

**Generic Parameters:**
- T

**Methods:**

- `fn new() -> Self` - Select full range of `i64`
- `fn range<B>(self: Self, range: B) -> Self` - Narrow the supported range

**Traits:** Copy

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **TypedValueParser**
  - `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, raw_value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>`
- **From**
  - `fn from(range: B) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> RangedI64ValueParser<T>`



## clap_builder::builder::value_parser::RangedU64ValueParser

*Struct*

Parse number that fall within a range of values

# Example

Usage:
```rust
# use clap_builder as clap;
let mut cmd = clap::Command::new("raw")
    .arg(
        clap::Arg::new("port")
            .long("port")
            .value_parser(clap::value_parser!(u64).range(3000..))
            .action(clap::ArgAction::Set)
            .required(true)
    );

let m = cmd.try_get_matches_from_mut(["cmd", "--port", "3001"]).unwrap();
let port: u64 = *m.get_one("port")
    .expect("required");
assert_eq!(port, 3001);
```

Semantics:
```rust
# use clap_builder as clap;
# use std::ffi::OsStr;
# use clap::builder::TypedValueParser;
# let cmd = clap::Command::new("test");
# let arg = None;
let value_parser = clap::builder::RangedU64ValueParser::<u32>::new().range(0..200);
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("random")).is_err());
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("")).is_err());
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("-200")).is_err());
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("300")).is_err());
assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("-1")).is_err());
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("0")).unwrap(), 0);
assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("50")).unwrap(), 50);
```

**Generic Parameters:**
- T

**Methods:**

- `fn new() -> Self` - Select full range of `u64`
- `fn range<B>(self: Self, range: B) -> Self` - Narrow the supported range

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> RangedU64ValueParser<T>`
- **TypedValueParser**
  - `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, raw_value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>`
- **Default**
  - `fn default() -> Self`
- **From**
  - `fn from(range: B) -> Self`



## clap_builder::builder::value_parser::StringValueParser

*Struct*

Implementation for [`ValueParser::string`]

Useful for composing new [`TypedValueParser`]s

**Methods:**

- `fn new() -> Self` - Implementation for [`ValueParser::string`]

**Traits:** Copy

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> StringValueParser`
- **TypedValueParser**
  - `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>`
  - `fn parse(self: &Self, cmd: &crate::Command, _arg: Option<&crate::Arg>, value: std::ffi::OsString) -> Result<<Self as >::Value, crate::Error>`



## clap_builder::builder::value_parser::TryMapValueParser

*Struct*

Adapt a `TypedValueParser` from one value to another

See [`TypedValueParser::try_map`]

**Generic Parameters:**
- P
- F

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> TryMapValueParser<P, F>`
- **TypedValueParser**
  - `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>`
  - `fn possible_values(self: &Self) -> Option<Box<dyn Iterator>>`



## clap_builder::builder::value_parser::TypedValueParser

*Trait*

Parse/validate argument values

As alternatives to implementing `TypedValueParser`,
- Use `Fn(&str) -> Result<T, E>` which implements `TypedValueParser`
- [`TypedValueParser::map`] or [`TypedValueParser::try_map`] to adapt an existing `TypedValueParser`

See `ValueParserFactory` to register `TypedValueParser::Value` with
[`value_parser!`][crate::value_parser].

# Example

```rust
# #[cfg(feature = "error-context")] {
# use clap_builder as clap;
# use clap::error::ErrorKind;
# use clap::error::ContextKind;
# use clap::error::ContextValue;
#[derive(Clone)]
struct Custom(u32);

#[derive(Clone)]
struct CustomValueParser;

impl clap::builder::TypedValueParser for CustomValueParser {
    type Value = Custom;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let inner = clap::value_parser!(u32);
        let val = inner.parse_ref(cmd, arg, value)?;

        const INVALID_VALUE: u32 = 10;
        if val == INVALID_VALUE {
            let mut err = clap::Error::new(ErrorKind::ValueValidation)
                .with_cmd(cmd);
            if let Some(arg) = arg {
                err.insert(ContextKind::InvalidArg, ContextValue::String(arg.to_string()));
            }
            err.insert(ContextKind::InvalidValue, ContextValue::String(INVALID_VALUE.to_string()));
            return Err(err);
        }

        Ok(Custom(val))
    }
}
# }
```

**Methods:**

- `Value`: Argument's value type
- `parse_ref`: Parse the argument value
- `parse_ref_`: Parse the argument value
- `parse`: Parse the argument value
- `parse_`: Parse the argument value
- `possible_values`: Reflect on enumerated value properties
- `map`: Adapt a `TypedValueParser` from one value to another
- `try_map`: Adapt a `TypedValueParser` from one value to another



## clap_builder::builder::value_parser::UnknownArgumentValueParser

*Struct*

When encountered, report [`ErrorKind::UnknownArgument`][crate::error::ErrorKind::UnknownArgument]

Useful to help users migrate, either from old versions or similar tools.

# Examples

```rust
# use clap_builder as clap;
# use clap::Command;
# use clap::Arg;
let cmd = Command::new("mycmd")
    .args([
        Arg::new("current-dir")
            .short('C'),
        Arg::new("current-dir-unknown")
            .long("cwd")
            .aliases(["current-dir", "directory", "working-directory", "root"])
            .value_parser(clap::builder::UnknownArgumentValueParser::suggest_arg("-C"))
            .hide(true),
    ]);

// Use a supported version of the argument
let matches = cmd.clone().try_get_matches_from(["mycmd", "-C", ".."]).unwrap();
assert!(matches.contains_id("current-dir"));
assert_eq!(
    matches.get_many::<String>("current-dir").unwrap_or_default().map(|v| v.as_str()).collect::<Vec<_>>(),
    vec![".."]
);

// Use one of the invalid versions
let err = cmd.try_get_matches_from(["mycmd", "--cwd", ".."]).unwrap_err();
assert_eq!(err.kind(), clap::error::ErrorKind::UnknownArgument);
```

**Methods:**

- `fn suggest_arg<impl Into<Str>>(arg: impl Trait) -> Self` - Suggest an alternative argument
- `fn suggest<impl Into<StyledStr>>(text: impl Trait) -> Self` - Provide a general suggestion
- `fn and_suggest<impl Into<StyledStr>>(self: Self, text: impl Trait) -> Self` - Extend the suggestions

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **TypedValueParser**
  - `fn parse_ref(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, value: &std::ffi::OsStr) -> Result<<Self as >::Value, crate::Error>`
  - `fn parse_ref_(self: &Self, cmd: &crate::Command, arg: Option<&crate::Arg>, _value: &std::ffi::OsStr, source: ValueSource) -> Result<<Self as >::Value, crate::Error>`
- **Clone**
  - `fn clone(self: &Self) -> UnknownArgumentValueParser`



## clap_builder::builder::value_parser::ValueParser

*Struct*

Parse/validate argument values

Specified with [`Arg::value_parser`][crate::Arg::value_parser].

`ValueParser` defines how to convert a raw argument value into a validated and typed value for
use within an application.

See
- [`value_parser!`][crate::value_parser] for automatically selecting an implementation for a given type
- [`ValueParser::new`] for additional [`TypedValueParser`] that can be used

# Example

```rust
# use clap_builder as clap;
let mut cmd = clap::Command::new("raw")
    .arg(
        clap::Arg::new("color")
            .long("color")
            .value_parser(["always", "auto", "never"])
            .default_value("auto")
    )
    .arg(
        clap::Arg::new("hostname")
            .long("hostname")
            .value_parser(clap::builder::NonEmptyStringValueParser::new())
            .action(clap::ArgAction::Set)
            .required(true)
    )
    .arg(
        clap::Arg::new("port")
            .long("port")
            .value_parser(clap::value_parser!(u16).range(3000..))
            .action(clap::ArgAction::Set)
            .required(true)
    );

let m = cmd.try_get_matches_from_mut(
    ["cmd", "--hostname", "rust-lang.org", "--port", "3001"]
).unwrap();

let color: &String = m.get_one("color")
    .expect("default");
assert_eq!(color, "auto");

let hostname: &String = m.get_one("hostname")
    .expect("required");
assert_eq!(hostname, "rust-lang.org");

let port: u16 = *m.get_one("port")
    .expect("required");
assert_eq!(port, 3001);
```

**Tuple Struct**: `()`

**Methods:**

- `fn new<P>(other: P) -> Self` - Custom parser for argument values
- `fn bool() -> Self` - [`bool`] parser for argument values
- `fn string() -> Self` - [`String`] parser for argument values
- `fn os_string() -> Self` - [`OsString`][std::ffi::OsString] parser for argument values
- `fn path_buf() -> Self` - [`PathBuf`][std::path::PathBuf] parser for argument values
- `fn type_id(self: &Self) -> AnyValueId` - Describes the content of `AnyValue`
- `fn possible_values(self: &Self) -> Option<Box<dyn Iterator>>` - Reflect on enumerated value properties

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut std::fmt::Formatter) -> Result<(), std::fmt::Error>`
- **From**
  - `fn from(value: std::ops::RangeToInclusive<i64>) -> Self`
- **From**
  - `fn from(value: std::ops::Range<i64>) -> Self`
- **From**
  - `fn from(values: Vec<P>) -> Self`
- **From**
  - `fn from(value: std::ops::RangeTo<i64>) -> Self`
- **From**
  - `fn from(p: P) -> Self`
- **From**
  - `fn from(values: [P; C]) -> Self`
- **From**
  - `fn from(value: std::ops::RangeFrom<i64>) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **From**
  - `fn from(value: std::ops::RangeFull) -> Self`
- **From**
  - `fn from(value: std::ops::RangeInclusive<i64>) -> Self`



## clap_builder::builder::value_parser::ValueParserFactory

*Trait*

Register a type with [`value_parser!`][crate::value_parser!]

# Example

```rust
# use clap_builder as clap;
#[derive(Copy, Clone, Debug)]
pub struct Custom(u32);

impl clap::builder::ValueParserFactory for Custom {
    type Parser = CustomValueParser;
    fn value_parser() -> Self::Parser {
        CustomValueParser
    }
}

#[derive(Clone, Debug)]
pub struct CustomValueParser;
impl clap::builder::TypedValueParser for CustomValueParser {
    type Value = Custom;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let inner = clap::value_parser!(u32);
        let val = inner.parse_ref(cmd, arg, value)?;
        Ok(Custom(val))
    }
}

let parser: CustomValueParser = clap::value_parser!(Custom);
```

**Methods:**

- `Parser`: Generated parser, usually [`ValueParser`].
- `value_parser`: Create the specified [`Self::Parser`]



