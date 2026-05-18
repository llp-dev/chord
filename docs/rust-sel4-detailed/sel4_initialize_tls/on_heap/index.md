*[sel4_initialize_tls](../index.md) / [on_heap](index.md)*

---

# Module `on_heap`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`HeapTlsReservation`](#heaptlsreservation) | struct |  |

## Structs

### `HeapTlsReservation`

```rust
struct HeapTlsReservation {
    start: *mut u8,
    layout: core::alloc::Layout,
    thread_pointer: usize,
}
```

#### Implementations

- <span id="heaptlsreservation-initialize"></span>`fn initialize(tls_image: &TlsImage) -> Self` — [`TlsImage`](../index.md#tlsimage)

- <span id="heaptlsreservation-thread-pointer"></span>`fn thread_pointer(&self) -> usize`

#### Trait Implementations

##### `impl Drop for HeapTlsReservation`

- <span id="heaptlsreservation-drop"></span>`fn drop(&mut self)`

