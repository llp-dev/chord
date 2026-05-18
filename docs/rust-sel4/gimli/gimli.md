**gimli**

# Module: gimli

## Contents

**Modules**

- [`constants`](#constants) - Constant definitions.
- [`leb128`](#leb128) - Read and write DWARF's "Little Endian Base 128" (LEB128) variable length
- [`read`](#read) - Read DWARF debugging information.

---

## Module: constants

Constant definitions.

The DWARF spec's `DW_AT_*` type is represented as `struct DwAt(u16)`,
`DW_FORM_*` as `DwForm(u16)`, etc.

There are also exported const definitions for each constant.



## Module: leb128

Read and write DWARF's "Little Endian Base 128" (LEB128) variable length
integer encoding.

The implementation is a direct translation of the psuedocode in the DWARF 4
standard's appendix C.

Read and write signed integers:

```
# #[cfg(all(feature = "read", feature = "write"))] {
use gimli::{EndianSlice, NativeEndian, leb128};

let mut buf = [0; 1024];

// Write to anything that implements `std::io::Write`.
{
    let mut writable = &mut buf[..];
    leb128::write::signed(&mut writable, -12345).expect("Should write number");
}

// Read from anything that implements `gimli::Reader`.
let mut readable = EndianSlice::new(&buf[..], NativeEndian);
let val = leb128::read::signed(&mut readable).expect("Should read number");
assert_eq!(val, -12345);
# }
```

Or read and write unsigned integers:

```
# #[cfg(all(feature = "read", feature = "write"))] {
use gimli::{EndianSlice, NativeEndian, leb128};

let mut buf = [0; 1024];

{
    let mut writable = &mut buf[..];
    leb128::write::unsigned(&mut writable, 98765).expect("Should write number");
}

let mut readable = EndianSlice::new(&buf[..], NativeEndian);
let val = leb128::read::unsigned(&mut readable).expect("Should read number");
assert_eq!(val, 98765);
# }
```



## Module: read

Read DWARF debugging information.

* [Example Usage](#example-usage)
* [API Structure](#api-structure)
* [Using with `FallibleIterator`](#using-with-fallibleiterator)

## Example Usage

Print out all of the functions in the debuggee program:

```rust,no_run
# fn example() -> Result<(), gimli::Error> {
# type R = gimli::EndianSlice<'static, gimli::LittleEndian>;
# let get_file_section_reader = |name| -> Result<R, gimli::Error> { unimplemented!() };
# let get_sup_file_section_reader = |name| -> Result<R, gimli::Error> { unimplemented!() };
// Read the DWARF sections with whatever object loader you're using.
// These closures should return a `Reader` instance (e.g. `EndianSlice`).
let loader = |section: gimli::SectionId| { get_file_section_reader(section.name()) };
let sup_loader = |section: gimli::SectionId| { get_sup_file_section_reader(section.name()) };
let mut dwarf = gimli::Dwarf::load(loader)?;
dwarf.load_sup(sup_loader)?;

// Iterate over all compilation units.
let mut iter = dwarf.units();
while let Some(header) = iter.next()? {
    // Parse the abbreviations and other information for this compilation unit.
    let unit = dwarf.unit(header)?;

    // Iterate over all of this compilation unit's entries.
    let mut entries = unit.entries();
    while let Some((_, entry)) = entries.next_dfs()? {
        // If we find an entry for a function, print it.
        if entry.tag() == gimli::DW_TAG_subprogram {
            println!("Found a function: {:?}", entry);
        }
    }
}
# unreachable!()
# }
```

Full example programs:

  * [A simple `.debug_info` parser](https://github.com/gimli-rs/gimli/blob/master/crates/examples/src/bin/simple.rs)

  * [A simple `.debug_line` parser](https://github.com/gimli-rs/gimli/blob/master/crates/examples/src/bin/simple_line.rs)

  * [A `dwarfdump`
    clone](https://github.com/gimli-rs/gimli/blob/master/crates/examples/src/bin/dwarfdump.rs)

  * [An `addr2line` clone](https://github.com/gimli-rs/addr2line)

  * [`ddbug`](https://github.com/gimli-rs/ddbug), a utility giving insight into
    code generation by making debugging information readable

  * [`dwprod`](https://github.com/fitzgen/dwprod), a tiny utility to list the
    compilers used to create each compilation unit within a shared library or
    executable (via `DW_AT_producer`)

  * [`dwarf-validate`](https://github.com/gimli-rs/gimli/blob/master/crates/examples/src/bin/dwarf-validate.rs),
    a program to validate the integrity of some DWARF and its references
    between sections and compilation units.

## API Structure

* Basic familiarity with DWARF is assumed.

* The [`Dwarf`](./struct.Dwarf.html) type contains the commonly used DWARF
  sections. It has methods that simplify access to debugging data that spans
  multiple sections. Use of this type is optional, but recommended.

* The [`DwarfPackage`](./struct.Dwarf.html) type contains the DWARF
  package (DWP) sections. It has methods to find a DWARF object (DWO)
  within the package.

* Each section gets its own type. Consider these types the entry points to
  the library:

  * [`DebugAbbrev`](./struct.DebugAbbrev.html): The `.debug_abbrev` section.

  * [`DebugAddr`](./struct.DebugAddr.html): The `.debug_addr` section.

  * [`DebugAranges`](./struct.DebugAranges.html): The `.debug_aranges`
    section.

  * [`DebugFrame`](./struct.DebugFrame.html): The `.debug_frame` section.

  * [`DebugInfo`](./struct.DebugInfo.html): The `.debug_info` section.

  * [`DebugLine`](./struct.DebugLine.html): The `.debug_line` section.

  * [`DebugLineStr`](./struct.DebugLineStr.html): The `.debug_line_str` section.

  * [`DebugLoc`](./struct.DebugLoc.html): The `.debug_loc` section.

  * [`DebugLocLists`](./struct.DebugLocLists.html): The `.debug_loclists` section.

  * [`DebugPubNames`](./struct.DebugPubNames.html): The `.debug_pubnames`
    section.

  * [`DebugPubTypes`](./struct.DebugPubTypes.html): The `.debug_pubtypes`
    section.

  * [`DebugRanges`](./struct.DebugRanges.html): The `.debug_ranges` section.

  * [`DebugRngLists`](./struct.DebugRngLists.html): The `.debug_rnglists` section.

  * [`DebugStr`](./struct.DebugStr.html): The `.debug_str` section.

  * [`DebugStrOffsets`](./struct.DebugStrOffsets.html): The `.debug_str_offsets` section.

  * [`DebugTypes`](./struct.DebugTypes.html): The `.debug_types` section.

  * [`DebugCuIndex`](./struct.DebugCuIndex.html): The `.debug_cu_index` section.

  * [`DebugTuIndex`](./struct.DebugTuIndex.html): The `.debug_tu_index` section.

  * [`EhFrame`](./struct.EhFrame.html): The `.eh_frame` section.

  * [`EhFrameHdr`](./struct.EhFrameHdr.html): The `.eh_frame_hdr` section.

* Each section type exposes methods for accessing the debugging data encoded
  in that section. For example, the [`DebugInfo`](./struct.DebugInfo.html)
  struct has the [`units`](./struct.DebugInfo.html#method.units) method for
  iterating over the compilation units defined within it.

* Offsets into a section are strongly typed: an offset into `.debug_info` is
  the [`DebugInfoOffset`](./struct.DebugInfoOffset.html) type. It cannot be
  used to index into the [`DebugLine`](./struct.DebugLine.html) type because
  `DebugLine` represents the `.debug_line` section. There are similar types
  for offsets relative to a compilation unit rather than a section.

## Using with `FallibleIterator`

The standard library's `Iterator` trait and related APIs do not play well
with iterators where the `next` operation is fallible. One can make the
`Iterator`'s associated `Item` type be a `Result<T, E>`, however the
provided methods cannot gracefully handle the case when an `Err` is
returned.

This situation led to the
[`fallible-iterator`](https://crates.io/crates/fallible-iterator) crate's
existence. You can read more of the rationale for its existence in its
docs. The crate provides the helpers you have come to expect (eg `map`,
`filter`, etc) for iterators that can fail.

`gimli`'s many lazy parsing iterators are a perfect match for the
`fallible-iterator` crate's `FallibleIterator` trait because parsing is not
done eagerly. Parse errors later in the input might only be discovered after
having iterated through many items.

To use `gimli` iterators with `FallibleIterator`, import the crate and trait
into your code:

```
# #[cfg(feature = "fallible-iterator")]
# fn foo() {
// Use the `FallibleIterator` trait so its methods are in scope!
use fallible_iterator::FallibleIterator;
use gimli::{DebugAranges, EndianSlice, LittleEndian};

fn find_sum_of_address_range_lengths(aranges: DebugAranges<EndianSlice<LittleEndian>>)
    -> gimli::Result<u64>
{
    // `DebugAranges::headers` returns a `FallibleIterator`!
    aranges.headers()
        // `flat_map` is provided by `FallibleIterator`!
        .flat_map(|header| Ok(header.entries()))
        // `map` is provided by `FallibleIterator`!
        .map(|arange| Ok(arange.length()))
        // `fold` is provided by `FallibleIterator`!
        .fold(0, |sum, len| Ok(sum + len))
}
# }
# fn main() {}
```



