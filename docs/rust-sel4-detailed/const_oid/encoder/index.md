*[const_oid](../index.md) / [encoder](index.md)*

---

# Module `encoder`

OID encoder with `const` support.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Encoder`](#encoder) | struct | BER/DER encoder |
| [`State`](#state) | enum | Current state of the encoder |
| [`base128_len`](#base128-len) | fn | Compute the length - 1 of an arc when encoded in base 128. |

## Structs

### `Encoder`

```rust
struct Encoder {
    state: State,
    bytes: [u8; 39],
    cursor: usize,
}
```

BER/DER encoder

#### Fields

- **`state`**: `State`

  Current state

- **`bytes`**: `[u8; 39]`

  Bytes of the OID being encoded in-progress

- **`cursor`**: `usize`

  Current position within the byte buffer

#### Implementations

- <span id="encoder-new"></span>`const fn new() -> Self`

  Create a new encoder initialized to an empty default state.

- <span id="encoder-extend"></span>`const fn extend(oid: ObjectIdentifier) -> Self` — [`ObjectIdentifier`](../index.md#objectidentifier)

  Extend an existing OID.

- <span id="encoder-arc"></span>`const fn arc(self, arc: Arc) -> Result<Self>` — [`Arc`](../arcs/index.md#arc), [`Result`](../error/index.md#result)

  Encode an [`Arc`](../arcs/index.md) as base 128 into the internal buffer.

- <span id="encoder-finish"></span>`const fn finish(self) -> Result<ObjectIdentifier>` — [`Result`](../error/index.md#result), [`ObjectIdentifier`](../index.md#objectidentifier)

  Finish encoding an OID.

- <span id="encoder-encode-base128-byte"></span>`const fn encode_base128_byte(self, n: u32, i: usize, continued: bool) -> Result<Self>` — [`Result`](../error/index.md#result)

  Encode a single byte of a Base 128 value.

#### Trait Implementations

##### `impl Debug for Encoder`

- <span id="encoder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Enums

### `State`

```rust
enum State {
    Initial,
    FirstArc(crate::Arc),
    Body,
}
```

Current state of the encoder

#### Variants

- **`Initial`**

  Initial state - no arcs yet encoded

- **`FirstArc`**

  First arc parsed

- **`Body`**

  Encoding base 128 body of the OID

#### Trait Implementations

##### `impl Debug for State`

- <span id="state-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

### `base128_len`

```rust
const fn base128_len(arc: crate::Arc) -> usize
```

Compute the length - 1 of an arc when encoded in base 128.

