**object > read > elf > attributes**

# Module: read::elf::attributes

## Contents

**Structs**

- [`AttributeIndexIterator`](#attributeindexiterator) - An iterator over the indices in an [`AttributesSubsubsection`].
- [`AttributeReader`](#attributereader) - A parser for the attributes in an [`AttributesSubsubsection`].
- [`AttributesSection`](#attributessection) - An ELF attributes section.
- [`AttributesSubsection`](#attributessubsection) - A subsection in an [`AttributesSection`].
- [`AttributesSubsectionIterator`](#attributessubsectioniterator) - An iterator for the subsections in an [`AttributesSection`].
- [`AttributesSubsubsection`](#attributessubsubsection) - A sub-subsection in an [`AttributesSubsection`].
- [`AttributesSubsubsectionIterator`](#attributessubsubsectioniterator) - An iterator for the sub-subsections in an [`AttributesSubsection`].

---

## object::read::elf::attributes::AttributeIndexIterator

*Struct*

An iterator over the indices in an [`AttributesSubsubsection`].

**Generic Parameters:**
- 'data

**Methods:**

- `fn next(self: & mut Self) -> Result<Option<u32>>` - Parse the next index.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> AttributeIndexIterator<'data>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## object::read::elf::attributes::AttributeReader

*Struct*

A parser for the attributes in an [`AttributesSubsubsection`].

The parser relies on the caller to know the format of the data for each attribute tag.

**Generic Parameters:**
- 'data

**Methods:**

- `fn read_tag(self: & mut Self) -> Result<Option<u64>>` - Parse a tag.
- `fn read_integer(self: & mut Self) -> Result<u64>` - Parse an integer value.
- `fn read_string(self: & mut Self) -> Result<&'data [u8]>` - Parse a string value.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> AttributeReader<'data>`



## object::read::elf::attributes::AttributesSection

*Struct*

An ELF attributes section.

This may be a GNU attributes section, or an architecture specific attributes section.

An attributes section contains a series of [`AttributesSubsection`].

Returned by [`SectionHeader::attributes`](super::SectionHeader::attributes)
and [`SectionHeader::gnu_attributes`](super::SectionHeader::gnu_attributes).

**Generic Parameters:**
- 'data
- Elf

**Methods:**

- `fn new(endian: <Elf as >::Endian, data: &'data [u8]) -> Result<Self>` - Parse an ELF attributes section given the section data.
- `fn version(self: &Self) -> u8` - Return the version of the attributes section.
- `fn subsections(self: &Self) -> Result<AttributesSubsectionIterator<'data, Elf>>` - Return an iterator over the subsections.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> AttributesSection<'data, Elf>`



## object::read::elf::attributes::AttributesSubsection

*Struct*

A subsection in an [`AttributesSection`].

A subsection is identified by a vendor name.  It contains a series of
[`AttributesSubsubsection`].

**Generic Parameters:**
- 'data
- Elf

**Methods:**

- `fn length(self: &Self) -> u32` - Return the length of the attributes subsection.
- `fn vendor(self: &Self) -> &'data [u8]` - Return the vendor name of the attributes subsection.
- `fn subsubsections(self: &Self) -> AttributesSubsubsectionIterator<'data, Elf>` - Return an iterator over the sub-subsections.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> AttributesSubsection<'data, Elf>`



## object::read::elf::attributes::AttributesSubsectionIterator

*Struct*

An iterator for the subsections in an [`AttributesSection`].

**Generic Parameters:**
- 'data
- Elf

**Methods:**

- `fn next(self: & mut Self) -> Result<Option<AttributesSubsection<'data, Elf>>>` - Return the next subsection.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> AttributesSubsectionIterator<'data, Elf>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## object::read::elf::attributes::AttributesSubsubsection

*Struct*

A sub-subsection in an [`AttributesSubsection`].

A sub-subsection is identified by a tag.  It contains an optional series of indices,
followed by a series of attributes.

**Generic Parameters:**
- 'data

**Methods:**

- `fn tag(self: &Self) -> u8` - Return the tag of the attributes sub-subsection.
- `fn length(self: &Self) -> u32` - Return the length of the attributes sub-subsection.
- `fn indices_data(self: &Self) -> &'data [u8]` - Return the data containing the indices.
- `fn indices(self: &Self) -> AttributeIndexIterator<'data>` - Return the indices.
- `fn attributes_data(self: &Self) -> &'data [u8]` - Return the data containing the attributes.
- `fn attributes(self: &Self) -> AttributeReader<'data>` - Return a parser for the data containing the attributes.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> AttributesSubsubsection<'data>`



## object::read::elf::attributes::AttributesSubsubsectionIterator

*Struct*

An iterator for the sub-subsections in an [`AttributesSubsection`].

**Generic Parameters:**
- 'data
- Elf

**Methods:**

- `fn next(self: & mut Self) -> Result<Option<AttributesSubsubsection<'data>>>` - Return the next sub-subsection.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> AttributesSubsubsectionIterator<'data, Elf>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



