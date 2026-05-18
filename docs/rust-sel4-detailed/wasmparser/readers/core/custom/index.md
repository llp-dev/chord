*[wasmparser](../../../index.md) / [readers](../../index.md) / [core](../index.md) / [custom](index.md)*

---

# Module `custom`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`CustomSectionReader`](#customsectionreader) | struct | A reader for custom sections of a WebAssembly module. |
| [`KnownCustom`](#knowncustom) | enum | Return value of [`CustomSectionReader::as_known`]. |

## Structs

### `CustomSectionReader<'a>`

```rust
struct CustomSectionReader<'a> {
    name: &'a str,
    reader: crate::BinaryReader<'a>,
}
```

A reader for custom sections of a WebAssembly module.

#### Implementations

- <span id="customsectionreader-new"></span>`fn new(reader: BinaryReader<'a>) -> Result<CustomSectionReader<'a>>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result), [`CustomSectionReader`](../index.md#customsectionreader)

  Constructs a new `CustomSectionReader` for the given data and offset.

- <span id="customsectionreader-name"></span>`fn name(&self) -> &'a str`

  The name of the custom section.

- <span id="customsectionreader-data-offset"></span>`fn data_offset(&self) -> usize`

  The offset, relative to the start of the original module or component,

  that the `data` payload for this custom section starts at.

- <span id="customsectionreader-data"></span>`fn data(&self) -> &'a [u8]`

  The actual contents of the custom section.

- <span id="customsectionreader-range"></span>`fn range(&self) -> Range<usize>`

  The range of bytes that specify this whole custom section (including

  both the name of this custom section and its data) specified in

  offsets relative to the start of the byte stream.

- <span id="customsectionreader-as-known"></span>`fn as_known(&self) -> KnownCustom<'a>` — [`KnownCustom`](../index.md#knowncustom)

  Attempts to match and see if this custom section is statically known to

  `wasmparser` with any known section reader.

  

  This will inspect `self.name()` and return a [`KnownCustom`](../index.md) if the name

  matches a known custom section where there is a parser available for it.

  This can also be used as a convenience function for creating such

  parsers.

  

  If the custom section name is not known, or if a reader could not be

  created, then `KnownCustom::Unknown` is returned.

#### Trait Implementations

##### `impl Clone for CustomSectionReader<'a>`

- <span id="customsectionreader-clone"></span>`fn clone(&self) -> CustomSectionReader<'a>` — [`CustomSectionReader`](../index.md#customsectionreader)

##### `impl Debug for CustomSectionReader<'a>`

- <span id="customsectionreader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Enums

### `KnownCustom<'a>`

```rust
enum KnownCustom<'a> {
    Name(crate::NameSectionReader<'a>),
    BranchHints(crate::BranchHintSectionReader<'a>),
    Producers(crate::ProducersSectionReader<'a>),
    Dylink0(crate::Dylink0SectionReader<'a>),
    CoreDump(crate::CoreDumpSection<'a>),
    CoreDumpStack(crate::CoreDumpStackSection<'a>),
    CoreDumpInstances(crate::CoreDumpInstancesSection),
    CoreDumpModules(crate::CoreDumpModulesSection<'a>),
    Linking(crate::LinkingSectionReader<'a>),
    Reloc(crate::RelocSectionReader<'a>),
    Unknown,
}
```

Return value of `CustomSectionReader::as_known`.

Note that this is `#[non_exhaustive]` because depending on crate features
this enumeration will different entries.

