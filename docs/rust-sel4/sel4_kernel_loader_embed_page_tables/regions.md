**sel4_kernel_loader_embed_page_tables > regions**

# Module: regions

## Contents

**Structs**

- [`AbstractRegion`](#abstractregion)
- [`AbstractRegions`](#abstractregions)
- [`AbstractRegionsBuilder`](#abstractregionsbuilder)

---

## sel4_kernel_loader_embed_page_tables::regions::AbstractRegion

*Struct*

**Generic Parameters:**
- T

**Methods:**

- `fn new(range: Range<u64>, content: T) -> Self`
- `fn valid<impl MkLeafFn<T> + 'static>(range: Range<u64>, mk_leaf: impl Trait) -> Self`
- `fn invalid(range: Range<u64>) -> Self`

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &AbstractRegion<T>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> AbstractRegion<T>`



## sel4_kernel_loader_embed_page_tables::regions::AbstractRegions

*Struct*

**Generic Parameters:**
- T

**Methods:**

- `fn as_slice(self: &Self) -> &[AbstractRegion<Arc<T>>]`
- `fn bounds(self: &Self) -> Range<u64>`
- `fn construct_table(self: &Self) -> Table<T>`

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &AbstractRegions<T>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> AbstractRegions<T>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_kernel_loader_embed_page_tables::regions::AbstractRegionsBuilder

*Struct*

**Generic Parameters:**
- T

**Methods:**

- `fn build(self: Self) -> AbstractRegions<T>`
- `fn insert(self: Self, region: AbstractRegion<T>) -> Self`
- `fn new_with_background(background: AbstractRegion<T>) -> Self`
- `fn new() -> Self`

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &AbstractRegionsBuilder<T>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> AbstractRegionsBuilder<T>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



