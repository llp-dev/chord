# Crate `dlmalloc`

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

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`dlmalloc`](#dlmalloc) | mod |  |
| [`sys`](#sys) | mod |  |
| [`Dlmalloc`](#dlmalloc) | struct | An allocator instance |
| [`Allocator`](#allocator) | trait | In order for this crate to efficiently manage memory, it needs a way to communicate with the underlying platform. |

## Modules

- [`dlmalloc`](dlmalloc/index.md)
- [`sys`](sys/index.md)

## Structs

### `Dlmalloc<A>`

```rust
struct Dlmalloc<A>(dlmalloc::Dlmalloc<A>);
```

An allocator instance

Instances of this type are used to allocate blocks of memory. For best
results only use one of these. Currently doesn't implement `Drop` to release
lingering memory back to the OS. That may happen eventually though!

#### Implementations

- <span id="dlmalloc-new"></span>`const fn new() -> Dlmalloc<System>` — [`Dlmalloc`](#dlmalloc), [`System`](sys/index.md#system)

  Creates a new instance of an allocator

## Traits

### `Allocator`

```rust
trait Allocator: Send { ... }
```

In order for this crate to efficiently manage memory, it needs a way to communicate with the
underlying platform. This `Allocator` trait provides an interface for this communication.

#### Required Methods

- `fn alloc(&self, size: usize) -> (*mut u8, usize, u32)`

  Allocates system memory region of at least `size` bytes

- `fn remap(&self, ptr: *mut u8, oldsize: usize, newsize: usize, can_move: bool) -> *mut u8`

  Remaps system memory region at `ptr` with size `oldsize` to a potential new location with

- `fn free_part(&self, ptr: *mut u8, oldsize: usize, newsize: usize) -> bool`

  Frees a part of a memory chunk. The original memory chunk starts at `ptr` with size `oldsize`

- `fn free(&self, ptr: *mut u8, size: usize) -> bool`

  Frees an entire memory region. Returns `true` iff the operation succeeded. When `false` is

- `fn can_release_part(&self, flags: u32) -> bool`

  Indicates if the system can release a part of memory. For the `flags` argument, see

- `fn allocates_zeros(&self) -> bool`

  Indicates whether newly allocated regions contain zeros.

- `fn page_size(&self) -> usize`

  Returns the page size. Must be a power of two

#### Implementors

- [`System`](sys/index.md#system)

