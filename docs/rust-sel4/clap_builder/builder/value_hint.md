**clap_builder > builder > value_hint**

# Module: builder::value_hint

## Contents

**Enums**

- [`ValueHint`](#valuehint) - Provide shell with hint on how to complete an argument.

---

## clap_builder::builder::value_hint::ValueHint

*Enum*

Provide shell with hint on how to complete an argument.

See [`Arg::value_hint`][crate::Arg::value_hint] to set this on an argument.

See the `clap_complete` crate for completion script generation.

Overview of which hints are supported by which shell:

| Hint                   | zsh | fish[^1] | dynamic |
| ---------------------- | --- | ---------|---------|
| `AnyPath`              | Yes | Yes      | Yes     |
| `FilePath`             | Yes | Yes      | Yes     |
| `DirPath`              | Yes | Yes      | Yes     |
| `ExecutablePath`       | Yes | Partial  | Yes     |
| `CommandName`          | Yes | Yes      | No      |
| `CommandString`        | Yes | Partial  | No      |
| `CommandWithArguments` | Yes |          | No      |
| `Username`             | Yes | Yes      | No      |
| `Hostname`             | Yes | Yes      | No      |
| `Url`                  | Yes |          | No      |
| `EmailAddress`         | Yes |          | No      |

[^1]: fish completions currently only support named arguments (e.g. -o or --opt), not
      positional arguments.

**Variants:**
- `Unknown` - Default value if hint is not specified. Follows shell default behavior, which is usually
- `Other` - None of the hints below apply. Disables shell completion for this argument.
- `AnyPath` - Any existing path.
- `FilePath` - Path to a file.
- `DirPath` - Path to a directory.
- `ExecutablePath` - Path to an executable file.
- `CommandName` - Name of a command, without arguments. May be relative to PATH, or full path to executable.
- `CommandString` - A single string containing a command and its arguments.
- `CommandWithArguments` - Capture the remaining arguments as a command name and arguments for that command. This is
- `Username` - Name of a local operating system user.
- `Hostname` - Host name of a computer.
- `Url` - Complete web address.
- `EmailAddress` - Email address.

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &ValueHint) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ValueHint`
- **IntoResettable**
  - `fn into_resettable(self: Self) -> Resettable<ValueHint>`
- **FromStr**
  - `fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err>`
- **Default**
  - `fn default() -> ValueHint`



