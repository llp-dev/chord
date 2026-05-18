**sel4_elf_header**

# Module: sel4_elf_header

## Contents

**Structs**

- [`ElfHeader`](#elfheader)
- [`ElfHeaderIdent`](#elfheaderident)
- [`ProgramHeader`](#programheader)

**Constants**

- [`ELFMAG`](#elfmag)
- [`PT_GNU_EH_FRAME`](#pt_gnu_eh_frame)
- [`PT_LOAD`](#pt_load)
- [`PT_NULL`](#pt_null)
- [`PT_TLS`](#pt_tls)

---

## sel4_elf_header::ELFMAG

*Constant*: `[u8; 4]`



## sel4_elf_header::ElfHeader

*Struct*

**Fields:**
- `e_ident: ElfHeaderIdent`
- `e_type: u16`
- `e_machine: u16`
- `e_version: u32`
- `e_entry: usize`
- `e_phoff: usize`
- `e_shoff: usize`
- `e_flags: u32`
- `e_ehsize: u16`
- `e_phentsize: u16`
- `e_phnum: u16`
- `e_shentsize: u16`
- `e_shnum: u16`
- `e_shstrndx: u16`

**Methods:**

- `fn is_magic_valid(self: &Self) -> bool`
- `fn locate_phdrs(self: &'static Self) -> &'static [ProgramHeader]`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ElfHeader`
- **Default**
  - `fn default() -> ElfHeader`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &ElfHeader) -> bool`



## sel4_elf_header::ElfHeaderIdent

*Struct*

**Fields:**
- `magic: [u8; 4]`
- `class: u8`
- `data: u8`
- `version: u8`
- `os_abi: u8`
- `abi_version: u8`
- `padding: [u8; 7]`

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &ElfHeaderIdent) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> ElfHeaderIdent`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> ElfHeaderIdent`



## sel4_elf_header::PT_GNU_EH_FRAME

*Constant*: `u32`



## sel4_elf_header::PT_LOAD

*Constant*: `u32`



## sel4_elf_header::PT_NULL

*Constant*: `u32`



## sel4_elf_header::PT_TLS

*Constant*: `u32`



## sel4_elf_header::ProgramHeader

*Struct*

**Fields:**
- `p_type: u32`
- `p_flags: u32`
- `p_offset: usize`
- `p_vaddr: usize`
- `p_paddr: usize`
- `p_filesz: usize`
- `p_memsz: usize`
- `p_align: usize`

**Methods:**

- `fn vaddr_range(self: &Self) -> Range<usize>`

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &ProgramHeader) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> ProgramHeader`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> ProgramHeader`



