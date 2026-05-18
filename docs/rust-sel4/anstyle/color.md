**anstyle > color**

# Module: color

## Contents

**Structs**

- [`Ansi256Color`](#ansi256color) - 256 (8-bit) color support
- [`RgbColor`](#rgbcolor) - 24-bit ANSI RGB color codes

**Enums**

- [`AnsiColor`](#ansicolor) - Available 4-bit ANSI color palette codes
- [`Color`](#color) - Any ANSI color code scheme

---

## anstyle::color::Ansi256Color

*Struct*

256 (8-bit) color support

- `0..16` are [`AnsiColor`] palette codes
- `0..232` map to [`RgbColor`] color values
- `232..` map to [`RgbColor`] gray-scale values

**Tuple Struct**: `(u8)`

**Methods:**

- `fn on<impl Into<Color>>(self: Self, background: impl Trait) -> crate::Style` - Create a [`Style`][crate::Style] with this as the foreground
- `fn on_default(self: Self) -> crate::Style` - Create a [`Style`][crate::Style] with this as the foreground
- `fn index(self: Self) -> u8` - Get the raw value
- `fn into_ansi(self: Self) -> Option<AnsiColor>` - Convert to [`AnsiColor`] when there is a 1:1 mapping
- `fn from_ansi(color: AnsiColor) -> Self` - Losslessly convert from [`AnsiColor`]
- `fn render_fg(self: Self) -> impl Trait` - Render the ANSI code for a foreground color
- `fn render_bg(self: Self) -> impl Trait` - Render the ANSI code for a background color

**Traits:** Eq, Copy

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Ord**
  - `fn cmp(self: &Self, other: &Ansi256Color) -> $crate::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Ansi256Color) -> $crate::option::Option<$crate::cmp::Ordering>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Ansi256Color) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Ansi256Color`
- **From**
  - `fn from(inner: AnsiColor) -> Self`
- **From**
  - `fn from(inner: u8) -> Self`



## anstyle::color::AnsiColor

*Enum*

Available 4-bit ANSI color palette codes

The user's terminal defines the meaning of each palette code.

**Variants:**
- `Black` - Black: #0 (foreground code `30`, background code `40`).
- `Red` - Red: #1 (foreground code `31`, background code `41`).
- `Green` - Green: #2 (foreground code `32`, background code `42`).
- `Yellow` - Yellow: #3 (foreground code `33`, background code `43`).
- `Blue` - Blue: #4 (foreground code `34`, background code `44`).
- `Magenta` - Magenta: #5 (foreground code `35`, background code `45`).
- `Cyan` - Cyan: #6 (foreground code `36`, background code `46`).
- `White` - White: #7 (foreground code `37`, background code `47`).
- `BrightBlack` - Bright black: #0 (foreground code `90`, background code `100`).
- `BrightRed` - Bright red: #1 (foreground code `91`, background code `101`).
- `BrightGreen` - Bright green: #2 (foreground code `92`, background code `102`).
- `BrightYellow` - Bright yellow: #3 (foreground code `93`, background code `103`).
- `BrightBlue` - Bright blue: #4 (foreground code `94`, background code `104`).
- `BrightMagenta` - Bright magenta: #5 (foreground code `95`, background code `105`).
- `BrightCyan` - Bright cyan: #6 (foreground code `96`, background code `106`).
- `BrightWhite` - Bright white: #7 (foreground code `97`, background code `107`).

**Methods:**

- `fn on<impl Into<Color>>(self: Self, background: impl Trait) -> crate::Style` - Create a [`Style`][crate::Style] with this as the foreground
- `fn on_default(self: Self) -> crate::Style` - Create a [`Style`][crate::Style] with this as the foreground
- `fn render_fg(self: Self) -> impl Trait` - Render the ANSI code for a foreground color
- `fn render_bg(self: Self) -> impl Trait` - Render the ANSI code for a background color
- `fn bright(self: Self, yes: bool) -> Self` - Change the color to/from bright
- `fn is_bright(self: Self) -> bool` - Report whether the color is bright

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> AnsiColor`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Ord**
  - `fn cmp(self: &Self, other: &AnsiColor) -> $crate::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &AnsiColor) -> $crate::option::Option<$crate::cmp::Ordering>`
- **PartialEq**
  - `fn eq(self: &Self, other: &AnsiColor) -> bool`



## anstyle::color::Color

*Enum*

Any ANSI color code scheme

**Variants:**
- `Ansi(AnsiColor)` - Available 4-bit ANSI color palette codes
- `Ansi256(Ansi256Color)` - 256 (8-bit) color support
- `Rgb(RgbColor)` - 24-bit ANSI RGB color codes

**Methods:**

- `fn on<impl Into<Color>>(self: Self, background: impl Trait) -> crate::Style` - Create a [`Style`][crate::Style] with this as the foreground
- `fn on_default(self: Self) -> crate::Style` - Create a [`Style`][crate::Style] with this as the foreground
- `fn render_fg(self: Self) -> impl Trait` - Render the ANSI code for a foreground color
- `fn render_bg(self: Self) -> impl Trait` - Render the ANSI code for a background color

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(inner: Ansi256Color) -> Self`
- **From**
  - `fn from(inner: AnsiColor) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Color`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Ord**
  - `fn cmp(self: &Self, other: &Color) -> $crate::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Color) -> $crate::option::Option<$crate::cmp::Ordering>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Color) -> bool`
- **From**
  - `fn from(inner: (u8, u8, u8)) -> Self`
- **From**
  - `fn from(inner: u8) -> Self`
- **From**
  - `fn from(inner: RgbColor) -> Self`



## anstyle::color::RgbColor

*Struct*

24-bit ANSI RGB color codes

**Tuple Struct**: `(u8, u8, u8)`

**Methods:**

- `fn on<impl Into<Color>>(self: Self, background: impl Trait) -> crate::Style` - Create a [`Style`][crate::Style] with this as the foreground
- `fn on_default(self: Self) -> crate::Style` - Create a [`Style`][crate::Style] with this as the foreground
- `fn r(self: Self) -> u8` - Red
- `fn g(self: Self) -> u8` - Green
- `fn b(self: Self) -> u8` - Blue
- `fn render_fg(self: Self) -> impl Trait` - Render the ANSI code for a foreground color
- `fn render_bg(self: Self) -> impl Trait` - Render the ANSI code for a background color

**Traits:** Eq, Copy

**Trait Implementations:**

- **From**
  - `fn from(inner: (u8, u8, u8)) -> Self`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Ord**
  - `fn cmp(self: &Self, other: &RgbColor) -> $crate::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &RgbColor) -> $crate::option::Option<$crate::cmp::Ordering>`
- **PartialEq**
  - `fn eq(self: &Self, other: &RgbColor) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> RgbColor`



