# Crate `sel4_elf_header`

## Contents

- [Structs](#structs)
  - [`ElfHeader`](#elfheader)
  - [`ElfHeaderIdent`](#elfheaderident)
  - [`ProgramHeader`](#programheader)
- [Constants](#constants)
  - [`ELFMAG`](#elfmag)
  - [`PT_NULL`](#pt-null)
  - [`PT_LOAD`](#pt-load)
  - [`PT_TLS`](#pt-tls)
  - [`PT_GNU_EH_FRAME`](#pt-gnu-eh-frame)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ElfHeader`](#elfheader) | struct |  |
| [`ElfHeaderIdent`](#elfheaderident) | struct |  |
| [`ProgramHeader`](#programheader) | struct |  |
| [`ELFMAG`](#elfmag) | const |  |
| [`PT_NULL`](#pt-null) | const |  |
| [`PT_LOAD`](#pt-load) | const |  |
| [`PT_TLS`](#pt-tls) | const |  |
| [`PT_GNU_EH_FRAME`](#pt-gnu-eh-frame) | const |  |

## Structs

### `ElfHeader`

```rust
struct ElfHeader {
    pub e_ident: ElfHeaderIdent,
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: usize,
    pub e_phoff: usize,
    pub e_shoff: usize,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}
```

#### Implementations

- <span id="elfheader-is-magic-valid"></span>`fn is_magic_valid(&self) -> bool`

- <span id="elfheader-locate-phdrs"></span>`fn locate_phdrs(self: &'static Self) -> &'static [ProgramHeader]` — [`ProgramHeader`](#programheader)

#### Trait Implementations

##### `impl Clone for ElfHeader`

- <span id="elfheader-clone"></span>`fn clone(&self) -> ElfHeader` — [`ElfHeader`](#elfheader)

##### `impl Copy for ElfHeader`

##### `impl Debug for ElfHeader`

- <span id="elfheader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ElfHeader`

- <span id="elfheader-default"></span>`fn default() -> ElfHeader` — [`ElfHeader`](#elfheader)

##### `impl Eq for ElfHeader`

##### `impl PartialEq for ElfHeader`

- <span id="elfheader-partialeq-eq"></span>`fn eq(&self, other: &ElfHeader) -> bool` — [`ElfHeader`](#elfheader)

##### `impl StructuralPartialEq for ElfHeader`

### `ElfHeaderIdent`

```rust
struct ElfHeaderIdent {
    pub magic: [u8; 4],
    pub class: u8,
    pub data: u8,
    pub version: u8,
    pub os_abi: u8,
    pub abi_version: u8,
    pub padding: [u8; 7],
}
```

#### Trait Implementations

##### `impl Clone for ElfHeaderIdent`

- <span id="elfheaderident-clone"></span>`fn clone(&self) -> ElfHeaderIdent` — [`ElfHeaderIdent`](#elfheaderident)

##### `impl Copy for ElfHeaderIdent`

##### `impl Debug for ElfHeaderIdent`

- <span id="elfheaderident-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ElfHeaderIdent`

- <span id="elfheaderident-default"></span>`fn default() -> ElfHeaderIdent` — [`ElfHeaderIdent`](#elfheaderident)

##### `impl Eq for ElfHeaderIdent`

##### `impl PartialEq for ElfHeaderIdent`

- <span id="elfheaderident-partialeq-eq"></span>`fn eq(&self, other: &ElfHeaderIdent) -> bool` — [`ElfHeaderIdent`](#elfheaderident)

##### `impl StructuralPartialEq for ElfHeaderIdent`

### `ProgramHeader`

```rust
struct ProgramHeader {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: usize,
    pub p_vaddr: usize,
    pub p_paddr: usize,
    pub p_filesz: usize,
    pub p_memsz: usize,
    pub p_align: usize,
}
```

#### Implementations

- <span id="programheader-vaddr-range"></span>`fn vaddr_range(&self) -> Range<usize>`

#### Trait Implementations

##### `impl Clone for ProgramHeader`

- <span id="programheader-clone"></span>`fn clone(&self) -> ProgramHeader` — [`ProgramHeader`](#programheader)

##### `impl Copy for ProgramHeader`

##### `impl Debug for ProgramHeader`

- <span id="programheader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ProgramHeader`

- <span id="programheader-default"></span>`fn default() -> ProgramHeader` — [`ProgramHeader`](#programheader)

##### `impl Eq for ProgramHeader`

##### `impl PartialEq for ProgramHeader`

- <span id="programheader-partialeq-eq"></span>`fn eq(&self, other: &ProgramHeader) -> bool` — [`ProgramHeader`](#programheader)

##### `impl StructuralPartialEq for ProgramHeader`

## Constants

### `ELFMAG`
```rust
const ELFMAG: [u8; 4];
```

### `PT_NULL`
```rust
const PT_NULL: u32 = 0u32;
```

### `PT_LOAD`
```rust
const PT_LOAD: u32 = 1u32;
```

### `PT_TLS`
```rust
const PT_TLS: u32 = 7u32;
```

### `PT_GNU_EH_FRAME`
```rust
const PT_GNU_EH_FRAME: u32 = 1_685_382_480u32;
```

