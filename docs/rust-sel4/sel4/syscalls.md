**sel4 > syscalls**

# Module: syscalls

## Contents

**Structs**

- [`CallWithMRs`](#callwithmrs) - The result of [`cap::Endpoint::call_with_mrs`].
- [`RecvWithMRs`](#recvwithmrs) - The result of [`cap::Endpoint::recv_with_mrs`].

**Functions**

- [`reply`](#reply) - Corresponds to `seL4_Reply`.
- [`yield`](#yield) - Corresponds to `seL4_Yield`.

**Traits**

- [`FastMessages`](#fastmessages) - Trait for types which can hold the contents of a set of inline message registers.
- [`IpcCapType`](#ipccaptype) - Trait for [`CapType`]s which are used as targets of IPC syscalls.

**Constants**

- [`NUM_MESSAGE_REGISTERS`](#num_message_registers) - Number of message registers in the IPC buffer.

**Type Aliases**

- [`Badge`](#badge) - A capability badge.

---

## sel4::syscalls::Badge

*Type Alias*: `crate::Word`

A capability badge.



## sel4::syscalls::CallWithMRs

*Struct*

The result of [`cap::Endpoint::call_with_mrs`].

**Fields:**
- `info: crate::MessageInfo`
- `msg: [crate::Word; 4]`



## sel4::syscalls::FastMessages

*Trait*

Trait for types which can hold the contents of a set of inline message registers.

**Methods:**

- `prepare_in`
- `prepare_in_out`



## sel4::syscalls::IpcCapType

*Trait*

Trait for [`CapType`]s which are used as targets of IPC syscalls.



## sel4::syscalls::NUM_MESSAGE_REGISTERS

*Constant*: `usize`

Number of message registers in the IPC buffer.



## sel4::syscalls::RecvWithMRs

*Struct*

The result of [`cap::Endpoint::recv_with_mrs`].

**Fields:**
- `info: crate::MessageInfo`
- `badge: Badge`
- `msg: [crate::Word; 4]`



## sel4::syscalls::reply

*Function*

Corresponds to `seL4_Reply`.

```rust
fn reply(ipc_buffer: & mut crate::IpcBuffer, info: crate::MessageInfo)
```



## sel4::syscalls::yield

*Function*

Corresponds to `seL4_Yield`.

```rust
fn yield()
```



