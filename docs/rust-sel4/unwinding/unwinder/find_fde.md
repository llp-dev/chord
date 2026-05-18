**unwinding > unwinder > find_fde**

# Module: unwinder::find_fde

## Contents

**Modules**

- [`custom`](#custom)
- [`custom_eh_frame_finder`](#custom_eh_frame_finder)

**Structs**

- [`FDESearchResult`](#fdesearchresult)
- [`GlobalFinder`](#globalfinder)

**Functions**

- [`get_finder`](#get_finder)

**Traits**

- [`FDEFinder`](#fdefinder)

---

## unwinding::unwinder::find_fde::FDEFinder

*Trait*

**Methods:**

- `find_fde`



## unwinding::unwinder::find_fde::FDESearchResult

*Struct*

**Fields:**
- `fde: gimli::FrameDescriptionEntry<gimli::EndianSlice<'static, gimli::NativeEndian>>`
- `bases: gimli::BaseAddresses`
- `eh_frame: gimli::EhFrame<gimli::EndianSlice<'static, gimli::NativeEndian>>`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## unwinding::unwinder::find_fde::GlobalFinder

*Struct*

**Tuple Struct**: `(())`

**Trait Implementations:**

- **FDEFinder**
  - `fn find_fde(self: &Self, pc: usize) -> Option<FDESearchResult>`



## Module: custom



## Module: custom_eh_frame_finder



## unwinding::unwinder::find_fde::get_finder

*Function*

```rust
fn get_finder() -> &'static GlobalFinder
```



