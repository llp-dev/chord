# dlmalloc

A Rust port of the `dlmalloc` allocator.

The `dlmalloc` allocator is described at
<https://gee.cs.oswego.edu/dl/html/malloc.html> and this Rust crate is a straight
port of the C code for the allocator into Rust. The implementation is
wrapped up in a `Dlmalloc` type and has support for Linux, OSX, and Wasm
currently.

The primary purpose of this crate is that it serves as the default memory
allocator for the `wasm32-unknown-unknown` target in the standard library.
Support for other platforms is largely untested and unused, but is used when
testing this crate.

## Modules

### [`dlmalloc`](dlmalloc.md)

*1 struct, 1 trait*

### [`sys`](sys.md)

*1 struct*

