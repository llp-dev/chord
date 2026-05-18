*[dlmalloc](../index.md) / [dlmalloc](index.md)*

---

# Module `dlmalloc`

## Contents

- [Structs](#structs)
  - [`Dlmalloc`](#dlmalloc)
  - [`Chunk`](#chunk)
  - [`TreeChunk`](#treechunk)
  - [`Segment`](#segment)
- [Functions](#functions)
  - [`align_up`](#align-up)
  - [`left_bits`](#left-bits)
  - [`least_bit`](#least-bit)
  - [`leftshift_for_tree_index`](#leftshift-for-tree-index)
- [Constants](#constants)
  - [`NSMALLBINS`](#nsmallbins)
  - [`NTREEBINS`](#ntreebins)
  - [`SMALLBIN_SHIFT`](#smallbin-shift)
  - [`TREEBIN_SHIFT`](#treebin-shift)
  - [`NSMALLBINS_U32`](#nsmallbins-u32)
  - [`NTREEBINS_U32`](#ntreebins-u32)
  - [`DEFAULT_GRANULARITY`](#default-granularity)
  - [`DEFAULT_TRIM_THRESHOLD`](#default-trim-threshold)
  - [`MAX_RELEASE_CHECK_RATE`](#max-release-check-rate)
  - [`PINUSE`](#pinuse)
  - [`CINUSE`](#cinuse)
  - [`FLAG4`](#flag4)
  - [`INUSE`](#inuse)
  - [`FLAG_BITS`](#flag-bits)
  - [`EXTERN`](#extern)
- [Macros](#macros)
  - [`debug_assert!`](#debug-assert)
  - [`debug_assert_eq!`](#debug-assert-eq)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Dlmalloc`](#dlmalloc) | struct |  |
| [`Chunk`](#chunk) | struct |  |
| [`TreeChunk`](#treechunk) | struct |  |
| [`Segment`](#segment) | struct |  |
| [`align_up`](#align-up) | fn |  |
| [`left_bits`](#left-bits) | fn |  |
| [`least_bit`](#least-bit) | fn |  |
| [`leftshift_for_tree_index`](#leftshift-for-tree-index) | fn |  |
| [`NSMALLBINS`](#nsmallbins) | const |  |
| [`NTREEBINS`](#ntreebins) | const |  |
| [`SMALLBIN_SHIFT`](#smallbin-shift) | const |  |
| [`TREEBIN_SHIFT`](#treebin-shift) | const |  |
| [`NSMALLBINS_U32`](#nsmallbins-u32) | const |  |
| [`NTREEBINS_U32`](#ntreebins-u32) | const |  |
| [`DEFAULT_GRANULARITY`](#default-granularity) | const |  |
| [`DEFAULT_TRIM_THRESHOLD`](#default-trim-threshold) | const |  |
| [`MAX_RELEASE_CHECK_RATE`](#max-release-check-rate) | const |  |
| [`PINUSE`](#pinuse) | const |  |
| [`CINUSE`](#cinuse) | const |  |
| [`FLAG4`](#flag4) | const |  |
| [`INUSE`](#inuse) | const |  |
| [`FLAG_BITS`](#flag-bits) | const |  |
| [`EXTERN`](#extern) | const |  |
| [`debug_assert!`](#debug-assert) | macro |  |
| [`debug_assert_eq!`](#debug-assert-eq) | macro |  |

## Structs

### `Dlmalloc<A>`

```rust
struct Dlmalloc<A> {
    smallmap: u32,
    treemap: u32,
    smallbins: [*mut Chunk; 66],
    treebins: [*mut TreeChunk; 32],
    dvsize: usize,
    topsize: usize,
    dv: *mut Chunk,
    top: *mut Chunk,
    footprint: usize,
    max_footprint: usize,
    seg: Segment,
    trim_check: usize,
    least_addr: *mut u8,
    release_checks: usize,
    system_allocator: A,
}
```

#### Implementations

- <span id="dlmalloc-new"></span>`const fn new(system_allocator: A) -> Dlmalloc<A>` — [`Dlmalloc`](#dlmalloc)

- <span id="dlmalloc-allocator"></span>`fn allocator(&self) -> &A`

- <span id="dlmalloc-allocator-mut"></span>`fn allocator_mut(&mut self) -> &mut A`

#### Trait Implementations

##### `impl<A: Send> Send for Dlmalloc<A>`

### `Chunk`

```rust
struct Chunk {
    prev_foot: usize,
    head: usize,
    prev: *mut Chunk,
    next: *mut Chunk,
}
```

#### Implementations

- <span id="chunk-fencepost-head"></span>`unsafe fn fencepost_head() -> usize`

- <span id="chunk-size"></span>`unsafe fn size(me: *mut Chunk) -> usize` — [`Chunk`](#chunk)

- <span id="chunk-next"></span>`unsafe fn next(me: *mut Chunk) -> *mut Chunk` — [`Chunk`](#chunk)

- <span id="chunk-prev"></span>`unsafe fn prev(me: *mut Chunk) -> *mut Chunk` — [`Chunk`](#chunk)

- <span id="chunk-cinuse"></span>`unsafe fn cinuse(me: *mut Chunk) -> bool` — [`Chunk`](#chunk)

- <span id="chunk-pinuse"></span>`unsafe fn pinuse(me: *mut Chunk) -> bool` — [`Chunk`](#chunk)

- <span id="chunk-clear-pinuse"></span>`unsafe fn clear_pinuse(me: *mut Chunk)` — [`Chunk`](#chunk)

- <span id="chunk-inuse"></span>`unsafe fn inuse(me: *mut Chunk) -> bool` — [`Chunk`](#chunk)

- <span id="chunk-mmapped"></span>`unsafe fn mmapped(me: *mut Chunk) -> bool` — [`Chunk`](#chunk)

- <span id="chunk-set-inuse"></span>`unsafe fn set_inuse(me: *mut Chunk, size: usize)` — [`Chunk`](#chunk)

- <span id="chunk-set-inuse-and-pinuse"></span>`unsafe fn set_inuse_and_pinuse(me: *mut Chunk, size: usize)` — [`Chunk`](#chunk)

- <span id="chunk-set-size-and-pinuse-of-inuse-chunk"></span>`unsafe fn set_size_and_pinuse_of_inuse_chunk(me: *mut Chunk, size: usize)` — [`Chunk`](#chunk)

- <span id="chunk-set-size-and-pinuse-of-free-chunk"></span>`unsafe fn set_size_and_pinuse_of_free_chunk(me: *mut Chunk, size: usize)` — [`Chunk`](#chunk)

- <span id="chunk-set-free-with-pinuse"></span>`unsafe fn set_free_with_pinuse(p: *mut Chunk, size: usize, n: *mut Chunk)` — [`Chunk`](#chunk)

- <span id="chunk-set-foot"></span>`unsafe fn set_foot(me: *mut Chunk, size: usize)` — [`Chunk`](#chunk)

- <span id="chunk-plus-offset"></span>`unsafe fn plus_offset(me: *mut Chunk, offset: usize) -> *mut Chunk` — [`Chunk`](#chunk)

- <span id="chunk-minus-offset"></span>`unsafe fn minus_offset(me: *mut Chunk, offset: usize) -> *mut Chunk` — [`Chunk`](#chunk)

- <span id="chunk-to-mem"></span>`unsafe fn to_mem(me: *mut Chunk) -> *mut u8` — [`Chunk`](#chunk)

- <span id="chunk-mem-offset"></span>`fn mem_offset() -> usize`

- <span id="chunk-from-mem"></span>`unsafe fn from_mem(mem: *mut u8) -> *mut Chunk` — [`Chunk`](#chunk)

### `TreeChunk`

```rust
struct TreeChunk {
    chunk: Chunk,
    child: [*mut TreeChunk; 2],
    parent: *mut TreeChunk,
    index: u32,
}
```

#### Implementations

- <span id="treechunk-leftmost-child"></span>`unsafe fn leftmost_child(me: *mut TreeChunk) -> *mut TreeChunk` — [`TreeChunk`](#treechunk)

- <span id="treechunk-chunk"></span>`unsafe fn chunk(me: *mut TreeChunk) -> *mut Chunk` — [`TreeChunk`](#treechunk), [`Chunk`](#chunk)

- <span id="treechunk-next"></span>`unsafe fn next(me: *mut TreeChunk) -> *mut TreeChunk` — [`TreeChunk`](#treechunk)

- <span id="treechunk-prev"></span>`unsafe fn prev(me: *mut TreeChunk) -> *mut TreeChunk` — [`TreeChunk`](#treechunk)

### `Segment`

```rust
struct Segment {
    base: *mut u8,
    size: usize,
    next: *mut Segment,
    flags: u32,
}
```

#### Implementations

- <span id="segment-is-extern"></span>`unsafe fn is_extern(seg: *mut Segment) -> bool` — [`Segment`](#segment)

- <span id="segment-can-release-part"></span>`unsafe fn can_release_part<A: Allocator>(system_allocator: &A, seg: *mut Segment) -> bool` — [`Segment`](#segment)

- <span id="segment-sys-flags"></span>`unsafe fn sys_flags(seg: *mut Segment) -> u32` — [`Segment`](#segment)

- <span id="segment-holds"></span>`unsafe fn holds(seg: *mut Segment, addr: *mut u8) -> bool` — [`Segment`](#segment)

- <span id="segment-top"></span>`unsafe fn top(seg: *mut Segment) -> *mut u8` — [`Segment`](#segment)

#### Trait Implementations

##### `impl Clone for Segment`

- <span id="segment-clone"></span>`fn clone(&self) -> Segment` — [`Segment`](#segment)

##### `impl Copy for Segment`

## Functions

### `align_up`

```rust
fn align_up(a: usize, alignment: usize) -> usize
```

### `left_bits`

```rust
fn left_bits(x: u32) -> u32
```

### `least_bit`

```rust
fn least_bit(x: u32) -> u32
```

### `leftshift_for_tree_index`

```rust
fn leftshift_for_tree_index(x: u32) -> u32
```

## Constants

### `NSMALLBINS`
```rust
const NSMALLBINS: usize = 32usize;
```

### `NTREEBINS`
```rust
const NTREEBINS: usize = 32usize;
```

### `SMALLBIN_SHIFT`
```rust
const SMALLBIN_SHIFT: usize = 3usize;
```

### `TREEBIN_SHIFT`
```rust
const TREEBIN_SHIFT: usize = 8usize;
```

### `NSMALLBINS_U32`
```rust
const NSMALLBINS_U32: u32 = 32u32;
```

### `NTREEBINS_U32`
```rust
const NTREEBINS_U32: u32 = 32u32;
```

### `DEFAULT_GRANULARITY`
```rust
const DEFAULT_GRANULARITY: usize = 65_536usize;
```

### `DEFAULT_TRIM_THRESHOLD`
```rust
const DEFAULT_TRIM_THRESHOLD: usize = 2_097_152usize;
```

### `MAX_RELEASE_CHECK_RATE`
```rust
const MAX_RELEASE_CHECK_RATE: usize = 4_095usize;
```

### `PINUSE`
```rust
const PINUSE: usize = 1usize;
```

### `CINUSE`
```rust
const CINUSE: usize = 2usize;
```

### `FLAG4`
```rust
const FLAG4: usize = 4usize;
```

### `INUSE`
```rust
const INUSE: usize = 3usize;
```

### `FLAG_BITS`
```rust
const FLAG_BITS: usize = 7usize;
```

### `EXTERN`
```rust
const EXTERN: u32 = 1u32;
```

## Macros

### `debug_assert!`

### `debug_assert_eq!`

