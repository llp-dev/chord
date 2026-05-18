**sel4_sys > bf**

# Module: bf

## Contents

**Modules**

- [`seL4_Fault_tag`](#sel4_fault_tag)

**Structs**

- [`seL4_CNode_CapData`](#sel4_cnode_capdata)
- [`seL4_CNode_CapData_Unpacked`](#sel4_cnode_capdata_unpacked)
- [`seL4_CapRights`](#sel4_caprights)
- [`seL4_CapRights_Unpacked`](#sel4_caprights_unpacked)
- [`seL4_Fault`](#sel4_fault)
- [`seL4_Fault_CapFault`](#sel4_fault_capfault)
- [`seL4_Fault_CapFault_Unpacked`](#sel4_fault_capfault_unpacked)
- [`seL4_Fault_NullFault`](#sel4_fault_nullfault)
- [`seL4_Fault_NullFault_Unpacked`](#sel4_fault_nullfault_unpacked)
- [`seL4_Fault_UnknownSyscall`](#sel4_fault_unknownsyscall)
- [`seL4_Fault_UnknownSyscall_Unpacked`](#sel4_fault_unknownsyscall_unpacked)
- [`seL4_Fault_UserException`](#sel4_fault_userexception)
- [`seL4_Fault_UserException_Unpacked`](#sel4_fault_userexception_unpacked)
- [`seL4_Fault_VMFault`](#sel4_fault_vmfault)
- [`seL4_Fault_VMFault_Unpacked`](#sel4_fault_vmfault_unpacked)
- [`seL4_MessageInfo`](#sel4_messageinfo)
- [`seL4_MessageInfo_Unpacked`](#sel4_messageinfo_unpacked)

**Enums**

- [`seL4_Fault_Splayed`](#sel4_fault_splayed)

**Type Aliases**

- [`seL4_CNode_CapData_t`](#sel4_cnode_capdata_t)
- [`seL4_CapRights_t`](#sel4_caprights_t)
- [`seL4_MessageInfo_t`](#sel4_messageinfo_t)

---

## sel4_sys::bf::seL4_CNode_CapData

*Struct*

**Tuple Struct**: `(sel4_bitfield_ops::Bitfield<[u64; N], u64>)`

**Methods:**

- `fn new(guard: u64, guardSize: u64) -> Self`
- `fn unpack(self: &Self) -> seL4_CNode_CapData_Unpacked`
- `fn get_guard(self: &Self) -> u64`
- `fn set_guard(self: & mut Self, guard: u64)`
- `fn width_of_guard() -> usize`
- `fn get_guardSize(self: &Self) -> u64`
- `fn set_guardSize(self: & mut Self, guardSize: u64)`
- `fn width_of_guardSize() -> usize`

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &seL4_CNode_CapData) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> seL4_CNode_CapData`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## sel4_sys::bf::seL4_CNode_CapData_Unpacked

*Struct*

**Fields:**
- `guard: u64`
- `guardSize: u64`

**Methods:**

- `fn pack(self: Self) -> seL4_CNode_CapData`

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> seL4_CNode_CapData_Unpacked`
- **PartialEq**
  - `fn eq(self: &Self, other: &seL4_CNode_CapData_Unpacked) -> bool`



## sel4_sys::bf::seL4_CNode_CapData_t

*Type Alias*: `seL4_CNode_CapData`



## sel4_sys::bf::seL4_CapRights

*Struct*

**Tuple Struct**: `(sel4_bitfield_ops::Bitfield<[u64; N], u64>)`

**Methods:**

- `fn new(capAllowGrantReply: u64, capAllowGrant: u64, capAllowRead: u64, capAllowWrite: u64) -> Self`
- `fn unpack(self: &Self) -> seL4_CapRights_Unpacked`
- `fn get_capAllowGrantReply(self: &Self) -> u64`
- `fn set_capAllowGrantReply(self: & mut Self, capAllowGrantReply: u64)`
- `fn width_of_capAllowGrantReply() -> usize`
- `fn get_capAllowGrant(self: &Self) -> u64`
- `fn set_capAllowGrant(self: & mut Self, capAllowGrant: u64)`
- `fn width_of_capAllowGrant() -> usize`
- `fn get_capAllowRead(self: &Self) -> u64`
- `fn set_capAllowRead(self: & mut Self, capAllowRead: u64)`
- `fn width_of_capAllowRead() -> usize`
- `fn get_capAllowWrite(self: &Self) -> u64`
- `fn set_capAllowWrite(self: & mut Self, capAllowWrite: u64)`
- `fn width_of_capAllowWrite() -> usize`

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &seL4_CapRights) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> seL4_CapRights`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## sel4_sys::bf::seL4_CapRights_Unpacked

*Struct*

**Fields:**
- `capAllowGrantReply: u64`
- `capAllowGrant: u64`
- `capAllowRead: u64`
- `capAllowWrite: u64`

**Methods:**

- `fn pack(self: Self) -> seL4_CapRights`

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &seL4_CapRights_Unpacked) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> seL4_CapRights_Unpacked`



## sel4_sys::bf::seL4_CapRights_t

*Type Alias*: `seL4_CapRights`



## sel4_sys::bf::seL4_Fault

*Struct*

**Tuple Struct**: `(sel4_bitfield_ops::Bitfield<[u64; N], u64>)`

**Methods:**

- `fn get_from_ipc_buffer(info: &seL4_MessageInfo, ipcbuf: &seL4_IPCBuffer) -> Self`
- `fn get_with<impl Fn(core::ffi::c_ulong) -> seL4_Word>(label: seL4_Word, length: seL4_Word, f: impl Trait) -> Self`
- `fn splay(self: Self) -> seL4_Fault_Splayed`
- `fn get_tag(self: &Self) -> u64`

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> seL4_Fault`
- **PartialEq**
  - `fn eq(self: &Self, other: &seL4_Fault) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## sel4_sys::bf::seL4_Fault_CapFault

*Struct*

**Tuple Struct**: `(sel4_bitfield_ops::Bitfield<[u64; N], u64>)`

**Methods:**

- `fn unsplay(self: Self) -> seL4_Fault`
- `fn new(IP: u64, Addr: u64, InRecvPhase: u64, LookupFailureType: u64, MR4: u64, MR5: u64, MR6: u64) -> Self`
- `fn unpack(self: &Self) -> seL4_Fault_CapFault_Unpacked`
- `fn get_IP(self: &Self) -> u64`
- `fn set_IP(self: & mut Self, IP: u64)`
- `fn width_of_IP() -> usize`
- `fn get_Addr(self: &Self) -> u64`
- `fn set_Addr(self: & mut Self, Addr: u64)`
- `fn width_of_Addr() -> usize`
- `fn get_InRecvPhase(self: &Self) -> u64`
- `fn set_InRecvPhase(self: & mut Self, InRecvPhase: u64)`
- `fn width_of_InRecvPhase() -> usize`
- `fn get_LookupFailureType(self: &Self) -> u64`
- `fn set_LookupFailureType(self: & mut Self, LookupFailureType: u64)`
- `fn width_of_LookupFailureType() -> usize`
- `fn get_MR4(self: &Self) -> u64`
- `fn set_MR4(self: & mut Self, MR4: u64)`
- `fn width_of_MR4() -> usize`
- `fn get_MR5(self: &Self) -> u64`
- `fn set_MR5(self: & mut Self, MR5: u64)`
- `fn width_of_MR5() -> usize`
- `fn get_MR6(self: &Self) -> u64`
- `fn set_MR6(self: & mut Self, MR6: u64)`
- `fn width_of_MR6() -> usize`

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &seL4_Fault_CapFault) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> seL4_Fault_CapFault`



## sel4_sys::bf::seL4_Fault_CapFault_Unpacked

*Struct*

**Fields:**
- `IP: u64`
- `Addr: u64`
- `InRecvPhase: u64`
- `LookupFailureType: u64`
- `MR4: u64`
- `MR5: u64`
- `MR6: u64`

**Methods:**

- `fn unsplay(self: Self) -> seL4_Fault`
- `fn pack(self: Self) -> seL4_Fault_CapFault`

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> seL4_Fault_CapFault_Unpacked`
- **PartialEq**
  - `fn eq(self: &Self, other: &seL4_Fault_CapFault_Unpacked) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_sys::bf::seL4_Fault_NullFault

*Struct*

**Tuple Struct**: `(sel4_bitfield_ops::Bitfield<[u64; N], u64>)`

**Methods:**

- `fn unsplay(self: Self) -> seL4_Fault`
- `fn new() -> Self`
- `fn unpack(self: &Self) -> seL4_Fault_NullFault_Unpacked`

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &seL4_Fault_NullFault) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> seL4_Fault_NullFault`



## sel4_sys::bf::seL4_Fault_NullFault_Unpacked

*Struct*

**Methods:**

- `fn pack(self: Self) -> seL4_Fault_NullFault`
- `fn unsplay(self: Self) -> seL4_Fault`

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &seL4_Fault_NullFault_Unpacked) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> seL4_Fault_NullFault_Unpacked`



## sel4_sys::bf::seL4_Fault_Splayed

*Enum*

**Variants:**
- `NullFault(seL4_Fault_NullFault)`
- `CapFault(seL4_Fault_CapFault)`
- `UnknownSyscall(seL4_Fault_UnknownSyscall)`
- `UserException(seL4_Fault_UserException)`
- `VMFault(seL4_Fault_VMFault)`

**Methods:**

- `fn unsplay(self: Self) -> seL4_Fault`

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> seL4_Fault_Splayed`
- **PartialEq**
  - `fn eq(self: &Self, other: &seL4_Fault_Splayed) -> bool`



## sel4_sys::bf::seL4_Fault_UnknownSyscall

*Struct*

**Tuple Struct**: `(sel4_bitfield_ops::Bitfield<[u64; N], u64>)`

**Methods:**

- `fn new(RAX: u64, RBX: u64, RCX: u64, RDX: u64, RSI: u64, RDI: u64, RBP: u64, R8: u64, R9: u64, R10: u64, R11: u64, R12: u64, R13: u64, R14: u64, R15: u64, FaultIP: u64, RSP: u64, FLAGS: u64, Syscall: u64) -> Self`
- `fn unpack(self: &Self) -> seL4_Fault_UnknownSyscall_Unpacked`
- `fn get_RAX(self: &Self) -> u64`
- `fn set_RAX(self: & mut Self, RAX: u64)`
- `fn width_of_RAX() -> usize`
- `fn get_RBX(self: &Self) -> u64`
- `fn set_RBX(self: & mut Self, RBX: u64)`
- `fn width_of_RBX() -> usize`
- `fn get_RCX(self: &Self) -> u64`
- `fn set_RCX(self: & mut Self, RCX: u64)`
- `fn width_of_RCX() -> usize`
- `fn get_RDX(self: &Self) -> u64`
- `fn set_RDX(self: & mut Self, RDX: u64)`
- `fn width_of_RDX() -> usize`
- `fn get_RSI(self: &Self) -> u64`
- `fn set_RSI(self: & mut Self, RSI: u64)`
- `fn width_of_RSI() -> usize`
- `fn get_RDI(self: &Self) -> u64`
- `fn set_RDI(self: & mut Self, RDI: u64)`
- `fn width_of_RDI() -> usize`
- `fn get_RBP(self: &Self) -> u64`
- `fn set_RBP(self: & mut Self, RBP: u64)`
- `fn width_of_RBP() -> usize`
- `fn get_R8(self: &Self) -> u64`
- `fn set_R8(self: & mut Self, R8: u64)`
- `fn width_of_R8() -> usize`
- `fn get_R9(self: &Self) -> u64`
- `fn set_R9(self: & mut Self, R9: u64)`
- `fn width_of_R9() -> usize`
- `fn get_R10(self: &Self) -> u64`
- `fn set_R10(self: & mut Self, R10: u64)`
- `fn width_of_R10() -> usize`
- `fn get_R11(self: &Self) -> u64`
- `fn set_R11(self: & mut Self, R11: u64)`
- `fn width_of_R11() -> usize`
- `fn get_R12(self: &Self) -> u64`
- `fn set_R12(self: & mut Self, R12: u64)`
- `fn width_of_R12() -> usize`
- `fn get_R13(self: &Self) -> u64`
- `fn set_R13(self: & mut Self, R13: u64)`
- `fn width_of_R13() -> usize`
- `fn get_R14(self: &Self) -> u64`
- `fn set_R14(self: & mut Self, R14: u64)`
- `fn width_of_R14() -> usize`
- `fn get_R15(self: &Self) -> u64`
- `fn set_R15(self: & mut Self, R15: u64)`
- `fn width_of_R15() -> usize`
- `fn get_FaultIP(self: &Self) -> u64`
- `fn set_FaultIP(self: & mut Self, FaultIP: u64)`
- `fn width_of_FaultIP() -> usize`
- `fn get_RSP(self: &Self) -> u64`
- `fn set_RSP(self: & mut Self, RSP: u64)`
- `fn width_of_RSP() -> usize`
- `fn get_FLAGS(self: &Self) -> u64`
- `fn set_FLAGS(self: & mut Self, FLAGS: u64)`
- `fn width_of_FLAGS() -> usize`
- `fn get_Syscall(self: &Self) -> u64`
- `fn set_Syscall(self: & mut Self, Syscall: u64)`
- `fn width_of_Syscall() -> usize`
- `fn unsplay(self: Self) -> seL4_Fault`

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &seL4_Fault_UnknownSyscall) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> seL4_Fault_UnknownSyscall`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## sel4_sys::bf::seL4_Fault_UnknownSyscall_Unpacked

*Struct*

**Fields:**
- `RAX: u64`
- `RBX: u64`
- `RCX: u64`
- `RDX: u64`
- `RSI: u64`
- `RDI: u64`
- `RBP: u64`
- `R8: u64`
- `R9: u64`
- `R10: u64`
- `R11: u64`
- `R12: u64`
- `R13: u64`
- `R14: u64`
- `R15: u64`
- `FaultIP: u64`
- `RSP: u64`
- `FLAGS: u64`
- `Syscall: u64`

**Methods:**

- `fn pack(self: Self) -> seL4_Fault_UnknownSyscall`
- `fn unsplay(self: Self) -> seL4_Fault`

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> seL4_Fault_UnknownSyscall_Unpacked`
- **PartialEq**
  - `fn eq(self: &Self, other: &seL4_Fault_UnknownSyscall_Unpacked) -> bool`



## sel4_sys::bf::seL4_Fault_UserException

*Struct*

**Tuple Struct**: `(sel4_bitfield_ops::Bitfield<[u64; N], u64>)`

**Methods:**

- `fn new(FaultIP: u64, Stack: u64, FLAGS: u64, Number: u64, Code: u64) -> Self`
- `fn unpack(self: &Self) -> seL4_Fault_UserException_Unpacked`
- `fn get_FaultIP(self: &Self) -> u64`
- `fn set_FaultIP(self: & mut Self, FaultIP: u64)`
- `fn width_of_FaultIP() -> usize`
- `fn get_Stack(self: &Self) -> u64`
- `fn set_Stack(self: & mut Self, Stack: u64)`
- `fn width_of_Stack() -> usize`
- `fn get_FLAGS(self: &Self) -> u64`
- `fn set_FLAGS(self: & mut Self, FLAGS: u64)`
- `fn width_of_FLAGS() -> usize`
- `fn get_Number(self: &Self) -> u64`
- `fn set_Number(self: & mut Self, Number: u64)`
- `fn width_of_Number() -> usize`
- `fn get_Code(self: &Self) -> u64`
- `fn set_Code(self: & mut Self, Code: u64)`
- `fn width_of_Code() -> usize`
- `fn unsplay(self: Self) -> seL4_Fault`

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &seL4_Fault_UserException) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> seL4_Fault_UserException`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## sel4_sys::bf::seL4_Fault_UserException_Unpacked

*Struct*

**Fields:**
- `FaultIP: u64`
- `Stack: u64`
- `FLAGS: u64`
- `Number: u64`
- `Code: u64`

**Methods:**

- `fn unsplay(self: Self) -> seL4_Fault`
- `fn pack(self: Self) -> seL4_Fault_UserException`

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> seL4_Fault_UserException_Unpacked`
- **PartialEq**
  - `fn eq(self: &Self, other: &seL4_Fault_UserException_Unpacked) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_sys::bf::seL4_Fault_VMFault

*Struct*

**Tuple Struct**: `(sel4_bitfield_ops::Bitfield<[u64; N], u64>)`

**Methods:**

- `fn unsplay(self: Self) -> seL4_Fault`
- `fn new(IP: u64, Addr: u64, PrefetchFault: u64, FSR: u64) -> Self`
- `fn unpack(self: &Self) -> seL4_Fault_VMFault_Unpacked`
- `fn get_IP(self: &Self) -> u64`
- `fn set_IP(self: & mut Self, IP: u64)`
- `fn width_of_IP() -> usize`
- `fn get_Addr(self: &Self) -> u64`
- `fn set_Addr(self: & mut Self, Addr: u64)`
- `fn width_of_Addr() -> usize`
- `fn get_PrefetchFault(self: &Self) -> u64`
- `fn set_PrefetchFault(self: & mut Self, PrefetchFault: u64)`
- `fn width_of_PrefetchFault() -> usize`
- `fn get_FSR(self: &Self) -> u64`
- `fn set_FSR(self: & mut Self, FSR: u64)`
- `fn width_of_FSR() -> usize`

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &seL4_Fault_VMFault) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> seL4_Fault_VMFault`



## sel4_sys::bf::seL4_Fault_VMFault_Unpacked

*Struct*

**Fields:**
- `IP: u64`
- `Addr: u64`
- `PrefetchFault: u64`
- `FSR: u64`

**Methods:**

- `fn unsplay(self: Self) -> seL4_Fault`
- `fn pack(self: Self) -> seL4_Fault_VMFault`

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> seL4_Fault_VMFault_Unpacked`
- **PartialEq**
  - `fn eq(self: &Self, other: &seL4_Fault_VMFault_Unpacked) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## Module: seL4_Fault_tag



## sel4_sys::bf::seL4_MessageInfo

*Struct*

**Tuple Struct**: `(sel4_bitfield_ops::Bitfield<[u64; N], u64>)`

**Methods:**

- `fn new(label: u64, capsUnwrapped: u64, extraCaps: u64, length: u64) -> Self`
- `fn unpack(self: &Self) -> seL4_MessageInfo_Unpacked`
- `fn get_label(self: &Self) -> u64`
- `fn set_label(self: & mut Self, label: u64)`
- `fn width_of_label() -> usize`
- `fn get_capsUnwrapped(self: &Self) -> u64`
- `fn set_capsUnwrapped(self: & mut Self, capsUnwrapped: u64)`
- `fn width_of_capsUnwrapped() -> usize`
- `fn get_extraCaps(self: &Self) -> u64`
- `fn set_extraCaps(self: & mut Self, extraCaps: u64)`
- `fn width_of_extraCaps() -> usize`
- `fn get_length(self: &Self) -> u64`
- `fn set_length(self: & mut Self, length: u64)`
- `fn width_of_length() -> usize`

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &seL4_MessageInfo) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> seL4_MessageInfo`



## sel4_sys::bf::seL4_MessageInfo_Unpacked

*Struct*

**Fields:**
- `label: u64`
- `capsUnwrapped: u64`
- `extraCaps: u64`
- `length: u64`

**Methods:**

- `fn pack(self: Self) -> seL4_MessageInfo`

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> seL4_MessageInfo_Unpacked`
- **PartialEq**
  - `fn eq(self: &Self, other: &seL4_MessageInfo_Unpacked) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_sys::bf::seL4_MessageInfo_t

*Type Alias*: `seL4_MessageInfo`



