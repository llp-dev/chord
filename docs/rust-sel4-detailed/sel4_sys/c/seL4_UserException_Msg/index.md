*[sel4_sys](../../index.md) / [c](../index.md) / [seL4_UserException_Msg](index.md)*

---

# Module `seL4_UserException_Msg`

## Contents

- [Type Aliases](#type-aliases)
  - [`Type`](#type)
- [Constants](#constants)
  - [`seL4_UserException_FaultIP`](#sel4-userexception-faultip)
  - [`seL4_UserException_SP`](#sel4-userexception-sp)
  - [`seL4_UserException_FLAGS`](#sel4-userexception-flags)
  - [`seL4_UserException_Number`](#sel4-userexception-number)
  - [`seL4_UserException_Code`](#sel4-userexception-code)
  - [`seL4_UserException_Length`](#sel4-userexception-length)
  - [`_enum_pad_seL4_UserException_Msg`](#enum-pad-sel4-userexception-msg)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Type`](#type) | type |  |
| [`seL4_UserException_FaultIP`](#sel4-userexception-faultip) | const |  |
| [`seL4_UserException_SP`](#sel4-userexception-sp) | const |  |
| [`seL4_UserException_FLAGS`](#sel4-userexception-flags) | const |  |
| [`seL4_UserException_Number`](#sel4-userexception-number) | const |  |
| [`seL4_UserException_Code`](#sel4-userexception-code) | const |  |
| [`seL4_UserException_Length`](#sel4-userexception-length) | const |  |
| [`_enum_pad_seL4_UserException_Msg`](#enum-pad-sel4-userexception-msg) | const |  |

## Type Aliases

### `Type`

```rust
type Type = ::core::ffi::c_ulong;
```

## Constants

### `seL4_UserException_FaultIP`
```rust
const seL4_UserException_FaultIP: Type = 0u64;
```

### `seL4_UserException_SP`
```rust
const seL4_UserException_SP: Type = 1u64;
```

### `seL4_UserException_FLAGS`
```rust
const seL4_UserException_FLAGS: Type = 2u64;
```

### `seL4_UserException_Number`
```rust
const seL4_UserException_Number: Type = 3u64;
```

### `seL4_UserException_Code`
```rust
const seL4_UserException_Code: Type = 4u64;
```

### `seL4_UserException_Length`
```rust
const seL4_UserException_Length: Type = 5u64;
```

### `_enum_pad_seL4_UserException_Msg`
```rust
const _enum_pad_seL4_UserException_Msg: Type = 9_223_372_036_854_775_807u64;
```

