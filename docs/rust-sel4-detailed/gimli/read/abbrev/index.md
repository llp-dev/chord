*[gimli](../../index.md) / [read](../index.md) / [abbrev](index.md)*

---

# Module `abbrev`

Functions for parsing DWARF debugging abbreviations.

## Contents

- [Structs](#structs)
  - [`DebugAbbrev`](#debugabbrev)
  - [`AbbreviationsCache`](#abbreviationscache)
  - [`Abbreviations`](#abbreviations)
  - [`Abbreviation`](#abbreviation)
  - [`AttributeSpecification`](#attributespecification)
- [Enums](#enums)
  - [`AbbreviationsCacheStrategy`](#abbreviationscachestrategy)
  - [`Attributes`](#attributes)
- [Functions](#functions)
  - [`get_attribute_size`](#get-attribute-size)
- [Constants](#constants)
  - [`MAX_ATTRIBUTES_INLINE`](#max-attributes-inline)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DebugAbbrev`](#debugabbrev) | struct | The `DebugAbbrev` struct represents the abbreviations describing `DebuggingInformationEntry`s' attribute names and forms found in the `.debug_abbrev` section. |
| [`AbbreviationsCache`](#abbreviationscache) | struct | A cache of previously parsed `Abbreviations`. |
| [`Abbreviations`](#abbreviations) | struct | A set of type abbreviations. |
| [`Abbreviation`](#abbreviation) | struct | An abbreviation describes the shape of a `DebuggingInformationEntry`'s type: its code, tag type, whether it has children, and its set of attributes. |
| [`AttributeSpecification`](#attributespecification) | struct | The description of an attribute in an abbreviated type. |
| [`AbbreviationsCacheStrategy`](#abbreviationscachestrategy) | enum | The strategy to use for caching abbreviations. |
| [`Attributes`](#attributes) | enum | A list of attributes found in an `Abbreviation` |
| [`get_attribute_size`](#get-attribute-size) | fn |  |
| [`MAX_ATTRIBUTES_INLINE`](#max-attributes-inline) | const |  |

## Structs

### `DebugAbbrev<R>`

```rust
struct DebugAbbrev<R> {
    debug_abbrev_section: R,
}
```

The `DebugAbbrev` struct represents the abbreviations describing
`DebuggingInformationEntry`s' attribute names and forms found in the
`.debug_abbrev` section.

#### Implementations

- <span id="debugabbrev-new"></span>`fn new(debug_abbrev_section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugAbbrev` instance from the data in the `.debug_abbrev`

  section.

  

  It is the caller's responsibility to read the `.debug_abbrev` section and

  present it as a `&[u8]` slice. That means using some ELF loader on

  Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugAbbrev, LittleEndian};

  

  let buf = [0x00, 0x01, 0x02, 0x03];

  let read_debug_abbrev_section_somehow = || &buf;

  let debug_abbrev = DebugAbbrev::new(read_debug_abbrev_section_somehow(), LittleEndian);

  ```

#### Trait Implementations

##### `impl<R: clone::Clone> Clone for DebugAbbrev<R>`

- <span id="debugabbrev-clone"></span>`fn clone(&self) -> DebugAbbrev<R>` — [`DebugAbbrev`](../index.md#debugabbrev)

##### `impl<R: marker::Copy> Copy for DebugAbbrev<R>`

##### `impl<R: fmt::Debug> Debug for DebugAbbrev<R>`

- <span id="debugabbrev-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugAbbrev<R>`

- <span id="debugabbrev-default"></span>`fn default() -> DebugAbbrev<R>` — [`DebugAbbrev`](../index.md#debugabbrev)

##### `impl<R> Section for DebugAbbrev<R>`

- <span id="debugabbrev-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md#sectionid)

- <span id="debugabbrev-section-reader"></span>`fn reader(&self) -> &R`

### `AbbreviationsCache`

```rust
struct AbbreviationsCache {
    abbreviations: btree_map::BTreeMap<u64, crate::read::Result<alloc::sync::Arc<Abbreviations>>>,
}
```

A cache of previously parsed `Abbreviations`.

#### Implementations

- <span id="abbreviationscache-new"></span>`fn new() -> Self`

  Create an empty abbreviations cache.

- <span id="abbreviationscache-populate"></span>`fn populate<R: Reader>(&mut self, strategy: AbbreviationsCacheStrategy, debug_abbrev: &DebugAbbrev<R>, units: DebugInfoUnitHeadersIter<R>)` — [`AbbreviationsCacheStrategy`](../index.md#abbreviationscachestrategy), [`DebugAbbrev`](../index.md#debugabbrev), [`DebugInfoUnitHeadersIter`](../index.md#debuginfounitheadersiter)

  Parse abbreviations and store them in the cache.

  

  This will iterate over the given units to determine the abbreviations

  offsets. Any existing cache entries are discarded.

  

  Errors during parsing abbreviations are also stored in the cache.

  Errors during iterating over the units are ignored.

- <span id="abbreviationscache-set"></span>`fn set<R: Reader>(&mut self, offset: DebugAbbrevOffset<<R as >::Offset>, abbreviations: Arc<Abbreviations>)` — [`DebugAbbrevOffset`](../../index.md#debugabbrevoffset), [`Reader`](../index.md#reader), [`Abbreviations`](../index.md#abbreviations)

  Set an entry in the abbreviations cache.

  

  This is only required if you want to manually populate the cache.

- <span id="abbreviationscache-get"></span>`fn get<R: Reader>(&self, debug_abbrev: &DebugAbbrev<R>, offset: DebugAbbrevOffset<<R as >::Offset>) -> Result<Arc<Abbreviations>>` — [`DebugAbbrev`](../index.md#debugabbrev), [`DebugAbbrevOffset`](../../index.md#debugabbrevoffset), [`Reader`](../index.md#reader), [`Result`](../../index.md#result), [`Abbreviations`](../index.md#abbreviations)

  Parse the abbreviations at the given offset.

  

  This uses the cache if possible, but does not update it.

#### Trait Implementations

##### `impl Debug for AbbreviationsCache`

- <span id="abbreviationscache-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AbbreviationsCache`

- <span id="abbreviationscache-default"></span>`fn default() -> AbbreviationsCache` — [`AbbreviationsCache`](../index.md#abbreviationscache)

### `Abbreviations`

```rust
struct Abbreviations {
    vec: alloc::vec::Vec<Abbreviation>,
    map: btree_map::BTreeMap<u64, Abbreviation>,
}
```

A set of type abbreviations.

Construct an `Abbreviations` instance with the
[`abbreviations()`](#unitheader-abbreviations)
method.

#### Implementations

- <span id="abbreviations-empty"></span>`fn empty() -> Abbreviations` — [`Abbreviations`](../index.md#abbreviations)

  Construct a new, empty set of abbreviations.

- <span id="abbreviations-insert"></span>`fn insert(&mut self, abbrev: Abbreviation) -> ::core::result::Result<(), ()>` — [`Abbreviation`](../index.md#abbreviation)

  Insert an abbreviation into the set.

  

  Returns `Ok` if it is the first abbreviation in the set with its code,

  `Err` if the code is a duplicate and there already exists an

  abbreviation in the set with the given abbreviation's code.

- <span id="abbreviations-get"></span>`fn get(&self, code: u64) -> Option<&Abbreviation>` — [`Abbreviation`](../index.md#abbreviation)

  Get the abbreviation associated with the given code.

- <span id="abbreviations-parse"></span>`fn parse<R: Reader>(input: &mut R) -> Result<Abbreviations>` — [`Result`](../../index.md#result), [`Abbreviations`](../index.md#abbreviations)

  Parse a series of abbreviations, terminated by a null abbreviation.

#### Trait Implementations

##### `impl Clone for Abbreviations`

- <span id="abbreviations-clone"></span>`fn clone(&self) -> Abbreviations` — [`Abbreviations`](../index.md#abbreviations)

##### `impl Debug for Abbreviations`

- <span id="abbreviations-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Abbreviations`

- <span id="abbreviations-default"></span>`fn default() -> Abbreviations` — [`Abbreviations`](../index.md#abbreviations)

### `Abbreviation`

```rust
struct Abbreviation {
    code: u64,
    tag: constants::DwTag,
    has_children: constants::DwChildren,
    attributes: Attributes,
}
```

An abbreviation describes the shape of a `DebuggingInformationEntry`'s type:
its code, tag type, whether it has children, and its set of attributes.

#### Implementations

- <span id="abbreviation-new"></span>`fn new(code: u64, tag: constants::DwTag, has_children: constants::DwChildren, attributes: Attributes) -> Abbreviation` — [`DwTag`](../../index.md#dwtag), [`DwChildren`](../../index.md#dwchildren), [`Attributes`](#attributes), [`Abbreviation`](../index.md#abbreviation)

  Construct a new `Abbreviation`.

  

  ### Panics

  

  Panics if `code` is `0`.

- <span id="abbreviation-code"></span>`fn code(&self) -> u64`

  Get this abbreviation's code.

- <span id="abbreviation-tag"></span>`fn tag(&self) -> constants::DwTag` — [`DwTag`](../../index.md#dwtag)

  Get this abbreviation's tag.

- <span id="abbreviation-has-children"></span>`fn has_children(&self) -> bool`

  Return true if this abbreviation's type has children, false otherwise.

- <span id="abbreviation-attributes"></span>`fn attributes(&self) -> &[AttributeSpecification]` — [`AttributeSpecification`](../index.md#attributespecification)

  Get this abbreviation's attributes.

- <span id="abbreviation-parse-tag"></span>`fn parse_tag<R: Reader>(input: &mut R) -> Result<constants::DwTag>` — [`Result`](../../index.md#result), [`DwTag`](../../index.md#dwtag)

  Parse an abbreviation's tag.

- <span id="abbreviation-parse-has-children"></span>`fn parse_has_children<R: Reader>(input: &mut R) -> Result<constants::DwChildren>` — [`Result`](../../index.md#result), [`DwChildren`](../../index.md#dwchildren)

  Parse an abbreviation's "does the type have children?" byte.

- <span id="abbreviation-parse-attributes"></span>`fn parse_attributes<R: Reader>(input: &mut R) -> Result<Attributes>` — [`Result`](../../index.md#result), [`Attributes`](#attributes)

  Parse a series of attribute specifications, terminated by a null attribute

  specification.

- <span id="abbreviation-parse"></span>`fn parse<R: Reader>(input: &mut R) -> Result<Option<Abbreviation>>` — [`Result`](../../index.md#result), [`Abbreviation`](../index.md#abbreviation)

  Parse an abbreviation. Return `None` for the null abbreviation, `Some`

  for an actual abbreviation.

#### Trait Implementations

##### `impl Clone for Abbreviation`

- <span id="abbreviation-clone"></span>`fn clone(&self) -> Abbreviation` — [`Abbreviation`](../index.md#abbreviation)

##### `impl Debug for Abbreviation`

- <span id="abbreviation-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Abbreviation`

##### `impl PartialEq for Abbreviation`

- <span id="abbreviation-partialeq-eq"></span>`fn eq(&self, other: &Abbreviation) -> bool` — [`Abbreviation`](../index.md#abbreviation)

##### `impl StructuralPartialEq for Abbreviation`

### `AttributeSpecification`

```rust
struct AttributeSpecification {
    name: constants::DwAt,
    form: constants::DwForm,
    implicit_const_value: i64,
}
```

The description of an attribute in an abbreviated type. It is a pair of name
and form.

#### Implementations

- <span id="attributespecification-new"></span>`fn new(name: constants::DwAt, form: constants::DwForm, implicit_const_value: Option<i64>) -> AttributeSpecification` — [`DwAt`](../../index.md#dwat), [`DwForm`](../../index.md#dwform), [`AttributeSpecification`](../index.md#attributespecification)

  Construct a new `AttributeSpecification` from the given name and form

  and implicit const value.

- <span id="attributespecification-name"></span>`fn name(&self) -> constants::DwAt` — [`DwAt`](../../index.md#dwat)

  Get the attribute's name.

- <span id="attributespecification-form"></span>`fn form(&self) -> constants::DwForm` — [`DwForm`](../../index.md#dwform)

  Get the attribute's form.

- <span id="attributespecification-implicit-const-value"></span>`fn implicit_const_value(&self) -> Option<i64>`

  Get the attribute's implicit const value.

- <span id="attributespecification-size"></span>`fn size<R: Reader>(&self, header: &UnitHeader<R>) -> Option<usize>` — [`UnitHeader`](../index.md#unitheader)

  Return the size of the attribute, in bytes.

  

  Note that because some attributes are variably sized, the size cannot

  always be known without parsing, in which case we return `None`.

- <span id="attributespecification-parse"></span>`fn parse<R: Reader>(input: &mut R) -> Result<Option<AttributeSpecification>>` — [`Result`](../../index.md#result), [`AttributeSpecification`](../index.md#attributespecification)

  Parse an attribute specification. Returns `None` for the null attribute

  specification, `Some` for an actual attribute specification.

#### Trait Implementations

##### `impl Clone for AttributeSpecification`

- <span id="attributespecification-clone"></span>`fn clone(&self) -> AttributeSpecification` — [`AttributeSpecification`](../index.md#attributespecification)

##### `impl Copy for AttributeSpecification`

##### `impl Debug for AttributeSpecification`

- <span id="attributespecification-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for AttributeSpecification`

##### `impl FromIterator for Attributes`

- <span id="attributes-fromiterator-from-iter"></span>`fn from_iter<I>(iter: I) -> Attributes` — [`Attributes`](#attributes)

##### `impl PartialEq for AttributeSpecification`

- <span id="attributespecification-partialeq-eq"></span>`fn eq(&self, other: &AttributeSpecification) -> bool` — [`AttributeSpecification`](../index.md#attributespecification)

##### `impl StructuralPartialEq for AttributeSpecification`

## Enums

### `AbbreviationsCacheStrategy`

```rust
enum AbbreviationsCacheStrategy {
    Duplicates,
    All,
}
```

The strategy to use for caching abbreviations.

#### Variants

- **`Duplicates`**

  Cache abbreviations that are used more than once.
  
  This is useful if the units in the `.debug_info` section will be parsed only once.

- **`All`**

  Cache all abbreviations.
  
  This is useful if the units in the `.debug_info` section will be parsed more than once.

#### Trait Implementations

##### `impl Clone for AbbreviationsCacheStrategy`

- <span id="abbreviationscachestrategy-clone"></span>`fn clone(&self) -> AbbreviationsCacheStrategy` — [`AbbreviationsCacheStrategy`](../index.md#abbreviationscachestrategy)

##### `impl Copy for AbbreviationsCacheStrategy`

##### `impl Debug for AbbreviationsCacheStrategy`

- <span id="abbreviationscachestrategy-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for AbbreviationsCacheStrategy`

##### `impl PartialEq for AbbreviationsCacheStrategy`

- <span id="abbreviationscachestrategy-partialeq-eq"></span>`fn eq(&self, other: &AbbreviationsCacheStrategy) -> bool` — [`AbbreviationsCacheStrategy`](../index.md#abbreviationscachestrategy)

##### `impl StructuralPartialEq for AbbreviationsCacheStrategy`

### `Attributes`

```rust
enum Attributes {
    Inline {
        buf: [AttributeSpecification; 5],
        len: usize,
    },
    Heap(alloc::vec::Vec<AttributeSpecification>),
}
```

A list of attributes found in an `Abbreviation`

#### Implementations

- <span id="attributes-new"></span>`fn new() -> Attributes` — [`Attributes`](#attributes)

  Returns a new empty list of attributes

- <span id="attributes-push"></span>`fn push(&mut self, attr: AttributeSpecification)` — [`AttributeSpecification`](../index.md#attributespecification)

  Pushes a new value onto this list of attributes.

#### Trait Implementations

##### `impl Clone for Attributes`

- <span id="attributes-clone"></span>`fn clone(&self) -> Attributes` — [`Attributes`](#attributes)

##### `impl Debug for Attributes`

- <span id="attributes-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deref for Attributes`

- <span id="attributes-deref-type-target"></span>`type Target = [AttributeSpecification]`

- <span id="attributes-deref"></span>`fn deref(&self) -> &[AttributeSpecification]` — [`AttributeSpecification`](../index.md#attributespecification)

##### `impl Eq for Attributes`

##### `impl FromIterator for Attributes`

- <span id="attributes-fromiterator-from-iter"></span>`fn from_iter<I>(iter: I) -> Attributes` — [`Attributes`](#attributes)

##### `impl PartialEq for Attributes`

- <span id="attributes-partialeq-eq"></span>`fn eq(&self, other: &Attributes) -> bool` — [`Attributes`](#attributes)

##### `impl Receiver for Attributes`

- <span id="attributes-receiver-type-target"></span>`type Target = T`

## Functions

### `get_attribute_size`

```rust
fn get_attribute_size(form: constants::DwForm, encoding: crate::common::Encoding) -> Option<u8>
```

## Constants

### `MAX_ATTRIBUTES_INLINE`
```rust
const MAX_ATTRIBUTES_INLINE: usize = 5usize;
```

