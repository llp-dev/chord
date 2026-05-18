*[sel4_sys](../index.md) / [bf](index.md)*

---

# Module `bf`

## Contents

- [Modules](#modules)
  - [`seL4_Fault_tag`](#sel4-fault-tag)
- [Structs](#structs)
  - [`seL4_Fault_NullFault`](#sel4-fault-nullfault)
  - [`seL4_Fault_NullFault_Unpacked`](#sel4-fault-nullfault-unpacked)
  - [`seL4_Fault_CapFault`](#sel4-fault-capfault)
  - [`seL4_Fault_CapFault_Unpacked`](#sel4-fault-capfault-unpacked)
  - [`seL4_Fault_UnknownSyscall`](#sel4-fault-unknownsyscall)
  - [`seL4_Fault_UnknownSyscall_Unpacked`](#sel4-fault-unknownsyscall-unpacked)
  - [`seL4_Fault_UserException`](#sel4-fault-userexception)
  - [`seL4_Fault_UserException_Unpacked`](#sel4-fault-userexception-unpacked)
  - [`seL4_Fault_VMFault`](#sel4-fault-vmfault)
  - [`seL4_Fault_VMFault_Unpacked`](#sel4-fault-vmfault-unpacked)
  - [`seL4_Fault`](#sel4-fault)
  - [`seL4_MessageInfo`](#sel4-messageinfo)
  - [`seL4_MessageInfo_Unpacked`](#sel4-messageinfo-unpacked)
  - [`seL4_CapRights`](#sel4-caprights)
  - [`seL4_CapRights_Unpacked`](#sel4-caprights-unpacked)
  - [`seL4_CNode_CapData`](#sel4-cnode-capdata)
  - [`seL4_CNode_CapData_Unpacked`](#sel4-cnode-capdata-unpacked)
- [Enums](#enums)
  - [`seL4_Fault_Splayed`](#sel4-fault-splayed)
- [Type Aliases](#type-aliases)
  - [`SeL4Bitfield`](#sel4bitfield)
  - [`seL4_MessageInfo_t`](#sel4-messageinfo-t)
  - [`seL4_CapRights_t`](#sel4-caprights-t)
  - [`seL4_CNode_CapData_t`](#sel4-cnode-capdata-t)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`seL4_Fault_tag`](#sel4-fault-tag) | mod |  |
| [`seL4_Fault_NullFault`](#sel4-fault-nullfault) | struct |  |
| [`seL4_Fault_NullFault_Unpacked`](#sel4-fault-nullfault-unpacked) | struct |  |
| [`seL4_Fault_CapFault`](#sel4-fault-capfault) | struct |  |
| [`seL4_Fault_CapFault_Unpacked`](#sel4-fault-capfault-unpacked) | struct |  |
| [`seL4_Fault_UnknownSyscall`](#sel4-fault-unknownsyscall) | struct |  |
| [`seL4_Fault_UnknownSyscall_Unpacked`](#sel4-fault-unknownsyscall-unpacked) | struct |  |
| [`seL4_Fault_UserException`](#sel4-fault-userexception) | struct |  |
| [`seL4_Fault_UserException_Unpacked`](#sel4-fault-userexception-unpacked) | struct |  |
| [`seL4_Fault_VMFault`](#sel4-fault-vmfault) | struct |  |
| [`seL4_Fault_VMFault_Unpacked`](#sel4-fault-vmfault-unpacked) | struct |  |
| [`seL4_Fault`](#sel4-fault) | struct |  |
| [`seL4_MessageInfo`](#sel4-messageinfo) | struct |  |
| [`seL4_MessageInfo_Unpacked`](#sel4-messageinfo-unpacked) | struct |  |
| [`seL4_CapRights`](#sel4-caprights) | struct |  |
| [`seL4_CapRights_Unpacked`](#sel4-caprights-unpacked) | struct |  |
| [`seL4_CNode_CapData`](#sel4-cnode-capdata) | struct |  |
| [`seL4_CNode_CapData_Unpacked`](#sel4-cnode-capdata-unpacked) | struct |  |
| [`seL4_Fault_Splayed`](#sel4-fault-splayed) | enum |  |
| [`SeL4Bitfield`](#sel4bitfield) | type |  |
| [`seL4_MessageInfo_t`](#sel4-messageinfo-t) | type |  |
| [`seL4_CapRights_t`](#sel4-caprights-t) | type |  |
| [`seL4_CNode_CapData_t`](#sel4-cnode-capdata-t) | type |  |

## Modules

- [`seL4_Fault_tag`](seL4_Fault_tag/index.md)

## Structs

### `seL4_Fault_NullFault`

```rust
struct seL4_Fault_NullFault(sel4_bitfield_ops::Bitfield<[u64; N], u64>);
```

#### Implementations

- <span id="sel4-fault-nullfault-new"></span>`fn new() -> Self`

- <span id="sel4-fault-nullfault-unpack"></span>`fn unpack(&self) -> seL4_Fault_NullFault_Unpacked` — [`seL4_Fault_NullFault_Unpacked`](../index.md#sel4-fault-nullfault-unpacked)

- <span id="sel4-fault-nullfault-get-sel4-faulttype"></span>`fn get_seL4_FaultType(&self) -> u64`

- <span id="sel4-fault-nullfault-set-sel4-faulttype"></span>`fn set_seL4_FaultType(&mut self, seL4_FaultType: u64)`

- <span id="sel4-fault-nullfault-width-of-sel4-faulttype"></span>`const fn width_of_seL4_FaultType() -> usize`

#### Trait Implementations

##### `impl Clone for seL4_Fault_NullFault`

- <span id="sel4-fault-nullfault-clone"></span>`fn clone(&self) -> seL4_Fault_NullFault` — [`seL4_Fault_NullFault`](../index.md#sel4-fault-nullfault)

##### `impl Debug for seL4_Fault_NullFault`

- <span id="sel4-fault-nullfault-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for seL4_Fault_NullFault`

##### `impl PartialEq for seL4_Fault_NullFault`

- <span id="sel4-fault-nullfault-partialeq-eq"></span>`fn eq(&self, other: &seL4_Fault_NullFault) -> bool` — [`seL4_Fault_NullFault`](../index.md#sel4-fault-nullfault)

##### `impl StructuralPartialEq for seL4_Fault_NullFault`

### `seL4_Fault_NullFault_Unpacked`

```rust
struct seL4_Fault_NullFault_Unpacked {
}
```

#### Implementations

- <span id="sel4-fault-nullfault-unpacked-pack"></span>`fn pack(self) -> seL4_Fault_NullFault` — [`seL4_Fault_NullFault`](../index.md#sel4-fault-nullfault)

#### Trait Implementations

##### `impl Clone for seL4_Fault_NullFault_Unpacked`

- <span id="sel4-fault-nullfault-unpacked-clone"></span>`fn clone(&self) -> seL4_Fault_NullFault_Unpacked` — [`seL4_Fault_NullFault_Unpacked`](../index.md#sel4-fault-nullfault-unpacked)

##### `impl Debug for seL4_Fault_NullFault_Unpacked`

- <span id="sel4-fault-nullfault-unpacked-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for seL4_Fault_NullFault_Unpacked`

##### `impl PartialEq for seL4_Fault_NullFault_Unpacked`

- <span id="sel4-fault-nullfault-unpacked-partialeq-eq"></span>`fn eq(&self, other: &seL4_Fault_NullFault_Unpacked) -> bool` — [`seL4_Fault_NullFault_Unpacked`](../index.md#sel4-fault-nullfault-unpacked)

##### `impl StructuralPartialEq for seL4_Fault_NullFault_Unpacked`

### `seL4_Fault_CapFault`

```rust
struct seL4_Fault_CapFault(sel4_bitfield_ops::Bitfield<[u64; N], u64>);
```

#### Implementations

- <span id="sel4-fault-capfault-new"></span>`fn new(IP: u64, Addr: u64, InRecvPhase: u64, LookupFailureType: u64, MR4: u64, MR5: u64, MR6: u64) -> Self`

- <span id="sel4-fault-capfault-unpack"></span>`fn unpack(&self) -> seL4_Fault_CapFault_Unpacked` — [`seL4_Fault_CapFault_Unpacked`](../index.md#sel4-fault-capfault-unpacked)

- <span id="sel4-fault-capfault-get-ip"></span>`fn get_IP(&self) -> u64`

- <span id="sel4-fault-capfault-set-ip"></span>`fn set_IP(&mut self, IP: u64)`

- <span id="sel4-fault-capfault-width-of-ip"></span>`const fn width_of_IP() -> usize`

- <span id="sel4-fault-capfault-get-addr"></span>`fn get_Addr(&self) -> u64`

- <span id="sel4-fault-capfault-set-addr"></span>`fn set_Addr(&mut self, Addr: u64)`

- <span id="sel4-fault-capfault-width-of-addr"></span>`const fn width_of_Addr() -> usize`

- <span id="sel4-fault-capfault-get-inrecvphase"></span>`fn get_InRecvPhase(&self) -> u64`

- <span id="sel4-fault-capfault-set-inrecvphase"></span>`fn set_InRecvPhase(&mut self, InRecvPhase: u64)`

- <span id="sel4-fault-capfault-width-of-inrecvphase"></span>`const fn width_of_InRecvPhase() -> usize`

- <span id="sel4-fault-capfault-get-lookupfailuretype"></span>`fn get_LookupFailureType(&self) -> u64`

- <span id="sel4-fault-capfault-set-lookupfailuretype"></span>`fn set_LookupFailureType(&mut self, LookupFailureType: u64)`

- <span id="sel4-fault-capfault-width-of-lookupfailuretype"></span>`const fn width_of_LookupFailureType() -> usize`

- <span id="sel4-fault-capfault-get-mr4"></span>`fn get_MR4(&self) -> u64`

- <span id="sel4-fault-capfault-set-mr4"></span>`fn set_MR4(&mut self, MR4: u64)`

- <span id="sel4-fault-capfault-width-of-mr4"></span>`const fn width_of_MR4() -> usize`

- <span id="sel4-fault-capfault-get-mr5"></span>`fn get_MR5(&self) -> u64`

- <span id="sel4-fault-capfault-set-mr5"></span>`fn set_MR5(&mut self, MR5: u64)`

- <span id="sel4-fault-capfault-width-of-mr5"></span>`const fn width_of_MR5() -> usize`

- <span id="sel4-fault-capfault-get-mr6"></span>`fn get_MR6(&self) -> u64`

- <span id="sel4-fault-capfault-set-mr6"></span>`fn set_MR6(&mut self, MR6: u64)`

- <span id="sel4-fault-capfault-width-of-mr6"></span>`const fn width_of_MR6() -> usize`

- <span id="sel4-fault-capfault-get-sel4-faulttype"></span>`fn get_seL4_FaultType(&self) -> u64`

- <span id="sel4-fault-capfault-set-sel4-faulttype"></span>`fn set_seL4_FaultType(&mut self, seL4_FaultType: u64)`

- <span id="sel4-fault-capfault-width-of-sel4-faulttype"></span>`const fn width_of_seL4_FaultType() -> usize`

#### Trait Implementations

##### `impl Clone for seL4_Fault_CapFault`

- <span id="sel4-fault-capfault-clone"></span>`fn clone(&self) -> seL4_Fault_CapFault` — [`seL4_Fault_CapFault`](../index.md#sel4-fault-capfault)

##### `impl Debug for seL4_Fault_CapFault`

- <span id="sel4-fault-capfault-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for seL4_Fault_CapFault`

##### `impl PartialEq for seL4_Fault_CapFault`

- <span id="sel4-fault-capfault-partialeq-eq"></span>`fn eq(&self, other: &seL4_Fault_CapFault) -> bool` — [`seL4_Fault_CapFault`](../index.md#sel4-fault-capfault)

##### `impl StructuralPartialEq for seL4_Fault_CapFault`

### `seL4_Fault_CapFault_Unpacked`

```rust
struct seL4_Fault_CapFault_Unpacked {
    pub IP: u64,
    pub Addr: u64,
    pub InRecvPhase: u64,
    pub LookupFailureType: u64,
    pub MR4: u64,
    pub MR5: u64,
    pub MR6: u64,
}
```

#### Implementations

- <span id="sel4-fault-capfault-unpacked-pack"></span>`fn pack(self) -> seL4_Fault_CapFault` — [`seL4_Fault_CapFault`](../index.md#sel4-fault-capfault)

#### Trait Implementations

##### `impl Clone for seL4_Fault_CapFault_Unpacked`

- <span id="sel4-fault-capfault-unpacked-clone"></span>`fn clone(&self) -> seL4_Fault_CapFault_Unpacked` — [`seL4_Fault_CapFault_Unpacked`](../index.md#sel4-fault-capfault-unpacked)

##### `impl Debug for seL4_Fault_CapFault_Unpacked`

- <span id="sel4-fault-capfault-unpacked-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for seL4_Fault_CapFault_Unpacked`

##### `impl PartialEq for seL4_Fault_CapFault_Unpacked`

- <span id="sel4-fault-capfault-unpacked-partialeq-eq"></span>`fn eq(&self, other: &seL4_Fault_CapFault_Unpacked) -> bool` — [`seL4_Fault_CapFault_Unpacked`](../index.md#sel4-fault-capfault-unpacked)

##### `impl StructuralPartialEq for seL4_Fault_CapFault_Unpacked`

### `seL4_Fault_UnknownSyscall`

```rust
struct seL4_Fault_UnknownSyscall(sel4_bitfield_ops::Bitfield<[u64; N], u64>);
```

#### Implementations

- <span id="sel4-fault-unknownsyscall-new"></span>`fn new(RAX: u64, RBX: u64, RCX: u64, RDX: u64, RSI: u64, RDI: u64, RBP: u64, R8: u64, R9: u64, R10: u64, R11: u64, R12: u64, R13: u64, R14: u64, R15: u64, FaultIP: u64, RSP: u64, FLAGS: u64, Syscall: u64) -> Self`

- <span id="sel4-fault-unknownsyscall-unpack"></span>`fn unpack(&self) -> seL4_Fault_UnknownSyscall_Unpacked` — [`seL4_Fault_UnknownSyscall_Unpacked`](../index.md#sel4-fault-unknownsyscall-unpacked)

- <span id="sel4-fault-unknownsyscall-get-rax"></span>`fn get_RAX(&self) -> u64`

- <span id="sel4-fault-unknownsyscall-set-rax"></span>`fn set_RAX(&mut self, RAX: u64)`

- <span id="sel4-fault-unknownsyscall-width-of-rax"></span>`const fn width_of_RAX() -> usize`

- <span id="sel4-fault-unknownsyscall-get-rbx"></span>`fn get_RBX(&self) -> u64`

- <span id="sel4-fault-unknownsyscall-set-rbx"></span>`fn set_RBX(&mut self, RBX: u64)`

- <span id="sel4-fault-unknownsyscall-width-of-rbx"></span>`const fn width_of_RBX() -> usize`

- <span id="sel4-fault-unknownsyscall-get-rcx"></span>`fn get_RCX(&self) -> u64`

- <span id="sel4-fault-unknownsyscall-set-rcx"></span>`fn set_RCX(&mut self, RCX: u64)`

- <span id="sel4-fault-unknownsyscall-width-of-rcx"></span>`const fn width_of_RCX() -> usize`

- <span id="sel4-fault-unknownsyscall-get-rdx"></span>`fn get_RDX(&self) -> u64`

- <span id="sel4-fault-unknownsyscall-set-rdx"></span>`fn set_RDX(&mut self, RDX: u64)`

- <span id="sel4-fault-unknownsyscall-width-of-rdx"></span>`const fn width_of_RDX() -> usize`

- <span id="sel4-fault-unknownsyscall-get-rsi"></span>`fn get_RSI(&self) -> u64`

- <span id="sel4-fault-unknownsyscall-set-rsi"></span>`fn set_RSI(&mut self, RSI: u64)`

- <span id="sel4-fault-unknownsyscall-width-of-rsi"></span>`const fn width_of_RSI() -> usize`

- <span id="sel4-fault-unknownsyscall-get-rdi"></span>`fn get_RDI(&self) -> u64`

- <span id="sel4-fault-unknownsyscall-set-rdi"></span>`fn set_RDI(&mut self, RDI: u64)`

- <span id="sel4-fault-unknownsyscall-width-of-rdi"></span>`const fn width_of_RDI() -> usize`

- <span id="sel4-fault-unknownsyscall-get-rbp"></span>`fn get_RBP(&self) -> u64`

- <span id="sel4-fault-unknownsyscall-set-rbp"></span>`fn set_RBP(&mut self, RBP: u64)`

- <span id="sel4-fault-unknownsyscall-width-of-rbp"></span>`const fn width_of_RBP() -> usize`

- <span id="sel4-fault-unknownsyscall-get-r8"></span>`fn get_R8(&self) -> u64`

- <span id="sel4-fault-unknownsyscall-set-r8"></span>`fn set_R8(&mut self, R8: u64)`

- <span id="sel4-fault-unknownsyscall-width-of-r8"></span>`const fn width_of_R8() -> usize`

- <span id="sel4-fault-unknownsyscall-get-r9"></span>`fn get_R9(&self) -> u64`

- <span id="sel4-fault-unknownsyscall-set-r9"></span>`fn set_R9(&mut self, R9: u64)`

- <span id="sel4-fault-unknownsyscall-width-of-r9"></span>`const fn width_of_R9() -> usize`

- <span id="sel4-fault-unknownsyscall-get-r10"></span>`fn get_R10(&self) -> u64`

- <span id="sel4-fault-unknownsyscall-set-r10"></span>`fn set_R10(&mut self, R10: u64)`

- <span id="sel4-fault-unknownsyscall-width-of-r10"></span>`const fn width_of_R10() -> usize`

- <span id="sel4-fault-unknownsyscall-get-r11"></span>`fn get_R11(&self) -> u64`

- <span id="sel4-fault-unknownsyscall-set-r11"></span>`fn set_R11(&mut self, R11: u64)`

- <span id="sel4-fault-unknownsyscall-width-of-r11"></span>`const fn width_of_R11() -> usize`

- <span id="sel4-fault-unknownsyscall-get-r12"></span>`fn get_R12(&self) -> u64`

- <span id="sel4-fault-unknownsyscall-set-r12"></span>`fn set_R12(&mut self, R12: u64)`

- <span id="sel4-fault-unknownsyscall-width-of-r12"></span>`const fn width_of_R12() -> usize`

- <span id="sel4-fault-unknownsyscall-get-r13"></span>`fn get_R13(&self) -> u64`

- <span id="sel4-fault-unknownsyscall-set-r13"></span>`fn set_R13(&mut self, R13: u64)`

- <span id="sel4-fault-unknownsyscall-width-of-r13"></span>`const fn width_of_R13() -> usize`

- <span id="sel4-fault-unknownsyscall-get-r14"></span>`fn get_R14(&self) -> u64`

- <span id="sel4-fault-unknownsyscall-set-r14"></span>`fn set_R14(&mut self, R14: u64)`

- <span id="sel4-fault-unknownsyscall-width-of-r14"></span>`const fn width_of_R14() -> usize`

- <span id="sel4-fault-unknownsyscall-get-r15"></span>`fn get_R15(&self) -> u64`

- <span id="sel4-fault-unknownsyscall-set-r15"></span>`fn set_R15(&mut self, R15: u64)`

- <span id="sel4-fault-unknownsyscall-width-of-r15"></span>`const fn width_of_R15() -> usize`

- <span id="sel4-fault-unknownsyscall-get-faultip"></span>`fn get_FaultIP(&self) -> u64`

- <span id="sel4-fault-unknownsyscall-set-faultip"></span>`fn set_FaultIP(&mut self, FaultIP: u64)`

- <span id="sel4-fault-unknownsyscall-width-of-faultip"></span>`const fn width_of_FaultIP() -> usize`

- <span id="sel4-fault-unknownsyscall-get-rsp"></span>`fn get_RSP(&self) -> u64`

- <span id="sel4-fault-unknownsyscall-set-rsp"></span>`fn set_RSP(&mut self, RSP: u64)`

- <span id="sel4-fault-unknownsyscall-width-of-rsp"></span>`const fn width_of_RSP() -> usize`

- <span id="sel4-fault-unknownsyscall-get-flags"></span>`fn get_FLAGS(&self) -> u64`

- <span id="sel4-fault-unknownsyscall-set-flags"></span>`fn set_FLAGS(&mut self, FLAGS: u64)`

- <span id="sel4-fault-unknownsyscall-width-of-flags"></span>`const fn width_of_FLAGS() -> usize`

- <span id="sel4-fault-unknownsyscall-get-syscall"></span>`fn get_Syscall(&self) -> u64`

- <span id="sel4-fault-unknownsyscall-set-syscall"></span>`fn set_Syscall(&mut self, Syscall: u64)`

- <span id="sel4-fault-unknownsyscall-width-of-syscall"></span>`const fn width_of_Syscall() -> usize`

- <span id="sel4-fault-unknownsyscall-get-sel4-faulttype"></span>`fn get_seL4_FaultType(&self) -> u64`

- <span id="sel4-fault-unknownsyscall-set-sel4-faulttype"></span>`fn set_seL4_FaultType(&mut self, seL4_FaultType: u64)`

- <span id="sel4-fault-unknownsyscall-width-of-sel4-faulttype"></span>`const fn width_of_seL4_FaultType() -> usize`

#### Trait Implementations

##### `impl Clone for seL4_Fault_UnknownSyscall`

- <span id="sel4-fault-unknownsyscall-clone"></span>`fn clone(&self) -> seL4_Fault_UnknownSyscall` — [`seL4_Fault_UnknownSyscall`](../index.md#sel4-fault-unknownsyscall)

##### `impl Debug for seL4_Fault_UnknownSyscall`

- <span id="sel4-fault-unknownsyscall-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for seL4_Fault_UnknownSyscall`

##### `impl PartialEq for seL4_Fault_UnknownSyscall`

- <span id="sel4-fault-unknownsyscall-partialeq-eq"></span>`fn eq(&self, other: &seL4_Fault_UnknownSyscall) -> bool` — [`seL4_Fault_UnknownSyscall`](../index.md#sel4-fault-unknownsyscall)

##### `impl StructuralPartialEq for seL4_Fault_UnknownSyscall`

### `seL4_Fault_UnknownSyscall_Unpacked`

```rust
struct seL4_Fault_UnknownSyscall_Unpacked {
    pub RAX: u64,
    pub RBX: u64,
    pub RCX: u64,
    pub RDX: u64,
    pub RSI: u64,
    pub RDI: u64,
    pub RBP: u64,
    pub R8: u64,
    pub R9: u64,
    pub R10: u64,
    pub R11: u64,
    pub R12: u64,
    pub R13: u64,
    pub R14: u64,
    pub R15: u64,
    pub FaultIP: u64,
    pub RSP: u64,
    pub FLAGS: u64,
    pub Syscall: u64,
}
```

#### Implementations

- <span id="sel4-fault-unknownsyscall-unpacked-pack"></span>`fn pack(self) -> seL4_Fault_UnknownSyscall` — [`seL4_Fault_UnknownSyscall`](../index.md#sel4-fault-unknownsyscall)

#### Trait Implementations

##### `impl Clone for seL4_Fault_UnknownSyscall_Unpacked`

- <span id="sel4-fault-unknownsyscall-unpacked-clone"></span>`fn clone(&self) -> seL4_Fault_UnknownSyscall_Unpacked` — [`seL4_Fault_UnknownSyscall_Unpacked`](../index.md#sel4-fault-unknownsyscall-unpacked)

##### `impl Debug for seL4_Fault_UnknownSyscall_Unpacked`

- <span id="sel4-fault-unknownsyscall-unpacked-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for seL4_Fault_UnknownSyscall_Unpacked`

##### `impl PartialEq for seL4_Fault_UnknownSyscall_Unpacked`

- <span id="sel4-fault-unknownsyscall-unpacked-partialeq-eq"></span>`fn eq(&self, other: &seL4_Fault_UnknownSyscall_Unpacked) -> bool` — [`seL4_Fault_UnknownSyscall_Unpacked`](../index.md#sel4-fault-unknownsyscall-unpacked)

##### `impl StructuralPartialEq for seL4_Fault_UnknownSyscall_Unpacked`

### `seL4_Fault_UserException`

```rust
struct seL4_Fault_UserException(sel4_bitfield_ops::Bitfield<[u64; N], u64>);
```

#### Implementations

- <span id="sel4-fault-userexception-new"></span>`fn new(FaultIP: u64, Stack: u64, FLAGS: u64, Number: u64, Code: u64) -> Self`

- <span id="sel4-fault-userexception-unpack"></span>`fn unpack(&self) -> seL4_Fault_UserException_Unpacked` — [`seL4_Fault_UserException_Unpacked`](../index.md#sel4-fault-userexception-unpacked)

- <span id="sel4-fault-userexception-get-faultip"></span>`fn get_FaultIP(&self) -> u64`

- <span id="sel4-fault-userexception-set-faultip"></span>`fn set_FaultIP(&mut self, FaultIP: u64)`

- <span id="sel4-fault-userexception-width-of-faultip"></span>`const fn width_of_FaultIP() -> usize`

- <span id="sel4-fault-userexception-get-stack"></span>`fn get_Stack(&self) -> u64`

- <span id="sel4-fault-userexception-set-stack"></span>`fn set_Stack(&mut self, Stack: u64)`

- <span id="sel4-fault-userexception-width-of-stack"></span>`const fn width_of_Stack() -> usize`

- <span id="sel4-fault-userexception-get-flags"></span>`fn get_FLAGS(&self) -> u64`

- <span id="sel4-fault-userexception-set-flags"></span>`fn set_FLAGS(&mut self, FLAGS: u64)`

- <span id="sel4-fault-userexception-width-of-flags"></span>`const fn width_of_FLAGS() -> usize`

- <span id="sel4-fault-userexception-get-number"></span>`fn get_Number(&self) -> u64`

- <span id="sel4-fault-userexception-set-number"></span>`fn set_Number(&mut self, Number: u64)`

- <span id="sel4-fault-userexception-width-of-number"></span>`const fn width_of_Number() -> usize`

- <span id="sel4-fault-userexception-get-code"></span>`fn get_Code(&self) -> u64`

- <span id="sel4-fault-userexception-set-code"></span>`fn set_Code(&mut self, Code: u64)`

- <span id="sel4-fault-userexception-width-of-code"></span>`const fn width_of_Code() -> usize`

- <span id="sel4-fault-userexception-get-sel4-faulttype"></span>`fn get_seL4_FaultType(&self) -> u64`

- <span id="sel4-fault-userexception-set-sel4-faulttype"></span>`fn set_seL4_FaultType(&mut self, seL4_FaultType: u64)`

- <span id="sel4-fault-userexception-width-of-sel4-faulttype"></span>`const fn width_of_seL4_FaultType() -> usize`

#### Trait Implementations

##### `impl Clone for seL4_Fault_UserException`

- <span id="sel4-fault-userexception-clone"></span>`fn clone(&self) -> seL4_Fault_UserException` — [`seL4_Fault_UserException`](../index.md#sel4-fault-userexception)

##### `impl Debug for seL4_Fault_UserException`

- <span id="sel4-fault-userexception-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for seL4_Fault_UserException`

##### `impl PartialEq for seL4_Fault_UserException`

- <span id="sel4-fault-userexception-partialeq-eq"></span>`fn eq(&self, other: &seL4_Fault_UserException) -> bool` — [`seL4_Fault_UserException`](../index.md#sel4-fault-userexception)

##### `impl StructuralPartialEq for seL4_Fault_UserException`

### `seL4_Fault_UserException_Unpacked`

```rust
struct seL4_Fault_UserException_Unpacked {
    pub FaultIP: u64,
    pub Stack: u64,
    pub FLAGS: u64,
    pub Number: u64,
    pub Code: u64,
}
```

#### Implementations

- <span id="sel4-fault-userexception-unpacked-pack"></span>`fn pack(self) -> seL4_Fault_UserException` — [`seL4_Fault_UserException`](../index.md#sel4-fault-userexception)

#### Trait Implementations

##### `impl Clone for seL4_Fault_UserException_Unpacked`

- <span id="sel4-fault-userexception-unpacked-clone"></span>`fn clone(&self) -> seL4_Fault_UserException_Unpacked` — [`seL4_Fault_UserException_Unpacked`](../index.md#sel4-fault-userexception-unpacked)

##### `impl Debug for seL4_Fault_UserException_Unpacked`

- <span id="sel4-fault-userexception-unpacked-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for seL4_Fault_UserException_Unpacked`

##### `impl PartialEq for seL4_Fault_UserException_Unpacked`

- <span id="sel4-fault-userexception-unpacked-partialeq-eq"></span>`fn eq(&self, other: &seL4_Fault_UserException_Unpacked) -> bool` — [`seL4_Fault_UserException_Unpacked`](../index.md#sel4-fault-userexception-unpacked)

##### `impl StructuralPartialEq for seL4_Fault_UserException_Unpacked`

### `seL4_Fault_VMFault`

```rust
struct seL4_Fault_VMFault(sel4_bitfield_ops::Bitfield<[u64; N], u64>);
```

#### Implementations

- <span id="sel4-fault-vmfault-new"></span>`fn new(IP: u64, Addr: u64, PrefetchFault: u64, FSR: u64) -> Self`

- <span id="sel4-fault-vmfault-unpack"></span>`fn unpack(&self) -> seL4_Fault_VMFault_Unpacked` — [`seL4_Fault_VMFault_Unpacked`](../index.md#sel4-fault-vmfault-unpacked)

- <span id="sel4-fault-vmfault-get-ip"></span>`fn get_IP(&self) -> u64`

- <span id="sel4-fault-vmfault-set-ip"></span>`fn set_IP(&mut self, IP: u64)`

- <span id="sel4-fault-vmfault-width-of-ip"></span>`const fn width_of_IP() -> usize`

- <span id="sel4-fault-vmfault-get-addr"></span>`fn get_Addr(&self) -> u64`

- <span id="sel4-fault-vmfault-set-addr"></span>`fn set_Addr(&mut self, Addr: u64)`

- <span id="sel4-fault-vmfault-width-of-addr"></span>`const fn width_of_Addr() -> usize`

- <span id="sel4-fault-vmfault-get-prefetchfault"></span>`fn get_PrefetchFault(&self) -> u64`

- <span id="sel4-fault-vmfault-set-prefetchfault"></span>`fn set_PrefetchFault(&mut self, PrefetchFault: u64)`

- <span id="sel4-fault-vmfault-width-of-prefetchfault"></span>`const fn width_of_PrefetchFault() -> usize`

- <span id="sel4-fault-vmfault-get-fsr"></span>`fn get_FSR(&self) -> u64`

- <span id="sel4-fault-vmfault-set-fsr"></span>`fn set_FSR(&mut self, FSR: u64)`

- <span id="sel4-fault-vmfault-width-of-fsr"></span>`const fn width_of_FSR() -> usize`

- <span id="sel4-fault-vmfault-get-sel4-faulttype"></span>`fn get_seL4_FaultType(&self) -> u64`

- <span id="sel4-fault-vmfault-set-sel4-faulttype"></span>`fn set_seL4_FaultType(&mut self, seL4_FaultType: u64)`

- <span id="sel4-fault-vmfault-width-of-sel4-faulttype"></span>`const fn width_of_seL4_FaultType() -> usize`

#### Trait Implementations

##### `impl Clone for seL4_Fault_VMFault`

- <span id="sel4-fault-vmfault-clone"></span>`fn clone(&self) -> seL4_Fault_VMFault` — [`seL4_Fault_VMFault`](../index.md#sel4-fault-vmfault)

##### `impl Debug for seL4_Fault_VMFault`

- <span id="sel4-fault-vmfault-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for seL4_Fault_VMFault`

##### `impl PartialEq for seL4_Fault_VMFault`

- <span id="sel4-fault-vmfault-partialeq-eq"></span>`fn eq(&self, other: &seL4_Fault_VMFault) -> bool` — [`seL4_Fault_VMFault`](../index.md#sel4-fault-vmfault)

##### `impl StructuralPartialEq for seL4_Fault_VMFault`

### `seL4_Fault_VMFault_Unpacked`

```rust
struct seL4_Fault_VMFault_Unpacked {
    pub IP: u64,
    pub Addr: u64,
    pub PrefetchFault: u64,
    pub FSR: u64,
}
```

#### Implementations

- <span id="sel4-fault-vmfault-unpacked-pack"></span>`fn pack(self) -> seL4_Fault_VMFault` — [`seL4_Fault_VMFault`](../index.md#sel4-fault-vmfault)

#### Trait Implementations

##### `impl Clone for seL4_Fault_VMFault_Unpacked`

- <span id="sel4-fault-vmfault-unpacked-clone"></span>`fn clone(&self) -> seL4_Fault_VMFault_Unpacked` — [`seL4_Fault_VMFault_Unpacked`](../index.md#sel4-fault-vmfault-unpacked)

##### `impl Debug for seL4_Fault_VMFault_Unpacked`

- <span id="sel4-fault-vmfault-unpacked-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for seL4_Fault_VMFault_Unpacked`

##### `impl PartialEq for seL4_Fault_VMFault_Unpacked`

- <span id="sel4-fault-vmfault-unpacked-partialeq-eq"></span>`fn eq(&self, other: &seL4_Fault_VMFault_Unpacked) -> bool` — [`seL4_Fault_VMFault_Unpacked`](../index.md#sel4-fault-vmfault-unpacked)

##### `impl StructuralPartialEq for seL4_Fault_VMFault_Unpacked`

### `seL4_Fault`

```rust
struct seL4_Fault(sel4_bitfield_ops::Bitfield<[u64; N], u64>);
```

#### Implementations

- <span id="sel4-fault-splay"></span>`fn splay(self) -> seL4_Fault_Splayed` — [`seL4_Fault_Splayed`](../index.md#sel4-fault-splayed)

- <span id="sel4-fault-get-tag"></span>`fn get_tag(&self) -> u64`

#### Trait Implementations

##### `impl Clone for seL4_Fault`

- <span id="sel4-fault-clone"></span>`fn clone(&self) -> seL4_Fault` — [`seL4_Fault`](../index.md#sel4-fault)

##### `impl Debug for seL4_Fault`

- <span id="sel4-fault-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for seL4_Fault`

##### `impl PartialEq for seL4_Fault`

- <span id="sel4-fault-partialeq-eq"></span>`fn eq(&self, other: &seL4_Fault) -> bool` — [`seL4_Fault`](../index.md#sel4-fault)

##### `impl StructuralPartialEq for seL4_Fault`

### `seL4_MessageInfo`

```rust
struct seL4_MessageInfo(sel4_bitfield_ops::Bitfield<[u64; N], u64>);
```

#### Implementations

- <span id="sel4-messageinfo-new"></span>`fn new(label: u64, capsUnwrapped: u64, extraCaps: u64, length: u64) -> Self`

- <span id="sel4-messageinfo-unpack"></span>`fn unpack(&self) -> seL4_MessageInfo_Unpacked` — [`seL4_MessageInfo_Unpacked`](../index.md#sel4-messageinfo-unpacked)

- <span id="sel4-messageinfo-get-label"></span>`fn get_label(&self) -> u64`

- <span id="sel4-messageinfo-set-label"></span>`fn set_label(&mut self, label: u64)`

- <span id="sel4-messageinfo-width-of-label"></span>`const fn width_of_label() -> usize`

- <span id="sel4-messageinfo-get-capsunwrapped"></span>`fn get_capsUnwrapped(&self) -> u64`

- <span id="sel4-messageinfo-set-capsunwrapped"></span>`fn set_capsUnwrapped(&mut self, capsUnwrapped: u64)`

- <span id="sel4-messageinfo-width-of-capsunwrapped"></span>`const fn width_of_capsUnwrapped() -> usize`

- <span id="sel4-messageinfo-get-extracaps"></span>`fn get_extraCaps(&self) -> u64`

- <span id="sel4-messageinfo-set-extracaps"></span>`fn set_extraCaps(&mut self, extraCaps: u64)`

- <span id="sel4-messageinfo-width-of-extracaps"></span>`const fn width_of_extraCaps() -> usize`

- <span id="sel4-messageinfo-get-length"></span>`fn get_length(&self) -> u64`

- <span id="sel4-messageinfo-set-length"></span>`fn set_length(&mut self, length: u64)`

- <span id="sel4-messageinfo-width-of-length"></span>`const fn width_of_length() -> usize`

#### Trait Implementations

##### `impl Clone for seL4_MessageInfo`

- <span id="sel4-messageinfo-clone"></span>`fn clone(&self) -> seL4_MessageInfo` — [`seL4_MessageInfo`](../index.md#sel4-messageinfo)

##### `impl Debug for seL4_MessageInfo`

- <span id="sel4-messageinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for seL4_MessageInfo`

##### `impl PartialEq for seL4_MessageInfo`

- <span id="sel4-messageinfo-partialeq-eq"></span>`fn eq(&self, other: &seL4_MessageInfo) -> bool` — [`seL4_MessageInfo`](../index.md#sel4-messageinfo)

##### `impl StructuralPartialEq for seL4_MessageInfo`

### `seL4_MessageInfo_Unpacked`

```rust
struct seL4_MessageInfo_Unpacked {
    pub label: u64,
    pub capsUnwrapped: u64,
    pub extraCaps: u64,
    pub length: u64,
}
```

#### Implementations

- <span id="sel4-messageinfo-unpacked-pack"></span>`fn pack(self) -> seL4_MessageInfo` — [`seL4_MessageInfo`](../index.md#sel4-messageinfo)

#### Trait Implementations

##### `impl Clone for seL4_MessageInfo_Unpacked`

- <span id="sel4-messageinfo-unpacked-clone"></span>`fn clone(&self) -> seL4_MessageInfo_Unpacked` — [`seL4_MessageInfo_Unpacked`](../index.md#sel4-messageinfo-unpacked)

##### `impl Debug for seL4_MessageInfo_Unpacked`

- <span id="sel4-messageinfo-unpacked-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for seL4_MessageInfo_Unpacked`

##### `impl PartialEq for seL4_MessageInfo_Unpacked`

- <span id="sel4-messageinfo-unpacked-partialeq-eq"></span>`fn eq(&self, other: &seL4_MessageInfo_Unpacked) -> bool` — [`seL4_MessageInfo_Unpacked`](../index.md#sel4-messageinfo-unpacked)

##### `impl StructuralPartialEq for seL4_MessageInfo_Unpacked`

### `seL4_CapRights`

```rust
struct seL4_CapRights(sel4_bitfield_ops::Bitfield<[u64; N], u64>);
```

#### Implementations

- <span id="sel4-caprights-new"></span>`fn new(capAllowGrantReply: u64, capAllowGrant: u64, capAllowRead: u64, capAllowWrite: u64) -> Self`

- <span id="sel4-caprights-unpack"></span>`fn unpack(&self) -> seL4_CapRights_Unpacked` — [`seL4_CapRights_Unpacked`](../index.md#sel4-caprights-unpacked)

- <span id="sel4-caprights-get-capallowgrantreply"></span>`fn get_capAllowGrantReply(&self) -> u64`

- <span id="sel4-caprights-set-capallowgrantreply"></span>`fn set_capAllowGrantReply(&mut self, capAllowGrantReply: u64)`

- <span id="sel4-caprights-width-of-capallowgrantreply"></span>`const fn width_of_capAllowGrantReply() -> usize`

- <span id="sel4-caprights-get-capallowgrant"></span>`fn get_capAllowGrant(&self) -> u64`

- <span id="sel4-caprights-set-capallowgrant"></span>`fn set_capAllowGrant(&mut self, capAllowGrant: u64)`

- <span id="sel4-caprights-width-of-capallowgrant"></span>`const fn width_of_capAllowGrant() -> usize`

- <span id="sel4-caprights-get-capallowread"></span>`fn get_capAllowRead(&self) -> u64`

- <span id="sel4-caprights-set-capallowread"></span>`fn set_capAllowRead(&mut self, capAllowRead: u64)`

- <span id="sel4-caprights-width-of-capallowread"></span>`const fn width_of_capAllowRead() -> usize`

- <span id="sel4-caprights-get-capallowwrite"></span>`fn get_capAllowWrite(&self) -> u64`

- <span id="sel4-caprights-set-capallowwrite"></span>`fn set_capAllowWrite(&mut self, capAllowWrite: u64)`

- <span id="sel4-caprights-width-of-capallowwrite"></span>`const fn width_of_capAllowWrite() -> usize`

#### Trait Implementations

##### `impl Clone for seL4_CapRights`

- <span id="sel4-caprights-clone"></span>`fn clone(&self) -> seL4_CapRights` — [`seL4_CapRights`](../index.md#sel4-caprights)

##### `impl Debug for seL4_CapRights`

- <span id="sel4-caprights-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for seL4_CapRights`

##### `impl PartialEq for seL4_CapRights`

- <span id="sel4-caprights-partialeq-eq"></span>`fn eq(&self, other: &seL4_CapRights) -> bool` — [`seL4_CapRights`](../index.md#sel4-caprights)

##### `impl StructuralPartialEq for seL4_CapRights`

### `seL4_CapRights_Unpacked`

```rust
struct seL4_CapRights_Unpacked {
    pub capAllowGrantReply: u64,
    pub capAllowGrant: u64,
    pub capAllowRead: u64,
    pub capAllowWrite: u64,
}
```

#### Implementations

- <span id="sel4-caprights-unpacked-pack"></span>`fn pack(self) -> seL4_CapRights` — [`seL4_CapRights`](../index.md#sel4-caprights)

#### Trait Implementations

##### `impl Clone for seL4_CapRights_Unpacked`

- <span id="sel4-caprights-unpacked-clone"></span>`fn clone(&self) -> seL4_CapRights_Unpacked` — [`seL4_CapRights_Unpacked`](../index.md#sel4-caprights-unpacked)

##### `impl Debug for seL4_CapRights_Unpacked`

- <span id="sel4-caprights-unpacked-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for seL4_CapRights_Unpacked`

##### `impl PartialEq for seL4_CapRights_Unpacked`

- <span id="sel4-caprights-unpacked-partialeq-eq"></span>`fn eq(&self, other: &seL4_CapRights_Unpacked) -> bool` — [`seL4_CapRights_Unpacked`](../index.md#sel4-caprights-unpacked)

##### `impl StructuralPartialEq for seL4_CapRights_Unpacked`

### `seL4_CNode_CapData`

```rust
struct seL4_CNode_CapData(sel4_bitfield_ops::Bitfield<[u64; N], u64>);
```

#### Implementations

- <span id="sel4-cnode-capdata-new"></span>`fn new(guard: u64, guardSize: u64) -> Self`

- <span id="sel4-cnode-capdata-unpack"></span>`fn unpack(&self) -> seL4_CNode_CapData_Unpacked` — [`seL4_CNode_CapData_Unpacked`](../index.md#sel4-cnode-capdata-unpacked)

- <span id="sel4-cnode-capdata-get-guard"></span>`fn get_guard(&self) -> u64`

- <span id="sel4-cnode-capdata-set-guard"></span>`fn set_guard(&mut self, guard: u64)`

- <span id="sel4-cnode-capdata-width-of-guard"></span>`const fn width_of_guard() -> usize`

- <span id="sel4-cnode-capdata-get-guardsize"></span>`fn get_guardSize(&self) -> u64`

- <span id="sel4-cnode-capdata-set-guardsize"></span>`fn set_guardSize(&mut self, guardSize: u64)`

- <span id="sel4-cnode-capdata-width-of-guardsize"></span>`const fn width_of_guardSize() -> usize`

#### Trait Implementations

##### `impl Clone for seL4_CNode_CapData`

- <span id="sel4-cnode-capdata-clone"></span>`fn clone(&self) -> seL4_CNode_CapData` — [`seL4_CNode_CapData`](../index.md#sel4-cnode-capdata)

##### `impl Debug for seL4_CNode_CapData`

- <span id="sel4-cnode-capdata-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for seL4_CNode_CapData`

##### `impl PartialEq for seL4_CNode_CapData`

- <span id="sel4-cnode-capdata-partialeq-eq"></span>`fn eq(&self, other: &seL4_CNode_CapData) -> bool` — [`seL4_CNode_CapData`](../index.md#sel4-cnode-capdata)

##### `impl StructuralPartialEq for seL4_CNode_CapData`

### `seL4_CNode_CapData_Unpacked`

```rust
struct seL4_CNode_CapData_Unpacked {
    pub guard: u64,
    pub guardSize: u64,
}
```

#### Implementations

- <span id="sel4-cnode-capdata-unpacked-pack"></span>`fn pack(self) -> seL4_CNode_CapData` — [`seL4_CNode_CapData`](../index.md#sel4-cnode-capdata)

#### Trait Implementations

##### `impl Clone for seL4_CNode_CapData_Unpacked`

- <span id="sel4-cnode-capdata-unpacked-clone"></span>`fn clone(&self) -> seL4_CNode_CapData_Unpacked` — [`seL4_CNode_CapData_Unpacked`](../index.md#sel4-cnode-capdata-unpacked)

##### `impl Debug for seL4_CNode_CapData_Unpacked`

- <span id="sel4-cnode-capdata-unpacked-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for seL4_CNode_CapData_Unpacked`

##### `impl PartialEq for seL4_CNode_CapData_Unpacked`

- <span id="sel4-cnode-capdata-unpacked-partialeq-eq"></span>`fn eq(&self, other: &seL4_CNode_CapData_Unpacked) -> bool` — [`seL4_CNode_CapData_Unpacked`](../index.md#sel4-cnode-capdata-unpacked)

##### `impl StructuralPartialEq for seL4_CNode_CapData_Unpacked`

## Enums

### `seL4_Fault_Splayed`

```rust
enum seL4_Fault_Splayed {
    NullFault(seL4_Fault_NullFault),
    CapFault(seL4_Fault_CapFault),
    UnknownSyscall(seL4_Fault_UnknownSyscall),
    UserException(seL4_Fault_UserException),
    VMFault(seL4_Fault_VMFault),
}
```

#### Implementations

- <span id="sel4-fault-splayed-unsplay"></span>`fn unsplay(self) -> seL4_Fault` — [`seL4_Fault`](../index.md#sel4-fault)

#### Trait Implementations

##### `impl Clone for seL4_Fault_Splayed`

- <span id="sel4-fault-splayed-clone"></span>`fn clone(&self) -> seL4_Fault_Splayed` — [`seL4_Fault_Splayed`](../index.md#sel4-fault-splayed)

##### `impl Debug for seL4_Fault_Splayed`

- <span id="sel4-fault-splayed-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for seL4_Fault_Splayed`

##### `impl PartialEq for seL4_Fault_Splayed`

- <span id="sel4-fault-splayed-partialeq-eq"></span>`fn eq(&self, other: &seL4_Fault_Splayed) -> bool` — [`seL4_Fault_Splayed`](../index.md#sel4-fault-splayed)

##### `impl StructuralPartialEq for seL4_Fault_Splayed`

## Type Aliases

### `SeL4Bitfield<T, const N: usize>`

```rust
type SeL4Bitfield<T, const N: usize> = sel4_bitfield_ops::Bitfield<[T; N], T>;
```

### `seL4_MessageInfo_t`

```rust
type seL4_MessageInfo_t = seL4_MessageInfo;
```

### `seL4_CapRights_t`

```rust
type seL4_CapRights_t = seL4_CapRights;
```

### `seL4_CNode_CapData_t`

```rust
type seL4_CNode_CapData_t = seL4_CNode_CapData;
```

