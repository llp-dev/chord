**clap_builder > builder > action**

# Module: builder::action

## Contents

**Enums**

- [`ArgAction`](#argaction) - Behavior of arguments when they are encountered while parsing

---

## clap_builder::builder::action::ArgAction

*Enum*

Behavior of arguments when they are encountered while parsing

# Examples

```rust
# #[cfg(feature = "help")] {
# use clap_builder as clap;
# use clap::Command;
# use clap::Arg;
let cmd = Command::new("mycmd")
    .arg(
        Arg::new("special-help")
            .short('?')
            .action(clap::ArgAction::Help)
    );

// Existing help still exists
let err = cmd.clone().try_get_matches_from(["mycmd", "-h"]).unwrap_err();
assert_eq!(err.kind(), clap::error::ErrorKind::DisplayHelp);

// New help available
let err = cmd.try_get_matches_from(["mycmd", "-?"]).unwrap_err();
assert_eq!(err.kind(), clap::error::ErrorKind::DisplayHelp);
# }
```

**Variants:**
- `Set` - When encountered, store the associated value(s) in [`ArgMatches`][crate::ArgMatches]
- `Append` - When encountered, store the associated value(s) in [`ArgMatches`][crate::ArgMatches]
- `SetTrue` - When encountered, act as if `"true"` was encountered on the command-line
- `SetFalse` - When encountered, act as if `"false"` was encountered on the command-line
- `Count` - When encountered, increment a `u8` counter starting from `0`.
- `Help` - When encountered, display [`Command::print_help`][super::Command::print_help]
- `HelpShort` - When encountered, display [`Command::print_help`][super::Command::print_help]
- `HelpLong` - When encountered, display [`Command::print_long_help`][super::Command::print_long_help]
- `Version` - When encountered, display [`Command::version`][super::Command::version]

**Methods:**

- `fn takes_values(self: &Self) -> bool` - Returns whether this action accepts values on the command-line

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ArgAction`
- **IntoResettable**
  - `fn into_resettable(self: Self) -> Resettable<ArgAction>`



