*[clap_builder](../../index.md) / [builder](../index.md) / [styling](index.md)*

---

# Module `styling`

Terminal [`Styles`](#styles) for help and error output

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Styles`](#styles) | struct | Terminal styling definitions |

## Structs

### `Styles`

```rust
struct Styles {
    header: Style,
    error: Style,
    usage: Style,
    literal: Style,
    placeholder: Style,
    valid: Style,
    invalid: Style,
    context: Style,
    context_value: Option<Style>,
}
```

Terminal styling definitions

See also `Command::styles`.

# Example

clap v3 styling
```rust
use clap_builder as clap;
use clap::builder::styling::*;
let styles = Styles::styled()
    .header(AnsiColor::Yellow.on_default())
    .usage(AnsiColor::Green.on_default())
    .literal(AnsiColor::Green.on_default())
    .placeholder(AnsiColor::Green.on_default());
```

#### Implementations

- <span id="styles-plain"></span>`const fn plain() -> Self`

  No terminal styling

- <span id="styles-styled"></span>`const fn styled() -> Self`

  Default terminal styling

- <span id="styles-header"></span>`const fn header(self, style: Style) -> Self`

  General Heading style, e.g. `help_heading`

- <span id="styles-error"></span>`const fn error(self, style: Style) -> Self`

  Error heading

- <span id="styles-usage"></span>`const fn usage(self, style: Style) -> Self`

  Usage heading

- <span id="styles-literal"></span>`const fn literal(self, style: Style) -> Self`

  Literal command-line syntax, e.g. `--help`

- <span id="styles-placeholder"></span>`const fn placeholder(self, style: Style) -> Self`

  Descriptions within command-line syntax, e.g. `value_name`

- <span id="styles-valid"></span>`const fn valid(self, style: Style) -> Self`

  Highlight suggested usage

- <span id="styles-invalid"></span>`const fn invalid(self, style: Style) -> Self`

  Highlight invalid usage

- <span id="styles-context"></span>`const fn context(self, style: Style) -> Self`

  Highlight all specified contexts, e.g. `[default: false]`

  

  To specialize the style of the value within the context, see `Styles::context_value`

- <span id="styles-context-value"></span>`const fn context_value(self, style: Style) -> Self`

  Highlight values within all of the context, e.g. the `false` in `[default: false]`

  

  If not explicitly set, falls back to `context`'s style.

#### Trait Implementations

##### `impl AppExt for Styles`

##### `impl Clone for Styles`

- <span id="styles-clone"></span>`fn clone(&self) -> Styles` — [`Styles`](#styles)

##### `impl Debug for Styles`

- <span id="styles-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Styles`

- <span id="styles-default"></span>`fn default() -> Self`

