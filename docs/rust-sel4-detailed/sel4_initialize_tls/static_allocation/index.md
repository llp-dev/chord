*[sel4_initialize_tls](../index.md) / [static_allocation](index.md)*

---

# Module `static_allocation`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`StaticTlsAllocation`](#statictlsallocation) | struct |  |

## Structs

### `StaticTlsAllocation<const N: usize, A>`

```rust
struct StaticTlsAllocation<const N: usize, A> {
    _alignment: [A; 0],
    space: core::cell::UnsafeCell<[u8; N]>,
}
```

#### Implementations

- <span id="statictlsallocation-new"></span>`const fn new() -> Self`

- <span id="statictlsallocation-size"></span>`const fn size(&self) -> usize`

- <span id="statictlsallocation-start"></span>`const fn start(&self) -> *mut u8`

- <span id="statictlsallocation-region"></span>`const fn region(&self) -> Region` — [`Region`](../index.md#region)

#### Trait Implementations

##### `impl<A> Default for StaticTlsAllocation<N, A>`

- <span id="statictlsallocation-default"></span>`fn default() -> Self`

##### `impl<A> Sync for StaticTlsAllocation<N, A>`

