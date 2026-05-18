*[postcard](../../index.md) / [ser](../index.md) / [flavors](index.md)*

---

# Module `flavors`

# Serialization Flavors

"Flavors" in `postcard` are used as modifiers to the serialization or deserialization
process. Flavors typically modify one or both of the following:

1. The output medium of the serialization, e.g. whether the data is serialized to a `[u8]` slice, or a `heapless::Vec`.
2. The format of the serialization, such as encoding the serialized output in a COBS format, performing CRC32 checksumming while serializing, etc.

Flavors are implemented using the [`Flavor`](#flavor) trait, which acts as a "middleware" for receiving the bytes as serialized by `serde`.
Multiple flavors may be combined to obtain a desired combination of behavior and storage.
When flavors are combined, it is expected that the storage flavor (such as `Slice` or `HVec`) is the innermost flavor.

Custom flavors may be defined by users of the `postcard` crate, however some commonly useful flavors have been provided in
this module. If you think your custom flavor would be useful to others, PRs adding flavors are very welcome!

## Usability

Flavors may not always be convenient to use directly, as they may expose some implementation details of how the
inner workings of the flavor behaves. It is typical to provide a convenience method for using a flavor, to prevent
the user from having to specify generic parameters, setting correct initialization values, or handling the output of
the flavor correctly. See `postcard::to_vec()` for an example of this.

It is recommended to use the [`serialize_with_flavor()`](../fn.serialize_with_flavor.html) method for serialization. See it's documentation for information
regarding usage and generic type parameters.

## When to use (multiple) flavors

Combining flavors are nice for convenience, as they perform potentially multiple steps of
serialization at one time.

This can often be more memory efficient, as intermediate buffers are not typically required.

## When NOT to use (multiple) flavors

The downside of passing serialization through multiple steps is that it is typically slower than
performing each step serially. Said simply, "cobs encoding while serializing" is often slower
than "serialize then cobs encode", due to the ability to handle longer "runs" of data in each
stage. The downside is that if these stages can not be performed in-place on the buffer, you
will need additional buffers for each stage.

## Examples

### Using a single flavor

In the first example, we use the `Slice` flavor, to store the serialized output into a mutable `[u8]` slice.
No other modification is made to the serialization process.

```rust
use postcard::{
    serialize_with_flavor,
    ser_flavors::Slice,
};

let mut buf = [0u8; 32];

let data: &[u8] = &[0x01, 0x00, 0x20, 0x30];
let buffer = &mut [0u8; 32];
let res = serialize_with_flavor::<[u8], Slice, &mut [u8]>(
    data,
    Slice::new(buffer)
).unwrap();

assert_eq!(res, &[0x04, 0x01, 0x00, 0x20, 0x30]);
```

### Using combined flavors

In the second example, we mix `Slice` with `Cobs`, to cobs encode the output while
the data is serialized. Notice how `Slice` (the storage flavor) is the innermost flavor used.

```rust
use postcard::{
    serialize_with_flavor,
    ser_flavors::{Cobs, Slice},
};

let mut buf = [0u8; 32];

let data: &[u8] = &[0x01, 0x00, 0x20, 0x30];
let buffer = &mut [0u8; 32];
let res = serialize_with_flavor::<[u8], Cobs<Slice>, &mut [u8]>(
    data,
    Cobs::try_new(Slice::new(buffer)).unwrap(),
).unwrap();

assert_eq!(res, &[0x03, 0x04, 0x01, 0x03, 0x20, 0x30, 0x00]);
```

## Contents

- [Modules](#modules)
  - [`alloc_vec`](#alloc-vec)
- [Structs](#structs)
  - [`Slice`](#slice)
  - [`ExtendFlavor`](#extendflavor)
  - [`Cobs`](#cobs)
  - [`Size`](#size)
  - [`AllocVec`](#allocvec)
- [Traits](#traits)
  - [`Flavor`](#flavor)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`alloc_vec`](#alloc-vec) | mod |  |
| [`Slice`](#slice) | struct | The `Slice` flavor is a storage flavor, storing the serialized (or otherwise modified) bytes into a plain `[u8]` slice. |
| [`ExtendFlavor`](#extendflavor) | struct | Wrapper over a [`core::iter::Extend<u8>`] that implements the flavor trait |
| [`Cobs`](#cobs) | struct | The `Cobs` flavor implements [Consistent Overhead Byte Stuffing] on the serialized data. |
| [`Size`](#size) | struct | The `Size` flavor is a measurement flavor, which accumulates the number of bytes needed to serialize the data. |
| [`AllocVec`](#allocvec) | struct | The `AllocVec` flavor is a wrapper type around an [`alloc::vec::Vec`]. |
| [`Flavor`](#flavor) | trait | The serialization Flavor trait |

## Modules

- [`alloc_vec`](alloc_vec/index.md)

## Structs

### `Slice<'a>`

```rust
struct Slice<'a> {
    start: *mut u8,
    cursor: *mut u8,
    end: *mut u8,
    _pl: core::marker::PhantomData<&'a [u8]>,
}
```

The `Slice` flavor is a storage flavor, storing the serialized (or otherwise modified) bytes into a plain
`[u8]` slice. The `Slice` flavor resolves into a sub-slice of the original slice buffer.

#### Implementations

- <span id="slice-new"></span>`fn new(buf: &'a mut [u8]) -> Self`

  Create a new `Slice` flavor from a given backing buffer

#### Trait Implementations

##### `impl Flavor for Slice<'a>`

- <span id="slice-flavor-type-output"></span>`type Output = &'a mut [u8]`

- <span id="slice-flavor-try-push"></span>`fn try_push(&mut self, b: u8) -> Result<()>` — [`Result`](../../error/index.md#result)

- <span id="slice-flavor-try-extend"></span>`fn try_extend(&mut self, b: &[u8]) -> Result<()>` — [`Result`](../../error/index.md#result)

- <span id="slice-flavor-finalize"></span>`fn finalize(self) -> Result<<Self as >::Output>` — [`Result`](../../error/index.md#result), [`Flavor`](#flavor)

##### `impl Index for Slice<'_>`

- <span id="slice-index-type-output"></span>`type Output = u8`

- <span id="slice-index"></span>`fn index(&self, idx: usize) -> &u8`

##### `impl IndexMut for Slice<'_>`

- <span id="slice-indexmut-index-mut"></span>`fn index_mut(&mut self, idx: usize) -> &mut u8`

### `ExtendFlavor<T>`

```rust
struct ExtendFlavor<T> {
    iter: T,
}
```

Wrapper over a `core::iter::Extend<u8>` that implements the flavor trait

#### Implementations

- <span id="extendflavor-new"></span>`fn new(iter: T) -> Self`

  Create a new [`Self`](#self) flavor from a given `core::iter::Extend<u8>`

#### Trait Implementations

##### `impl<T> Flavor for ExtendFlavor<T>`

- <span id="extendflavor-flavor-type-output"></span>`type Output = T`

- <span id="extendflavor-flavor-try-push"></span>`fn try_push(&mut self, data: u8) -> Result<()>` — [`Result`](../../error/index.md#result)

- <span id="extendflavor-flavor-try-extend"></span>`fn try_extend(&mut self, b: &[u8]) -> Result<()>` — [`Result`](../../error/index.md#result)

- <span id="extendflavor-flavor-finalize"></span>`fn finalize(self) -> Result<<Self as >::Output>` — [`Result`](../../error/index.md#result), [`Flavor`](#flavor)

### `Cobs<B>`

```rust
struct Cobs<B>
where
    B: Flavor + IndexMut<usize, Output = u8> {
    flav: B,
    cobs: cobs::EncoderState,
}
```

The `Cobs` flavor implements [Consistent Overhead Byte Stuffing] on
the serialized data. The output of this flavor includes the termination/sentinel
byte of `0x00`.

This protocol is useful when sending data over a serial interface without framing such as a UART


#### Implementations

- <span id="cobs-try-new"></span>`fn try_new(bee: B) -> Result<Self>` — [`Result`](../../error/index.md#result)

  Create a new Cobs modifier Flavor. If there is insufficient space

  to push the leading header byte, the method will return an Error

#### Trait Implementations

##### `impl<B> Flavor for Cobs<B>`

- <span id="cobs-flavor-type-output"></span>`type Output = <B as Flavor>::Output`

- <span id="cobs-flavor-try-push"></span>`fn try_push(&mut self, data: u8) -> Result<()>` — [`Result`](../../error/index.md#result)

- <span id="cobs-flavor-finalize"></span>`fn finalize(self) -> Result<<Self as >::Output>` — [`Result`](../../error/index.md#result), [`Flavor`](#flavor)

### `Size`

```rust
struct Size {
    size: usize,
}
```

The `Size` flavor is a measurement flavor, which accumulates the number of bytes needed to
serialize the data.

```rust
use postcard::{serialize_with_flavor, ser_flavors};

let value = false;
let size = serialize_with_flavor(&value, ser_flavors::Size::default()).unwrap();

assert_eq!(size, 1);
```

#### Trait Implementations

##### `impl Default for Size`

- <span id="size-default"></span>`fn default() -> Size` — [`Size`](#size)

##### `impl Flavor for Size`

- <span id="size-flavor-type-output"></span>`type Output = usize`

- <span id="size-flavor-try-push"></span>`fn try_push(&mut self, _b: u8) -> Result<()>` — [`Result`](../../error/index.md#result)

- <span id="size-flavor-try-extend"></span>`fn try_extend(&mut self, b: &[u8]) -> Result<()>` — [`Result`](../../error/index.md#result)

- <span id="size-flavor-finalize"></span>`fn finalize(self) -> Result<<Self as >::Output>` — [`Result`](../../error/index.md#result), [`Flavor`](#flavor)

### `AllocVec`

```rust
struct AllocVec {
    vec: alloc::vec::Vec<u8>,
}
```

The `AllocVec` flavor is a wrapper type around an [`alloc::vec::Vec`](../../../addr2line/maybe_small/index.md).

This type is only available when the (non-default) `alloc` feature is active

#### Fields

- **`vec`**: `alloc::vec::Vec<u8>`

  The vec to be used for serialization

#### Implementations

- <span id="allocvec-new"></span>`fn new() -> Self`

  Create a new, currently empty, [`alloc::vec::Vec`](../../../addr2line/maybe_small/index.md) to be used for storing serialized

  output data.

#### Trait Implementations

##### `impl Default for AllocVec`

- <span id="allocvec-default"></span>`fn default() -> AllocVec` — [`AllocVec`](#allocvec)

##### `impl Flavor for AllocVec`

- <span id="allocvec-flavor-type-output"></span>`type Output = Vec<u8>`

- <span id="allocvec-flavor-try-extend"></span>`fn try_extend(&mut self, data: &[u8]) -> Result<()>` — [`Result`](../../error/index.md#result)

- <span id="allocvec-flavor-try-push"></span>`fn try_push(&mut self, data: u8) -> Result<()>` — [`Result`](../../error/index.md#result)

- <span id="allocvec-flavor-finalize"></span>`fn finalize(self) -> Result<<Self as >::Output>` — [`Result`](../../error/index.md#result), [`Flavor`](#flavor)

##### `impl Index for AllocVec`

- <span id="allocvec-index-type-output"></span>`type Output = u8`

- <span id="allocvec-index"></span>`fn index(&self, idx: usize) -> &u8`

##### `impl IndexMut for AllocVec`

- <span id="allocvec-indexmut-index-mut"></span>`fn index_mut(&mut self, idx: usize) -> &mut u8`

## Traits

### `Flavor`

```rust
trait Flavor { ... }
```

The serialization Flavor trait

This is used as the primary way to encode serialized data into some kind of buffer,
or modify that data in a middleware style pattern.

See the module level docs for an example of how flavors are used.

#### Associated Types

- `type Output`

#### Required Methods

- `fn try_push(&mut self, data: u8) -> Result<()>`

  Push a single byte to be modified and/or stored.

- `fn finalize(self) -> Result<<Self as >::Output>`

  Finalize the serialization process.

#### Provided Methods

- `fn try_extend(&mut self, data: &[u8]) -> Result<()>`

  Override this method when you want to customize processing

#### Implementors

- [`AllocVec`](#allocvec)
- [`Cobs`](#cobs)
- [`ExtendFlavor`](#extendflavor)
- [`Size`](#size)
- [`Slice`](#slice)

