**ttf_parser > tables > os2**

# Module: tables::os2

## Contents

**Structs**

- [`ScriptMetrics`](#scriptmetrics) - A script metrics used by subscript and superscript.
- [`SelectionFlags`](#selectionflags)
- [`Table`](#table) - A [OS/2 and Windows Metrics Table](https://docs.microsoft.com/en-us/typography/opentype/spec/os2).
- [`UnicodeRanges`](#unicoderanges) - [Unicode Ranges](https://docs.microsoft.com/en-us/typography/opentype/spec/os2#ur).

**Enums**

- [`Permissions`](#permissions) - Face [permissions](https://docs.microsoft.com/en-us/typography/opentype/spec/os2#fst).
- [`Style`](#style) - A face style.
- [`Weight`](#weight) - A face [weight](https://docs.microsoft.com/en-us/typography/opentype/spec/os2#usweightclass).
- [`Width`](#width) - A face [width](https://docs.microsoft.com/en-us/typography/opentype/spec/os2#uswidthclass).

**Functions**

- [`char_range_index`](#char_range_index)

**Constants**

- [`CAP_HEIGHT_OFFSET`](#cap_height_offset)
- [`SELECTION_OFFSET`](#selection_offset)
- [`TYPE_OFFSET`](#type_offset)
- [`TYPO_ASCENDER_OFFSET`](#typo_ascender_offset)
- [`TYPO_DESCENDER_OFFSET`](#typo_descender_offset)
- [`TYPO_LINE_GAP_OFFSET`](#typo_line_gap_offset)
- [`UNICODE_RANGES_OFFSET`](#unicode_ranges_offset)
- [`WEIGHT_CLASS_OFFSET`](#weight_class_offset)
- [`WIDTH_CLASS_OFFSET`](#width_class_offset)
- [`WIN_ASCENT`](#win_ascent)
- [`WIN_DESCENT`](#win_descent)
- [`X_HEIGHT_OFFSET`](#x_height_offset)
- [`Y_STRIKEOUT_POSITION_OFFSET`](#y_strikeout_position_offset)
- [`Y_STRIKEOUT_SIZE_OFFSET`](#y_strikeout_size_offset)
- [`Y_SUBSCRIPT_X_SIZE_OFFSET`](#y_subscript_x_size_offset)
- [`Y_SUPERSCRIPT_X_SIZE_OFFSET`](#y_superscript_x_size_offset)

---

## ttf_parser::tables::os2::CAP_HEIGHT_OFFSET

*Constant*: `usize`



## ttf_parser::tables::os2::Permissions

*Enum*

Face [permissions](https://docs.microsoft.com/en-us/typography/opentype/spec/os2#fst).

**Variants:**
- `Installable`
- `Restricted`
- `PreviewAndPrint`
- `Editable`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Permissions`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Permissions) -> $crate::option::Option<$crate::cmp::Ordering>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Permissions) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Ord**
  - `fn cmp(self: &Self, other: &Permissions) -> $crate::cmp::Ordering`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## ttf_parser::tables::os2::SELECTION_OFFSET

*Constant*: `usize`



## ttf_parser::tables::os2::ScriptMetrics

*Struct*

A script metrics used by subscript and superscript.

**Fields:**
- `x_size: i16` - Horizontal face size.
- `y_size: i16` - Vertical face size.
- `x_offset: i16` - X offset.
- `y_offset: i16` - Y offset.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &ScriptMetrics) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ScriptMetrics`



## ttf_parser::tables::os2::SelectionFlags

*Struct*

**Tuple Struct**: `(u16)`

**Methods:**

- `fn italic(self: Self) -> bool`
- `fn bold(self: Self) -> bool`
- `fn use_typo_metrics(self: Self) -> bool`
- `fn oblique(self: Self) -> bool`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> SelectionFlags`



## ttf_parser::tables::os2::Style

*Enum*

A face style.

**Variants:**
- `Normal` - A face that is neither italic not obliqued.
- `Italic` - A form that is generally cursive in nature.
- `Oblique` - A typically-sloped version of the regular face.

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Style) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Style`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Default**
  - `fn default() -> Style`



## ttf_parser::tables::os2::TYPE_OFFSET

*Constant*: `usize`



## ttf_parser::tables::os2::TYPO_ASCENDER_OFFSET

*Constant*: `usize`



## ttf_parser::tables::os2::TYPO_DESCENDER_OFFSET

*Constant*: `usize`



## ttf_parser::tables::os2::TYPO_LINE_GAP_OFFSET

*Constant*: `usize`



## ttf_parser::tables::os2::Table

*Struct*

A [OS/2 and Windows Metrics Table](https://docs.microsoft.com/en-us/typography/opentype/spec/os2).

**Generic Parameters:**
- 'a

**Fields:**
- `version: u8` - Table version.
- `data: &'a [u8]`

**Methods:**

- `fn parse(data: &'a [u8]) -> Option<Self>` - Parses a table from raw data.
- `fn weight(self: &Self) -> Weight` - Returns weight class.
- `fn width(self: &Self) -> Width` - Returns face width.
- `fn permissions(self: &Self) -> Option<Permissions>` - Returns face permissions.
- `fn is_subsetting_allowed(self: &Self) -> bool` - Checks if the face allows embedding a subset, further restricted by [`Self::permissions`].
- `fn is_outline_embedding_allowed(self: &Self) -> bool` - Checks if the face allows outline data to be embedded.
- `fn subscript_metrics(self: &Self) -> ScriptMetrics` - Returns subscript metrics.
- `fn superscript_metrics(self: &Self) -> ScriptMetrics` - Returns superscript metrics.
- `fn strikeout_metrics(self: &Self) -> LineMetrics` - Returns strikeout metrics.
- `fn unicode_ranges(self: &Self) -> UnicodeRanges` - Returns Unicode ranges.
- `fn fs_selection(self: &Self) -> u16`
- `fn style(self: &Self) -> Style` - Returns style.
- `fn is_bold(self: &Self) -> bool` - Checks if face is bold.
- `fn use_typographic_metrics(self: &Self) -> bool` - Checks if typographic metrics should be used.
- `fn typographic_ascender(self: &Self) -> i16` - Returns typographic ascender.
- `fn typographic_descender(self: &Self) -> i16` - Returns typographic descender.
- `fn typographic_line_gap(self: &Self) -> i16` - Returns typographic line gap.
- `fn windows_ascender(self: &Self) -> i16` - Returns Windows ascender.
- `fn windows_descender(self: &Self) -> i16` - Returns Windows descender.
- `fn x_height(self: &Self) -> Option<i16>` - Returns x height.
- `fn capital_height(self: &Self) -> Option<i16>` - Returns capital height.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Table<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



## ttf_parser::tables::os2::UNICODE_RANGES_OFFSET

*Constant*: `usize`



## ttf_parser::tables::os2::UnicodeRanges

*Struct*

[Unicode Ranges](https://docs.microsoft.com/en-us/typography/opentype/spec/os2#ur).

**Tuple Struct**: `(u128)`

**Methods:**

- `fn contains_char(self: &Self, c: char) -> bool` - Checks if ranges contain the specified character.

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> UnicodeRanges`
- **Default**
  - `fn default() -> UnicodeRanges`



## ttf_parser::tables::os2::WEIGHT_CLASS_OFFSET

*Constant*: `usize`



## ttf_parser::tables::os2::WIDTH_CLASS_OFFSET

*Constant*: `usize`



## ttf_parser::tables::os2::WIN_ASCENT

*Constant*: `usize`



## ttf_parser::tables::os2::WIN_DESCENT

*Constant*: `usize`



## ttf_parser::tables::os2::Weight

*Enum*

A face [weight](https://docs.microsoft.com/en-us/typography/opentype/spec/os2#usweightclass).

**Variants:**
- `Thin`
- `ExtraLight`
- `Light`
- `Normal`
- `Medium`
- `SemiBold`
- `Bold`
- `ExtraBold`
- `Black`
- `Other(u16)`

**Methods:**

- `fn to_number(self: Self) -> u16` - Returns a numeric representation of a weight.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **From**
  - `fn from(value: u16) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Weight`
- **Default**
  - `fn default() -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Weight) -> bool`



## ttf_parser::tables::os2::Width

*Enum*

A face [width](https://docs.microsoft.com/en-us/typography/opentype/spec/os2#uswidthclass).

**Variants:**
- `UltraCondensed`
- `ExtraCondensed`
- `Condensed`
- `SemiCondensed`
- `Normal`
- `SemiExpanded`
- `Expanded`
- `ExtraExpanded`
- `UltraExpanded`

**Methods:**

- `fn to_number(self: Self) -> u16` - Returns a numeric representation of a width.

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Width) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Ord**
  - `fn cmp(self: &Self, other: &Width) -> $crate::cmp::Ordering`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> Width`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Width) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Default**
  - `fn default() -> Self`



## ttf_parser::tables::os2::X_HEIGHT_OFFSET

*Constant*: `usize`



## ttf_parser::tables::os2::Y_STRIKEOUT_POSITION_OFFSET

*Constant*: `usize`



## ttf_parser::tables::os2::Y_STRIKEOUT_SIZE_OFFSET

*Constant*: `usize`



## ttf_parser::tables::os2::Y_SUBSCRIPT_X_SIZE_OFFSET

*Constant*: `usize`



## ttf_parser::tables::os2::Y_SUPERSCRIPT_X_SIZE_OFFSET

*Constant*: `usize`



## ttf_parser::tables::os2::char_range_index

*Function*

```rust
fn char_range_index(c: char) -> i8
```



