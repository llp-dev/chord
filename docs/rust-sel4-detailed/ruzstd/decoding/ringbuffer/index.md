*[ruzstd](../../index.md) / [decoding](../index.md) / [ringbuffer](index.md)*

---

# Module `ringbuffer`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`RingBuffer`](#ringbuffer) | struct |  |
| [`copy_bytes_overshooting`](#copy-bytes-overshooting) | fn | Similar to ptr::copy_nonoverlapping |
| [`copy_without_checks`](#copy-without-checks) | fn |  |
| [`copy_with_checks`](#copy-with-checks) | fn |  |
| [`copy_with_nobranch_check`](#copy-with-nobranch-check) | fn |  |

## Structs

### `RingBuffer`

```rust
struct RingBuffer {
    buf: core::ptr::NonNull<u8>,
    cap: usize,
    head: usize,
    tail: usize,
}
```

#### Implementations

- <span id="ringbuffer-new"></span>`fn new() -> Self`

- <span id="ringbuffer-len"></span>`fn len(&self) -> usize`

  Return the number of bytes in the buffer.

- <span id="ringbuffer-free"></span>`fn free(&self) -> usize`

  Return the amount of available space (in bytes) of the buffer.

- <span id="ringbuffer-clear"></span>`fn clear(&mut self)`

  Empty the buffer and reset the head and tail.

- <span id="ringbuffer-reserve"></span>`fn reserve(&mut self, amount: usize)`

  Ensure that there's space for `amount` elements in the buffer.

- <span id="ringbuffer-reserve-amortized"></span>`fn reserve_amortized(&mut self, amount: usize)`

- <span id="ringbuffer-push-back"></span>`fn push_back(&mut self, byte: u8)`

- <span id="ringbuffer-get"></span>`fn get(&self, idx: usize) -> Option<u8>`

  Fetch the byte stored at the selected index from the buffer, returning it, or

  `None` if the index is out of bounds.

- <span id="ringbuffer-extend"></span>`fn extend(&mut self, data: &[u8])`

  Append the provided data to the end of `self`.

- <span id="ringbuffer-drop-first-n"></span>`fn drop_first_n(&mut self, amount: usize)`

  Advance head past `amount` elements, effectively removing

  them from the buffer.

- <span id="ringbuffer-data-slice-lengths"></span>`fn data_slice_lengths(&self) -> (usize, usize)`

  Return the size of the two contiguous occupied sections of memory used

  by the buffer.

- <span id="ringbuffer-data-slice-parts"></span>`fn data_slice_parts(&self) -> ((*const u8, usize), (*const u8, usize))`

  Return pointers to the head and tail, and the length of each section.

- <span id="ringbuffer-as-slices"></span>`fn as_slices(&self) -> (&[u8], &[u8])`

  Return references to each part of the ring buffer.

- <span id="ringbuffer-free-slice-lengths"></span>`fn free_slice_lengths(&self) -> (usize, usize)`

  Returns the size of the two unoccupied sections of memory used by the buffer.

- <span id="ringbuffer-free-slice-parts"></span>`fn free_slice_parts(&self) -> ((*mut u8, usize), (*mut u8, usize))`

  Returns mutable references to the available space and the size of that available space,

  for the two sections in the buffer.

- <span id="ringbuffer-extend-from-within"></span>`fn extend_from_within(&mut self, start: usize, len: usize)`

  Copies elements from the provided range to the end of the buffer.

- <span id="ringbuffer-extend-from-within-unchecked"></span>`unsafe fn extend_from_within_unchecked(&mut self, start: usize, len: usize)`

  Copies data from the provided range to the end of the buffer, without

  first verifying that the unoccupied capacity is available.

  

  SAFETY:

  For this to be safe two requirements need to hold:

  1. start + len <= self.len() so we do not copy uninitialised memory

  2. More then len reserved space so we do not write out-of-bounds

- <span id="ringbuffer-extend-from-within-unchecked-branchless"></span>`unsafe fn extend_from_within_unchecked_branchless(&mut self, start: usize, len: usize)`

  This function is functionally the same as [RingBuffer::extend_from_within_unchecked],

  but it does not contain any branching operations.

  

  SAFETY:

  Needs start + len <= self.len()

  And more then len reserved space

#### Trait Implementations

##### `impl Drop for RingBuffer`

- <span id="ringbuffer-drop"></span>`fn drop(&mut self)`

##### `impl Send for RingBuffer`

##### `impl Sync for RingBuffer`

## Functions

### `copy_bytes_overshooting`

```rust
unsafe fn copy_bytes_overshooting(src: (*const u8, usize), dst: (*mut u8, usize), copy_at_least: usize)
```

Similar to ptr::copy_nonoverlapping

But it might overshoot the desired copy length if deemed useful

src and dst specify the entire length they are eligible for reading/writing respectively
in addition to the desired copy length.

This function will then copy in chunks and might copy up to chunk size - 1 more bytes from src to dst
if that operation does not read/write memory that does not belong to src/dst.

The chunk size is not part of the contract and may change depending on the target platform.

If that isn't possible we just fall back to ptr::copy_nonoverlapping

### `copy_without_checks`

```rust
unsafe fn copy_without_checks(m1_ptr: *const u8, m2_ptr: *const u8, f1_ptr: *mut u8, f2_ptr: *mut u8, m1_in_f1: usize, m2_in_f1: usize, m1_in_f2: usize, m2_in_f2: usize)
```

### `copy_with_checks`

```rust
unsafe fn copy_with_checks(m1_ptr: *const u8, m2_ptr: *const u8, f1_ptr: *mut u8, f2_ptr: *mut u8, m1_in_f1: usize, m2_in_f1: usize, m1_in_f2: usize, m2_in_f2: usize)
```

### `copy_with_nobranch_check`

```rust
unsafe fn copy_with_nobranch_check(m1_ptr: *const u8, m2_ptr: *const u8, f1_ptr: *mut u8, f2_ptr: *mut u8, m1_in_f1: usize, m2_in_f1: usize, m1_in_f2: usize, m2_in_f2: usize)
```

