*[sel4](../../index.md) / [init_thread](../index.md) / [slot](index.md)*

---

# Module `slot`

Initial CSpace slot constants corresponding to `seL4_Cap*`.

## Contents

- [Constants](#constants)
  - [`NULL`](#null)
  - [`TCB`](#tcb)
  - [`CNODE`](#cnode)
  - [`VSPACE`](#vspace)
  - [`IRQ_CONTROL`](#irq-control)
  - [`ASID_CONTROL`](#asid-control)
  - [`ASID_POOL`](#asid-pool)
  - [`IO_PORT_CONTROL`](#io-port-control)
  - [`BOOT_INFO_FRAME`](#boot-info-frame)
  - [`IPC_BUFFER`](#ipc-buffer)
  - [`DOMAIN_SET`](#domain-set)
- [Macros](#macros)
  - [`mk!`](#mk)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`NULL`](#null) | const | Corresponds to ` seL4_CapNull `. |
| [`TCB`](#tcb) | const | Corresponds to ` seL4_CapInitThreadTCB `. |
| [`CNODE`](#cnode) | const | Corresponds to ` seL4_CapInitThreadCNode `. |
| [`VSPACE`](#vspace) | const | Corresponds to ` seL4_CapInitThreadVSpace `. |
| [`IRQ_CONTROL`](#irq-control) | const | Corresponds to ` seL4_CapIRQControl `. |
| [`ASID_CONTROL`](#asid-control) | const | Corresponds to ` seL4_CapASIDControl `. |
| [`ASID_POOL`](#asid-pool) | const | Corresponds to ` seL4_CapInitThreadASIDPool `. |
| [`IO_PORT_CONTROL`](#io-port-control) | const | Corresponds to ` seL4_CapIOPortControl `. |
| [`BOOT_INFO_FRAME`](#boot-info-frame) | const | Corresponds to ` seL4_CapBootInfoFrame `. |
| [`IPC_BUFFER`](#ipc-buffer) | const | Corresponds to ` seL4_CapInitThreadIPCBuffer `. |
| [`DOMAIN_SET`](#domain-set) | const | Corresponds to ` seL4_CapDomain `. |
| [`mk!`](#mk) | macro |  |

## Constants

### `NULL`
```rust
const NULL: Slot<cap_type::Null>;
```

Corresponds to `
seL4_CapNull
`.

### `TCB`
```rust
const TCB: Slot<cap_type::Tcb>;
```

Corresponds to `
seL4_CapInitThreadTCB
`.

### `CNODE`
```rust
const CNODE: Slot<cap_type::CNode>;
```

Corresponds to `
seL4_CapInitThreadCNode
`.

### `VSPACE`
```rust
const VSPACE: Slot<cap_type::VSpace>;
```

Corresponds to `
seL4_CapInitThreadVSpace
`.

### `IRQ_CONTROL`
```rust
const IRQ_CONTROL: Slot<cap_type::IrqControl>;
```

Corresponds to `
seL4_CapIRQControl
`.

### `ASID_CONTROL`
```rust
const ASID_CONTROL: Slot<cap_type::AsidControl>;
```

Corresponds to `
seL4_CapASIDControl
`.

### `ASID_POOL`
```rust
const ASID_POOL: Slot<cap_type::AsidPool>;
```

Corresponds to `
seL4_CapInitThreadASIDPool
`.

### `IO_PORT_CONTROL`
```rust
const IO_PORT_CONTROL: Slot<cap_type::IOPortControl>;
```

Corresponds to `
seL4_CapIOPortControl
`.

### `BOOT_INFO_FRAME`
```rust
const BOOT_INFO_FRAME: Slot<cap_type::Granule>;
```

Corresponds to `
seL4_CapBootInfoFrame
`.

### `IPC_BUFFER`
```rust
const IPC_BUFFER: Slot<cap_type::Granule>;
```

Corresponds to `
seL4_CapInitThreadIPCBuffer
`.

### `DOMAIN_SET`
```rust
const DOMAIN_SET: Slot<cap_type::DomainSet>;
```

Corresponds to `
seL4_CapDomain
`.

## Macros

### `mk!`

