**anstyle_query**

# Module: anstyle_query

## Contents

**Modules**

- [`windows`](#windows) - Windows-specific style queries

**Functions**

- [`clicolor`](#clicolor) - Check [CLICOLOR] status
- [`clicolor_force`](#clicolor_force) - Check [CLICOLOR_FORCE] status
- [`is_ci`](#is_ci) - Report whether this is running in CI
- [`no_color`](#no_color) - Check [NO_COLOR] status
- [`term_supports_ansi_color`](#term_supports_ansi_color) - Check `TERM` for ANSI color support
- [`term_supports_color`](#term_supports_color) - Check `TERM` for color support
- [`truecolor`](#truecolor) - Check [COLORTERM] for truecolor support

---

## anstyle_query::clicolor

*Function*

Check [CLICOLOR] status

- When `true`, ANSI colors are supported and should be used when the program isn't piped,
  similar to [`term_supports_color`]
- When `false`, don’t output ANSI color escape codes, similar to [`no_color`]

See also:
- [terminfo](https://crates.io/crates/terminfo) or [term](https://crates.io/crates/term) for
  checking termcaps
- [termbg](https://crates.io/crates/termbg) for detecting background color

[CLICOLOR]: https://bixense.com/clicolors/

```rust
fn clicolor() -> Option<bool>
```



## anstyle_query::clicolor_force

*Function*

Check [CLICOLOR_FORCE] status

ANSI colors should be enabled no matter what.

[CLICOLOR_FORCE]: https://bixense.com/clicolors/

```rust
fn clicolor_force() -> bool
```



## anstyle_query::is_ci

*Function*

Report whether this is running in CI

CI is a common environment where, despite being piped, ansi color codes are supported

This is not as exhaustive as you'd find in a crate like `is_ci` but it should work in enough
cases.

```rust
fn is_ci() -> bool
```



## anstyle_query::no_color

*Function*

Check [NO_COLOR] status

When `true`, should prevent the addition of ANSI color.

User-level configuration files and per-instance command-line arguments should override
[NO_COLOR]. A user should be able to export `$NO_COLOR` in their shell configuration file as a
default, but configure a specific program in its configuration file to specifically enable
color.

[NO_COLOR]: https://no-color.org/

```rust
fn no_color() -> bool
```



## anstyle_query::term_supports_ansi_color

*Function*

Check `TERM` for ANSI color support

On Windows, you might need to also check [`windows::enable_ansi_colors`] as ANSI color support
is opt-in, rather than assumed.

```rust
fn term_supports_ansi_color() -> bool
```



## anstyle_query::term_supports_color

*Function*

Check `TERM` for color support

```rust
fn term_supports_color() -> bool
```



## anstyle_query::truecolor

*Function*

Check [COLORTERM] for truecolor support

[COLORTERM]: https://github.com/termstandard/colors

```rust
fn truecolor() -> bool
```



## Module: windows

Windows-specific style queries



