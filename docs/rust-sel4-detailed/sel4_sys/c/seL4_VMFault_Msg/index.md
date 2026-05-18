*[sel4_sys](../../index.md) / [c](../index.md) / [seL4_VMFault_Msg](index.md)*

---

# Module `seL4_VMFault_Msg`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Type`](#type) | type |  |
| [`seL4_VMFault_IP`](#sel4-vmfault-ip) | const |  |
| [`seL4_VMFault_Addr`](#sel4-vmfault-addr) | const |  |
| [`seL4_VMFault_PrefetchFault`](#sel4-vmfault-prefetchfault) | const |  |
| [`seL4_VMFault_FSR`](#sel4-vmfault-fsr) | const |  |
| [`seL4_VMFault_Length`](#sel4-vmfault-length) | const |  |
| [`_enum_pad_seL4_VMFault_Msg`](#enum-pad-sel4-vmfault-msg) | const |  |

## Type Aliases

### `Type`

```rust
type Type = ::core::ffi::c_ulong;
```

## Constants

### `seL4_VMFault_IP`
```rust
const seL4_VMFault_IP: Type = 0u64;
```

### `seL4_VMFault_Addr`
```rust
const seL4_VMFault_Addr: Type = 1u64;
```

### `seL4_VMFault_PrefetchFault`
```rust
const seL4_VMFault_PrefetchFault: Type = 2u64;
```

### `seL4_VMFault_FSR`
```rust
const seL4_VMFault_FSR: Type = 3u64;
```

### `seL4_VMFault_Length`
```rust
const seL4_VMFault_Length: Type = 4u64;
```

### `_enum_pad_seL4_VMFault_Msg`
```rust
const _enum_pad_seL4_VMFault_Msg: Type = 9_223_372_036_854_775_807u64;
```

