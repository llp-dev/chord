*[object](../../index.md) / [build](../index.md) / [bytes](index.md)*

---

# Module `bytes`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Bytes`](#bytes) | struct | A byte slice. |
| [`DebugByte`](#debugbyte) | struct |  |
| [`DebugLen`](#debuglen) | struct |  |
| [`ByteString`](#bytestring) | struct | A byte slice that is a string of an unknown encoding. |
| [`debug_list_bytes`](#debug-list-bytes) | fn |  |

## Structs

### `Bytes<'a>`

```rust
struct Bytes<'a>(alloc::borrow::Cow<'a, [u8]>);
```

A byte slice.

Uses copy-on-write to avoid unnecessary allocations. The bytes can be
accessed as a slice using the `Deref` trait, or as a mutable `Vec` using the
`to_mut` method.

Provides a `Debug` implementation that shows the first 8 bytes and the length.

#### Implementations

- <span id="bytes-to-mut"></span>`fn to_mut(&mut self) -> &mut Vec<u8>`

  Acquire a mutable reference to the bytes.

  

  Clones the bytes if they are shared.

- <span id="bytes-as-slice"></span>`fn as_slice(&self) -> &[u8]`

  Get the bytes as a slice.

#### Trait Implementations

##### `impl Clone for Bytes<'a>`

- <span id="bytes-clone"></span>`fn clone(&self) -> Bytes<'a>` — [`Bytes`](#bytes)

##### `impl Debug for Bytes<'a>`

- <span id="bytes-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Bytes<'a>`

- <span id="bytes-default"></span>`fn default() -> Bytes<'a>` — [`Bytes`](#bytes)

##### `impl Deref for Bytes<'a>`

- <span id="bytes-deref-type-target"></span>`type Target = [u8]`

- <span id="bytes-deref"></span>`fn deref(&self) -> &[u8]`

##### `impl Eq for Bytes<'a>`

##### `impl<K> Equivalent for Bytes<'a>`

- <span id="bytes-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl PartialEq for Bytes<'a>`

- <span id="bytes-partialeq-eq"></span>`fn eq(&self, other: &Bytes<'a>) -> bool` — [`Bytes`](#bytes)

##### `impl Receiver for Bytes<'a>`

- <span id="bytes-receiver-type-target"></span>`type Target = T`

##### `impl StructuralPartialEq for Bytes<'a>`

### `DebugByte`

```rust
struct DebugByte(u8);
```

#### Trait Implementations

##### `impl Debug for DebugByte`

- <span id="debugbyte-debug-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

### `DebugLen`

```rust
struct DebugLen(usize);
```

#### Trait Implementations

##### `impl Debug for DebugLen`

- <span id="debuglen-debug-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ByteString<'a>`

```rust
struct ByteString<'a>(alloc::borrow::Cow<'a, [u8]>);
```

A byte slice that is a string of an unknown encoding.

Uses copy-on-write to avoid unnecessary allocations. The bytes can be
accessed as a slice using the `Deref` trait, or as a mutable `Vec` using the
`to_mut` method.

Provides a `Debug` implementation that interprets the bytes as UTF-8.

#### Implementations

- <span id="bytestring-to-mut"></span>`fn to_mut(&mut self) -> &mut Vec<u8>`

  Acquire a mutable reference to the bytes.

  

  Clones the bytes if they are shared.

- <span id="bytestring-as-slice"></span>`fn as_slice(&self) -> &[u8]`

  Get the bytes as a slice.

#### Trait Implementations

##### `impl Clone for ByteString<'a>`

- <span id="bytestring-clone"></span>`fn clone(&self) -> ByteString<'a>` — [`ByteString`](#bytestring)

##### `impl Debug for ByteString<'a>`

- <span id="bytestring-debug-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ByteString<'a>`

- <span id="bytestring-default"></span>`fn default() -> ByteString<'a>` — [`ByteString`](#bytestring)

##### `impl Deref for ByteString<'a>`

- <span id="bytestring-deref-type-target"></span>`type Target = [u8]`

- <span id="bytestring-deref"></span>`fn deref(&self) -> &[u8]`

##### `impl Display for ByteString<'a>`

- <span id="bytestring-display-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ByteString<'a>`

##### `impl<K> Equivalent for ByteString<'a>`

- <span id="bytestring-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for ByteString<'a>`

- <span id="bytestring-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for ByteString<'a>`

- <span id="bytestring-partialeq-eq"></span>`fn eq(&self, other: &ByteString<'a>) -> bool` — [`ByteString`](#bytestring)

##### `impl Receiver for ByteString<'a>`

- <span id="bytestring-receiver-type-target"></span>`type Target = T`

##### `impl StructuralPartialEq for ByteString<'a>`

##### `impl ToString for ByteString<'a>`

- <span id="bytestring-tostring-to-string"></span>`fn to_string(&self) -> String`

## Functions

### `debug_list_bytes`

```rust
fn debug_list_bytes(bytes: &[u8], fmt: &mut fmt::Formatter<'_>) -> fmt::Result
```

