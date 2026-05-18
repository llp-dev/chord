*[wasmparser](../../../index.md) / [readers](../../index.md) / [core](../index.md) / [operators](index.md)*

---

# Module `operators`

## Contents

- [Structs](#structs)
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
- [Enums](#enums)
  - [`BlockType`](#blocktype)
  - [`FrameKind`](#framekind)
  - [`Ordering`](#ordering)
  - [`Operator`](#operator)
  - [`Catch`](#catch)
  - [`Handle`](#handle)
- [Traits](#traits)
  - [`FrameStack`](#framestack)
  - [`VisitOperator`](#visitoperator)
- [Macros](#macros)
  - [`define_operator!`](#define-operator)
  - [`define_visit_operator!`](#define-visit-operator)
  - [`define_visit_operator_delegate!`](#define-visit-operator-delegate)
  - [`define_visit_operator!`](#define-visit-operator)
  - [`define_visit_operator_stack_adapter!`](#define-visit-operator-stack-adapter)
  - [`define_passthrough_visit_operator!`](#define-passthrough-visit-operator)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
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
| [`BlockType`](#blocktype) | enum | Represents a block type. |
| [`FrameKind`](#framekind) | enum | The kind of a control flow `Frame`. |
| [`Ordering`](#ordering) | enum | Represents the memory ordering for atomic instructions. |
| [`Operator`](#operator) | enum | Instructions as defined [here]. |
| [`Catch`](#catch) | enum | Catch clauses that can be specified in [`TryTable`]. |
| [`Handle`](#handle) | enum | Handle clauses that can be specified in [`ResumeTable`]. |
| [`FrameStack`](#framestack) | trait | A trait representing the stack of frames within a function. |
| [`VisitOperator`](#visitoperator) | trait | Trait implemented by types that can visit all [`Operator`] variants. |
| [`define_operator!`](#define-operator) | macro |  |
| [`define_visit_operator!`](#define-visit-operator) | macro |  |
| [`define_visit_operator_delegate!`](#define-visit-operator-delegate) | macro |  |
| [`define_visit_operator!`](#define-visit-operator) | macro |  |
| [`define_visit_operator_stack_adapter!`](#define-visit-operator-stack-adapter) | macro |  |
| [`define_passthrough_visit_operator!`](#define-passthrough-visit-operator) | macro |  |

## Structs

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

- <span id="memarg-clone"></span>`fn clone(&self) -> MemArg` — [`MemArg`](../index.md#memarg)

##### `impl Copy for MemArg`

##### `impl Debug for MemArg`

- <span id="memarg-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for MemArg`

##### `impl PartialEq for MemArg`

- <span id="memarg-partialeq-eq"></span>`fn eq(&self, other: &MemArg) -> bool` — [`MemArg`](../index.md#memarg)

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

- <span id="brtable-targets"></span>`fn targets(&self) -> BrTableTargets<'_>` — [`BrTableTargets`](../index.md#brtabletargets)

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

- <span id="brtable-clone"></span>`fn clone(&self) -> BrTable<'a>` — [`BrTable`](../index.md#brtable)

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

An iterator over the targets of a [`BrTable`](../index.md).

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

- <span id="ieee32-clone"></span>`fn clone(&self) -> Ieee32` — [`Ieee32`](../index.md#ieee32)

##### `impl Copy for Ieee32`

##### `impl Debug for Ieee32`

- <span id="ieee32-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Ieee32`

##### `impl Hash for Ieee32`

- <span id="ieee32-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Ieee32`

- <span id="ieee32-partialeq-eq"></span>`fn eq(&self, other: &Ieee32) -> bool` — [`Ieee32`](../index.md#ieee32)

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

- <span id="ieee64-clone"></span>`fn clone(&self) -> Ieee64` — [`Ieee64`](../index.md#ieee64)

##### `impl Copy for Ieee64`

##### `impl Debug for Ieee64`

- <span id="ieee64-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Ieee64`

##### `impl Hash for Ieee64`

- <span id="ieee64-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Ieee64`

- <span id="ieee64-partialeq-eq"></span>`fn eq(&self, other: &Ieee64) -> bool` — [`Ieee64`](../index.md#ieee64)

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

- <span id="v128-clone"></span>`fn clone(&self) -> V128` — [`V128`](../index.md#v128)

##### `impl Copy for V128`

##### `impl Debug for V128`

- <span id="v128-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for V128`

##### `impl Hash for V128`

- <span id="v128-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for V128`

- <span id="v128-partialeq-eq"></span>`fn eq(&self, other: &V128) -> bool` — [`V128`](../index.md#v128)

##### `impl StructuralPartialEq for V128`

### `ControlStack`

```rust
struct ControlStack {
    frames: Vec<FrameKind>,
    top: Option<FrameKind>,
}
```

The Wasm control stack for the [`OperatorsReader`](../index.md).

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

- <span id="controlstack-push"></span>`fn push(&mut self, frame: FrameKind)` — [`FrameKind`](../index.md#framekind)

  Pushes the `frame` to `self`.

- <span id="controlstack-pop"></span>`fn pop(&mut self) -> Option<FrameKind>` — [`FrameKind`](../index.md#framekind)

  Pops the top-most [`FrameKind`](../index.md) from `self`.

- <span id="controlstack-last"></span>`fn last(&self) -> Option<FrameKind>` — [`FrameKind`](../index.md#framekind)

  Returns the top-mot [`FrameKind`](../index.md).

#### Trait Implementations

##### `impl Clone for ControlStack`

- <span id="controlstack-clone"></span>`fn clone(&self) -> ControlStack` — [`ControlStack`](../index.md#controlstack)

##### `impl Debug for ControlStack`

- <span id="controlstack-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ControlStack`

- <span id="controlstack-default"></span>`fn default() -> ControlStack` — [`ControlStack`](../index.md#controlstack)

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

- <span id="framestackadapter-framestack-current-frame"></span>`fn current_frame(&self) -> Option<FrameKind>` — [`FrameKind`](../index.md#framekind)

##### `impl<T: VisitOperator<'a>> VisitOperator for FrameStackAdapter<'_, T>`

- <span id="framestackadapter-visitoperator-type-output"></span>`type Output = <T as VisitOperator>::Output`

- <span id="framestackadapter-visitoperator-visit-unreachable"></span>`fn visit_unreachable(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-nop"></span>`fn visit_nop(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-block"></span>`fn visit_block(&mut self, blockty: BlockType) -> <T as >::Output` — [`BlockType`](../index.md#blocktype), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-loop"></span>`fn visit_loop(&mut self, blockty: BlockType) -> <T as >::Output` — [`BlockType`](../index.md#blocktype), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-if"></span>`fn visit_if(&mut self, blockty: BlockType) -> <T as >::Output` — [`BlockType`](../index.md#blocktype), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-else"></span>`fn visit_else(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-end"></span>`fn visit_end(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-br"></span>`fn visit_br(&mut self, relative_depth: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-br-if"></span>`fn visit_br_if(&mut self, relative_depth: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-br-table"></span>`fn visit_br_table(&mut self, targets: BrTable<'a>) -> <T as >::Output` — [`BrTable`](../index.md#brtable), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-return"></span>`fn visit_return(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-call"></span>`fn visit_call(&mut self, function_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-call-indirect"></span>`fn visit_call_indirect(&mut self, type_index: u32, table_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-drop"></span>`fn visit_drop(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-select"></span>`fn visit_select(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-local-get"></span>`fn visit_local_get(&mut self, local_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-local-set"></span>`fn visit_local_set(&mut self, local_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-local-tee"></span>`fn visit_local_tee(&mut self, local_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-global-get"></span>`fn visit_global_get(&mut self, global_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-global-set"></span>`fn visit_global_set(&mut self, global_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-load"></span>`fn visit_i32_load(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-load"></span>`fn visit_i64_load(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-load"></span>`fn visit_f32_load(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-load"></span>`fn visit_f64_load(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-load8-s"></span>`fn visit_i32_load8_s(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-load8-u"></span>`fn visit_i32_load8_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-load16-s"></span>`fn visit_i32_load16_s(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-load16-u"></span>`fn visit_i32_load16_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-load8-s"></span>`fn visit_i64_load8_s(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-load8-u"></span>`fn visit_i64_load8_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-load16-s"></span>`fn visit_i64_load16_s(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-load16-u"></span>`fn visit_i64_load16_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-load32-s"></span>`fn visit_i64_load32_s(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-load32-u"></span>`fn visit_i64_load32_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-store"></span>`fn visit_i32_store(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-store"></span>`fn visit_i64_store(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-store"></span>`fn visit_f32_store(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-store"></span>`fn visit_f64_store(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-store8"></span>`fn visit_i32_store8(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-store16"></span>`fn visit_i32_store16(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-store8"></span>`fn visit_i64_store8(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-store16"></span>`fn visit_i64_store16(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-store32"></span>`fn visit_i64_store32(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-memory-size"></span>`fn visit_memory_size(&mut self, mem: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-memory-grow"></span>`fn visit_memory_grow(&mut self, mem: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-const"></span>`fn visit_i32_const(&mut self, value: i32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-const"></span>`fn visit_i64_const(&mut self, value: i64) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-const"></span>`fn visit_f32_const(&mut self, value: Ieee32) -> <T as >::Output` — [`Ieee32`](../index.md#ieee32), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-const"></span>`fn visit_f64_const(&mut self, value: Ieee64) -> <T as >::Output` — [`Ieee64`](../index.md#ieee64), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-eqz"></span>`fn visit_i32_eqz(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-eq"></span>`fn visit_i32_eq(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-ne"></span>`fn visit_i32_ne(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-lt-s"></span>`fn visit_i32_lt_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-lt-u"></span>`fn visit_i32_lt_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-gt-s"></span>`fn visit_i32_gt_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-gt-u"></span>`fn visit_i32_gt_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-le-s"></span>`fn visit_i32_le_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-le-u"></span>`fn visit_i32_le_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-ge-s"></span>`fn visit_i32_ge_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-ge-u"></span>`fn visit_i32_ge_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-eqz"></span>`fn visit_i64_eqz(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-eq"></span>`fn visit_i64_eq(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-ne"></span>`fn visit_i64_ne(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-lt-s"></span>`fn visit_i64_lt_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-lt-u"></span>`fn visit_i64_lt_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-gt-s"></span>`fn visit_i64_gt_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-gt-u"></span>`fn visit_i64_gt_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-le-s"></span>`fn visit_i64_le_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-le-u"></span>`fn visit_i64_le_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-ge-s"></span>`fn visit_i64_ge_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-ge-u"></span>`fn visit_i64_ge_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-eq"></span>`fn visit_f32_eq(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-ne"></span>`fn visit_f32_ne(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-lt"></span>`fn visit_f32_lt(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-gt"></span>`fn visit_f32_gt(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-le"></span>`fn visit_f32_le(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-ge"></span>`fn visit_f32_ge(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-eq"></span>`fn visit_f64_eq(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-ne"></span>`fn visit_f64_ne(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-lt"></span>`fn visit_f64_lt(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-gt"></span>`fn visit_f64_gt(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-le"></span>`fn visit_f64_le(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-ge"></span>`fn visit_f64_ge(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-clz"></span>`fn visit_i32_clz(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-ctz"></span>`fn visit_i32_ctz(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-popcnt"></span>`fn visit_i32_popcnt(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-add"></span>`fn visit_i32_add(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-sub"></span>`fn visit_i32_sub(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-mul"></span>`fn visit_i32_mul(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-div-s"></span>`fn visit_i32_div_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-div-u"></span>`fn visit_i32_div_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-rem-s"></span>`fn visit_i32_rem_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-rem-u"></span>`fn visit_i32_rem_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-and"></span>`fn visit_i32_and(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-or"></span>`fn visit_i32_or(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-xor"></span>`fn visit_i32_xor(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-shl"></span>`fn visit_i32_shl(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-shr-s"></span>`fn visit_i32_shr_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-shr-u"></span>`fn visit_i32_shr_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-rotl"></span>`fn visit_i32_rotl(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-rotr"></span>`fn visit_i32_rotr(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-clz"></span>`fn visit_i64_clz(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-ctz"></span>`fn visit_i64_ctz(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-popcnt"></span>`fn visit_i64_popcnt(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-add"></span>`fn visit_i64_add(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-sub"></span>`fn visit_i64_sub(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-mul"></span>`fn visit_i64_mul(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-div-s"></span>`fn visit_i64_div_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-div-u"></span>`fn visit_i64_div_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-rem-s"></span>`fn visit_i64_rem_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-rem-u"></span>`fn visit_i64_rem_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-and"></span>`fn visit_i64_and(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-or"></span>`fn visit_i64_or(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-xor"></span>`fn visit_i64_xor(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-shl"></span>`fn visit_i64_shl(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-shr-s"></span>`fn visit_i64_shr_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-shr-u"></span>`fn visit_i64_shr_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-rotl"></span>`fn visit_i64_rotl(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-rotr"></span>`fn visit_i64_rotr(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-abs"></span>`fn visit_f32_abs(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-neg"></span>`fn visit_f32_neg(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-ceil"></span>`fn visit_f32_ceil(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-floor"></span>`fn visit_f32_floor(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-trunc"></span>`fn visit_f32_trunc(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-nearest"></span>`fn visit_f32_nearest(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-sqrt"></span>`fn visit_f32_sqrt(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-add"></span>`fn visit_f32_add(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-sub"></span>`fn visit_f32_sub(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-mul"></span>`fn visit_f32_mul(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-div"></span>`fn visit_f32_div(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-min"></span>`fn visit_f32_min(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-max"></span>`fn visit_f32_max(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-copysign"></span>`fn visit_f32_copysign(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-abs"></span>`fn visit_f64_abs(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-neg"></span>`fn visit_f64_neg(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-ceil"></span>`fn visit_f64_ceil(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-floor"></span>`fn visit_f64_floor(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-trunc"></span>`fn visit_f64_trunc(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-nearest"></span>`fn visit_f64_nearest(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-sqrt"></span>`fn visit_f64_sqrt(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-add"></span>`fn visit_f64_add(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-sub"></span>`fn visit_f64_sub(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-mul"></span>`fn visit_f64_mul(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-div"></span>`fn visit_f64_div(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-min"></span>`fn visit_f64_min(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-max"></span>`fn visit_f64_max(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-copysign"></span>`fn visit_f64_copysign(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-wrap-i64"></span>`fn visit_i32_wrap_i64(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-trunc-f32-s"></span>`fn visit_i32_trunc_f32_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-trunc-f32-u"></span>`fn visit_i32_trunc_f32_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-trunc-f64-s"></span>`fn visit_i32_trunc_f64_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-trunc-f64-u"></span>`fn visit_i32_trunc_f64_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-extend-i32-s"></span>`fn visit_i64_extend_i32_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-extend-i32-u"></span>`fn visit_i64_extend_i32_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-trunc-f32-s"></span>`fn visit_i64_trunc_f32_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-trunc-f32-u"></span>`fn visit_i64_trunc_f32_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-trunc-f64-s"></span>`fn visit_i64_trunc_f64_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-trunc-f64-u"></span>`fn visit_i64_trunc_f64_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-convert-i32-s"></span>`fn visit_f32_convert_i32_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-convert-i32-u"></span>`fn visit_f32_convert_i32_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-convert-i64-s"></span>`fn visit_f32_convert_i64_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-convert-i64-u"></span>`fn visit_f32_convert_i64_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-demote-f64"></span>`fn visit_f32_demote_f64(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-convert-i32-s"></span>`fn visit_f64_convert_i32_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-convert-i32-u"></span>`fn visit_f64_convert_i32_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-convert-i64-s"></span>`fn visit_f64_convert_i64_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-convert-i64-u"></span>`fn visit_f64_convert_i64_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-promote-f32"></span>`fn visit_f64_promote_f32(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-reinterpret-f32"></span>`fn visit_i32_reinterpret_f32(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-reinterpret-f64"></span>`fn visit_i64_reinterpret_f64(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f32-reinterpret-i32"></span>`fn visit_f32_reinterpret_i32(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-f64-reinterpret-i64"></span>`fn visit_f64_reinterpret_i64(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-extend8-s"></span>`fn visit_i32_extend8_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-extend16-s"></span>`fn visit_i32_extend16_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-extend8-s"></span>`fn visit_i64_extend8_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-extend16-s"></span>`fn visit_i64_extend16_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-extend32-s"></span>`fn visit_i64_extend32_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-ref-eq"></span>`fn visit_ref_eq(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-struct-new"></span>`fn visit_struct_new(&mut self, struct_type_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-struct-new-default"></span>`fn visit_struct_new_default(&mut self, struct_type_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-struct-get"></span>`fn visit_struct_get(&mut self, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-struct-get-s"></span>`fn visit_struct_get_s(&mut self, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-struct-get-u"></span>`fn visit_struct_get_u(&mut self, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-struct-set"></span>`fn visit_struct_set(&mut self, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-array-new"></span>`fn visit_array_new(&mut self, array_type_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-array-new-default"></span>`fn visit_array_new_default(&mut self, array_type_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-array-new-fixed"></span>`fn visit_array_new_fixed(&mut self, array_type_index: u32, array_size: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-array-new-data"></span>`fn visit_array_new_data(&mut self, array_type_index: u32, array_data_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-array-new-elem"></span>`fn visit_array_new_elem(&mut self, array_type_index: u32, array_elem_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-array-get"></span>`fn visit_array_get(&mut self, array_type_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-array-get-s"></span>`fn visit_array_get_s(&mut self, array_type_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-array-get-u"></span>`fn visit_array_get_u(&mut self, array_type_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-array-set"></span>`fn visit_array_set(&mut self, array_type_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-array-len"></span>`fn visit_array_len(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-array-fill"></span>`fn visit_array_fill(&mut self, array_type_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-array-copy"></span>`fn visit_array_copy(&mut self, array_type_index_dst: u32, array_type_index_src: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-array-init-data"></span>`fn visit_array_init_data(&mut self, array_type_index: u32, array_data_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-array-init-elem"></span>`fn visit_array_init_elem(&mut self, array_type_index: u32, array_elem_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-ref-test-non-null"></span>`fn visit_ref_test_non_null(&mut self, hty: HeapType) -> <T as >::Output` — [`HeapType`](../index.md#heaptype), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-ref-test-nullable"></span>`fn visit_ref_test_nullable(&mut self, hty: HeapType) -> <T as >::Output` — [`HeapType`](../index.md#heaptype), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-ref-cast-non-null"></span>`fn visit_ref_cast_non_null(&mut self, hty: HeapType) -> <T as >::Output` — [`HeapType`](../index.md#heaptype), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-ref-cast-nullable"></span>`fn visit_ref_cast_nullable(&mut self, hty: HeapType) -> <T as >::Output` — [`HeapType`](../index.md#heaptype), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-br-on-cast"></span>`fn visit_br_on_cast(&mut self, relative_depth: u32, from_ref_type: RefType, to_ref_type: RefType) -> <T as >::Output` — [`RefType`](../index.md#reftype), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-br-on-cast-fail"></span>`fn visit_br_on_cast_fail(&mut self, relative_depth: u32, from_ref_type: RefType, to_ref_type: RefType) -> <T as >::Output` — [`RefType`](../index.md#reftype), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-any-convert-extern"></span>`fn visit_any_convert_extern(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-extern-convert-any"></span>`fn visit_extern_convert_any(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-ref-i31"></span>`fn visit_ref_i31(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i31-get-s"></span>`fn visit_i31_get_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i31-get-u"></span>`fn visit_i31_get_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-struct-new-desc"></span>`fn visit_struct_new_desc(&mut self, struct_type_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-struct-new-default-desc"></span>`fn visit_struct_new_default_desc(&mut self, struct_type_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-ref-get-desc"></span>`fn visit_ref_get_desc(&mut self, type_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-ref-cast-desc-non-null"></span>`fn visit_ref_cast_desc_non_null(&mut self, hty: HeapType) -> <T as >::Output` — [`HeapType`](../index.md#heaptype), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-ref-cast-desc-nullable"></span>`fn visit_ref_cast_desc_nullable(&mut self, hty: HeapType) -> <T as >::Output` — [`HeapType`](../index.md#heaptype), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-br-on-cast-desc"></span>`fn visit_br_on_cast_desc(&mut self, relative_depth: u32, from_ref_type: RefType, to_ref_type: RefType) -> <T as >::Output` — [`RefType`](../index.md#reftype), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-br-on-cast-desc-fail"></span>`fn visit_br_on_cast_desc_fail(&mut self, relative_depth: u32, from_ref_type: RefType, to_ref_type: RefType) -> <T as >::Output` — [`RefType`](../index.md#reftype), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-trunc-sat-f32-s"></span>`fn visit_i32_trunc_sat_f32_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-trunc-sat-f32-u"></span>`fn visit_i32_trunc_sat_f32_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-trunc-sat-f64-s"></span>`fn visit_i32_trunc_sat_f64_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-trunc-sat-f64-u"></span>`fn visit_i32_trunc_sat_f64_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-trunc-sat-f32-s"></span>`fn visit_i64_trunc_sat_f32_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-trunc-sat-f32-u"></span>`fn visit_i64_trunc_sat_f32_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-trunc-sat-f64-s"></span>`fn visit_i64_trunc_sat_f64_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-trunc-sat-f64-u"></span>`fn visit_i64_trunc_sat_f64_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-memory-init"></span>`fn visit_memory_init(&mut self, data_index: u32, mem: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-data-drop"></span>`fn visit_data_drop(&mut self, data_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-memory-copy"></span>`fn visit_memory_copy(&mut self, dst_mem: u32, src_mem: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-memory-fill"></span>`fn visit_memory_fill(&mut self, mem: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-table-init"></span>`fn visit_table_init(&mut self, elem_index: u32, table: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-elem-drop"></span>`fn visit_elem_drop(&mut self, elem_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-table-copy"></span>`fn visit_table_copy(&mut self, dst_table: u32, src_table: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-typed-select"></span>`fn visit_typed_select(&mut self, ty: ValType) -> <T as >::Output` — [`ValType`](../index.md#valtype), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-typed-select-multi"></span>`fn visit_typed_select_multi(&mut self, tys: Vec<ValType>) -> <T as >::Output` — [`Vec`](../../../prelude/index.md#vec), [`ValType`](../index.md#valtype), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-ref-null"></span>`fn visit_ref_null(&mut self, hty: HeapType) -> <T as >::Output` — [`HeapType`](../index.md#heaptype), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-ref-is-null"></span>`fn visit_ref_is_null(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-ref-func"></span>`fn visit_ref_func(&mut self, function_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-table-fill"></span>`fn visit_table_fill(&mut self, table: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-table-get"></span>`fn visit_table_get(&mut self, table: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-table-set"></span>`fn visit_table_set(&mut self, table: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-table-grow"></span>`fn visit_table_grow(&mut self, table: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-table-size"></span>`fn visit_table_size(&mut self, table: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-return-call"></span>`fn visit_return_call(&mut self, function_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-return-call-indirect"></span>`fn visit_return_call_indirect(&mut self, type_index: u32, table_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-memory-discard"></span>`fn visit_memory_discard(&mut self, mem: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-memory-atomic-notify"></span>`fn visit_memory_atomic_notify(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-memory-atomic-wait32"></span>`fn visit_memory_atomic_wait32(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-memory-atomic-wait64"></span>`fn visit_memory_atomic_wait64(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-atomic-fence"></span>`fn visit_atomic_fence(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-load"></span>`fn visit_i32_atomic_load(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-load"></span>`fn visit_i64_atomic_load(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-load8-u"></span>`fn visit_i32_atomic_load8_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-load16-u"></span>`fn visit_i32_atomic_load16_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-load8-u"></span>`fn visit_i64_atomic_load8_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-load16-u"></span>`fn visit_i64_atomic_load16_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-load32-u"></span>`fn visit_i64_atomic_load32_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-store"></span>`fn visit_i32_atomic_store(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-store"></span>`fn visit_i64_atomic_store(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-store8"></span>`fn visit_i32_atomic_store8(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-store16"></span>`fn visit_i32_atomic_store16(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-store8"></span>`fn visit_i64_atomic_store8(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-store16"></span>`fn visit_i64_atomic_store16(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-store32"></span>`fn visit_i64_atomic_store32(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-rmw-add"></span>`fn visit_i32_atomic_rmw_add(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw-add"></span>`fn visit_i64_atomic_rmw_add(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-rmw8-add-u"></span>`fn visit_i32_atomic_rmw8_add_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-rmw16-add-u"></span>`fn visit_i32_atomic_rmw16_add_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw8-add-u"></span>`fn visit_i64_atomic_rmw8_add_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw16-add-u"></span>`fn visit_i64_atomic_rmw16_add_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw32-add-u"></span>`fn visit_i64_atomic_rmw32_add_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-rmw-sub"></span>`fn visit_i32_atomic_rmw_sub(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw-sub"></span>`fn visit_i64_atomic_rmw_sub(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-rmw8-sub-u"></span>`fn visit_i32_atomic_rmw8_sub_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-rmw16-sub-u"></span>`fn visit_i32_atomic_rmw16_sub_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw8-sub-u"></span>`fn visit_i64_atomic_rmw8_sub_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw16-sub-u"></span>`fn visit_i64_atomic_rmw16_sub_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw32-sub-u"></span>`fn visit_i64_atomic_rmw32_sub_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-rmw-and"></span>`fn visit_i32_atomic_rmw_and(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw-and"></span>`fn visit_i64_atomic_rmw_and(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-rmw8-and-u"></span>`fn visit_i32_atomic_rmw8_and_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-rmw16-and-u"></span>`fn visit_i32_atomic_rmw16_and_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw8-and-u"></span>`fn visit_i64_atomic_rmw8_and_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw16-and-u"></span>`fn visit_i64_atomic_rmw16_and_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw32-and-u"></span>`fn visit_i64_atomic_rmw32_and_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-rmw-or"></span>`fn visit_i32_atomic_rmw_or(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw-or"></span>`fn visit_i64_atomic_rmw_or(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-rmw8-or-u"></span>`fn visit_i32_atomic_rmw8_or_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-rmw16-or-u"></span>`fn visit_i32_atomic_rmw16_or_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw8-or-u"></span>`fn visit_i64_atomic_rmw8_or_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw16-or-u"></span>`fn visit_i64_atomic_rmw16_or_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw32-or-u"></span>`fn visit_i64_atomic_rmw32_or_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-rmw-xor"></span>`fn visit_i32_atomic_rmw_xor(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw-xor"></span>`fn visit_i64_atomic_rmw_xor(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-rmw8-xor-u"></span>`fn visit_i32_atomic_rmw8_xor_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-rmw16-xor-u"></span>`fn visit_i32_atomic_rmw16_xor_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw8-xor-u"></span>`fn visit_i64_atomic_rmw8_xor_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw16-xor-u"></span>`fn visit_i64_atomic_rmw16_xor_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw32-xor-u"></span>`fn visit_i64_atomic_rmw32_xor_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-rmw-xchg"></span>`fn visit_i32_atomic_rmw_xchg(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw-xchg"></span>`fn visit_i64_atomic_rmw_xchg(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-rmw8-xchg-u"></span>`fn visit_i32_atomic_rmw8_xchg_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-rmw16-xchg-u"></span>`fn visit_i32_atomic_rmw16_xchg_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw8-xchg-u"></span>`fn visit_i64_atomic_rmw8_xchg_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw16-xchg-u"></span>`fn visit_i64_atomic_rmw16_xchg_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw32-xchg-u"></span>`fn visit_i64_atomic_rmw32_xchg_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-rmw-cmpxchg"></span>`fn visit_i32_atomic_rmw_cmpxchg(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw-cmpxchg"></span>`fn visit_i64_atomic_rmw_cmpxchg(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-rmw8-cmpxchg-u"></span>`fn visit_i32_atomic_rmw8_cmpxchg_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i32-atomic-rmw16-cmpxchg-u"></span>`fn visit_i32_atomic_rmw16_cmpxchg_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw8-cmpxchg-u"></span>`fn visit_i64_atomic_rmw8_cmpxchg_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw16-cmpxchg-u"></span>`fn visit_i64_atomic_rmw16_cmpxchg_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-atomic-rmw32-cmpxchg-u"></span>`fn visit_i64_atomic_rmw32_cmpxchg_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-try-table"></span>`fn visit_try_table(&mut self, try_table: TryTable) -> <T as >::Output` — [`TryTable`](../index.md#trytable), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-throw"></span>`fn visit_throw(&mut self, tag_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-throw-ref"></span>`fn visit_throw_ref(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-try"></span>`fn visit_try(&mut self, blockty: BlockType) -> <T as >::Output` — [`BlockType`](../index.md#blocktype), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-catch"></span>`fn visit_catch(&mut self, tag_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-rethrow"></span>`fn visit_rethrow(&mut self, relative_depth: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-delegate"></span>`fn visit_delegate(&mut self, relative_depth: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-catch-all"></span>`fn visit_catch_all(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-global-atomic-get"></span>`fn visit_global_atomic_get(&mut self, ordering: Ordering, global_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-global-atomic-set"></span>`fn visit_global_atomic_set(&mut self, ordering: Ordering, global_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-global-atomic-rmw-add"></span>`fn visit_global_atomic_rmw_add(&mut self, ordering: Ordering, global_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-global-atomic-rmw-sub"></span>`fn visit_global_atomic_rmw_sub(&mut self, ordering: Ordering, global_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-global-atomic-rmw-and"></span>`fn visit_global_atomic_rmw_and(&mut self, ordering: Ordering, global_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-global-atomic-rmw-or"></span>`fn visit_global_atomic_rmw_or(&mut self, ordering: Ordering, global_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-global-atomic-rmw-xor"></span>`fn visit_global_atomic_rmw_xor(&mut self, ordering: Ordering, global_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-global-atomic-rmw-xchg"></span>`fn visit_global_atomic_rmw_xchg(&mut self, ordering: Ordering, global_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-global-atomic-rmw-cmpxchg"></span>`fn visit_global_atomic_rmw_cmpxchg(&mut self, ordering: Ordering, global_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-table-atomic-get"></span>`fn visit_table_atomic_get(&mut self, ordering: Ordering, table_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-table-atomic-set"></span>`fn visit_table_atomic_set(&mut self, ordering: Ordering, table_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-table-atomic-rmw-xchg"></span>`fn visit_table_atomic_rmw_xchg(&mut self, ordering: Ordering, table_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-table-atomic-rmw-cmpxchg"></span>`fn visit_table_atomic_rmw_cmpxchg(&mut self, ordering: Ordering, table_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-struct-atomic-get"></span>`fn visit_struct_atomic_get(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-struct-atomic-get-s"></span>`fn visit_struct_atomic_get_s(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-struct-atomic-get-u"></span>`fn visit_struct_atomic_get_u(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-struct-atomic-set"></span>`fn visit_struct_atomic_set(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-struct-atomic-rmw-add"></span>`fn visit_struct_atomic_rmw_add(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-struct-atomic-rmw-sub"></span>`fn visit_struct_atomic_rmw_sub(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-struct-atomic-rmw-and"></span>`fn visit_struct_atomic_rmw_and(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-struct-atomic-rmw-or"></span>`fn visit_struct_atomic_rmw_or(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-struct-atomic-rmw-xor"></span>`fn visit_struct_atomic_rmw_xor(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-struct-atomic-rmw-xchg"></span>`fn visit_struct_atomic_rmw_xchg(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-struct-atomic-rmw-cmpxchg"></span>`fn visit_struct_atomic_rmw_cmpxchg(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-array-atomic-get"></span>`fn visit_array_atomic_get(&mut self, ordering: Ordering, array_type_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-array-atomic-get-s"></span>`fn visit_array_atomic_get_s(&mut self, ordering: Ordering, array_type_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-array-atomic-get-u"></span>`fn visit_array_atomic_get_u(&mut self, ordering: Ordering, array_type_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-array-atomic-set"></span>`fn visit_array_atomic_set(&mut self, ordering: Ordering, array_type_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-array-atomic-rmw-add"></span>`fn visit_array_atomic_rmw_add(&mut self, ordering: Ordering, array_type_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-array-atomic-rmw-sub"></span>`fn visit_array_atomic_rmw_sub(&mut self, ordering: Ordering, array_type_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-array-atomic-rmw-and"></span>`fn visit_array_atomic_rmw_and(&mut self, ordering: Ordering, array_type_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-array-atomic-rmw-or"></span>`fn visit_array_atomic_rmw_or(&mut self, ordering: Ordering, array_type_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-array-atomic-rmw-xor"></span>`fn visit_array_atomic_rmw_xor(&mut self, ordering: Ordering, array_type_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-array-atomic-rmw-xchg"></span>`fn visit_array_atomic_rmw_xchg(&mut self, ordering: Ordering, array_type_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-array-atomic-rmw-cmpxchg"></span>`fn visit_array_atomic_rmw_cmpxchg(&mut self, ordering: Ordering, array_type_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-ref-i31-shared"></span>`fn visit_ref_i31_shared(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-call-ref"></span>`fn visit_call_ref(&mut self, type_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-return-call-ref"></span>`fn visit_return_call_ref(&mut self, type_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-ref-as-non-null"></span>`fn visit_ref_as_non_null(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-br-on-null"></span>`fn visit_br_on_null(&mut self, relative_depth: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-br-on-non-null"></span>`fn visit_br_on_non_null(&mut self, relative_depth: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-cont-new"></span>`fn visit_cont_new(&mut self, cont_type_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-cont-bind"></span>`fn visit_cont_bind(&mut self, argument_index: u32, result_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-suspend"></span>`fn visit_suspend(&mut self, tag_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-resume"></span>`fn visit_resume(&mut self, cont_type_index: u32, resume_table: ResumeTable) -> <T as >::Output` — [`ResumeTable`](../index.md#resumetable), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-resume-throw"></span>`fn visit_resume_throw(&mut self, cont_type_index: u32, tag_index: u32, resume_table: ResumeTable) -> <T as >::Output` — [`ResumeTable`](../index.md#resumetable), [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-switch"></span>`fn visit_switch(&mut self, cont_type_index: u32, tag_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-add128"></span>`fn visit_i64_add128(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-sub128"></span>`fn visit_i64_sub128(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-mul-wide-s"></span>`fn visit_i64_mul_wide_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="framestackadapter-visitoperator-visit-i64-mul-wide-u"></span>`fn visit_i64_mul_wide_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

### `SingleFrameAdapter<'a, T>`

```rust
struct SingleFrameAdapter<'a, T> {
    current_frame: FrameKind,
    visitor: &'a mut T,
}
```

#### Trait Implementations

##### `impl<T> FrameStack for SingleFrameAdapter<'_, T>`

- <span id="singleframeadapter-framestack-current-frame"></span>`fn current_frame(&self) -> Option<FrameKind>` — [`FrameKind`](../index.md#framekind)

##### `impl<T: VisitOperator<'a>> VisitOperator for SingleFrameAdapter<'_, T>`

- <span id="singleframeadapter-visitoperator-type-output"></span>`type Output = <T as VisitOperator>::Output`

- <span id="singleframeadapter-visitoperator-visit-unreachable"></span>`fn visit_unreachable(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-nop"></span>`fn visit_nop(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-block"></span>`fn visit_block(&mut self, blockty: BlockType) -> <T as >::Output` — [`BlockType`](../index.md#blocktype), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-loop"></span>`fn visit_loop(&mut self, blockty: BlockType) -> <T as >::Output` — [`BlockType`](../index.md#blocktype), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-if"></span>`fn visit_if(&mut self, blockty: BlockType) -> <T as >::Output` — [`BlockType`](../index.md#blocktype), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-else"></span>`fn visit_else(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-end"></span>`fn visit_end(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-br"></span>`fn visit_br(&mut self, relative_depth: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-br-if"></span>`fn visit_br_if(&mut self, relative_depth: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-br-table"></span>`fn visit_br_table(&mut self, targets: BrTable<'a>) -> <T as >::Output` — [`BrTable`](../index.md#brtable), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-return"></span>`fn visit_return(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-call"></span>`fn visit_call(&mut self, function_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-call-indirect"></span>`fn visit_call_indirect(&mut self, type_index: u32, table_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-drop"></span>`fn visit_drop(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-select"></span>`fn visit_select(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-local-get"></span>`fn visit_local_get(&mut self, local_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-local-set"></span>`fn visit_local_set(&mut self, local_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-local-tee"></span>`fn visit_local_tee(&mut self, local_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-global-get"></span>`fn visit_global_get(&mut self, global_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-global-set"></span>`fn visit_global_set(&mut self, global_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-load"></span>`fn visit_i32_load(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-load"></span>`fn visit_i64_load(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-load"></span>`fn visit_f32_load(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-load"></span>`fn visit_f64_load(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-load8-s"></span>`fn visit_i32_load8_s(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-load8-u"></span>`fn visit_i32_load8_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-load16-s"></span>`fn visit_i32_load16_s(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-load16-u"></span>`fn visit_i32_load16_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-load8-s"></span>`fn visit_i64_load8_s(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-load8-u"></span>`fn visit_i64_load8_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-load16-s"></span>`fn visit_i64_load16_s(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-load16-u"></span>`fn visit_i64_load16_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-load32-s"></span>`fn visit_i64_load32_s(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-load32-u"></span>`fn visit_i64_load32_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-store"></span>`fn visit_i32_store(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-store"></span>`fn visit_i64_store(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-store"></span>`fn visit_f32_store(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-store"></span>`fn visit_f64_store(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-store8"></span>`fn visit_i32_store8(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-store16"></span>`fn visit_i32_store16(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-store8"></span>`fn visit_i64_store8(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-store16"></span>`fn visit_i64_store16(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-store32"></span>`fn visit_i64_store32(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-memory-size"></span>`fn visit_memory_size(&mut self, mem: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-memory-grow"></span>`fn visit_memory_grow(&mut self, mem: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-const"></span>`fn visit_i32_const(&mut self, value: i32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-const"></span>`fn visit_i64_const(&mut self, value: i64) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-const"></span>`fn visit_f32_const(&mut self, value: Ieee32) -> <T as >::Output` — [`Ieee32`](../index.md#ieee32), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-const"></span>`fn visit_f64_const(&mut self, value: Ieee64) -> <T as >::Output` — [`Ieee64`](../index.md#ieee64), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-eqz"></span>`fn visit_i32_eqz(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-eq"></span>`fn visit_i32_eq(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-ne"></span>`fn visit_i32_ne(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-lt-s"></span>`fn visit_i32_lt_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-lt-u"></span>`fn visit_i32_lt_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-gt-s"></span>`fn visit_i32_gt_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-gt-u"></span>`fn visit_i32_gt_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-le-s"></span>`fn visit_i32_le_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-le-u"></span>`fn visit_i32_le_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-ge-s"></span>`fn visit_i32_ge_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-ge-u"></span>`fn visit_i32_ge_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-eqz"></span>`fn visit_i64_eqz(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-eq"></span>`fn visit_i64_eq(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-ne"></span>`fn visit_i64_ne(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-lt-s"></span>`fn visit_i64_lt_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-lt-u"></span>`fn visit_i64_lt_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-gt-s"></span>`fn visit_i64_gt_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-gt-u"></span>`fn visit_i64_gt_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-le-s"></span>`fn visit_i64_le_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-le-u"></span>`fn visit_i64_le_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-ge-s"></span>`fn visit_i64_ge_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-ge-u"></span>`fn visit_i64_ge_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-eq"></span>`fn visit_f32_eq(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-ne"></span>`fn visit_f32_ne(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-lt"></span>`fn visit_f32_lt(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-gt"></span>`fn visit_f32_gt(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-le"></span>`fn visit_f32_le(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-ge"></span>`fn visit_f32_ge(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-eq"></span>`fn visit_f64_eq(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-ne"></span>`fn visit_f64_ne(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-lt"></span>`fn visit_f64_lt(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-gt"></span>`fn visit_f64_gt(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-le"></span>`fn visit_f64_le(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-ge"></span>`fn visit_f64_ge(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-clz"></span>`fn visit_i32_clz(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-ctz"></span>`fn visit_i32_ctz(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-popcnt"></span>`fn visit_i32_popcnt(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-add"></span>`fn visit_i32_add(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-sub"></span>`fn visit_i32_sub(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-mul"></span>`fn visit_i32_mul(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-div-s"></span>`fn visit_i32_div_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-div-u"></span>`fn visit_i32_div_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-rem-s"></span>`fn visit_i32_rem_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-rem-u"></span>`fn visit_i32_rem_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-and"></span>`fn visit_i32_and(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-or"></span>`fn visit_i32_or(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-xor"></span>`fn visit_i32_xor(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-shl"></span>`fn visit_i32_shl(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-shr-s"></span>`fn visit_i32_shr_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-shr-u"></span>`fn visit_i32_shr_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-rotl"></span>`fn visit_i32_rotl(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-rotr"></span>`fn visit_i32_rotr(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-clz"></span>`fn visit_i64_clz(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-ctz"></span>`fn visit_i64_ctz(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-popcnt"></span>`fn visit_i64_popcnt(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-add"></span>`fn visit_i64_add(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-sub"></span>`fn visit_i64_sub(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-mul"></span>`fn visit_i64_mul(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-div-s"></span>`fn visit_i64_div_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-div-u"></span>`fn visit_i64_div_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-rem-s"></span>`fn visit_i64_rem_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-rem-u"></span>`fn visit_i64_rem_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-and"></span>`fn visit_i64_and(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-or"></span>`fn visit_i64_or(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-xor"></span>`fn visit_i64_xor(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-shl"></span>`fn visit_i64_shl(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-shr-s"></span>`fn visit_i64_shr_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-shr-u"></span>`fn visit_i64_shr_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-rotl"></span>`fn visit_i64_rotl(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-rotr"></span>`fn visit_i64_rotr(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-abs"></span>`fn visit_f32_abs(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-neg"></span>`fn visit_f32_neg(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-ceil"></span>`fn visit_f32_ceil(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-floor"></span>`fn visit_f32_floor(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-trunc"></span>`fn visit_f32_trunc(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-nearest"></span>`fn visit_f32_nearest(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-sqrt"></span>`fn visit_f32_sqrt(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-add"></span>`fn visit_f32_add(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-sub"></span>`fn visit_f32_sub(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-mul"></span>`fn visit_f32_mul(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-div"></span>`fn visit_f32_div(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-min"></span>`fn visit_f32_min(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-max"></span>`fn visit_f32_max(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-copysign"></span>`fn visit_f32_copysign(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-abs"></span>`fn visit_f64_abs(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-neg"></span>`fn visit_f64_neg(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-ceil"></span>`fn visit_f64_ceil(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-floor"></span>`fn visit_f64_floor(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-trunc"></span>`fn visit_f64_trunc(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-nearest"></span>`fn visit_f64_nearest(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-sqrt"></span>`fn visit_f64_sqrt(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-add"></span>`fn visit_f64_add(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-sub"></span>`fn visit_f64_sub(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-mul"></span>`fn visit_f64_mul(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-div"></span>`fn visit_f64_div(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-min"></span>`fn visit_f64_min(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-max"></span>`fn visit_f64_max(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-copysign"></span>`fn visit_f64_copysign(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-wrap-i64"></span>`fn visit_i32_wrap_i64(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-trunc-f32-s"></span>`fn visit_i32_trunc_f32_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-trunc-f32-u"></span>`fn visit_i32_trunc_f32_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-trunc-f64-s"></span>`fn visit_i32_trunc_f64_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-trunc-f64-u"></span>`fn visit_i32_trunc_f64_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-extend-i32-s"></span>`fn visit_i64_extend_i32_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-extend-i32-u"></span>`fn visit_i64_extend_i32_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-trunc-f32-s"></span>`fn visit_i64_trunc_f32_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-trunc-f32-u"></span>`fn visit_i64_trunc_f32_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-trunc-f64-s"></span>`fn visit_i64_trunc_f64_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-trunc-f64-u"></span>`fn visit_i64_trunc_f64_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-convert-i32-s"></span>`fn visit_f32_convert_i32_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-convert-i32-u"></span>`fn visit_f32_convert_i32_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-convert-i64-s"></span>`fn visit_f32_convert_i64_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-convert-i64-u"></span>`fn visit_f32_convert_i64_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-demote-f64"></span>`fn visit_f32_demote_f64(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-convert-i32-s"></span>`fn visit_f64_convert_i32_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-convert-i32-u"></span>`fn visit_f64_convert_i32_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-convert-i64-s"></span>`fn visit_f64_convert_i64_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-convert-i64-u"></span>`fn visit_f64_convert_i64_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-promote-f32"></span>`fn visit_f64_promote_f32(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-reinterpret-f32"></span>`fn visit_i32_reinterpret_f32(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-reinterpret-f64"></span>`fn visit_i64_reinterpret_f64(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f32-reinterpret-i32"></span>`fn visit_f32_reinterpret_i32(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-f64-reinterpret-i64"></span>`fn visit_f64_reinterpret_i64(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-extend8-s"></span>`fn visit_i32_extend8_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-extend16-s"></span>`fn visit_i32_extend16_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-extend8-s"></span>`fn visit_i64_extend8_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-extend16-s"></span>`fn visit_i64_extend16_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-extend32-s"></span>`fn visit_i64_extend32_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-ref-eq"></span>`fn visit_ref_eq(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-struct-new"></span>`fn visit_struct_new(&mut self, struct_type_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-struct-new-default"></span>`fn visit_struct_new_default(&mut self, struct_type_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-struct-get"></span>`fn visit_struct_get(&mut self, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-struct-get-s"></span>`fn visit_struct_get_s(&mut self, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-struct-get-u"></span>`fn visit_struct_get_u(&mut self, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-struct-set"></span>`fn visit_struct_set(&mut self, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-array-new"></span>`fn visit_array_new(&mut self, array_type_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-array-new-default"></span>`fn visit_array_new_default(&mut self, array_type_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-array-new-fixed"></span>`fn visit_array_new_fixed(&mut self, array_type_index: u32, array_size: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-array-new-data"></span>`fn visit_array_new_data(&mut self, array_type_index: u32, array_data_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-array-new-elem"></span>`fn visit_array_new_elem(&mut self, array_type_index: u32, array_elem_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-array-get"></span>`fn visit_array_get(&mut self, array_type_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-array-get-s"></span>`fn visit_array_get_s(&mut self, array_type_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-array-get-u"></span>`fn visit_array_get_u(&mut self, array_type_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-array-set"></span>`fn visit_array_set(&mut self, array_type_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-array-len"></span>`fn visit_array_len(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-array-fill"></span>`fn visit_array_fill(&mut self, array_type_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-array-copy"></span>`fn visit_array_copy(&mut self, array_type_index_dst: u32, array_type_index_src: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-array-init-data"></span>`fn visit_array_init_data(&mut self, array_type_index: u32, array_data_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-array-init-elem"></span>`fn visit_array_init_elem(&mut self, array_type_index: u32, array_elem_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-ref-test-non-null"></span>`fn visit_ref_test_non_null(&mut self, hty: HeapType) -> <T as >::Output` — [`HeapType`](../index.md#heaptype), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-ref-test-nullable"></span>`fn visit_ref_test_nullable(&mut self, hty: HeapType) -> <T as >::Output` — [`HeapType`](../index.md#heaptype), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-ref-cast-non-null"></span>`fn visit_ref_cast_non_null(&mut self, hty: HeapType) -> <T as >::Output` — [`HeapType`](../index.md#heaptype), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-ref-cast-nullable"></span>`fn visit_ref_cast_nullable(&mut self, hty: HeapType) -> <T as >::Output` — [`HeapType`](../index.md#heaptype), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-br-on-cast"></span>`fn visit_br_on_cast(&mut self, relative_depth: u32, from_ref_type: RefType, to_ref_type: RefType) -> <T as >::Output` — [`RefType`](../index.md#reftype), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-br-on-cast-fail"></span>`fn visit_br_on_cast_fail(&mut self, relative_depth: u32, from_ref_type: RefType, to_ref_type: RefType) -> <T as >::Output` — [`RefType`](../index.md#reftype), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-any-convert-extern"></span>`fn visit_any_convert_extern(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-extern-convert-any"></span>`fn visit_extern_convert_any(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-ref-i31"></span>`fn visit_ref_i31(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i31-get-s"></span>`fn visit_i31_get_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i31-get-u"></span>`fn visit_i31_get_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-struct-new-desc"></span>`fn visit_struct_new_desc(&mut self, struct_type_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-struct-new-default-desc"></span>`fn visit_struct_new_default_desc(&mut self, struct_type_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-ref-get-desc"></span>`fn visit_ref_get_desc(&mut self, type_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-ref-cast-desc-non-null"></span>`fn visit_ref_cast_desc_non_null(&mut self, hty: HeapType) -> <T as >::Output` — [`HeapType`](../index.md#heaptype), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-ref-cast-desc-nullable"></span>`fn visit_ref_cast_desc_nullable(&mut self, hty: HeapType) -> <T as >::Output` — [`HeapType`](../index.md#heaptype), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-br-on-cast-desc"></span>`fn visit_br_on_cast_desc(&mut self, relative_depth: u32, from_ref_type: RefType, to_ref_type: RefType) -> <T as >::Output` — [`RefType`](../index.md#reftype), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-br-on-cast-desc-fail"></span>`fn visit_br_on_cast_desc_fail(&mut self, relative_depth: u32, from_ref_type: RefType, to_ref_type: RefType) -> <T as >::Output` — [`RefType`](../index.md#reftype), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-trunc-sat-f32-s"></span>`fn visit_i32_trunc_sat_f32_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-trunc-sat-f32-u"></span>`fn visit_i32_trunc_sat_f32_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-trunc-sat-f64-s"></span>`fn visit_i32_trunc_sat_f64_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-trunc-sat-f64-u"></span>`fn visit_i32_trunc_sat_f64_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-trunc-sat-f32-s"></span>`fn visit_i64_trunc_sat_f32_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-trunc-sat-f32-u"></span>`fn visit_i64_trunc_sat_f32_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-trunc-sat-f64-s"></span>`fn visit_i64_trunc_sat_f64_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-trunc-sat-f64-u"></span>`fn visit_i64_trunc_sat_f64_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-memory-init"></span>`fn visit_memory_init(&mut self, data_index: u32, mem: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-data-drop"></span>`fn visit_data_drop(&mut self, data_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-memory-copy"></span>`fn visit_memory_copy(&mut self, dst_mem: u32, src_mem: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-memory-fill"></span>`fn visit_memory_fill(&mut self, mem: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-table-init"></span>`fn visit_table_init(&mut self, elem_index: u32, table: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-elem-drop"></span>`fn visit_elem_drop(&mut self, elem_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-table-copy"></span>`fn visit_table_copy(&mut self, dst_table: u32, src_table: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-typed-select"></span>`fn visit_typed_select(&mut self, ty: ValType) -> <T as >::Output` — [`ValType`](../index.md#valtype), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-typed-select-multi"></span>`fn visit_typed_select_multi(&mut self, tys: Vec<ValType>) -> <T as >::Output` — [`Vec`](../../../prelude/index.md#vec), [`ValType`](../index.md#valtype), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-ref-null"></span>`fn visit_ref_null(&mut self, hty: HeapType) -> <T as >::Output` — [`HeapType`](../index.md#heaptype), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-ref-is-null"></span>`fn visit_ref_is_null(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-ref-func"></span>`fn visit_ref_func(&mut self, function_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-table-fill"></span>`fn visit_table_fill(&mut self, table: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-table-get"></span>`fn visit_table_get(&mut self, table: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-table-set"></span>`fn visit_table_set(&mut self, table: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-table-grow"></span>`fn visit_table_grow(&mut self, table: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-table-size"></span>`fn visit_table_size(&mut self, table: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-return-call"></span>`fn visit_return_call(&mut self, function_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-return-call-indirect"></span>`fn visit_return_call_indirect(&mut self, type_index: u32, table_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-memory-discard"></span>`fn visit_memory_discard(&mut self, mem: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-memory-atomic-notify"></span>`fn visit_memory_atomic_notify(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-memory-atomic-wait32"></span>`fn visit_memory_atomic_wait32(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-memory-atomic-wait64"></span>`fn visit_memory_atomic_wait64(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-atomic-fence"></span>`fn visit_atomic_fence(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-load"></span>`fn visit_i32_atomic_load(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-load"></span>`fn visit_i64_atomic_load(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-load8-u"></span>`fn visit_i32_atomic_load8_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-load16-u"></span>`fn visit_i32_atomic_load16_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-load8-u"></span>`fn visit_i64_atomic_load8_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-load16-u"></span>`fn visit_i64_atomic_load16_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-load32-u"></span>`fn visit_i64_atomic_load32_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-store"></span>`fn visit_i32_atomic_store(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-store"></span>`fn visit_i64_atomic_store(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-store8"></span>`fn visit_i32_atomic_store8(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-store16"></span>`fn visit_i32_atomic_store16(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-store8"></span>`fn visit_i64_atomic_store8(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-store16"></span>`fn visit_i64_atomic_store16(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-store32"></span>`fn visit_i64_atomic_store32(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-rmw-add"></span>`fn visit_i32_atomic_rmw_add(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw-add"></span>`fn visit_i64_atomic_rmw_add(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-rmw8-add-u"></span>`fn visit_i32_atomic_rmw8_add_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-rmw16-add-u"></span>`fn visit_i32_atomic_rmw16_add_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw8-add-u"></span>`fn visit_i64_atomic_rmw8_add_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw16-add-u"></span>`fn visit_i64_atomic_rmw16_add_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw32-add-u"></span>`fn visit_i64_atomic_rmw32_add_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-rmw-sub"></span>`fn visit_i32_atomic_rmw_sub(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw-sub"></span>`fn visit_i64_atomic_rmw_sub(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-rmw8-sub-u"></span>`fn visit_i32_atomic_rmw8_sub_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-rmw16-sub-u"></span>`fn visit_i32_atomic_rmw16_sub_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw8-sub-u"></span>`fn visit_i64_atomic_rmw8_sub_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw16-sub-u"></span>`fn visit_i64_atomic_rmw16_sub_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw32-sub-u"></span>`fn visit_i64_atomic_rmw32_sub_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-rmw-and"></span>`fn visit_i32_atomic_rmw_and(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw-and"></span>`fn visit_i64_atomic_rmw_and(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-rmw8-and-u"></span>`fn visit_i32_atomic_rmw8_and_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-rmw16-and-u"></span>`fn visit_i32_atomic_rmw16_and_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw8-and-u"></span>`fn visit_i64_atomic_rmw8_and_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw16-and-u"></span>`fn visit_i64_atomic_rmw16_and_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw32-and-u"></span>`fn visit_i64_atomic_rmw32_and_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-rmw-or"></span>`fn visit_i32_atomic_rmw_or(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw-or"></span>`fn visit_i64_atomic_rmw_or(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-rmw8-or-u"></span>`fn visit_i32_atomic_rmw8_or_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-rmw16-or-u"></span>`fn visit_i32_atomic_rmw16_or_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw8-or-u"></span>`fn visit_i64_atomic_rmw8_or_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw16-or-u"></span>`fn visit_i64_atomic_rmw16_or_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw32-or-u"></span>`fn visit_i64_atomic_rmw32_or_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-rmw-xor"></span>`fn visit_i32_atomic_rmw_xor(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw-xor"></span>`fn visit_i64_atomic_rmw_xor(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-rmw8-xor-u"></span>`fn visit_i32_atomic_rmw8_xor_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-rmw16-xor-u"></span>`fn visit_i32_atomic_rmw16_xor_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw8-xor-u"></span>`fn visit_i64_atomic_rmw8_xor_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw16-xor-u"></span>`fn visit_i64_atomic_rmw16_xor_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw32-xor-u"></span>`fn visit_i64_atomic_rmw32_xor_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-rmw-xchg"></span>`fn visit_i32_atomic_rmw_xchg(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw-xchg"></span>`fn visit_i64_atomic_rmw_xchg(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-rmw8-xchg-u"></span>`fn visit_i32_atomic_rmw8_xchg_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-rmw16-xchg-u"></span>`fn visit_i32_atomic_rmw16_xchg_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw8-xchg-u"></span>`fn visit_i64_atomic_rmw8_xchg_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw16-xchg-u"></span>`fn visit_i64_atomic_rmw16_xchg_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw32-xchg-u"></span>`fn visit_i64_atomic_rmw32_xchg_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-rmw-cmpxchg"></span>`fn visit_i32_atomic_rmw_cmpxchg(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw-cmpxchg"></span>`fn visit_i64_atomic_rmw_cmpxchg(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-rmw8-cmpxchg-u"></span>`fn visit_i32_atomic_rmw8_cmpxchg_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i32-atomic-rmw16-cmpxchg-u"></span>`fn visit_i32_atomic_rmw16_cmpxchg_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw8-cmpxchg-u"></span>`fn visit_i64_atomic_rmw8_cmpxchg_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw16-cmpxchg-u"></span>`fn visit_i64_atomic_rmw16_cmpxchg_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-atomic-rmw32-cmpxchg-u"></span>`fn visit_i64_atomic_rmw32_cmpxchg_u(&mut self, memarg: MemArg) -> <T as >::Output` — [`MemArg`](../index.md#memarg), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-try-table"></span>`fn visit_try_table(&mut self, try_table: TryTable) -> <T as >::Output` — [`TryTable`](../index.md#trytable), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-throw"></span>`fn visit_throw(&mut self, tag_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-throw-ref"></span>`fn visit_throw_ref(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-try"></span>`fn visit_try(&mut self, blockty: BlockType) -> <T as >::Output` — [`BlockType`](../index.md#blocktype), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-catch"></span>`fn visit_catch(&mut self, tag_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-rethrow"></span>`fn visit_rethrow(&mut self, relative_depth: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-delegate"></span>`fn visit_delegate(&mut self, relative_depth: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-catch-all"></span>`fn visit_catch_all(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-global-atomic-get"></span>`fn visit_global_atomic_get(&mut self, ordering: Ordering, global_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-global-atomic-set"></span>`fn visit_global_atomic_set(&mut self, ordering: Ordering, global_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-global-atomic-rmw-add"></span>`fn visit_global_atomic_rmw_add(&mut self, ordering: Ordering, global_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-global-atomic-rmw-sub"></span>`fn visit_global_atomic_rmw_sub(&mut self, ordering: Ordering, global_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-global-atomic-rmw-and"></span>`fn visit_global_atomic_rmw_and(&mut self, ordering: Ordering, global_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-global-atomic-rmw-or"></span>`fn visit_global_atomic_rmw_or(&mut self, ordering: Ordering, global_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-global-atomic-rmw-xor"></span>`fn visit_global_atomic_rmw_xor(&mut self, ordering: Ordering, global_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-global-atomic-rmw-xchg"></span>`fn visit_global_atomic_rmw_xchg(&mut self, ordering: Ordering, global_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-global-atomic-rmw-cmpxchg"></span>`fn visit_global_atomic_rmw_cmpxchg(&mut self, ordering: Ordering, global_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-table-atomic-get"></span>`fn visit_table_atomic_get(&mut self, ordering: Ordering, table_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-table-atomic-set"></span>`fn visit_table_atomic_set(&mut self, ordering: Ordering, table_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-table-atomic-rmw-xchg"></span>`fn visit_table_atomic_rmw_xchg(&mut self, ordering: Ordering, table_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-table-atomic-rmw-cmpxchg"></span>`fn visit_table_atomic_rmw_cmpxchg(&mut self, ordering: Ordering, table_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-struct-atomic-get"></span>`fn visit_struct_atomic_get(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-struct-atomic-get-s"></span>`fn visit_struct_atomic_get_s(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-struct-atomic-get-u"></span>`fn visit_struct_atomic_get_u(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-struct-atomic-set"></span>`fn visit_struct_atomic_set(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-struct-atomic-rmw-add"></span>`fn visit_struct_atomic_rmw_add(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-struct-atomic-rmw-sub"></span>`fn visit_struct_atomic_rmw_sub(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-struct-atomic-rmw-and"></span>`fn visit_struct_atomic_rmw_and(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-struct-atomic-rmw-or"></span>`fn visit_struct_atomic_rmw_or(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-struct-atomic-rmw-xor"></span>`fn visit_struct_atomic_rmw_xor(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-struct-atomic-rmw-xchg"></span>`fn visit_struct_atomic_rmw_xchg(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-struct-atomic-rmw-cmpxchg"></span>`fn visit_struct_atomic_rmw_cmpxchg(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-array-atomic-get"></span>`fn visit_array_atomic_get(&mut self, ordering: Ordering, array_type_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-array-atomic-get-s"></span>`fn visit_array_atomic_get_s(&mut self, ordering: Ordering, array_type_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-array-atomic-get-u"></span>`fn visit_array_atomic_get_u(&mut self, ordering: Ordering, array_type_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-array-atomic-set"></span>`fn visit_array_atomic_set(&mut self, ordering: Ordering, array_type_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-array-atomic-rmw-add"></span>`fn visit_array_atomic_rmw_add(&mut self, ordering: Ordering, array_type_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-array-atomic-rmw-sub"></span>`fn visit_array_atomic_rmw_sub(&mut self, ordering: Ordering, array_type_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-array-atomic-rmw-and"></span>`fn visit_array_atomic_rmw_and(&mut self, ordering: Ordering, array_type_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-array-atomic-rmw-or"></span>`fn visit_array_atomic_rmw_or(&mut self, ordering: Ordering, array_type_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-array-atomic-rmw-xor"></span>`fn visit_array_atomic_rmw_xor(&mut self, ordering: Ordering, array_type_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-array-atomic-rmw-xchg"></span>`fn visit_array_atomic_rmw_xchg(&mut self, ordering: Ordering, array_type_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-array-atomic-rmw-cmpxchg"></span>`fn visit_array_atomic_rmw_cmpxchg(&mut self, ordering: Ordering, array_type_index: u32) -> <T as >::Output` — [`Ordering`](../index.md#ordering), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-ref-i31-shared"></span>`fn visit_ref_i31_shared(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-call-ref"></span>`fn visit_call_ref(&mut self, type_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-return-call-ref"></span>`fn visit_return_call_ref(&mut self, type_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-ref-as-non-null"></span>`fn visit_ref_as_non_null(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-br-on-null"></span>`fn visit_br_on_null(&mut self, relative_depth: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-br-on-non-null"></span>`fn visit_br_on_non_null(&mut self, relative_depth: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-cont-new"></span>`fn visit_cont_new(&mut self, cont_type_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-cont-bind"></span>`fn visit_cont_bind(&mut self, argument_index: u32, result_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-suspend"></span>`fn visit_suspend(&mut self, tag_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-resume"></span>`fn visit_resume(&mut self, cont_type_index: u32, resume_table: ResumeTable) -> <T as >::Output` — [`ResumeTable`](../index.md#resumetable), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-resume-throw"></span>`fn visit_resume_throw(&mut self, cont_type_index: u32, tag_index: u32, resume_table: ResumeTable) -> <T as >::Output` — [`ResumeTable`](../index.md#resumetable), [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-switch"></span>`fn visit_switch(&mut self, cont_type_index: u32, tag_index: u32) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-add128"></span>`fn visit_i64_add128(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-sub128"></span>`fn visit_i64_sub128(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-mul-wide-s"></span>`fn visit_i64_mul_wide_s(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

- <span id="singleframeadapter-visitoperator-visit-i64-mul-wide-u"></span>`fn visit_i64_mul_wide_u(&mut self) -> <T as >::Output` — [`VisitOperator`](../index.md#visitoperator)

### `OperatorsReader<'a>`

```rust
struct OperatorsReader<'a> {
    reader: crate::BinaryReader<'a>,
    stack: ControlStack,
}
```

A reader for a core WebAssembly function's operators. The [`OperatorsReader`](../index.md) internally
maintains a stack of the kinds of frames within an expression or function body.
This is necessary to enforce the syntactic requirements of the binary format.
The BinaryReader can also be used to read the operators by providing an external [`FrameStack`](../index.md) instance.

#### Implementations

- <span id="operatorsreader-new"></span>`fn new(reader: BinaryReader<'a>) -> Self` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader)

  Creates a new reader for an expression (instruction sequence).

  

  This method, in conjunction with `OperatorsReader::into_allocations`,

  provides a means to reuse allocations across reading each

  individual expression or function body. Note that it is also sufficient

  to call this method with `Default::default()` if no prior allocations are

  available.

- <span id="operatorsreader-new-with-allocs"></span>`fn new_with_allocs(reader: BinaryReader<'a>, allocs: OperatorsReaderAllocations) -> Self` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`OperatorsReaderAllocations`](../index.md#operatorsreaderallocations)

  Same as `OperatorsReader::new` except that the

  [`OperatorsReaderAllocations`](../index.md) can be specified here to amortize the

  cost of them over multiple readers.

- <span id="operatorsreader-get-binary-reader"></span>`fn get_binary_reader(&self) -> BinaryReader<'a>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader)

  Get binary reader

- <span id="operatorsreader-eof"></span>`fn eof(&self) -> bool`

  Determines if the reader is at the end of the operators.

- <span id="operatorsreader-original-position"></span>`fn original_position(&self) -> usize`

  Gets the original position of the reader.

- <span id="operatorsreader-is-end-then-eof"></span>`fn is_end_then_eof(&self) -> bool`

  Returns whether there is an `end` opcode followed by eof remaining in

  this reader.

- <span id="operatorsreader-into-allocations"></span>`fn into_allocations(self) -> OperatorsReaderAllocations` — [`OperatorsReaderAllocations`](../index.md#operatorsreaderallocations)

  Consumes this reader and returns the underlying allocations that

  were used to store the frame stack.

  

  The returned value here can be paired with

  `OperatorsReader::new` to reuse the allocations already

  created by this reader.

- <span id="operatorsreader-read"></span>`fn read(&mut self) -> Result<Operator<'a>>` — [`Result`](../../../binary_reader/index.md#result), [`Operator`](../index.md#operator)

  Reads the next available `Operator`.

  

  # Errors

  

  If `OperatorsReader` has less bytes remaining than required to parse

  the `Operator`, or if the input is malformed.

- <span id="operatorsreader-visit-operator"></span>`fn visit_operator<T>(&mut self, visitor: &mut T) -> Result<<T as VisitOperator>::Output>` — [`Result`](../../../binary_reader/index.md#result), [`VisitOperator`](../index.md#visitoperator)

  Visit the next available operator with the specified [`VisitOperator`](../index.md) instance.

  

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

- <span id="operatorsreader-read-with-offset"></span>`fn read_with_offset(&mut self) -> Result<(Operator<'a>, usize)>` — [`Result`](../../../binary_reader/index.md#result), [`Operator`](../index.md#operator)

  Reads an operator with its offset.

- <span id="operatorsreader-into-iter-with-offsets"></span>`fn into_iter_with_offsets(self) -> OperatorsIteratorWithOffsets<'a>` — [`OperatorsIteratorWithOffsets`](../index.md#operatorsiteratorwithoffsets)

  Converts to an iterator of operators paired with offsets.

- <span id="operatorsreader-skip-const-expr"></span>`fn skip_const_expr(&mut self) -> Result<()>` — [`Result`](../../../binary_reader/index.md#result)

- <span id="operatorsreader-finish"></span>`fn finish(&self) -> Result<()>` — [`Result`](../../../binary_reader/index.md#result)

  Function that must be called after the last opcode has been processed.

  

  This function returns an error if there is extra data after the operators.

  It does *not* check the binary format requirement that if the data count

  section is absent, a data index may not occur in the code section.

#### Trait Implementations

##### `impl Clone for OperatorsReader<'a>`

- <span id="operatorsreader-clone"></span>`fn clone(&self) -> OperatorsReader<'a>` — [`OperatorsReader`](../index.md#operatorsreader)

##### `impl FrameStack for OperatorsReader<'a>`

- <span id="operatorsreader-framestack-current-frame"></span>`fn current_frame(&self) -> Option<FrameKind>` — [`FrameKind`](../index.md#framekind)

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

- <span id="operatorsreaderallocations-default"></span>`fn default() -> OperatorsReaderAllocations` — [`OperatorsReaderAllocations`](../index.md#operatorsreaderallocations)

### `OperatorsIterator<'a>`

```rust
struct OperatorsIterator<'a> {
    reader: OperatorsReader<'a>,
    err: bool,
}
```

An iterator over a function's operators.

#### Implementations

- <span id="operatorsiterator-into-allocations"></span>`fn into_allocations(self) -> OperatorsReaderAllocations` — [`OperatorsReaderAllocations`](../index.md#operatorsreaderallocations)

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

- <span id="operatorsiteratorwithoffsets-into-allocations"></span>`fn into_allocations(self) -> OperatorsReaderAllocations` — [`OperatorsReaderAllocations`](../index.md#operatorsreaderallocations)

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

- <span id="trytable-clone"></span>`fn clone(&self) -> TryTable` — [`TryTable`](../index.md#trytable)

##### `impl Debug for TryTable`

- <span id="trytable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for TryTable`

##### `impl FromReader for TryTable`

- <span id="trytable-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

##### `impl PartialEq for TryTable`

- <span id="trytable-partialeq-eq"></span>`fn eq(&self, other: &TryTable) -> bool` — [`TryTable`](../index.md#trytable)

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

- <span id="resumetable-clone"></span>`fn clone(&self) -> ResumeTable` — [`ResumeTable`](../index.md#resumetable)

##### `impl Debug for ResumeTable`

- <span id="resumetable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ResumeTable`

##### `impl FromReader for ResumeTable`

- <span id="resumetable-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

##### `impl PartialEq for ResumeTable`

- <span id="resumetable-partialeq-eq"></span>`fn eq(&self, other: &ResumeTable) -> bool` — [`ResumeTable`](../index.md#resumetable)

##### `impl StructuralPartialEq for ResumeTable`

### `OperatorFactory`

```rust
struct OperatorFactory;
```

A factory to construct [`Operator`](../index.md) instances via the [`VisitOperator`](../index.md) trait.

#### Trait Implementations

##### `impl VisitOperator for OperatorFactory`

- <span id="operatorfactory-visitoperator-type-output"></span>`type Output = Operator<'a>`

- <span id="operatorfactory-visitoperator-visit-unreachable"></span>`fn visit_unreachable(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-nop"></span>`fn visit_nop(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-block"></span>`fn visit_block(&mut self, blockty: BlockType) -> Operator<'a>` — [`BlockType`](../index.md#blocktype), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-loop"></span>`fn visit_loop(&mut self, blockty: BlockType) -> Operator<'a>` — [`BlockType`](../index.md#blocktype), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-if"></span>`fn visit_if(&mut self, blockty: BlockType) -> Operator<'a>` — [`BlockType`](../index.md#blocktype), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-else"></span>`fn visit_else(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-end"></span>`fn visit_end(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-br"></span>`fn visit_br(&mut self, relative_depth: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-br-if"></span>`fn visit_br_if(&mut self, relative_depth: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-br-table"></span>`fn visit_br_table(&mut self, targets: BrTable<'a>) -> Operator<'a>` — [`BrTable`](../index.md#brtable), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-return"></span>`fn visit_return(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-call"></span>`fn visit_call(&mut self, function_index: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-call-indirect"></span>`fn visit_call_indirect(&mut self, type_index: u32, table_index: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-drop"></span>`fn visit_drop(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-select"></span>`fn visit_select(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-local-get"></span>`fn visit_local_get(&mut self, local_index: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-local-set"></span>`fn visit_local_set(&mut self, local_index: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-local-tee"></span>`fn visit_local_tee(&mut self, local_index: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-global-get"></span>`fn visit_global_get(&mut self, global_index: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-global-set"></span>`fn visit_global_set(&mut self, global_index: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-load"></span>`fn visit_i32_load(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-load"></span>`fn visit_i64_load(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f32-load"></span>`fn visit_f32_load(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f64-load"></span>`fn visit_f64_load(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-load8-s"></span>`fn visit_i32_load8_s(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-load8-u"></span>`fn visit_i32_load8_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-load16-s"></span>`fn visit_i32_load16_s(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-load16-u"></span>`fn visit_i32_load16_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-load8-s"></span>`fn visit_i64_load8_s(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-load8-u"></span>`fn visit_i64_load8_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-load16-s"></span>`fn visit_i64_load16_s(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-load16-u"></span>`fn visit_i64_load16_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-load32-s"></span>`fn visit_i64_load32_s(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-load32-u"></span>`fn visit_i64_load32_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-store"></span>`fn visit_i32_store(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-store"></span>`fn visit_i64_store(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f32-store"></span>`fn visit_f32_store(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f64-store"></span>`fn visit_f64_store(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-store8"></span>`fn visit_i32_store8(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-store16"></span>`fn visit_i32_store16(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-store8"></span>`fn visit_i64_store8(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-store16"></span>`fn visit_i64_store16(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-store32"></span>`fn visit_i64_store32(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-memory-size"></span>`fn visit_memory_size(&mut self, mem: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-memory-grow"></span>`fn visit_memory_grow(&mut self, mem: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-const"></span>`fn visit_i32_const(&mut self, value: i32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-const"></span>`fn visit_i64_const(&mut self, value: i64) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f32-const"></span>`fn visit_f32_const(&mut self, value: Ieee32) -> Operator<'a>` — [`Ieee32`](../index.md#ieee32), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f64-const"></span>`fn visit_f64_const(&mut self, value: Ieee64) -> Operator<'a>` — [`Ieee64`](../index.md#ieee64), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-eqz"></span>`fn visit_i32_eqz(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-eq"></span>`fn visit_i32_eq(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-ne"></span>`fn visit_i32_ne(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-lt-s"></span>`fn visit_i32_lt_s(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-lt-u"></span>`fn visit_i32_lt_u(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-gt-s"></span>`fn visit_i32_gt_s(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-gt-u"></span>`fn visit_i32_gt_u(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-le-s"></span>`fn visit_i32_le_s(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-le-u"></span>`fn visit_i32_le_u(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-ge-s"></span>`fn visit_i32_ge_s(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-ge-u"></span>`fn visit_i32_ge_u(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-eqz"></span>`fn visit_i64_eqz(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-eq"></span>`fn visit_i64_eq(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-ne"></span>`fn visit_i64_ne(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-lt-s"></span>`fn visit_i64_lt_s(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-lt-u"></span>`fn visit_i64_lt_u(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-gt-s"></span>`fn visit_i64_gt_s(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-gt-u"></span>`fn visit_i64_gt_u(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-le-s"></span>`fn visit_i64_le_s(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-le-u"></span>`fn visit_i64_le_u(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-ge-s"></span>`fn visit_i64_ge_s(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-ge-u"></span>`fn visit_i64_ge_u(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f32-eq"></span>`fn visit_f32_eq(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f32-ne"></span>`fn visit_f32_ne(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f32-lt"></span>`fn visit_f32_lt(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f32-gt"></span>`fn visit_f32_gt(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f32-le"></span>`fn visit_f32_le(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f32-ge"></span>`fn visit_f32_ge(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f64-eq"></span>`fn visit_f64_eq(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f64-ne"></span>`fn visit_f64_ne(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f64-lt"></span>`fn visit_f64_lt(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f64-gt"></span>`fn visit_f64_gt(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f64-le"></span>`fn visit_f64_le(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f64-ge"></span>`fn visit_f64_ge(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-clz"></span>`fn visit_i32_clz(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-ctz"></span>`fn visit_i32_ctz(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-popcnt"></span>`fn visit_i32_popcnt(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-add"></span>`fn visit_i32_add(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-sub"></span>`fn visit_i32_sub(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-mul"></span>`fn visit_i32_mul(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-div-s"></span>`fn visit_i32_div_s(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-div-u"></span>`fn visit_i32_div_u(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-rem-s"></span>`fn visit_i32_rem_s(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-rem-u"></span>`fn visit_i32_rem_u(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-and"></span>`fn visit_i32_and(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-or"></span>`fn visit_i32_or(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-xor"></span>`fn visit_i32_xor(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-shl"></span>`fn visit_i32_shl(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-shr-s"></span>`fn visit_i32_shr_s(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-shr-u"></span>`fn visit_i32_shr_u(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-rotl"></span>`fn visit_i32_rotl(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-rotr"></span>`fn visit_i32_rotr(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-clz"></span>`fn visit_i64_clz(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-ctz"></span>`fn visit_i64_ctz(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-popcnt"></span>`fn visit_i64_popcnt(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-add"></span>`fn visit_i64_add(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-sub"></span>`fn visit_i64_sub(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-mul"></span>`fn visit_i64_mul(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-div-s"></span>`fn visit_i64_div_s(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-div-u"></span>`fn visit_i64_div_u(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-rem-s"></span>`fn visit_i64_rem_s(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-rem-u"></span>`fn visit_i64_rem_u(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-and"></span>`fn visit_i64_and(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-or"></span>`fn visit_i64_or(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-xor"></span>`fn visit_i64_xor(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-shl"></span>`fn visit_i64_shl(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-shr-s"></span>`fn visit_i64_shr_s(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-shr-u"></span>`fn visit_i64_shr_u(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-rotl"></span>`fn visit_i64_rotl(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-rotr"></span>`fn visit_i64_rotr(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f32-abs"></span>`fn visit_f32_abs(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f32-neg"></span>`fn visit_f32_neg(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f32-ceil"></span>`fn visit_f32_ceil(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f32-floor"></span>`fn visit_f32_floor(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f32-trunc"></span>`fn visit_f32_trunc(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f32-nearest"></span>`fn visit_f32_nearest(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f32-sqrt"></span>`fn visit_f32_sqrt(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f32-add"></span>`fn visit_f32_add(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f32-sub"></span>`fn visit_f32_sub(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f32-mul"></span>`fn visit_f32_mul(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f32-div"></span>`fn visit_f32_div(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f32-min"></span>`fn visit_f32_min(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f32-max"></span>`fn visit_f32_max(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f32-copysign"></span>`fn visit_f32_copysign(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f64-abs"></span>`fn visit_f64_abs(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f64-neg"></span>`fn visit_f64_neg(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f64-ceil"></span>`fn visit_f64_ceil(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f64-floor"></span>`fn visit_f64_floor(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f64-trunc"></span>`fn visit_f64_trunc(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f64-nearest"></span>`fn visit_f64_nearest(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f64-sqrt"></span>`fn visit_f64_sqrt(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f64-add"></span>`fn visit_f64_add(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f64-sub"></span>`fn visit_f64_sub(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f64-mul"></span>`fn visit_f64_mul(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f64-div"></span>`fn visit_f64_div(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f64-min"></span>`fn visit_f64_min(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f64-max"></span>`fn visit_f64_max(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f64-copysign"></span>`fn visit_f64_copysign(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-wrap-i64"></span>`fn visit_i32_wrap_i64(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-trunc-f32-s"></span>`fn visit_i32_trunc_f32_s(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-trunc-f32-u"></span>`fn visit_i32_trunc_f32_u(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-trunc-f64-s"></span>`fn visit_i32_trunc_f64_s(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-trunc-f64-u"></span>`fn visit_i32_trunc_f64_u(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-extend-i32-s"></span>`fn visit_i64_extend_i32_s(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-extend-i32-u"></span>`fn visit_i64_extend_i32_u(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-trunc-f32-s"></span>`fn visit_i64_trunc_f32_s(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-trunc-f32-u"></span>`fn visit_i64_trunc_f32_u(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-trunc-f64-s"></span>`fn visit_i64_trunc_f64_s(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-trunc-f64-u"></span>`fn visit_i64_trunc_f64_u(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f32-convert-i32-s"></span>`fn visit_f32_convert_i32_s(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f32-convert-i32-u"></span>`fn visit_f32_convert_i32_u(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f32-convert-i64-s"></span>`fn visit_f32_convert_i64_s(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f32-convert-i64-u"></span>`fn visit_f32_convert_i64_u(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f32-demote-f64"></span>`fn visit_f32_demote_f64(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f64-convert-i32-s"></span>`fn visit_f64_convert_i32_s(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f64-convert-i32-u"></span>`fn visit_f64_convert_i32_u(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f64-convert-i64-s"></span>`fn visit_f64_convert_i64_s(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f64-convert-i64-u"></span>`fn visit_f64_convert_i64_u(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f64-promote-f32"></span>`fn visit_f64_promote_f32(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-reinterpret-f32"></span>`fn visit_i32_reinterpret_f32(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-reinterpret-f64"></span>`fn visit_i64_reinterpret_f64(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f32-reinterpret-i32"></span>`fn visit_f32_reinterpret_i32(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-f64-reinterpret-i64"></span>`fn visit_f64_reinterpret_i64(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-extend8-s"></span>`fn visit_i32_extend8_s(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-extend16-s"></span>`fn visit_i32_extend16_s(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-extend8-s"></span>`fn visit_i64_extend8_s(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-extend16-s"></span>`fn visit_i64_extend16_s(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-extend32-s"></span>`fn visit_i64_extend32_s(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-ref-eq"></span>`fn visit_ref_eq(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-struct-new"></span>`fn visit_struct_new(&mut self, struct_type_index: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-struct-new-default"></span>`fn visit_struct_new_default(&mut self, struct_type_index: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-struct-get"></span>`fn visit_struct_get(&mut self, struct_type_index: u32, field_index: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-struct-get-s"></span>`fn visit_struct_get_s(&mut self, struct_type_index: u32, field_index: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-struct-get-u"></span>`fn visit_struct_get_u(&mut self, struct_type_index: u32, field_index: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-struct-set"></span>`fn visit_struct_set(&mut self, struct_type_index: u32, field_index: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-array-new"></span>`fn visit_array_new(&mut self, array_type_index: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-array-new-default"></span>`fn visit_array_new_default(&mut self, array_type_index: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-array-new-fixed"></span>`fn visit_array_new_fixed(&mut self, array_type_index: u32, array_size: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-array-new-data"></span>`fn visit_array_new_data(&mut self, array_type_index: u32, array_data_index: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-array-new-elem"></span>`fn visit_array_new_elem(&mut self, array_type_index: u32, array_elem_index: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-array-get"></span>`fn visit_array_get(&mut self, array_type_index: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-array-get-s"></span>`fn visit_array_get_s(&mut self, array_type_index: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-array-get-u"></span>`fn visit_array_get_u(&mut self, array_type_index: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-array-set"></span>`fn visit_array_set(&mut self, array_type_index: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-array-len"></span>`fn visit_array_len(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-array-fill"></span>`fn visit_array_fill(&mut self, array_type_index: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-array-copy"></span>`fn visit_array_copy(&mut self, array_type_index_dst: u32, array_type_index_src: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-array-init-data"></span>`fn visit_array_init_data(&mut self, array_type_index: u32, array_data_index: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-array-init-elem"></span>`fn visit_array_init_elem(&mut self, array_type_index: u32, array_elem_index: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-ref-test-non-null"></span>`fn visit_ref_test_non_null(&mut self, hty: HeapType) -> Operator<'a>` — [`HeapType`](../index.md#heaptype), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-ref-test-nullable"></span>`fn visit_ref_test_nullable(&mut self, hty: HeapType) -> Operator<'a>` — [`HeapType`](../index.md#heaptype), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-ref-cast-non-null"></span>`fn visit_ref_cast_non_null(&mut self, hty: HeapType) -> Operator<'a>` — [`HeapType`](../index.md#heaptype), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-ref-cast-nullable"></span>`fn visit_ref_cast_nullable(&mut self, hty: HeapType) -> Operator<'a>` — [`HeapType`](../index.md#heaptype), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-br-on-cast"></span>`fn visit_br_on_cast(&mut self, relative_depth: u32, from_ref_type: RefType, to_ref_type: RefType) -> Operator<'a>` — [`RefType`](../index.md#reftype), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-br-on-cast-fail"></span>`fn visit_br_on_cast_fail(&mut self, relative_depth: u32, from_ref_type: RefType, to_ref_type: RefType) -> Operator<'a>` — [`RefType`](../index.md#reftype), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-any-convert-extern"></span>`fn visit_any_convert_extern(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-extern-convert-any"></span>`fn visit_extern_convert_any(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-ref-i31"></span>`fn visit_ref_i31(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i31-get-s"></span>`fn visit_i31_get_s(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i31-get-u"></span>`fn visit_i31_get_u(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-struct-new-desc"></span>`fn visit_struct_new_desc(&mut self, struct_type_index: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-struct-new-default-desc"></span>`fn visit_struct_new_default_desc(&mut self, struct_type_index: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-ref-get-desc"></span>`fn visit_ref_get_desc(&mut self, type_index: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-ref-cast-desc-non-null"></span>`fn visit_ref_cast_desc_non_null(&mut self, hty: HeapType) -> Operator<'a>` — [`HeapType`](../index.md#heaptype), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-ref-cast-desc-nullable"></span>`fn visit_ref_cast_desc_nullable(&mut self, hty: HeapType) -> Operator<'a>` — [`HeapType`](../index.md#heaptype), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-br-on-cast-desc"></span>`fn visit_br_on_cast_desc(&mut self, relative_depth: u32, from_ref_type: RefType, to_ref_type: RefType) -> Operator<'a>` — [`RefType`](../index.md#reftype), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-br-on-cast-desc-fail"></span>`fn visit_br_on_cast_desc_fail(&mut self, relative_depth: u32, from_ref_type: RefType, to_ref_type: RefType) -> Operator<'a>` — [`RefType`](../index.md#reftype), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-trunc-sat-f32-s"></span>`fn visit_i32_trunc_sat_f32_s(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-trunc-sat-f32-u"></span>`fn visit_i32_trunc_sat_f32_u(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-trunc-sat-f64-s"></span>`fn visit_i32_trunc_sat_f64_s(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-trunc-sat-f64-u"></span>`fn visit_i32_trunc_sat_f64_u(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-trunc-sat-f32-s"></span>`fn visit_i64_trunc_sat_f32_s(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-trunc-sat-f32-u"></span>`fn visit_i64_trunc_sat_f32_u(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-trunc-sat-f64-s"></span>`fn visit_i64_trunc_sat_f64_s(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-trunc-sat-f64-u"></span>`fn visit_i64_trunc_sat_f64_u(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-memory-init"></span>`fn visit_memory_init(&mut self, data_index: u32, mem: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-data-drop"></span>`fn visit_data_drop(&mut self, data_index: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-memory-copy"></span>`fn visit_memory_copy(&mut self, dst_mem: u32, src_mem: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-memory-fill"></span>`fn visit_memory_fill(&mut self, mem: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-table-init"></span>`fn visit_table_init(&mut self, elem_index: u32, table: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-elem-drop"></span>`fn visit_elem_drop(&mut self, elem_index: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-table-copy"></span>`fn visit_table_copy(&mut self, dst_table: u32, src_table: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-typed-select"></span>`fn visit_typed_select(&mut self, ty: ValType) -> Operator<'a>` — [`ValType`](../index.md#valtype), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-typed-select-multi"></span>`fn visit_typed_select_multi(&mut self, tys: Vec<ValType>) -> Operator<'a>` — [`Vec`](../../../prelude/index.md#vec), [`ValType`](../index.md#valtype), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-ref-null"></span>`fn visit_ref_null(&mut self, hty: HeapType) -> Operator<'a>` — [`HeapType`](../index.md#heaptype), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-ref-is-null"></span>`fn visit_ref_is_null(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-ref-func"></span>`fn visit_ref_func(&mut self, function_index: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-table-fill"></span>`fn visit_table_fill(&mut self, table: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-table-get"></span>`fn visit_table_get(&mut self, table: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-table-set"></span>`fn visit_table_set(&mut self, table: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-table-grow"></span>`fn visit_table_grow(&mut self, table: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-table-size"></span>`fn visit_table_size(&mut self, table: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-return-call"></span>`fn visit_return_call(&mut self, function_index: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-return-call-indirect"></span>`fn visit_return_call_indirect(&mut self, type_index: u32, table_index: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-memory-discard"></span>`fn visit_memory_discard(&mut self, mem: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-memory-atomic-notify"></span>`fn visit_memory_atomic_notify(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-memory-atomic-wait32"></span>`fn visit_memory_atomic_wait32(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-memory-atomic-wait64"></span>`fn visit_memory_atomic_wait64(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-atomic-fence"></span>`fn visit_atomic_fence(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-load"></span>`fn visit_i32_atomic_load(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-load"></span>`fn visit_i64_atomic_load(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-load8-u"></span>`fn visit_i32_atomic_load8_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-load16-u"></span>`fn visit_i32_atomic_load16_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-load8-u"></span>`fn visit_i64_atomic_load8_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-load16-u"></span>`fn visit_i64_atomic_load16_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-load32-u"></span>`fn visit_i64_atomic_load32_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-store"></span>`fn visit_i32_atomic_store(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-store"></span>`fn visit_i64_atomic_store(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-store8"></span>`fn visit_i32_atomic_store8(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-store16"></span>`fn visit_i32_atomic_store16(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-store8"></span>`fn visit_i64_atomic_store8(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-store16"></span>`fn visit_i64_atomic_store16(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-store32"></span>`fn visit_i64_atomic_store32(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-rmw-add"></span>`fn visit_i32_atomic_rmw_add(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw-add"></span>`fn visit_i64_atomic_rmw_add(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-rmw8-add-u"></span>`fn visit_i32_atomic_rmw8_add_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-rmw16-add-u"></span>`fn visit_i32_atomic_rmw16_add_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw8-add-u"></span>`fn visit_i64_atomic_rmw8_add_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw16-add-u"></span>`fn visit_i64_atomic_rmw16_add_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw32-add-u"></span>`fn visit_i64_atomic_rmw32_add_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-rmw-sub"></span>`fn visit_i32_atomic_rmw_sub(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw-sub"></span>`fn visit_i64_atomic_rmw_sub(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-rmw8-sub-u"></span>`fn visit_i32_atomic_rmw8_sub_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-rmw16-sub-u"></span>`fn visit_i32_atomic_rmw16_sub_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw8-sub-u"></span>`fn visit_i64_atomic_rmw8_sub_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw16-sub-u"></span>`fn visit_i64_atomic_rmw16_sub_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw32-sub-u"></span>`fn visit_i64_atomic_rmw32_sub_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-rmw-and"></span>`fn visit_i32_atomic_rmw_and(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw-and"></span>`fn visit_i64_atomic_rmw_and(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-rmw8-and-u"></span>`fn visit_i32_atomic_rmw8_and_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-rmw16-and-u"></span>`fn visit_i32_atomic_rmw16_and_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw8-and-u"></span>`fn visit_i64_atomic_rmw8_and_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw16-and-u"></span>`fn visit_i64_atomic_rmw16_and_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw32-and-u"></span>`fn visit_i64_atomic_rmw32_and_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-rmw-or"></span>`fn visit_i32_atomic_rmw_or(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw-or"></span>`fn visit_i64_atomic_rmw_or(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-rmw8-or-u"></span>`fn visit_i32_atomic_rmw8_or_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-rmw16-or-u"></span>`fn visit_i32_atomic_rmw16_or_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw8-or-u"></span>`fn visit_i64_atomic_rmw8_or_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw16-or-u"></span>`fn visit_i64_atomic_rmw16_or_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw32-or-u"></span>`fn visit_i64_atomic_rmw32_or_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-rmw-xor"></span>`fn visit_i32_atomic_rmw_xor(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw-xor"></span>`fn visit_i64_atomic_rmw_xor(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-rmw8-xor-u"></span>`fn visit_i32_atomic_rmw8_xor_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-rmw16-xor-u"></span>`fn visit_i32_atomic_rmw16_xor_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw8-xor-u"></span>`fn visit_i64_atomic_rmw8_xor_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw16-xor-u"></span>`fn visit_i64_atomic_rmw16_xor_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw32-xor-u"></span>`fn visit_i64_atomic_rmw32_xor_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-rmw-xchg"></span>`fn visit_i32_atomic_rmw_xchg(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw-xchg"></span>`fn visit_i64_atomic_rmw_xchg(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-rmw8-xchg-u"></span>`fn visit_i32_atomic_rmw8_xchg_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-rmw16-xchg-u"></span>`fn visit_i32_atomic_rmw16_xchg_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw8-xchg-u"></span>`fn visit_i64_atomic_rmw8_xchg_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw16-xchg-u"></span>`fn visit_i64_atomic_rmw16_xchg_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw32-xchg-u"></span>`fn visit_i64_atomic_rmw32_xchg_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-rmw-cmpxchg"></span>`fn visit_i32_atomic_rmw_cmpxchg(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw-cmpxchg"></span>`fn visit_i64_atomic_rmw_cmpxchg(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-rmw8-cmpxchg-u"></span>`fn visit_i32_atomic_rmw8_cmpxchg_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i32-atomic-rmw16-cmpxchg-u"></span>`fn visit_i32_atomic_rmw16_cmpxchg_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw8-cmpxchg-u"></span>`fn visit_i64_atomic_rmw8_cmpxchg_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw16-cmpxchg-u"></span>`fn visit_i64_atomic_rmw16_cmpxchg_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-atomic-rmw32-cmpxchg-u"></span>`fn visit_i64_atomic_rmw32_cmpxchg_u(&mut self, memarg: MemArg) -> Operator<'a>` — [`MemArg`](../index.md#memarg), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-try-table"></span>`fn visit_try_table(&mut self, try_table: TryTable) -> Operator<'a>` — [`TryTable`](../index.md#trytable), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-throw"></span>`fn visit_throw(&mut self, tag_index: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-throw-ref"></span>`fn visit_throw_ref(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-try"></span>`fn visit_try(&mut self, blockty: BlockType) -> Operator<'a>` — [`BlockType`](../index.md#blocktype), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-catch"></span>`fn visit_catch(&mut self, tag_index: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-rethrow"></span>`fn visit_rethrow(&mut self, relative_depth: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-delegate"></span>`fn visit_delegate(&mut self, relative_depth: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-catch-all"></span>`fn visit_catch_all(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-global-atomic-get"></span>`fn visit_global_atomic_get(&mut self, ordering: Ordering, global_index: u32) -> Operator<'a>` — [`Ordering`](../index.md#ordering), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-global-atomic-set"></span>`fn visit_global_atomic_set(&mut self, ordering: Ordering, global_index: u32) -> Operator<'a>` — [`Ordering`](../index.md#ordering), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-global-atomic-rmw-add"></span>`fn visit_global_atomic_rmw_add(&mut self, ordering: Ordering, global_index: u32) -> Operator<'a>` — [`Ordering`](../index.md#ordering), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-global-atomic-rmw-sub"></span>`fn visit_global_atomic_rmw_sub(&mut self, ordering: Ordering, global_index: u32) -> Operator<'a>` — [`Ordering`](../index.md#ordering), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-global-atomic-rmw-and"></span>`fn visit_global_atomic_rmw_and(&mut self, ordering: Ordering, global_index: u32) -> Operator<'a>` — [`Ordering`](../index.md#ordering), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-global-atomic-rmw-or"></span>`fn visit_global_atomic_rmw_or(&mut self, ordering: Ordering, global_index: u32) -> Operator<'a>` — [`Ordering`](../index.md#ordering), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-global-atomic-rmw-xor"></span>`fn visit_global_atomic_rmw_xor(&mut self, ordering: Ordering, global_index: u32) -> Operator<'a>` — [`Ordering`](../index.md#ordering), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-global-atomic-rmw-xchg"></span>`fn visit_global_atomic_rmw_xchg(&mut self, ordering: Ordering, global_index: u32) -> Operator<'a>` — [`Ordering`](../index.md#ordering), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-global-atomic-rmw-cmpxchg"></span>`fn visit_global_atomic_rmw_cmpxchg(&mut self, ordering: Ordering, global_index: u32) -> Operator<'a>` — [`Ordering`](../index.md#ordering), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-table-atomic-get"></span>`fn visit_table_atomic_get(&mut self, ordering: Ordering, table_index: u32) -> Operator<'a>` — [`Ordering`](../index.md#ordering), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-table-atomic-set"></span>`fn visit_table_atomic_set(&mut self, ordering: Ordering, table_index: u32) -> Operator<'a>` — [`Ordering`](../index.md#ordering), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-table-atomic-rmw-xchg"></span>`fn visit_table_atomic_rmw_xchg(&mut self, ordering: Ordering, table_index: u32) -> Operator<'a>` — [`Ordering`](../index.md#ordering), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-table-atomic-rmw-cmpxchg"></span>`fn visit_table_atomic_rmw_cmpxchg(&mut self, ordering: Ordering, table_index: u32) -> Operator<'a>` — [`Ordering`](../index.md#ordering), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-struct-atomic-get"></span>`fn visit_struct_atomic_get(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> Operator<'a>` — [`Ordering`](../index.md#ordering), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-struct-atomic-get-s"></span>`fn visit_struct_atomic_get_s(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> Operator<'a>` — [`Ordering`](../index.md#ordering), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-struct-atomic-get-u"></span>`fn visit_struct_atomic_get_u(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> Operator<'a>` — [`Ordering`](../index.md#ordering), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-struct-atomic-set"></span>`fn visit_struct_atomic_set(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> Operator<'a>` — [`Ordering`](../index.md#ordering), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-struct-atomic-rmw-add"></span>`fn visit_struct_atomic_rmw_add(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> Operator<'a>` — [`Ordering`](../index.md#ordering), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-struct-atomic-rmw-sub"></span>`fn visit_struct_atomic_rmw_sub(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> Operator<'a>` — [`Ordering`](../index.md#ordering), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-struct-atomic-rmw-and"></span>`fn visit_struct_atomic_rmw_and(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> Operator<'a>` — [`Ordering`](../index.md#ordering), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-struct-atomic-rmw-or"></span>`fn visit_struct_atomic_rmw_or(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> Operator<'a>` — [`Ordering`](../index.md#ordering), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-struct-atomic-rmw-xor"></span>`fn visit_struct_atomic_rmw_xor(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> Operator<'a>` — [`Ordering`](../index.md#ordering), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-struct-atomic-rmw-xchg"></span>`fn visit_struct_atomic_rmw_xchg(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> Operator<'a>` — [`Ordering`](../index.md#ordering), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-struct-atomic-rmw-cmpxchg"></span>`fn visit_struct_atomic_rmw_cmpxchg(&mut self, ordering: Ordering, struct_type_index: u32, field_index: u32) -> Operator<'a>` — [`Ordering`](../index.md#ordering), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-array-atomic-get"></span>`fn visit_array_atomic_get(&mut self, ordering: Ordering, array_type_index: u32) -> Operator<'a>` — [`Ordering`](../index.md#ordering), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-array-atomic-get-s"></span>`fn visit_array_atomic_get_s(&mut self, ordering: Ordering, array_type_index: u32) -> Operator<'a>` — [`Ordering`](../index.md#ordering), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-array-atomic-get-u"></span>`fn visit_array_atomic_get_u(&mut self, ordering: Ordering, array_type_index: u32) -> Operator<'a>` — [`Ordering`](../index.md#ordering), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-array-atomic-set"></span>`fn visit_array_atomic_set(&mut self, ordering: Ordering, array_type_index: u32) -> Operator<'a>` — [`Ordering`](../index.md#ordering), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-array-atomic-rmw-add"></span>`fn visit_array_atomic_rmw_add(&mut self, ordering: Ordering, array_type_index: u32) -> Operator<'a>` — [`Ordering`](../index.md#ordering), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-array-atomic-rmw-sub"></span>`fn visit_array_atomic_rmw_sub(&mut self, ordering: Ordering, array_type_index: u32) -> Operator<'a>` — [`Ordering`](../index.md#ordering), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-array-atomic-rmw-and"></span>`fn visit_array_atomic_rmw_and(&mut self, ordering: Ordering, array_type_index: u32) -> Operator<'a>` — [`Ordering`](../index.md#ordering), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-array-atomic-rmw-or"></span>`fn visit_array_atomic_rmw_or(&mut self, ordering: Ordering, array_type_index: u32) -> Operator<'a>` — [`Ordering`](../index.md#ordering), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-array-atomic-rmw-xor"></span>`fn visit_array_atomic_rmw_xor(&mut self, ordering: Ordering, array_type_index: u32) -> Operator<'a>` — [`Ordering`](../index.md#ordering), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-array-atomic-rmw-xchg"></span>`fn visit_array_atomic_rmw_xchg(&mut self, ordering: Ordering, array_type_index: u32) -> Operator<'a>` — [`Ordering`](../index.md#ordering), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-array-atomic-rmw-cmpxchg"></span>`fn visit_array_atomic_rmw_cmpxchg(&mut self, ordering: Ordering, array_type_index: u32) -> Operator<'a>` — [`Ordering`](../index.md#ordering), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-ref-i31-shared"></span>`fn visit_ref_i31_shared(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-call-ref"></span>`fn visit_call_ref(&mut self, type_index: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-return-call-ref"></span>`fn visit_return_call_ref(&mut self, type_index: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-ref-as-non-null"></span>`fn visit_ref_as_non_null(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-br-on-null"></span>`fn visit_br_on_null(&mut self, relative_depth: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-br-on-non-null"></span>`fn visit_br_on_non_null(&mut self, relative_depth: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-cont-new"></span>`fn visit_cont_new(&mut self, cont_type_index: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-cont-bind"></span>`fn visit_cont_bind(&mut self, argument_index: u32, result_index: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-suspend"></span>`fn visit_suspend(&mut self, tag_index: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-resume"></span>`fn visit_resume(&mut self, cont_type_index: u32, resume_table: ResumeTable) -> Operator<'a>` — [`ResumeTable`](../index.md#resumetable), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-resume-throw"></span>`fn visit_resume_throw(&mut self, cont_type_index: u32, tag_index: u32, resume_table: ResumeTable) -> Operator<'a>` — [`ResumeTable`](../index.md#resumetable), [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-switch"></span>`fn visit_switch(&mut self, cont_type_index: u32, tag_index: u32) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-add128"></span>`fn visit_i64_add128(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-sub128"></span>`fn visit_i64_sub128(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-mul-wide-s"></span>`fn visit_i64_mul_wide_s(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

- <span id="operatorfactory-visitoperator-visit-i64-mul-wide-u"></span>`fn visit_i64_mul_wide_u(&mut self) -> Operator<'a>` — [`Operator`](../index.md#operator)

## Enums

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

- <span id="blocktype-clone"></span>`fn clone(&self) -> BlockType` — [`BlockType`](../index.md#blocktype)

##### `impl Copy for BlockType`

##### `impl Debug for BlockType`

- <span id="blocktype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for BlockType`

##### `impl PartialEq for BlockType`

- <span id="blocktype-partialeq-eq"></span>`fn eq(&self, other: &BlockType) -> bool` — [`BlockType`](../index.md#blocktype)

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

- <span id="framekind-clone"></span>`fn clone(&self) -> FrameKind` — [`FrameKind`](../index.md#framekind)

##### `impl Copy for FrameKind`

##### `impl Debug for FrameKind`

- <span id="framekind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for FrameKind`

##### `impl PartialEq for FrameKind`

- <span id="framekind-partialeq-eq"></span>`fn eq(&self, other: &FrameKind) -> bool` — [`FrameKind`](../index.md#framekind)

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

- <span id="ordering-clone"></span>`fn clone(&self) -> Ordering` — [`Ordering`](../index.md#ordering)

##### `impl Copy for Ordering`

##### `impl Debug for Ordering`

- <span id="ordering-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Ordering`

##### `impl Hash for Ordering`

- <span id="ordering-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Ordering`

- <span id="ordering-partialeq-eq"></span>`fn eq(&self, other: &Ordering) -> bool` — [`Ordering`](../index.md#ordering)

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

- <span id="crateoperator-operator-arity"></span>`fn operator_arity(&self, module: &impl ModuleArity) -> Option<(u32, u32)>` — [`ModuleArity`](../../../index.md#modulearity)

  Compute the arity (param and result counts) of the operator, given

  an impl ModuleArity, which stores the necessary module state.

#### Trait Implementations

##### `impl Clone for Operator<'a>`

- <span id="operator-clone"></span>`fn clone(&self) -> Operator<'a>` — [`Operator`](../index.md#operator)

##### `impl Debug for Operator<'a>`

- <span id="operator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Operator<'a>`

##### `impl PartialEq for Operator<'a>`

- <span id="operator-partialeq-eq"></span>`fn eq(&self, other: &Operator<'a>) -> bool` — [`Operator`](../index.md#operator)

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

Catch clauses that can be specified in [`TryTable`](../index.md).

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

- <span id="catch-clone"></span>`fn clone(&self) -> Catch` — [`Catch`](../index.md#catch)

##### `impl Copy for Catch`

##### `impl Debug for Catch`

- <span id="catch-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Catch`

##### `impl FromReader for Catch`

- <span id="catch-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

##### `impl PartialEq for Catch`

- <span id="catch-partialeq-eq"></span>`fn eq(&self, other: &Catch) -> bool` — [`Catch`](../index.md#catch)

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

Handle clauses that can be specified in [`ResumeTable`](../index.md).

#### Variants

- **`OnLabel`**

  Equivalent of `(on $tag $lbl)`.

- **`OnSwitch`**

  Equivalent of `(on $tag switch)`.

#### Trait Implementations

##### `impl Clone for Handle`

- <span id="handle-clone"></span>`fn clone(&self) -> Handle` — [`Handle`](../index.md#handle)

##### `impl Copy for Handle`

##### `impl Debug for Handle`

- <span id="handle-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Handle`

##### `impl FromReader for Handle`

- <span id="handle-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

##### `impl PartialEq for Handle`

- <span id="handle-partialeq-eq"></span>`fn eq(&self, other: &Handle) -> bool` — [`Handle`](../index.md#handle)

##### `impl StructuralPartialEq for Handle`

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

- [`FrameStackAdapter`](#framestackadapter)
- [`OperatorsReader`](../index.md#operatorsreader)
- [`SingleFrameAdapter`](#singleframeadapter)

### `VisitOperator<'a>`

```rust
trait VisitOperator<'a> { ... }
```

Trait implemented by types that can visit all [`Operator`](../index.md) variants.

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

  Visits the [`Operator`](../index.md) `op` using the given `offset`.

#### Implementors

- [`Box`](../../../prelude/index.md#box)
- [`FrameStackAdapter`](#framestackadapter)
- [`OperatorFactory`](#operatorfactory)
- [`SingleFrameAdapter`](#singleframeadapter)
- `&'b mut V`

## Macros

### `define_operator!`

### `define_visit_operator!`

### `define_visit_operator_delegate!`

### `define_visit_operator!`

### `define_visit_operator_stack_adapter!`

### `define_passthrough_visit_operator!`

