*[ruzstd](../../index.md) / [fse](../index.md) / [fse_encoder](index.md)*

---

# Module `fse_encoder`

## Contents

- [Structs](#structs)
  - [`FSEEncoder`](#fseencoder)
  - [`FSETable`](#fsetable)
  - [`SymbolStates`](#symbolstates)
  - [`State`](#state)
- [Functions](#functions)
  - [`build_table_from_data`](#build-table-from-data)
  - [`build_table_from_counts`](#build-table-from-counts)
  - [`build_table_from_probabilities`](#build-table-from-probabilities)
  - [`next_position`](#next-position)
  - [`default_ml_table`](#default-ml-table)
  - [`default_ll_table`](#default-ll-table)
  - [`default_of_table`](#default-of-table)
- [Constants](#constants)
  - [`ML_DIST`](#ml-dist)
  - [`LL_DIST`](#ll-dist)
  - [`OF_DIST`](#of-dist)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FSEEncoder`](#fseencoder) | struct |  |
| [`FSETable`](#fsetable) | struct |  |
| [`SymbolStates`](#symbolstates) | struct |  |
| [`State`](#state) | struct |  |
| [`build_table_from_data`](#build-table-from-data) | fn |  |
| [`build_table_from_counts`](#build-table-from-counts) | fn |  |
| [`build_table_from_probabilities`](#build-table-from-probabilities) | fn |  |
| [`next_position`](#next-position) | fn | Calculate the position of the next entry of the table given the current position and size of the table. |
| [`default_ml_table`](#default-ml-table) | fn |  |
| [`default_ll_table`](#default-ll-table) | fn |  |
| [`default_of_table`](#default-of-table) | fn |  |
| [`ML_DIST`](#ml-dist) | const |  |
| [`LL_DIST`](#ll-dist) | const |  |
| [`OF_DIST`](#of-dist) | const |  |

## Structs

### `FSEEncoder<'output, V: AsMut<alloc::vec::Vec<u8>>>`

```rust
struct FSEEncoder<'output, V: AsMut<alloc::vec::Vec<u8>>> {
    table: FSETable,
    writer: &'output mut crate::bit_io::BitWriter<V>,
}
```

#### Implementations

- <span id="fseencoder-new"></span>`fn new(table: FSETable, writer: &mut BitWriter<V>) -> FSEEncoder<'_, V>` — [`FSETable`](#fsetable), [`BitWriter`](../../bit_io/bit_writer/index.md#bitwriter), [`FSEEncoder`](#fseencoder)

- <span id="fseencoder-encode-interleaved"></span>`fn encode_interleaved(&mut self, data: &[u8])`

  Encodes the data using the provided table but with two interleaved streams

  Writes

  * Table description

  * Encoded data with two interleaved states

  * Both Last state indexes

  * Padding bits to fill up last byte

- <span id="fseencoder-write-table"></span>`fn write_table(&mut self)`

- <span id="fseencoder-acc-log"></span>`fn acc_log(&self) -> u8`

### `FSETable`

```rust
struct FSETable {
    states: [SymbolStates; 256],
    table_size: usize,
}
```

#### Fields

- **`states`**: `[SymbolStates; 256]`

  Indexed by symbol

- **`table_size`**: `usize`

  Sum of all states.states.len()

#### Implementations

- <span id="fsetable-next-state"></span>`fn next_state(&self, symbol: u8, idx: usize) -> &State` — [`State`](#state)

- <span id="fsetable-start-state"></span>`fn start_state(&self, symbol: u8) -> &State` — [`State`](#state)

- <span id="fsetable-acc-log"></span>`fn acc_log(&self) -> u8`

- <span id="fsetable-write-table"></span>`fn write_table<V: AsMut<Vec<u8>>>(&self, writer: &mut BitWriter<V>)` — [`BitWriter`](../../bit_io/bit_writer/index.md#bitwriter)

#### Trait Implementations

##### `impl Clone for FSETable`

- <span id="fsetable-clone"></span>`fn clone(&self) -> FSETable` — [`FSETable`](#fsetable)

##### `impl Debug for FSETable`

- <span id="fsetable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `SymbolStates`

```rust
struct SymbolStates {
    states: alloc::vec::Vec<State>,
    probability: i32,
}
```

#### Fields

- **`states`**: `alloc::vec::Vec<State>`

  Sorted by baseline to allow easy lookup using an index

#### Implementations

- <span id="symbolstates-get"></span>`fn get(&self, idx: usize, max_idx: usize) -> &State` — [`State`](#state)

#### Trait Implementations

##### `impl Clone for SymbolStates`

- <span id="symbolstates-clone"></span>`fn clone(&self) -> SymbolStates` — [`SymbolStates`](#symbolstates)

##### `impl Debug for SymbolStates`

- <span id="symbolstates-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `State`

```rust
struct State {
    num_bits: u8,
    baseline: usize,
    last_index: usize,
    index: usize,
}
```

#### Fields

- **`num_bits`**: `u8`

  How many bits the range of this state needs to be encoded as

- **`baseline`**: `usize`

  The first index targeted by this state

- **`last_index`**: `usize`

  The last index targeted by this state (baseline + the maximum number with numbits bits allows)

- **`index`**: `usize`

  Index of this state in the decoding table

#### Implementations

- <span id="state-contains"></span>`fn contains(&self, idx: usize) -> bool`

#### Trait Implementations

##### `impl Clone for State`

- <span id="state-clone"></span>`fn clone(&self) -> State` — [`State`](#state)

##### `impl Debug for State`

- <span id="state-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

### `build_table_from_data`

```rust
fn build_table_from_data(data: impl Iterator<Item = u8>, max_log: u8, avoid_0_numbit: bool) -> FSETable
```

### `build_table_from_counts`

```rust
fn build_table_from_counts(counts: &[usize], max_log: u8, avoid_0_numbit: bool) -> FSETable
```

### `build_table_from_probabilities`

```rust
fn build_table_from_probabilities(probs: &[i32], acc_log: u8) -> FSETable
```

### `next_position`

```rust
fn next_position(p: usize, table_size: usize) -> usize
```

Calculate the position of the next entry of the table given the current
position and size of the table.

### `default_ml_table`

```rust
fn default_ml_table() -> FSETable
```

### `default_ll_table`

```rust
fn default_ll_table() -> FSETable
```

### `default_of_table`

```rust
fn default_of_table() -> FSETable
```

## Constants

### `ML_DIST`
```rust
const ML_DIST: &[i32];
```

### `LL_DIST`
```rust
const LL_DIST: &[i32];
```

### `OF_DIST`
```rust
const OF_DIST: &[i32];
```

