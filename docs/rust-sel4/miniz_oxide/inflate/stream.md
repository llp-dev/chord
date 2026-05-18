**miniz_oxide > inflate > stream**

# Module: inflate::stream

## Contents

**Structs**

- [`FullReset`](#fullreset) - Full reset of the state, including zeroing memory.
- [`InflateState`](#inflatestate) - A struct that compbines a decompressor with extra data for streaming decompression.
- [`MinReset`](#minreset) - Resets state, without performing expensive ops (e.g. zeroing buffer)
- [`ZeroReset`](#zeroreset) - Resets state and zero memory, continuing to use the same data format.

**Functions**

- [`inflate`](#inflate) - Try to decompress from `input` to `output` with the given [`InflateState`]

**Traits**

- [`ResetPolicy`](#resetpolicy) - Tag that determines reset policy of [InflateState](struct.InflateState.html)

---

## miniz_oxide::inflate::stream::FullReset

*Struct*

Full reset of the state, including zeroing memory.

Requires to provide new data format.

**Tuple Struct**: `(crate::DataFormat)`

**Trait Implementations:**

- **ResetPolicy**
  - `fn reset(self: &Self, state: & mut InflateState)`



## miniz_oxide::inflate::stream::InflateState

*Struct*

A struct that compbines a decompressor with extra data for streaming decompression.


**Methods:**

- `fn new(data_format: DataFormat) -> InflateState` - Create a new state.
- `fn new_boxed(data_format: DataFormat) -> Box<InflateState>` - Create a new state on the heap.
- `fn decompressor(self: & mut Self) -> & mut DecompressorOxide` - Access the innner decompressor.
- `fn last_status(self: &Self) -> TINFLStatus` - Return the status of the last call to `inflate` with this `InflateState`.
- `fn new_boxed_with_window_bits(window_bits: i32) -> Box<InflateState>` - Create a new state using miniz/zlib style window bits parameter.
- `fn reset(self: & mut Self, data_format: DataFormat)` - Reset the decompressor without re-allocating memory, using the given
- `fn reset_as<T>(self: & mut Self, policy: T)` - Resets the state according to specified policy.

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> InflateState`



## miniz_oxide::inflate::stream::MinReset

*Struct*

Resets state, without performing expensive ops (e.g. zeroing buffer)

Note that not zeroing buffer can lead to security issues when dealing with untrusted input.

**Unit Struct**

**Trait Implementations:**

- **ResetPolicy**
  - `fn reset(self: &Self, state: & mut InflateState)`



## miniz_oxide::inflate::stream::ResetPolicy

*Trait*

Tag that determines reset policy of [InflateState](struct.InflateState.html)

**Methods:**

- `reset`: Performs reset



## miniz_oxide::inflate::stream::ZeroReset

*Struct*

Resets state and zero memory, continuing to use the same data format.

**Unit Struct**

**Trait Implementations:**

- **ResetPolicy**
  - `fn reset(self: &Self, state: & mut InflateState)`



## miniz_oxide::inflate::stream::inflate

*Function*

Try to decompress from `input` to `output` with the given [`InflateState`]

# `flush`

Generally, the various [`MZFlush`] flags have meaning only on the compression side.  They can be
supplied here, but the only one that has any semantic meaning is [`MZFlush::Finish`], which is a
signal that the stream is expected to finish, and failing to do so is an error.  It isn't
necessary to specify it when the stream ends; you'll still get returned a
[`MZStatus::StreamEnd`] anyway.  Other values either have no effect or cause errors.  It's
likely that you'll almost always just want to use [`MZFlush::None`].

# Errors

Returns [`MZError::Buf`] if the size of the `output` slice is empty or no progress was made due
to lack of expected input data, or if called with [`MZFlush::Finish`] and input wasn't all
consumed.

Returns [`MZError::Data`] if this or a a previous call failed with an error return from
[`TINFLStatus`]; probably indicates corrupted data.

Returns [`MZError::Stream`] when called with [`MZFlush::Full`] (meaningless on
decompression), or when called without [`MZFlush::Finish`] after an earlier call with
[`MZFlush::Finish`] has been made.

```rust
fn inflate(state: & mut InflateState, input: &[u8], output: & mut [u8], flush: crate::MZFlush) -> crate::StreamResult
```



