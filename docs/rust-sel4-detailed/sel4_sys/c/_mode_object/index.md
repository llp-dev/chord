*[sel4_sys](../../index.md) / [c](../index.md) / [_mode_object](index.md)*

---

# Module `_mode_object`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Type`](#type) | type |  |
| [`seL4_X86_PDPTObject`](#sel4-x86-pdptobject) | const |  |
| [`seL4_X64_PML4Object`](#sel4-x64-pml4object) | const |  |
| [`seL4_X64_HugePageObject`](#sel4-x64-hugepageobject) | const |  |
| [`seL4_ModeObjectTypeCount`](#sel4-modeobjecttypecount) | const |  |

## Type Aliases

### `Type`

```rust
type Type = ::core::ffi::c_uint;
```

## Constants

### `seL4_X86_PDPTObject`
```rust
const seL4_X86_PDPTObject: Type = 5u32;
```

### `seL4_X64_PML4Object`
```rust
const seL4_X64_PML4Object: Type = 6u32;
```

### `seL4_X64_HugePageObject`
```rust
const seL4_X64_HugePageObject: Type = 7u32;
```

### `seL4_ModeObjectTypeCount`
```rust
const seL4_ModeObjectTypeCount: Type = 8u32;
```

