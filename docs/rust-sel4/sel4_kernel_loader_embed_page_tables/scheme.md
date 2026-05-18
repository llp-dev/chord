**sel4_kernel_loader_embed_page_tables > scheme**

# Module: scheme

## Contents

**Structs**

- [`AArch32LeafDescriptor`](#aarch32leafdescriptor)
- [`AArch64LeafDescriptor`](#aarch64leafdescriptor)
- [`RiscV32Sv32LeafDescriptor`](#riscv32sv32leafdescriptor)
- [`RiscV64Sv39LeafDescriptor`](#riscv64sv39leafdescriptor)
- [`SchemeHelpers`](#schemehelpers)

**Enums**

- [`AArch32`](#aarch32)
- [`AArch64`](#aarch64)
- [`RiscV32Sv32`](#riscv32sv32)
- [`RiscV64Sv39`](#riscv64sv39)

**Traits**

- [`Scheme`](#scheme)
- [`SchemeLeafDescriptor`](#schemeleafdescriptor)

---

## sel4_kernel_loader_embed_page_tables::scheme::AArch32

*Enum*

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Scheme**
  - `fn level_bits(level: usize) -> usize`



## sel4_kernel_loader_embed_page_tables::scheme::AArch32LeafDescriptor

*Struct*



## sel4_kernel_loader_embed_page_tables::scheme::AArch64

*Enum*

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Scheme**
  - `fn level_bits(_level: usize) -> usize`



## sel4_kernel_loader_embed_page_tables::scheme::AArch64LeafDescriptor

*Struct*

**Tuple Struct**: `()`



## sel4_kernel_loader_embed_page_tables::scheme::RiscV32Sv32

*Enum*

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Scheme**
  - `fn level_bits(_level: usize) -> usize`



## sel4_kernel_loader_embed_page_tables::scheme::RiscV32Sv32LeafDescriptor

*Struct*

**Tuple Struct**: `()`



## sel4_kernel_loader_embed_page_tables::scheme::RiscV64Sv39

*Enum*

**Trait Implementations:**

- **Scheme**
  - `fn level_bits(_level: usize) -> usize`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_kernel_loader_embed_page_tables::scheme::RiscV64Sv39LeafDescriptor

*Struct*

**Tuple Struct**: `()`



## sel4_kernel_loader_embed_page_tables::scheme::Scheme

*Trait*

**Methods:**

- `WordPrimitive`
- `PAGE_BITS`
- `NUM_LEVELS`
- `level_bits`
- `level_align_bits`
- `MIN_LEVEL_FOR_LEAF`
- `LeafDescriptor`
- `EMPTY_DESCRIPTOR`
- `SYMBOLIC_BRANCH_DESCRIPTOR_OFFSET`
- `RUNTIME_SCHEME_IDENT`



## sel4_kernel_loader_embed_page_tables::scheme::SchemeHelpers

*Struct*

**Generic Parameters:**
- T

**Tuple Struct**: `()`

**Methods:**

- `fn word_bits() -> usize`
- `fn num_entries_in_table(level: usize) -> usize`
- `fn vaddr_bits() -> usize`
- `fn vaddr_mask() -> u64`
- `fn virt_bounds() -> Range<u64>`
- `fn largest_leaf_size_bits() -> usize`



## sel4_kernel_loader_embed_page_tables::scheme::SchemeLeafDescriptor

*Trait*

**Methods:**

- `from_paddr`
- `to_raw`



