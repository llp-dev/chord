*[wasmparser](../../../index.md) / [readers](../../index.md) / [core](../index.md) / [types](index.md)*

---

# Module `types`

## Contents

- [Structs](#structs)
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
  - [`UnpackedIndex`](#unpackedindex)
  - [`RecGroupInner`](#recgroupinner)
  - [`CompositeInnerType`](#compositeinnertype)
  - [`StorageType`](#storagetype)
  - [`ValType`](#valtype)
  - [`HeapType`](#heaptype)
  - [`AbstractHeapType`](#abstractheaptype)
  - [`TagKind`](#tagkind)
- [Functions](#functions)
  - [`read_composite_type`](#read-composite-type)
- [Type Aliases](#type-aliases)
  - [`TypeSectionReader`](#typesectionreader)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
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
| [`UnpackedIndex`](#unpackedindex) | enum | The uncompressed form of a `PackedIndex`. |
| [`RecGroupInner`](#recgroupinner) | enum |  |
| [`CompositeInnerType`](#compositeinnertype) | enum | A [`CompositeType`] can contain one of these types. |
| [`StorageType`](#storagetype) | enum | Represents storage types introduced in the GC spec for array and struct fields. |
| [`ValType`](#valtype) | enum | Represents the types of values in a WebAssembly module. |
| [`HeapType`](#heaptype) | enum | A heap type. |
| [`AbstractHeapType`](#abstractheaptype) | enum | An abstract heap type. |
| [`TagKind`](#tagkind) | enum | Represents a tag kind. |
| [`read_composite_type`](#read-composite-type) | fn |  |
| [`TypeSectionReader`](#typesectionreader) | type | A reader for the type section of a WebAssembly module. |

## Structs

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

- <span id="packedindex-unpack"></span>`fn unpack(&self) -> UnpackedIndex` — [`UnpackedIndex`](../index.md#unpackedindex)

  Uncompress this packed index into an actual `enum` that can be matched

  on.

- <span id="packedindex-as-module-index"></span>`fn as_module_index(&self) -> Option<u32>`

  Get the underlying index into a module's types space, if any.

- <span id="packedindex-as-rec-group-index"></span>`fn as_rec_group_index(&self) -> Option<u32>`

  Get the underlying index into the containing recursion group, if any.

#### Trait Implementations

##### `impl Clone for PackedIndex`

- <span id="packedindex-clone"></span>`fn clone(&self) -> PackedIndex` — [`PackedIndex`](../index.md#packedindex)

##### `impl Copy for PackedIndex`

##### `impl Debug for PackedIndex`

- <span id="packedindex-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Display for PackedIndex`

- <span id="packedindex-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for PackedIndex`

##### `impl Hash for PackedIndex`

- <span id="packedindex-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for PackedIndex`

- <span id="packedindex-ord-cmp"></span>`fn cmp(&self, other: &PackedIndex) -> cmp::Ordering` — [`PackedIndex`](../index.md#packedindex)

##### `impl PartialEq for PackedIndex`

- <span id="packedindex-partialeq-eq"></span>`fn eq(&self, other: &PackedIndex) -> bool` — [`PackedIndex`](../index.md#packedindex)

##### `impl PartialOrd for PackedIndex`

- <span id="packedindex-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &PackedIndex) -> option::Option<cmp::Ordering>` — [`PackedIndex`](../index.md#packedindex)

##### `impl StructuralPartialEq for PackedIndex`

##### `impl ToString for PackedIndex`

- <span id="packedindex-tostring-to-string"></span>`fn to_string(&self) -> String` — [`String`](../../../prelude/index.md#string)

### `RecGroup`

```rust
struct RecGroup {
    inner: RecGroupInner,
}
```

Represents a recursive type group in a WebAssembly module.

#### Implementations

- <span id="recgroup-explicit"></span>`fn explicit(types: Vec<(usize, SubType)>) -> Self` — [`Vec`](../../../prelude/index.md#vec), [`SubType`](../index.md#subtype)

  Create an explicit `RecGroup` for the given types.

- <span id="recgroup-implicit"></span>`fn implicit(offset: usize, ty: SubType) -> Self` — [`SubType`](../index.md#subtype)

  Create an implicit `RecGroup` for a type that was not contained

  in a `(rec ...)`.

- <span id="recgroup-is-explicit-rec-group"></span>`fn is_explicit_rec_group(&self) -> bool`

  Is this an explicit recursion group?

- <span id="recgroup-types"></span>`fn types(&self) -> impl ExactSizeIterator<Item = &SubType> + '_` — [`SubType`](../index.md#subtype)

  Returns the list of subtypes in the recursive type group.

- <span id="recgroup-into-types"></span>`fn into_types(self) -> impl ExactSizeIterator<Item = SubType>` — [`SubType`](../index.md#subtype)

  Returns an owning iterator of all subtypes in this recursion

  group.

- <span id="recgroup-into-types-and-offsets"></span>`fn into_types_and_offsets(self) -> impl ExactSizeIterator<Item = (usize, SubType)>` — [`SubType`](../index.md#subtype)

  Returns an owning iterator of all subtypes in this recursion

  group, along with their offset.

#### Trait Implementations

##### `impl Clone for RecGroup`

- <span id="recgroup-clone"></span>`fn clone(&self) -> RecGroup` — [`RecGroup`](../index.md#recgroup)

##### `impl Debug for RecGroup`

- <span id="recgroup-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RecGroup`

##### `impl FromReader for RecGroup`

- <span id="recgroup-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

##### `impl Hash for RecGroup`

- <span id="recgroup-hash"></span>`fn hash<H: Hasher>(&self, hasher: &mut H)`

##### `impl Ord for RecGroup`

- <span id="recgroup-ord-cmp"></span>`fn cmp(&self, other: &Self) -> Ordering`

##### `impl PartialEq for RecGroup`

- <span id="recgroup-partialeq-eq"></span>`fn eq(&self, other: &RecGroup) -> bool` — [`RecGroup`](../index.md#recgroup)

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

- <span id="subtype-unwrap-array"></span>`fn unwrap_array(&self) -> &ArrayType` — [`ArrayType`](../index.md#arraytype)

  Unwrap an `ArrayType` or panic.

  

  Does not check finality or whether there is a supertype.

- <span id="subtype-func"></span>`fn func(signature: FuncType, shared: bool) -> Self` — [`FuncType`](../index.md#functype)

  Construct a function `SubType`.

- <span id="subtype-unwrap-func"></span>`fn unwrap_func(&self) -> &FuncType` — [`FuncType`](../index.md#functype)

  Unwrap an `FuncType` or panic.

  

  Does not check finality or whether there is a supertype.

- <span id="subtype-unwrap-struct"></span>`fn unwrap_struct(&self) -> &StructType` — [`StructType`](../index.md#structtype)

  Unwrap an `StructType` or panic.

  

  Does not check finality or whether there is a supertype.

- <span id="subtype-unwrap-cont"></span>`fn unwrap_cont(&self) -> &ContType` — [`ContType`](../index.md#conttype)

  Unwrap an `ContType` or panic.

  

  Does not check finality or whether there is a supertype.

#### Trait Implementations

##### `impl Clone for SubType`

- <span id="subtype-clone"></span>`fn clone(&self) -> SubType` — [`SubType`](../index.md#subtype)

##### `impl Debug for SubType`

- <span id="subtype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for SubType`

- <span id="subtype-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SubType`

##### `impl FromReader for SubType`

- <span id="subtype-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

##### `impl Hash for SubType`

- <span id="subtype-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for SubType`

- <span id="subtype-ord-cmp"></span>`fn cmp(&self, other: &SubType) -> cmp::Ordering` — [`SubType`](../index.md#subtype)

##### `impl PartialEq for SubType`

- <span id="subtype-partialeq-eq"></span>`fn eq(&self, other: &SubType) -> bool` — [`SubType`](../index.md#subtype)

##### `impl PartialOrd for SubType`

- <span id="subtype-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &SubType) -> option::Option<cmp::Ordering>` — [`SubType`](../index.md#subtype)

##### `impl StructuralPartialEq for SubType`

##### `impl ToString for SubType`

- <span id="subtype-tostring-to-string"></span>`fn to_string(&self) -> String` — [`String`](../../../prelude/index.md#string)

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

- <span id="compositetype-unwrap-func"></span>`fn unwrap_func(&self) -> &FuncType` — [`FuncType`](../index.md#functype)

  Unwrap a `FuncType` or panic.

- <span id="compositetype-unwrap-array"></span>`fn unwrap_array(&self) -> &ArrayType` — [`ArrayType`](../index.md#arraytype)

  Unwrap a `ArrayType` or panic.

- <span id="compositetype-unwrap-struct"></span>`fn unwrap_struct(&self) -> &StructType` — [`StructType`](../index.md#structtype)

  Unwrap a `StructType` or panic.

- <span id="compositetype-unwrap-cont"></span>`fn unwrap_cont(&self) -> &ContType` — [`ContType`](../index.md#conttype)

  Unwrap a `ContType` or panic.

#### Trait Implementations

##### `impl Clone for CompositeType`

- <span id="compositetype-clone"></span>`fn clone(&self) -> CompositeType` — [`CompositeType`](../index.md#compositetype)

##### `impl Debug for CompositeType`

- <span id="compositetype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for CompositeType`

- <span id="compositetype-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CompositeType`

##### `impl FromReader for CompositeType`

- <span id="compositetype-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

##### `impl Hash for CompositeType`

- <span id="compositetype-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for CompositeType`

- <span id="compositetype-ord-cmp"></span>`fn cmp(&self, other: &CompositeType) -> cmp::Ordering` — [`CompositeType`](../index.md#compositetype)

##### `impl PartialEq for CompositeType`

- <span id="compositetype-partialeq-eq"></span>`fn eq(&self, other: &CompositeType) -> bool` — [`CompositeType`](../index.md#compositetype)

##### `impl PartialOrd for CompositeType`

- <span id="compositetype-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &CompositeType) -> option::Option<cmp::Ordering>` — [`CompositeType`](../index.md#compositetype)

##### `impl StructuralPartialEq for CompositeType`

##### `impl ToString for CompositeType`

- <span id="compositetype-tostring-to-string"></span>`fn to_string(&self) -> String` — [`String`](../../../prelude/index.md#string)

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

  Creates a new [`FuncType`](../index.md) from the given `params` and `results`.

- <span id="functype-from-raw-parts"></span>`fn from_raw_parts(params_results: Box<[ValType]>, len_params: usize) -> Self` — [`Box`](../../../prelude/index.md#box), [`ValType`](../index.md#valtype)

  Creates a new [`FuncType`](../index.md) fom its raw parts.

  

  # Panics

  

  If `len_params` is greater than the length of `params_results` combined.

- <span id="functype-params"></span>`fn params(&self) -> &[ValType]` — [`ValType`](../index.md#valtype)

  Returns a shared slice to the parameter types of the [`FuncType`](../index.md).

- <span id="functype-results"></span>`fn results(&self) -> &[ValType]` — [`ValType`](../index.md#valtype)

  Returns a shared slice to the result types of the [`FuncType`](../index.md).

#### Trait Implementations

##### `impl Clone for FuncType`

- <span id="functype-clone"></span>`fn clone(&self) -> FuncType` — [`FuncType`](../index.md#functype)

##### `impl Debug for FuncType`

- <span id="functype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for FuncType`

- <span id="functype-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for FuncType`

##### `impl FromReader for FuncType`

- <span id="functype-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

##### `impl Hash for FuncType`

- <span id="functype-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for FuncType`

- <span id="functype-ord-cmp"></span>`fn cmp(&self, other: &FuncType) -> cmp::Ordering` — [`FuncType`](../index.md#functype)

##### `impl PartialEq for FuncType`

- <span id="functype-partialeq-eq"></span>`fn eq(&self, other: &FuncType) -> bool` — [`FuncType`](../index.md#functype)

##### `impl PartialOrd for FuncType`

- <span id="functype-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &FuncType) -> option::Option<cmp::Ordering>` — [`FuncType`](../index.md#functype)

##### `impl StructuralPartialEq for FuncType`

##### `impl ToString for FuncType`

- <span id="functype-tostring-to-string"></span>`fn to_string(&self) -> String` — [`String`](../../../prelude/index.md#string)

### `ArrayType`

```rust
struct ArrayType(FieldType);
```

Represents a type of an array in a WebAssembly module.

#### Trait Implementations

##### `impl Clone for ArrayType`

- <span id="arraytype-clone"></span>`fn clone(&self) -> ArrayType` — [`ArrayType`](../index.md#arraytype)

##### `impl Copy for ArrayType`

##### `impl Debug for ArrayType`

- <span id="arraytype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ArrayType`

- <span id="arraytype-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ArrayType`

##### `impl FromReader for ArrayType`

- <span id="arraytype-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

##### `impl Hash for ArrayType`

- <span id="arraytype-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for ArrayType`

- <span id="arraytype-ord-cmp"></span>`fn cmp(&self, other: &ArrayType) -> cmp::Ordering` — [`ArrayType`](../index.md#arraytype)

##### `impl PartialEq for ArrayType`

- <span id="arraytype-partialeq-eq"></span>`fn eq(&self, other: &ArrayType) -> bool` — [`ArrayType`](../index.md#arraytype)

##### `impl PartialOrd for ArrayType`

- <span id="arraytype-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &ArrayType) -> option::Option<cmp::Ordering>` — [`ArrayType`](../index.md#arraytype)

##### `impl StructuralPartialEq for ArrayType`

##### `impl ToString for ArrayType`

- <span id="arraytype-tostring-to-string"></span>`fn to_string(&self) -> String` — [`String`](../../../prelude/index.md#string)

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

- <span id="fieldtype-clone"></span>`fn clone(&self) -> FieldType` — [`FieldType`](../index.md#fieldtype)

##### `impl Copy for FieldType`

##### `impl Debug for FieldType`

- <span id="fieldtype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for FieldType`

- <span id="fieldtype-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for FieldType`

##### `impl FromReader for FieldType`

- <span id="fieldtype-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

##### `impl Hash for FieldType`

- <span id="fieldtype-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for FieldType`

- <span id="fieldtype-ord-cmp"></span>`fn cmp(&self, other: &FieldType) -> cmp::Ordering` — [`FieldType`](../index.md#fieldtype)

##### `impl PartialEq for FieldType`

- <span id="fieldtype-partialeq-eq"></span>`fn eq(&self, other: &FieldType) -> bool` — [`FieldType`](../index.md#fieldtype)

##### `impl PartialOrd for FieldType`

- <span id="fieldtype-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &FieldType) -> option::Option<cmp::Ordering>` — [`FieldType`](../index.md#fieldtype)

##### `impl StructuralPartialEq for FieldType`

##### `impl ToString for FieldType`

- <span id="fieldtype-tostring-to-string"></span>`fn to_string(&self) -> String` — [`String`](../../../prelude/index.md#string)

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

- <span id="structtype-clone"></span>`fn clone(&self) -> StructType` — [`StructType`](../index.md#structtype)

##### `impl Debug for StructType`

- <span id="structtype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for StructType`

- <span id="structtype-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for StructType`

##### `impl FromReader for StructType`

- <span id="structtype-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

##### `impl Hash for StructType`

- <span id="structtype-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for StructType`

- <span id="structtype-ord-cmp"></span>`fn cmp(&self, other: &StructType) -> cmp::Ordering` — [`StructType`](../index.md#structtype)

##### `impl PartialEq for StructType`

- <span id="structtype-partialeq-eq"></span>`fn eq(&self, other: &StructType) -> bool` — [`StructType`](../index.md#structtype)

##### `impl PartialOrd for StructType`

- <span id="structtype-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &StructType) -> option::Option<cmp::Ordering>` — [`StructType`](../index.md#structtype)

##### `impl StructuralPartialEq for StructType`

##### `impl ToString for StructType`

- <span id="structtype-tostring-to-string"></span>`fn to_string(&self) -> String` — [`String`](../../../prelude/index.md#string)

### `ContType`

```rust
struct ContType(PackedIndex);
```

Represents a type of a continuation in a WebAssembly module.

#### Trait Implementations

##### `impl Clone for ContType`

- <span id="conttype-clone"></span>`fn clone(&self) -> ContType` — [`ContType`](../index.md#conttype)

##### `impl Debug for ContType`

- <span id="conttype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ContType`

- <span id="conttype-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ContType`

##### `impl FromReader for ContType`

- <span id="conttype-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

##### `impl Hash for ContType`

- <span id="conttype-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for ContType`

- <span id="conttype-ord-cmp"></span>`fn cmp(&self, other: &ContType) -> cmp::Ordering` — [`ContType`](../index.md#conttype)

##### `impl PartialEq for ContType`

- <span id="conttype-partialeq-eq"></span>`fn eq(&self, other: &ContType) -> bool` — [`ContType`](../index.md#conttype)

##### `impl PartialOrd for ContType`

- <span id="conttype-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &ContType) -> option::Option<cmp::Ordering>` — [`ContType`](../index.md#conttype)

##### `impl StructuralPartialEq for ContType`

##### `impl ToString for ContType`

- <span id="conttype-tostring-to-string"></span>`fn to_string(&self) -> String` — [`String`](../../../prelude/index.md#string)

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

- <span id="reftype-concrete"></span>`fn concrete(nullable: bool, index: PackedIndex) -> Self` — [`PackedIndex`](../index.md#packedindex)

  Create a reference to a concrete Wasm-defined type at the given

  index.

  

  Returns `None` when the type index is beyond this crate's

  implementation limits and therefore is not representable.

- <span id="reftype-exact"></span>`fn exact(nullable: bool, index: PackedIndex) -> Self` — [`PackedIndex`](../index.md#packedindex)

  Create a reference to exact type.

- <span id="reftype-new"></span>`fn new(nullable: bool, heap_type: HeapType) -> Option<Self>` — [`HeapType`](../index.md#heaptype)

  Create a new `RefType`.

  

  Returns `None` when the heap type's type index (if any) is beyond this

  crate's implementation limits and therefore is not representable.

- <span id="reftype-difference"></span>`fn difference(a: RefType, b: RefType) -> RefType` — [`RefType`](../index.md#reftype)

  Compute the [type difference] between the two given ref types.

- <span id="reftype-is-concrete-type-ref"></span>`const fn is_concrete_type_ref(&self) -> bool`

  Is this a reference to an concrete type?

- <span id="reftype-is-exact-type-ref"></span>`const fn is_exact_type_ref(&self) -> bool`

  Is this an exact reference to a type?

- <span id="reftype-type-index"></span>`fn type_index(&self) -> Option<PackedIndex>` — [`PackedIndex`](../index.md#packedindex)

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

- <span id="reftype-heap-type"></span>`fn heap_type(&self) -> HeapType` — [`HeapType`](../index.md#heaptype)

  Get the heap type that this is a reference to.

#### Trait Implementations

##### `impl Clone for RefType`

- <span id="reftype-clone"></span>`fn clone(&self) -> RefType` — [`RefType`](../index.md#reftype)

##### `impl Copy for RefType`

##### `impl Debug for RefType`

- <span id="reftype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for RefType`

- <span id="reftype-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RefType`

##### `impl FromReader for RefType`

- <span id="reftype-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

##### `impl Hash for RefType`

- <span id="reftype-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for RefType`

- <span id="reftype-ord-cmp"></span>`fn cmp(&self, other: &RefType) -> cmp::Ordering` — [`RefType`](../index.md#reftype)

##### `impl PartialEq for RefType`

- <span id="reftype-partialeq-eq"></span>`fn eq(&self, other: &RefType) -> bool` — [`RefType`](../index.md#reftype)

##### `impl PartialOrd for RefType`

- <span id="reftype-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &RefType) -> option::Option<cmp::Ordering>` — [`RefType`](../index.md#reftype)

##### `impl StructuralPartialEq for RefType`

##### `impl ToString for RefType`

- <span id="reftype-tostring-to-string"></span>`fn to_string(&self) -> String` — [`String`](../../../prelude/index.md#string)

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

- <span id="tabletype-index-type"></span>`fn index_type(&self) -> ValType` — [`ValType`](../index.md#valtype)

  Gets the index type for the table.

#### Trait Implementations

##### `impl Clone for TableType`

- <span id="tabletype-clone"></span>`fn clone(&self) -> TableType` — [`TableType`](../index.md#tabletype)

##### `impl Copy for TableType`

##### `impl Debug for TableType`

- <span id="tabletype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for TableType`

##### `impl FromReader for crate::TableType`

- <span id="cratetabletype-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

##### `impl Hash for TableType`

- <span id="tabletype-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for TableType`

- <span id="tabletype-partialeq-eq"></span>`fn eq(&self, other: &TableType) -> bool` — [`TableType`](../index.md#tabletype)

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

- <span id="memorytype-index-type"></span>`fn index_type(&self) -> ValType` — [`ValType`](../index.md#valtype)

  Gets the index type for the memory.

#### Trait Implementations

##### `impl Clone for MemoryType`

- <span id="memorytype-clone"></span>`fn clone(&self) -> MemoryType` — [`MemoryType`](../index.md#memorytype)

##### `impl Copy for MemoryType`

##### `impl Debug for MemoryType`

- <span id="memorytype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for MemoryType`

##### `impl FromReader for crate::MemoryType`

- <span id="cratememorytype-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

##### `impl Hash for MemoryType`

- <span id="memorytype-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for MemoryType`

- <span id="memorytype-partialeq-eq"></span>`fn eq(&self, other: &MemoryType) -> bool` — [`MemoryType`](../index.md#memorytype)

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

- <span id="globaltype-clone"></span>`fn clone(&self) -> GlobalType` — [`GlobalType`](../index.md#globaltype)

##### `impl Copy for GlobalType`

##### `impl Debug for GlobalType`

- <span id="globaltype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for GlobalType`

##### `impl FromReader for crate::GlobalType`

- <span id="crateglobaltype-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

##### `impl Hash for GlobalType`

- <span id="globaltype-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for GlobalType`

- <span id="globaltype-partialeq-eq"></span>`fn eq(&self, other: &GlobalType) -> bool` — [`GlobalType`](../index.md#globaltype)

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

- <span id="tagtype-clone"></span>`fn clone(&self) -> TagType` — [`TagType`](../index.md#tagtype)

##### `impl Copy for TagType`

##### `impl Debug for TagType`

- <span id="tagtype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for TagType`

##### `impl FromReader for crate::TagType`

- <span id="cratetagtype-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

##### `impl PartialEq for TagType`

- <span id="tagtype-partialeq-eq"></span>`fn eq(&self, other: &TagType) -> bool` — [`TagType`](../index.md#tagtype)

##### `impl StructuralPartialEq for TagType`

## Enums

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

- <span id="unpackedindex-pack"></span>`fn pack(&self) -> Option<PackedIndex>` — [`PackedIndex`](../index.md#packedindex)

  Compress this index into its packed form.

  

  Returns `None` if an index is beyond implementation limits.

- <span id="unpackedindex-as-module-index"></span>`fn as_module_index(&self) -> Option<u32>`

  Get the underlying index into a module's types space, if any.

- <span id="unpackedindex-as-rec-group-index"></span>`fn as_rec_group_index(&self) -> Option<u32>`

  Get the underlying index into the containing recursion group, if any.

#### Trait Implementations

##### `impl Clone for UnpackedIndex`

- <span id="unpackedindex-clone"></span>`fn clone(&self) -> UnpackedIndex` — [`UnpackedIndex`](../index.md#unpackedindex)

##### `impl Copy for UnpackedIndex`

##### `impl Debug for UnpackedIndex`

- <span id="unpackedindex-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for UnpackedIndex`

- <span id="unpackedindex-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for UnpackedIndex`

##### `impl Hash for UnpackedIndex`

- <span id="unpackedindex-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for UnpackedIndex`

- <span id="unpackedindex-ord-cmp"></span>`fn cmp(&self, other: &UnpackedIndex) -> cmp::Ordering` — [`UnpackedIndex`](../index.md#unpackedindex)

##### `impl PartialEq for UnpackedIndex`

- <span id="unpackedindex-partialeq-eq"></span>`fn eq(&self, other: &UnpackedIndex) -> bool` — [`UnpackedIndex`](../index.md#unpackedindex)

##### `impl PartialOrd for UnpackedIndex`

- <span id="unpackedindex-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &UnpackedIndex) -> option::Option<cmp::Ordering>` — [`UnpackedIndex`](../index.md#unpackedindex)

##### `impl StructuralPartialEq for UnpackedIndex`

##### `impl ToString for UnpackedIndex`

- <span id="unpackedindex-tostring-to-string"></span>`fn to_string(&self) -> String` — [`String`](../../../prelude/index.md#string)

### `RecGroupInner`

```rust
enum RecGroupInner {
    Implicit((usize, SubType)),
    Explicit(Vec<(usize, SubType)>),
}
```

#### Trait Implementations

##### `impl Clone for RecGroupInner`

- <span id="recgroupinner-clone"></span>`fn clone(&self) -> RecGroupInner` — [`RecGroupInner`](#recgroupinner)

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

A [`CompositeType`](../index.md) can contain one of these types.

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

- <span id="compositeinnertype-clone"></span>`fn clone(&self) -> CompositeInnerType` — [`CompositeInnerType`](../index.md#compositeinnertype)

##### `impl Debug for CompositeInnerType`

- <span id="compositeinnertype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for CompositeInnerType`

- <span id="compositeinnertype-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CompositeInnerType`

##### `impl Hash for CompositeInnerType`

- <span id="compositeinnertype-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for CompositeInnerType`

- <span id="compositeinnertype-ord-cmp"></span>`fn cmp(&self, other: &CompositeInnerType) -> cmp::Ordering` — [`CompositeInnerType`](../index.md#compositeinnertype)

##### `impl PartialEq for CompositeInnerType`

- <span id="compositeinnertype-partialeq-eq"></span>`fn eq(&self, other: &CompositeInnerType) -> bool` — [`CompositeInnerType`](../index.md#compositeinnertype)

##### `impl PartialOrd for CompositeInnerType`

- <span id="compositeinnertype-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &CompositeInnerType) -> option::Option<cmp::Ordering>` — [`CompositeInnerType`](../index.md#compositeinnertype)

##### `impl StructuralPartialEq for CompositeInnerType`

##### `impl ToString for CompositeInnerType`

- <span id="compositeinnertype-tostring-to-string"></span>`fn to_string(&self) -> String` — [`String`](../../../prelude/index.md#string)

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

- <span id="storagetype-unpack"></span>`fn unpack(&self) -> ValType` — [`ValType`](../index.md#valtype)

  Unpack this storage type into the valtype that it is represented as on

  the operand stack.

#### Trait Implementations

##### `impl Clone for StorageType`

- <span id="storagetype-clone"></span>`fn clone(&self) -> StorageType` — [`StorageType`](../index.md#storagetype)

##### `impl Copy for StorageType`

##### `impl Debug for StorageType`

- <span id="storagetype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for StorageType`

- <span id="storagetype-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for StorageType`

##### `impl FromReader for StorageType`

- <span id="storagetype-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

##### `impl Hash for StorageType`

- <span id="storagetype-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for StorageType`

- <span id="storagetype-ord-cmp"></span>`fn cmp(&self, other: &StorageType) -> cmp::Ordering` — [`StorageType`](../index.md#storagetype)

##### `impl PartialEq for StorageType`

- <span id="storagetype-partialeq-eq"></span>`fn eq(&self, other: &StorageType) -> bool` — [`StorageType`](../index.md#storagetype)

##### `impl PartialOrd for StorageType`

- <span id="storagetype-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &StorageType) -> option::Option<cmp::Ordering>` — [`StorageType`](../index.md#storagetype)

##### `impl StructuralPartialEq for StorageType`

##### `impl ToString for StorageType`

- <span id="storagetype-tostring-to-string"></span>`fn to_string(&self) -> String` — [`String`](../../../prelude/index.md#string)

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

- <span id="valtype-as-reference-type"></span>`fn as_reference_type(&self) -> Option<RefType>` — [`RefType`](../index.md#reftype)

  Get the underlying reference type, if any.

- <span id="valtype-is-defaultable"></span>`fn is_defaultable(&self) -> bool`

  Whether the type is defaultable, i.e. it is not a non-nullable reference

  type.

#### Trait Implementations

##### `impl Clone for ValType`

- <span id="valtype-clone"></span>`fn clone(&self) -> ValType` — [`ValType`](../index.md#valtype)

##### `impl Copy for ValType`

##### `impl Debug for ValType`

- <span id="valtype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ValType`

- <span id="valtype-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ValType`

##### `impl FromReader for ValType`

- <span id="valtype-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

##### `impl Hash for ValType`

- <span id="valtype-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for ValType`

- <span id="valtype-ord-cmp"></span>`fn cmp(&self, other: &ValType) -> cmp::Ordering` — [`ValType`](../index.md#valtype)

##### `impl PartialEq for ValType`

- <span id="valtype-partialeq-eq"></span>`fn eq(&self, other: &ValType) -> bool` — [`ValType`](../index.md#valtype)

##### `impl PartialOrd for ValType`

- <span id="valtype-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &ValType) -> option::Option<cmp::Ordering>` — [`ValType`](../index.md#valtype)

##### `impl StructuralPartialEq for ValType`

##### `impl ToString for ValType`

- <span id="valtype-tostring-to-string"></span>`fn to_string(&self) -> String` — [`String`](../../../prelude/index.md#string)

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

- <span id="heaptype-clone"></span>`fn clone(&self) -> HeapType` — [`HeapType`](../index.md#heaptype)

##### `impl Copy for HeapType`

##### `impl Debug for HeapType`

- <span id="heaptype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for HeapType`

##### `impl FromReader for HeapType`

- <span id="heaptype-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

##### `impl Hash for HeapType`

- <span id="heaptype-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for HeapType`

- <span id="heaptype-partialeq-eq"></span>`fn eq(&self, other: &HeapType) -> bool` — [`HeapType`](../index.md#heaptype)

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

- <span id="abstractheaptype-clone"></span>`fn clone(&self) -> AbstractHeapType` — [`AbstractHeapType`](../index.md#abstractheaptype)

##### `impl Copy for AbstractHeapType`

##### `impl Debug for AbstractHeapType`

- <span id="abstractheaptype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for AbstractHeapType`

##### `impl FromReader for AbstractHeapType`

- <span id="abstractheaptype-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

##### `impl Hash for AbstractHeapType`

- <span id="abstractheaptype-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for AbstractHeapType`

- <span id="abstractheaptype-partialeq-eq"></span>`fn eq(&self, other: &AbstractHeapType) -> bool` — [`AbstractHeapType`](../index.md#abstractheaptype)

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

- <span id="tagkind-clone"></span>`fn clone(&self) -> TagKind` — [`TagKind`](../index.md#tagkind)

##### `impl Copy for TagKind`

##### `impl Debug for TagKind`

- <span id="tagkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for TagKind`

##### `impl PartialEq for TagKind`

- <span id="tagkind-partialeq-eq"></span>`fn eq(&self, other: &TagKind) -> bool` — [`TagKind`](../index.md#tagkind)

##### `impl StructuralPartialEq for TagKind`

## Functions

### `read_composite_type`

```rust
fn read_composite_type(opcode: u8, reader: &mut crate::BinaryReader<'_>) -> crate::Result<CompositeType, crate::BinaryReaderError>
```

## Type Aliases

### `TypeSectionReader<'a>`

```rust
type TypeSectionReader<'a> = crate::SectionLimited<'a, RecGroup>;
```

A reader for the type section of a WebAssembly module.

