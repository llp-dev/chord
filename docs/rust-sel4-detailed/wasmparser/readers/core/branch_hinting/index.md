*[wasmparser](../../../index.md) / [readers](../../index.md) / [core](../index.md) / [branch_hinting](index.md)*

---

# Module `branch_hinting`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`BranchHintFunction`](#branchhintfunction) | struct | Branch hints for a single function. |
| [`BranchHint`](#branchhint) | struct | A hint for a single branch. |
| [`BranchHintSectionReader`](#branchhintsectionreader) | type | A reader for the `metadata.code.branch_hint` custom section. |

## Structs

### `BranchHintFunction<'a>`

```rust
struct BranchHintFunction<'a> {
    pub func: u32,
    pub hints: crate::SectionLimited<'a, BranchHint>,
}
```

Branch hints for a single function.

Produced from [`BranchHintSectionReader`](../index.md).

#### Fields

- **`func`**: `u32`

  The function that these branch hints apply to.

- **`hints`**: `crate::SectionLimited<'a, BranchHint>`

  The branch hints available for this function.

#### Trait Implementations

##### `impl Clone for BranchHintFunction<'a>`

- <span id="branchhintfunction-clone"></span>`fn clone(&self) -> BranchHintFunction<'a>` — [`BranchHintFunction`](../index.md#branchhintfunction)

##### `impl Debug for BranchHintFunction<'a>`

- <span id="branchhintfunction-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FromReader for BranchHintFunction<'a>`

- <span id="branchhintfunction-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

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

- <span id="branchhint-clone"></span>`fn clone(&self) -> BranchHint` — [`BranchHint`](../index.md#branchhint)

##### `impl Copy for BranchHint`

##### `impl Debug for BranchHint`

- <span id="branchhint-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FromReader for BranchHint`

- <span id="branchhint-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

## Type Aliases

### `BranchHintSectionReader<'a>`

```rust
type BranchHintSectionReader<'a> = crate::SectionLimited<'a, BranchHintFunction<'a>>;
```

A reader for the `metadata.code.branch_hint` custom section.

