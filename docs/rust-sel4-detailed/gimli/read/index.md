*[gimli](../index.md) / [read](index.md)*

---

# Module `read`

Read DWARF debugging information.

* [Example Usage](#example-usage)
* [API Structure](#api-structure)

## Example Usage

Print out all of the functions in the debuggee program:

```rust,no_run
fn example() -> Result<(), gimli::Error> {
type R = gimli::EndianSlice<'static, gimli::LittleEndian>;
let get_file_section_reader = |name| -> Result<R, gimli::Error> { unimplemented!() };
let get_sup_file_section_reader = |name| -> Result<R, gimli::Error> { unimplemented!() };
// Read the DWARF sections with whatever object loader you're using.
// These closures should return a `Reader` instance (e.g. `EndianSlice`).
let loader = |section: gimli::SectionId| { get_file_section_reader(section.name()) };
let sup_loader = |section: gimli::SectionId| { get_sup_file_section_reader(section.name()) };
let mut dwarf = gimli::Dwarf::load(loader)?;
dwarf.load_sup(sup_loader)?;

// Iterate over all compilation units.
let mut iter = dwarf.units();
while let Some(header) = iter.next()? {
    // Parse the abbreviations and other information for this compilation unit.
    let unit = dwarf.unit(header)?;

    // Iterate over all of this compilation unit's entries.
    let mut entries = unit.entries();
    while let Some(entry) = entries.next_dfs()? {
        // If we find an entry for a function, print it.
        if entry.tag() == gimli::DW_TAG_subprogram {
            println!("Found a function: {:?}", entry);
        }
    }
}
unreachable!()
}
```

Full example programs:

  * [A simple `.debug_info` parser](https://github.com/gimli-rs/gimli/blob/master/crates/examples/src/bin/simple.rs)

  * [A simple `.debug_line` parser](https://github.com/gimli-rs/gimli/blob/master/crates/examples/src/bin/simple_line.rs)

  * [A `dwarfdump`
    clone](https://github.com/gimli-rs/gimli/blob/master/crates/examples/src/bin/dwarfdump.rs)

  * [An `addr2line` clone](https://github.com/gimli-rs/addr2line)

  * [`ddbug`](https://github.com/gimli-rs/ddbug), a utility giving insight into
    code generation by making debugging information readable

  * [`dwprod`](https://github.com/fitzgen/dwprod), a tiny utility to list the
    compilers used to create each compilation unit within a shared library or
    executable (via `DW_AT_producer`)

  * [`dwarf-validate`](https://github.com/gimli-rs/gimli/blob/master/crates/examples/src/bin/dwarf-validate.rs),
    a program to validate the integrity of some DWARF and its references
    between sections and compilation units.

## API Structure

* Basic familiarity with DWARF is assumed.

* The [`Dwarf`](./struct.Dwarf.html) type contains the commonly used DWARF
  sections. It has methods that simplify access to debugging data that spans
  multiple sections. Use of this type is optional, but recommended.

* The [`DwarfPackage`](./struct.Dwarf.html) type contains the DWARF
  package (DWP) sections. It has methods to find a DWARF object (DWO)
  within the package.

* Each section gets its own type. Consider these types the entry points to
  the library:

  * [`DebugAbbrev`](./struct.DebugAbbrev.html): The `.debug_abbrev` section.

  * [`DebugAddr`](./struct.DebugAddr.html): The `.debug_addr` section.

  * [`DebugAranges`](./struct.DebugAranges.html): The `.debug_aranges`
    section.

  * [`DebugFrame`](./struct.DebugFrame.html): The `.debug_frame` section.

  * [`DebugInfo`](./struct.DebugInfo.html): The `.debug_info` section.

  * [`DebugLine`](./struct.DebugLine.html): The `.debug_line` section.

  * [`DebugLineStr`](./struct.DebugLineStr.html): The `.debug_line_str` section.

  * [`DebugLoc`](./struct.DebugLoc.html): The `.debug_loc` section.

  * [`DebugLocLists`](./struct.DebugLocLists.html): The `.debug_loclists` section.

  * [`DebugNames`](./struct.DebugNames.html): The `.debug_names` section.

  * [`DebugPubNames`](./struct.DebugPubNames.html): The `.debug_pubnames`
    section.

  * [`DebugPubTypes`](./struct.DebugPubTypes.html): The `.debug_pubtypes`
    section.

  * [`DebugRanges`](./struct.DebugRanges.html): The `.debug_ranges` section.

  * [`DebugRngLists`](./struct.DebugRngLists.html): The `.debug_rnglists` section.

  * [`DebugStr`](./struct.DebugStr.html): The `.debug_str` section.

  * [`DebugStrOffsets`](./struct.DebugStrOffsets.html): The `.debug_str_offsets` section.

  * [`DebugTypes`](./struct.DebugTypes.html): The `.debug_types` section.

  * [`DebugCuIndex`](./struct.DebugCuIndex.html): The `.debug_cu_index` section.

  * [`DebugTuIndex`](./struct.DebugTuIndex.html): The `.debug_tu_index` section.

  * [`EhFrame`](./struct.EhFrame.html): The `.eh_frame` section.

  * [`EhFrameHdr`](./struct.EhFrameHdr.html): The `.eh_frame_hdr` section.

* Each section type exposes methods for accessing the debugging data encoded
  in that section. For example, the [`DebugInfo`](./struct.DebugInfo.html)
  struct has the [`units`](./struct.DebugInfo.html#method.units) method for
  iterating over the compilation units defined within it.

* Offsets into a section are strongly typed: an offset into `.debug_info` is
  the [`DebugInfoOffset`](./struct.DebugInfoOffset.html) type. It cannot be
  used to index into the [`DebugLine`](./struct.DebugLine.html) type because
  `DebugLine` represents the `.debug_line` section. There are similar types
  for offsets relative to a compilation unit rather than a section.

## Contents

- [Modules](#modules)
  - [`util`](#util)
  - [`addr`](#addr)
  - [`cfi`](#cfi)
  - [`dwarf`](#dwarf)
  - [`endian_slice`](#endian-slice)
  - [`endian_reader`](#endian-reader)
  - [`reader`](#reader)
  - [`relocate`](#relocate)
  - [`abbrev`](#abbrev)
  - [`aranges`](#aranges)
  - [`index`](#index)
  - [`line`](#line)
  - [`lists`](#lists)
  - [`loclists`](#loclists)
  - [`lookup`](#lookup)
  - [`macros`](#macros)
  - [`names`](#names)
  - [`op`](#op)
  - [`pubnames`](#pubnames)
  - [`pubtypes`](#pubtypes)
  - [`rnglists`](#rnglists)
  - [`str`](#str)
  - [`unit`](#unit)
  - [`value`](#value)
  - [`sealed`](#sealed)
- [Structs](#structs)
  - [`UnitOffset`](#unitoffset)
  - [`StoreOnHeap`](#storeonheap)
  - [`ArrayVec`](#arrayvec)
  - [`DebugAddr`](#debugaddr)
  - [`AddrHeaderIter`](#addrheaderiter)
  - [`AddrHeader`](#addrheader)
  - [`AddrEntryIter`](#addrentryiter)
  - [`DebugFrame`](#debugframe)
  - [`EhFrameHdr`](#ehframehdr)
  - [`ParsedEhFrameHdr`](#parsedehframehdr)
  - [`EhHdrTableIter`](#ehhdrtableiter)
  - [`EhHdrTable`](#ehhdrtable)
  - [`EhFrame`](#ehframe)
  - [`BaseAddresses`](#baseaddresses)
  - [`SectionBaseAddresses`](#sectionbaseaddresses)
  - [`CfiEntriesIter`](#cfientriesiter)
  - [`CfiEntryPrefix`](#cfientryprefix)
  - [`Augmentation`](#augmentation)
  - [`AugmentationData`](#augmentationdata)
  - [`CommonInformationEntry`](#commoninformationentry)
  - [`PartialFrameDescriptionEntry`](#partialframedescriptionentry)
  - [`FrameDescriptionEntry`](#framedescriptionentry)
  - [`UnwindContext`](#unwindcontext)
  - [`UnwindTable`](#unwindtable)
  - [`RegisterRuleMap`](#registerrulemap)
  - [`RegisterRuleIter`](#registerruleiter)
  - [`UnwindTableRow`](#unwindtablerow)
  - [`CallFrameInstructionIter`](#callframeinstructioniter)
  - [`UnwindExpression`](#unwindexpression)
  - [`PointerEncodingParameters`](#pointerencodingparameters)
  - [`DwarfSections`](#dwarfsections)
  - [`Dwarf`](#dwarf)
  - [`DwarfPackageSections`](#dwarfpackagesections)
  - [`DwarfPackage`](#dwarfpackage)
  - [`Unit`](#unit)
  - [`UnitRef`](#unitref)
  - [`RangeIter`](#rangeiter)
  - [`EndianSlice`](#endianslice)
  - [`DebugBytes`](#debugbytes)
  - [`DebugByte`](#debugbyte)
  - [`DebugLen`](#debuglen)
  - [`EndianReader`](#endianreader)
  - [`SubRange`](#subrange)
  - [`ReaderOffsetId`](#readeroffsetid)
  - [`RelocateReader`](#relocatereader)
  - [`DebugAbbrev`](#debugabbrev)
  - [`AbbreviationsCache`](#abbreviationscache)
  - [`Abbreviations`](#abbreviations)
  - [`Abbreviation`](#abbreviation)
  - [`AttributeSpecification`](#attributespecification)
  - [`DebugAranges`](#debugaranges)
  - [`ArangeHeaderIter`](#arangeheaderiter)
  - [`ArangeHeader`](#arangeheader)
  - [`ArangeEntryIter`](#arangeentryiter)
  - [`ArangeEntry`](#arangeentry)
  - [`DebugCuIndex`](#debugcuindex)
  - [`DebugTuIndex`](#debugtuindex)
  - [`UnitIndex`](#unitindex)
  - [`UnitIndexSectionIterator`](#unitindexsectioniterator)
  - [`UnitIndexSection`](#unitindexsection)
  - [`DebugLine`](#debugline)
  - [`LineRows`](#linerows)
  - [`LineInstructions`](#lineinstructions)
  - [`LineRow`](#linerow)
  - [`LineSequence`](#linesequence)
  - [`LineProgramHeader`](#lineprogramheader)
  - [`IncompleteLineProgram`](#incompletelineprogram)
  - [`CompleteLineProgram`](#completelineprogram)
  - [`FileEntry`](#fileentry)
  - [`FileEntryFormat`](#fileentryformat)
  - [`DebugLoc`](#debugloc)
  - [`DebugLocLists`](#debugloclists)
  - [`LocationLists`](#locationlists)
  - [`RawLocListIter`](#rawloclistiter)
  - [`LocListIter`](#loclistiter)
  - [`LocationListEntry`](#locationlistentry)
  - [`DebugMacinfo`](#debugmacinfo)
  - [`DebugMacro`](#debugmacro)
  - [`MacroUnitHeader`](#macrounitheader)
  - [`MacroIter`](#macroiter)
  - [`DebugNames`](#debugnames)
  - [`NameIndexHeaderIter`](#nameindexheaderiter)
  - [`NameIndexHeader`](#nameindexheader)
  - [`NameTableIndex`](#nametableindex)
  - [`NameIndex`](#nameindex)
  - [`NameTableIter`](#nametableiter)
  - [`NameBucketIter`](#namebucketiter)
  - [`NameHashIter`](#namehashiter)
  - [`NameEntryIter`](#nameentryiter)
  - [`NameEntryOffset`](#nameentryoffset)
  - [`NameEntry`](#nameentry)
  - [`NameAttribute`](#nameattribute)
  - [`NameAbbreviations`](#nameabbreviations)
  - [`NameAbbreviation`](#nameabbreviation)
  - [`NameAbbreviationAttribute`](#nameabbreviationattribute)
  - [`Piece`](#piece)
  - [`Expression`](#expression)
  - [`OperationIter`](#operationiter)
  - [`Evaluation`](#evaluation)
  - [`PubNamesEntry`](#pubnamesentry)
  - [`DebugPubNames`](#debugpubnames)
  - [`PubNamesEntryIter`](#pubnamesentryiter)
  - [`PubTypesEntry`](#pubtypesentry)
  - [`DebugPubTypes`](#debugpubtypes)
  - [`PubTypesEntryIter`](#pubtypesentryiter)
  - [`DebugRanges`](#debugranges)
  - [`DebugRngLists`](#debugrnglists)
  - [`RangeLists`](#rangelists)
  - [`RawRngListIter`](#rawrnglistiter)
  - [`RngListIter`](#rnglistiter)
  - [`RawRange`](#rawrange)
  - [`Range`](#range)
  - [`DebugStr`](#debugstr)
  - [`DebugStrOffsets`](#debugstroffsets)
  - [`DebugLineStr`](#debuglinestr)
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
  - [`Error`](#error)
  - [`CieOrFde`](#cieorfde)
  - [`CfaRule`](#cfarule)
  - [`RegisterRule`](#registerrule)
  - [`CallFrameInstruction`](#callframeinstruction)
  - [`Pointer`](#pointer)
  - [`RangeIterInner`](#rangeiterinner)
  - [`AbbreviationsCacheStrategy`](#abbreviationscachestrategy)
  - [`Attributes`](#attributes)
  - [`IndexSectionId`](#indexsectionid)
  - [`LineInstruction`](#lineinstruction)
  - [`ColumnType`](#columntype)
  - [`LocListsFormat`](#loclistsformat)
  - [`RawLocListEntry`](#rawloclistentry)
  - [`MacroString`](#macrostring)
  - [`MacroEntry`](#macroentry)
  - [`NameTypeUnit`](#nametypeunit)
  - [`NameAttributeValue`](#nameattributevalue)
  - [`DieReference`](#diereference)
  - [`Operation`](#operation)
  - [`OperationEvaluationResult`](#operationevaluationresult)
  - [`Location`](#location)
  - [`EvaluationState`](#evaluationstate)
  - [`EvaluationWaiting`](#evaluationwaiting)
  - [`EvaluationResult`](#evaluationresult)
  - [`RangeListsFormat`](#rangelistsformat)
  - [`RawRngListEntry`](#rawrnglistentry)
  - [`UnitType`](#unittype)
  - [`AttributeValue`](#attributevalue)
  - [`ValueType`](#valuetype)
  - [`Value`](#value)
- [Traits](#traits)
  - [`Section`](#section)
  - [`ArrayLike`](#arraylike)
  - [`UnwindOffset`](#unwindoffset)
  - [`UnwindSection`](#unwindsection)
  - [`UnwindContextStorage`](#unwindcontextstorage)
  - [`ReaderOffset`](#readeroffset)
  - [`ReaderAddress`](#readeraddress)
  - [`Reader`](#reader)
  - [`Relocate`](#relocate)
  - [`LineProgram`](#lineprogram)
  - [`EvaluationStorage`](#evaluationstorage)
- [Functions](#functions)
  - [`parse_cfi_entry`](#parse-cfi-entry)
  - [`parse_cfi_entry_prefix`](#parse-cfi-entry-prefix)
  - [`parse_encoded_pointer`](#parse-encoded-pointer)
  - [`parse_encoded_value`](#parse-encoded-value)
  - [`get_attribute_size`](#get-attribute-size)
  - [`parse_directory_v5`](#parse-directory-v5)
  - [`parse_file_v5`](#parse-file-v5)
  - [`parse_attribute`](#parse-attribute)
  - [`parse_data`](#parse-data)
  - [`read_debug_names_form_value`](#read-debug-names-form-value)
  - [`compute_pc`](#compute-pc)
  - [`generic_type`](#generic-type)
  - [`parse_unit_header`](#parse-unit-header)
  - [`allow_section_offset`](#allow-section-offset)
  - [`parse_attribute`](#parse-attribute)
  - [`skip_attributes`](#skip-attributes)
  - [`sign_extend`](#sign-extend)
  - [`mask_bit_size`](#mask-bit-size)
- [Type Aliases](#type-aliases)
  - [`Result`](#result)
  - [`EndianRcSlice`](#endianrcslice)
  - [`EndianArcSlice`](#endianarcslice)
  - [`OneShotLineRows`](#oneshotlinerows)
  - [`ResumedLineRows`](#resumedlinerows)
  - [`LocListsHeader`](#loclistsheader)
  - [`RngListsHeader`](#rnglistsheader)
- [Constants](#constants)
  - [`MAX_RULES`](#max-rules)
  - [`MAX_UNWIND_STACK_DEPTH`](#max-unwind-stack-depth)
  - [`CFI_INSTRUCTION_HIGH_BITS_MASK`](#cfi-instruction-high-bits-mask)
  - [`CFI_INSTRUCTION_LOW_BITS_MASK`](#cfi-instruction-low-bits-mask)
  - [`MAX_ATTRIBUTES_INLINE`](#max-attributes-inline)
  - [`SECTION_COUNT_MAX`](#section-count-max)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`util`](#util) | mod |  |
| [`addr`](#addr) | mod |  |
| [`cfi`](#cfi) | mod |  |
| [`dwarf`](#dwarf) | mod |  |
| [`endian_slice`](#endian-slice) | mod | Working with byte slices that have an associated endianity. |
| [`endian_reader`](#endian-reader) | mod | Defining custom `Reader`s quickly. |
| [`reader`](#reader) | mod |  |
| [`relocate`](#relocate) | mod |  |
| [`abbrev`](#abbrev) | mod | Functions for parsing DWARF debugging abbreviations. |
| [`aranges`](#aranges) | mod |  |
| [`index`](#index) | mod |  |
| [`line`](#line) | mod |  |
| [`lists`](#lists) | mod |  |
| [`loclists`](#loclists) | mod |  |
| [`lookup`](#lookup) | mod |  |
| [`macros`](#macros) | mod |  |
| [`names`](#names) | mod | Functions for parsing DWARF 5 `.debug_names` sections. |
| [`op`](#op) | mod | Functions for parsing and evaluating DWARF expressions. |
| [`pubnames`](#pubnames) | mod |  |
| [`pubtypes`](#pubtypes) | mod |  |
| [`rnglists`](#rnglists) | mod |  |
| [`str`](#str) | mod |  |
| [`unit`](#unit) | mod | Functions for parsing DWARF `.debug_info` and `.debug_types` sections. |
| [`value`](#value) | mod | Definitions for values used in DWARF expressions. |
| [`sealed`](#sealed) | mod |  |
| [`UnitOffset`](#unitoffset) | struct | An offset into the current compilation or type unit. |
| [`StoreOnHeap`](#storeonheap) | struct | Indicates that storage should be allocated on heap. |
| [`ArrayVec`](#arrayvec) | struct |  |
| [`DebugAddr`](#debugaddr) | struct | The raw contents of the `.debug_addr` section. |
| [`AddrHeaderIter`](#addrheaderiter) | struct | An iterator over the headers of a `.debug_addr` section. |
| [`AddrHeader`](#addrheader) | struct | A header for a set of entries in the `.debug_addr` section. |
| [`AddrEntryIter`](#addrentryiter) | struct | An iterator over the addresses from a `.debug_addr` section. |
| [`DebugFrame`](#debugframe) | struct | `DebugFrame` contains the `.debug_frame` section's frame unwinding information required to unwind to and recover registers from older frames on the stack. |
| [`EhFrameHdr`](#ehframehdr) | struct | `EhFrameHdr` contains the information about the `.eh_frame_hdr` section. |
| [`ParsedEhFrameHdr`](#parsedehframehdr) | struct | `ParsedEhFrameHdr` contains the parsed information from the `.eh_frame_hdr` section. |
| [`EhHdrTableIter`](#ehhdrtableiter) | struct | An iterator for `.eh_frame_hdr` section's binary search table. |
| [`EhHdrTable`](#ehhdrtable) | struct | The CFI binary search table that is an optional part of the `.eh_frame_hdr` section. |
| [`EhFrame`](#ehframe) | struct | `EhFrame` contains the frame unwinding information needed during exception handling found in the `.eh_frame` section. |
| [`BaseAddresses`](#baseaddresses) | struct | Optional base addresses for the relative `DW_EH_PE_*` encoded pointers. |
| [`SectionBaseAddresses`](#sectionbaseaddresses) | struct | Optional base addresses for the relative `DW_EH_PE_*` encoded pointers in a particular section. |
| [`CfiEntriesIter`](#cfientriesiter) | struct | An iterator over CIE and FDE entries in a `.debug_frame` or `.eh_frame` section. |
| [`CfiEntryPrefix`](#cfientryprefix) | struct | The common prefix of a CIE or FDE. |
| [`Augmentation`](#augmentation) | struct | We support the z-style augmentation [defined by `.eh_frame`][ehframe]. |
| [`AugmentationData`](#augmentationdata) | struct | Parsed augmentation data for a `FrameDescriptEntry`. |
| [`CommonInformationEntry`](#commoninformationentry) | struct | > A Common Information Entry holds information that is shared among many > Frame Description Entries. |
| [`PartialFrameDescriptionEntry`](#partialframedescriptionentry) | struct | A partially parsed `FrameDescriptionEntry`. |
| [`FrameDescriptionEntry`](#framedescriptionentry) | struct | A `FrameDescriptionEntry` is a set of CFA instructions for an address range. |
| [`UnwindContext`](#unwindcontext) | struct | Common context needed when evaluating the call frame unwinding information. |
| [`UnwindTable`](#unwindtable) | struct | The `UnwindTable` iteratively evaluates a `FrameDescriptionEntry`'s `CallFrameInstruction` program, yielding the each row one at a time. |
| [`RegisterRuleMap`](#registerrulemap) | struct |  |
| [`RegisterRuleIter`](#registerruleiter) | struct | An unordered iterator for register rules. |
| [`UnwindTableRow`](#unwindtablerow) | struct | A row in the virtual unwind table that describes how to find the values of the registers in the *previous* frame for a range of PC addresses. |
| [`CallFrameInstructionIter`](#callframeinstructioniter) | struct | A lazy iterator parsing call frame instructions. |
| [`UnwindExpression`](#unwindexpression) | struct | The location of a DWARF expression within an unwind section. |
| [`PointerEncodingParameters`](#pointerencodingparameters) | struct |  |
| [`DwarfSections`](#dwarfsections) | struct | All of the commonly used DWARF sections. |
| [`Dwarf`](#dwarf) | struct | All of the commonly used DWARF sections, and other common information. |
| [`DwarfPackageSections`](#dwarfpackagesections) | struct | The sections from a `.dwp` file. |
| [`DwarfPackage`](#dwarfpackage) | struct | The sections from a `.dwp` file, with parsed indices. |
| [`Unit`](#unit) | struct | All of the commonly used information for a unit in the `.debug_info` or `.debug_types` sections. |
| [`UnitRef`](#unitref) | struct | A reference to a `Unit` and its associated `Dwarf`. |
| [`RangeIter`](#rangeiter) | struct | An iterator for the address ranges of a `DebuggingInformationEntry`. |
| [`EndianSlice`](#endianslice) | struct | A `&[u8]` slice with endianity metadata. |
| [`DebugBytes`](#debugbytes) | struct |  |
| [`DebugByte`](#debugbyte) | struct |  |
| [`DebugLen`](#debuglen) | struct |  |
| [`EndianReader`](#endianreader) | struct | An easy way to define a custom `Reader` implementation with a reference to a generic buffer of bytes and an associated endianity. |
| [`SubRange`](#subrange) | struct |  |
| [`ReaderOffsetId`](#readeroffsetid) | struct | An identifier for an offset within a section reader. |
| [`RelocateReader`](#relocatereader) | struct | A `Reader` which applies relocations to addresses and offsets. |
| [`DebugAbbrev`](#debugabbrev) | struct | The `DebugAbbrev` struct represents the abbreviations describing `DebuggingInformationEntry`s' attribute names and forms found in the `.debug_abbrev` section. |
| [`AbbreviationsCache`](#abbreviationscache) | struct | A cache of previously parsed `Abbreviations`. |
| [`Abbreviations`](#abbreviations) | struct | A set of type abbreviations. |
| [`Abbreviation`](#abbreviation) | struct | An abbreviation describes the shape of a `DebuggingInformationEntry`'s type: its code, tag type, whether it has children, and its set of attributes. |
| [`AttributeSpecification`](#attributespecification) | struct | The description of an attribute in an abbreviated type. |
| [`DebugAranges`](#debugaranges) | struct | The `DebugAranges` struct represents the DWARF address range information found in the `.debug_aranges` section. |
| [`ArangeHeaderIter`](#arangeheaderiter) | struct | An iterator over the headers of a `.debug_aranges` section. |
| [`ArangeHeader`](#arangeheader) | struct | A header for a set of entries in the `.debug_arange` section. |
| [`ArangeEntryIter`](#arangeentryiter) | struct | An iterator over the aranges from a `.debug_aranges` section. |
| [`ArangeEntry`](#arangeentry) | struct | A single parsed arange. |
| [`DebugCuIndex`](#debugcuindex) | struct | The data in the `.debug_cu_index` section of a `.dwp` file. |
| [`DebugTuIndex`](#debugtuindex) | struct | The data in the `.debug_tu_index` section of a `.dwp` file. |
| [`UnitIndex`](#unitindex) | struct | The partially parsed index from a `DebugCuIndex` or `DebugTuIndex`. |
| [`UnitIndexSectionIterator`](#unitindexsectioniterator) | struct | An iterator over the section offsets and sizes for a row in a `UnitIndex`. |
| [`UnitIndexSection`](#unitindexsection) | struct | Information about a unit's contribution to a section in a `.dwp` file. |
| [`DebugLine`](#debugline) | struct | The `DebugLine` struct contains the source location to instruction mapping found in the `.debug_line` section. |
| [`LineRows`](#linerows) | struct | Executes a `LineProgram` to iterate over the rows in the matrix of line number information. |
| [`LineInstructions`](#lineinstructions) | struct | An iterator yielding parsed instructions. |
| [`LineRow`](#linerow) | struct | A row in the line number program's resulting matrix. |
| [`LineSequence`](#linesequence) | struct | A sequence within a line number program. |
| [`LineProgramHeader`](#lineprogramheader) | struct | A header for a line number program in the `.debug_line` section, as defined in section 6.2.4 of the standard. |
| [`IncompleteLineProgram`](#incompletelineprogram) | struct | A line number program that has not been run to completion. |
| [`CompleteLineProgram`](#completelineprogram) | struct | A line number program that has previously been run to completion. |
| [`FileEntry`](#fileentry) | struct | An entry in the `LineProgramHeader`'s `file_names` set. |
| [`FileEntryFormat`](#fileentryformat) | struct | The format of a component of an include directory or file name entry. |
| [`DebugLoc`](#debugloc) | struct | The raw contents of the `.debug_loc` section. |
| [`DebugLocLists`](#debugloclists) | struct | The `DebugLocLists` struct represents the DWARF data found in the `.debug_loclists` section. |
| [`LocationLists`](#locationlists) | struct | The DWARF data found in `.debug_loc` and `.debug_loclists` sections. |
| [`RawLocListIter`](#rawloclistiter) | struct | A raw iterator over a location list. |
| [`LocListIter`](#loclistiter) | struct | An iterator over a location list. |
| [`LocationListEntry`](#locationlistentry) | struct | A location list entry from the `.debug_loc` or `.debug_loclists` sections. |
| [`DebugMacinfo`](#debugmacinfo) | struct | The raw contents of the `.debug_macinfo` section. |
| [`DebugMacro`](#debugmacro) | struct | The raw contents of the `.debug_macro` section. |
| [`MacroUnitHeader`](#macrounitheader) | struct |  |
| [`MacroIter`](#macroiter) | struct | Iterator over the entries in the `.debug_macro` section. |
| [`DebugNames`](#debugnames) | struct | The `DebugNames` struct represents the DWARF 5 name index information found in the `.debug_names` section. |
| [`NameIndexHeaderIter`](#nameindexheaderiter) | struct | An iterator over the name index headers in the `.debug_names` section. |
| [`NameIndexHeader`](#nameindexheader) | struct | The header of a name index in the `.debug_names` section. |
| [`NameTableIndex`](#nametableindex) | struct | An index into the name table of a `NameIndex`. |
| [`NameIndex`](#nameindex) | struct | A single name index from the `.debug_names` section. |
| [`NameTableIter`](#nametableiter) | struct | An iterator over the indexes of all names in a name index. |
| [`NameBucketIter`](#namebucketiter) | struct | An iterator over the hash entries for a bucket in a name index hash table. |
| [`NameHashIter`](#namehashiter) | struct | An iterator over the indexes of the names in a name index hash table that match a hash value. |
| [`NameEntryIter`](#nameentryiter) | struct | An iterator for a series of name entries in a name index entry pool. |
| [`NameEntryOffset`](#nameentryoffset) | struct | An offset into the entry pool of a name index. |
| [`NameEntry`](#nameentry) | struct | A parsed entry from the `.debug_names` section. |
| [`NameAttribute`](#nameattribute) | struct | A parsed attribute for a [`NameEntry`]. |
| [`NameAbbreviations`](#nameabbreviations) | struct | A table of name entry abbreviations. |
| [`NameAbbreviation`](#nameabbreviation) | struct | A name abbreviation entry defines how name entries are encoded. |
| [`NameAbbreviationAttribute`](#nameabbreviationattribute) | struct | An attribute specification in a name abbreviation. |
| [`Piece`](#piece) | struct | The description of a single piece of the result of a DWARF expression. |
| [`Expression`](#expression) | struct | The bytecode for a DWARF expression or location description. |
| [`OperationIter`](#operationiter) | struct | An iterator for the operations in an expression. |
| [`Evaluation`](#evaluation) | struct | A DWARF expression evaluator. |
| [`PubNamesEntry`](#pubnamesentry) | struct | A single parsed pubname. |
| [`DebugPubNames`](#debugpubnames) | struct | The `DebugPubNames` struct represents the DWARF public names information found in the `.debug_pubnames` section. |
| [`PubNamesEntryIter`](#pubnamesentryiter) | struct | An iterator over the pubnames from a `.debug_pubnames` section. |
| [`PubTypesEntry`](#pubtypesentry) | struct | A single parsed pubtype. |
| [`DebugPubTypes`](#debugpubtypes) | struct | The `DebugPubTypes` struct represents the DWARF public types information found in the `.debug_info` section. |
| [`PubTypesEntryIter`](#pubtypesentryiter) | struct | An iterator over the pubtypes from a `.debug_pubtypes` section. |
| [`DebugRanges`](#debugranges) | struct | The raw contents of the `.debug_ranges` section. |
| [`DebugRngLists`](#debugrnglists) | struct | The `DebugRngLists` struct represents the contents of the `.debug_rnglists` section. |
| [`RangeLists`](#rangelists) | struct | The DWARF data found in `.debug_ranges` and `.debug_rnglists` sections. |
| [`RawRngListIter`](#rawrnglistiter) | struct | A raw iterator over an address range list. |
| [`RngListIter`](#rnglistiter) | struct | An iterator over an address range list. |
| [`RawRange`](#rawrange) | struct | A raw address range from the `.debug_ranges` section. |
| [`Range`](#range) | struct | An address range from the `.debug_ranges`, `.debug_rnglists`, or `.debug_aranges` sections. |
| [`DebugStr`](#debugstr) | struct | The `DebugStr` struct represents the DWARF strings found in the `.debug_str` section. |
| [`DebugStrOffsets`](#debugstroffsets) | struct | The raw contents of the `.debug_str_offsets` section. |
| [`DebugLineStr`](#debuglinestr) | struct | The `DebugLineStr` struct represents the DWARF strings found in the `.debug_line_str` section. |
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
| [`Error`](#error) | enum | An error that occurred when parsing. |
| [`CieOrFde`](#cieorfde) | enum | Either a `CommonInformationEntry` (CIE) or a `FrameDescriptionEntry` (FDE). |
| [`CfaRule`](#cfarule) | enum | The canonical frame address (CFA) recovery rules. |
| [`RegisterRule`](#registerrule) | enum | An entry in the abstract CFI table that describes how to find the value of a register. |
| [`CallFrameInstruction`](#callframeinstruction) | enum | A parsed call frame instruction. |
| [`Pointer`](#pointer) | enum | A decoded pointer. |
| [`RangeIterInner`](#rangeiterinner) | enum |  |
| [`AbbreviationsCacheStrategy`](#abbreviationscachestrategy) | enum | The strategy to use for caching abbreviations. |
| [`Attributes`](#attributes) | enum | A list of attributes found in an `Abbreviation` |
| [`IndexSectionId`](#indexsectionid) | enum | Section kinds which are permitted in a `.dwp` index. |
| [`LineInstruction`](#lineinstruction) | enum | A parsed line number program instruction. |
| [`ColumnType`](#columntype) | enum | The type of column that a row is referring to. |
| [`LocListsFormat`](#loclistsformat) | enum |  |
| [`RawLocListEntry`](#rawloclistentry) | enum | A raw entry in .debug_loclists. |
| [`MacroString`](#macrostring) | enum | A string in a macro entry. |
| [`MacroEntry`](#macroentry) | enum | an Entry in the `.debug_macro` section. |
| [`NameTypeUnit`](#nametypeunit) | enum | A reference to a type unit. |
| [`NameAttributeValue`](#nameattributevalue) | enum | A parsed attribute value for a [`NameEntry`]. |
| [`DieReference`](#diereference) | enum | A reference to a DIE, either relative to the current CU or relative to the section. |
| [`Operation`](#operation) | enum | A single decoded DWARF expression operation. |
| [`OperationEvaluationResult`](#operationevaluationresult) | enum |  |
| [`Location`](#location) | enum | A single location of a piece of the result of a DWARF expression. |
| [`EvaluationState`](#evaluationstate) | enum |  |
| [`EvaluationWaiting`](#evaluationwaiting) | enum |  |
| [`EvaluationResult`](#evaluationresult) | enum | The state of an `Evaluation` after evaluating a DWARF expression. |
| [`RangeListsFormat`](#rangelistsformat) | enum |  |
| [`RawRngListEntry`](#rawrnglistentry) | enum | A raw entry in .debug_rnglists |
| [`UnitType`](#unittype) | enum | This enum specifies the type of the unit and any type specific data carried in the header (e.g. the type signature/type offset of a type unit). |
| [`AttributeValue`](#attributevalue) | enum | The value of an attribute in a `DebuggingInformationEntry`. |
| [`ValueType`](#valuetype) | enum | The type of an entry on the DWARF stack. |
| [`Value`](#value) | enum | The value of an entry on the DWARF stack. |
| [`Section`](#section) | trait | A convenience trait for loading DWARF sections from object files. |
| [`ArrayLike`](#arraylike) | trait | Marker trait for types that can be used as backing storage when a growable array type is needed. |
| [`UnwindOffset`](#unwindoffset) | trait | An offset into an `UnwindSection`. |
| [`UnwindSection`](#unwindsection) | trait | A section holding unwind information: either `.debug_frame` or `.eh_frame`. |
| [`UnwindContextStorage`](#unwindcontextstorage) | trait | Specification of what storage should be used for [`UnwindContext`]. |
| [`ReaderOffset`](#readeroffset) | trait | A trait for offsets with a DWARF section. |
| [`ReaderAddress`](#readeraddress) | trait | A trait for addresses within a DWARF section. |
| [`Reader`](#reader) | trait | A trait for reading the data from a DWARF section. |
| [`Relocate`](#relocate) | trait | Trait for relocating addresses and offsets while reading a section. |
| [`LineProgram`](#lineprogram) | trait | A `LineProgram` provides access to a `LineProgramHeader` and a way to add files to the files table if necessary. |
| [`EvaluationStorage`](#evaluationstorage) | trait | Specification of what storage should be used for [`Evaluation`]. |
| [`parse_cfi_entry`](#parse-cfi-entry) | fn |  |
| [`parse_cfi_entry_prefix`](#parse-cfi-entry-prefix) | fn |  |
| [`parse_encoded_pointer`](#parse-encoded-pointer) | fn |  |
| [`parse_encoded_value`](#parse-encoded-value) | fn |  |
| [`get_attribute_size`](#get-attribute-size) | fn |  |
| [`parse_directory_v5`](#parse-directory-v5) | fn |  |
| [`parse_file_v5`](#parse-file-v5) | fn |  |
| [`parse_attribute`](#parse-attribute) | fn |  |
| [`parse_data`](#parse-data) | fn |  |
| [`read_debug_names_form_value`](#read-debug-names-form-value) | fn | Read an attribute value. |
| [`compute_pc`](#compute-pc) | fn |  |
| [`generic_type`](#generic-type) | fn |  |
| [`parse_unit_header`](#parse-unit-header) | fn | Parse a unit header. |
| [`allow_section_offset`](#allow-section-offset) | fn |  |
| [`parse_attribute`](#parse-attribute) | fn |  |
| [`skip_attributes`](#skip-attributes) | fn |  |
| [`sign_extend`](#sign-extend) | fn | Convert a u64 to an i64, with sign extension if required. |
| [`mask_bit_size`](#mask-bit-size) | fn |  |
| [`Result`](#result) | type | The result of a parse. |
| [`EndianRcSlice`](#endianrcslice) | type | A reference counted, non-thread-safe slice of bytes and associated endianity. |
| [`EndianArcSlice`](#endianarcslice) | type | An atomically reference counted, thread-safe slice of bytes and associated endianity. |
| [`OneShotLineRows`](#oneshotlinerows) | type |  |
| [`ResumedLineRows`](#resumedlinerows) | type |  |
| [`LocListsHeader`](#loclistsheader) | type |  |
| [`RngListsHeader`](#rnglistsheader) | type |  |
| [`MAX_RULES`](#max-rules) | const |  |
| [`MAX_UNWIND_STACK_DEPTH`](#max-unwind-stack-depth) | const |  |
| [`CFI_INSTRUCTION_HIGH_BITS_MASK`](#cfi-instruction-high-bits-mask) | const |  |
| [`CFI_INSTRUCTION_LOW_BITS_MASK`](#cfi-instruction-low-bits-mask) | const |  |
| [`MAX_ATTRIBUTES_INLINE`](#max-attributes-inline) | const |  |
| [`SECTION_COUNT_MAX`](#section-count-max) | const |  |

## Modules

- [`util`](util/index.md)
- [`addr`](addr/index.md)
- [`cfi`](cfi/index.md)
- [`dwarf`](dwarf/index.md)
- [`endian_slice`](endian_slice/index.md) — Working with byte slices that have an associated endianity.
- [`endian_reader`](endian_reader/index.md) — Defining custom `Reader`s quickly.
- [`reader`](reader/index.md)
- [`relocate`](relocate/index.md)
- [`abbrev`](abbrev/index.md) — Functions for parsing DWARF debugging abbreviations.
- [`aranges`](aranges/index.md)
- [`index`](index/index.md)
- [`line`](line/index.md)
- [`lists`](lists/index.md)
- [`loclists`](loclists/index.md)
- [`lookup`](lookup/index.md)
- [`macros`](macros/index.md)
- [`names`](names/index.md) — Functions for parsing DWARF 5 `.debug_names` sections.
- [`op`](op/index.md) — Functions for parsing and evaluating DWARF expressions.
- [`pubnames`](pubnames/index.md)
- [`pubtypes`](pubtypes/index.md)
- [`rnglists`](rnglists/index.md)
- [`str`](str/index.md)
- [`unit`](unit/index.md) — Functions for parsing DWARF `.debug_info` and `.debug_types` sections.
- [`value`](value/index.md) — Definitions for values used in DWARF expressions.
- [`sealed`](sealed/index.md)

## Structs

### `UnitOffset<T>`

```rust
struct UnitOffset<T>(T);
```

An offset into the current compilation or type unit.

#### Implementations

- <span id="cratereadunitoffset-is-in-bounds"></span>`fn is_in_bounds<R>(&self, unit: &UnitHeader<R>) -> bool` — [`UnitHeader`](#unitheader)

  Return true if this offset is within the entries of the given unit.

  

  This only checks that the offset is within the range of the data for unit entries,

  not that there is a valid DIE at this offset.

- <span id="cratereadunitoffset-to-unit-section-offset"></span>`fn to_unit_section_offset<R>(&self, unit: &UnitHeader<R>) -> UnitSectionOffset<T>` — [`UnitHeader`](#unitheader), [`UnitSectionOffset`](../index.md#unitsectionoffset)

  Convert an offset to be relative to the start of the section containing the unit,

  instead of relative to the start of the given unit.

  

  Does not check that the offset is in bounds or valid. This is because often you

  will know that the offset came from this unit so it must be valid.

  For offsets of uncertain origin, use `UnitOffset::is_in_bounds` first.

- <span id="cratereadunitoffset-to-debug-info-offset"></span>`fn to_debug_info_offset<R>(&self, unit: &UnitHeader<R>) -> Option<DebugInfoOffset<T>>` — [`UnitHeader`](#unitheader), [`DebugInfoOffset`](../index.md#debuginfooffset)

  Convert an offset to be relative to the start of the `.debug_info` section,

  instead of relative to the start of the given unit.

  

  Returns `None` if the unit is not within the `.debug_info` section.

- <span id="cratereadunitoffset-to-debug-types-offset"></span>`fn to_debug_types_offset<R>(&self, unit: &UnitHeader<R>) -> Option<DebugTypesOffset<T>>` — [`UnitHeader`](#unitheader), [`DebugTypesOffset`](../index.md#debugtypesoffset)

  Convert an offset to be relative to the start of the `.debug_types` section,

  instead of relative to the start of the given unit.

  

  Returns `None` if the unit is not within the `.debug_types` section.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for UnitOffset<T>`

- <span id="unitoffset-clone"></span>`fn clone(&self) -> UnitOffset<T>` — [`UnitOffset`](../index.md#unitoffset)

##### `impl<T: marker::Copy> Copy for UnitOffset<T>`

##### `impl<T: fmt::Debug> Debug for UnitOffset<T>`

- <span id="unitoffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for UnitOffset<T>`

##### `impl<T: hash::Hash> Hash for UnitOffset<T>`

- <span id="unitoffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T: cmp::Ord> Ord for UnitOffset<T>`

- <span id="unitoffset-ord-cmp"></span>`fn cmp(&self, other: &UnitOffset<T>) -> cmp::Ordering` — [`UnitOffset`](../index.md#unitoffset)

##### `impl<T: cmp::PartialEq> PartialEq for UnitOffset<T>`

- <span id="unitoffset-partialeq-eq"></span>`fn eq(&self, other: &UnitOffset<T>) -> bool` — [`UnitOffset`](../index.md#unitoffset)

##### `impl<T: cmp::PartialOrd> PartialOrd for UnitOffset<T>`

- <span id="unitoffset-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &UnitOffset<T>) -> option::Option<cmp::Ordering>` — [`UnitOffset`](../index.md#unitoffset)

##### `impl<T> StructuralPartialEq for UnitOffset<T>`

### `StoreOnHeap`

```rust
struct StoreOnHeap;
```

Indicates that storage should be allocated on heap.

#### Trait Implementations

##### `impl Clone for StoreOnHeap`

- <span id="storeonheap-clone"></span>`fn clone(&self) -> StoreOnHeap` — [`StoreOnHeap`](../index.md#storeonheap)

##### `impl Copy for StoreOnHeap`

##### `impl Debug for StoreOnHeap`

- <span id="storeonheap-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for StoreOnHeap`

##### `impl<R: Reader> EvaluationStorage for crate::read::StoreOnHeap`

- <span id="cratereadstoreonheap-evaluationstorage-type-stack"></span>`type Stack = Vec<Value>`

- <span id="cratereadstoreonheap-evaluationstorage-type-expressionstack"></span>`type ExpressionStack = Vec<(R, R)>`

- <span id="cratereadstoreonheap-evaluationstorage-type-result"></span>`type Result = Vec<Piece<R>>`

##### `impl PartialEq for StoreOnHeap`

- <span id="storeonheap-partialeq-eq"></span>`fn eq(&self, other: &StoreOnHeap) -> bool` — [`StoreOnHeap`](../index.md#storeonheap)

##### `impl StructuralPartialEq for StoreOnHeap`

##### `impl<T: ReaderOffset> UnwindContextStorage for crate::read::StoreOnHeap`

- <span id="cratereadstoreonheap-unwindcontextstorage-type-rules"></span>`type Rules = [(Register, RegisterRule<T>); 192]`

- <span id="cratereadstoreonheap-unwindcontextstorage-type-stack"></span>`type Stack = Box<[UnwindTableRow<T>; 4]>`

### `ArrayVec<A: ArrayLike>`

```rust
struct ArrayVec<A: ArrayLike> {
    storage: <A as >::Storage,
    len: usize,
}
```

#### Implementations

- <span id="arrayvec-new"></span>`fn new() -> Self`

- <span id="arrayvec-clear"></span>`fn clear(&mut self)`

- <span id="arrayvec-try-push"></span>`fn try_push(&mut self, value: <A as >::Item) -> Result<(), CapacityFull>` — [`ArrayLike`](#arraylike), [`CapacityFull`](util/sealed/index.md#capacityfull)

- <span id="arrayvec-try-insert"></span>`fn try_insert(&mut self, index: usize, element: <A as >::Item) -> Result<(), CapacityFull>` — [`ArrayLike`](#arraylike), [`CapacityFull`](util/sealed/index.md#capacityfull)

- <span id="arrayvec-pop"></span>`fn pop(&mut self) -> Option<<A as >::Item>` — [`ArrayLike`](#arraylike)

- <span id="arrayvec-swap-remove"></span>`fn swap_remove(&mut self, index: usize) -> <A as >::Item` — [`ArrayLike`](#arraylike)

#### Trait Implementations

##### `impl<A: ArrayLike> Clone for ArrayVec<A>`

- <span id="arrayvec-clone"></span>`fn clone(&self) -> Self`

##### `impl<A: ArrayLike> Debug for ArrayVec<A>`

- <span id="arrayvec-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<A: ArrayLike> Default for ArrayVec<A>`

- <span id="arrayvec-default"></span>`fn default() -> Self`

##### `impl<A: ArrayLike> Deref for ArrayVec<A>`

- <span id="arrayvec-deref-type-target"></span>`type Target = [<A as ArrayLike>::Item]`

- <span id="arrayvec-deref"></span>`fn deref(&self) -> &[<A as >::Item]` — [`ArrayLike`](#arraylike)

##### `impl<A: ArrayLike> DerefMut for ArrayVec<A>`

- <span id="arrayvec-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut [<A as >::Item]` — [`ArrayLike`](#arraylike)

##### `impl<A: ArrayLike> Drop for ArrayVec<A>`

- <span id="arrayvec-drop"></span>`fn drop(&mut self)`

##### `impl<A: ArrayLike> Eq for ArrayVec<A>`

##### `impl<A: ArrayLike> PartialEq for ArrayVec<A>`

- <span id="arrayvec-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl Receiver for ArrayVec<A>`

- <span id="arrayvec-receiver-type-target"></span>`type Target = T`

### `DebugAddr<R>`

```rust
struct DebugAddr<R> {
    section: R,
}
```

The raw contents of the `.debug_addr` section.

#### Implementations

- <span id="debugaddr-get-address"></span>`fn get_address(&self, address_size: u8, base: DebugAddrBase<<R as >::Offset>, index: DebugAddrIndex<<R as >::Offset>) -> Result<u64>` — [`DebugAddrBase`](../index.md#debugaddrbase), [`Reader`](#reader), [`DebugAddrIndex`](../index.md#debugaddrindex), [`Result`](../index.md#result)

  Returns the address at the given `base` and `index`.

  

  A set of addresses in the `.debug_addr` section consists of a header

  followed by a series of addresses.

  

  The `base` must be the `DW_AT_addr_base` value from the compilation unit DIE.

  This is an offset that points to the first address following the header.

  

  The `index` is the value of a `DW_FORM_addrx` attribute.

  

  The `address_size` must be the size of the address for the compilation unit.

  This value must also match the header. However, note that we do not parse the

  header to validate this, since locating the header is unreliable, and the GNU

  extensions do not emit it.

- <span id="debugaddr-headers"></span>`fn headers(&self) -> AddrHeaderIter<R>` — [`AddrHeaderIter`](#addrheaderiter)

  Iterate the sets of entries in the `.debug_addr` section.

  

  Each set of entries belongs to a single unit.

#### Trait Implementations

##### `impl<R: clone::Clone> Clone for DebugAddr<R>`

- <span id="debugaddr-clone"></span>`fn clone(&self) -> DebugAddr<R>` — [`DebugAddr`](#debugaddr)

##### `impl<R: marker::Copy> Copy for DebugAddr<R>`

##### `impl<R: fmt::Debug> Debug for DebugAddr<R>`

- <span id="debugaddr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugAddr<R>`

- <span id="debugaddr-default"></span>`fn default() -> DebugAddr<R>` — [`DebugAddr`](#debugaddr)

##### `impl<R> Section for DebugAddr<R>`

- <span id="debugaddr-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../index.md#sectionid)

- <span id="debugaddr-section-reader"></span>`fn reader(&self) -> &R`

### `AddrHeaderIter<R: Reader>`

```rust
struct AddrHeaderIter<R: Reader> {
    input: R,
    offset: crate::common::DebugAddrOffset<<R as >::Offset>,
}
```

An iterator over the headers of a `.debug_addr` section.

#### Implementations

- <span id="addrheaderiter-next"></span>`fn next(&mut self) -> Result<Option<AddrHeader<R>>>` — [`Result`](../index.md#result), [`AddrHeader`](#addrheader)

  Advance the iterator to the next header.

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for AddrHeaderIter<R>`

- <span id="addrheaderiter-clone"></span>`fn clone(&self) -> AddrHeaderIter<R>` — [`AddrHeaderIter`](#addrheaderiter)

##### `impl<R: fmt::Debug + Reader> Debug for AddrHeaderIter<R>`

- <span id="addrheaderiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for AddrHeaderIter<R>`

- <span id="addrheaderiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="addrheaderiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="addrheaderiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: Reader> Iterator for AddrHeaderIter<R>`

- <span id="addrheaderiter-iterator-type-item"></span>`type Item = Result<AddrHeader<R>, Error>`

- <span id="addrheaderiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `AddrHeader<R, Offset>`

```rust
struct AddrHeader<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    offset: crate::common::DebugAddrOffset<Offset>,
    encoding: crate::common::Encoding,
    length: Offset,
    entries: R,
}
```

A header for a set of entries in the `.debug_addr` section.

These entries all belong to a single unit.

#### Implementations

- <span id="addrheader-parse"></span>`fn parse(input: &mut R, offset: DebugAddrOffset<Offset>) -> Result<Self>` — [`DebugAddrOffset`](../index.md#debugaddroffset), [`Result`](../index.md#result)

- <span id="addrheader-offset"></span>`fn offset(&self) -> DebugAddrOffset<Offset>` — [`DebugAddrOffset`](../index.md#debugaddroffset)

  Return the offset of this header within the `.debug_addr` section.

- <span id="addrheader-length"></span>`fn length(&self) -> Offset`

  Return the length of this set of entries, including the header.

- <span id="addrheader-encoding"></span>`fn encoding(&self) -> Encoding` — [`Encoding`](../index.md#encoding)

  Return the encoding parameters for this set of entries.

- <span id="addrheader-entries"></span>`fn entries(&self) -> AddrEntryIter<R>` — [`AddrEntryIter`](#addrentryiter)

  Return the address entries in this set.

#### Trait Implementations

##### `impl<R, Offset> Clone for AddrHeader<R, Offset>`

- <span id="addrheader-clone"></span>`fn clone(&self) -> AddrHeader<R, Offset>` — [`AddrHeader`](#addrheader)

##### `impl<R, Offset> Debug for AddrHeader<R, Offset>`

- <span id="addrheader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Offset> Eq for AddrHeader<R, Offset>`

##### `impl<R, Offset> PartialEq for AddrHeader<R, Offset>`

- <span id="addrheader-partialeq-eq"></span>`fn eq(&self, other: &AddrHeader<R, Offset>) -> bool` — [`AddrHeader`](#addrheader)

##### `impl<R, Offset> StructuralPartialEq for AddrHeader<R, Offset>`

### `AddrEntryIter<R: Reader>`

```rust
struct AddrEntryIter<R: Reader> {
    input: R,
    encoding: crate::common::Encoding,
}
```

An iterator over the addresses from a `.debug_addr` section.

#### Implementations

- <span id="addrentryiter-next"></span>`fn next(&mut self) -> Result<Option<u64>>` — [`Result`](../index.md#result)

  Advance the iterator and return the next address.

  

  Returns the newly parsed address as `Ok(Some(addr))`. Returns `Ok(None)`

  when iteration is complete and all addresses have already been parsed and

  yielded. If an error occurs while parsing the next address, then this error

  is returned as `Err(e)`, and all subsequent calls return `Ok(None)`.

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for AddrEntryIter<R>`

- <span id="addrentryiter-clone"></span>`fn clone(&self) -> AddrEntryIter<R>` — [`AddrEntryIter`](#addrentryiter)

##### `impl<R: fmt::Debug + Reader> Debug for AddrEntryIter<R>`

- <span id="addrentryiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for AddrEntryIter<R>`

- <span id="addrentryiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="addrentryiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="addrentryiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: Reader> Iterator for AddrEntryIter<R>`

- <span id="addrentryiter-iterator-type-item"></span>`type Item = Result<u64, Error>`

- <span id="addrentryiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `DebugFrame<R: Reader>`

```rust
struct DebugFrame<R: Reader> {
    section: R,
    address_size: u8,
    vendor: crate::common::Vendor,
}
```

`DebugFrame` contains the `.debug_frame` section's frame unwinding
information required to unwind to and recover registers from older frames on
the stack. For example, this is useful for a debugger that wants to print
locals in a backtrace.

Most interesting methods are defined in the
[`UnwindSection`](#unwindsection) trait.

### Differences between `.debug_frame` and `.eh_frame`

While the `.debug_frame` section's information has a lot of overlap with the
`.eh_frame` section's information, the `.eh_frame` information tends to only
encode the subset of information needed for exception handling. Often, only
one of `.eh_frame` or `.debug_frame` will be present in an object file.

#### Implementations

- <span id="debugframe-set-address-size"></span>`fn set_address_size(&mut self, address_size: u8)`

  Set the size of a target address in bytes.

  

  This defaults to the native word size.

  This is only used if the CIE version is less than 4.

- <span id="debugframe-set-vendor"></span>`fn set_vendor(&mut self, vendor: Vendor)` — [`Vendor`](../index.md#vendor)

  Set the vendor extensions to use.

  

  This defaults to `Vendor::Default`.

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for DebugFrame<R>`

- <span id="debugframe-clone"></span>`fn clone(&self) -> DebugFrame<R>` — [`DebugFrame`](#debugframe)

##### `impl<R: marker::Copy + Reader> Copy for DebugFrame<R>`

##### `impl<R: fmt::Debug + Reader> Debug for DebugFrame<R>`

- <span id="debugframe-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: cmp::Eq + Reader> Eq for DebugFrame<R>`

##### `impl<R: cmp::PartialEq + Reader> PartialEq for DebugFrame<R>`

- <span id="debugframe-partialeq-eq"></span>`fn eq(&self, other: &DebugFrame<R>) -> bool` — [`DebugFrame`](#debugframe)

##### `impl<R: Reader> Section for DebugFrame<R>`

- <span id="debugframe-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../index.md#sectionid)

- <span id="debugframe-section-reader"></span>`fn reader(&self) -> &R`

##### `impl<R: Reader> StructuralPartialEq for DebugFrame<R>`

##### `impl<R: Reader> UnwindSection for DebugFrame<R>`

- <span id="debugframe-unwindsection-type-offset"></span>`type Offset = DebugFrameOffset<<R as Reader>::Offset>`

### `EhFrameHdr<R: Reader>`

```rust
struct EhFrameHdr<R: Reader>(R);
```

`EhFrameHdr` contains the information about the `.eh_frame_hdr` section.

A pointer to the start of the `.eh_frame` data, and optionally, a binary
search table of pointers to the `.eh_frame` records that are found in this section.

#### Implementations

- <span id="ehframehdr-new"></span>`fn new(section: &'input [u8], endian: Endian) -> Self`

  Constructs a new `EhFrameHdr` instance from the data in the `.eh_frame_hdr` section.

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for EhFrameHdr<R>`

- <span id="ehframehdr-clone"></span>`fn clone(&self) -> EhFrameHdr<R>` — [`EhFrameHdr`](#ehframehdr)

##### `impl<R: marker::Copy + Reader> Copy for EhFrameHdr<R>`

##### `impl<R: fmt::Debug + Reader> Debug for EhFrameHdr<R>`

- <span id="ehframehdr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: cmp::Eq + Reader> Eq for EhFrameHdr<R>`

##### `impl<R: cmp::PartialEq + Reader> PartialEq for EhFrameHdr<R>`

- <span id="ehframehdr-partialeq-eq"></span>`fn eq(&self, other: &EhFrameHdr<R>) -> bool` — [`EhFrameHdr`](#ehframehdr)

##### `impl<R: Reader> Section for EhFrameHdr<R>`

- <span id="ehframehdr-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../index.md#sectionid)

- <span id="ehframehdr-section-reader"></span>`fn reader(&self) -> &R`

##### `impl<R: Reader> StructuralPartialEq for EhFrameHdr<R>`

### `ParsedEhFrameHdr<R: Reader>`

```rust
struct ParsedEhFrameHdr<R: Reader> {
    address_size: u8,
    section: R,
    eh_frame_ptr: Pointer,
    fde_count: u64,
    table_enc: crate::constants::DwEhPe,
    table: R,
}
```

`ParsedEhFrameHdr` contains the parsed information from the `.eh_frame_hdr` section.

#### Implementations

- <span id="parsedehframehdr-eh-frame-ptr"></span>`fn eh_frame_ptr(&self) -> Pointer` — [`Pointer`](#pointer)

  Returns the address of the binary's `.eh_frame` section.

- <span id="parsedehframehdr-table"></span>`fn table(&self) -> Option<EhHdrTable<'_, R>>` — [`EhHdrTable`](#ehhdrtable)

  Retrieves the CFI binary search table, if there is one.

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for ParsedEhFrameHdr<R>`

- <span id="parsedehframehdr-clone"></span>`fn clone(&self) -> ParsedEhFrameHdr<R>` — [`ParsedEhFrameHdr`](#parsedehframehdr)

##### `impl<R: fmt::Debug + Reader> Debug for ParsedEhFrameHdr<R>`

- <span id="parsedehframehdr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `EhHdrTableIter<'a, 'bases, R: Reader>`

```rust
struct EhHdrTableIter<'a, 'bases, R: Reader> {
    hdr: &'a ParsedEhFrameHdr<R>,
    table: R,
    bases: &'bases BaseAddresses,
    remain: u64,
}
```

An iterator for `.eh_frame_hdr` section's binary search table.

Each table entry consists of a tuple containing an  `initial_location` and `address`.
The `initial location` represents the first address that the targeted FDE
is able to decode. The `address` is the address of the FDE in the `.eh_frame` section.
The `address` can be converted with `EhHdrTable::pointer_to_offset` and `EhFrame::fde_from_offset` to an FDE.

#### Implementations

- <span id="ehhdrtableiter-next"></span>`fn next(&mut self) -> Result<Option<(Pointer, Pointer)>>` — [`Result`](../index.md#result), [`Pointer`](#pointer)

  Yield the next entry in the `EhHdrTableIter`.

- <span id="ehhdrtableiter-nth"></span>`fn nth(&mut self, n: usize) -> Result<Option<(Pointer, Pointer)>>` — [`Result`](../index.md#result), [`Pointer`](#pointer)

  Yield the nth entry in the `EhHdrTableIter`

#### Trait Implementations

##### `impl<R: fmt::Debug + Reader> Debug for EhHdrTableIter<'a, 'bases, R>`

- <span id="ehhdrtableiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for EhHdrTableIter<'a, 'bases, R>`

- <span id="ehhdrtableiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="ehhdrtableiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="ehhdrtableiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: Reader> Iterator for EhHdrTableIter<'a, 'bases, R>`

- <span id="ehhdrtableiter-iterator-type-item"></span>`type Item = Result<(Pointer, Pointer), Error>`

- <span id="ehhdrtableiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="ehhdrtableiter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="ehhdrtableiter-iterator-nth"></span>`fn nth(&mut self, n: usize) -> Option<<Self as >::Item>`

### `EhHdrTable<'a, R: Reader>`

```rust
struct EhHdrTable<'a, R: Reader> {
    hdr: &'a ParsedEhFrameHdr<R>,
}
```

The CFI binary search table that is an optional part of the `.eh_frame_hdr` section.

#### Implementations

- <span id="ehhdrtable-iter"></span>`fn iter<'bases>(&self, bases: &'bases BaseAddresses) -> EhHdrTableIter<'_, 'bases, R>` — [`BaseAddresses`](#baseaddresses), [`EhHdrTableIter`](#ehhdrtableiter)

  Return an iterator that can walk the `.eh_frame_hdr` table.

  

  Each table entry consists of a tuple containing an `initial_location` and `address`.

  The `initial location` represents the first address that the targeted FDE

  is able to decode. The `address` is the address of the FDE in the `.eh_frame` section.

  The `address` can be converted with `EhHdrTable::pointer_to_offset` and `EhFrame::fde_from_offset` to an FDE.

- <span id="ehhdrtable-lookup"></span>`fn lookup(&self, address: u64, bases: &BaseAddresses) -> Result<Pointer>` — [`BaseAddresses`](#baseaddresses), [`Result`](../index.md#result), [`Pointer`](#pointer)

  *Probably* returns a pointer to the FDE for the given address.

  

  This performs a binary search, so if there is no FDE for the given address,

  this function **will** return a pointer to any other FDE that's close by.

  

  To be sure, you **must** call `contains` on the FDE.

- <span id="ehhdrtable-pointer-to-offset"></span>`fn pointer_to_offset(&self, ptr: Pointer) -> Result<EhFrameOffset<<R as >::Offset>>` — [`Pointer`](#pointer), [`Result`](../index.md#result), [`EhFrameOffset`](../index.md#ehframeoffset), [`Reader`](#reader)

  Convert a `Pointer` to a section offset.

  

  This does not support indirect pointers.

- <span id="ehhdrtable-fde-for-address"></span>`fn fde_for_address<F>(&self, frame: &EhFrame<R>, bases: &BaseAddresses, address: u64, get_cie: F) -> Result<FrameDescriptionEntry<R>>` — [`EhFrame`](#ehframe), [`BaseAddresses`](#baseaddresses), [`Result`](../index.md#result), [`FrameDescriptionEntry`](#framedescriptionentry)

  Returns a parsed FDE for the given address, or `NoUnwindInfoForAddress`

  if there are none.

  

  You must provide a function to get its associated CIE. See

  `PartialFrameDescriptionEntry::parse` for more information.

  

  # Example

  

  ```rust

  use gimli::{BaseAddresses, EhFrame, ParsedEhFrameHdr, EndianSlice, NativeEndian, Error, UnwindSection};

  fn foo() -> Result<(), Error> {

  let eh_frame: EhFrame<EndianSlice<NativeEndian>> = unreachable!();

  let eh_frame_hdr: ParsedEhFrameHdr<EndianSlice<NativeEndian>> = unimplemented!();

  let addr = 0;

  let bases = unimplemented!();

  let table = eh_frame_hdr.table().unwrap();

  let fde = table.fde_for_address(&eh_frame, &bases, addr, EhFrame::cie_from_offset)?;

  Ok(())

  }

  ```

- <span id="ehhdrtable-unwind-info-for-address"></span>`fn unwind_info_for_address<'ctx, F, S>(&self, frame: &EhFrame<R>, bases: &BaseAddresses, ctx: &'ctx mut UnwindContext<<R as >::Offset, S>, address: u64, get_cie: F) -> Result<&'ctx UnwindTableRow<<R as >::Offset, S>>` — [`EhFrame`](#ehframe), [`BaseAddresses`](#baseaddresses), [`UnwindContext`](#unwindcontext), [`Reader`](#reader), [`Result`](../index.md#result), [`UnwindTableRow`](#unwindtablerow)

  Returns the frame unwind information for the given address,

  or `NoUnwindInfoForAddress` if there are none.

  

  You must provide a function to get the associated CIE. See

  `PartialFrameDescriptionEntry::parse` for more information.

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for EhHdrTable<'a, R>`

- <span id="ehhdrtable-clone"></span>`fn clone(&self) -> EhHdrTable<'a, R>` — [`EhHdrTable`](#ehhdrtable)

##### `impl<R: fmt::Debug + Reader> Debug for EhHdrTable<'a, R>`

- <span id="ehhdrtable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `EhFrame<R: Reader>`

```rust
struct EhFrame<R: Reader> {
    section: R,
    address_size: u8,
    vendor: crate::common::Vendor,
}
```

`EhFrame` contains the frame unwinding information needed during exception
handling found in the `.eh_frame` section.

Most interesting methods are defined in the
[`UnwindSection`](#unwindsection) trait.

See
[`DebugFrame`](./struct.DebugFrame.html#differences-between-debug_frame-and-eh_frame)
for some discussion on the differences between `.debug_frame` and
`.eh_frame`.

#### Implementations

- <span id="ehframe-set-address-size"></span>`fn set_address_size(&mut self, address_size: u8)`

  Set the size of a target address in bytes.

  

  This defaults to the native word size.

- <span id="ehframe-set-vendor"></span>`fn set_vendor(&mut self, vendor: Vendor)` — [`Vendor`](../index.md#vendor)

  Set the vendor extensions to use.

  

  This defaults to `Vendor::Default`.

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for EhFrame<R>`

- <span id="ehframe-clone"></span>`fn clone(&self) -> EhFrame<R>` — [`EhFrame`](#ehframe)

##### `impl<R: marker::Copy + Reader> Copy for EhFrame<R>`

##### `impl<R: fmt::Debug + Reader> Debug for EhFrame<R>`

- <span id="ehframe-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: cmp::Eq + Reader> Eq for EhFrame<R>`

##### `impl<R: cmp::PartialEq + Reader> PartialEq for EhFrame<R>`

- <span id="ehframe-partialeq-eq"></span>`fn eq(&self, other: &EhFrame<R>) -> bool` — [`EhFrame`](#ehframe)

##### `impl<R: Reader> Section for EhFrame<R>`

- <span id="ehframe-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../index.md#sectionid)

- <span id="ehframe-section-reader"></span>`fn reader(&self) -> &R`

##### `impl<R: Reader> StructuralPartialEq for EhFrame<R>`

##### `impl<R: Reader> UnwindSection for EhFrame<R>`

- <span id="ehframe-unwindsection-type-offset"></span>`type Offset = EhFrameOffset<<R as Reader>::Offset>`

### `BaseAddresses`

```rust
struct BaseAddresses {
    pub eh_frame_hdr: SectionBaseAddresses,
    pub eh_frame: SectionBaseAddresses,
}
```

Optional base addresses for the relative `DW_EH_PE_*` encoded pointers.

During CIE/FDE parsing, if a relative pointer is encountered for a base
address that is unknown, an Err will be returned.

```rust
use gimli::BaseAddresses;

fn foo() {
let address_of_eh_frame_hdr_section_in_memory = unimplemented!();
let address_of_eh_frame_section_in_memory = unimplemented!();
let address_of_text_section_in_memory = unimplemented!();
let address_of_got_section_in_memory = unimplemented!();
let address_of_the_start_of_current_func = unimplemented!();
let bases = BaseAddresses::default()
    .set_eh_frame_hdr(address_of_eh_frame_hdr_section_in_memory)
    .set_eh_frame(address_of_eh_frame_section_in_memory)
    .set_text(address_of_text_section_in_memory)
    .set_got(address_of_got_section_in_memory);
let _ = bases;
}
```

#### Fields

- **`eh_frame_hdr`**: `SectionBaseAddresses`

  The base addresses to use for pointers in the `.eh_frame_hdr` section.

- **`eh_frame`**: `SectionBaseAddresses`

  The base addresses to use for pointers in the `.eh_frame` section.

#### Implementations

- <span id="baseaddresses-set-eh-frame-hdr"></span>`fn set_eh_frame_hdr(self, addr: u64) -> Self`

  Set the `.eh_frame_hdr` section base address.

- <span id="baseaddresses-set-eh-frame"></span>`fn set_eh_frame(self, addr: u64) -> Self`

  Set the `.eh_frame` section base address.

- <span id="baseaddresses-set-text"></span>`fn set_text(self, addr: u64) -> Self`

  Set the `.text` section base address.

- <span id="baseaddresses-set-got"></span>`fn set_got(self, addr: u64) -> Self`

  Set the `.got` section base address.

#### Trait Implementations

##### `impl Clone for BaseAddresses`

- <span id="baseaddresses-clone"></span>`fn clone(&self) -> BaseAddresses` — [`BaseAddresses`](#baseaddresses)

##### `impl Debug for BaseAddresses`

- <span id="baseaddresses-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for BaseAddresses`

- <span id="baseaddresses-default"></span>`fn default() -> BaseAddresses` — [`BaseAddresses`](#baseaddresses)

##### `impl Eq for BaseAddresses`

##### `impl PartialEq for BaseAddresses`

- <span id="baseaddresses-partialeq-eq"></span>`fn eq(&self, other: &BaseAddresses) -> bool` — [`BaseAddresses`](#baseaddresses)

##### `impl StructuralPartialEq for BaseAddresses`

### `SectionBaseAddresses`

```rust
struct SectionBaseAddresses {
    pub section: Option<u64>,
    pub text: Option<u64>,
    pub data: Option<u64>,
}
```

Optional base addresses for the relative `DW_EH_PE_*` encoded pointers
in a particular section.

See `BaseAddresses` for methods that are helpful in setting these addresses.

#### Fields

- **`section`**: `Option<u64>`

  The address of the section containing the pointer.

- **`text`**: `Option<u64>`

  The base address for text relative pointers.
  This is generally the address of the `.text` section.

- **`data`**: `Option<u64>`

  The base address for data relative pointers.
  
  For pointers in the `.eh_frame_hdr` section, this is the address
  of the `.eh_frame_hdr` section
  
  For pointers in the `.eh_frame` section, this is generally the
  global pointer, such as the address of the `.got` section.

#### Trait Implementations

##### `impl Clone for SectionBaseAddresses`

- <span id="sectionbaseaddresses-clone"></span>`fn clone(&self) -> SectionBaseAddresses` — [`SectionBaseAddresses`](#sectionbaseaddresses)

##### `impl Debug for SectionBaseAddresses`

- <span id="sectionbaseaddresses-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for SectionBaseAddresses`

- <span id="sectionbaseaddresses-default"></span>`fn default() -> SectionBaseAddresses` — [`SectionBaseAddresses`](#sectionbaseaddresses)

##### `impl Eq for SectionBaseAddresses`

##### `impl PartialEq for SectionBaseAddresses`

- <span id="sectionbaseaddresses-partialeq-eq"></span>`fn eq(&self, other: &SectionBaseAddresses) -> bool` — [`SectionBaseAddresses`](#sectionbaseaddresses)

##### `impl StructuralPartialEq for SectionBaseAddresses`

### `CfiEntriesIter<'bases, Section, R>`

```rust
struct CfiEntriesIter<'bases, Section, R>
where
    R: Reader,
    Section: UnwindSection<R> {
    section: Section,
    bases: &'bases BaseAddresses,
    input: R,
}
```

An iterator over CIE and FDE entries in a `.debug_frame` or `.eh_frame`
section.

Some pointers may be encoded relative to various base addresses. Use the
[`BaseAddresses`](./struct.BaseAddresses.html) parameter to provide them. By
default, none are provided. If a relative pointer is encountered for a base
address that is unknown, an `Err` will be returned and iteration will abort.

```rust
use gimli::{BaseAddresses, EhFrame, EndianSlice, NativeEndian, UnwindSection};

fn foo() -> gimli::Result<()> {
let read_eh_frame_somehow = || unimplemented!();
let eh_frame = EhFrame::new(read_eh_frame_somehow(), NativeEndian);

let address_of_eh_frame_hdr_section_in_memory = unimplemented!();
let address_of_eh_frame_section_in_memory = unimplemented!();
let address_of_text_section_in_memory = unimplemented!();
let address_of_got_section_in_memory = unimplemented!();
let address_of_the_start_of_current_func = unimplemented!();
// Provide base addresses for relative pointers.
let bases = BaseAddresses::default()
    .set_eh_frame_hdr(address_of_eh_frame_hdr_section_in_memory)
    .set_eh_frame(address_of_eh_frame_section_in_memory)
    .set_text(address_of_text_section_in_memory)
    .set_got(address_of_got_section_in_memory);

let mut entries = eh_frame.entries(&bases);

let do_stuff_with = |_| unimplemented!();
while let Some(entry) = entries.next()? {
    do_stuff_with(entry)
}
unreachable!()
}
```

#### Implementations

- <span id="cfientriesiter-next"></span>`fn next(&mut self) -> Result<Option<CieOrFde<'bases, Section, R>>>` — [`Result`](../index.md#result), [`CieOrFde`](#cieorfde)

  Advance the iterator to the next entry.

#### Trait Implementations

##### `impl<Section, R> Clone for CfiEntriesIter<'bases, Section, R>`

- <span id="cfientriesiter-clone"></span>`fn clone(&self) -> CfiEntriesIter<'bases, Section, R>` — [`CfiEntriesIter`](#cfientriesiter)

##### `impl<Section, R> Debug for CfiEntriesIter<'bases, Section, R>`

- <span id="cfientriesiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for CfiEntriesIter<'bases, Section, R>`

- <span id="cfientriesiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="cfientriesiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="cfientriesiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Section, R> Iterator for CfiEntriesIter<'bases, Section, R>`

- <span id="cfientriesiter-iterator-type-item"></span>`type Item = Result<CieOrFde<'bases, Section, R>, Error>`

- <span id="cfientriesiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `CfiEntryPrefix<R>`

```rust
struct CfiEntryPrefix<R>
where
    R: Reader {
    offset: <R as >::Offset,
    length: <R as >::Offset,
    format: crate::common::Format,
    cie_offset_base: <R as >::Offset,
    cie_id_or_offset: u64,
    rest: R,
}
```

The common prefix of a CIE or FDE.

#### Trait Implementations

##### `impl<R> Clone for CfiEntryPrefix<R>`

- <span id="cfientryprefix-clone"></span>`fn clone(&self) -> CfiEntryPrefix<R>` — [`CfiEntryPrefix`](cfi/index.md#cfientryprefix)

##### `impl<R> Debug for CfiEntryPrefix<R>`

- <span id="cfientryprefix-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Augmentation`

```rust
struct Augmentation {
    lsda: Option<constants::DwEhPe>,
    personality: Option<(constants::DwEhPe, Pointer)>,
    fde_address_encoding: Option<constants::DwEhPe>,
    is_signal_trampoline: bool,
}
```

We support the z-style augmentation [defined by `.eh_frame`][ehframe].


#### Fields

- **`lsda`**: `Option<constants::DwEhPe>`

  > A 'L' may be present at any position after the first character of the
  > string. This character may only be present if 'z' is the first character
  > of the string. If present, it indicates the presence of one argument in
  > the Augmentation Data of the CIE, and a corresponding argument in the
  > Augmentation Data of the FDE. The argument in the Augmentation Data of
  > the CIE is 1-byte and represents the pointer encoding used for the
  > argument in the Augmentation Data of the FDE, which is the address of a
  > language-specific data area (LSDA). The size of the LSDA pointer is
  > specified by the pointer encoding used.

- **`personality`**: `Option<(constants::DwEhPe, Pointer)>`

  > A 'P' may be present at any position after the first character of the
  > string. This character may only be present if 'z' is the first character
  > of the string. If present, it indicates the presence of two arguments in
  > the Augmentation Data of the CIE. The first argument is 1-byte and
  > represents the pointer encoding used for the second argument, which is
  > the address of a personality routine handler. The size of the
  > personality routine pointer is specified by the pointer encoding used.

- **`fde_address_encoding`**: `Option<constants::DwEhPe>`

  > A 'R' may be present at any position after the first character of the
  > string. This character may only be present if 'z' is the first character
  > of the string. If present, The Augmentation Data shall include a 1 byte
  > argument that represents the pointer encoding for the address pointers
  > used in the FDE.

- **`is_signal_trampoline`**: `bool`

  True if this CIE's FDEs are trampolines for signal handlers.

#### Implementations

- <span id="augmentation-parse"></span>`fn parse<Section, R>(augmentation_str: &mut R, bases: &BaseAddresses, address_size: u8, section: &Section, input: &mut R) -> Result<Augmentation>` — [`BaseAddresses`](#baseaddresses), [`Result`](../index.md#result), [`Augmentation`](#augmentation)

#### Trait Implementations

##### `impl Clone for Augmentation`

- <span id="augmentation-clone"></span>`fn clone(&self) -> Augmentation` — [`Augmentation`](#augmentation)

##### `impl Copy for Augmentation`

##### `impl Debug for Augmentation`

- <span id="augmentation-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Augmentation`

- <span id="augmentation-default"></span>`fn default() -> Augmentation` — [`Augmentation`](#augmentation)

##### `impl Eq for Augmentation`

##### `impl PartialEq for Augmentation`

- <span id="augmentation-partialeq-eq"></span>`fn eq(&self, other: &Augmentation) -> bool` — [`Augmentation`](#augmentation)

##### `impl StructuralPartialEq for Augmentation`

### `AugmentationData`

```rust
struct AugmentationData {
    lsda: Option<Pointer>,
}
```

Parsed augmentation data for a `FrameDescriptEntry`.

#### Implementations

- <span id="augmentationdata-parse"></span>`fn parse<R: Reader>(augmentation: &Augmentation, encoding_parameters: &PointerEncodingParameters<'_, R>, input: &mut R) -> Result<AugmentationData>` — [`Augmentation`](#augmentation), [`PointerEncodingParameters`](cfi/index.md#pointerencodingparameters), [`Result`](../index.md#result), [`AugmentationData`](cfi/index.md#augmentationdata)

#### Trait Implementations

##### `impl Clone for AugmentationData`

- <span id="augmentationdata-clone"></span>`fn clone(&self) -> AugmentationData` — [`AugmentationData`](cfi/index.md#augmentationdata)

##### `impl Debug for AugmentationData`

- <span id="augmentationdata-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AugmentationData`

- <span id="augmentationdata-default"></span>`fn default() -> AugmentationData` — [`AugmentationData`](cfi/index.md#augmentationdata)

##### `impl Eq for AugmentationData`

##### `impl PartialEq for AugmentationData`

- <span id="augmentationdata-partialeq-eq"></span>`fn eq(&self, other: &AugmentationData) -> bool` — [`AugmentationData`](cfi/index.md#augmentationdata)

##### `impl StructuralPartialEq for AugmentationData`

### `CommonInformationEntry<R, Offset>`

```rust
struct CommonInformationEntry<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    offset: Offset,
    length: Offset,
    format: crate::common::Format,
    version: u8,
    augmentation: Option<Augmentation>,
    address_size: u8,
    code_alignment_factor: u64,
    data_alignment_factor: i64,
    return_address_register: crate::common::Register,
    initial_instructions: R,
}
```

> A Common Information Entry holds information that is shared among many
> Frame Description Entries. There is at least one CIE in every non-empty
> `.debug_frame` section.

#### Fields

- **`offset`**: `Offset`

  The offset of this entry from the start of its containing section.

- **`length`**: `Offset`

  > A constant that gives the number of bytes of the CIE structure, not
  > including the length field itself (see Section 7.2.2). The size of the
  > length field plus the value of length must be an integral multiple of
  > the address size.

- **`version`**: `u8`

  > A version number (see Section 7.23). This number is specific to the
  > call frame information and is independent of the DWARF version number.

- **`augmentation`**: `Option<Augmentation>`

  The parsed augmentation, if any.

- **`address_size`**: `u8`

  > The size of a target address in this CIE and any FDEs that use it, in
  > bytes. If a compilation unit exists for this frame, its address size
  > must match the address size here.

- **`code_alignment_factor`**: `u64`

  "A constant that is factored out of all advance location instructions
  (see Section 6.4.2.1)."

- **`data_alignment_factor`**: `i64`

  > A constant that is factored out of certain offset instructions (see
  > below). The resulting value is (operand * data_alignment_factor).

- **`return_address_register`**: `crate::common::Register`

  > An unsigned LEB128 constant that indicates which column in the rule
  > table represents the return address of the function. Note that this
  > column might not correspond to an actual machine register.

- **`initial_instructions`**: `R`

  > A sequence of rules that are interpreted to create the initial setting
  > of each column in the table.
  
  > The default rule for all columns before interpretation of the initial
  > instructions is the undefined rule. However, an ABI authoring body or a
  > compilation system authoring body may specify an alternate default
  > value for any or all columns.
  
  This is followed by `DW_CFA_nop` padding until the end of `length` bytes
  in the input.

#### Implementations

- <span id="commoninformationentry-parse"></span>`fn parse<Section: UnwindSection<R>>(section: &Section, bases: &BaseAddresses, input: &mut R, offset: <R as >::Offset) -> Result<CommonInformationEntry<R>>` — [`BaseAddresses`](#baseaddresses), [`Reader`](#reader), [`Result`](../index.md#result), [`CommonInformationEntry`](#commoninformationentry)

- <span id="commoninformationentry-from-prefix"></span>`fn from_prefix<Section: UnwindSection<R>>(section: &Section, bases: &BaseAddresses, prefix: CfiEntryPrefix<R>) -> Result<CommonInformationEntry<R>>` — [`BaseAddresses`](#baseaddresses), [`CfiEntryPrefix`](cfi/index.md#cfientryprefix), [`Result`](../index.md#result), [`CommonInformationEntry`](#commoninformationentry)

#### Trait Implementations

##### `impl<R, Offset> Clone for CommonInformationEntry<R, Offset>`

- <span id="commoninformationentry-clone"></span>`fn clone(&self) -> CommonInformationEntry<R, Offset>` — [`CommonInformationEntry`](#commoninformationentry)

##### `impl<R, Offset> Debug for CommonInformationEntry<R, Offset>`

- <span id="commoninformationentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Offset> Eq for CommonInformationEntry<R, Offset>`

##### `impl<R, Offset> PartialEq for CommonInformationEntry<R, Offset>`

- <span id="commoninformationentry-partialeq-eq"></span>`fn eq(&self, other: &CommonInformationEntry<R, Offset>) -> bool` — [`CommonInformationEntry`](#commoninformationentry)

##### `impl<R, Offset> StructuralPartialEq for CommonInformationEntry<R, Offset>`

### `PartialFrameDescriptionEntry<'bases, Section, R>`

```rust
struct PartialFrameDescriptionEntry<'bases, Section, R>
where
    R: Reader,
    Section: UnwindSection<R> {
    offset: <R as >::Offset,
    length: <R as >::Offset,
    format: crate::common::Format,
    cie_offset: <Section as >::Offset,
    rest: R,
    section: Section,
    bases: &'bases BaseAddresses,
}
```

A partially parsed `FrameDescriptionEntry`.

Fully parsing this FDE requires first parsing its CIE.

#### Implementations

- <span id="partialframedescriptionentry-parse-partial"></span>`fn parse_partial(section: &Section, bases: &'bases BaseAddresses, input: &mut R, offset: <R as >::Offset) -> Result<PartialFrameDescriptionEntry<'bases, Section, R>>` — [`BaseAddresses`](#baseaddresses), [`Reader`](#reader), [`Result`](../index.md#result), [`PartialFrameDescriptionEntry`](#partialframedescriptionentry)

- <span id="partialframedescriptionentry-from-prefix"></span>`fn from_prefix(section: &Section, bases: &'bases BaseAddresses, prefix: CfiEntryPrefix<R>) -> Result<PartialFrameDescriptionEntry<'bases, Section, R>>` — [`BaseAddresses`](#baseaddresses), [`CfiEntryPrefix`](cfi/index.md#cfientryprefix), [`Result`](../index.md#result), [`PartialFrameDescriptionEntry`](#partialframedescriptionentry)

- <span id="partialframedescriptionentry-parse"></span>`fn parse<F>(&self, get_cie: F) -> Result<FrameDescriptionEntry<R>>` — [`Result`](../index.md#result), [`FrameDescriptionEntry`](#framedescriptionentry)

  Fully parse this FDE.

  

  You must provide a function get its associated CIE (either by parsing it

  on demand, or looking it up in some table mapping offsets to CIEs that

  you've already parsed, etc.)

- <span id="partialframedescriptionentry-offset"></span>`fn offset(&self) -> <R as >::Offset` — [`Reader`](#reader)

  Get the offset of this entry from the start of its containing section.

- <span id="partialframedescriptionentry-cie-offset"></span>`fn cie_offset(&self) -> <Section as >::Offset` — [`UnwindSection`](#unwindsection)

  Get the offset of this FDE's CIE.

- <span id="partialframedescriptionentry-entry-len"></span>`fn entry_len(&self) -> <R as >::Offset` — [`Reader`](#reader)

  > A constant that gives the number of bytes of the header and

  > instruction stream for this function, not including the length field

  > itself (see Section 7.2.2). The size of the length field plus the value

  > of length must be an integral multiple of the address size.

#### Trait Implementations

##### `impl<Section, R> Clone for PartialFrameDescriptionEntry<'bases, Section, R>`

- <span id="partialframedescriptionentry-clone"></span>`fn clone(&self) -> PartialFrameDescriptionEntry<'bases, Section, R>` — [`PartialFrameDescriptionEntry`](#partialframedescriptionentry)

##### `impl<Section, R> Debug for PartialFrameDescriptionEntry<'bases, Section, R>`

- <span id="partialframedescriptionentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Section, R> Eq for PartialFrameDescriptionEntry<'bases, Section, R>`

##### `impl<Section, R> PartialEq for PartialFrameDescriptionEntry<'bases, Section, R>`

- <span id="partialframedescriptionentry-partialeq-eq"></span>`fn eq(&self, other: &PartialFrameDescriptionEntry<'bases, Section, R>) -> bool` — [`PartialFrameDescriptionEntry`](#partialframedescriptionentry)

##### `impl<Section, R> StructuralPartialEq for PartialFrameDescriptionEntry<'bases, Section, R>`

### `FrameDescriptionEntry<R, Offset>`

```rust
struct FrameDescriptionEntry<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    offset: Offset,
    length: Offset,
    format: crate::common::Format,
    cie: CommonInformationEntry<R, Offset>,
    initial_address: u64,
    address_range: u64,
    augmentation: Option<AugmentationData>,
    instructions: R,
}
```

A `FrameDescriptionEntry` is a set of CFA instructions for an address range.

#### Fields

- **`offset`**: `Offset`

  The start of this entry within its containing section.

- **`length`**: `Offset`

  > A constant that gives the number of bytes of the header and
  > instruction stream for this function, not including the length field
  > itself (see Section 7.2.2). The size of the length field plus the value
  > of length must be an integral multiple of the address size.

- **`cie`**: `CommonInformationEntry<R, Offset>`

  "A constant offset into the .debug_frame section that denotes the CIE
  that is associated with this FDE."
  
  This is the CIE at that offset.

- **`initial_address`**: `u64`

  > The address of the first location associated with this table entry. If
  > the segment_size field of this FDE's CIE is non-zero, the initial
  > location is preceded by a segment selector of the given length.

- **`address_range`**: `u64`

  "The number of bytes of program instructions described by this entry."

- **`augmentation`**: `Option<AugmentationData>`

  The parsed augmentation data, if we have any.

- **`instructions`**: `R`

  "A sequence of table defining instructions that are described below."
  
  This is followed by `DW_CFA_nop` padding until `length` bytes of the
  input are consumed.

#### Implementations

- <span id="framedescriptionentry-parse-rest"></span>`fn parse_rest<Section, F>(offset: <R as >::Offset, length: <R as >::Offset, format: Format, cie_pointer: <Section as >::Offset, rest: R, section: &Section, bases: &BaseAddresses, get_cie: F) -> Result<FrameDescriptionEntry<R>>` — [`Reader`](#reader), [`Format`](../index.md#format), [`BaseAddresses`](#baseaddresses), [`Result`](../index.md#result), [`FrameDescriptionEntry`](#framedescriptionentry)

- <span id="framedescriptionentry-parse-addresses"></span>`fn parse_addresses(input: &mut R, cie: &CommonInformationEntry<R>, parameters: &PointerEncodingParameters<'_, R>) -> Result<(u64, u64)>` — [`CommonInformationEntry`](#commoninformationentry), [`PointerEncodingParameters`](cfi/index.md#pointerencodingparameters), [`Result`](../index.md#result)

- <span id="framedescriptionentry-rows"></span>`fn rows<'a, 'ctx, Section, S>(&self, section: &'a Section, bases: &'a BaseAddresses, ctx: &'ctx mut UnwindContext<<R as >::Offset, S>) -> Result<UnwindTable<'a, 'ctx, R, S>>` — [`BaseAddresses`](#baseaddresses), [`UnwindContext`](#unwindcontext), [`Reader`](#reader), [`Result`](../index.md#result), [`UnwindTable`](#unwindtable)

  Return the table of unwind information for this FDE.

- <span id="framedescriptionentry-unwind-info-for-address"></span>`fn unwind_info_for_address<'ctx, Section, S>(&self, section: &Section, bases: &BaseAddresses, ctx: &'ctx mut UnwindContext<<R as >::Offset, S>, address: u64) -> Result<&'ctx UnwindTableRow<<R as >::Offset, S>>` — [`BaseAddresses`](#baseaddresses), [`UnwindContext`](#unwindcontext), [`Reader`](#reader), [`Result`](../index.md#result), [`UnwindTableRow`](#unwindtablerow)

  Find the frame unwind information for the given address.

  

  If found, the unwind information is returned along with the reset

  context in the form `Ok((unwind_info, context))`. If not found,

  `Err(gimli::Error::NoUnwindInfoForAddress)` is returned. If parsing or

  CFI evaluation fails, the error is returned.

#### Trait Implementations

##### `impl<R, Offset> Clone for FrameDescriptionEntry<R, Offset>`

- <span id="framedescriptionentry-clone"></span>`fn clone(&self) -> FrameDescriptionEntry<R, Offset>` — [`FrameDescriptionEntry`](#framedescriptionentry)

##### `impl<R, Offset> Debug for FrameDescriptionEntry<R, Offset>`

- <span id="framedescriptionentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Offset> Eq for FrameDescriptionEntry<R, Offset>`

##### `impl<R, Offset> PartialEq for FrameDescriptionEntry<R, Offset>`

- <span id="framedescriptionentry-partialeq-eq"></span>`fn eq(&self, other: &FrameDescriptionEntry<R, Offset>) -> bool` — [`FrameDescriptionEntry`](#framedescriptionentry)

##### `impl<R, Offset> StructuralPartialEq for FrameDescriptionEntry<R, Offset>`

### `UnwindContext<T, S>`

```rust
struct UnwindContext<T, S>
where
    T: ReaderOffset,
    S: UnwindContextStorage<T> {
    stack: super::util::ArrayVec<<S as >::Stack>,
    initial_rule: Option<Option<(crate::common::Register, RegisterRule<T>)>>,
    is_initialized: bool,
}
```

Common context needed when evaluating the call frame unwinding information.

By default, this structure is small and allocates its internal storage
on the heap using [`Box`](../../allocator_api2/stable/boxed/index.md) during `UnwindContext::new`.

This can be overridden by providing a custom [`UnwindContextStorage`](#unwindcontextstorage) type parameter.
When using a custom storage with in-line arrays, the [`UnwindContext`](#unwindcontext) type itself
will be big, so in that case it's recommended to place [`UnwindContext`](#unwindcontext) on the
heap, e.g. using `Box::new(UnwindContext::<R, MyCustomStorage>::new_in())`.

To avoid re-allocating the context multiple times when evaluating multiple
CFI programs, the same [`UnwindContext`](#unwindcontext) can be reused for multiple unwinds.

```rust
use gimli::{UnwindContext, UnwindTable};

fn foo<'a>(some_fde: gimli::FrameDescriptionEntry<gimli::EndianSlice<'a, gimli::LittleEndian>>)
           -> gimli::Result<()> {
let eh_frame: gimli::EhFrame<_> = unreachable!();
let bases = unimplemented!();
// An uninitialized context.
let mut ctx = UnwindContext::new();

// Initialize the context by evaluating the CIE's initial instruction program,
// and generate the unwind table.
let mut table = some_fde.rows(&eh_frame, &bases, &mut ctx)?;
while let Some(row) = table.next_row()? {
    // Do stuff with each row...
  let _ = row;
}
unreachable!()
}
```

#### Implementations

- <span id="unwindcontext-new"></span>`fn new() -> Self`

  Construct a new call frame unwinding context.

#### Trait Implementations

##### `impl<T, S> Clone for UnwindContext<T, S>`

- <span id="unwindcontext-clone"></span>`fn clone(&self) -> UnwindContext<T, S>` — [`UnwindContext`](#unwindcontext)

##### `impl<T, S> Debug for UnwindContext<T, S>`

- <span id="unwindcontext-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, S> Default for UnwindContext<T, S>`

- <span id="unwindcontext-default"></span>`fn default() -> Self`

##### `impl<T, S> Eq for UnwindContext<T, S>`

##### `impl<T, S> PartialEq for UnwindContext<T, S>`

- <span id="unwindcontext-partialeq-eq"></span>`fn eq(&self, other: &UnwindContext<T, S>) -> bool` — [`UnwindContext`](#unwindcontext)

##### `impl<T, S> StructuralPartialEq for UnwindContext<T, S>`

### `UnwindTable<'a, 'ctx, R, S>`

```rust
struct UnwindTable<'a, 'ctx, R, S>
where
    R: Reader,
    S: UnwindContextStorage<<R as >::Offset> {
    code_alignment_factor: core::num::Wrapping<u64>,
    data_alignment_factor: core::num::Wrapping<i64>,
    address_size: u8,
    next_start_address: u64,
    last_end_address: u64,
    returned_last_row: bool,
    current_row_valid: bool,
    instructions: CallFrameInstructionIter<'a, R>,
    ctx: &'ctx mut UnwindContext<<R as >::Offset, S>,
}
```

The `UnwindTable` iteratively evaluates a `FrameDescriptionEntry`'s
`CallFrameInstruction` program, yielding the each row one at a time.

> 6.4.1 Structure of Call Frame Information
>
> DWARF supports virtual unwinding by defining an architecture independent
> basis for recording how procedures save and restore registers during their
> lifetimes. This basis must be augmented on some machines with specific
> information that is defined by an architecture specific ABI authoring
> committee, a hardware vendor, or a compiler producer. The body defining a
> specific augmentation is referred to below as the “augmenter.”
>
> Abstractly, this mechanism describes a very large table that has the
> following structure:
>
> <table>
>   <tr>
>     <th>LOC</th><th>CFA</th><th>R0</th><th>R1</th><td>...</td><th>RN</th>
>   </tr>
>   <tr>
>     <th>L0</th> <td></td>   <td></td>  <td></td>  <td></td>   <td></td>
>   </tr>
>   <tr>
>     <th>L1</th> <td></td>   <td></td>  <td></td>  <td></td>   <td></td>
>   </tr>
>   <tr>
>     <td>...</td><td></td>   <td></td>  <td></td>  <td></td>   <td></td>
>   </tr>
>   <tr>
>     <th>LN</th> <td></td>   <td></td>  <td></td>  <td></td>   <td></td>
>   </tr>
> </table>
>
> The first column indicates an address for every location that contains code
> in a program. (In shared objects, this is an object-relative offset.) The
> remaining columns contain virtual unwinding rules that are associated with
> the indicated location.
>
> The CFA column defines the rule which computes the Canonical Frame Address
> value; it may be either a register and a signed offset that are added
> together, or a DWARF expression that is evaluated.
>
> The remaining columns are labeled by register number. This includes some
> registers that have special designation on some architectures such as the PC
> and the stack pointer register. (The actual mapping of registers for a
> particular architecture is defined by the augmenter.) The register columns
> contain rules that describe whether a given register has been saved and the
> rule to find the value for the register in the previous frame.
>
> ...
>
> This table would be extremely large if actually constructed as
> described. Most of the entries at any point in the table are identical to
> the ones above them. The whole table can be represented quite compactly by
> recording just the differences starting at the beginning address of each
> subroutine in the program.

#### Implementations

- <span id="unwindtable-new"></span>`fn new<Section: UnwindSection<R>>(section: &'a Section, bases: &'a BaseAddresses, ctx: &'ctx mut UnwindContext<<R as >::Offset, S>, fde: &FrameDescriptionEntry<R>) -> Result<Self>` — [`BaseAddresses`](#baseaddresses), [`UnwindContext`](#unwindcontext), [`Reader`](#reader), [`FrameDescriptionEntry`](#framedescriptionentry), [`Result`](../index.md#result)

  Construct a new `UnwindTable` for the given

  `FrameDescriptionEntry`'s CFI unwinding program.

- <span id="unwindtable-new-for-fde"></span>`fn new_for_fde<Section: UnwindSection<R>>(section: &'a Section, bases: &'a BaseAddresses, ctx: &'ctx mut UnwindContext<<R as >::Offset, S>, fde: &FrameDescriptionEntry<R>) -> Self` — [`BaseAddresses`](#baseaddresses), [`UnwindContext`](#unwindcontext), [`Reader`](#reader), [`FrameDescriptionEntry`](#framedescriptionentry)

- <span id="unwindtable-new-for-cie"></span>`fn new_for_cie<Section: UnwindSection<R>>(section: &'a Section, bases: &'a BaseAddresses, ctx: &'ctx mut UnwindContext<<R as >::Offset, S>, cie: &CommonInformationEntry<R>) -> Self` — [`BaseAddresses`](#baseaddresses), [`UnwindContext`](#unwindcontext), [`Reader`](#reader), [`CommonInformationEntry`](#commoninformationentry)

- <span id="unwindtable-next-row"></span>`fn next_row(&mut self) -> Result<Option<&UnwindTableRow<<R as >::Offset, S>>>` — [`Result`](../index.md#result), [`UnwindTableRow`](#unwindtablerow), [`Reader`](#reader)

  Evaluate call frame instructions until the next row of the table is

  completed, and return it.

  

  Unfortunately, this cannot be used with `Iterator` because of

  the restricted lifetime of the yielded item.

- <span id="unwindtable-into-current-row"></span>`fn into_current_row(self) -> Option<&'ctx UnwindTableRow<<R as >::Offset, S>>` — [`UnwindTableRow`](#unwindtablerow), [`Reader`](#reader)

  Returns the current row with the lifetime of the context.

- <span id="unwindtable-evaluate"></span>`fn evaluate(&mut self, instruction: CallFrameInstruction<<R as >::Offset>) -> Result<bool>` — [`CallFrameInstruction`](#callframeinstruction), [`Reader`](#reader), [`Result`](../index.md#result)

  Evaluate one call frame instruction. Return `Ok(true)` if the row is

  complete, `Ok(false)` otherwise.

#### Trait Implementations

##### `impl<R, S> Debug for UnwindTable<'a, 'ctx, R, S>`

- <span id="unwindtable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `RegisterRuleMap<T, S>`

```rust
struct RegisterRuleMap<T, S>
where
    T: ReaderOffset,
    S: UnwindContextStorage<T> {
    rules: super::util::ArrayVec<<S as >::Rules>,
}
```

#### Implementations

- <span id="registerrulemap-is-default"></span>`fn is_default(&self) -> bool`

- <span id="registerrulemap-get"></span>`fn get(&self, register: Register) -> Option<RegisterRule<T>>` — [`Register`](../index.md#register), [`RegisterRule`](#registerrule)

- <span id="registerrulemap-clear"></span>`fn clear(&mut self, register: Register) -> Result<()>` — [`Register`](../index.md#register), [`Result`](../index.md#result)

- <span id="registerrulemap-set"></span>`fn set(&mut self, register: Register, rule: RegisterRule<T>) -> Result<()>` — [`Register`](../index.md#register), [`RegisterRule`](#registerrule), [`Result`](../index.md#result)

- <span id="registerrulemap-iter"></span>`fn iter(&self) -> RegisterRuleIter<'_, T>` — [`RegisterRuleIter`](#registerruleiter)

#### Trait Implementations

##### `impl<T, S> Clone for RegisterRuleMap<T, S>`

- <span id="registerrulemap-clone"></span>`fn clone(&self) -> Self`

##### `impl<T, S> Debug for RegisterRuleMap<T, S>`

- <span id="registerrulemap-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, S> Default for RegisterRuleMap<T, S>`

- <span id="registerrulemap-default"></span>`fn default() -> Self`

##### `impl<T, S> Eq for RegisterRuleMap<T, S>`

##### `impl<R, S> FromIterator for RegisterRuleMap<R, S>`

- <span id="registerrulemap-fromiterator-from-iter"></span>`fn from_iter<T>(iter: T) -> Self`

##### `impl<T, S> PartialEq for RegisterRuleMap<T, S>`

- <span id="registerrulemap-partialeq-eq"></span>`fn eq(&self, rhs: &Self) -> bool`

### `RegisterRuleIter<'iter, T>`

```rust
struct RegisterRuleIter<'iter, T>(::core::slice::Iter<'iter, (crate::common::Register, RegisterRule<T>)>)
where
    T: ReaderOffset;
```

An unordered iterator for register rules.

#### Trait Implementations

##### `impl<T> Clone for RegisterRuleIter<'iter, T>`

- <span id="registerruleiter-clone"></span>`fn clone(&self) -> RegisterRuleIter<'iter, T>` — [`RegisterRuleIter`](#registerruleiter)

##### `impl<T> Debug for RegisterRuleIter<'iter, T>`

- <span id="registerruleiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for RegisterRuleIter<'iter, T>`

- <span id="registerruleiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="registerruleiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="registerruleiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T: ReaderOffset> Iterator for RegisterRuleIter<'iter, T>`

- <span id="registerruleiter-iterator-type-item"></span>`type Item = &'iter (Register, RegisterRule<T>)`

- <span id="registerruleiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `UnwindTableRow<T, S>`

```rust
struct UnwindTableRow<T, S>
where
    T: ReaderOffset,
    S: UnwindContextStorage<T> {
    start_address: u64,
    end_address: u64,
    saved_args_size: u64,
    cfa: CfaRule<T>,
    registers: RegisterRuleMap<T, S>,
}
```

A row in the virtual unwind table that describes how to find the values of
the registers in the *previous* frame for a range of PC addresses.

#### Implementations

- <span id="unwindtablerow-is-default"></span>`fn is_default(&self) -> bool`

- <span id="unwindtablerow-start-address"></span>`fn start_address(&self) -> u64`

  Get the starting PC address that this row applies to.

- <span id="unwindtablerow-end-address"></span>`fn end_address(&self) -> u64`

  Get the end PC address where this row's register rules become

  unapplicable.

  

  In other words, this row describes how to recover the last frame's

  registers for all PCs where `row.start_address() <= PC <

  row.end_address()`. This row does NOT describe how to recover registers

  when `PC == row.end_address()`.

- <span id="unwindtablerow-contains"></span>`fn contains(&self, address: u64) -> bool`

  Return `true` if the given `address` is within this row's address range,

  `false` otherwise.

- <span id="unwindtablerow-saved-args-size"></span>`fn saved_args_size(&self) -> u64`

  Returns the amount of args currently on the stack.

  

  When unwinding, if the personality function requested a change in IP,

  the SP needs to be adjusted by saved_args_size.

- <span id="unwindtablerow-cfa"></span>`fn cfa(&self) -> &CfaRule<T>` — [`CfaRule`](#cfarule)

  Get the canonical frame address (CFA) recovery rule for this row.

- <span id="unwindtablerow-register"></span>`fn register(&self, register: Register) -> Option<RegisterRule<T>>` — [`Register`](../index.md#register), [`RegisterRule`](#registerrule)

  Get the register recovery rule for the given register number.

  

  Returns `None` if the register has the default rule, the value of which depends

  on the ABI or compilation system.

  

  The register number mapping is architecture dependent. For example, in

  the x86-64 ABI the register number mapping is defined in Figure 3.36:

  

  > Figure 3.36: DWARF Register Number Mapping

  >

  > <table>

  >   <tr><th>Register Name</th>                    <th>Number</th>  <th>Abbreviation</th></tr>

  >   <tr><td>General Purpose Register RAX</td>     <td>0</td>       <td>%rax</td></tr>

  >   <tr><td>General Purpose Register RDX</td>     <td>1</td>       <td>%rdx</td></tr>

  >   <tr><td>General Purpose Register RCX</td>     <td>2</td>       <td>%rcx</td></tr>

  >   <tr><td>General Purpose Register RBX</td>     <td>3</td>       <td>%rbx</td></tr>

  >   <tr><td>General Purpose Register RSI</td>     <td>4</td>       <td>%rsi</td></tr>

  >   <tr><td>General Purpose Register RDI</td>     <td>5</td>       <td>%rdi</td></tr>

  >   <tr><td>General Purpose Register RBP</td>     <td>6</td>       <td>%rbp</td></tr>

  >   <tr><td>Stack Pointer Register RSP</td>       <td>7</td>       <td>%rsp</td></tr>

  >   <tr><td>Extended Integer Registers 8-15</td>  <td>8-15</td>    <td>%r8-%r15</td></tr>

  >   <tr><td>Return Address RA</td>                <td>16</td>      <td></td></tr>

  >   <tr><td>Vector Registers 0–7</td>             <td>17-24</td>   <td>%xmm0–%xmm7</td></tr>

  >   <tr><td>Extended Vector Registers 8–15</td>   <td>25-32</td>   <td>%xmm8–%xmm15</td></tr>

  >   <tr><td>Floating Point Registers 0–7</td>     <td>33-40</td>   <td>%st0–%st7</td></tr>

  >   <tr><td>MMX Registers 0–7</td>                <td>41-48</td>   <td>%mm0–%mm7</td></tr>

  >   <tr><td>Flag Register</td>                    <td>49</td>      <td>%rFLAGS</td></tr>

  >   <tr><td>Segment Register ES</td>              <td>50</td>      <td>%es</td></tr>

  >   <tr><td>Segment Register CS</td>              <td>51</td>      <td>%cs</td></tr>

  >   <tr><td>Segment Register SS</td>              <td>52</td>      <td>%ss</td></tr>

  >   <tr><td>Segment Register DS</td>              <td>53</td>      <td>%ds</td></tr>

  >   <tr><td>Segment Register FS</td>              <td>54</td>      <td>%fs</td></tr>

  >   <tr><td>Segment Register GS</td>              <td>55</td>      <td>%gs</td></tr>

  >   <tr><td>Reserved</td>                         <td>56-57</td>   <td></td></tr>

  >   <tr><td>FS Base address</td>                  <td>58</td>      <td>%fs.base</td></tr>

  >   <tr><td>GS Base address</td>                  <td>59</td>      <td>%gs.base</td></tr>

  >   <tr><td>Reserved</td>                         <td>60-61</td>   <td></td></tr>

  >   <tr><td>Task Register</td>                    <td>62</td>      <td>%tr</td></tr>

  >   <tr><td>LDT Register</td>                     <td>63</td>      <td>%ldtr</td></tr>

  >   <tr><td>128-bit Media Control and Status</td> <td>64</td>      <td>%mxcsr</td></tr>

  >   <tr><td>x87 Control Word</td>                 <td>65</td>      <td>%fcw</td></tr>

  >   <tr><td>x87 Status Word</td>                  <td>66</td>      <td>%fsw</td></tr>

  >   <tr><td>Upper Vector Registers 16–31</td>     <td>67-82</td>   <td>%xmm16–%xmm31</td></tr>

  >   <tr><td>Reserved</td>                         <td>83-117</td>  <td></td></tr>

  >   <tr><td>Vector Mask Registers 0–7</td>        <td>118-125</td> <td>%k0–%k7</td></tr>

  >   <tr><td>Reserved</td>                         <td>126-129</td> <td></td></tr>

  > </table>

- <span id="unwindtablerow-registers"></span>`fn registers(&self) -> RegisterRuleIter<'_, T>` — [`RegisterRuleIter`](#registerruleiter)

  Iterate over all non-default register `(number, rule)` pairs.

  

  The rules are not iterated in any guaranteed order. Any register that

  does not make an appearance in the iterator implicitly has the default

  rule, the value of which depends on the ABI or compilation system.

  

  ```rust

  use gimli::{EndianSlice, LittleEndian, UnwindTableRow};

  fn foo<'input>(unwind_table_row: UnwindTableRow<usize>) {

  for &(register, ref rule) in unwind_table_row.registers() {

      // ...

      drop(register); drop(rule);

  }

  }

  ```

#### Trait Implementations

##### `impl<T, S> Clone for UnwindTableRow<T, S>`

- <span id="unwindtablerow-clone"></span>`fn clone(&self) -> Self`

##### `impl<T, S> Debug for UnwindTableRow<T, S>`

- <span id="unwindtablerow-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, S> Default for UnwindTableRow<T, S>`

- <span id="unwindtablerow-default"></span>`fn default() -> Self`

##### `impl<T, S> Eq for UnwindTableRow<T, S>`

##### `impl<T, S> PartialEq for UnwindTableRow<T, S>`

- <span id="unwindtablerow-partialeq-eq"></span>`fn eq(&self, other: &UnwindTableRow<T, S>) -> bool` — [`UnwindTableRow`](#unwindtablerow)

##### `impl<T, S> StructuralPartialEq for UnwindTableRow<T, S>`

### `CallFrameInstructionIter<'a, R: Reader>`

```rust
struct CallFrameInstructionIter<'a, R: Reader> {
    input: R,
    address_encoding: Option<constants::DwEhPe>,
    parameters: PointerEncodingParameters<'a, R>,
    vendor: crate::common::Vendor,
}
```

A lazy iterator parsing call frame instructions.

#### Implementations

- <span id="callframeinstructioniter-next"></span>`fn next(&mut self) -> Result<Option<CallFrameInstruction<<R as >::Offset>>>` — [`Result`](../index.md#result), [`CallFrameInstruction`](#callframeinstruction), [`Reader`](#reader)

  Parse the next call frame instruction.

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for CallFrameInstructionIter<'a, R>`

- <span id="callframeinstructioniter-clone"></span>`fn clone(&self) -> CallFrameInstructionIter<'a, R>` — [`CallFrameInstructionIter`](#callframeinstructioniter)

##### `impl<R: fmt::Debug + Reader> Debug for CallFrameInstructionIter<'a, R>`

- <span id="callframeinstructioniter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for CallFrameInstructionIter<'a, R>`

- <span id="callframeinstructioniter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="callframeinstructioniter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="callframeinstructioniter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: Reader> Iterator for CallFrameInstructionIter<'a, R>`

- <span id="callframeinstructioniter-iterator-type-item"></span>`type Item = Result<CallFrameInstruction<<R as Reader>::Offset>, Error>`

- <span id="callframeinstructioniter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `UnwindExpression<T: ReaderOffset>`

```rust
struct UnwindExpression<T: ReaderOffset> {
    pub offset: T,
    pub length: T,
}
```

The location of a DWARF expression within an unwind section.

This is stored as an offset and length within the section instead of as a
`Reader` to avoid lifetime issues when reusing [`UnwindContext`](#unwindcontext).

# Example
```rust
use gimli::{EhFrame, EndianSlice, NativeEndian, Error, FrameDescriptionEntry, UnwindExpression, EvaluationResult};
fn foo() -> Result<(), Error> {
let eh_frame: EhFrame<EndianSlice<NativeEndian>> = unreachable!();
let fde: FrameDescriptionEntry<EndianSlice<NativeEndian>> = unimplemented!();
let unwind_expression: UnwindExpression<_> = unimplemented!();
let expression = unwind_expression.get(&eh_frame)?;
let mut evaluation = expression.evaluation(fde.cie().encoding());
let mut result = evaluation.evaluate()?;
loop {
  match result {
     EvaluationResult::Complete => break,
     // Provide information to the evaluation.
     _ => { unimplemented!()}
  }
}
let value = evaluation.value_result();
Ok(())
}
```

#### Fields

- **`offset`**: `T`

  The offset of the expression within the section.

- **`length`**: `T`

  The length of the expression.

#### Implementations

- <span id="unwindexpression-get"></span>`fn get<R, S>(&self, section: &S) -> Result<Expression<R>>` — [`Result`](../index.md#result), [`Expression`](#expression)

  Get the expression from the section.

  

  The offset and length were previously validated when the

  `UnwindExpression` was created, so this should not fail.

#### Trait Implementations

##### `impl<T: clone::Clone + ReaderOffset> Clone for UnwindExpression<T>`

- <span id="unwindexpression-clone"></span>`fn clone(&self) -> UnwindExpression<T>` — [`UnwindExpression`](#unwindexpression)

##### `impl<T: marker::Copy + ReaderOffset> Copy for UnwindExpression<T>`

##### `impl<T: fmt::Debug + ReaderOffset> Debug for UnwindExpression<T>`

- <span id="unwindexpression-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq + ReaderOffset> Eq for UnwindExpression<T>`

##### `impl<T: cmp::PartialEq + ReaderOffset> PartialEq for UnwindExpression<T>`

- <span id="unwindexpression-partialeq-eq"></span>`fn eq(&self, other: &UnwindExpression<T>) -> bool` — [`UnwindExpression`](#unwindexpression)

##### `impl<T: ReaderOffset> StructuralPartialEq for UnwindExpression<T>`

### `PointerEncodingParameters<'a, R: Reader>`

```rust
struct PointerEncodingParameters<'a, R: Reader> {
    bases: &'a SectionBaseAddresses,
    func_base: Option<u64>,
    address_size: u8,
    section: &'a R,
}
```

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for PointerEncodingParameters<'a, R>`

- <span id="pointerencodingparameters-clone"></span>`fn clone(&self) -> PointerEncodingParameters<'a, R>` — [`PointerEncodingParameters`](cfi/index.md#pointerencodingparameters)

##### `impl<R: fmt::Debug + Reader> Debug for PointerEncodingParameters<'a, R>`

- <span id="pointerencodingparameters-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `DwarfSections<T>`

```rust
struct DwarfSections<T> {
    pub debug_abbrev: crate::read::DebugAbbrev<T>,
    pub debug_addr: crate::read::DebugAddr<T>,
    pub debug_aranges: crate::read::DebugAranges<T>,
    pub debug_info: crate::read::DebugInfo<T>,
    pub debug_line: crate::read::DebugLine<T>,
    pub debug_line_str: crate::read::DebugLineStr<T>,
    pub debug_macinfo: crate::read::DebugMacinfo<T>,
    pub debug_macro: crate::read::DebugMacro<T>,
    pub debug_names: crate::read::DebugNames<T>,
    pub debug_str: crate::read::DebugStr<T>,
    pub debug_str_offsets: crate::read::DebugStrOffsets<T>,
    pub debug_types: crate::read::DebugTypes<T>,
    pub debug_loc: crate::read::DebugLoc<T>,
    pub debug_loclists: crate::read::DebugLocLists<T>,
    pub debug_ranges: crate::read::DebugRanges<T>,
    pub debug_rnglists: crate::read::DebugRngLists<T>,
}
```

All of the commonly used DWARF sections.

This is useful for storing sections when `T` does not implement `Reader`.
It can be used to create a `Dwarf` that references the data in `self`.
If `T` does implement `Reader`, then use `Dwarf` directly.

## Example Usage

It can be useful to load DWARF sections into owned data structures,
such as `Vec`. However, we do not implement the `Reader` trait
for `Vec`, because it would be very inefficient, but this trait
is required for all of the methods that parse the DWARF data.
So we first load the DWARF sections into `Vec`s, and then use
`borrow` to create `Reader`s that reference the data.

```rust,no_run
fn example() -> Result<(), gimli::Error> {
let loader = |name| -> Result<_, gimli::Error> { unimplemented!() };
// Read the DWARF sections into `Vec`s with whatever object loader you're using.
let dwarf_sections: gimli::DwarfSections<Vec<u8>> = gimli::DwarfSections::load(loader)?;
// Create references to the DWARF sections.
let dwarf: gimli::Dwarf<_> = dwarf_sections.borrow(|section| {
    gimli::EndianSlice::new(&section, gimli::LittleEndian)
});
unreachable!()
}
```

#### Fields

- **`debug_abbrev`**: `crate::read::DebugAbbrev<T>`

  The `.debug_abbrev` section.

- **`debug_addr`**: `crate::read::DebugAddr<T>`

  The `.debug_addr` section.

- **`debug_aranges`**: `crate::read::DebugAranges<T>`

  The `.debug_aranges` section.

- **`debug_info`**: `crate::read::DebugInfo<T>`

  The `.debug_info` section.

- **`debug_line`**: `crate::read::DebugLine<T>`

  The `.debug_line` section.

- **`debug_line_str`**: `crate::read::DebugLineStr<T>`

  The `.debug_line_str` section.

- **`debug_macinfo`**: `crate::read::DebugMacinfo<T>`

  The `.debug_macinfo` section.

- **`debug_macro`**: `crate::read::DebugMacro<T>`

  The `.debug_macro` section.

- **`debug_names`**: `crate::read::DebugNames<T>`

  The `.debug_names` section.

- **`debug_str`**: `crate::read::DebugStr<T>`

  The `.debug_str` section.

- **`debug_str_offsets`**: `crate::read::DebugStrOffsets<T>`

  The `.debug_str_offsets` section.

- **`debug_types`**: `crate::read::DebugTypes<T>`

  The `.debug_types` section.

- **`debug_loc`**: `crate::read::DebugLoc<T>`

  The `.debug_loc` section.

- **`debug_loclists`**: `crate::read::DebugLocLists<T>`

  The `.debug_loclists` section.

- **`debug_ranges`**: `crate::read::DebugRanges<T>`

  The `.debug_ranges` section.

- **`debug_rnglists`**: `crate::read::DebugRngLists<T>`

  The `.debug_rnglists` section.

#### Implementations

- <span id="dwarfsections-load"></span>`fn load<F, E>(section: F) -> core::result::Result<Self, E>`

  Try to load the DWARF sections using the given loader function.

  

  `section` loads a DWARF section from the object file.

  It should return an empty section if the section does not exist.

- <span id="dwarfsections-borrow"></span>`fn borrow<'a, F, R>(self: &'a Self, borrow: F) -> Dwarf<R>` — [`Dwarf`](#dwarf)

  Create a `Dwarf` structure that references the data in `self`.

- <span id="dwarfsections-borrow-with-sup"></span>`fn borrow_with_sup<'a, F, R>(self: &'a Self, sup: Option<&'a Self>, borrow: F) -> Dwarf<R>` — [`Dwarf`](#dwarf)

  Create a `Dwarf` structure that references the data in `self` and `sup`.

  

  This is like `borrow`, but also includes the supplementary object file.

  This is useful when `R` implements `Reader` but `T` does not.

  

  ## Example Usage

  

  ```rust,no_run

  fn example() -> Result<(), gimli::Error> {

  let loader = |name| -> Result<_, gimli::Error> { unimplemented!() };

  let sup_loader = |name| -> Result<_, gimli::Error> { unimplemented!() };

  // Read the DWARF sections into `Vec`s with whatever object loader you're using.

  let dwarf_sections: gimli::DwarfSections<Vec<u8>> = gimli::DwarfSections::load(loader)?;

  let dwarf_sup_sections: gimli::DwarfSections<Vec<u8>> = gimli::DwarfSections::load(sup_loader)?;

  // Create references to the DWARF sections.

  let dwarf = dwarf_sections.borrow_with_sup(Some(&dwarf_sup_sections), |section| {

      gimli::EndianSlice::new(&section, gimli::LittleEndian)

  });

  unreachable!()

  }

  ```

#### Trait Implementations

##### `impl<T: fmt::Debug> Debug for DwarfSections<T>`

- <span id="dwarfsections-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: default::Default> Default for DwarfSections<T>`

- <span id="dwarfsections-default"></span>`fn default() -> DwarfSections<T>` — [`DwarfSections`](#dwarfsections)

### `Dwarf<R>`

```rust
struct Dwarf<R> {
    pub debug_abbrev: crate::read::DebugAbbrev<R>,
    pub debug_addr: crate::read::DebugAddr<R>,
    pub debug_aranges: crate::read::DebugAranges<R>,
    pub debug_info: crate::read::DebugInfo<R>,
    pub debug_line: crate::read::DebugLine<R>,
    pub debug_line_str: crate::read::DebugLineStr<R>,
    pub debug_macinfo: crate::read::DebugMacinfo<R>,
    pub debug_macro: crate::read::DebugMacro<R>,
    pub debug_names: crate::read::DebugNames<R>,
    pub debug_str: crate::read::DebugStr<R>,
    pub debug_str_offsets: crate::read::DebugStrOffsets<R>,
    pub debug_types: crate::read::DebugTypes<R>,
    pub locations: crate::read::LocationLists<R>,
    pub ranges: crate::read::RangeLists<R>,
    pub file_type: crate::common::DwarfFileType,
    pub sup: Option<alloc::sync::Arc<Dwarf<R>>>,
    pub abbreviations_cache: crate::read::AbbreviationsCache,
}
```

All of the commonly used DWARF sections, and other common information.

#### Fields

- **`debug_abbrev`**: `crate::read::DebugAbbrev<R>`

  The `.debug_abbrev` section.

- **`debug_addr`**: `crate::read::DebugAddr<R>`

  The `.debug_addr` section.

- **`debug_aranges`**: `crate::read::DebugAranges<R>`

  The `.debug_aranges` section.

- **`debug_info`**: `crate::read::DebugInfo<R>`

  The `.debug_info` section.

- **`debug_line`**: `crate::read::DebugLine<R>`

  The `.debug_line` section.

- **`debug_line_str`**: `crate::read::DebugLineStr<R>`

  The `.debug_line_str` section.

- **`debug_macinfo`**: `crate::read::DebugMacinfo<R>`

  The `.debug_macinfo` section.

- **`debug_macro`**: `crate::read::DebugMacro<R>`

  The `.debug_macro` section.

- **`debug_names`**: `crate::read::DebugNames<R>`

  The `.debug_names` section.

- **`debug_str`**: `crate::read::DebugStr<R>`

  The `.debug_str` section.

- **`debug_str_offsets`**: `crate::read::DebugStrOffsets<R>`

  The `.debug_str_offsets` section.

- **`debug_types`**: `crate::read::DebugTypes<R>`

  The `.debug_types` section.

- **`locations`**: `crate::read::LocationLists<R>`

  The location lists in the `.debug_loc` and `.debug_loclists` sections.

- **`ranges`**: `crate::read::RangeLists<R>`

  The range lists in the `.debug_ranges` and `.debug_rnglists` sections.

- **`file_type`**: `crate::common::DwarfFileType`

  The type of this file.

- **`sup`**: `Option<alloc::sync::Arc<Dwarf<R>>>`

  The DWARF sections for a supplementary object file.

- **`abbreviations_cache`**: `crate::read::AbbreviationsCache`

  A cache of previously parsed abbreviations for units in this file.

#### Implementations

- <span id="dwarf-load"></span>`fn load<F, E>(section: F) -> core::result::Result<Self, E>`

  Try to load the DWARF sections using the given loader function.

  

  `section` loads a DWARF section from the object file.

  It should return an empty section if the section does not exist.

  

  After loading, the user should set the `file_type` field and

  call `load_sup` if required.

- <span id="dwarf-load-sup"></span>`fn load_sup<F, E>(&mut self, section: F) -> core::result::Result<(), E>`

  Load the DWARF sections from the supplementary object file.

  

  `section` operates the same as for `load`.

  

  Sets `self.sup`, replacing any previous value.

- <span id="dwarf-from-sections"></span>`fn from_sections(sections: DwarfSections<T>) -> Self` — [`DwarfSections`](#dwarfsections)

  Create a `Dwarf` structure from the given sections.

  

  The caller should set the `file_type` and `sup` fields if required.

- <span id="dwarf-borrow"></span>`fn borrow<'a, F, R>(self: &'a Self, borrow: F) -> Dwarf<R>` — [`Dwarf`](#dwarf)

  Create a `Dwarf` structure that references the data in `self`.

  

  This is useful when `R` implements `Reader` but `T` does not.

  

  ## Example Usage

  

  It can be useful to load DWARF sections into owned data structures,

  such as `Vec`. However, we do not implement the `Reader` trait

  for `Vec`, because it would be very inefficient, but this trait

  is required for all of the methods that parse the DWARF data.

  So we first load the DWARF sections into `Vec`s, and then use

  `borrow` to create `Reader`s that reference the data.

  

  ```rust,no_run

  fn example() -> Result<(), gimli::Error> {

  let loader = |name| -> Result<_, gimli::Error> { unimplemented!() };

  let sup_loader = |name| -> Result<_, gimli::Error> { unimplemented!() };

  // Read the DWARF sections into `Vec`s with whatever object loader you're using.

  let mut owned_dwarf: gimli::Dwarf<Vec<u8>> = gimli::Dwarf::load(loader)?;

  owned_dwarf.load_sup(sup_loader)?;

  // Create references to the DWARF sections.

  let dwarf = owned_dwarf.borrow(|section| {

      gimli::EndianSlice::new(&section, gimli::LittleEndian)

  });

  unreachable!()

  }

  ```

- <span id="dwarf-set-sup"></span>`fn set_sup(&mut self, sup: Dwarf<T>)` — [`Dwarf`](#dwarf)

  Store the DWARF sections for the supplementary object file.

- <span id="dwarf-sup"></span>`fn sup(&self) -> Option<&Dwarf<T>>` — [`Dwarf`](#dwarf)

  Return a reference to the DWARF sections for the supplementary object file.

#### Trait Implementations

##### `impl<R: fmt::Debug> Debug for Dwarf<R>`

- <span id="dwarf-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for Dwarf<R>`

- <span id="dwarf-default"></span>`fn default() -> Dwarf<R>` — [`Dwarf`](#dwarf)

### `DwarfPackageSections<T>`

```rust
struct DwarfPackageSections<T> {
    pub cu_index: crate::read::DebugCuIndex<T>,
    pub tu_index: crate::read::DebugTuIndex<T>,
    pub debug_abbrev: crate::read::DebugAbbrev<T>,
    pub debug_info: crate::read::DebugInfo<T>,
    pub debug_line: crate::read::DebugLine<T>,
    pub debug_macinfo: crate::read::DebugMacinfo<T>,
    pub debug_macro: crate::read::DebugMacro<T>,
    pub debug_str: crate::read::DebugStr<T>,
    pub debug_str_offsets: crate::read::DebugStrOffsets<T>,
    pub debug_loc: crate::read::DebugLoc<T>,
    pub debug_loclists: crate::read::DebugLocLists<T>,
    pub debug_rnglists: crate::read::DebugRngLists<T>,
    pub debug_types: crate::read::DebugTypes<T>,
}
```

The sections from a `.dwp` file.

This is useful for storing sections when `T` does not implement `Reader`.
It can be used to create a `DwarfPackage` that references the data in `self`.
If `T` does implement `Reader`, then use `DwarfPackage` directly.

## Example Usage

It can be useful to load DWARF sections into owned data structures,
such as `Vec`. However, we do not implement the `Reader` trait
for `Vec`, because it would be very inefficient, but this trait
is required for all of the methods that parse the DWARF data.
So we first load the DWARF sections into `Vec`s, and then use
`borrow` to create `Reader`s that reference the data.

```rust,no_run
fn example() -> Result<(), gimli::Error> {
let loader = |name| -> Result<_, gimli::Error> { unimplemented!() };
// Read the DWARF sections into `Vec`s with whatever object loader you're using.
let dwp_sections: gimli::DwarfPackageSections<Vec<u8>> = gimli::DwarfPackageSections::load(loader)?;
// Create references to the DWARF sections.
let dwp: gimli::DwarfPackage<_> = dwp_sections.borrow(
    |section| gimli::EndianSlice::new(&section, gimli::LittleEndian),
    gimli::EndianSlice::new(&[], gimli::LittleEndian),
)?;
unreachable!()
}
```

#### Fields

- **`cu_index`**: `crate::read::DebugCuIndex<T>`

  The `.debug_cu_index` section.

- **`tu_index`**: `crate::read::DebugTuIndex<T>`

  The `.debug_tu_index` section.

- **`debug_abbrev`**: `crate::read::DebugAbbrev<T>`

  The `.debug_abbrev.dwo` section.

- **`debug_info`**: `crate::read::DebugInfo<T>`

  The `.debug_info.dwo` section.

- **`debug_line`**: `crate::read::DebugLine<T>`

  The `.debug_line.dwo` section.

- **`debug_macinfo`**: `crate::read::DebugMacinfo<T>`

  The `.debug_macinfo.dwo` section.
  
  Only present when using GNU split-dwarf extension to DWARF 4.

- **`debug_macro`**: `crate::read::DebugMacro<T>`

  The `.debug_macro.dwo` section.

- **`debug_str`**: `crate::read::DebugStr<T>`

  The `.debug_str.dwo` section.

- **`debug_str_offsets`**: `crate::read::DebugStrOffsets<T>`

  The `.debug_str_offsets.dwo` section.

- **`debug_loc`**: `crate::read::DebugLoc<T>`

  The `.debug_loc.dwo` section.
  
  Only present when using GNU split-dwarf extension to DWARF 4.

- **`debug_loclists`**: `crate::read::DebugLocLists<T>`

  The `.debug_loclists.dwo` section.

- **`debug_rnglists`**: `crate::read::DebugRngLists<T>`

  The `.debug_rnglists.dwo` section.

- **`debug_types`**: `crate::read::DebugTypes<T>`

  The `.debug_types.dwo` section.
  
  Only present when using GNU split-dwarf extension to DWARF 4.

#### Implementations

- <span id="dwarfpackagesections-load"></span>`fn load<F, E>(section: F) -> core::result::Result<Self, E>`

  Try to load the `.dwp` sections using the given loader function.

  

  `section` loads a DWARF section from the object file.

  It should return an empty section if the section does not exist.

- <span id="dwarfpackagesections-borrow"></span>`fn borrow<'a, F, R>(self: &'a Self, borrow: F, empty: R) -> Result<DwarfPackage<R>>` — [`Result`](../index.md#result), [`DwarfPackage`](#dwarfpackage)

  Create a `DwarfPackage` structure that references the data in `self`.

#### Trait Implementations

##### `impl<T: fmt::Debug> Debug for DwarfPackageSections<T>`

- <span id="dwarfpackagesections-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: default::Default> Default for DwarfPackageSections<T>`

- <span id="dwarfpackagesections-default"></span>`fn default() -> DwarfPackageSections<T>` — [`DwarfPackageSections`](#dwarfpackagesections)

### `DwarfPackage<R: Reader>`

```rust
struct DwarfPackage<R: Reader> {
    pub cu_index: crate::read::UnitIndex<R>,
    pub tu_index: crate::read::UnitIndex<R>,
    pub debug_abbrev: crate::read::DebugAbbrev<R>,
    pub debug_info: crate::read::DebugInfo<R>,
    pub debug_line: crate::read::DebugLine<R>,
    pub debug_macinfo: crate::read::DebugMacinfo<R>,
    pub debug_macro: crate::read::DebugMacro<R>,
    pub debug_str: crate::read::DebugStr<R>,
    pub debug_str_offsets: crate::read::DebugStrOffsets<R>,
    pub debug_loc: crate::read::DebugLoc<R>,
    pub debug_loclists: crate::read::DebugLocLists<R>,
    pub debug_rnglists: crate::read::DebugRngLists<R>,
    pub debug_types: crate::read::DebugTypes<R>,
    pub empty: R,
}
```

The sections from a `.dwp` file, with parsed indices.

#### Fields

- **`cu_index`**: `crate::read::UnitIndex<R>`

  The compilation unit index in the `.debug_cu_index` section.

- **`tu_index`**: `crate::read::UnitIndex<R>`

  The type unit index in the `.debug_tu_index` section.

- **`debug_abbrev`**: `crate::read::DebugAbbrev<R>`

  The `.debug_abbrev.dwo` section.

- **`debug_info`**: `crate::read::DebugInfo<R>`

  The `.debug_info.dwo` section.

- **`debug_line`**: `crate::read::DebugLine<R>`

  The `.debug_line.dwo` section.

- **`debug_macinfo`**: `crate::read::DebugMacinfo<R>`

  The `.debug_macinfo.dwo` section.

- **`debug_macro`**: `crate::read::DebugMacro<R>`

  The `.debug_macro.dwo` section.

- **`debug_str`**: `crate::read::DebugStr<R>`

  The `.debug_str.dwo` section.

- **`debug_str_offsets`**: `crate::read::DebugStrOffsets<R>`

  The `.debug_str_offsets.dwo` section.

- **`debug_loc`**: `crate::read::DebugLoc<R>`

  The `.debug_loc.dwo` section.
  
  Only present when using GNU split-dwarf extension to DWARF 4.

- **`debug_loclists`**: `crate::read::DebugLocLists<R>`

  The `.debug_loclists.dwo` section.

- **`debug_rnglists`**: `crate::read::DebugRngLists<R>`

  The `.debug_rnglists.dwo` section.

- **`debug_types`**: `crate::read::DebugTypes<R>`

  The `.debug_types.dwo` section.
  
  Only present when using GNU split-dwarf extension to DWARF 4.

- **`empty`**: `R`

  An empty section.
  
  Used when creating `Dwarf<R>`.

#### Implementations

- <span id="dwarfpackage-load"></span>`fn load<F, E>(section: F, empty: R) -> core::result::Result<Self, E>`

  Try to load the `.dwp` sections using the given loader function.

  

  `section` loads a DWARF section from the object file.

  It should return an empty section if the section does not exist.

- <span id="dwarfpackage-from-sections"></span>`fn from_sections(sections: DwarfPackageSections<R>, empty: R) -> Result<Self>` — [`DwarfPackageSections`](#dwarfpackagesections), [`Result`](../index.md#result)

  Create a `DwarfPackage` structure from the given sections.

- <span id="dwarfpackage-find-cu"></span>`fn find_cu(&self, id: DwoId, parent: &Dwarf<R>) -> Result<Option<Dwarf<R>>>` — [`DwoId`](../index.md#dwoid), [`Dwarf`](#dwarf), [`Result`](../index.md#result)

  Find the compilation unit with the given DWO identifier and return its section

  contributions.

  

  ## Example Usage

  

  ```rust,no_run

  fn example<R: gimli::Reader>(

         dwarf: &gimli::Dwarf<R>,

         dwp: &gimli::DwarfPackage<R>,

         dwo_id: gimli::DwoId,

  ) -> Result<(), gimli::Error> {

  if let Some(dwo) = dwp.find_cu(dwo_id, dwarf)? {

     let dwo_header = dwo.units().next()?.expect("DWO should have one unit");

     let dwo_unit = dwo.unit(dwo_header)?;

     // Do something with `dwo_unit`.

  }

  unreachable!()

  }

- <span id="dwarfpackage-find-tu"></span>`fn find_tu(&self, signature: DebugTypeSignature, parent: &Dwarf<R>) -> Result<Option<Dwarf<R>>>` — [`DebugTypeSignature`](../index.md#debugtypesignature), [`Dwarf`](#dwarf), [`Result`](../index.md#result)

  Find the type unit with the given type signature and return its section

  contributions.

- <span id="dwarfpackage-cu-sections"></span>`fn cu_sections(&self, index: u32, parent: &Dwarf<R>) -> Result<Dwarf<R>>` — [`Dwarf`](#dwarf), [`Result`](../index.md#result)

  Return the section contributions of the compilation unit at the given index.

  

  The index must be in the range `1..cu_index.unit_count`.

  

  This function should only be needed by low level parsers.

- <span id="dwarfpackage-tu-sections"></span>`fn tu_sections(&self, index: u32, parent: &Dwarf<R>) -> Result<Dwarf<R>>` — [`Dwarf`](#dwarf), [`Result`](../index.md#result)

  Return the section contributions of the compilation unit at the given index.

  

  The index must be in the range `1..tu_index.unit_count`.

  

  This function should only be needed by low level parsers.

- <span id="dwarfpackage-sections"></span>`fn sections(&self, sections: UnitIndexSectionIterator<'_, R>, parent: &Dwarf<R>) -> Result<Dwarf<R>>` — [`UnitIndexSectionIterator`](#unitindexsectioniterator), [`Dwarf`](#dwarf), [`Result`](../index.md#result)

  Return the section contributions of a unit.

  

  This function should only be needed by low level parsers.

#### Trait Implementations

##### `impl<R: fmt::Debug + Reader> Debug for DwarfPackage<R>`

- <span id="dwarfpackage-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Unit<R, Offset>`

```rust
struct Unit<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    pub header: crate::read::UnitHeader<R, Offset>,
    pub abbreviations: alloc::sync::Arc<crate::read::Abbreviations>,
    pub name: Option<R>,
    pub comp_dir: Option<R>,
    pub low_pc: u64,
    pub str_offsets_base: crate::common::DebugStrOffsetsBase<Offset>,
    pub addr_base: crate::common::DebugAddrBase<Offset>,
    pub loclists_base: crate::common::DebugLocListsBase<Offset>,
    pub rnglists_base: crate::common::DebugRngListsBase<Offset>,
    pub line_program: Option<crate::read::IncompleteLineProgram<R, Offset>>,
    pub dwo_id: Option<crate::common::DwoId>,
}
```

All of the commonly used information for a unit in the `.debug_info` or `.debug_types`
sections.

#### Fields

- **`header`**: `crate::read::UnitHeader<R, Offset>`

  The header of the unit.

- **`abbreviations`**: `alloc::sync::Arc<crate::read::Abbreviations>`

  The parsed abbreviations for the unit.

- **`name`**: `Option<R>`

  The `DW_AT_name` attribute of the unit.

- **`comp_dir`**: `Option<R>`

  The `DW_AT_comp_dir` attribute of the unit.

- **`low_pc`**: `u64`

  The `DW_AT_low_pc` attribute of the unit. Defaults to 0.

- **`str_offsets_base`**: `crate::common::DebugStrOffsetsBase<Offset>`

  The `DW_AT_str_offsets_base` attribute of the unit. Defaults to 0.

- **`addr_base`**: `crate::common::DebugAddrBase<Offset>`

  The `DW_AT_addr_base` attribute of the unit. Defaults to 0.

- **`loclists_base`**: `crate::common::DebugLocListsBase<Offset>`

  The `DW_AT_loclists_base` attribute of the unit. Defaults to 0.

- **`rnglists_base`**: `crate::common::DebugRngListsBase<Offset>`

  The `DW_AT_rnglists_base` attribute of the unit. Defaults to 0.

- **`line_program`**: `Option<crate::read::IncompleteLineProgram<R, Offset>>`

  The line number program of the unit.

- **`dwo_id`**: `Option<crate::common::DwoId>`

  The DWO ID of a skeleton unit or split compilation unit.

#### Implementations

- <span id="unit-new"></span>`fn new(dwarf: &Dwarf<R>, header: UnitHeader<R>) -> Result<Self>` — [`Dwarf`](#dwarf), [`UnitHeader`](#unitheader), [`Result`](../index.md#result)

  Construct a new `Unit` from the given unit header.

- <span id="unit-new-with-abbreviations"></span>`fn new_with_abbreviations(dwarf: &Dwarf<R>, header: UnitHeader<R>, abbreviations: Arc<Abbreviations>) -> Result<Self>` — [`Dwarf`](#dwarf), [`UnitHeader`](#unitheader), [`Abbreviations`](#abbreviations), [`Result`](../index.md#result)

  Construct a new `Unit` from the given unit header and abbreviations.

  

  The abbreviations for this call can be obtained using `dwarf.abbreviations(&header)`.

  The caller may implement caching to reuse the `Abbreviations` across units with the

  same `header.debug_abbrev_offset()` value.

- <span id="unit-unit-ref"></span>`fn unit_ref<'a>(self: &'a Self, dwarf: &'a Dwarf<R>) -> UnitRef<'a, R>` — [`Dwarf`](#dwarf), [`UnitRef`](#unitref)

  Return a reference to this unit and its associated `Dwarf`.

- <span id="unit-encoding"></span>`fn encoding(&self) -> Encoding` — [`Encoding`](../index.md#encoding)

  Return the encoding parameters for this unit.

- <span id="unit-entry"></span>`fn entry(&self, offset: UnitOffset<<R as >::Offset>) -> Result<DebuggingInformationEntry<R>>` — [`UnitOffset`](../index.md#unitoffset), [`Reader`](#reader), [`Result`](../index.md#result), [`DebuggingInformationEntry`](#debugginginformationentry)

  Read the `DebuggingInformationEntry` at the given offset.

- <span id="unit-entries"></span>`fn entries(&self) -> EntriesCursor<'_, R>` — [`EntriesCursor`](#entriescursor)

  Navigate this unit's `DebuggingInformationEntry`s.

- <span id="unit-entries-at-offset"></span>`fn entries_at_offset(&self, offset: UnitOffset<<R as >::Offset>) -> Result<EntriesCursor<'_, R>>` — [`UnitOffset`](../index.md#unitoffset), [`Reader`](#reader), [`Result`](../index.md#result), [`EntriesCursor`](#entriescursor)

  Navigate this unit's `DebuggingInformationEntry`s

  starting at the given offset.

- <span id="unit-entries-tree"></span>`fn entries_tree(&self, offset: Option<UnitOffset<<R as >::Offset>>) -> Result<EntriesTree<'_, R>>` — [`UnitOffset`](../index.md#unitoffset), [`Reader`](#reader), [`Result`](../index.md#result), [`EntriesTree`](#entriestree)

  Navigate this unit's `DebuggingInformationEntry`s as a tree

  starting at the given offset.

- <span id="unit-entries-raw"></span>`fn entries_raw(&self, offset: Option<UnitOffset<<R as >::Offset>>) -> Result<EntriesRaw<'_, R>>` — [`UnitOffset`](../index.md#unitoffset), [`Reader`](#reader), [`Result`](../index.md#result), [`EntriesRaw`](#entriesraw)

  Read the raw data that defines the Debugging Information Entries.

- <span id="unit-copy-relocated-attributes"></span>`fn copy_relocated_attributes(&mut self, other: &Unit<R>)` — [`Unit`](#unit)

  Copy attributes that are subject to relocation from another unit. This is intended

  to be used to copy attributes from a skeleton compilation unit to the corresponding

  split compilation unit.

- <span id="unit-dwo-name"></span>`fn dwo_name(&self) -> Result<Option<AttributeValue<R>>>` — [`Result`](../index.md#result), [`AttributeValue`](#attributevalue)

  Find the dwo name (if any) for this unit, automatically handling the differences

  between the standardized DWARF 5 split DWARF format and the pre-DWARF 5 GNU

  extension.

  

  The returned value is relative to this unit's `comp_dir`.

#### Trait Implementations

##### `impl<R, Offset> Debug for Unit<R, Offset>`

- <span id="unit-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: Reader> Deref for Unit<R>`

- <span id="unit-deref-type-target"></span>`type Target = UnitHeader<R>`

- <span id="unit-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl Receiver for Unit<R, Offset>`

- <span id="unit-receiver-type-target"></span>`type Target = T`

### `UnitRef<'a, R: Reader>`

```rust
struct UnitRef<'a, R: Reader> {
    pub dwarf: &'a Dwarf<R>,
    pub unit: &'a Unit<R>,
}
```

A reference to a `Unit` and its associated `Dwarf`.

These often need to be passed around together, so this struct makes that easier.

It implements `Deref` to `&'a Unit`, so you can use it as if it were a `Unit`.
It also implements methods that correspond to methods on `Dwarf` that take a `Unit`.

#### Fields

- **`dwarf`**: `&'a Dwarf<R>`

  The `Dwarf` that contains the unit.

- **`unit`**: `&'a Unit<R>`

  The `Unit` being referenced.

#### Implementations

- <span id="unitref-new"></span>`fn new(dwarf: &'a Dwarf<R>, unit: &'a Unit<R>) -> Self` — [`Dwarf`](#dwarf), [`Unit`](#unit)

  Construct a new `UnitRef` from a `Dwarf` and a `Unit`.

- <span id="unitref-string-offset"></span>`fn string_offset(&self, index: DebugStrOffsetsIndex<<R as >::Offset>) -> Result<DebugStrOffset<<R as >::Offset>>` — [`DebugStrOffsetsIndex`](../index.md#debugstroffsetsindex), [`Reader`](#reader), [`Result`](../index.md#result), [`DebugStrOffset`](../index.md#debugstroffset)

  Return the string offset at the given index.

- <span id="unitref-string"></span>`fn string(&self, offset: DebugStrOffset<<R as >::Offset>) -> Result<R>` — [`DebugStrOffset`](../index.md#debugstroffset), [`Reader`](#reader), [`Result`](../index.md#result)

  Return the string at the given offset in `.debug_str`.

- <span id="unitref-line-string"></span>`fn line_string(&self, offset: DebugLineStrOffset<<R as >::Offset>) -> Result<R>` — [`DebugLineStrOffset`](../index.md#debuglinestroffset), [`Reader`](#reader), [`Result`](../index.md#result)

  Return the string at the given offset in `.debug_line_str`.

- <span id="unitref-sup-string"></span>`fn sup_string(&self, offset: DebugStrOffset<<R as >::Offset>) -> Result<R>` — [`DebugStrOffset`](../index.md#debugstroffset), [`Reader`](#reader), [`Result`](../index.md#result)

  Return the string at the given offset in the `.debug_str`

  in the supplementary object file.

- <span id="unitref-attr-string"></span>`fn attr_string(&self, attr: AttributeValue<R>) -> Result<R>` — [`AttributeValue`](#attributevalue), [`Result`](../index.md#result)

  Return an attribute value as a string slice.

  

  See `Dwarf::attr_string` for more information.

- <span id="unitref-address"></span>`fn address(&self, index: DebugAddrIndex<<R as >::Offset>) -> Result<u64>` — [`DebugAddrIndex`](../index.md#debugaddrindex), [`Reader`](#reader), [`Result`](../index.md#result)

  Return the address at the given index.

- <span id="unitref-attr-address"></span>`fn attr_address(&self, attr: AttributeValue<R>) -> Result<Option<u64>>` — [`AttributeValue`](#attributevalue), [`Result`](../index.md#result)

  Try to return an attribute value as an address.

  

  See `Dwarf::attr_address` for more information.

- <span id="unitref-ranges-offset-from-raw"></span>`fn ranges_offset_from_raw(&self, offset: RawRangeListsOffset<<R as >::Offset>) -> RangeListsOffset<<R as >::Offset>` — [`RawRangeListsOffset`](../index.md#rawrangelistsoffset), [`Reader`](#reader), [`RangeListsOffset`](../index.md#rangelistsoffset)

  Return the range list offset for the given raw offset.

  

  This handles adding `DW_AT_GNU_ranges_base` if required.

- <span id="unitref-ranges-offset"></span>`fn ranges_offset(&self, index: DebugRngListsIndex<<R as >::Offset>) -> Result<RangeListsOffset<<R as >::Offset>>` — [`DebugRngListsIndex`](../index.md#debugrnglistsindex), [`Reader`](#reader), [`Result`](../index.md#result), [`RangeListsOffset`](../index.md#rangelistsoffset)

  Return the range list offset at the given index.

- <span id="unitref-ranges"></span>`fn ranges(&self, offset: RangeListsOffset<<R as >::Offset>) -> Result<RngListIter<R>>` — [`RangeListsOffset`](../index.md#rangelistsoffset), [`Reader`](#reader), [`Result`](../index.md#result), [`RngListIter`](#rnglistiter)

  Iterate over the `RangeListEntry`s starting at the given offset.

- <span id="unitref-raw-ranges"></span>`fn raw_ranges(&self, offset: RangeListsOffset<<R as >::Offset>) -> Result<RawRngListIter<R>>` — [`RangeListsOffset`](../index.md#rangelistsoffset), [`Reader`](#reader), [`Result`](../index.md#result), [`RawRngListIter`](#rawrnglistiter)

  Iterate over the `RawRngListEntry`ies starting at the given offset.

- <span id="unitref-attr-ranges-offset"></span>`fn attr_ranges_offset(&self, attr: AttributeValue<R>) -> Result<Option<RangeListsOffset<<R as >::Offset>>>` — [`AttributeValue`](#attributevalue), [`Result`](../index.md#result), [`RangeListsOffset`](../index.md#rangelistsoffset), [`Reader`](#reader)

  Try to return an attribute value as a range list offset.

  

  See `Dwarf::attr_ranges_offset` for more information.

- <span id="unitref-attr-ranges"></span>`fn attr_ranges(&self, attr: AttributeValue<R>) -> Result<Option<RngListIter<R>>>` — [`AttributeValue`](#attributevalue), [`Result`](../index.md#result), [`RngListIter`](#rnglistiter)

  Try to return an attribute value as a range list entry iterator.

  

  See `Dwarf::attr_ranges` for more information.

- <span id="unitref-die-ranges"></span>`fn die_ranges(&self, entry: &DebuggingInformationEntry<R>) -> Result<RangeIter<R>>` — [`DebuggingInformationEntry`](#debugginginformationentry), [`Result`](../index.md#result), [`RangeIter`](#rangeiter)

  Return an iterator for the address ranges of a `DebuggingInformationEntry`.

  

  This uses `DW_AT_low_pc`, `DW_AT_high_pc` and `DW_AT_ranges`.

- <span id="unitref-unit-ranges"></span>`fn unit_ranges(&self) -> Result<RangeIter<R>>` — [`Result`](../index.md#result), [`RangeIter`](#rangeiter)

  Return an iterator for the address ranges of the `Unit`.

  

  This uses `DW_AT_low_pc`, `DW_AT_high_pc` and `DW_AT_ranges` of the

  root `DebuggingInformationEntry`.

- <span id="unitref-locations-offset"></span>`fn locations_offset(&self, index: DebugLocListsIndex<<R as >::Offset>) -> Result<LocationListsOffset<<R as >::Offset>>` — [`DebugLocListsIndex`](../index.md#debugloclistsindex), [`Reader`](#reader), [`Result`](../index.md#result), [`LocationListsOffset`](../index.md#locationlistsoffset)

  Return the location list offset at the given index.

- <span id="unitref-locations"></span>`fn locations(&self, offset: LocationListsOffset<<R as >::Offset>) -> Result<LocListIter<R>>` — [`LocationListsOffset`](../index.md#locationlistsoffset), [`Reader`](#reader), [`Result`](../index.md#result), [`LocListIter`](#loclistiter)

  Iterate over the `LocationListEntry`s starting at the given offset.

- <span id="unitref-raw-locations"></span>`fn raw_locations(&self, offset: LocationListsOffset<<R as >::Offset>) -> Result<RawLocListIter<R>>` — [`LocationListsOffset`](../index.md#locationlistsoffset), [`Reader`](#reader), [`Result`](../index.md#result), [`RawLocListIter`](#rawloclistiter)

  Iterate over the raw `LocationListEntry`s starting at the given offset.

- <span id="unitref-attr-locations-offset"></span>`fn attr_locations_offset(&self, attr: AttributeValue<R>) -> Result<Option<LocationListsOffset<<R as >::Offset>>>` — [`AttributeValue`](#attributevalue), [`Result`](../index.md#result), [`LocationListsOffset`](../index.md#locationlistsoffset), [`Reader`](#reader)

  Try to return an attribute value as a location list offset.

  

  See `Dwarf::attr_locations_offset` for more information.

- <span id="unitref-attr-locations"></span>`fn attr_locations(&self, attr: AttributeValue<R>) -> Result<Option<LocListIter<R>>>` — [`AttributeValue`](#attributevalue), [`Result`](../index.md#result), [`LocListIter`](#loclistiter)

  Try to return an attribute value as a location list entry iterator.

  

  See `Dwarf::attr_locations` for more information.

- <span id="unitref-macinfo"></span>`fn macinfo(&self, offset: DebugMacinfoOffset<<R as >::Offset>) -> Result<MacroIter<R>>` — [`DebugMacinfoOffset`](../index.md#debugmacinfooffset), [`Reader`](#reader), [`Result`](../index.md#result), [`MacroIter`](#macroiter)

  Try to return an iterator for the list of macros at the given `.debug_macinfo` offset.

- <span id="unitref-macros"></span>`fn macros(&self, offset: DebugMacroOffset<<R as >::Offset>) -> Result<MacroIter<R>>` — [`DebugMacroOffset`](../index.md#debugmacrooffset), [`Reader`](#reader), [`Result`](../index.md#result), [`MacroIter`](#macroiter)

  Try to return an iterator for the list of macros at the given `.debug_macro` offset.

#### Trait Implementations

##### `impl<R: Reader> Clone for UnitRef<'a, R>`

- <span id="unitref-clone"></span>`fn clone(&self) -> Self`

##### `impl<R: Reader> Copy for UnitRef<'a, R>`

##### `impl<R: fmt::Debug + Reader> Debug for UnitRef<'a, R>`

- <span id="unitref-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: Reader> Deref for UnitRef<'a, R>`

- <span id="unitref-deref-type-target"></span>`type Target = &'a Unit<R>`

- <span id="unitref-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl Receiver for UnitRef<'a, R>`

- <span id="unitref-receiver-type-target"></span>`type Target = T`

### `RangeIter<R: Reader>`

```rust
struct RangeIter<R: Reader>(RangeIterInner<R>);
```

An iterator for the address ranges of a `DebuggingInformationEntry`.

Returned by `Dwarf::die_ranges` and `Dwarf::unit_ranges`.

#### Implementations

- <span id="rangeiter-next"></span>`fn next(&mut self) -> Result<Option<Range>>` — [`Result`](../index.md#result), [`Range`](#range)

  Advance the iterator to the next range.

#### Trait Implementations

##### `impl<R: fmt::Debug + Reader> Debug for RangeIter<R>`

- <span id="rangeiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: Reader> Default for RangeIter<R>`

- <span id="rangeiter-default"></span>`fn default() -> Self`

##### `impl IntoIterator for RangeIter<R>`

- <span id="rangeiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="rangeiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="rangeiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: Reader> Iterator for RangeIter<R>`

- <span id="rangeiter-iterator-type-item"></span>`type Item = Result<Range, Error>`

- <span id="rangeiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `EndianSlice<'input, Endian>`

```rust
struct EndianSlice<'input, Endian>
where
    Endian: Endianity {
    slice: &'input [u8],
    endian: Endian,
}
```

A `&[u8]` slice with endianity metadata.

This implements the `Reader` trait, which is used for all reading of DWARF sections.

#### Implementations

- <span id="endianslice-new"></span>`fn new(slice: &'input [u8], endian: Endian) -> EndianSlice<'input, Endian>` — [`EndianSlice`](#endianslice)

  Construct a new `EndianSlice` with the given slice and endianity.

- <span id="endianslice-slice"></span>`fn slice(&self) -> &'input [u8]`

  Return a reference to the raw slice.

- <span id="endianslice-split-at"></span>`fn split_at(&self, idx: usize) -> (EndianSlice<'input, Endian>, EndianSlice<'input, Endian>)` — [`EndianSlice`](#endianslice)

  Split the slice in two at the given index, resulting in the tuple where

  the first item has range [0, idx), and the second has range [idx,

  len). Panics if the index is out of bounds.

- <span id="endianslice-find"></span>`fn find(&self, byte: u8) -> Option<usize>`

  Find the first occurrence of a byte in the slice, and return its index.

- <span id="endianslice-offset-from"></span>`fn offset_from(&self, base: EndianSlice<'input, Endian>) -> usize` — [`EndianSlice`](#endianslice)

  Return the offset of the start of the slice relative to the start

  of the given slice.

- <span id="endianslice-to-string"></span>`fn to_string(&self) -> Result<&'input str>` — [`Result`](../index.md#result)

  Converts the slice to a string using `str::from_utf8`.

  

  Returns an error if the slice contains invalid characters.

- <span id="endianslice-to-string-lossy"></span>`fn to_string_lossy(&self) -> Cow<'input, str>`

  Converts the slice to a string, including invalid characters,

  using `String::from_utf8_lossy`.

- <span id="endianslice-read-slice"></span>`fn read_slice(&mut self, len: usize) -> Result<&'input [u8]>` — [`Result`](../index.md#result)

#### Trait Implementations

##### `impl<Endian> Clone for EndianSlice<'input, Endian>`

- <span id="endianslice-clone"></span>`fn clone(&self) -> EndianSlice<'input, Endian>` — [`EndianSlice`](#endianslice)

##### `impl<Endian> Copy for EndianSlice<'input, Endian>`

##### `impl<Endian: Endianity> Debug for EndianSlice<'input, Endian>`

- <span id="endianslice-debug-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> core::result::Result<(), fmt::Error>`

##### `impl<Endian> Default for EndianSlice<'input, Endian>`

- <span id="endianslice-default"></span>`fn default() -> EndianSlice<'input, Endian>` — [`EndianSlice`](#endianslice)

##### `impl<Endian> Deref for EndianSlice<'input, Endian>`

- <span id="endianslice-deref-type-target"></span>`type Target = [u8]`

- <span id="endianslice-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<Endian> Eq for EndianSlice<'input, Endian>`

##### `impl<Endian> Hash for EndianSlice<'input, Endian>`

- <span id="endianslice-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<Endian> PartialEq for EndianSlice<'input, Endian>`

- <span id="endianslice-partialeq-eq"></span>`fn eq(&self, other: &EndianSlice<'input, Endian>) -> bool` — [`EndianSlice`](#endianslice)

##### `impl<Endian> Reader for EndianSlice<'input, Endian>`

- <span id="endianslice-reader-type-endian"></span>`type Endian = Endian`

- <span id="endianslice-reader-type-offset"></span>`type Offset = usize`

- <span id="endianslice-reader-endian"></span>`fn endian(&self) -> Endian`

- <span id="endianslice-reader-len"></span>`fn len(&self) -> usize`

- <span id="endianslice-reader-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="endianslice-reader-empty"></span>`fn empty(&mut self)`

- <span id="endianslice-reader-truncate"></span>`fn truncate(&mut self, len: usize) -> Result<()>` — [`Result`](../index.md#result)

- <span id="endianslice-reader-offset-from"></span>`fn offset_from(&self, base: &Self) -> usize`

- <span id="endianslice-reader-offset-id"></span>`fn offset_id(&self) -> ReaderOffsetId` — [`ReaderOffsetId`](#readeroffsetid)

- <span id="endianslice-reader-lookup-offset-id"></span>`fn lookup_offset_id(&self, id: ReaderOffsetId) -> Option<<Self as >::Offset>` — [`ReaderOffsetId`](#readeroffsetid), [`Reader`](#reader)

- <span id="endianslice-reader-find"></span>`fn find(&self, byte: u8) -> Result<usize>` — [`Result`](../index.md#result)

- <span id="endianslice-reader-skip"></span>`fn skip(&mut self, len: usize) -> Result<()>` — [`Result`](../index.md#result)

- <span id="endianslice-reader-split"></span>`fn split(&mut self, len: usize) -> Result<Self>` — [`Result`](../index.md#result)

- <span id="endianslice-reader-to-slice"></span>`fn to_slice(&self) -> Result<Cow<'_, [u8]>>` — [`Result`](../index.md#result)

- <span id="endianslice-reader-to-string"></span>`fn to_string(&self) -> Result<Cow<'_, str>>` — [`Result`](../index.md#result)

- <span id="endianslice-reader-to-string-lossy"></span>`fn to_string_lossy(&self) -> Result<Cow<'_, str>>` — [`Result`](../index.md#result)

- <span id="endianslice-reader-read-slice"></span>`fn read_slice(&mut self, buf: &mut [u8]) -> Result<()>` — [`Result`](../index.md#result)

##### `impl Receiver for EndianSlice<'input, Endian>`

- <span id="endianslice-receiver-type-target"></span>`type Target = T`

##### `impl<Endian> StructuralPartialEq for EndianSlice<'input, Endian>`

### `DebugBytes<'input>`

```rust
struct DebugBytes<'input>(&'input [u8]);
```

#### Trait Implementations

##### `impl Debug for DebugBytes<'input>`

- <span id="debugbytes-debug-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> core::result::Result<(), fmt::Error>`

### `DebugByte`

```rust
struct DebugByte(u8);
```

#### Trait Implementations

##### `impl Debug for DebugByte`

- <span id="debugbyte-debug-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

### `DebugLen`

```rust
struct DebugLen(usize);
```

#### Trait Implementations

##### `impl Debug for DebugLen`

- <span id="debuglen-debug-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

### `EndianReader<Endian, T>`

```rust
struct EndianReader<Endian, T>
where
    Endian: Endianity,
    T: CloneStableDeref<Target = [u8]> + Debug {
    range: SubRange<T>,
    endian: Endian,
}
```

An easy way to define a custom `Reader` implementation with a reference to a
generic buffer of bytes and an associated endianity.

Note that the whole original buffer is kept alive in memory even if there is
only one reader that references only a handful of bytes from that original
buffer. That is, `EndianReader` will not do any copying, moving, or
compacting in order to free up unused regions of the original buffer. If you
require this kind of behavior, it is up to you to implement `Reader`
directly by-hand.

# Example

Say you have an `mmap`ed file that you want to serve as a `gimli::Reader`.
You can wrap that `mmap`ed file up in a `MmapFile` type and use
`EndianReader<Rc<MmapFile>>` or `EndianReader<Arc<MmapFile>>` as readers as
long as `MmapFile` dereferences to the underlying `[u8]` data.

```rust
use std::io;
use std::ops::Deref;
use std::path::Path;
use std::slice;
use std::sync::Arc;

/// A type that represents an `mmap`ed file.
#[derive(Debug)]
pub struct MmapFile {
    ptr: *const u8,
    len: usize,
}

impl MmapFile {
    pub fn new(path: &Path) -> io::Result<MmapFile> {
        // Call `mmap` and check for errors and all that...
      unimplemented!()
    }
}

impl Drop for MmapFile {
    fn drop(&mut self) {
        // Call `munmap` to clean up after ourselves...
      unimplemented!()
    }
}

// And `MmapFile` can deref to a slice of the `mmap`ed region of memory.
impl Deref for MmapFile {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self.ptr, self.len)
        }
    }
}

/// A type that represents a shared `mmap`ed file.
#[derive(Debug, Clone)]
pub struct ArcMmapFile(Arc<MmapFile>);

// And `ArcMmapFile` can deref to a slice of the `mmap`ed region of memory.
impl Deref for ArcMmapFile {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &self.0
    }
}

// These are both valid for any `Rc` or `Arc`.
unsafe impl gimli::StableDeref for ArcMmapFile {}
unsafe impl gimli::CloneStableDeref for ArcMmapFile {}

/// A `gimli::Reader` that is backed by an `mmap`ed file!
pub type MmapFileReader<Endian> = gimli::EndianReader<Endian, ArcMmapFile>;
fn test(_: &MmapFileReader<gimli::NativeEndian>) { }
```

#### Implementations

- <span id="endianreader-new"></span>`fn new(bytes: T, endian: Endian) -> EndianReader<Endian, T>` — [`EndianReader`](#endianreader)

  Construct a new `EndianReader` with the given bytes.

- <span id="endianreader-bytes"></span>`fn bytes(&self) -> &[u8]`

  Return a reference to the raw bytes underlying this reader.

#### Trait Implementations

##### `impl<Endian, T> Clone for EndianReader<Endian, T>`

- <span id="endianreader-clone"></span>`fn clone(&self) -> EndianReader<Endian, T>` — [`EndianReader`](#endianreader)

##### `impl<Endian, T> Copy for EndianReader<Endian, T>`

##### `impl<Endian, T> Debug for EndianReader<Endian, T>`

- <span id="endianreader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Endian, T> Deref for EndianReader<Endian, T>`

- <span id="endianreader-deref-type-target"></span>`type Target = [u8]`

- <span id="endianreader-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<Endian, T> Eq for EndianReader<Endian, T>`

##### `impl<Endian, T> Hash for EndianReader<Endian, T>`

- <span id="endianreader-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl<Endian, T> Index for EndianReader<Endian, T>`

- <span id="endianreader-index-type-output"></span>`type Output = u8`

- <span id="endianreader-index"></span>`fn index(&self, idx: usize) -> &<Self as >::Output`

##### `impl<Endian, T1, T2> PartialEq for EndianReader<Endian, T1>`

- <span id="endianreader-partialeq-eq"></span>`fn eq(&self, rhs: &EndianReader<Endian, T2>) -> bool` — [`EndianReader`](#endianreader)

##### `impl<Endian, T> Reader for EndianReader<Endian, T>`

- <span id="endianreader-reader-type-endian"></span>`type Endian = Endian`

- <span id="endianreader-reader-type-offset"></span>`type Offset = usize`

- <span id="endianreader-reader-endian"></span>`fn endian(&self) -> Endian`

- <span id="endianreader-reader-len"></span>`fn len(&self) -> usize`

- <span id="endianreader-reader-empty"></span>`fn empty(&mut self)`

- <span id="endianreader-reader-truncate"></span>`fn truncate(&mut self, len: usize) -> Result<()>` — [`Result`](../index.md#result)

- <span id="endianreader-reader-offset-from"></span>`fn offset_from(&self, base: &EndianReader<Endian, T>) -> usize` — [`EndianReader`](#endianreader)

- <span id="endianreader-reader-offset-id"></span>`fn offset_id(&self) -> ReaderOffsetId` — [`ReaderOffsetId`](#readeroffsetid)

- <span id="endianreader-reader-lookup-offset-id"></span>`fn lookup_offset_id(&self, id: ReaderOffsetId) -> Option<<Self as >::Offset>` — [`ReaderOffsetId`](#readeroffsetid), [`Reader`](#reader)

- <span id="endianreader-reader-find"></span>`fn find(&self, byte: u8) -> Result<usize>` — [`Result`](../index.md#result)

- <span id="endianreader-reader-skip"></span>`fn skip(&mut self, len: usize) -> Result<()>` — [`Result`](../index.md#result)

- <span id="endianreader-reader-split"></span>`fn split(&mut self, len: usize) -> Result<Self>` — [`Result`](../index.md#result)

- <span id="endianreader-reader-to-slice"></span>`fn to_slice(&self) -> Result<Cow<'_, [u8]>>` — [`Result`](../index.md#result)

- <span id="endianreader-reader-to-string"></span>`fn to_string(&self) -> Result<Cow<'_, str>>` — [`Result`](../index.md#result)

- <span id="endianreader-reader-to-string-lossy"></span>`fn to_string_lossy(&self) -> Result<Cow<'_, str>>` — [`Result`](../index.md#result)

- <span id="endianreader-reader-read-slice"></span>`fn read_slice(&mut self, buf: &mut [u8]) -> Result<()>` — [`Result`](../index.md#result)

##### `impl<T> Receiver for EndianReader<Endian, T>`

- <span id="endianreader-receiver-type-target"></span>`type Target = T`

### `SubRange<T>`

```rust
struct SubRange<T>
where
    T: CloneStableDeref<Target = [u8]> + Debug {
    bytes: T,
    ptr: *const u8,
    len: usize,
}
```

#### Implementations

- <span id="subrange-new"></span>`fn new(bytes: T) -> Self`

- <span id="subrange-bytes"></span>`fn bytes(&self) -> &[u8]`

- <span id="subrange-len"></span>`fn len(&self) -> usize`

- <span id="subrange-truncate"></span>`fn truncate(&mut self, len: usize)`

- <span id="subrange-skip"></span>`fn skip(&mut self, len: usize)`

- <span id="subrange-read-slice"></span>`fn read_slice(&mut self, len: usize) -> Option<&[u8]>`

#### Trait Implementations

##### `impl<T> Clone for SubRange<T>`

- <span id="subrange-clone"></span>`fn clone(&self) -> SubRange<T>` — [`SubRange`](endian_reader/index.md#subrange)

##### `impl<T> Copy for SubRange<T>`

##### `impl<T> Debug for SubRange<T>`

- <span id="subrange-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Eq for SubRange<T>`

##### `impl<T> Hash for SubRange<T>`

- <span id="subrange-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T> PartialEq for SubRange<T>`

- <span id="subrange-partialeq-eq"></span>`fn eq(&self, other: &SubRange<T>) -> bool` — [`SubRange`](endian_reader/index.md#subrange)

##### `impl<T> Send for SubRange<T>`

##### `impl<T> StructuralPartialEq for SubRange<T>`

##### `impl<T> Sync for SubRange<T>`

### `ReaderOffsetId`

```rust
struct ReaderOffsetId(u64);
```

An identifier for an offset within a section reader.

This is used for error reporting. The meaning of this value is specific to
each reader implementation. The values should be chosen to be unique amongst
all readers. If values are not unique then errors may point to the wrong reader.

#### Trait Implementations

##### `impl Clone for ReaderOffsetId`

- <span id="readeroffsetid-clone"></span>`fn clone(&self) -> ReaderOffsetId` — [`ReaderOffsetId`](#readeroffsetid)

##### `impl Copy for ReaderOffsetId`

##### `impl Debug for ReaderOffsetId`

- <span id="readeroffsetid-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ReaderOffsetId`

##### `impl PartialEq for ReaderOffsetId`

- <span id="readeroffsetid-partialeq-eq"></span>`fn eq(&self, other: &ReaderOffsetId) -> bool` — [`ReaderOffsetId`](#readeroffsetid)

##### `impl StructuralPartialEq for ReaderOffsetId`

### `RelocateReader<R: Reader, T: Relocate<<R as >::Offset>>`

```rust
struct RelocateReader<R: Reader, T: Relocate<<R as >::Offset>> {
    section: R,
    reader: R,
    relocate: T,
}
```

A `Reader` which applies relocations to addresses and offsets.

This is useful for reading sections which contain relocations,
such as those in a relocatable object file.
It is generally not used for reading sections in an executable file.

#### Implementations

- <span id="relocatereader-new"></span>`fn new(section: R, relocate: T) -> Self`

  Create a new `RelocateReader` which applies relocations to the given section reader.

- <span id="relocatereader-inner"></span>`fn inner(&self) -> &R`

  Get a reference to the underlying section reader.

#### Trait Implementations

##### `impl<R: clone::Clone + Reader, T: clone::Clone + Relocate<<R as >::Offset>> Clone for RelocateReader<R, T>`

- <span id="relocatereader-clone"></span>`fn clone(&self) -> RelocateReader<R, T>` — [`RelocateReader`](#relocatereader)

##### `impl<R, T> Debug for RelocateReader<R, T>`

- <span id="relocatereader-debug-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> core::result::Result<(), fmt::Error>`

##### `impl<R, T> Reader for RelocateReader<R, T>`

- <span id="relocatereader-reader-type-endian"></span>`type Endian = <R as Reader>::Endian`

- <span id="relocatereader-reader-type-offset"></span>`type Offset = <R as Reader>::Offset`

- <span id="relocatereader-reader-read-address"></span>`fn read_address(&mut self, address_size: u8) -> Result<u64>` — [`Result`](../index.md#result)

- <span id="relocatereader-reader-read-offset"></span>`fn read_offset(&mut self, format: Format) -> Result<<R as >::Offset>` — [`Format`](../index.md#format), [`Result`](../index.md#result), [`Reader`](#reader)

- <span id="relocatereader-reader-read-sized-offset"></span>`fn read_sized_offset(&mut self, size: u8) -> Result<<R as >::Offset>` — [`Result`](../index.md#result), [`Reader`](#reader)

- <span id="relocatereader-reader-split"></span>`fn split(&mut self, len: <Self as >::Offset) -> Result<Self>` — [`Reader`](#reader), [`Result`](../index.md#result)

- <span id="relocatereader-reader-endian"></span>`fn endian(&self) -> <Self as >::Endian` — [`Reader`](#reader)

- <span id="relocatereader-reader-len"></span>`fn len(&self) -> <Self as >::Offset` — [`Reader`](#reader)

- <span id="relocatereader-reader-empty"></span>`fn empty(&mut self)`

- <span id="relocatereader-reader-truncate"></span>`fn truncate(&mut self, len: <Self as >::Offset) -> Result<()>` — [`Reader`](#reader), [`Result`](../index.md#result)

- <span id="relocatereader-reader-offset-from"></span>`fn offset_from(&self, base: &Self) -> <Self as >::Offset` — [`Reader`](#reader)

- <span id="relocatereader-reader-offset-id"></span>`fn offset_id(&self) -> ReaderOffsetId` — [`ReaderOffsetId`](#readeroffsetid)

- <span id="relocatereader-reader-lookup-offset-id"></span>`fn lookup_offset_id(&self, id: ReaderOffsetId) -> Option<<Self as >::Offset>` — [`ReaderOffsetId`](#readeroffsetid), [`Reader`](#reader)

- <span id="relocatereader-reader-find"></span>`fn find(&self, byte: u8) -> Result<<Self as >::Offset>` — [`Result`](../index.md#result), [`Reader`](#reader)

- <span id="relocatereader-reader-skip"></span>`fn skip(&mut self, len: <Self as >::Offset) -> Result<()>` — [`Reader`](#reader), [`Result`](../index.md#result)

- <span id="relocatereader-reader-to-slice"></span>`fn to_slice(&self) -> Result<Cow<'_, [u8]>>` — [`Result`](../index.md#result)

- <span id="relocatereader-reader-to-string"></span>`fn to_string(&self) -> Result<Cow<'_, str>>` — [`Result`](../index.md#result)

- <span id="relocatereader-reader-to-string-lossy"></span>`fn to_string_lossy(&self) -> Result<Cow<'_, str>>` — [`Result`](../index.md#result)

- <span id="relocatereader-reader-read-slice"></span>`fn read_slice(&mut self, buf: &mut [u8]) -> Result<()>` — [`Result`](../index.md#result)

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

- <span id="debugabbrev-clone"></span>`fn clone(&self) -> DebugAbbrev<R>` — [`DebugAbbrev`](#debugabbrev)

##### `impl<R: marker::Copy> Copy for DebugAbbrev<R>`

##### `impl<R: fmt::Debug> Debug for DebugAbbrev<R>`

- <span id="debugabbrev-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugAbbrev<R>`

- <span id="debugabbrev-default"></span>`fn default() -> DebugAbbrev<R>` — [`DebugAbbrev`](#debugabbrev)

##### `impl<R> Section for DebugAbbrev<R>`

- <span id="debugabbrev-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../index.md#sectionid)

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

- <span id="abbreviationscache-populate"></span>`fn populate<R: Reader>(&mut self, strategy: AbbreviationsCacheStrategy, debug_abbrev: &DebugAbbrev<R>, units: DebugInfoUnitHeadersIter<R>)` — [`AbbreviationsCacheStrategy`](#abbreviationscachestrategy), [`DebugAbbrev`](#debugabbrev), [`DebugInfoUnitHeadersIter`](#debuginfounitheadersiter)

  Parse abbreviations and store them in the cache.

  

  This will iterate over the given units to determine the abbreviations

  offsets. Any existing cache entries are discarded.

  

  Errors during parsing abbreviations are also stored in the cache.

  Errors during iterating over the units are ignored.

- <span id="abbreviationscache-set"></span>`fn set<R: Reader>(&mut self, offset: DebugAbbrevOffset<<R as >::Offset>, abbreviations: Arc<Abbreviations>)` — [`DebugAbbrevOffset`](../index.md#debugabbrevoffset), [`Reader`](#reader), [`Abbreviations`](#abbreviations)

  Set an entry in the abbreviations cache.

  

  This is only required if you want to manually populate the cache.

- <span id="abbreviationscache-get"></span>`fn get<R: Reader>(&self, debug_abbrev: &DebugAbbrev<R>, offset: DebugAbbrevOffset<<R as >::Offset>) -> Result<Arc<Abbreviations>>` — [`DebugAbbrev`](#debugabbrev), [`DebugAbbrevOffset`](../index.md#debugabbrevoffset), [`Reader`](#reader), [`Result`](../index.md#result), [`Abbreviations`](#abbreviations)

  Parse the abbreviations at the given offset.

  

  This uses the cache if possible, but does not update it.

#### Trait Implementations

##### `impl Debug for AbbreviationsCache`

- <span id="abbreviationscache-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AbbreviationsCache`

- <span id="abbreviationscache-default"></span>`fn default() -> AbbreviationsCache` — [`AbbreviationsCache`](#abbreviationscache)

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

- <span id="abbreviations-empty"></span>`fn empty() -> Abbreviations` — [`Abbreviations`](#abbreviations)

  Construct a new, empty set of abbreviations.

- <span id="abbreviations-insert"></span>`fn insert(&mut self, abbrev: Abbreviation) -> ::core::result::Result<(), ()>` — [`Abbreviation`](#abbreviation)

  Insert an abbreviation into the set.

  

  Returns `Ok` if it is the first abbreviation in the set with its code,

  `Err` if the code is a duplicate and there already exists an

  abbreviation in the set with the given abbreviation's code.

- <span id="abbreviations-get"></span>`fn get(&self, code: u64) -> Option<&Abbreviation>` — [`Abbreviation`](#abbreviation)

  Get the abbreviation associated with the given code.

- <span id="abbreviations-parse"></span>`fn parse<R: Reader>(input: &mut R) -> Result<Abbreviations>` — [`Result`](../index.md#result), [`Abbreviations`](#abbreviations)

  Parse a series of abbreviations, terminated by a null abbreviation.

#### Trait Implementations

##### `impl Clone for Abbreviations`

- <span id="abbreviations-clone"></span>`fn clone(&self) -> Abbreviations` — [`Abbreviations`](#abbreviations)

##### `impl Debug for Abbreviations`

- <span id="abbreviations-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Abbreviations`

- <span id="abbreviations-default"></span>`fn default() -> Abbreviations` — [`Abbreviations`](#abbreviations)

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

- <span id="abbreviation-new"></span>`fn new(code: u64, tag: constants::DwTag, has_children: constants::DwChildren, attributes: Attributes) -> Abbreviation` — [`DwTag`](../index.md#dwtag), [`DwChildren`](../index.md#dwchildren), [`Attributes`](abbrev/index.md#attributes), [`Abbreviation`](#abbreviation)

  Construct a new `Abbreviation`.

  

  ### Panics

  

  Panics if `code` is `0`.

- <span id="abbreviation-code"></span>`fn code(&self) -> u64`

  Get this abbreviation's code.

- <span id="abbreviation-tag"></span>`fn tag(&self) -> constants::DwTag` — [`DwTag`](../index.md#dwtag)

  Get this abbreviation's tag.

- <span id="abbreviation-has-children"></span>`fn has_children(&self) -> bool`

  Return true if this abbreviation's type has children, false otherwise.

- <span id="abbreviation-attributes"></span>`fn attributes(&self) -> &[AttributeSpecification]` — [`AttributeSpecification`](#attributespecification)

  Get this abbreviation's attributes.

- <span id="abbreviation-parse-tag"></span>`fn parse_tag<R: Reader>(input: &mut R) -> Result<constants::DwTag>` — [`Result`](../index.md#result), [`DwTag`](../index.md#dwtag)

  Parse an abbreviation's tag.

- <span id="abbreviation-parse-has-children"></span>`fn parse_has_children<R: Reader>(input: &mut R) -> Result<constants::DwChildren>` — [`Result`](../index.md#result), [`DwChildren`](../index.md#dwchildren)

  Parse an abbreviation's "does the type have children?" byte.

- <span id="abbreviation-parse-attributes"></span>`fn parse_attributes<R: Reader>(input: &mut R) -> Result<Attributes>` — [`Result`](../index.md#result), [`Attributes`](abbrev/index.md#attributes)

  Parse a series of attribute specifications, terminated by a null attribute

  specification.

- <span id="abbreviation-parse"></span>`fn parse<R: Reader>(input: &mut R) -> Result<Option<Abbreviation>>` — [`Result`](../index.md#result), [`Abbreviation`](#abbreviation)

  Parse an abbreviation. Return `None` for the null abbreviation, `Some`

  for an actual abbreviation.

#### Trait Implementations

##### `impl Clone for Abbreviation`

- <span id="abbreviation-clone"></span>`fn clone(&self) -> Abbreviation` — [`Abbreviation`](#abbreviation)

##### `impl Debug for Abbreviation`

- <span id="abbreviation-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Abbreviation`

##### `impl PartialEq for Abbreviation`

- <span id="abbreviation-partialeq-eq"></span>`fn eq(&self, other: &Abbreviation) -> bool` — [`Abbreviation`](#abbreviation)

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

- <span id="attributespecification-new"></span>`fn new(name: constants::DwAt, form: constants::DwForm, implicit_const_value: Option<i64>) -> AttributeSpecification` — [`DwAt`](../index.md#dwat), [`DwForm`](../index.md#dwform), [`AttributeSpecification`](#attributespecification)

  Construct a new `AttributeSpecification` from the given name and form

  and implicit const value.

- <span id="attributespecification-name"></span>`fn name(&self) -> constants::DwAt` — [`DwAt`](../index.md#dwat)

  Get the attribute's name.

- <span id="attributespecification-form"></span>`fn form(&self) -> constants::DwForm` — [`DwForm`](../index.md#dwform)

  Get the attribute's form.

- <span id="attributespecification-implicit-const-value"></span>`fn implicit_const_value(&self) -> Option<i64>`

  Get the attribute's implicit const value.

- <span id="attributespecification-size"></span>`fn size<R: Reader>(&self, header: &UnitHeader<R>) -> Option<usize>` — [`UnitHeader`](#unitheader)

  Return the size of the attribute, in bytes.

  

  Note that because some attributes are variably sized, the size cannot

  always be known without parsing, in which case we return `None`.

- <span id="attributespecification-parse"></span>`fn parse<R: Reader>(input: &mut R) -> Result<Option<AttributeSpecification>>` — [`Result`](../index.md#result), [`AttributeSpecification`](#attributespecification)

  Parse an attribute specification. Returns `None` for the null attribute

  specification, `Some` for an actual attribute specification.

#### Trait Implementations

##### `impl Clone for AttributeSpecification`

- <span id="attributespecification-clone"></span>`fn clone(&self) -> AttributeSpecification` — [`AttributeSpecification`](#attributespecification)

##### `impl Copy for AttributeSpecification`

##### `impl Debug for AttributeSpecification`

- <span id="attributespecification-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for AttributeSpecification`

##### `impl FromIterator for Attributes`

- <span id="attributes-fromiterator-from-iter"></span>`fn from_iter<I>(iter: I) -> Attributes` — [`Attributes`](abbrev/index.md#attributes)

##### `impl PartialEq for AttributeSpecification`

- <span id="attributespecification-partialeq-eq"></span>`fn eq(&self, other: &AttributeSpecification) -> bool` — [`AttributeSpecification`](#attributespecification)

##### `impl StructuralPartialEq for AttributeSpecification`

### `DebugAranges<R>`

```rust
struct DebugAranges<R> {
    section: R,
}
```

The `DebugAranges` struct represents the DWARF address range information
found in the `.debug_aranges` section.

#### Implementations

- <span id="debugaranges-new"></span>`fn new(section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugAranges` instance from the data in the `.debug_aranges`

  section.

  

  It is the caller's responsibility to read the `.debug_aranges` section and

  present it as a `&[u8]` slice. That means using some ELF loader on

  Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugAranges, LittleEndian};

  

  let buf = [];

  let read_debug_aranges_section = || &buf;

  let debug_aranges =

      DebugAranges::new(read_debug_aranges_section(), LittleEndian);

  ```

#### Trait Implementations

##### `impl<R: clone::Clone> Clone for DebugAranges<R>`

- <span id="debugaranges-clone"></span>`fn clone(&self) -> DebugAranges<R>` — [`DebugAranges`](#debugaranges)

##### `impl<R: marker::Copy> Copy for DebugAranges<R>`

##### `impl<R: fmt::Debug> Debug for DebugAranges<R>`

- <span id="debugaranges-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugAranges<R>`

- <span id="debugaranges-default"></span>`fn default() -> DebugAranges<R>` — [`DebugAranges`](#debugaranges)

##### `impl<R> Section for DebugAranges<R>`

- <span id="debugaranges-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../index.md#sectionid)

- <span id="debugaranges-section-reader"></span>`fn reader(&self) -> &R`

### `ArangeHeaderIter<R: Reader>`

```rust
struct ArangeHeaderIter<R: Reader> {
    input: R,
    offset: crate::common::DebugArangesOffset<<R as >::Offset>,
}
```

An iterator over the headers of a `.debug_aranges` section.

#### Implementations

- <span id="arangeheaderiter-next"></span>`fn next(&mut self) -> Result<Option<ArangeHeader<R>>>` — [`Result`](../index.md#result), [`ArangeHeader`](#arangeheader)

  Advance the iterator to the next header.

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for ArangeHeaderIter<R>`

- <span id="arangeheaderiter-clone"></span>`fn clone(&self) -> ArangeHeaderIter<R>` — [`ArangeHeaderIter`](#arangeheaderiter)

##### `impl<R: fmt::Debug + Reader> Debug for ArangeHeaderIter<R>`

- <span id="arangeheaderiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for ArangeHeaderIter<R>`

- <span id="arangeheaderiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="arangeheaderiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="arangeheaderiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: Reader> Iterator for ArangeHeaderIter<R>`

- <span id="arangeheaderiter-iterator-type-item"></span>`type Item = Result<ArangeHeader<R>, Error>`

- <span id="arangeheaderiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `ArangeHeader<R, Offset>`

```rust
struct ArangeHeader<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    offset: crate::common::DebugArangesOffset<Offset>,
    encoding: crate::common::Encoding,
    length: Offset,
    debug_info_offset: crate::common::DebugInfoOffset<Offset>,
    entries: R,
}
```

A header for a set of entries in the `.debug_arange` section.

These entries all belong to a single unit.

#### Implementations

- <span id="arangeheader-parse"></span>`fn parse(input: &mut R, offset: DebugArangesOffset<Offset>) -> Result<Self>` — [`DebugArangesOffset`](../index.md#debugarangesoffset), [`Result`](../index.md#result)

- <span id="arangeheader-offset"></span>`fn offset(&self) -> DebugArangesOffset<Offset>` — [`DebugArangesOffset`](../index.md#debugarangesoffset)

  Return the offset of this header within the `.debug_aranges` section.

- <span id="arangeheader-length"></span>`fn length(&self) -> Offset`

  Return the length of this set of entries, including the header.

- <span id="arangeheader-encoding"></span>`fn encoding(&self) -> Encoding` — [`Encoding`](../index.md#encoding)

  Return the encoding parameters for this set of entries.

- <span id="arangeheader-debug-info-offset"></span>`fn debug_info_offset(&self) -> DebugInfoOffset<Offset>` — [`DebugInfoOffset`](../index.md#debuginfooffset)

  Return the offset into the .debug_info section for this set of arange entries.

- <span id="arangeheader-entries"></span>`fn entries(&self) -> ArangeEntryIter<R>` — [`ArangeEntryIter`](#arangeentryiter)

  Return the arange entries in this set.

#### Trait Implementations

##### `impl<R, Offset> Clone for ArangeHeader<R, Offset>`

- <span id="arangeheader-clone"></span>`fn clone(&self) -> ArangeHeader<R, Offset>` — [`ArangeHeader`](#arangeheader)

##### `impl<R, Offset> Debug for ArangeHeader<R, Offset>`

- <span id="arangeheader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Offset> Eq for ArangeHeader<R, Offset>`

##### `impl<R, Offset> PartialEq for ArangeHeader<R, Offset>`

- <span id="arangeheader-partialeq-eq"></span>`fn eq(&self, other: &ArangeHeader<R, Offset>) -> bool` — [`ArangeHeader`](#arangeheader)

##### `impl<R, Offset> StructuralPartialEq for ArangeHeader<R, Offset>`

### `ArangeEntryIter<R: Reader>`

```rust
struct ArangeEntryIter<R: Reader> {
    input: R,
    encoding: crate::common::Encoding,
}
```

An iterator over the aranges from a `.debug_aranges` section.

#### Implementations

- <span id="arangeentryiter-next"></span>`fn next(&mut self) -> Result<Option<ArangeEntry>>` — [`Result`](../index.md#result), [`ArangeEntry`](#arangeentry)

  Advance the iterator and return the next arange.

  

  Returns the newly parsed arange as `Ok(Some(arange))`. Returns `Ok(None)`

  when iteration is complete and all aranges have already been parsed and

  yielded. If an error occurs while parsing the next arange, then this error

  is returned as `Err(e)`, and all subsequent calls return `Ok(None)`.

- <span id="arangeentryiter-next-raw"></span>`fn next_raw(&mut self) -> Result<Option<ArangeEntry>>` — [`Result`](../index.md#result), [`ArangeEntry`](#arangeentry)

  Advance the iterator and return the next arange without validating it.

  

  The returned entry will have `range.end` set to 0.

  This will return tombstone entries as well.

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for ArangeEntryIter<R>`

- <span id="arangeentryiter-clone"></span>`fn clone(&self) -> ArangeEntryIter<R>` — [`ArangeEntryIter`](#arangeentryiter)

##### `impl<R: fmt::Debug + Reader> Debug for ArangeEntryIter<R>`

- <span id="arangeentryiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for ArangeEntryIter<R>`

- <span id="arangeentryiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="arangeentryiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="arangeentryiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: Reader> Iterator for ArangeEntryIter<R>`

- <span id="arangeentryiter-iterator-type-item"></span>`type Item = Result<ArangeEntry, Error>`

- <span id="arangeentryiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `ArangeEntry`

```rust
struct ArangeEntry {
    range: crate::read::Range,
    length: u64,
}
```

A single parsed arange.

#### Implementations

- <span id="arangeentry-parse"></span>`fn parse<R: Reader>(input: &mut R, encoding: Encoding) -> Result<Option<Self>>` — [`Encoding`](../index.md#encoding), [`Result`](../index.md#result)

  Parse a single arange. Return `None` for the null arange, `Some` for an actual arange.

- <span id="arangeentry-address"></span>`fn address(&self) -> u64`

  Return the beginning address of this arange.

- <span id="arangeentry-length"></span>`fn length(&self) -> u64`

  Return the length of this arange.

- <span id="arangeentry-range"></span>`fn range(&self) -> Range` — [`Range`](#range)

  Return the range.

#### Trait Implementations

##### `impl Clone for ArangeEntry`

- <span id="arangeentry-clone"></span>`fn clone(&self) -> ArangeEntry` — [`ArangeEntry`](#arangeentry)

##### `impl Debug for ArangeEntry`

- <span id="arangeentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ArangeEntry`

##### `impl Ord for ArangeEntry`

- <span id="arangeentry-ord-cmp"></span>`fn cmp(&self, other: &ArangeEntry) -> cmp::Ordering` — [`ArangeEntry`](#arangeentry)

##### `impl PartialEq for ArangeEntry`

- <span id="arangeentry-partialeq-eq"></span>`fn eq(&self, other: &ArangeEntry) -> bool` — [`ArangeEntry`](#arangeentry)

##### `impl PartialOrd for ArangeEntry`

- <span id="arangeentry-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &ArangeEntry) -> option::Option<cmp::Ordering>` — [`ArangeEntry`](#arangeentry)

##### `impl StructuralPartialEq for ArangeEntry`

### `DebugCuIndex<R>`

```rust
struct DebugCuIndex<R> {
    section: R,
}
```

The data in the `.debug_cu_index` section of a `.dwp` file.

This section contains the compilation unit index.

#### Implementations

- <span id="debugcuindex-new"></span>`fn new(section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugCuIndex` instance from the data in the `.debug_cu_index`

  section.

#### Trait Implementations

##### `impl<R: clone::Clone> Clone for DebugCuIndex<R>`

- <span id="debugcuindex-clone"></span>`fn clone(&self) -> DebugCuIndex<R>` — [`DebugCuIndex`](#debugcuindex)

##### `impl<R: marker::Copy> Copy for DebugCuIndex<R>`

##### `impl<R: fmt::Debug> Debug for DebugCuIndex<R>`

- <span id="debugcuindex-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugCuIndex<R>`

- <span id="debugcuindex-default"></span>`fn default() -> DebugCuIndex<R>` — [`DebugCuIndex`](#debugcuindex)

##### `impl<R> Section for DebugCuIndex<R>`

- <span id="debugcuindex-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../index.md#sectionid)

- <span id="debugcuindex-section-reader"></span>`fn reader(&self) -> &R`

### `DebugTuIndex<R>`

```rust
struct DebugTuIndex<R> {
    section: R,
}
```

The data in the `.debug_tu_index` section of a `.dwp` file.

This section contains the type unit index.

#### Implementations

- <span id="debugtuindex-new"></span>`fn new(section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugTuIndex` instance from the data in the `.debug_tu_index`

  section.

#### Trait Implementations

##### `impl<R: clone::Clone> Clone for DebugTuIndex<R>`

- <span id="debugtuindex-clone"></span>`fn clone(&self) -> DebugTuIndex<R>` — [`DebugTuIndex`](#debugtuindex)

##### `impl<R: marker::Copy> Copy for DebugTuIndex<R>`

##### `impl<R: fmt::Debug> Debug for DebugTuIndex<R>`

- <span id="debugtuindex-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugTuIndex<R>`

- <span id="debugtuindex-default"></span>`fn default() -> DebugTuIndex<R>` — [`DebugTuIndex`](#debugtuindex)

##### `impl<R> Section for DebugTuIndex<R>`

- <span id="debugtuindex-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../index.md#sectionid)

- <span id="debugtuindex-section-reader"></span>`fn reader(&self) -> &R`

### `UnitIndex<R: Reader>`

```rust
struct UnitIndex<R: Reader> {
    version: u16,
    section_count: u32,
    unit_count: u32,
    slot_count: u32,
    hash_ids: R,
    hash_rows: R,
    sections: [IndexSectionId; 8],
    offsets: R,
    sizes: R,
}
```

The partially parsed index from a `DebugCuIndex` or `DebugTuIndex`.

#### Implementations

- <span id="unitindex-parse"></span>`fn parse(input: R) -> Result<UnitIndex<R>>` — [`Result`](../index.md#result), [`UnitIndex`](#unitindex)

- <span id="unitindex-find"></span>`fn find(&self, id: u64) -> Option<u32>`

  Find `id` in the index hash table, and return the row index.

  

  `id` may be a compilation unit ID if this index is from `.debug_cu_index`,

  or a type signature if this index is from `.debug_tu_index`.

- <span id="unitindex-sections"></span>`fn sections(&self, row: u32) -> Result<UnitIndexSectionIterator<'_, R>>` — [`Result`](../index.md#result), [`UnitIndexSectionIterator`](#unitindexsectioniterator)

  Return the section offsets and sizes for the given row index.

- <span id="unitindex-version"></span>`fn version(&self) -> u16`

  Return the version.

  

  Defaults to 0 for empty sections.

- <span id="unitindex-section-count"></span>`fn section_count(&self) -> u32`

  Return the number of sections.

- <span id="unitindex-unit-count"></span>`fn unit_count(&self) -> u32`

  Return the number of units.

- <span id="unitindex-slot-count"></span>`fn slot_count(&self) -> u32`

  Return the number of slots.

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for UnitIndex<R>`

- <span id="unitindex-clone"></span>`fn clone(&self) -> UnitIndex<R>` — [`UnitIndex`](#unitindex)

##### `impl<R: fmt::Debug + Reader> Debug for UnitIndex<R>`

- <span id="unitindex-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `UnitIndexSectionIterator<'index, R: Reader>`

```rust
struct UnitIndexSectionIterator<'index, R: Reader> {
    sections: slice::Iter<'index, IndexSectionId>,
    offsets: R,
    sizes: R,
}
```

An iterator over the section offsets and sizes for a row in a `UnitIndex`.

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for UnitIndexSectionIterator<'index, R>`

- <span id="unitindexsectioniterator-clone"></span>`fn clone(&self) -> UnitIndexSectionIterator<'index, R>` — [`UnitIndexSectionIterator`](#unitindexsectioniterator)

##### `impl<R: fmt::Debug + Reader> Debug for UnitIndexSectionIterator<'index, R>`

- <span id="unitindexsectioniterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for UnitIndexSectionIterator<'index, R>`

- <span id="unitindexsectioniterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="unitindexsectioniterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="unitindexsectioniterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: Reader> Iterator for UnitIndexSectionIterator<'index, R>`

- <span id="unitindexsectioniterator-iterator-type-item"></span>`type Item = UnitIndexSection`

- <span id="unitindexsectioniterator-iterator-next"></span>`fn next(&mut self) -> Option<UnitIndexSection>` — [`UnitIndexSection`](#unitindexsection)

### `UnitIndexSection`

```rust
struct UnitIndexSection {
    pub section: IndexSectionId,
    pub offset: u32,
    pub size: u32,
}
```

Information about a unit's contribution to a section in a `.dwp` file.

#### Fields

- **`section`**: `IndexSectionId`

  The section kind.

- **`offset`**: `u32`

  The base offset of the unit's contribution to the section.

- **`size`**: `u32`

  The size of the unit's contribution to the section.

#### Trait Implementations

##### `impl Clone for UnitIndexSection`

- <span id="unitindexsection-clone"></span>`fn clone(&self) -> UnitIndexSection` — [`UnitIndexSection`](#unitindexsection)

##### `impl Copy for UnitIndexSection`

##### `impl Debug for UnitIndexSection`

- <span id="unitindexsection-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for UnitIndexSection`

##### `impl PartialEq for UnitIndexSection`

- <span id="unitindexsection-partialeq-eq"></span>`fn eq(&self, other: &UnitIndexSection) -> bool` — [`UnitIndexSection`](#unitindexsection)

##### `impl StructuralPartialEq for UnitIndexSection`

### `DebugLine<R>`

```rust
struct DebugLine<R> {
    debug_line_section: R,
}
```

The `DebugLine` struct contains the source location to instruction mapping
found in the `.debug_line` section.

#### Implementations

- <span id="debugline-new"></span>`fn new(debug_line_section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugLine` instance from the data in the `.debug_line`

  section.

  

  It is the caller's responsibility to read the `.debug_line` section and

  present it as a `&[u8]` slice. That means using some ELF loader on

  Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugLine, LittleEndian};

  

  let buf = [0x00, 0x01, 0x02, 0x03];

  let read_debug_line_section_somehow = || &buf;

  let debug_line = DebugLine::new(read_debug_line_section_somehow(), LittleEndian);

  ```

#### Trait Implementations

##### `impl<R: clone::Clone> Clone for DebugLine<R>`

- <span id="debugline-clone"></span>`fn clone(&self) -> DebugLine<R>` — [`DebugLine`](#debugline)

##### `impl<R: marker::Copy> Copy for DebugLine<R>`

##### `impl<R: fmt::Debug> Debug for DebugLine<R>`

- <span id="debugline-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugLine<R>`

- <span id="debugline-default"></span>`fn default() -> DebugLine<R>` — [`DebugLine`](#debugline)

##### `impl<R> Section for DebugLine<R>`

- <span id="debugline-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../index.md#sectionid)

- <span id="debugline-section-reader"></span>`fn reader(&self) -> &R`

### `LineRows<R, Program, Offset>`

```rust
struct LineRows<R, Program, Offset>
where
    Program: LineProgram<R, Offset>,
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    program: Program,
    row: LineRow,
    instructions: LineInstructions<R>,
}
```

Executes a `LineProgram` to iterate over the rows in the matrix of line number information.

"The hypothetical machine used by a consumer of the line number information
to expand the byte-coded instruction stream into a matrix of line number
information." -- Section 6.2.1

#### Implementations

- <span id="linerows-new"></span>`fn new(program: IncompleteLineProgram<R, Offset>) -> LineRows<R, IncompleteLineProgram<R, Offset>, Offset>` — [`IncompleteLineProgram`](#incompletelineprogram), [`LineRows`](#linerows)

- <span id="linerows-resume"></span>`fn resume<'program>(program: &'program CompleteLineProgram<R, Offset>, sequence: &LineSequence<R>) -> LineRows<R, &'program CompleteLineProgram<R, Offset>, Offset>` — [`CompleteLineProgram`](#completelineprogram), [`LineSequence`](#linesequence), [`LineRows`](#linerows)

- <span id="linerows-header"></span>`fn header(&self) -> &LineProgramHeader<R, Offset>` — [`LineProgramHeader`](#lineprogramheader)

  Get a reference to the header for this state machine's line number

  program.

- <span id="linerows-next-row"></span>`fn next_row(&mut self) -> Result<Option<(&LineProgramHeader<R, Offset>, &LineRow)>>` — [`Result`](../index.md#result), [`LineProgramHeader`](#lineprogramheader), [`LineRow`](#linerow)

  Parse and execute the next instructions in the line number program until

  another row in the line number matrix is computed.

  

  The freshly computed row is returned as `Ok(Some((header, row)))`.

  If the matrix is complete, and there are no more new rows in the line

  number matrix, then `Ok(None)` is returned. If there was an error parsing

  an instruction, then `Err(e)` is returned.

  

  Unfortunately, the references mean that this cannot be a

  `Iterator`.

#### Trait Implementations

##### `impl<R, Program, Offset> Clone for LineRows<R, Program, Offset>`

- <span id="linerows-clone"></span>`fn clone(&self) -> LineRows<R, Program, Offset>` — [`LineRows`](#linerows)

##### `impl<R, Program, Offset> Debug for LineRows<R, Program, Offset>`

- <span id="linerows-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `LineInstructions<R: Reader>`

```rust
struct LineInstructions<R: Reader> {
    input: R,
}
```

An iterator yielding parsed instructions.

See
[`LineProgramHeader::instructions`](./struct.LineProgramHeader.html#method.instructions)
for more details.

#### Implementations

- <span id="lineinstructions-remove-trailing"></span>`fn remove_trailing(&self, other: &LineInstructions<R>) -> Result<LineInstructions<R>>` — [`LineInstructions`](#lineinstructions), [`Result`](../index.md#result)

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for LineInstructions<R>`

- <span id="lineinstructions-clone"></span>`fn clone(&self) -> LineInstructions<R>` — [`LineInstructions`](#lineinstructions)

##### `impl<R: fmt::Debug + Reader> Debug for LineInstructions<R>`

- <span id="lineinstructions-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `LineRow`

```rust
struct LineRow {
    tombstone: bool,
    address: u64,
    op_index: core::num::Wrapping<u64>,
    file: u64,
    line: core::num::Wrapping<u64>,
    column: u64,
    is_stmt: bool,
    basic_block: bool,
    end_sequence: bool,
    prologue_end: bool,
    epilogue_begin: bool,
    isa: u64,
    discriminator: u64,
}
```

A row in the line number program's resulting matrix.

Each row is a copy of the registers of the state machine, as defined in section 6.2.2.

#### Implementations

- <span id="linerow-new"></span>`fn new<R: Reader>(header: &LineProgramHeader<R>) -> Self` — [`LineProgramHeader`](#lineprogramheader)

  Create a line number row in the initial state for the given program.

- <span id="linerow-address"></span>`fn address(&self) -> u64`

  "The program-counter value corresponding to a machine instruction

  generated by the compiler."

- <span id="linerow-op-index"></span>`fn op_index(&self) -> u64`

  > An unsigned integer representing the index of an operation within a VLIW

  > instruction. The index of the first operation is 0. For non-VLIW

  > architectures, this register will always be 0.

  >

  > The address and op_index registers, taken together, form an operation

  > pointer that can reference any individual operation with the

  > instruction stream.

- <span id="linerow-file-index"></span>`fn file_index(&self) -> u64`

  "An unsigned integer indicating the identity of the source file

  corresponding to a machine instruction."

- <span id="linerow-file"></span>`fn file<'header, R: Reader>(&self, header: &'header LineProgramHeader<R>) -> Option<&'header FileEntry<R>>` — [`LineProgramHeader`](#lineprogramheader), [`FileEntry`](#fileentry)

  The source file corresponding to the current machine instruction.

- <span id="linerow-line"></span>`fn line(&self) -> Option<NonZeroU64>`

  "An unsigned integer indicating a source line number. Lines are numbered

  beginning at 1. The compiler may emit the value 0 in cases where an

  instruction cannot be attributed to any source line."

  Line number values of 0 are represented as `None`.

- <span id="linerow-column"></span>`fn column(&self) -> ColumnType` — [`ColumnType`](#columntype)

  "An unsigned integer indicating a column number within a source

  line. Columns are numbered beginning at 1. The value 0 is reserved to

  indicate that a statement begins at the “left edge” of the line."

- <span id="linerow-is-stmt"></span>`fn is_stmt(&self) -> bool`

  "A boolean indicating that the current instruction is a recommended

  breakpoint location. A recommended breakpoint location is intended to

  “represent” a line, a statement and/or a semantically distinct subpart

  of a statement."

- <span id="linerow-basic-block"></span>`fn basic_block(&self) -> bool`

  "A boolean indicating that the current instruction is the beginning of a

  basic block."

- <span id="linerow-end-sequence"></span>`fn end_sequence(&self) -> bool`

  "A boolean indicating that the current address is that of the first byte

  after the end of a sequence of target machine instructions. end_sequence

  terminates a sequence of lines; therefore other information in the same

  row is not meaningful."

- <span id="linerow-prologue-end"></span>`fn prologue_end(&self) -> bool`

  "A boolean indicating that the current address is one (of possibly many)

  where execution should be suspended for an entry breakpoint of a

  function."

- <span id="linerow-epilogue-begin"></span>`fn epilogue_begin(&self) -> bool`

  "A boolean indicating that the current address is one (of possibly many)

  where execution should be suspended for an exit breakpoint of a

  function."

- <span id="linerow-isa"></span>`fn isa(&self) -> u64`

  Tag for the current instruction set architecture.

  

  > An unsigned integer whose value encodes the applicable instruction set

  > architecture for the current instruction.

  >

  > The encoding of instruction sets should be shared by all users of a

  > given architecture. It is recommended that this encoding be defined by

  > the ABI authoring committee for each architecture.

- <span id="linerow-discriminator"></span>`fn discriminator(&self) -> u64`

  "An unsigned integer identifying the block to which the current

  instruction belongs. Discriminator values are assigned arbitrarily by

  the DWARF producer and serve to distinguish among multiple blocks that

  may all be associated with the same source file, line, and column. Where

  only one block exists for a given source position, the discriminator

  value should be zero."

- <span id="linerow-execute"></span>`fn execute<R, Program>(&mut self, instruction: LineInstruction<R>, program: &mut Program) -> Result<bool>` — [`LineInstruction`](#lineinstruction), [`Result`](../index.md#result)

  Execute the given instruction, and return true if a new row in the

  line number matrix needs to be generated.

  

  Unknown opcodes are treated as no-ops.

- <span id="linerow-reset"></span>`fn reset<R: Reader>(&mut self, header: &LineProgramHeader<R>)` — [`LineProgramHeader`](#lineprogramheader)

  Perform any reset that was required after copying the previous row.

- <span id="linerow-apply-line-advance"></span>`fn apply_line_advance(&mut self, line_increment: i64)`

  Step 1 of section 6.2.5.1

- <span id="linerow-apply-operation-advance"></span>`fn apply_operation_advance<R: Reader>(&mut self, operation_advance: u64, header: &LineProgramHeader<R>) -> Result<()>` — [`LineProgramHeader`](#lineprogramheader), [`Result`](../index.md#result)

  Step 2 of section 6.2.5.1

- <span id="linerow-adjust-opcode"></span>`fn adjust_opcode<R: Reader>(&self, opcode: u8, header: &LineProgramHeader<R>) -> u8` — [`LineProgramHeader`](#lineprogramheader)

- <span id="linerow-exec-special-opcode"></span>`fn exec_special_opcode<R: Reader>(&mut self, opcode: u8, header: &LineProgramHeader<R>) -> Result<()>` — [`LineProgramHeader`](#lineprogramheader), [`Result`](../index.md#result)

  Section 6.2.5.1

#### Trait Implementations

##### `impl Clone for LineRow`

- <span id="linerow-clone"></span>`fn clone(&self) -> LineRow` — [`LineRow`](#linerow)

##### `impl Copy for LineRow`

##### `impl Debug for LineRow`

- <span id="linerow-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for LineRow`

##### `impl PartialEq for LineRow`

- <span id="linerow-partialeq-eq"></span>`fn eq(&self, other: &LineRow) -> bool` — [`LineRow`](#linerow)

##### `impl StructuralPartialEq for LineRow`

### `LineSequence<R: Reader>`

```rust
struct LineSequence<R: Reader> {
    pub start: u64,
    pub end: u64,
    instructions: LineInstructions<R>,
}
```

A sequence within a line number program.  A sequence, as defined in section
6.2.5 of the standard, is a linear subset of a line number program within
which addresses are monotonically increasing.

#### Fields

- **`start`**: `u64`

  The first address that is covered by this sequence within the line number
  program.

- **`end`**: `u64`

  The first address that is *not* covered by this sequence within the line
  number program.

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for LineSequence<R>`

- <span id="linesequence-clone"></span>`fn clone(&self) -> LineSequence<R>` — [`LineSequence`](#linesequence)

##### `impl<R: fmt::Debug + Reader> Debug for LineSequence<R>`

- <span id="linesequence-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `LineProgramHeader<R, Offset>`

```rust
struct LineProgramHeader<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    encoding: crate::common::Encoding,
    offset: crate::common::DebugLineOffset<Offset>,
    unit_length: Offset,
    header_length: Offset,
    line_encoding: crate::common::LineEncoding,
    opcode_base: u8,
    standard_opcode_lengths: R,
    directory_entry_format: alloc::vec::Vec<FileEntryFormat>,
    include_directories: alloc::vec::Vec<crate::read::AttributeValue<R, Offset>>,
    file_name_entry_format: alloc::vec::Vec<FileEntryFormat>,
    file_names: alloc::vec::Vec<FileEntry<R, Offset>>,
    program_buf: R,
    comp_dir: Option<R>,
    comp_file: Option<FileEntry<R, Offset>>,
}
```

A header for a line number program in the `.debug_line` section, as defined
in section 6.2.4 of the standard.

#### Fields

- **`opcode_base`**: `u8`

  "The number assigned to the first special opcode."

- **`standard_opcode_lengths`**: `R`

  "This array specifies the number of LEB128 operands for each of the
  standard opcodes. The first element of the array corresponds to the
  opcode whose value is 1, and the last element corresponds to the opcode
  whose value is `opcode_base - 1`."

- **`directory_entry_format`**: `alloc::vec::Vec<FileEntryFormat>`

  "A sequence of directory entry format descriptions."

- **`include_directories`**: `alloc::vec::Vec<crate::read::AttributeValue<R, Offset>>`

  > Entries in this sequence describe each path that was searched for
  > included source files in this compilation. (The paths include those
  > directories specified explicitly by the user for the compiler to search
  > and those the compiler searches without explicit direction.) Each path
  > entry is either a full path name or is relative to the current directory
  > of the compilation.
  >
  > The last entry is followed by a single null byte.

- **`file_name_entry_format`**: `alloc::vec::Vec<FileEntryFormat>`

  "A sequence of file entry format descriptions."

- **`file_names`**: `alloc::vec::Vec<FileEntry<R, Offset>>`

  "Entries in this sequence describe source files that contribute to the
  line number information for this compilation unit or is used in other
  contexts."

- **`program_buf`**: `R`

  The encoded line program instructions.

- **`comp_dir`**: `Option<R>`

  The current directory of the compilation.

- **`comp_file`**: `Option<FileEntry<R, Offset>>`

  The primary source file.

#### Implementations

- <span id="lineprogramheader-offset"></span>`fn offset(&self) -> DebugLineOffset<<R as >::Offset>` — [`DebugLineOffset`](../index.md#debuglineoffset), [`Reader`](#reader)

  Return the offset of the line number program header in the `.debug_line` section.

- <span id="lineprogramheader-unit-length"></span>`fn unit_length(&self) -> <R as >::Offset` — [`Reader`](#reader)

  Return the length of the line number program and header, not including

  the length of the encoded length itself.

- <span id="lineprogramheader-encoding"></span>`fn encoding(&self) -> Encoding` — [`Encoding`](../index.md#encoding)

  Return the encoding parameters for this header's line program.

- <span id="lineprogramheader-version"></span>`fn version(&self) -> u16`

  Get the version of this header's line program.

- <span id="lineprogramheader-header-length"></span>`fn header_length(&self) -> <R as >::Offset` — [`Reader`](#reader)

  Get the length of the encoded line number program header, not including

  the length of the encoded length itself.

- <span id="lineprogramheader-address-size"></span>`fn address_size(&self) -> u8`

  Get the size in bytes of a target machine address.

- <span id="lineprogramheader-format"></span>`fn format(&self) -> Format` — [`Format`](../index.md#format)

  Whether this line program is encoded in 64- or 32-bit DWARF.

- <span id="lineprogramheader-line-encoding"></span>`fn line_encoding(&self) -> LineEncoding` — [`LineEncoding`](../index.md#lineencoding)

  Get the line encoding parameters for this header's line program.

- <span id="lineprogramheader-minimum-instruction-length"></span>`fn minimum_instruction_length(&self) -> u8`

  Get the minimum instruction length any instruction in this header's line

  program may have.

- <span id="lineprogramheader-maximum-operations-per-instruction"></span>`fn maximum_operations_per_instruction(&self) -> u8`

  Get the maximum number of operations each instruction in this header's

  line program may have.

- <span id="lineprogramheader-default-is-stmt"></span>`fn default_is_stmt(&self) -> bool`

  Get the default value of the `is_stmt` register for this header's line

  program.

- <span id="lineprogramheader-line-base"></span>`fn line_base(&self) -> i8`

  Get the line base for this header's line program.

- <span id="lineprogramheader-line-range"></span>`fn line_range(&self) -> u8`

  Get the line range for this header's line program.

- <span id="lineprogramheader-opcode-base"></span>`fn opcode_base(&self) -> u8`

  Get opcode base for this header's line program.

- <span id="lineprogramheader-standard-opcode-lengths"></span>`fn standard_opcode_lengths(&self) -> &R`

  An array of `u8` that specifies the number of LEB128 operands for

  each of the standard opcodes.

- <span id="lineprogramheader-directory-entry-format"></span>`fn directory_entry_format(&self) -> &[FileEntryFormat]` — [`FileEntryFormat`](#fileentryformat)

  Get the format of a directory entry.

- <span id="lineprogramheader-include-directories"></span>`fn include_directories(&self) -> &[AttributeValue<R, Offset>]` — [`AttributeValue`](#attributevalue)

  Get the set of include directories for this header's line program.

  

  For DWARF version <= 4, the compilation's current directory is not included

  in the return value, but is implicitly considered to be in the set per spec.

- <span id="lineprogramheader-directory"></span>`fn directory(&self, directory: u64) -> Option<AttributeValue<R, Offset>>` — [`AttributeValue`](#attributevalue)

  The include directory with the given directory index.

  

  A directory index of 0 corresponds to the compilation unit directory.

- <span id="lineprogramheader-file-name-entry-format"></span>`fn file_name_entry_format(&self) -> &[FileEntryFormat]` — [`FileEntryFormat`](#fileentryformat)

  Get the format of a file name entry.

- <span id="lineprogramheader-file-has-timestamp"></span>`fn file_has_timestamp(&self) -> bool`

  Return true if the file entries may have valid timestamps.

  

  Only returns false if we definitely know that all timestamp fields

  are invalid.

- <span id="lineprogramheader-file-has-size"></span>`fn file_has_size(&self) -> bool`

  Return true if the file entries may have valid sizes.

  

  Only returns false if we definitely know that all size fields

  are invalid.

- <span id="lineprogramheader-file-has-md5"></span>`fn file_has_md5(&self) -> bool`

  Return true if the file name entry format contains an MD5 field.

- <span id="lineprogramheader-file-has-source"></span>`fn file_has_source(&self) -> bool`

  Return true if the file name entry format contains a source field.

- <span id="lineprogramheader-file-names"></span>`fn file_names(&self) -> &[FileEntry<R, Offset>]` — [`FileEntry`](#fileentry)

  Get the list of source files that appear in this header's line program.

- <span id="lineprogramheader-file"></span>`fn file(&self, file: u64) -> Option<&FileEntry<R, Offset>>` — [`FileEntry`](#fileentry)

  The source file with the given file index.

  

  A file index of 0 corresponds to the compilation unit file.

  Note that a file index of 0 is invalid for DWARF version <= 4,

  but we support it anyway.

- <span id="lineprogramheader-raw-program-buf"></span>`fn raw_program_buf(&self) -> R`

  Get the raw, un-parsed `EndianSlice` containing this header's line number

  program.

  

  ```rust

  fn foo() {

  use gimli::{LineProgramHeader, EndianSlice, NativeEndian};

  

  fn get_line_number_program_header<'a>() -> LineProgramHeader<EndianSlice<'a, NativeEndian>> {

      // Get a line number program header from some offset in a

      // `.debug_line` section...

    unimplemented!()

  }

  

  let header = get_line_number_program_header();

  let raw_program = header.raw_program_buf();

  println!("The length of the raw program in bytes is {}", raw_program.len());

  }

  ```

- <span id="lineprogramheader-instructions"></span>`fn instructions(&self) -> LineInstructions<R>` — [`LineInstructions`](#lineinstructions)

  Iterate over the instructions in this header's line number program, parsing

  them as we go.

- <span id="lineprogramheader-parse"></span>`fn parse(input: &mut R, offset: DebugLineOffset<Offset>, address_size: u8, comp_dir: Option<R>, comp_name: Option<R>) -> Result<LineProgramHeader<R, Offset>>` — [`DebugLineOffset`](../index.md#debuglineoffset), [`Result`](../index.md#result), [`LineProgramHeader`](#lineprogramheader)

#### Trait Implementations

##### `impl<R, Offset> Clone for LineProgramHeader<R, Offset>`

- <span id="lineprogramheader-clone"></span>`fn clone(&self) -> LineProgramHeader<R, Offset>` — [`LineProgramHeader`](#lineprogramheader)

##### `impl<R, Offset> Debug for LineProgramHeader<R, Offset>`

- <span id="lineprogramheader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Offset> Eq for LineProgramHeader<R, Offset>`

##### `impl<R, Offset> PartialEq for LineProgramHeader<R, Offset>`

- <span id="lineprogramheader-partialeq-eq"></span>`fn eq(&self, other: &LineProgramHeader<R, Offset>) -> bool` — [`LineProgramHeader`](#lineprogramheader)

##### `impl<R, Offset> StructuralPartialEq for LineProgramHeader<R, Offset>`

### `IncompleteLineProgram<R, Offset>`

```rust
struct IncompleteLineProgram<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    header: LineProgramHeader<R, Offset>,
}
```

A line number program that has not been run to completion.

#### Implementations

- <span id="incompletelineprogram-header"></span>`fn header(&self) -> &LineProgramHeader<R, Offset>` — [`LineProgramHeader`](#lineprogramheader)

  Retrieve the `LineProgramHeader` for this program.

- <span id="incompletelineprogram-rows"></span>`fn rows(self) -> LineRows<R, IncompleteLineProgram<R, Offset>, Offset>` — [`LineRows`](#linerows), [`IncompleteLineProgram`](#incompletelineprogram)

  Construct a new `LineRows` for executing this program to iterate

  over rows in the line information matrix.

- <span id="incompletelineprogram-sequences"></span>`fn sequences(self) -> Result<(CompleteLineProgram<R, Offset>, Vec<LineSequence<R>>)>` — [`Result`](../index.md#result), [`CompleteLineProgram`](#completelineprogram), [`LineSequence`](#linesequence)

  Execute the line number program, completing the `IncompleteLineProgram`

  into a `CompleteLineProgram` and producing an array of sequences within

  the line number program that can later be used with

  `CompleteLineProgram::resume_from`.

  

  ```rust

  fn foo() {

  use gimli::{IncompleteLineProgram, EndianSlice, NativeEndian};

  

  fn get_line_number_program<'a>() -> IncompleteLineProgram<EndianSlice<'a, NativeEndian>> {

      // Get a line number program from some offset in a

      // `.debug_line` section...

    unimplemented!()

  }

  

  let program = get_line_number_program();

  let (program, sequences) = program.sequences().unwrap();

  println!("There are {} sequences in this line number program", sequences.len());

  }

  ```

#### Trait Implementations

##### `impl<R, Offset> Clone for IncompleteLineProgram<R, Offset>`

- <span id="incompletelineprogram-clone"></span>`fn clone(&self) -> IncompleteLineProgram<R, Offset>` — [`IncompleteLineProgram`](#incompletelineprogram)

##### `impl<R, Offset> Debug for IncompleteLineProgram<R, Offset>`

- <span id="incompletelineprogram-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Offset> Eq for IncompleteLineProgram<R, Offset>`

##### `impl<R, Offset> LineProgram for IncompleteLineProgram<R, Offset>`

- <span id="incompletelineprogram-lineprogram-header"></span>`fn header(&self) -> &LineProgramHeader<R, Offset>` — [`LineProgramHeader`](#lineprogramheader)

- <span id="incompletelineprogram-lineprogram-add-file"></span>`fn add_file(&mut self, file: FileEntry<R, Offset>)` — [`FileEntry`](#fileentry)

##### `impl<R, Offset> PartialEq for IncompleteLineProgram<R, Offset>`

- <span id="incompletelineprogram-partialeq-eq"></span>`fn eq(&self, other: &IncompleteLineProgram<R, Offset>) -> bool` — [`IncompleteLineProgram`](#incompletelineprogram)

##### `impl<R, Offset> StructuralPartialEq for IncompleteLineProgram<R, Offset>`

### `CompleteLineProgram<R, Offset>`

```rust
struct CompleteLineProgram<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    header: LineProgramHeader<R, Offset>,
}
```

A line number program that has previously been run to completion.

#### Implementations

- <span id="completelineprogram-header"></span>`fn header(&self) -> &LineProgramHeader<R, Offset>` — [`LineProgramHeader`](#lineprogramheader)

  Retrieve the `LineProgramHeader` for this program.

- <span id="completelineprogram-resume-from"></span>`fn resume_from<'program>(self: &'program Self, sequence: &LineSequence<R>) -> LineRows<R, &'program CompleteLineProgram<R, Offset>, Offset>` — [`LineSequence`](#linesequence), [`LineRows`](#linerows), [`CompleteLineProgram`](#completelineprogram)

  Construct a new `LineRows` for executing the subset of the line

  number program identified by 'sequence' and  generating the line information

  matrix.

  

  ```rust

  fn foo() {

  use gimli::{IncompleteLineProgram, EndianSlice, NativeEndian};

  

  fn get_line_number_program<'a>() -> IncompleteLineProgram<EndianSlice<'a, NativeEndian>> {

      // Get a line number program from some offset in a

      // `.debug_line` section...

    unimplemented!()

  }

  

  let program = get_line_number_program();

  let (program, sequences) = program.sequences().unwrap();

  for sequence in &sequences {

      let mut sm = program.resume_from(sequence);

  }

  }

  ```

#### Trait Implementations

##### `impl<R, Offset> Clone for CompleteLineProgram<R, Offset>`

- <span id="completelineprogram-clone"></span>`fn clone(&self) -> CompleteLineProgram<R, Offset>` — [`CompleteLineProgram`](#completelineprogram)

##### `impl<R, Offset> Debug for CompleteLineProgram<R, Offset>`

- <span id="completelineprogram-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Offset> Eq for CompleteLineProgram<R, Offset>`

##### `impl<R, Offset> LineProgram for &'program CompleteLineProgram<R, Offset>`

- <span id="program-completelineprogram-lineprogram-header"></span>`fn header(&self) -> &LineProgramHeader<R, Offset>` — [`LineProgramHeader`](#lineprogramheader)

- <span id="program-completelineprogram-lineprogram-add-file"></span>`fn add_file(&mut self, _: FileEntry<R, Offset>)` — [`FileEntry`](#fileentry)

##### `impl<R, Offset> PartialEq for CompleteLineProgram<R, Offset>`

- <span id="completelineprogram-partialeq-eq"></span>`fn eq(&self, other: &CompleteLineProgram<R, Offset>) -> bool` — [`CompleteLineProgram`](#completelineprogram)

##### `impl<R, Offset> StructuralPartialEq for CompleteLineProgram<R, Offset>`

### `FileEntry<R, Offset>`

```rust
struct FileEntry<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    path_name: crate::read::AttributeValue<R, Offset>,
    directory_index: u64,
    timestamp: u64,
    size: u64,
    md5: [u8; 16],
    source: Option<crate::read::AttributeValue<R, Offset>>,
}
```

An entry in the `LineProgramHeader`'s `file_names` set.

#### Implementations

- <span id="fileentry-parse"></span>`fn parse(input: &mut R, path_name: R) -> Result<FileEntry<R, Offset>>` — [`Result`](../index.md#result), [`FileEntry`](#fileentry)

- <span id="fileentry-path-name"></span>`fn path_name(&self) -> AttributeValue<R, Offset>` — [`AttributeValue`](#attributevalue)

  > A slice containing the full or relative path name of

  > a source file. If the entry contains a file name or a relative path

  > name, the file is located relative to either the compilation directory

  > (as specified by the DW_AT_comp_dir attribute given in the compilation

  > unit) or one of the directories in the include_directories section.

- <span id="fileentry-directory-index"></span>`fn directory_index(&self) -> u64`

  > An unsigned LEB128 number representing the directory index of the

  > directory in which the file was found.

  >

  > ...

  >

  > The directory index represents an entry in the include_directories

  > section of the line number program header. The index is 0 if the file

  > was found in the current directory of the compilation, 1 if it was found

  > in the first directory in the include_directories section, and so

  > on. The directory index is ignored for file names that represent full

  > path names.

- <span id="fileentry-directory"></span>`fn directory(&self, header: &LineProgramHeader<R>) -> Option<AttributeValue<R, Offset>>` — [`LineProgramHeader`](#lineprogramheader), [`AttributeValue`](#attributevalue)

  Get this file's directory.

  

  A directory index of 0 corresponds to the compilation unit directory.

- <span id="fileentry-timestamp"></span>`fn timestamp(&self) -> u64`

  The implementation-defined time of last modification of the file,

  or 0 if not available.

- <span id="fileentry-size"></span>`fn size(&self) -> u64`

  The size of the file in bytes, or 0 if not available.

- <span id="fileentry-md5"></span>`fn md5(&self) -> &[u8; 16]`

  A 16-byte MD5 digest of the file contents.

  

  Only valid if `LineProgramHeader::file_has_md5` returns `true`.

- <span id="fileentry-source"></span>`fn source(&self) -> Option<AttributeValue<R, Offset>>` — [`AttributeValue`](#attributevalue)

  The source code of this file. (UTF-8 source text string with "\n" line

  endings).

  

  Note: For DWARF v5 files this may return an empty attribute that

  indicates that no source code is available, which this function

  represents as `Some(<zero-length attr>)`.

#### Trait Implementations

##### `impl<R, Offset> Clone for FileEntry<R, Offset>`

- <span id="fileentry-clone"></span>`fn clone(&self) -> FileEntry<R, Offset>` — [`FileEntry`](#fileentry)

##### `impl<R, Offset> Copy for FileEntry<R, Offset>`

##### `impl<R, Offset> Debug for FileEntry<R, Offset>`

- <span id="fileentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Offset> Eq for FileEntry<R, Offset>`

##### `impl<R, Offset> PartialEq for FileEntry<R, Offset>`

- <span id="fileentry-partialeq-eq"></span>`fn eq(&self, other: &FileEntry<R, Offset>) -> bool` — [`FileEntry`](#fileentry)

##### `impl<R, Offset> StructuralPartialEq for FileEntry<R, Offset>`

### `FileEntryFormat`

```rust
struct FileEntryFormat {
    pub content_type: constants::DwLnct,
    pub form: constants::DwForm,
}
```

The format of a component of an include directory or file name entry.

#### Fields

- **`content_type`**: `constants::DwLnct`

  The type of information that is represented by the component.

- **`form`**: `constants::DwForm`

  The encoding form of the component value.

#### Implementations

- <span id="fileentryformat-parse"></span>`fn parse<R: Reader>(input: &mut R) -> Result<Vec<FileEntryFormat>>` — [`Result`](../index.md#result), [`FileEntryFormat`](#fileentryformat)

#### Trait Implementations

##### `impl Clone for FileEntryFormat`

- <span id="fileentryformat-clone"></span>`fn clone(&self) -> FileEntryFormat` — [`FileEntryFormat`](#fileentryformat)

##### `impl Copy for FileEntryFormat`

##### `impl Debug for FileEntryFormat`

- <span id="fileentryformat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for FileEntryFormat`

##### `impl PartialEq for FileEntryFormat`

- <span id="fileentryformat-partialeq-eq"></span>`fn eq(&self, other: &FileEntryFormat) -> bool` — [`FileEntryFormat`](#fileentryformat)

##### `impl StructuralPartialEq for FileEntryFormat`

### `DebugLoc<R>`

```rust
struct DebugLoc<R> {
    section: R,
}
```

The raw contents of the `.debug_loc` section.

#### Implementations

- <span id="debugloc-new"></span>`fn new(section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugLoc` instance from the data in the `.debug_loc`

  section.

  

  It is the caller's responsibility to read the `.debug_loc` section and

  present it as a `&[u8]` slice. That means using some ELF loader on

  Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugLoc, LittleEndian};

  

  let buf = [0x00, 0x01, 0x02, 0x03];

  let read_debug_loc_section_somehow = || &buf;

  let debug_loc = DebugLoc::new(read_debug_loc_section_somehow(), LittleEndian);

  ```

#### Trait Implementations

##### `impl<R: clone::Clone> Clone for DebugLoc<R>`

- <span id="debugloc-clone"></span>`fn clone(&self) -> DebugLoc<R>` — [`DebugLoc`](#debugloc)

##### `impl<R: marker::Copy> Copy for DebugLoc<R>`

##### `impl<R: fmt::Debug> Debug for DebugLoc<R>`

- <span id="debugloc-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugLoc<R>`

- <span id="debugloc-default"></span>`fn default() -> DebugLoc<R>` — [`DebugLoc`](#debugloc)

##### `impl<R> Section for DebugLoc<R>`

- <span id="debugloc-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../index.md#sectionid)

- <span id="debugloc-section-reader"></span>`fn reader(&self) -> &R`

### `DebugLocLists<R>`

```rust
struct DebugLocLists<R> {
    section: R,
}
```

The `DebugLocLists` struct represents the DWARF data
found in the `.debug_loclists` section.

#### Implementations

- <span id="debugloclists-new"></span>`fn new(section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugLocLists` instance from the data in the `.debug_loclists`

  section.

  

  It is the caller's responsibility to read the `.debug_loclists` section and

  present it as a `&[u8]` slice. That means using some ELF loader on

  Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugLocLists, LittleEndian};

  

  let buf = [0x00, 0x01, 0x02, 0x03];

  let read_debug_loclists_section_somehow = || &buf;

  let debug_loclists = DebugLocLists::new(read_debug_loclists_section_somehow(), LittleEndian);

  ```

#### Trait Implementations

##### `impl<R: clone::Clone> Clone for DebugLocLists<R>`

- <span id="debugloclists-clone"></span>`fn clone(&self) -> DebugLocLists<R>` — [`DebugLocLists`](#debugloclists)

##### `impl<R: marker::Copy> Copy for DebugLocLists<R>`

##### `impl<R: fmt::Debug> Debug for DebugLocLists<R>`

- <span id="debugloclists-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugLocLists<R>`

- <span id="debugloclists-default"></span>`fn default() -> DebugLocLists<R>` — [`DebugLocLists`](#debugloclists)

##### `impl<R> Section for DebugLocLists<R>`

- <span id="debugloclists-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../index.md#sectionid)

- <span id="debugloclists-section-reader"></span>`fn reader(&self) -> &R`

### `LocationLists<R>`

```rust
struct LocationLists<R> {
    debug_loc: DebugLoc<R>,
    debug_loclists: DebugLocLists<R>,
}
```

The DWARF data found in `.debug_loc` and `.debug_loclists` sections.

#### Implementations

- <span id="locationlists-new"></span>`fn new(debug_loc: DebugLoc<R>, debug_loclists: DebugLocLists<R>) -> LocationLists<R>` — [`DebugLoc`](#debugloc), [`DebugLocLists`](#debugloclists), [`LocationLists`](#locationlists)

  Construct a new `LocationLists` instance from the data in the `.debug_loc` and

  `.debug_loclists` sections.

#### Trait Implementations

##### `impl<R: clone::Clone> Clone for LocationLists<R>`

- <span id="locationlists-clone"></span>`fn clone(&self) -> LocationLists<R>` — [`LocationLists`](#locationlists)

##### `impl<R: marker::Copy> Copy for LocationLists<R>`

##### `impl<R: fmt::Debug> Debug for LocationLists<R>`

- <span id="locationlists-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for LocationLists<R>`

- <span id="locationlists-default"></span>`fn default() -> LocationLists<R>` — [`LocationLists`](#locationlists)

### `RawLocListIter<R: Reader>`

```rust
struct RawLocListIter<R: Reader> {
    input: R,
    encoding: crate::common::Encoding,
    format: LocListsFormat,
}
```

A raw iterator over a location list.

This iterator does not perform any processing of the location entries,
such as handling base addresses.

#### Implementations

- <span id="rawloclistiter-new"></span>`fn new(input: R, encoding: Encoding, format: LocListsFormat) -> RawLocListIter<R>` — [`Encoding`](../index.md#encoding), [`LocListsFormat`](loclists/index.md#loclistsformat), [`RawLocListIter`](#rawloclistiter)

  Construct a `RawLocListIter`.

- <span id="rawloclistiter-next"></span>`fn next(&mut self) -> Result<Option<RawLocListEntry<R>>>` — [`Result`](../index.md#result), [`RawLocListEntry`](#rawloclistentry)

  Advance the iterator to the next location.

#### Trait Implementations

##### `impl<R: fmt::Debug + Reader> Debug for RawLocListIter<R>`

- <span id="rawloclistiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for RawLocListIter<R>`

- <span id="rawloclistiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="rawloclistiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="rawloclistiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: Reader> Iterator for RawLocListIter<R>`

- <span id="rawloclistiter-iterator-type-item"></span>`type Item = Result<RawLocListEntry<R>, Error>`

- <span id="rawloclistiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `LocListIter<R: Reader>`

```rust
struct LocListIter<R: Reader> {
    raw: RawLocListIter<R>,
    base_address: u64,
    debug_addr: crate::read::DebugAddr<R>,
    debug_addr_base: crate::common::DebugAddrBase<<R as >::Offset>,
}
```

An iterator over a location list.

This iterator internally handles processing of base address selection entries
and list end entries.  Thus, it only returns location entries that are valid
and already adjusted for the base address.

#### Implementations

- <span id="loclistiter-new"></span>`fn new(raw: RawLocListIter<R>, base_address: u64, debug_addr: DebugAddr<R>, debug_addr_base: DebugAddrBase<<R as >::Offset>) -> LocListIter<R>` — [`RawLocListIter`](#rawloclistiter), [`DebugAddr`](#debugaddr), [`DebugAddrBase`](../index.md#debugaddrbase), [`Reader`](#reader), [`LocListIter`](#loclistiter)

  Construct a `LocListIter`.

- <span id="loclistiter-get-address"></span>`fn get_address(&self, index: DebugAddrIndex<<R as >::Offset>) -> Result<u64>` — [`DebugAddrIndex`](../index.md#debugaddrindex), [`Reader`](#reader), [`Result`](../index.md#result)

- <span id="loclistiter-next"></span>`fn next(&mut self) -> Result<Option<LocationListEntry<R>>>` — [`Result`](../index.md#result), [`LocationListEntry`](#locationlistentry)

  Advance the iterator to the next location.

#### Trait Implementations

##### `impl<R: fmt::Debug + Reader> Debug for LocListIter<R>`

- <span id="loclistiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for LocListIter<R>`

- <span id="loclistiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="loclistiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="loclistiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: Reader> Iterator for LocListIter<R>`

- <span id="loclistiter-iterator-type-item"></span>`type Item = Result<LocationListEntry<R>, Error>`

- <span id="loclistiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `LocationListEntry<R: Reader>`

```rust
struct LocationListEntry<R: Reader> {
    pub range: crate::read::Range,
    pub data: crate::read::Expression<R>,
}
```

A location list entry from the `.debug_loc` or `.debug_loclists` sections.

#### Fields

- **`range`**: `crate::read::Range`

  The address range that this location is valid for.

- **`data`**: `crate::read::Expression<R>`

  The data containing a single location description.

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for LocationListEntry<R>`

- <span id="locationlistentry-clone"></span>`fn clone(&self) -> LocationListEntry<R>` — [`LocationListEntry`](#locationlistentry)

##### `impl<R: marker::Copy + Reader> Copy for LocationListEntry<R>`

##### `impl<R: fmt::Debug + Reader> Debug for LocationListEntry<R>`

- <span id="locationlistentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: cmp::Eq + Reader> Eq for LocationListEntry<R>`

##### `impl<R: hash::Hash + Reader> Hash for LocationListEntry<R>`

- <span id="locationlistentry-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<R: cmp::PartialEq + Reader> PartialEq for LocationListEntry<R>`

- <span id="locationlistentry-partialeq-eq"></span>`fn eq(&self, other: &LocationListEntry<R>) -> bool` — [`LocationListEntry`](#locationlistentry)

##### `impl<R: Reader> StructuralPartialEq for LocationListEntry<R>`

### `DebugMacinfo<R>`

```rust
struct DebugMacinfo<R> {
    section: R,
}
```

The raw contents of the `.debug_macinfo` section.

#### Implementations

- <span id="debugmacinfo-new"></span>`fn new(macinfo_section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugMacinfo` instance from the data in the `.debug_macinfo`

  section.

  

  It is the caller's responsibility to read the `.debug_macinfo` section and

  present it as a `&[u8]` slice. That means using some ELF loader on

  Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugMacinfo, LittleEndian};

  

  let buf = [1, 0, 95, 95, 83, 84, 68, 67, 95, 95, 32, 49, 0];

  let read_section_somehow = || &buf;

  let debug_str = DebugMacinfo::new(read_section_somehow(), LittleEndian);

  ```

#### Trait Implementations

##### `impl<R: clone::Clone> Clone for DebugMacinfo<R>`

- <span id="debugmacinfo-clone"></span>`fn clone(&self) -> DebugMacinfo<R>` — [`DebugMacinfo`](#debugmacinfo)

##### `impl<R: marker::Copy> Copy for DebugMacinfo<R>`

##### `impl<R: fmt::Debug> Debug for DebugMacinfo<R>`

- <span id="debugmacinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugMacinfo<R>`

- <span id="debugmacinfo-default"></span>`fn default() -> DebugMacinfo<R>` — [`DebugMacinfo`](#debugmacinfo)

##### `impl<R> Section for DebugMacinfo<R>`

- <span id="debugmacinfo-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../index.md#sectionid)

- <span id="debugmacinfo-section-reader"></span>`fn reader(&self) -> &R`

### `DebugMacro<R>`

```rust
struct DebugMacro<R> {
    section: R,
}
```

The raw contents of the `.debug_macro` section.

#### Implementations

- <span id="debugmacro-new"></span>`fn new(macro_section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugMacro` instance from the data in the `.debug_macro`

  section.

  

  It is the caller's responsibility to read the `.debug_macro` section and

  present it as a `&[u8]` slice. That means using some ELF loader on

  Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugMacro, LittleEndian};

  

  let buf = [1, 0, 95, 95, 83, 84, 68, 67, 95, 95, 32, 49, 0];

  let read_section_somehow = || &buf;

  let debug_str = DebugMacro::new(read_section_somehow(), LittleEndian);

  ```

#### Trait Implementations

##### `impl<R: clone::Clone> Clone for DebugMacro<R>`

- <span id="debugmacro-clone"></span>`fn clone(&self) -> DebugMacro<R>` — [`DebugMacro`](#debugmacro)

##### `impl<R: marker::Copy> Copy for DebugMacro<R>`

##### `impl<R: fmt::Debug> Debug for DebugMacro<R>`

- <span id="debugmacro-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugMacro<R>`

- <span id="debugmacro-default"></span>`fn default() -> DebugMacro<R>` — [`DebugMacro`](#debugmacro)

##### `impl<R> Section for DebugMacro<R>`

- <span id="debugmacro-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../index.md#sectionid)

- <span id="debugmacro-section-reader"></span>`fn reader(&self) -> &R`

### `MacroUnitHeader<R: Reader>`

```rust
struct MacroUnitHeader<R: Reader> {
    _version: u16,
    flags: u8,
    _debug_line_offset: crate::DebugLineOffset<<R as >::Offset>,
}
```

#### Fields

- **`_version`**: `u16`

  The version of the macro unit header. At the moment only version 5 is defined.

#### Implementations

- <span id="macrounitheader-const-offset-size-flag"></span>`const OFFSET_SIZE_FLAG: u8`

- <span id="macrounitheader-const-debug-line-offset-flag"></span>`const DEBUG_LINE_OFFSET_FLAG: u8`

- <span id="macrounitheader-const-opcode-operands-table-flag"></span>`const OPCODE_OPERANDS_TABLE_FLAG: u8`

- <span id="macrounitheader-parse"></span>`fn parse(input: &mut R) -> Result<Self>` — [`Result`](../index.md#result)

- <span id="macrounitheader-format"></span>`fn format(&self) -> Format` — [`Format`](../index.md#format)

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for MacroUnitHeader<R>`

- <span id="macrounitheader-clone"></span>`fn clone(&self) -> MacroUnitHeader<R>` — [`MacroUnitHeader`](macros/index.md#macrounitheader)

##### `impl<R: fmt::Debug + Reader> Debug for MacroUnitHeader<R>`

- <span id="macrounitheader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `MacroIter<R: Reader>`

```rust
struct MacroIter<R: Reader> {
    input: R,
    format: crate::Format,
    is_macro: bool,
}
```

Iterator over the entries in the `.debug_macro` section.

#### Implementations

- <span id="macroiter-next"></span>`fn next(&mut self) -> Result<Option<MacroEntry<R>>>` — [`Result`](../index.md#result), [`MacroEntry`](#macroentry)

  Advance the iterator to the next entry in the `.debug_macro` section.

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for MacroIter<R>`

- <span id="macroiter-clone"></span>`fn clone(&self) -> MacroIter<R>` — [`MacroIter`](#macroiter)

##### `impl<R: fmt::Debug + Reader> Debug for MacroIter<R>`

- <span id="macroiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for MacroIter<R>`

- <span id="macroiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="macroiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="macroiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: Reader> Iterator for MacroIter<R>`

- <span id="macroiter-iterator-type-item"></span>`type Item = Result<MacroEntry<R>, Error>`

- <span id="macroiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `DebugNames<R>`

```rust
struct DebugNames<R> {
    section: R,
}
```

The `DebugNames` struct represents the DWARF 5 name index information
found in the `.debug_names` section.

The `.debug_names` section provides an index for efficiently finding
debugging information entries (DIEs) by name. It contains hash tables
that map names to DIE offsets, allowing debuggers to quickly locate
functions, variables, types, and other named entities.

#### Implementations

- <span id="debugnames-new"></span>`fn new(debug_names_section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugNames` instance from the data in the `.debug_names`

  section.

  

  It is the caller's responsibility to read the `.debug_names` section and

  present it as a `&[u8]` slice. That means using some ELF loader on

  Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugNames, LittleEndian};

  

  let buf = [0x00, 0x01, 0x02, 0x03];

  let read_debug_names_section_somehow = || &buf;

  let debug_names =

      DebugNames::new(read_debug_names_section_somehow(), LittleEndian);

  ```

#### Trait Implementations

##### `impl<R: clone::Clone> Clone for DebugNames<R>`

- <span id="debugnames-clone"></span>`fn clone(&self) -> DebugNames<R>` — [`DebugNames`](#debugnames)

##### `impl<R: marker::Copy> Copy for DebugNames<R>`

##### `impl<R: fmt::Debug> Debug for DebugNames<R>`

- <span id="debugnames-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugNames<R>`

- <span id="debugnames-default"></span>`fn default() -> DebugNames<R>` — [`DebugNames`](#debugnames)

##### `impl<R> Section for DebugNames<R>`

- <span id="debugnames-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../index.md#sectionid)

- <span id="debugnames-section-reader"></span>`fn reader(&self) -> &R`

### `NameIndexHeaderIter<R: Reader>`

```rust
struct NameIndexHeaderIter<R: Reader> {
    input: R,
    end_offset: <R as >::Offset,
}
```

An iterator over the name index headers in the `.debug_names` section.

#### Implementations

- <span id="nameindexheaderiter-next"></span>`fn next(&mut self) -> Result<Option<NameIndexHeader<R>>>` — [`Result`](../index.md#result), [`NameIndexHeader`](#nameindexheader)

  Advance the iterator and return the next name index header.

  

  Returns `Ok(None)` when iteration is complete.

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for NameIndexHeaderIter<R>`

- <span id="nameindexheaderiter-clone"></span>`fn clone(&self) -> NameIndexHeaderIter<R>` — [`NameIndexHeaderIter`](#nameindexheaderiter)

##### `impl<R: fmt::Debug + Reader> Debug for NameIndexHeaderIter<R>`

- <span id="nameindexheaderiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for NameIndexHeaderIter<R>`

- <span id="nameindexheaderiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="nameindexheaderiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="nameindexheaderiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: Reader> Iterator for NameIndexHeaderIter<R>`

- <span id="nameindexheaderiter-iterator-type-item"></span>`type Item = Result<NameIndexHeader<R>, Error>`

- <span id="nameindexheaderiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `NameIndexHeader<R: Reader>`

```rust
struct NameIndexHeader<R: Reader> {
    offset: crate::common::DebugNamesOffset<<R as >::Offset>,
    length: <R as >::Offset,
    format: crate::common::Format,
    version: u16,
    compile_unit_count: u32,
    local_type_unit_count: u32,
    foreign_type_unit_count: u32,
    bucket_count: u32,
    name_count: u32,
    abbrev_table_size: u32,
    augmentation_string: Option<R>,
    content: R,
}
```

The header of a name index in the `.debug_names` section.

#### Fields

- **`offset`**: `crate::common::DebugNamesOffset<<R as >::Offset>`

  The section offset of the header.

- **`length`**: `<R as >::Offset`

  The length of this name index.

- **`format`**: `crate::common::Format`

  The format of the unit.

- **`version`**: `u16`

  Version of the name index format (should be 5 for DWARF 5).

- **`compile_unit_count`**: `u32`

  Number of compilation units in the CU list.

- **`local_type_unit_count`**: `u32`

  Number of type units in the local TU list.

- **`foreign_type_unit_count`**: `u32`

  Number of type units in the foreign TU list.

- **`bucket_count`**: `u32`

  Number of buckets in the hash table.

- **`name_count`**: `u32`

  Number of unique name entries.

- **`abbrev_table_size`**: `u32`

  Size of the abbreviations table in bytes.

- **`augmentation_string`**: `Option<R>`

  The augmentation string.

- **`content`**: `R`

  The remaining unparsed contents of the index.

#### Implementations

- <span id="nameindexheader-index"></span>`fn index(self) -> Result<NameIndex<R>>` — [`Result`](../index.md#result), [`NameIndex`](#nameindex)

  Convert the header into a `NameIndex`.

- <span id="nameindexheader-offset"></span>`fn offset(&self) -> DebugNamesOffset<<R as >::Offset>` — [`DebugNamesOffset`](../index.md#debugnamesoffset), [`Reader`](#reader)

  Return the section offset of this name index.

- <span id="nameindexheader-length"></span>`fn length(&self) -> <R as >::Offset` — [`Reader`](#reader)

  Return the index length.

- <span id="nameindexheader-format"></span>`fn format(&self) -> Format` — [`Format`](../index.md#format)

  Return the format (DWARF32 or DWARF64).

- <span id="nameindexheader-version"></span>`fn version(&self) -> u16`

  Return the version of this name index.

- <span id="nameindexheader-compile-unit-count"></span>`fn compile_unit_count(&self) -> u32`

  Return the number of compilation units in this index.

- <span id="nameindexheader-local-type-unit-count"></span>`fn local_type_unit_count(&self) -> u32`

  Return the number of local type units in this index.

- <span id="nameindexheader-foreign-type-unit-count"></span>`fn foreign_type_unit_count(&self) -> u32`

  Return the number of foreign type units in this index.

- <span id="nameindexheader-bucket-count"></span>`fn bucket_count(&self) -> u32`

  Return the number of buckets in the hash table.

- <span id="nameindexheader-name-count"></span>`fn name_count(&self) -> u32`

  Return the number of unique name entries.

- <span id="nameindexheader-abbrev-table-size"></span>`fn abbrev_table_size(&self) -> u32`

  Return the size of the abbreviations table in bytes.

- <span id="nameindexheader-augmentation-string"></span>`fn augmentation_string(&self) -> Option<&R>`

  Return the augmentation string.

- <span id="nameindexheader-parse"></span>`fn parse(input: &mut R, offset: DebugNamesOffset<<R as >::Offset>) -> Result<Self>` — [`DebugNamesOffset`](../index.md#debugnamesoffset), [`Reader`](#reader), [`Result`](../index.md#result)

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for NameIndexHeader<R>`

- <span id="nameindexheader-clone"></span>`fn clone(&self) -> NameIndexHeader<R>` — [`NameIndexHeader`](#nameindexheader)

##### `impl<R: fmt::Debug + Reader> Debug for NameIndexHeader<R>`

- <span id="nameindexheader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `NameTableIndex`

```rust
struct NameTableIndex(u32);
```

An index into the name table of a `NameIndex`.

This is used as an index into the list of string offsets, the list of entry
offsets, and the list of hashes.

Note that while the DWARF standard specifies that indexes in the DWARF data
start at 1, we use a zero based index here. Functions that read an index from
the data will automatically adjust the index to start at 0.

#### Trait Implementations

##### `impl Clone for NameTableIndex`

- <span id="nametableindex-clone"></span>`fn clone(&self) -> NameTableIndex` — [`NameTableIndex`](#nametableindex)

##### `impl Copy for NameTableIndex`

##### `impl Debug for NameTableIndex`

- <span id="nametableindex-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for NameTableIndex`

##### `impl PartialEq for NameTableIndex`

- <span id="nametableindex-partialeq-eq"></span>`fn eq(&self, other: &NameTableIndex) -> bool` — [`NameTableIndex`](#nametableindex)

##### `impl StructuralPartialEq for NameTableIndex`

### `NameIndex<R: Reader>`

```rust
struct NameIndex<R: Reader> {
    format: crate::common::Format,
    compile_unit_count: u32,
    local_type_unit_count: u32,
    foreign_type_unit_count: u32,
    bucket_count: u32,
    name_count: u32,
    compile_unit_list: R,
    local_type_unit_list: R,
    foreign_type_unit_list: R,
    bucket_data: R,
    hash_table_data: R,
    name_table_data: R,
    entry_offset_data: R,
    entry_pool: R,
    abbreviations: NameAbbreviations,
}
```

A single name index from the `.debug_names` section.

It provides access to the compilation unit table, type unit tables, hash table, name
table, and entry pool that make up the accelerated lookup structure.

#### Implementations

- <span id="nameindex-new"></span>`fn new(header: NameIndexHeader<R>) -> Result<Self>` — [`NameIndexHeader`](#nameindexheader), [`Result`](../index.md#result)

  Create a new name index from a header.

- <span id="nameindex-compile-unit-count"></span>`fn compile_unit_count(&self) -> u32`

  Return the number of compilation units in this index.

- <span id="nameindex-compile-unit"></span>`fn compile_unit(&self, index: u32) -> Result<DebugInfoOffset<<R as >::Offset>>` — [`Result`](../index.md#result), [`DebugInfoOffset`](../index.md#debuginfooffset), [`Reader`](#reader)

  Get the `.debug_info` offset of a compilation unit.

  

  `index` must be less than `Self::compile_unit_count`.

  

  Returns an error if `index` is invalid.

- <span id="nameindex-default-compile-unit"></span>`fn default_compile_unit(&self) -> Result<Option<DebugInfoOffset<<R as >::Offset>>>` — [`Result`](../index.md#result), [`DebugInfoOffset`](../index.md#debuginfooffset), [`Reader`](#reader)

  Return the `.debug_info` offset of the default compilation unit, if any.

  

  If there is only one compilation unit, then entries may omit the `DW_IDX_compile_unit`

  attribute.

- <span id="nameindex-local-type-unit-count"></span>`fn local_type_unit_count(&self) -> u32`

  Return the number of local type units in this index.

- <span id="nameindex-local-type-unit"></span>`fn local_type_unit(&self, index: u32) -> Result<DebugInfoOffset<<R as >::Offset>>` — [`Result`](../index.md#result), [`DebugInfoOffset`](../index.md#debuginfooffset), [`Reader`](#reader)

  Get the `.debug_info` offset of a local type unit.

  

  `index` must be less than `Self::local_type_unit_count`.

  

  Returns an error if `index` is invalid.

- <span id="nameindex-foreign-type-unit-count"></span>`fn foreign_type_unit_count(&self) -> u32`

  Return the number of foreign type units in this index.

- <span id="nameindex-foreign-type-unit"></span>`fn foreign_type_unit(&self, index: u32) -> Result<DebugTypeSignature>` — [`Result`](../index.md#result), [`DebugTypeSignature`](../index.md#debugtypesignature)

  Get the signature of a foreign type unit.

  

  `index` must be less than `Self::foreign_type_unit_count`.

  

  Returns an error if `index` is invalid.

- <span id="nameindex-type-unit-count"></span>`fn type_unit_count(&self) -> u32`

  Return the number of type units in this index, both local and foreign.

- <span id="nameindex-type-unit"></span>`fn type_unit(&self, index: u32) -> Result<NameTypeUnit<<R as >::Offset>>` — [`Result`](../index.md#result), [`NameTypeUnit`](#nametypeunit), [`Reader`](#reader)

  Get a type unit reference.

  

  `index` must be less than `Self::type_unit_count`, and normally is

  obtained from a `DW_IDX_type_unit` attribute.

  

  Returns an error if `index` is invalid.

- <span id="nameindex-has-hash-table"></span>`fn has_hash_table(&self) -> bool`

  Return true if the name index contains a hash table.

- <span id="nameindex-bucket-count"></span>`fn bucket_count(&self) -> u32`

  Return the number of buckets in the hash table.

- <span id="nameindex-find-by-bucket"></span>`fn find_by_bucket(&self, bucket_index: u32) -> Result<Option<NameBucketIter<R>>>` — [`Result`](../index.md#result), [`NameBucketIter`](#namebucketiter)

  Iterate over the hash entries for a bucket in the hash table.

  

  This function is only for diagnostic uses. Usually `Self::find_by_hash` should be

  called instead.

  

  The given bucket index is 0 based, and must be less than `Self::bucket_count`.

  

  Returns an error if there is no hash table or the bucket index is invalid.

  Returns `Ok(None)` if the bucket is empty.

- <span id="nameindex-find-by-hash"></span>`fn find_by_hash(&self, hash_value: u32) -> Result<NameHashIter<R>>` — [`Result`](../index.md#result), [`NameHashIter`](#namehashiter)

  Iterate over the indexes of the names with the given hash value.

  

  The user must then check each name to see if it matches the desired name.

  

  Returns an error if there is no hash table.

- <span id="nameindex-name-count"></span>`fn name_count(&self) -> u32`

  Get the number of names in the name index.

  

  This is 1 greater than the maximum valid [`NameTableIndex`](#nametableindex).

- <span id="nameindex-names"></span>`fn names(&self) -> NameTableIter` — [`NameTableIter`](#nametableiter)

  Iterate over the indexes of all names in the name table.

- <span id="nameindex-name-string-offset"></span>`fn name_string_offset(&self, index: NameTableIndex) -> Result<DebugStrOffset<<R as >::Offset>>` — [`NameTableIndex`](#nametableindex), [`Result`](../index.md#result), [`DebugStrOffset`](../index.md#debugstroffset), [`Reader`](#reader)

  Get the string table offset for the name at the given index.

  

  Returns an error if `index` is invalid.

- <span id="nameindex-name-string"></span>`fn name_string(&self, index: NameTableIndex, debug_str: &DebugStr<R>) -> Result<R>` — [`NameTableIndex`](#nametableindex), [`DebugStr`](#debugstr), [`Result`](../index.md#result)

  Get the name at the given index using the provided `.debug_str` section.

  

  Returns an error if `index` is invalid, or the string table offset is invalid.

- <span id="nameindex-name-entries"></span>`fn name_entries(&self, index: NameTableIndex) -> Result<NameEntryIter<'_, R>>` — [`NameTableIndex`](#nametableindex), [`Result`](../index.md#result), [`NameEntryIter`](#nameentryiter)

  Iterate over the series of entries for the given name table index.

  

  Each name in the name table has a corresponding series of entries

  with that name in the entry pool.

  

  Returns an error if `index` is invalid, or the entry pool offset is invalid.

- <span id="nameindex-name-entry"></span>`fn name_entry(&self, offset: NameEntryOffset<<R as >::Offset>) -> Result<NameEntry<R>>` — [`NameEntryOffset`](#nameentryoffset), [`Reader`](#reader), [`Result`](../index.md#result), [`NameEntry`](#nameentry)

  Parse the entry at the given entry pool offset.

  

  This is useful for reading the entry referenced by a `DW_IDX_parent` attribute.

- <span id="nameindex-abbreviations"></span>`fn abbreviations(&self) -> &NameAbbreviations` — [`NameAbbreviations`](#nameabbreviations)

  Get the abbreviation table for name entries in this name index.

#### Trait Implementations

##### `impl<R: fmt::Debug + Reader> Debug for NameIndex<R>`

- <span id="nameindex-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `NameTableIter`

```rust
struct NameTableIter {
    name_table_index: NameTableIndex,
    name_count: u32,
}
```

An iterator over the indexes of all names in a name index.

#### Implementations

- <span id="nametableiter-new"></span>`fn new<R: Reader>(name_index: &NameIndex<R>) -> Self` — [`NameIndex`](#nameindex)

#### Trait Implementations

##### `impl Debug for NameTableIter`

- <span id="nametableiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for NameTableIter`

- <span id="nametableiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="nametableiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="nametableiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for NameTableIter`

- <span id="nametableiter-iterator-type-item"></span>`type Item = NameTableIndex`

- <span id="nametableiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `NameBucketIter<R: Reader>`

```rust
struct NameBucketIter<R: Reader> {
    reader: R,
    name_table_index: NameTableIndex,
    name_count: u32,
    bucket_index: u32,
    bucket_count: u32,
}
```

An iterator over the hash entries for a bucket in a name index hash table.

#### Implementations

- <span id="namebucketiter-new"></span>`fn new(name_index: &NameIndex<R>, bucket_index: u32) -> Result<Option<Self>>` — [`NameIndex`](#nameindex), [`Result`](../index.md#result)

- <span id="namebucketiter-next"></span>`fn next(&mut self) -> Result<Option<(NameTableIndex, u32)>>` — [`Result`](../index.md#result), [`NameTableIndex`](#nametableindex)

  Advance the iterator and return the next name table index and hash.

#### Trait Implementations

##### `impl<R: fmt::Debug + Reader> Debug for NameBucketIter<R>`

- <span id="namebucketiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for NameBucketIter<R>`

- <span id="namebucketiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="namebucketiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="namebucketiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: Reader> Iterator for NameBucketIter<R>`

- <span id="namebucketiter-iterator-type-item"></span>`type Item = Result<(NameTableIndex, u32), Error>`

- <span id="namebucketiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `NameHashIter<R: Reader>`

```rust
struct NameHashIter<R: Reader> {
    bucket_iter: Option<NameBucketIter<R>>,
    hash: u32,
}
```

An iterator over the indexes of the names in a name index hash table that match a hash
value.

#### Implementations

- <span id="namehashiter-new"></span>`fn new(name_index: &NameIndex<R>, hash: u32) -> Result<Self>` — [`NameIndex`](#nameindex), [`Result`](../index.md#result)

- <span id="namehashiter-next"></span>`fn next(&mut self) -> Result<Option<NameTableIndex>>` — [`Result`](../index.md#result), [`NameTableIndex`](#nametableindex)

  Advance the iterator and return the next name table index.

#### Trait Implementations

##### `impl<R: fmt::Debug + Reader> Debug for NameHashIter<R>`

- <span id="namehashiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for NameHashIter<R>`

- <span id="namehashiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="namehashiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="namehashiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: Reader> Iterator for NameHashIter<R>`

- <span id="namehashiter-iterator-type-item"></span>`type Item = Result<NameTableIndex, Error>`

- <span id="namehashiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `NameEntryIter<'a, R: Reader>`

```rust
struct NameEntryIter<'a, R: Reader> {
    entries: R,
    end_offset: <R as >::Offset,
    abbreviations: &'a NameAbbreviations,
}
```

An iterator for a series of name entries in a name index entry pool.

Each name in a name index corresponds to a series of entries
with that name.

#### Implementations

- <span id="nameentryiter-new"></span>`fn new(name_index: &'a NameIndex<R>, index: NameTableIndex) -> Result<Self>` — [`NameIndex`](#nameindex), [`NameTableIndex`](#nametableindex), [`Result`](../index.md#result)

- <span id="nameentryiter-next"></span>`fn next(&mut self) -> Result<Option<NameEntry<R>>>` — [`Result`](../index.md#result), [`NameEntry`](#nameentry)

  Advance the iterator and return the next name entry.

#### Trait Implementations

##### `impl<R: fmt::Debug + Reader> Debug for NameEntryIter<'a, R>`

- <span id="nameentryiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for NameEntryIter<'a, R>`

- <span id="nameentryiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="nameentryiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="nameentryiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: Reader> Iterator for NameEntryIter<'a, R>`

- <span id="nameentryiter-iterator-type-item"></span>`type Item = Result<NameEntry<R>, Error>`

- <span id="nameentryiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `NameEntryOffset<T>`

```rust
struct NameEntryOffset<T>(T);
```

An offset into the entry pool of a name index.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for NameEntryOffset<T>`

- <span id="nameentryoffset-clone"></span>`fn clone(&self) -> NameEntryOffset<T>` — [`NameEntryOffset`](#nameentryoffset)

##### `impl<T: marker::Copy> Copy for NameEntryOffset<T>`

##### `impl<T: fmt::Debug> Debug for NameEntryOffset<T>`

- <span id="nameentryoffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for NameEntryOffset<T>`

##### `impl<T: hash::Hash> Hash for NameEntryOffset<T>`

- <span id="nameentryoffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T: cmp::Ord> Ord for NameEntryOffset<T>`

- <span id="nameentryoffset-ord-cmp"></span>`fn cmp(&self, other: &NameEntryOffset<T>) -> cmp::Ordering` — [`NameEntryOffset`](#nameentryoffset)

##### `impl<T: cmp::PartialEq> PartialEq for NameEntryOffset<T>`

- <span id="nameentryoffset-partialeq-eq"></span>`fn eq(&self, other: &NameEntryOffset<T>) -> bool` — [`NameEntryOffset`](#nameentryoffset)

##### `impl<T: cmp::PartialOrd> PartialOrd for NameEntryOffset<T>`

- <span id="nameentryoffset-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &NameEntryOffset<T>) -> option::Option<cmp::Ordering>` — [`NameEntryOffset`](#nameentryoffset)

##### `impl<T> StructuralPartialEq for NameEntryOffset<T>`

### `NameEntry<R: Reader>`

```rust
struct NameEntry<R: Reader> {
    pub offset: NameEntryOffset<<R as >::Offset>,
    pub abbrev_code: u64,
    pub tag: constants::DwTag,
    pub attrs: alloc::vec::Vec<NameAttribute<R>>,
}
```

A parsed entry from the `.debug_names` section.

#### Fields

- **`offset`**: `NameEntryOffset<<R as >::Offset>`

  The offset of the entry in the entries pool.

- **`abbrev_code`**: `u64`

  The abbreviation code for this entry.

- **`tag`**: `constants::DwTag`

  The DIE tag for this entry.

- **`attrs`**: `alloc::vec::Vec<NameAttribute<R>>`

  The attributes for this entry.

#### Implementations

- <span id="nameentry-compile-unit"></span>`fn compile_unit(&self, names: &NameIndex<R>) -> Result<Option<DebugInfoOffset<<R as >::Offset>>>` — [`NameIndex`](#nameindex), [`Result`](../index.md#result), [`DebugInfoOffset`](../index.md#debuginfooffset), [`Reader`](#reader)

  Get the value of the `DW_IDX_compile_unit` attribute, if any.

  

  If neither `DW_IDX_compile_unit` nor `DW_IDX_type_unit` exist then you should use

  `NameIndex::default_compile_unit`.

  

  If both `DW_IDX_compile_unit` and `DW_IDX_type_unit` exist then this value is for

  a skeleton CU that may be used to locate a split DWARF object file containing

  the type unit.

- <span id="nameentry-type-unit"></span>`fn type_unit(&self, names: &NameIndex<R>) -> Result<Option<NameTypeUnit<<R as >::Offset>>>` — [`NameIndex`](#nameindex), [`Result`](../index.md#result), [`NameTypeUnit`](#nametypeunit), [`Reader`](#reader)

  Get the value of the `DW_IDX_type_unit` attribute, if any.

- <span id="nameentry-die-offset"></span>`fn die_offset(&self) -> Result<Option<UnitOffset<<R as >::Offset>>>` — [`Result`](../index.md#result), [`UnitOffset`](../index.md#unitoffset), [`Reader`](#reader)

  Get the value of the `DW_IDX_die_offset` attribute, if any.

  

  This is the offset of the DIE within the compile unit or type unit.

- <span id="nameentry-parent"></span>`fn parent(&self) -> Result<Option<Option<NameEntryOffset<<R as >::Offset>>>>` — [`Result`](../index.md#result), [`NameEntryOffset`](#nameentryoffset), [`Reader`](#reader)

  Get the value of the `DW_IDX_parent` attribute, if any.

  

  Returns `Ok(Some(Some(offset)))` if the DIE parent is indexed.

  Returns `Ok(Some(None))` if the DIE parent is not indexed.

  Returns `Ok(None)` if it is unknown whether the DIE parent is indexed

  because the producer did not generate a `DW_IDX_parent` attribute.

- <span id="nameentry-type-hash"></span>`fn type_hash(&self) -> Result<Option<u64>>` — [`Result`](../index.md#result)

  Get the value of the `DW_IDX_type_hash` attribute, if any.

- <span id="nameentry-parse"></span>`fn parse(entry_reader: &mut R, offset: NameEntryOffset<<R as >::Offset>, abbreviations: &NameAbbreviations) -> Result<Option<NameEntry<R>>>` — [`NameEntryOffset`](#nameentryoffset), [`Reader`](#reader), [`NameAbbreviations`](#nameabbreviations), [`Result`](../index.md#result), [`NameEntry`](#nameentry)

  Parse a single entry from the entry pool.

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for NameEntry<R>`

- <span id="nameentry-clone"></span>`fn clone(&self) -> NameEntry<R>` — [`NameEntry`](#nameentry)

##### `impl<R: fmt::Debug + Reader> Debug for NameEntry<R>`

- <span id="nameentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: cmp::Eq + Reader> Eq for NameEntry<R>`

##### `impl<R: cmp::PartialEq + Reader> PartialEq for NameEntry<R>`

- <span id="nameentry-partialeq-eq"></span>`fn eq(&self, other: &NameEntry<R>) -> bool` — [`NameEntry`](#nameentry)

##### `impl<R: Reader> StructuralPartialEq for NameEntry<R>`

### `NameAttribute<R: Reader>`

```rust
struct NameAttribute<R: Reader> {
    name: constants::DwIdx,
    form: constants::DwForm,
    value: NameAttributeValue<R>,
}
```

A parsed attribute for a [`NameEntry`](#nameentry).

#### Implementations

- <span id="nameattribute-name"></span>`fn name(&self) -> constants::DwIdx` — [`DwIdx`](../index.md#dwidx)

  Get the attribute name.

- <span id="nameattribute-form"></span>`fn form(&self) -> constants::DwForm` — [`DwForm`](../index.md#dwform)

  Get the attribute form.

- <span id="nameattribute-value"></span>`fn value(&self) -> &NameAttributeValue<R>` — [`NameAttributeValue`](#nameattributevalue)

  Get the attribute value.

  

  Interpretation of this value depends on the name and form.

- <span id="nameattribute-compile-unit"></span>`fn compile_unit(&self, names: &NameIndex<R>) -> Result<DebugInfoOffset<<R as >::Offset>>` — [`NameIndex`](#nameindex), [`Result`](../index.md#result), [`DebugInfoOffset`](../index.md#debuginfooffset), [`Reader`](#reader)

  Get the value of a `DW_IDX_compile_unit` attribute.

- <span id="nameattribute-type-unit"></span>`fn type_unit(&self, names: &NameIndex<R>) -> Result<NameTypeUnit<<R as >::Offset>>` — [`NameIndex`](#nameindex), [`Result`](../index.md#result), [`NameTypeUnit`](#nametypeunit), [`Reader`](#reader)

  Get the value of a `DW_IDX_type_unit` attribute.

- <span id="nameattribute-die-offset"></span>`fn die_offset(&self) -> Result<UnitOffset<<R as >::Offset>>` — [`Result`](../index.md#result), [`UnitOffset`](../index.md#unitoffset), [`Reader`](#reader)

  Get the value of a `DW_IDX_die_offset` attribute.

- <span id="nameattribute-parent"></span>`fn parent(&self) -> Result<Option<NameEntryOffset<<R as >::Offset>>>` — [`Result`](../index.md#result), [`NameEntryOffset`](#nameentryoffset), [`Reader`](#reader)

  Get the value of a `DW_IDX_parent` attribute.

  

  Returns `Ok(Some(offset))` if the DIE parent is indexed.

  Returns `Ok(None)` if the DIE parent is not indexed.

- <span id="nameattribute-type-hash"></span>`fn type_hash(&self) -> Result<u64>` — [`Result`](../index.md#result)

  Get the value of a `DW_IDX_type_hash` attribute.

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for NameAttribute<R>`

- <span id="nameattribute-clone"></span>`fn clone(&self) -> NameAttribute<R>` — [`NameAttribute`](#nameattribute)

##### `impl<R: marker::Copy + Reader> Copy for NameAttribute<R>`

##### `impl<R: fmt::Debug + Reader> Debug for NameAttribute<R>`

- <span id="nameattribute-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: cmp::Eq + Reader> Eq for NameAttribute<R>`

##### `impl<R: cmp::PartialEq + Reader> PartialEq for NameAttribute<R>`

- <span id="nameattribute-partialeq-eq"></span>`fn eq(&self, other: &NameAttribute<R>) -> bool` — [`NameAttribute`](#nameattribute)

##### `impl<R: Reader> StructuralPartialEq for NameAttribute<R>`

### `NameAbbreviations`

```rust
struct NameAbbreviations {
    abbreviations: alloc::vec::Vec<NameAbbreviation>,
}
```

A table of name entry abbreviations.

#### Fields

- **`abbreviations`**: `alloc::vec::Vec<NameAbbreviation>`

  The abbreviations in this table.

#### Implementations

- <span id="nameabbreviations-get"></span>`fn get(&self, code: u64) -> Option<&NameAbbreviation>` — [`NameAbbreviation`](#nameabbreviation)

  Get an abbreviation by its code.

- <span id="nameabbreviations-abbreviations"></span>`fn abbreviations(&self) -> &[NameAbbreviation]` — [`NameAbbreviation`](#nameabbreviation)

  Get all abbreviations.

- <span id="nameabbreviations-parse"></span>`fn parse<R: Reader>(reader: R) -> Result<NameAbbreviations>` — [`Result`](../index.md#result), [`NameAbbreviations`](#nameabbreviations)

  Parse the abbreviation table from a reader.

#### Trait Implementations

##### `impl Clone for NameAbbreviations`

- <span id="nameabbreviations-clone"></span>`fn clone(&self) -> NameAbbreviations` — [`NameAbbreviations`](#nameabbreviations)

##### `impl Debug for NameAbbreviations`

- <span id="nameabbreviations-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for NameAbbreviations`

- <span id="nameabbreviations-default"></span>`fn default() -> NameAbbreviations` — [`NameAbbreviations`](#nameabbreviations)

### `NameAbbreviation`

```rust
struct NameAbbreviation {
    code: u64,
    tag: constants::DwTag,
    attributes: alloc::vec::Vec<NameAbbreviationAttribute>,
}
```

A name abbreviation entry defines how name entries are encoded.

#### Fields

- **`code`**: `u64`

  The abbreviation code.

- **`tag`**: `constants::DwTag`

  The DIE tag.

- **`attributes`**: `alloc::vec::Vec<NameAbbreviationAttribute>`

  The list of attribute specifications.

#### Implementations

- <span id="nameabbreviation-code"></span>`fn code(&self) -> u64`

  Get the abbreviation code.

- <span id="nameabbreviation-tag"></span>`fn tag(&self) -> constants::DwTag` — [`DwTag`](../index.md#dwtag)

  Get the DIE tag.

- <span id="nameabbreviation-attributes"></span>`fn attributes(&self) -> &[NameAbbreviationAttribute]` — [`NameAbbreviationAttribute`](#nameabbreviationattribute)

  Get the attribute specifications.

#### Trait Implementations

##### `impl Clone for NameAbbreviation`

- <span id="nameabbreviation-clone"></span>`fn clone(&self) -> NameAbbreviation` — [`NameAbbreviation`](#nameabbreviation)

##### `impl Debug for NameAbbreviation`

- <span id="nameabbreviation-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `NameAbbreviationAttribute`

```rust
struct NameAbbreviationAttribute {
    name: constants::DwIdx,
    form: constants::DwForm,
}
```

An attribute specification in a name abbreviation.

#### Fields

- **`name`**: `constants::DwIdx`

  The attribute name (index type).

- **`form`**: `constants::DwForm`

  The attribute form.

#### Implementations

- <span id="nameabbreviationattribute-name"></span>`fn name(&self) -> constants::DwIdx` — [`DwIdx`](../index.md#dwidx)

  Get the attribute name (index type).

- <span id="nameabbreviationattribute-form"></span>`fn form(&self) -> constants::DwForm` — [`DwForm`](../index.md#dwform)

  Get the attribute form.

#### Trait Implementations

##### `impl Clone for NameAbbreviationAttribute`

- <span id="nameabbreviationattribute-clone"></span>`fn clone(&self) -> NameAbbreviationAttribute` — [`NameAbbreviationAttribute`](#nameabbreviationattribute)

##### `impl Debug for NameAbbreviationAttribute`

- <span id="nameabbreviationattribute-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Piece<R, Offset>`

```rust
struct Piece<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    pub size_in_bits: Option<u64>,
    pub bit_offset: Option<u64>,
    pub location: Location<R, Offset>,
}
```

The description of a single piece of the result of a DWARF
expression.

#### Fields

- **`size_in_bits`**: `Option<u64>`

  If given, the size of the piece in bits.  If `None`, there
  must be only one piece whose size is all of the object.

- **`bit_offset`**: `Option<u64>`

  If given, the bit offset of the piece within the location.
  If the location is a `Location::Register` or `Location::Value`,
  then this offset is from the least significant bit end of
  the register or value.
  If the location is a `Location::Address` then the offset uses
  the bit numbering and direction conventions of the language
  and target system.
  
  If `None`, the piece starts at the location. If the
  location is a register whose size is larger than the piece,
  then placement within the register is defined by the ABI.

- **`location`**: `Location<R, Offset>`

  Where this piece is to be found.

#### Trait Implementations

##### `impl<R, Offset> Clone for Piece<R, Offset>`

- <span id="piece-clone"></span>`fn clone(&self) -> Piece<R, Offset>` — [`Piece`](#piece)

##### `impl<R, Offset> Copy for Piece<R, Offset>`

##### `impl<R, Offset> Debug for Piece<R, Offset>`

- <span id="piece-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Offset> PartialEq for Piece<R, Offset>`

- <span id="piece-partialeq-eq"></span>`fn eq(&self, other: &Piece<R, Offset>) -> bool` — [`Piece`](#piece)

##### `impl<R, Offset> StructuralPartialEq for Piece<R, Offset>`

### `Expression<R: Reader>`

```rust
struct Expression<R: Reader>(R);
```

The bytecode for a DWARF expression or location description.

#### Implementations

- <span id="expression-evaluation"></span>`fn evaluation(self, encoding: Encoding) -> Evaluation<R>` — [`Encoding`](../index.md#encoding), [`Evaluation`](#evaluation)

  Create an evaluation for this expression.

  

  The `encoding` is determined by the

  [`CompilationUnitHeader`](#compilationunitheader) or

  [`TypeUnitHeader`](#typeunitheader) that this expression

  relates to.

  

  # Examples

  ```rust,no_run

  use gimli::Expression;

  let endian = gimli::LittleEndian;

  let debug_info = gimli::DebugInfo::from(gimli::EndianSlice::new(&[], endian));

  let unit = debug_info.units().next().unwrap().unwrap();

  let bytecode = gimli::EndianSlice::new(&[], endian);

  let expression = gimli::Expression(bytecode);

  let mut eval = expression.evaluation(unit.encoding());

  let mut result = eval.evaluate().unwrap();

  ```

- <span id="expression-operations"></span>`fn operations(self, encoding: Encoding) -> OperationIter<R>` — [`Encoding`](../index.md#encoding), [`OperationIter`](#operationiter)

  Return an iterator for the operations in the expression.

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for Expression<R>`

- <span id="expression-clone"></span>`fn clone(&self) -> Expression<R>` — [`Expression`](#expression)

##### `impl<R: marker::Copy + Reader> Copy for Expression<R>`

##### `impl<R: fmt::Debug + Reader> Debug for Expression<R>`

- <span id="expression-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: cmp::Eq + Reader> Eq for Expression<R>`

##### `impl<R: hash::Hash + Reader> Hash for Expression<R>`

- <span id="expression-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<R: cmp::PartialEq + Reader> PartialEq for Expression<R>`

- <span id="expression-partialeq-eq"></span>`fn eq(&self, other: &Expression<R>) -> bool` — [`Expression`](#expression)

##### `impl<R: Reader> StructuralPartialEq for Expression<R>`

### `OperationIter<R: Reader>`

```rust
struct OperationIter<R: Reader> {
    input: R,
    encoding: crate::common::Encoding,
}
```

An iterator for the operations in an expression.

#### Implementations

- <span id="operationiter-next"></span>`fn next(&mut self) -> Result<Option<Operation<R>>>` — [`Result`](../index.md#result), [`Operation`](#operation)

  Read the next operation in an expression.

- <span id="operationiter-offset-from"></span>`fn offset_from(&self, expression: &Expression<R>) -> <R as >::Offset` — [`Expression`](#expression), [`Reader`](#reader)

  Return the current byte offset of the iterator.

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for OperationIter<R>`

- <span id="operationiter-clone"></span>`fn clone(&self) -> OperationIter<R>` — [`OperationIter`](#operationiter)

##### `impl<R: marker::Copy + Reader> Copy for OperationIter<R>`

##### `impl<R: fmt::Debug + Reader> Debug for OperationIter<R>`

- <span id="operationiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for OperationIter<R>`

- <span id="operationiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="operationiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="operationiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: Reader> Iterator for OperationIter<R>`

- <span id="operationiter-iterator-type-item"></span>`type Item = Result<Operation<R>, Error>`

- <span id="operationiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `Evaluation<R: Reader, S: EvaluationStorage<R>>`

```rust
struct Evaluation<R: Reader, S: EvaluationStorage<R>> {
    bytecode: R,
    encoding: crate::common::Encoding,
    object_address: Option<u64>,
    max_iterations: Option<u32>,
    iteration: u32,
    state: EvaluationState<R>,
    addr_mask: u64,
    stack: super::util::ArrayVec<<S as >::Stack>,
    pc: R,
    expression_stack: super::util::ArrayVec<<S as >::ExpressionStack>,
    value_result: Option<crate::read::Value>,
    result: super::util::ArrayVec<<S as >::Result>,
}
```

A DWARF expression evaluator.

# Usage
A DWARF expression may require additional data to produce a final result,
such as the value of a register or a memory location.  Once initial setup
is complete (i.e. `set_initial_value()`, `set_object_address()`) the
consumer calls the `evaluate()` method.  That returns an `EvaluationResult`,
which is either `EvaluationResult::Complete` or a value indicating what
data is needed to resume the `Evaluation`.  The consumer is responsible for
producing that data and resuming the computation with the correct method,
as documented for `EvaluationResult`.  Only once an `EvaluationResult::Complete`
is returned can the consumer call `result()`.

This design allows the consumer of `Evaluation` to decide how and when to
produce the required data and resume the computation.  The `Evaluation` can
be driven synchronously (as shown below) or by some asynchronous mechanism
such as futures.

# Examples
```rust,no_run
use gimli::{Evaluation, EvaluationResult, Expression};
let bytecode = gimli::EndianSlice::new(&[], gimli::LittleEndian);
let encoding = unimplemented!();
let get_register_value = |_, _| gimli::Value::Generic(42);
let get_frame_base = || 0xdeadbeef;

let mut eval = Evaluation::new(bytecode, encoding);
let mut result = eval.evaluate().unwrap();
while result != EvaluationResult::Complete {
  match result {
    EvaluationResult::RequiresRegister { register, base_type } => {
      let value = get_register_value(register, base_type);
      result = eval.resume_with_register(value).unwrap();
    },
    EvaluationResult::RequiresFrameBase => {
      let frame_base = get_frame_base();
      result = eval.resume_with_frame_base(frame_base).unwrap();
    },
    _ => unimplemented!(),
  };
}

let result = eval.result();
println!("{:?}", result);
```

#### Implementations

- <span id="evaluation-new"></span>`fn new(bytecode: R, encoding: Encoding) -> Self` — [`Encoding`](../index.md#encoding)

  Create a new DWARF expression evaluator.

  

  The new evaluator is created without an initial value, without

  an object address, and without a maximum number of iterations.

- <span id="evaluation-result"></span>`fn result(self) -> Vec<Piece<R>>` — [`Piece`](#piece)

  Get the result of this `Evaluation`.

  

  # Panics

  Panics if this `Evaluation` has not been driven to completion.

#### Trait Implementations

##### `impl<R: fmt::Debug + Reader, S: fmt::Debug + EvaluationStorage<R>> Debug for Evaluation<R, S>`

- <span id="evaluation-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `PubNamesEntry<R: Reader>`

```rust
struct PubNamesEntry<R: Reader> {
    unit_header_offset: crate::common::DebugInfoOffset<<R as >::Offset>,
    die_offset: crate::read::UnitOffset<<R as >::Offset>,
    name: R,
}
```

A single parsed pubname.

#### Implementations

- <span id="pubnamesentry-name"></span>`fn name(&self) -> &R`

  Returns the name this entry refers to.

- <span id="pubnamesentry-unit-header-offset"></span>`fn unit_header_offset(&self) -> DebugInfoOffset<<R as >::Offset>` — [`DebugInfoOffset`](../index.md#debuginfooffset), [`Reader`](#reader)

  Returns the offset into the .debug_info section for the header of the compilation unit

  which contains this name.

- <span id="pubnamesentry-die-offset"></span>`fn die_offset(&self) -> UnitOffset<<R as >::Offset>` — [`UnitOffset`](../index.md#unitoffset), [`Reader`](#reader)

  Returns the offset into the compilation unit for the debugging information entry which

  has this name.

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for PubNamesEntry<R>`

- <span id="pubnamesentry-clone"></span>`fn clone(&self) -> PubNamesEntry<R>` — [`PubNamesEntry`](#pubnamesentry)

##### `impl<R: fmt::Debug + Reader> Debug for PubNamesEntry<R>`

- <span id="pubnamesentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: Reader> PubStuffEntry for PubNamesEntry<R>`

- <span id="pubnamesentry-pubstuffentry-new"></span>`fn new(die_offset: UnitOffset<<R as >::Offset>, name: R, unit_header_offset: DebugInfoOffset<<R as >::Offset>) -> Self` — [`UnitOffset`](../index.md#unitoffset), [`Reader`](#reader), [`DebugInfoOffset`](../index.md#debuginfooffset)

### `DebugPubNames<R: Reader>`

```rust
struct DebugPubNames<R: Reader>(crate::read::lookup::DebugLookup<R, crate::read::lookup::PubStuffParser<R, PubNamesEntry<R>>>);
```

The `DebugPubNames` struct represents the DWARF public names information
found in the `.debug_pubnames` section.

#### Implementations

- <span id="debugpubnames-new"></span>`fn new(debug_pubnames_section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugPubNames` instance from the data in the `.debug_pubnames`

  section.

  

  It is the caller's responsibility to read the `.debug_pubnames` section and

  present it as a `&[u8]` slice. That means using some ELF loader on

  Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugPubNames, LittleEndian};

  

  let buf = [];

  let read_debug_pubnames_section_somehow = || &buf;

  let debug_pubnames =

      DebugPubNames::new(read_debug_pubnames_section_somehow(), LittleEndian);

  ```

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for DebugPubNames<R>`

- <span id="debugpubnames-clone"></span>`fn clone(&self) -> DebugPubNames<R>` — [`DebugPubNames`](#debugpubnames)

##### `impl<R: fmt::Debug + Reader> Debug for DebugPubNames<R>`

- <span id="debugpubnames-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: Reader> Section for DebugPubNames<R>`

- <span id="debugpubnames-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../index.md#sectionid)

- <span id="debugpubnames-section-reader"></span>`fn reader(&self) -> &R`

### `PubNamesEntryIter<R: Reader>`

```rust
struct PubNamesEntryIter<R: Reader>(crate::read::lookup::LookupEntryIter<R, crate::read::lookup::PubStuffParser<R, PubNamesEntry<R>>>);
```

An iterator over the pubnames from a `.debug_pubnames` section.

#### Implementations

- <span id="pubnamesentryiter-next"></span>`fn next(&mut self) -> Result<Option<PubNamesEntry<R>>>` — [`Result`](../index.md#result), [`PubNamesEntry`](#pubnamesentry)

  Advance the iterator and return the next pubname.

  

  Returns the newly parsed pubname as `Ok(Some(pubname))`. Returns

  `Ok(None)` when iteration is complete and all pubnames have already been

  parsed and yielded. If an error occurs while parsing the next pubname,

  then this error is returned as `Err(e)`, and all subsequent calls return

  `Ok(None)`.

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for PubNamesEntryIter<R>`

- <span id="pubnamesentryiter-clone"></span>`fn clone(&self) -> PubNamesEntryIter<R>` — [`PubNamesEntryIter`](#pubnamesentryiter)

##### `impl<R: fmt::Debug + Reader> Debug for PubNamesEntryIter<R>`

- <span id="pubnamesentryiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for PubNamesEntryIter<R>`

- <span id="pubnamesentryiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="pubnamesentryiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="pubnamesentryiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: Reader> Iterator for PubNamesEntryIter<R>`

- <span id="pubnamesentryiter-iterator-type-item"></span>`type Item = Result<PubNamesEntry<R>, Error>`

- <span id="pubnamesentryiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `PubTypesEntry<R: Reader>`

```rust
struct PubTypesEntry<R: Reader> {
    unit_header_offset: crate::common::DebugInfoOffset<<R as >::Offset>,
    die_offset: crate::read::UnitOffset<<R as >::Offset>,
    name: R,
}
```

A single parsed pubtype.

#### Implementations

- <span id="pubtypesentry-name"></span>`fn name(&self) -> &R`

  Returns the name of the type this entry refers to.

- <span id="pubtypesentry-unit-header-offset"></span>`fn unit_header_offset(&self) -> DebugInfoOffset<<R as >::Offset>` — [`DebugInfoOffset`](../index.md#debuginfooffset), [`Reader`](#reader)

  Returns the offset into the .debug_info section for the header of the compilation unit

  which contains the type with this name.

- <span id="pubtypesentry-die-offset"></span>`fn die_offset(&self) -> UnitOffset<<R as >::Offset>` — [`UnitOffset`](../index.md#unitoffset), [`Reader`](#reader)

  Returns the offset into the compilation unit for the debugging information entry which

  the type with this name.

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for PubTypesEntry<R>`

- <span id="pubtypesentry-clone"></span>`fn clone(&self) -> PubTypesEntry<R>` — [`PubTypesEntry`](#pubtypesentry)

##### `impl<R: fmt::Debug + Reader> Debug for PubTypesEntry<R>`

- <span id="pubtypesentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: Reader> PubStuffEntry for PubTypesEntry<R>`

- <span id="pubtypesentry-pubstuffentry-new"></span>`fn new(die_offset: UnitOffset<<R as >::Offset>, name: R, unit_header_offset: DebugInfoOffset<<R as >::Offset>) -> Self` — [`UnitOffset`](../index.md#unitoffset), [`Reader`](#reader), [`DebugInfoOffset`](../index.md#debuginfooffset)

### `DebugPubTypes<R: Reader>`

```rust
struct DebugPubTypes<R: Reader>(crate::read::lookup::DebugLookup<R, crate::read::lookup::PubStuffParser<R, PubTypesEntry<R>>>);
```

The `DebugPubTypes` struct represents the DWARF public types information
found in the `.debug_info` section.

#### Implementations

- <span id="debugpubtypes-new"></span>`fn new(debug_pubtypes_section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugPubTypes` instance from the data in the `.debug_pubtypes`

  section.

  

  It is the caller's responsibility to read the `.debug_pubtypes` section and

  present it as a `&[u8]` slice. That means using some ELF loader on

  Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugPubTypes, LittleEndian};

  

  let buf = [];

  let read_debug_pubtypes_somehow = || &buf;

  let debug_pubtypes =

      DebugPubTypes::new(read_debug_pubtypes_somehow(), LittleEndian);

  ```

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for DebugPubTypes<R>`

- <span id="debugpubtypes-clone"></span>`fn clone(&self) -> DebugPubTypes<R>` — [`DebugPubTypes`](#debugpubtypes)

##### `impl<R: fmt::Debug + Reader> Debug for DebugPubTypes<R>`

- <span id="debugpubtypes-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: Reader> Section for DebugPubTypes<R>`

- <span id="debugpubtypes-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../index.md#sectionid)

- <span id="debugpubtypes-section-reader"></span>`fn reader(&self) -> &R`

### `PubTypesEntryIter<R: Reader>`

```rust
struct PubTypesEntryIter<R: Reader>(crate::read::lookup::LookupEntryIter<R, crate::read::lookup::PubStuffParser<R, PubTypesEntry<R>>>);
```

An iterator over the pubtypes from a `.debug_pubtypes` section.

#### Implementations

- <span id="pubtypesentryiter-next"></span>`fn next(&mut self) -> Result<Option<PubTypesEntry<R>>>` — [`Result`](../index.md#result), [`PubTypesEntry`](#pubtypesentry)

  Advance the iterator and return the next pubtype.

  

  Returns the newly parsed pubtype as `Ok(Some(pubtype))`. Returns

  `Ok(None)` when iteration is complete and all pubtypes have already been

  parsed and yielded. If an error occurs while parsing the next pubtype,

  then this error is returned as `Err(e)`, and all subsequent calls return

  `Ok(None)`.

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for PubTypesEntryIter<R>`

- <span id="pubtypesentryiter-clone"></span>`fn clone(&self) -> PubTypesEntryIter<R>` — [`PubTypesEntryIter`](#pubtypesentryiter)

##### `impl<R: fmt::Debug + Reader> Debug for PubTypesEntryIter<R>`

- <span id="pubtypesentryiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for PubTypesEntryIter<R>`

- <span id="pubtypesentryiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="pubtypesentryiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="pubtypesentryiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: Reader> Iterator for PubTypesEntryIter<R>`

- <span id="pubtypesentryiter-iterator-type-item"></span>`type Item = Result<PubTypesEntry<R>, Error>`

- <span id="pubtypesentryiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `DebugRanges<R>`

```rust
struct DebugRanges<R> {
    section: R,
}
```

The raw contents of the `.debug_ranges` section.

#### Implementations

- <span id="debugranges-new"></span>`fn new(section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugRanges` instance from the data in the `.debug_ranges`

  section.

  

  It is the caller's responsibility to read the `.debug_ranges` section and

  present it as a `&[u8]` slice. That means using some ELF loader on

  Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugRanges, LittleEndian};

  

  let buf = [0x00, 0x01, 0x02, 0x03];

  let read_debug_ranges_section_somehow = || &buf;

  let debug_ranges = DebugRanges::new(read_debug_ranges_section_somehow(), LittleEndian);

  ```

#### Trait Implementations

##### `impl<R: clone::Clone> Clone for DebugRanges<R>`

- <span id="debugranges-clone"></span>`fn clone(&self) -> DebugRanges<R>` — [`DebugRanges`](#debugranges)

##### `impl<R: marker::Copy> Copy for DebugRanges<R>`

##### `impl<R: fmt::Debug> Debug for DebugRanges<R>`

- <span id="debugranges-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugRanges<R>`

- <span id="debugranges-default"></span>`fn default() -> DebugRanges<R>` — [`DebugRanges`](#debugranges)

##### `impl<R> Section for DebugRanges<R>`

- <span id="debugranges-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../index.md#sectionid)

- <span id="debugranges-section-reader"></span>`fn reader(&self) -> &R`

### `DebugRngLists<R>`

```rust
struct DebugRngLists<R> {
    section: R,
}
```

The `DebugRngLists` struct represents the contents of the
`.debug_rnglists` section.

#### Implementations

- <span id="debugrnglists-new"></span>`fn new(section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugRngLists` instance from the data in the

  `.debug_rnglists` section.

  

  It is the caller's responsibility to read the `.debug_rnglists`

  section and present it as a `&[u8]` slice. That means using some ELF

  loader on Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugRngLists, LittleEndian};

  

  let buf = [0x00, 0x01, 0x02, 0x03];

  let read_debug_rnglists_section_somehow = || &buf;

  let debug_rnglists =

      DebugRngLists::new(read_debug_rnglists_section_somehow(), LittleEndian);

  ```

#### Trait Implementations

##### `impl<R: clone::Clone> Clone for DebugRngLists<R>`

- <span id="debugrnglists-clone"></span>`fn clone(&self) -> DebugRngLists<R>` — [`DebugRngLists`](#debugrnglists)

##### `impl<R: marker::Copy> Copy for DebugRngLists<R>`

##### `impl<R: fmt::Debug> Debug for DebugRngLists<R>`

- <span id="debugrnglists-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugRngLists<R>`

- <span id="debugrnglists-default"></span>`fn default() -> DebugRngLists<R>` — [`DebugRngLists`](#debugrnglists)

##### `impl<R> Section for DebugRngLists<R>`

- <span id="debugrnglists-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../index.md#sectionid)

- <span id="debugrnglists-section-reader"></span>`fn reader(&self) -> &R`

### `RangeLists<R>`

```rust
struct RangeLists<R> {
    debug_ranges: DebugRanges<R>,
    debug_rnglists: DebugRngLists<R>,
}
```

The DWARF data found in `.debug_ranges` and `.debug_rnglists` sections.

#### Implementations

- <span id="rangelists-new"></span>`fn new(debug_ranges: DebugRanges<R>, debug_rnglists: DebugRngLists<R>) -> RangeLists<R>` — [`DebugRanges`](#debugranges), [`DebugRngLists`](#debugrnglists), [`RangeLists`](#rangelists)

  Construct a new `RangeLists` instance from the data in the `.debug_ranges` and

  `.debug_rnglists` sections.

- <span id="rangelists-debug-ranges"></span>`fn debug_ranges(&self) -> &DebugRanges<R>` — [`DebugRanges`](#debugranges)

  Return the `.debug_ranges` section.

- <span id="rangelists-set-debug-ranges"></span>`fn set_debug_ranges(&mut self, debug_ranges: DebugRanges<R>)` — [`DebugRanges`](#debugranges)

  Replace the `.debug_ranges` section.

  

  This is useful for `.dwo` files when using the GNU split-dwarf extension to DWARF 4.

- <span id="rangelists-debug-rnglists"></span>`fn debug_rnglists(&self) -> &DebugRngLists<R>` — [`DebugRngLists`](#debugrnglists)

  Return the `.debug_rnglists` section.

#### Trait Implementations

##### `impl<R: clone::Clone> Clone for RangeLists<R>`

- <span id="rangelists-clone"></span>`fn clone(&self) -> RangeLists<R>` — [`RangeLists`](#rangelists)

##### `impl<R: marker::Copy> Copy for RangeLists<R>`

##### `impl<R: fmt::Debug> Debug for RangeLists<R>`

- <span id="rangelists-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for RangeLists<R>`

- <span id="rangelists-default"></span>`fn default() -> RangeLists<R>` — [`RangeLists`](#rangelists)

### `RawRngListIter<R: Reader>`

```rust
struct RawRngListIter<R: Reader> {
    input: R,
    encoding: crate::common::Encoding,
    format: RangeListsFormat,
}
```

A raw iterator over an address range list.

This iterator does not perform any processing of the range entries,
such as handling base addresses.

#### Implementations

- <span id="rawrnglistiter-new"></span>`fn new(input: R, encoding: Encoding, format: RangeListsFormat) -> RawRngListIter<R>` — [`Encoding`](../index.md#encoding), [`RangeListsFormat`](rnglists/index.md#rangelistsformat), [`RawRngListIter`](#rawrnglistiter)

  Construct a `RawRngListIter`.

- <span id="rawrnglistiter-next"></span>`fn next(&mut self) -> Result<Option<RawRngListEntry<<R as >::Offset>>>` — [`Result`](../index.md#result), [`RawRngListEntry`](#rawrnglistentry), [`Reader`](#reader)

  Advance the iterator to the next range.

#### Trait Implementations

##### `impl<R: fmt::Debug + Reader> Debug for RawRngListIter<R>`

- <span id="rawrnglistiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for RawRngListIter<R>`

- <span id="rawrnglistiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="rawrnglistiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="rawrnglistiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: Reader> Iterator for RawRngListIter<R>`

- <span id="rawrnglistiter-iterator-type-item"></span>`type Item = Result<RawRngListEntry<<R as Reader>::Offset>, Error>`

- <span id="rawrnglistiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `RngListIter<R: Reader>`

```rust
struct RngListIter<R: Reader> {
    raw: RawRngListIter<R>,
    base_address: u64,
    debug_addr: crate::read::DebugAddr<R>,
    debug_addr_base: crate::common::DebugAddrBase<<R as >::Offset>,
}
```

An iterator over an address range list.

This iterator internally handles processing of base addresses and different
entry types.  Thus, it only returns range entries that are valid
and already adjusted for the base address.

#### Implementations

- <span id="rnglistiter-new"></span>`fn new(raw: RawRngListIter<R>, base_address: u64, debug_addr: DebugAddr<R>, debug_addr_base: DebugAddrBase<<R as >::Offset>) -> RngListIter<R>` — [`RawRngListIter`](#rawrnglistiter), [`DebugAddr`](#debugaddr), [`DebugAddrBase`](../index.md#debugaddrbase), [`Reader`](#reader), [`RngListIter`](#rnglistiter)

  Construct a `RngListIter`.

- <span id="rnglistiter-get-address"></span>`fn get_address(&self, index: DebugAddrIndex<<R as >::Offset>) -> Result<u64>` — [`DebugAddrIndex`](../index.md#debugaddrindex), [`Reader`](#reader), [`Result`](../index.md#result)

- <span id="rnglistiter-next"></span>`fn next(&mut self) -> Result<Option<Range>>` — [`Result`](../index.md#result), [`Range`](#range)

  Advance the iterator to the next range.

#### Trait Implementations

##### `impl<R: fmt::Debug + Reader> Debug for RngListIter<R>`

- <span id="rnglistiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for RngListIter<R>`

- <span id="rnglistiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="rnglistiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="rnglistiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: Reader> Iterator for RngListIter<R>`

- <span id="rnglistiter-iterator-type-item"></span>`type Item = Result<Range, Error>`

- <span id="rnglistiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `RawRange`

```rust
struct RawRange {
    pub begin: u64,
    pub end: u64,
}
```

A raw address range from the `.debug_ranges` section.

#### Fields

- **`begin`**: `u64`

  The beginning address of the range.

- **`end`**: `u64`

  The first address past the end of the range.

#### Implementations

- <span id="rawrange-is-end"></span>`fn is_end(&self) -> bool`

  Check if this is a range end entry.

- <span id="rawrange-is-base-address"></span>`fn is_base_address(&self, address_size: u8) -> bool`

  Check if this is a base address selection entry.

  

  A base address selection entry changes the base address that subsequent

  range entries are relative to.

- <span id="rawrange-parse"></span>`fn parse<R: Reader>(input: &mut R, address_size: u8) -> Result<RawRange>` — [`Result`](../index.md#result), [`RawRange`](rnglists/index.md#rawrange)

  Parse an address range entry from `.debug_ranges` or `.debug_loc`.

#### Trait Implementations

##### `impl Clone for RawRange`

- <span id="rawrange-clone"></span>`fn clone(&self) -> RawRange` — [`RawRange`](rnglists/index.md#rawrange)

##### `impl Copy for RawRange`

##### `impl Debug for RawRange`

- <span id="rawrange-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RawRange`

##### `impl Hash for RawRange`

- <span id="rawrange-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for RawRange`

- <span id="rawrange-partialeq-eq"></span>`fn eq(&self, other: &RawRange) -> bool` — [`RawRange`](rnglists/index.md#rawrange)

##### `impl StructuralPartialEq for RawRange`

### `Range`

```rust
struct Range {
    pub begin: u64,
    pub end: u64,
}
```

An address range from the `.debug_ranges`, `.debug_rnglists`, or `.debug_aranges` sections.

#### Fields

- **`begin`**: `u64`

  The beginning address of the range.

- **`end`**: `u64`

  The first address past the end of the range.

#### Implementations

- <span id="range-add-base-address"></span>`fn add_base_address(&mut self, base_address: u64, address_size: u8)`

  Add a base address to this range.

#### Trait Implementations

##### `impl Clone for Range`

- <span id="range-clone"></span>`fn clone(&self) -> Range` — [`Range`](#range)

##### `impl Copy for Range`

##### `impl Debug for Range`

- <span id="range-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Range`

##### `impl Hash for Range`

- <span id="range-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Range`

- <span id="range-ord-cmp"></span>`fn cmp(&self, other: &Range) -> cmp::Ordering` — [`Range`](#range)

##### `impl PartialEq for Range`

- <span id="range-partialeq-eq"></span>`fn eq(&self, other: &Range) -> bool` — [`Range`](#range)

##### `impl PartialOrd for Range`

- <span id="range-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Range) -> option::Option<cmp::Ordering>` — [`Range`](#range)

##### `impl StructuralPartialEq for Range`

### `DebugStr<R>`

```rust
struct DebugStr<R> {
    debug_str_section: R,
}
```

The `DebugStr` struct represents the DWARF strings
found in the `.debug_str` section.

#### Implementations

- <span id="debugstr-new"></span>`fn new(debug_str_section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugStr` instance from the data in the `.debug_str`

  section.

  

  It is the caller's responsibility to read the `.debug_str` section and

  present it as a `&[u8]` slice. That means using some ELF loader on

  Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugStr, LittleEndian};

  

  let buf = [0x00, 0x01, 0x02, 0x03];

  let read_debug_str_section_somehow = || &buf;

  let debug_str = DebugStr::new(read_debug_str_section_somehow(), LittleEndian);

  ```

#### Trait Implementations

##### `impl<R: clone::Clone> Clone for DebugStr<R>`

- <span id="debugstr-clone"></span>`fn clone(&self) -> DebugStr<R>` — [`DebugStr`](#debugstr)

##### `impl<R: marker::Copy> Copy for DebugStr<R>`

##### `impl<R: fmt::Debug> Debug for DebugStr<R>`

- <span id="debugstr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugStr<R>`

- <span id="debugstr-default"></span>`fn default() -> DebugStr<R>` — [`DebugStr`](#debugstr)

##### `impl<R> Section for DebugStr<R>`

- <span id="debugstr-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../index.md#sectionid)

- <span id="debugstr-section-reader"></span>`fn reader(&self) -> &R`

### `DebugStrOffsets<R>`

```rust
struct DebugStrOffsets<R> {
    section: R,
}
```

The raw contents of the `.debug_str_offsets` section.

#### Implementations

- <span id="debugstroffsets-get-str-offset"></span>`fn get_str_offset(&self, format: Format, base: DebugStrOffsetsBase<<R as >::Offset>, index: DebugStrOffsetsIndex<<R as >::Offset>) -> Result<DebugStrOffset<<R as >::Offset>>` — [`Format`](../index.md#format), [`DebugStrOffsetsBase`](../index.md#debugstroffsetsbase), [`Reader`](#reader), [`DebugStrOffsetsIndex`](../index.md#debugstroffsetsindex), [`Result`](../index.md#result), [`DebugStrOffset`](../index.md#debugstroffset)

  Returns the `.debug_str` offset at the given `base` and `index`.

  

  A set of entries in the `.debug_str_offsets` section consists of a header

  followed by a series of string table offsets.

  

  The `base` must be the `DW_AT_str_offsets_base` value from the compilation unit DIE.

  This is an offset that points to the first entry following the header.

  

  The `index` is the value of a `DW_FORM_strx` attribute.

  

  The `format` must be the DWARF format of the compilation unit. This format must

  match the header. However, note that we do not parse the header to validate this,

  since locating the header is unreliable, and the GNU extensions do not emit it.

#### Trait Implementations

##### `impl<R: clone::Clone> Clone for DebugStrOffsets<R>`

- <span id="debugstroffsets-clone"></span>`fn clone(&self) -> DebugStrOffsets<R>` — [`DebugStrOffsets`](#debugstroffsets)

##### `impl<R: marker::Copy> Copy for DebugStrOffsets<R>`

##### `impl<R: fmt::Debug> Debug for DebugStrOffsets<R>`

- <span id="debugstroffsets-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugStrOffsets<R>`

- <span id="debugstroffsets-default"></span>`fn default() -> DebugStrOffsets<R>` — [`DebugStrOffsets`](#debugstroffsets)

##### `impl<R> Section for DebugStrOffsets<R>`

- <span id="debugstroffsets-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../index.md#sectionid)

- <span id="debugstroffsets-section-reader"></span>`fn reader(&self) -> &R`

### `DebugLineStr<R>`

```rust
struct DebugLineStr<R> {
    section: R,
}
```

The `DebugLineStr` struct represents the DWARF strings
found in the `.debug_line_str` section.

#### Implementations

- <span id="debuglinestr-new"></span>`fn new(debug_line_str_section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugLineStr` instance from the data in the `.debug_line_str`

  section.

  

  It is the caller's responsibility to read the `.debug_line_str` section and

  present it as a `&[u8]` slice. That means using some ELF loader on

  Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugLineStr, LittleEndian};

  

  let buf = [0x00, 0x01, 0x02, 0x03];

  let read_debug_line_str_section_somehow = || &buf;

  let debug_str = DebugLineStr::new(read_debug_line_str_section_somehow(), LittleEndian);

  ```

#### Trait Implementations

##### `impl<R: clone::Clone> Clone for DebugLineStr<R>`

- <span id="debuglinestr-clone"></span>`fn clone(&self) -> DebugLineStr<R>` — [`DebugLineStr`](#debuglinestr)

##### `impl<R: marker::Copy> Copy for DebugLineStr<R>`

##### `impl<R: fmt::Debug> Debug for DebugLineStr<R>`

- <span id="debuglinestr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugLineStr<R>`

- <span id="debuglinestr-default"></span>`fn default() -> DebugLineStr<R>` — [`DebugLineStr`](#debuglinestr)

##### `impl<R> Section for DebugLineStr<R>`

- <span id="debuglinestr-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../index.md#sectionid)

- <span id="debuglinestr-section-reader"></span>`fn reader(&self) -> &R`

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

- <span id="debuginfo-clone"></span>`fn clone(&self) -> DebugInfo<R>` — [`DebugInfo`](#debuginfo)

##### `impl<R: marker::Copy> Copy for DebugInfo<R>`

##### `impl<R: fmt::Debug> Debug for DebugInfo<R>`

- <span id="debuginfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugInfo<R>`

- <span id="debuginfo-default"></span>`fn default() -> DebugInfo<R>` — [`DebugInfo`](#debuginfo)

##### `impl<R> Section for DebugInfo<R>`

- <span id="debuginfo-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../index.md#sectionid)

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

- <span id="debuginfounitheadersiter-next"></span>`fn next(&mut self) -> Result<Option<UnitHeader<R>>>` — [`Result`](../index.md#result), [`UnitHeader`](#unitheader)

  Advance the iterator to the next unit header.

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for DebugInfoUnitHeadersIter<R>`

- <span id="debuginfounitheadersiter-clone"></span>`fn clone(&self) -> DebugInfoUnitHeadersIter<R>` — [`DebugInfoUnitHeadersIter`](#debuginfounitheadersiter)

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

- <span id="unitheader-new"></span>`fn new(encoding: Encoding, unit_length: Offset, unit_type: UnitType<Offset>, debug_abbrev_offset: DebugAbbrevOffset<Offset>, section: SectionId, unit_offset: UnitSectionOffset<Offset>, entries_buf: R) -> Self` — [`Encoding`](../index.md#encoding), [`UnitType`](#unittype), [`DebugAbbrevOffset`](../index.md#debugabbrevoffset), [`SectionId`](../index.md#sectionid), [`UnitSectionOffset`](../index.md#unitsectionoffset)

  Construct a new `UnitHeader`.

#### Trait Implementations

##### `impl<R, Offset> Clone for UnitHeader<R, Offset>`

- <span id="unitheader-clone"></span>`fn clone(&self) -> UnitHeader<R, Offset>` — [`UnitHeader`](#unitheader)

##### `impl<R, Offset> Copy for UnitHeader<R, Offset>`

##### `impl<R, Offset> Debug for UnitHeader<R, Offset>`

- <span id="unitheader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Offset> Eq for UnitHeader<R, Offset>`

##### `impl<R, Offset> PartialEq for UnitHeader<R, Offset>`

- <span id="unitheader-partialeq-eq"></span>`fn eq(&self, other: &UnitHeader<R, Offset>) -> bool` — [`UnitHeader`](#unitheader)

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

- <span id="debugginginformationentry-new"></span>`fn new(tag: constants::DwTag, has_children: bool, attrs: Vec<Attribute<R>>, offset: UnitOffset<Offset>) -> Self` — [`DwTag`](../index.md#dwtag), [`Attribute`](#attribute), [`UnitOffset`](../index.md#unitoffset)

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

- <span id="debugginginformationentry-offset"></span>`fn offset(&self) -> UnitOffset<Offset>` — [`UnitOffset`](../index.md#unitoffset)

  Get this entry's offset.

- <span id="debugginginformationentry-tag"></span>`fn tag(&self) -> constants::DwTag` — [`DwTag`](../index.md#dwtag)

  Get this entry's tag.

- <span id="debugginginformationentry-has-children"></span>`fn has_children(&self) -> bool`

  Return true if this entry's type can have children, false otherwise.

- <span id="debugginginformationentry-attrs"></span>`fn attrs(&self) -> &[Attribute<R>]` — [`Attribute`](#attribute)

  Return this entry's set of attributes.

- <span id="debugginginformationentry-has-attr"></span>`fn has_attr(&self, name: constants::DwAt) -> bool` — [`DwAt`](../index.md#dwat)

  Return true if this entry has an attribute with the given name.

- <span id="debugginginformationentry-attr"></span>`fn attr(&self, name: constants::DwAt) -> Option<&Attribute<R>>` — [`DwAt`](../index.md#dwat), [`Attribute`](#attribute)

  Find the first attribute in this entry which has the given name.

  

  Returns `None` if no attribute is found.

- <span id="debugginginformationentry-attr-value-raw"></span>`fn attr_value_raw(&self, name: constants::DwAt) -> Option<AttributeValue<R>>` — [`DwAt`](../index.md#dwat), [`AttributeValue`](#attributevalue)

  Find the first attribute in this entry which has the given name,

  and return its raw value.

  

  Returns `None` if no attribute is found.

- <span id="debugginginformationentry-attr-value"></span>`fn attr_value(&self, name: constants::DwAt) -> Option<AttributeValue<R>>` — [`DwAt`](../index.md#dwat), [`AttributeValue`](#attributevalue)

  Find the first attribute in this entry which has the given name,

  and return its normalized value.

  

  Returns `None` if no attribute is found.

- <span id="debugginginformationentry-sibling"></span>`fn sibling(&self) -> Option<UnitOffset<<R as >::Offset>>` — [`UnitOffset`](../index.md#unitoffset), [`Reader`](#reader)

  Use the `DW_AT_sibling` attribute to find the offset for the

  next sibling. Returns `None` if the attribute is missing or invalid.

#### Trait Implementations

##### `impl<R, Offset> Clone for DebuggingInformationEntry<R, Offset>`

- <span id="debugginginformationentry-clone"></span>`fn clone(&self) -> DebuggingInformationEntry<R, Offset>` — [`DebuggingInformationEntry`](#debugginginformationentry)

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

- <span id="attribute-name"></span>`fn name(&self) -> constants::DwAt` — [`DwAt`](../index.md#dwat)

  Get this attribute's name.

- <span id="attribute-form"></span>`fn form(&self) -> constants::DwForm` — [`DwForm`](../index.md#dwform)

  Get the form that was used to encode this attribute's value.

- <span id="attribute-raw-value"></span>`fn raw_value(&self) -> AttributeValue<R>` — [`AttributeValue`](#attributevalue)

  Get this attribute's raw value.

- <span id="attribute-value"></span>`fn value(&self) -> AttributeValue<R>` — [`AttributeValue`](#attributevalue)

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

- <span id="attribute-offset-value"></span>`fn offset_value(&self) -> Option<<R as >::Offset>` — [`Reader`](#reader)

  Try to convert this attribute's value to an offset.

- <span id="attribute-exprloc-value"></span>`fn exprloc_value(&self) -> Option<Expression<R>>` — [`Expression`](#expression)

  Try to convert this attribute's value to an expression or location buffer.

  

  Expressions and locations may be `DW_FORM_block*` or `DW_FORM_exprloc`.

  The standard doesn't mention `DW_FORM_block*` as a possible form, but

  it is encountered in practice.

- <span id="attribute-string-value"></span>`fn string_value(&self, debug_str: &DebugStr<R>) -> Option<R>` — [`DebugStr`](#debugstr)

  Try to return this attribute's value as a string slice.

  

  If this attribute's value is either an inline `DW_FORM_string` string,

  or a `DW_FORM_strp` reference to an offset into the `.debug_str`

  section, return the attribute's string value as `Some`. Other attribute

  value forms are returned as `None`.

  

  Warning: this function does not handle all possible string forms.

  Use `Dwarf::attr_string` instead.

- <span id="attribute-string-value-sup"></span>`fn string_value_sup(&self, debug_str: &DebugStr<R>, debug_str_sup: Option<&DebugStr<R>>) -> Option<R>` — [`DebugStr`](#debugstr)

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

- <span id="attribute-clone"></span>`fn clone(&self) -> Attribute<R>` — [`Attribute`](#attribute)

##### `impl<R: marker::Copy + Reader> Copy for Attribute<R>`

##### `impl<R: fmt::Debug + Reader> Debug for Attribute<R>`

- <span id="attribute-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: cmp::Eq + Reader> Eq for Attribute<R>`

##### `impl<R: cmp::PartialEq + Reader> PartialEq for Attribute<R>`

- <span id="attribute-partialeq-eq"></span>`fn eq(&self, other: &Attribute<R>) -> bool` — [`Attribute`](#attribute)

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

- <span id="entriesraw-new"></span>`fn new(input: R, encoding: Encoding, abbreviations: &'abbrev Abbreviations, offset: UnitOffset<<R as >::Offset>) -> Self` — [`Encoding`](../index.md#encoding), [`Abbreviations`](#abbreviations), [`UnitOffset`](../index.md#unitoffset), [`Reader`](#reader)

  Construct a new `EntriesRaw`.

  

  `input` may be anywhere within the entries for a unit, including partway

  through an entry. It is up to the caller to know what needs to be parsed

  next.

  

  `offset` may be any value. It is used as the initial value returned by

  `Self::next_offset`.

- <span id="entriesraw-empty"></span>`fn empty(&mut self)`

- <span id="entriesraw-is-empty"></span>`fn is_empty(&self) -> bool`

  Return true if there is no more input.

- <span id="entriesraw-seek-forward"></span>`fn seek_forward(&mut self, offset: UnitOffset<<R as >::Offset>, depth: isize) -> bool` — [`UnitOffset`](../index.md#unitoffset), [`Reader`](#reader)

- <span id="entriesraw-next-offset"></span>`fn next_offset(&self) -> UnitOffset<<R as >::Offset>` — [`UnitOffset`](../index.md#unitoffset), [`Reader`](#reader)

  Return the unit offset at which the reader will read next.

  

  If you want the offset of the next entry, then this must be called prior to reading

  the next entry.

- <span id="entriesraw-next-depth"></span>`fn next_depth(&self) -> isize`

  Return the depth of the next entry.

  

  This depth is updated when `read_abbreviation` is called, and is updated

  based on null entries and the `has_children` field in the abbreviation.

- <span id="entriesraw-read-entry"></span>`fn read_entry(&mut self, entry: &mut DebuggingInformationEntry<R>) -> Result<bool>` — [`DebuggingInformationEntry`](#debugginginformationentry), [`Result`](../index.md#result)

  Read an entry into a placeholder.

  

  This reads the abbreviation code and all attributes. The attributes are stored in

  a `Vec` in the entry, so you should try to reuse this entry for future calls.

  

  Returns `Ok(false)` if a null entry is read. All fields in the entry will be

  set appropriately for a null entry.

  

  Returns `Err` if an error occurred. Some fields in the entry may be modified

  depending on where the error occurred.

- <span id="entriesraw-read-abbreviation"></span>`fn read_abbreviation(&mut self) -> Result<Option<&'abbrev Abbreviation>>` — [`Result`](../index.md#result), [`Abbreviation`](#abbreviation)

  Read an abbreviation code and lookup the corresponding `Abbreviation`.

  

  Returns `Ok(None)` for null entries.

  

  Returns `Err` if end of input is reached, or the code is invalid.

- <span id="entriesraw-read-attribute"></span>`fn read_attribute(&mut self, spec: AttributeSpecification) -> Result<Attribute<R>>` — [`AttributeSpecification`](#attributespecification), [`Result`](../index.md#result), [`Attribute`](#attribute)

  Read an attribute.

  

  This function is never inlined. Consider using `read_attribute_inline` instead

  if you only call this from a small number of places.

- <span id="entriesraw-read-attribute-inline"></span>`fn read_attribute_inline(&mut self, spec: AttributeSpecification) -> Result<Attribute<R>>` — [`AttributeSpecification`](#attributespecification), [`Result`](../index.md#result), [`Attribute`](#attribute)

  Read an attribute.

  

  Identical to `read_attribute`, but has the `#[inline(always)]` attribute.

  This allows better optimisation at the cost of code size.

- <span id="entriesraw-read-attributes"></span>`fn read_attributes(&mut self, specs: &[AttributeSpecification], attrs: &mut Vec<Attribute<R>>) -> Result<()>` — [`AttributeSpecification`](#attributespecification), [`Attribute`](#attribute), [`Result`](../index.md#result)

  Read all attributes into a `Vec`.

  

  This will clear `attrs` before reading.

  

  It is recommended to reuse the same `Vec` for multiple calls to avoid allocations.

- <span id="entriesraw-skip-attributes"></span>`fn skip_attributes(&mut self, specs: &[AttributeSpecification]) -> Result<()>` — [`AttributeSpecification`](#attributespecification), [`Result`](../index.md#result)

  Skip all the attributes of an abbreviation.

#### Trait Implementations

##### `impl<R> Clone for EntriesRaw<'abbrev, R>`

- <span id="entriesraw-clone"></span>`fn clone(&self) -> EntriesRaw<'abbrev, R>` — [`EntriesRaw`](#entriesraw)

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

- <span id="entriescursor-new"></span>`fn new(input: R, encoding: Encoding, abbreviations: &'abbrev Abbreviations, offset: UnitOffset<<R as >::Offset>) -> Self` — [`Encoding`](../index.md#encoding), [`Abbreviations`](#abbreviations), [`UnitOffset`](../index.md#unitoffset), [`Reader`](#reader)

- <span id="entriescursor-current"></span>`fn current(&self) -> Option<&DebuggingInformationEntry<R>>` — [`DebuggingInformationEntry`](#debugginginformationentry)

  Get a reference to the entry that the cursor is currently pointing to.

  

  If the cursor is not pointing at an entry, or if the current entry is a

  null entry, then `None` is returned.

- <span id="entriescursor-offset"></span>`fn offset(&self) -> UnitOffset<<R as >::Offset>` — [`UnitOffset`](../index.md#unitoffset), [`Reader`](#reader)

  The unit offset of the current DIE.

  

  This works even if the cursor is positioned at a null DIE.

- <span id="entriescursor-depth"></span>`fn depth(&self) -> isize`

  The tree depth of the current DIE.

  

  This works even if the cursor is positioned at a null DIE.

- <span id="entriescursor-next-offset"></span>`fn next_offset(&self) -> UnitOffset<<R as >::Offset>` — [`UnitOffset`](../index.md#unitoffset), [`Reader`](#reader)

  The unit offset of the next DIE that `Self::next_entry` will read.

- <span id="entriescursor-next-depth"></span>`fn next_depth(&self) -> isize`

  The tree depth of the next DIE that `Self::next_entry` will read.

- <span id="entriescursor-next-entry"></span>`fn next_entry(&mut self) -> Result<bool>` — [`Result`](../index.md#result)

  Move the cursor to the next DIE in the tree.

  

  Returns `true` if there is a next entry, even if this entry is null.

  If there is no next entry, then `false` is returned.

- <span id="entriescursor-next-dfs"></span>`fn next_dfs(&mut self) -> Result<Option<&DebuggingInformationEntry<R>>>` — [`Result`](../index.md#result), [`DebuggingInformationEntry`](#debugginginformationentry)

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

- <span id="entriescursor-next-sibling"></span>`fn next_sibling(&mut self) -> Result<Option<&DebuggingInformationEntry<R>>>` — [`Result`](../index.md#result), [`DebuggingInformationEntry`](#debugginginformationentry)

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

- <span id="entriescursor-clone"></span>`fn clone(&self) -> EntriesCursor<'abbrev, R>` — [`EntriesCursor`](#entriescursor)

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

- <span id="entriestree-new"></span>`fn new(root: R, encoding: Encoding, abbreviations: &'abbrev Abbreviations, offset: UnitOffset<<R as >::Offset>) -> Self` — [`Encoding`](../index.md#encoding), [`Abbreviations`](#abbreviations), [`UnitOffset`](../index.md#unitoffset), [`Reader`](#reader)

- <span id="entriestree-root"></span>`fn root<'me>(self: &'me mut Self) -> Result<EntriesTreeNode<'abbrev, 'me, R>>` — [`Result`](../index.md#result), [`EntriesTreeNode`](#entriestreenode)

  Returns the root node of the tree.

- <span id="entriestree-next"></span>`fn next(&mut self, depth: isize) -> Result<bool>` — [`Result`](../index.md#result)

  Move the cursor to the next entry at the specified depth.

  

  Requires `depth <= self.depth + 1`.

  

  Returns `true` if successful.

#### Trait Implementations

##### `impl<R> Clone for EntriesTree<'abbrev, R>`

- <span id="entriestree-clone"></span>`fn clone(&self) -> EntriesTree<'abbrev, R>` — [`EntriesTree`](#entriestree)

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

- <span id="entriestreenode-new"></span>`fn new(tree: &'tree mut EntriesTree<'abbrev, R>, depth: isize) -> EntriesTreeNode<'abbrev, 'tree, R>` — [`EntriesTree`](#entriestree), [`EntriesTreeNode`](#entriestreenode)

- <span id="entriestreenode-entry"></span>`fn entry(&self) -> &DebuggingInformationEntry<R>` — [`DebuggingInformationEntry`](#debugginginformationentry)

  Returns the current entry in the tree.

- <span id="entriestreenode-children"></span>`fn children(self) -> EntriesTreeIter<'abbrev, 'tree, R>` — [`EntriesTreeIter`](#entriestreeiter)

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

- <span id="entriestreeiter-new"></span>`fn new(tree: &'tree mut EntriesTree<'abbrev, R>, depth: isize) -> EntriesTreeIter<'abbrev, 'tree, R>` — [`EntriesTree`](#entriestree), [`EntriesTreeIter`](#entriestreeiter)

- <span id="entriestreeiter-next"></span>`fn next<'me>(self: &'me mut Self) -> Result<Option<EntriesTreeNode<'abbrev, 'me, R>>>` — [`Result`](../index.md#result), [`EntriesTreeNode`](#entriestreenode)

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

- <span id="debugtypes-clone"></span>`fn clone(&self) -> DebugTypes<R>` — [`DebugTypes`](#debugtypes)

##### `impl<R: marker::Copy> Copy for DebugTypes<R>`

##### `impl<R: fmt::Debug> Debug for DebugTypes<R>`

- <span id="debugtypes-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugTypes<R>`

- <span id="debugtypes-default"></span>`fn default() -> DebugTypes<R>` — [`DebugTypes`](#debugtypes)

##### `impl<R> Section for DebugTypes<R>`

- <span id="debugtypes-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../index.md#sectionid)

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

- <span id="debugtypesunitheadersiter-next"></span>`fn next(&mut self) -> Result<Option<UnitHeader<R>>>` — [`Result`](../index.md#result), [`UnitHeader`](#unitheader)

  Advance the iterator to the next type unit header.

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for DebugTypesUnitHeadersIter<R>`

- <span id="debugtypesunitheadersiter-clone"></span>`fn clone(&self) -> DebugTypesUnitHeadersIter<R>` — [`DebugTypesUnitHeadersIter`](#debugtypesunitheadersiter)

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

### `Error`

```rust
enum Error {
    Io,
    PcRelativePointerButSectionBaseIsUndefined,
    TextRelativePointerButTextBaseIsUndefined,
    DataRelativePointerButDataBaseIsUndefined,
    FuncRelativePointerInBadContext,
    CannotParseOmitPointerEncoding,
    BadUnsignedLeb128,
    BadSignedLeb128,
    AbbreviationTagZero,
    AttributeNameZero,
    AttributeFormZero,
    InvalidAbbreviationChildren(constants::DwChildren),
    UnknownForm(constants::DwForm),
    DuplicateAbbreviationCode(u64),
    UnknownReservedLength(u32),
    UnknownVersion(u64),
    InvalidAbbreviationCode(u64),
    UnexpectedEof(ReaderOffsetId),
    UnknownLocListsEntry(constants::DwLle),
    UnknownRangeListsEntry(constants::DwRle),
    UnsupportedAddressSize(u8),
    UnsupportedOffsetSize(u8),
    MinimumInstructionLengthZero,
    MaximumOperationsPerInstructionZero,
    LineRangeZero,
    OpcodeBaseZero,
    BadUtf8,
    NotCieId(u64),
    NotCiePointer(u64),
    BadBranchTarget(u64),
    InvalidPushObjectAddress,
    NotEnoughStackItems,
    TooManyIterations,
    InvalidExpression(constants::DwOp),
    UnsupportedEvaluation,
    InvalidPiece,
    InvalidExpressionTerminator(u64),
    DivisionByZero,
    TypeMismatch,
    IntegralTypeRequired,
    UnsupportedTypeOperation,
    InvalidShiftExpression,
    InvalidDerefSize(u8),
    UnknownCallFrameInstruction(constants::DwCfa),
    InvalidCfiSetLoc(u64),
    AddressOverflow,
    CfiInstructionInInvalidContext,
    PopWithEmptyStack,
    NoUnwindInfoForAddress,
    UnsupportedOffset,
    UnknownPointerEncoding(constants::DwEhPe),
    NoEntryAtGivenOffset(u64),
    OffsetOutOfBounds(u64),
    UnknownAugmentation,
    UnsupportedPointerEncoding(constants::DwEhPe),
    UnsupportedIndirectPointer,
    UnsupportedRegister(u64),
    TooManyRegisterRules,
    StackFull,
    UnknownUnitType(constants::DwUt),
    UnsupportedSegmentSize(u8),
    MissingUnitDie,
    MissingSplitUnit,
    UnsupportedAttributeForm(constants::DwForm),
    MissingFileEntryFormatPath,
    ExpectedStringAttributeValue,
    InvalidImplicitConst,
    UnsupportedIndexSectionCount(u32),
    InvalidIndexSlotCount(u32),
    InvalidIndexRow(u32),
    UnknownIndexSection(constants::DwSect),
    UnknownIndexSectionV2(constants::DwSectV2),
    InvalidMacinfoType(constants::DwMacinfo),
    InvalidMacroType(constants::DwMacro),
    UnsupportedOpcodeOperandsTable,
    InvalidNameAttributeIndex(u64),
}
```

An error that occurred when parsing.

#### Variants

- **`Io`**

  An I/O error occurred while reading.

- **`PcRelativePointerButSectionBaseIsUndefined`**

  Found a PC relative pointer, but the section base is undefined.

- **`TextRelativePointerButTextBaseIsUndefined`**

  Found a `.text` relative pointer, but the `.text` base is undefined.

- **`DataRelativePointerButDataBaseIsUndefined`**

  Found a data relative pointer, but the data base is undefined.

- **`FuncRelativePointerInBadContext`**

  Found a function relative pointer in a context that does not have a
  function base.

- **`CannotParseOmitPointerEncoding`**

  Cannot parse a pointer with a `DW_EH_PE_omit` encoding.

- **`BadUnsignedLeb128`**

  An error parsing an unsigned LEB128 value.

- **`BadSignedLeb128`**

  An error parsing a signed LEB128 value.

- **`AbbreviationTagZero`**

  An abbreviation declared that its tag is zero, but zero is reserved for
  null records.

- **`AttributeNameZero`**

  An attribute specification declared that its name is zero, but zero is
  reserved for null records.

- **`AttributeFormZero`**

  An attribute specification declared that its form is zero, but zero is
  reserved for null records.

- **`InvalidAbbreviationChildren`**

  The abbreviation's has-children byte was not one of
  `DW_CHILDREN_{yes,no}`.

- **`UnknownForm`**

  Found an unknown `DW_FORM_*` type.

- **`DuplicateAbbreviationCode`**

  Found an abbreviation code that has already been used.

- **`UnknownReservedLength`**

  Found an unknown reserved length value.

- **`UnknownVersion`**

  Found an unknown DWARF version.

- **`InvalidAbbreviationCode`**

  Found an entry with an invalid abbreviation code.

- **`UnexpectedEof`**

  Hit the end of input before it was expected.

- **`UnknownLocListsEntry`**

  Found an unknown location-lists format.

- **`UnknownRangeListsEntry`**

  Found an unknown range-lists format.

- **`UnsupportedAddressSize`**

  The specified address size is not supported.

- **`UnsupportedOffsetSize`**

  The specified offset size is not supported.

- **`MinimumInstructionLengthZero`**

  The minimum instruction length must not be zero.

- **`MaximumOperationsPerInstructionZero`**

  The maximum operations per instruction must not be zero.

- **`LineRangeZero`**

  The line range must not be zero.

- **`OpcodeBaseZero`**

  The opcode base must not be zero.

- **`BadUtf8`**

  Found an invalid UTF-8 string.

- **`NotCieId`**

  Expected to find the CIE ID, but found something else.

- **`NotCiePointer`**

  Expected to find a pointer to a CIE, but found the CIE ID instead.

- **`BadBranchTarget`**

  Invalid branch target for a DW_OP_bra or DW_OP_skip.

- **`InvalidPushObjectAddress`**

  DW_OP_push_object_address used but no address passed in.

- **`NotEnoughStackItems`**

  Not enough items on the stack when evaluating an expression.

- **`TooManyIterations`**

  Too many iterations to compute the expression.

- **`InvalidExpression`**

  An unrecognized operation was found while parsing a DWARF
  expression.

- **`UnsupportedEvaluation`**

  An unsupported operation was found while evaluating a DWARF expression.

- **`InvalidPiece`**

  The expression had a piece followed by an expression
  terminator without a piece.

- **`InvalidExpressionTerminator`**

  An expression-terminating operation was followed by something
  other than the end of the expression or a piece operation.

- **`DivisionByZero`**

  Division or modulus by zero when evaluating an expression.

- **`TypeMismatch`**

  An expression operation used mismatching types.

- **`IntegralTypeRequired`**

  An expression operation required an integral type but saw a
  floating point type.

- **`UnsupportedTypeOperation`**

  An expression operation used types that are not supported.

- **`InvalidShiftExpression`**

  The shift value in an expression must be a non-negative integer.

- **`InvalidDerefSize`**

  The size of a deref expression must not be larger than the size of an address.

- **`UnknownCallFrameInstruction`**

  An unknown DW_CFA_* instruction.

- **`InvalidCfiSetLoc`**

  A `DW_CFA_set_loc` instruction moved the address backward.

- **`AddressOverflow`**

  An address calculation overflowed.
  
  This is returned in cases where the address is expected to be
  larger than a previous address, but the calculation overflowed.

- **`CfiInstructionInInvalidContext`**

  Encountered a call frame instruction in a context in which it is not
  valid.

- **`PopWithEmptyStack`**

  When evaluating call frame instructions, found a `DW_CFA_restore_state`
  stack pop instruction, but the stack was empty, and had nothing to pop.

- **`NoUnwindInfoForAddress`**

  Do not have unwind info for the given address.

- **`UnsupportedOffset`**

  An offset value was larger than the maximum supported value.

- **`UnknownPointerEncoding`**

  The given pointer encoding is either unknown or invalid.

- **`NoEntryAtGivenOffset`**

  Did not find an entry at the given offset.

- **`OffsetOutOfBounds`**

  The given offset is out of bounds.

- **`UnknownAugmentation`**

  Found an unknown CFI augmentation.

- **`UnsupportedPointerEncoding`**

  We do not support the given pointer encoding yet.

- **`UnsupportedIndirectPointer`**

  We do not support dereferencing indirect pointers.

- **`UnsupportedRegister`**

  Registers larger than `u16` are not supported.

- **`TooManyRegisterRules`**

  The CFI program defined more register rules than we have storage for.

- **`StackFull`**

  Attempted to push onto the CFI or evaluation stack, but it was already
  at full capacity.

- **`UnknownUnitType`**

  The `DW_UT_*` value for this unit is not supported yet.

- **`UnsupportedSegmentSize`**

  Nonzero segment selector sizes aren't supported yet.

- **`MissingUnitDie`**

  A compilation unit or type unit is missing its top level DIE.

- **`MissingSplitUnit`**

  A split DWARF section does not contain the split compilation unit.

- **`UnsupportedAttributeForm`**

  A DIE attribute used an unsupported form.

- **`MissingFileEntryFormatPath`**

  Missing DW_LNCT_path in file entry format.

- **`ExpectedStringAttributeValue`**

  Expected an attribute value to be a string form.

- **`InvalidImplicitConst`**

  An attribute with an indirect form cannot use `DW_FORM_implicit_const`.

- **`UnsupportedIndexSectionCount`**

  Invalid section count in `.dwp` index.

- **`InvalidIndexSlotCount`**

  Invalid slot count in `.dwp` index.

- **`InvalidIndexRow`**

  Invalid row index in `.dwp` index.

- **`UnknownIndexSection`**

  Unknown section type in `.dwp` index.

- **`UnknownIndexSectionV2`**

  Unknown section type in version 2 `.dwp` index.

- **`InvalidMacinfoType`**

  Invalid macinfo type in `.debug_macinfo`.

- **`InvalidMacroType`**

  Invalid macro type in `.debug_macro`.

- **`UnsupportedOpcodeOperandsTable`**

  The optional `opcode_operands_table` in `.debug_macro` is currently not supported.

- **`InvalidNameAttributeIndex`**

  Invalid index in a `.debug_names` attribute value.

#### Trait Implementations

##### `impl Clone for Error`

- <span id="error-clone"></span>`fn clone(&self) -> Error` — [`Error`](../index.md#error)

##### `impl Copy for Error`

##### `impl Debug for Error`

- <span id="error-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Error`

- <span id="error-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> ::core::result::Result<(), fmt::Error>`

##### `impl Eq for Error`

##### `impl Error for Error`

##### `impl PartialEq for Error`

- <span id="error-partialeq-eq"></span>`fn eq(&self, other: &Error) -> bool` — [`Error`](../index.md#error)

##### `impl StructuralPartialEq for Error`

##### `impl ToString for Error`

- <span id="error-tostring-to-string"></span>`fn to_string(&self) -> String`

### `CieOrFde<'bases, Section, R>`

```rust
enum CieOrFde<'bases, Section, R>
where
    R: Reader,
    Section: UnwindSection<R> {
    Cie(CommonInformationEntry<R>),
    Fde(PartialFrameDescriptionEntry<'bases, Section, R>),
}
```

Either a `CommonInformationEntry` (CIE) or a `FrameDescriptionEntry` (FDE).

#### Variants

- **`Cie`**

  This CFI entry is a `CommonInformationEntry`.

- **`Fde`**

  This CFI entry is a `FrameDescriptionEntry`, however fully parsing it
  requires parsing its CIE first, so it is left in a partially parsed
  state.

#### Trait Implementations

##### `impl<Section, R> Clone for CieOrFde<'bases, Section, R>`

- <span id="cieorfde-clone"></span>`fn clone(&self) -> CieOrFde<'bases, Section, R>` — [`CieOrFde`](#cieorfde)

##### `impl<Section, R> Debug for CieOrFde<'bases, Section, R>`

- <span id="cieorfde-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Section, R> Eq for CieOrFde<'bases, Section, R>`

##### `impl<Section, R> PartialEq for CieOrFde<'bases, Section, R>`

- <span id="cieorfde-partialeq-eq"></span>`fn eq(&self, other: &CieOrFde<'bases, Section, R>) -> bool` — [`CieOrFde`](#cieorfde)

##### `impl<Section, R> StructuralPartialEq for CieOrFde<'bases, Section, R>`

### `CfaRule<T: ReaderOffset>`

```rust
enum CfaRule<T: ReaderOffset> {
    RegisterAndOffset {
        register: crate::common::Register,
        offset: i64,
    },
    Expression(UnwindExpression<T>),
}
```

The canonical frame address (CFA) recovery rules.

#### Variants

- **`RegisterAndOffset`**

  The CFA is given offset from the given register's value.

- **`Expression`**

  The CFA is obtained by evaluating a DWARF expression program.

#### Implementations

- <span id="cfarule-is-default"></span>`fn is_default(&self) -> bool`

#### Trait Implementations

##### `impl<T: clone::Clone + ReaderOffset> Clone for CfaRule<T>`

- <span id="cfarule-clone"></span>`fn clone(&self) -> CfaRule<T>` — [`CfaRule`](#cfarule)

##### `impl<T: fmt::Debug + ReaderOffset> Debug for CfaRule<T>`

- <span id="cfarule-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: ReaderOffset> Default for CfaRule<T>`

- <span id="cfarule-default"></span>`fn default() -> Self`

##### `impl<T: cmp::Eq + ReaderOffset> Eq for CfaRule<T>`

##### `impl<T: cmp::PartialEq + ReaderOffset> PartialEq for CfaRule<T>`

- <span id="cfarule-partialeq-eq"></span>`fn eq(&self, other: &CfaRule<T>) -> bool` — [`CfaRule`](#cfarule)

##### `impl<T: ReaderOffset> StructuralPartialEq for CfaRule<T>`

### `RegisterRule<T: ReaderOffset>`

```rust
enum RegisterRule<T: ReaderOffset> {
    Undefined,
    SameValue,
    Offset(i64),
    ValOffset(i64),
    Register(crate::common::Register),
    Expression(UnwindExpression<T>),
    ValExpression(UnwindExpression<T>),
    Architectural,
    Constant(u64),
}
```

An entry in the abstract CFI table that describes how to find the value of a
register.

"The register columns contain rules that describe whether a given register
has been saved and the rule to find the value for the register in the
previous frame."

#### Variants

- **`Undefined`**

  > A register that has this rule has no recoverable value in the previous
  > frame. (By convention, it is not preserved by a callee.)

- **`SameValue`**

  > This register has not been modified from the previous frame. (By
  > convention, it is preserved by the callee, but the callee has not
  > modified it.)

- **`Offset`**

  "The previous value of this register is saved at the address CFA+N where
  CFA is the current CFA value and N is a signed offset."

- **`ValOffset`**

  "The previous value of this register is the value CFA+N where CFA is the
  current CFA value and N is a signed offset."

- **`Register`**

  "The previous value of this register is stored in another register
  numbered R."

- **`Expression`**

  "The previous value of this register is located at the address produced
  by executing the DWARF expression."

- **`ValExpression`**

  "The previous value of this register is the value produced by executing
  the DWARF expression."

- **`Architectural`**

  "The rule is defined externally to this specification by the augmenter."

- **`Constant`**

  This is a pseudo-register with a constant value.

#### Implementations

- <span id="registerrule-is-defined"></span>`fn is_defined(&self) -> bool`

#### Trait Implementations

##### `impl<T: clone::Clone + ReaderOffset> Clone for RegisterRule<T>`

- <span id="registerrule-clone"></span>`fn clone(&self) -> RegisterRule<T>` — [`RegisterRule`](#registerrule)

##### `impl<T: fmt::Debug + ReaderOffset> Debug for RegisterRule<T>`

- <span id="registerrule-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq + ReaderOffset> Eq for RegisterRule<T>`

##### `impl<T: cmp::PartialEq + ReaderOffset> PartialEq for RegisterRule<T>`

- <span id="registerrule-partialeq-eq"></span>`fn eq(&self, other: &RegisterRule<T>) -> bool` — [`RegisterRule`](#registerrule)

##### `impl<T: ReaderOffset> StructuralPartialEq for RegisterRule<T>`

### `CallFrameInstruction<T: ReaderOffset>`

```rust
enum CallFrameInstruction<T: ReaderOffset> {
    SetLoc {
        address: u64,
    },
    AdvanceLoc {
        delta: u32,
    },
    DefCfa {
        register: crate::common::Register,
        offset: u64,
    },
    DefCfaSf {
        register: crate::common::Register,
        factored_offset: i64,
    },
    DefCfaRegister {
        register: crate::common::Register,
    },
    DefCfaOffset {
        offset: u64,
    },
    DefCfaOffsetSf {
        factored_offset: i64,
    },
    DefCfaExpression {
        expression: UnwindExpression<T>,
    },
    Undefined {
        register: crate::common::Register,
    },
    SameValue {
        register: crate::common::Register,
    },
    Offset {
        register: crate::common::Register,
        factored_offset: u64,
    },
    OffsetExtendedSf {
        register: crate::common::Register,
        factored_offset: i64,
    },
    ValOffset {
        register: crate::common::Register,
        factored_offset: u64,
    },
    ValOffsetSf {
        register: crate::common::Register,
        factored_offset: i64,
    },
    Register {
        dest_register: crate::common::Register,
        src_register: crate::common::Register,
    },
    Expression {
        register: crate::common::Register,
        expression: UnwindExpression<T>,
    },
    ValExpression {
        register: crate::common::Register,
        expression: UnwindExpression<T>,
    },
    Restore {
        register: crate::common::Register,
    },
    RememberState,
    RestoreState,
    ArgsSize {
        size: u64,
    },
    NegateRaState,
    Nop,
}
```

A parsed call frame instruction.

#### Variants

- **`SetLoc`**

  > 1. DW_CFA_set_loc
  >
  > The DW_CFA_set_loc instruction takes a single operand that represents
  > a target address. The required action is to create a new table row
  > using the specified address as the location. All other values in the
  > new row are initially identical to the current row. The new location
  > value is always greater than the current one. If the segment_size
  > field of this FDE's CIE is non- zero, the initial location is preceded
  > by a segment selector of the given length.

- **`AdvanceLoc`**

  The `AdvanceLoc` instruction is used for all of `DW_CFA_advance_loc` and
  `DW_CFA_advance_loc{1,2,4}`.
  
  > 2. DW_CFA_advance_loc
  >
  > The DW_CFA_advance instruction takes a single operand (encoded with
  > the opcode) that represents a constant delta. The required action is
  > to create a new table row with a location value that is computed by
  > taking the current entry’s location value and adding the value of
  > delta * code_alignment_factor. All other values in the new row are
  > initially identical to the current row.

- **`DefCfa`**

  > 1. DW_CFA_def_cfa
  >
  > The DW_CFA_def_cfa instruction takes two unsigned LEB128 operands
  > representing a register number and a (non-factored) offset. The
  > required action is to define the current CFA rule to use the provided
  > register and offset.

- **`DefCfaSf`**

  > 2. DW_CFA_def_cfa_sf
  >
  > The DW_CFA_def_cfa_sf instruction takes two operands: an unsigned
  > LEB128 value representing a register number and a signed LEB128
  > factored offset. This instruction is identical to DW_CFA_def_cfa
  > except that the second operand is signed and factored. The resulting
  > offset is factored_offset * data_alignment_factor.

- **`DefCfaRegister`**

  > 3. DW_CFA_def_cfa_register
  >
  > The DW_CFA_def_cfa_register instruction takes a single unsigned LEB128
  > operand representing a register number. The required action is to
  > define the current CFA rule to use the provided register (but to keep
  > the old offset). This operation is valid only if the current CFA rule
  > is defined to use a register and offset.

- **`DefCfaOffset`**

  > 4. DW_CFA_def_cfa_offset
  >
  > The DW_CFA_def_cfa_offset instruction takes a single unsigned LEB128
  > operand representing a (non-factored) offset. The required action is
  > to define the current CFA rule to use the provided offset (but to keep
  > the old register). This operation is valid only if the current CFA
  > rule is defined to use a register and offset.

- **`DefCfaOffsetSf`**

  > 5. DW_CFA_def_cfa_offset_sf
  >
  > The DW_CFA_def_cfa_offset_sf instruction takes a signed LEB128 operand
  > representing a factored offset. This instruction is identical to
  > DW_CFA_def_cfa_offset except that the operand is signed and
  > factored. The resulting offset is factored_offset *
  > data_alignment_factor. This operation is valid only if the current CFA
  > rule is defined to use a register and offset.

- **`DefCfaExpression`**

  > 6. DW_CFA_def_cfa_expression
  >
  > The DW_CFA_def_cfa_expression instruction takes a single operand
  > encoded as a DW_FORM_exprloc value representing a DWARF
  > expression. The required action is to establish that expression as the
  > means by which the current CFA is computed.

- **`Undefined`**

  > 1. DW_CFA_undefined
  >
  > The DW_CFA_undefined instruction takes a single unsigned LEB128
  > operand that represents a register number. The required action is to
  > set the rule for the specified register to “undefined.”

- **`SameValue`**

  > 2. DW_CFA_same_value
  >
  > The DW_CFA_same_value instruction takes a single unsigned LEB128
  > operand that represents a register number. The required action is to
  > set the rule for the specified register to “same value.”

- **`Offset`**

  The `Offset` instruction represents both `DW_CFA_offset` and
  `DW_CFA_offset_extended`.
  
  > 3. DW_CFA_offset
  >
  > The DW_CFA_offset instruction takes two operands: a register number
  > (encoded with the opcode) and an unsigned LEB128 constant representing
  > a factored offset. The required action is to change the rule for the
  > register indicated by the register number to be an offset(N) rule
  > where the value of N is factored offset * data_alignment_factor.

- **`OffsetExtendedSf`**

  > 5. DW_CFA_offset_extended_sf
  >
  > The DW_CFA_offset_extended_sf instruction takes two operands: an
  > unsigned LEB128 value representing a register number and a signed
  > LEB128 factored offset. This instruction is identical to
  > DW_CFA_offset_extended except that the second operand is signed and
  > factored. The resulting offset is factored_offset *
  > data_alignment_factor.

- **`ValOffset`**

  > 6. DW_CFA_val_offset
  >
  > The DW_CFA_val_offset instruction takes two unsigned LEB128 operands
  > representing a register number and a factored offset. The required
  > action is to change the rule for the register indicated by the
  > register number to be a val_offset(N) rule where the value of N is
  > factored_offset * data_alignment_factor.

- **`ValOffsetSf`**

  > 7. DW_CFA_val_offset_sf
  >
  > The DW_CFA_val_offset_sf instruction takes two operands: an unsigned
  > LEB128 value representing a register number and a signed LEB128
  > factored offset. This instruction is identical to DW_CFA_val_offset
  > except that the second operand is signed and factored. The resulting
  > offset is factored_offset * data_alignment_factor.

- **`Register`**

  > 8. DW_CFA_register
  >
  > The DW_CFA_register instruction takes two unsigned LEB128 operands
  > representing register numbers. The required action is to set the rule
  > for the first register to be register(R) where R is the second
  > register.

- **`Expression`**

  > 9. DW_CFA_expression
  >
  > The DW_CFA_expression instruction takes two operands: an unsigned
  > LEB128 value representing a register number, and a DW_FORM_block value
  > representing a DWARF expression. The required action is to change the
  > rule for the register indicated by the register number to be an
  > expression(E) rule where E is the DWARF expression. That is, the DWARF
  > expression computes the address. The value of the CFA is pushed on the
  > DWARF evaluation stack prior to execution of the DWARF expression.

- **`ValExpression`**

  > 10. DW_CFA_val_expression
  >
  > The DW_CFA_val_expression instruction takes two operands: an unsigned
  > LEB128 value representing a register number, and a DW_FORM_block value
  > representing a DWARF expression. The required action is to change the
  > rule for the register indicated by the register number to be a
  > val_expression(E) rule where E is the DWARF expression. That is, the
  > DWARF expression computes the value of the given register. The value
  > of the CFA is pushed on the DWARF evaluation stack prior to execution
  > of the DWARF expression.

- **`Restore`**

  The `Restore` instruction represents both `DW_CFA_restore` and
  `DW_CFA_restore_extended`.
  
  > 11. DW_CFA_restore
  >
  > The DW_CFA_restore instruction takes a single operand (encoded with
  > the opcode) that represents a register number. The required action is
  > to change the rule for the indicated register to the rule assigned it
  > by the initial_instructions in the CIE.

- **`RememberState`**

  > 1. DW_CFA_remember_state
  >
  > The DW_CFA_remember_state instruction takes no operands. The required
  > action is to push the set of rules for every register onto an implicit
  > stack.

- **`RestoreState`**

  > 2. DW_CFA_restore_state
  >
  > The DW_CFA_restore_state instruction takes no operands. The required
  > action is to pop the set of rules off the implicit stack and place
  > them in the current row.

- **`ArgsSize`**

  > DW_CFA_GNU_args_size
  >
  > GNU Extension
  >
  > The DW_CFA_GNU_args_size instruction takes an unsigned LEB128 operand
  > representing an argument size. This instruction specifies the total of
  > the size of the arguments which have been pushed onto the stack.

- **`NegateRaState`**

  > DW_CFA_AARCH64_negate_ra_state
  >
  > AArch64 Extension
  >
  > The DW_CFA_AARCH64_negate_ra_state operation negates bit 0 of the
  > RA_SIGN_STATE pseudo-register. It does not take any operands. The
  > DW_CFA_AARCH64_negate_ra_state must not be mixed with other DWARF Register
  > Rule Instructions on the RA_SIGN_STATE pseudo-register in one Common
  > Information Entry (CIE) and Frame Descriptor Entry (FDE) program sequence.

- **`Nop`**

  > 1. DW_CFA_nop
  >
  > The DW_CFA_nop instruction has no operands and no required actions. It
  > is used as padding to make a CIE or FDE an appropriate size.

#### Implementations

- <span id="callframeinstruction-parse"></span>`fn parse<R: Reader<Offset = T>>(input: &mut R, address_encoding: Option<DwEhPe>, parameters: &PointerEncodingParameters<'_, R>, vendor: Vendor) -> Result<CallFrameInstruction<T>>` — [`DwEhPe`](../index.md#dwehpe), [`PointerEncodingParameters`](cfi/index.md#pointerencodingparameters), [`Vendor`](../index.md#vendor), [`Result`](../index.md#result), [`CallFrameInstruction`](#callframeinstruction)

#### Trait Implementations

##### `impl<T: clone::Clone + ReaderOffset> Clone for CallFrameInstruction<T>`

- <span id="callframeinstruction-clone"></span>`fn clone(&self) -> CallFrameInstruction<T>` — [`CallFrameInstruction`](#callframeinstruction)

##### `impl<T: fmt::Debug + ReaderOffset> Debug for CallFrameInstruction<T>`

- <span id="callframeinstruction-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq + ReaderOffset> Eq for CallFrameInstruction<T>`

##### `impl<T: cmp::PartialEq + ReaderOffset> PartialEq for CallFrameInstruction<T>`

- <span id="callframeinstruction-partialeq-eq"></span>`fn eq(&self, other: &CallFrameInstruction<T>) -> bool` — [`CallFrameInstruction`](#callframeinstruction)

##### `impl<T: ReaderOffset> StructuralPartialEq for CallFrameInstruction<T>`

### `Pointer`

```rust
enum Pointer {
    Direct(u64),
    Indirect(u64),
}
```

A decoded pointer.

#### Variants

- **`Direct`**

  This value is the decoded pointer value.

- **`Indirect`**

  This value is *not* the pointer value, but points to the address of
  where the real pointer value lives. In other words, deref this pointer
  to get the real pointer value.
  
  Chase this pointer at your own risk: do you trust the DWARF data it came
  from?

#### Implementations

- <span id="pointer-new"></span>`fn new(encoding: constants::DwEhPe, address: u64) -> Pointer` — [`DwEhPe`](../index.md#dwehpe), [`Pointer`](#pointer)

- <span id="pointer-direct"></span>`fn direct(self) -> Result<u64>` — [`Result`](../index.md#result)

  Return the direct pointer value.

- <span id="pointer-pointer"></span>`fn pointer(self) -> u64`

  Return the pointer value, discarding indirectness information.

#### Trait Implementations

##### `impl Clone for Pointer`

- <span id="pointer-clone"></span>`fn clone(&self) -> Pointer` — [`Pointer`](#pointer)

##### `impl Copy for Pointer`

##### `impl Debug for Pointer`

- <span id="pointer-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Pointer`

- <span id="pointer-default"></span>`fn default() -> Self`

##### `impl Eq for Pointer`

##### `impl PartialEq for Pointer`

- <span id="pointer-partialeq-eq"></span>`fn eq(&self, other: &Pointer) -> bool` — [`Pointer`](#pointer)

##### `impl StructuralPartialEq for Pointer`

### `RangeIterInner<R: Reader>`

```rust
enum RangeIterInner<R: Reader> {
    Single(Option<crate::read::Range>),
    List(crate::read::RngListIter<R>),
}
```

#### Trait Implementations

##### `impl<R: fmt::Debug + Reader> Debug for RangeIterInner<R>`

- <span id="rangeiterinner-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="abbreviationscachestrategy-clone"></span>`fn clone(&self) -> AbbreviationsCacheStrategy` — [`AbbreviationsCacheStrategy`](#abbreviationscachestrategy)

##### `impl Copy for AbbreviationsCacheStrategy`

##### `impl Debug for AbbreviationsCacheStrategy`

- <span id="abbreviationscachestrategy-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for AbbreviationsCacheStrategy`

##### `impl PartialEq for AbbreviationsCacheStrategy`

- <span id="abbreviationscachestrategy-partialeq-eq"></span>`fn eq(&self, other: &AbbreviationsCacheStrategy) -> bool` — [`AbbreviationsCacheStrategy`](#abbreviationscachestrategy)

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

- <span id="attributes-new"></span>`fn new() -> Attributes` — [`Attributes`](abbrev/index.md#attributes)

  Returns a new empty list of attributes

- <span id="attributes-push"></span>`fn push(&mut self, attr: AttributeSpecification)` — [`AttributeSpecification`](#attributespecification)

  Pushes a new value onto this list of attributes.

#### Trait Implementations

##### `impl Clone for Attributes`

- <span id="attributes-clone"></span>`fn clone(&self) -> Attributes` — [`Attributes`](abbrev/index.md#attributes)

##### `impl Debug for Attributes`

- <span id="attributes-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deref for Attributes`

- <span id="attributes-deref-type-target"></span>`type Target = [AttributeSpecification]`

- <span id="attributes-deref"></span>`fn deref(&self) -> &[AttributeSpecification]` — [`AttributeSpecification`](#attributespecification)

##### `impl Eq for Attributes`

##### `impl FromIterator for Attributes`

- <span id="attributes-fromiterator-from-iter"></span>`fn from_iter<I>(iter: I) -> Attributes` — [`Attributes`](abbrev/index.md#attributes)

##### `impl PartialEq for Attributes`

- <span id="attributes-partialeq-eq"></span>`fn eq(&self, other: &Attributes) -> bool` — [`Attributes`](abbrev/index.md#attributes)

##### `impl Receiver for Attributes`

- <span id="attributes-receiver-type-target"></span>`type Target = T`

### `IndexSectionId`

```rust
enum IndexSectionId {
    DebugAbbrev,
    DebugInfo,
    DebugLine,
    DebugLoc,
    DebugLocLists,
    DebugMacinfo,
    DebugMacro,
    DebugRngLists,
    DebugStrOffsets,
    DebugTypes,
}
```

Section kinds which are permitted in a `.dwp` index.

#### Variants

- **`DebugAbbrev`**

  The `.debug_abbrev.dwo` section.

- **`DebugInfo`**

  The `.debug_info.dwo` section.

- **`DebugLine`**

  The `.debug_line.dwo` section.

- **`DebugLoc`**

  The `.debug_loc.dwo` section.

- **`DebugLocLists`**

  The `.debug_loclists.dwo` section.

- **`DebugMacinfo`**

  The `.debug_macinfo.dwo` section.

- **`DebugMacro`**

  The `.debug_macro.dwo` section.

- **`DebugRngLists`**

  The `.debug_rnglists.dwo` section.

- **`DebugStrOffsets`**

  The `.debug_str_offsets.dwo` section.

- **`DebugTypes`**

  The `.debug_types.dwo` section.

#### Implementations

- <span id="indexsectionid-section-id"></span>`fn section_id(self) -> SectionId` — [`SectionId`](../index.md#sectionid)

  Returns the corresponding `SectionId`.

- <span id="indexsectionid-dwo-name"></span>`fn dwo_name(self) -> &'static str`

  Returns the ELF section name for this kind, when found in a .dwo or .dwp file.

#### Trait Implementations

##### `impl Clone for IndexSectionId`

- <span id="indexsectionid-clone"></span>`fn clone(&self) -> IndexSectionId` — [`IndexSectionId`](#indexsectionid)

##### `impl Copy for IndexSectionId`

##### `impl Debug for IndexSectionId`

- <span id="indexsectionid-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for IndexSectionId`

##### `impl PartialEq for IndexSectionId`

- <span id="indexsectionid-partialeq-eq"></span>`fn eq(&self, other: &IndexSectionId) -> bool` — [`IndexSectionId`](#indexsectionid)

##### `impl StructuralPartialEq for IndexSectionId`

### `LineInstruction<R, Offset>`

```rust
enum LineInstruction<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    Special(u8),
    Copy,
    AdvancePc(u64),
    AdvanceLine(i64),
    SetFile(u64),
    SetColumn(u64),
    NegateStatement,
    SetBasicBlock,
    ConstAddPc,
    FixedAddPc(u16),
    SetPrologueEnd,
    SetEpilogueBegin,
    SetIsa(u64),
    UnknownStandard0(constants::DwLns),
    UnknownStandard1(constants::DwLns, u64),
    UnknownStandardN(constants::DwLns, R),
    EndSequence,
    SetAddress(u64),
    DefineFile(FileEntry<R, Offset>),
    SetDiscriminator(u64),
    UnknownExtended(constants::DwLne, R),
}
```

A parsed line number program instruction.

#### Variants

- **`Special`**

  > ### 6.2.5.1 Special Opcodes
  >
  > Each ubyte special opcode has the following effect on the state machine:
  >
  >   1. Add a signed integer to the line register.
  >
  >   2. Modify the operation pointer by incrementing the address and
  >      op_index registers as described below.
  >
  >   3. Append a row to the matrix using the current values of the state
  >      machine registers.
  >
  >   4. Set the basic_block register to “false.”
  >
  >   5. Set the prologue_end register to “false.”
  >
  >   6. Set the epilogue_begin register to “false.”
  >
  >   7. Set the discriminator register to 0.
  >
  > All of the special opcodes do those same seven things; they differ from
  > one another only in what values they add to the line, address and
  > op_index registers.

- **`Copy`**

  "[`LineInstruction::Copy`](../index.md) appends a row to the matrix using the current
  values of the state machine registers. Then it sets the discriminator
  register to 0, and sets the basic_block, prologue_end and epilogue_begin
  registers to “false.”"

- **`AdvancePc`**

  "The DW_LNS_advance_pc opcode takes a single unsigned LEB128 operand as
  the operation advance and modifies the address and op_index registers
  [the same as `LineInstruction::Special`]"

- **`AdvanceLine`**

  "The DW_LNS_advance_line opcode takes a single signed LEB128 operand and
  adds that value to the line register of the state machine."

- **`SetFile`**

  "The DW_LNS_set_file opcode takes a single unsigned LEB128 operand and
  stores it in the file register of the state machine."

- **`SetColumn`**

  "The DW_LNS_set_column opcode takes a single unsigned LEB128 operand and
  stores it in the column register of the state machine."

- **`NegateStatement`**

  "The DW_LNS_negate_stmt opcode takes no operands. It sets the is_stmt
  register of the state machine to the logical negation of its current
  value."

- **`SetBasicBlock`**

  "The DW_LNS_set_basic_block opcode takes no operands. It sets the
  basic_block register of the state machine to “true.”"

- **`ConstAddPc`**

  > The DW_LNS_const_add_pc opcode takes no operands. It advances the
  > address and op_index registers by the increments corresponding to
  > special opcode 255.
  >
  > When the line number program needs to advance the address by a small
  > amount, it can use a single special opcode, which occupies a single
  > byte. When it needs to advance the address by up to twice the range of
  > the last special opcode, it can use DW_LNS_const_add_pc followed by a
  > special opcode, for a total of two bytes. Only if it needs to advance
  > the address by more than twice that range will it need to use both
  > DW_LNS_advance_pc and a special opcode, requiring three or more bytes.

- **`FixedAddPc`**

  > The DW_LNS_fixed_advance_pc opcode takes a single uhalf (unencoded)
  > operand and adds it to the address register of the state machine and
  > sets the op_index register to 0. This is the only standard opcode whose
  > operand is not a variable length number. It also does not multiply the
  > operand by the minimum_instruction_length field of the header.

- **`SetPrologueEnd`**

  "[`LineInstruction::SetPrologueEnd`](../index.md) sets the prologue_end register to “true”."

- **`SetEpilogueBegin`**

  "[`LineInstruction::SetEpilogueBegin`](../index.md) sets the epilogue_begin register to
  “true”."

- **`SetIsa`**

  "The DW_LNS_set_isa opcode takes a single unsigned LEB128 operand and
  stores that value in the isa register of the state machine."

- **`UnknownStandard0`**

  An unknown standard opcode with zero operands.

- **`UnknownStandard1`**

  An unknown standard opcode with one operand.

- **`UnknownStandardN`**

  An unknown standard opcode with multiple operands.

- **`EndSequence`**

  > [`LineInstruction::EndSequence`](../index.md) sets the end_sequence register of the state
  > machine to “true” and appends a row to the matrix using the current
  > values of the state-machine registers. Then it resets the registers to
  > the initial values specified above (see Section 6.2.2). Every line
  > number program sequence must end with a DW_LNE_end_sequence instruction
  > which creates a row whose address is that of the byte after the last
  > target machine instruction of the sequence.

- **`SetAddress`**

  > The DW_LNE_set_address opcode takes a single relocatable address as an
  > operand. The size of the operand is the size of an address on the target
  > machine. It sets the address register to the value given by the
  > relocatable address and sets the op_index register to 0.
  >
  > All of the other line number program opcodes that affect the address
  > register add a delta to it. This instruction stores a relocatable value
  > into it instead.

- **`DefineFile`**

  Defines a new source file in the line number program and appends it to
  the line number program header's list of source files.

- **`SetDiscriminator`**

  "The DW_LNE_set_discriminator opcode takes a single parameter, an
  unsigned LEB128 integer. It sets the discriminator register to the new
  value."

- **`UnknownExtended`**

  An unknown extended opcode and the slice of its unparsed operands.

#### Implementations

- <span id="lineinstruction-parse"></span>`fn parse<'header>(header: &'header LineProgramHeader<R>, input: &mut R) -> Result<LineInstruction<R>>` — [`LineProgramHeader`](#lineprogramheader), [`Result`](../index.md#result), [`LineInstruction`](#lineinstruction)

#### Trait Implementations

##### `impl<R, Offset> Clone for LineInstruction<R, Offset>`

- <span id="lineinstruction-clone"></span>`fn clone(&self) -> LineInstruction<R, Offset>` — [`LineInstruction`](#lineinstruction)

##### `impl<R, Offset> Copy for LineInstruction<R, Offset>`

##### `impl<R, Offset> Debug for LineInstruction<R, Offset>`

- <span id="lineinstruction-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Offset> Eq for LineInstruction<R, Offset>`

##### `impl<R, Offset> PartialEq for LineInstruction<R, Offset>`

- <span id="lineinstruction-partialeq-eq"></span>`fn eq(&self, other: &LineInstruction<R, Offset>) -> bool` — [`LineInstruction`](#lineinstruction)

##### `impl<R, Offset> StructuralPartialEq for LineInstruction<R, Offset>`

### `ColumnType`

```rust
enum ColumnType {
    LeftEdge,
    Column(core::num::NonZeroU64),
}
```

The type of column that a row is referring to.

#### Variants

- **`LeftEdge`**

  The `LeftEdge` means that the statement begins at the start of the new
  line.

- **`Column`**

  A column number, whose range begins at 1.

#### Trait Implementations

##### `impl Clone for ColumnType`

- <span id="columntype-clone"></span>`fn clone(&self) -> ColumnType` — [`ColumnType`](#columntype)

##### `impl Copy for ColumnType`

##### `impl Debug for ColumnType`

- <span id="columntype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ColumnType`

##### `impl Ord for ColumnType`

- <span id="columntype-ord-cmp"></span>`fn cmp(&self, other: &ColumnType) -> cmp::Ordering` — [`ColumnType`](#columntype)

##### `impl PartialEq for ColumnType`

- <span id="columntype-partialeq-eq"></span>`fn eq(&self, other: &ColumnType) -> bool` — [`ColumnType`](#columntype)

##### `impl PartialOrd for ColumnType`

- <span id="columntype-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &ColumnType) -> option::Option<cmp::Ordering>` — [`ColumnType`](#columntype)

##### `impl StructuralPartialEq for ColumnType`

### `LocListsFormat`

```rust
enum LocListsFormat {
    Bare,
    Lle,
}
```

#### Variants

- **`Bare`**

  The bare location list format used before DWARF 5.

- **`Lle`**

  The DW_LLE encoded range list format used in DWARF 5 and the non-standard GNU
  split dwarf extension.

#### Trait Implementations

##### `impl Clone for LocListsFormat`

- <span id="loclistsformat-clone"></span>`fn clone(&self) -> LocListsFormat` — [`LocListsFormat`](loclists/index.md#loclistsformat)

##### `impl Copy for LocListsFormat`

##### `impl Debug for LocListsFormat`

- <span id="loclistsformat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for LocListsFormat`

##### `impl PartialEq for LocListsFormat`

- <span id="loclistsformat-partialeq-eq"></span>`fn eq(&self, other: &LocListsFormat) -> bool` — [`LocListsFormat`](loclists/index.md#loclistsformat)

##### `impl StructuralPartialEq for LocListsFormat`

### `RawLocListEntry<R: Reader>`

```rust
enum RawLocListEntry<R: Reader> {
    AddressOrOffsetPair {
        begin: u64,
        end: u64,
        data: crate::read::Expression<R>,
    },
    BaseAddress {
        addr: u64,
    },
    BaseAddressx {
        addr: crate::common::DebugAddrIndex<<R as >::Offset>,
    },
    StartxEndx {
        begin: crate::common::DebugAddrIndex<<R as >::Offset>,
        end: crate::common::DebugAddrIndex<<R as >::Offset>,
        data: crate::read::Expression<R>,
    },
    StartxLength {
        begin: crate::common::DebugAddrIndex<<R as >::Offset>,
        length: u64,
        data: crate::read::Expression<R>,
    },
    OffsetPair {
        begin: u64,
        end: u64,
        data: crate::read::Expression<R>,
    },
    DefaultLocation {
        data: crate::read::Expression<R>,
    },
    StartEnd {
        begin: u64,
        end: u64,
        data: crate::read::Expression<R>,
    },
    StartLength {
        begin: u64,
        length: u64,
        data: crate::read::Expression<R>,
    },
}
```

A raw entry in .debug_loclists.

#### Variants

- **`AddressOrOffsetPair`**

  A location from DWARF version <= 4.

- **`BaseAddress`**

  DW_LLE_base_address

- **`BaseAddressx`**

  DW_LLE_base_addressx

- **`StartxEndx`**

  DW_LLE_startx_endx

- **`StartxLength`**

  DW_LLE_startx_length

- **`OffsetPair`**

  DW_LLE_offset_pair

- **`DefaultLocation`**

  DW_LLE_default_location

- **`StartEnd`**

  DW_LLE_start_end

- **`StartLength`**

  DW_LLE_start_length

#### Implementations

- <span id="rawloclistentry-parse"></span>`fn parse(input: &mut R, encoding: Encoding, format: LocListsFormat) -> Result<Option<Self>>` — [`Encoding`](../index.md#encoding), [`LocListsFormat`](loclists/index.md#loclistsformat), [`Result`](../index.md#result)

  Parse a location list entry from `.debug_loclists`

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for RawLocListEntry<R>`

- <span id="rawloclistentry-clone"></span>`fn clone(&self) -> RawLocListEntry<R>` — [`RawLocListEntry`](#rawloclistentry)

##### `impl<R: fmt::Debug + Reader> Debug for RawLocListEntry<R>`

- <span id="rawloclistentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `MacroString<R, Offset>`

```rust
enum MacroString<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    Direct(R),
    StringPointer(crate::DebugStrOffset<Offset>),
    IndirectStringPointer(crate::DebugStrOffsetsIndex<Offset>),
    Supplementary(crate::DebugStrOffset<Offset>),
}
```

A string in a macro entry.

#### Variants

- **`Direct`**

  The string is directly embedded in the macro entry

- **`StringPointer`**

  The macro refers to a string in the `.debug_str` section using a `DebugStrOffset`.

- **`IndirectStringPointer`**

  The macro contains an index into an array in the `.debug_str_offsets`
  section, which refers to a string in the `.debug_str` section.

- **`Supplementary`**

  The macro refers to a string in the `.debug_str` section in the supplementary object file

#### Implementations

- <span id="macrostring-string"></span>`fn string(&self, unit: UnitRef<'_, R>) -> Result<R>` — [`UnitRef`](#unitref), [`Result`](../index.md#result)

  Get the string slice from the macro entry.

#### Trait Implementations

##### `impl<R, Offset> Clone for MacroString<R, Offset>`

- <span id="macrostring-clone"></span>`fn clone(&self) -> MacroString<R, Offset>` — [`MacroString`](#macrostring)

##### `impl<R, Offset> Debug for MacroString<R, Offset>`

- <span id="macrostring-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Offset> Eq for MacroString<R, Offset>`

##### `impl<R, Offset> PartialEq for MacroString<R, Offset>`

- <span id="macrostring-partialeq-eq"></span>`fn eq(&self, other: &MacroString<R, Offset>) -> bool` — [`MacroString`](#macrostring)

##### `impl<R, Offset> StructuralPartialEq for MacroString<R, Offset>`

### `MacroEntry<R, Offset>`

```rust
enum MacroEntry<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    Define {
        line: u64,
        text: MacroString<R>,
    },
    Undef {
        line: u64,
        name: MacroString<R>,
    },
    StartFile {
        line: u64,
        file: u64,
    },
    EndFile,
    Import {
        offset: crate::DebugMacroOffset<Offset>,
    },
    ImportSup {
        offset: crate::DebugMacroOffset<Offset>,
    },
    VendorExt {
        numeric: u64,
        string: R,
    },
}
```

an Entry in the `.debug_macro` section.

#### Variants

- **`Define`**

  A macro definition.

- **`Undef`**

  A macro undefinition.

- **`StartFile`**

  The start of a file.

- **`EndFile`**

  The end of the current included file.

- **`Import`**

  import a macro unit

- **`ImportSup`**

  import a macro unit from the supplementary object file

- **`VendorExt`**

  A vendor-specific extension.

#### Trait Implementations

##### `impl<R, Offset> Clone for MacroEntry<R, Offset>`

- <span id="macroentry-clone"></span>`fn clone(&self) -> MacroEntry<R, Offset>` — [`MacroEntry`](#macroentry)

##### `impl<R, Offset> Debug for MacroEntry<R, Offset>`

- <span id="macroentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Offset> Eq for MacroEntry<R, Offset>`

##### `impl<R, Offset> PartialEq for MacroEntry<R, Offset>`

- <span id="macroentry-partialeq-eq"></span>`fn eq(&self, other: &MacroEntry<R, Offset>) -> bool` — [`MacroEntry`](#macroentry)

##### `impl<R, Offset> StructuralPartialEq for MacroEntry<R, Offset>`

### `NameTypeUnit<T>`

```rust
enum NameTypeUnit<T> {
    Local(crate::common::DebugInfoOffset<T>),
    Foreign(crate::common::DebugTypeSignature),
}
```

A reference to a type unit.

This is the result of looking up a type unit index obtained from a `DW_IDX_type_unit`
attribute.

#### Variants

- **`Local`**

  The offset of a local type unit in the `.debug_info` section.

- **`Foreign`**

  The type signature of a foreign type unit.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for NameTypeUnit<T>`

- <span id="nametypeunit-clone"></span>`fn clone(&self) -> NameTypeUnit<T>` — [`NameTypeUnit`](#nametypeunit)

##### `impl<T: marker::Copy> Copy for NameTypeUnit<T>`

##### `impl<T: fmt::Debug> Debug for NameTypeUnit<T>`

- <span id="nametypeunit-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for NameTypeUnit<T>`

##### `impl<T: cmp::PartialEq> PartialEq for NameTypeUnit<T>`

- <span id="nametypeunit-partialeq-eq"></span>`fn eq(&self, other: &NameTypeUnit<T>) -> bool` — [`NameTypeUnit`](#nametypeunit)

##### `impl<T> StructuralPartialEq for NameTypeUnit<T>`

### `NameAttributeValue<R: Reader>`

```rust
enum NameAttributeValue<R: Reader> {
    Unsigned(u64),
    Offset(<R as >::Offset),
    Flag(bool),
}
```

A parsed attribute value for a [`NameEntry`](#nameentry).

#### Variants

- **`Unsigned`**

  An unsigned integer.
  
  This can be from the following forms:
  `DW_FORM_data1`, `DW_FORM_data2`, `DW_FORM_data4`, `DW_FORM_data8`, `DW_FORM_udata`

- **`Offset`**

  An offset within a DWARF section or part thereof.
  
  This can be from the following forms:
  `DW_FORM_ref1`, `DW_FORM_ref2`, `DW_FORM_ref4`, `DW_FORM_ref8`, `DW_FORM_ref_udata`

- **`Flag`**

  A boolean flag.
  
  This can be from the following forms:
  `DW_FORM_flag`, `DW_FORM_flag_present`

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for NameAttributeValue<R>`

- <span id="nameattributevalue-clone"></span>`fn clone(&self) -> NameAttributeValue<R>` — [`NameAttributeValue`](#nameattributevalue)

##### `impl<R: marker::Copy + Reader> Copy for NameAttributeValue<R>`

##### `impl<R: fmt::Debug + Reader> Debug for NameAttributeValue<R>`

- <span id="nameattributevalue-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: cmp::Eq + Reader> Eq for NameAttributeValue<R>`

##### `impl<R: cmp::PartialEq + Reader> PartialEq for NameAttributeValue<R>`

- <span id="nameattributevalue-partialeq-eq"></span>`fn eq(&self, other: &NameAttributeValue<R>) -> bool` — [`NameAttributeValue`](#nameattributevalue)

##### `impl<R: Reader> StructuralPartialEq for NameAttributeValue<R>`

### `DieReference<T>`

```rust
enum DieReference<T> {
    UnitRef(crate::read::UnitOffset<T>),
    DebugInfoRef(crate::common::DebugInfoOffset<T>),
}
```

A reference to a DIE, either relative to the current CU or
relative to the section.

#### Variants

- **`UnitRef`**

  A CU-relative reference.

- **`DebugInfoRef`**

  A section-relative reference.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for DieReference<T>`

- <span id="diereference-clone"></span>`fn clone(&self) -> DieReference<T>` — [`DieReference`](#diereference)

##### `impl<T: marker::Copy> Copy for DieReference<T>`

##### `impl<T: fmt::Debug> Debug for DieReference<T>`

- <span id="diereference-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for DieReference<T>`

##### `impl<T: cmp::PartialEq> PartialEq for DieReference<T>`

- <span id="diereference-partialeq-eq"></span>`fn eq(&self, other: &DieReference<T>) -> bool` — [`DieReference`](#diereference)

##### `impl<T> StructuralPartialEq for DieReference<T>`

### `Operation<R, Offset>`

```rust
enum Operation<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    Deref {
        base_type: crate::read::UnitOffset<Offset>,
        size: u8,
        space: bool,
    },
    Drop,
    Pick {
        index: u8,
    },
    Swap,
    Rot,
    Abs,
    And,
    Div,
    Minus,
    Mod,
    Mul,
    Neg,
    Not,
    Or,
    Plus,
    PlusConstant {
        value: u64,
    },
    Shl,
    Shr,
    Shra,
    Xor,
    Bra {
        target: i16,
    },
    Eq,
    Ge,
    Gt,
    Le,
    Lt,
    Ne,
    Skip {
        target: i16,
    },
    UnsignedConstant {
        value: u64,
    },
    SignedConstant {
        value: i64,
    },
    Register {
        register: crate::common::Register,
    },
    RegisterOffset {
        register: crate::common::Register,
        offset: i64,
        base_type: crate::read::UnitOffset<Offset>,
    },
    FrameOffset {
        offset: i64,
    },
    Nop,
    PushObjectAddress,
    Call {
        offset: DieReference<Offset>,
    },
    VariableValue {
        offset: crate::common::DebugInfoOffset<Offset>,
    },
    TLS,
    CallFrameCFA,
    Piece {
        size_in_bits: u64,
        bit_offset: Option<u64>,
    },
    ImplicitValue {
        data: R,
    },
    StackValue,
    ImplicitPointer {
        value: crate::common::DebugInfoOffset<Offset>,
        byte_offset: i64,
    },
    EntryValue {
        expression: R,
    },
    ParameterRef {
        offset: crate::read::UnitOffset<Offset>,
    },
    Address {
        address: u64,
    },
    AddressIndex {
        index: crate::common::DebugAddrIndex<Offset>,
    },
    ConstantIndex {
        index: crate::common::DebugAddrIndex<Offset>,
    },
    TypedLiteral {
        base_type: crate::read::UnitOffset<Offset>,
        value: R,
    },
    Convert {
        base_type: crate::read::UnitOffset<Offset>,
    },
    Reinterpret {
        base_type: crate::read::UnitOffset<Offset>,
    },
    Uninitialized,
    WasmLocal {
        index: u32,
    },
    WasmGlobal {
        index: u32,
    },
    WasmStack {
        index: u32,
    },
}
```

A single decoded DWARF expression operation.

DWARF expression evaluation is done in two parts: first the raw
bytes of the next part of the expression are decoded; and then the
decoded operation is evaluated.  This approach lets other
consumers inspect the DWARF expression without reimplementing the
decoding operation.

Multiple DWARF opcodes may decode into a single `Operation`.  For
example, both `DW_OP_deref` and `DW_OP_xderef` are represented
using `Operation::Deref`.

#### Variants

- **`Deref`**

  Dereference the topmost value of the stack.

- **`Drop`**

  Drop an item from the stack.

- **`Pick`**

  Pick an item from the stack and push it on top of the stack.
  This operation handles `DW_OP_pick`, `DW_OP_dup`, and
  `DW_OP_over`.

- **`Swap`**

  Swap the top two stack items.

- **`Rot`**

  Rotate the top three stack items.

- **`Abs`**

  Take the absolute value of the top of the stack.

- **`And`**

  Bitwise `and` of the top two values on the stack.

- **`Div`**

  Divide the top two values on the stack.

- **`Minus`**

  Subtract the top two values on the stack.

- **`Mod`**

  Modulus of the top two values on the stack.

- **`Mul`**

  Multiply the top two values on the stack.

- **`Neg`**

  Negate the top of the stack.

- **`Not`**

  Bitwise `not` of the top of the stack.

- **`Or`**

  Bitwise `or` of the top two values on the stack.

- **`Plus`**

  Add the top two values on the stack.

- **`PlusConstant`**

  Add a constant to the topmost value on the stack.

- **`Shl`**

  Logical left shift of the 2nd value on the stack by the number
  of bits given by the topmost value on the stack.

- **`Shr`**

  Right shift of the 2nd value on the stack by the number of
  bits given by the topmost value on the stack.

- **`Shra`**

  Arithmetic left shift of the 2nd value on the stack by the
  number of bits given by the topmost value on the stack.

- **`Xor`**

  Bitwise `xor` of the top two values on the stack.

- **`Bra`**

  Branch to the target location if the top of stack is nonzero.

- **`Eq`**

  Compare the top two stack values for equality.

- **`Ge`**

  Compare the top two stack values using `>=`.

- **`Gt`**

  Compare the top two stack values using `>`.

- **`Le`**

  Compare the top two stack values using `<=`.

- **`Lt`**

  Compare the top two stack values using `<`.

- **`Ne`**

  Compare the top two stack values using `!=`.

- **`Skip`**

  Unconditional branch to the target location.

- **`UnsignedConstant`**

  Push an unsigned constant value on the stack.  This handles multiple
  DWARF opcodes.

- **`SignedConstant`**

  Push a signed constant value on the stack.  This handles multiple
  DWARF opcodes.

- **`Register`**

  Indicate that this piece's location is in the given register.
  
  Completes the piece or expression.

- **`RegisterOffset`**

  Find the value of the given register, add the offset, and then
  push the resulting sum on the stack.

- **`FrameOffset`**

  Compute the frame base (using `DW_AT_frame_base`), add the
  given offset, and then push the resulting sum on the stack.

- **`Nop`**

  No operation.

- **`PushObjectAddress`**

  Push the object address on the stack.

- **`Call`**

  Evaluate a DWARF expression as a subroutine.  The expression
  comes from the `DW_AT_location` attribute of the indicated
  DIE.

- **`VariableValue`**

  Compute the value of a variable and push it on the stack.
  
  Represents `DW_OP_GNU_variable_value`.

- **`TLS`**

  Compute the address of a thread-local variable and push it on
  the stack.

- **`CallFrameCFA`**

  Compute the call frame CFA and push it on the stack.

- **`Piece`**

  Terminate a piece.

- **`ImplicitValue`**

  The object has no location, but has a known constant value.
  
  Represents `DW_OP_implicit_value`.
  Completes the piece or expression.

- **`StackValue`**

  The object has no location, but its value is at the top of the stack.
  
  Represents `DW_OP_stack_value`.
  Completes the piece or expression.

- **`ImplicitPointer`**

  The object is a pointer to a value which has no actual location,
  such as an implicit value or a stack value.
  
  Represents `DW_OP_implicit_pointer`.
  Completes the piece or expression.

- **`EntryValue`**

  Evaluate an expression at the entry to the current subprogram, and push it on the stack.
  
  Represents `DW_OP_entry_value`.

- **`ParameterRef`**

  This represents a parameter that was optimized out.
  
  The offset points to the definition of the parameter, and is
  matched to the `DW_TAG_GNU_call_site_parameter` in the caller that also
  points to the same definition of the parameter.
  
  Represents `DW_OP_GNU_parameter_ref`.

- **`Address`**

  Relocate the address if needed, and push it on the stack.
  
  Represents `DW_OP_addr`.

- **`AddressIndex`**

  Read the address at the given index in `.debug_addr, relocate the address if needed,
  and push it on the stack.
  
  Represents `DW_OP_addrx`.

- **`ConstantIndex`**

  Read the address at the given index in `.debug_addr, and push it on the stack.
  Do not relocate the address.
  
  Represents `DW_OP_constx`.

- **`TypedLiteral`**

  Interpret the value bytes as a constant of a given type, and push it on the stack.
  
  Represents `DW_OP_const_type`.

- **`Convert`**

  Pop the top stack entry, convert it to a different type, and push it on the stack.
  
  Represents `DW_OP_convert`.

- **`Reinterpret`**

  Pop the top stack entry, reinterpret the bits in its value as a different type,
  and push it on the stack.
  
  Represents `DW_OP_reinterpret`.

- **`Uninitialized`**

  Indicates that the value in the computed location is uninitialized.
  
  Represents `DW_OP_GNU_uninit`.

- **`WasmLocal`**

  The index of a local in the currently executing function.
  
  Represents `DW_OP_WASM_location 0x00`.
  Completes the piece or expression.

- **`WasmGlobal`**

  The index of a global.
  
  Represents `DW_OP_WASM_location 0x01` or `DW_OP_WASM_location 0x03`.
  Completes the piece or expression.

- **`WasmStack`**

  The index of an item on the operand stack.
  
  Represents `DW_OP_WASM_location 0x02`.
  Completes the piece or expression.

#### Implementations

- <span id="operation-parse"></span>`fn parse(bytes: &mut R, encoding: Encoding) -> Result<Operation<R, Offset>>` — [`Encoding`](../index.md#encoding), [`Result`](../index.md#result), [`Operation`](#operation)

  Parse a single DWARF expression operation.

  

  This is useful when examining a DWARF expression for reasons other

  than direct evaluation.

  

  `bytes` points to a the operation to decode.  It should point into

  the same array as `bytecode`, which should be the entire

  expression.

#### Trait Implementations

##### `impl<R, Offset> Clone for Operation<R, Offset>`

- <span id="operation-clone"></span>`fn clone(&self) -> Operation<R, Offset>` — [`Operation`](#operation)

##### `impl<R, Offset> Copy for Operation<R, Offset>`

##### `impl<R, Offset> Debug for Operation<R, Offset>`

- <span id="operation-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Offset> Eq for Operation<R, Offset>`

##### `impl<R, Offset> PartialEq for Operation<R, Offset>`

- <span id="operation-partialeq-eq"></span>`fn eq(&self, other: &Operation<R, Offset>) -> bool` — [`Operation`](#operation)

##### `impl<R, Offset> StructuralPartialEq for Operation<R, Offset>`

### `OperationEvaluationResult<R: Reader>`

```rust
enum OperationEvaluationResult<R: Reader> {
    Piece,
    Incomplete,
    Complete {
        location: Location<R>,
    },
    Waiting(EvaluationWaiting<R>, EvaluationResult<R>),
}
```

#### Trait Implementations

##### `impl<R: fmt::Debug + Reader> Debug for OperationEvaluationResult<R>`

- <span id="operationevaluationresult-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Location<R, Offset>`

```rust
enum Location<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    Empty,
    Register {
        register: crate::common::Register,
    },
    Address {
        address: u64,
    },
    Value {
        value: crate::read::Value,
    },
    Bytes {
        value: R,
    },
    ImplicitPointer {
        value: crate::common::DebugInfoOffset<Offset>,
        byte_offset: i64,
    },
}
```

A single location of a piece of the result of a DWARF expression.

#### Variants

- **`Empty`**

  The piece is empty.  Ordinarily this means the piece has been
  optimized away.

- **`Register`**

  The piece is found in a register.

- **`Address`**

  The piece is found in memory.

- **`Value`**

  The piece has no location but its value is known.

- **`Bytes`**

  The piece is represented by some constant bytes.

- **`ImplicitPointer`**

  The piece is a pointer to a value which has no actual location.

#### Implementations

- <span id="location-is-empty"></span>`fn is_empty(&self) -> bool`

  Return true if the piece is empty.

#### Trait Implementations

##### `impl<R, Offset> Clone for Location<R, Offset>`

- <span id="location-clone"></span>`fn clone(&self) -> Location<R, Offset>` — [`Location`](#location)

##### `impl<R, Offset> Copy for Location<R, Offset>`

##### `impl<R, Offset> Debug for Location<R, Offset>`

- <span id="location-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Offset> PartialEq for Location<R, Offset>`

- <span id="location-partialeq-eq"></span>`fn eq(&self, other: &Location<R, Offset>) -> bool` — [`Location`](#location)

##### `impl<R, Offset> StructuralPartialEq for Location<R, Offset>`

### `EvaluationState<R: Reader>`

```rust
enum EvaluationState<R: Reader> {
    Start(Option<u64>),
    Ready,
    Error(crate::read::Error),
    Complete,
    Waiting(EvaluationWaiting<R>),
}
```

#### Trait Implementations

##### `impl<R: fmt::Debug + Reader> Debug for EvaluationState<R>`

- <span id="evaluationstate-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `EvaluationWaiting<R: Reader>`

```rust
enum EvaluationWaiting<R: Reader> {
    Memory,
    Register {
        offset: i64,
    },
    FrameBase {
        offset: i64,
    },
    Tls,
    Cfa,
    AtLocation,
    EntryValue,
    ParameterRef,
    RelocatedAddress,
    IndexedAddress,
    TypedLiteral {
        value: R,
    },
    Convert,
    Reinterpret,
}
```

#### Trait Implementations

##### `impl<R: fmt::Debug + Reader> Debug for EvaluationWaiting<R>`

- <span id="evaluationwaiting-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `EvaluationResult<R: Reader>`

```rust
enum EvaluationResult<R: Reader> {
    Complete,
    RequiresMemory {
        address: u64,
        size: u8,
        space: Option<u64>,
        base_type: crate::read::UnitOffset<<R as >::Offset>,
    },
    RequiresRegister {
        register: crate::common::Register,
        base_type: crate::read::UnitOffset<<R as >::Offset>,
    },
    RequiresFrameBase,
    RequiresTls(u64),
    RequiresCallFrameCfa,
    RequiresAtLocation(DieReference<<R as >::Offset>),
    RequiresEntryValue(Expression<R>),
    RequiresParameterRef(crate::read::UnitOffset<<R as >::Offset>),
    RequiresRelocatedAddress(u64),
    RequiresIndexedAddress {
        index: crate::common::DebugAddrIndex<<R as >::Offset>,
        relocate: bool,
    },
    RequiresBaseType(crate::read::UnitOffset<<R as >::Offset>),
}
```

The state of an `Evaluation` after evaluating a DWARF expression.
The evaluation is either `Complete`, or it requires more data
to continue, as described by the variant.

#### Variants

- **`Complete`**

  The `Evaluation` is complete, and `Evaluation::result()` can be called.

- **`RequiresMemory`**

  The `Evaluation` needs a value from memory to proceed further.  Once the
  caller determines what value to provide it should resume the `Evaluation`
  by calling `Evaluation::resume_with_memory`.

- **`RequiresRegister`**

  The `Evaluation` needs a value from a register to proceed further.  Once
  the caller determines what value to provide it should resume the
  `Evaluation` by calling `Evaluation::resume_with_register`.

- **`RequiresFrameBase`**

  The `Evaluation` needs the frame base address to proceed further.  Once
  the caller determines what value to provide it should resume the
  `Evaluation` by calling `Evaluation::resume_with_frame_base`.  The frame
  base address is the address produced by the location description in the
  `DW_AT_frame_base` attribute of the current function.

- **`RequiresTls`**

  The `Evaluation` needs a value from TLS to proceed further.  Once the
  caller determines what value to provide it should resume the
  `Evaluation` by calling `Evaluation::resume_with_tls`.

- **`RequiresCallFrameCfa`**

  The `Evaluation` needs the CFA to proceed further.  Once the caller
  determines what value to provide it should resume the `Evaluation` by
  calling `Evaluation::resume_with_call_frame_cfa`.

- **`RequiresAtLocation`**

  The `Evaluation` needs the DWARF expression at the given location to
  proceed further.  Once the caller determines what value to provide it
  should resume the `Evaluation` by calling
  `Evaluation::resume_with_at_location`.

- **`RequiresEntryValue`**

  The `Evaluation` needs the value produced by evaluating a DWARF
  expression at the entry point of the current subprogram.  Once the
  caller determines what value to provide it should resume the
  `Evaluation` by calling `Evaluation::resume_with_entry_value`.

- **`RequiresParameterRef`**

  The `Evaluation` needs the value of the parameter at the given location
  in the current function's caller.  Once the caller determines what value
  to provide it should resume the `Evaluation` by calling
  `Evaluation::resume_with_parameter_ref`.

- **`RequiresRelocatedAddress`**

  The `Evaluation` needs an address to be relocated to proceed further.
  Once the caller determines what value to provide it should resume the
  `Evaluation` by calling `Evaluation::resume_with_relocated_address`.

- **`RequiresIndexedAddress`**

  The `Evaluation` needs an address from the `.debug_addr` section.
  This address may also need to be relocated.
  Once the caller determines what value to provide it should resume the
  `Evaluation` by calling `Evaluation::resume_with_indexed_address`.

- **`RequiresBaseType`**

  The `Evaluation` needs the `ValueType` for the base type DIE at
  the give unit offset.  Once the caller determines what value to provide it
  should resume the `Evaluation` by calling
  `Evaluation::resume_with_base_type`.

#### Trait Implementations

##### `impl<R: fmt::Debug + Reader> Debug for EvaluationResult<R>`

- <span id="evaluationresult-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: cmp::PartialEq + Reader> PartialEq for EvaluationResult<R>`

- <span id="evaluationresult-partialeq-eq"></span>`fn eq(&self, other: &EvaluationResult<R>) -> bool` — [`EvaluationResult`](#evaluationresult)

##### `impl<R: Reader> StructuralPartialEq for EvaluationResult<R>`

### `RangeListsFormat`

```rust
enum RangeListsFormat {
    Bare,
    Rle,
}
```

#### Variants

- **`Bare`**

  The bare range list format used before DWARF 5.

- **`Rle`**

  The DW_RLE encoded range list format used in DWARF 5.

#### Trait Implementations

##### `impl Clone for RangeListsFormat`

- <span id="rangelistsformat-clone"></span>`fn clone(&self) -> RangeListsFormat` — [`RangeListsFormat`](rnglists/index.md#rangelistsformat)

##### `impl Copy for RangeListsFormat`

##### `impl Debug for RangeListsFormat`

- <span id="rangelistsformat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RangeListsFormat`

##### `impl PartialEq for RangeListsFormat`

- <span id="rangelistsformat-partialeq-eq"></span>`fn eq(&self, other: &RangeListsFormat) -> bool` — [`RangeListsFormat`](rnglists/index.md#rangelistsformat)

##### `impl StructuralPartialEq for RangeListsFormat`

### `RawRngListEntry<T>`

```rust
enum RawRngListEntry<T> {
    AddressOrOffsetPair {
        begin: u64,
        end: u64,
    },
    BaseAddress {
        addr: u64,
    },
    BaseAddressx {
        addr: crate::common::DebugAddrIndex<T>,
    },
    StartxEndx {
        begin: crate::common::DebugAddrIndex<T>,
        end: crate::common::DebugAddrIndex<T>,
    },
    StartxLength {
        begin: crate::common::DebugAddrIndex<T>,
        length: u64,
    },
    OffsetPair {
        begin: u64,
        end: u64,
    },
    StartEnd {
        begin: u64,
        end: u64,
    },
    StartLength {
        begin: u64,
        length: u64,
    },
}
```

A raw entry in .debug_rnglists

#### Variants

- **`AddressOrOffsetPair`**

  A range from DWARF version <= 4.

- **`BaseAddress`**

  DW_RLE_base_address

- **`BaseAddressx`**

  DW_RLE_base_addressx

- **`StartxEndx`**

  DW_RLE_startx_endx

- **`StartxLength`**

  DW_RLE_startx_length

- **`OffsetPair`**

  DW_RLE_offset_pair

- **`StartEnd`**

  DW_RLE_start_end

- **`StartLength`**

  DW_RLE_start_length

#### Implementations

- <span id="rawrnglistentry-parse"></span>`fn parse<R: Reader<Offset = T>>(input: &mut R, encoding: Encoding, format: RangeListsFormat) -> Result<Option<Self>>` — [`Encoding`](../index.md#encoding), [`RangeListsFormat`](rnglists/index.md#rangelistsformat), [`Result`](../index.md#result)

  Parse a range entry from `.debug_rnglists`

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for RawRngListEntry<T>`

- <span id="rawrnglistentry-clone"></span>`fn clone(&self) -> RawRngListEntry<T>` — [`RawRngListEntry`](#rawrnglistentry)

##### `impl<T: fmt::Debug> Debug for RawRngListEntry<T>`

- <span id="rawrnglistentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="unittype-dw-ut"></span>`fn dw_ut(&self) -> constants::DwUt` — [`DwUt`](../index.md#dwut)

#### Trait Implementations

##### `impl<Offset> Clone for UnitType<Offset>`

- <span id="unittype-clone"></span>`fn clone(&self) -> UnitType<Offset>` — [`UnitType`](#unittype)

##### `impl<Offset> Copy for UnitType<Offset>`

##### `impl<Offset> Debug for UnitType<Offset>`

- <span id="unittype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Offset> Eq for UnitType<Offset>`

##### `impl<Offset> PartialEq for UnitType<Offset>`

- <span id="unittype-partialeq-eq"></span>`fn eq(&self, other: &UnitType<Offset>) -> bool` — [`UnitType`](#unittype)

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

- <span id="attributevalue-offset-value"></span>`fn offset_value(&self) -> Option<<R as >::Offset>` — [`Reader`](#reader)

  Try to convert this attribute's value to an offset.

- <span id="attributevalue-exprloc-value"></span>`fn exprloc_value(&self) -> Option<Expression<R>>` — [`Expression`](#expression)

  Try to convert this attribute's value to an expression or location buffer.

  

  Expressions and locations may be `DW_FORM_block*` or `DW_FORM_exprloc`.

  The standard doesn't mention `DW_FORM_block*` as a possible form, but

  it is encountered in practice.

- <span id="attributevalue-string-value"></span>`fn string_value(&self, debug_str: &DebugStr<R>) -> Option<R>` — [`DebugStr`](#debugstr)

  Try to return this attribute's value as a string slice.

  

  If this attribute's value is either an inline `DW_FORM_string` string,

  or a `DW_FORM_strp` reference to an offset into the `.debug_str`

  section, return the attribute's string value as `Some`. Other attribute

  value forms are returned as `None`.

  

  Warning: this function does not handle all possible string forms.

  Use `Dwarf::attr_string` instead.

- <span id="attributevalue-string-value-sup"></span>`fn string_value_sup(&self, debug_str: &DebugStr<R>, debug_str_sup: Option<&DebugStr<R>>) -> Option<R>` — [`DebugStr`](#debugstr)

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

- <span id="attributevalue-clone"></span>`fn clone(&self) -> AttributeValue<R, Offset>` — [`AttributeValue`](#attributevalue)

##### `impl<R, Offset> Copy for AttributeValue<R, Offset>`

##### `impl<R, Offset> Debug for AttributeValue<R, Offset>`

- <span id="attributevalue-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R, Offset> Eq for AttributeValue<R, Offset>`

##### `impl<R, Offset> PartialEq for AttributeValue<R, Offset>`

- <span id="attributevalue-partialeq-eq"></span>`fn eq(&self, other: &AttributeValue<R, Offset>) -> bool` — [`AttributeValue`](#attributevalue)

##### `impl<R, Offset> StructuralPartialEq for AttributeValue<R, Offset>`

### `ValueType`

```rust
enum ValueType {
    Generic,
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    F32,
    F64,
}
```

The type of an entry on the DWARF stack.

#### Variants

- **`Generic`**

  The generic type, which is address-sized and of unspecified sign,
  as specified in the DWARF 5 standard, section 2.5.1.
  This type is also used to represent address base types.

- **`I8`**

  Signed 8-bit integer type.

- **`U8`**

  Unsigned 8-bit integer type.

- **`I16`**

  Signed 16-bit integer type.

- **`U16`**

  Unsigned 16-bit integer type.

- **`I32`**

  Signed 32-bit integer type.

- **`U32`**

  Unsigned 32-bit integer type.

- **`I64`**

  Signed 64-bit integer type.

- **`U64`**

  Unsigned 64-bit integer type.

- **`F32`**

  32-bit floating point type.

- **`F64`**

  64-bit floating point type.

#### Implementations

- <span id="valuetype-bit-size"></span>`fn bit_size(self, addr_mask: u64) -> u32`

  The size in bits of a value for this type.

- <span id="valuetype-from-encoding"></span>`fn from_encoding(encoding: constants::DwAte, byte_size: u64) -> Option<ValueType>` — [`DwAte`](../index.md#dwate), [`ValueType`](#valuetype)

  Construct a `ValueType` from the attributes of a base type DIE.

- <span id="valuetype-from-entry"></span>`fn from_entry<R: Reader>(entry: &DebuggingInformationEntry<R>) -> Result<Option<ValueType>>` — [`DebuggingInformationEntry`](#debugginginformationentry), [`Result`](../index.md#result), [`ValueType`](#valuetype)

  Construct a `ValueType` from a base type DIE.

#### Trait Implementations

##### `impl Clone for ValueType`

- <span id="valuetype-clone"></span>`fn clone(&self) -> ValueType` — [`ValueType`](#valuetype)

##### `impl Copy for ValueType`

##### `impl Debug for ValueType`

- <span id="valuetype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ValueType`

##### `impl PartialEq for ValueType`

- <span id="valuetype-partialeq-eq"></span>`fn eq(&self, other: &ValueType) -> bool` — [`ValueType`](#valuetype)

##### `impl StructuralPartialEq for ValueType`

### `Value`

```rust
enum Value {
    Generic(u64),
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    F32(f32),
    F64(f64),
}
```

The value of an entry on the DWARF stack.

#### Variants

- **`Generic`**

  A generic value, which is address-sized and of unspecified sign.

- **`I8`**

  A signed 8-bit integer value.

- **`U8`**

  An unsigned 8-bit integer value.

- **`I16`**

  A signed 16-bit integer value.

- **`U16`**

  An unsigned 16-bit integer value.

- **`I32`**

  A signed 32-bit integer value.

- **`U32`**

  An unsigned 32-bit integer value.

- **`I64`**

  A signed 64-bit integer value.

- **`U64`**

  An unsigned 64-bit integer value.

- **`F32`**

  A 32-bit floating point value.

- **`F64`**

  A 64-bit floating point value.

#### Implementations

- <span id="value-value-type"></span>`fn value_type(&self) -> ValueType` — [`ValueType`](#valuetype)

  Return the `ValueType` corresponding to this `Value`.

- <span id="value-parse"></span>`fn parse<R: Reader>(value_type: ValueType, bytes: R) -> Result<Value>` — [`ValueType`](#valuetype), [`Result`](../index.md#result), [`Value`](#value)

  Read a `Value` with the given `value_type` from a `Reader`.

- <span id="value-to-u64"></span>`fn to_u64(self, addr_mask: u64) -> Result<u64>` — [`Result`](../index.md#result)

  Convert a `Value` to a `u64`.

  

  The `ValueType` of `self` must be integral.

  Values are sign extended if the source value is signed.

- <span id="value-from-u64"></span>`fn from_u64(value_type: ValueType, value: u64) -> Result<Value>` — [`ValueType`](#valuetype), [`Result`](../index.md#result), [`Value`](#value)

  Create a `Value` with the given `value_type` from a `u64` value.

  

  The `value_type` may be integral or floating point.

  The result is truncated if the `u64` value does

  not fit the bounds of the `value_type`.

- <span id="value-from-f32"></span>`fn from_f32(value_type: ValueType, value: f32) -> Result<Value>` — [`ValueType`](#valuetype), [`Result`](../index.md#result), [`Value`](#value)

  Create a `Value` with the given `value_type` from a `f32` value.

  

  The `value_type` may be integral or floating point.

  The result is not defined if the `f32` value does

  not fit the bounds of the `value_type`.

- <span id="value-from-f64"></span>`fn from_f64(value_type: ValueType, value: f64) -> Result<Value>` — [`ValueType`](#valuetype), [`Result`](../index.md#result), [`Value`](#value)

  Create a `Value` with the given `value_type` from a `f64` value.

  

  The `value_type` may be integral or floating point.

  The result is not defined if the `f64` value does

  not fit the bounds of the `value_type`.

- <span id="value-convert"></span>`fn convert(self, value_type: ValueType, addr_mask: u64) -> Result<Value>` — [`ValueType`](#valuetype), [`Result`](../index.md#result), [`Value`](#value)

  Convert a `Value` to the given `value_type`.

  

  When converting between integral types, the result is truncated

  if the source value does not fit the bounds of the `value_type`.

  When converting from floating point types, the result is not defined

  if the source value does not fit the bounds of the `value_type`.

  

  This corresponds to the DWARF `DW_OP_convert` operation.

- <span id="value-reinterpret"></span>`fn reinterpret(self, value_type: ValueType, addr_mask: u64) -> Result<Value>` — [`ValueType`](#valuetype), [`Result`](../index.md#result), [`Value`](#value)

  Reinterpret the bits in a `Value` as the given `value_type`.

  

  The source and result value types must have equal sizes.

  

  This corresponds to the DWARF `DW_OP_reinterpret` operation.

- <span id="value-abs"></span>`fn abs(self, addr_mask: u64) -> Result<Value>` — [`Result`](../index.md#result), [`Value`](#value)

  Perform an absolute value operation.

  

  If the value type is `Generic`, then it is interpreted as a signed value.

  

  This corresponds to the DWARF `DW_OP_abs` operation.

- <span id="value-neg"></span>`fn neg(self, addr_mask: u64) -> Result<Value>` — [`Result`](../index.md#result), [`Value`](#value)

  Perform a negation operation.

  

  If the value type is `Generic`, then it is interpreted as a signed value.

  

  This corresponds to the DWARF `DW_OP_neg` operation.

- <span id="value-add"></span>`fn add(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](#value), [`Result`](../index.md#result)

  Perform an addition operation.

  

  This operation requires matching types.

  

  This corresponds to the DWARF `DW_OP_plus` operation.

- <span id="value-sub"></span>`fn sub(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](#value), [`Result`](../index.md#result)

  Perform a subtraction operation.

  

  This operation requires matching types.

  

  This corresponds to the DWARF `DW_OP_minus` operation.

- <span id="value-mul"></span>`fn mul(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](#value), [`Result`](../index.md#result)

  Perform a multiplication operation.

  

  This operation requires matching types.

  

  This corresponds to the DWARF `DW_OP_mul` operation.

- <span id="value-div"></span>`fn div(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](#value), [`Result`](../index.md#result)

  Perform a division operation.

  

  This operation requires matching types.

  If the value type is `Generic`, then it is interpreted as a signed value.

  

  This corresponds to the DWARF `DW_OP_div` operation.

- <span id="value-rem"></span>`fn rem(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](#value), [`Result`](../index.md#result)

  Perform a remainder operation.

  

  This operation requires matching integral types.

  If the value type is `Generic`, then it is interpreted as an unsigned value.

  

  This corresponds to the DWARF `DW_OP_mod` operation.

- <span id="value-not"></span>`fn not(self, addr_mask: u64) -> Result<Value>` — [`Result`](../index.md#result), [`Value`](#value)

  Perform a bitwise not operation.

  

  This operation requires matching integral types.

  

  This corresponds to the DWARF `DW_OP_not` operation.

- <span id="value-and"></span>`fn and(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](#value), [`Result`](../index.md#result)

  Perform a bitwise and operation.

  

  This operation requires matching integral types.

  

  This corresponds to the DWARF `DW_OP_and` operation.

- <span id="value-or"></span>`fn or(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](#value), [`Result`](../index.md#result)

  Perform a bitwise or operation.

  

  This operation requires matching integral types.

  

  This corresponds to the DWARF `DW_OP_or` operation.

- <span id="value-xor"></span>`fn xor(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](#value), [`Result`](../index.md#result)

  Perform a bitwise exclusive-or operation.

  

  This operation requires matching integral types.

  

  This corresponds to the DWARF `DW_OP_xor` operation.

- <span id="value-shift-length"></span>`fn shift_length(self) -> Result<u64>` — [`Result`](../index.md#result)

  Convert value to bit length suitable for a shift operation.

  

  If the value is negative then an error is returned.

- <span id="value-shl"></span>`fn shl(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](#value), [`Result`](../index.md#result)

  Perform a shift left operation.

  

  This operation requires integral types.

  If the shift length exceeds the type size, then 0 is returned.

  If the shift length is negative then an error is returned.

  

  This corresponds to the DWARF `DW_OP_shl` operation.

- <span id="value-shr"></span>`fn shr(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](#value), [`Result`](../index.md#result)

  Perform a logical shift right operation.

  

  This operation requires an unsigned integral type for the value.

  If the value type is `Generic`, then it is interpreted as an unsigned value.

  

  This operation requires an integral type for the shift length.

  If the shift length exceeds the type size, then 0 is returned.

  If the shift length is negative then an error is returned.

  

  This corresponds to the DWARF `DW_OP_shr` operation.

- <span id="value-shra"></span>`fn shra(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](#value), [`Result`](../index.md#result)

  Perform an arithmetic shift right operation.

  

  This operation requires a signed integral type for the value.

  If the value type is `Generic`, then it is interpreted as a signed value.

  

  This operation requires an integral type for the shift length.

  If the shift length exceeds the type size, then 0 is returned for positive values,

  and -1 is returned for negative values.

  If the shift length is negative then an error is returned.

  

  This corresponds to the DWARF `DW_OP_shra` operation.

- <span id="value-eq"></span>`fn eq(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](#value), [`Result`](../index.md#result)

  Perform the `==` relational operation.

  

  This operation requires matching integral types.

  If the value type is `Generic`, then it is interpreted as a signed value.

  

  This corresponds to the DWARF `DW_OP_eq` operation.

- <span id="value-ge"></span>`fn ge(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](#value), [`Result`](../index.md#result)

  Perform the `>=` relational operation.

  

  This operation requires matching integral types.

  If the value type is `Generic`, then it is interpreted as a signed value.

  

  This corresponds to the DWARF `DW_OP_ge` operation.

- <span id="value-gt"></span>`fn gt(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](#value), [`Result`](../index.md#result)

  Perform the `>` relational operation.

  

  This operation requires matching integral types.

  If the value type is `Generic`, then it is interpreted as a signed value.

  

  This corresponds to the DWARF `DW_OP_gt` operation.

- <span id="value-le"></span>`fn le(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](#value), [`Result`](../index.md#result)

  Perform the `<= relational operation.

  

  This operation requires matching integral types.

  If the value type is `Generic`, then it is interpreted as a signed value.

  

  This corresponds to the DWARF `DW_OP_le` operation.

- <span id="value-lt"></span>`fn lt(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](#value), [`Result`](../index.md#result)

  Perform the `< relational operation.

  

  This operation requires matching integral types.

  If the value type is `Generic`, then it is interpreted as a signed value.

  

  This corresponds to the DWARF `DW_OP_lt` operation.

- <span id="value-ne"></span>`fn ne(self, rhs: Value, addr_mask: u64) -> Result<Value>` — [`Value`](#value), [`Result`](../index.md#result)

  Perform the `!= relational operation.

  

  This operation requires matching integral types.

  If the value type is `Generic`, then it is interpreted as a signed value.

  

  This corresponds to the DWARF `DW_OP_ne` operation.

#### Trait Implementations

##### `impl Clone for Value`

- <span id="value-clone"></span>`fn clone(&self) -> Value` — [`Value`](#value)

##### `impl Copy for Value`

##### `impl Debug for Value`

- <span id="value-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl PartialEq for Value`

- <span id="value-partialeq-eq"></span>`fn eq(&self, other: &Value) -> bool` — [`Value`](#value)

##### `impl StructuralPartialEq for Value`

## Traits

### `Section<R>`

```rust
trait Section<R>: From<R> { ... }
```

A convenience trait for loading DWARF sections from object files.  To be
used like:

```rust
use gimli::{DebugInfo, EndianSlice, LittleEndian, Reader, Section};

let buf = [0x00, 0x01, 0x02, 0x03];
let reader = EndianSlice::new(&buf, LittleEndian);
let loader = |name| -> Result<_, ()> { Ok(reader) };

let debug_info: DebugInfo<_> = Section::load(loader).unwrap();
```

#### Required Methods

- `fn id() -> SectionId`

  Returns the section id for this type.

- `fn reader(&self) -> &R`

  Returns the `Reader` for this section.

#### Provided Methods

- `fn section_name() -> &'static str`

  Returns the ELF section name for this type.

- `fn dwo_section_name() -> Option<&'static str>`

  Returns the ELF section name (if any) for this type when used in a dwo

- `fn xcoff_section_name() -> Option<&'static str>`

  Returns the XCOFF section name (if any) for this type when used in a XCOFF

- `fn load<F, E>(f: F) -> core::result::Result<Self, E>`

  Try to load the section using the given loader function.

- `fn dwp_range(&self, offset: u32, size: u32) -> Result<Self>`

  Returns the subrange of the section that is the contribution of

- `fn lookup_offset_id(&self, id: ReaderOffsetId) -> Option<(SectionId, <R as >::Offset)>`

  Returns the `Reader` for this section.

#### Implementors

- [`DebugAbbrev`](#debugabbrev)
- [`DebugAddr`](#debugaddr)
- [`DebugAranges`](#debugaranges)
- [`DebugCuIndex`](#debugcuindex)
- [`DebugFrame`](#debugframe)
- [`DebugInfo`](#debuginfo)
- [`DebugLineStr`](#debuglinestr)
- [`DebugLine`](#debugline)
- [`DebugLocLists`](#debugloclists)
- [`DebugLoc`](#debugloc)
- [`DebugMacinfo`](#debugmacinfo)
- [`DebugMacro`](#debugmacro)
- [`DebugNames`](#debugnames)
- [`DebugPubNames`](#debugpubnames)
- [`DebugPubTypes`](#debugpubtypes)
- [`DebugRanges`](#debugranges)
- [`DebugRngLists`](#debugrnglists)
- [`DebugStrOffsets`](#debugstroffsets)
- [`DebugStr`](#debugstr)
- [`DebugTuIndex`](#debugtuindex)
- [`DebugTypes`](#debugtypes)
- [`EhFrameHdr`](#ehframehdr)
- [`EhFrame`](#ehframe)

### `ArrayLike`

```rust
trait ArrayLike: Sealed { ... }
```

Marker trait for types that can be used as backing storage when a growable array type is needed.

This trait is sealed and cannot be implemented for types outside this crate.

#### Associated Types

- `type Item`

#### Implementors

- `[T; N]`
- `alloc::boxed::Box<[T; N]>`
- `alloc::vec::Vec<T>`

### `UnwindOffset<T>`

```rust
trait UnwindOffset<T>: Copy + Debug + Eq + From<T>
where
    T: ReaderOffset { ... }
```

An offset into an `UnwindSection`.

#### Required Methods

- `fn into(self) -> T`

  Convert an `UnwindOffset<T>` into a `T`.

#### Implementors

- [`DebugFrameOffset`](../index.md#debugframeoffset)
- [`EhFrameOffset`](../index.md#ehframeoffset)

### `UnwindSection<R: Reader>`

```rust
trait UnwindSection<R: Reader>: Clone + Debug + _UnwindSectionPrivate<R> { ... }
```

A section holding unwind information: either `.debug_frame` or
`.eh_frame`. See [`DebugFrame`](./struct.DebugFrame.html) and
[`EhFrame`](./struct.EhFrame.html) respectively.

#### Associated Types

- `type Offset: 1`

#### Provided Methods

- `fn entries<'bases>(&self, bases: &'bases BaseAddresses) -> CfiEntriesIter<'bases, Self, R>`

  Iterate over the `CommonInformationEntry`s and `FrameDescriptionEntry`s

- `fn cie_from_offset(&self, bases: &BaseAddresses, offset: <Self as >::Offset) -> Result<CommonInformationEntry<R>>`

  Parse the `CommonInformationEntry` at the given offset.

- `fn partial_fde_from_offset<'bases>(&self, bases: &'bases BaseAddresses, offset: <Self as >::Offset) -> Result<PartialFrameDescriptionEntry<'bases, Self, R>>`

  Parse the `PartialFrameDescriptionEntry` at the given offset.

- `fn fde_from_offset<F>(&self, bases: &BaseAddresses, offset: <Self as >::Offset, get_cie: F) -> Result<FrameDescriptionEntry<R>>`

  Parse the `FrameDescriptionEntry` at the given offset.

- `fn fde_for_address<F>(&self, bases: &BaseAddresses, address: u64, get_cie: F) -> Result<FrameDescriptionEntry<R>>`

  Find the `FrameDescriptionEntry` for the given address.

- `fn unwind_info_for_address<'ctx, F, S>(&self, bases: &BaseAddresses, ctx: &'ctx mut UnwindContext<<R as >::Offset, S>, address: u64, get_cie: F) -> Result<&'ctx UnwindTableRow<<R as >::Offset, S>>`

  Find the frame unwind information for the given address.

#### Implementors

- [`DebugFrame`](#debugframe)
- [`EhFrame`](#ehframe)

### `UnwindContextStorage<T: ReaderOffset>`

```rust
trait UnwindContextStorage<T: ReaderOffset>: Sized { ... }
```

Specification of what storage should be used for [`UnwindContext`](#unwindcontext).

Normally you would only need to use [`StoreOnHeap`](../index.md), which places the stack
on the heap using [`Box`](../../allocator_api2/stable/boxed/index.md). This is the default storage type parameter for [`UnwindContext`](#unwindcontext).

You may want to supply your own storage type for one of the following reasons:

  1. In rare cases you may run into failed unwinds due to the fixed stack size
     used by [`StoreOnHeap`](../index.md), so you may want to try a larger `Box`. If denial
     of service is not a concern, then you could also try a `Vec`-based stack which
     can grow as needed.
  2. You may want to avoid heap allocations entirely. You can use a fixed-size
     stack with in-line arrays, which will place the entire storage in-line into
     [`UnwindContext`](#unwindcontext).

Here's an implementation which uses a fixed-size stack and allocates everything in-line,
which will cause `UnwindContext` to be large:

```rust,no_run
use gimli::*;

fn foo<'a>(some_fde: gimli::FrameDescriptionEntry<gimli::EndianSlice<'a, gimli::LittleEndian>>)
           -> gimli::Result<()> {
let eh_frame: gimli::EhFrame<_> = unreachable!();
let bases = unimplemented!();

struct StoreOnStack;

impl<T: ReaderOffset> UnwindContextStorage<T> for StoreOnStack {
    type Rules = [(Register, RegisterRule<T>); 192];
    type Stack = [UnwindTableRow<T, Self>; 4];
}

let mut ctx = UnwindContext::<_, StoreOnStack>::new_in();

// Initialize the context by evaluating the CIE's initial instruction program,
// and generate the unwind table.
let mut table = some_fde.rows(&eh_frame, &bases, &mut ctx)?;
while let Some(row) = table.next_row()? {
    // Do stuff with each row...
  let _ = row;
}
unreachable!()
}
```

#### Associated Types

- `type Rules: 1`

- `type Stack: 1`

#### Implementors

- [`StoreOnHeap`](../index.md#storeonheap)

### `ReaderOffset`

```rust
trait ReaderOffset: Debug + Copy + Eq + Ord + Hash + Add<Output = Self> + AddAssign + Sub<Output = Self> { ... }
```

A trait for offsets with a DWARF section.

This allows consumers to choose a size that is appropriate for their address space.

#### Required Methods

- `fn from_u8(offset: u8) -> Self`

  Convert a u8 to an offset.

- `fn from_u16(offset: u16) -> Self`

  Convert a u16 to an offset.

- `fn from_i16(offset: i16) -> Self`

  Convert an i16 to an offset.

- `fn from_u32(offset: u32) -> Self`

  Convert a u32 to an offset.

- `fn from_u64(offset: u64) -> Result<Self>`

  Convert a u64 to an offset.

- `fn into_u64(self) -> u64`

  Convert an offset to a u64.

- `fn wrapping_add(self, other: Self) -> Self`

  Wrapping (modular) addition. Computes `self + other`.

- `fn checked_sub(self, other: Self) -> Option<Self>`

  Checked subtraction. Computes `self - other`.

#### Implementors

- `u32`
- `u64`
- `usize`

### `ReaderAddress`

```rust
trait ReaderAddress: Sized { ... }
```

A trait for addresses within a DWARF section.

Currently this is a simple extension trait for `u64`, but it may be expanded
in the future to support user-defined address types.

#### Required Methods

- `fn add_sized(self, length: u64, size: u8) -> Result<Self>`

  Add a length to an address of the given size.

- `fn wrapping_add_sized(self, length: u64, size: u8) -> Self`

  Add a length to an address of the given size.

- `fn zeros() -> Self`

  The all-zeros value of an address.

- `fn ones_sized(size: u8) -> Self`

  The all-ones value of an address of the given size.

#### Provided Methods

- `fn min_tombstone(size: u8) -> Self`

  Return the minimum value for a tombstone address.

#### Implementors

- `u64`

### `Reader`

```rust
trait Reader: Debug + Clone { ... }
```

A trait for reading the data from a DWARF section.

All read operations advance the section offset of the reader
unless specified otherwise.

## Choosing a `Reader` Implementation

`gimli` comes with a few different `Reader` implementations and lets you
choose the one that is right for your use case. A `Reader` is essentially a
view into the raw bytes that make up some DWARF, but this view might borrow
the underlying data or use reference counting ownership, and it might be
thread safe or not.

| Implementation    | Ownership         | Thread Safe | Notes |
|:------------------|:------------------|:------------|:------|
| [`EndianSlice`](./struct.EndianSlice.html)        | Borrowed          | Yes         | Fastest, but requires that all of your code work with borrows. |
| [`EndianRcSlice`](./struct.EndianRcSlice.html)    | Reference counted | No          | Shared ownership via reference counting, which alleviates the borrow restrictions of `EndianSlice` but imposes reference counting increments and decrements. Cannot be sent across threads, because the reference count is not atomic. |
| [`EndianArcSlice`](./struct.EndianArcSlice.html)  | Reference counted | Yes         | The same as `EndianRcSlice`, but uses atomic reference counting, and therefore reference counting operations are slower but `EndianArcSlice`s may be sent across threads. |
| [`EndianReader<T>`](./struct.EndianReader.html)   | Same as `T`       | Same as `T` | Escape hatch for easily defining your own type of `Reader`. |

#### Associated Types

- `type Endian: 1`

- `type Offset: 1`

#### Required Methods

- `fn endian(&self) -> <Self as >::Endian`

  Return the endianity of bytes that are read.

- `fn len(&self) -> <Self as >::Offset`

  Return the number of bytes remaining.

- `fn empty(&mut self)`

  Set the number of bytes remaining to zero.

- `fn truncate(&mut self, len: <Self as >::Offset) -> Result<()>`

  Set the number of bytes remaining to the specified length.

- `fn offset_from(&self, base: &Self) -> <Self as >::Offset`

  Return the offset of this reader's data relative to the start of

- `fn offset_id(&self) -> ReaderOffsetId`

  Return an identifier for the current reader offset.

- `fn lookup_offset_id(&self, id: ReaderOffsetId) -> Option<<Self as >::Offset>`

  Return the offset corresponding to the given `id` if

- `fn find(&self, byte: u8) -> Result<<Self as >::Offset>`

  Find the index of the first occurrence of the given byte.

- `fn skip(&mut self, len: <Self as >::Offset) -> Result<()>`

  Discard the specified number of bytes.

- `fn split(&mut self, len: <Self as >::Offset) -> Result<Self>`

  Split a reader in two.

- `fn to_slice(&self) -> Result<Cow<'_, [u8]>>`

  Return all remaining data as a clone-on-write slice.

- `fn to_string(&self) -> Result<Cow<'_, str>>`

  Convert all remaining data to a clone-on-write string.

- `fn to_string_lossy(&self) -> Result<Cow<'_, str>>`

  Convert all remaining data to a clone-on-write string, including invalid characters.

- `fn read_slice(&mut self, buf: &mut [u8]) -> Result<()>`

  Read exactly `buf.len()` bytes into `buf`.

#### Provided Methods

- `fn read_u8_array<A>(&mut self) -> Result<A>`

  Read a u8 array.

- `fn is_empty(&self) -> bool`

  Return true if the number of bytes remaining is zero.

- `fn read_u8(&mut self) -> Result<u8>`

  Read a u8.

- `fn read_i8(&mut self) -> Result<i8>`

  Read an i8.

- `fn read_u16(&mut self) -> Result<u16>`

  Read a u16.

- `fn read_i16(&mut self) -> Result<i16>`

  Read an i16.

- `fn read_u32(&mut self) -> Result<u32>`

  Read a u32.

- `fn read_i32(&mut self) -> Result<i32>`

  Read an i32.

- `fn read_u64(&mut self) -> Result<u64>`

  Read a u64.

- `fn read_u128(&mut self) -> Result<u128>`

  Read a u128.

- `fn read_i64(&mut self) -> Result<i64>`

  Read an i64.

- `fn read_f32(&mut self) -> Result<f32>`

  Read a f32.

- `fn read_f64(&mut self) -> Result<f64>`

  Read a f64.

- `fn read_uint(&mut self, n: usize) -> Result<u64>`

  Read an unsigned n-bytes integer u64.

- `fn read_null_terminated_slice(&mut self) -> Result<Self>`

  Read a null-terminated slice, and return it (excluding the null).

- `fn skip_leb128(&mut self) -> Result<()>`

  Skip a LEB128 encoded integer.

- `fn read_uleb128(&mut self) -> Result<u64>`

  Read an unsigned LEB128 encoded integer.

- `fn read_uleb128_u32(&mut self) -> Result<u32>`

  Read an unsigned LEB128 encoded u32.

- `fn read_uleb128_u16(&mut self) -> Result<u16>`

  Read an unsigned LEB128 encoded u16.

- `fn read_sleb128(&mut self) -> Result<i64>`

  Read a signed LEB128 encoded integer.

- `fn read_initial_length(&mut self) -> Result<(<Self as >::Offset, Format)>`

  Read an initial length field.

- `fn read_address_size(&mut self) -> Result<u8>`

  Read a byte and validate it as an address size.

- `fn read_address(&mut self, address_size: u8) -> Result<u64>`

  Read an address-sized integer, and return it as a `u64`.

- `fn read_word(&mut self, format: Format) -> Result<<Self as >::Offset>`

  Parse a word-sized integer according to the DWARF format.

- `fn read_length(&mut self, format: Format) -> Result<<Self as >::Offset>`

  Parse a word-sized section length according to the DWARF format.

- `fn read_offset(&mut self, format: Format) -> Result<<Self as >::Offset>`

  Parse a word-sized section offset according to the DWARF format.

- `fn read_sized_offset(&mut self, size: u8) -> Result<<Self as >::Offset>`

  Parse a section offset of the given size.

#### Implementors

- [`EndianReader`](#endianreader)
- [`EndianSlice`](#endianslice)
- [`RelocateReader`](#relocatereader)

### `Relocate<T: ReaderOffset>`

```rust
trait Relocate<T: ReaderOffset> { ... }
```

Trait for relocating addresses and offsets while reading a section.

#### Required Methods

- `fn relocate_address(&self, offset: T, value: u64) -> Result<u64>`

  Relocate an address which was read from the given section offset.

- `fn relocate_offset(&self, offset: T, value: T) -> Result<T>`

  Relocate a value which was read from the given section offset.

### `LineProgram<R, Offset>`

```rust
trait LineProgram<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset { ... }
```

A `LineProgram` provides access to a `LineProgramHeader` and
a way to add files to the files table if necessary. Gimli consumers should
never need to use or see this trait.

#### Required Methods

- `fn header(&self) -> &LineProgramHeader<R, Offset>`

  Get a reference to the held `LineProgramHeader`.

- `fn add_file(&mut self, file: FileEntry<R, Offset>)`

  Add a file to the file table if necessary.

#### Implementors

- [`IncompleteLineProgram`](#incompletelineprogram)
- `&'program CompleteLineProgram<R, Offset>`

### `EvaluationStorage<R: Reader>`

```rust
trait EvaluationStorage<R: Reader> { ... }
```

Specification of what storage should be used for [`Evaluation`](#evaluation).

Normally you would only need to use [`StoreOnHeap`](../index.md), which places the stacks and the results
on the heap using [`Vec`](../../addr2line/maybe_small/index.md). This is the default storage type parameter for [`Evaluation`](#evaluation).

If you need to avoid [`Evaluation`](#evaluation) from allocating memory, e.g. for signal safety,
you can provide you own storage specification:
```rust,no_run
use gimli::*;
let bytecode = EndianSlice::new(&[], LittleEndian);
let encoding = unimplemented!();
let get_register_value = |_, _| Value::Generic(42);
let get_frame_base = || 0xdeadbeef;

struct StoreOnStack;

impl<R: Reader> EvaluationStorage<R> for StoreOnStack {
    type Stack = [Value; 64];
    type ExpressionStack = [(R, R); 4];
    type Result = [Piece<R>; 1];
}

let mut eval = Evaluation::<_, StoreOnStack>::new_in(bytecode, encoding);
let mut result = eval.evaluate().unwrap();
while result != EvaluationResult::Complete {
  match result {
    EvaluationResult::RequiresRegister { register, base_type } => {
      let value = get_register_value(register, base_type);
      result = eval.resume_with_register(value).unwrap();
    },
    EvaluationResult::RequiresFrameBase => {
      let frame_base = get_frame_base();
      result = eval.resume_with_frame_base(frame_base).unwrap();
    },
    _ => unimplemented!(),
  };
}

let result = eval.as_result();
println!("{:?}", result);
```

#### Associated Types

- `type Stack: 1`

- `type ExpressionStack: 1`

- `type Result: 1`

#### Implementors

- [`StoreOnHeap`](../index.md#storeonheap)

## Functions

### `parse_cfi_entry`

```rust
fn parse_cfi_entry<'bases, Section, R>(section: &Section, bases: &'bases BaseAddresses, input: &mut R) -> crate::read::Result<Option<CieOrFde<'bases, Section, R>>>
where
    R: Reader,
    Section: UnwindSection<R>
```

### `parse_cfi_entry_prefix`

```rust
fn parse_cfi_entry_prefix<Section, R>(section: &Section, input: &mut R) -> crate::read::Result<Option<CfiEntryPrefix<R>>>
where
    R: Reader,
    Section: UnwindSection<R>
```

### `parse_encoded_pointer`

```rust
fn parse_encoded_pointer<R: Reader>(encoding: constants::DwEhPe, parameters: &PointerEncodingParameters<'_, R>, input: &mut R) -> crate::read::Result<Pointer>
```

### `parse_encoded_value`

```rust
fn parse_encoded_value<R: Reader>(encoding: constants::DwEhPe, parameters: &PointerEncodingParameters<'_, R>, input: &mut R) -> crate::read::Result<u64>
```

### `get_attribute_size`

```rust
fn get_attribute_size(form: constants::DwForm, encoding: crate::common::Encoding) -> Option<u8>
```

### `parse_directory_v5`

```rust
fn parse_directory_v5<R: Reader>(input: &mut R, encoding: crate::common::Encoding, formats: &[FileEntryFormat]) -> crate::read::Result<crate::read::AttributeValue<R>>
```

### `parse_file_v5`

```rust
fn parse_file_v5<R: Reader>(input: &mut R, encoding: crate::common::Encoding, formats: &[FileEntryFormat]) -> crate::read::Result<FileEntry<R>>
```

### `parse_attribute`

```rust
fn parse_attribute<R: Reader>(input: &mut R, encoding: crate::common::Encoding, form: constants::DwForm) -> crate::read::Result<crate::read::AttributeValue<R>>
```

### `parse_data`

```rust
fn parse_data<R: Reader>(input: &mut R, encoding: crate::common::Encoding) -> crate::read::Result<crate::read::Expression<R>>
```

### `read_debug_names_form_value`

```rust
fn read_debug_names_form_value<R: Reader>(input: &mut R, form: constants::DwForm) -> crate::read::Result<NameAttributeValue<R>>
```

Read an attribute value.

This handles the subset of DWARF forms used in `.debug_names` entry pools
(`DW_IDX_*` attributes).

### `compute_pc`

```rust
fn compute_pc<R: Reader>(pc: &R, bytecode: &R, offset: i16) -> crate::read::Result<R>
```

### `generic_type`

```rust
fn generic_type<O: ReaderOffset>() -> crate::read::UnitOffset<O>
```

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

### `sign_extend`

```rust
fn sign_extend(value: u64, mask: u64) -> i64
```

Convert a u64 to an i64, with sign extension if required.

This is primarily used when needing to treat `Value::Generic`
as a signed value.

### `mask_bit_size`

```rust
fn mask_bit_size(addr_mask: u64) -> u32
```

## Type Aliases

### `Result<T>`

```rust
type Result<T> = result::Result<T, Error>;
```

The result of a parse.

### `EndianRcSlice<Endian>`

```rust
type EndianRcSlice<Endian> = EndianReader<Endian, alloc::rc::Rc<[u8]>>;
```

A reference counted, non-thread-safe slice of bytes and associated
endianity.

```rust
#[cfg(feature = "std")] {
use std::rc::Rc;

let buf = Rc::from(&[1, 2, 3, 4][..]);
let reader = gimli::EndianRcSlice::new(buf, gimli::NativeEndian);
let _ = reader;
}
```

### `EndianArcSlice<Endian>`

```rust
type EndianArcSlice<Endian> = EndianReader<Endian, alloc::sync::Arc<[u8]>>;
```

An atomically reference counted, thread-safe slice of bytes and associated
endianity.

```rust
#[cfg(feature = "std")] {
use std::sync::Arc;

let buf = Arc::from(&[1, 2, 3, 4][..]);
let reader = gimli::EndianArcSlice::new(buf, gimli::NativeEndian);
let _ = reader;
}
```

### `OneShotLineRows<R, Offset>`

```rust
type OneShotLineRows<R, Offset> = LineRows<R, IncompleteLineProgram<R, Offset>, Offset>;
```

### `ResumedLineRows<'program, R, Offset>`

```rust
type ResumedLineRows<'program, R, Offset> = LineRows<R, &'program CompleteLineProgram<R, Offset>, Offset>;
```

### `LocListsHeader`

```rust
type LocListsHeader = crate::read::lists::ListsHeader;
```

### `RngListsHeader`

```rust
type RngListsHeader = crate::read::lists::ListsHeader;
```

## Constants

### `MAX_RULES`
```rust
const MAX_RULES: usize = 192usize;
```

### `MAX_UNWIND_STACK_DEPTH`
```rust
const MAX_UNWIND_STACK_DEPTH: usize = 4usize;
```

### `CFI_INSTRUCTION_HIGH_BITS_MASK`
```rust
const CFI_INSTRUCTION_HIGH_BITS_MASK: u8 = 192u8;
```

### `CFI_INSTRUCTION_LOW_BITS_MASK`
```rust
const CFI_INSTRUCTION_LOW_BITS_MASK: u8 = 63u8;
```

### `MAX_ATTRIBUTES_INLINE`
```rust
const MAX_ATTRIBUTES_INLINE: usize = 5usize;
```

### `SECTION_COUNT_MAX`
```rust
const SECTION_COUNT_MAX: u8 = 8u8;
```

