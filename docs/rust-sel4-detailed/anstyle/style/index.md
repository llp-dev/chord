*[anstyle](../index.md) / [style](index.md)*

---

# Module `style`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Style`](#style) | struct | ANSI Text styling |
| [`StyleDisplay`](#styledisplay) | struct |  |

## Structs

### `Style`

```rust
struct Style {
    fg: Option<crate::Color>,
    bg: Option<crate::Color>,
    underline: Option<crate::Color>,
    effects: crate::Effects,
}
```

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

#### Implementations

- <span id="style-new"></span>`const fn new() -> Self`

  No effects enabled

  

  # Examples

  

  ```rust

  let style = anstyle::Style::new();

  ```

- <span id="style-fg-color"></span>`const fn fg_color(self, fg: Option<crate::Color>) -> Self` — [`Color`](../index.md#color)

  Set foreground color

  

  # Examples

  

  ```rust

  let style = anstyle::Style::new().fg_color(Some(anstyle::AnsiColor::Red.into()));

  ```

- <span id="style-bg-color"></span>`const fn bg_color(self, bg: Option<crate::Color>) -> Self` — [`Color`](../index.md#color)

  Set background color

  

  # Examples

  

  ```rust

  let style = anstyle::Style::new().bg_color(Some(anstyle::AnsiColor::Red.into()));

  ```

- <span id="style-underline-color"></span>`const fn underline_color(self, underline: Option<crate::Color>) -> Self` — [`Color`](../index.md#color)

  Set underline color

  

  # Examples

  

  ```rust

  let style = anstyle::Style::new().underline_color(Some(anstyle::AnsiColor::Red.into()));

  ```

- <span id="style-effects"></span>`const fn effects(self, effects: crate::Effects) -> Self` — [`Effects`](../index.md#effects)

  Set text effects

  

  # Examples

  

  ```rust

  let style = anstyle::Style::new().effects(anstyle::Effects::BOLD | anstyle::Effects::UNDERLINE);

  ```

- <span id="style-render"></span>`fn render(self) -> impl core::fmt::Display + Copy`

  Render the ANSI code

  

  `Style` also implements `Display` directly, so calling this method is optional.

- <span id="style-fmt-to"></span>`fn fmt_to(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

- <span id="style-write-to"></span>`fn write_to(self, write: &mut dyn std::io::Write) -> std::io::Result<()>`

  Write the ANSI code

- <span id="style-render-reset"></span>`fn render_reset(self) -> impl core::fmt::Display + Copy`

  Renders the relevant `Reset` code

  

  Unlike `Reset::render`, this will elide the code if there is nothing to reset.

- <span id="style-write-reset-to"></span>`fn write_reset_to(self, write: &mut dyn std::io::Write) -> std::io::Result<()>`

  Write the relevant `Reset` code

  

  Unlike `Reset::render`, this will elide the code if there is nothing to reset.

#### Trait Implementations

##### `impl BitOr for Style`

- <span id="style-bitor-type-output"></span>`type Output = Style`

- <span id="style-bitor"></span>`fn bitor(self, rhs: crate::Effects) -> Self` — [`Effects`](../index.md#effects)

##### `impl BitOrAssign for Style`

- <span id="style-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: crate::Effects)` — [`Effects`](../index.md#effects)

##### `impl Clone for Style`

- <span id="style-clone"></span>`fn clone(&self) -> Style` — [`Style`](../index.md#style)

##### `impl Copy for Style`

##### `impl Debug for Style`

- <span id="style-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Style`

- <span id="style-default"></span>`fn default() -> Style` — [`Style`](../index.md#style)

##### `impl Display for Style`

- <span id="style-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for Style`

##### `impl Hash for Style`

- <span id="style-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Style`

- <span id="style-ord-cmp"></span>`fn cmp(&self, other: &Style) -> cmp::Ordering` — [`Style`](../index.md#style)

##### `impl PartialEq for Style`

- <span id="style-partialeq-eq"></span>`fn eq(&self, other: &Style) -> bool` — [`Style`](../index.md#style)

##### `impl PartialOrd for Style`

- <span id="style-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Style) -> option::Option<cmp::Ordering>` — [`Style`](../index.md#style)

##### `impl StructuralPartialEq for Style`

##### `impl Sub for Style`

- <span id="style-sub-type-output"></span>`type Output = Style`

- <span id="style-sub"></span>`fn sub(self, other: crate::Effects) -> Self` — [`Effects`](../index.md#effects)

##### `impl SubAssign for Style`

- <span id="style-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: crate::Effects)` — [`Effects`](../index.md#effects)

##### `impl ToString for Style`

- <span id="style-tostring-to-string"></span>`fn to_string(&self) -> String`

### `StyleDisplay`

```rust
struct StyleDisplay(Style);
```

#### Trait Implementations

##### `impl Clone for StyleDisplay`

- <span id="styledisplay-clone"></span>`fn clone(&self) -> StyleDisplay` — [`StyleDisplay`](#styledisplay)

##### `impl Copy for StyleDisplay`

##### `impl Debug for StyleDisplay`

- <span id="styledisplay-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for StyleDisplay`

- <span id="styledisplay-default"></span>`fn default() -> StyleDisplay` — [`StyleDisplay`](#styledisplay)

##### `impl Display for StyleDisplay`

- <span id="styledisplay-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl ToString for StyleDisplay`

- <span id="styledisplay-tostring-to-string"></span>`fn to_string(&self) -> String`

