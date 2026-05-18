**ttf_parser > tables > stat**

# Module: tables::stat

## Contents

**Structs**

- [`AxisRecord`](#axisrecord) - The [axis record](https://learn.microsoft.com/en-us/typography/opentype/spec/stat#axis-records) struct provides information about a single design axis.
- [`AxisValue`](#axisvalue) - Axis-value pairing for [`AxisValueSubtableFormat4`].
- [`AxisValueFlags`](#axisvalueflags) - [Flags](https://learn.microsoft.com/en-us/typography/opentype/spec/stat#flags) for [`AxisValueSubtable`].
- [`AxisValueSubtableFormat1`](#axisvaluesubtableformat1) - Axis value subtable [format 1](https://learn.microsoft.com/en-us/typography/opentype/spec/stat#axis-value-table-format-1).
- [`AxisValueSubtableFormat2`](#axisvaluesubtableformat2) - Axis value subtable [format 2](https://learn.microsoft.com/en-us/typography/opentype/spec/stat#axis-value-table-format-2).
- [`AxisValueSubtableFormat3`](#axisvaluesubtableformat3) - Axis value subtable [format 3](https://learn.microsoft.com/en-us/typography/opentype/spec/stat#axis-value-table-format-3).
- [`AxisValueSubtableFormat4`](#axisvaluesubtableformat4) - Axis value subtable [format 4](https://learn.microsoft.com/en-us/typography/opentype/spec/stat#axis-value-table-format-4).
- [`AxisValueSubtables`](#axisvaluesubtables) - Iterator over axis value subtables.
- [`Table`](#table) - A [Style Attributes Table](https://docs.microsoft.com/en-us/typography/opentype/spec/stat).

**Enums**

- [`AxisValueSubtable`](#axisvaluesubtable) - An [axis value subtable](https://learn.microsoft.com/en-us/typography/opentype/spec/stat#axis-value-tables).

---

## ttf_parser::tables::stat::AxisRecord

*Struct*

The [axis record](https://learn.microsoft.com/en-us/typography/opentype/spec/stat#axis-records) struct provides information about a single design axis.

**Fields:**
- `tag: crate::Tag` - Axis tag.
- `name_id: u16` - The name ID for entries in the 'name' table that provide a display string for this axis.
- `ordering: u16` - Sort order for e.g. composing font family or face names.

**Traits:** Copy

**Trait Implementations:**

- **FromData**
  - `fn parse(data: &[u8]) -> Option<Self>`
- **Clone**
  - `fn clone(self: &Self) -> AxisRecord`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ttf_parser::tables::stat::AxisValue

*Struct*

Axis-value pairing for [`AxisValueSubtableFormat4`].

**Fields:**
- `axis_index: u16` - Zero-based index into [`Table::axes`].
- `value: crate::Fixed` - Numeric value for this axis.

**Traits:** Copy

**Trait Implementations:**

- **FromData**
  - `fn parse(data: &[u8]) -> Option<Self>`
- **Clone**
  - `fn clone(self: &Self) -> AxisValue`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ttf_parser::tables::stat::AxisValueFlags

*Struct*

[Flags](https://learn.microsoft.com/en-us/typography/opentype/spec/stat#flags) for [`AxisValueSubtable`].

**Tuple Struct**: `(u16)`

**Methods:**

- `fn older_sibling_attribute(self: Self) -> bool` - If set, this value also applies to older versions of this font.
- `fn elidable(self: Self) -> bool` - If set, this value is the normal (a.k.a. "regular") value for the font family.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> AxisValueFlags`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



## ttf_parser::tables::stat::AxisValueSubtable

*Enum*

An [axis value subtable](https://learn.microsoft.com/en-us/typography/opentype/spec/stat#axis-value-tables).

**Generic Parameters:**
- 'a

**Variants:**
- `Format1(AxisValueSubtableFormat1)`
- `Format2(AxisValueSubtableFormat2)`
- `Format3(AxisValueSubtableFormat3)`
- `Format4(AxisValueSubtableFormat4<'a>)`

**Methods:**

- `fn value(self: &Self) -> Option<Fixed>` - Returns the value from an axis value subtable.
- `fn contains(self: &Self, value: Fixed) -> bool` - Returns `true` if the axis subtable either is the value or is a range that contains the
- `fn name_id(self: &Self) -> u16` - Returns the associated name ID.
- `fn flags(self: &Self) -> AxisValueFlags`
- `fn is_elidable(self: &Self) -> bool` - Returns `true` if the axis subtable has the `ELIDABLE_AXIS_VALUE_NAME` flag set.
- `fn is_older_sibling(self: &Self) -> bool` - Returns `true` if the axis subtable has the `OLDER_SIBLING_FONT_ATTRIBUTE` flag set.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> AxisValueSubtable<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ttf_parser::tables::stat::AxisValueSubtableFormat1

*Struct*

Axis value subtable [format 1](https://learn.microsoft.com/en-us/typography/opentype/spec/stat#axis-value-table-format-1).

**Fields:**
- `axis_index: u16` - Zero-based index into [`Table::axes`].
- `flags: AxisValueFlags` - Flags for [`AxisValueSubtable`].
- `value_name_id: u16` - The name ID of the display string.
- `value: crate::Fixed` - Numeric value for this record.

**Traits:** Copy

**Trait Implementations:**

- **FromData**
  - `fn parse(data: &[u8]) -> Option<Self>`
- **Clone**
  - `fn clone(self: &Self) -> AxisValueSubtableFormat1`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ttf_parser::tables::stat::AxisValueSubtableFormat2

*Struct*

Axis value subtable [format 2](https://learn.microsoft.com/en-us/typography/opentype/spec/stat#axis-value-table-format-2).

**Fields:**
- `axis_index: u16` - Zero-based index into [`Table::axes`].
- `flags: AxisValueFlags` - Flags for [`AxisValueSubtable`].
- `value_name_id: u16` - The name ID of the display string.
- `nominal_value: crate::Fixed` - Nominal numeric value for this record.
- `range_min_value: crate::Fixed` - The minimum value for this record.
- `range_max_value: crate::Fixed` - The maximum value for this record.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> AxisValueSubtableFormat2`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **FromData**
  - `fn parse(data: &[u8]) -> Option<Self>`



## ttf_parser::tables::stat::AxisValueSubtableFormat3

*Struct*

Axis value subtable [format 3](https://learn.microsoft.com/en-us/typography/opentype/spec/stat#axis-value-table-format-3).

**Fields:**
- `axis_index: u16` - Zero-based index into [`Table::axes`].
- `flags: AxisValueFlags` - Flags for [`AxisValueSubtable`].
- `value_name_id: u16` - The name ID of the display string.
- `value: crate::Fixed` - Numeric value for this record.
- `linked_value: crate::Fixed` - Numeric value for a style-linked mapping.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> AxisValueSubtableFormat3`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **FromData**
  - `fn parse(data: &[u8]) -> Option<Self>`



## ttf_parser::tables::stat::AxisValueSubtableFormat4

*Struct*

Axis value subtable [format 4](https://learn.microsoft.com/en-us/typography/opentype/spec/stat#axis-value-table-format-4).

**Generic Parameters:**
- 'a

**Fields:**
- `flags: AxisValueFlags` - Flags for [`AxisValueSubtable`].
- `value_name_id: u16` - The name ID of the display string.
- `values: crate::LazyArray16<'a, AxisValue>` - List of axis-value pairings.

**Methods:**

- `fn parse(data: &'a [u8]) -> Option<Self>`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> AxisValueSubtableFormat4<'a>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## ttf_parser::tables::stat::AxisValueSubtables

*Struct*

Iterator over axis value subtables.

**Generic Parameters:**
- 'a

**Fields:**
- `data: crate::parser::Stream<'a>`
- `start: crate::parser::Offset32`
- `offsets: crate::LazyArray16<'a, crate::parser::Offset16>`
- `index: u16`
- `version: u32`

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> AxisValueSubtables<'a>`



## ttf_parser::tables::stat::Table

*Struct*

A [Style Attributes Table](https://docs.microsoft.com/en-us/typography/opentype/spec/stat).

**Generic Parameters:**
- 'a

**Fields:**
- `axes: crate::LazyArray16<'a, AxisRecord>` - List of axes
- `fallback_name_id: Option<u16>` - Fallback name when everything can be elided.
- `version: u32`
- `data: &'a [u8]`
- `value_lookup_start: crate::parser::Offset32`
- `value_offsets: crate::LazyArray16<'a, crate::parser::Offset16>`

**Methods:**

- `fn parse(data: &'a [u8]) -> Option<Self>` - Parses a table from raw data.
- `fn subtables(self: &Self) -> AxisValueSubtables<'a>` - Returns an iterator over the collection of axis value tables.
- `fn subtable_for_axis(self: &Self, axis: Tag, match_value: Option<Fixed>) -> Option<AxisValueSubtable>` - Returns the first matching subtable for a given axis.

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Table<'a>`



