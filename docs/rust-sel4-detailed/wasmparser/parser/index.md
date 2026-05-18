*[wasmparser](../index.md) / [parser](index.md)*

---

# Module `parser`

## Contents

- [Structs](#structs)
  - [`ParserCounts`](#parsercounts)
  - [`Parser`](#parser)
- [Enums](#enums)
  - [`Encoding`](#encoding)
  - [`Order`](#order)
  - [`State`](#state)
  - [`Chunk`](#chunk)
  - [`Payload`](#payload)
- [Functions](#functions)
  - [`usize_to_u64`](#usize-to-u64)
  - [`section`](#section)
  - [`single_item`](#single-item)
  - [`delimited`](#delimited)
  - [`clear_hint`](#clear-hint)
- [Constants](#constants)
  - [`WASM_MODULE_VERSION`](#wasm-module-version)
  - [`WASM_COMPONENT_VERSION`](#wasm-component-version)
  - [`KIND_MODULE`](#kind-module)
  - [`KIND_COMPONENT`](#kind-component)
  - [`CUSTOM_SECTION`](#custom-section)
  - [`TYPE_SECTION`](#type-section)
  - [`IMPORT_SECTION`](#import-section)
  - [`FUNCTION_SECTION`](#function-section)
  - [`TABLE_SECTION`](#table-section)
  - [`MEMORY_SECTION`](#memory-section)
  - [`GLOBAL_SECTION`](#global-section)
  - [`EXPORT_SECTION`](#export-section)
  - [`START_SECTION`](#start-section)
  - [`ELEMENT_SECTION`](#element-section)
  - [`CODE_SECTION`](#code-section)
  - [`DATA_SECTION`](#data-section)
  - [`DATA_COUNT_SECTION`](#data-count-section)
  - [`TAG_SECTION`](#tag-section)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ParserCounts`](#parsercounts) | struct |  |
| [`Parser`](#parser) | struct | An incremental parser of a binary WebAssembly module or component. |
| [`Encoding`](#encoding) | enum | The supported encoding formats for the parser. |
| [`Order`](#order) | enum |  |
| [`State`](#state) | enum |  |
| [`Chunk`](#chunk) | enum | A successful return payload from [`Parser::parse`]. |
| [`Payload`](#payload) | enum | Values that can be parsed from a WebAssembly module or component. |
| [`usize_to_u64`](#usize-to-u64) | fn |  |
| [`section`](#section) | fn | Parses an entire section resident in memory into a `Payload`. |
| [`single_item`](#single-item) | fn | Reads a section that is represented by a single uleb-encoded `u32`. |
| [`delimited`](#delimited) | fn | Attempts to parse using `f`. |
| [`clear_hint`](#clear-hint) | fn |  |
| [`WASM_MODULE_VERSION`](#wasm-module-version) | const |  |
| [`WASM_COMPONENT_VERSION`](#wasm-component-version) | const |  |
| [`KIND_MODULE`](#kind-module) | const |  |
| [`KIND_COMPONENT`](#kind-component) | const |  |
| [`CUSTOM_SECTION`](#custom-section) | const |  |
| [`TYPE_SECTION`](#type-section) | const |  |
| [`IMPORT_SECTION`](#import-section) | const |  |
| [`FUNCTION_SECTION`](#function-section) | const |  |
| [`TABLE_SECTION`](#table-section) | const |  |
| [`MEMORY_SECTION`](#memory-section) | const |  |
| [`GLOBAL_SECTION`](#global-section) | const |  |
| [`EXPORT_SECTION`](#export-section) | const |  |
| [`START_SECTION`](#start-section) | const |  |
| [`ELEMENT_SECTION`](#element-section) | const |  |
| [`CODE_SECTION`](#code-section) | const |  |
| [`DATA_SECTION`](#data-section) | const |  |
| [`DATA_COUNT_SECTION`](#data-count-section) | const |  |
| [`TAG_SECTION`](#tag-section) | const |  |

## Structs

### `ParserCounts`

```rust
struct ParserCounts {
    function_entries: Option<u32>,
    code_entries: Option<u32>,
    data_entries: Option<u32>,
    data_count: Option<u32>,
}
```

#### Trait Implementations

##### `impl Clone for ParserCounts`

- <span id="parsercounts-clone"></span>`fn clone(&self) -> ParserCounts` — [`ParserCounts`](#parsercounts)

##### `impl Debug for ParserCounts`

- <span id="parsercounts-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ParserCounts`

- <span id="parsercounts-default"></span>`fn default() -> ParserCounts` — [`ParserCounts`](#parsercounts)

### `Parser`

```rust
struct Parser {
    state: State,
    offset: u64,
    max_size: u64,
    encoding: Encoding,
    counts: ParserCounts,
    order: (Order, u64),
}
```

An incremental parser of a binary WebAssembly module or component.

This type is intended to be used to incrementally parse a WebAssembly module
or component as bytes become available for the module. This can also be used
to parse modules or components that are already entirely resident within memory.

This primary function for a parser is the `Parser::parse` function which
will incrementally consume input. You can also use the `Parser::parse_all`
function to parse a module or component that is entirely resident in memory.

#### Implementations

- <span id="parser-new"></span>`fn new(offset: u64) -> Parser` — [`Parser`](../index.md#parser)

  Creates a new parser.

  

  Reports errors and ranges relative to `offset` provided, where `offset`

  is some logical offset within the input stream that we're parsing.

- <span id="parser-is-core-wasm"></span>`fn is_core_wasm(bytes: &[u8]) -> bool`

  Tests whether `bytes` looks like a core WebAssembly module.

  

  This will inspect the first 8 bytes of `bytes` and return `true` if it

  starts with the standard core WebAssembly header.

- <span id="parser-is-component"></span>`fn is_component(bytes: &[u8]) -> bool`

  Tests whether `bytes` looks like a WebAssembly component.

  

  This will inspect the first 8 bytes of `bytes` and return `true` if it

  starts with the standard WebAssembly component header.

- <span id="parser-offset"></span>`fn offset(&self) -> u64`

  Returns the original offset that this parser is currently at.

- <span id="parser-parse"></span>`fn parse<'a>(&mut self, data: &'a [u8], eof: bool) -> Result<Chunk<'a>>` — [`Result`](../binary_reader/index.md#result), [`Chunk`](../index.md#chunk)

  Attempts to parse a chunk of data.

  

  This method will attempt to parse the next incremental portion of a

  WebAssembly binary. Data available for the module or component is

  provided as `data`, and the data can be incomplete if more data has yet

  to arrive. The `eof` flag indicates whether more data will ever be received.

  

  There are two ways parsing can succeed with this method:

  

  * `Chunk::NeedMoreData` - this indicates that there is not enough bytes

    in `data` to parse a payload. The caller needs to wait for more data to

    be available in this situation before calling this method again. It is

    guaranteed that this is only returned if `eof` is `false`.

  

  * `Chunk::Parsed` - this indicates that a chunk of the input was

    successfully parsed. The payload is available in this variant of what

    was parsed, and this also indicates how many bytes of `data` was

    consumed. It's expected that the caller will not provide these bytes

    back to the [`Parser`](../index.md) again.

  

  Note that all `Chunk` return values are connected, with a lifetime, to

  the input buffer. Each parsed chunk borrows the input buffer and is a

  view into it for successfully parsed chunks.

  

  It is expected that you'll call this method until `Payload::End` is

  reached, at which point you're guaranteed that the parse has completed.

  Note that complete parsing, for the top-level module or component,

  implies that `data` is empty and `eof` is `true`.

  

  # Errors

  

  Parse errors are returned as an `Err`. Errors can happen when the

  structure of the data is unexpected or if sections are too large for

  example. Note that errors are not returned for malformed *contents* of

  sections here. Sections are generally not individually parsed and each

  returned [`Payload`](../index.md) needs to be iterated over further to detect all

  errors.

  

  # Examples

  

  An example of reading a wasm file from a stream (`std::io::Read`) and

  incrementally parsing it.

  

  ```rust

  use std::io::Read;

  use anyhow::Result;

  use wasmparser::{Parser, Chunk, Payload::*};

  

  fn parse(mut reader: impl Read) -> Result<()> {

      let mut buf = Vec::new();

      let mut cur = Parser::new(0);

      let mut eof = false;

      let mut stack = Vec::new();

  

      loop {

          let (payload, consumed) = match cur.parse(&buf, eof)? {

              Chunk::NeedMoreData(hint) => {

                  assert!(!eof); // otherwise an error would be returned

  

                  // Use the hint to preallocate more space, then read

                  // some more data into our buffer.

                  //

                  // Note that the buffer management here is not ideal,

                  // but it's compact enough to fit in an example!

                  let len = buf.len();

                  buf.extend((0..hint).map(|_| 0u8));

                  let n = reader.read(&mut buf[len..])?;

                  buf.truncate(len + n);

                  eof = n == 0;

                  continue;

              }

  

              Chunk::Parsed { consumed, payload } => (payload, consumed),

          };

  

          match payload {

              // Sections for WebAssembly modules

              Version { .. } => { /* ... */ }

              TypeSection(_) => { /* ... */ }

              ImportSection(_) => { /* ... */ }

              FunctionSection(_) => { /* ... */ }

              TableSection(_) => { /* ... */ }

              MemorySection(_) => { /* ... */ }

              TagSection(_) => { /* ... */ }

              GlobalSection(_) => { /* ... */ }

              ExportSection(_) => { /* ... */ }

              StartSection { .. } => { /* ... */ }

              ElementSection(_) => { /* ... */ }

              DataCountSection { .. } => { /* ... */ }

              DataSection(_) => { /* ... */ }

  

              // Here we know how many functions we'll be receiving as

              // `CodeSectionEntry`, so we can prepare for that, and

              // afterwards we can parse and handle each function

              // individually.

              CodeSectionStart { .. } => { /* ... */ }

              CodeSectionEntry(body) => {

                  // here we can iterate over `body` to parse the function

                  // and its locals

              }

  

              // Sections for WebAssembly components

              InstanceSection(_) => { /* ... */ }

              CoreTypeSection(_) => { /* ... */ }

              ComponentInstanceSection(_) => { /* ... */ }

              ComponentAliasSection(_) => { /* ... */ }

              ComponentTypeSection(_) => { /* ... */ }

              ComponentCanonicalSection(_) => { /* ... */ }

              ComponentStartSection { .. } => { /* ... */ }

              ComponentImportSection(_) => { /* ... */ }

              ComponentExportSection(_) => { /* ... */ }

  

              ModuleSection { parser, .. }

              | ComponentSection { parser, .. } => {

                  stack.push(cur.clone());

                  cur = parser.clone();

              }

  

              CustomSection(_) => { /* ... */ }

  

              // Once we've reached the end of a parser we either resume

              // at the parent parser or we break out of the loop because

              // we're done.

              End(_) => {

                  if let Some(parent_parser) = stack.pop() {

                      cur = parent_parser;

                  } else {

                      break;

                  }

              }

  

              // most likely you'd return an error here

              _ => { /* ... */ }

          }

  

          // once we're done processing the payload we can forget the

          // original.

          buf.drain(..consumed);

      }

  

      Ok(())

  }

  

  parse(&b"\0asm\x01\0\0\0"[..]).unwrap();

  ```

- <span id="parser-update-order"></span>`fn update_order(&mut self, order: Order, pos: usize) -> Result<()>` — [`Order`](#order), [`Result`](../binary_reader/index.md#result)

- <span id="parser-parse-reader"></span>`fn parse_reader<'a>(&mut self, reader: &mut BinaryReader<'a>, eof: bool) -> Result<Payload<'a>>` — [`BinaryReader`](../binary_reader/index.md#binaryreader), [`Result`](../binary_reader/index.md#result), [`Payload`](../index.md#payload)

- <span id="parser-parse-all"></span>`fn parse_all(self, data: &[u8]) -> impl Iterator<Item = Result<Payload<'_>>>` — [`Result`](../binary_reader/index.md#result), [`Payload`](../index.md#payload)

  Convenience function that can be used to parse a module or component

  that is entirely resident in memory.

  

  This function will parse the `data` provided as a WebAssembly module

  or component.

  

  Note that when this function yields sections that provide parsers,

  no further action is required for those sections as payloads from

  those parsers will be automatically returned.

  

  # Examples

  

  An example of reading a wasm file from a stream (`std::io::Read`) into

  a buffer and then parsing it.

  

  ```rust

  use std::io::Read;

  use anyhow::Result;

  use wasmparser::{Parser, Chunk, Payload::*};

  

  fn parse(mut reader: impl Read) -> Result<()> {

      let mut buf = Vec::new();

      reader.read_to_end(&mut buf)?;

      let parser = Parser::new(0);

  

      for payload in parser.parse_all(&buf) {

          match payload? {

              // Sections for WebAssembly modules

              Version { .. } => { /* ... */ }

              TypeSection(_) => { /* ... */ }

              ImportSection(_) => { /* ... */ }

              FunctionSection(_) => { /* ... */ }

              TableSection(_) => { /* ... */ }

              MemorySection(_) => { /* ... */ }

              TagSection(_) => { /* ... */ }

              GlobalSection(_) => { /* ... */ }

              ExportSection(_) => { /* ... */ }

              StartSection { .. } => { /* ... */ }

              ElementSection(_) => { /* ... */ }

              DataCountSection { .. } => { /* ... */ }

              DataSection(_) => { /* ... */ }

  

              // Here we know how many functions we'll be receiving as

              // `CodeSectionEntry`, so we can prepare for that, and

              // afterwards we can parse and handle each function

              // individually.

              CodeSectionStart { .. } => { /* ... */ }

              CodeSectionEntry(body) => {

                  // here we can iterate over `body` to parse the function

                  // and its locals

              }

  

              // Sections for WebAssembly components

              ModuleSection { .. } => { /* ... */ }

              InstanceSection(_) => { /* ... */ }

              CoreTypeSection(_) => { /* ... */ }

              ComponentSection { .. } => { /* ... */ }

              ComponentInstanceSection(_) => { /* ... */ }

              ComponentAliasSection(_) => { /* ... */ }

              ComponentTypeSection(_) => { /* ... */ }

              ComponentCanonicalSection(_) => { /* ... */ }

              ComponentStartSection { .. } => { /* ... */ }

              ComponentImportSection(_) => { /* ... */ }

              ComponentExportSection(_) => { /* ... */ }

  

              CustomSection(_) => { /* ... */ }

  

              // Once we've reached the end of a parser we either resume

              // at the parent parser or the payload iterator is at its

              // end and we're done.

              End(_) => {}

  

              // most likely you'd return an error here, but if you want

              // you can also inspect the raw contents of unknown sections

              other => {

                  match other.as_section() {

                      Some((id, range)) => { /* ... */ }

                      None => { /* ... */ }

                  }

              }

          }

      }

  

      Ok(())

  }

  

  parse(&b"\0asm\x01\0\0\0"[..]).unwrap();

  ```

- <span id="parser-skip-section"></span>`fn skip_section(&mut self)`

  Skip parsing the code section entirely.

  

  This function can be used to indicate, after receiving

  `CodeSectionStart`, that the section will not be parsed.

  

  The caller will be responsible for skipping `size` bytes (found in the

  `CodeSectionStart` payload). Bytes should only be fed into `parse`

  after the `size` bytes have been skipped.

  

  # Panics

  

  This function will panic if the parser is not in a state where it's

  parsing the code section.

  

  # Examples

  

  ```rust

  use wasmparser::{Result, Parser, Chunk, Payload::*};

  use core::ops::Range;

  

  fn objdump_headers(mut wasm: &[u8]) -> Result<()> {

      let mut parser = Parser::new(0);

      loop {

          let payload = match parser.parse(wasm, true)? {

              Chunk::Parsed { consumed, payload } => {

                  wasm = &wasm[consumed..];

                  payload

              }

              // this state isn't possible with `eof = true`

              Chunk::NeedMoreData(_) => unreachable!(),

          };

          match payload {

              TypeSection(s) => print_range("type section", &s.range()),

              ImportSection(s) => print_range("import section", &s.range()),

              // .. other sections

  

              // Print the range of the code section we see, but don't

              // actually iterate over each individual function.

              CodeSectionStart { range, size, .. } => {

                  print_range("code section", &range);

                  parser.skip_section();

                  wasm = &wasm[size as usize..];

              }

              End(_) => break,

              _ => {}

          }

      }

      Ok(())

  }

  

  fn print_range(section: &str, range: &Range<usize>) {

      println!("{:>40}: {:#010x} - {:#010x}", section, range.start, range.end);

  }

  ```

- <span id="parser-check-function-code-counts"></span>`fn check_function_code_counts(&self, pos: usize) -> Result<()>` — [`Result`](../binary_reader/index.md#result)

- <span id="parser-check-data-count"></span>`fn check_data_count(&self, pos: usize) -> Result<()>` — [`Result`](../binary_reader/index.md#result)

#### Trait Implementations

##### `impl Clone for Parser`

- <span id="parser-clone"></span>`fn clone(&self) -> Parser` — [`Parser`](../index.md#parser)

##### `impl Debug for Parser`

- <span id="parser-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Parser`

- <span id="parser-default"></span>`fn default() -> Parser` — [`Parser`](../index.md#parser)

## Enums

### `Encoding`

```rust
enum Encoding {
    Module,
    Component,
}
```

The supported encoding formats for the parser.

#### Variants

- **`Module`**

  The encoding format is a WebAssembly module.

- **`Component`**

  The encoding format is a WebAssembly component.

#### Trait Implementations

##### `impl Clone for Encoding`

- <span id="encoding-clone"></span>`fn clone(&self) -> Encoding` — [`Encoding`](../index.md#encoding)

##### `impl Copy for Encoding`

##### `impl Debug for Encoding`

- <span id="encoding-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Encoding`

##### `impl PartialEq for Encoding`

- <span id="encoding-partialeq-eq"></span>`fn eq(&self, other: &Encoding) -> bool` — [`Encoding`](../index.md#encoding)

##### `impl StructuralPartialEq for Encoding`

### `Order`

```rust
enum Order {
    Initial,
    Type,
    Import,
    Function,
    Table,
    Memory,
    Tag,
    Global,
    Export,
    Start,
    Element,
    DataCount,
    Code,
    Data,
}
```

#### Trait Implementations

##### `impl Clone for Order`

- <span id="order-clone"></span>`fn clone(&self) -> Order` — [`Order`](#order)

##### `impl Copy for Order`

##### `impl Debug for Order`

- <span id="order-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Order`

- <span id="order-default"></span>`fn default() -> Order` — [`Order`](#order)

##### `impl Eq for Order`

##### `impl Ord for Order`

- <span id="order-ord-cmp"></span>`fn cmp(&self, other: &Order) -> cmp::Ordering` — [`Order`](#order)

##### `impl PartialEq for Order`

- <span id="order-partialeq-eq"></span>`fn eq(&self, other: &Order) -> bool` — [`Order`](#order)

##### `impl PartialOrd for Order`

- <span id="order-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Order) -> option::Option<cmp::Ordering>` — [`Order`](#order)

##### `impl StructuralPartialEq for Order`

### `State`

```rust
enum State {
    Header,
    SectionStart,
    FunctionBody {
        remaining: u32,
        len: u32,
    },
}
```

#### Trait Implementations

##### `impl Clone for State`

- <span id="state-clone"></span>`fn clone(&self) -> State` — [`State`](#state)

##### `impl Debug for State`

- <span id="state-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Chunk<'a>`

```rust
enum Chunk<'a> {
    NeedMoreData(u64),
    Parsed {
        consumed: usize,
        payload: Payload<'a>,
    },
}
```

A successful return payload from `Parser::parse`.

On success one of two possible values can be returned, either that more data
is needed to continue parsing or a chunk of the input was parsed, indicating
how much of it was parsed.

#### Variants

- **`NeedMoreData`**

  This can be returned at any time and indicates that more data is needed
  to proceed with parsing. Zero bytes were consumed from the input to
  `Parser::parse`. The `u64` value here is a hint as to how many more
  bytes are needed to continue parsing.

- **`Parsed`**

  A chunk was successfully parsed.

#### Trait Implementations

##### `impl Debug for Chunk<'a>`

- <span id="chunk-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Payload<'a>`

```rust
enum Payload<'a> {
    Version {
        num: u16,
        encoding: Encoding,
        range: core::ops::Range<usize>,
    },
    TypeSection(crate::TypeSectionReader<'a>),
    ImportSection(crate::ImportSectionReader<'a>),
    FunctionSection(crate::FunctionSectionReader<'a>),
    TableSection(crate::TableSectionReader<'a>),
    MemorySection(crate::MemorySectionReader<'a>),
    TagSection(crate::TagSectionReader<'a>),
    GlobalSection(crate::GlobalSectionReader<'a>),
    ExportSection(crate::ExportSectionReader<'a>),
    StartSection {
        func: u32,
        range: core::ops::Range<usize>,
    },
    ElementSection(crate::ElementSectionReader<'a>),
    DataCountSection {
        count: u32,
        range: core::ops::Range<usize>,
    },
    DataSection(crate::DataSectionReader<'a>),
    CodeSectionStart {
        count: u32,
        range: core::ops::Range<usize>,
        size: u32,
    },
    CodeSectionEntry(crate::FunctionBody<'a>),
    CustomSection(crate::CustomSectionReader<'a>),
    UnknownSection {
        id: u8,
        contents: &'a [u8],
        range: core::ops::Range<usize>,
    },
    End(usize),
}
```

Values that can be parsed from a WebAssembly module or component.

This enumeration is all possible chunks of pieces that can be parsed by a
[`Parser`](../index.md) from a binary WebAssembly module or component. Note that for many
sections the entire section is parsed all at once, whereas other functions,
like the code section, are parsed incrementally. This is a distinction where some
sections, like the type section, are required to be fully resident in memory
(fully downloaded) before proceeding. Other sections, like the code section,
can be processed in a streaming fashion where each function is extracted
individually so it can possibly be shipped to another thread while you wait
for more functions to get downloaded.

Note that payloads, when returned, do not indicate that the module or component
is valid. For example when you receive a `Payload::TypeSection` the type
section itself has not yet actually been parsed. The reader returned will be
able to parse it, but you'll have to actually iterate the reader to do the
full parse. Each payload returned is intended to be a *window* into the
original `data` passed to `Parser::parse` which can be further processed
if necessary.

#### Variants

- **`Version`**

  Indicates the header of a WebAssembly module or component.

- **`TypeSection`**

  A module type section was received and the provided reader can be
  used to parse the contents of the type section.

- **`ImportSection`**

  A module import section was received and the provided reader can be
  used to parse the contents of the import section.

- **`FunctionSection`**

  A module function section was received and the provided reader can be
  used to parse the contents of the function section.

- **`TableSection`**

  A module table section was received and the provided reader can be
  used to parse the contents of the table section.

- **`MemorySection`**

  A module memory section was received and the provided reader can be
  used to parse the contents of the memory section.

- **`TagSection`**

  A module tag section was received, and the provided reader can be
  used to parse the contents of the tag section.

- **`GlobalSection`**

  A module global section was received and the provided reader can be
  used to parse the contents of the global section.

- **`ExportSection`**

  A module export section was received, and the provided reader can be
  used to parse the contents of the export section.

- **`StartSection`**

  A module start section was received.

- **`ElementSection`**

  A module element section was received and the provided reader can be
  used to parse the contents of the element section.

- **`DataCountSection`**

  A module data count section was received.

- **`DataSection`**

  A module data section was received and the provided reader can be
  used to parse the contents of the data section.

- **`CodeSectionStart`**

  Indicator of the start of the code section of a WebAssembly module.
  
  This entry is returned whenever the code section starts. The `count`
  field indicates how many entries are in this code section. After
  receiving this start marker you're guaranteed that the next `count`
  items will be either `CodeSectionEntry` or an error will be returned.
  
  This, unlike other sections, is intended to be used for streaming the
  contents of the code section. The code section is not required to be
  fully resident in memory when we parse it. Instead a [`Parser`](../index.md) is
  capable of parsing piece-by-piece of a code section.

- **`CodeSectionEntry`**

  An entry of the code section, a function, was parsed from a WebAssembly
  module.
  
  This entry indicates that a function was successfully received from the
  code section, and the payload here is the window into the original input
  where the function resides. Note that the function itself has not been
  parsed, it's only been outlined. You'll need to process the
  `FunctionBody` provided to test whether it parses and/or is valid.

- **`CustomSection`**

  A module or component custom section was received.

- **`UnknownSection`**

  An unknown section was found.
  
  This variant is returned for all unknown sections encountered. This
  likely wants to be interpreted as an error by consumers of the parser,
  but this can also be used to parse sections currently unsupported by
  the parser.

- **`End`**

  The end of the WebAssembly module or component was reached.
  
  The value is the offset in the input byte stream where the end
  was reached.

#### Implementations

- <span id="payload-as-section"></span>`fn as_section(&self) -> Option<(u8, Range<usize>)>`

  If this `Payload` represents a section in the original wasm module then

  the section's id and range within the original wasm binary are returned.

  

  Not all payloads refer to entire sections, such as the `Version` and

  `CodeSectionEntry` variants. These variants will return `None` from this

  function.

  

  Otherwise this function will return `Some` where the first element is

  the byte identifier for the section and the second element is the range

  of the contents of the section within the original wasm binary.

  

  The purpose of this method is to enable tools to easily iterate over

  entire sections if necessary and handle sections uniformly, for example

  dropping custom sections while preserving all other sections.

#### Trait Implementations

##### `impl Debug for Payload<'_>`

- <span id="payload-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

### `usize_to_u64`

```rust
fn usize_to_u64(a: usize) -> u64
```

### `section`

```rust
fn section<'a, T>(reader: &mut crate::BinaryReader<'a>, len: u32, ctor: fn(crate::BinaryReader<'a>) -> crate::Result<T>, variant: fn(T) -> Payload<'a>) -> crate::Result<Payload<'a>>
```

Parses an entire section resident in memory into a `Payload`.

Requires that `len` bytes are resident in `reader` and uses `ctor`/`variant`
to construct the section to return.

### `single_item`

```rust
fn single_item<'a, T>(reader: &mut crate::BinaryReader<'a>, len: u32, desc: &str) -> crate::Result<(T, core::ops::Range<usize>)>
where
    T: FromReader<'a>
```

Reads a section that is represented by a single uleb-encoded `u32`.

### `delimited`

```rust
fn delimited<'a, T>(reader: &mut crate::BinaryReader<'a>, len: &mut u32, f: impl FnOnce(&mut crate::BinaryReader<'a>) -> crate::Result<T>) -> crate::Result<T>
```

Attempts to parse using `f`.

This will update `*len` with the number of bytes consumed, and it will cause
a failure to be returned instead of the number of bytes consumed exceeds
what `*len` currently is.

### `clear_hint`

```rust
fn clear_hint(err: crate::BinaryReaderError) -> crate::BinaryReaderError
```

## Constants

### `WASM_MODULE_VERSION`
```rust
const WASM_MODULE_VERSION: u16 = 1u16;
```

### `WASM_COMPONENT_VERSION`
```rust
const WASM_COMPONENT_VERSION: u16 = 13u16;
```

### `KIND_MODULE`
```rust
const KIND_MODULE: u16 = 0u16;
```

### `KIND_COMPONENT`
```rust
const KIND_COMPONENT: u16 = 1u16;
```

### `CUSTOM_SECTION`
```rust
const CUSTOM_SECTION: u8 = 0u8;
```

### `TYPE_SECTION`
```rust
const TYPE_SECTION: u8 = 1u8;
```

### `IMPORT_SECTION`
```rust
const IMPORT_SECTION: u8 = 2u8;
```

### `FUNCTION_SECTION`
```rust
const FUNCTION_SECTION: u8 = 3u8;
```

### `TABLE_SECTION`
```rust
const TABLE_SECTION: u8 = 4u8;
```

### `MEMORY_SECTION`
```rust
const MEMORY_SECTION: u8 = 5u8;
```

### `GLOBAL_SECTION`
```rust
const GLOBAL_SECTION: u8 = 6u8;
```

### `EXPORT_SECTION`
```rust
const EXPORT_SECTION: u8 = 7u8;
```

### `START_SECTION`
```rust
const START_SECTION: u8 = 8u8;
```

### `ELEMENT_SECTION`
```rust
const ELEMENT_SECTION: u8 = 9u8;
```

### `CODE_SECTION`
```rust
const CODE_SECTION: u8 = 10u8;
```

### `DATA_SECTION`
```rust
const DATA_SECTION: u8 = 11u8;
```

### `DATA_COUNT_SECTION`
```rust
const DATA_COUNT_SECTION: u8 = 12u8;
```

### `TAG_SECTION`
```rust
const TAG_SECTION: u8 = 13u8;
```

