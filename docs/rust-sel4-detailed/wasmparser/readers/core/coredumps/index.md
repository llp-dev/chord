*[wasmparser](../../../index.md) / [readers](../../index.md) / [core](../index.md) / [coredumps](index.md)*

---

# Module `coredumps`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`CoreDumpSection`](#coredumpsection) | struct | The data portion of a custom section representing a core dump. |
| [`CoreDumpModulesSection`](#coredumpmodulessection) | struct | The data portion of a "coremodules" custom section. |
| [`CoreDumpInstancesSection`](#coredumpinstancessection) | struct | A custom section representing the instances involved in a given coredump |
| [`CoreDumpInstance`](#coredumpinstance) | struct | A single instance from a coredump instances section |
| [`CoreDumpStackSection`](#coredumpstacksection) | struct | The data portion of a custom section representing a core dump stack. |
| [`CoreDumpStackFrame`](#coredumpstackframe) | struct | A single stack frame from a core dump |
| [`CoreDumpValue`](#coredumpvalue) | enum | Local and stack values are encoded using one byte for the type (similar to Wasm's Number Types) followed by bytes representing the actual value See the tool-conventions repo for more details. |

## Structs

### `CoreDumpSection<'a>`

```rust
struct CoreDumpSection<'a> {
    pub name: &'a str,
}
```

The data portion of a custom section representing a core dump. Per the
tool-conventions repo, this section just specifies the executable name that
the core dump came from while the rest of the core dump information is
contained in a corestack custom section

# Examples

```rust
use wasmparser::{BinaryReader, CoreDumpSection, FromReader, Result};
let data: &[u8] = &[0x00, 0x09, 0x74, 0x65, 0x73, 0x74, 0x2e, 0x77, 0x61,
     0x73, 0x6d];
let mut reader = BinaryReader::new(data, 0);
let core = CoreDumpSection::new(reader).unwrap();
assert!(core.name == "test.wasm")
```

#### Fields

- **`name`**: `&'a str`

  The name of the process that created the core dump

#### Implementations

- <span id="coredumpsection-new"></span>`fn new(reader: BinaryReader<'a>) -> Result<CoreDumpSection<'a>>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result), [`CoreDumpSection`](../index.md#coredumpsection)

  Parses this section from the provided `reader`, derived from a custom

  section.

### `CoreDumpModulesSection<'a>`

```rust
struct CoreDumpModulesSection<'a> {
    pub modules: Vec<&'a str>,
}
```

The data portion of a "coremodules" custom section. This contains a vec of
module names that will be referenced by index by other coredump sections.

# Example

```rust
use wasmparser::{BinaryReader, CoreDumpModulesSection, FromReader, Result};
let data: &[u8] = &[0x01, 0x00, 0x04, 0x74, 0x65, 0x73, 0x74];
let reader = BinaryReader::new(data, 0);
let modules_section = CoreDumpModulesSection::new(reader).unwrap();
assert!(modules_section.modules[0] == "test")
```

#### Fields

- **`modules`**: `Vec<&'a str>`

  A list of module names, which may be URLs, file paths, or other
  identifiers for the module.

#### Implementations

- <span id="coredumpmodulessection-new"></span>`fn new(reader: BinaryReader<'a>) -> Result<CoreDumpModulesSection<'a>>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result), [`CoreDumpModulesSection`](../index.md#coredumpmodulessection)

  Parses this section from the provided `reader`, derived from a custom

  section.

#### Trait Implementations

##### `impl Debug for CoreDumpModulesSection<'a>`

- <span id="coredumpmodulessection-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `CoreDumpInstancesSection`

```rust
struct CoreDumpInstancesSection {
    pub instances: Vec<CoreDumpInstance>,
}
```

A custom section representing the instances involved in a given coredump

#### Fields

- **`instances`**: `Vec<CoreDumpInstance>`

  The instances for the coredump

#### Implementations

- <span id="coredumpinstancessection-new"></span>`fn new(reader: BinaryReader<'_>) -> Result<CoreDumpInstancesSection>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result), [`CoreDumpInstancesSection`](../index.md#coredumpinstancessection)

  Parses this section from the provided `reader`, derived from a custom

  section.

### `CoreDumpInstance`

```rust
struct CoreDumpInstance {
    pub module_index: u32,
    pub memories: Vec<u32>,
    pub globals: Vec<u32>,
}
```

A single instance from a coredump instances section

#### Fields

- **`module_index`**: `u32`

  The module that this is an instance of, as an index into a "coremodules"
  section.

- **`memories`**: `Vec<u32>`

  Which of the coredump's memories are this instance's memories, via
  indexing into the memory index space.

- **`globals`**: `Vec<u32>`

  Which of the coredump's globals are this instance's globals, via
  indexing into the global index space.

#### Trait Implementations

##### `impl Debug for CoreDumpInstance`

- <span id="coredumpinstance-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FromReader for CoreDumpInstance`

- <span id="coredumpinstance-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

### `CoreDumpStackSection<'a>`

```rust
struct CoreDumpStackSection<'a> {
    pub name: &'a str,
    pub frames: Vec<CoreDumpStackFrame>,
}
```

The data portion of a custom section representing a core dump stack. The
structure of this follows the coredump spec in the tool-conventions repo

# Examples

```rust
use wasmparser::{BinaryReader, CoreDumpStackSection, FromReader};

let data: &[u8] = &[0x00, 0x04, 0x6d, 0x61, 0x69, 0x6e, 0x01, 0x00, 0x04,
    0x2a, 0x33, 0x01, 0x7f, 0x01, 0x01, 0x7f, 0x02];
let reader = BinaryReader::new(data, 0);
let corestack = CoreDumpStackSection::new(reader).unwrap();
assert!(corestack.name == "main");
assert!(corestack.frames.len() == 1);
let frame = &corestack.frames[0];
assert!(frame.instanceidx == 4);
assert!(frame.funcidx == 42);
assert!(frame.codeoffset == 51);
assert!(frame.locals.len() == 1);
assert!(frame.stack.len() == 1);
```

#### Fields

- **`name`**: `&'a str`

  The thread name

- **`frames`**: `Vec<CoreDumpStackFrame>`

  The stack frames for the core dump

#### Implementations

- <span id="coredumpstacksection-new"></span>`fn new(reader: BinaryReader<'a>) -> Result<CoreDumpStackSection<'a>>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result), [`CoreDumpStackSection`](../index.md#coredumpstacksection)

  Parses this section from the provided `reader`, derived from a custom

  section.

### `CoreDumpStackFrame`

```rust
struct CoreDumpStackFrame {
    pub instanceidx: u32,
    pub funcidx: u32,
    pub codeoffset: u32,
    pub locals: Vec<CoreDumpValue>,
    pub stack: Vec<CoreDumpValue>,
}
```

A single stack frame from a core dump

#### Fields

- **`instanceidx`**: `u32`

  The instance that this stack frame belongs to.

- **`funcidx`**: `u32`

  The function index in the module

- **`codeoffset`**: `u32`

  The instruction's offset relative to the function's start

- **`locals`**: `Vec<CoreDumpValue>`

  The locals for this stack frame (including function parameters)

- **`stack`**: `Vec<CoreDumpValue>`

  The values on the stack

#### Trait Implementations

##### `impl Debug for CoreDumpStackFrame`

- <span id="coredumpstackframe-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FromReader for CoreDumpStackFrame`

- <span id="coredumpstackframe-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

## Enums

### `CoreDumpValue`

```rust
enum CoreDumpValue {
    Missing,
    I32(i32),
    I64(i64),
    F32(crate::Ieee32),
    F64(crate::Ieee64),
}
```

Local and stack values are encoded using one byte for the type (similar to
Wasm's Number Types) followed by bytes representing the actual value
See the tool-conventions repo for more details.

#### Variants

- **`Missing`**

  A missing value (usually missing because it was optimized out)

- **`I32`**

  An i32 value

- **`I64`**

  An i64 value

- **`F32`**

  An f32 value

- **`F64`**

  An f64 value

#### Trait Implementations

##### `impl Clone for CoreDumpValue`

- <span id="coredumpvalue-clone"></span>`fn clone(&self) -> CoreDumpValue` — [`CoreDumpValue`](../index.md#coredumpvalue)

##### `impl Debug for CoreDumpValue`

- <span id="coredumpvalue-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FromReader for CoreDumpValue`

- <span id="coredumpvalue-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

