*[gimli](../../index.md) / [read](../index.md) / [dwarf](index.md)*

---

# Module `dwarf`

## Contents

- [Structs](#structs)
  - [`DwarfSections`](#dwarfsections)
  - [`Dwarf`](#dwarf)
  - [`DwarfPackageSections`](#dwarfpackagesections)
  - [`DwarfPackage`](#dwarfpackage)
  - [`Unit`](#unit)
  - [`UnitRef`](#unitref)
  - [`RangeIter`](#rangeiter)
- [Enums](#enums)
  - [`RangeIterInner`](#rangeiterinner)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DwarfSections`](#dwarfsections) | struct | All of the commonly used DWARF sections. |
| [`Dwarf`](#dwarf) | struct | All of the commonly used DWARF sections, and other common information. |
| [`DwarfPackageSections`](#dwarfpackagesections) | struct | The sections from a `.dwp` file. |
| [`DwarfPackage`](#dwarfpackage) | struct | The sections from a `.dwp` file, with parsed indices. |
| [`Unit`](#unit) | struct | All of the commonly used information for a unit in the `.debug_info` or `.debug_types` sections. |
| [`UnitRef`](#unitref) | struct | A reference to a `Unit` and its associated `Dwarf`. |
| [`RangeIter`](#rangeiter) | struct | An iterator for the address ranges of a `DebuggingInformationEntry`. |
| [`RangeIterInner`](#rangeiterinner) | enum |  |

## Structs

### `DwarfSections<T>`

```rust
struct DwarfSections<T> {
    pub debug_abbrev: crate::read::DebugAbbrev<T>,
    pub debug_addr: crate::read::DebugAddr<T>,
    pub debug_aranges: crate::read::DebugAranges<T>,
    pub debug_info: crate::read::DebugInfo<T>,
    pub debug_line: crate::read::DebugLine<T>,
    pub debug_line_str: crate::read::DebugLineStr<T>,
    pub debug_macinfo: crate::read::DebugMacinfo<T>,
    pub debug_macro: crate::read::DebugMacro<T>,
    pub debug_names: crate::read::DebugNames<T>,
    pub debug_str: crate::read::DebugStr<T>,
    pub debug_str_offsets: crate::read::DebugStrOffsets<T>,
    pub debug_types: crate::read::DebugTypes<T>,
    pub debug_loc: crate::read::DebugLoc<T>,
    pub debug_loclists: crate::read::DebugLocLists<T>,
    pub debug_ranges: crate::read::DebugRanges<T>,
    pub debug_rnglists: crate::read::DebugRngLists<T>,
}
```

All of the commonly used DWARF sections.

This is useful for storing sections when `T` does not implement `Reader`.
It can be used to create a `Dwarf` that references the data in `self`.
If `T` does implement `Reader`, then use `Dwarf` directly.

## Example Usage

It can be useful to load DWARF sections into owned data structures,
such as `Vec`. However, we do not implement the `Reader` trait
for `Vec`, because it would be very inefficient, but this trait
is required for all of the methods that parse the DWARF data.
So we first load the DWARF sections into `Vec`s, and then use
`borrow` to create `Reader`s that reference the data.

```rust,no_run
fn example() -> Result<(), gimli::Error> {
let loader = |name| -> Result<_, gimli::Error> { unimplemented!() };
// Read the DWARF sections into `Vec`s with whatever object loader you're using.
let dwarf_sections: gimli::DwarfSections<Vec<u8>> = gimli::DwarfSections::load(loader)?;
// Create references to the DWARF sections.
let dwarf: gimli::Dwarf<_> = dwarf_sections.borrow(|section| {
    gimli::EndianSlice::new(&section, gimli::LittleEndian)
});
unreachable!()
}
```

#### Fields

- **`debug_abbrev`**: `crate::read::DebugAbbrev<T>`

  The `.debug_abbrev` section.

- **`debug_addr`**: `crate::read::DebugAddr<T>`

  The `.debug_addr` section.

- **`debug_aranges`**: `crate::read::DebugAranges<T>`

  The `.debug_aranges` section.

- **`debug_info`**: `crate::read::DebugInfo<T>`

  The `.debug_info` section.

- **`debug_line`**: `crate::read::DebugLine<T>`

  The `.debug_line` section.

- **`debug_line_str`**: `crate::read::DebugLineStr<T>`

  The `.debug_line_str` section.

- **`debug_macinfo`**: `crate::read::DebugMacinfo<T>`

  The `.debug_macinfo` section.

- **`debug_macro`**: `crate::read::DebugMacro<T>`

  The `.debug_macro` section.

- **`debug_names`**: `crate::read::DebugNames<T>`

  The `.debug_names` section.

- **`debug_str`**: `crate::read::DebugStr<T>`

  The `.debug_str` section.

- **`debug_str_offsets`**: `crate::read::DebugStrOffsets<T>`

  The `.debug_str_offsets` section.

- **`debug_types`**: `crate::read::DebugTypes<T>`

  The `.debug_types` section.

- **`debug_loc`**: `crate::read::DebugLoc<T>`

  The `.debug_loc` section.

- **`debug_loclists`**: `crate::read::DebugLocLists<T>`

  The `.debug_loclists` section.

- **`debug_ranges`**: `crate::read::DebugRanges<T>`

  The `.debug_ranges` section.

- **`debug_rnglists`**: `crate::read::DebugRngLists<T>`

  The `.debug_rnglists` section.

#### Implementations

- <span id="dwarfsections-load"></span>`fn load<F, E>(section: F) -> core::result::Result<Self, E>`

  Try to load the DWARF sections using the given loader function.

  

  `section` loads a DWARF section from the object file.

  It should return an empty section if the section does not exist.

- <span id="dwarfsections-borrow"></span>`fn borrow<'a, F, R>(self: &'a Self, borrow: F) -> Dwarf<R>` — [`Dwarf`](../index.md#dwarf)

  Create a `Dwarf` structure that references the data in `self`.

- <span id="dwarfsections-borrow-with-sup"></span>`fn borrow_with_sup<'a, F, R>(self: &'a Self, sup: Option<&'a Self>, borrow: F) -> Dwarf<R>` — [`Dwarf`](../index.md#dwarf)

  Create a `Dwarf` structure that references the data in `self` and `sup`.

  

  This is like `borrow`, but also includes the supplementary object file.

  This is useful when `R` implements `Reader` but `T` does not.

  

  ## Example Usage

  

  ```rust,no_run

  fn example() -> Result<(), gimli::Error> {

  let loader = |name| -> Result<_, gimli::Error> { unimplemented!() };

  let sup_loader = |name| -> Result<_, gimli::Error> { unimplemented!() };

  // Read the DWARF sections into `Vec`s with whatever object loader you're using.

  let dwarf_sections: gimli::DwarfSections<Vec<u8>> = gimli::DwarfSections::load(loader)?;

  let dwarf_sup_sections: gimli::DwarfSections<Vec<u8>> = gimli::DwarfSections::load(sup_loader)?;

  // Create references to the DWARF sections.

  let dwarf = dwarf_sections.borrow_with_sup(Some(&dwarf_sup_sections), |section| {

      gimli::EndianSlice::new(&section, gimli::LittleEndian)

  });

  unreachable!()

  }

  ```

#### Trait Implementations

##### `impl<T: fmt::Debug> Debug for DwarfSections<T>`

- <span id="dwarfsections-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: default::Default> Default for DwarfSections<T>`

- <span id="dwarfsections-default"></span>`fn default() -> DwarfSections<T>` — [`DwarfSections`](../index.md#dwarfsections)

### `Dwarf<R>`

```rust
struct Dwarf<R> {
    pub debug_abbrev: crate::read::DebugAbbrev<R>,
    pub debug_addr: crate::read::DebugAddr<R>,
    pub debug_aranges: crate::read::DebugAranges<R>,
    pub debug_info: crate::read::DebugInfo<R>,
    pub debug_line: crate::read::DebugLine<R>,
    pub debug_line_str: crate::read::DebugLineStr<R>,
    pub debug_macinfo: crate::read::DebugMacinfo<R>,
    pub debug_macro: crate::read::DebugMacro<R>,
    pub debug_names: crate::read::DebugNames<R>,
    pub debug_str: crate::read::DebugStr<R>,
    pub debug_str_offsets: crate::read::DebugStrOffsets<R>,
    pub debug_types: crate::read::DebugTypes<R>,
    pub locations: crate::read::LocationLists<R>,
    pub ranges: crate::read::RangeLists<R>,
    pub file_type: crate::common::DwarfFileType,
    pub sup: Option<alloc::sync::Arc<Dwarf<R>>>,
    pub abbreviations_cache: crate::read::AbbreviationsCache,
}
```

All of the commonly used DWARF sections, and other common information.

#### Fields

- **`debug_abbrev`**: `crate::read::DebugAbbrev<R>`

  The `.debug_abbrev` section.

- **`debug_addr`**: `crate::read::DebugAddr<R>`

  The `.debug_addr` section.

- **`debug_aranges`**: `crate::read::DebugAranges<R>`

  The `.debug_aranges` section.

- **`debug_info`**: `crate::read::DebugInfo<R>`

  The `.debug_info` section.

- **`debug_line`**: `crate::read::DebugLine<R>`

  The `.debug_line` section.

- **`debug_line_str`**: `crate::read::DebugLineStr<R>`

  The `.debug_line_str` section.

- **`debug_macinfo`**: `crate::read::DebugMacinfo<R>`

  The `.debug_macinfo` section.

- **`debug_macro`**: `crate::read::DebugMacro<R>`

  The `.debug_macro` section.

- **`debug_names`**: `crate::read::DebugNames<R>`

  The `.debug_names` section.

- **`debug_str`**: `crate::read::DebugStr<R>`

  The `.debug_str` section.

- **`debug_str_offsets`**: `crate::read::DebugStrOffsets<R>`

  The `.debug_str_offsets` section.

- **`debug_types`**: `crate::read::DebugTypes<R>`

  The `.debug_types` section.

- **`locations`**: `crate::read::LocationLists<R>`

  The location lists in the `.debug_loc` and `.debug_loclists` sections.

- **`ranges`**: `crate::read::RangeLists<R>`

  The range lists in the `.debug_ranges` and `.debug_rnglists` sections.

- **`file_type`**: `crate::common::DwarfFileType`

  The type of this file.

- **`sup`**: `Option<alloc::sync::Arc<Dwarf<R>>>`

  The DWARF sections for a supplementary object file.

- **`abbreviations_cache`**: `crate::read::AbbreviationsCache`

  A cache of previously parsed abbreviations for units in this file.

#### Implementations

- <span id="dwarf-load"></span>`fn load<F, E>(section: F) -> core::result::Result<Self, E>`

  Try to load the DWARF sections using the given loader function.

  

  `section` loads a DWARF section from the object file.

  It should return an empty section if the section does not exist.

  

  After loading, the user should set the `file_type` field and

  call `load_sup` if required.

- <span id="dwarf-load-sup"></span>`fn load_sup<F, E>(&mut self, section: F) -> core::result::Result<(), E>`

  Load the DWARF sections from the supplementary object file.

  

  `section` operates the same as for `load`.

  

  Sets `self.sup`, replacing any previous value.

- <span id="dwarf-from-sections"></span>`fn from_sections(sections: DwarfSections<T>) -> Self` — [`DwarfSections`](../index.md#dwarfsections)

  Create a `Dwarf` structure from the given sections.

  

  The caller should set the `file_type` and `sup` fields if required.

- <span id="dwarf-borrow"></span>`fn borrow<'a, F, R>(self: &'a Self, borrow: F) -> Dwarf<R>` — [`Dwarf`](../index.md#dwarf)

  Create a `Dwarf` structure that references the data in `self`.

  

  This is useful when `R` implements `Reader` but `T` does not.

  

  ## Example Usage

  

  It can be useful to load DWARF sections into owned data structures,

  such as `Vec`. However, we do not implement the `Reader` trait

  for `Vec`, because it would be very inefficient, but this trait

  is required for all of the methods that parse the DWARF data.

  So we first load the DWARF sections into `Vec`s, and then use

  `borrow` to create `Reader`s that reference the data.

  

  ```rust,no_run

  fn example() -> Result<(), gimli::Error> {

  let loader = |name| -> Result<_, gimli::Error> { unimplemented!() };

  let sup_loader = |name| -> Result<_, gimli::Error> { unimplemented!() };

  // Read the DWARF sections into `Vec`s with whatever object loader you're using.

  let mut owned_dwarf: gimli::Dwarf<Vec<u8>> = gimli::Dwarf::load(loader)?;

  owned_dwarf.load_sup(sup_loader)?;

  // Create references to the DWARF sections.

  let dwarf = owned_dwarf.borrow(|section| {

      gimli::EndianSlice::new(&section, gimli::LittleEndian)

  });

  unreachable!()

  }

  ```

- <span id="dwarf-set-sup"></span>`fn set_sup(&mut self, sup: Dwarf<T>)` — [`Dwarf`](../index.md#dwarf)

  Store the DWARF sections for the supplementary object file.

- <span id="dwarf-sup"></span>`fn sup(&self) -> Option<&Dwarf<T>>` — [`Dwarf`](../index.md#dwarf)

  Return a reference to the DWARF sections for the supplementary object file.

#### Trait Implementations

##### `impl<R: fmt::Debug> Debug for Dwarf<R>`

- <span id="dwarf-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for Dwarf<R>`

- <span id="dwarf-default"></span>`fn default() -> Dwarf<R>` — [`Dwarf`](../index.md#dwarf)

### `DwarfPackageSections<T>`

```rust
struct DwarfPackageSections<T> {
    pub cu_index: crate::read::DebugCuIndex<T>,
    pub tu_index: crate::read::DebugTuIndex<T>,
    pub debug_abbrev: crate::read::DebugAbbrev<T>,
    pub debug_info: crate::read::DebugInfo<T>,
    pub debug_line: crate::read::DebugLine<T>,
    pub debug_macinfo: crate::read::DebugMacinfo<T>,
    pub debug_macro: crate::read::DebugMacro<T>,
    pub debug_str: crate::read::DebugStr<T>,
    pub debug_str_offsets: crate::read::DebugStrOffsets<T>,
    pub debug_loc: crate::read::DebugLoc<T>,
    pub debug_loclists: crate::read::DebugLocLists<T>,
    pub debug_rnglists: crate::read::DebugRngLists<T>,
    pub debug_types: crate::read::DebugTypes<T>,
}
```

The sections from a `.dwp` file.

This is useful for storing sections when `T` does not implement `Reader`.
It can be used to create a `DwarfPackage` that references the data in `self`.
If `T` does implement `Reader`, then use `DwarfPackage` directly.

## Example Usage

It can be useful to load DWARF sections into owned data structures,
such as `Vec`. However, we do not implement the `Reader` trait
for `Vec`, because it would be very inefficient, but this trait
is required for all of the methods that parse the DWARF data.
So we first load the DWARF sections into `Vec`s, and then use
`borrow` to create `Reader`s that reference the data.

```rust,no_run
fn example() -> Result<(), gimli::Error> {
let loader = |name| -> Result<_, gimli::Error> { unimplemented!() };
// Read the DWARF sections into `Vec`s with whatever object loader you're using.
let dwp_sections: gimli::DwarfPackageSections<Vec<u8>> = gimli::DwarfPackageSections::load(loader)?;
// Create references to the DWARF sections.
let dwp: gimli::DwarfPackage<_> = dwp_sections.borrow(
    |section| gimli::EndianSlice::new(&section, gimli::LittleEndian),
    gimli::EndianSlice::new(&[], gimli::LittleEndian),
)?;
unreachable!()
}
```

#### Fields

- **`cu_index`**: `crate::read::DebugCuIndex<T>`

  The `.debug_cu_index` section.

- **`tu_index`**: `crate::read::DebugTuIndex<T>`

  The `.debug_tu_index` section.

- **`debug_abbrev`**: `crate::read::DebugAbbrev<T>`

  The `.debug_abbrev.dwo` section.

- **`debug_info`**: `crate::read::DebugInfo<T>`

  The `.debug_info.dwo` section.

- **`debug_line`**: `crate::read::DebugLine<T>`

  The `.debug_line.dwo` section.

- **`debug_macinfo`**: `crate::read::DebugMacinfo<T>`

  The `.debug_macinfo.dwo` section.
  
  Only present when using GNU split-dwarf extension to DWARF 4.

- **`debug_macro`**: `crate::read::DebugMacro<T>`

  The `.debug_macro.dwo` section.

- **`debug_str`**: `crate::read::DebugStr<T>`

  The `.debug_str.dwo` section.

- **`debug_str_offsets`**: `crate::read::DebugStrOffsets<T>`

  The `.debug_str_offsets.dwo` section.

- **`debug_loc`**: `crate::read::DebugLoc<T>`

  The `.debug_loc.dwo` section.
  
  Only present when using GNU split-dwarf extension to DWARF 4.

- **`debug_loclists`**: `crate::read::DebugLocLists<T>`

  The `.debug_loclists.dwo` section.

- **`debug_rnglists`**: `crate::read::DebugRngLists<T>`

  The `.debug_rnglists.dwo` section.

- **`debug_types`**: `crate::read::DebugTypes<T>`

  The `.debug_types.dwo` section.
  
  Only present when using GNU split-dwarf extension to DWARF 4.

#### Implementations

- <span id="dwarfpackagesections-load"></span>`fn load<F, E>(section: F) -> core::result::Result<Self, E>`

  Try to load the `.dwp` sections using the given loader function.

  

  `section` loads a DWARF section from the object file.

  It should return an empty section if the section does not exist.

- <span id="dwarfpackagesections-borrow"></span>`fn borrow<'a, F, R>(self: &'a Self, borrow: F, empty: R) -> Result<DwarfPackage<R>>` — [`Result`](../../index.md#result), [`DwarfPackage`](../index.md#dwarfpackage)

  Create a `DwarfPackage` structure that references the data in `self`.

#### Trait Implementations

##### `impl<T: fmt::Debug> Debug for DwarfPackageSections<T>`

- <span id="dwarfpackagesections-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: default::Default> Default for DwarfPackageSections<T>`

- <span id="dwarfpackagesections-default"></span>`fn default() -> DwarfPackageSections<T>` — [`DwarfPackageSections`](../index.md#dwarfpackagesections)

### `DwarfPackage<R: Reader>`

```rust
struct DwarfPackage<R: Reader> {
    pub cu_index: crate::read::UnitIndex<R>,
    pub tu_index: crate::read::UnitIndex<R>,
    pub debug_abbrev: crate::read::DebugAbbrev<R>,
    pub debug_info: crate::read::DebugInfo<R>,
    pub debug_line: crate::read::DebugLine<R>,
    pub debug_macinfo: crate::read::DebugMacinfo<R>,
    pub debug_macro: crate::read::DebugMacro<R>,
    pub debug_str: crate::read::DebugStr<R>,
    pub debug_str_offsets: crate::read::DebugStrOffsets<R>,
    pub debug_loc: crate::read::DebugLoc<R>,
    pub debug_loclists: crate::read::DebugLocLists<R>,
    pub debug_rnglists: crate::read::DebugRngLists<R>,
    pub debug_types: crate::read::DebugTypes<R>,
    pub empty: R,
}
```

The sections from a `.dwp` file, with parsed indices.

#### Fields

- **`cu_index`**: `crate::read::UnitIndex<R>`

  The compilation unit index in the `.debug_cu_index` section.

- **`tu_index`**: `crate::read::UnitIndex<R>`

  The type unit index in the `.debug_tu_index` section.

- **`debug_abbrev`**: `crate::read::DebugAbbrev<R>`

  The `.debug_abbrev.dwo` section.

- **`debug_info`**: `crate::read::DebugInfo<R>`

  The `.debug_info.dwo` section.

- **`debug_line`**: `crate::read::DebugLine<R>`

  The `.debug_line.dwo` section.

- **`debug_macinfo`**: `crate::read::DebugMacinfo<R>`

  The `.debug_macinfo.dwo` section.

- **`debug_macro`**: `crate::read::DebugMacro<R>`

  The `.debug_macro.dwo` section.

- **`debug_str`**: `crate::read::DebugStr<R>`

  The `.debug_str.dwo` section.

- **`debug_str_offsets`**: `crate::read::DebugStrOffsets<R>`

  The `.debug_str_offsets.dwo` section.

- **`debug_loc`**: `crate::read::DebugLoc<R>`

  The `.debug_loc.dwo` section.
  
  Only present when using GNU split-dwarf extension to DWARF 4.

- **`debug_loclists`**: `crate::read::DebugLocLists<R>`

  The `.debug_loclists.dwo` section.

- **`debug_rnglists`**: `crate::read::DebugRngLists<R>`

  The `.debug_rnglists.dwo` section.

- **`debug_types`**: `crate::read::DebugTypes<R>`

  The `.debug_types.dwo` section.
  
  Only present when using GNU split-dwarf extension to DWARF 4.

- **`empty`**: `R`

  An empty section.
  
  Used when creating `Dwarf<R>`.

#### Implementations

- <span id="dwarfpackage-load"></span>`fn load<F, E>(section: F, empty: R) -> core::result::Result<Self, E>`

  Try to load the `.dwp` sections using the given loader function.

  

  `section` loads a DWARF section from the object file.

  It should return an empty section if the section does not exist.

- <span id="dwarfpackage-from-sections"></span>`fn from_sections(sections: DwarfPackageSections<R>, empty: R) -> Result<Self>` — [`DwarfPackageSections`](../index.md#dwarfpackagesections), [`Result`](../../index.md#result)

  Create a `DwarfPackage` structure from the given sections.

- <span id="dwarfpackage-find-cu"></span>`fn find_cu(&self, id: DwoId, parent: &Dwarf<R>) -> Result<Option<Dwarf<R>>>` — [`DwoId`](../../index.md#dwoid), [`Dwarf`](../index.md#dwarf), [`Result`](../../index.md#result)

  Find the compilation unit with the given DWO identifier and return its section

  contributions.

  

  ## Example Usage

  

  ```rust,no_run

  fn example<R: gimli::Reader>(

         dwarf: &gimli::Dwarf<R>,

         dwp: &gimli::DwarfPackage<R>,

         dwo_id: gimli::DwoId,

  ) -> Result<(), gimli::Error> {

  if let Some(dwo) = dwp.find_cu(dwo_id, dwarf)? {

     let dwo_header = dwo.units().next()?.expect("DWO should have one unit");

     let dwo_unit = dwo.unit(dwo_header)?;

     // Do something with `dwo_unit`.

  }

  unreachable!()

  }

- <span id="dwarfpackage-find-tu"></span>`fn find_tu(&self, signature: DebugTypeSignature, parent: &Dwarf<R>) -> Result<Option<Dwarf<R>>>` — [`DebugTypeSignature`](../../index.md#debugtypesignature), [`Dwarf`](../index.md#dwarf), [`Result`](../../index.md#result)

  Find the type unit with the given type signature and return its section

  contributions.

- <span id="dwarfpackage-cu-sections"></span>`fn cu_sections(&self, index: u32, parent: &Dwarf<R>) -> Result<Dwarf<R>>` — [`Dwarf`](../index.md#dwarf), [`Result`](../../index.md#result)

  Return the section contributions of the compilation unit at the given index.

  

  The index must be in the range `1..cu_index.unit_count`.

  

  This function should only be needed by low level parsers.

- <span id="dwarfpackage-tu-sections"></span>`fn tu_sections(&self, index: u32, parent: &Dwarf<R>) -> Result<Dwarf<R>>` — [`Dwarf`](../index.md#dwarf), [`Result`](../../index.md#result)

  Return the section contributions of the compilation unit at the given index.

  

  The index must be in the range `1..tu_index.unit_count`.

  

  This function should only be needed by low level parsers.

- <span id="dwarfpackage-sections"></span>`fn sections(&self, sections: UnitIndexSectionIterator<'_, R>, parent: &Dwarf<R>) -> Result<Dwarf<R>>` — [`UnitIndexSectionIterator`](../index.md#unitindexsectioniterator), [`Dwarf`](../index.md#dwarf), [`Result`](../../index.md#result)

  Return the section contributions of a unit.

  

  This function should only be needed by low level parsers.

#### Trait Implementations

##### `impl<R: fmt::Debug + Reader> Debug for DwarfPackage<R>`

- <span id="dwarfpackage-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Unit<R, Offset>`

```rust
struct Unit<R, Offset>
where
    R: Reader<Offset = Offset>,
    Offset: ReaderOffset {
    pub header: crate::read::UnitHeader<R, Offset>,
    pub abbreviations: alloc::sync::Arc<crate::read::Abbreviations>,
    pub name: Option<R>,
    pub comp_dir: Option<R>,
    pub low_pc: u64,
    pub str_offsets_base: crate::common::DebugStrOffsetsBase<Offset>,
    pub addr_base: crate::common::DebugAddrBase<Offset>,
    pub loclists_base: crate::common::DebugLocListsBase<Offset>,
    pub rnglists_base: crate::common::DebugRngListsBase<Offset>,
    pub line_program: Option<crate::read::IncompleteLineProgram<R, Offset>>,
    pub dwo_id: Option<crate::common::DwoId>,
}
```

All of the commonly used information for a unit in the `.debug_info` or `.debug_types`
sections.

#### Fields

- **`header`**: `crate::read::UnitHeader<R, Offset>`

  The header of the unit.

- **`abbreviations`**: `alloc::sync::Arc<crate::read::Abbreviations>`

  The parsed abbreviations for the unit.

- **`name`**: `Option<R>`

  The `DW_AT_name` attribute of the unit.

- **`comp_dir`**: `Option<R>`

  The `DW_AT_comp_dir` attribute of the unit.

- **`low_pc`**: `u64`

  The `DW_AT_low_pc` attribute of the unit. Defaults to 0.

- **`str_offsets_base`**: `crate::common::DebugStrOffsetsBase<Offset>`

  The `DW_AT_str_offsets_base` attribute of the unit. Defaults to 0.

- **`addr_base`**: `crate::common::DebugAddrBase<Offset>`

  The `DW_AT_addr_base` attribute of the unit. Defaults to 0.

- **`loclists_base`**: `crate::common::DebugLocListsBase<Offset>`

  The `DW_AT_loclists_base` attribute of the unit. Defaults to 0.

- **`rnglists_base`**: `crate::common::DebugRngListsBase<Offset>`

  The `DW_AT_rnglists_base` attribute of the unit. Defaults to 0.

- **`line_program`**: `Option<crate::read::IncompleteLineProgram<R, Offset>>`

  The line number program of the unit.

- **`dwo_id`**: `Option<crate::common::DwoId>`

  The DWO ID of a skeleton unit or split compilation unit.

#### Implementations

- <span id="unit-new"></span>`fn new(dwarf: &Dwarf<R>, header: UnitHeader<R>) -> Result<Self>` — [`Dwarf`](../index.md#dwarf), [`UnitHeader`](../index.md#unitheader), [`Result`](../../index.md#result)

  Construct a new `Unit` from the given unit header.

- <span id="unit-new-with-abbreviations"></span>`fn new_with_abbreviations(dwarf: &Dwarf<R>, header: UnitHeader<R>, abbreviations: Arc<Abbreviations>) -> Result<Self>` — [`Dwarf`](../index.md#dwarf), [`UnitHeader`](../index.md#unitheader), [`Abbreviations`](../index.md#abbreviations), [`Result`](../../index.md#result)

  Construct a new `Unit` from the given unit header and abbreviations.

  

  The abbreviations for this call can be obtained using `dwarf.abbreviations(&header)`.

  The caller may implement caching to reuse the `Abbreviations` across units with the

  same `header.debug_abbrev_offset()` value.

- <span id="unit-unit-ref"></span>`fn unit_ref<'a>(self: &'a Self, dwarf: &'a Dwarf<R>) -> UnitRef<'a, R>` — [`Dwarf`](../index.md#dwarf), [`UnitRef`](../index.md#unitref)

  Return a reference to this unit and its associated `Dwarf`.

- <span id="unit-encoding"></span>`fn encoding(&self) -> Encoding` — [`Encoding`](../../index.md#encoding)

  Return the encoding parameters for this unit.

- <span id="unit-entry"></span>`fn entry(&self, offset: UnitOffset<<R as >::Offset>) -> Result<DebuggingInformationEntry<R>>` — [`UnitOffset`](../../index.md#unitoffset), [`Reader`](../index.md#reader), [`Result`](../../index.md#result), [`DebuggingInformationEntry`](../index.md#debugginginformationentry)

  Read the `DebuggingInformationEntry` at the given offset.

- <span id="unit-entries"></span>`fn entries(&self) -> EntriesCursor<'_, R>` — [`EntriesCursor`](../index.md#entriescursor)

  Navigate this unit's `DebuggingInformationEntry`s.

- <span id="unit-entries-at-offset"></span>`fn entries_at_offset(&self, offset: UnitOffset<<R as >::Offset>) -> Result<EntriesCursor<'_, R>>` — [`UnitOffset`](../../index.md#unitoffset), [`Reader`](../index.md#reader), [`Result`](../../index.md#result), [`EntriesCursor`](../index.md#entriescursor)

  Navigate this unit's `DebuggingInformationEntry`s

  starting at the given offset.

- <span id="unit-entries-tree"></span>`fn entries_tree(&self, offset: Option<UnitOffset<<R as >::Offset>>) -> Result<EntriesTree<'_, R>>` — [`UnitOffset`](../../index.md#unitoffset), [`Reader`](../index.md#reader), [`Result`](../../index.md#result), [`EntriesTree`](../index.md#entriestree)

  Navigate this unit's `DebuggingInformationEntry`s as a tree

  starting at the given offset.

- <span id="unit-entries-raw"></span>`fn entries_raw(&self, offset: Option<UnitOffset<<R as >::Offset>>) -> Result<EntriesRaw<'_, R>>` — [`UnitOffset`](../../index.md#unitoffset), [`Reader`](../index.md#reader), [`Result`](../../index.md#result), [`EntriesRaw`](../index.md#entriesraw)

  Read the raw data that defines the Debugging Information Entries.

- <span id="unit-copy-relocated-attributes"></span>`fn copy_relocated_attributes(&mut self, other: &Unit<R>)` — [`Unit`](../index.md#unit)

  Copy attributes that are subject to relocation from another unit. This is intended

  to be used to copy attributes from a skeleton compilation unit to the corresponding

  split compilation unit.

- <span id="unit-dwo-name"></span>`fn dwo_name(&self) -> Result<Option<AttributeValue<R>>>` — [`Result`](../../index.md#result), [`AttributeValue`](../index.md#attributevalue)

  Find the dwo name (if any) for this unit, automatically handling the differences

  between the standardized DWARF 5 split DWARF format and the pre-DWARF 5 GNU

  extension.

  

  The returned value is relative to this unit's `comp_dir`.

#### Trait Implementations

##### `impl<R, Offset> Debug for Unit<R, Offset>`

- <span id="unit-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: Reader> Deref for Unit<R>`

- <span id="unit-deref-type-target"></span>`type Target = UnitHeader<R>`

- <span id="unit-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl Receiver for Unit<R, Offset>`

- <span id="unit-receiver-type-target"></span>`type Target = T`

### `UnitRef<'a, R: Reader>`

```rust
struct UnitRef<'a, R: Reader> {
    pub dwarf: &'a Dwarf<R>,
    pub unit: &'a Unit<R>,
}
```

A reference to a `Unit` and its associated `Dwarf`.

These often need to be passed around together, so this struct makes that easier.

It implements `Deref` to `&'a Unit`, so you can use it as if it were a `Unit`.
It also implements methods that correspond to methods on `Dwarf` that take a `Unit`.

#### Fields

- **`dwarf`**: `&'a Dwarf<R>`

  The `Dwarf` that contains the unit.

- **`unit`**: `&'a Unit<R>`

  The `Unit` being referenced.

#### Implementations

- <span id="unitref-new"></span>`fn new(dwarf: &'a Dwarf<R>, unit: &'a Unit<R>) -> Self` — [`Dwarf`](../index.md#dwarf), [`Unit`](../index.md#unit)

  Construct a new `UnitRef` from a `Dwarf` and a `Unit`.

- <span id="unitref-string-offset"></span>`fn string_offset(&self, index: DebugStrOffsetsIndex<<R as >::Offset>) -> Result<DebugStrOffset<<R as >::Offset>>` — [`DebugStrOffsetsIndex`](../../index.md#debugstroffsetsindex), [`Reader`](../index.md#reader), [`Result`](../../index.md#result), [`DebugStrOffset`](../../index.md#debugstroffset)

  Return the string offset at the given index.

- <span id="unitref-string"></span>`fn string(&self, offset: DebugStrOffset<<R as >::Offset>) -> Result<R>` — [`DebugStrOffset`](../../index.md#debugstroffset), [`Reader`](../index.md#reader), [`Result`](../../index.md#result)

  Return the string at the given offset in `.debug_str`.

- <span id="unitref-line-string"></span>`fn line_string(&self, offset: DebugLineStrOffset<<R as >::Offset>) -> Result<R>` — [`DebugLineStrOffset`](../../index.md#debuglinestroffset), [`Reader`](../index.md#reader), [`Result`](../../index.md#result)

  Return the string at the given offset in `.debug_line_str`.

- <span id="unitref-sup-string"></span>`fn sup_string(&self, offset: DebugStrOffset<<R as >::Offset>) -> Result<R>` — [`DebugStrOffset`](../../index.md#debugstroffset), [`Reader`](../index.md#reader), [`Result`](../../index.md#result)

  Return the string at the given offset in the `.debug_str`

  in the supplementary object file.

- <span id="unitref-attr-string"></span>`fn attr_string(&self, attr: AttributeValue<R>) -> Result<R>` — [`AttributeValue`](../index.md#attributevalue), [`Result`](../../index.md#result)

  Return an attribute value as a string slice.

  

  See `Dwarf::attr_string` for more information.

- <span id="unitref-address"></span>`fn address(&self, index: DebugAddrIndex<<R as >::Offset>) -> Result<u64>` — [`DebugAddrIndex`](../../index.md#debugaddrindex), [`Reader`](../index.md#reader), [`Result`](../../index.md#result)

  Return the address at the given index.

- <span id="unitref-attr-address"></span>`fn attr_address(&self, attr: AttributeValue<R>) -> Result<Option<u64>>` — [`AttributeValue`](../index.md#attributevalue), [`Result`](../../index.md#result)

  Try to return an attribute value as an address.

  

  See `Dwarf::attr_address` for more information.

- <span id="unitref-ranges-offset-from-raw"></span>`fn ranges_offset_from_raw(&self, offset: RawRangeListsOffset<<R as >::Offset>) -> RangeListsOffset<<R as >::Offset>` — [`RawRangeListsOffset`](../../index.md#rawrangelistsoffset), [`Reader`](../index.md#reader), [`RangeListsOffset`](../../index.md#rangelistsoffset)

  Return the range list offset for the given raw offset.

  

  This handles adding `DW_AT_GNU_ranges_base` if required.

- <span id="unitref-ranges-offset"></span>`fn ranges_offset(&self, index: DebugRngListsIndex<<R as >::Offset>) -> Result<RangeListsOffset<<R as >::Offset>>` — [`DebugRngListsIndex`](../../index.md#debugrnglistsindex), [`Reader`](../index.md#reader), [`Result`](../../index.md#result), [`RangeListsOffset`](../../index.md#rangelistsoffset)

  Return the range list offset at the given index.

- <span id="unitref-ranges"></span>`fn ranges(&self, offset: RangeListsOffset<<R as >::Offset>) -> Result<RngListIter<R>>` — [`RangeListsOffset`](../../index.md#rangelistsoffset), [`Reader`](../index.md#reader), [`Result`](../../index.md#result), [`RngListIter`](../index.md#rnglistiter)

  Iterate over the `RangeListEntry`s starting at the given offset.

- <span id="unitref-raw-ranges"></span>`fn raw_ranges(&self, offset: RangeListsOffset<<R as >::Offset>) -> Result<RawRngListIter<R>>` — [`RangeListsOffset`](../../index.md#rangelistsoffset), [`Reader`](../index.md#reader), [`Result`](../../index.md#result), [`RawRngListIter`](../index.md#rawrnglistiter)

  Iterate over the `RawRngListEntry`ies starting at the given offset.

- <span id="unitref-attr-ranges-offset"></span>`fn attr_ranges_offset(&self, attr: AttributeValue<R>) -> Result<Option<RangeListsOffset<<R as >::Offset>>>` — [`AttributeValue`](../index.md#attributevalue), [`Result`](../../index.md#result), [`RangeListsOffset`](../../index.md#rangelistsoffset), [`Reader`](../index.md#reader)

  Try to return an attribute value as a range list offset.

  

  See `Dwarf::attr_ranges_offset` for more information.

- <span id="unitref-attr-ranges"></span>`fn attr_ranges(&self, attr: AttributeValue<R>) -> Result<Option<RngListIter<R>>>` — [`AttributeValue`](../index.md#attributevalue), [`Result`](../../index.md#result), [`RngListIter`](../index.md#rnglistiter)

  Try to return an attribute value as a range list entry iterator.

  

  See `Dwarf::attr_ranges` for more information.

- <span id="unitref-die-ranges"></span>`fn die_ranges(&self, entry: &DebuggingInformationEntry<R>) -> Result<RangeIter<R>>` — [`DebuggingInformationEntry`](../index.md#debugginginformationentry), [`Result`](../../index.md#result), [`RangeIter`](../index.md#rangeiter)

  Return an iterator for the address ranges of a `DebuggingInformationEntry`.

  

  This uses `DW_AT_low_pc`, `DW_AT_high_pc` and `DW_AT_ranges`.

- <span id="unitref-unit-ranges"></span>`fn unit_ranges(&self) -> Result<RangeIter<R>>` — [`Result`](../../index.md#result), [`RangeIter`](../index.md#rangeiter)

  Return an iterator for the address ranges of the `Unit`.

  

  This uses `DW_AT_low_pc`, `DW_AT_high_pc` and `DW_AT_ranges` of the

  root `DebuggingInformationEntry`.

- <span id="unitref-locations-offset"></span>`fn locations_offset(&self, index: DebugLocListsIndex<<R as >::Offset>) -> Result<LocationListsOffset<<R as >::Offset>>` — [`DebugLocListsIndex`](../../index.md#debugloclistsindex), [`Reader`](../index.md#reader), [`Result`](../../index.md#result), [`LocationListsOffset`](../../index.md#locationlistsoffset)

  Return the location list offset at the given index.

- <span id="unitref-locations"></span>`fn locations(&self, offset: LocationListsOffset<<R as >::Offset>) -> Result<LocListIter<R>>` — [`LocationListsOffset`](../../index.md#locationlistsoffset), [`Reader`](../index.md#reader), [`Result`](../../index.md#result), [`LocListIter`](../index.md#loclistiter)

  Iterate over the `LocationListEntry`s starting at the given offset.

- <span id="unitref-raw-locations"></span>`fn raw_locations(&self, offset: LocationListsOffset<<R as >::Offset>) -> Result<RawLocListIter<R>>` — [`LocationListsOffset`](../../index.md#locationlistsoffset), [`Reader`](../index.md#reader), [`Result`](../../index.md#result), [`RawLocListIter`](../index.md#rawloclistiter)

  Iterate over the raw `LocationListEntry`s starting at the given offset.

- <span id="unitref-attr-locations-offset"></span>`fn attr_locations_offset(&self, attr: AttributeValue<R>) -> Result<Option<LocationListsOffset<<R as >::Offset>>>` — [`AttributeValue`](../index.md#attributevalue), [`Result`](../../index.md#result), [`LocationListsOffset`](../../index.md#locationlistsoffset), [`Reader`](../index.md#reader)

  Try to return an attribute value as a location list offset.

  

  See `Dwarf::attr_locations_offset` for more information.

- <span id="unitref-attr-locations"></span>`fn attr_locations(&self, attr: AttributeValue<R>) -> Result<Option<LocListIter<R>>>` — [`AttributeValue`](../index.md#attributevalue), [`Result`](../../index.md#result), [`LocListIter`](../index.md#loclistiter)

  Try to return an attribute value as a location list entry iterator.

  

  See `Dwarf::attr_locations` for more information.

- <span id="unitref-macinfo"></span>`fn macinfo(&self, offset: DebugMacinfoOffset<<R as >::Offset>) -> Result<MacroIter<R>>` — [`DebugMacinfoOffset`](../../index.md#debugmacinfooffset), [`Reader`](../index.md#reader), [`Result`](../../index.md#result), [`MacroIter`](../index.md#macroiter)

  Try to return an iterator for the list of macros at the given `.debug_macinfo` offset.

- <span id="unitref-macros"></span>`fn macros(&self, offset: DebugMacroOffset<<R as >::Offset>) -> Result<MacroIter<R>>` — [`DebugMacroOffset`](../../index.md#debugmacrooffset), [`Reader`](../index.md#reader), [`Result`](../../index.md#result), [`MacroIter`](../index.md#macroiter)

  Try to return an iterator for the list of macros at the given `.debug_macro` offset.

#### Trait Implementations

##### `impl<R: Reader> Clone for UnitRef<'a, R>`

- <span id="unitref-clone"></span>`fn clone(&self) -> Self`

##### `impl<R: Reader> Copy for UnitRef<'a, R>`

##### `impl<R: fmt::Debug + Reader> Debug for UnitRef<'a, R>`

- <span id="unitref-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: Reader> Deref for UnitRef<'a, R>`

- <span id="unitref-deref-type-target"></span>`type Target = &'a Unit<R>`

- <span id="unitref-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl Receiver for UnitRef<'a, R>`

- <span id="unitref-receiver-type-target"></span>`type Target = T`

### `RangeIter<R: Reader>`

```rust
struct RangeIter<R: Reader>(RangeIterInner<R>);
```

An iterator for the address ranges of a `DebuggingInformationEntry`.

Returned by `Dwarf::die_ranges` and `Dwarf::unit_ranges`.

#### Implementations

- <span id="rangeiter-next"></span>`fn next(&mut self) -> Result<Option<Range>>` — [`Result`](../../index.md#result), [`Range`](../index.md#range)

  Advance the iterator to the next range.

#### Trait Implementations

##### `impl<R: fmt::Debug + Reader> Debug for RangeIter<R>`

- <span id="rangeiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: Reader> Default for RangeIter<R>`

- <span id="rangeiter-default"></span>`fn default() -> Self`

##### `impl IntoIterator for RangeIter<R>`

- <span id="rangeiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="rangeiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="rangeiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: Reader> Iterator for RangeIter<R>`

- <span id="rangeiter-iterator-type-item"></span>`type Item = Result<Range, Error>`

- <span id="rangeiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

## Enums

### `RangeIterInner<R: Reader>`

```rust
enum RangeIterInner<R: Reader> {
    Single(Option<crate::read::Range>),
    List(crate::read::RngListIter<R>),
}
```

#### Trait Implementations

##### `impl<R: fmt::Debug + Reader> Debug for RangeIterInner<R>`

- <span id="rangeiterinner-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

