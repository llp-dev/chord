*[sel4_sys](../../index.md) / [c](../index.md) / [seL4_LookupFailureType](index.md)*

---

# Module `seL4_LookupFailureType`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Type`](#type) | type |  |
| [`seL4_NoFailure`](#sel4-nofailure) | const |  |
| [`seL4_InvalidRoot`](#sel4-invalidroot) | const |  |
| [`seL4_MissingCapability`](#sel4-missingcapability) | const |  |
| [`seL4_DepthMismatch`](#sel4-depthmismatch) | const |  |
| [`seL4_GuardMismatch`](#sel4-guardmismatch) | const |  |
| [`_enum_pad_seL4_LookupFailureType`](#enum-pad-sel4-lookupfailuretype) | const |  |

## Type Aliases

### `Type`

```rust
type Type = ::core::ffi::c_ulong;
```

## Constants

### `seL4_NoFailure`
```rust
const seL4_NoFailure: Type = 0u64;
```

### `seL4_InvalidRoot`
```rust
const seL4_InvalidRoot: Type = 1u64;
```

### `seL4_MissingCapability`
```rust
const seL4_MissingCapability: Type = 2u64;
```

### `seL4_DepthMismatch`
```rust
const seL4_DepthMismatch: Type = 3u64;
```

### `seL4_GuardMismatch`
```rust
const seL4_GuardMismatch: Type = 4u64;
```

### `_enum_pad_seL4_LookupFailureType`
```rust
const _enum_pad_seL4_LookupFailureType: Type = 9_223_372_036_854_775_807u64;
```

