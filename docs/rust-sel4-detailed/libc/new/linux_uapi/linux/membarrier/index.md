*[libc](../../../../index.md) / [new](../../../index.md) / [linux_uapi](../../index.md) / [linux](../index.md) / [membarrier](index.md)*

---

# Module `membarrier`

Header: `uapi/linux/membarrier.h`

## Contents

- [Type Aliases](#type-aliases)
  - [`membarrier_cmd`](#membarrier-cmd)
- [Constants](#constants)
  - [`MEMBARRIER_CMD_QUERY`](#membarrier-cmd-query)
  - [`MEMBARRIER_CMD_GLOBAL`](#membarrier-cmd-global)
  - [`MEMBARRIER_CMD_GLOBAL_EXPEDITED`](#membarrier-cmd-global-expedited)
  - [`MEMBARRIER_CMD_REGISTER_GLOBAL_EXPEDITED`](#membarrier-cmd-register-global-expedited)
  - [`MEMBARRIER_CMD_PRIVATE_EXPEDITED`](#membarrier-cmd-private-expedited)
  - [`MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED`](#membarrier-cmd-register-private-expedited)
  - [`MEMBARRIER_CMD_PRIVATE_EXPEDITED_SYNC_CORE`](#membarrier-cmd-private-expedited-sync-core)
  - [`MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_SYNC_CORE`](#membarrier-cmd-register-private-expedited-sync-core)
  - [`MEMBARRIER_CMD_PRIVATE_EXPEDITED_RSEQ`](#membarrier-cmd-private-expedited-rseq)
  - [`MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_RSEQ`](#membarrier-cmd-register-private-expedited-rseq)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`membarrier_cmd`](#membarrier-cmd) | type |  |
| [`MEMBARRIER_CMD_QUERY`](#membarrier-cmd-query) | const |  |
| [`MEMBARRIER_CMD_GLOBAL`](#membarrier-cmd-global) | const |  |
| [`MEMBARRIER_CMD_GLOBAL_EXPEDITED`](#membarrier-cmd-global-expedited) | const |  |
| [`MEMBARRIER_CMD_REGISTER_GLOBAL_EXPEDITED`](#membarrier-cmd-register-global-expedited) | const |  |
| [`MEMBARRIER_CMD_PRIVATE_EXPEDITED`](#membarrier-cmd-private-expedited) | const |  |
| [`MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED`](#membarrier-cmd-register-private-expedited) | const |  |
| [`MEMBARRIER_CMD_PRIVATE_EXPEDITED_SYNC_CORE`](#membarrier-cmd-private-expedited-sync-core) | const |  |
| [`MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_SYNC_CORE`](#membarrier-cmd-register-private-expedited-sync-core) | const |  |
| [`MEMBARRIER_CMD_PRIVATE_EXPEDITED_RSEQ`](#membarrier-cmd-private-expedited-rseq) | const |  |
| [`MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_RSEQ`](#membarrier-cmd-register-private-expedited-rseq) | const |  |

## Type Aliases

### `membarrier_cmd`

```rust
type membarrier_cmd = c_int;
```

## Constants

### `MEMBARRIER_CMD_QUERY`
```rust
const MEMBARRIER_CMD_QUERY: membarrier_cmd = 0i32;
```

### `MEMBARRIER_CMD_GLOBAL`
```rust
const MEMBARRIER_CMD_GLOBAL: membarrier_cmd = 1i32;
```

### `MEMBARRIER_CMD_GLOBAL_EXPEDITED`
```rust
const MEMBARRIER_CMD_GLOBAL_EXPEDITED: membarrier_cmd = 2i32;
```

### `MEMBARRIER_CMD_REGISTER_GLOBAL_EXPEDITED`
```rust
const MEMBARRIER_CMD_REGISTER_GLOBAL_EXPEDITED: membarrier_cmd = 4i32;
```

### `MEMBARRIER_CMD_PRIVATE_EXPEDITED`
```rust
const MEMBARRIER_CMD_PRIVATE_EXPEDITED: membarrier_cmd = 8i32;
```

### `MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED`
```rust
const MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED: membarrier_cmd = 16i32;
```

### `MEMBARRIER_CMD_PRIVATE_EXPEDITED_SYNC_CORE`
```rust
const MEMBARRIER_CMD_PRIVATE_EXPEDITED_SYNC_CORE: membarrier_cmd = 32i32;
```

### `MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_SYNC_CORE`
```rust
const MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_SYNC_CORE: membarrier_cmd = 64i32;
```

### `MEMBARRIER_CMD_PRIVATE_EXPEDITED_RSEQ`
```rust
const MEMBARRIER_CMD_PRIVATE_EXPEDITED_RSEQ: membarrier_cmd = 128i32;
```

### `MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_RSEQ`
```rust
const MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_RSEQ: membarrier_cmd = 256i32;
```

