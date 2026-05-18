*[sel4_sys](../../index.md) / [c](../index.md) / [seL4_Error](index.md)*

---

# Module `seL4_Error`

## Contents

- [Type Aliases](#type-aliases)
  - [`Type`](#type)
- [Constants](#constants)
  - [`seL4_NoError`](#sel4-noerror)
  - [`seL4_InvalidArgument`](#sel4-invalidargument)
  - [`seL4_InvalidCapability`](#sel4-invalidcapability)
  - [`seL4_IllegalOperation`](#sel4-illegaloperation)
  - [`seL4_RangeError`](#sel4-rangeerror)
  - [`seL4_AlignmentError`](#sel4-alignmenterror)
  - [`seL4_FailedLookup`](#sel4-failedlookup)
  - [`seL4_TruncatedMessage`](#sel4-truncatedmessage)
  - [`seL4_DeleteFirst`](#sel4-deletefirst)
  - [`seL4_RevokeFirst`](#sel4-revokefirst)
  - [`seL4_NotEnoughMemory`](#sel4-notenoughmemory)
  - [`seL4_NumErrors`](#sel4-numerrors)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Type`](#type) | type |  |
| [`seL4_NoError`](#sel4-noerror) | const |  |
| [`seL4_InvalidArgument`](#sel4-invalidargument) | const |  |
| [`seL4_InvalidCapability`](#sel4-invalidcapability) | const |  |
| [`seL4_IllegalOperation`](#sel4-illegaloperation) | const |  |
| [`seL4_RangeError`](#sel4-rangeerror) | const |  |
| [`seL4_AlignmentError`](#sel4-alignmenterror) | const |  |
| [`seL4_FailedLookup`](#sel4-failedlookup) | const |  |
| [`seL4_TruncatedMessage`](#sel4-truncatedmessage) | const |  |
| [`seL4_DeleteFirst`](#sel4-deletefirst) | const |  |
| [`seL4_RevokeFirst`](#sel4-revokefirst) | const |  |
| [`seL4_NotEnoughMemory`](#sel4-notenoughmemory) | const |  |
| [`seL4_NumErrors`](#sel4-numerrors) | const |  |

## Type Aliases

### `Type`

```rust
type Type = ::core::ffi::c_uint;
```

## Constants

### `seL4_NoError`
```rust
const seL4_NoError: Type = 0u32;
```

### `seL4_InvalidArgument`
```rust
const seL4_InvalidArgument: Type = 1u32;
```

### `seL4_InvalidCapability`
```rust
const seL4_InvalidCapability: Type = 2u32;
```

### `seL4_IllegalOperation`
```rust
const seL4_IllegalOperation: Type = 3u32;
```

### `seL4_RangeError`
```rust
const seL4_RangeError: Type = 4u32;
```

### `seL4_AlignmentError`
```rust
const seL4_AlignmentError: Type = 5u32;
```

### `seL4_FailedLookup`
```rust
const seL4_FailedLookup: Type = 6u32;
```

### `seL4_TruncatedMessage`
```rust
const seL4_TruncatedMessage: Type = 7u32;
```

### `seL4_DeleteFirst`
```rust
const seL4_DeleteFirst: Type = 8u32;
```

### `seL4_RevokeFirst`
```rust
const seL4_RevokeFirst: Type = 9u32;
```

### `seL4_NotEnoughMemory`
```rust
const seL4_NotEnoughMemory: Type = 10u32;
```

### `seL4_NumErrors`
```rust
const seL4_NumErrors: Type = 11u32;
```

