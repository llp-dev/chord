**addr2line**

# Module: addr2line

## Contents

**Structs**

- [`Context`](#context) - The state necessary to perform address to line translation.

---

## addr2line::Context

*Struct*

The state necessary to perform address to line translation.

Constructing a `Context` is somewhat costly, so users should aim to reuse `Context`s
when performing lookups for many addresses in the same executable.

**Generic Parameters:**
- R

**Methods:**

- `fn from_sections(debug_abbrev: gimli::DebugAbbrev<R>, debug_addr: gimli::DebugAddr<R>, debug_aranges: gimli::DebugAranges<R>, debug_info: gimli::DebugInfo<R>, debug_line: gimli::DebugLine<R>, debug_line_str: gimli::DebugLineStr<R>, debug_ranges: gimli::DebugRanges<R>, debug_rnglists: gimli::DebugRngLists<R>, debug_str: gimli::DebugStr<R>, debug_str_offsets: gimli::DebugStrOffsets<R>, default_section: R) -> Result<Self, gimli::Error>` - Construct a new `Context` from DWARF sections.
- `fn from_dwarf(sections: gimli::Dwarf<R>) -> Result<Context<R>, gimli::Error>` - Construct a new `Context` from an existing [`gimli::Dwarf`] object.
- `fn from_arc_dwarf(sections: Arc<gimli::Dwarf<R>>) -> Result<Context<R>, gimli::Error>` - Construct a new `Context` from an existing [`gimli::Dwarf`] object.
- `fn find_dwarf_and_unit(self: &Self, probe: u64) -> LookupResult<impl Trait>` - Find the DWARF unit corresponding to the given virtual memory address.
- `fn find_location(self: &Self, probe: u64) -> Result<Option<Location>, gimli::Error>` - Find the source file and line corresponding to the given virtual memory address.
- `fn find_location_range(self: &Self, probe_low: u64, probe_high: u64) -> Result<LocationRangeIter<R>, gimli::Error>` - Return source file and lines for a range of addresses. For each location it also
- `fn find_frames(self: &Self, probe: u64) -> LookupResult<impl Trait>` - Return an iterator for the function frames corresponding to the given virtual
- `fn preload_units(self: &Self, probe: u64) -> impl Trait` - Preload units for `probe`.



