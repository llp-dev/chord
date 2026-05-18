*[sel4_sys](../../index.md) / [c](../index.md) / [seL4_UnknownSyscall_Msg](index.md)*

---

# Module `seL4_UnknownSyscall_Msg`

## Contents

- [Type Aliases](#type-aliases)
  - [`Type`](#type)
- [Constants](#constants)
  - [`seL4_UnknownSyscall_RAX`](#sel4-unknownsyscall-rax)
  - [`seL4_UnknownSyscall_RBX`](#sel4-unknownsyscall-rbx)
  - [`seL4_UnknownSyscall_RCX`](#sel4-unknownsyscall-rcx)
  - [`seL4_UnknownSyscall_RDX`](#sel4-unknownsyscall-rdx)
  - [`seL4_UnknownSyscall_RSI`](#sel4-unknownsyscall-rsi)
  - [`seL4_UnknownSyscall_RDI`](#sel4-unknownsyscall-rdi)
  - [`seL4_UnknownSyscall_RBP`](#sel4-unknownsyscall-rbp)
  - [`seL4_UnknownSyscall_R8`](#sel4-unknownsyscall-r8)
  - [`seL4_UnknownSyscall_R9`](#sel4-unknownsyscall-r9)
  - [`seL4_UnknownSyscall_R10`](#sel4-unknownsyscall-r10)
  - [`seL4_UnknownSyscall_R11`](#sel4-unknownsyscall-r11)
  - [`seL4_UnknownSyscall_R12`](#sel4-unknownsyscall-r12)
  - [`seL4_UnknownSyscall_R13`](#sel4-unknownsyscall-r13)
  - [`seL4_UnknownSyscall_R14`](#sel4-unknownsyscall-r14)
  - [`seL4_UnknownSyscall_R15`](#sel4-unknownsyscall-r15)
  - [`seL4_UnknownSyscall_FaultIP`](#sel4-unknownsyscall-faultip)
  - [`seL4_UnknownSyscall_SP`](#sel4-unknownsyscall-sp)
  - [`seL4_UnknownSyscall_FLAGS`](#sel4-unknownsyscall-flags)
  - [`seL4_UnknownSyscall_Syscall`](#sel4-unknownsyscall-syscall)
  - [`seL4_UnknownSyscall_Length`](#sel4-unknownsyscall-length)
  - [`_enum_pad_seL4_UnknownSyscall_Msg`](#enum-pad-sel4-unknownsyscall-msg)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Type`](#type) | type |  |
| [`seL4_UnknownSyscall_RAX`](#sel4-unknownsyscall-rax) | const |  |
| [`seL4_UnknownSyscall_RBX`](#sel4-unknownsyscall-rbx) | const |  |
| [`seL4_UnknownSyscall_RCX`](#sel4-unknownsyscall-rcx) | const |  |
| [`seL4_UnknownSyscall_RDX`](#sel4-unknownsyscall-rdx) | const |  |
| [`seL4_UnknownSyscall_RSI`](#sel4-unknownsyscall-rsi) | const |  |
| [`seL4_UnknownSyscall_RDI`](#sel4-unknownsyscall-rdi) | const |  |
| [`seL4_UnknownSyscall_RBP`](#sel4-unknownsyscall-rbp) | const |  |
| [`seL4_UnknownSyscall_R8`](#sel4-unknownsyscall-r8) | const |  |
| [`seL4_UnknownSyscall_R9`](#sel4-unknownsyscall-r9) | const |  |
| [`seL4_UnknownSyscall_R10`](#sel4-unknownsyscall-r10) | const |  |
| [`seL4_UnknownSyscall_R11`](#sel4-unknownsyscall-r11) | const |  |
| [`seL4_UnknownSyscall_R12`](#sel4-unknownsyscall-r12) | const |  |
| [`seL4_UnknownSyscall_R13`](#sel4-unknownsyscall-r13) | const |  |
| [`seL4_UnknownSyscall_R14`](#sel4-unknownsyscall-r14) | const |  |
| [`seL4_UnknownSyscall_R15`](#sel4-unknownsyscall-r15) | const |  |
| [`seL4_UnknownSyscall_FaultIP`](#sel4-unknownsyscall-faultip) | const |  |
| [`seL4_UnknownSyscall_SP`](#sel4-unknownsyscall-sp) | const |  |
| [`seL4_UnknownSyscall_FLAGS`](#sel4-unknownsyscall-flags) | const |  |
| [`seL4_UnknownSyscall_Syscall`](#sel4-unknownsyscall-syscall) | const |  |
| [`seL4_UnknownSyscall_Length`](#sel4-unknownsyscall-length) | const |  |
| [`_enum_pad_seL4_UnknownSyscall_Msg`](#enum-pad-sel4-unknownsyscall-msg) | const |  |

## Type Aliases

### `Type`

```rust
type Type = ::core::ffi::c_ulong;
```

## Constants

### `seL4_UnknownSyscall_RAX`
```rust
const seL4_UnknownSyscall_RAX: Type = 0u64;
```

### `seL4_UnknownSyscall_RBX`
```rust
const seL4_UnknownSyscall_RBX: Type = 1u64;
```

### `seL4_UnknownSyscall_RCX`
```rust
const seL4_UnknownSyscall_RCX: Type = 2u64;
```

### `seL4_UnknownSyscall_RDX`
```rust
const seL4_UnknownSyscall_RDX: Type = 3u64;
```

### `seL4_UnknownSyscall_RSI`
```rust
const seL4_UnknownSyscall_RSI: Type = 4u64;
```

### `seL4_UnknownSyscall_RDI`
```rust
const seL4_UnknownSyscall_RDI: Type = 5u64;
```

### `seL4_UnknownSyscall_RBP`
```rust
const seL4_UnknownSyscall_RBP: Type = 6u64;
```

### `seL4_UnknownSyscall_R8`
```rust
const seL4_UnknownSyscall_R8: Type = 7u64;
```

### `seL4_UnknownSyscall_R9`
```rust
const seL4_UnknownSyscall_R9: Type = 8u64;
```

### `seL4_UnknownSyscall_R10`
```rust
const seL4_UnknownSyscall_R10: Type = 9u64;
```

### `seL4_UnknownSyscall_R11`
```rust
const seL4_UnknownSyscall_R11: Type = 10u64;
```

### `seL4_UnknownSyscall_R12`
```rust
const seL4_UnknownSyscall_R12: Type = 11u64;
```

### `seL4_UnknownSyscall_R13`
```rust
const seL4_UnknownSyscall_R13: Type = 12u64;
```

### `seL4_UnknownSyscall_R14`
```rust
const seL4_UnknownSyscall_R14: Type = 13u64;
```

### `seL4_UnknownSyscall_R15`
```rust
const seL4_UnknownSyscall_R15: Type = 14u64;
```

### `seL4_UnknownSyscall_FaultIP`
```rust
const seL4_UnknownSyscall_FaultIP: Type = 15u64;
```

### `seL4_UnknownSyscall_SP`
```rust
const seL4_UnknownSyscall_SP: Type = 16u64;
```

### `seL4_UnknownSyscall_FLAGS`
```rust
const seL4_UnknownSyscall_FLAGS: Type = 17u64;
```

### `seL4_UnknownSyscall_Syscall`
```rust
const seL4_UnknownSyscall_Syscall: Type = 18u64;
```

### `seL4_UnknownSyscall_Length`
```rust
const seL4_UnknownSyscall_Length: Type = 19u64;
```

### `_enum_pad_seL4_UnknownSyscall_Msg`
```rust
const _enum_pad_seL4_UnknownSyscall_Msg: Type = 9_223_372_036_854_775_807u64;
```

