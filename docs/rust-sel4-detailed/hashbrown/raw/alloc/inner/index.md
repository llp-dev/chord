*[hashbrown](../../../index.md) / [raw](../../index.md) / [alloc](../index.md) / [inner](index.md)*

---

# Module `inner`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Global`](#global) | struct |  |
| [`Allocator`](#allocator) | fn |  |
| [`do_alloc`](#do-alloc) | fn |  |

## Structs

### `Global<'a, R: gimli::Reader>`

```rust
struct Global<'a, R: gimli::Reader> {
    entries: gimli::EntriesRaw<'a, R>,
    functions: alloc::vec::Vec<InlinedFunction<R>>,
    addresses: alloc::vec::Vec<InlinedFunctionAddress>,
    file: crate::DebugFile,
    unit: gimli::UnitRef<'a, R>,
    ctx: &'a crate::Context<R>,
}
```

*Re-exported from `addr2line`*

## Functions

### `Allocator`

```rust
fn Allocator(dw_die_offset: gimli::UnitOffset<<R as >::Offset>, file: DebugFile, unit: gimli::UnitRef<'_, R>, ctx: &Context<R>) -> Result<Self, gimli::Error>
```

### `do_alloc`

```rust
fn do_alloc<A: Allocator>(alloc: &A, layout: crate::alloc::alloc::Layout) -> Result<core::ptr::NonNull<[u8]>, ()>
```

