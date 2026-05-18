**sel4_kernel_loader_embed_page_tables > table**

# Module: table

## Contents

**Structs**

- [`LeafLocation`](#leaflocation)
- [`RegionContent`](#regioncontent)
- [`Table`](#table)

**Traits**

- [`MkLeafFn`](#mkleaffn)

---

## sel4_kernel_loader_embed_page_tables::table::LeafLocation

*Struct*

**Methods:**

- `fn level(self: &Self) -> usize`
- `fn vaddr(self: &Self) -> u64`
- `fn map<T, impl FnOnce(u64) -> u64>(self: &Self, vaddr_to_paddr: impl Trait) -> <T as >::LeafDescriptor`
- `fn map_identity<T>(self: &Self) -> <T as >::LeafDescriptor`



## sel4_kernel_loader_embed_page_tables::table::MkLeafFn

*Trait*



## sel4_kernel_loader_embed_page_tables::table::RegionContent

*Struct*

**Generic Parameters:**
- T



## sel4_kernel_loader_embed_page_tables::table::Table

*Struct*

**Generic Parameters:**
- T

**Methods:**

- `fn construct(regions: &AbstractRegions<Option<RegionContent<T>>>) -> Self`
- `fn embed(self: &Self, symbol_ident: Ident, runtime_mod_ident: Ident) -> TokenStream`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



