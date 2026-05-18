*[unwinding](../index.md) / [util](index.md)*

---

# Module `util`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`get_unlimited_slice`](#get-unlimited-slice) | fn |  |
| [`deref_pointer`](#deref-pointer) | fn |  |
| [`StaticSlice`](#staticslice) | type |  |
| [`c_int`](#c-int) | type |  |

## Functions

### `get_unlimited_slice`

```rust
unsafe fn get_unlimited_slice<'a>(start: *const u8) -> &'a [u8]
```

### `deref_pointer`

```rust
unsafe fn deref_pointer(ptr: gimli::Pointer) -> usize
```

## Type Aliases

### `StaticSlice`

```rust
type StaticSlice = gimli::EndianSlice<'static, gimli::NativeEndian>;
```

### `c_int`

```rust
type c_int = i32;
```

