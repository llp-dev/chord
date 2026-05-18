*[ring](../index.md) / [digest](index.md)*

---

# Module `digest`

SHA-2 and the legacy SHA-1 digest algorithm.

If all the data is available in a single contiguous slice then the `digest`
function should be used. Otherwise, the digest can be calculated in
multiple steps using `Context`.

## Contents

- [Modules](#modules)
  - [`sha1`](#sha1)
  - [`sha2`](#sha2)
- [Structs](#structs)
  - [`BlockContext`](#blockcontext)
  - [`Context`](#context)
  - [`Digest`](#digest)
  - [`Algorithm`](#algorithm)
  - [`Output`](#output)
- [Enums](#enums)
  - [`AlgorithmID`](#algorithmid)
- [Functions](#functions)
  - [`digest`](#digest)
  - [`sha256_format_output`](#sha256-format-output)
  - [`sha512_format_output`](#sha512-format-output)
  - [`format_output`](#format-output)
- [Constants](#constants)
  - [`MAX_BLOCK_LEN`](#max-block-len)
  - [`MAX_OUTPUT_LEN`](#max-output-len)
  - [`MAX_CHAINING_LEN`](#max-chaining-len)
  - [`SHA1_OUTPUT_LEN`](#sha1-output-len)
  - [`SHA256_OUTPUT_LEN`](#sha256-output-len)
  - [`SHA384_OUTPUT_LEN`](#sha384-output-len)
  - [`SHA512_OUTPUT_LEN`](#sha512-output-len)
  - [`SHA512_256_OUTPUT_LEN`](#sha512-256-output-len)
  - [`SHA512_BLOCK_LEN`](#sha512-block-len)
  - [`SHA512_LEN_LEN`](#sha512-len-len)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`sha1`](#sha1) | mod |  |
| [`sha2`](#sha2) | mod |  |
| [`BlockContext`](#blockcontext) | struct |  |
| [`Context`](#context) | struct | A context for multi-step (Init-Update-Finish) digest calculations. |
| [`Digest`](#digest) | struct | A calculated digest value. |
| [`Algorithm`](#algorithm) | struct | A digest algorithm. |
| [`Output`](#output) | struct |  |
| [`AlgorithmID`](#algorithmid) | enum |  |
| [`digest`](#digest) | fn | Returns the digest of `data` using the given digest algorithm. |
| [`sha256_format_output`](#sha256-format-output) | fn |  |
| [`sha512_format_output`](#sha512-format-output) | fn |  |
| [`format_output`](#format-output) | fn |  |
| [`MAX_BLOCK_LEN`](#max-block-len) | const | The maximum block length ([`Algorithm::block_len()`]) of all the algorithms in this module. |
| [`MAX_OUTPUT_LEN`](#max-output-len) | const | The maximum output length ([`Algorithm::output_len()`]) of all the algorithms in this module. |
| [`MAX_CHAINING_LEN`](#max-chaining-len) | const | The maximum chaining length ([`Algorithm::chaining_len()`]) of all the algorithms in this module. |
| [`SHA1_OUTPUT_LEN`](#sha1-output-len) | const | The length of the output of SHA-1, in bytes. |
| [`SHA256_OUTPUT_LEN`](#sha256-output-len) | const | The length of the output of SHA-256, in bytes. |
| [`SHA384_OUTPUT_LEN`](#sha384-output-len) | const | The length of the output of SHA-384, in bytes. |
| [`SHA512_OUTPUT_LEN`](#sha512-output-len) | const | The length of the output of SHA-512, in bytes. |
| [`SHA512_256_OUTPUT_LEN`](#sha512-256-output-len) | const | The length of the output of SHA-512/256, in bytes. |
| [`SHA512_BLOCK_LEN`](#sha512-block-len) | const | The length of a block for SHA-512-based algorithms, in bytes. |
| [`SHA512_LEN_LEN`](#sha512-len-len) | const | The length of the length field for SHA-512-based algorithms, in bytes. |

## Modules

- [`sha1`](sha1/index.md)
- [`sha2`](sha2/index.md)

## Structs

### `BlockContext`

```rust
struct BlockContext {
    state: State,
    completed_data_blocks: u64,
    pub algorithm: &'static Algorithm,
}
```

#### Fields

- **`algorithm`**: `&'static Algorithm`

  The context's algorithm.

#### Implementations

- <span id="blockcontext-new"></span>`fn new(algorithm: &'static Algorithm) -> Self` — [`Algorithm`](#algorithm)

- <span id="blockcontext-update"></span>`fn update(&mut self, input: &[u8])`

- <span id="blockcontext-finish"></span>`fn finish(self, pending: &mut [u8], num_pending: usize) -> Digest` — [`Digest`](#digest)

- <span id="blockcontext-block-data-order"></span>`unsafe fn block_data_order(&mut self, pending: *const u8, num_blocks: usize, _cpu_features: cpu::Features)` — [`Features`](../cpu/index.md#features)

#### Trait Implementations

##### `impl Clone for BlockContext`

- <span id="blockcontext-clone"></span>`fn clone(&self) -> BlockContext` — [`BlockContext`](#blockcontext)

### `Context`

```rust
struct Context {
    block: BlockContext,
    pending: [u8; 128],
    num_pending: usize,
}
```

A context for multi-step (Init-Update-Finish) digest calculations.

# Examples

```rust
use ring::digest;

let one_shot = digest::digest(&digest::SHA384, b"hello, world");

let mut ctx = digest::Context::new(&digest::SHA384);
ctx.update(b"hello");
ctx.update(b", ");
ctx.update(b"world");
let multi_part = ctx.finish();

assert_eq!(&one_shot.as_ref(), &multi_part.as_ref());
```

#### Implementations

- <span id="context-new"></span>`fn new(algorithm: &'static Algorithm) -> Self` — [`Algorithm`](#algorithm)

  Constructs a new context.

- <span id="context-clone-from"></span>`fn clone_from(block: &BlockContext) -> Self` — [`BlockContext`](#blockcontext)

- <span id="context-update"></span>`fn update(&mut self, data: &[u8])`

  Updates the digest with all the data in `data`.

- <span id="context-finish"></span>`fn finish(self) -> Digest` — [`Digest`](#digest)

  Finalizes the digest calculation and returns the digest value.

  

  `finish` consumes the context so it cannot be (mis-)used after `finish`

  has been called.

- <span id="context-algorithm"></span>`fn algorithm(&self) -> &'static Algorithm` — [`Algorithm`](#algorithm)

  The algorithm that this context is using.

#### Trait Implementations

##### `impl Clone for Context`

- <span id="context-clone"></span>`fn clone(&self) -> Context` — [`Context`](#context)

### `Digest`

```rust
struct Digest {
    value: Output,
    algorithm: &'static Algorithm,
}
```

A calculated digest value.

Use `Self::as_ref` to get the value as a `&[u8]`.

#### Implementations

- <span id="digest-algorithm"></span>`fn algorithm(&self) -> &'static Algorithm` — [`Algorithm`](#algorithm)

  The algorithm that was used to calculate the digest value.

#### Trait Implementations

##### `impl AsRef for Digest`

- <span id="digest-asref-as-ref"></span>`fn as_ref(&self) -> &[u8]`

##### `impl Clone for Digest`

- <span id="digest-clone"></span>`fn clone(&self) -> Digest` — [`Digest`](#digest)

##### `impl Copy for Digest`

##### `impl Debug for Digest`

- <span id="digest-debug-fmt"></span>`fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `Algorithm`

```rust
struct Algorithm {
    output_len: usize,
    chaining_len: usize,
    block_len: usize,
    len_len: usize,
    block_data_order: fn(&mut State, *const u8, usize),
    format_output: fn(State) -> Output,
    initial_state: State,
    id: AlgorithmID,
}
```

A digest algorithm.

#### Fields

- **`len_len`**: `usize`

  The length of the length in the padding.

#### Implementations

- <span id="algorithm-block-len"></span>`fn block_len(&self) -> usize`

  The internal block length.

- <span id="algorithm-chaining-len"></span>`fn chaining_len(&self) -> usize`

  The size of the chaining value of the digest function, in bytes.

  

  For non-truncated algorithms (SHA-1, SHA-256, SHA-512), this is equal

  to `Self::output_len()`. For truncated algorithms (e.g. SHA-384,

  SHA-512/256), this is equal to the length before truncation. This is

  mostly helpful for determining the size of an HMAC key that is

  appropriate for the digest algorithm.

- <span id="algorithm-output-len"></span>`fn output_len(&self) -> usize`

  The length of a finalized digest.

#### Trait Implementations

##### `impl Debug for Algorithm`

- <span id="algorithm-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> Result<(), ::core::fmt::Error>`

##### `impl Eq for Algorithm`

##### `impl PartialEq for Algorithm`

- <span id="algorithm-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

### `Output`

```rust
struct Output([u8; 64]);
```

#### Trait Implementations

##### `impl Clone for Output`

- <span id="output-clone"></span>`fn clone(&self) -> Output` — [`Output`](#output)

##### `impl Copy for Output`

## Enums

### `AlgorithmID`

```rust
enum AlgorithmID {
    SHA1,
    SHA256,
    SHA384,
    SHA512,
    SHA512_256,
}
```

#### Trait Implementations

##### `impl Debug for AlgorithmID`

- <span id="algorithmid-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for AlgorithmID`

##### `impl PartialEq for AlgorithmID`

- <span id="algorithmid-partialeq-eq"></span>`fn eq(&self, other: &AlgorithmID) -> bool` — [`AlgorithmID`](#algorithmid)

##### `impl StructuralPartialEq for AlgorithmID`

## Functions

### `digest`

```rust
fn digest(algorithm: &'static Algorithm, data: &[u8]) -> Digest
```

Returns the digest of `data` using the given digest algorithm.

# Examples:

```rust
#[cfg(feature = "alloc")]
{
use ring::{digest, test};
let expected_hex = "09ca7e4eaa6e8ae9c7d261167129184883644d07dfba7cbfbc4c8a2e08360d5b";
let expected: Vec<u8> = test::from_hex(expected_hex).unwrap();
let actual = digest::digest(&digest::SHA256, b"hello, world");

assert_eq!(&expected, &actual.as_ref());
}
```

### `sha256_format_output`

```rust
fn sha256_format_output(input: State) -> Output
```

### `sha512_format_output`

```rust
fn sha512_format_output(input: State) -> Output
```

### `format_output`

```rust
fn format_output<T, F, const N: usize>(input: [core::num::Wrapping<T>; 8], f: F) -> Output
where
    F: Fn(T) -> [u8; N],
    T: Copy
```

## Constants

### `MAX_BLOCK_LEN`
```rust
const MAX_BLOCK_LEN: usize = 128usize;
```

The maximum block length (`Algorithm::block_len()`) of all the algorithms
in this module.

### `MAX_OUTPUT_LEN`
```rust
const MAX_OUTPUT_LEN: usize = 64usize;
```

The maximum output length (`Algorithm::output_len()`) of all the
algorithms in this module.

### `MAX_CHAINING_LEN`
```rust
const MAX_CHAINING_LEN: usize = 64usize;
```

The maximum chaining length (`Algorithm::chaining_len()`) of all the
algorithms in this module.

### `SHA1_OUTPUT_LEN`
```rust
const SHA1_OUTPUT_LEN: usize = 20usize;
```

The length of the output of SHA-1, in bytes.

### `SHA256_OUTPUT_LEN`
```rust
const SHA256_OUTPUT_LEN: usize = 32usize;
```

The length of the output of SHA-256, in bytes.

### `SHA384_OUTPUT_LEN`
```rust
const SHA384_OUTPUT_LEN: usize = 48usize;
```

The length of the output of SHA-384, in bytes.

### `SHA512_OUTPUT_LEN`
```rust
const SHA512_OUTPUT_LEN: usize = 64usize;
```

The length of the output of SHA-512, in bytes.

### `SHA512_256_OUTPUT_LEN`
```rust
const SHA512_256_OUTPUT_LEN: usize = 32usize;
```

The length of the output of SHA-512/256, in bytes.

### `SHA512_BLOCK_LEN`
```rust
const SHA512_BLOCK_LEN: usize = 128usize;
```

The length of a block for SHA-512-based algorithms, in bytes.

### `SHA512_LEN_LEN`
```rust
const SHA512_LEN_LEN: usize = 16usize;
```

The length of the length field for SHA-512-based algorithms, in bytes.

