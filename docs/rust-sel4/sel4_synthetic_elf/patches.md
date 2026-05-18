**sel4_synthetic_elf > patches**

# Module: patches

## Contents

**Structs**

- [`Patches`](#patches)

**Enums**

- [`PatchesAddFromSymbolError`](#patchesaddfromsymbolerror)
- [`PatchesApplyError`](#patchesapplyerror)

**Traits**

- [`PatchValue`](#patchvalue)

---

## sel4_synthetic_elf::patches::PatchValue

*Trait*

**Methods:**

- `to_bytes`



## sel4_synthetic_elf::patches::Patches

*Struct*

**Methods:**

- `fn new() -> Self`
- `fn add_bytes(self: & mut Self, vaddr: u64, value: Vec<u8>)`
- `fn add<impl PatchValue, impl Endian>(self: & mut Self, vaddr: u64, value: impl Trait, endian: impl Trait)`
- `fn add_bytes_via_symbol<'data, T, R>(self: & mut Self, elf_file_for_symbols: &ElfFile<'data, T, R>, name: &str, value: Vec<u8>) -> Result<u64, PatchesAddFromSymbolError>`
- `fn add_via_symbol<'data, T, R, impl PatchValue, impl Endian>(self: & mut Self, elf_file_for_symbols: &ElfFile<'data, T, R>, name: &str, value: impl Trait, endian: impl Trait) -> Result<u64, PatchesAddFromSymbolError>`
- `fn apply(self: &Self, elf_file_data: & mut [u8]) -> Result<(), PatchesApplyError>`

**Trait Implementations:**

- **Default**
  - `fn default() -> Patches`



## sel4_synthetic_elf::patches::PatchesAddFromSymbolError

*Enum*

**Variants:**
- `ReadError(object::read::Error)`
- `SymbolMissing(String)`
- `SymbolSizeMismatch(String)`



## sel4_synthetic_elf::patches::PatchesApplyError

*Enum*

**Variants:**
- `ReadError(object::read::Error)`
- `AddrRangeNotMappedWithData(u64, usize)`



