# Crate `crc32fast`

Fast, SIMD-accelerated CRC32 (IEEE) checksum computation.

## Usage

### Simple usage

For simple use-cases, you can call the [`hash()`](#hash) convenience function to
directly compute the CRC32 checksum for a given byte slice:

```rust
let checksum = crc32fast::hash(b"foo bar baz");
```

### Advanced usage

For use-cases that require more flexibility or performance, for example when
processing large amounts of data, you can create and manipulate a [`Hasher`](#hasher):

```rust
use crc32fast::Hasher;

let mut hasher = Hasher::new();
hasher.update(b"foo bar baz");
let checksum = hasher.finalize();
```

## Performance

This crate contains multiple CRC32 implementations:

- A fast baseline implementation which processes up to 16 bytes per iteration
- An optimized implementation for modern `x86` using `sse` and `pclmulqdq` instructions

Calling the `Hasher::new` constructor at runtime will perform a feature detection to select the most
optimal implementation for the current CPU feature set.

## Contents

- [Modules](#modules)
  - [`baseline`](#baseline)
  - [`combine`](#combine)
  - [`specialized`](#specialized)
  - [`table`](#table)
- [Structs](#structs)
  - [`Hasher`](#hasher)
- [Enums](#enums)
  - [`State`](#state)
- [Functions](#functions)
  - [`hash`](#hash)
- [Constants](#constants)
  - [`DEFAULT_INIT_STATE`](#default-init-state)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`baseline`](#baseline) | mod |  |
| [`combine`](#combine) | mod |  |
| [`specialized`](#specialized) | mod |  |
| [`table`](#table) | mod |  |
| [`Hasher`](#hasher) | struct | Represents an in-progress CRC32 computation. |
| [`State`](#state) | enum |  |
| [`hash`](#hash) | fn | Computes the CRC32 hash of a byte slice. |
| [`DEFAULT_INIT_STATE`](#default-init-state) | const |  |

## Modules

- [`baseline`](baseline/index.md)
- [`combine`](combine/index.md)
- [`specialized`](specialized/index.md)
- [`table`](table/index.md)

## Structs

### `Hasher`

```rust
struct Hasher {
    amount: u64,
    state: State,
}
```

Represents an in-progress CRC32 computation.

#### Implementations

- <span id="hasher-new"></span>`fn new() -> Self`

  Create a new `Hasher`.

  

  This will perform a CPU feature detection at runtime to select the most

  optimal implementation for the current processor architecture.

- <span id="hasher-new-with-initial"></span>`fn new_with_initial(init: u32) -> Self`

  Create a new `Hasher` with an initial CRC32 state.

  

  This works just like `Hasher::new`, except that it allows for an initial

  CRC32 state to be passed in.

- <span id="hasher-new-with-initial-len"></span>`fn new_with_initial_len(init: u32, amount: u64) -> Self`

  Create a new `Hasher` with an initial CRC32 state.

  

  As `new_with_initial`, but also accepts a length (in bytes). The

  resulting object can then be used with `combine` to compute `crc(a ||

  b)` from `crc(a)`, `crc(b)`, and `len(b)`.

- <span id="hasher-update"></span>`fn update(&mut self, buf: &[u8])`

  Process the given byte slice and update the hash state.

- <span id="hasher-finalize"></span>`fn finalize(self) -> u32`

  Finalize the hash state and return the computed CRC32 value.

- <span id="hasher-reset"></span>`fn reset(&mut self)`

  Reset the hash state.

- <span id="hasher-combine"></span>`fn combine(&mut self, other: &Self)`

  Combine the hash state with the hash state for the subsequent block of bytes.

#### Trait Implementations

##### `impl Clone for Hasher`

- <span id="hasher-clone"></span>`fn clone(&self) -> Hasher` — [`Hasher`](#hasher)

##### `impl Debug for Hasher`

- <span id="hasher-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Hasher`

- <span id="hasher-default"></span>`fn default() -> Self`

##### `impl Hasher for Hasher`

- <span id="hasher-hasher-write"></span>`fn write(&mut self, bytes: &[u8])`

- <span id="hasher-hasher-finish"></span>`fn finish(&self) -> u64`

## Enums

### `State`

```rust
enum State {
    Baseline(baseline::State),
    Specialized(specialized::State),
}
```

#### Trait Implementations

##### `impl Clone for State`

- <span id="state-clone"></span>`fn clone(&self) -> State` — [`State`](#state)

## Functions

### `hash`

```rust
fn hash(buf: &[u8]) -> u32
```

Computes the CRC32 hash of a byte slice.

Check out [`Hasher`](#hasher) for more advanced use-cases.

## Constants

### `DEFAULT_INIT_STATE`
```rust
const DEFAULT_INIT_STATE: u32 = 0u32;
```

