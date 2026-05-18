**sel4_kernel_loader_embed_page_tables_runtime**

# Module: sel4_kernel_loader_embed_page_tables_runtime

## Contents

**Structs**

- [`A1024`](#a1024)
- [`A16384`](#a16384)
- [`A4096`](#a4096)
- [`Entry`](#entry)
- [`Table`](#table)
- [`TablePtr`](#tableptr)
- [`TablePtrs`](#tableptrs)

---

## sel4_kernel_loader_embed_page_tables_runtime::A1024

*Struct*

**Unit Struct**



## sel4_kernel_loader_embed_page_tables_runtime::A16384

*Struct*

**Unit Struct**



## sel4_kernel_loader_embed_page_tables_runtime::A4096

*Struct*

**Unit Struct**



## sel4_kernel_loader_embed_page_tables_runtime::Entry

*Struct*

**Tuple Struct**: `()`

**Methods:**

- `fn new(ptr: Option<*mut ()>, offset: usize) -> Self`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Entry`



## sel4_kernel_loader_embed_page_tables_runtime::Table

*Struct*

**Generic Parameters:**
- A
- const N

**Methods:**

- `fn new(entries: [Entry; N]) -> Self`
- `fn ptr(self: &Self) -> TablePtr`

**Traits:** Sync



## sel4_kernel_loader_embed_page_tables_runtime::TablePtr

*Struct*

**Methods:**

- `fn new(ptr: *mut [Entry]) -> Self`
- `fn value(self: &Self) -> *mut ()`

**Traits:** Sync, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> TablePtr`



## sel4_kernel_loader_embed_page_tables_runtime::TablePtrs

*Struct*

**Generic Parameters:**
- 'a

**Methods:**

- `fn new(ptrs: &'a [TablePtr]) -> Self`
- `fn table(self: &Self, index: usize) -> TablePtr`
- `fn root(self: &Self) -> TablePtr`
- `fn finish_for_riscv(self: &Self)`



