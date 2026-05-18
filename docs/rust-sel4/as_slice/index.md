# as_slice

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

## Modules

### [`as_slice`](as_slice.md)

*2 traits*

