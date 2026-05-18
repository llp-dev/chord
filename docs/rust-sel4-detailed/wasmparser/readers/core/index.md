*[wasmparser](../../index.md) / [readers](../index.md) / [core](index.md)*

---

# Module `core`

## Contents

- [Modules](#modules)
  - [`branch_hinting`](#branch-hinting)
  - [`code`](#code)
  - [`coredumps`](#coredumps)
  - [`custom`](#custom)
  - [`data`](#data)
  - [`dylink0`](#dylink0)
  - [`elements`](#elements)
  - [`exports`](#exports)
  - [`functions`](#functions)
  - [`globals`](#globals)
  - [`imports`](#imports)
  - [`init`](#init)
  - [`linking`](#linking)
  - [`memories`](#memories)
  - [`names`](#names)
  - [`operators`](#operators)
  - [`producers`](#producers)
  - [`reloc`](#reloc)
  - [`tables`](#tables)
  - [`tags`](#tags)
  - [`types`](#types)
- [Structs](#structs)
  - [`BranchHintFunction`](#branchhintfunction)
  - [`BranchHint`](#branchhint)
  - [`FunctionBody`](#functionbody)
  - [`LocalsReader`](#localsreader)
  - [`LocalsIterator`](#localsiterator)
  - [`CoreDumpSection`](#coredumpsection)
  - [`CoreDumpModulesSection`](#coredumpmodulessection)
  - [`CoreDumpInstancesSection`](#coredumpinstancessection)
  - [`CoreDumpInstance`](#coredumpinstance)
  - [`CoreDumpStackSection`](#coredumpstacksection)
  - [`CoreDumpStackFrame`](#coredumpstackframe)
  - [`CustomSectionReader`](#customsectionreader)
  - [`Data`](#data)
  - [`MemInfo`](#meminfo)
  - [`ExportInfo`](#exportinfo)
  - [`ImportInfo`](#importinfo)
  - [`Element`](#element)
  - [`Export`](#export)
  - [`Global`](#global)
  - [`Import`](#import)
  - [`ConstExpr`](#constexpr)
  - [`SymbolFlags`](#symbolflags)
  - [`SegmentFlags`](#segmentflags)
  - [`LinkingSectionReader`](#linkingsectionreader)
  - [`Segment`](#segment)
  - [`InitFunc`](#initfunc)
  - [`Comdat`](#comdat)
  - [`ComdatSymbol`](#comdatsymbol)
  - [`DefinedDataSymbol`](#defineddatasymbol)
  - [`Naming`](#naming)
  - [`IndirectNaming`](#indirectnaming)
  - [`MemArg`](#memarg)
  - [`BrTable`](#brtable)
  - [`BrTableTargets`](#brtabletargets)
  - [`Ieee32`](#ieee32)
  - [`Ieee64`](#ieee64)
  - [`V128`](#v128)
  - [`ControlStack`](#controlstack)
  - [`FrameStackAdapter`](#framestackadapter)
  - [`SingleFrameAdapter`](#singleframeadapter)
  - [`OperatorsReader`](#operatorsreader)
  - [`OperatorsReaderAllocations`](#operatorsreaderallocations)
  - [`OperatorsIterator`](#operatorsiterator)
  - [`OperatorsIteratorWithOffsets`](#operatorsiteratorwithoffsets)
  - [`TryTable`](#trytable)
  - [`ResumeTable`](#resumetable)
  - [`OperatorFactory`](#operatorfactory)
  - [`ProducersField`](#producersfield)
  - [`ProducersFieldValue`](#producersfieldvalue)
  - [`RelocSectionReader`](#relocsectionreader)
  - [`RelocationEntry`](#relocationentry)
  - [`Table`](#table)
  - [`PackedIndex`](#packedindex)
  - [`RecGroup`](#recgroup)
  - [`SubType`](#subtype)
  - [`CompositeType`](#compositetype)
  - [`FuncType`](#functype)
  - [`ArrayType`](#arraytype)
  - [`FieldType`](#fieldtype)
  - [`StructType`](#structtype)
  - [`ContType`](#conttype)
  - [`RefType`](#reftype)
  - [`TableType`](#tabletype)
  - [`MemoryType`](#memorytype)
  - [`GlobalType`](#globaltype)
  - [`TagType`](#tagtype)
- [Enums](#enums)
  - [`CoreDumpValue`](#coredumpvalue)
  - [`KnownCustom`](#knowncustom)
  - [`DataKind`](#datakind)
  - [`Dylink0Subsection`](#dylink0subsection)
  - [`ElementKind`](#elementkind)
  - [`ElementItems`](#elementitems)
  - [`ExternalKind`](#externalkind)
  - [`TypeRef`](#typeref)
  - [`ComdatSymbolKind`](#comdatsymbolkind)
  - [`SymbolInfo`](#symbolinfo)
  - [`Linking`](#linking)
  - [`Name`](#name)
  - [`BlockType`](#blocktype)
  - [`FrameKind`](#framekind)
  - [`Ordering`](#ordering)
  - [`Operator`](#operator)
  - [`Catch`](#catch)
  - [`Handle`](#handle)
  - [`RelocationType`](#relocationtype)
  - [`RelocAddendKind`](#relocaddendkind)
  - [`TableInit`](#tableinit)
  - [`UnpackedIndex`](#unpackedindex)
  - [`RecGroupInner`](#recgroupinner)
  - [`CompositeInnerType`](#compositeinnertype)
  - [`StorageType`](#storagetype)
  - [`ValType`](#valtype)
  - [`HeapType`](#heaptype)
  - [`AbstractHeapType`](#abstractheaptype)
  - [`TagKind`](#tagkind)
- [Traits](#traits)
  - [`FrameStack`](#framestack)
  - [`VisitOperator`](#visitoperator)
- [Functions](#functions)
  - [`read_composite_type`](#read-composite-type)
- [Type Aliases](#type-aliases)
  - [`BranchHintSectionReader`](#branchhintsectionreader)
  - [`CodeSectionReader`](#codesectionreader)
  - [`DataSectionReader`](#datasectionreader)
  - [`Dylink0SectionReader`](#dylink0sectionreader)
  - [`ElementSectionReader`](#elementsectionreader)
  - [`ExportSectionReader`](#exportsectionreader)
  - [`FunctionSectionReader`](#functionsectionreader)
  - [`GlobalSectionReader`](#globalsectionreader)
  - [`ImportSectionReader`](#importsectionreader)
  - [`SegmentMap`](#segmentmap)
  - [`InitFuncMap`](#initfuncmap)
  - [`ComdatMap`](#comdatmap)
  - [`SymbolInfoMap`](#symbolinfomap)
  - [`MemorySectionReader`](#memorysectionreader)
  - [`NameMap`](#namemap)
  - [`IndirectNameMap`](#indirectnamemap)
  - [`NameSectionReader`](#namesectionreader)
  - [`ProducersSectionReader`](#producerssectionreader)
  - [`RelocationEntryReader`](#relocationentryreader)
  - [`TableSectionReader`](#tablesectionreader)
  - [`TagSectionReader`](#tagsectionreader)
  - [`TypeSectionReader`](#typesectionreader)
- [Constants](#constants)
  - [`WASM_DYLINK_MEM_INFO`](#wasm-dylink-mem-info)
  - [`WASM_DYLINK_NEEDED`](#wasm-dylink-needed)
  - [`WASM_DYLINK_EXPORT_INFO`](#wasm-dylink-export-info)
  - [`WASM_DYLINK_IMPORT_INFO`](#wasm-dylink-import-info)
  - [`WASM_DYLINK_RUNTIME_PATH`](#wasm-dylink-runtime-path)
- [Macros](#macros)
  - [`define_operator!`](#define-operator)
  - [`define_visit_operator!`](#define-visit-operator)
  - [`define_visit_operator_delegate!`](#define-visit-operator-delegate)
  - [`define_visit_operator!`](#define-visit-operator)
  - [`define_visit_operator_stack_adapter!`](#define-visit-operator-stack-adapter)
  - [`define_passthrough_visit_operator!`](#define-passthrough-visit-operator)
  - [`back_to_enum!`](#back-to-enum)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`branch_hinting`](#branch-hinting) | mod |  |
| [`code`](#code) | mod |  |
| [`coredumps`](#coredumps) | mod |  |
| [`custom`](#custom) | mod |  |
| [`data`](#data) | mod |  |
| [`dylink0`](#dylink0) | mod |  |
| [`elements`](#elements) | mod |  |
| [`exports`](#exports) | mod |  |
| [`functions`](#functions) | mod |  |
| [`globals`](#globals) | mod |  |
| [`imports`](#imports) | mod |  |
| [`init`](#init) | mod |  |
| [`linking`](#linking) | mod |  |
| [`memories`](#memories) | mod |  |
| [`names`](#names) | mod |  |
| [`operators`](#operators) | mod |  |
| [`producers`](#producers) | mod |  |
| [`reloc`](#reloc) | mod |  |
| [`tables`](#tables) | mod |  |
| [`tags`](#tags) | mod |  |
| [`types`](#types) | mod |  |
| [`BranchHintFunction`](#branchhintfunction) | struct | Branch hints for a single function. |
| [`BranchHint`](#branchhint) | struct | A hint for a single branch. |
| [`FunctionBody`](#functionbody) | struct | Represents a WebAssembly function body. |
| [`LocalsReader`](#localsreader) | struct | A reader for a function body's locals. |
| [`LocalsIterator`](#localsiterator) | struct | An iterator over locals in a function body. |
| [`CoreDumpSection`](#coredumpsection) | struct | The data portion of a custom section representing a core dump. |
| [`CoreDumpModulesSection`](#coredumpmodulessection) | struct | The data portion of a "coremodules" custom section. |
| [`CoreDumpInstancesSection`](#coredumpinstancessection) | struct | A custom section representing the instances involved in a given coredump |
| [`CoreDumpInstance`](#coredumpinstance) | struct | A single instance from a coredump instances section |
| [`CoreDumpStackSection`](#coredumpstacksection) | struct | The data portion of a custom section representing a core dump stack. |
| [`CoreDumpStackFrame`](#coredumpstackframe) | struct | A single stack frame from a core dump |
| [`CustomSectionReader`](#customsectionreader) | struct | A reader for custom sections of a WebAssembly module. |
| [`Data`](#data) | struct | Represents a data segment in a core WebAssembly module. |
| [`MemInfo`](#meminfo) | struct | Represents a `WASM_DYLINK_MEM_INFO` field |
| [`ExportInfo`](#exportinfo) | struct |  |
| [`ImportInfo`](#importinfo) | struct |  |
| [`Element`](#element) | struct | Represents a core WebAssembly element segment. |
| [`Export`](#export) | struct | Represents an export in a WebAssembly module. |
| [`Global`](#global) | struct | Represents a core WebAssembly global. |
| [`Import`](#import) | struct | Represents an import in a WebAssembly module. |
| [`ConstExpr`](#constexpr) | struct | Represents an initialization expression. |
| [`SymbolFlags`](#symbolflags) | struct | Flags for WebAssembly symbols. |
| [`SegmentFlags`](#segmentflags) | struct | Flags for WebAssembly segments. |
| [`LinkingSectionReader`](#linkingsectionreader) | struct | A reader for the `linking` custom section of a WebAssembly module. |
| [`Segment`](#segment) | struct | Represents extra metadata about the data segments. |
| [`InitFunc`](#initfunc) | struct | Represents an init function in the linking custom section. |
| [`Comdat`](#comdat) | struct | Represents [COMDAT](https://llvm.org/docs/LangRef.html#comdats) data in the linking custom section. |
| [`ComdatSymbol`](#comdatsymbol) | struct | Represents a symbol that is part of a comdat. |
| [`DefinedDataSymbol`](#defineddatasymbol) | struct | Represents the metadata about a data symbol defined in the wasm file. |
| [`Naming`](#naming) | struct | Represents a name for an index from the names section. |
| [`IndirectNaming`](#indirectnaming) | struct | Represents an indirect name in the names custom section. |
| [`MemArg`](#memarg) | struct | Represents a memory immediate in a WebAssembly memory instruction. |
| [`BrTable`](#brtable) | struct | A br_table entries representation. |
| [`BrTableTargets`](#brtabletargets) | struct | An iterator over the targets of a [`BrTable`]. |
| [`Ieee32`](#ieee32) | struct | An IEEE binary32 immediate floating point value, represented as a u32 containing the bit pattern. |
| [`Ieee64`](#ieee64) | struct | An IEEE binary64 immediate floating point value, represented as a u64 containing the bit pattern. |
| [`V128`](#v128) | struct | Represents a 128-bit vector value. |
| [`ControlStack`](#controlstack) | struct | The Wasm control stack for the [`OperatorsReader`]. |
| [`FrameStackAdapter`](#framestackadapter) | struct | Adapters from VisitOperators to FrameStacks |
| [`SingleFrameAdapter`](#singleframeadapter) | struct |  |
| [`OperatorsReader`](#operatorsreader) | struct | A reader for a core WebAssembly function's operators. |
| [`OperatorsReaderAllocations`](#operatorsreaderallocations) | struct | External handle to the internal allocations used by the OperatorsReader |
| [`OperatorsIterator`](#operatorsiterator) | struct | An iterator over a function's operators. |
| [`OperatorsIteratorWithOffsets`](#operatorsiteratorwithoffsets) | struct | An iterator over a function's operators with offsets. |
| [`TryTable`](#trytable) | struct | A `try_table` entries representation. |
| [`ResumeTable`](#resumetable) | struct | A representation of dispatch tables on `resume` and `resume_throw` instructions. |
| [`OperatorFactory`](#operatorfactory) | struct | A factory to construct [`Operator`] instances via the [`VisitOperator`] trait. |
| [`ProducersField`](#producersfield) | struct | A field from the producers custom section. |
| [`ProducersFieldValue`](#producersfieldvalue) | struct | Represents a field value in the producers custom section. |
| [`RelocSectionReader`](#relocsectionreader) | struct | Reader for reloc.* sections as defined by <https://github.com/WebAssembly/tool-conventions/blob/main/Linking.md#relocation-sections>. |
| [`RelocationEntry`](#relocationentry) | struct | Single relocation entry within a `reloc.*` section, as defined at <https://github.com/WebAssembly/tool-conventions/blob/main/Linking.md#relocation-sections>. |
| [`Table`](#table) | struct | Type information about a table defined in the table section of a WebAssembly module. |
| [`PackedIndex`](#packedindex) | struct | A packed representation of a type index. |
| [`RecGroup`](#recgroup) | struct | Represents a recursive type group in a WebAssembly module. |
| [`SubType`](#subtype) | struct | Represents a subtype of possible other types in a WebAssembly module. |
| [`CompositeType`](#compositetype) | struct | Represents a composite type in a WebAssembly module. |
| [`FuncType`](#functype) | struct | Represents a type of a function in a WebAssembly module. |
| [`ArrayType`](#arraytype) | struct | Represents a type of an array in a WebAssembly module. |
| [`FieldType`](#fieldtype) | struct | Represents a field type of an array or a struct. |
| [`StructType`](#structtype) | struct | Represents a type of a struct in a WebAssembly module. |
| [`ContType`](#conttype) | struct | Represents a type of a continuation in a WebAssembly module. |
| [`RefType`](#reftype) | struct | A reference type. |
| [`TableType`](#tabletype) | struct | Represents a table's type. |
| [`MemoryType`](#memorytype) | struct | Represents a memory's type. |
| [`GlobalType`](#globaltype) | struct | Represents a global's type. |
| [`TagType`](#tagtype) | struct | A tag's type. |
| [`CoreDumpValue`](#coredumpvalue) | enum | Local and stack values are encoded using one byte for the type (similar to Wasm's Number Types) followed by bytes representing the actual value See the tool-conventions repo for more details. |
| [`KnownCustom`](#knowncustom) | enum | Return value of [`CustomSectionReader::as_known`]. |
| [`DataKind`](#datakind) | enum | The kind of data segment. |
| [`Dylink0Subsection`](#dylink0subsection) | enum | Possible subsections of the `dylink.0` custom section. |
| [`ElementKind`](#elementkind) | enum | The kind of element segment. |
| [`ElementItems`](#elementitems) | enum | Represents the items of an element segment. |
| [`ExternalKind`](#externalkind) | enum | External types as defined [here]. |
| [`TypeRef`](#typeref) | enum | Represents a reference to a type definition in a WebAssembly module. |
| [`ComdatSymbolKind`](#comdatsymbolkind) | enum | Represents a symbol kind. |
| [`SymbolInfo`](#symbolinfo) | enum | Represents extra information about symbols in the linking custom section. |
| [`Linking`](#linking) | enum | Represents a subsection read from the linking custom section. |
| [`Name`](#name) | enum | Represents a name read from the names custom section. |
| [`BlockType`](#blocktype) | enum | Represents a block type. |
| [`FrameKind`](#framekind) | enum | The kind of a control flow `Frame`. |
| [`Ordering`](#ordering) | enum | Represents the memory ordering for atomic instructions. |
| [`Operator`](#operator) | enum | Instructions as defined [here]. |
| [`Catch`](#catch) | enum | Catch clauses that can be specified in [`TryTable`]. |
| [`Handle`](#handle) | enum | Handle clauses that can be specified in [`ResumeTable`]. |
| [`RelocationType`](#relocationtype) | enum | Relocation entry type. |
| [`RelocAddendKind`](#relocaddendkind) | enum | Indicates the kind of addend that applies to a relocation entry. |
| [`TableInit`](#tableinit) | enum | Different modes of initializing a table. |
| [`UnpackedIndex`](#unpackedindex) | enum | The uncompressed form of a `PackedIndex`. |
| [`RecGroupInner`](#recgroupinner) | enum |  |
| [`CompositeInnerType`](#compositeinnertype) | enum | A [`CompositeType`] can contain one of these types. |
| [`StorageType`](#storagetype) | enum | Represents storage types introduced in the GC spec for array and struct fields. |
| [`ValType`](#valtype) | enum | Represents the types of values in a WebAssembly module. |
| [`HeapType`](#heaptype) | enum | A heap type. |
| [`AbstractHeapType`](#abstractheaptype) | enum | An abstract heap type. |
| [`TagKind`](#tagkind) | enum | Represents a tag kind. |
| [`FrameStack`](#framestack) | trait | A trait representing the stack of frames within a function. |
| [`VisitOperator`](#visitoperator) | trait | Trait implemented by types that can visit all [`Operator`] variants. |
| [`read_composite_type`](#read-composite-type) | fn |  |
| [`BranchHintSectionReader`](#branchhintsectionreader) | type | A reader for the `metadata.code.branch_hint` custom section. |
| [`CodeSectionReader`](#codesectionreader) | type | A reader for the code section of a WebAssembly module. |
| [`DataSectionReader`](#datasectionreader) | type | A reader for the data section of a WebAssembly module. |
| [`Dylink0SectionReader`](#dylink0sectionreader) | type | Parser for the dynamic linking `dylink.0` custom section. |
| [`ElementSectionReader`](#elementsectionreader) | type | A reader for the element section of a WebAssembly module. |
| [`ExportSectionReader`](#exportsectionreader) | type | A reader for the export section of a WebAssembly module. |
| [`FunctionSectionReader`](#functionsectionreader) | type | A reader for the function section of a WebAssembly module. |
| [`GlobalSectionReader`](#globalsectionreader) | type | A reader for the global section of a WebAssembly module. |
| [`ImportSectionReader`](#importsectionreader) | type | A reader for the import section of a WebAssembly module. |
| [`SegmentMap`](#segmentmap) | type | Represents a reader for segments from the linking custom section. |
| [`InitFuncMap`](#initfuncmap) | type | Represents a reader for init functions from the linking custom section. |
| [`ComdatMap`](#comdatmap) | type | Represents a reader for COMDAT data from the linking custom section. |
| [`SymbolInfoMap`](#symbolinfomap) | type | Represents a reader for symbol info from the linking custom section. |
| [`MemorySectionReader`](#memorysectionreader) | type | A reader for the memory section of a WebAssembly module. |
| [`NameMap`](#namemap) | type | Represents a name map from the names custom section. |
| [`IndirectNameMap`](#indirectnamemap) | type | Represents a reader for indirect names from the names custom section. |
| [`NameSectionReader`](#namesectionreader) | type | A reader for the name custom section of a WebAssembly module. |
| [`ProducersSectionReader`](#producerssectionreader) | type | A reader for the producers custom section of a WebAssembly module. |
| [`RelocationEntryReader`](#relocationentryreader) | type | Reader for relocation entries within a `reloc.*` section. |
| [`TableSectionReader`](#tablesectionreader) | type | A reader for the table section of a WebAssembly module. |
| [`TagSectionReader`](#tagsectionreader) | type | A reader for the tags section of a WebAssembly module. |
| [`TypeSectionReader`](#typesectionreader) | type | A reader for the type section of a WebAssembly module. |
| [`WASM_DYLINK_MEM_INFO`](#wasm-dylink-mem-info) | const |  |
| [`WASM_DYLINK_NEEDED`](#wasm-dylink-needed) | const |  |
| [`WASM_DYLINK_EXPORT_INFO`](#wasm-dylink-export-info) | const |  |
| [`WASM_DYLINK_IMPORT_INFO`](#wasm-dylink-import-info) | const |  |
| [`WASM_DYLINK_RUNTIME_PATH`](#wasm-dylink-runtime-path) | const |  |
| [`define_operator!`](#define-operator) | macro |  |
| [`define_visit_operator!`](#define-visit-operator) | macro |  |
| [`define_visit_operator_delegate!`](#define-visit-operator-delegate) | macro |  |
| [`define_visit_operator!`](#define-visit-operator) | macro |  |
| [`define_visit_operator_stack_adapter!`](#define-visit-operator-stack-adapter) | macro |  |
| [`define_passthrough_visit_operator!`](#define-passthrough-visit-operator) | macro |  |
| [`back_to_enum!`](#back-to-enum) | macro |  |

## Modules

- [`branch_hinting`](branch_hinting/index.md)
- [`code`](code/index.md)
- [`coredumps`](coredumps/index.md)
- [`custom`](custom/index.md)
- [`data`](data/index.md)
- [`dylink0`](dylink0/index.md)
- [`elements`](elements/index.md)
- [`exports`](exports/index.md)
- [`functions`](functions/index.md)
- [`globals`](globals/index.md)
- [`imports`](imports/index.md)
- [`init`](init/index.md)
- [`linking`](linking/index.md)
- [`memories`](memories/index.md)
- [`names`](names/index.md)
- [`operators`](operators/index.md)
- [`producers`](producers/index.md)
- [`reloc`](reloc/index.md)
- [`tables`](tables/index.md)
- [`tags`](tags/index.md)
- [`types`](types/index.md)

## Structs

### `BranchHintFunction<'a>`

```rust
struct BranchHintFunction<'a> {
    pub func: u32,
    pub hints: crate::SectionLimited<'a, BranchHint>,
}
```

Branch hints for a single function.

Produced from [`BranchHintSectionReader`](#branchhintsectionreader).

#### Fields

- **`func`**: `u32`

  The function that these branch hints apply to.

- **`hints`**: `crate::SectionLimited<'a, BranchHint>`

  The branch hints available for this function.

#### Trait Implementations

##### `impl Clone for BranchHintFunction<'a>`

- <span id="branchhintfunction-clone"></span>`fn clone(&self) -> BranchHintFunction<'a>` — [`BranchHintFunction`](#branchhintfunction)

##### `impl Debug for BranchHintFunction<'a>`

- <span id="branchhintfunction-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FromReader for BranchHintFunction<'a>`

- <span id="branchhintfunction-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

### `BranchHint`

```rust
struct BranchHint {
    pub func_offset: u32,
    pub taken: bool,
}
```

A hint for a single branch.

#### Fields

- **`func_offset`**: `u32`

  The byte offset, from the start of the function's body, of where the
  hinted instruction lives.

- **`taken`**: `bool`

  Whether or not the branch is hinted to be taken or not.

#### Trait Implementations

##### `impl Clone for BranchHint`

- <span id="branchhint-clone"></span>`fn clone(&self) -> BranchHint` — [`BranchHint`](#branchhint)

##### `impl Copy for BranchHint`

##### `impl Debug for BranchHint`

- <span id="branchhint-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FromReader for BranchHint`

- <span id="branchhint-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

### `FunctionBody<'a>`

```rust
struct FunctionBody<'a> {
    reader: crate::BinaryReader<'a>,
}
```

Represents a WebAssembly function body.

#### Implementations

- <span id="functionbody-new"></span>`fn new(reader: BinaryReader<'a>) -> Self` — [`BinaryReader`](../../binary_reader/index.md#binaryreader)

  Constructs a new `FunctionBody` for the given data and offset.

- <span id="functionbody-get-binary-reader"></span>`fn get_binary_reader(&self) -> BinaryReader<'a>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader)

  Gets a binary reader for this function body.

- <span id="functionbody-skip-locals"></span>`fn skip_locals(reader: &mut BinaryReader<'_>) -> Result<()>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

- <span id="functionbody-get-locals-reader"></span>`fn get_locals_reader(&self) -> Result<LocalsReader<'a>>` — [`Result`](../../binary_reader/index.md#result), [`LocalsReader`](#localsreader)

  Gets the locals reader for this function body.

- <span id="functionbody-get-binary-reader-for-operators"></span>`fn get_binary_reader_for_operators(&self) -> Result<BinaryReader<'a>>` — [`Result`](../../binary_reader/index.md#result), [`BinaryReader`](../../binary_reader/index.md#binaryreader)

  Gets a binary reader for this function body, after skipping locals.

- <span id="functionbody-get-operators-reader"></span>`fn get_operators_reader(&self) -> Result<OperatorsReader<'a>>` — [`Result`](../../binary_reader/index.md#result), [`OperatorsReader`](#operatorsreader)

  Uses `FunctionBody::get_binary_reader_for_operators` and then converts

  that to an [`OperatorsReader`](#operatorsreader).

- <span id="functionbody-range"></span>`fn range(&self) -> Range<usize>`

  Gets the range of the function body.

- <span id="functionbody-as-bytes"></span>`fn as_bytes(&self) -> &'a [u8]`

  Returns the body of this function as a list of bytes.

  

  Note that the returned bytes start with the function locals declaration.

#### Trait Implementations

##### `impl Clone for FunctionBody<'a>`

- <span id="functionbody-clone"></span>`fn clone(&self) -> FunctionBody<'a>` — [`FunctionBody`](#functionbody)

##### `impl Debug for FunctionBody<'a>`

- <span id="functionbody-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FromReader for FunctionBody<'a>`

- <span id="functionbody-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

### `LocalsReader<'a>`

```rust
struct LocalsReader<'a> {
    reader: crate::BinaryReader<'a>,
    declaration_count: u32,
    total_count: u32,
}
```

A reader for a function body's locals.

#### Implementations

- <span id="localsreader-get-count"></span>`fn get_count(&self) -> u32`

  Gets the count of locals declarations in the function body.

- <span id="localsreader-original-position"></span>`fn original_position(&self) -> usize`

  Gets the original position of the reader.

- <span id="localsreader-read"></span>`fn read(&mut self) -> Result<(u32, ValType)>` — [`Result`](../../binary_reader/index.md#result), [`ValType`](#valtype)

  Reads an item from the reader.

- <span id="localsreader-get-binary-reader"></span>`fn get_binary_reader(self) -> BinaryReader<'a>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader)

  Gets the binary reader from this LocalsReader

#### Trait Implementations

##### `impl IntoIterator for LocalsReader<'a>`

- <span id="localsreader-intoiterator-type-item"></span>`type Item = Result<(u32, ValType), BinaryReaderError>`

- <span id="localsreader-intoiterator-type-intoiter"></span>`type IntoIter = LocalsIterator<'a>`

- <span id="localsreader-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

### `LocalsIterator<'a>`

```rust
struct LocalsIterator<'a> {
    reader: LocalsReader<'a>,
    left: u32,
    err: bool,
}
```

An iterator over locals in a function body.

#### Implementations

- <span id="localsiterator-into-binary-reader-for-operators"></span>`fn into_binary_reader_for_operators(self) -> BinaryReader<'a>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader)

  After reading the locals, the BinaryReader is ready to read the operators.

- <span id="localsiterator-into-operators-reader"></span>`fn into_operators_reader(self) -> OperatorsReader<'a>` — [`OperatorsReader`](#operatorsreader)

  After reading the locals, the BinaryReader is ready to read the operators.

#### Trait Implementations

##### `impl IntoIterator for LocalsIterator<'a>`

- <span id="localsiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="localsiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="localsiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for LocalsIterator<'a>`

- <span id="localsiterator-iterator-type-item"></span>`type Item = Result<(u32, ValType), BinaryReaderError>`

- <span id="localsiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="localsiterator-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

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

- <span id="coredumpsection-new"></span>`fn new(reader: BinaryReader<'a>) -> Result<CoreDumpSection<'a>>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result), [`CoreDumpSection`](#coredumpsection)

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

- <span id="coredumpmodulessection-new"></span>`fn new(reader: BinaryReader<'a>) -> Result<CoreDumpModulesSection<'a>>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result), [`CoreDumpModulesSection`](#coredumpmodulessection)

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

- <span id="coredumpinstancessection-new"></span>`fn new(reader: BinaryReader<'_>) -> Result<CoreDumpInstancesSection>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result), [`CoreDumpInstancesSection`](#coredumpinstancessection)

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

- <span id="coredumpinstance-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

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

- <span id="coredumpstacksection-new"></span>`fn new(reader: BinaryReader<'a>) -> Result<CoreDumpStackSection<'a>>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result), [`CoreDumpStackSection`](#coredumpstacksection)

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

- <span id="coredumpstackframe-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

### `CustomSectionReader<'a>`

```rust
struct CustomSectionReader<'a> {
    name: &'a str,
    reader: crate::BinaryReader<'a>,
}
```

A reader for custom sections of a WebAssembly module.

#### Implementations

- <span id="customsectionreader-new"></span>`fn new(reader: BinaryReader<'a>) -> Result<CustomSectionReader<'a>>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result), [`CustomSectionReader`](#customsectionreader)

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

- <span id="customsectionreader-as-known"></span>`fn as_known(&self) -> KnownCustom<'a>` — [`KnownCustom`](#knowncustom)

  Attempts to match and see if this custom section is statically known to

  `wasmparser` with any known section reader.

  

  This will inspect `self.name()` and return a [`KnownCustom`](#knowncustom) if the name

  matches a known custom section where there is a parser available for it.

  This can also be used as a convenience function for creating such

  parsers.

  

  If the custom section name is not known, or if a reader could not be

  created, then `KnownCustom::Unknown` is returned.

#### Trait Implementations

##### `impl Clone for CustomSectionReader<'a>`

- <span id="customsectionreader-clone"></span>`fn clone(&self) -> CustomSectionReader<'a>` — [`CustomSectionReader`](#customsectionreader)

##### `impl Debug for CustomSectionReader<'a>`

- <span id="customsectionreader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Data<'a>`

```rust
struct Data<'a> {
    pub kind: DataKind<'a>,
    pub data: &'a [u8],
    pub range: core::ops::Range<usize>,
}
```

Represents a data segment in a core WebAssembly module.

#### Fields

- **`kind`**: `DataKind<'a>`

  The kind of data segment.

- **`data`**: `&'a [u8]`

  The data of the data segment.

- **`range`**: `core::ops::Range<usize>`

  The range of the data segment.

#### Trait Implementations

##### `impl Clone for Data<'a>`

- <span id="data-clone"></span>`fn clone(&self) -> Data<'a>` — [`Data`](#data)

##### `impl Debug for Data<'a>`

- <span id="data-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FromReader for Data<'a>`

- <span id="data-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

### `MemInfo`

```rust
struct MemInfo {
    pub memory_size: u32,
    pub memory_alignment: u32,
    pub table_size: u32,
    pub table_alignment: u32,
}
```

Represents a `WASM_DYLINK_MEM_INFO` field

#### Fields

- **`memory_size`**: `u32`

  Size of the memory area the loader should reserve for the module, which
  will begin at `env.__memory_base`.

- **`memory_alignment`**: `u32`

  The required alignment of the memory area, in bytes, encoded as a power
  of 2.

- **`table_size`**: `u32`

  Size of the table area the loader should reserve for the module, which
  will begin at `env.__table_base`.

- **`table_alignment`**: `u32`

  The required alignment of the table area, in elements, encoded as a
  power of 2.

#### Trait Implementations

##### `impl Clone for MemInfo`

- <span id="meminfo-clone"></span>`fn clone(&self) -> MemInfo` — [`MemInfo`](#meminfo)

##### `impl Copy for MemInfo`

##### `impl Debug for MemInfo`

- <span id="meminfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ExportInfo<'a>`

```rust
struct ExportInfo<'a> {
    pub name: &'a str,
    pub flags: crate::SymbolFlags,
}
```

#### Trait Implementations

##### `impl Debug for ExportInfo<'a>`

- <span id="exportinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ImportInfo<'a>`

```rust
struct ImportInfo<'a> {
    pub module: &'a str,
    pub field: &'a str,
    pub flags: crate::SymbolFlags,
}
```

#### Trait Implementations

##### `impl Debug for ImportInfo<'a>`

- <span id="importinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Element<'a>`

```rust
struct Element<'a> {
    pub kind: ElementKind<'a>,
    pub items: ElementItems<'a>,
    pub range: core::ops::Range<usize>,
}
```

Represents a core WebAssembly element segment.

#### Fields

- **`kind`**: `ElementKind<'a>`

  The kind of the element segment.

- **`items`**: `ElementItems<'a>`

  The initial elements of the element segment.

- **`range`**: `core::ops::Range<usize>`

  The range of the the element segment.

#### Trait Implementations

##### `impl Clone for Element<'a>`

- <span id="element-clone"></span>`fn clone(&self) -> Element<'a>` — [`Element`](#element)

##### `impl FromReader for Element<'a>`

- <span id="element-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

### `Export<'a>`

```rust
struct Export<'a> {
    pub name: &'a str,
    pub kind: ExternalKind,
    pub index: u32,
}
```

Represents an export in a WebAssembly module.

#### Fields

- **`name`**: `&'a str`

  The name of the exported item.

- **`kind`**: `ExternalKind`

  The kind of the export.

- **`index`**: `u32`

  The index of the exported item.

#### Trait Implementations

##### `impl Clone for Export<'a>`

- <span id="export-clone"></span>`fn clone(&self) -> Export<'a>` — [`Export`](#export)

##### `impl Copy for Export<'a>`

##### `impl Debug for Export<'a>`

- <span id="export-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Export<'a>`

##### `impl FromReader for Export<'a>`

- <span id="export-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

##### `impl PartialEq for Export<'a>`

- <span id="export-partialeq-eq"></span>`fn eq(&self, other: &Export<'a>) -> bool` — [`Export`](#export)

##### `impl StructuralPartialEq for Export<'a>`

### `Global<'a>`

```rust
struct Global<'a> {
    pub ty: crate::GlobalType,
    pub init_expr: crate::ConstExpr<'a>,
}
```

Represents a core WebAssembly global.

#### Fields

- **`ty`**: `crate::GlobalType`

  The global's type.

- **`init_expr`**: `crate::ConstExpr<'a>`

  The global's initialization expression.

#### Trait Implementations

##### `impl Clone for Global<'a>`

- <span id="global-clone"></span>`fn clone(&self) -> Global<'a>` — [`Global`](#global)

##### `impl Debug for Global<'a>`

- <span id="global-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FromReader for Global<'a>`

- <span id="global-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

### `Import<'a>`

```rust
struct Import<'a> {
    pub module: &'a str,
    pub name: &'a str,
    pub ty: TypeRef,
}
```

Represents an import in a WebAssembly module.

#### Fields

- **`module`**: `&'a str`

  The module being imported from.

- **`name`**: `&'a str`

  The name of the imported item.

- **`ty`**: `TypeRef`

  The type of the imported item.

#### Trait Implementations

##### `impl Clone for Import<'a>`

- <span id="import-clone"></span>`fn clone(&self) -> Import<'a>` — [`Import`](#import)

##### `impl Copy for Import<'a>`

##### `impl Debug for Import<'a>`

- <span id="import-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Import<'a>`

##### `impl FromReader for Import<'a>`

- <span id="import-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

##### `impl PartialEq for Import<'a>`

- <span id="import-partialeq-eq"></span>`fn eq(&self, other: &Import<'a>) -> bool` — [`Import`](#import)

##### `impl StructuralPartialEq for Import<'a>`

### `ConstExpr<'a>`

```rust
struct ConstExpr<'a> {
    reader: crate::BinaryReader<'a>,
}
```

Represents an initialization expression.

#### Implementations

- <span id="constexpr-new"></span>`fn new(reader: BinaryReader<'a>) -> ConstExpr<'a>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`ConstExpr`](#constexpr)

  Constructs a new `ConstExpr` from the given data and offset.

- <span id="constexpr-get-binary-reader"></span>`fn get_binary_reader(&self) -> BinaryReader<'a>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader)

  Gets a binary reader for the initialization expression.

- <span id="constexpr-get-operators-reader"></span>`fn get_operators_reader(&self) -> OperatorsReader<'a>` — [`OperatorsReader`](#operatorsreader)

  Gets an operators parser for the initialization expression.

#### Trait Implementations

##### `impl Clone for ConstExpr<'a>`

- <span id="constexpr-clone"></span>`fn clone(&self) -> ConstExpr<'a>` — [`ConstExpr`](#constexpr)

##### `impl Debug for ConstExpr<'_>`

- <span id="constexpr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ConstExpr<'_>`

##### `impl FromReader for ConstExpr<'a>`

- <span id="constexpr-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

##### `impl PartialEq for ConstExpr<'_>`

- <span id="constexpr-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

### `SymbolFlags`

```rust
struct SymbolFlags(<SymbolFlags as __private::PublicFlags>::Internal);
```

Flags for WebAssembly symbols.

These flags correspond to those described in
<https://github.com/WebAssembly/tool-conventions/blob/main/Linking.md>
with the `WASM_SYM_*` prefix.

#### Implementations

- <span id="symbolflags-const-binding-weak"></span>`const BINDING_WEAK: Self`

- <span id="symbolflags-const-binding-local"></span>`const BINDING_LOCAL: Self`

- <span id="symbolflags-const-visibility-hidden"></span>`const VISIBILITY_HIDDEN: Self`

- <span id="symbolflags-const-undefined"></span>`const UNDEFINED: Self`

- <span id="symbolflags-const-exported"></span>`const EXPORTED: Self`

- <span id="symbolflags-const-explicit-name"></span>`const EXPLICIT_NAME: Self`

- <span id="symbolflags-const-no-strip"></span>`const NO_STRIP: Self`

- <span id="symbolflags-const-tls"></span>`const TLS: Self`

- <span id="symbolflags-const-absolute"></span>`const ABSOLUTE: Self`

#### Trait Implementations

##### `impl Binary for SymbolFlags`

- <span id="symbolflags-binary-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl BitAnd for SymbolFlags`

- <span id="symbolflags-bitand-type-output"></span>`type Output = SymbolFlags`

- <span id="symbolflags-bitand"></span>`fn bitand(self, other: Self) -> Self`

  The bitwise and (`&`) of the bits in two flags values.

##### `impl BitAndAssign for SymbolFlags`

- <span id="symbolflags-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, other: Self)`

  The bitwise and (`&`) of the bits in two flags values.

##### `impl BitOr for SymbolFlags`

- <span id="symbolflags-bitor-type-output"></span>`type Output = SymbolFlags`

- <span id="symbolflags-bitor"></span>`fn bitor(self, other: SymbolFlags) -> Self` — [`SymbolFlags`](#symbolflags)

  The bitwise or (`|`) of the bits in two flags values.

##### `impl BitOrAssign for SymbolFlags`

- <span id="symbolflags-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: Self)`

  The bitwise or (`|`) of the bits in two flags values.

##### `impl BitXor for SymbolFlags`

- <span id="symbolflags-bitxor-type-output"></span>`type Output = SymbolFlags`

- <span id="symbolflags-bitxor"></span>`fn bitxor(self, other: Self) -> Self`

  The bitwise exclusive-or (`^`) of the bits in two flags values.

##### `impl BitXorAssign for SymbolFlags`

- <span id="symbolflags-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: Self)`

  The bitwise exclusive-or (`^`) of the bits in two flags values.

##### `impl Clone for SymbolFlags`

- <span id="symbolflags-clone"></span>`fn clone(&self) -> SymbolFlags` — [`SymbolFlags`](#symbolflags)

##### `impl Copy for SymbolFlags`

##### `impl Debug for SymbolFlags`

- <span id="symbolflags-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for SymbolFlags`

- <span id="symbolflags-default"></span>`fn default() -> SymbolFlags` — [`SymbolFlags`](#symbolflags)

##### `impl Eq for SymbolFlags`

##### `impl Extend for SymbolFlags`

- <span id="symbolflags-extend"></span>`fn extend<T: __private::core::iter::IntoIterator<Item = Self>>(&mut self, iterator: T)`

  The bitwise or (`|`) of the bits in each flags value.

##### `impl Flags for SymbolFlags`

- <span id="symbolflags-flags-const-flags"></span>`const FLAGS: &'static [Flag<SymbolFlags>]`

- <span id="symbolflags-flags-type-bits"></span>`type Bits = u32`

- <span id="symbolflags-flags-bits"></span>`fn bits(&self) -> u32`

- <span id="symbolflags-flags-from-bits-retain"></span>`fn from_bits_retain(bits: u32) -> SymbolFlags` — [`SymbolFlags`](#symbolflags)

##### `impl FromIterator for SymbolFlags`

- <span id="symbolflags-fromiterator-from-iter"></span>`fn from_iter<T: __private::core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self`

  The bitwise or (`|`) of the bits in each flags value.

##### `impl FromReader for SymbolFlags`

- <span id="symbolflags-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

##### `impl Hash for SymbolFlags`

- <span id="symbolflags-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl IntoIterator for SymbolFlags`

- <span id="symbolflags-intoiterator-type-item"></span>`type Item = SymbolFlags`

- <span id="symbolflags-intoiterator-type-intoiter"></span>`type IntoIter = Iter<SymbolFlags>`

- <span id="symbolflags-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl LowerHex for SymbolFlags`

- <span id="symbolflags-lowerhex-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl Not for SymbolFlags`

- <span id="symbolflags-not-type-output"></span>`type Output = SymbolFlags`

- <span id="symbolflags-not"></span>`fn not(self) -> Self`

  The bitwise negation (`!`) of the bits in a flags value, truncating the result.

##### `impl Octal for SymbolFlags`

- <span id="symbolflags-octal-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl Ord for SymbolFlags`

- <span id="symbolflags-ord-cmp"></span>`fn cmp(&self, other: &SymbolFlags) -> cmp::Ordering` — [`SymbolFlags`](#symbolflags)

##### `impl PartialEq for SymbolFlags`

- <span id="symbolflags-partialeq-eq"></span>`fn eq(&self, other: &SymbolFlags) -> bool` — [`SymbolFlags`](#symbolflags)

##### `impl PartialOrd for SymbolFlags`

- <span id="symbolflags-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &SymbolFlags) -> option::Option<cmp::Ordering>` — [`SymbolFlags`](#symbolflags)

##### `impl PublicFlags for SymbolFlags`

- <span id="symbolflags-publicflags-type-primitive"></span>`type Primitive = u32`

- <span id="symbolflags-publicflags-type-internal"></span>`type Internal = InternalBitFlags`

##### `impl StructuralPartialEq for SymbolFlags`

##### `impl Sub for SymbolFlags`

- <span id="symbolflags-sub-type-output"></span>`type Output = SymbolFlags`

- <span id="symbolflags-sub"></span>`fn sub(self, other: Self) -> Self`

  The intersection of a source flags value with the complement of a target flags value (`&!`).

  

  This method is not equivalent to `self & !other` when `other` has unknown bits set.

  `difference` won't truncate `other`, but the `!` operator will.

##### `impl SubAssign for SymbolFlags`

- <span id="symbolflags-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: Self)`

  The intersection of a source flags value with the complement of a target flags value (`&!`).

  

  This method is not equivalent to `self & !other` when `other` has unknown bits set.

  `difference` won't truncate `other`, but the `!` operator will.

##### `impl UpperHex for SymbolFlags`

- <span id="symbolflags-upperhex-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

### `SegmentFlags`

```rust
struct SegmentFlags(<SegmentFlags as __private::PublicFlags>::Internal);
```

Flags for WebAssembly segments.

These flags are defined by implementation at the time of writing:
<https://github.com/llvm/llvm-project/blob/llvmorg-17.0.6/llvm/include/llvm/BinaryFormat/Wasm.h#L391-L394>

#### Implementations

- <span id="segmentflags-const-strings"></span>`const STRINGS: Self`

- <span id="segmentflags-const-tls"></span>`const TLS: Self`

#### Trait Implementations

##### `impl Binary for SegmentFlags`

- <span id="segmentflags-binary-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl BitAnd for SegmentFlags`

- <span id="segmentflags-bitand-type-output"></span>`type Output = SegmentFlags`

- <span id="segmentflags-bitand"></span>`fn bitand(self, other: Self) -> Self`

  The bitwise and (`&`) of the bits in two flags values.

##### `impl BitAndAssign for SegmentFlags`

- <span id="segmentflags-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, other: Self)`

  The bitwise and (`&`) of the bits in two flags values.

##### `impl BitOr for SegmentFlags`

- <span id="segmentflags-bitor-type-output"></span>`type Output = SegmentFlags`

- <span id="segmentflags-bitor"></span>`fn bitor(self, other: SegmentFlags) -> Self` — [`SegmentFlags`](#segmentflags)

  The bitwise or (`|`) of the bits in two flags values.

##### `impl BitOrAssign for SegmentFlags`

- <span id="segmentflags-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: Self)`

  The bitwise or (`|`) of the bits in two flags values.

##### `impl BitXor for SegmentFlags`

- <span id="segmentflags-bitxor-type-output"></span>`type Output = SegmentFlags`

- <span id="segmentflags-bitxor"></span>`fn bitxor(self, other: Self) -> Self`

  The bitwise exclusive-or (`^`) of the bits in two flags values.

##### `impl BitXorAssign for SegmentFlags`

- <span id="segmentflags-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: Self)`

  The bitwise exclusive-or (`^`) of the bits in two flags values.

##### `impl Clone for SegmentFlags`

- <span id="segmentflags-clone"></span>`fn clone(&self) -> SegmentFlags` — [`SegmentFlags`](#segmentflags)

##### `impl Copy for SegmentFlags`

##### `impl Debug for SegmentFlags`

- <span id="segmentflags-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for SegmentFlags`

- <span id="segmentflags-default"></span>`fn default() -> SegmentFlags` — [`SegmentFlags`](#segmentflags)

##### `impl Eq for SegmentFlags`

##### `impl Extend for SegmentFlags`

- <span id="segmentflags-extend"></span>`fn extend<T: __private::core::iter::IntoIterator<Item = Self>>(&mut self, iterator: T)`

  The bitwise or (`|`) of the bits in each flags value.

##### `impl Flags for SegmentFlags`

- <span id="segmentflags-flags-const-flags"></span>`const FLAGS: &'static [Flag<SegmentFlags>]`

- <span id="segmentflags-flags-type-bits"></span>`type Bits = u32`

- <span id="segmentflags-flags-bits"></span>`fn bits(&self) -> u32`

- <span id="segmentflags-flags-from-bits-retain"></span>`fn from_bits_retain(bits: u32) -> SegmentFlags` — [`SegmentFlags`](#segmentflags)

##### `impl FromIterator for SegmentFlags`

- <span id="segmentflags-fromiterator-from-iter"></span>`fn from_iter<T: __private::core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self`

  The bitwise or (`|`) of the bits in each flags value.

##### `impl FromReader for SegmentFlags`

- <span id="segmentflags-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

##### `impl Hash for SegmentFlags`

- <span id="segmentflags-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl IntoIterator for SegmentFlags`

- <span id="segmentflags-intoiterator-type-item"></span>`type Item = SegmentFlags`

- <span id="segmentflags-intoiterator-type-intoiter"></span>`type IntoIter = Iter<SegmentFlags>`

- <span id="segmentflags-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl LowerHex for SegmentFlags`

- <span id="segmentflags-lowerhex-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl Not for SegmentFlags`

- <span id="segmentflags-not-type-output"></span>`type Output = SegmentFlags`

- <span id="segmentflags-not"></span>`fn not(self) -> Self`

  The bitwise negation (`!`) of the bits in a flags value, truncating the result.

##### `impl Octal for SegmentFlags`

- <span id="segmentflags-octal-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl PartialEq for SegmentFlags`

- <span id="segmentflags-partialeq-eq"></span>`fn eq(&self, other: &SegmentFlags) -> bool` — [`SegmentFlags`](#segmentflags)

##### `impl PublicFlags for SegmentFlags`

- <span id="segmentflags-publicflags-type-primitive"></span>`type Primitive = u32`

- <span id="segmentflags-publicflags-type-internal"></span>`type Internal = InternalBitFlags`

##### `impl StructuralPartialEq for SegmentFlags`

##### `impl Sub for SegmentFlags`

- <span id="segmentflags-sub-type-output"></span>`type Output = SegmentFlags`

- <span id="segmentflags-sub"></span>`fn sub(self, other: Self) -> Self`

  The intersection of a source flags value with the complement of a target flags value (`&!`).

  

  This method is not equivalent to `self & !other` when `other` has unknown bits set.

  `difference` won't truncate `other`, but the `!` operator will.

##### `impl SubAssign for SegmentFlags`

- <span id="segmentflags-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: Self)`

  The intersection of a source flags value with the complement of a target flags value (`&!`).

  

  This method is not equivalent to `self & !other` when `other` has unknown bits set.

  `difference` won't truncate `other`, but the `!` operator will.

##### `impl UpperHex for SegmentFlags`

- <span id="segmentflags-upperhex-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

### `LinkingSectionReader<'a>`

```rust
struct LinkingSectionReader<'a> {
    version: u32,
    subsections: crate::Subsections<'a, Linking<'a>>,
    range: core::ops::Range<usize>,
}
```

A reader for the `linking` custom section of a WebAssembly module.

This format is currently defined upstream at
<https://github.com/WebAssembly/tool-conventions/blob/main/Linking.md>.

#### Fields

- **`version`**: `u32`

  The version of linking metadata contained in this section.

- **`subsections`**: `crate::Subsections<'a, Linking<'a>>`

  The subsections in this section.

- **`range`**: `core::ops::Range<usize>`

  The range of the entire section, including the version.

#### Implementations

- <span id="linkingsectionreader-new"></span>`fn new(reader: BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

  Creates a new reader for the linking section contents starting at

  `offset` within the original wasm file.

- <span id="linkingsectionreader-version"></span>`fn version(&self) -> u32`

  Returns the version of linking metadata contained in this section.

- <span id="linkingsectionreader-original-position"></span>`fn original_position(&self) -> usize`

  Returns the original byte offset of this section.

- <span id="linkingsectionreader-range"></span>`fn range(&self) -> Range<usize>`

  Returns the range, as byte offsets, of this section within the original

  wasm binary.

- <span id="linkingsectionreader-subsections"></span>`fn subsections(&self) -> Subsections<'a, Linking<'a>>` — [`Subsections`](../../index.md#subsections), [`Linking`](#linking)

  Returns the iterator for advancing through the subsections.

  

  You can also use `IntoIterator::into_iter` directly on this type.

#### Trait Implementations

##### `impl Clone for LinkingSectionReader<'a>`

- <span id="linkingsectionreader-clone"></span>`fn clone(&self) -> LinkingSectionReader<'a>` — [`LinkingSectionReader`](#linkingsectionreader)

##### `impl Debug for LinkingSectionReader<'a>`

- <span id="linkingsectionreader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for LinkingSectionReader<'a>`

- <span id="linkingsectionreader-intoiterator-type-item"></span>`type Item = Result<Linking<'a>, BinaryReaderError>`

- <span id="linkingsectionreader-intoiterator-type-intoiter"></span>`type IntoIter = Subsections<'a, Linking<'a>>`

- <span id="linkingsectionreader-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

### `Segment<'a>`

```rust
struct Segment<'a> {
    pub name: &'a str,
    pub alignment: u32,
    pub flags: SegmentFlags,
}
```

Represents extra metadata about the data segments.

#### Fields

- **`name`**: `&'a str`

  The name for the segment.

- **`alignment`**: `u32`

  The required alignment of the segment, encoded as a power of 2.

- **`flags`**: `SegmentFlags`

  The flags for the segment.

#### Trait Implementations

##### `impl Clone for Segment<'a>`

- <span id="segment-clone"></span>`fn clone(&self) -> Segment<'a>` — [`Segment`](#segment)

##### `impl Copy for Segment<'a>`

##### `impl Debug for Segment<'a>`

- <span id="segment-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FromReader for Segment<'a>`

- <span id="segment-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

### `InitFunc`

```rust
struct InitFunc {
    pub priority: u32,
    pub symbol_index: u32,
}
```

Represents an init function in the linking custom section.

#### Fields

- **`priority`**: `u32`

  The priority of the init function.

- **`symbol_index`**: `u32`

  The symbol index of init function (*not* the function index).

#### Trait Implementations

##### `impl Clone for InitFunc`

- <span id="initfunc-clone"></span>`fn clone(&self) -> InitFunc` — [`InitFunc`](#initfunc)

##### `impl Copy for InitFunc`

##### `impl Debug for InitFunc`

- <span id="initfunc-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FromReader for InitFunc`

- <span id="initfunc-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

### `Comdat<'a>`

```rust
struct Comdat<'a> {
    pub name: &'a str,
    pub flags: u32,
    pub symbols: crate::SectionLimited<'a, ComdatSymbol>,
}
```

Represents [COMDAT](https://llvm.org/docs/LangRef.html#comdats) data in the linking custom section.

#### Fields

- **`name`**: `&'a str`

  The name of this comdat.

- **`flags`**: `u32`

  The flags.

- **`symbols`**: `crate::SectionLimited<'a, ComdatSymbol>`

  The member symbols of this comdat.

#### Trait Implementations

##### `impl Clone for Comdat<'a>`

- <span id="comdat-clone"></span>`fn clone(&self) -> Comdat<'a>` — [`Comdat`](#comdat)

##### `impl Debug for Comdat<'a>`

- <span id="comdat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FromReader for Comdat<'a>`

- <span id="comdat-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

### `ComdatSymbol`

```rust
struct ComdatSymbol {
    pub kind: ComdatSymbolKind,
    pub index: u32,
}
```

Represents a symbol that is part of a comdat.

#### Fields

- **`kind`**: `ComdatSymbolKind`

  The kind of the symbol.

- **`index`**: `u32`

  The index of the symbol. Must not be an import.

#### Trait Implementations

##### `impl Clone for ComdatSymbol`

- <span id="comdatsymbol-clone"></span>`fn clone(&self) -> ComdatSymbol` — [`ComdatSymbol`](#comdatsymbol)

##### `impl Copy for ComdatSymbol`

##### `impl Debug for ComdatSymbol`

- <span id="comdatsymbol-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FromReader for ComdatSymbol`

- <span id="comdatsymbol-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

### `DefinedDataSymbol`

```rust
struct DefinedDataSymbol {
    pub index: u32,
    pub offset: u32,
    pub size: u32,
}
```

Represents the metadata about a data symbol defined in the wasm file.

#### Fields

- **`index`**: `u32`

  The index of the data segment.

- **`offset`**: `u32`

  The offset within the segment. Must be <= the segment's size.

- **`size`**: `u32`

  The size of the data, which can be zero. `offset + size` must be <= the segment's size.

#### Trait Implementations

##### `impl Clone for DefinedDataSymbol`

- <span id="defineddatasymbol-clone"></span>`fn clone(&self) -> DefinedDataSymbol` — [`DefinedDataSymbol`](#defineddatasymbol)

##### `impl Copy for DefinedDataSymbol`

##### `impl Debug for DefinedDataSymbol`

- <span id="defineddatasymbol-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FromReader for DefinedDataSymbol`

- <span id="defineddatasymbol-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

### `Naming<'a>`

```rust
struct Naming<'a> {
    pub index: u32,
    pub name: &'a str,
}
```

Represents a name for an index from the names section.

#### Fields

- **`index`**: `u32`

  The index being named.

- **`name`**: `&'a str`

  The name for the index.

#### Trait Implementations

##### `impl Clone for Naming<'a>`

- <span id="naming-clone"></span>`fn clone(&self) -> Naming<'a>` — [`Naming`](#naming)

##### `impl Copy for Naming<'a>`

##### `impl Debug for Naming<'a>`

- <span id="naming-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FromReader for Naming<'a>`

- <span id="naming-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

### `IndirectNaming<'a>`

```rust
struct IndirectNaming<'a> {
    pub index: u32,
    pub names: NameMap<'a>,
}
```

Represents an indirect name in the names custom section.

#### Fields

- **`index`**: `u32`

  The indirect index of the name.

- **`names`**: `NameMap<'a>`

  The map of names within the `index` prior.

#### Trait Implementations

##### `impl Clone for IndirectNaming<'a>`

- <span id="indirectnaming-clone"></span>`fn clone(&self) -> IndirectNaming<'a>` — [`IndirectNaming`](#indirectnaming)

##### `impl Debug for IndirectNaming<'a>`

- <span id="indirectnaming-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FromReader for IndirectNaming<'a>`

- <span id="indirectnaming-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

### `MemArg`

```rust
struct MemArg {
    pub align: u8,
    pub max_align: u8,
    pub offset: u64,
    pub memory: u32,
}
```

Represents a memory immediate in a WebAssembly memory instruction.

#### Fields

- **`align`**: `u8`

  Alignment, stored as `n` where the actual alignment is `2^n`

- **`max_align`**: `u8`

  Maximum alignment, stored as `n` where the actual alignment is `2^n`.
  
  Note that this field is not actually read from the binary format, it
  will be a constant depending on which instruction this `MemArg` is a
  payload for.

- **`offset`**: `u64`

  A fixed byte-offset that this memory immediate specifies.
  
  Note that the memory64 proposal can specify a full 64-bit byte offset
  while otherwise only 32-bit offsets are allowed. Once validated
  memory immediates for 32-bit memories are guaranteed to be at most
  `u32::MAX` whereas 64-bit memories can use the full 64-bits.

- **`memory`**: `u32`

  The index of the memory this immediate points to.
  
  Note that this points within the module's own memory index space, and
  is always zero unless the multi-memory proposal of WebAssembly is
  enabled.

#### Trait Implementations

##### `impl Clone for MemArg`

- <span id="memarg-clone"></span>`fn clone(&self) -> MemArg` — [`MemArg`](#memarg)

##### `impl Copy for MemArg`

##### `impl Debug for MemArg`

- <span id="memarg-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for MemArg`

##### `impl PartialEq for MemArg`

- <span id="memarg-partialeq-eq"></span>`fn eq(&self, other: &MemArg) -> bool` — [`MemArg`](#memarg)

##### `impl StructuralPartialEq for MemArg`

### `BrTable<'a>`

```rust
struct BrTable<'a> {
    reader: crate::BinaryReader<'a>,
    cnt: u32,
    default: u32,
}
```

A br_table entries representation.

#### Implementations

- <span id="brtable-len"></span>`fn len(&self) -> u32`

  Returns the number of `br_table` entries, not including the default

  label

- <span id="brtable-is-empty"></span>`fn is_empty(&self) -> bool`

  Returns whether `BrTable` doesn't have any labels apart from the default one.

- <span id="brtable-default"></span>`fn default(&self) -> u32`

  Returns the default target of this `br_table` instruction.

- <span id="brtable-targets"></span>`fn targets(&self) -> BrTableTargets<'_>` — [`BrTableTargets`](#brtabletargets)

  Returns the list of targets that this `br_table` instruction will be

  jumping to.

  

  This method will return an iterator which parses each target of this

  `br_table` except the default target. The returned iterator will

  yield `self.len()` elements.

  

  # Examples

  

  ```rust

  use wasmparser::{BinaryReader, OperatorsReader, Operator};

  

  let buf = [0x0e, 0x02, 0x01, 0x02, 0x00];

  let mut reader = OperatorsReader::new(BinaryReader::new(&buf, 0));

  let op = reader.read().unwrap();

  if let Operator::BrTable { targets } = op {

      let targets = targets.targets().collect::<Result<Vec<_>, _>>().unwrap();

      assert_eq!(targets, [1, 2]);

  }

  ```

#### Trait Implementations

##### `impl Clone for BrTable<'a>`

- <span id="brtable-clone"></span>`fn clone(&self) -> BrTable<'a>` — [`BrTable`](#brtable)

##### `impl Debug for BrTable<'_>`

- <span id="brtable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for BrTable<'_>`

##### `impl PartialEq for BrTable<'_>`

- <span id="brtable-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

### `BrTableTargets<'a>`

```rust
struct BrTableTargets<'a> {
    reader: crate::BinaryReader<'a>,
    remaining: u32,
}
```

An iterator over the targets of a [`BrTable`](#brtable).

# Note

This iterator parses each target of the underlying `br_table`
except for the default target.
The iterator will yield exactly as many targets as the `br_table` has.

#### Trait Implementations

##### `impl IntoIterator for BrTableTargets<'a>`

- <span id="brtabletargets-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="brtabletargets-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="brtabletargets-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for BrTableTargets<'a>`

- <span id="brtabletargets-iterator-type-item"></span>`type Item = Result<u32, BinaryReaderError>`

- <span id="brtabletargets-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="brtabletargets-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `Ieee32`

```rust
struct Ieee32(u32);
```

An IEEE binary32 immediate floating point value, represented as a u32
containing the bit pattern.

All bit patterns are allowed.

#### Implementations

- <span id="ieee32-bits"></span>`fn bits(self) -> u32`

  Gets the underlying bits of the 32-bit float.

#### Trait Implementations

##### `impl Clone for Ieee32`

- <span id="ieee32-clone"></span>`fn clone(&self) -> Ieee32` — [`Ieee32`](#ieee32)

##### `impl Copy for Ieee32`

##### `impl Debug for Ieee32`

- <span id="ieee32-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Ieee32`

##### `impl Hash for Ieee32`

- <span id="ieee32-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Ieee32`

- <span id="ieee32-partialeq-eq"></span>`fn eq(&self, other: &Ieee32) -> bool` — [`Ieee32`](#ieee32)

##### `impl StructuralPartialEq for Ieee32`

### `Ieee64`

```rust
struct Ieee64(u64);
```

An IEEE binary64 immediate floating point value, represented as a u64
containing the bit pattern.

All bit patterns are allowed.

#### Implementations

- <span id="ieee64-bits"></span>`fn bits(self) -> u64`

  Gets the underlying bits of the 64-bit float.

#### Trait Implementations

##### `impl Clone for Ieee64`

- <span id="ieee64-clone"></span>`fn clone(&self) -> Ieee64` — [`Ieee64`](#ieee64)

##### `impl Copy for Ieee64`

##### `impl Debug for Ieee64`

- <span id="ieee64-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Ieee64`

##### `impl Hash for Ieee64`

- <span id="ieee64-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Ieee64`

- <span id="ieee64-partialeq-eq"></span>`fn eq(&self, other: &Ieee64) -> bool` — [`Ieee64`](#ieee64)

##### `impl StructuralPartialEq for Ieee64`

### `V128`

```rust
struct V128([u8; 16]);
```

Represents a 128-bit vector value.

#### Implementations

- <span id="v128-bytes"></span>`fn bytes(&self) -> &[u8; 16]`

  Gets the bytes of the vector value.

- <span id="v128-i128"></span>`fn i128(&self) -> i128`

  Gets a signed 128-bit integer value from the vector's bytes.

#### Trait Implementations

##### `impl Clone for V128`

- <span id="v128-clone"></span>`fn clone(&self) -> V128` — [`V128`](#v128)

##### `impl Copy for V128`

##### `impl Debug for V128`

- <span id="v128-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for V128`

##### `impl Hash for V128`

- <span id="v128-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for V128`

- <span id="v128-partialeq-eq"></span>`fn eq(&self, other: &V128) -> bool` — [`V128`](#v128)

##### `impl StructuralPartialEq for V128`

### `ControlStack`

```rust
struct ControlStack {
    frames: Vec<FrameKind>,
    top: Option<FrameKind>,
}
```

The Wasm control stack for the [`OperatorsReader`](#operatorsreader).

#### Fields

- **`frames`**: `Vec<FrameKind>`

  All frames on the control stack exclusing the top-most frame.

- **`top`**: `Option<FrameKind>`

  The top-most frame on the control stack if any.

#### Implementations

- <span id="controlstack-clear"></span>`fn clear(&mut self)`

  Resets `self` but keeps heap allocations.

- <span id="controlstack-is-empty"></span>`fn is_empty(&self) -> bool`

  Returns `true` if `self` is empty.

- <span id="controlstack-push"></span>`fn push(&mut self, frame: FrameKind)` — [`FrameKind`](#framekind)

  Pushes the `frame` to `self`.

- <span id="controlstack-pop"></span>`fn pop(&mut self) -> Option<FrameKind>` — [`FrameKind`](#framekind)

  Pops the top-most [`FrameKind`](#framekind) from `self`.

- <span id="controlstack-last"></span>`fn last(&self) -> Option<FrameKind>` — [`FrameKind`](#framekind)

  Returns the top-mot [`FrameKind`](#framekind).

#### Trait Implementations

##### `impl Clone for ControlStack`

- <span id="controlstack-clone"></span>`fn clone(&self) -> ControlStack` — [`ControlStack`](#controlstack)

##### `impl Debug for ControlStack`

- <span id="controlstack-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ControlStack`

- <span id="controlstack-default"></span>`fn default() -> ControlStack` — [`ControlStack`](#controlstack)

### `FrameStackAdapter<'a, T>`

```rust
struct FrameStackAdapter<'a, T> {
    stack: &'a mut ControlStack,
    visitor: &'a mut T,
}
```

Adapters from VisitOperators to FrameStacks

#### Trait Implementations

##### `impl<T> FrameStack for FrameStackAdapter<'_, T>`

- <span id="framestackadapter-framestack-current-frame"></span>`fn current_frame(&self) -> Option<FrameKind>` — [`FrameKind`](#framekind)

##### `impl<T: VisitOperator<'a>> VisitOperator for FrameStackAdapter<'_, T>`

- <span id="framestackadapter-visitoperator-type-output"></span>`type Output = <T as VisitOperator>::Output`

- <span id="framestackadapter-visitoperator-visit-unreachable"></span>`fn visit_unreachable(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-nop"></span>`fn visit_nop(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-block"></span>`fn visit_block(&mut self, blockty: BlockType) -> <T as >::Output` — [`BlockType`](#blocktype), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-loop"></span>`fn visit_loop(&mut self, blockty: BlockType) -> <T as >::Output` — [`BlockType`](#blocktype), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-if"></span>`fn visit_if(&mut self, blockty: BlockType) -> <T as >::Output` — [`BlockType`](#blocktype), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-else"></span>`fn visit_else(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-end"></span>`fn visit_end(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-br"></span>`fn visit_br(&mut self, relative_depth: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-br-if"></span>`fn visit_br_if(&mut self, relative_depth: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-br-table"></span>`fn visit_br_table(&mut self, targets: BrTable<'a>) -> <T as >::Output` — [`BrTable`](#brtable), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-return"></span>`fn visit_return(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-call"></span>`fn visit_call(&mut self, function_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-call-indirect"></span>`fn visit_call_indirect(&mut self, type_index: u32, table_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-drop"></span>`fn visit_drop(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-select"></span>`fn visit_select(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-local-get"></span>`fn visit_local_get(&mut self, local_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-local-set"></span>`fn visit_local_set(&mut self, local_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-local-tee"></span>`fn visit_local_tee(&mut self, local_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-global-get"></span>`fn visit_global_get(&mut self, global_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-global-set"></span>`fn visit_global_set(&mut self, global_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-load"></span>`fn visit_i32_load(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-load"></span>`fn visit_i64_load(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-load"></span>`fn visit_f32_load(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-load"></span>`fn visit_f64_load(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-load8-s"></span>`fn visit_i32_load8_s(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-load8-u"></span>`fn visit_i32_load8_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-load16-s"></span>`fn visit_i32_load16_s(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-load16-u"></span>`fn visit_i32_load16_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-load8-s"></span>`fn visit_i64_load8_s(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-load8-u"></span>`fn visit_i64_load8_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-load16-s"></span>`fn visit_i64_load16_s(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-load16-u"></span>`fn visit_i64_load16_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-load32-s"></span>`fn visit_i64_load32_s(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-load32-u"></span>`fn visit_i64_load32_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-store"></span>`fn visit_i32_store(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-store"></span>`fn visit_i64_store(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-store"></span>`fn visit_f32_store(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-store"></span>`fn visit_f64_store(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-store8"></span>`fn visit_i32_store8(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-store16"></span>`fn visit_i32_store16(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-store8"></span>`fn visit_i64_store8(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-store16"></span>`fn visit_i64_store16(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-store32"></span>`fn visit_i64_store32(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-memory-size"></span>`fn visit_memory_size(&mut self, mem: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-memory-grow"></span>`fn visit_memory_grow(&mut self, mem: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-const"></span>`fn visit_i32_const(&mut self, value: i32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-const"></span>`fn visit_i64_const(&mut self, value: i64) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-const"></span>`fn visit_f32_const(&mut self, value: Ieee32) -> <T as >::Output` — [`Ieee32`](#ieee32), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-const"></span>`fn visit_f64_const(&mut self, value: Ieee64) -> <T as >::Output` — [`Ieee64`](#ieee64), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-eqz"></span>`fn visit_i32_eqz(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-eq"></span>`fn visit_i32_eq(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-ne"></span>`fn visit_i32_ne(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-lt-s"></span>`fn visit_i32_lt_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-lt-u"></span>`fn visit_i32_lt_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-gt-s"></span>`fn visit_i32_gt_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-gt-u"></span>`fn visit_i32_gt_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-le-s"></span>`fn visit_i32_le_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-le-u"></span>`fn visit_i32_le_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-ge-s"></span>`fn visit_i32_ge_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-ge-u"></span>`fn visit_i32_ge_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-eqz"></span>`fn visit_i64_eqz(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-eq"></span>`fn visit_i64_eq(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-ne"></span>`fn visit_i64_ne(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-lt-s"></span>`fn visit_i64_lt_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-lt-u"></span>`fn visit_i64_lt_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-gt-s"></span>`fn visit_i64_gt_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-gt-u"></span>`fn visit_i64_gt_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-le-s"></span>`fn visit_i64_le_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-le-u"></span>`fn visit_i64_le_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-ge-s"></span>`fn visit_i64_ge_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-ge-u"></span>`fn visit_i64_ge_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-eq"></span>`fn visit_f32_eq(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-ne"></span>`fn visit_f32_ne(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-lt"></span>`fn visit_f32_lt(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-gt"></span>`fn visit_f32_gt(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-le"></span>`fn visit_f32_le(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-ge"></span>`fn visit_f32_ge(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-eq"></span>`fn visit_f64_eq(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-ne"></span>`fn visit_f64_ne(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-lt"></span>`fn visit_f64_lt(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-gt"></span>`fn visit_f64_gt(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-le"></span>`fn visit_f64_le(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-ge"></span>`fn visit_f64_ge(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-clz"></span>`fn visit_i32_clz(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-ctz"></span>`fn visit_i32_ctz(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-popcnt"></span>`fn visit_i32_popcnt(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-add"></span>`fn visit_i32_add(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-sub"></span>`fn visit_i32_sub(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-mul"></span>`fn visit_i32_mul(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-div-s"></span>`fn visit_i32_div_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-div-u"></span>`fn visit_i32_div_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-rem-s"></span>`fn visit_i32_rem_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-rem-u"></span>`fn visit_i32_rem_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-and"></span>`fn visit_i32_and(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-or"></span>`fn visit_i32_or(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-xor"></span>`fn visit_i32_xor(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-shl"></span>`fn visit_i32_shl(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-shr-s"></span>`fn visit_i32_shr_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-shr-u"></span>`fn visit_i32_shr_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-rotl"></span>`fn visit_i32_rotl(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-rotr"></span>`fn visit_i32_rotr(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-clz"></span>`fn visit_i64_clz(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-ctz"></span>`fn visit_i64_ctz(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-popcnt"></span>`fn visit_i64_popcnt(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-add"></span>`fn visit_i64_add(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-sub"></span>`fn visit_i64_sub(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-mul"></span>`fn visit_i64_mul(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-div-s"></span>`fn visit_i64_div_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-div-u"></span>`fn visit_i64_div_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-rem-s"></span>`fn visit_i64_rem_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-rem-u"></span>`fn visit_i64_rem_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-and"></span>`fn visit_i64_and(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-or"></span>`fn visit_i64_or(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-xor"></span>`fn visit_i64_xor(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-shl"></span>`fn visit_i64_shl(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-shr-s"></span>`fn visit_i64_shr_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-shr-u"></span>`fn visit_i64_shr_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-rotl"></span>`fn visit_i64_rotl(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-rotr"></span>`fn visit_i64_rotr(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-abs"></span>`fn visit_f32_abs(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-neg"></span>`fn visit_f32_neg(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-ceil"></span>`fn visit_f32_ceil(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-floor"></span>`fn visit_f32_floor(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-trunc"></span>`fn visit_f32_trunc(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-nearest"></span>`fn visit_f32_nearest(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-sqrt"></span>`fn visit_f32_sqrt(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-add"></span>`fn visit_f32_add(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-sub"></span>`fn visit_f32_sub(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-mul"></span>`fn visit_f32_mul(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-div"></span>`fn visit_f32_div(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-min"></span>`fn visit_f32_min(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-max"></span>`fn visit_f32_max(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-copysign"></span>`fn visit_f32_copysign(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-abs"></span>`fn visit_f64_abs(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-neg"></span>`fn visit_f64_neg(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-ceil"></span>`fn visit_f64_ceil(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-floor"></span>`fn visit_f64_floor(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-trunc"></span>`fn visit_f64_trunc(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-nearest"></span>`fn visit_f64_nearest(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-sqrt"></span>`fn visit_f64_sqrt(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-add"></span>`fn visit_f64_add(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-sub"></span>`fn visit_f64_sub(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-mul"></span>`fn visit_f64_mul(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-div"></span>`fn visit_f64_div(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-min"></span>`fn visit_f64_min(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-max"></span>`fn visit_f64_max(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-copysign"></span>`fn visit_f64_copysign(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-wrap-i64"></span>`fn visit_i32_wrap_i64(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-trunc-f32-s"></span>`fn visit_i32_trunc_f32_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-trunc-f32-u"></span>`fn visit_i32_trunc_f32_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-trunc-f64-s"></span>`fn visit_i32_trunc_f64_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-trunc-f64-u"></span>`fn visit_i32_trunc_f64_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-extend-i32-s"></span>`fn visit_i64_extend_i32_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-extend-i32-u"></span>`fn visit_i64_extend_i32_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-trunc-f32-s"></span>`fn visit_i64_trunc_f32_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-trunc-f32-u"></span>`fn visit_i64_trunc_f32_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-trunc-f64-s"></span>`fn visit_i64_trunc_f64_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-trunc-f64-u"></span>`fn visit_i64_trunc_f64_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-convert-i32-s"></span>`fn visit_f32_convert_i32_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-convert-i32-u"></span>`fn visit_f32_convert_i32_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-convert-i64-s"></span>`fn visit_f32_convert_i64_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-convert-i64-u"></span>`fn visit_f32_convert_i64_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-demote-f64"></span>`fn visit_f32_demote_f64(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-convert-i32-s"></span>`fn visit_f64_convert_i32_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-convert-i32-u"></span>`fn visit_f64_convert_i32_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-convert-i64-s"></span>`fn visit_f64_convert_i64_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-convert-i64-u"></span>`fn visit_f64_convert_i64_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-promote-f32"></span>`fn visit_f64_promote_f32(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-reinterpret-f32"></span>`fn visit_i32_reinterpret_f32(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-reinterpret-f64"></span>`fn visit_i64_reinterpret_f64(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-reinterpret-i32"></span>`fn visit_f32_reinterpret_i32(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-reinterpret-i64"></span>`fn visit_f64_reinterpret_i64(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-extend8-s"></span>`fn visit_i32_extend8_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-extend16-s"></span>`fn visit_i32_extend16_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-extend8-s"></span>`fn visit_i64_extend8_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-extend16-s"></span>`fn visit_i64_extend16_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-extend32-s"></span>`fn visit_i64_extend32_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-ref-eq"></span>`fn visit_ref_eq(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-struct-new"></span>`fn visit_struct_new(&mut self, struct_type_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-struct-new-default"></span>`fn visit_struct_new_default(&mut self, struct_type_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-struct-get"></span>`fn visit_struct_get(&mut self, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-struct-get-s"></span>`fn visit_struct_get_s(&mut self, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-struct-get-u"></span>`fn visit_struct_get_u(&mut self, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-struct-set"></span>`fn visit_struct_set(&mut self, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-array-new"></span>`fn visit_array_new(&mut self, array_type_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-array-new-default"></span>`fn visit_array_new_default(&mut self, array_type_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-array-new-fixed"></span>`fn visit_array_new_fixed(&mut self, array_type_index: u32, array_size: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-array-new-data"></span>`fn visit_array_new_data(&mut self, array_type_index: u32, array_data_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-array-new-elem"></span>`fn visit_array_new_elem(&mut self, array_type_index: u32, array_elem_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-array-get"></span>`fn visit_array_get(&mut self, array_type_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-array-get-s"></span>`fn visit_array_get_s(&mut self, array_type_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-array-get-u"></span>`fn visit_array_get_u(&mut self, array_type_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-array-set"></span>`fn visit_array_set(&mut self, array_type_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-array-len"></span>`fn visit_array_len(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-array-fill"></span>`fn visit_array_fill(&mut self, array_type_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-array-copy"></span>`fn visit_array_copy(&mut self, array_type_index_dst: u32, array_type_index_src: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-array-init-data"></span>`fn visit_array_init_data(&mut self, array_type_index: u32, array_data_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-array-init-elem"></span>`fn visit_array_init_elem(&mut self, array_type_index: u32, array_elem_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-ref-test-non-null"></span>`fn visit_ref_test_non_null(&mut self, hty: HeapType) -> <T as >::Output` — [`HeapType`](#heaptype), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-ref-test-nullable"></span>`fn visit_ref_test_nullable(&mut self, hty: HeapType) -> <T as >::Output` — [`HeapType`](#heaptype), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-ref-cast-non-null"></span>`fn visit_ref_cast_non_null(&mut self, hty: HeapType) -> <T as >::Output` — [`HeapType`](#heaptype), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-ref-cast-nullable"></span>`fn visit_ref_cast_nullable(&mut self, hty: HeapType) -> <T as >::Output` — [`HeapType`](#heaptype), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-br-on-cast"></span>`fn visit_br_on_cast(&mut self, relative_depth: u32, from_ref_type: RefType, to_ref_type: RefType) -> <T as >::Output` — [`RefType`](#reftype), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-br-on-cast-fail"></span>`fn visit_br_on_cast_fail(&mut self, relative_depth: u32, from_ref_type: RefType, to_ref_type: RefType) -> <T as >::Output` — [`RefType`](#reftype), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-any-convert-extern"></span>`fn visit_any_convert_extern(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-extern-convert-any"></span>`fn visit_extern_convert_any(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-ref-i31"></span>`fn visit_ref_i31(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i31-get-s"></span>`fn visit_i31_get_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i31-get-u"></span>`fn visit_i31_get_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-struct-new-desc"></span>`fn visit_struct_new_desc(&mut self, struct_type_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-struct-new-default-desc"></span>`fn visit_struct_new_default_desc(&mut self, struct_type_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-ref-get-desc"></span>`fn visit_ref_get_desc(&mut self, type_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-ref-cast-desc-non-null"></span>`fn visit_ref_cast_desc_non_null(&mut self, hty: HeapType) -> <T as >::Output` — [`HeapType`](#heaptype), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-ref-cast-desc-nullable"></span>`fn visit_ref_cast_desc_nullable(&mut self, hty: HeapType) -> <T as >::Output` — [`HeapType`](#heaptype), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-br-on-cast-desc"></span>`fn visit_br_on_cast_desc(&mut self, relative_depth: u32, from_ref_type: RefType, to_ref_type: RefType) -> <T as >::Output` — [`RefType`](#reftype), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-br-on-cast-desc-fail"></span>`fn visit_br_on_cast_desc_fail(&mut self, relative_depth: u32, from_ref_type: RefType, to_ref_type: RefType) -> <T as >::Output` — [`RefType`](#reftype), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-trunc-sat-f32-s"></span>`fn visit_i32_trunc_sat_f32_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-trunc-sat-f32-u"></span>`fn visit_i32_trunc_sat_f32_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-trunc-sat-f64-s"></span>`fn visit_i32_trunc_sat_f64_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-trunc-sat-f64-u"></span>`fn visit_i32_trunc_sat_f64_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-trunc-sat-f32-s"></span>`fn visit_i64_trunc_sat_f32_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-trunc-sat-f32-u"></span>`fn visit_i64_trunc_sat_f32_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-trunc-sat-f64-s"></span>`fn visit_i64_trunc_sat_f64_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-trunc-sat-f64-u"></span>`fn visit_i64_trunc_sat_f64_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-memory-init"></span>`fn visit_memory_init(&mut self, data_index: u32, mem: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-data-drop"></span>`fn visit_data_drop(&mut self, data_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-memory-copy"></span>`fn visit_memory_copy(&mut self, dst_mem: u32, src_mem: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-memory-fill"></span>`fn visit_memory_fill(&mut self, mem: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-table-init"></span>`fn visit_table_init(&mut self, elem_index: u32, table: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-elem-drop"></span>`fn visit_elem_drop(&mut self, elem_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-table-copy"></span>`fn visit_table_copy(&mut self, dst_table: u32, src_table: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-typed-select"></span>`fn visit_typed_select(&mut self, ty: ValType) -> <T as >::Output` — [`ValType`](#valtype), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-typed-select-multi"></span>`fn visit_typed_select_multi(&mut self, tys: Vec<ValType>) -> <T as >::Output` — [`Vec`](../../prelude/index.md#vec), [`ValType`](#valtype), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-ref-null"></span>`fn visit_ref_null(&mut self, hty: HeapType) -> <T as >::Output` — [`HeapType`](#heaptype), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-ref-is-null"></span>`fn visit_ref_is_null(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-ref-func"></span>`fn visit_ref_func(&mut self, function_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-table-fill"></span>`fn visit_table_fill(&mut self, table: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-table-get"></span>`fn visit_table_get(&mut self, table: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-table-set"></span>`fn visit_table_set(&mut self, table: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-table-grow"></span>`fn visit_table_grow(&mut self, table: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-table-size"></span>`fn visit_table_size(&mut self, table: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-return-call"></span>`fn visit_return_call(&mut self, function_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-return-call-indirect"></span>`fn visit_return_call_indirect(&mut self, type_index: u32, table_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-memory-discard"></span>`fn visit_memory_discard(&mut self, mem: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-memory-atomic-notify"></span>`fn visit_memory_atomic_notify(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-memory-atomic-wait32"></span>`fn visit_memory_atomic_wait32(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-memory-atomic-wait64"></span>`fn visit_memory_atomic_wait64(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-atomic-fence"></span>`fn visit_atomic_fence(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-load"></span>`fn visit_i32_atomic_load(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-load"></span>`fn visit_i64_atomic_load(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-load8-u"></span>`fn visit_i32_atomic_load8_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-load16-u"></span>`fn visit_i32_atomic_load16_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-load8-u"></span>`fn visit_i64_atomic_load8_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-load16-u"></span>`fn visit_i64_atomic_load16_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-load32-u"></span>`fn visit_i64_atomic_load32_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-store"></span>`fn visit_i32_atomic_store(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-store"></span>`fn visit_i64_atomic_store(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-store8"></span>`fn visit_i32_atomic_store8(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-store16"></span>`fn visit_i32_atomic_store16(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-store8"></span>`fn visit_i64_atomic_store8(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-store16"></span>`fn visit_i64_atomic_store16(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-store32"></span>`fn visit_i64_atomic_store32(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-rmw-add"></span>`fn visit_i32_atomic_rmw_add(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw-add"></span>`fn visit_i64_atomic_rmw_add(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-rmw8-add-u"></span>`fn visit_i32_atomic_rmw8_add_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-rmw16-add-u"></span>`fn visit_i32_atomic_rmw16_add_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw8-add-u"></span>`fn visit_i64_atomic_rmw8_add_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw16-add-u"></span>`fn visit_i64_atomic_rmw16_add_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw32-add-u"></span>`fn visit_i64_atomic_rmw32_add_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-rmw-sub"></span>`fn visit_i32_atomic_rmw_sub(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw-sub"></span>`fn visit_i64_atomic_rmw_sub(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-rmw8-sub-u"></span>`fn visit_i32_atomic_rmw8_sub_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-rmw16-sub-u"></span>`fn visit_i32_atomic_rmw16_sub_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw8-sub-u"></span>`fn visit_i64_atomic_rmw8_sub_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw16-sub-u"></span>`fn visit_i64_atomic_rmw16_sub_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw32-sub-u"></span>`fn visit_i64_atomic_rmw32_sub_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-rmw-and"></span>`fn visit_i32_atomic_rmw_and(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw-and"></span>`fn visit_i64_atomic_rmw_and(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-rmw8-and-u"></span>`fn visit_i32_atomic_rmw8_and_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-rmw16-and-u"></span>`fn visit_i32_atomic_rmw16_and_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw8-and-u"></span>`fn visit_i64_atomic_rmw8_and_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw16-and-u"></span>`fn visit_i64_atomic_rmw16_and_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw32-and-u"></span>`fn visit_i64_atomic_rmw32_and_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-rmw-or"></span>`fn visit_i32_atomic_rmw_or(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw-or"></span>`fn visit_i64_atomic_rmw_or(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-rmw8-or-u"></span>`fn visit_i32_atomic_rmw8_or_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-rmw16-or-u"></span>`fn visit_i32_atomic_rmw16_or_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw8-or-u"></span>`fn visit_i64_atomic_rmw8_or_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw16-or-u"></span>`fn visit_i64_atomic_rmw16_or_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw32-or-u"></span>`fn visit_i64_atomic_rmw32_or_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-rmw-xor"></span>`fn visit_i32_atomic_rmw_xor(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw-xor"></span>`fn visit_i64_atomic_rmw_xor(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-rmw8-xor-u"></span>`fn visit_i32_atomic_rmw8_xor_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-rmw16-xor-u"></span>`fn visit_i32_atomic_rmw16_xor_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw8-xor-u"></span>`fn visit_i64_atomic_rmw8_xor_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw16-xor-u"></span>`fn visit_i64_atomic_rmw16_xor_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw32-xor-u"></span>`fn visit_i64_atomic_rmw32_xor_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-rmw-xchg"></span>`fn visit_i32_atomic_rmw_xchg(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw-xchg"></span>`fn visit_i64_atomic_rmw_xchg(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-rmw8-xchg-u"></span>`fn visit_i32_atomic_rmw8_xchg_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-rmw16-xchg-u"></span>`fn visit_i32_atomic_rmw16_xchg_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw8-xchg-u"></span>`fn visit_i64_atomic_rmw8_xchg_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw16-xchg-u"></span>`fn visit_i64_atomic_rmw16_xchg_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw32-xchg-u"></span>`fn visit_i64_atomic_rmw32_xchg_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-rmw-cmpxchg"></span>`fn visit_i32_atomic_rmw_cmpxchg(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw-cmpxchg"></span>`fn visit_i64_atomic_rmw_cmpxchg(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-rmw8-cmpxchg-u"></span>`fn visit_i32_atomic_rmw8_cmpxchg_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-rmw16-cmpxchg-u"></span>`fn visit_i32_atomic_rmw16_cmpxchg_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw8-cmpxchg-u"></span>`fn visit_i64_atomic_rmw8_cmpxchg_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw16-cmpxchg-u"></span>`fn visit_i64_atomic_rmw16_cmpxchg_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw32-cmpxchg-u"></span>`fn visit_i64_atomic_rmw32_cmpxchg_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-try-table"></span>`fn visit_try_table(&mut self, try_table: TryTable) -> <T as >::Output` — [`TryTable`](#trytable), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-throw"></span>`fn visit_throw(&mut self, tag_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-throw-ref"></span>`fn visit_throw_ref(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-try"></span>`fn visit_try(&mut self, blockty: BlockType) -> <T as >::Output` — [`BlockType`](#blocktype), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-catch"></span>`fn visit_catch(&mut self, tag_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-rethrow"></span>`fn visit_rethrow(&mut self, relative_depth: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-delegate"></span>`fn visit_delegate(&mut self, relative_depth: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-catch-all"></span>`fn visit_catch_all(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-global-atomic-get"></span>`fn visit_global_atomic_get(&mut self, ordering: Ordering, global_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-global-atomic-set"></span>`fn visit_global_atomic_set(&mut self, ordering: Ordering, global_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-global-atomic-rmw-add"></span>`fn visit_global_atomic_rmw_add(&mut self, ordering: Ordering, global_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-global-atomic-rmw-sub"></span>`fn visit_global_atomic_rmw_sub(&mut self, ordering: Ordering, global_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-global-atomic-rmw-and"></span>`fn visit_global_atomic_rmw_and(&mut self, ordering: Ordering, global_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-global-atomic-rmw-or"></span>`fn visit_global_atomic_rmw_or(&mut self, ordering: Ordering, global_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-global-atomic-rmw-xor"></span>`fn visit_global_atomic_rmw_xor(&mut self, ordering: Ordering, global_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-global-atomic-rmw-xchg"></span>`fn visit_global_atomic_rmw_xchg(&mut self, ordering: Ordering, global_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-global-atomic-rmw-cmpxchg"></span>`fn visit_global_atomic_rmw_cmpxchg(&mut self, ordering: Ordering, global_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-table-atomic-get"></span>`fn visit_table_atomic_get(&mut self, ordering: Ordering, table_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-table-atomic-set"></span>`fn visit_table_atomic_set(&mut self, ordering: Ordering, table_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-table-atomic-rmw-xchg"></span>`fn visit_table_atomic_rmw_xchg(&mut self, ordering: Ordering, table_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-table-atomic-rmw-cmpxchg"></span>`fn visit_table_atomic_rmw_cmpxchg(&mut self, ordering: Ordering, table_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-struct-atomic-get"></span>`fn visit_struct_atomic_get(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-struct-atomic-get-s"></span>`fn visit_struct_atomic_get_s(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-struct-atomic-get-u"></span>`fn visit_struct_atomic_get_u(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-struct-atomic-set"></span>`fn visit_struct_atomic_set(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-struct-atomic-rmw-add"></span>`fn visit_struct_atomic_rmw_add(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-struct-atomic-rmw-sub"></span>`fn visit_struct_atomic_rmw_sub(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-struct-atomic-rmw-and"></span>`fn visit_struct_atomic_rmw_and(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-struct-atomic-rmw-or"></span>`fn visit_struct_atomic_rmw_or(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-struct-atomic-rmw-xor"></span>`fn visit_struct_atomic_rmw_xor(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-struct-atomic-rmw-xchg"></span>`fn visit_struct_atomic_rmw_xchg(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-struct-atomic-rmw-cmpxchg"></span>`fn visit_struct_atomic_rmw_cmpxchg(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-array-atomic-get"></span>`fn visit_array_atomic_get(&mut self, ordering: Ordering, array_type_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-array-atomic-get-s"></span>`fn visit_array_atomic_get_s(&mut self, ordering: Ordering, array_type_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-array-atomic-get-u"></span>`fn visit_array_atomic_get_u(&mut self, ordering: Ordering, array_type_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-array-atomic-set"></span>`fn visit_array_atomic_set(&mut self, ordering: Ordering, array_type_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-array-atomic-rmw-add"></span>`fn visit_array_atomic_rmw_add(&mut self, ordering: Ordering, array_type_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-array-atomic-rmw-sub"></span>`fn visit_array_atomic_rmw_sub(&mut self, ordering: Ordering, array_type_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-array-atomic-rmw-and"></span>`fn visit_array_atomic_rmw_and(&mut self, ordering: Ordering, array_type_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-array-atomic-rmw-or"></span>`fn visit_array_atomic_rmw_or(&mut self, ordering: Ordering, array_type_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-array-atomic-rmw-xor"></span>`fn visit_array_atomic_rmw_xor(&mut self, ordering: Ordering, array_type_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-array-atomic-rmw-xchg"></span>`fn visit_array_atomic_rmw_xchg(&mut self, ordering: Ordering, array_type_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-array-atomic-rmw-cmpxchg"></span>`fn visit_array_atomic_rmw_cmpxchg(&mut self, ordering: Ordering, array_type_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-ref-i31-shared"></span>`fn visit_ref_i31_shared(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-call-ref"></span>`fn visit_call_ref(&mut self, type_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-return-call-ref"></span>`fn visit_return_call_ref(&mut self, type_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-ref-as-non-null"></span>`fn visit_ref_as_non_null(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-br-on-null"></span>`fn visit_br_on_null(&mut self, relative_depth: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-br-on-non-null"></span>`fn visit_br_on_non_null(&mut self, relative_depth: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-cont-new"></span>`fn visit_cont_new(&mut self, cont_type_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-cont-bind"></span>`fn visit_cont_bind(&mut self, argument_index: u32, result_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-suspend"></span>`fn visit_suspend(&mut self, tag_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-resume"></span>`fn visit_resume(&mut self, cont_type_index: u32, resume_table: ResumeTable) -> <T as >::Output` — [`ResumeTable`](#resumetable), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-resume-throw"></span>`fn visit_resume_throw(&mut self, cont_type_index: u32, tag_index: u32, resume_table: ResumeTable) -> <T as >::Output` — [`ResumeTable`](#resumetable), [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-switch"></span>`fn visit_switch(&mut self, cont_type_index: u32, tag_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-add128"></span>`fn visit_i64_add128(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-sub128"></span>`fn visit_i64_sub128(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-mul-wide-s"></span>`fn visit_i64_mul_wide_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-mul-wide-u"></span>`fn visit_i64_mul_wide_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

### `SingleFrameAdapter<'a, T>`

```rust
struct SingleFrameAdapter<'a, T> {
    current_frame: FrameKind,
    visitor: &'a mut T,
}
```

#### Trait Implementations

##### `impl<T> FrameStack for SingleFrameAdapter<'_, T>`

- <span id="singleframeadapter-framestack-current-frame"></span>`fn current_frame(&self) -> Option<FrameKind>` — [`FrameKind`](#framekind)

##### `impl<T: VisitOperator<'a>> VisitOperator for SingleFrameAdapter<'_, T>`

- <span id="singleframeadapter-visitoperator-type-output"></span>`type Output = <T as VisitOperator>::Output`

- <span id="singleframeadapter-visitoperator-visit-unreachable"></span>`fn visit_unreachable(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-nop"></span>`fn visit_nop(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-block"></span>`fn visit_block(&mut self, blockty: BlockType) -> <T as >::Output` — [`BlockType`](#blocktype), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-loop"></span>`fn visit_loop(&mut self, blockty: BlockType) -> <T as >::Output` — [`BlockType`](#blocktype), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-if"></span>`fn visit_if(&mut self, blockty: BlockType) -> <T as >::Output` — [`BlockType`](#blocktype), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-else"></span>`fn visit_else(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-end"></span>`fn visit_end(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-br"></span>`fn visit_br(&mut self, relative_depth: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-br-if"></span>`fn visit_br_if(&mut self, relative_depth: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-br-table"></span>`fn visit_br_table(&mut self, targets: BrTable<'a>) -> <T as >::Output` — [`BrTable`](#brtable), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-return"></span>`fn visit_return(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-call"></span>`fn visit_call(&mut self, function_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-call-indirect"></span>`fn visit_call_indirect(&mut self, type_index: u32, table_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-drop"></span>`fn visit_drop(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-select"></span>`fn visit_select(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-local-get"></span>`fn visit_local_get(&mut self, local_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-local-set"></span>`fn visit_local_set(&mut self, local_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-local-tee"></span>`fn visit_local_tee(&mut self, local_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-global-get"></span>`fn visit_global_get(&mut self, global_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-global-set"></span>`fn visit_global_set(&mut self, global_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-load"></span>`fn visit_i32_load(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-load"></span>`fn visit_i64_load(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-load"></span>`fn visit_f32_load(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-load"></span>`fn visit_f64_load(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-load8-s"></span>`fn visit_i32_load8_s(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-load8-u"></span>`fn visit_i32_load8_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-load16-s"></span>`fn visit_i32_load16_s(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-load16-u"></span>`fn visit_i32_load16_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-load8-s"></span>`fn visit_i64_load8_s(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-load8-u"></span>`fn visit_i64_load8_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-load16-s"></span>`fn visit_i64_load16_s(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-load16-u"></span>`fn visit_i64_load16_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-load32-s"></span>`fn visit_i64_load32_s(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-load32-u"></span>`fn visit_i64_load32_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-store"></span>`fn visit_i32_store(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-store"></span>`fn visit_i64_store(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-store"></span>`fn visit_f32_store(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-store"></span>`fn visit_f64_store(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-store8"></span>`fn visit_i32_store8(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-store16"></span>`fn visit_i32_store16(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-store8"></span>`fn visit_i64_store8(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-store16"></span>`fn visit_i64_store16(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-store32"></span>`fn visit_i64_store32(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-memory-size"></span>`fn visit_memory_size(&mut self, mem: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-memory-grow"></span>`fn visit_memory_grow(&mut self, mem: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-const"></span>`fn visit_i32_const(&mut self, value: i32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-const"></span>`fn visit_i64_const(&mut self, value: i64) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-const"></span>`fn visit_f32_const(&mut self, value: Ieee32) -> <T as >::Output` — [`Ieee32`](#ieee32), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-const"></span>`fn visit_f64_const(&mut self, value: Ieee64) -> <T as >::Output` — [`Ieee64`](#ieee64), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-eqz"></span>`fn visit_i32_eqz(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-eq"></span>`fn visit_i32_eq(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-ne"></span>`fn visit_i32_ne(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-lt-s"></span>`fn visit_i32_lt_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-lt-u"></span>`fn visit_i32_lt_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-gt-s"></span>`fn visit_i32_gt_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-gt-u"></span>`fn visit_i32_gt_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-le-s"></span>`fn visit_i32_le_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-le-u"></span>`fn visit_i32_le_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-ge-s"></span>`fn visit_i32_ge_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-ge-u"></span>`fn visit_i32_ge_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-eqz"></span>`fn visit_i64_eqz(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-eq"></span>`fn visit_i64_eq(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-ne"></span>`fn visit_i64_ne(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-lt-s"></span>`fn visit_i64_lt_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-lt-u"></span>`fn visit_i64_lt_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-gt-s"></span>`fn visit_i64_gt_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-gt-u"></span>`fn visit_i64_gt_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-le-s"></span>`fn visit_i64_le_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-le-u"></span>`fn visit_i64_le_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-ge-s"></span>`fn visit_i64_ge_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-ge-u"></span>`fn visit_i64_ge_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-eq"></span>`fn visit_f32_eq(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-ne"></span>`fn visit_f32_ne(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-lt"></span>`fn visit_f32_lt(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-gt"></span>`fn visit_f32_gt(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-le"></span>`fn visit_f32_le(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-ge"></span>`fn visit_f32_ge(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-eq"></span>`fn visit_f64_eq(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-ne"></span>`fn visit_f64_ne(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-lt"></span>`fn visit_f64_lt(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-gt"></span>`fn visit_f64_gt(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-le"></span>`fn visit_f64_le(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-ge"></span>`fn visit_f64_ge(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-clz"></span>`fn visit_i32_clz(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-ctz"></span>`fn visit_i32_ctz(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-popcnt"></span>`fn visit_i32_popcnt(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-add"></span>`fn visit_i32_add(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-sub"></span>`fn visit_i32_sub(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-mul"></span>`fn visit_i32_mul(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-div-s"></span>`fn visit_i32_div_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-div-u"></span>`fn visit_i32_div_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-rem-s"></span>`fn visit_i32_rem_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-rem-u"></span>`fn visit_i32_rem_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-and"></span>`fn visit_i32_and(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-or"></span>`fn visit_i32_or(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-xor"></span>`fn visit_i32_xor(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-shl"></span>`fn visit_i32_shl(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-shr-s"></span>`fn visit_i32_shr_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-shr-u"></span>`fn visit_i32_shr_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-rotl"></span>`fn visit_i32_rotl(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-rotr"></span>`fn visit_i32_rotr(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-clz"></span>`fn visit_i64_clz(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-ctz"></span>`fn visit_i64_ctz(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-popcnt"></span>`fn visit_i64_popcnt(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-add"></span>`fn visit_i64_add(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-sub"></span>`fn visit_i64_sub(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-mul"></span>`fn visit_i64_mul(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-div-s"></span>`fn visit_i64_div_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-div-u"></span>`fn visit_i64_div_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-rem-s"></span>`fn visit_i64_rem_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-rem-u"></span>`fn visit_i64_rem_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-and"></span>`fn visit_i64_and(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-or"></span>`fn visit_i64_or(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-xor"></span>`fn visit_i64_xor(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-shl"></span>`fn visit_i64_shl(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-shr-s"></span>`fn visit_i64_shr_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-shr-u"></span>`fn visit_i64_shr_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-rotl"></span>`fn visit_i64_rotl(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-rotr"></span>`fn visit_i64_rotr(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-abs"></span>`fn visit_f32_abs(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-neg"></span>`fn visit_f32_neg(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-ceil"></span>`fn visit_f32_ceil(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-floor"></span>`fn visit_f32_floor(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-trunc"></span>`fn visit_f32_trunc(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-nearest"></span>`fn visit_f32_nearest(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-sqrt"></span>`fn visit_f32_sqrt(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-add"></span>`fn visit_f32_add(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-sub"></span>`fn visit_f32_sub(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-mul"></span>`fn visit_f32_mul(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-div"></span>`fn visit_f32_div(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-min"></span>`fn visit_f32_min(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-max"></span>`fn visit_f32_max(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-copysign"></span>`fn visit_f32_copysign(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-abs"></span>`fn visit_f64_abs(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-neg"></span>`fn visit_f64_neg(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-ceil"></span>`fn visit_f64_ceil(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-floor"></span>`fn visit_f64_floor(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-trunc"></span>`fn visit_f64_trunc(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-nearest"></span>`fn visit_f64_nearest(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-sqrt"></span>`fn visit_f64_sqrt(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-add"></span>`fn visit_f64_add(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-sub"></span>`fn visit_f64_sub(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-mul"></span>`fn visit_f64_mul(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-div"></span>`fn visit_f64_div(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-min"></span>`fn visit_f64_min(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-max"></span>`fn visit_f64_max(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-copysign"></span>`fn visit_f64_copysign(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-wrap-i64"></span>`fn visit_i32_wrap_i64(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-trunc-f32-s"></span>`fn visit_i32_trunc_f32_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-trunc-f32-u"></span>`fn visit_i32_trunc_f32_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-trunc-f64-s"></span>`fn visit_i32_trunc_f64_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-trunc-f64-u"></span>`fn visit_i32_trunc_f64_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-extend-i32-s"></span>`fn visit_i64_extend_i32_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-extend-i32-u"></span>`fn visit_i64_extend_i32_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-trunc-f32-s"></span>`fn visit_i64_trunc_f32_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-trunc-f32-u"></span>`fn visit_i64_trunc_f32_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-trunc-f64-s"></span>`fn visit_i64_trunc_f64_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-trunc-f64-u"></span>`fn visit_i64_trunc_f64_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-convert-i32-s"></span>`fn visit_f32_convert_i32_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-convert-i32-u"></span>`fn visit_f32_convert_i32_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-convert-i64-s"></span>`fn visit_f32_convert_i64_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-convert-i64-u"></span>`fn visit_f32_convert_i64_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-demote-f64"></span>`fn visit_f32_demote_f64(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-convert-i32-s"></span>`fn visit_f64_convert_i32_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-convert-i32-u"></span>`fn visit_f64_convert_i32_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-convert-i64-s"></span>`fn visit_f64_convert_i64_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-convert-i64-u"></span>`fn visit_f64_convert_i64_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-promote-f32"></span>`fn visit_f64_promote_f32(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-reinterpret-f32"></span>`fn visit_i32_reinterpret_f32(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-reinterpret-f64"></span>`fn visit_i64_reinterpret_f64(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-reinterpret-i32"></span>`fn visit_f32_reinterpret_i32(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-reinterpret-i64"></span>`fn visit_f64_reinterpret_i64(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-extend8-s"></span>`fn visit_i32_extend8_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-extend16-s"></span>`fn visit_i32_extend16_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-extend8-s"></span>`fn visit_i64_extend8_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-extend16-s"></span>`fn visit_i64_extend16_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-extend32-s"></span>`fn visit_i64_extend32_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-ref-eq"></span>`fn visit_ref_eq(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-struct-new"></span>`fn visit_struct_new(&mut self, struct_type_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-struct-new-default"></span>`fn visit_struct_new_default(&mut self, struct_type_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-struct-get"></span>`fn visit_struct_get(&mut self, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-struct-get-s"></span>`fn visit_struct_get_s(&mut self, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-struct-get-u"></span>`fn visit_struct_get_u(&mut self, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-struct-set"></span>`fn visit_struct_set(&mut self, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-array-new"></span>`fn visit_array_new(&mut self, array_type_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-array-new-default"></span>`fn visit_array_new_default(&mut self, array_type_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-array-new-fixed"></span>`fn visit_array_new_fixed(&mut self, array_type_index: u32, array_size: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-array-new-data"></span>`fn visit_array_new_data(&mut self, array_type_index: u32, array_data_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-array-new-elem"></span>`fn visit_array_new_elem(&mut self, array_type_index: u32, array_elem_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-array-get"></span>`fn visit_array_get(&mut self, array_type_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-array-get-s"></span>`fn visit_array_get_s(&mut self, array_type_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-array-get-u"></span>`fn visit_array_get_u(&mut self, array_type_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-array-set"></span>`fn visit_array_set(&mut self, array_type_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-array-len"></span>`fn visit_array_len(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-array-fill"></span>`fn visit_array_fill(&mut self, array_type_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-array-copy"></span>`fn visit_array_copy(&mut self, array_type_index_dst: u32, array_type_index_src: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-array-init-data"></span>`fn visit_array_init_data(&mut self, array_type_index: u32, array_data_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-array-init-elem"></span>`fn visit_array_init_elem(&mut self, array_type_index: u32, array_elem_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-ref-test-non-null"></span>`fn visit_ref_test_non_null(&mut self, hty: HeapType) -> <T as >::Output` — [`HeapType`](#heaptype), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-ref-test-nullable"></span>`fn visit_ref_test_nullable(&mut self, hty: HeapType) -> <T as >::Output` — [`HeapType`](#heaptype), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-ref-cast-non-null"></span>`fn visit_ref_cast_non_null(&mut self, hty: HeapType) -> <T as >::Output` — [`HeapType`](#heaptype), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-ref-cast-nullable"></span>`fn visit_ref_cast_nullable(&mut self, hty: HeapType) -> <T as >::Output` — [`HeapType`](#heaptype), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-br-on-cast"></span>`fn visit_br_on_cast(&mut self, relative_depth: u32, from_ref_type: RefType, to_ref_type: RefType) -> <T as >::Output` — [`RefType`](#reftype), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-br-on-cast-fail"></span>`fn visit_br_on_cast_fail(&mut self, relative_depth: u32, from_ref_type: RefType, to_ref_type: RefType) -> <T as >::Output` — [`RefType`](#reftype), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-any-convert-extern"></span>`fn visit_any_convert_extern(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-extern-convert-any"></span>`fn visit_extern_convert_any(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-ref-i31"></span>`fn visit_ref_i31(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i31-get-s"></span>`fn visit_i31_get_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i31-get-u"></span>`fn visit_i31_get_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-struct-new-desc"></span>`fn visit_struct_new_desc(&mut self, struct_type_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-struct-new-default-desc"></span>`fn visit_struct_new_default_desc(&mut self, struct_type_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-ref-get-desc"></span>`fn visit_ref_get_desc(&mut self, type_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-ref-cast-desc-non-null"></span>`fn visit_ref_cast_desc_non_null(&mut self, hty: HeapType) -> <T as >::Output` — [`HeapType`](#heaptype), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-ref-cast-desc-nullable"></span>`fn visit_ref_cast_desc_nullable(&mut self, hty: HeapType) -> <T as >::Output` — [`HeapType`](#heaptype), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-br-on-cast-desc"></span>`fn visit_br_on_cast_desc(&mut self, relative_depth: u32, from_ref_type: RefType, to_ref_type: RefType) -> <T as >::Output` — [`RefType`](#reftype), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-br-on-cast-desc-fail"></span>`fn visit_br_on_cast_desc_fail(&mut self, relative_depth: u32, from_ref_type: RefType, to_ref_type: RefType) -> <T as >::Output` — [`RefType`](#reftype), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-trunc-sat-f32-s"></span>`fn visit_i32_trunc_sat_f32_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-trunc-sat-f32-u"></span>`fn visit_i32_trunc_sat_f32_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-trunc-sat-f64-s"></span>`fn visit_i32_trunc_sat_f64_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-trunc-sat-f64-u"></span>`fn visit_i32_trunc_sat_f64_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-trunc-sat-f32-s"></span>`fn visit_i64_trunc_sat_f32_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-trunc-sat-f32-u"></span>`fn visit_i64_trunc_sat_f32_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-trunc-sat-f64-s"></span>`fn visit_i64_trunc_sat_f64_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-trunc-sat-f64-u"></span>`fn visit_i64_trunc_sat_f64_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-memory-init"></span>`fn visit_memory_init(&mut self, data_index: u32, mem: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-data-drop"></span>`fn visit_data_drop(&mut self, data_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-memory-copy"></span>`fn visit_memory_copy(&mut self, dst_mem: u32, src_mem: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-memory-fill"></span>`fn visit_memory_fill(&mut self, mem: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-table-init"></span>`fn visit_table_init(&mut self, elem_index: u32, table: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-elem-drop"></span>`fn visit_elem_drop(&mut self, elem_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-table-copy"></span>`fn visit_table_copy(&mut self, dst_table: u32, src_table: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-typed-select"></span>`fn visit_typed_select(&mut self, ty: ValType) -> <T as >::Output` — [`ValType`](#valtype), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-typed-select-multi"></span>`fn visit_typed_select_multi(&mut self, tys: Vec<ValType>) -> <T as >::Output` — [`Vec`](../../prelude/index.md#vec), [`ValType`](#valtype), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-ref-null"></span>`fn visit_ref_null(&mut self, hty: HeapType) -> <T as >::Output` — [`HeapType`](#heaptype), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-ref-is-null"></span>`fn visit_ref_is_null(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-ref-func"></span>`fn visit_ref_func(&mut self, function_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-table-fill"></span>`fn visit_table_fill(&mut self, table: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-table-get"></span>`fn visit_table_get(&mut self, table: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-table-set"></span>`fn visit_table_set(&mut self, table: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-table-grow"></span>`fn visit_table_grow(&mut self, table: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-table-size"></span>`fn visit_table_size(&mut self, table: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-return-call"></span>`fn visit_return_call(&mut self, function_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-return-call-indirect"></span>`fn visit_return_call_indirect(&mut self, type_index: u32, table_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-memory-discard"></span>`fn visit_memory_discard(&mut self, mem: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-memory-atomic-notify"></span>`fn visit_memory_atomic_notify(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-memory-atomic-wait32"></span>`fn visit_memory_atomic_wait32(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-memory-atomic-wait64"></span>`fn visit_memory_atomic_wait64(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-atomic-fence"></span>`fn visit_atomic_fence(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-load"></span>`fn visit_i32_atomic_load(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-load"></span>`fn visit_i64_atomic_load(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-load8-u"></span>`fn visit_i32_atomic_load8_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-load16-u"></span>`fn visit_i32_atomic_load16_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-load8-u"></span>`fn visit_i64_atomic_load8_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-load16-u"></span>`fn visit_i64_atomic_load16_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-load32-u"></span>`fn visit_i64_atomic_load32_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-store"></span>`fn visit_i32_atomic_store(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-store"></span>`fn visit_i64_atomic_store(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-store8"></span>`fn visit_i32_atomic_store8(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-store16"></span>`fn visit_i32_atomic_store16(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-store8"></span>`fn visit_i64_atomic_store8(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-store16"></span>`fn visit_i64_atomic_store16(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-store32"></span>`fn visit_i64_atomic_store32(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-rmw-add"></span>`fn visit_i32_atomic_rmw_add(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw-add"></span>`fn visit_i64_atomic_rmw_add(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-rmw8-add-u"></span>`fn visit_i32_atomic_rmw8_add_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-rmw16-add-u"></span>`fn visit_i32_atomic_rmw16_add_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw8-add-u"></span>`fn visit_i64_atomic_rmw8_add_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw16-add-u"></span>`fn visit_i64_atomic_rmw16_add_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw32-add-u"></span>`fn visit_i64_atomic_rmw32_add_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-rmw-sub"></span>`fn visit_i32_atomic_rmw_sub(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw-sub"></span>`fn visit_i64_atomic_rmw_sub(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-rmw8-sub-u"></span>`fn visit_i32_atomic_rmw8_sub_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-rmw16-sub-u"></span>`fn visit_i32_atomic_rmw16_sub_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw8-sub-u"></span>`fn visit_i64_atomic_rmw8_sub_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw16-sub-u"></span>`fn visit_i64_atomic_rmw16_sub_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw32-sub-u"></span>`fn visit_i64_atomic_rmw32_sub_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-rmw-and"></span>`fn visit_i32_atomic_rmw_and(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw-and"></span>`fn visit_i64_atomic_rmw_and(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-rmw8-and-u"></span>`fn visit_i32_atomic_rmw8_and_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-rmw16-and-u"></span>`fn visit_i32_atomic_rmw16_and_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw8-and-u"></span>`fn visit_i64_atomic_rmw8_and_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw16-and-u"></span>`fn visit_i64_atomic_rmw16_and_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw32-and-u"></span>`fn visit_i64_atomic_rmw32_and_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-rmw-or"></span>`fn visit_i32_atomic_rmw_or(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw-or"></span>`fn visit_i64_atomic_rmw_or(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-rmw8-or-u"></span>`fn visit_i32_atomic_rmw8_or_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-rmw16-or-u"></span>`fn visit_i32_atomic_rmw16_or_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw8-or-u"></span>`fn visit_i64_atomic_rmw8_or_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw16-or-u"></span>`fn visit_i64_atomic_rmw16_or_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw32-or-u"></span>`fn visit_i64_atomic_rmw32_or_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-rmw-xor"></span>`fn visit_i32_atomic_rmw_xor(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw-xor"></span>`fn visit_i64_atomic_rmw_xor(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-rmw8-xor-u"></span>`fn visit_i32_atomic_rmw8_xor_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-rmw16-xor-u"></span>`fn visit_i32_atomic_rmw16_xor_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw8-xor-u"></span>`fn visit_i64_atomic_rmw8_xor_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw16-xor-u"></span>`fn visit_i64_atomic_rmw16_xor_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw32-xor-u"></span>`fn visit_i64_atomic_rmw32_xor_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-rmw-xchg"></span>`fn visit_i32_atomic_rmw_xchg(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw-xchg"></span>`fn visit_i64_atomic_rmw_xchg(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-rmw8-xchg-u"></span>`fn visit_i32_atomic_rmw8_xchg_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-rmw16-xchg-u"></span>`fn visit_i32_atomic_rmw16_xchg_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw8-xchg-u"></span>`fn visit_i64_atomic_rmw8_xchg_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw16-xchg-u"></span>`fn visit_i64_atomic_rmw16_xchg_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw32-xchg-u"></span>`fn visit_i64_atomic_rmw32_xchg_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-rmw-cmpxchg"></span>`fn visit_i32_atomic_rmw_cmpxchg(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw-cmpxchg"></span>`fn visit_i64_atomic_rmw_cmpxchg(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-rmw8-cmpxchg-u"></span>`fn visit_i32_atomic_rmw8_cmpxchg_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-rmw16-cmpxchg-u"></span>`fn visit_i32_atomic_rmw16_cmpxchg_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw8-cmpxchg-u"></span>`fn visit_i64_atomic_rmw8_cmpxchg_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw16-cmpxchg-u"></span>`fn visit_i64_atomic_rmw16_cmpxchg_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw32-cmpxchg-u"></span>`fn visit_i64_atomic_rmw32_cmpxchg_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](#memarg), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-try-table"></span>`fn visit_try_table(&mut self, try_table: TryTable) -> <T as >::Output` — [`TryTable`](#trytable), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-throw"></span>`fn visit_throw(&mut self, tag_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-throw-ref"></span>`fn visit_throw_ref(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-try"></span>`fn visit_try(&mut self, blockty: BlockType) -> <T as >::Output` — [`BlockType`](#blocktype), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-catch"></span>`fn visit_catch(&mut self, tag_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-rethrow"></span>`fn visit_rethrow(&mut self, relative_depth: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-delegate"></span>`fn visit_delegate(&mut self, relative_depth: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-catch-all"></span>`fn visit_catch_all(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-global-atomic-get"></span>`fn visit_global_atomic_get(&mut self, ordering: Ordering, global_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-global-atomic-set"></span>`fn visit_global_atomic_set(&mut self, ordering: Ordering, global_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-global-atomic-rmw-add"></span>`fn visit_global_atomic_rmw_add(&mut self, ordering: Ordering, global_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-global-atomic-rmw-sub"></span>`fn visit_global_atomic_rmw_sub(&mut self, ordering: Ordering, global_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-global-atomic-rmw-and"></span>`fn visit_global_atomic_rmw_and(&mut self, ordering: Ordering, global_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-global-atomic-rmw-or"></span>`fn visit_global_atomic_rmw_or(&mut self, ordering: Ordering, global_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-global-atomic-rmw-xor"></span>`fn visit_global_atomic_rmw_xor(&mut self, ordering: Ordering, global_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-global-atomic-rmw-xchg"></span>`fn visit_global_atomic_rmw_xchg(&mut self, ordering: Ordering, global_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-global-atomic-rmw-cmpxchg"></span>`fn visit_global_atomic_rmw_cmpxchg(&mut self, ordering: Ordering, global_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-table-atomic-get"></span>`fn visit_table_atomic_get(&mut self, ordering: Ordering, table_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-table-atomic-set"></span>`fn visit_table_atomic_set(&mut self, ordering: Ordering, table_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-table-atomic-rmw-xchg"></span>`fn visit_table_atomic_rmw_xchg(&mut self, ordering: Ordering, table_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-table-atomic-rmw-cmpxchg"></span>`fn visit_table_atomic_rmw_cmpxchg(&mut self, ordering: Ordering, table_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-struct-atomic-get"></span>`fn visit_struct_atomic_get(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-struct-atomic-get-s"></span>`fn visit_struct_atomic_get_s(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-struct-atomic-get-u"></span>`fn visit_struct_atomic_get_u(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-struct-atomic-set"></span>`fn visit_struct_atomic_set(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-struct-atomic-rmw-add"></span>`fn visit_struct_atomic_rmw_add(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-struct-atomic-rmw-sub"></span>`fn visit_struct_atomic_rmw_sub(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-struct-atomic-rmw-and"></span>`fn visit_struct_atomic_rmw_and(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-struct-atomic-rmw-or"></span>`fn visit_struct_atomic_rmw_or(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-struct-atomic-rmw-xor"></span>`fn visit_struct_atomic_rmw_xor(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-struct-atomic-rmw-xchg"></span>`fn visit_struct_atomic_rmw_xchg(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-struct-atomic-rmw-cmpxchg"></span>`fn visit_struct_atomic_rmw_cmpxchg(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-array-atomic-get"></span>`fn visit_array_atomic_get(&mut self, ordering: Ordering, array_type_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-array-atomic-get-s"></span>`fn visit_array_atomic_get_s(&mut self, ordering: Ordering, array_type_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-array-atomic-get-u"></span>`fn visit_array_atomic_get_u(&mut self, ordering: Ordering, array_type_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-array-atomic-set"></span>`fn visit_array_atomic_set(&mut self, ordering: Ordering, array_type_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-array-atomic-rmw-add"></span>`fn visit_array_atomic_rmw_add(&mut self, ordering: Ordering, array_type_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-array-atomic-rmw-sub"></span>`fn visit_array_atomic_rmw_sub(&mut self, ordering: Ordering, array_type_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-array-atomic-rmw-and"></span>`fn visit_array_atomic_rmw_and(&mut self, ordering: Ordering, array_type_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-array-atomic-rmw-or"></span>`fn visit_array_atomic_rmw_or(&mut self, ordering: Ordering, array_type_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-array-atomic-rmw-xor"></span>`fn visit_array_atomic_rmw_xor(&mut self, ordering: Ordering, array_type_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-array-atomic-rmw-xchg"></span>`fn visit_array_atomic_rmw_xchg(&mut self, ordering: Ordering, array_type_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-array-atomic-rmw-cmpxchg"></span>`fn visit_array_atomic_rmw_cmpxchg(&mut self, ordering: Ordering, array_type_index: u32) -> <T as >::Output` — [`Ordering`](#ordering), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-ref-i31-shared"></span>`fn visit_ref_i31_shared(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-call-ref"></span>`fn visit_call_ref(&mut self, type_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-return-call-ref"></span>`fn visit_return_call_ref(&mut self, type_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-ref-as-non-null"></span>`fn visit_ref_as_non_null(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-br-on-null"></span>`fn visit_br_on_null(&mut self, relative_depth: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-br-on-non-null"></span>`fn visit_br_on_non_null(&mut self, relative_depth: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-cont-new"></span>`fn visit_cont_new(&mut self, cont_type_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-cont-bind"></span>`fn visit_cont_bind(&mut self, argument_index: u32, result_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-suspend"></span>`fn visit_suspend(&mut self, tag_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-resume"></span>`fn visit_resume(&mut self, cont_type_index: u32, resume_table: ResumeTable) -> <T as >::Output` — [`ResumeTable`](#resumetable), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-resume-throw"></span>`fn visit_resume_throw(&mut self, cont_type_index: u32, tag_index: u32, resume_table: ResumeTable) -> <T as >::Output` — [`ResumeTable`](#resumetable), [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-switch"></span>`fn visit_switch(&mut self, cont_type_index: u32, tag_index: u32) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-add128"></span>`fn visit_i64_add128(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-sub128"></span>`fn visit_i64_sub128(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-mul-wide-s"></span>`fn visit_i64_mul_wide_s(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-mul-wide-u"></span>`fn visit_i64_mul_wide_u(&mut self) -> <T as >::Output` — [`VisitOperator`](#visitoperator)

### `OperatorsReader<'a>`

```rust
struct OperatorsReader<'a> {
    reader: crate::BinaryReader<'a>,
    stack: ControlStack,
}
```

A reader for a core WebAssembly function's operators. The [`OperatorsReader`](#operatorsreader) internally
maintains a stack of the kinds of frames within an expression or function body.
This is necessary to enforce the syntactic requirements of the binary format.
The BinaryReader can also be used to read the operators by providing an external [`FrameStack`](#framestack) instance.

#### Implementations

- <span id="operatorsreader-new"></span>`fn new(reader: BinaryReader<'a>) -> Self` — [`BinaryReader`](../../binary_reader/index.md#binaryreader)

  Creates a new reader for an expression (instruction sequence).

  

  This method, in conjunction with `OperatorsReader::into_allocations`,

  provides a means to reuse allocations across reading each

  individual expression or function body. Note that it is also sufficient

  to call this method with `Default::default()` if no prior allocations are

  available.

- <span id="operatorsreader-new-with-allocs"></span>`fn new_with_allocs(reader: BinaryReader<'a>, allocs: OperatorsReaderAllocations) -> Self` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`OperatorsReaderAllocations`](#operatorsreaderallocations)

  Same as `OperatorsReader::new` except that the

  [`OperatorsReaderAllocations`](#operatorsreaderallocations) can be specified here to amortize the

  cost of them over multiple readers.

- <span id="operatorsreader-get-binary-reader"></span>`fn get_binary_reader(&self) -> BinaryReader<'a>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader)

  Get binary reader

- <span id="operatorsreader-eof"></span>`fn eof(&self) -> bool`

  Determines if the reader is at the end of the operators.

- <span id="operatorsreader-original-position"></span>`fn original_position(&self) -> usize`

  Gets the original position of the reader.

- <span id="operatorsreader-is-end-then-eof"></span>`fn is_end_then_eof(&self) -> bool`

  Returns whether there is an `end` opcode followed by eof remaining in

  this reader.

- <span id="operatorsreader-into-allocations"></span>`fn into_allocations(self) -> OperatorsReaderAllocations` — [`OperatorsReaderAllocations`](#operatorsreaderallocations)

  Consumes this reader and returns the underlying allocations that

  were used to store the frame stack.

  

  The returned value here can be paired with

  `OperatorsReader::new` to reuse the allocations already

  created by this reader.

- <span id="operatorsreader-read"></span>`fn read(&mut self) -> Result<Operator<'a>>` — [`Result`](../../binary_reader/index.md#result), [`Operator`](#operator)

  Reads the next available `Operator`.

  

  # Errors

  

  If `OperatorsReader` has less bytes remaining than required to parse

  the `Operator`, or if the input is malformed.

- <span id="operatorsreader-visit-operator"></span>`fn visit_operator<T>(&mut self, visitor: &mut T) -> Result<<T as VisitOperator>::Output>` — [`Result`](../../binary_reader/index.md#result), [`VisitOperator`](#visitoperator)

  Visit the next available operator with the specified [`VisitOperator`](#visitoperator) instance.

  

  Note that this does not implicitly propagate any additional information such as instruction

  offsets. In order to do so, consider storing such data within the visitor before visiting.

  

  # Errors

  

  If `OperatorsReader` has less bytes remaining than required to parse the `Operator`,

  or if the input is malformed.

  

  # Examples

  

  Store an offset for use in diagnostics or any other purposes:

  

  ```rust

  use wasmparser::{OperatorsReader, VisitOperator, Result, for_each_visit_operator};

  

  pub fn dump(mut reader: OperatorsReader) -> Result<()> {

      let mut visitor = Dumper { offset: 0 };

      while !reader.eof() {

          visitor.offset = reader.original_position();

          reader.visit_operator(&mut visitor)?;

      }

      Ok(())

  }

  

  struct Dumper {

      offset: usize

  }

  

  macro_rules! define_visit_operator {

      ($(@$proposal:ident $op:ident $({ $($arg:ident: $argty:ty),* })? => $visit:ident ($($ann:tt)*))*) => {

          $(

              fn $visit(&mut self $($(,$arg: $argty)*)?) -> Self::Output {

                  println!("{}: {}", self.offset, stringify!($visit));

              }

          )*

      }

  }

  

  impl<'a> VisitOperator<'a> for Dumper {

      type Output = ();

      for_each_visit_operator!(define_visit_operator);

  }

  

  ```

- <span id="operatorsreader-read-with-offset"></span>`fn read_with_offset(&mut self) -> Result<(Operator<'a>, usize)>` — [`Result`](../../binary_reader/index.md#result), [`Operator`](#operator)

  Reads an operator with its offset.

- <span id="operatorsreader-into-iter-with-offsets"></span>`fn into_iter_with_offsets(self) -> OperatorsIteratorWithOffsets<'a>` — [`OperatorsIteratorWithOffsets`](#operatorsiteratorwithoffsets)

  Converts to an iterator of operators paired with offsets.

- <span id="operatorsreader-skip-const-expr"></span>`fn skip_const_expr(&mut self) -> Result<()>` — [`Result`](../../binary_reader/index.md#result)

- <span id="operatorsreader-finish"></span>`fn finish(&self) -> Result<()>` — [`Result`](../../binary_reader/index.md#result)

  Function that must be called after the last opcode has been processed.

  

  This function returns an error if there is extra data after the operators.

  It does *not* check the binary format requirement that if the data count

  section is absent, a data index may not occur in the code section.

#### Trait Implementations

##### `impl Clone for OperatorsReader<'a>`

- <span id="operatorsreader-clone"></span>`fn clone(&self) -> OperatorsReader<'a>` — [`OperatorsReader`](#operatorsreader)

##### `impl FrameStack for OperatorsReader<'a>`

- <span id="operatorsreader-framestack-current-frame"></span>`fn current_frame(&self) -> Option<FrameKind>` — [`FrameKind`](#framekind)

##### `impl IntoIterator for OperatorsReader<'a>`

- <span id="operatorsreader-intoiterator-type-item"></span>`type Item = Result<Operator<'a>, BinaryReaderError>`

- <span id="operatorsreader-intoiterator-type-intoiter"></span>`type IntoIter = OperatorsIterator<'a>`

- <span id="operatorsreader-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

  Reads content of the code section.

  

  # Examples

  ```rust

  use wasmparser::{Operator, CodeSectionReader, Result, BinaryReader};

  let data: &[u8] = &[

      0x01, 0x03, 0x00, 0x01, 0x0b];

  let reader = BinaryReader::new(data, 0);

  let code_reader = CodeSectionReader::new(reader).unwrap();

  for body in code_reader {

      let body = body.expect("function body");

      let mut op_reader = body.get_operators_reader().expect("op reader");

      let ops = op_reader.into_iter().collect::<Result<Vec<Operator>>>().expect("ops");

      assert!(

          if let [Operator::Nop, Operator::End] = ops.as_slice() { true } else { false },

          "found {:?}",

          ops

      );

  }

  ```

### `OperatorsReaderAllocations`

```rust
struct OperatorsReaderAllocations(ControlStack);
```

External handle to the internal allocations used by the OperatorsReader

This is created with either the `Default` implementation or with
`OperatorsReader::into_allocations`. It is then passed as an argument to
`OperatorsReader::new` to provide a means of reusing allocations
between each expression or function body.

#### Trait Implementations

##### `impl Default for OperatorsReaderAllocations`

- <span id="operatorsreaderallocations-default"></span>`fn default() -> OperatorsReaderAllocations` — [`OperatorsReaderAllocations`](#operatorsreaderallocations)

### `OperatorsIterator<'a>`

```rust
struct OperatorsIterator<'a> {
    reader: OperatorsReader<'a>,
    err: bool,
}
```

An iterator over a function's operators.

#### Implementations

- <span id="operatorsiterator-into-allocations"></span>`fn into_allocations(self) -> OperatorsReaderAllocations` — [`OperatorsReaderAllocations`](#operatorsreaderallocations)

  Consumes this iterator and returns the underlying allocations.

  See `OperatorsReader::into_allocations`.

#### Trait Implementations

##### `impl IntoIterator for OperatorsIterator<'a>`

- <span id="operatorsiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="operatorsiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="operatorsiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for OperatorsIterator<'a>`

- <span id="operatorsiterator-iterator-type-item"></span>`type Item = Result<Operator<'a>, BinaryReaderError>`

- <span id="operatorsiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `OperatorsIteratorWithOffsets<'a>`

```rust
struct OperatorsIteratorWithOffsets<'a> {
    reader: OperatorsReader<'a>,
    err: bool,
}
```

An iterator over a function's operators with offsets.

#### Implementations

- <span id="operatorsiteratorwithoffsets-into-allocations"></span>`fn into_allocations(self) -> OperatorsReaderAllocations` — [`OperatorsReaderAllocations`](#operatorsreaderallocations)

  Consumes this iterator and returns the underlying allocations.

  See `OperatorsReader::into_allocations`.

#### Trait Implementations

##### `impl IntoIterator for OperatorsIteratorWithOffsets<'a>`

- <span id="operatorsiteratorwithoffsets-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="operatorsiteratorwithoffsets-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="operatorsiteratorwithoffsets-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for OperatorsIteratorWithOffsets<'a>`

- <span id="operatorsiteratorwithoffsets-iterator-type-item"></span>`type Item = Result<(Operator<'a>, usize), BinaryReaderError>`

- <span id="operatorsiteratorwithoffsets-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

  Reads content of the code section with offsets.

  

  # Examples

  ```rust

  use wasmparser::{Operator, CodeSectionReader, Result, BinaryReader};

  let data: &[u8] = &[

      0x01, 0x03, 0x00, /* offset = 23 */ 0x01, 0x0b];

  let reader = BinaryReader::new(data, 20);

  let code_reader = CodeSectionReader::new(reader).unwrap();

  for body in code_reader {

      let body = body.expect("function body");

      let mut op_reader = body.get_operators_reader().expect("op reader");

      let ops = op_reader.into_iter_with_offsets().collect::<Result<Vec<(Operator, usize)>>>().expect("ops");

      assert!(

          if let [(Operator::Nop, 23), (Operator::End, 24)] = ops.as_slice() { true } else { false },

          "found {:?}",

          ops

      );

  }

  ```

### `TryTable`

```rust
struct TryTable {
    pub ty: BlockType,
    pub catches: Vec<Catch>,
}
```

A `try_table` entries representation.

#### Fields

- **`ty`**: `BlockType`

  The block type describing the try block itself.

- **`catches`**: `Vec<Catch>`

  Outer blocks which will receive exceptions.

#### Trait Implementations

##### `impl Clone for TryTable`

- <span id="trytable-clone"></span>`fn clone(&self) -> TryTable` — [`TryTable`](#trytable)

##### `impl Debug for TryTable`

- <span id="trytable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for TryTable`

##### `impl FromReader for TryTable`

- <span id="trytable-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

##### `impl PartialEq for TryTable`

- <span id="trytable-partialeq-eq"></span>`fn eq(&self, other: &TryTable) -> bool` — [`TryTable`](#trytable)

##### `impl StructuralPartialEq for TryTable`

### `ResumeTable`

```rust
struct ResumeTable {
    pub handlers: Vec<Handle>,
}
```

A representation of dispatch tables on `resume` and `resume_throw`
instructions.

#### Fields

- **`handlers`**: `Vec<Handle>`

  Either the outer blocks which will handle suspensions or
  "switch-to" handlers.

#### Implementations

- <span id="resumetable-len"></span>`fn len(&self) -> usize`

  Returns the number of entries in the table.

#### Trait Implementations

##### `impl Clone for ResumeTable`

- <span id="resumetable-clone"></span>`fn clone(&self) -> ResumeTable` — [`ResumeTable`](#resumetable)

##### `impl Debug for ResumeTable`

- <span id="resumetable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ResumeTable`

##### `impl FromReader for ResumeTable`

- <span id="resumetable-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

##### `impl PartialEq for ResumeTable`

- <span id="resumetable-partialeq-eq"></span>`fn eq(&self, other: &ResumeTable) -> bool` — [`ResumeTable`](#resumetable)

##### `impl StructuralPartialEq for ResumeTable`

### `OperatorFactory`

```rust
struct OperatorFactory;
```

A factory to construct [`Operator`](#operator) instances via the [`VisitOperator`](#visitoperator) trait.

#### Trait Implementations

##### `impl VisitOperator for OperatorFactory`

- <span id="operatorfactory-visitoperator-type-output"></span>`type Output = Operator<'a>`

- <span id="operatorfactory-visitoperator-visit-unreachable"></span>`fn visit_unreachable(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-nop"></span>`fn visit_nop(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-block"></span>`fn visit_block(&mut self, blockty: BlockType) -> Operator<'a>` — [`BlockType`](#blocktype), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-loop"></span>`fn visit_loop(&mut self, blockty: BlockType) -> Operator<'a>` — [`BlockType`](#blocktype), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-if"></span>`fn visit_if(&mut self, blockty: BlockType) -> Operator<'a>` — [`BlockType`](#blocktype), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-else"></span>`fn visit_else(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-end"></span>`fn visit_end(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-br"></span>`fn visit_br(&mut self, relative_depth: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-br-if"></span>`fn visit_br_if(&mut self, relative_depth: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-br-table"></span>`fn visit_br_table(&mut self, targets: BrTable<'a>) -> Operator<'a>` — [`BrTable`](#brtable), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-return"></span>`fn visit_return(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-call"></span>`fn visit_call(&mut self, function_index: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-call-indirect"></span>`fn visit_call_indirect(&mut self, type_index: u32, table_index: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-drop"></span>`fn visit_drop(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-select"></span>`fn visit_select(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-local-get"></span>`fn visit_local_get(&mut self, local_index: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-local-set"></span>`fn visit_local_set(&mut self, local_index: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-local-tee"></span>`fn visit_local_tee(&mut self, local_index: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-global-get"></span>`fn visit_global_get(&mut self, global_index: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-global-set"></span>`fn visit_global_set(&mut self, global_index: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-load"></span>`fn visit_i32_load(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-load"></span>`fn visit_i64_load(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f32-load"></span>`fn visit_f32_load(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f64-load"></span>`fn visit_f64_load(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-load8-s"></span>`fn visit_i32_load8_s(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-load8-u"></span>`fn visit_i32_load8_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-load16-s"></span>`fn visit_i32_load16_s(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-load16-u"></span>`fn visit_i32_load16_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-load8-s"></span>`fn visit_i64_load8_s(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-load8-u"></span>`fn visit_i64_load8_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-load16-s"></span>`fn visit_i64_load16_s(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-load16-u"></span>`fn visit_i64_load16_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-load32-s"></span>`fn visit_i64_load32_s(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-load32-u"></span>`fn visit_i64_load32_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-store"></span>`fn visit_i32_store(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-store"></span>`fn visit_i64_store(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f32-store"></span>`fn visit_f32_store(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f64-store"></span>`fn visit_f64_store(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-store8"></span>`fn visit_i32_store8(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-store16"></span>`fn visit_i32_store16(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-store8"></span>`fn visit_i64_store8(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-store16"></span>`fn visit_i64_store16(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-store32"></span>`fn visit_i64_store32(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-memory-size"></span>`fn visit_memory_size(&mut self, mem: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-memory-grow"></span>`fn visit_memory_grow(&mut self, mem: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-const"></span>`fn visit_i32_const(&mut self, value: i32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-const"></span>`fn visit_i64_const(&mut self, value: i64) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f32-const"></span>`fn visit_f32_const(&mut self, value: Ieee32) -> Operator<'a>` — [`Ieee32`](#ieee32), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f64-const"></span>`fn visit_f64_const(&mut self, value: Ieee64) -> Operator<'a>` — [`Ieee64`](#ieee64), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-eqz"></span>`fn visit_i32_eqz(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-eq"></span>`fn visit_i32_eq(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-ne"></span>`fn visit_i32_ne(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-lt-s"></span>`fn visit_i32_lt_s(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-lt-u"></span>`fn visit_i32_lt_u(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-gt-s"></span>`fn visit_i32_gt_s(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-gt-u"></span>`fn visit_i32_gt_u(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-le-s"></span>`fn visit_i32_le_s(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-le-u"></span>`fn visit_i32_le_u(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-ge-s"></span>`fn visit_i32_ge_s(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-ge-u"></span>`fn visit_i32_ge_u(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-eqz"></span>`fn visit_i64_eqz(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-eq"></span>`fn visit_i64_eq(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-ne"></span>`fn visit_i64_ne(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-lt-s"></span>`fn visit_i64_lt_s(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-lt-u"></span>`fn visit_i64_lt_u(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-gt-s"></span>`fn visit_i64_gt_s(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-gt-u"></span>`fn visit_i64_gt_u(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-le-s"></span>`fn visit_i64_le_s(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-le-u"></span>`fn visit_i64_le_u(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-ge-s"></span>`fn visit_i64_ge_s(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-ge-u"></span>`fn visit_i64_ge_u(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f32-eq"></span>`fn visit_f32_eq(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f32-ne"></span>`fn visit_f32_ne(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f32-lt"></span>`fn visit_f32_lt(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f32-gt"></span>`fn visit_f32_gt(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f32-le"></span>`fn visit_f32_le(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f32-ge"></span>`fn visit_f32_ge(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f64-eq"></span>`fn visit_f64_eq(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f64-ne"></span>`fn visit_f64_ne(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f64-lt"></span>`fn visit_f64_lt(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f64-gt"></span>`fn visit_f64_gt(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f64-le"></span>`fn visit_f64_le(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f64-ge"></span>`fn visit_f64_ge(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-clz"></span>`fn visit_i32_clz(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-ctz"></span>`fn visit_i32_ctz(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-popcnt"></span>`fn visit_i32_popcnt(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-add"></span>`fn visit_i32_add(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-sub"></span>`fn visit_i32_sub(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-mul"></span>`fn visit_i32_mul(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-div-s"></span>`fn visit_i32_div_s(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-div-u"></span>`fn visit_i32_div_u(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-rem-s"></span>`fn visit_i32_rem_s(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-rem-u"></span>`fn visit_i32_rem_u(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-and"></span>`fn visit_i32_and(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-or"></span>`fn visit_i32_or(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-xor"></span>`fn visit_i32_xor(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-shl"></span>`fn visit_i32_shl(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-shr-s"></span>`fn visit_i32_shr_s(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-shr-u"></span>`fn visit_i32_shr_u(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-rotl"></span>`fn visit_i32_rotl(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-rotr"></span>`fn visit_i32_rotr(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-clz"></span>`fn visit_i64_clz(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-ctz"></span>`fn visit_i64_ctz(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-popcnt"></span>`fn visit_i64_popcnt(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-add"></span>`fn visit_i64_add(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-sub"></span>`fn visit_i64_sub(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-mul"></span>`fn visit_i64_mul(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-div-s"></span>`fn visit_i64_div_s(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-div-u"></span>`fn visit_i64_div_u(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-rem-s"></span>`fn visit_i64_rem_s(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-rem-u"></span>`fn visit_i64_rem_u(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-and"></span>`fn visit_i64_and(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-or"></span>`fn visit_i64_or(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-xor"></span>`fn visit_i64_xor(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-shl"></span>`fn visit_i64_shl(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-shr-s"></span>`fn visit_i64_shr_s(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-shr-u"></span>`fn visit_i64_shr_u(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-rotl"></span>`fn visit_i64_rotl(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-rotr"></span>`fn visit_i64_rotr(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f32-abs"></span>`fn visit_f32_abs(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f32-neg"></span>`fn visit_f32_neg(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f32-ceil"></span>`fn visit_f32_ceil(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f32-floor"></span>`fn visit_f32_floor(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f32-trunc"></span>`fn visit_f32_trunc(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f32-nearest"></span>`fn visit_f32_nearest(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f32-sqrt"></span>`fn visit_f32_sqrt(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f32-add"></span>`fn visit_f32_add(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f32-sub"></span>`fn visit_f32_sub(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f32-mul"></span>`fn visit_f32_mul(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f32-div"></span>`fn visit_f32_div(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f32-min"></span>`fn visit_f32_min(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f32-max"></span>`fn visit_f32_max(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f32-copysign"></span>`fn visit_f32_copysign(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f64-abs"></span>`fn visit_f64_abs(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f64-neg"></span>`fn visit_f64_neg(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f64-ceil"></span>`fn visit_f64_ceil(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f64-floor"></span>`fn visit_f64_floor(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f64-trunc"></span>`fn visit_f64_trunc(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f64-nearest"></span>`fn visit_f64_nearest(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f64-sqrt"></span>`fn visit_f64_sqrt(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f64-add"></span>`fn visit_f64_add(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f64-sub"></span>`fn visit_f64_sub(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f64-mul"></span>`fn visit_f64_mul(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f64-div"></span>`fn visit_f64_div(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f64-min"></span>`fn visit_f64_min(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f64-max"></span>`fn visit_f64_max(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f64-copysign"></span>`fn visit_f64_copysign(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-wrap-i64"></span>`fn visit_i32_wrap_i64(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-trunc-f32-s"></span>`fn visit_i32_trunc_f32_s(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-trunc-f32-u"></span>`fn visit_i32_trunc_f32_u(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-trunc-f64-s"></span>`fn visit_i32_trunc_f64_s(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-trunc-f64-u"></span>`fn visit_i32_trunc_f64_u(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-extend-i32-s"></span>`fn visit_i64_extend_i32_s(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-extend-i32-u"></span>`fn visit_i64_extend_i32_u(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-trunc-f32-s"></span>`fn visit_i64_trunc_f32_s(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-trunc-f32-u"></span>`fn visit_i64_trunc_f32_u(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-trunc-f64-s"></span>`fn visit_i64_trunc_f64_s(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-trunc-f64-u"></span>`fn visit_i64_trunc_f64_u(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f32-convert-i32-s"></span>`fn visit_f32_convert_i32_s(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f32-convert-i32-u"></span>`fn visit_f32_convert_i32_u(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f32-convert-i64-s"></span>`fn visit_f32_convert_i64_s(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f32-convert-i64-u"></span>`fn visit_f32_convert_i64_u(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f32-demote-f64"></span>`fn visit_f32_demote_f64(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f64-convert-i32-s"></span>`fn visit_f64_convert_i32_s(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f64-convert-i32-u"></span>`fn visit_f64_convert_i32_u(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f64-convert-i64-s"></span>`fn visit_f64_convert_i64_s(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f64-convert-i64-u"></span>`fn visit_f64_convert_i64_u(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f64-promote-f32"></span>`fn visit_f64_promote_f32(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-reinterpret-f32"></span>`fn visit_i32_reinterpret_f32(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-reinterpret-f64"></span>`fn visit_i64_reinterpret_f64(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f32-reinterpret-i32"></span>`fn visit_f32_reinterpret_i32(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-f64-reinterpret-i64"></span>`fn visit_f64_reinterpret_i64(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-extend8-s"></span>`fn visit_i32_extend8_s(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-extend16-s"></span>`fn visit_i32_extend16_s(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-extend8-s"></span>`fn visit_i64_extend8_s(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-extend16-s"></span>`fn visit_i64_extend16_s(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-extend32-s"></span>`fn visit_i64_extend32_s(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-ref-eq"></span>`fn visit_ref_eq(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-struct-new"></span>`fn visit_struct_new(&mut self, struct_type_index: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-struct-new-default"></span>`fn visit_struct_new_default(&mut self, struct_type_index: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-struct-get"></span>`fn visit_struct_get(&mut self, struct_type_index: u32, field_index: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-struct-get-s"></span>`fn visit_struct_get_s(&mut self, struct_type_index: u32, field_index: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-struct-get-u"></span>`fn visit_struct_get_u(&mut self, struct_type_index: u32, field_index: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-struct-set"></span>`fn visit_struct_set(&mut self, struct_type_index: u32, field_index: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-array-new"></span>`fn visit_array_new(&mut self, array_type_index: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-array-new-default"></span>`fn visit_array_new_default(&mut self, array_type_index: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-array-new-fixed"></span>`fn visit_array_new_fixed(&mut self, array_type_index: u32, array_size: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-array-new-data"></span>`fn visit_array_new_data(&mut self, array_type_index: u32, array_data_index: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-array-new-elem"></span>`fn visit_array_new_elem(&mut self, array_type_index: u32, array_elem_index: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-array-get"></span>`fn visit_array_get(&mut self, array_type_index: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-array-get-s"></span>`fn visit_array_get_s(&mut self, array_type_index: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-array-get-u"></span>`fn visit_array_get_u(&mut self, array_type_index: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-array-set"></span>`fn visit_array_set(&mut self, array_type_index: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-array-len"></span>`fn visit_array_len(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-array-fill"></span>`fn visit_array_fill(&mut self, array_type_index: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-array-copy"></span>`fn visit_array_copy(&mut self, array_type_index_dst: u32, array_type_index_src: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-array-init-data"></span>`fn visit_array_init_data(&mut self, array_type_index: u32, array_data_index: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-array-init-elem"></span>`fn visit_array_init_elem(&mut self, array_type_index: u32, array_elem_index: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-ref-test-non-null"></span>`fn visit_ref_test_non_null(&mut self, hty: HeapType) -> Operator<'a>` — [`HeapType`](#heaptype), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-ref-test-nullable"></span>`fn visit_ref_test_nullable(&mut self, hty: HeapType) -> Operator<'a>` — [`HeapType`](#heaptype), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-ref-cast-non-null"></span>`fn visit_ref_cast_non_null(&mut self, hty: HeapType) -> Operator<'a>` — [`HeapType`](#heaptype), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-ref-cast-nullable"></span>`fn visit_ref_cast_nullable(&mut self, hty: HeapType) -> Operator<'a>` — [`HeapType`](#heaptype), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-br-on-cast"></span>`fn visit_br_on_cast(&mut self, relative_depth: u32, from_ref_type: RefType, to_ref_type: RefType) -> Operator<'a>` — [`RefType`](#reftype), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-br-on-cast-fail"></span>`fn visit_br_on_cast_fail(&mut self, relative_depth: u32, from_ref_type: RefType, to_ref_type: RefType) -> Operator<'a>` — [`RefType`](#reftype), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-any-convert-extern"></span>`fn visit_any_convert_extern(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-extern-convert-any"></span>`fn visit_extern_convert_any(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-ref-i31"></span>`fn visit_ref_i31(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i31-get-s"></span>`fn visit_i31_get_s(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i31-get-u"></span>`fn visit_i31_get_u(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-struct-new-desc"></span>`fn visit_struct_new_desc(&mut self, struct_type_index: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-struct-new-default-desc"></span>`fn visit_struct_new_default_desc(&mut self, struct_type_index: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-ref-get-desc"></span>`fn visit_ref_get_desc(&mut self, type_index: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-ref-cast-desc-non-null"></span>`fn visit_ref_cast_desc_non_null(&mut self, hty: HeapType) -> Operator<'a>` — [`HeapType`](#heaptype), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-ref-cast-desc-nullable"></span>`fn visit_ref_cast_desc_nullable(&mut self, hty: HeapType) -> Operator<'a>` — [`HeapType`](#heaptype), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-br-on-cast-desc"></span>`fn visit_br_on_cast_desc(&mut self, relative_depth: u32, from_ref_type: RefType, to_ref_type: RefType) -> Operator<'a>` — [`RefType`](#reftype), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-br-on-cast-desc-fail"></span>`fn visit_br_on_cast_desc_fail(&mut self, relative_depth: u32, from_ref_type: RefType, to_ref_type: RefType) -> Operator<'a>` — [`RefType`](#reftype), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-trunc-sat-f32-s"></span>`fn visit_i32_trunc_sat_f32_s(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-trunc-sat-f32-u"></span>`fn visit_i32_trunc_sat_f32_u(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-trunc-sat-f64-s"></span>`fn visit_i32_trunc_sat_f64_s(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-trunc-sat-f64-u"></span>`fn visit_i32_trunc_sat_f64_u(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-trunc-sat-f32-s"></span>`fn visit_i64_trunc_sat_f32_s(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-trunc-sat-f32-u"></span>`fn visit_i64_trunc_sat_f32_u(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-trunc-sat-f64-s"></span>`fn visit_i64_trunc_sat_f64_s(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-trunc-sat-f64-u"></span>`fn visit_i64_trunc_sat_f64_u(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-memory-init"></span>`fn visit_memory_init(&mut self, data_index: u32, mem: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-data-drop"></span>`fn visit_data_drop(&mut self, data_index: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-memory-copy"></span>`fn visit_memory_copy(&mut self, dst_mem: u32, src_mem: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-memory-fill"></span>`fn visit_memory_fill(&mut self, mem: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-table-init"></span>`fn visit_table_init(&mut self, elem_index: u32, table: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-elem-drop"></span>`fn visit_elem_drop(&mut self, elem_index: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-table-copy"></span>`fn visit_table_copy(&mut self, dst_table: u32, src_table: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-typed-select"></span>`fn visit_typed_select(&mut self, ty: ValType) -> Operator<'a>` — [`ValType`](#valtype), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-typed-select-multi"></span>`fn visit_typed_select_multi(&mut self, tys: Vec<ValType>) -> Operator<'a>` — [`Vec`](../../prelude/index.md#vec), [`ValType`](#valtype), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-ref-null"></span>`fn visit_ref_null(&mut self, hty: HeapType) -> Operator<'a>` — [`HeapType`](#heaptype), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-ref-is-null"></span>`fn visit_ref_is_null(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-ref-func"></span>`fn visit_ref_func(&mut self, function_index: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-table-fill"></span>`fn visit_table_fill(&mut self, table: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-table-get"></span>`fn visit_table_get(&mut self, table: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-table-set"></span>`fn visit_table_set(&mut self, table: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-table-grow"></span>`fn visit_table_grow(&mut self, table: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-table-size"></span>`fn visit_table_size(&mut self, table: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-return-call"></span>`fn visit_return_call(&mut self, function_index: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-return-call-indirect"></span>`fn visit_return_call_indirect(&mut self, type_index: u32, table_index: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-memory-discard"></span>`fn visit_memory_discard(&mut self, mem: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-memory-atomic-notify"></span>`fn visit_memory_atomic_notify(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-memory-atomic-wait32"></span>`fn visit_memory_atomic_wait32(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-memory-atomic-wait64"></span>`fn visit_memory_atomic_wait64(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-atomic-fence"></span>`fn visit_atomic_fence(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-load"></span>`fn visit_i32_atomic_load(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-load"></span>`fn visit_i64_atomic_load(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-load8-u"></span>`fn visit_i32_atomic_load8_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-load16-u"></span>`fn visit_i32_atomic_load16_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-load8-u"></span>`fn visit_i64_atomic_load8_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-load16-u"></span>`fn visit_i64_atomic_load16_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-load32-u"></span>`fn visit_i64_atomic_load32_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-store"></span>`fn visit_i32_atomic_store(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-store"></span>`fn visit_i64_atomic_store(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-store8"></span>`fn visit_i32_atomic_store8(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-store16"></span>`fn visit_i32_atomic_store16(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-store8"></span>`fn visit_i64_atomic_store8(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-store16"></span>`fn visit_i64_atomic_store16(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-store32"></span>`fn visit_i64_atomic_store32(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-rmw-add"></span>`fn visit_i32_atomic_rmw_add(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw-add"></span>`fn visit_i64_atomic_rmw_add(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-rmw8-add-u"></span>`fn visit_i32_atomic_rmw8_add_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-rmw16-add-u"></span>`fn visit_i32_atomic_rmw16_add_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw8-add-u"></span>`fn visit_i64_atomic_rmw8_add_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw16-add-u"></span>`fn visit_i64_atomic_rmw16_add_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw32-add-u"></span>`fn visit_i64_atomic_rmw32_add_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-rmw-sub"></span>`fn visit_i32_atomic_rmw_sub(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw-sub"></span>`fn visit_i64_atomic_rmw_sub(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-rmw8-sub-u"></span>`fn visit_i32_atomic_rmw8_sub_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-rmw16-sub-u"></span>`fn visit_i32_atomic_rmw16_sub_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw8-sub-u"></span>`fn visit_i64_atomic_rmw8_sub_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw16-sub-u"></span>`fn visit_i64_atomic_rmw16_sub_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw32-sub-u"></span>`fn visit_i64_atomic_rmw32_sub_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-rmw-and"></span>`fn visit_i32_atomic_rmw_and(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw-and"></span>`fn visit_i64_atomic_rmw_and(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-rmw8-and-u"></span>`fn visit_i32_atomic_rmw8_and_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-rmw16-and-u"></span>`fn visit_i32_atomic_rmw16_and_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw8-and-u"></span>`fn visit_i64_atomic_rmw8_and_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw16-and-u"></span>`fn visit_i64_atomic_rmw16_and_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw32-and-u"></span>`fn visit_i64_atomic_rmw32_and_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-rmw-or"></span>`fn visit_i32_atomic_rmw_or(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw-or"></span>`fn visit_i64_atomic_rmw_or(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-rmw8-or-u"></span>`fn visit_i32_atomic_rmw8_or_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-rmw16-or-u"></span>`fn visit_i32_atomic_rmw16_or_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw8-or-u"></span>`fn visit_i64_atomic_rmw8_or_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw16-or-u"></span>`fn visit_i64_atomic_rmw16_or_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw32-or-u"></span>`fn visit_i64_atomic_rmw32_or_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-rmw-xor"></span>`fn visit_i32_atomic_rmw_xor(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw-xor"></span>`fn visit_i64_atomic_rmw_xor(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-rmw8-xor-u"></span>`fn visit_i32_atomic_rmw8_xor_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-rmw16-xor-u"></span>`fn visit_i32_atomic_rmw16_xor_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw8-xor-u"></span>`fn visit_i64_atomic_rmw8_xor_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw16-xor-u"></span>`fn visit_i64_atomic_rmw16_xor_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw32-xor-u"></span>`fn visit_i64_atomic_rmw32_xor_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-rmw-xchg"></span>`fn visit_i32_atomic_rmw_xchg(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw-xchg"></span>`fn visit_i64_atomic_rmw_xchg(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-rmw8-xchg-u"></span>`fn visit_i32_atomic_rmw8_xchg_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-rmw16-xchg-u"></span>`fn visit_i32_atomic_rmw16_xchg_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw8-xchg-u"></span>`fn visit_i64_atomic_rmw8_xchg_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw16-xchg-u"></span>`fn visit_i64_atomic_rmw16_xchg_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw32-xchg-u"></span>`fn visit_i64_atomic_rmw32_xchg_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-rmw-cmpxchg"></span>`fn visit_i32_atomic_rmw_cmpxchg(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw-cmpxchg"></span>`fn visit_i64_atomic_rmw_cmpxchg(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-rmw8-cmpxchg-u"></span>`fn visit_i32_atomic_rmw8_cmpxchg_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-rmw16-cmpxchg-u"></span>`fn visit_i32_atomic_rmw16_cmpxchg_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw8-cmpxchg-u"></span>`fn visit_i64_atomic_rmw8_cmpxchg_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw16-cmpxchg-u"></span>`fn visit_i64_atomic_rmw16_cmpxchg_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw32-cmpxchg-u"></span>`fn visit_i64_atomic_rmw32_cmpxchg_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](#memarg), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-try-table"></span>`fn visit_try_table(&mut self, try_table: TryTable) -> Operator<'a>` — [`TryTable`](#trytable), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-throw"></span>`fn visit_throw(&mut self, tag_index: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-throw-ref"></span>`fn visit_throw_ref(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-try"></span>`fn visit_try(&mut self, blockty: BlockType) -> Operator<'a>` — [`BlockType`](#blocktype), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-catch"></span>`fn visit_catch(&mut self, tag_index: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-rethrow"></span>`fn visit_rethrow(&mut self, relative_depth: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-delegate"></span>`fn visit_delegate(&mut self, relative_depth: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-catch-all"></span>`fn visit_catch_all(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-global-atomic-get"></span>`fn visit_global_atomic_get(&mut self, ordering: Ordering, global_index: u32) -> Operator<'a>` — [`Ordering`](#ordering), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-global-atomic-set"></span>`fn visit_global_atomic_set(&mut self, ordering: Ordering, global_index: u32) -> Operator<'a>` — [`Ordering`](#ordering), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-global-atomic-rmw-add"></span>`fn visit_global_atomic_rmw_add(&mut self, ordering: Ordering, global_index: u32) -> Operator<'a>` — [`Ordering`](#ordering), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-global-atomic-rmw-sub"></span>`fn visit_global_atomic_rmw_sub(&mut self, ordering: Ordering, global_index: u32) -> Operator<'a>` — [`Ordering`](#ordering), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-global-atomic-rmw-and"></span>`fn visit_global_atomic_rmw_and(&mut self, ordering: Ordering, global_index: u32) -> Operator<'a>` — [`Ordering`](#ordering), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-global-atomic-rmw-or"></span>`fn visit_global_atomic_rmw_or(&mut self, ordering: Ordering, global_index: u32) -> Operator<'a>` — [`Ordering`](#ordering), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-global-atomic-rmw-xor"></span>`fn visit_global_atomic_rmw_xor(&mut self, ordering: Ordering, global_index: u32) -> Operator<'a>` — [`Ordering`](#ordering), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-global-atomic-rmw-xchg"></span>`fn visit_global_atomic_rmw_xchg(&mut self, ordering: Ordering, global_index: u32) -> Operator<'a>` — [`Ordering`](#ordering), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-global-atomic-rmw-cmpxchg"></span>`fn visit_global_atomic_rmw_cmpxchg(&mut self, ordering: Ordering, global_index: u32) -> Operator<'a>` — [`Ordering`](#ordering), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-table-atomic-get"></span>`fn visit_table_atomic_get(&mut self, ordering: Ordering, table_index: u32) -> Operator<'a>` — [`Ordering`](#ordering), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-table-atomic-set"></span>`fn visit_table_atomic_set(&mut self, ordering: Ordering, table_index: u32) -> Operator<'a>` — [`Ordering`](#ordering), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-table-atomic-rmw-xchg"></span>`fn visit_table_atomic_rmw_xchg(&mut self, ordering: Ordering, table_index: u32) -> Operator<'a>` — [`Ordering`](#ordering), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-table-atomic-rmw-cmpxchg"></span>`fn visit_table_atomic_rmw_cmpxchg(&mut self, ordering: Ordering, table_index: u32) -> Operator<'a>` — [`Ordering`](#ordering), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-struct-atomic-get"></span>`fn visit_struct_atomic_get(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> Operator<'a>` — [`Ordering`](#ordering), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-struct-atomic-get-s"></span>`fn visit_struct_atomic_get_s(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> Operator<'a>` — [`Ordering`](#ordering), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-struct-atomic-get-u"></span>`fn visit_struct_atomic_get_u(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> Operator<'a>` — [`Ordering`](#ordering), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-struct-atomic-set"></span>`fn visit_struct_atomic_set(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> Operator<'a>` — [`Ordering`](#ordering), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-struct-atomic-rmw-add"></span>`fn visit_struct_atomic_rmw_add(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> Operator<'a>` — [`Ordering`](#ordering), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-struct-atomic-rmw-sub"></span>`fn visit_struct_atomic_rmw_sub(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> Operator<'a>` — [`Ordering`](#ordering), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-struct-atomic-rmw-and"></span>`fn visit_struct_atomic_rmw_and(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> Operator<'a>` — [`Ordering`](#ordering), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-struct-atomic-rmw-or"></span>`fn visit_struct_atomic_rmw_or(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> Operator<'a>` — [`Ordering`](#ordering), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-struct-atomic-rmw-xor"></span>`fn visit_struct_atomic_rmw_xor(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> Operator<'a>` — [`Ordering`](#ordering), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-struct-atomic-rmw-xchg"></span>`fn visit_struct_atomic_rmw_xchg(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> Operator<'a>` — [`Ordering`](#ordering), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-struct-atomic-rmw-cmpxchg"></span>`fn visit_struct_atomic_rmw_cmpxchg(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> Operator<'a>` — [`Ordering`](#ordering), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-array-atomic-get"></span>`fn visit_array_atomic_get(&mut self, ordering: Ordering, array_type_index: u32) -> Operator<'a>` — [`Ordering`](#ordering), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-array-atomic-get-s"></span>`fn visit_array_atomic_get_s(&mut self, ordering: Ordering, array_type_index: u32) -> Operator<'a>` — [`Ordering`](#ordering), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-array-atomic-get-u"></span>`fn visit_array_atomic_get_u(&mut self, ordering: Ordering, array_type_index: u32) -> Operator<'a>` — [`Ordering`](#ordering), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-array-atomic-set"></span>`fn visit_array_atomic_set(&mut self, ordering: Ordering, array_type_index: u32) -> Operator<'a>` — [`Ordering`](#ordering), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-array-atomic-rmw-add"></span>`fn visit_array_atomic_rmw_add(&mut self, ordering: Ordering, array_type_index: u32) -> Operator<'a>` — [`Ordering`](#ordering), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-array-atomic-rmw-sub"></span>`fn visit_array_atomic_rmw_sub(&mut self, ordering: Ordering, array_type_index: u32) -> Operator<'a>` — [`Ordering`](#ordering), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-array-atomic-rmw-and"></span>`fn visit_array_atomic_rmw_and(&mut self, ordering: Ordering, array_type_index: u32) -> Operator<'a>` — [`Ordering`](#ordering), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-array-atomic-rmw-or"></span>`fn visit_array_atomic_rmw_or(&mut self, ordering: Ordering, array_type_index: u32) -> Operator<'a>` — [`Ordering`](#ordering), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-array-atomic-rmw-xor"></span>`fn visit_array_atomic_rmw_xor(&mut self, ordering: Ordering, array_type_index: u32) -> Operator<'a>` — [`Ordering`](#ordering), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-array-atomic-rmw-xchg"></span>`fn visit_array_atomic_rmw_xchg(&mut self, ordering: Ordering, array_type_index: u32) -> Operator<'a>` — [`Ordering`](#ordering), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-array-atomic-rmw-cmpxchg"></span>`fn visit_array_atomic_rmw_cmpxchg(&mut self, ordering: Ordering, array_type_index: u32) -> Operator<'a>` — [`Ordering`](#ordering), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-ref-i31-shared"></span>`fn visit_ref_i31_shared(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-call-ref"></span>`fn visit_call_ref(&mut self, type_index: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-return-call-ref"></span>`fn visit_return_call_ref(&mut self, type_index: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-ref-as-non-null"></span>`fn visit_ref_as_non_null(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-br-on-null"></span>`fn visit_br_on_null(&mut self, relative_depth: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-br-on-non-null"></span>`fn visit_br_on_non_null(&mut self, relative_depth: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-cont-new"></span>`fn visit_cont_new(&mut self, cont_type_index: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-cont-bind"></span>`fn visit_cont_bind(&mut self, argument_index: u32, result_index: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-suspend"></span>`fn visit_suspend(&mut self, tag_index: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-resume"></span>`fn visit_resume(&mut self, cont_type_index: u32, resume_table: ResumeTable) -> Operator<'a>` — [`ResumeTable`](#resumetable), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-resume-throw"></span>`fn visit_resume_throw(&mut self, cont_type_index: u32, tag_index: u32, resume_table: ResumeTable) -> Operator<'a>` — [`ResumeTable`](#resumetable), [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-switch"></span>`fn visit_switch(&mut self, cont_type_index: u32, tag_index: u32) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-add128"></span>`fn visit_i64_add128(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-sub128"></span>`fn visit_i64_sub128(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-mul-wide-s"></span>`fn visit_i64_mul_wide_s(&mut self) -> Operator<'a>` — [`Operator`](#operator)

- <span id="operatorfactory-visitoperator-visit-i64-mul-wide-u"></span>`fn visit_i64_mul_wide_u(&mut self) -> Operator<'a>` — [`Operator`](#operator)

### `ProducersField<'a>`

```rust
struct ProducersField<'a> {
    pub name: &'a str,
    pub values: crate::SectionLimited<'a, ProducersFieldValue<'a>>,
}
```

A field from the producers custom section.

#### Fields

- **`name`**: `&'a str`

  The name of the field.

- **`values`**: `crate::SectionLimited<'a, ProducersFieldValue<'a>>`

  The values specified for this field

#### Trait Implementations

##### `impl Clone for ProducersField<'a>`

- <span id="producersfield-clone"></span>`fn clone(&self) -> ProducersField<'a>` — [`ProducersField`](#producersfield)

##### `impl Debug for ProducersField<'a>`

- <span id="producersfield-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FromReader for ProducersField<'a>`

- <span id="producersfield-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

### `ProducersFieldValue<'a>`

```rust
struct ProducersFieldValue<'a> {
    pub name: &'a str,
    pub version: &'a str,
}
```

Represents a field value in the producers custom section.

#### Fields

- **`name`**: `&'a str`

  The field name.

- **`version`**: `&'a str`

  The field version.

#### Trait Implementations

##### `impl Clone for ProducersFieldValue<'a>`

- <span id="producersfieldvalue-clone"></span>`fn clone(&self) -> ProducersFieldValue<'a>` — [`ProducersFieldValue`](#producersfieldvalue)

##### `impl Copy for ProducersFieldValue<'a>`

##### `impl Debug for ProducersFieldValue<'a>`

- <span id="producersfieldvalue-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FromReader for ProducersFieldValue<'a>`

- <span id="producersfieldvalue-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

### `RelocSectionReader<'a>`

```rust
struct RelocSectionReader<'a> {
    section: u32,
    range: core::ops::Range<usize>,
    entries: crate::SectionLimited<'a, RelocationEntry>,
}
```

Reader for reloc.* sections as defined by
<https://github.com/WebAssembly/tool-conventions/blob/main/Linking.md#relocation-sections>.

#### Implementations

- <span id="relocsectionreader-new"></span>`fn new(reader: BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

  Creates a new reader for a `reloc.*` section starting at

  `original_position` within the wasm file.

- <span id="relocsectionreader-section-index"></span>`fn section_index(&self) -> u32`

  Index of section to which the relocations apply.

- <span id="relocsectionreader-range"></span>`fn range(&self) -> Range<usize>`

  The byte range of the entire section.

- <span id="relocsectionreader-entries"></span>`fn entries(&self) -> SectionLimited<'a, RelocationEntry>` — [`SectionLimited`](../../index.md#sectionlimited), [`RelocationEntry`](#relocationentry)

  The relocation entries.

#### Trait Implementations

##### `impl Clone for RelocSectionReader<'a>`

- <span id="relocsectionreader-clone"></span>`fn clone(&self) -> RelocSectionReader<'a>` — [`RelocSectionReader`](#relocsectionreader)

##### `impl Debug for RelocSectionReader<'a>`

- <span id="relocsectionreader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `RelocationEntry`

```rust
struct RelocationEntry {
    pub ty: RelocationType,
    pub offset: u32,
    pub index: u32,
    pub addend: i64,
}
```

Single relocation entry within a `reloc.*` section, as defined at
<https://github.com/WebAssembly/tool-conventions/blob/main/Linking.md#relocation-sections>.

#### Fields

- **`ty`**: `RelocationType`

  Relocation entry type.

- **`offset`**: `u32`

  Offset in bytes from the start of the section indicated by
  `RelocSectionReader::section` targeted by this relocation.

- **`index`**: `u32`

  Index in the symbol table contained in the linking section that
  corresponds to the value at `offset`.

- **`addend`**: `i64`

  Addend to add to the address, or `0` if not applicable. The value must
  be consistent with the `self.ty.addend_kind()`.

#### Implementations

- <span id="relocationentry-relocation-range"></span>`fn relocation_range(&self) -> Range<usize>`

  Byte range relative to the start of the section indicated by

  `RelocSectionReader::section` targeted by this relocation.

#### Trait Implementations

##### `impl Clone for RelocationEntry`

- <span id="relocationentry-clone"></span>`fn clone(&self) -> RelocationEntry` — [`RelocationEntry`](#relocationentry)

##### `impl Copy for RelocationEntry`

##### `impl Debug for RelocationEntry`

- <span id="relocationentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RelocationEntry`

##### `impl FromReader for RelocationEntry`

- <span id="relocationentry-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

##### `impl Hash for RelocationEntry`

- <span id="relocationentry-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for RelocationEntry`

- <span id="relocationentry-partialeq-eq"></span>`fn eq(&self, other: &RelocationEntry) -> bool` — [`RelocationEntry`](#relocationentry)

##### `impl StructuralPartialEq for RelocationEntry`

### `Table<'a>`

```rust
struct Table<'a> {
    pub ty: crate::TableType,
    pub init: TableInit<'a>,
}
```

Type information about a table defined in the table section of a WebAssembly
module.

#### Fields

- **`ty`**: `crate::TableType`

  The type of this table, including its element type and its limits.

- **`init`**: `TableInit<'a>`

  The initialization expression for the table.

#### Trait Implementations

##### `impl Clone for Table<'a>`

- <span id="table-clone"></span>`fn clone(&self) -> Table<'a>` — [`Table`](#table)

##### `impl Debug for Table<'a>`

- <span id="table-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FromReader for Table<'a>`

- <span id="table-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

### `PackedIndex`

```rust
struct PackedIndex(u32);
```

A packed representation of a type index.

This type is morally an `enum` of either:

1. An index into a Wasm module's type space.

2. A `CoreTypeId` identifier.

3. An index into a recursion group's elements.

The latter two variants are *canonical* while the first is not. Reading raw
types will produce (1), while working with types after validation will
produce (2) and (3).

#### Implementations

- <span id="packedindex-const-unused-mask"></span>`const UNUSED_MASK: u32`

- <span id="packedindex-const-kind-mask"></span>`const KIND_MASK: u32`

- <span id="packedindex-const-index-mask"></span>`const INDEX_MASK: u32`

- <span id="packedindex-const-module-kind"></span>`const MODULE_KIND: u32`

- <span id="packedindex-const-rec-group-kind"></span>`const REC_GROUP_KIND: u32`

- <span id="packedindex-unchecked-from-u32"></span>`fn unchecked_from_u32(x: u32) -> Self`

- <span id="packedindex-to-u32"></span>`fn to_u32(id: Self) -> u32`

- <span id="packedindex-can-represent-index"></span>`fn can_represent_index(index: u32) -> bool`

- <span id="packedindex-kind"></span>`fn kind(&self) -> u32`

- <span id="packedindex-index"></span>`fn index(&self) -> u32`

- <span id="packedindex-from-module-index"></span>`fn from_module_index(index: u32) -> Option<Self>`

  Construct a `PackedIndex` from an index into a module's types space.

- <span id="packedindex-from-rec-group-index"></span>`fn from_rec_group_index(index: u32) -> Option<Self>`

  Construct a `PackedIndex` from an index into the index's containing

  recursion group.

- <span id="packedindex-unpack"></span>`fn unpack(&self) -> UnpackedIndex` — [`UnpackedIndex`](#unpackedindex)

  Uncompress this packed index into an actual `enum` that can be matched

  on.

- <span id="packedindex-as-module-index"></span>`fn as_module_index(&self) -> Option<u32>`

  Get the underlying index into a module's types space, if any.

- <span id="packedindex-as-rec-group-index"></span>`fn as_rec_group_index(&self) -> Option<u32>`

  Get the underlying index into the containing recursion group, if any.

#### Trait Implementations

##### `impl Clone for PackedIndex`

- <span id="packedindex-clone"></span>`fn clone(&self) -> PackedIndex` — [`PackedIndex`](#packedindex)

##### `impl Copy for PackedIndex`

##### `impl Debug for PackedIndex`

- <span id="packedindex-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Display for PackedIndex`

- <span id="packedindex-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for PackedIndex`

##### `impl Hash for PackedIndex`

- <span id="packedindex-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for PackedIndex`

- <span id="packedindex-ord-cmp"></span>`fn cmp(&self, other: &PackedIndex) -> cmp::Ordering` — [`PackedIndex`](#packedindex)

##### `impl PartialEq for PackedIndex`

- <span id="packedindex-partialeq-eq"></span>`fn eq(&self, other: &PackedIndex) -> bool` — [`PackedIndex`](#packedindex)

##### `impl PartialOrd for PackedIndex`

- <span id="packedindex-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &PackedIndex) -> option::Option<cmp::Ordering>` — [`PackedIndex`](#packedindex)

##### `impl StructuralPartialEq for PackedIndex`

##### `impl ToString for PackedIndex`

- <span id="packedindex-tostring-to-string"></span>`fn to_string(&self) -> String` — [`String`](../../prelude/index.md#string)

### `RecGroup`

```rust
struct RecGroup {
    inner: RecGroupInner,
}
```

Represents a recursive type group in a WebAssembly module.

#### Implementations

- <span id="recgroup-explicit"></span>`fn explicit(types: Vec<(usize, SubType)>) -> Self` — [`Vec`](../../prelude/index.md#vec), [`SubType`](#subtype)

  Create an explicit `RecGroup` for the given types.

- <span id="recgroup-implicit"></span>`fn implicit(offset: usize, ty: SubType) -> Self` — [`SubType`](#subtype)

  Create an implicit `RecGroup` for a type that was not contained

  in a `(rec ...)`.

- <span id="recgroup-is-explicit-rec-group"></span>`fn is_explicit_rec_group(&self) -> bool`

  Is this an explicit recursion group?

- <span id="recgroup-types"></span>`fn types(&self) -> impl ExactSizeIterator<Item = &SubType> + '_` — [`SubType`](#subtype)

  Returns the list of subtypes in the recursive type group.

- <span id="recgroup-into-types"></span>`fn into_types(self) -> impl ExactSizeIterator<Item = SubType>` — [`SubType`](#subtype)

  Returns an owning iterator of all subtypes in this recursion

  group.

- <span id="recgroup-into-types-and-offsets"></span>`fn into_types_and_offsets(self) -> impl ExactSizeIterator<Item = (usize, SubType)>` — [`SubType`](#subtype)

  Returns an owning iterator of all subtypes in this recursion

  group, along with their offset.

#### Trait Implementations

##### `impl Clone for RecGroup`

- <span id="recgroup-clone"></span>`fn clone(&self) -> RecGroup` — [`RecGroup`](#recgroup)

##### `impl Debug for RecGroup`

- <span id="recgroup-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RecGroup`

##### `impl FromReader for RecGroup`

- <span id="recgroup-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

##### `impl Hash for RecGroup`

- <span id="recgroup-hash"></span>`fn hash<H: Hasher>(&self, hasher: &mut H)`

##### `impl Ord for RecGroup`

- <span id="recgroup-ord-cmp"></span>`fn cmp(&self, other: &Self) -> Ordering`

##### `impl PartialEq for RecGroup`

- <span id="recgroup-partialeq-eq"></span>`fn eq(&self, other: &RecGroup) -> bool` — [`RecGroup`](#recgroup)

##### `impl PartialOrd for RecGroup`

- <span id="recgroup-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering>`

### `SubType`

```rust
struct SubType {
    pub is_final: bool,
    pub supertype_idx: Option<PackedIndex>,
    pub composite_type: CompositeType,
}
```

Represents a subtype of possible other types in a WebAssembly module.

#### Fields

- **`is_final`**: `bool`

  Is the subtype final.

- **`supertype_idx`**: `Option<PackedIndex>`

  The list of supertype indexes. As of GC MVP, there can be at most one supertype.

- **`composite_type`**: `CompositeType`

  The composite type of the subtype.

#### Implementations

- <span id="subtype-unwrap-array"></span>`fn unwrap_array(&self) -> &ArrayType` — [`ArrayType`](#arraytype)

  Unwrap an `ArrayType` or panic.

  

  Does not check finality or whether there is a supertype.

- <span id="subtype-func"></span>`fn func(signature: FuncType, shared: bool) -> Self` — [`FuncType`](#functype)

  Construct a function `SubType`.

- <span id="subtype-unwrap-func"></span>`fn unwrap_func(&self) -> &FuncType` — [`FuncType`](#functype)

  Unwrap an `FuncType` or panic.

  

  Does not check finality or whether there is a supertype.

- <span id="subtype-unwrap-struct"></span>`fn unwrap_struct(&self) -> &StructType` — [`StructType`](#structtype)

  Unwrap an `StructType` or panic.

  

  Does not check finality or whether there is a supertype.

- <span id="subtype-unwrap-cont"></span>`fn unwrap_cont(&self) -> &ContType` — [`ContType`](#conttype)

  Unwrap an `ContType` or panic.

  

  Does not check finality or whether there is a supertype.

#### Trait Implementations

##### `impl Clone for SubType`

- <span id="subtype-clone"></span>`fn clone(&self) -> SubType` — [`SubType`](#subtype)

##### `impl Debug for SubType`

- <span id="subtype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for SubType`

- <span id="subtype-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SubType`

##### `impl FromReader for SubType`

- <span id="subtype-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

##### `impl Hash for SubType`

- <span id="subtype-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for SubType`

- <span id="subtype-ord-cmp"></span>`fn cmp(&self, other: &SubType) -> cmp::Ordering` — [`SubType`](#subtype)

##### `impl PartialEq for SubType`

- <span id="subtype-partialeq-eq"></span>`fn eq(&self, other: &SubType) -> bool` — [`SubType`](#subtype)

##### `impl PartialOrd for SubType`

- <span id="subtype-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &SubType) -> option::Option<cmp::Ordering>` — [`SubType`](#subtype)

##### `impl StructuralPartialEq for SubType`

##### `impl ToString for SubType`

- <span id="subtype-tostring-to-string"></span>`fn to_string(&self) -> String` — [`String`](../../prelude/index.md#string)

### `CompositeType`

```rust
struct CompositeType {
    pub inner: CompositeInnerType,
    pub shared: bool,
    pub descriptor_idx: Option<PackedIndex>,
    pub describes_idx: Option<PackedIndex>,
}
```

Represents a composite type in a WebAssembly module.

#### Fields

- **`inner`**: `CompositeInnerType`

  The type defined inside the composite type.

- **`shared`**: `bool`

  Is the composite type shared? This is part of the
  shared-everything-threads proposal.

- **`descriptor_idx`**: `Option<PackedIndex>`

  The descriptor type.

- **`describes_idx`**: `Option<PackedIndex>`

  The descriptor for type.

#### Implementations

- <span id="compositetype-unwrap-func"></span>`fn unwrap_func(&self) -> &FuncType` — [`FuncType`](#functype)

  Unwrap a `FuncType` or panic.

- <span id="compositetype-unwrap-array"></span>`fn unwrap_array(&self) -> &ArrayType` — [`ArrayType`](#arraytype)

  Unwrap a `ArrayType` or panic.

- <span id="compositetype-unwrap-struct"></span>`fn unwrap_struct(&self) -> &StructType` — [`StructType`](#structtype)

  Unwrap a `StructType` or panic.

- <span id="compositetype-unwrap-cont"></span>`fn unwrap_cont(&self) -> &ContType` — [`ContType`](#conttype)

  Unwrap a `ContType` or panic.

#### Trait Implementations

##### `impl Clone for CompositeType`

- <span id="compositetype-clone"></span>`fn clone(&self) -> CompositeType` — [`CompositeType`](#compositetype)

##### `impl Debug for CompositeType`

- <span id="compositetype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for CompositeType`

- <span id="compositetype-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CompositeType`

##### `impl FromReader for CompositeType`

- <span id="compositetype-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

##### `impl Hash for CompositeType`

- <span id="compositetype-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for CompositeType`

- <span id="compositetype-ord-cmp"></span>`fn cmp(&self, other: &CompositeType) -> cmp::Ordering` — [`CompositeType`](#compositetype)

##### `impl PartialEq for CompositeType`

- <span id="compositetype-partialeq-eq"></span>`fn eq(&self, other: &CompositeType) -> bool` — [`CompositeType`](#compositetype)

##### `impl PartialOrd for CompositeType`

- <span id="compositetype-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &CompositeType) -> option::Option<cmp::Ordering>` — [`CompositeType`](#compositetype)

##### `impl StructuralPartialEq for CompositeType`

##### `impl ToString for CompositeType`

- <span id="compositetype-tostring-to-string"></span>`fn to_string(&self) -> String` — [`String`](../../prelude/index.md#string)

### `FuncType`

```rust
struct FuncType {
    params_results: Box<[ValType]>,
    len_params: usize,
}
```

Represents a type of a function in a WebAssembly module.

#### Fields

- **`params_results`**: `Box<[ValType]>`

  The combined parameters and result types.

- **`len_params`**: `usize`

  The number of parameter types.

#### Implementations

- <span id="functype-new"></span>`fn new<P, R>(params: P, results: R) -> Self`

  Creates a new [`FuncType`](#functype) from the given `params` and `results`.

- <span id="functype-from-raw-parts"></span>`fn from_raw_parts(params_results: Box<[ValType]>, len_params: usize) -> Self` — [`Box`](../../prelude/index.md#box), [`ValType`](#valtype)

  Creates a new [`FuncType`](#functype) fom its raw parts.

  

  # Panics

  

  If `len_params` is greater than the length of `params_results` combined.

- <span id="functype-params"></span>`fn params(&self) -> &[ValType]` — [`ValType`](#valtype)

  Returns a shared slice to the parameter types of the [`FuncType`](#functype).

- <span id="functype-results"></span>`fn results(&self) -> &[ValType]` — [`ValType`](#valtype)

  Returns a shared slice to the result types of the [`FuncType`](#functype).

#### Trait Implementations

##### `impl Clone for FuncType`

- <span id="functype-clone"></span>`fn clone(&self) -> FuncType` — [`FuncType`](#functype)

##### `impl Debug for FuncType`

- <span id="functype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for FuncType`

- <span id="functype-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for FuncType`

##### `impl FromReader for FuncType`

- <span id="functype-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

##### `impl Hash for FuncType`

- <span id="functype-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for FuncType`

- <span id="functype-ord-cmp"></span>`fn cmp(&self, other: &FuncType) -> cmp::Ordering` — [`FuncType`](#functype)

##### `impl PartialEq for FuncType`

- <span id="functype-partialeq-eq"></span>`fn eq(&self, other: &FuncType) -> bool` — [`FuncType`](#functype)

##### `impl PartialOrd for FuncType`

- <span id="functype-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &FuncType) -> option::Option<cmp::Ordering>` — [`FuncType`](#functype)

##### `impl StructuralPartialEq for FuncType`

##### `impl ToString for FuncType`

- <span id="functype-tostring-to-string"></span>`fn to_string(&self) -> String` — [`String`](../../prelude/index.md#string)

### `ArrayType`

```rust
struct ArrayType(FieldType);
```

Represents a type of an array in a WebAssembly module.

#### Trait Implementations

##### `impl Clone for ArrayType`

- <span id="arraytype-clone"></span>`fn clone(&self) -> ArrayType` — [`ArrayType`](#arraytype)

##### `impl Copy for ArrayType`

##### `impl Debug for ArrayType`

- <span id="arraytype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ArrayType`

- <span id="arraytype-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ArrayType`

##### `impl FromReader for ArrayType`

- <span id="arraytype-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

##### `impl Hash for ArrayType`

- <span id="arraytype-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for ArrayType`

- <span id="arraytype-ord-cmp"></span>`fn cmp(&self, other: &ArrayType) -> cmp::Ordering` — [`ArrayType`](#arraytype)

##### `impl PartialEq for ArrayType`

- <span id="arraytype-partialeq-eq"></span>`fn eq(&self, other: &ArrayType) -> bool` — [`ArrayType`](#arraytype)

##### `impl PartialOrd for ArrayType`

- <span id="arraytype-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &ArrayType) -> option::Option<cmp::Ordering>` — [`ArrayType`](#arraytype)

##### `impl StructuralPartialEq for ArrayType`

##### `impl ToString for ArrayType`

- <span id="arraytype-tostring-to-string"></span>`fn to_string(&self) -> String` — [`String`](../../prelude/index.md#string)

### `FieldType`

```rust
struct FieldType {
    pub element_type: StorageType,
    pub mutable: bool,
}
```

Represents a field type of an array or a struct.

#### Fields

- **`element_type`**: `StorageType`

  Array element type.

- **`mutable`**: `bool`

  Are elements mutable.

#### Trait Implementations

##### `impl Clone for FieldType`

- <span id="fieldtype-clone"></span>`fn clone(&self) -> FieldType` — [`FieldType`](#fieldtype)

##### `impl Copy for FieldType`

##### `impl Debug for FieldType`

- <span id="fieldtype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for FieldType`

- <span id="fieldtype-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for FieldType`

##### `impl FromReader for FieldType`

- <span id="fieldtype-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

##### `impl Hash for FieldType`

- <span id="fieldtype-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for FieldType`

- <span id="fieldtype-ord-cmp"></span>`fn cmp(&self, other: &FieldType) -> cmp::Ordering` — [`FieldType`](#fieldtype)

##### `impl PartialEq for FieldType`

- <span id="fieldtype-partialeq-eq"></span>`fn eq(&self, other: &FieldType) -> bool` — [`FieldType`](#fieldtype)

##### `impl PartialOrd for FieldType`

- <span id="fieldtype-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &FieldType) -> option::Option<cmp::Ordering>` — [`FieldType`](#fieldtype)

##### `impl StructuralPartialEq for FieldType`

##### `impl ToString for FieldType`

- <span id="fieldtype-tostring-to-string"></span>`fn to_string(&self) -> String` — [`String`](../../prelude/index.md#string)

### `StructType`

```rust
struct StructType {
    pub fields: Box<[FieldType]>,
}
```

Represents a type of a struct in a WebAssembly module.

#### Fields

- **`fields`**: `Box<[FieldType]>`

  Struct fields.

#### Trait Implementations

##### `impl Clone for StructType`

- <span id="structtype-clone"></span>`fn clone(&self) -> StructType` — [`StructType`](#structtype)

##### `impl Debug for StructType`

- <span id="structtype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for StructType`

- <span id="structtype-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for StructType`

##### `impl FromReader for StructType`

- <span id="structtype-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

##### `impl Hash for StructType`

- <span id="structtype-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for StructType`

- <span id="structtype-ord-cmp"></span>`fn cmp(&self, other: &StructType) -> cmp::Ordering` — [`StructType`](#structtype)

##### `impl PartialEq for StructType`

- <span id="structtype-partialeq-eq"></span>`fn eq(&self, other: &StructType) -> bool` — [`StructType`](#structtype)

##### `impl PartialOrd for StructType`

- <span id="structtype-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &StructType) -> option::Option<cmp::Ordering>` — [`StructType`](#structtype)

##### `impl StructuralPartialEq for StructType`

##### `impl ToString for StructType`

- <span id="structtype-tostring-to-string"></span>`fn to_string(&self) -> String` — [`String`](../../prelude/index.md#string)

### `ContType`

```rust
struct ContType(PackedIndex);
```

Represents a type of a continuation in a WebAssembly module.

#### Trait Implementations

##### `impl Clone for ContType`

- <span id="conttype-clone"></span>`fn clone(&self) -> ContType` — [`ContType`](#conttype)

##### `impl Debug for ContType`

- <span id="conttype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ContType`

- <span id="conttype-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ContType`

##### `impl FromReader for ContType`

- <span id="conttype-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

##### `impl Hash for ContType`

- <span id="conttype-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for ContType`

- <span id="conttype-ord-cmp"></span>`fn cmp(&self, other: &ContType) -> cmp::Ordering` — [`ContType`](#conttype)

##### `impl PartialEq for ContType`

- <span id="conttype-partialeq-eq"></span>`fn eq(&self, other: &ContType) -> bool` — [`ContType`](#conttype)

##### `impl PartialOrd for ContType`

- <span id="conttype-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &ContType) -> option::Option<cmp::Ordering>` — [`ContType`](#conttype)

##### `impl StructuralPartialEq for ContType`

##### `impl ToString for ContType`

- <span id="conttype-tostring-to-string"></span>`fn to_string(&self) -> String` — [`String`](../../prelude/index.md#string)

### `RefType`

```rust
struct RefType(u32);
```

A reference type.

The reference types proposal first introduced `externref` and
`funcref`.

The function references proposal introduced typed function
references.

The GC proposal introduces heap types: any, eq, i31, struct, array,
nofunc, noextern, none.

#### Implementations

- <span id="reftype-const-concrete-bit"></span>`const CONCRETE_BIT: u32`

- <span id="reftype-const-exact-bit"></span>`const EXACT_BIT: u32`

- <span id="reftype-const-nullable-bit"></span>`const NULLABLE_BIT: u32`

- <span id="reftype-const-shared-bit"></span>`const SHARED_BIT: u32`

- <span id="reftype-const-abstype-mask"></span>`const ABSTYPE_MASK: u32`

- <span id="reftype-const-any-abstype"></span>`const ANY_ABSTYPE: u32`

- <span id="reftype-const-eq-abstype"></span>`const EQ_ABSTYPE: u32`

- <span id="reftype-const-i31-abstype"></span>`const I31_ABSTYPE: u32`

- <span id="reftype-const-struct-abstype"></span>`const STRUCT_ABSTYPE: u32`

- <span id="reftype-const-array-abstype"></span>`const ARRAY_ABSTYPE: u32`

- <span id="reftype-const-func-abstype"></span>`const FUNC_ABSTYPE: u32`

- <span id="reftype-const-nofunc-abstype"></span>`const NOFUNC_ABSTYPE: u32`

- <span id="reftype-const-extern-abstype"></span>`const EXTERN_ABSTYPE: u32`

- <span id="reftype-const-noextern-abstype"></span>`const NOEXTERN_ABSTYPE: u32`

- <span id="reftype-const-exn-abstype"></span>`const EXN_ABSTYPE: u32`

- <span id="reftype-const-noexn-abstype"></span>`const NOEXN_ABSTYPE: u32`

- <span id="reftype-const-none-abstype"></span>`const NONE_ABSTYPE: u32`

- <span id="reftype-const-cont-abstype"></span>`const CONT_ABSTYPE: u32`

- <span id="reftype-const-nocont-abstype"></span>`const NOCONT_ABSTYPE: u32`

- <span id="reftype-const-index-mask"></span>`const INDEX_MASK: u32`

- <span id="reftype-const-funcref"></span>`const FUNCREF: Self`

- <span id="reftype-const-externref"></span>`const EXTERNREF: Self`

- <span id="reftype-const-anyref"></span>`const ANYREF: Self`

- <span id="reftype-const-nullref"></span>`const NULLREF: Self`

- <span id="reftype-const-nullexternref"></span>`const NULLEXTERNREF: Self`

- <span id="reftype-const-nullfuncref"></span>`const NULLFUNCREF: Self`

- <span id="reftype-const-eqref"></span>`const EQREF: Self`

- <span id="reftype-const-structref"></span>`const STRUCTREF: Self`

- <span id="reftype-const-arrayref"></span>`const ARRAYREF: Self`

- <span id="reftype-const-i31ref"></span>`const I31REF: Self`

- <span id="reftype-const-exnref"></span>`const EXNREF: Self`

- <span id="reftype-const-nullexnref"></span>`const NULLEXNREF: Self`

- <span id="reftype-const-contref"></span>`const CONTREF: Self`

- <span id="reftype-const-nullcontref"></span>`const NULLCONTREF: Self`

- <span id="reftype-const-func"></span>`const FUNC: Self`

- <span id="reftype-const-extern"></span>`const EXTERN: Self`

- <span id="reftype-const-any"></span>`const ANY: Self`

- <span id="reftype-const-none"></span>`const NONE: Self`

- <span id="reftype-const-noextern"></span>`const NOEXTERN: Self`

- <span id="reftype-const-nofunc"></span>`const NOFUNC: Self`

- <span id="reftype-const-eq"></span>`const EQ: Self`

- <span id="reftype-const-struct"></span>`const STRUCT: Self`

- <span id="reftype-const-array"></span>`const ARRAY: Self`

- <span id="reftype-const-i31"></span>`const I31: Self`

- <span id="reftype-const-exn"></span>`const EXN: Self`

- <span id="reftype-const-noexn"></span>`const NOEXN: Self`

- <span id="reftype-const-cont"></span>`const CONT: Self`

- <span id="reftype-const-nocont"></span>`const NOCONT: Self`

- <span id="reftype-can-represent-type-index"></span>`const fn can_represent_type_index(index: u32) -> bool`

- <span id="reftype-as-u32"></span>`const fn as_u32(&self) -> u32`

- <span id="reftype-from-u32"></span>`const fn from_u32(x: u32) -> Self`

- <span id="reftype-concrete"></span>`fn concrete(nullable: bool, index: PackedIndex) -> Self` — [`PackedIndex`](#packedindex)

  Create a reference to a concrete Wasm-defined type at the given

  index.

  

  Returns `None` when the type index is beyond this crate's

  implementation limits and therefore is not representable.

- <span id="reftype-exact"></span>`fn exact(nullable: bool, index: PackedIndex) -> Self` — [`PackedIndex`](#packedindex)

  Create a reference to exact type.

- <span id="reftype-new"></span>`fn new(nullable: bool, heap_type: HeapType) -> Option<Self>` — [`HeapType`](#heaptype)

  Create a new `RefType`.

  

  Returns `None` when the heap type's type index (if any) is beyond this

  crate's implementation limits and therefore is not representable.

- <span id="reftype-difference"></span>`fn difference(a: RefType, b: RefType) -> RefType` — [`RefType`](#reftype)

  Compute the [type difference] between the two given ref types.

- <span id="reftype-is-concrete-type-ref"></span>`const fn is_concrete_type_ref(&self) -> bool`

  Is this a reference to an concrete type?

- <span id="reftype-is-exact-type-ref"></span>`const fn is_exact_type_ref(&self) -> bool`

  Is this an exact reference to a type?

- <span id="reftype-type-index"></span>`fn type_index(&self) -> Option<PackedIndex>` — [`PackedIndex`](#packedindex)

  If this is a reference to a concrete Wasm-defined type, get its

  type index.

- <span id="reftype-abstype"></span>`const fn abstype(&self) -> u32`

- <span id="reftype-is-func-ref"></span>`const fn is_func_ref(&self) -> bool`

  Is this the abstract untyped function reference type aka `(ref

  null func)` aka `funcref` aka `anyfunc`?

- <span id="reftype-is-extern-ref"></span>`const fn is_extern_ref(&self) -> bool`

  Is this the abstract external reference type aka `(ref null

  extern)` aka `externref`?

- <span id="reftype-is-array-ref"></span>`const fn is_array_ref(&self) -> bool`

  Is this the abstract untyped array reference type aka `(ref null

  array)` aka `arrayref`?

- <span id="reftype-is-struct-ref"></span>`const fn is_struct_ref(&self) -> bool`

  Is this the abstract untyped struct reference type aka `(ref

  null struct)` aka `structref`?

- <span id="reftype-is-cont-ref"></span>`const fn is_cont_ref(&self) -> bool`

  Is this the abstract untyped cont reference type aka `(ref

  null cont)` aka `contref`?

- <span id="reftype-is-none-ref"></span>`const fn is_none_ref(&self) -> bool`

  Is this none type?

- <span id="reftype-is-nullable"></span>`const fn is_nullable(&self) -> bool`

  Is this ref type nullable?

- <span id="reftype-as-non-null"></span>`const fn as_non_null(&self) -> Self`

  Get the non-nullable version of this ref type.

- <span id="reftype-nullable"></span>`const fn nullable(&self) -> Self`

  Get the nullable version of this ref type.

- <span id="reftype-shared"></span>`const fn shared(&self) -> Option<Self>`

  Get the shared version of this ref type as long as it is abstract.

- <span id="reftype-heap-type"></span>`fn heap_type(&self) -> HeapType` — [`HeapType`](#heaptype)

  Get the heap type that this is a reference to.

#### Trait Implementations

##### `impl Clone for RefType`

- <span id="reftype-clone"></span>`fn clone(&self) -> RefType` — [`RefType`](#reftype)

##### `impl Copy for RefType`

##### `impl Debug for RefType`

- <span id="reftype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for RefType`

- <span id="reftype-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RefType`

##### `impl FromReader for RefType`

- <span id="reftype-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

##### `impl Hash for RefType`

- <span id="reftype-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for RefType`

- <span id="reftype-ord-cmp"></span>`fn cmp(&self, other: &RefType) -> cmp::Ordering` — [`RefType`](#reftype)

##### `impl PartialEq for RefType`

- <span id="reftype-partialeq-eq"></span>`fn eq(&self, other: &RefType) -> bool` — [`RefType`](#reftype)

##### `impl PartialOrd for RefType`

- <span id="reftype-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &RefType) -> option::Option<cmp::Ordering>` — [`RefType`](#reftype)

##### `impl StructuralPartialEq for RefType`

##### `impl ToString for RefType`

- <span id="reftype-tostring-to-string"></span>`fn to_string(&self) -> String` — [`String`](../../prelude/index.md#string)

### `TableType`

```rust
struct TableType {
    pub element_type: RefType,
    pub table64: bool,
    pub initial: u64,
    pub maximum: Option<u64>,
    pub shared: bool,
}
```

Represents a table's type.

#### Fields

- **`element_type`**: `RefType`

  The table's element type.

- **`table64`**: `bool`

  Whether or not this is a 64-bit table.
  
  This is part of the memory64 proposal in WebAssembly.

- **`initial`**: `u64`

  Initial size of this table, in elements.
  
  For 32-bit tables (when `table64` is `false`) this is guaranteed to
  be at most `u32::MAX` for valid types.

- **`maximum`**: `Option<u64>`

  Optional maximum size of the table, in elements.
  
  For 32-bit tables (when `table64` is `false`) this is guaranteed to
  be at most `u32::MAX` for valid types.

- **`shared`**: `bool`

  Whether this table is shared or not.
  
  This is included the shared-everything-threads proposal.

#### Implementations

- <span id="tabletype-index-type"></span>`fn index_type(&self) -> ValType` — [`ValType`](#valtype)

  Gets the index type for the table.

#### Trait Implementations

##### `impl Clone for TableType`

- <span id="tabletype-clone"></span>`fn clone(&self) -> TableType` — [`TableType`](#tabletype)

##### `impl Copy for TableType`

##### `impl Debug for TableType`

- <span id="tabletype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for TableType`

##### `impl FromReader for crate::TableType`

- <span id="cratetabletype-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

##### `impl Hash for TableType`

- <span id="tabletype-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for TableType`

- <span id="tabletype-partialeq-eq"></span>`fn eq(&self, other: &TableType) -> bool` — [`TableType`](#tabletype)

##### `impl StructuralPartialEq for TableType`

### `MemoryType`

```rust
struct MemoryType {
    pub memory64: bool,
    pub shared: bool,
    pub initial: u64,
    pub maximum: Option<u64>,
    pub page_size_log2: Option<u32>,
}
```

Represents a memory's type.

#### Fields

- **`memory64`**: `bool`

  Whether or not this is a 64-bit memory, using i64 as an index. If this
  is false it's a 32-bit memory using i32 as an index.
  
  This is part of the memory64 proposal in WebAssembly.

- **`shared`**: `bool`

  Whether or not this is a "shared" memory, indicating that it should be
  send-able across threads and the `maximum` field is always present for
  valid types.
  
  This is part of the threads proposal in WebAssembly.

- **`initial`**: `u64`

  Initial size of this memory, in wasm pages.
  
  For 32-bit memories (when `memory64` is `false`) this is guaranteed to
  be at most `u32::MAX` for valid types.

- **`maximum`**: `Option<u64>`

  Optional maximum size of this memory, in wasm pages.
  
  For 32-bit memories (when `memory64` is `false`) this is guaranteed to
  be at most `u32::MAX` for valid types. This field is always present for
  valid wasm memories when `shared` is `true`.

- **`page_size_log2`**: `Option<u32>`

  The log base 2 of the memory's custom page size.
  
  Memory pages are, by default, 64KiB large (i.e. 2<sup>16</sup> or
  `65536`).
  
  [The custom-page-sizes proposal] allows changing it to other values.
  

#### Implementations

- <span id="memorytype-index-type"></span>`fn index_type(&self) -> ValType` — [`ValType`](#valtype)

  Gets the index type for the memory.

#### Trait Implementations

##### `impl Clone for MemoryType`

- <span id="memorytype-clone"></span>`fn clone(&self) -> MemoryType` — [`MemoryType`](#memorytype)

##### `impl Copy for MemoryType`

##### `impl Debug for MemoryType`

- <span id="memorytype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for MemoryType`

##### `impl FromReader for crate::MemoryType`

- <span id="cratememorytype-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

##### `impl Hash for MemoryType`

- <span id="memorytype-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for MemoryType`

- <span id="memorytype-partialeq-eq"></span>`fn eq(&self, other: &MemoryType) -> bool` — [`MemoryType`](#memorytype)

##### `impl StructuralPartialEq for MemoryType`

### `GlobalType`

```rust
struct GlobalType {
    pub content_type: ValType,
    pub mutable: bool,
    pub shared: bool,
}
```

Represents a global's type.

#### Fields

- **`content_type`**: `ValType`

  The global's type.

- **`mutable`**: `bool`

  Whether or not the global is mutable.

- **`shared`**: `bool`

  Whether or not the global is shared.

#### Trait Implementations

##### `impl Clone for GlobalType`

- <span id="globaltype-clone"></span>`fn clone(&self) -> GlobalType` — [`GlobalType`](#globaltype)

##### `impl Copy for GlobalType`

##### `impl Debug for GlobalType`

- <span id="globaltype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for GlobalType`

##### `impl FromReader for crate::GlobalType`

- <span id="crateglobaltype-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

##### `impl Hash for GlobalType`

- <span id="globaltype-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for GlobalType`

- <span id="globaltype-partialeq-eq"></span>`fn eq(&self, other: &GlobalType) -> bool` — [`GlobalType`](#globaltype)

##### `impl StructuralPartialEq for GlobalType`

### `TagType`

```rust
struct TagType {
    pub kind: TagKind,
    pub func_type_idx: u32,
}
```

A tag's type.

#### Fields

- **`kind`**: `TagKind`

  The kind of tag

- **`func_type_idx`**: `u32`

  The function type this tag uses.

#### Trait Implementations

##### `impl Clone for TagType`

- <span id="tagtype-clone"></span>`fn clone(&self) -> TagType` — [`TagType`](#tagtype)

##### `impl Copy for TagType`

##### `impl Debug for TagType`

- <span id="tagtype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for TagType`

##### `impl FromReader for crate::TagType`

- <span id="cratetagtype-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

##### `impl PartialEq for TagType`

- <span id="tagtype-partialeq-eq"></span>`fn eq(&self, other: &TagType) -> bool` — [`TagType`](#tagtype)

##### `impl StructuralPartialEq for TagType`

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

- <span id="coredumpvalue-clone"></span>`fn clone(&self) -> CoreDumpValue` — [`CoreDumpValue`](#coredumpvalue)

##### `impl Debug for CoreDumpValue`

- <span id="coredumpvalue-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FromReader for CoreDumpValue`

- <span id="coredumpvalue-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

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

### `DataKind<'a>`

```rust
enum DataKind<'a> {
    Passive,
    Active {
        memory_index: u32,
        offset_expr: crate::ConstExpr<'a>,
    },
}
```

The kind of data segment.

#### Variants

- **`Passive`**

  The data segment is passive.

- **`Active`**

  The data segment is active.

#### Trait Implementations

##### `impl Clone for DataKind<'a>`

- <span id="datakind-clone"></span>`fn clone(&self) -> DataKind<'a>` — [`DataKind`](#datakind)

##### `impl Debug for DataKind<'a>`

- <span id="datakind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Dylink0Subsection<'a>`

```rust
enum Dylink0Subsection<'a> {
    MemInfo(MemInfo),
    Needed(Vec<&'a str>),
    ExportInfo(Vec<ExportInfo<'a>>),
    ImportInfo(Vec<ImportInfo<'a>>),
    RuntimePath(Vec<&'a str>),
    Unknown {
        ty: u8,
        data: &'a [u8],
        range: core::ops::Range<usize>,
    },
}
```

Possible subsections of the `dylink.0` custom section.

#### Trait Implementations

##### `impl Debug for Dylink0Subsection<'a>`

- <span id="dylink0subsection-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Subsection for Dylink0Subsection<'a>`

- <span id="dylink0subsection-subsection-from-reader"></span>`fn from_reader(id: u8, reader: BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

### `ElementKind<'a>`

```rust
enum ElementKind<'a> {
    Passive,
    Active {
        table_index: Option<u32>,
        offset_expr: crate::ConstExpr<'a>,
    },
    Declared,
}
```

The kind of element segment.

#### Variants

- **`Passive`**

  The element segment is passive.

- **`Active`**

  The element segment is active.

- **`Declared`**

  The element segment is declared.

#### Trait Implementations

##### `impl Clone for ElementKind<'a>`

- <span id="elementkind-clone"></span>`fn clone(&self) -> ElementKind<'a>` — [`ElementKind`](#elementkind)

### `ElementItems<'a>`

```rust
enum ElementItems<'a> {
    Functions(crate::SectionLimited<'a, u32>),
    Expressions(crate::RefType, crate::SectionLimited<'a, crate::ConstExpr<'a>>),
}
```

Represents the items of an element segment.

#### Variants

- **`Functions`**

  This element contains function indices.

- **`Expressions`**

  This element contains constant expressions used to initialize the table.

#### Trait Implementations

##### `impl Clone for ElementItems<'a>`

- <span id="elementitems-clone"></span>`fn clone(&self) -> ElementItems<'a>` — [`ElementItems`](#elementitems)

### `ExternalKind`

```rust
enum ExternalKind {
    Func,
    Table,
    Memory,
    Global,
    Tag,
    FuncExact,
}
```

External types as defined [here].


#### Variants

- **`Func`**

  The external kind is a function.

- **`Table`**

  The external kind if a table.

- **`Memory`**

  The external kind is a memory.

- **`Global`**

  The external kind is a global.

- **`Tag`**

  The external kind is a tag.

- **`FuncExact`**

  The external kind is a function with the exact type.

#### Trait Implementations

##### `impl Clone for ExternalKind`

- <span id="externalkind-clone"></span>`fn clone(&self) -> ExternalKind` — [`ExternalKind`](#externalkind)

##### `impl Copy for ExternalKind`

##### `impl Debug for ExternalKind`

- <span id="externalkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ExternalKind`

##### `impl FromReader for ExternalKind`

- <span id="externalkind-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

##### `impl PartialEq for ExternalKind`

- <span id="externalkind-partialeq-eq"></span>`fn eq(&self, other: &ExternalKind) -> bool` — [`ExternalKind`](#externalkind)

##### `impl StructuralPartialEq for ExternalKind`

### `TypeRef`

```rust
enum TypeRef {
    Func(u32),
    Table(crate::TableType),
    Memory(crate::MemoryType),
    Global(crate::GlobalType),
    Tag(crate::TagType),
    FuncExact(u32),
}
```

Represents a reference to a type definition in a WebAssembly module.

#### Variants

- **`Func`**

  The type is a function.

- **`Table`**

  The type is a table.

- **`Memory`**

  The type is a memory.

- **`Global`**

  The type is a global.

- **`Tag`**

  The type is a tag.
  
  This variant is only used for the exception handling proposal.
  
  The value is an index in the types index space.

- **`FuncExact`**

  The type is a function.

#### Trait Implementations

##### `impl Clone for TypeRef`

- <span id="typeref-clone"></span>`fn clone(&self) -> TypeRef` — [`TypeRef`](#typeref)

##### `impl Copy for TypeRef`

##### `impl Debug for TypeRef`

- <span id="typeref-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for TypeRef`

##### `impl FromReader for TypeRef`

- <span id="typeref-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

##### `impl PartialEq for TypeRef`

- <span id="typeref-partialeq-eq"></span>`fn eq(&self, other: &TypeRef) -> bool` — [`TypeRef`](#typeref)

##### `impl StructuralPartialEq for TypeRef`

### `ComdatSymbolKind`

```rust
enum ComdatSymbolKind {
    Data,
    Func,
    Global,
    Event,
    Table,
    Section,
}
```

Represents a symbol kind.

#### Variants

- **`Data`**

  The symbol is a data segment.

- **`Func`**

  The symbol is a function.

- **`Global`**

  The symbol is a global.

- **`Event`**

  The symbol is an event.

- **`Table`**

  The symbol is a table.

- **`Section`**

  The symbol is a section.

#### Trait Implementations

##### `impl Clone for ComdatSymbolKind`

- <span id="comdatsymbolkind-clone"></span>`fn clone(&self) -> ComdatSymbolKind` — [`ComdatSymbolKind`](#comdatsymbolkind)

##### `impl Copy for ComdatSymbolKind`

##### `impl Debug for ComdatSymbolKind`

- <span id="comdatsymbolkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ComdatSymbolKind`

##### `impl FromReader for ComdatSymbolKind`

- <span id="comdatsymbolkind-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

##### `impl Hash for ComdatSymbolKind`

- <span id="comdatsymbolkind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for ComdatSymbolKind`

- <span id="comdatsymbolkind-partialeq-eq"></span>`fn eq(&self, other: &ComdatSymbolKind) -> bool` — [`ComdatSymbolKind`](#comdatsymbolkind)

##### `impl StructuralPartialEq for ComdatSymbolKind`

### `SymbolInfo<'a>`

```rust
enum SymbolInfo<'a> {
    Func {
        flags: SymbolFlags,
        index: u32,
        name: Option<&'a str>,
    },
    Data {
        flags: SymbolFlags,
        name: &'a str,
        symbol: Option<DefinedDataSymbol>,
    },
    Global {
        flags: SymbolFlags,
        index: u32,
        name: Option<&'a str>,
    },
    Section {
        flags: SymbolFlags,
        section: u32,
    },
    Event {
        flags: SymbolFlags,
        index: u32,
        name: Option<&'a str>,
    },
    Table {
        flags: SymbolFlags,
        index: u32,
        name: Option<&'a str>,
    },
}
```

Represents extra information about symbols in the linking custom section.

The symbol flags correspond to those described in
<https://github.com/WebAssembly/tool-conventions/blob/main/Linking.md>
with the `WASM_SYM_*` prefix.

#### Variants

- **`Func`**

  The symbol is a function.

- **`Data`**

  The symbol is a data symbol.

- **`Global`**

  The symbol is a global.

- **`Section`**

  The symbol is a section.

- **`Event`**

  The symbol is an event.

- **`Table`**

  The symbol is a table.

#### Trait Implementations

##### `impl Clone for SymbolInfo<'a>`

- <span id="symbolinfo-clone"></span>`fn clone(&self) -> SymbolInfo<'a>` — [`SymbolInfo`](#symbolinfo)

##### `impl Copy for SymbolInfo<'a>`

##### `impl Debug for SymbolInfo<'a>`

- <span id="symbolinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FromReader for SymbolInfo<'a>`

- <span id="symbolinfo-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

### `Linking<'a>`

```rust
enum Linking<'a> {
    SegmentInfo(SegmentMap<'a>),
    InitFuncs(InitFuncMap<'a>),
    ComdatInfo(ComdatMap<'a>),
    SymbolTable(SymbolInfoMap<'a>),
    Unknown {
        ty: u8,
        data: &'a [u8],
        range: core::ops::Range<usize>,
    },
}
```

Represents a subsection read from the linking custom section.

#### Variants

- **`SegmentInfo`**

  Extra metadata about the data segments.

- **`InitFuncs`**

  A list of constructor functions to be called at startup.

- **`ComdatInfo`**

  The [COMDAT](https://llvm.org/docs/LangRef.html#comdats) groups of associated linking objects.

- **`SymbolTable`**

  Extra information about the symbols present in the module.

- **`Unknown`**

  An unknown [linking subsection](https://github.com/WebAssembly/tool-conventions/blob/main/Linking.md#linking-metadata-section).

#### Trait Implementations

##### `impl Clone for Linking<'a>`

- <span id="linking-clone"></span>`fn clone(&self) -> Linking<'a>` — [`Linking`](#linking)

##### `impl Debug for Linking<'a>`

- <span id="linking-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Subsection for Linking<'a>`

- <span id="linking-subsection-from-reader"></span>`fn from_reader(id: u8, reader: BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

### `Name<'a>`

```rust
enum Name<'a> {
    Module {
        name: &'a str,
        name_range: core::ops::Range<usize>,
    },
    Function(NameMap<'a>),
    Local(IndirectNameMap<'a>),
    Label(IndirectNameMap<'a>),
    Type(NameMap<'a>),
    Table(NameMap<'a>),
    Memory(NameMap<'a>),
    Global(NameMap<'a>),
    Element(NameMap<'a>),
    Data(NameMap<'a>),
    Field(IndirectNameMap<'a>),
    Tag(NameMap<'a>),
    Unknown {
        ty: u8,
        data: &'a [u8],
        range: core::ops::Range<usize>,
    },
}
```

Represents a name read from the names custom section.

#### Variants

- **`Module`**

  The name is for the module.

- **`Function`**

  The name is for the functions.

- **`Local`**

  The name is for the function locals.

- **`Label`**

  The name is for the function labels.

- **`Type`**

  The name is for the types.

- **`Table`**

  The name is for the tables.

- **`Memory`**

  The name is for the memories.

- **`Global`**

  The name is for the globals.

- **`Element`**

  The name is for the element segments.

- **`Data`**

  The name is for the data segments.

- **`Field`**

  The name is for fields.

- **`Tag`**

  The name is for tags.

- **`Unknown`**

  An unknown [name subsection](https://webassembly.github.io/spec/core/appendix/custom.html#subsections).

#### Trait Implementations

##### `impl Clone for Name<'a>`

- <span id="name-clone"></span>`fn clone(&self) -> Name<'a>` — [`Name`](#name)

##### `impl Subsection for Name<'a>`

- <span id="name-subsection-from-reader"></span>`fn from_reader(id: u8, reader: BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

### `BlockType`

```rust
enum BlockType {
    Empty,
    Type(crate::ValType),
    FuncType(u32),
}
```

Represents a block type.

#### Variants

- **`Empty`**

  The block produces consumes nor produces any values.

- **`Type`**

  The block produces a singular value of the given type ([] -> \[t]).

- **`FuncType`**

  The block is described by a function type.
  
  The index is to a function type in the types section.

#### Trait Implementations

##### `impl Clone for BlockType`

- <span id="blocktype-clone"></span>`fn clone(&self) -> BlockType` — [`BlockType`](#blocktype)

##### `impl Copy for BlockType`

##### `impl Debug for BlockType`

- <span id="blocktype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for BlockType`

##### `impl PartialEq for BlockType`

- <span id="blocktype-partialeq-eq"></span>`fn eq(&self, other: &BlockType) -> bool` — [`BlockType`](#blocktype)

##### `impl StructuralPartialEq for BlockType`

### `FrameKind`

```rust
enum FrameKind {
    Block,
    If,
    Else,
    Loop,
    TryTable,
    LegacyTry,
    LegacyCatch,
    LegacyCatchAll,
}
```

The kind of a control flow `Frame`.

#### Variants

- **`Block`**

  A Wasm `block` control block.

- **`If`**

  A Wasm `if` control block.

- **`Else`**

  A Wasm `else` control block.

- **`Loop`**

  A Wasm `loop` control block.

- **`TryTable`**

  A Wasm `try` control block.
  
  # Note
  
  This belongs to the Wasm exception handling proposal.

- **`LegacyTry`**

  A Wasm legacy `try` control block.
  
  # Note
  
  See: `WasmFeatures::legacy_exceptions` Note in `crates/wasmparser/src/features.rs`

- **`LegacyCatch`**

  A Wasm legacy `catch` control block.
  
  # Note
  
  See: `WasmFeatures::legacy_exceptions` Note in `crates/wasmparser/src/features.rs`

- **`LegacyCatchAll`**

  A Wasm legacy `catch_all` control block.
  
  # Note
  
  See: `WasmFeatures::legacy_exceptions` Note in `crates/wasmparser/src/features.rs`

#### Trait Implementations

##### `impl Clone for FrameKind`

- <span id="framekind-clone"></span>`fn clone(&self) -> FrameKind` — [`FrameKind`](#framekind)

##### `impl Copy for FrameKind`

##### `impl Debug for FrameKind`

- <span id="framekind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for FrameKind`

##### `impl PartialEq for FrameKind`

- <span id="framekind-partialeq-eq"></span>`fn eq(&self, other: &FrameKind) -> bool` — [`FrameKind`](#framekind)

##### `impl StructuralPartialEq for FrameKind`

### `Ordering`

```rust
enum Ordering {
    AcqRel,
    SeqCst,
}
```

Represents the memory ordering for atomic instructions.

For an in-depth explanation of memory orderings, see the C++ documentation
for `memory_order` or the Rust documentation for `atomic::Ordering`.



#### Variants

- **`AcqRel`**

  For a load, it acquires; this orders all operations before the last
  "releasing" store. For a store, it releases; this orders all operations
  before it at the next "acquiring" load.

- **`SeqCst`**

  Like `AcqRel` but all threads see all sequentially consistent operations
  in the same order.

#### Trait Implementations

##### `impl Clone for Ordering`

- <span id="ordering-clone"></span>`fn clone(&self) -> Ordering` — [`Ordering`](#ordering)

##### `impl Copy for Ordering`

##### `impl Debug for Ordering`

- <span id="ordering-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Ordering`

##### `impl Hash for Ordering`

- <span id="ordering-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Ordering`

- <span id="ordering-partialeq-eq"></span>`fn eq(&self, other: &Ordering) -> bool` — [`Ordering`](#ordering)

##### `impl StructuralPartialEq for Ordering`

### `Operator<'a>`

```rust
enum Operator<'a> {
    Unreachable,
    Nop,
    Block {
        blockty: BlockType,
    },
    Loop {
        blockty: BlockType,
    },
    If {
        blockty: BlockType,
    },
    Else,
    End,
    Br {
        relative_depth: u32,
    },
    BrIf {
        relative_depth: u32,
    },
    BrTable {
        targets: BrTable<'a>,
    },
    Return,
    Call {
        function_index: u32,
    },
    CallIndirect {
        type_index: u32,
        table_index: u32,
    },
    Drop,
    Select,
    LocalGet {
        local_index: u32,
    },
    LocalSet {
        local_index: u32,
    },
    LocalTee {
        local_index: u32,
    },
    GlobalGet {
        global_index: u32,
    },
    GlobalSet {
        global_index: u32,
    },
    I32Load {
        memarg: MemArg,
    },
    I64Load {
        memarg: MemArg,
    },
    F32Load {
        memarg: MemArg,
    },
    F64Load {
        memarg: MemArg,
    },
    I32Load8S {
        memarg: MemArg,
    },
    I32Load8U {
        memarg: MemArg,
    },
    I32Load16S {
        memarg: MemArg,
    },
    I32Load16U {
        memarg: MemArg,
    },
    I64Load8S {
        memarg: MemArg,
    },
    I64Load8U {
        memarg: MemArg,
    },
    I64Load16S {
        memarg: MemArg,
    },
    I64Load16U {
        memarg: MemArg,
    },
    I64Load32S {
        memarg: MemArg,
    },
    I64Load32U {
        memarg: MemArg,
    },
    I32Store {
        memarg: MemArg,
    },
    I64Store {
        memarg: MemArg,
    },
    F32Store {
        memarg: MemArg,
    },
    F64Store {
        memarg: MemArg,
    },
    I32Store8 {
        memarg: MemArg,
    },
    I32Store16 {
        memarg: MemArg,
    },
    I64Store8 {
        memarg: MemArg,
    },
    I64Store16 {
        memarg: MemArg,
    },
    I64Store32 {
        memarg: MemArg,
    },
    MemorySize {
        mem: u32,
    },
    MemoryGrow {
        mem: u32,
    },
    I32Const {
        value: i32,
    },
    I64Const {
        value: i64,
    },
    F32Const {
        value: Ieee32,
    },
    F64Const {
        value: Ieee64,
    },
    I32Eqz,
    I32Eq,
    I32Ne,
    I32LtS,
    I32LtU,
    I32GtS,
    I32GtU,
    I32LeS,
    I32LeU,
    I32GeS,
    I32GeU,
    I64Eqz,
    I64Eq,
    I64Ne,
    I64LtS,
    I64LtU,
    I64GtS,
    I64GtU,
    I64LeS,
    I64LeU,
    I64GeS,
    I64GeU,
    F32Eq,
    F32Ne,
    F32Lt,
    F32Gt,
    F32Le,
    F32Ge,
    F64Eq,
    F64Ne,
    F64Lt,
    F64Gt,
    F64Le,
    F64Ge,
    I32Clz,
    I32Ctz,
    I32Popcnt,
    I32Add,
    I32Sub,
    I32Mul,
    I32DivS,
    I32DivU,
    I32RemS,
    I32RemU,
    I32And,
    I32Or,
    I32Xor,
    I32Shl,
    I32ShrS,
    I32ShrU,
    I32Rotl,
    I32Rotr,
    I64Clz,
    I64Ctz,
    I64Popcnt,
    I64Add,
    I64Sub,
    I64Mul,
    I64DivS,
    I64DivU,
    I64RemS,
    I64RemU,
    I64And,
    I64Or,
    I64Xor,
    I64Shl,
    I64ShrS,
    I64ShrU,
    I64Rotl,
    I64Rotr,
    F32Abs,
    F32Neg,
    F32Ceil,
    F32Floor,
    F32Trunc,
    F32Nearest,
    F32Sqrt,
    F32Add,
    F32Sub,
    F32Mul,
    F32Div,
    F32Min,
    F32Max,
    F32Copysign,
    F64Abs,
    F64Neg,
    F64Ceil,
    F64Floor,
    F64Trunc,
    F64Nearest,
    F64Sqrt,
    F64Add,
    F64Sub,
    F64Mul,
    F64Div,
    F64Min,
    F64Max,
    F64Copysign,
    I32WrapI64,
    I32TruncF32S,
    I32TruncF32U,
    I32TruncF64S,
    I32TruncF64U,
    I64ExtendI32S,
    I64ExtendI32U,
    I64TruncF32S,
    I64TruncF32U,
    I64TruncF64S,
    I64TruncF64U,
    F32ConvertI32S,
    F32ConvertI32U,
    F32ConvertI64S,
    F32ConvertI64U,
    F32DemoteF64,
    F64ConvertI32S,
    F64ConvertI32U,
    F64ConvertI64S,
    F64ConvertI64U,
    F64PromoteF32,
    I32ReinterpretF32,
    I64ReinterpretF64,
    F32ReinterpretI32,
    F64ReinterpretI64,
    I32Extend8S,
    I32Extend16S,
    I64Extend8S,
    I64Extend16S,
    I64Extend32S,
    RefEq,
    StructNew {
        struct_type_index: u32,
    },
    StructNewDefault {
        struct_type_index: u32,
    },
    StructGet {
        struct_type_index: u32,
        field_index: u32,
    },
    StructGetS {
        struct_type_index: u32,
        field_index: u32,
    },
    StructGetU {
        struct_type_index: u32,
        field_index: u32,
    },
    StructSet {
        struct_type_index: u32,
        field_index: u32,
    },
    ArrayNew {
        array_type_index: u32,
    },
    ArrayNewDefault {
        array_type_index: u32,
    },
    ArrayNewFixed {
        array_type_index: u32,
        array_size: u32,
    },
    ArrayNewData {
        array_type_index: u32,
        array_data_index: u32,
    },
    ArrayNewElem {
        array_type_index: u32,
        array_elem_index: u32,
    },
    ArrayGet {
        array_type_index: u32,
    },
    ArrayGetS {
        array_type_index: u32,
    },
    ArrayGetU {
        array_type_index: u32,
    },
    ArraySet {
        array_type_index: u32,
    },
    ArrayLen,
    ArrayFill {
        array_type_index: u32,
    },
    ArrayCopy {
        array_type_index_dst: u32,
        array_type_index_src: u32,
    },
    ArrayInitData {
        array_type_index: u32,
        array_data_index: u32,
    },
    ArrayInitElem {
        array_type_index: u32,
        array_elem_index: u32,
    },
    RefTestNonNull {
        hty: HeapType,
    },
    RefTestNullable {
        hty: HeapType,
    },
    RefCastNonNull {
        hty: HeapType,
    },
    RefCastNullable {
        hty: HeapType,
    },
    BrOnCast {
        relative_depth: u32,
        from_ref_type: RefType,
        to_ref_type: RefType,
    },
    BrOnCastFail {
        relative_depth: u32,
        from_ref_type: RefType,
        to_ref_type: RefType,
    },
    AnyConvertExtern,
    ExternConvertAny,
    RefI31,
    I31GetS,
    I31GetU,
    StructNewDesc {
        struct_type_index: u32,
    },
    StructNewDefaultDesc {
        struct_type_index: u32,
    },
    RefGetDesc {
        type_index: u32,
    },
    RefCastDescNonNull {
        hty: HeapType,
    },
    RefCastDescNullable {
        hty: HeapType,
    },
    BrOnCastDesc {
        relative_depth: u32,
        from_ref_type: RefType,
        to_ref_type: RefType,
    },
    BrOnCastDescFail {
        relative_depth: u32,
        from_ref_type: RefType,
        to_ref_type: RefType,
    },
    I32TruncSatF32S,
    I32TruncSatF32U,
    I32TruncSatF64S,
    I32TruncSatF64U,
    I64TruncSatF32S,
    I64TruncSatF32U,
    I64TruncSatF64S,
    I64TruncSatF64U,
    MemoryInit {
        data_index: u32,
        mem: u32,
    },
    DataDrop {
        data_index: u32,
    },
    MemoryCopy {
        dst_mem: u32,
        src_mem: u32,
    },
    MemoryFill {
        mem: u32,
    },
    TableInit {
        elem_index: u32,
        table: u32,
    },
    ElemDrop {
        elem_index: u32,
    },
    TableCopy {
        dst_table: u32,
        src_table: u32,
    },
    TypedSelect {
        ty: ValType,
    },
    TypedSelectMulti {
        tys: Vec<ValType>,
    },
    RefNull {
        hty: HeapType,
    },
    RefIsNull,
    RefFunc {
        function_index: u32,
    },
    TableFill {
        table: u32,
    },
    TableGet {
        table: u32,
    },
    TableSet {
        table: u32,
    },
    TableGrow {
        table: u32,
    },
    TableSize {
        table: u32,
    },
    ReturnCall {
        function_index: u32,
    },
    ReturnCallIndirect {
        type_index: u32,
        table_index: u32,
    },
    MemoryDiscard {
        mem: u32,
    },
    MemoryAtomicNotify {
        memarg: MemArg,
    },
    MemoryAtomicWait32 {
        memarg: MemArg,
    },
    MemoryAtomicWait64 {
        memarg: MemArg,
    },
    AtomicFence,
    I32AtomicLoad {
        memarg: MemArg,
    },
    I64AtomicLoad {
        memarg: MemArg,
    },
    I32AtomicLoad8U {
        memarg: MemArg,
    },
    I32AtomicLoad16U {
        memarg: MemArg,
    },
    I64AtomicLoad8U {
        memarg: MemArg,
    },
    I64AtomicLoad16U {
        memarg: MemArg,
    },
    I64AtomicLoad32U {
        memarg: MemArg,
    },
    I32AtomicStore {
        memarg: MemArg,
    },
    I64AtomicStore {
        memarg: MemArg,
    },
    I32AtomicStore8 {
        memarg: MemArg,
    },
    I32AtomicStore16 {
        memarg: MemArg,
    },
    I64AtomicStore8 {
        memarg: MemArg,
    },
    I64AtomicStore16 {
        memarg: MemArg,
    },
    I64AtomicStore32 {
        memarg: MemArg,
    },
    I32AtomicRmwAdd {
        memarg: MemArg,
    },
    I64AtomicRmwAdd {
        memarg: MemArg,
    },
    I32AtomicRmw8AddU {
        memarg: MemArg,
    },
    I32AtomicRmw16AddU {
        memarg: MemArg,
    },
    I64AtomicRmw8AddU {
        memarg: MemArg,
    },
    I64AtomicRmw16AddU {
        memarg: MemArg,
    },
    I64AtomicRmw32AddU {
        memarg: MemArg,
    },
    I32AtomicRmwSub {
        memarg: MemArg,
    },
    I64AtomicRmwSub {
        memarg: MemArg,
    },
    I32AtomicRmw8SubU {
        memarg: MemArg,
    },
    I32AtomicRmw16SubU {
        memarg: MemArg,
    },
    I64AtomicRmw8SubU {
        memarg: MemArg,
    },
    I64AtomicRmw16SubU {
        memarg: MemArg,
    },
    I64AtomicRmw32SubU {
        memarg: MemArg,
    },
    I32AtomicRmwAnd {
        memarg: MemArg,
    },
    I64AtomicRmwAnd {
        memarg: MemArg,
    },
    I32AtomicRmw8AndU {
        memarg: MemArg,
    },
    I32AtomicRmw16AndU {
        memarg: MemArg,
    },
    I64AtomicRmw8AndU {
        memarg: MemArg,
    },
    I64AtomicRmw16AndU {
        memarg: MemArg,
    },
    I64AtomicRmw32AndU {
        memarg: MemArg,
    },
    I32AtomicRmwOr {
        memarg: MemArg,
    },
    I64AtomicRmwOr {
        memarg: MemArg,
    },
    I32AtomicRmw8OrU {
        memarg: MemArg,
    },
    I32AtomicRmw16OrU {
        memarg: MemArg,
    },
    I64AtomicRmw8OrU {
        memarg: MemArg,
    },
    I64AtomicRmw16OrU {
        memarg: MemArg,
    },
    I64AtomicRmw32OrU {
        memarg: MemArg,
    },
    I32AtomicRmwXor {
        memarg: MemArg,
    },
    I64AtomicRmwXor {
        memarg: MemArg,
    },
    I32AtomicRmw8XorU {
        memarg: MemArg,
    },
    I32AtomicRmw16XorU {
        memarg: MemArg,
    },
    I64AtomicRmw8XorU {
        memarg: MemArg,
    },
    I64AtomicRmw16XorU {
        memarg: MemArg,
    },
    I64AtomicRmw32XorU {
        memarg: MemArg,
    },
    I32AtomicRmwXchg {
        memarg: MemArg,
    },
    I64AtomicRmwXchg {
        memarg: MemArg,
    },
    I32AtomicRmw8XchgU {
        memarg: MemArg,
    },
    I32AtomicRmw16XchgU {
        memarg: MemArg,
    },
    I64AtomicRmw8XchgU {
        memarg: MemArg,
    },
    I64AtomicRmw16XchgU {
        memarg: MemArg,
    },
    I64AtomicRmw32XchgU {
        memarg: MemArg,
    },
    I32AtomicRmwCmpxchg {
        memarg: MemArg,
    },
    I64AtomicRmwCmpxchg {
        memarg: MemArg,
    },
    I32AtomicRmw8CmpxchgU {
        memarg: MemArg,
    },
    I32AtomicRmw16CmpxchgU {
        memarg: MemArg,
    },
    I64AtomicRmw8CmpxchgU {
        memarg: MemArg,
    },
    I64AtomicRmw16CmpxchgU {
        memarg: MemArg,
    },
    I64AtomicRmw32CmpxchgU {
        memarg: MemArg,
    },
    TryTable {
        try_table: TryTable,
    },
    Throw {
        tag_index: u32,
    },
    ThrowRef,
    Try {
        blockty: BlockType,
    },
    Catch {
        tag_index: u32,
    },
    Rethrow {
        relative_depth: u32,
    },
    Delegate {
        relative_depth: u32,
    },
    CatchAll,
    GlobalAtomicGet {
        ordering: Ordering,
        global_index: u32,
    },
    GlobalAtomicSet {
        ordering: Ordering,
        global_index: u32,
    },
    GlobalAtomicRmwAdd {
        ordering: Ordering,
        global_index: u32,
    },
    GlobalAtomicRmwSub {
        ordering: Ordering,
        global_index: u32,
    },
    GlobalAtomicRmwAnd {
        ordering: Ordering,
        global_index: u32,
    },
    GlobalAtomicRmwOr {
        ordering: Ordering,
        global_index: u32,
    },
    GlobalAtomicRmwXor {
        ordering: Ordering,
        global_index: u32,
    },
    GlobalAtomicRmwXchg {
        ordering: Ordering,
        global_index: u32,
    },
    GlobalAtomicRmwCmpxchg {
        ordering: Ordering,
        global_index: u32,
    },
    TableAtomicGet {
        ordering: Ordering,
        table_index: u32,
    },
    TableAtomicSet {
        ordering: Ordering,
        table_index: u32,
    },
    TableAtomicRmwXchg {
        ordering: Ordering,
        table_index: u32,
    },
    TableAtomicRmwCmpxchg {
        ordering: Ordering,
        table_index: u32,
    },
    StructAtomicGet {
        ordering: Ordering,
        struct_type_index: u32,
        field_index: u32,
    },
    StructAtomicGetS {
        ordering: Ordering,
        struct_type_index: u32,
        field_index: u32,
    },
    StructAtomicGetU {
        ordering: Ordering,
        struct_type_index: u32,
        field_index: u32,
    },
    StructAtomicSet {
        ordering: Ordering,
        struct_type_index: u32,
        field_index: u32,
    },
    StructAtomicRmwAdd {
        ordering: Ordering,
        struct_type_index: u32,
        field_index: u32,
    },
    StructAtomicRmwSub {
        ordering: Ordering,
        struct_type_index: u32,
        field_index: u32,
    },
    StructAtomicRmwAnd {
        ordering: Ordering,
        struct_type_index: u32,
        field_index: u32,
    },
    StructAtomicRmwOr {
        ordering: Ordering,
        struct_type_index: u32,
        field_index: u32,
    },
    StructAtomicRmwXor {
        ordering: Ordering,
        struct_type_index: u32,
        field_index: u32,
    },
    StructAtomicRmwXchg {
        ordering: Ordering,
        struct_type_index: u32,
        field_index: u32,
    },
    StructAtomicRmwCmpxchg {
        ordering: Ordering,
        struct_type_index: u32,
        field_index: u32,
    },
    ArrayAtomicGet {
        ordering: Ordering,
        array_type_index: u32,
    },
    ArrayAtomicGetS {
        ordering: Ordering,
        array_type_index: u32,
    },
    ArrayAtomicGetU {
        ordering: Ordering,
        array_type_index: u32,
    },
    ArrayAtomicSet {
        ordering: Ordering,
        array_type_index: u32,
    },
    ArrayAtomicRmwAdd {
        ordering: Ordering,
        array_type_index: u32,
    },
    ArrayAtomicRmwSub {
        ordering: Ordering,
        array_type_index: u32,
    },
    ArrayAtomicRmwAnd {
        ordering: Ordering,
        array_type_index: u32,
    },
    ArrayAtomicRmwOr {
        ordering: Ordering,
        array_type_index: u32,
    },
    ArrayAtomicRmwXor {
        ordering: Ordering,
        array_type_index: u32,
    },
    ArrayAtomicRmwXchg {
        ordering: Ordering,
        array_type_index: u32,
    },
    ArrayAtomicRmwCmpxchg {
        ordering: Ordering,
        array_type_index: u32,
    },
    RefI31Shared,
    CallRef {
        type_index: u32,
    },
    ReturnCallRef {
        type_index: u32,
    },
    RefAsNonNull,
    BrOnNull {
        relative_depth: u32,
    },
    BrOnNonNull {
        relative_depth: u32,
    },
    ContNew {
        cont_type_index: u32,
    },
    ContBind {
        argument_index: u32,
        result_index: u32,
    },
    Suspend {
        tag_index: u32,
    },
    Resume {
        cont_type_index: u32,
        resume_table: ResumeTable,
    },
    ResumeThrow {
        cont_type_index: u32,
        tag_index: u32,
        resume_table: ResumeTable,
    },
    Switch {
        cont_type_index: u32,
        tag_index: u32,
    },
    I64Add128,
    I64Sub128,
    I64MulWideS,
    I64MulWideU,
}
```

Instructions as defined [here].


#### Implementations

- <span id="crateoperator-operator-arity"></span>`fn operator_arity(&self, module: &impl ModuleArity) -> Option<(u32, u32)>` — [`ModuleArity`](../../index.md#modulearity)

  Compute the arity (param and result counts) of the operator, given

  an impl ModuleArity, which stores the necessary module state.

#### Trait Implementations

##### `impl Clone for Operator<'a>`

- <span id="operator-clone"></span>`fn clone(&self) -> Operator<'a>` — [`Operator`](#operator)

##### `impl Debug for Operator<'a>`

- <span id="operator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Operator<'a>`

##### `impl PartialEq for Operator<'a>`

- <span id="operator-partialeq-eq"></span>`fn eq(&self, other: &Operator<'a>) -> bool` — [`Operator`](#operator)

##### `impl StructuralPartialEq for Operator<'a>`

### `Catch`

```rust
enum Catch {
    One {
        tag: u32,
        label: u32,
    },
    OneRef {
        tag: u32,
        label: u32,
    },
    All {
        label: u32,
    },
    AllRef {
        label: u32,
    },
}
```

Catch clauses that can be specified in [`TryTable`](#trytable).

#### Variants

- **`One`**

  Equivalent of `catch`

- **`OneRef`**

  Equivalent of `catch_ref`

- **`All`**

  Equivalent of `catch_all`

- **`AllRef`**

  Equivalent of `catch_all_ref`

#### Trait Implementations

##### `impl Clone for Catch`

- <span id="catch-clone"></span>`fn clone(&self) -> Catch` — [`Catch`](#catch)

##### `impl Copy for Catch`

##### `impl Debug for Catch`

- <span id="catch-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Catch`

##### `impl FromReader for Catch`

- <span id="catch-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

##### `impl PartialEq for Catch`

- <span id="catch-partialeq-eq"></span>`fn eq(&self, other: &Catch) -> bool` — [`Catch`](#catch)

##### `impl StructuralPartialEq for Catch`

### `Handle`

```rust
enum Handle {
    OnLabel {
        tag: u32,
        label: u32,
    },
    OnSwitch {
        tag: u32,
    },
}
```

Handle clauses that can be specified in [`ResumeTable`](#resumetable).

#### Variants

- **`OnLabel`**

  Equivalent of `(on $tag $lbl)`.

- **`OnSwitch`**

  Equivalent of `(on $tag switch)`.

#### Trait Implementations

##### `impl Clone for Handle`

- <span id="handle-clone"></span>`fn clone(&self) -> Handle` — [`Handle`](#handle)

##### `impl Copy for Handle`

##### `impl Debug for Handle`

- <span id="handle-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Handle`

##### `impl FromReader for Handle`

- <span id="handle-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

##### `impl PartialEq for Handle`

- <span id="handle-partialeq-eq"></span>`fn eq(&self, other: &Handle) -> bool` — [`Handle`](#handle)

##### `impl StructuralPartialEq for Handle`

### `RelocationType`

```rust
enum RelocationType {
    FunctionIndexLeb,
    TableIndexSleb,
    TableIndexI32,
    MemoryAddrLeb,
    MemoryAddrSleb,
    MemoryAddrI32,
    TypeIndexLeb,
    GlobalIndexLeb,
    FunctionOffsetI32,
    SectionOffsetI32,
    EventIndexLeb,
    MemoryAddrRelSleb,
    TableIndexRelSleb,
    GlobalIndexI32,
    MemoryAddrLeb64,
    MemoryAddrSleb64,
    MemoryAddrI64,
    MemoryAddrRelSleb64,
    TableIndexSleb64,
    TableIndexI64,
    TableNumberLeb,
    MemoryAddrTlsSleb,
    FunctionOffsetI64,
    MemoryAddrLocrelI32,
    TableIndexRelSleb64,
    MemoryAddrTlsSleb64,
    FunctionIndexI32,
}
```

Relocation entry type. Each entry type corresponds to one of the
`R_WASM_*` constants defined at
<https://github.com/llvm/llvm-project/blob/main/llvm/include/llvm/BinaryFormat/WasmRelocs.def>
and
<https://github.com/WebAssembly/tool-conventions/blob/main/Linking.md#relocation-sections>.

#### Variants

- **`FunctionIndexLeb`**

  A function index encoded as a 5-byte varuint32. Used for the
  immediate argument of a call instruction. (since LLVM 10.0)

- **`TableIndexSleb`**

  A function table index encoded as a 5-byte varint32. Used to refer
  to the immediate argument of a i32.const instruction, e.g. taking
  the address of a function. (since LLVM 10.0)

- **`TableIndexI32`**

  A function table index encoded as a uint32, e.g. taking the address
  of a function in a static data initializer. (since LLVM 10.0)

- **`MemoryAddrLeb`**

  A linear memory index encoded as a 5-byte varuint32. Used for the
  immediate argument of a load or store instruction, e.g. directly
  loading from or storing to a C++ global. (since LLVM 10.0)

- **`MemoryAddrSleb`**

  A linear memory index encoded as a 5-byte varint32. Used for the
  immediate argument of a i32.const instruction, e.g. taking the
  address of a C++ global. (since LLVM 10.0)

- **`MemoryAddrI32`**

  A linear memory index encoded as a uint32, e.g. taking the address
  of a C++ global in a static data initializer. (since LLVM 10.0)

- **`TypeIndexLeb`**

  A type index encoded as a 5-byte varuint32, e.g. the type immediate
  in a call_indirect. (since LLVM 10.0)

- **`GlobalIndexLeb`**

  A global index encoded as a 5-byte varuint32, e.g. the index
  immediate in a get_global. (since LLVM 10.0)

- **`FunctionOffsetI32`**

  A byte offset within code section for the specific function encoded
  as a uint32. The offsets start at the actual function code excluding
  its size field. (since LLVM 10.0)

- **`SectionOffsetI32`**

  A byte offset from start of the specified section encoded as a
  uint32. (since LLVM 10.0)

- **`EventIndexLeb`**

  An event index encoded as a 5-byte varuint32. Used for the immediate
  argument of a throw and if_except instruction. (since LLVM 10.0)

- **`MemoryAddrRelSleb`**

  A memory address relative to the __memory_base wasm global. Used in
  position independent code (-fPIC) where absolute memory addresses
  are not known at link time.

- **`TableIndexRelSleb`**

  A function address (table index) relative to the __table_base wasm
  global. Used in position independent code (-fPIC) where absolute
  function addresses are not known at link time.

- **`GlobalIndexI32`**

  A global index encoded as uint32. (since LLVM 11.0)

- **`MemoryAddrLeb64`**

  The 64-bit counterpart of `MemoryAddrLeb`. A 64-bit linear memory
  index encoded as a 10-byte varuint64, Used for the immediate
  argument of a load or store instruction on a 64-bit linear memory
  array. (since LLVM 11.0)

- **`MemoryAddrSleb64`**

  The 64-bit counterpart of `MemoryAddrSleb`. A 64-bit linear memory
  index encoded as a 10-byte varint64. Used for the immediate argument
  of a i64.const instruction. (since LLVM 11.0)

- **`MemoryAddrI64`**

  The 64-bit counterpart of `MemoryAddrI32`. A 64-bit linear memory
  index encoded as a uint64, e.g. taking the 64-bit address of a C++
  global in a static data initializer. (since LLVM 11.0)

- **`MemoryAddrRelSleb64`**

  The 64-bit counterpart of `MemoryAddrRelSleb`.

- **`TableIndexSleb64`**

  The 64-bit counterpart of `TableIndexSleb`. A function table index
  encoded as a 10-byte varint64. Used to refer to the immediate
  argument of a i64.const instruction, e.g. taking the address of a
  function in Wasm64. (in LLVM 12.0)

- **`TableIndexI64`**

  The 64-bit counterpart of `TableIndexI32`. A function table index
  encoded as a uint64, e.g. taking the address of a function in a
  static data initializer. (in LLVM 12.0)

- **`TableNumberLeb`**

  A table number encoded as a 5-byte varuint32. Used for the table
  immediate argument in the table.* instructions. (in LLVM 12.0)

- **`MemoryAddrTlsSleb`**

  An offset from the __tls_base symbol encoded as a 5-byte varint32.
  Used for PIC case to avoid absolute relocation. (in LLVM 12.0)

- **`FunctionOffsetI64`**

  The 64-bit counterpart of `FunctionOffsetI32`. A byte offset within
  code section for the specific function encoded as a uint64. (in LLVM
  12.0)

- **`MemoryAddrLocrelI32`**

  A byte offset between the relocating address and a linear memory
  index encoded as a uint32. Used for pointer-relative addressing. (in
  LLVM 13.0)

- **`TableIndexRelSleb64`**

  The 64-bit counterpart of `TableIndexRelSleb`. A function table
  index encoded as a 10-byte varint64. (in LLVM 13.0)

- **`MemoryAddrTlsSleb64`**

  The 64-bit counterpart of `MemoryAddrTlsSleb`. (in LLVM 13.0)

- **`FunctionIndexI32`**

  A function index encoded as a uint32. Used in custom sections for
  function annotations (`__attribute__((annotate(<name>)))`) (in LLVM
  17.0)

#### Implementations

- <span id="relocationtype-addend-kind"></span>`const fn addend_kind(self) -> RelocAddendKind` — [`RelocAddendKind`](#relocaddendkind)

  Indicates if this relocation type has an associated `RelocEntry::addend`.

- <span id="relocationtype-extent"></span>`const fn extent(self) -> usize`

  Indicates the number of bytes that this relocation type targets.

#### Trait Implementations

##### `impl Clone for RelocationType`

- <span id="relocationtype-clone"></span>`fn clone(&self) -> RelocationType` — [`RelocationType`](#relocationtype)

##### `impl Copy for RelocationType`

##### `impl Debug for RelocationType`

- <span id="relocationtype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RelocationType`

##### `impl FromReader for RelocationType`

- <span id="relocationtype-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

##### `impl Hash for RelocationType`

- <span id="relocationtype-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for RelocationType`

- <span id="relocationtype-partialeq-eq"></span>`fn eq(&self, other: &RelocationType) -> bool` — [`RelocationType`](#relocationtype)

##### `impl StructuralPartialEq for RelocationType`

### `RelocAddendKind`

```rust
enum RelocAddendKind {
    None,
    Addend32,
    Addend64,
}
```

Indicates the kind of addend that applies to a relocation entry.

#### Variants

- **`None`**

  Relocation entry does not include an addend.

- **`Addend32`**

  Relocation entry includes a 32-bit addend.

- **`Addend64`**

  Relocation entry includes a 64-bit addend.

#### Trait Implementations

##### `impl Clone for RelocAddendKind`

- <span id="relocaddendkind-clone"></span>`fn clone(&self) -> RelocAddendKind` — [`RelocAddendKind`](#relocaddendkind)

##### `impl Copy for RelocAddendKind`

##### `impl Debug for RelocAddendKind`

- <span id="relocaddendkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RelocAddendKind`

##### `impl Hash for RelocAddendKind`

- <span id="relocaddendkind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for RelocAddendKind`

- <span id="relocaddendkind-partialeq-eq"></span>`fn eq(&self, other: &RelocAddendKind) -> bool` — [`RelocAddendKind`](#relocaddendkind)

##### `impl StructuralPartialEq for RelocAddendKind`

### `TableInit<'a>`

```rust
enum TableInit<'a> {
    RefNull,
    Expr(crate::ConstExpr<'a>),
}
```

Different modes of initializing a table.

#### Variants

- **`RefNull`**

  The table is initialized to all null elements.

- **`Expr`**

  Each element in the table is initialized with the specified constant
  expression.

#### Trait Implementations

##### `impl Clone for TableInit<'a>`

- <span id="tableinit-clone"></span>`fn clone(&self) -> TableInit<'a>` — [`TableInit`](#tableinit)

##### `impl Debug for TableInit<'a>`

- <span id="tableinit-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `UnpackedIndex`

```rust
enum UnpackedIndex {
    Module(u32),
    RecGroup(u32),
}
```

The uncompressed form of a `PackedIndex`.

Can be used for `match` statements.

#### Variants

- **`Module`**

  An index into a Wasm module's types space.

- **`RecGroup`**

  An index into the containing recursion group's elements.

#### Implementations

- <span id="unpackedindex-pack"></span>`fn pack(&self) -> Option<PackedIndex>` — [`PackedIndex`](#packedindex)

  Compress this index into its packed form.

  

  Returns `None` if an index is beyond implementation limits.

- <span id="unpackedindex-as-module-index"></span>`fn as_module_index(&self) -> Option<u32>`

  Get the underlying index into a module's types space, if any.

- <span id="unpackedindex-as-rec-group-index"></span>`fn as_rec_group_index(&self) -> Option<u32>`

  Get the underlying index into the containing recursion group, if any.

#### Trait Implementations

##### `impl Clone for UnpackedIndex`

- <span id="unpackedindex-clone"></span>`fn clone(&self) -> UnpackedIndex` — [`UnpackedIndex`](#unpackedindex)

##### `impl Copy for UnpackedIndex`

##### `impl Debug for UnpackedIndex`

- <span id="unpackedindex-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for UnpackedIndex`

- <span id="unpackedindex-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for UnpackedIndex`

##### `impl Hash for UnpackedIndex`

- <span id="unpackedindex-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for UnpackedIndex`

- <span id="unpackedindex-ord-cmp"></span>`fn cmp(&self, other: &UnpackedIndex) -> cmp::Ordering` — [`UnpackedIndex`](#unpackedindex)

##### `impl PartialEq for UnpackedIndex`

- <span id="unpackedindex-partialeq-eq"></span>`fn eq(&self, other: &UnpackedIndex) -> bool` — [`UnpackedIndex`](#unpackedindex)

##### `impl PartialOrd for UnpackedIndex`

- <span id="unpackedindex-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &UnpackedIndex) -> option::Option<cmp::Ordering>` — [`UnpackedIndex`](#unpackedindex)

##### `impl StructuralPartialEq for UnpackedIndex`

##### `impl ToString for UnpackedIndex`

- <span id="unpackedindex-tostring-to-string"></span>`fn to_string(&self) -> String` — [`String`](../../prelude/index.md#string)

### `RecGroupInner`

```rust
enum RecGroupInner {
    Implicit((usize, SubType)),
    Explicit(Vec<(usize, SubType)>),
}
```

#### Trait Implementations

##### `impl Clone for RecGroupInner`

- <span id="recgroupinner-clone"></span>`fn clone(&self) -> RecGroupInner` — [`RecGroupInner`](types/index.md#recgroupinner)

##### `impl Debug for RecGroupInner`

- <span id="recgroupinner-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `CompositeInnerType`

```rust
enum CompositeInnerType {
    Func(FuncType),
    Array(ArrayType),
    Struct(StructType),
    Cont(ContType),
}
```

A [`CompositeType`](#compositetype) can contain one of these types.

#### Variants

- **`Func`**

  The type is for a function.

- **`Array`**

  The type is for an array.

- **`Struct`**

  The type is for a struct.

- **`Cont`**

  The type is for a continuation.

#### Trait Implementations

##### `impl Clone for CompositeInnerType`

- <span id="compositeinnertype-clone"></span>`fn clone(&self) -> CompositeInnerType` — [`CompositeInnerType`](#compositeinnertype)

##### `impl Debug for CompositeInnerType`

- <span id="compositeinnertype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for CompositeInnerType`

- <span id="compositeinnertype-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CompositeInnerType`

##### `impl Hash for CompositeInnerType`

- <span id="compositeinnertype-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for CompositeInnerType`

- <span id="compositeinnertype-ord-cmp"></span>`fn cmp(&self, other: &CompositeInnerType) -> cmp::Ordering` — [`CompositeInnerType`](#compositeinnertype)

##### `impl PartialEq for CompositeInnerType`

- <span id="compositeinnertype-partialeq-eq"></span>`fn eq(&self, other: &CompositeInnerType) -> bool` — [`CompositeInnerType`](#compositeinnertype)

##### `impl PartialOrd for CompositeInnerType`

- <span id="compositeinnertype-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &CompositeInnerType) -> option::Option<cmp::Ordering>` — [`CompositeInnerType`](#compositeinnertype)

##### `impl StructuralPartialEq for CompositeInnerType`

##### `impl ToString for CompositeInnerType`

- <span id="compositeinnertype-tostring-to-string"></span>`fn to_string(&self) -> String` — [`String`](../../prelude/index.md#string)

### `StorageType`

```rust
enum StorageType {
    I8,
    I16,
    Val(ValType),
}
```

Represents storage types introduced in the GC spec for array and struct fields.

#### Variants

- **`I8`**

  The storage type is i8.

- **`I16`**

  The storage type is i16.

- **`Val`**

  The storage type is a value type.

#### Implementations

- <span id="storagetype-is-packed"></span>`fn is_packed(&self) -> bool`

  Is this a packed storage type, i.e. one that must be sign- or

  zero-extended when converted to a `ValType`?

- <span id="storagetype-unpack"></span>`fn unpack(&self) -> ValType` — [`ValType`](#valtype)

  Unpack this storage type into the valtype that it is represented as on

  the operand stack.

#### Trait Implementations

##### `impl Clone for StorageType`

- <span id="storagetype-clone"></span>`fn clone(&self) -> StorageType` — [`StorageType`](#storagetype)

##### `impl Copy for StorageType`

##### `impl Debug for StorageType`

- <span id="storagetype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for StorageType`

- <span id="storagetype-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for StorageType`

##### `impl FromReader for StorageType`

- <span id="storagetype-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

##### `impl Hash for StorageType`

- <span id="storagetype-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for StorageType`

- <span id="storagetype-ord-cmp"></span>`fn cmp(&self, other: &StorageType) -> cmp::Ordering` — [`StorageType`](#storagetype)

##### `impl PartialEq for StorageType`

- <span id="storagetype-partialeq-eq"></span>`fn eq(&self, other: &StorageType) -> bool` — [`StorageType`](#storagetype)

##### `impl PartialOrd for StorageType`

- <span id="storagetype-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &StorageType) -> option::Option<cmp::Ordering>` — [`StorageType`](#storagetype)

##### `impl StructuralPartialEq for StorageType`

##### `impl ToString for StorageType`

- <span id="storagetype-tostring-to-string"></span>`fn to_string(&self) -> String` — [`String`](../../prelude/index.md#string)

### `ValType`

```rust
enum ValType {
    I32,
    I64,
    F32,
    F64,
    V128,
    Ref(RefType),
}
```

Represents the types of values in a WebAssembly module.

#### Variants

- **`I32`**

  The value type is i32.

- **`I64`**

  The value type is i64.

- **`F32`**

  The value type is f32.

- **`F64`**

  The value type is f64.

- **`V128`**

  The value type is v128.

- **`Ref`**

  The value type is a reference.

#### Implementations

- <span id="valtype-const-funcref"></span>`const FUNCREF: ValType`

- <span id="valtype-const-externref"></span>`const EXTERNREF: ValType`

- <span id="valtype-const-exnref"></span>`const EXNREF: ValType`

- <span id="valtype-const-contref"></span>`const CONTREF: ValType`

- <span id="valtype-is-reference-type"></span>`fn is_reference_type(&self) -> bool`

  Returns whether this value type is a "reference type".

  

  Only reference types are allowed in tables, for example, and with some

  instructions. Current reference types include `funcref` and `externref`.

- <span id="valtype-as-reference-type"></span>`fn as_reference_type(&self) -> Option<RefType>` — [`RefType`](#reftype)

  Get the underlying reference type, if any.

- <span id="valtype-is-defaultable"></span>`fn is_defaultable(&self) -> bool`

  Whether the type is defaultable, i.e. it is not a non-nullable reference

  type.

#### Trait Implementations

##### `impl Clone for ValType`

- <span id="valtype-clone"></span>`fn clone(&self) -> ValType` — [`ValType`](#valtype)

##### `impl Copy for ValType`

##### `impl Debug for ValType`

- <span id="valtype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ValType`

- <span id="valtype-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ValType`

##### `impl FromReader for ValType`

- <span id="valtype-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

##### `impl Hash for ValType`

- <span id="valtype-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for ValType`

- <span id="valtype-ord-cmp"></span>`fn cmp(&self, other: &ValType) -> cmp::Ordering` — [`ValType`](#valtype)

##### `impl PartialEq for ValType`

- <span id="valtype-partialeq-eq"></span>`fn eq(&self, other: &ValType) -> bool` — [`ValType`](#valtype)

##### `impl PartialOrd for ValType`

- <span id="valtype-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &ValType) -> option::Option<cmp::Ordering>` — [`ValType`](#valtype)

##### `impl StructuralPartialEq for ValType`

##### `impl ToString for ValType`

- <span id="valtype-tostring-to-string"></span>`fn to_string(&self) -> String` — [`String`](../../prelude/index.md#string)

### `HeapType`

```rust
enum HeapType {
    Abstract {
        shared: bool,
        ty: AbstractHeapType,
    },
    Concrete(UnpackedIndex),
    Exact(UnpackedIndex),
}
```

A heap type.

#### Variants

- **`Abstract`**

  An abstract heap type; e.g., `anyref`.

- **`Concrete`**

  A concrete, user-defined type.
  
  Introduced in the function-references proposal.

- **`Exact`**

  An exact, user-defined type.
  
  Introduced in the custom-descriptors proposal.

#### Implementations

- <span id="heaptype-const-func"></span>`const FUNC: Self`

- <span id="heaptype-const-extern"></span>`const EXTERN: Self`

#### Trait Implementations

##### `impl Clone for HeapType`

- <span id="heaptype-clone"></span>`fn clone(&self) -> HeapType` — [`HeapType`](#heaptype)

##### `impl Copy for HeapType`

##### `impl Debug for HeapType`

- <span id="heaptype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for HeapType`

##### `impl FromReader for HeapType`

- <span id="heaptype-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

##### `impl Hash for HeapType`

- <span id="heaptype-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for HeapType`

- <span id="heaptype-partialeq-eq"></span>`fn eq(&self, other: &HeapType) -> bool` — [`HeapType`](#heaptype)

##### `impl StructuralPartialEq for HeapType`

### `AbstractHeapType`

```rust
enum AbstractHeapType {
    Func,
    Extern,
    Any,
    None,
    NoExtern,
    NoFunc,
    Eq,
    Struct,
    Array,
    I31,
    Exn,
    NoExn,
    Cont,
    NoCont,
}
```

An abstract heap type.

#### Variants

- **`Func`**

  The abstract, untyped (any) function.
  
  Introduced in the references-types proposal.

- **`Extern`**

  The abstract, external heap type.
  
  Introduced in the references-types proposal.

- **`Any`**

  The abstract `any` heap type.
  
  The common supertype (a.k.a. top) of all internal types.
  
  Introduced in the GC proposal.

- **`None`**

  The abstract `none` heap type.
  
  The common subtype (a.k.a. bottom) of all internal types.
  
  Introduced in the GC proposal.

- **`NoExtern`**

  The abstract `noextern` heap type.
  
  The common subtype (a.k.a. bottom) of all external types.
  
  Introduced in the GC proposal.

- **`NoFunc`**

  The abstract `nofunc` heap type.
  
  The common subtype (a.k.a. bottom) of all function types.
  
  Introduced in the GC proposal.

- **`Eq`**

  The abstract `eq` heap type.
  
  The common supertype of all heap types on which the `ref.eq`
  instruction is allowed.
  
  Introduced in the GC proposal.

- **`Struct`**

  The abstract `struct` heap type.
  
  The common supertype of all struct types.
  
  Introduced in the GC proposal.

- **`Array`**

  The abstract `array` heap type.
  
  The common supertype of all array types.
  
  Introduced in the GC proposal.

- **`I31`**

  The abstract `i31` heap type.
  
  It is not expected that Wasm runtimes actually store these
  values on the heap, but unbox them inline into the `i31ref`s
  themselves instead.
  
  Introduced in the GC proposal.

- **`Exn`**

  The abstraction `exception` heap type.
  
  Introduced in the exception-handling proposal.

- **`NoExn`**

  The abstract `noexn` heap type.
  
  The common subtype (a.k.a. bottom) of all exception types.
  
  Introduced in the exception-handling proposal.

- **`Cont`**

  The abstract `continuation` heap type.
  
  Introduced in the stack-switching proposal.

- **`NoCont`**

  The abstract `noexn` heap type.
  
  The common subtype (a.k.a. bottom) of all continuation types.
  
  Introduced in the stack-switching proposal.

#### Implementations

- <span id="abstractheaptype-as-str"></span>`const fn as_str(&self, nullable: bool) -> &str`

#### Trait Implementations

##### `impl Clone for AbstractHeapType`

- <span id="abstractheaptype-clone"></span>`fn clone(&self) -> AbstractHeapType` — [`AbstractHeapType`](#abstractheaptype)

##### `impl Copy for AbstractHeapType`

##### `impl Debug for AbstractHeapType`

- <span id="abstractheaptype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for AbstractHeapType`

##### `impl FromReader for AbstractHeapType`

- <span id="abstractheaptype-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../binary_reader/index.md#binaryreader), [`Result`](../../binary_reader/index.md#result)

##### `impl Hash for AbstractHeapType`

- <span id="abstractheaptype-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for AbstractHeapType`

- <span id="abstractheaptype-partialeq-eq"></span>`fn eq(&self, other: &AbstractHeapType) -> bool` — [`AbstractHeapType`](#abstractheaptype)

##### `impl StructuralPartialEq for AbstractHeapType`

### `TagKind`

```rust
enum TagKind {
    Exception,
}
```

Represents a tag kind.

#### Variants

- **`Exception`**

  The tag is an exception type.

#### Trait Implementations

##### `impl Clone for TagKind`

- <span id="tagkind-clone"></span>`fn clone(&self) -> TagKind` — [`TagKind`](#tagkind)

##### `impl Copy for TagKind`

##### `impl Debug for TagKind`

- <span id="tagkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for TagKind`

##### `impl PartialEq for TagKind`

- <span id="tagkind-partialeq-eq"></span>`fn eq(&self, other: &TagKind) -> bool` — [`TagKind`](#tagkind)

##### `impl StructuralPartialEq for TagKind`

## Traits

### `FrameStack`

```rust
trait FrameStack { ... }
```

A trait representing the stack of frames within a function.

The `BinaryReader::visit_operator` and
[`OperatorsReaders`](crate::OperatorsReader) type use
information about the current frame kind to enforce the syntactic
requirements of the binary format.

#### Required Methods

- `fn current_frame(&self) -> Option<FrameKind>`

  The current frame kind.

#### Implementors

- [`FrameStackAdapter`](operators/index.md#framestackadapter)
- [`OperatorsReader`](#operatorsreader)
- [`SingleFrameAdapter`](operators/index.md#singleframeadapter)

### `VisitOperator<'a>`

```rust
trait VisitOperator<'a> { ... }
```

Trait implemented by types that can visit all [`Operator`](#operator) variants.

#### Associated Types

- `type Output: 1`

#### Required Methods

- `fn visit_unreachable(&mut self) -> <Self as >::Output`

- `fn visit_nop(&mut self) -> <Self as >::Output`

- `fn visit_block(&mut self, blockty: BlockType) -> <Self as >::Output`

- `fn visit_loop(&mut self, blockty: BlockType) -> <Self as >::Output`

- `fn visit_if(&mut self, blockty: BlockType) -> <Self as >::Output`

- `fn visit_else(&mut self) -> <Self as >::Output`

- `fn visit_end(&mut self) -> <Self as >::Output`

- `fn visit_br(&mut self, relative_depth: u32) -> <Self as >::Output`

- `fn visit_br_if(&mut self, relative_depth: u32) -> <Self as >::Output`

- `fn visit_br_table(&mut self, targets: BrTable<'a>) -> <Self as >::Output`

- `fn visit_return(&mut self) -> <Self as >::Output`

- `fn visit_call(&mut self, function_index: u32) -> <Self as >::Output`

- `fn visit_call_indirect(&mut self, type_index: u32, table_index: u32) -> <Self as >::Output`

- `fn visit_drop(&mut self) -> <Self as >::Output`

- `fn visit_select(&mut self) -> <Self as >::Output`

- `fn visit_local_get(&mut self, local_index: u32) -> <Self as >::Output`

- `fn visit_local_set(&mut self, local_index: u32) -> <Self as >::Output`

- `fn visit_local_tee(&mut self, local_index: u32) -> <Self as >::Output`

- `fn visit_global_get(&mut self, global_index: u32) -> <Self as >::Output`

- `fn visit_global_set(&mut self, global_index: u32) -> <Self as >::Output`

- `fn visit_i32_load(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i64_load(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_f32_load(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_f64_load(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i32_load8_s(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i32_load8_u(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i32_load16_s(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i32_load16_u(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i64_load8_s(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i64_load8_u(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i64_load16_s(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i64_load16_u(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i64_load32_s(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i64_load32_u(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i32_store(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i64_store(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_f32_store(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_f64_store(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i32_store8(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i32_store16(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i64_store8(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i64_store16(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i64_store32(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_memory_size(&mut self, mem: u32) -> <Self as >::Output`

- `fn visit_memory_grow(&mut self, mem: u32) -> <Self as >::Output`

- `fn visit_i32_const(&mut self, value: i32) -> <Self as >::Output`

- `fn visit_i64_const(&mut self, value: i64) -> <Self as >::Output`

- `fn visit_f32_const(&mut self, value: Ieee32) -> <Self as >::Output`

- `fn visit_f64_const(&mut self, value: Ieee64) -> <Self as >::Output`

- `fn visit_i32_eqz(&mut self) -> <Self as >::Output`

- `fn visit_i32_eq(&mut self) -> <Self as >::Output`

- `fn visit_i32_ne(&mut self) -> <Self as >::Output`

- `fn visit_i32_lt_s(&mut self) -> <Self as >::Output`

- `fn visit_i32_lt_u(&mut self) -> <Self as >::Output`

- `fn visit_i32_gt_s(&mut self) -> <Self as >::Output`

- `fn visit_i32_gt_u(&mut self) -> <Self as >::Output`

- `fn visit_i32_le_s(&mut self) -> <Self as >::Output`

- `fn visit_i32_le_u(&mut self) -> <Self as >::Output`

- `fn visit_i32_ge_s(&mut self) -> <Self as >::Output`

- `fn visit_i32_ge_u(&mut self) -> <Self as >::Output`

- `fn visit_i64_eqz(&mut self) -> <Self as >::Output`

- `fn visit_i64_eq(&mut self) -> <Self as >::Output`

- `fn visit_i64_ne(&mut self) -> <Self as >::Output`

- `fn visit_i64_lt_s(&mut self) -> <Self as >::Output`

- `fn visit_i64_lt_u(&mut self) -> <Self as >::Output`

- `fn visit_i64_gt_s(&mut self) -> <Self as >::Output`

- `fn visit_i64_gt_u(&mut self) -> <Self as >::Output`

- `fn visit_i64_le_s(&mut self) -> <Self as >::Output`

- `fn visit_i64_le_u(&mut self) -> <Self as >::Output`

- `fn visit_i64_ge_s(&mut self) -> <Self as >::Output`

- `fn visit_i64_ge_u(&mut self) -> <Self as >::Output`

- `fn visit_f32_eq(&mut self) -> <Self as >::Output`

- `fn visit_f32_ne(&mut self) -> <Self as >::Output`

- `fn visit_f32_lt(&mut self) -> <Self as >::Output`

- `fn visit_f32_gt(&mut self) -> <Self as >::Output`

- `fn visit_f32_le(&mut self) -> <Self as >::Output`

- `fn visit_f32_ge(&mut self) -> <Self as >::Output`

- `fn visit_f64_eq(&mut self) -> <Self as >::Output`

- `fn visit_f64_ne(&mut self) -> <Self as >::Output`

- `fn visit_f64_lt(&mut self) -> <Self as >::Output`

- `fn visit_f64_gt(&mut self) -> <Self as >::Output`

- `fn visit_f64_le(&mut self) -> <Self as >::Output`

- `fn visit_f64_ge(&mut self) -> <Self as >::Output`

- `fn visit_i32_clz(&mut self) -> <Self as >::Output`

- `fn visit_i32_ctz(&mut self) -> <Self as >::Output`

- `fn visit_i32_popcnt(&mut self) -> <Self as >::Output`

- `fn visit_i32_add(&mut self) -> <Self as >::Output`

- `fn visit_i32_sub(&mut self) -> <Self as >::Output`

- `fn visit_i32_mul(&mut self) -> <Self as >::Output`

- `fn visit_i32_div_s(&mut self) -> <Self as >::Output`

- `fn visit_i32_div_u(&mut self) -> <Self as >::Output`

- `fn visit_i32_rem_s(&mut self) -> <Self as >::Output`

- `fn visit_i32_rem_u(&mut self) -> <Self as >::Output`

- `fn visit_i32_and(&mut self) -> <Self as >::Output`

- `fn visit_i32_or(&mut self) -> <Self as >::Output`

- `fn visit_i32_xor(&mut self) -> <Self as >::Output`

- `fn visit_i32_shl(&mut self) -> <Self as >::Output`

- `fn visit_i32_shr_s(&mut self) -> <Self as >::Output`

- `fn visit_i32_shr_u(&mut self) -> <Self as >::Output`

- `fn visit_i32_rotl(&mut self) -> <Self as >::Output`

- `fn visit_i32_rotr(&mut self) -> <Self as >::Output`

- `fn visit_i64_clz(&mut self) -> <Self as >::Output`

- `fn visit_i64_ctz(&mut self) -> <Self as >::Output`

- `fn visit_i64_popcnt(&mut self) -> <Self as >::Output`

- `fn visit_i64_add(&mut self) -> <Self as >::Output`

- `fn visit_i64_sub(&mut self) -> <Self as >::Output`

- `fn visit_i64_mul(&mut self) -> <Self as >::Output`

- `fn visit_i64_div_s(&mut self) -> <Self as >::Output`

- `fn visit_i64_div_u(&mut self) -> <Self as >::Output`

- `fn visit_i64_rem_s(&mut self) -> <Self as >::Output`

- `fn visit_i64_rem_u(&mut self) -> <Self as >::Output`

- `fn visit_i64_and(&mut self) -> <Self as >::Output`

- `fn visit_i64_or(&mut self) -> <Self as >::Output`

- `fn visit_i64_xor(&mut self) -> <Self as >::Output`

- `fn visit_i64_shl(&mut self) -> <Self as >::Output`

- `fn visit_i64_shr_s(&mut self) -> <Self as >::Output`

- `fn visit_i64_shr_u(&mut self) -> <Self as >::Output`

- `fn visit_i64_rotl(&mut self) -> <Self as >::Output`

- `fn visit_i64_rotr(&mut self) -> <Self as >::Output`

- `fn visit_f32_abs(&mut self) -> <Self as >::Output`

- `fn visit_f32_neg(&mut self) -> <Self as >::Output`

- `fn visit_f32_ceil(&mut self) -> <Self as >::Output`

- `fn visit_f32_floor(&mut self) -> <Self as >::Output`

- `fn visit_f32_trunc(&mut self) -> <Self as >::Output`

- `fn visit_f32_nearest(&mut self) -> <Self as >::Output`

- `fn visit_f32_sqrt(&mut self) -> <Self as >::Output`

- `fn visit_f32_add(&mut self) -> <Self as >::Output`

- `fn visit_f32_sub(&mut self) -> <Self as >::Output`

- `fn visit_f32_mul(&mut self) -> <Self as >::Output`

- `fn visit_f32_div(&mut self) -> <Self as >::Output`

- `fn visit_f32_min(&mut self) -> <Self as >::Output`

- `fn visit_f32_max(&mut self) -> <Self as >::Output`

- `fn visit_f32_copysign(&mut self) -> <Self as >::Output`

- `fn visit_f64_abs(&mut self) -> <Self as >::Output`

- `fn visit_f64_neg(&mut self) -> <Self as >::Output`

- `fn visit_f64_ceil(&mut self) -> <Self as >::Output`

- `fn visit_f64_floor(&mut self) -> <Self as >::Output`

- `fn visit_f64_trunc(&mut self) -> <Self as >::Output`

- `fn visit_f64_nearest(&mut self) -> <Self as >::Output`

- `fn visit_f64_sqrt(&mut self) -> <Self as >::Output`

- `fn visit_f64_add(&mut self) -> <Self as >::Output`

- `fn visit_f64_sub(&mut self) -> <Self as >::Output`

- `fn visit_f64_mul(&mut self) -> <Self as >::Output`

- `fn visit_f64_div(&mut self) -> <Self as >::Output`

- `fn visit_f64_min(&mut self) -> <Self as >::Output`

- `fn visit_f64_max(&mut self) -> <Self as >::Output`

- `fn visit_f64_copysign(&mut self) -> <Self as >::Output`

- `fn visit_i32_wrap_i64(&mut self) -> <Self as >::Output`

- `fn visit_i32_trunc_f32_s(&mut self) -> <Self as >::Output`

- `fn visit_i32_trunc_f32_u(&mut self) -> <Self as >::Output`

- `fn visit_i32_trunc_f64_s(&mut self) -> <Self as >::Output`

- `fn visit_i32_trunc_f64_u(&mut self) -> <Self as >::Output`

- `fn visit_i64_extend_i32_s(&mut self) -> <Self as >::Output`

- `fn visit_i64_extend_i32_u(&mut self) -> <Self as >::Output`

- `fn visit_i64_trunc_f32_s(&mut self) -> <Self as >::Output`

- `fn visit_i64_trunc_f32_u(&mut self) -> <Self as >::Output`

- `fn visit_i64_trunc_f64_s(&mut self) -> <Self as >::Output`

- `fn visit_i64_trunc_f64_u(&mut self) -> <Self as >::Output`

- `fn visit_f32_convert_i32_s(&mut self) -> <Self as >::Output`

- `fn visit_f32_convert_i32_u(&mut self) -> <Self as >::Output`

- `fn visit_f32_convert_i64_s(&mut self) -> <Self as >::Output`

- `fn visit_f32_convert_i64_u(&mut self) -> <Self as >::Output`

- `fn visit_f32_demote_f64(&mut self) -> <Self as >::Output`

- `fn visit_f64_convert_i32_s(&mut self) -> <Self as >::Output`

- `fn visit_f64_convert_i32_u(&mut self) -> <Self as >::Output`

- `fn visit_f64_convert_i64_s(&mut self) -> <Self as >::Output`

- `fn visit_f64_convert_i64_u(&mut self) -> <Self as >::Output`

- `fn visit_f64_promote_f32(&mut self) -> <Self as >::Output`

- `fn visit_i32_reinterpret_f32(&mut self) -> <Self as >::Output`

- `fn visit_i64_reinterpret_f64(&mut self) -> <Self as >::Output`

- `fn visit_f32_reinterpret_i32(&mut self) -> <Self as >::Output`

- `fn visit_f64_reinterpret_i64(&mut self) -> <Self as >::Output`

- `fn visit_i32_extend8_s(&mut self) -> <Self as >::Output`

- `fn visit_i32_extend16_s(&mut self) -> <Self as >::Output`

- `fn visit_i64_extend8_s(&mut self) -> <Self as >::Output`

- `fn visit_i64_extend16_s(&mut self) -> <Self as >::Output`

- `fn visit_i64_extend32_s(&mut self) -> <Self as >::Output`

- `fn visit_ref_eq(&mut self) -> <Self as >::Output`

- `fn visit_struct_new(&mut self, struct_type_index: u32) -> <Self as >::Output`

- `fn visit_struct_new_default(&mut self, struct_type_index: u32) -> <Self as >::Output`

- `fn visit_struct_get(&mut self, struct_type_index: u32, field_index: u32) -> <Self as >::Output`

- `fn visit_struct_get_s(&mut self, struct_type_index: u32, field_index: u32) -> <Self as >::Output`

- `fn visit_struct_get_u(&mut self, struct_type_index: u32, field_index: u32) -> <Self as >::Output`

- `fn visit_struct_set(&mut self, struct_type_index: u32, field_index: u32) -> <Self as >::Output`

- `fn visit_array_new(&mut self, array_type_index: u32) -> <Self as >::Output`

- `fn visit_array_new_default(&mut self, array_type_index: u32) -> <Self as >::Output`

- `fn visit_array_new_fixed(&mut self, array_type_index: u32, array_size: u32) -> <Self as >::Output`

- `fn visit_array_new_data(&mut self, array_type_index: u32, array_data_index: u32) -> <Self as >::Output`

- `fn visit_array_new_elem(&mut self, array_type_index: u32, array_elem_index: u32) -> <Self as >::Output`

- `fn visit_array_get(&mut self, array_type_index: u32) -> <Self as >::Output`

- `fn visit_array_get_s(&mut self, array_type_index: u32) -> <Self as >::Output`

- `fn visit_array_get_u(&mut self, array_type_index: u32) -> <Self as >::Output`

- `fn visit_array_set(&mut self, array_type_index: u32) -> <Self as >::Output`

- `fn visit_array_len(&mut self) -> <Self as >::Output`

- `fn visit_array_fill(&mut self, array_type_index: u32) -> <Self as >::Output`

- `fn visit_array_copy(&mut self, array_type_index_dst: u32, array_type_index_src: u32) -> <Self as >::Output`

- `fn visit_array_init_data(&mut self, array_type_index: u32, array_data_index: u32) -> <Self as >::Output`

- `fn visit_array_init_elem(&mut self, array_type_index: u32, array_elem_index: u32) -> <Self as >::Output`

- `fn visit_ref_test_non_null(&mut self, hty: HeapType) -> <Self as >::Output`

- `fn visit_ref_test_nullable(&mut self, hty: HeapType) -> <Self as >::Output`

- `fn visit_ref_cast_non_null(&mut self, hty: HeapType) -> <Self as >::Output`

- `fn visit_ref_cast_nullable(&mut self, hty: HeapType) -> <Self as >::Output`

- `fn visit_br_on_cast(&mut self, relative_depth: u32, from_ref_type: RefType, to_ref_type: RefType) -> <Self as >::Output`

- `fn visit_br_on_cast_fail(&mut self, relative_depth: u32, from_ref_type: RefType, to_ref_type: RefType) -> <Self as >::Output`

- `fn visit_any_convert_extern(&mut self) -> <Self as >::Output`

- `fn visit_extern_convert_any(&mut self) -> <Self as >::Output`

- `fn visit_ref_i31(&mut self) -> <Self as >::Output`

- `fn visit_i31_get_s(&mut self) -> <Self as >::Output`

- `fn visit_i31_get_u(&mut self) -> <Self as >::Output`

- `fn visit_struct_new_desc(&mut self, struct_type_index: u32) -> <Self as >::Output`

- `fn visit_struct_new_default_desc(&mut self, struct_type_index: u32) -> <Self as >::Output`

- `fn visit_ref_get_desc(&mut self, type_index: u32) -> <Self as >::Output`

- `fn visit_ref_cast_desc_non_null(&mut self, hty: HeapType) -> <Self as >::Output`

- `fn visit_ref_cast_desc_nullable(&mut self, hty: HeapType) -> <Self as >::Output`

- `fn visit_br_on_cast_desc(&mut self, relative_depth: u32, from_ref_type: RefType, to_ref_type: RefType) -> <Self as >::Output`

- `fn visit_br_on_cast_desc_fail(&mut self, relative_depth: u32, from_ref_type: RefType, to_ref_type: RefType) -> <Self as >::Output`

- `fn visit_i32_trunc_sat_f32_s(&mut self) -> <Self as >::Output`

- `fn visit_i32_trunc_sat_f32_u(&mut self) -> <Self as >::Output`

- `fn visit_i32_trunc_sat_f64_s(&mut self) -> <Self as >::Output`

- `fn visit_i32_trunc_sat_f64_u(&mut self) -> <Self as >::Output`

- `fn visit_i64_trunc_sat_f32_s(&mut self) -> <Self as >::Output`

- `fn visit_i64_trunc_sat_f32_u(&mut self) -> <Self as >::Output`

- `fn visit_i64_trunc_sat_f64_s(&mut self) -> <Self as >::Output`

- `fn visit_i64_trunc_sat_f64_u(&mut self) -> <Self as >::Output`

- `fn visit_memory_init(&mut self, data_index: u32, mem: u32) -> <Self as >::Output`

- `fn visit_data_drop(&mut self, data_index: u32) -> <Self as >::Output`

- `fn visit_memory_copy(&mut self, dst_mem: u32, src_mem: u32) -> <Self as >::Output`

- `fn visit_memory_fill(&mut self, mem: u32) -> <Self as >::Output`

- `fn visit_table_init(&mut self, elem_index: u32, table: u32) -> <Self as >::Output`

- `fn visit_elem_drop(&mut self, elem_index: u32) -> <Self as >::Output`

- `fn visit_table_copy(&mut self, dst_table: u32, src_table: u32) -> <Self as >::Output`

- `fn visit_typed_select(&mut self, ty: ValType) -> <Self as >::Output`

- `fn visit_typed_select_multi(&mut self, tys: Vec<ValType>) -> <Self as >::Output`

- `fn visit_ref_null(&mut self, hty: HeapType) -> <Self as >::Output`

- `fn visit_ref_is_null(&mut self) -> <Self as >::Output`

- `fn visit_ref_func(&mut self, function_index: u32) -> <Self as >::Output`

- `fn visit_table_fill(&mut self, table: u32) -> <Self as >::Output`

- `fn visit_table_get(&mut self, table: u32) -> <Self as >::Output`

- `fn visit_table_set(&mut self, table: u32) -> <Self as >::Output`

- `fn visit_table_grow(&mut self, table: u32) -> <Self as >::Output`

- `fn visit_table_size(&mut self, table: u32) -> <Self as >::Output`

- `fn visit_return_call(&mut self, function_index: u32) -> <Self as >::Output`

- `fn visit_return_call_indirect(&mut self, type_index: u32, table_index: u32) -> <Self as >::Output`

- `fn visit_memory_discard(&mut self, mem: u32) -> <Self as >::Output`

- `fn visit_memory_atomic_notify(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_memory_atomic_wait32(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_memory_atomic_wait64(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_atomic_fence(&mut self) -> <Self as >::Output`

- `fn visit_i32_atomic_load(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i64_atomic_load(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i32_atomic_load8_u(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i32_atomic_load16_u(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i64_atomic_load8_u(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i64_atomic_load16_u(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i64_atomic_load32_u(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i32_atomic_store(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i64_atomic_store(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i32_atomic_store8(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i32_atomic_store16(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i64_atomic_store8(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i64_atomic_store16(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i64_atomic_store32(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i32_atomic_rmw_add(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i64_atomic_rmw_add(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i32_atomic_rmw8_add_u(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i32_atomic_rmw16_add_u(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i64_atomic_rmw8_add_u(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i64_atomic_rmw16_add_u(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i64_atomic_rmw32_add_u(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i32_atomic_rmw_sub(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i64_atomic_rmw_sub(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i32_atomic_rmw8_sub_u(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i32_atomic_rmw16_sub_u(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i64_atomic_rmw8_sub_u(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i64_atomic_rmw16_sub_u(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i64_atomic_rmw32_sub_u(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i32_atomic_rmw_and(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i64_atomic_rmw_and(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i32_atomic_rmw8_and_u(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i32_atomic_rmw16_and_u(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i64_atomic_rmw8_and_u(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i64_atomic_rmw16_and_u(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i64_atomic_rmw32_and_u(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i32_atomic_rmw_or(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i64_atomic_rmw_or(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i32_atomic_rmw8_or_u(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i32_atomic_rmw16_or_u(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i64_atomic_rmw8_or_u(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i64_atomic_rmw16_or_u(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i64_atomic_rmw32_or_u(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i32_atomic_rmw_xor(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i64_atomic_rmw_xor(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i32_atomic_rmw8_xor_u(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i32_atomic_rmw16_xor_u(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i64_atomic_rmw8_xor_u(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i64_atomic_rmw16_xor_u(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i64_atomic_rmw32_xor_u(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i32_atomic_rmw_xchg(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i64_atomic_rmw_xchg(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i32_atomic_rmw8_xchg_u(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i32_atomic_rmw16_xchg_u(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i64_atomic_rmw8_xchg_u(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i64_atomic_rmw16_xchg_u(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i64_atomic_rmw32_xchg_u(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i32_atomic_rmw_cmpxchg(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i64_atomic_rmw_cmpxchg(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i32_atomic_rmw8_cmpxchg_u(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i32_atomic_rmw16_cmpxchg_u(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i64_atomic_rmw8_cmpxchg_u(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i64_atomic_rmw16_cmpxchg_u(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_i64_atomic_rmw32_cmpxchg_u(&mut self, memarg: MemArg) -> <Self as >::Output`

- `fn visit_try_table(&mut self, try_table: TryTable) -> <Self as >::Output`

- `fn visit_throw(&mut self, tag_index: u32) -> <Self as >::Output`

- `fn visit_throw_ref(&mut self) -> <Self as >::Output`

- `fn visit_try(&mut self, blockty: BlockType) -> <Self as >::Output`

- `fn visit_catch(&mut self, tag_index: u32) -> <Self as >::Output`

- `fn visit_rethrow(&mut self, relative_depth: u32) -> <Self as >::Output`

- `fn visit_delegate(&mut self, relative_depth: u32) -> <Self as >::Output`

- `fn visit_catch_all(&mut self) -> <Self as >::Output`

- `fn visit_global_atomic_get(&mut self, ordering: Ordering, global_index: u32) -> <Self as >::Output`

- `fn visit_global_atomic_set(&mut self, ordering: Ordering, global_index: u32) -> <Self as >::Output`

- `fn visit_global_atomic_rmw_add(&mut self, ordering: Ordering, global_index: u32) -> <Self as >::Output`

- `fn visit_global_atomic_rmw_sub(&mut self, ordering: Ordering, global_index: u32) -> <Self as >::Output`

- `fn visit_global_atomic_rmw_and(&mut self, ordering: Ordering, global_index: u32) -> <Self as >::Output`

- `fn visit_global_atomic_rmw_or(&mut self, ordering: Ordering, global_index: u32) -> <Self as >::Output`

- `fn visit_global_atomic_rmw_xor(&mut self, ordering: Ordering, global_index: u32) -> <Self as >::Output`

- `fn visit_global_atomic_rmw_xchg(&mut self, ordering: Ordering, global_index: u32) -> <Self as >::Output`

- `fn visit_global_atomic_rmw_cmpxchg(&mut self, ordering: Ordering, global_index: u32) -> <Self as >::Output`

- `fn visit_table_atomic_get(&mut self, ordering: Ordering, table_index: u32) -> <Self as >::Output`

- `fn visit_table_atomic_set(&mut self, ordering: Ordering, table_index: u32) -> <Self as >::Output`

- `fn visit_table_atomic_rmw_xchg(&mut self, ordering: Ordering, table_index: u32) -> <Self as >::Output`

- `fn visit_table_atomic_rmw_cmpxchg(&mut self, ordering: Ordering, table_index: u32) -> <Self as >::Output`

- `fn visit_struct_atomic_get(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <Self as >::Output`

- `fn visit_struct_atomic_get_s(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <Self as >::Output`

- `fn visit_struct_atomic_get_u(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <Self as >::Output`

- `fn visit_struct_atomic_set(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <Self as >::Output`

- `fn visit_struct_atomic_rmw_add(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <Self as >::Output`

- `fn visit_struct_atomic_rmw_sub(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <Self as >::Output`

- `fn visit_struct_atomic_rmw_and(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <Self as >::Output`

- `fn visit_struct_atomic_rmw_or(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <Self as >::Output`

- `fn visit_struct_atomic_rmw_xor(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <Self as >::Output`

- `fn visit_struct_atomic_rmw_xchg(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <Self as >::Output`

- `fn visit_struct_atomic_rmw_cmpxchg(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <Self as >::Output`

- `fn visit_array_atomic_get(&mut self, ordering: Ordering, array_type_index: u32) -> <Self as >::Output`

- `fn visit_array_atomic_get_s(&mut self, ordering: Ordering, array_type_index: u32) -> <Self as >::Output`

- `fn visit_array_atomic_get_u(&mut self, ordering: Ordering, array_type_index: u32) -> <Self as >::Output`

- `fn visit_array_atomic_set(&mut self, ordering: Ordering, array_type_index: u32) -> <Self as >::Output`

- `fn visit_array_atomic_rmw_add(&mut self, ordering: Ordering, array_type_index: u32) -> <Self as >::Output`

- `fn visit_array_atomic_rmw_sub(&mut self, ordering: Ordering, array_type_index: u32) -> <Self as >::Output`

- `fn visit_array_atomic_rmw_and(&mut self, ordering: Ordering, array_type_index: u32) -> <Self as >::Output`

- `fn visit_array_atomic_rmw_or(&mut self, ordering: Ordering, array_type_index: u32) -> <Self as >::Output`

- `fn visit_array_atomic_rmw_xor(&mut self, ordering: Ordering, array_type_index: u32) -> <Self as >::Output`

- `fn visit_array_atomic_rmw_xchg(&mut self, ordering: Ordering, array_type_index: u32) -> <Self as >::Output`

- `fn visit_array_atomic_rmw_cmpxchg(&mut self, ordering: Ordering, array_type_index: u32) -> <Self as >::Output`

- `fn visit_ref_i31_shared(&mut self) -> <Self as >::Output`

- `fn visit_call_ref(&mut self, type_index: u32) -> <Self as >::Output`

- `fn visit_return_call_ref(&mut self, type_index: u32) -> <Self as >::Output`

- `fn visit_ref_as_non_null(&mut self) -> <Self as >::Output`

- `fn visit_br_on_null(&mut self, relative_depth: u32) -> <Self as >::Output`

- `fn visit_br_on_non_null(&mut self, relative_depth: u32) -> <Self as >::Output`

- `fn visit_cont_new(&mut self, cont_type_index: u32) -> <Self as >::Output`

- `fn visit_cont_bind(&mut self, argument_index: u32, result_index: u32) -> <Self as >::Output`

- `fn visit_suspend(&mut self, tag_index: u32) -> <Self as >::Output`

- `fn visit_resume(&mut self, cont_type_index: u32, resume_table: ResumeTable) -> <Self as >::Output`

- `fn visit_resume_throw(&mut self, cont_type_index: u32, tag_index: u32, resume_table: ResumeTable) -> <Self as >::Output`

- `fn visit_switch(&mut self, cont_type_index: u32, tag_index: u32) -> <Self as >::Output`

- `fn visit_i64_add128(&mut self) -> <Self as >::Output`

- `fn visit_i64_sub128(&mut self) -> <Self as >::Output`

- `fn visit_i64_mul_wide_s(&mut self) -> <Self as >::Output`

- `fn visit_i64_mul_wide_u(&mut self) -> <Self as >::Output`

#### Provided Methods

- `fn visit_operator(&mut self, op: &Operator<'a>) -> <Self as >::Output`

  Visits the [`Operator`](#operator) `op` using the given `offset`.

#### Implementors

- [`Box`](../../prelude/index.md#box)
- [`FrameStackAdapter`](operators/index.md#framestackadapter)
- [`OperatorFactory`](operators/index.md#operatorfactory)
- [`SingleFrameAdapter`](operators/index.md#singleframeadapter)
- `&'b mut V`

## Functions

### `read_composite_type`

```rust
fn read_composite_type(opcode: u8, reader: &mut crate::BinaryReader<'_>) -> crate::Result<CompositeType, crate::BinaryReaderError>
```

## Type Aliases

### `BranchHintSectionReader<'a>`

```rust
type BranchHintSectionReader<'a> = crate::SectionLimited<'a, BranchHintFunction<'a>>;
```

A reader for the `metadata.code.branch_hint` custom section.

### `CodeSectionReader<'a>`

```rust
type CodeSectionReader<'a> = crate::SectionLimited<'a, FunctionBody<'a>>;
```

A reader for the code section of a WebAssembly module.

### `DataSectionReader<'a>`

```rust
type DataSectionReader<'a> = crate::SectionLimited<'a, Data<'a>>;
```

A reader for the data section of a WebAssembly module.

### `Dylink0SectionReader<'a>`

```rust
type Dylink0SectionReader<'a> = crate::Subsections<'a, Dylink0Subsection<'a>>;
```

Parser for the dynamic linking `dylink.0` custom section.

This format is currently defined upstream at
<https://github.com/WebAssembly/tool-conventions/blob/main/DynamicLinking.md>.

### `ElementSectionReader<'a>`

```rust
type ElementSectionReader<'a> = crate::SectionLimited<'a, Element<'a>>;
```

A reader for the element section of a WebAssembly module.

### `ExportSectionReader<'a>`

```rust
type ExportSectionReader<'a> = crate::SectionLimited<'a, Export<'a>>;
```

A reader for the export section of a WebAssembly module.

### `FunctionSectionReader<'a>`

```rust
type FunctionSectionReader<'a> = crate::SectionLimited<'a, u32>;
```

A reader for the function section of a WebAssembly module.

### `GlobalSectionReader<'a>`

```rust
type GlobalSectionReader<'a> = crate::SectionLimited<'a, Global<'a>>;
```

A reader for the global section of a WebAssembly module.

### `ImportSectionReader<'a>`

```rust
type ImportSectionReader<'a> = crate::SectionLimited<'a, Import<'a>>;
```

A reader for the import section of a WebAssembly module.

### `SegmentMap<'a>`

```rust
type SegmentMap<'a> = crate::SectionLimited<'a, Segment<'a>>;
```

Represents a reader for segments from the linking custom section.

### `InitFuncMap<'a>`

```rust
type InitFuncMap<'a> = crate::SectionLimited<'a, InitFunc>;
```

Represents a reader for init functions from the linking custom section.

### `ComdatMap<'a>`

```rust
type ComdatMap<'a> = crate::SectionLimited<'a, Comdat<'a>>;
```

Represents a reader for COMDAT data from the linking custom section.

### `SymbolInfoMap<'a>`

```rust
type SymbolInfoMap<'a> = crate::SectionLimited<'a, SymbolInfo<'a>>;
```

Represents a reader for symbol info from the linking custom section.

### `MemorySectionReader<'a>`

```rust
type MemorySectionReader<'a> = crate::SectionLimited<'a, crate::MemoryType>;
```

A reader for the memory section of a WebAssembly module.

### `NameMap<'a>`

```rust
type NameMap<'a> = crate::SectionLimited<'a, Naming<'a>>;
```

Represents a name map from the names custom section.

### `IndirectNameMap<'a>`

```rust
type IndirectNameMap<'a> = crate::SectionLimited<'a, IndirectNaming<'a>>;
```

Represents a reader for indirect names from the names custom section.

### `NameSectionReader<'a>`

```rust
type NameSectionReader<'a> = crate::Subsections<'a, Name<'a>>;
```

A reader for the name custom section of a WebAssembly module.

### `ProducersSectionReader<'a>`

```rust
type ProducersSectionReader<'a> = crate::SectionLimited<'a, ProducersField<'a>>;
```

A reader for the producers custom section of a WebAssembly module.

# Examples

```rust
let data: &[u8] = &[0x01, 0x08, 0x6c, 0x61, 0x6e, 0x67, 0x75, 0x61, 0x67, 0x65,
    0x02, 0x03, 0x77, 0x61, 0x74, 0x01, 0x31, 0x01, 0x43, 0x03, 0x39, 0x2e, 0x30];
use wasmparser::{ProducersSectionReader, ProducersFieldValue, Result, BinaryReader};
let reader = BinaryReader::new(data, 0);
let reader = ProducersSectionReader::new(reader).expect("producers reader");
let field = reader.into_iter().next().unwrap().expect("producers field");
assert!(field.name == "language");
let value = field.values.into_iter().collect::<Result<Vec<_>>>().expect("values");
assert!(value.len() == 2);
assert!(value[0].name == "wat" && value[0].version == "1");
assert!(value[1].name == "C" && value[1].version == "9.0");
```

### `RelocationEntryReader<'a>`

```rust
type RelocationEntryReader<'a> = crate::SectionLimited<'a, RelocationEntry>;
```

Reader for relocation entries within a `reloc.*` section.

### `TableSectionReader<'a>`

```rust
type TableSectionReader<'a> = crate::SectionLimited<'a, Table<'a>>;
```

A reader for the table section of a WebAssembly module.

### `TagSectionReader<'a>`

```rust
type TagSectionReader<'a> = crate::SectionLimited<'a, crate::TagType>;
```

A reader for the tags section of a WebAssembly module.

### `TypeSectionReader<'a>`

```rust
type TypeSectionReader<'a> = crate::SectionLimited<'a, RecGroup>;
```

A reader for the type section of a WebAssembly module.

## Constants

### `WASM_DYLINK_MEM_INFO`
```rust
const WASM_DYLINK_MEM_INFO: u8 = 1u8;
```

### `WASM_DYLINK_NEEDED`
```rust
const WASM_DYLINK_NEEDED: u8 = 2u8;
```

### `WASM_DYLINK_EXPORT_INFO`
```rust
const WASM_DYLINK_EXPORT_INFO: u8 = 3u8;
```

### `WASM_DYLINK_IMPORT_INFO`
```rust
const WASM_DYLINK_IMPORT_INFO: u8 = 4u8;
```

### `WASM_DYLINK_RUNTIME_PATH`
```rust
const WASM_DYLINK_RUNTIME_PATH: u8 = 5u8;
```

## Macros

### `define_operator!`

### `define_visit_operator!`

### `define_visit_operator_delegate!`

### `define_visit_operator!`

### `define_visit_operator_stack_adapter!`

### `define_passthrough_visit_operator!`

### `back_to_enum!`

