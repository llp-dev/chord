*[gimli](../../index.md) / [read](../index.md) / [unit](index.md)*

---

# Module `unit`

Functions for parsing DWARF `.debug_info` and `.debug_types` sections.

## Contents

- [Structs](#structs)
  - [`DebugInfo`](#debuginfo)
  - [`DebugInfoUnitHeadersIter`](#debuginfounitheadersiter)
  - [`UnitHeader`](#unitheader)
  - [`DebuggingInformationEntry`](#debugginginformationentry)
  - [`Attribute`](#attribute)
  - [`EntriesRaw`](#entriesraw)
  - [`EntriesCursor`](#entriescursor)
  - [`EntriesTree`](#entriestree)
  - [`EntriesTreeNode`](#entriestreenode)
  - [`EntriesTreeIter`](#entriestreeiter)
  - [`DebugTypes`](#debugtypes)
  - [`DebugTypesUnitHeadersIter`](#debugtypesunitheadersiter)
- [Enums](#enums)
  - [`UnitType`](#unittype)
  - [`AttributeValue`](#attributevalue)
- [Functions](#functions)
  - [`parse_unit_header`](#parse-unit-header)
  - [`allow_section_offset`](#allow-section-offset)
  - [`parse_attribute`](#parse-attribute)
  - [`skip_attributes`](#skip-attributes)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DebugInfo`](#debuginfo) | struct | The `DebugInfo` struct represents the DWARF debugging information found in the `.debug_info` section. |
| [`DebugInfoUnitHeadersIter`](#debuginfounitheadersiter) | struct | An iterator over the units of a .debug_info section. |
| [`UnitHeader`](#unitheader) | struct | The common fields for the headers of compilation units and type units. |
| [`DebuggingInformationEntry`](#debugginginformationentry) | struct | A Debugging Information Entry (DIE). |
| [`Attribute`](#attribute) | struct | An attribute in a `DebuggingInformationEntry`, consisting of a name and associated value. |
| [`EntriesRaw`](#entriesraw) | struct | A raw reader of the data that defines the Debugging Information Entries. |
| [`EntriesCursor`](#entriescursor) | struct | A cursor into the Debugging Information Entries tree for a compilation unit. |
| [`EntriesTree`](#entriestree) | struct | The state information for a tree view of the Debugging Information Entries. |
| [`EntriesTreeNode`](#entriestreenode) | struct | A node in the Debugging Information Entry tree. |
| [`EntriesTreeIter`](#entriestreeiter) | struct | An iterator that allows traversal of the children of an `EntriesTreeNode`. |
| [`DebugTypes`](#debugtypes) | struct | The `DebugTypes` struct represents the DWARF type information found in the `.debug_types` section. |
| [`DebugTypesUnitHeadersIter`](#debugtypesunitheadersiter) | struct | An iterator over the type-units of this `.debug_types` section. |
| [`UnitType`](#unittype) | enum | This enum specifies the type of the unit and any type specific data carried in the header (e.g. the type signature/type offset of a type unit). |
| [`AttributeValue`](#attributevalue) | enum | The value of an attribute in a `DebuggingInformationEntry`. |
| [`parse_unit_header`](#parse-unit-header) | fn | Parse a unit header. |
| [`allow_section_offset`](#allow-section-offset) | fn |  |
| [`parse_attribute`](#parse-attribute) | fn |  |
| [`skip_attributes`](#skip-attributes) | fn |  |

## Structs

### `DebugInfo<R>`

```rust
struct DebugInfo<R> {
    debug_info_section: R,
}
```

The `DebugInfo` struct represents the DWARF debugging information found in
the `.debug_info` section.

#### Implementations

- <span id="debuginfo-new"></span>`fn new(debug_info_section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugInfo` instance from the data in the `.debug_info`

  section.

  

  It is the caller's responsibility to read the `.debug_info` section and

  present it as a `&[u8]` slice. That means using some ELF loader on

  Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugInfo, LittleEndian};

  

  let buf = [0x00, 0x01, 0x02, 0x03];

  let read_debug_info_section_somehow = || &buf;

  let debug_info = DebugInfo::new(read_debug_info_section_somehow(), LittleEndian);

  ```

#### Trait Implementations

##### `impl<R: clone::Clone> Clone for DebugInfo<R>`

- <span id="debuginfo-clone"></span>`fn clone(&self) -> DebugInfo<R>` — [`DebugInfo`](../index.md#debuginfo)

##### `impl<R: marker::Copy> Copy for DebugInfo<R>`

##### `impl<R: fmt::Debug> Debug for DebugInfo<R>`

- <span id="debuginfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugInfo<R>`

- <span id="debuginfo-default"></span>`fn default() -> DebugInfo<R>` — [`DebugInfo`](../index.md#debuginfo)

##### `impl<R> Section for DebugInfo<R>`

- <span id="debuginfo-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md#sectionid)

- <span id="debuginfo-section-reader"></span>`fn reader(&self) -> &R`

### `DebugInfoUnitHeadersIter<R: Reader>`

```rust
struct DebugInfoUnitHeadersIter<R: Reader> {
    input: R,
    offset: crate::common::UnitSectionOffset<<R as >::Offset>,
}
```

An iterator over the units of a .debug_info section.

See the [documentation on
`DebugInfo::units`](./struct.DebugInfo.html#method.units) for more detail.

#### Implementations

- <span id="debuginfounitheadersiter-next"></span>`fn next(&mut self) -> Result<Option<UnitHeader<R>>>` — [`Result`](../../index.md#result), [`UnitHeader`](../index.md#unitheader)

  Advance the iterator to the next unit header.

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for DebugInfoUnitHeadersIter<R>`

- <span id="debuginfounitheadersiter-clone"></span>`fn clone(&self) -> DebugInfoUnitHeadersIter<R>` — [`DebugInfoUnitHeadersIter`](../index.md#debuginfounitheadersiter)

##### `impl<R: fmt::Debug + Reader> Debug for DebugInfoUnitHeadersIter<R>`

- <span id="debuginfounitheadersiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for DebugInfoUnitHeadersIter<R>`

- <span id="debuginfounitheadersiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="debuginfounitheadersiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="debuginfounitheadersiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: Reader> Iterator for DebugInfoUnitHeadersIter<R>`

- <span id="debuginfounitheadersiter-iterator-type-item"></span>`type Item = Result<UnitHeader<R>, Error>`

- <span id="debuginfounitheadersiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `UnitHeader<R, Offset>`

```rust
struct UnitHeader<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    encoding: crate::common::Encoding,
    unit_length: Offset,
    unit_type: UnitType<Offset>,
    debug_abbrev_offset: crate::common::DebugAbbrevOffset<Offset>,
    section: crate::common::SectionId,
    unit_offset: crate::common::UnitSectionOffset<Offset>,
    entries_buf: R,
}
```

The common fields for the headers of compilation units and
type units.

#### Implementations

- <span id="unitheader-new"></span>`fn new(encoding: Encoding, unit_length: Offset, unit_type: UnitType<Offset>, debug_abbrev_offset: DebugAbbrevOffset<Offset>, section: SectionId, unit_offset: UnitSectionOffset<Offset>, entries_buf: R) -> Self` — [`Encoding`](../../index.md#encoding), [`UnitType`](../index.md#unittype), [`DebugAbbrevOffset`](../../index.md#debugabbrevoffset), [`SectionId`](../../index.md#sectionid), [`UnitSectionOffset`](../../index.md#unitsectionoffset)

  Construct a new `UnitHeader`.

#### Trait Implementations

##### `impl<R, Offset> Clone for UnitHeader<R, Offset>`

- <span id="unitheader-clone"></span>`fn clone(&self) -> UnitHeader<R, Offset>` — [`UnitHeader`](../index.md#unitheader)

##### `impl<R, Offset> Copy for UnitHeader<R, Offset>`

##### `impl<R, Offset> Debug for UnitHeader<R, Offset>`

- <span id="unitheader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Offset> Eq for UnitHeader<R, Offset>`

##### `impl<R, Offset> PartialEq for UnitHeader<R, Offset>`

- <span id="unitheader-partialeq-eq"></span>`fn eq(&self, other: &UnitHeader<R, Offset>) -> bool` — [`UnitHeader`](../index.md#unitheader)

##### `impl<R, Offset> StructuralPartialEq for UnitHeader<R, Offset>`

### `DebuggingInformationEntry<R, Offset>`

```rust
struct DebuggingInformationEntry<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    pub tag: constants::DwTag,
    pub has_children: bool,
    pub attrs: alloc::vec::Vec<Attribute<R>>,
    pub offset: crate::read::UnitOffset<Offset>,
    pub depth: isize,
}
```

A Debugging Information Entry (DIE).

DIEs have a set of attributes and optionally have children DIEs as well.

Creating a `DebuggingInformationEntry` requires an allocation, so frequent cloning of
this struct may be slow.

#### Fields

- **`tag`**: `constants::DwTag`

  The DWARF tag for this entry.

- **`has_children`**: `bool`

  Whether this entry has a null terminated list of children.
  
  Note that this list may be empty.

- **`attrs`**: `alloc::vec::Vec<Attribute<R>>`

  This entry's attributes.

- **`offset`**: `crate::read::UnitOffset<Offset>`

  The offset within the unit of this entry.

- **`depth`**: `isize`

  The tree depth of this entry relative to the entry where reading started.

#### Implementations

- <span id="debugginginformationentry-new"></span>`fn new(tag: constants::DwTag, has_children: bool, attrs: Vec<Attribute<R>>, offset: UnitOffset<Offset>) -> Self` — [`DwTag`](../../index.md#dwtag), [`Attribute`](../index.md#attribute), [`UnitOffset`](../../index.md#unitoffset)

  Construct a new `DebuggingInformationEntry`.

- <span id="debugginginformationentry-null"></span>`fn null() -> Self`

  Construct a new `DebuggingInformationEntry` with no tag or attributes.

- <span id="debugginginformationentry-is-null"></span>`fn is_null(&self) -> bool`

  Return true if the entry tag is `DW_TAG_null`.

- <span id="debugginginformationentry-set-null"></span>`fn set_null(&mut self)`

  Set the entry to a null entry.

  

  This sets the tag to `DW_TAG_null`, and resets the `has_children` and `attrs` fields.

- <span id="debugginginformationentry-depth"></span>`fn depth(&self) -> isize`

  Get the tree depth of this entry relative to the entry where reading started.

- <span id="debugginginformationentry-offset"></span>`fn offset(&self) -> UnitOffset<Offset>` — [`UnitOffset`](../../index.md#unitoffset)

  Get this entry's offset.

- <span id="debugginginformationentry-tag"></span>`fn tag(&self) -> constants::DwTag` — [`DwTag`](../../index.md#dwtag)

  Get this entry's tag.

- <span id="debugginginformationentry-has-children"></span>`fn has_children(&self) -> bool`

  Return true if this entry's type can have children, false otherwise.

- <span id="debugginginformationentry-attrs"></span>`fn attrs(&self) -> &[Attribute<R>]` — [`Attribute`](../index.md#attribute)

  Return this entry's set of attributes.

- <span id="debugginginformationentry-has-attr"></span>`fn has_attr(&self, name: constants::DwAt) -> bool` — [`DwAt`](../../index.md#dwat)

  Return true if this entry has an attribute with the given name.

- <span id="debugginginformationentry-attr"></span>`fn attr(&self, name: constants::DwAt) -> Option<&Attribute<R>>` — [`DwAt`](../../index.md#dwat), [`Attribute`](../index.md#attribute)

  Find the first attribute in this entry which has the given name.

  

  Returns `None` if no attribute is found.

- <span id="debugginginformationentry-attr-value-raw"></span>`fn attr_value_raw(&self, name: constants::DwAt) -> Option<AttributeValue<R>>` — [`DwAt`](../../index.md#dwat), [`AttributeValue`](../index.md#attributevalue)

  Find the first attribute in this entry which has the given name,

  and return its raw value.

  

  Returns `None` if no attribute is found.

- <span id="debugginginformationentry-attr-value"></span>`fn attr_value(&self, name: constants::DwAt) -> Option<AttributeValue<R>>` — [`DwAt`](../../index.md#dwat), [`AttributeValue`](../index.md#attributevalue)

  Find the first attribute in this entry which has the given name,

  and return its normalized value.

  

  Returns `None` if no attribute is found.

- <span id="debugginginformationentry-sibling"></span>`fn sibling(&self) -> Option<UnitOffset<<R as >::Offset>>` — [`UnitOffset`](../../index.md#unitoffset), [`Reader`](../index.md#reader)

  Use the `DW_AT_sibling` attribute to find the offset for the

  next sibling. Returns `None` if the attribute is missing or invalid.

#### Trait Implementations

##### `impl<R, Offset> Clone for DebuggingInformationEntry<R, Offset>`

- <span id="debugginginformationentry-clone"></span>`fn clone(&self) -> DebuggingInformationEntry<R, Offset>` — [`DebuggingInformationEntry`](../index.md#debugginginformationentry)

##### `impl<R, Offset> Debug for DebuggingInformationEntry<R, Offset>`

- <span id="debugginginformationentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Offset> Default for DebuggingInformationEntry<R, Offset>`

- <span id="debugginginformationentry-default"></span>`fn default() -> Self`

### `Attribute<R: Reader>`

```rust
struct Attribute<R: Reader> {
    name: constants::DwAt,
    form: constants::DwForm,
    value: AttributeValue<R>,
}
```

An attribute in a `DebuggingInformationEntry`, consisting of a name and
associated value.

#### Implementations

- <span id="attribute-name"></span>`fn name(&self) -> constants::DwAt` — [`DwAt`](../../index.md#dwat)

  Get this attribute's name.

- <span id="attribute-form"></span>`fn form(&self) -> constants::DwForm` — [`DwForm`](../../index.md#dwform)

  Get the form that was used to encode this attribute's value.

- <span id="attribute-raw-value"></span>`fn raw_value(&self) -> AttributeValue<R>` — [`AttributeValue`](../index.md#attributevalue)

  Get this attribute's raw value.

- <span id="attribute-value"></span>`fn value(&self) -> AttributeValue<R>` — [`AttributeValue`](../index.md#attributevalue)

  Get this attribute's normalized value.

  

  Attribute values can potentially be encoded in multiple equivalent forms,

  and may have special meaning depending on the attribute name.  This method

  converts the attribute value to a normalized form based on the attribute

  name.

  

  See "Table 7.5: Attribute encodings" and "Table 7.6: Attribute form encodings".

- <span id="attribute-u8-value"></span>`fn u8_value(&self) -> Option<u8>`

  Try to convert this attribute's value to a u8.

- <span id="attribute-u16-value"></span>`fn u16_value(&self) -> Option<u16>`

  Try to convert this attribute's value to a u16.

- <span id="attribute-udata-value"></span>`fn udata_value(&self) -> Option<u64>`

  Try to convert this attribute's value to an unsigned integer.

- <span id="attribute-sdata-value"></span>`fn sdata_value(&self) -> Option<i64>`

  Try to convert this attribute's value to a signed integer.

- <span id="attribute-offset-value"></span>`fn offset_value(&self) -> Option<<R as >::Offset>` — [`Reader`](../index.md#reader)

  Try to convert this attribute's value to an offset.

- <span id="attribute-exprloc-value"></span>`fn exprloc_value(&self) -> Option<Expression<R>>` — [`Expression`](../index.md#expression)

  Try to convert this attribute's value to an expression or location buffer.

  

  Expressions and locations may be `DW_FORM_block*` or `DW_FORM_exprloc`.

  The standard doesn't mention `DW_FORM_block*` as a possible form, but

  it is encountered in practice.

- <span id="attribute-string-value"></span>`fn string_value(&self, debug_str: &DebugStr<R>) -> Option<R>` — [`DebugStr`](../index.md#debugstr)

  Try to return this attribute's value as a string slice.

  

  If this attribute's value is either an inline `DW_FORM_string` string,

  or a `DW_FORM_strp` reference to an offset into the `.debug_str`

  section, return the attribute's string value as `Some`. Other attribute

  value forms are returned as `None`.

  

  Warning: this function does not handle all possible string forms.

  Use `Dwarf::attr_string` instead.

- <span id="attribute-string-value-sup"></span>`fn string_value_sup(&self, debug_str: &DebugStr<R>, debug_str_sup: Option<&DebugStr<R>>) -> Option<R>` — [`DebugStr`](../index.md#debugstr)

  Try to return this attribute's value as a string slice.

  

  If this attribute's value is either an inline `DW_FORM_string` string,

  or a `DW_FORM_strp` reference to an offset into the `.debug_str`

  section, or a `DW_FORM_strp_sup` reference to an offset into a supplementary

  object file, return the attribute's string value as `Some`. Other attribute

  value forms are returned as `None`.

  

  Warning: this function does not handle all possible string forms.

  Use `Dwarf::attr_string` instead.

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for Attribute<R>`

- <span id="attribute-clone"></span>`fn clone(&self) -> Attribute<R>` — [`Attribute`](../index.md#attribute)

##### `impl<R: marker::Copy + Reader> Copy for Attribute<R>`

##### `impl<R: fmt::Debug + Reader> Debug for Attribute<R>`

- <span id="attribute-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: cmp::Eq + Reader> Eq for Attribute<R>`

##### `impl<R: cmp::PartialEq + Reader> PartialEq for Attribute<R>`

- <span id="attribute-partialeq-eq"></span>`fn eq(&self, other: &Attribute<R>) -> bool` — [`Attribute`](../index.md#attribute)

##### `impl<R: Reader> StructuralPartialEq for Attribute<R>`

### `EntriesRaw<'abbrev, R>`

```rust
struct EntriesRaw<'abbrev, R>
where
    R: Reader {
    input: R,
    encoding: crate::common::Encoding,
    abbreviations: &'abbrev crate::read::Abbreviations,
    end_offset: crate::read::UnitOffset<<R as >::Offset>,
    depth: isize,
}
```

A raw reader of the data that defines the Debugging Information Entries.

`EntriesRaw` provides primitives to read the components of Debugging Information
Entries (DIEs). A DIE consists of an abbreviation code (read with `read_abbreviation`)
followed by a number of attributes (read with `read_attribute`).
The user must provide the control flow to read these correctly.
In particular, all attributes must always be read before reading another
abbreviation code.

Alternatively, you may call `Self::read_entry` to read the abbreviations and all
attributes. The attributes are stored in a `Vec` in the DIE, so you should try to
reuse this allocation for future calls.

`EntriesRaw` lacks some features of `EntriesCursor`, such as the ability to skip
to the next sibling DIE. However, this also allows it to optimize better, since it
does not need to perform the extra bookkeeping required to support these features,
and thus it is suitable for cases where performance is important.

## Example Usage
```rust,no_run
fn example() -> Result<(), gimli::Error> {
let debug_info = gimli::DebugInfo::new(&[], gimli::LittleEndian);
let get_some_unit = || debug_info.units().next().unwrap().unwrap();
let unit = get_some_unit();
let debug_abbrev = gimli::DebugAbbrev::new(&[], gimli::LittleEndian);
let get_abbrevs_for_unit = |_| unit.abbreviations(&debug_abbrev).unwrap();
let abbrevs = get_abbrevs_for_unit(&unit);

let mut entries = unit.entries_raw(&abbrevs, None)?;
while !entries.is_empty() {
    let abbrev = if let Some(abbrev) = entries.read_abbreviation()? {
        abbrev
    } else {
        // Null entry with no attributes.
        continue
    };
    match abbrev.tag() {
        gimli::DW_TAG_subprogram => {
            // Loop over attributes for DIEs we care about.
            for spec in abbrev.attributes() {
                let attr = entries.read_attribute(*spec)?;
                match attr.name() {
                    // Handle attributes.
                    _ => {}
                }
            }
        }
        _ => {
            // Skip attributes for DIEs we don't care about.
            entries.skip_attributes(abbrev.attributes());
        }
    }
}
unreachable!()
}
```

#### Implementations

- <span id="entriesraw-new"></span>`fn new(input: R, encoding: Encoding, abbreviations: &'abbrev Abbreviations, offset: UnitOffset<<R as >::Offset>) -> Self` — [`Encoding`](../../index.md#encoding), [`Abbreviations`](../index.md#abbreviations), [`UnitOffset`](../../index.md#unitoffset), [`Reader`](../index.md#reader)

  Construct a new `EntriesRaw`.

  

  `input` may be anywhere within the entries for a unit, including partway

  through an entry. It is up to the caller to know what needs to be parsed

  next.

  

  `offset` may be any value. It is used as the initial value returned by

  `Self::next_offset`.

- <span id="entriesraw-empty"></span>`fn empty(&mut self)`

- <span id="entriesraw-is-empty"></span>`fn is_empty(&self) -> bool`

  Return true if there is no more input.

- <span id="entriesraw-seek-forward"></span>`fn seek_forward(&mut self, offset: UnitOffset<<R as >::Offset>, depth: isize) -> bool` — [`UnitOffset`](../../index.md#unitoffset), [`Reader`](../index.md#reader)

- <span id="entriesraw-next-offset"></span>`fn next_offset(&self) -> UnitOffset<<R as >::Offset>` — [`UnitOffset`](../../index.md#unitoffset), [`Reader`](../index.md#reader)

  Return the unit offset at which the reader will read next.

  

  If you want the offset of the next entry, then this must be called prior to reading

  the next entry.

- <span id="entriesraw-next-depth"></span>`fn next_depth(&self) -> isize`

  Return the depth of the next entry.

  

  This depth is updated when `read_abbreviation` is called, and is updated

  based on null entries and the `has_children` field in the abbreviation.

- <span id="entriesraw-read-entry"></span>`fn read_entry(&mut self, entry: &mut DebuggingInformationEntry<R>) -> Result<bool>` — [`DebuggingInformationEntry`](../index.md#debugginginformationentry), [`Result`](../../index.md#result)

  Read an entry into a placeholder.

  

  This reads the abbreviation code and all attributes. The attributes are stored in

  a `Vec` in the entry, so you should try to reuse this entry for future calls.

  

  Returns `Ok(false)` if a null entry is read. All fields in the entry will be

  set appropriately for a null entry.

  

  Returns `Err` if an error occurred. Some fields in the entry may be modified

  depending on where the error occurred.

- <span id="entriesraw-read-abbreviation"></span>`fn read_abbreviation(&mut self) -> Result<Option<&'abbrev Abbreviation>>` — [`Result`](../../index.md#result), [`Abbreviation`](../index.md#abbreviation)

  Read an abbreviation code and lookup the corresponding `Abbreviation`.

  

  Returns `Ok(None)` for null entries.

  

  Returns `Err` if end of input is reached, or the code is invalid.

- <span id="entriesraw-read-attribute"></span>`fn read_attribute(&mut self, spec: AttributeSpecification) -> Result<Attribute<R>>` — [`AttributeSpecification`](../index.md#attributespecification), [`Result`](../../index.md#result), [`Attribute`](../index.md#attribute)

  Read an attribute.

  

  This function is never inlined. Consider using `read_attribute_inline` instead

  if you only call this from a small number of places.

- <span id="entriesraw-read-attribute-inline"></span>`fn read_attribute_inline(&mut self, spec: AttributeSpecification) -> Result<Attribute<R>>` — [`AttributeSpecification`](../index.md#attributespecification), [`Result`](../../index.md#result), [`Attribute`](../index.md#attribute)

  Read an attribute.

  

  Identical to `read_attribute`, but has the `#[inline(always)]` attribute.

  This allows better optimisation at the cost of code size.

- <span id="entriesraw-read-attributes"></span>`fn read_attributes(&mut self, specs: &[AttributeSpecification], attrs: &mut Vec<Attribute<R>>) -> Result<()>` — [`AttributeSpecification`](../index.md#attributespecification), [`Attribute`](../index.md#attribute), [`Result`](../../index.md#result)

  Read all attributes into a `Vec`.

  

  This will clear `attrs` before reading.

  

  It is recommended to reuse the same `Vec` for multiple calls to avoid allocations.

- <span id="entriesraw-skip-attributes"></span>`fn skip_attributes(&mut self, specs: &[AttributeSpecification]) -> Result<()>` — [`AttributeSpecification`](../index.md#attributespecification), [`Result`](../../index.md#result)

  Skip all the attributes of an abbreviation.

#### Trait Implementations

##### `impl<R> Clone for EntriesRaw<'abbrev, R>`

- <span id="entriesraw-clone"></span>`fn clone(&self) -> EntriesRaw<'abbrev, R>` — [`EntriesRaw`](../index.md#entriesraw)

##### `impl<R> Debug for EntriesRaw<'abbrev, R>`

- <span id="entriesraw-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `EntriesCursor<'abbrev, R>`

```rust
struct EntriesCursor<'abbrev, R>
where
    R: Reader {
    input: EntriesRaw<'abbrev, R>,
    cached_current: DebuggingInformationEntry<R>,
}
```

A cursor into the Debugging Information Entries tree for a compilation unit.

The `EntriesCursor` can traverse the DIE tree in DFS order using `Self::next_dfs`,
or skip to the next sibling of the entry the cursor is currently pointing to
using `Self::next_sibling`.

`Self::next_dfs` will skip over the null DIEs that delimit lists of children. Use
`Self::next_entry` if you wish to stop at null DIEs. This may be useful if you want
to read entries at a specific depth, such as moving to the first child prior to using
`Self::next_sibling`.

#### Implementations

- <span id="entriescursor-new"></span>`fn new(input: R, encoding: Encoding, abbreviations: &'abbrev Abbreviations, offset: UnitOffset<<R as >::Offset>) -> Self` — [`Encoding`](../../index.md#encoding), [`Abbreviations`](../index.md#abbreviations), [`UnitOffset`](../../index.md#unitoffset), [`Reader`](../index.md#reader)

- <span id="entriescursor-current"></span>`fn current(&self) -> Option<&DebuggingInformationEntry<R>>` — [`DebuggingInformationEntry`](../index.md#debugginginformationentry)

  Get a reference to the entry that the cursor is currently pointing to.

  

  If the cursor is not pointing at an entry, or if the current entry is a

  null entry, then `None` is returned.

- <span id="entriescursor-offset"></span>`fn offset(&self) -> UnitOffset<<R as >::Offset>` — [`UnitOffset`](../../index.md#unitoffset), [`Reader`](../index.md#reader)

  The unit offset of the current DIE.

  

  This works even if the cursor is positioned at a null DIE.

- <span id="entriescursor-depth"></span>`fn depth(&self) -> isize`

  The tree depth of the current DIE.

  

  This works even if the cursor is positioned at a null DIE.

- <span id="entriescursor-next-offset"></span>`fn next_offset(&self) -> UnitOffset<<R as >::Offset>` — [`UnitOffset`](../../index.md#unitoffset), [`Reader`](../index.md#reader)

  The unit offset of the next DIE that `Self::next_entry` will read.

- <span id="entriescursor-next-depth"></span>`fn next_depth(&self) -> isize`

  The tree depth of the next DIE that `Self::next_entry` will read.

- <span id="entriescursor-next-entry"></span>`fn next_entry(&mut self) -> Result<bool>` — [`Result`](../../index.md#result)

  Move the cursor to the next DIE in the tree.

  

  Returns `true` if there is a next entry, even if this entry is null.

  If there is no next entry, then `false` is returned.

- <span id="entriescursor-next-dfs"></span>`fn next_dfs(&mut self) -> Result<Option<&DebuggingInformationEntry<R>>>` — [`Result`](../../index.md#result), [`DebuggingInformationEntry`](../index.md#debugginginformationentry)

  Move the cursor to the next DIE in the tree in DFS order.

  

  Upon successful movement of the cursor, returns the entry, which may be:

  

    * The first child of the previous current entry, if any.

  

    * The sibling of the previous current entry, if any.

  

    * The sibling of the previous entry's parent, if any, and so on.

  

  If there is no next entry, then `None` is returned.

  

  Here is an example that prints the offset and depth of all entries in a

  compilation unit.

  

  ```rust

  use gimli::{DebugAbbrev, DebugInfo, LittleEndian};

  let info_buf = [

      // Comilation unit header

  

      // 32-bit unit length = 25

      0x19, 0x00, 0x00, 0x00,

      // Version 4

      0x04, 0x00,

      // debug_abbrev_offset

      0x00, 0x00, 0x00, 0x00,

      // Address size

      0x04,

  

      // DIEs

  

      // Abbreviation code

      0x01,

      // Attribute of form DW_FORM_string = "foo\0"

      0x66, 0x6f, 0x6f, 0x00,

  

        // Children

  

        // Abbreviation code

        0x01,

        // Attribute of form DW_FORM_string = "foo\0"

        0x66, 0x6f, 0x6f, 0x00,

  

          // Children

  

          // Abbreviation code

          0x01,

          // Attribute of form DW_FORM_string = "foo\0"

          0x66, 0x6f, 0x6f, 0x00,

  

            // Children

  

            // End of children

            0x00,

  

          // End of children

          0x00,

  

        // End of children

        0x00,

  ];

  let debug_info = DebugInfo::new(&info_buf, LittleEndian);

  

  let abbrev_buf = [

      // Code

      0x01,

      // DW_TAG_subprogram

      0x2e,

      // DW_CHILDREN_yes

      0x01,

      // Begin attributes

        // Attribute name = DW_AT_name

        0x03,

        // Attribute form = DW_FORM_string

        0x08,

      // End attributes

      0x00,

      0x00,

      // Null terminator

      0x00

  ];

  let debug_abbrev = DebugAbbrev::new(&abbrev_buf, LittleEndian);

  

  let get_some_unit = || debug_info.units().next().unwrap().unwrap();

  

  let unit = get_some_unit();

  let get_abbrevs_for_unit = |_| unit.abbreviations(&debug_abbrev).unwrap();

  let abbrevs = get_abbrevs_for_unit(&unit);

  let mut cursor = unit.entries(&abbrevs);

  

  // Traverse the DIE tree in depth-first search order.

  while let Some(current) = cursor.next_dfs().expect("Should parse next dfs") {

      println!(

          "Offset: {:x} Depth: {} Tag: {}",

          current.offset().0,

          current.depth(),

          current.tag()

      );

  }

  ```

- <span id="entriescursor-next-sibling"></span>`fn next_sibling(&mut self) -> Result<Option<&DebuggingInformationEntry<R>>>` — [`Result`](../../index.md#result), [`DebuggingInformationEntry`](../index.md#debugginginformationentry)

  Move the cursor to the next sibling DIE of the current one.

  

  Returns `Ok(Some(entry))` when the cursor has been moved to the next sibling,

  `Ok(None)` when there is no next sibling.

  

  The depth of the cursor is never changed if this method returns `Ok`. Once

  `Ok(None)` is returned, this method will continue to return `Ok(None)` until

  either `Self::next_entry` or `Self::next_dfs` is called.

  

  This method is useful for reading only the children of a DIE. However, you

  must first move the cursor to the first child. `Self::next_entry` is usually

  the easiest way to do this. You should also use either `Self::next_depth` or

  `DebuggingInformationEntry::has_children` to determine if the DIE can have

  children.

  

  Here is an example that iterates over all of the direct children of the

  root entry:

  

  ```rust

  use gimli::{DebugAbbrev, DebugInfo, LittleEndian};

  let info_buf = [

      // Comilation unit header

  

      // 32-bit unit length = 25

      0x19, 0x00, 0x00, 0x00,

      // Version 4

      0x04, 0x00,

      // debug_abbrev_offset

      0x00, 0x00, 0x00, 0x00,

      // Address size

      0x04,

  

      // DIEs

  

      // Abbreviation code

      0x01,

      // Attribute of form DW_FORM_string = "foo\0"

      0x66, 0x6f, 0x6f, 0x00,

  

        // Children

  

        // Abbreviation code

        0x01,

        // Attribute of form DW_FORM_string = "foo\0"

        0x66, 0x6f, 0x6f, 0x00,

  

          // Children

  

          // Abbreviation code

          0x01,

          // Attribute of form DW_FORM_string = "foo\0"

          0x66, 0x6f, 0x6f, 0x00,

  

            // Children

  

            // End of children

            0x00,

  

          // End of children

          0x00,

  

        // End of children

        0x00,

  ];

  let debug_info = DebugInfo::new(&info_buf, LittleEndian);

  

  let get_some_unit = || debug_info.units().next().unwrap().unwrap();

  

  let abbrev_buf = [

      // Code

      0x01,

      // DW_TAG_subprogram

      0x2e,

      // DW_CHILDREN_yes

      0x01,

      // Begin attributes

        // Attribute name = DW_AT_name

        0x03,

        // Attribute form = DW_FORM_string

        0x08,

      // End attributes

      0x00,

      0x00,

      // Null terminator

      0x00

  ];

  let debug_abbrev = DebugAbbrev::new(&abbrev_buf, LittleEndian);

  

  let unit = get_some_unit();

  let get_abbrevs_for_unit = |_| unit.abbreviations(&debug_abbrev).unwrap();

  let abbrevs = get_abbrevs_for_unit(&unit);

  

  let mut cursor = unit.entries(&abbrevs);

  

  // Move the cursor to the root.

  assert!(cursor.next_entry().unwrap());

  

  // Move the cursor to the root's first child, if any.

  assert!(cursor.current().unwrap().has_children());

  assert!(cursor.next_entry().unwrap());

  

  // Iterate the root's children.

  while let Some(current) = cursor.current() {

      println!("{:?} is a child of the root", current);

      cursor.next_sibling().expect("Should parse next sibling");

  }

  ```

#### Trait Implementations

##### `impl<R> Clone for EntriesCursor<'abbrev, R>`

- <span id="entriescursor-clone"></span>`fn clone(&self) -> EntriesCursor<'abbrev, R>` — [`EntriesCursor`](../index.md#entriescursor)

##### `impl<R> Debug for EntriesCursor<'abbrev, R>`

- <span id="entriescursor-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `EntriesTree<'abbrev, R>`

```rust
struct EntriesTree<'abbrev, R>
where
    R: Reader {
    root: R,
    input: EntriesRaw<'abbrev, R>,
    entry: DebuggingInformationEntry<R>,
}
```

The state information for a tree view of the Debugging Information Entries.

The `EntriesTree` can be used to recursively iterate through the DIE
tree, following the parent/child relationships. The `EntriesTree` contains
shared state for all nodes in the tree, avoiding any duplicate parsing of
entries during the traversal.

## Example Usage
```rust,no_run
fn example() -> Result<(), gimli::Error> {
let debug_info = gimli::DebugInfo::new(&[], gimli::LittleEndian);
let get_some_unit = || debug_info.units().next().unwrap().unwrap();
let unit = get_some_unit();
let debug_abbrev = gimli::DebugAbbrev::new(&[], gimli::LittleEndian);
let get_abbrevs_for_unit = |_| unit.abbreviations(&debug_abbrev).unwrap();
let abbrevs = get_abbrevs_for_unit(&unit);

let mut tree = unit.entries_tree(&abbrevs, None)?;
let root = tree.root()?;
process_tree(root)?;
unreachable!()
}

fn process_tree<R>(mut node: gimli::EntriesTreeNode<R>) -> gimli::Result<()>
    where R: gimli::Reader
{
    // Examine the entry attributes.
    for attr in node.entry().attrs() {
    }
    let mut children = node.children();
    while let Some(child) = children.next()? {
        // Recursively process a child.
        process_tree(child);
    }
    Ok(())
}
```

#### Implementations

- <span id="entriestree-new"></span>`fn new(root: R, encoding: Encoding, abbreviations: &'abbrev Abbreviations, offset: UnitOffset<<R as >::Offset>) -> Self` — [`Encoding`](../../index.md#encoding), [`Abbreviations`](../index.md#abbreviations), [`UnitOffset`](../../index.md#unitoffset), [`Reader`](../index.md#reader)

- <span id="entriestree-root"></span>`fn root<'me>(self: &'me mut Self) -> Result<EntriesTreeNode<'abbrev, 'me, R>>` — [`Result`](../../index.md#result), [`EntriesTreeNode`](../index.md#entriestreenode)

  Returns the root node of the tree.

- <span id="entriestree-next"></span>`fn next(&mut self, depth: isize) -> Result<bool>` — [`Result`](../../index.md#result)

  Move the cursor to the next entry at the specified depth.

  

  Requires `depth <= self.depth + 1`.

  

  Returns `true` if successful.

#### Trait Implementations

##### `impl<R> Clone for EntriesTree<'abbrev, R>`

- <span id="entriestree-clone"></span>`fn clone(&self) -> EntriesTree<'abbrev, R>` — [`EntriesTree`](../index.md#entriestree)

##### `impl<R> Debug for EntriesTree<'abbrev, R>`

- <span id="entriestree-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `EntriesTreeNode<'abbrev, 'tree, R: Reader>`

```rust
struct EntriesTreeNode<'abbrev, 'tree, R: Reader> {
    tree: &'tree mut EntriesTree<'abbrev, R>,
    depth: isize,
}
```

A node in the Debugging Information Entry tree.

The root node of a tree can be obtained
via [`EntriesTree::root`](./struct.EntriesTree.html#method.root).

#### Implementations

- <span id="entriestreenode-new"></span>`fn new(tree: &'tree mut EntriesTree<'abbrev, R>, depth: isize) -> EntriesTreeNode<'abbrev, 'tree, R>` — [`EntriesTree`](../index.md#entriestree), [`EntriesTreeNode`](../index.md#entriestreenode)

- <span id="entriestreenode-entry"></span>`fn entry(&self) -> &DebuggingInformationEntry<R>` — [`DebuggingInformationEntry`](../index.md#debugginginformationentry)

  Returns the current entry in the tree.

- <span id="entriestreenode-children"></span>`fn children(self) -> EntriesTreeIter<'abbrev, 'tree, R>` — [`EntriesTreeIter`](../index.md#entriestreeiter)

  Create an iterator for the children of the current entry.

  

  The current entry can no longer be accessed after creating the

  iterator.

#### Trait Implementations

##### `impl<R: fmt::Debug + Reader> Debug for EntriesTreeNode<'abbrev, 'tree, R>`

- <span id="entriestreenode-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `EntriesTreeIter<'abbrev, 'tree, R: Reader>`

```rust
struct EntriesTreeIter<'abbrev, 'tree, R: Reader> {
    tree: &'tree mut EntriesTree<'abbrev, R>,
    depth: isize,
    empty: bool,
}
```

An iterator that allows traversal of the children of an
`EntriesTreeNode`.

The items returned by this iterator are also `EntriesTreeNode`s,
which allow recursive traversal of grandchildren, etc.

#### Implementations

- <span id="entriestreeiter-new"></span>`fn new(tree: &'tree mut EntriesTree<'abbrev, R>, depth: isize) -> EntriesTreeIter<'abbrev, 'tree, R>` — [`EntriesTree`](../index.md#entriestree), [`EntriesTreeIter`](../index.md#entriestreeiter)

- <span id="entriestreeiter-next"></span>`fn next<'me>(self: &'me mut Self) -> Result<Option<EntriesTreeNode<'abbrev, 'me, R>>>` — [`Result`](../../index.md#result), [`EntriesTreeNode`](../index.md#entriestreenode)

  Returns an `EntriesTreeNode` for the next child entry.

  

  Returns `None` if there are no more children.

#### Trait Implementations

##### `impl<R: fmt::Debug + Reader> Debug for EntriesTreeIter<'abbrev, 'tree, R>`

- <span id="entriestreeiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `DebugTypes<R>`

```rust
struct DebugTypes<R> {
    debug_types_section: R,
}
```

The `DebugTypes` struct represents the DWARF type information
found in the `.debug_types` section.

#### Implementations

- <span id="debugtypes-new"></span>`fn new(debug_types_section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugTypes` instance from the data in the `.debug_types`

  section.

  

  It is the caller's responsibility to read the `.debug_types` section and

  present it as a `&[u8]` slice. That means using some ELF loader on

  Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugTypes, LittleEndian};

  

  let buf = [0x00, 0x01, 0x02, 0x03];

  let read_debug_types_section_somehow = || &buf;

  let debug_types = DebugTypes::new(read_debug_types_section_somehow(), LittleEndian);

  ```

#### Trait Implementations

##### `impl<R: clone::Clone> Clone for DebugTypes<R>`

- <span id="debugtypes-clone"></span>`fn clone(&self) -> DebugTypes<R>` — [`DebugTypes`](../index.md#debugtypes)

##### `impl<R: marker::Copy> Copy for DebugTypes<R>`

##### `impl<R: fmt::Debug> Debug for DebugTypes<R>`

- <span id="debugtypes-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugTypes<R>`

- <span id="debugtypes-default"></span>`fn default() -> DebugTypes<R>` — [`DebugTypes`](../index.md#debugtypes)

##### `impl<R> Section for DebugTypes<R>`

- <span id="debugtypes-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md#sectionid)

- <span id="debugtypes-section-reader"></span>`fn reader(&self) -> &R`

### `DebugTypesUnitHeadersIter<R: Reader>`

```rust
struct DebugTypesUnitHeadersIter<R: Reader> {
    input: R,
    offset: crate::common::UnitSectionOffset<<R as >::Offset>,
}
```

An iterator over the type-units of this `.debug_types` section.

See the [documentation on
`DebugTypes::units`](./struct.DebugTypes.html#method.units) for
more detail.

#### Implementations

- <span id="debugtypesunitheadersiter-next"></span>`fn next(&mut self) -> Result<Option<UnitHeader<R>>>` — [`Result`](../../index.md#result), [`UnitHeader`](../index.md#unitheader)

  Advance the iterator to the next type unit header.

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for DebugTypesUnitHeadersIter<R>`

- <span id="debugtypesunitheadersiter-clone"></span>`fn clone(&self) -> DebugTypesUnitHeadersIter<R>` — [`DebugTypesUnitHeadersIter`](../index.md#debugtypesunitheadersiter)

##### `impl<R: fmt::Debug + Reader> Debug for DebugTypesUnitHeadersIter<R>`

- <span id="debugtypesunitheadersiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for DebugTypesUnitHeadersIter<R>`

- <span id="debugtypesunitheadersiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="debugtypesunitheadersiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="debugtypesunitheadersiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: Reader> Iterator for DebugTypesUnitHeadersIter<R>`

- <span id="debugtypesunitheadersiter-iterator-type-item"></span>`type Item = Result<UnitHeader<R>, Error>`

- <span id="debugtypesunitheadersiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

## Enums

### `UnitType<Offset>`

```rust
enum UnitType<Offset>
where
    Offset: ReaderOffset {
    Compilation,
    Type {
        type_signature: crate::common::DebugTypeSignature,
        type_offset: crate::read::UnitOffset<Offset>,
    },
    Partial,
    Skeleton(crate::common::DwoId),
    SplitCompilation(crate::common::DwoId),
    SplitType {
        type_signature: crate::common::DebugTypeSignature,
        type_offset: crate::read::UnitOffset<Offset>,
    },
}
```

This enum specifies the type of the unit and any type
specific data carried in the header (e.g. the type
signature/type offset of a type unit).

#### Variants

- **`Compilation`**

  In DWARF5, a unit with type `DW_UT_compile`. In previous DWARF versions,
  any unit appearing in the .debug_info section.

- **`Type`**

  In DWARF5, a unit with type `DW_UT_type`. In DWARF4, any unit appearing
  in the .debug_types section.

- **`Partial`**

  A unit with type `DW_UT_partial`. The root DIE of this unit should be a
  `DW_TAG_partial_unit`.

- **`Skeleton`**

  A unit with type `DW_UT_skeleton`. The enclosed dwo_id can be used to
  link this with the corresponding `SplitCompilation` unit in a dwo file.
  NB: The non-standard GNU split DWARF extension to DWARF 4 will instead
  be a `Compilation` unit with the dwo_id present as an attribute on the
  root DIE.

- **`SplitCompilation`**

  A unit with type `DW_UT_split_compile`. The enclosed dwo_id can be used to
  link this with the corresponding `Skeleton` unit in the original binary.
  NB: The non-standard GNU split DWARF extension to DWARF 4 will instead
  be a `Compilation` unit with the dwo_id present as an attribute on the
  root DIE.

- **`SplitType`**

  A unit with type `DW_UT_split_type`. A split type unit is identical to a
  conventional type unit except for the section in which it appears.

#### Implementations

- <span id="unittype-dw-ut"></span>`fn dw_ut(&self) -> constants::DwUt` — [`DwUt`](../../index.md#dwut)

#### Trait Implementations

##### `impl<Offset> Clone for UnitType<Offset>`

- <span id="unittype-clone"></span>`fn clone(&self) -> UnitType<Offset>` — [`UnitType`](../index.md#unittype)

##### `impl<Offset> Copy for UnitType<Offset>`

##### `impl<Offset> Debug for UnitType<Offset>`

- <span id="unittype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Offset> Eq for UnitType<Offset>`

##### `impl<Offset> PartialEq for UnitType<Offset>`

- <span id="unittype-partialeq-eq"></span>`fn eq(&self, other: &UnitType<Offset>) -> bool` — [`UnitType`](../index.md#unittype)

##### `impl<Offset> StructuralPartialEq for UnitType<Offset>`

### `AttributeValue<R, Offset>`

```rust
enum AttributeValue<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    Addr(u64),
    Block(R),
    Data1(u8),
    Data2(u16),
    Data4(u32),
    Data8(u64),
    Data16(u128),
    Sdata(i64),
    Udata(u64),
    Exprloc(crate::read::Expression<R>),
    Flag(bool),
    SecOffset(Offset),
    DebugAddrBase(crate::common::DebugAddrBase<Offset>),
    DebugAddrIndex(crate::common::DebugAddrIndex<Offset>),
    UnitRef(crate::read::UnitOffset<Offset>),
    DebugInfoRef(crate::common::DebugInfoOffset<Offset>),
    DebugInfoRefSup(crate::common::DebugInfoOffset<Offset>),
    DebugLineRef(crate::common::DebugLineOffset<Offset>),
    LocationListsRef(crate::common::LocationListsOffset<Offset>),
    DebugLocListsBase(crate::common::DebugLocListsBase<Offset>),
    DebugLocListsIndex(crate::common::DebugLocListsIndex<Offset>),
    DebugMacinfoRef(crate::common::DebugMacinfoOffset<Offset>),
    DebugMacroRef(crate::common::DebugMacroOffset<Offset>),
    RangeListsRef(crate::common::RawRangeListsOffset<Offset>),
    DebugRngListsBase(crate::common::DebugRngListsBase<Offset>),
    DebugRngListsIndex(crate::common::DebugRngListsIndex<Offset>),
    DebugTypesRef(crate::common::DebugTypeSignature),
    DebugStrRef(crate::common::DebugStrOffset<Offset>),
    DebugStrRefSup(crate::common::DebugStrOffset<Offset>),
    DebugStrOffsetsBase(crate::common::DebugStrOffsetsBase<Offset>),
    DebugStrOffsetsIndex(crate::common::DebugStrOffsetsIndex<Offset>),
    DebugLineStrRef(crate::common::DebugLineStrOffset<Offset>),
    String(R),
    Encoding(constants::DwAte),
    DecimalSign(constants::DwDs),
    Endianity(constants::DwEnd),
    Accessibility(constants::DwAccess),
    Visibility(constants::DwVis),
    Virtuality(constants::DwVirtuality),
    Language(constants::DwLang),
    AddressClass(constants::DwAddr),
    IdentifierCase(constants::DwId),
    CallingConvention(constants::DwCc),
    Inline(constants::DwInl),
    Ordering(constants::DwOrd),
    FileIndex(u64),
    DwoId(crate::common::DwoId),
}
```

The value of an attribute in a `DebuggingInformationEntry`.

#### Variants

- **`Addr`**

  "Refers to some location in the address space of the described program."

- **`Block`**

  A slice of an arbitrary number of bytes.

- **`Data1`**

  A one byte constant data value. How to interpret the byte depends on context.
  
  From section 7 of the standard: "Depending on context, it may be a
  signed integer, an unsigned integer, a floating-point constant, or
  anything else."

- **`Data2`**

  A two byte constant data value. How to interpret the bytes depends on context.
  
  These bytes have been converted from `R::Endian`. This may need to be reversed
  if this was not required.
  
  From section 7 of the standard: "Depending on context, it may be a
  signed integer, an unsigned integer, a floating-point constant, or
  anything else."

- **`Data4`**

  A four byte constant data value. How to interpret the bytes depends on context.
  
  These bytes have been converted from `R::Endian`. This may need to be reversed
  if this was not required.
  
  From section 7 of the standard: "Depending on context, it may be a
  signed integer, an unsigned integer, a floating-point constant, or
  anything else."

- **`Data8`**

  An eight byte constant data value. How to interpret the bytes depends on context.
  
  These bytes have been converted from `R::Endian`. This may need to be reversed
  if this was not required.
  
  From section 7 of the standard: "Depending on context, it may be a
  signed integer, an unsigned integer, a floating-point constant, or
  anything else."

- **`Data16`**

  An sixteen byte constant data value. How to interpret the bytes depends on context.
  
  These bytes have been converted from `R::Endian`. This may need to be reversed
  if this was not required.
  
  From section 7 of the standard: "Depending on context, it may be a
  signed integer, an unsigned integer, a floating-point constant, or
  anything else."

- **`Sdata`**

  A signed integer constant.

- **`Udata`**

  An unsigned integer constant.

- **`Exprloc`**

  "The information bytes contain a DWARF expression (see Section 2.5) or
  location description (see Section 2.6)."

- **`Flag`**

  A boolean that indicates presence or absence of the attribute.

- **`SecOffset`**

  An offset into another section. Which section this is an offset into
  depends on context.

- **`DebugAddrBase`**

  An offset to a set of addresses in the `.debug_addr` section.

- **`DebugAddrIndex`**

  An index into a set of addresses in the `.debug_addr` section.

- **`UnitRef`**

  An offset into the current compilation unit.

- **`DebugInfoRef`**

  An offset into the current `.debug_info` section, but possibly a
  different compilation unit from the current one.

- **`DebugInfoRefSup`**

  An offset into the `.debug_info` section of the supplementary object file.

- **`DebugLineRef`**

  An offset into the `.debug_line` section.

- **`LocationListsRef`**

  An offset into either the `.debug_loc` section or the `.debug_loclists` section.

- **`DebugLocListsBase`**

  An offset to a set of offsets in the `.debug_loclists` section.

- **`DebugLocListsIndex`**

  An index into a set of offsets in the `.debug_loclists` section.

- **`DebugMacinfoRef`**

  An offset into the `.debug_macinfo` section.

- **`DebugMacroRef`**

  An offset into the `.debug_macro` section.

- **`RangeListsRef`**

  An offset into the `.debug_ranges` section.

- **`DebugRngListsBase`**

  An offset to a set of offsets in the `.debug_rnglists` section.

- **`DebugRngListsIndex`**

  An index into a set of offsets in the `.debug_rnglists` section.

- **`DebugTypesRef`**

  A type signature.

- **`DebugStrRef`**

  An offset into the `.debug_str` section.

- **`DebugStrRefSup`**

  An offset into the `.debug_str` section of the supplementary object file.

- **`DebugStrOffsetsBase`**

  An offset to a set of entries in the `.debug_str_offsets` section.

- **`DebugStrOffsetsIndex`**

  An index into a set of entries in the `.debug_str_offsets` section.

- **`DebugLineStrRef`**

  An offset into the `.debug_line_str` section.

- **`String`**

  A slice of bytes representing a string. Does not include a final null byte.
  Not guaranteed to be UTF-8 or anything like that.

- **`Encoding`**

  The value of a `DW_AT_encoding` attribute.

- **`DecimalSign`**

  The value of a `DW_AT_decimal_sign` attribute.

- **`Endianity`**

  The value of a `DW_AT_endianity` attribute.

- **`Accessibility`**

  The value of a `DW_AT_accessibility` attribute.

- **`Visibility`**

  The value of a `DW_AT_visibility` attribute.

- **`Virtuality`**

  The value of a `DW_AT_virtuality` attribute.

- **`Language`**

  The value of a `DW_AT_language` attribute.

- **`AddressClass`**

  The value of a `DW_AT_address_class` attribute.

- **`IdentifierCase`**

  The value of a `DW_AT_identifier_case` attribute.

- **`CallingConvention`**

  The value of a `DW_AT_calling_convention` attribute.

- **`Inline`**

  The value of a `DW_AT_inline` attribute.

- **`Ordering`**

  The value of a `DW_AT_ordering` attribute.

- **`FileIndex`**

  An index into the filename entries from the line number information
  table for the compilation unit containing this value.

- **`DwoId`**

  An implementation-defined identifier uniquely identifying a compilation
  unit.

#### Implementations

- <span id="attributevalue-u8-value"></span>`fn u8_value(&self) -> Option<u8>`

  Try to convert this attribute's value to a u8.

- <span id="attributevalue-u16-value"></span>`fn u16_value(&self) -> Option<u16>`

  Try to convert this attribute's value to a u16.

- <span id="attributevalue-udata-value"></span>`fn udata_value(&self) -> Option<u64>`

  Try to convert this attribute's value to an unsigned integer.

- <span id="attributevalue-sdata-value"></span>`fn sdata_value(&self) -> Option<i64>`

  Try to convert this attribute's value to a signed integer.

- <span id="attributevalue-offset-value"></span>`fn offset_value(&self) -> Option<<R as >::Offset>` — [`Reader`](../index.md#reader)

  Try to convert this attribute's value to an offset.

- <span id="attributevalue-exprloc-value"></span>`fn exprloc_value(&self) -> Option<Expression<R>>` — [`Expression`](../index.md#expression)

  Try to convert this attribute's value to an expression or location buffer.

  

  Expressions and locations may be `DW_FORM_block*` or `DW_FORM_exprloc`.

  The standard doesn't mention `DW_FORM_block*` as a possible form, but

  it is encountered in practice.

- <span id="attributevalue-string-value"></span>`fn string_value(&self, debug_str: &DebugStr<R>) -> Option<R>` — [`DebugStr`](../index.md#debugstr)

  Try to return this attribute's value as a string slice.

  

  If this attribute's value is either an inline `DW_FORM_string` string,

  or a `DW_FORM_strp` reference to an offset into the `.debug_str`

  section, return the attribute's string value as `Some`. Other attribute

  value forms are returned as `None`.

  

  Warning: this function does not handle all possible string forms.

  Use `Dwarf::attr_string` instead.

- <span id="attributevalue-string-value-sup"></span>`fn string_value_sup(&self, debug_str: &DebugStr<R>, debug_str_sup: Option<&DebugStr<R>>) -> Option<R>` — [`DebugStr`](../index.md#debugstr)

  Try to return this attribute's value as a string slice.

  

  If this attribute's value is either an inline `DW_FORM_string` string,

  or a `DW_FORM_strp` reference to an offset into the `.debug_str`

  section, or a `DW_FORM_strp_sup` reference to an offset into a supplementary

  object file, return the attribute's string value as `Some`. Other attribute

  value forms are returned as `None`.

  

  Warning: this function does not handle all possible string forms.

  Use `Dwarf::attr_string` instead.

#### Trait Implementations

##### `impl<R, Offset> Clone for AttributeValue<R, Offset>`

- <span id="attributevalue-clone"></span>`fn clone(&self) -> AttributeValue<R, Offset>` — [`AttributeValue`](../index.md#attributevalue)

##### `impl<R, Offset> Copy for AttributeValue<R, Offset>`

##### `impl<R, Offset> Debug for AttributeValue<R, Offset>`

- <span id="attributevalue-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Offset> Eq for AttributeValue<R, Offset>`

##### `impl<R, Offset> PartialEq for AttributeValue<R, Offset>`

- <span id="attributevalue-partialeq-eq"></span>`fn eq(&self, other: &AttributeValue<R, Offset>) -> bool` — [`AttributeValue`](../index.md#attributevalue)

##### `impl<R, Offset> StructuralPartialEq for AttributeValue<R, Offset>`

## Functions

### `parse_unit_header`

```rust
fn parse_unit_header<R, Offset>(input: &mut R, section: crate::common::SectionId, unit_offset: crate::common::UnitSectionOffset<Offset>) -> crate::read::Result<UnitHeader<R>>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset
```

Parse a unit header.

### `allow_section_offset`

```rust
fn allow_section_offset(name: constants::DwAt, version: u16) -> bool
```

### `parse_attribute`

```rust
fn parse_attribute<R: Reader>(input: &mut R, encoding: crate::common::Encoding, spec: crate::read::AttributeSpecification) -> crate::read::Result<Attribute<R>>
```

### `skip_attributes`

```rust
fn skip_attributes<R: Reader>(input: &mut R, encoding: crate::common::Encoding, specs: &[crate::read::AttributeSpecification]) -> crate::read::Result<()>
```

