*[dlmalloc](../index.md) / [sys](index.md)*

---

# Module `sys`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`System`](#system) | struct | System setting for Linux |

## Structs

### `System`

```rust
struct System {
    _priv: (),
}
```

System setting for Linux

#### Implementations

- <span id="system-new"></span>`const fn new() -> System` — [`System`](#system)

#### Trait Implementations

##### `impl Allocator for System`

- <span id="system-allocator-alloc"></span>`fn alloc(&self, size: usize) -> (*mut u8, usize, u32)`

- <span id="system-allocator-remap"></span>`fn remap(&self, ptr: *mut u8, oldsize: usize, newsize: usize, can_move: bool) -> *mut u8`

- <span id="system-allocator-free-part"></span>`fn free_part(&self, ptr: *mut u8, oldsize: usize, newsize: usize) -> bool`

- <span id="system-allocator-free"></span>`fn free(&self, ptr: *mut u8, size: usize) -> bool`

- <span id="system-allocator-can-release-part"></span>`fn can_release_part(&self, _flags: u32) -> bool`

- <span id="system-allocator-allocates-zeros"></span>`fn allocates_zeros(&self) -> bool`

- <span id="system-allocator-page-size"></span>`fn page_size(&self) -> usize`

