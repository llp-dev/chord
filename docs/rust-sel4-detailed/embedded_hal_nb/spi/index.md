*[embedded_hal_nb](../index.md) / [spi](index.md)*

---

# Module `spi`

SPI master mode traits using `nb`.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Polarity`](#polarity) | struct |  |
| [`FullDuplex`](#fullduplex) | trait | Full duplex SPI (master mode). |
| [`ErrorType`](#errortype) | fn |  |

## Structs

### `Polarity<'ctx, R>`

```rust
struct Polarity<'ctx, R>
where
    R: gimli::Reader {
    unit: &'ctx crate::unit::ResUnit<R>,
    sections: &'ctx gimli::Dwarf<R>,
    function: &'ctx crate::function::Function<R>,
    inlined_functions: iter::Rev<smallvec::IntoIter<[&'ctx crate::function::InlinedFunction<R>; 16]>>,
    next: Option<Location<'ctx>>,
}
```

*Re-exported from `addr2line`*

## Traits

### `FullDuplex<Word: Copy>`

```rust
trait FullDuplex<Word: Copy>: ErrorType { ... }
```

Full duplex SPI (master mode).

# Notes

- It's the task of the user of this interface to manage the slave select lines.

- Due to how full duplex SPI works each `read` call must be preceded by a `write` call.

- `read` calls only return the data received with the last `write` call.
Previously received data is discarded

- Data is only guaranteed to be clocked out when the `read` call succeeds.
The slave select line shouldn't be released before that.

- Some SPIs can work with 8-bit *and* 16-bit words. You can overload this trait with different
`Word` types to allow operation in both modes.

#### Required Methods

- `fn read(&mut self) -> nb::Result<Word, <Self as >::Error>`

  Reads the word stored in the shift register

- `fn write(&mut self, word: Word) -> nb::Result<(), <Self as >::Error>`

  Writes a word to the slave

#### Implementors

- `&mut T`

## Functions

### `ErrorType`

```rust
fn ErrorType(&mut self) -> Result<Option<Frame<'ctx, R>>, gimli::Error>
```

