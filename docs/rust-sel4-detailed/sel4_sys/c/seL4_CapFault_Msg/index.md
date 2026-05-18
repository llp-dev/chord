*[sel4_sys](../../index.md) / [c](../index.md) / [seL4_CapFault_Msg](index.md)*

---

# Module `seL4_CapFault_Msg`

## Contents

- [Type Aliases](#type-aliases)
  - [`Type`](#type)
- [Constants](#constants)
  - [`seL4_CapFault_IP`](#sel4-capfault-ip)
  - [`seL4_CapFault_Addr`](#sel4-capfault-addr)
  - [`seL4_CapFault_InRecvPhase`](#sel4-capfault-inrecvphase)
  - [`seL4_CapFault_LookupFailureType`](#sel4-capfault-lookupfailuretype)
  - [`seL4_CapFault_BitsLeft`](#sel4-capfault-bitsleft)
  - [`seL4_CapFault_DepthMismatch_BitsFound`](#sel4-capfault-depthmismatch-bitsfound)
  - [`seL4_CapFault_GuardMismatch_GuardFound`](#sel4-capfault-guardmismatch-guardfound)
  - [`seL4_CapFault_GuardMismatch_BitsFound`](#sel4-capfault-guardmismatch-bitsfound)
  - [`_enum_pad_seL4_CapFault_Msg`](#enum-pad-sel4-capfault-msg)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Type`](#type) | type |  |
| [`seL4_CapFault_IP`](#sel4-capfault-ip) | const |  |
| [`seL4_CapFault_Addr`](#sel4-capfault-addr) | const |  |
| [`seL4_CapFault_InRecvPhase`](#sel4-capfault-inrecvphase) | const |  |
| [`seL4_CapFault_LookupFailureType`](#sel4-capfault-lookupfailuretype) | const |  |
| [`seL4_CapFault_BitsLeft`](#sel4-capfault-bitsleft) | const |  |
| [`seL4_CapFault_DepthMismatch_BitsFound`](#sel4-capfault-depthmismatch-bitsfound) | const |  |
| [`seL4_CapFault_GuardMismatch_GuardFound`](#sel4-capfault-guardmismatch-guardfound) | const |  |
| [`seL4_CapFault_GuardMismatch_BitsFound`](#sel4-capfault-guardmismatch-bitsfound) | const |  |
| [`_enum_pad_seL4_CapFault_Msg`](#enum-pad-sel4-capfault-msg) | const |  |

## Type Aliases

### `Type`

```rust
type Type = ::core::ffi::c_ulong;
```

## Constants

### `seL4_CapFault_IP`
```rust
const seL4_CapFault_IP: Type = 0u64;
```

### `seL4_CapFault_Addr`
```rust
const seL4_CapFault_Addr: Type = 1u64;
```

### `seL4_CapFault_InRecvPhase`
```rust
const seL4_CapFault_InRecvPhase: Type = 2u64;
```

### `seL4_CapFault_LookupFailureType`
```rust
const seL4_CapFault_LookupFailureType: Type = 3u64;
```

### `seL4_CapFault_BitsLeft`
```rust
const seL4_CapFault_BitsLeft: Type = 4u64;
```

### `seL4_CapFault_DepthMismatch_BitsFound`
```rust
const seL4_CapFault_DepthMismatch_BitsFound: Type = 5u64;
```

### `seL4_CapFault_GuardMismatch_GuardFound`
```rust
const seL4_CapFault_GuardMismatch_GuardFound: Type = 5u64;
```

### `seL4_CapFault_GuardMismatch_BitsFound`
```rust
const seL4_CapFault_GuardMismatch_BitsFound: Type = 6u64;
```

### `_enum_pad_seL4_CapFault_Msg`
```rust
const _enum_pad_seL4_CapFault_Msg: Type = 9_223_372_036_854_775_807u64;
```

