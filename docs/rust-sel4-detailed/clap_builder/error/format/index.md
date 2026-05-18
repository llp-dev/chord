*[clap_builder](../../index.md) / [error](../index.md) / [format](index.md)*

---

# Module `format`

## Contents

- [Structs](#structs)
  - [`KindFormatter`](#kindformatter)
  - [`RichFormatter`](#richformatter)
- [Traits](#traits)
  - [`ErrorFormatter`](#errorformatter)
- [Functions](#functions)
  - [`start_error`](#start-error)
  - [`write_dynamic_context`](#write-dynamic-context)
  - [`write_values_list`](#write-values-list)
  - [`format_error_message`](#format-error-message)
  - [`singular_or_plural`](#singular-or-plural)
  - [`put_usage`](#put-usage)
  - [`get_help_flag`](#get-help-flag)
  - [`get_user_help_flag`](#get-user-help-flag)
  - [`try_help`](#try-help)
  - [`did_you_mean`](#did-you-mean)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`KindFormatter`](#kindformatter) | struct | Report [`ErrorKind`] |
| [`RichFormatter`](#richformatter) | struct | Richly formatted error context |
| [`ErrorFormatter`](#errorformatter) | trait | Defines how to format an error for displaying to the user |
| [`start_error`](#start-error) | fn |  |
| [`write_dynamic_context`](#write-dynamic-context) | fn |  |
| [`write_values_list`](#write-values-list) | fn |  |
| [`format_error_message`](#format-error-message) | fn |  |
| [`singular_or_plural`](#singular-or-plural) | fn | Returns the singular or plural form on the verb to be based on the argument's value. |
| [`put_usage`](#put-usage) | fn |  |
| [`get_help_flag`](#get-help-flag) | fn |  |
| [`get_user_help_flag`](#get-user-help-flag) | fn |  |
| [`try_help`](#try-help) | fn |  |
| [`did_you_mean`](#did-you-mean) | fn |  |

## Structs

### `KindFormatter`

```rust
struct KindFormatter;
```

Report [`ErrorKind`](../kind/index.md)

No context is included.

<div class="warning">

**NOTE:** Consider removing the `error-context` default feature if using this to remove all
overhead for [`RichFormatter`](#richformatter).

</div>

#### Trait Implementations

##### `impl ErrorFormatter for KindFormatter`

- <span id="kindformatter-errorformatter-format-error"></span>`fn format_error(error: &crate::error::Error<Self>) -> StyledStr` — [`Error`](../index.md#error), [`StyledStr`](../../builder/styled_str/index.md#styledstr)

### `RichFormatter`

```rust
struct RichFormatter;
```

Richly formatted error context

This follows the [rustc diagnostic style guide](https://rustc-dev-guide.rust-lang.org/diagnostics.html#suggestion-style-guide).

#### Trait Implementations

##### `impl ErrorFormatter for RichFormatter`

- <span id="richformatter-errorformatter-format-error"></span>`fn format_error(error: &crate::error::Error<Self>) -> StyledStr` — [`Error`](../index.md#error), [`StyledStr`](../../builder/styled_str/index.md#styledstr)

## Traits

### `ErrorFormatter`

```rust
trait ErrorFormatter: Sized { ... }
```

Defines how to format an error for displaying to the user

#### Required Methods

- `fn format_error(error: &crate::error::Error<Self>) -> StyledStr`

  Stylize the error for the terminal

#### Implementors

- [`KindFormatter`](#kindformatter)
- [`RichFormatter`](#richformatter)

## Functions

### `start_error`

```rust
fn start_error(styled: &mut crate::builder::StyledStr, styles: &crate::builder::Styles)
```

### `write_dynamic_context`

```rust
fn write_dynamic_context(error: &crate::error::Error, styled: &mut crate::builder::StyledStr, styles: &crate::builder::Styles) -> bool
```

### `write_values_list`

```rust
fn write_values_list(list_name: &'static str, styled: &mut crate::builder::StyledStr, valid: &anstyle::Style, possible_values: Option<&crate::error::ContextValue>)
```

### `format_error_message`

```rust
fn format_error_message(message: &str, styles: &crate::builder::Styles, cmd: Option<&crate::builder::Command>, usage: Option<&crate::builder::StyledStr>) -> crate::builder::StyledStr
```

### `singular_or_plural`

```rust
fn singular_or_plural(n: usize) -> &'static str
```

Returns the singular or plural form on the verb to be based on the argument's value.

### `put_usage`

```rust
fn put_usage(styled: &mut crate::builder::StyledStr, usage: &crate::builder::StyledStr)
```

### `get_help_flag`

```rust
fn get_help_flag(cmd: &crate::builder::Command) -> Option<std::borrow::Cow<'static, str>>
```

### `get_user_help_flag`

```rust
fn get_user_help_flag(cmd: &crate::builder::Command) -> Option<String>
```

### `try_help`

```rust
fn try_help(styled: &mut crate::builder::StyledStr, styles: &crate::builder::Styles, help: Option<&str>)
```

### `did_you_mean`

```rust
fn did_you_mean(styled: &mut crate::builder::StyledStr, styles: &crate::builder::Styles, context: &str, possibles: &crate::error::ContextValue)
```

