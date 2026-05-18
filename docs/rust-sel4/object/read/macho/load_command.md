**object > read > macho > load_command**

# Module: read::macho::load_command

## Contents

**Structs**

- [`LoadCommandData`](#loadcommanddata) - The data for a [`macho::LoadCommand`].
- [`LoadCommandIterator`](#loadcommanditerator) - An iterator for the load commands from a [`MachHeader`].

**Enums**

- [`LoadCommandVariant`](#loadcommandvariant) - A [`macho::LoadCommand`] that has been interpreted according to its `cmd` field.

---

## object::read::macho::load_command::LoadCommandData

*Struct*

The data for a [`macho::LoadCommand`].

**Generic Parameters:**
- 'data
- E

**Methods:**

- `fn cmd(self: &Self) -> u32` - Return the `cmd` field of the [`macho::LoadCommand`].
- `fn cmdsize(self: &Self) -> u32` - Return the `cmdsize` field of the [`macho::LoadCommand`].
- `fn data<T>(self: &Self) -> Result<&'data T>` - Parse the data as the given type.
- `fn raw_data(self: &Self) -> &'data [u8]` - Raw bytes of this [`macho::LoadCommand`] structure.
- `fn string(self: &Self, endian: E, s: macho::LcStr<E>) -> Result<&'data [u8]>` - Parse a load command string value.
- `fn variant(self: &Self) -> Result<LoadCommandVariant<'data, E>>` - Parse the command data according to the `cmd` field.
- `fn segment_32(self: Self) -> Result<Option<(&'data macho::SegmentCommand32<E>, &'data [u8])>>` - Try to parse this command as a [`macho::SegmentCommand32`].
- `fn symtab(self: Self) -> Result<Option<&'data macho::SymtabCommand<E>>>` - Try to parse this command as a [`macho::SymtabCommand`].
- `fn dysymtab(self: Self) -> Result<Option<&'data macho::DysymtabCommand<E>>>` - Try to parse this command as a [`macho::DysymtabCommand`].
- `fn dylib(self: Self) -> Result<Option<&'data macho::DylibCommand<E>>>` - Try to parse this command as a [`macho::DylibCommand`].
- `fn uuid(self: Self) -> Result<Option<&'data macho::UuidCommand<E>>>` - Try to parse this command as a [`macho::UuidCommand`].
- `fn segment_64(self: Self) -> Result<Option<(&'data macho::SegmentCommand64<E>, &'data [u8])>>` - Try to parse this command as a [`macho::SegmentCommand64`].
- `fn dyld_info(self: Self) -> Result<Option<&'data macho::DyldInfoCommand<E>>>` - Try to parse this command as a [`macho::DyldInfoCommand`].
- `fn entry_point(self: Self) -> Result<Option<&'data macho::EntryPointCommand<E>>>` - Try to parse this command as an [`macho::EntryPointCommand`].
- `fn build_version(self: Self) -> Result<Option<&'data macho::BuildVersionCommand<E>>>` - Try to parse this command as a [`macho::BuildVersionCommand`].

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> LoadCommandData<'data, E>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::read::macho::load_command::LoadCommandIterator

*Struct*

An iterator for the load commands from a [`MachHeader`].

**Generic Parameters:**
- 'data
- E

**Methods:**

- `fn next(self: & mut Self) -> Result<Option<LoadCommandData<'data, E>>>` - Return the next load command.

**Traits:** Copy

**Trait Implementations:**

- **Default**
  - `fn default() -> LoadCommandIterator<'data, E>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
- **Clone**
  - `fn clone(self: &Self) -> LoadCommandIterator<'data, E>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::read::macho::load_command::LoadCommandVariant

*Enum*

A [`macho::LoadCommand`] that has been interpreted according to its `cmd` field.

**Generic Parameters:**
- 'data
- E

**Variants:**
- `Segment32(&'data macho::SegmentCommand32<E>, &'data [u8])` - `LC_SEGMENT`
- `Symtab(&'data macho::SymtabCommand<E>)` - `LC_SYMTAB`
- `Thread(&'data macho::ThreadCommand<E>, &'data [u8])` - `LC_THREAD` or `LC_UNIXTHREAD`
- `Dysymtab(&'data macho::DysymtabCommand<E>)` - `LC_DYSYMTAB`
- `Dylib(&'data macho::DylibCommand<E>)` - `LC_LOAD_DYLIB`, `LC_LOAD_WEAK_DYLIB`, `LC_REEXPORT_DYLIB`,
- `IdDylib(&'data macho::DylibCommand<E>)` - `LC_ID_DYLIB`
- `LoadDylinker(&'data macho::DylinkerCommand<E>)` - `LC_LOAD_DYLINKER`
- `IdDylinker(&'data macho::DylinkerCommand<E>)` - `LC_ID_DYLINKER`
- `PreboundDylib(&'data macho::PreboundDylibCommand<E>)` - `LC_PREBOUND_DYLIB`
- `Routines32(&'data macho::RoutinesCommand32<E>)` - `LC_ROUTINES`
- `SubFramework(&'data macho::SubFrameworkCommand<E>)` - `LC_SUB_FRAMEWORK`
- `SubUmbrella(&'data macho::SubUmbrellaCommand<E>)` - `LC_SUB_UMBRELLA`
- `SubClient(&'data macho::SubClientCommand<E>)` - `LC_SUB_CLIENT`
- `SubLibrary(&'data macho::SubLibraryCommand<E>)` - `LC_SUB_LIBRARY`
- `TwolevelHints(&'data macho::TwolevelHintsCommand<E>)` - `LC_TWOLEVEL_HINTS`
- `PrebindCksum(&'data macho::PrebindCksumCommand<E>)` - `LC_PREBIND_CKSUM`
- `Segment64(&'data macho::SegmentCommand64<E>, &'data [u8])` - `LC_SEGMENT_64`
- `Routines64(&'data macho::RoutinesCommand64<E>)` - `LC_ROUTINES_64`
- `Uuid(&'data macho::UuidCommand<E>)` - `LC_UUID`
- `Rpath(&'data macho::RpathCommand<E>)` - `LC_RPATH`
- `LinkeditData(&'data macho::LinkeditDataCommand<E>)` - `LC_CODE_SIGNATURE`, `LC_SEGMENT_SPLIT_INFO`, `LC_FUNCTION_STARTS`,
- `EncryptionInfo32(&'data macho::EncryptionInfoCommand32<E>)` - `LC_ENCRYPTION_INFO`
- `DyldInfo(&'data macho::DyldInfoCommand<E>)` - `LC_DYLD_INFO` or `LC_DYLD_INFO_ONLY`
- `VersionMin(&'data macho::VersionMinCommand<E>)` - `LC_VERSION_MIN_MACOSX`, `LC_VERSION_MIN_IPHONEOS`, `LC_VERSION_MIN_WATCHOS`,
- `DyldEnvironment(&'data macho::DylinkerCommand<E>)` - `LC_DYLD_ENVIRONMENT`
- `EntryPoint(&'data macho::EntryPointCommand<E>)` - `LC_MAIN`
- `SourceVersion(&'data macho::SourceVersionCommand<E>)` - `LC_SOURCE_VERSION`
- `EncryptionInfo64(&'data macho::EncryptionInfoCommand64<E>)` - `LC_ENCRYPTION_INFO_64`
- `LinkerOption(&'data macho::LinkerOptionCommand<E>)` - `LC_LINKER_OPTION`
- `Note(&'data macho::NoteCommand<E>)` - `LC_NOTE`
- `BuildVersion(&'data macho::BuildVersionCommand<E>)` - `LC_BUILD_VERSION`
- `FilesetEntry(&'data macho::FilesetEntryCommand<E>)` - `LC_FILESET_ENTRY`
- `Other` - An unrecognized or obsolete load command.

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> LoadCommandVariant<'data, E>`



