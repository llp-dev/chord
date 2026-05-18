**object > common**

# Module: common

## Contents

**Enums**

- [`AddressSize`](#addresssize) - The size of an address value for an architecture.
- [`Architecture`](#architecture) - A CPU architecture.
- [`BinaryFormat`](#binaryformat) - A binary file format.
- [`ComdatKind`](#comdatkind) - The selection kind for a COMDAT section group.
- [`FileFlags`](#fileflags) - File flags that are specific to each file format.
- [`RelocationEncoding`](#relocationencoding) - Information about how the result of the relocation operation is encoded in the place.
- [`RelocationFlags`](#relocationflags) - Relocation fields that are specific to each file format and architecture.
- [`RelocationKind`](#relocationkind) - The operation used to calculate the result of the relocation.
- [`SectionFlags`](#sectionflags) - Section flags that are specific to each file format.
- [`SectionKind`](#sectionkind) - The kind of a section.
- [`SegmentFlags`](#segmentflags) - Segment flags that are specific to each file format.
- [`SubArchitecture`](#subarchitecture) - A CPU sub-architecture.
- [`SymbolFlags`](#symbolflags) - Symbol flags that are specific to each file format.
- [`SymbolKind`](#symbolkind) - The kind of a symbol.
- [`SymbolScope`](#symbolscope) - A symbol scope.

---

## object::common::AddressSize

*Enum*

The size of an address value for an architecture.

This may differ from the address size supported by the file format (such as for COFF).

**Variants:**
- `U8`
- `U16`
- `U32`
- `U64`

**Methods:**

- `fn bytes(self: Self) -> u8` - The size in bytes of an address value.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &AddressSize) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> AddressSize`



## object::common::Architecture

*Enum*

A CPU architecture.

**Variants:**
- `Unknown`
- `Aarch64`
- `Aarch64_Ilp32`
- `Alpha`
- `Arm`
- `Avr`
- `Bpf`
- `Csky`
- `E2K32`
- `E2K64`
- `I386`
- `X86_64`
- `X86_64_X32`
- `Hexagon`
- `Hppa`
- `LoongArch32`
- `LoongArch64`
- `M68k`
- `Mips`
- `Mips64`
- `Mips64_N32`
- `Msp430`
- `PowerPc`
- `PowerPc64`
- `Riscv32`
- `Riscv64`
- `S390x`
- `Sbf`
- `Sharc`
- `Sparc`
- `Sparc32Plus`
- `Sparc64`
- `SuperH`
- `Wasm32`
- `Wasm64`
- `Xtensa`

**Methods:**

- `fn address_size(self: Self) -> Option<AddressSize>` - The size of an address value for this architecture.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Architecture`
- **PartialEq**
  - `fn eq(self: &Self, other: &Architecture) -> bool`



## object::common::BinaryFormat

*Enum*

A binary file format.

**Variants:**
- `Coff`
- `Elf`
- `MachO`
- `Pe`
- `Wasm`
- `Xcoff`

**Methods:**

- `fn native_object() -> BinaryFormat` - The target's native binary format for relocatable object files.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &BinaryFormat) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> BinaryFormat`



## object::common::ComdatKind

*Enum*

The selection kind for a COMDAT section group.

This determines the way in which the linker resolves multiple definitions of the COMDAT
sections.

**Variants:**
- `Unknown` - The selection kind is unknown.
- `Any` - Multiple definitions are allowed.
- `NoDuplicates` - Multiple definitions are not allowed.
- `SameSize` - Multiple definitions must have the same size.
- `ExactMatch` - Multiple definitions must match exactly.
- `Largest` - Multiple definitions are allowed, and the largest is selected.
- `Newest` - Multiple definitions are allowed, and the newest is selected.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ComdatKind`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &ComdatKind) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## object::common::FileFlags

*Enum*

File flags that are specific to each file format.

**Variants:**
- `None` - No file flags.
- `Elf{ os_abi: u8, abi_version: u8, e_flags: u32 }` - ELF file flags.
- `MachO{ flags: u32 }` - Mach-O file flags.
- `Coff{ characteristics: u16 }` - COFF file flags.
- `Xcoff{ f_flags: u16 }` - XCOFF file flags.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &FileFlags) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> FileFlags`



## object::common::RelocationEncoding

*Enum*

Information about how the result of the relocation operation is encoded in the place.

This is usually architecture specific, such as specifying an addressing mode or
a specific instruction.

**Variants:**
- `Unknown` - The relocation encoding is unknown.
- `Generic` - Generic encoding.
- `X86Signed` - x86 sign extension at runtime.
- `X86RipRelative` - x86 rip-relative addressing.
- `X86RipRelativeMovq` - x86 rip-relative addressing in movq instruction.
- `X86Branch` - x86 branch instruction.
- `S390xDbl` - s390x PC-relative offset shifted right by one bit.
- `AArch64Call` - AArch64 call target.
- `LoongArchBranch` - LoongArch branch offset with two trailing zeros.
- `SharcTypeA` - SHARC+ 48-bit Type A instruction
- `SharcTypeB` - SHARC+ 32-bit Type B instruction
- `E2KLit` - E2K 64-bit value stored in two LTS
- `E2KDisp` - E2K 28-bit value stored in CS0

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> RelocationEncoding`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &RelocationEncoding) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## object::common::RelocationFlags

*Enum*

Relocation fields that are specific to each file format and architecture.

**Variants:**
- `Generic{ kind: RelocationKind, encoding: RelocationEncoding, size: u8 }` - Format independent representation.
- `Elf{ r_type: u32 }` - ELF relocation fields.
- `MachO{ r_type: u8, r_pcrel: bool, r_length: u8 }` - Mach-O relocation fields.
- `Coff{ typ: u16 }` - COFF relocation fields.
- `Xcoff{ r_rtype: u8, r_rsize: u8 }` - XCOFF relocation fields.

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &RelocationFlags) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> RelocationFlags`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::common::RelocationKind

*Enum*

The operation used to calculate the result of the relocation.

The relocation descriptions use the following definitions. Note that
these definitions probably don't match any ELF ABI.

* A - The value of the addend.
* G - The address of the symbol's entry within the global offset table.
* L - The address of the symbol's entry within the procedure linkage table.
* P - The address of the place of the relocation.
* S - The address of the symbol.
* GotBase - The address of the global offset table.
* Image - The base address of the image.
* Section - The address of the section containing the symbol.

'XxxRelative' means 'Xxx + A - P'.  'XxxOffset' means 'S + A - Xxx'.

**Variants:**
- `Unknown` - The operation is unknown.
- `None` - No relocation.
- `Absolute` - S + A
- `Relative` - S + A - P
- `Got` - G + A - GotBase
- `GotRelative` - G + A - P
- `GotBaseRelative` - GotBase + A - P
- `GotBaseOffset` - S + A - GotBase
- `PltRelative` - L + A - P
- `ImageOffset` - S + A - Image
- `SectionOffset` - S + A - Section
- `SectionIndex` - The index of the section containing the symbol.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &RelocationKind) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> RelocationKind`



## object::common::SectionFlags

*Enum*

Section flags that are specific to each file format.

**Variants:**
- `None` - No section flags.
- `Elf{ sh_flags: u64 }` - ELF section flags.
- `MachO{ flags: u32 }` - Mach-O section flags.
- `Coff{ characteristics: u32 }` - COFF section flags.
- `Xcoff{ s_flags: u32 }` - XCOFF section flags.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &SectionFlags) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> SectionFlags`



## object::common::SectionKind

*Enum*

The kind of a section.

**Variants:**
- `Unknown` - The section kind is unknown.
- `Text` - An executable code section.
- `Data` - A data section.
- `ReadOnlyData` - A read only data section.
- `ReadOnlyDataWithRel` - A read only data section with relocations.
- `ReadOnlyString` - A loadable string section.
- `UninitializedData` - An uninitialized data section.
- `Common` - An uninitialized common data section.
- `Tls` - A TLS data section.
- `UninitializedTls` - An uninitialized TLS data section.
- `TlsVariables` - A TLS variables section.
- `OtherString` - A non-loadable string section.
- `Other` - Some other non-loadable section.
- `Debug` - Debug information.
- `DebugString` - Debug strings.
- `Linker` - Information for the linker.
- `Note` - ELF note section.
- `Metadata` - Metadata such as symbols or relocations.
- `Elf(u32)` - Some other ELF section type.

**Methods:**

- `fn is_bss(self: Self) -> bool` - Return true if this section contains zerofill data.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &SectionKind) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> SectionKind`



## object::common::SegmentFlags

*Enum*

Segment flags that are specific to each file format.

**Variants:**
- `None` - No segment flags.
- `Elf{ p_flags: u32 }` - ELF segment flags.
- `MachO{ flags: u32, maxprot: u32, initprot: u32 }` - Mach-O segment flags.
- `Coff{ characteristics: u32 }` - COFF segment flags.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &SegmentFlags) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> SegmentFlags`



## object::common::SubArchitecture

*Enum*

A CPU sub-architecture.

**Variants:**
- `Arm64E`
- `Arm64EC`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &SubArchitecture) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> SubArchitecture`



## object::common::SymbolFlags

*Enum*

Symbol flags that are specific to each file format.

**Generic Parameters:**
- Section
- Symbol

**Variants:**
- `None` - No symbol flags.
- `Elf{ st_info: u8, st_other: u8 }` - ELF symbol flags.
- `MachO{ n_desc: u16 }` - Mach-O symbol flags.
- `CoffSection{ selection: u8, associative_section: Option<Section> }` - COFF flags for a section symbol.
- `Xcoff{ n_sclass: u8, x_smtyp: u8, x_smclas: u8, containing_csect: Option<Symbol> }` - XCOFF symbol flags.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &SymbolFlags<Section, Symbol>) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> SymbolFlags<Section, Symbol>`



## object::common::SymbolKind

*Enum*

The kind of a symbol.

**Variants:**
- `Unknown` - The symbol kind is unknown.
- `Text` - The symbol is for executable code.
- `Data` - The symbol is for a data object.
- `Section` - The symbol is for a section.
- `File` - The symbol is the name of a file. It precedes symbols within that file.
- `Label` - The symbol is for a code label.
- `Tls` - The symbol is for a thread local storage entity.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> SymbolKind`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &SymbolKind) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## object::common::SymbolScope

*Enum*

A symbol scope.

**Variants:**
- `Unknown` - Unknown scope.
- `Compilation` - Symbol is visible to the compilation unit.
- `Linkage` - Symbol is visible to the static linkage unit.
- `Dynamic` - Symbol is visible to dynamically linked objects.

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &SymbolScope) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> SymbolScope`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



