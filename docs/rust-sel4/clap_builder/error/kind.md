**clap_builder > error > kind**

# Module: error::kind

## Contents

**Enums**

- [`ErrorKind`](#errorkind) - Command line argument parser kind of error

---

## clap_builder::error::kind::ErrorKind

*Enum*

Command line argument parser kind of error

**Variants:**
- `InvalidValue` - Occurs when an [`Arg`][crate::Arg] has a set of possible values,
- `UnknownArgument` - Occurs when a user provides a flag, option, argument or subcommand which isn't defined.
- `InvalidSubcommand` - Occurs when the user provides an unrecognized [`Subcommand`] which meets the threshold for
- `NoEquals` - Occurs when the user doesn't use equals for an option that requires equal
- `ValueValidation` - Occurs when the user provides a value for an argument with a custom validation and the
- `TooManyValues` - Occurs when a user provides more values for an argument than were defined by setting
- `TooFewValues` - Occurs when the user provides fewer values for an argument than were defined by setting
- `WrongNumberOfValues` - Occurs when the user provides a different number of values for an argument than what's
- `ArgumentConflict` - Occurs when the user provides two values which conflict with each other and can't be used
- `MissingRequiredArgument` - Occurs when the user does not provide one or more required arguments.
- `MissingSubcommand` - Occurs when a subcommand is required (as defined by [`Command::subcommand_required`]),
- `InvalidUtf8` - Occurs when the user provides a value containing invalid UTF-8.
- `DisplayHelp` - Not a true "error" as it means `--help` or similar was used.
- `DisplayHelpOnMissingArgumentOrSubcommand` - Occurs when either an argument or a [`Subcommand`] is required, as defined by
- `DisplayVersion` - Not a true "error" as it means `--version` or similar was used.
- `Io` - Represents an [I/O error].
- `Format` - Represents a [Format error] (which is a part of [`Display`]).

**Methods:**

- `fn as_str(self: Self) -> Option<&'static str>` - End-user description of the error case, where relevant

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut std::fmt::Formatter) -> std::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ErrorKind`
- **PartialEq**
  - `fn eq(self: &Self, other: &ErrorKind) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



