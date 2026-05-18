*[clap_builder](../../index.md) / [output](../index.md) / [help_template](index.md)*

---

# Module `help_template`

## Contents

- [Structs](#structs)
  - [`AutoHelp`](#autohelp)
  - [`HelpTemplate`](#helptemplate)
- [Functions](#functions)
  - [`positional_sort_key`](#positional-sort-key)
  - [`option_sort_key`](#option-sort-key)
  - [`dimensions`](#dimensions)
  - [`should_show_arg`](#should-show-arg)
  - [`should_show_subcommand`](#should-show-subcommand)
- [Type Aliases](#type-aliases)
  - [`ArgSortKey`](#argsortkey)
- [Constants](#constants)
  - [`DEFAULT_TEMPLATE`](#default-template)
  - [`DEFAULT_NO_ARGS_TEMPLATE`](#default-no-args-template)
  - [`SHORT_SIZE`](#short-size)
  - [`NEXT_LINE_INDENT`](#next-line-indent)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`AutoHelp`](#autohelp) | struct | `clap` auto-generated help writer |
| [`HelpTemplate`](#helptemplate) | struct | Help template writer |
| [`positional_sort_key`](#positional-sort-key) | fn |  |
| [`option_sort_key`](#option-sort-key) | fn |  |
| [`dimensions`](#dimensions) | fn |  |
| [`should_show_arg`](#should-show-arg) | fn |  |
| [`should_show_subcommand`](#should-show-subcommand) | fn |  |
| [`ArgSortKey`](#argsortkey) | type |  |
| [`DEFAULT_TEMPLATE`](#default-template) | const |  |
| [`DEFAULT_NO_ARGS_TEMPLATE`](#default-no-args-template) | const |  |
| [`SHORT_SIZE`](#short-size) | const |  |
| [`NEXT_LINE_INDENT`](#next-line-indent) | const |  |

## Structs

### `AutoHelp<'cmd, 'writer>`

```rust
struct AutoHelp<'cmd, 'writer> {
    template: HelpTemplate<'cmd, 'writer>,
}
```

`clap` auto-generated help writer

#### Implementations

- <span id="autohelp-new"></span>`fn new(writer: &'writer mut StyledStr, cmd: &'cmd Command, usage: &'cmd Usage<'cmd>, use_long: bool) -> Self` — [`StyledStr`](../../builder/styled_str/index.md#styledstr), [`Command`](../../builder/command/index.md#command), [`Usage`](../usage/index.md#usage)

  Create a new `HelpTemplate` instance.

- <span id="autohelp-write-help"></span>`fn write_help(&mut self)`

### `HelpTemplate<'cmd, 'writer>`

```rust
struct HelpTemplate<'cmd, 'writer> {
    writer: &'writer mut crate::builder::StyledStr,
    cmd: &'cmd crate::builder::Command,
    styles: &'cmd crate::builder::Styles,
    usage: &'cmd self::usage::Usage<'cmd>,
    next_line_help: bool,
    term_w: usize,
    use_long: bool,
}
```

Help template writer

Wraps a writer stream providing different methods to generate help for `clap` objects.

#### Implementations

- <span id="helptemplate-new"></span>`fn new(writer: &'writer mut StyledStr, cmd: &'cmd Command, usage: &'cmd Usage<'cmd>, use_long: bool) -> Self` — [`StyledStr`](../../builder/styled_str/index.md#styledstr), [`Command`](../../builder/command/index.md#command), [`Usage`](../usage/index.md#usage)

  Create a new `HelpTemplate` instance.

- <span id="helptemplate-term-w"></span>`fn term_w(cmd: &'cmd Command) -> usize` — [`Command`](../../builder/command/index.md#command)

- <span id="helptemplate-write-templated-help"></span>`fn write_templated_help(&mut self, template: &str)`

  Write help to stream for the parser in the format defined by the template.

  

  For details about the template language see `Command::help_template`.

## Functions

### `positional_sort_key`

```rust
fn positional_sort_key(arg: &crate::builder::Arg) -> (usize, String)
```

### `option_sort_key`

```rust
fn option_sort_key(arg: &crate::builder::Arg) -> (usize, String)
```

### `dimensions`

```rust
fn dimensions() -> (Option<usize>, Option<usize>)
```

### `should_show_arg`

```rust
fn should_show_arg(use_long: bool, arg: &crate::builder::Arg) -> bool
```

### `should_show_subcommand`

```rust
fn should_show_subcommand(subcommand: &crate::builder::Command) -> bool
```

## Type Aliases

### `ArgSortKey`

```rust
type ArgSortKey = fn(&crate::builder::Arg) -> (usize, String);
```

## Constants

### `DEFAULT_TEMPLATE`
```rust
const DEFAULT_TEMPLATE: &str;
```

### `DEFAULT_NO_ARGS_TEMPLATE`
```rust
const DEFAULT_NO_ARGS_TEMPLATE: &str;
```

### `SHORT_SIZE`
```rust
const SHORT_SIZE: usize = 4usize;
```

### `NEXT_LINE_INDENT`
```rust
const NEXT_LINE_INDENT: &str;
```

