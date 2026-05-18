**addr2line > lookup**

# Module: lookup

## Contents

**Structs**

- [`SplitDwarfLoad`](#splitdwarfload) - This struct contains the information needed to find split DWARF data

**Enums**

- [`LookupResult`](#lookupresult) - Operations that consult debug information may require additional files

**Traits**

- [`LookupContinuation`](#lookupcontinuation) - This trait represents a partially complete operation that can be resumed

---

## addr2line::lookup::LookupContinuation

*Trait*

This trait represents a partially complete operation that can be resumed
once a load of needed split DWARF data is completed or abandoned by the
API consumer.

**Methods:**

- `Output`: The final output of this operation.
- `Buf`: The type of reader used.
- `resume`: Resumes the operation with the provided data.



## addr2line::lookup::LookupResult

*Enum*

Operations that consult debug information may require additional files
to be loaded if split DWARF is being used. This enum returns the result
of the operation in the `Output` variant, or information about the split
DWARF that is required and a continuation to invoke once it is available
in the `Load` variant.

This enum is intended to be used in a loop like so:
```no_run
  # use addr2line::*;
  # use std::sync::Arc;
  # let ctx: Context<gimli::EndianSlice<gimli::RunTimeEndian>> = todo!();
  # let do_split_dwarf_load = |load: SplitDwarfLoad<gimli::EndianSlice<gimli::RunTimeEndian>>| -> Option<Arc<gimli::Dwarf<gimli::EndianSlice<gimli::RunTimeEndian>>>> { None };
  const ADDRESS: u64 = 0xdeadbeef;
  let mut r = ctx.find_frames(ADDRESS);
  let result = loop {
    match r {
      LookupResult::Output(result) => break result,
      LookupResult::Load { load, continuation } => {
        let dwo = do_split_dwarf_load(load);
        r = continuation.resume(dwo);
      }
    }
  };
```

**Generic Parameters:**
- L

**Variants:**
- `Load{ load: SplitDwarfLoad<<L as LookupContinuation>::Buf>, continuation: L }` - The lookup requires split DWARF data to be loaded.
- `Output(<L as LookupContinuation>::Output)` - The lookup has completed and produced an output.

**Methods:**

- `fn skip_all_loads(self: Self) -> <L as >::Output` - Callers that do not handle split DWARF can call `skip_all_loads`



## addr2line::lookup::SplitDwarfLoad

*Struct*

This struct contains the information needed to find split DWARF data
and to produce a `gimli::Dwarf<R>` for it.

**Generic Parameters:**
- R

**Fields:**
- `dwo_id: gimli::DwoId` - The dwo id, for looking up in a DWARF package, or for
- `comp_dir: Option<R>` - The compilation directory `path` is relative to.
- `path: Option<R>` - A path on the filesystem, relative to `comp_dir` to find this dwo.
- `parent: alloc::sync::Arc<gimli::Dwarf<R>>` - Once the split DWARF data is loaded, the loader is expected



