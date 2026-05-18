# Crate `as_slice`

`AsSlice` and `AsMutSlice` traits

These traits are somewhat similar to the `AsRef` and `AsMut` except that they are **NOT**
polymorphic (no input type parameter) and their methods always return slices (`[T]`).

The main use case of these traits is writing generic code that accepts (fixed size) buffers. For
example, a bound `T: StableDeref + AsMutSlice<Element = u8> + 'static` will accepts types like
`&'static mut [u8]` and `&'static mut [u8; 128]` -- all
of them are appropriate for DMA transfers.

# Minimal Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.51 and up. It *might* compile on older
versions but that may change in any new patch release.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`AsSlice`](#asslice) | trait | Something that can be seen as an immutable slice |
| [`AsMutSlice`](#asmutslice) | trait | Something that can be seen as an mutable slice |

## Traits

### `AsSlice`

```rust
trait AsSlice { ... }
```

Something that can be seen as an immutable slice

#### Associated Types

- `type Element`

#### Required Methods

- `fn as_slice(&self) -> &[<Self as >::Element]`

  Returns the immutable slice view of `Self`

#### Implementors

- `&'a S`
- `&'a mut S`
- `[T; N]`
- `[T]`

### `AsMutSlice`

```rust
trait AsMutSlice: AsSlice { ... }
```

Something that can be seen as an mutable slice

#### Required Methods

- `fn as_mut_slice(&mut self) -> &mut [<Self as >::Element]`

  Returns the mutable slice view of `Self`

#### Implementors

- `&'a mut S`
- `[T; N]`
- `[T]`

