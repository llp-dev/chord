*[sel4_sys](../../index.md) / [c](../index.md) / [seL4_X86_VMAttributes](index.md)*

---

# Module `seL4_X86_VMAttributes`

## Contents

- [Type Aliases](#type-aliases)
  - [`Type`](#type)
- [Constants](#constants)
  - [`seL4_X86_Default_VMAttributes`](#sel4-x86-default-vmattributes)
  - [`seL4_X86_WriteBack`](#sel4-x86-writeback)
  - [`seL4_X86_WriteThrough`](#sel4-x86-writethrough)
  - [`seL4_X86_CacheDisabled`](#sel4-x86-cachedisabled)
  - [`seL4_X86_Uncacheable`](#sel4-x86-uncacheable)
  - [`seL4_X86_WriteCombining`](#sel4-x86-writecombining)
  - [`_enum_pad_seL4_X86_VMAttributes`](#enum-pad-sel4-x86-vmattributes)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Type`](#type) | type |  |
| [`seL4_X86_Default_VMAttributes`](#sel4-x86-default-vmattributes) | const |  |
| [`seL4_X86_WriteBack`](#sel4-x86-writeback) | const |  |
| [`seL4_X86_WriteThrough`](#sel4-x86-writethrough) | const |  |
| [`seL4_X86_CacheDisabled`](#sel4-x86-cachedisabled) | const |  |
| [`seL4_X86_Uncacheable`](#sel4-x86-uncacheable) | const |  |
| [`seL4_X86_WriteCombining`](#sel4-x86-writecombining) | const |  |
| [`_enum_pad_seL4_X86_VMAttributes`](#enum-pad-sel4-x86-vmattributes) | const |  |

## Type Aliases

### `Type`

```rust
type Type = ::core::ffi::c_ulong;
```

## Constants

### `seL4_X86_Default_VMAttributes`
```rust
const seL4_X86_Default_VMAttributes: Type = 0u64;
```

### `seL4_X86_WriteBack`
```rust
const seL4_X86_WriteBack: Type = 0u64;
```

### `seL4_X86_WriteThrough`
```rust
const seL4_X86_WriteThrough: Type = 1u64;
```

### `seL4_X86_CacheDisabled`
```rust
const seL4_X86_CacheDisabled: Type = 2u64;
```

### `seL4_X86_Uncacheable`
```rust
const seL4_X86_Uncacheable: Type = 3u64;
```

### `seL4_X86_WriteCombining`
```rust
const seL4_X86_WriteCombining: Type = 4u64;
```

### `_enum_pad_seL4_X86_VMAttributes`
```rust
const _enum_pad_seL4_X86_VMAttributes: Type = 9_223_372_036_854_775_807u64;
```

