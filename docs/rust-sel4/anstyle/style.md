**anstyle > style**

# Module: style

## Contents

**Structs**

- [`Style`](#style) - ANSI Text styling

---

## anstyle::style::Style

*Struct*

ANSI Text styling

You can print a `Style` to render the corresponding ANSI code.
Using the alternate flag `#` will render the ANSI reset code, if needed.
Together, this makes it convenient to render styles using inline format arguments.

# Examples

```rust
let style = anstyle::Style::new().bold();

let value = 42;
println!("{style}{value}{style:#}");
```

**Methods:**

- `fn get_fg_color(self: Self) -> Option<crate::Color>` - Get the foreground color
- `fn get_bg_color(self: Self) -> Option<crate::Color>` - Get the background color
- `fn get_underline_color(self: Self) -> Option<crate::Color>`
- `fn get_effects(self: Self) -> crate::Effects`
- `fn is_plain(self: Self) -> bool` - Check if no styling is enabled
- `fn bold(self: Self) -> Self` - Apply `bold` effect
- `fn dimmed(self: Self) -> Self` - Apply `dimmed` effect
- `fn italic(self: Self) -> Self` - Apply `italic` effect
- `fn underline(self: Self) -> Self` - Apply `underline` effect
- `fn blink(self: Self) -> Self` - Apply `blink` effect
- `fn invert(self: Self) -> Self` - Apply `invert` effect
- `fn hidden(self: Self) -> Self` - Apply `hidden` effect
- `fn strikethrough(self: Self) -> Self` - Apply `strikethrough` effect
- `fn new() -> Self` - No effects enabled
- `fn fg_color(self: Self, fg: Option<crate::Color>) -> Self` - Set foreground color
- `fn bg_color(self: Self, bg: Option<crate::Color>) -> Self` - Set background color
- `fn underline_color(self: Self, underline: Option<crate::Color>) -> Self` - Set underline color
- `fn effects(self: Self, effects: crate::Effects) -> Self` - Set text effects
- `fn render(self: Self) -> impl Trait` - Render the ANSI code
- `fn write_to(self: Self, write: & mut dyn std::io::Write) -> std::io::Result<()>` - Write the ANSI code
- `fn render_reset(self: Self) -> impl Trait` - Renders the relevant [`Reset`][crate::Reset] code
- `fn write_reset_to(self: Self, write: & mut dyn std::io::Write) -> std::io::Result<()>` - Write the relevant [`Reset`][crate::Reset] code

**Traits:** Eq, Copy

**Trait Implementations:**

- **BitOr**
  - `fn bitor(self: Self, rhs: crate::Effects) -> Self`
- **Ord**
  - `fn cmp(self: &Self, other: &Style) -> $crate::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Style) -> $crate::option::Option<$crate::cmp::Ordering>`
- **From**
  - `fn from(effects: crate::Effects) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Style) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Style`
- **Clone**
  - `fn clone(self: &Self) -> Style`
- **PartialEq**
  - `fn eq(self: &Self, other: &crate::Effects) -> bool`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: crate::Effects)`
- **Sub**
  - `fn sub(self: Self, other: crate::Effects) -> Self`
- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: crate::Effects)`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



