*[sel4_sys](../../index.md) / [c](../index.md) / [seL4_RootCNodeCapSlots](index.md)*

---

# Module `seL4_RootCNodeCapSlots`

## Contents

- [Type Aliases](#type-aliases)
  - [`Type`](#type)
- [Constants](#constants)
  - [`seL4_CapNull`](#sel4-capnull)
  - [`seL4_CapInitThreadTCB`](#sel4-capinitthreadtcb)
  - [`seL4_CapInitThreadCNode`](#sel4-capinitthreadcnode)
  - [`seL4_CapInitThreadVSpace`](#sel4-capinitthreadvspace)
  - [`seL4_CapIRQControl`](#sel4-capirqcontrol)
  - [`seL4_CapASIDControl`](#sel4-capasidcontrol)
  - [`seL4_CapInitThreadASIDPool`](#sel4-capinitthreadasidpool)
  - [`seL4_CapIOPortControl`](#sel4-capioportcontrol)
  - [`seL4_CapIOSpace`](#sel4-capiospace)
  - [`seL4_CapBootInfoFrame`](#sel4-capbootinfoframe)
  - [`seL4_CapInitThreadIPCBuffer`](#sel4-capinitthreadipcbuffer)
  - [`seL4_CapDomain`](#sel4-capdomain)
  - [`seL4_CapSMMUSIDControl`](#sel4-capsmmusidcontrol)
  - [`seL4_CapSMMUCBControl`](#sel4-capsmmucbcontrol)
  - [`seL4_CapInitThreadSC`](#sel4-capinitthreadsc)
  - [`seL4_CapSMC`](#sel4-capsmc)
  - [`seL4_NumInitialCaps`](#sel4-numinitialcaps)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Type`](#type) | type |  |
| [`seL4_CapNull`](#sel4-capnull) | const |  |
| [`seL4_CapInitThreadTCB`](#sel4-capinitthreadtcb) | const |  |
| [`seL4_CapInitThreadCNode`](#sel4-capinitthreadcnode) | const |  |
| [`seL4_CapInitThreadVSpace`](#sel4-capinitthreadvspace) | const |  |
| [`seL4_CapIRQControl`](#sel4-capirqcontrol) | const |  |
| [`seL4_CapASIDControl`](#sel4-capasidcontrol) | const |  |
| [`seL4_CapInitThreadASIDPool`](#sel4-capinitthreadasidpool) | const |  |
| [`seL4_CapIOPortControl`](#sel4-capioportcontrol) | const |  |
| [`seL4_CapIOSpace`](#sel4-capiospace) | const |  |
| [`seL4_CapBootInfoFrame`](#sel4-capbootinfoframe) | const |  |
| [`seL4_CapInitThreadIPCBuffer`](#sel4-capinitthreadipcbuffer) | const |  |
| [`seL4_CapDomain`](#sel4-capdomain) | const |  |
| [`seL4_CapSMMUSIDControl`](#sel4-capsmmusidcontrol) | const |  |
| [`seL4_CapSMMUCBControl`](#sel4-capsmmucbcontrol) | const |  |
| [`seL4_CapInitThreadSC`](#sel4-capinitthreadsc) | const |  |
| [`seL4_CapSMC`](#sel4-capsmc) | const |  |
| [`seL4_NumInitialCaps`](#sel4-numinitialcaps) | const |  |

## Type Aliases

### `Type`

```rust
type Type = ::core::ffi::c_uint;
```

## Constants

### `seL4_CapNull`
```rust
const seL4_CapNull: Type = 0u32;
```

### `seL4_CapInitThreadTCB`
```rust
const seL4_CapInitThreadTCB: Type = 1u32;
```

### `seL4_CapInitThreadCNode`
```rust
const seL4_CapInitThreadCNode: Type = 2u32;
```

### `seL4_CapInitThreadVSpace`
```rust
const seL4_CapInitThreadVSpace: Type = 3u32;
```

### `seL4_CapIRQControl`
```rust
const seL4_CapIRQControl: Type = 4u32;
```

### `seL4_CapASIDControl`
```rust
const seL4_CapASIDControl: Type = 5u32;
```

### `seL4_CapInitThreadASIDPool`
```rust
const seL4_CapInitThreadASIDPool: Type = 6u32;
```

### `seL4_CapIOPortControl`
```rust
const seL4_CapIOPortControl: Type = 7u32;
```

### `seL4_CapIOSpace`
```rust
const seL4_CapIOSpace: Type = 8u32;
```

### `seL4_CapBootInfoFrame`
```rust
const seL4_CapBootInfoFrame: Type = 9u32;
```

### `seL4_CapInitThreadIPCBuffer`
```rust
const seL4_CapInitThreadIPCBuffer: Type = 10u32;
```

### `seL4_CapDomain`
```rust
const seL4_CapDomain: Type = 11u32;
```

### `seL4_CapSMMUSIDControl`
```rust
const seL4_CapSMMUSIDControl: Type = 12u32;
```

### `seL4_CapSMMUCBControl`
```rust
const seL4_CapSMMUCBControl: Type = 13u32;
```

### `seL4_CapInitThreadSC`
```rust
const seL4_CapInitThreadSC: Type = 14u32;
```

### `seL4_CapSMC`
```rust
const seL4_CapSMC: Type = 15u32;
```

### `seL4_NumInitialCaps`
```rust
const seL4_NumInitialCaps: Type = 16u32;
```

