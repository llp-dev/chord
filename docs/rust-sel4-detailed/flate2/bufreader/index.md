*[flate2](../index.md) / [bufreader](index.md)*

---

# Module `bufreader`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`BufReader`](#bufreader) | struct |  |

## Structs

### `BufReader<R>`

```rust
struct BufReader<R> {
    inner: R,
    buf: Box<[u8]>,
    pos: usize,
    cap: usize,
}
```

#### Implementations

- <span id="bufreader-new"></span>`fn new(inner: R) -> BufReader<R>` — [`BufReader`](#bufreader)

- <span id="bufreader-with-buf"></span>`fn with_buf(buf: Vec<u8>, inner: R) -> BufReader<R>` — [`BufReader`](#bufreader)

#### Trait Implementations

##### `impl<R: Read> BufRead for BufReader<R>`

- <span id="bufreader-bufread-fill-buf"></span>`fn fill_buf(&mut self) -> io::Result<&[u8]>`

- <span id="bufreader-bufread-consume"></span>`fn consume(&mut self, amt: usize)`

##### `impl<R> Debug for BufReader<R>`

- <span id="bufreader-debug-fmt"></span>`fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error>`

##### `impl<R: Read> Read for BufReader<R>`

- <span id="bufreader-read"></span>`fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>`

