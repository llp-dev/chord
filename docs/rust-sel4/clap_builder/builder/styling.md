**clap_builder > builder > styling**

# Module: builder::styling

## Contents

**Structs**

- [`Styles`](#styles) - Terminal styling definitions

---

## clap_builder::builder::styling::Styles

*Struct*

Terminal styling definitions

See also [`Command::styles`][crate::Command::styles].

# Example

clap v3 styling
```rust
# use clap_builder as clap;
# use clap::builder::styling::*;
let styles = Styles::styled()
    .header(AnsiColor::Yellow.on_default())
    .usage(AnsiColor::Green.on_default())
    .literal(AnsiColor::Green.on_default())
    .placeholder(AnsiColor::Green.on_default());
```

**Methods:**

- `fn get_header(self: &Self) -> &Style` - General Heading style, e.g. [`help_heading`][crate::Arg::help_heading]
- `fn get_error(self: &Self) -> &Style` - Error heading
- `fn get_usage(self: &Self) -> &Style` - Usage heading
- `fn get_literal(self: &Self) -> &Style` - Literal command-line syntax, e.g. `--help`
- `fn get_placeholder(self: &Self) -> &Style` - Descriptions within command-line syntax, e.g. [`value_name`][crate::Arg::value_name]
- `fn get_valid(self: &Self) -> &Style` - Highlight suggested usage
- `fn get_invalid(self: &Self) -> &Style` - Highlight invalid usage
- `fn get_context(self: &Self) -> &Style` - Highlight all specified contexts, e.g. `[default: false]`
- `fn get_context_value(self: &Self) -> &Style` - Highlight values within all of the context, e.g. the `false` in `[default: false]`
- `fn plain() -> Self` - No terminal styling
- `fn styled() -> Self` - Default terminal styling
- `fn header(self: Self, style: Style) -> Self` - General Heading style, e.g. [`help_heading`][crate::Arg::help_heading]
- `fn error(self: Self, style: Style) -> Self` - Error heading
- `fn usage(self: Self, style: Style) -> Self` - Usage heading
- `fn literal(self: Self, style: Style) -> Self` - Literal command-line syntax, e.g. `--help`
- `fn placeholder(self: Self, style: Style) -> Self` - Descriptions within command-line syntax, e.g. [`value_name`][crate::Arg::value_name]
- `fn valid(self: Self, style: Style) -> Self` - Highlight suggested usage
- `fn invalid(self: Self, style: Style) -> Self` - Highlight invalid usage
- `fn context(self: Self, style: Style) -> Self` - Highlight all specified contexts, e.g. `[default: false]`
- `fn context_value(self: Self, style: Style) -> Self` - Highlight values within all of the context, e.g. the `false` in `[default: false]`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Styles`
- **Default**
  - `fn default() -> Self`



