*[sel4_sys](../../index.md) / [c](../index.md) / [seL4_X86_EPT_VMAttributes](index.md)*

---

# Module `seL4_X86_EPT_VMAttributes`

## Contents

- [Type Aliases](#type-aliases)
  - [`Type`](#type)
- [Constants](#constants)
  - [`seL4_X86_EPT_Uncached_VMAttributes`](#sel4-x86-ept-uncached-vmattributes)
  - [`seL4_X86_EPT_Uncacheable`](#sel4-x86-ept-uncacheable)
  - [`seL4_X86_EPT_WriteCombining`](#sel4-x86-ept-writecombining)
  - [`seL4_X86_EPT_WriteThrough`](#sel4-x86-ept-writethrough)
  - [`seL4_X86_EPT_WriteProtected`](#sel4-x86-ept-writeprotected)
  - [`seL4_X86_EPT_WriteBack`](#sel4-x86-ept-writeback)
  - [`seL4_X86_EPT_Default_VMAttributes`](#sel4-x86-ept-default-vmattributes)
  - [`_enum_pad_seL4_X86_EPT_VMAttributes`](#enum-pad-sel4-x86-ept-vmattributes)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Type`](#type) | type |  |
| [`seL4_X86_EPT_Uncached_VMAttributes`](#sel4-x86-ept-uncached-vmattributes) | const |  |
| [`seL4_X86_EPT_Uncacheable`](#sel4-x86-ept-uncacheable) | const |  |
| [`seL4_X86_EPT_WriteCombining`](#sel4-x86-ept-writecombining) | const |  |
| [`seL4_X86_EPT_WriteThrough`](#sel4-x86-ept-writethrough) | const |  |
| [`seL4_X86_EPT_WriteProtected`](#sel4-x86-ept-writeprotected) | const |  |
| [`seL4_X86_EPT_WriteBack`](#sel4-x86-ept-writeback) | const |  |
| [`seL4_X86_EPT_Default_VMAttributes`](#sel4-x86-ept-default-vmattributes) | const |  |
| [`_enum_pad_seL4_X86_EPT_VMAttributes`](#enum-pad-sel4-x86-ept-vmattributes) | const |  |

## Type Aliases

### `Type`

```rust
type Type = ::core::ffi::c_ulong;
```

## Constants

### `seL4_X86_EPT_Uncached_VMAttributes`
```rust
const seL4_X86_EPT_Uncached_VMAttributes: Type = 6u64;
```

### `seL4_X86_EPT_Uncacheable`
```rust
const seL4_X86_EPT_Uncacheable: Type = 0u64;
```

### `seL4_X86_EPT_WriteCombining`
```rust
const seL4_X86_EPT_WriteCombining: Type = 1u64;
```

### `seL4_X86_EPT_WriteThrough`
```rust
const seL4_X86_EPT_WriteThrough: Type = 4u64;
```

### `seL4_X86_EPT_WriteProtected`
```rust
const seL4_X86_EPT_WriteProtected: Type = 5u64;
```

### `seL4_X86_EPT_WriteBack`
```rust
const seL4_X86_EPT_WriteBack: Type = 6u64;
```

### `seL4_X86_EPT_Default_VMAttributes`
```rust
const seL4_X86_EPT_Default_VMAttributes: Type = 6u64;
```

### `_enum_pad_seL4_X86_EPT_VMAttributes`
```rust
const _enum_pad_seL4_X86_EPT_VMAttributes: Type = 9_223_372_036_854_775_807u64;
```

