*[flate2](../index.md) / [zio](index.md)*

---

# Module `zio`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Writer`](#writer) | struct |  |
| [`Ops`](#ops) | trait |  |
| [`Flush`](#flush) | trait |  |
| [`read`](#read) | fn |  |

## Structs

### `Writer<W: Write, D: Ops>`

```rust
struct Writer<W: Write, D: Ops> {
    obj: Option<W>,
    pub data: D,
    buf: Vec<u8>,
}
```

#### Implementations

- <span id="writer-new"></span>`fn new(w: W, d: D) -> Writer<W, D>` — [`Writer`](#writer)

- <span id="writer-finish"></span>`fn finish(&mut self) -> io::Result<()>`

- <span id="writer-replace"></span>`fn replace(&mut self, w: W) -> W`

- <span id="writer-get-ref"></span>`fn get_ref(&self) -> &W`

- <span id="writer-get-mut"></span>`fn get_mut(&mut self) -> &mut W`

- <span id="writer-take-inner"></span>`fn take_inner(&mut self) -> W`

- <span id="writer-is-present"></span>`fn is_present(&self) -> bool`

- <span id="writer-write-with-status"></span>`fn write_with_status(&mut self, buf: &[u8]) -> io::Result<(usize, Status)>` — [`Status`](../mem/index.md#status)

- <span id="writer-dump"></span>`fn dump(&mut self) -> io::Result<()>`

#### Trait Implementations

##### `impl<W: fmt::Debug + Write, D: fmt::Debug + Ops> Debug for Writer<W, D>`

- <span id="writer-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<W: Write, D: Ops> Drop for Writer<W, D>`

- <span id="writer-drop"></span>`fn drop(&mut self)`

##### `impl<W: Write, D: Ops> Write for Writer<W, D>`

- <span id="writer-write"></span>`fn write(&mut self, buf: &[u8]) -> io::Result<usize>`

- <span id="writer-write-flush"></span>`fn flush(&mut self) -> io::Result<()>`

## Traits

### `Ops`

```rust
trait Ops { ... }
```

#### Associated Types

- `type Error: 1`

- `type Flush: 1`

#### Required Methods

- `fn total_in(&self) -> u64`

- `fn total_out(&self) -> u64`

- `fn run(&mut self, input: &[u8], output: &mut [u8], flush: <Self as >::Flush) -> Result<Status, <Self as >::Error>`

- `fn run_vec(&mut self, input: &[u8], output: &mut Vec<u8>, flush: <Self as >::Flush) -> Result<Status, <Self as >::Error>`

#### Implementors

- [`Compress`](../mem/index.md#compress)
- [`Decompress`](../mem/index.md#decompress)

### `Flush`

```rust
trait Flush { ... }
```

#### Required Methods

- `fn none() -> Self`

- `fn sync() -> Self`

- `fn finish() -> Self`

#### Implementors

- [`FlushCompress`](../mem/index.md#flushcompress)
- [`FlushDecompress`](../mem/index.md#flushdecompress)

## Functions

### `read`

```rust
fn read<R, D>(obj: &mut R, data: &mut D, dst: &mut [u8]) -> io::Result<usize>
where
    R: BufRead,
    D: Ops
```

