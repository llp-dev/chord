*[sel4_sys](../index.md) / [syscalls](index.md)*

---

# Module `syscalls`

## Contents

- [Modules](#modules)
  - [`calls`](#calls)
  - [`helpers`](#helpers)
  - [`syscall_id`](#syscall-id)
- [Functions](#functions)
  - [`reply_authority_to_sys_arg`](#reply-authority-to-sys-arg)
  - [`sys_send_recv_simple`](#sys-send-recv-simple)
  - [`seL4_SendWithMRsWithoutIPCBuffer`](#sel4-sendwithmrswithoutipcbuffer)
  - [`seL4_NBSendWithMRsWithoutIPCBuffer`](#sel4-nbsendwithmrswithoutipcbuffer)
  - [`seL4_ReplyWithMRsWithoutIPCBuffer`](#sel4-replywithmrswithoutipcbuffer)
  - [`seL4_RecvWithMRsWithoutIPCBuffer`](#sel4-recvwithmrswithoutipcbuffer)
  - [`seL4_CallWithMRsWithoutIPCBuffer`](#sel4-callwithmrswithoutipcbuffer)
  - [`seL4_Yield`](#sel4-yield)
  - [`seL4_DebugPutChar`](#sel4-debugputchar)
  - [`seL4_DebugHalt`](#sel4-debughalt)
  - [`seL4_DebugSnapshot`](#sel4-debugsnapshot)
  - [`seL4_DebugCapIdentify`](#sel4-debugcapidentify)
  - [`seL4_DebugNameThread`](#sel4-debugnamethread)
- [Constants](#constants)
  - [`UNUSED_REPLY_ARG`](#unused-reply-arg)
- [Macros](#macros)
  - [`fill_mrs_from_ipc_buffer!`](#fill-mrs-from-ipc-buffer)
  - [`empty_mrs_to_ipc_buffer!`](#empty-mrs-to-ipc-buffer)
  - [`fill_mrs_from_args!`](#fill-mrs-from-args)
  - [`fill_mrs_from_in_args!`](#fill-mrs-from-in-args)
  - [`empty_mrs_to_args!`](#empty-mrs-to-args)
  - [`fence!`](#fence)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`calls`](#calls) | mod |  |
| [`helpers`](#helpers) | mod |  |
| [`syscall_id`](#syscall-id) | mod |  |
| [`reply_authority_to_sys_arg`](#reply-authority-to-sys-arg) | fn |  |
| [`sys_send_recv_simple`](#sys-send-recv-simple) | fn |  |
| [`seL4_SendWithMRsWithoutIPCBuffer`](#sel4-sendwithmrswithoutipcbuffer) | fn |  |
| [`seL4_NBSendWithMRsWithoutIPCBuffer`](#sel4-nbsendwithmrswithoutipcbuffer) | fn |  |
| [`seL4_ReplyWithMRsWithoutIPCBuffer`](#sel4-replywithmrswithoutipcbuffer) | fn |  |
| [`seL4_RecvWithMRsWithoutIPCBuffer`](#sel4-recvwithmrswithoutipcbuffer) | fn |  |
| [`seL4_CallWithMRsWithoutIPCBuffer`](#sel4-callwithmrswithoutipcbuffer) | fn |  |
| [`seL4_Yield`](#sel4-yield) | fn |  |
| [`seL4_DebugPutChar`](#sel4-debugputchar) | fn |  |
| [`seL4_DebugHalt`](#sel4-debughalt) | fn |  |
| [`seL4_DebugSnapshot`](#sel4-debugsnapshot) | fn |  |
| [`seL4_DebugCapIdentify`](#sel4-debugcapidentify) | fn |  |
| [`seL4_DebugNameThread`](#sel4-debugnamethread) | fn |  |
| [`UNUSED_REPLY_ARG`](#unused-reply-arg) | const |  |
| [`fill_mrs_from_ipc_buffer!`](#fill-mrs-from-ipc-buffer) | macro |  |
| [`empty_mrs_to_ipc_buffer!`](#empty-mrs-to-ipc-buffer) | macro |  |
| [`fill_mrs_from_args!`](#fill-mrs-from-args) | macro |  |
| [`fill_mrs_from_in_args!`](#fill-mrs-from-in-args) | macro |  |
| [`empty_mrs_to_args!`](#empty-mrs-to-args) | macro |  |
| [`fence!`](#fence) | macro |  |

## Modules

- [`calls`](calls/index.md)
- [`helpers`](helpers/index.md)
- [`syscall_id`](syscall_id/index.md)

## Functions

### `reply_authority_to_sys_arg`

```rust
fn reply_authority_to_sys_arg(authority: crate::ReplyAuthority) -> crate::seL4_Word
```

### `sys_send_recv_simple`

```rust
fn sys_send_recv_simple(sys_id: core::ffi::c_int, arg: crate::seL4_Word) -> crate::seL4_Word
```

### `seL4_SendWithMRsWithoutIPCBuffer`

```rust
fn seL4_SendWithMRsWithoutIPCBuffer(dest: crate::seL4_CPtr, msg_info: crate::seL4_MessageInfo, msg0: Option<crate::seL4_Word>, msg1: Option<crate::seL4_Word>, msg2: Option<crate::seL4_Word>, msg3: Option<crate::seL4_Word>)
```

### `seL4_NBSendWithMRsWithoutIPCBuffer`

```rust
fn seL4_NBSendWithMRsWithoutIPCBuffer(dest: crate::seL4_CPtr, msg_info: crate::seL4_MessageInfo, msg0: Option<crate::seL4_Word>, msg1: Option<crate::seL4_Word>, msg2: Option<crate::seL4_Word>, msg3: Option<crate::seL4_Word>)
```

### `seL4_ReplyWithMRsWithoutIPCBuffer`

```rust
fn seL4_ReplyWithMRsWithoutIPCBuffer(msg_info: crate::seL4_MessageInfo, msg0: Option<crate::seL4_Word>, msg1: Option<crate::seL4_Word>, msg2: Option<crate::seL4_Word>, msg3: Option<crate::seL4_Word>)
```

### `seL4_RecvWithMRsWithoutIPCBuffer`

```rust
fn seL4_RecvWithMRsWithoutIPCBuffer(src: crate::seL4_CPtr, msg0: Option<&mut crate::seL4_Word>, msg1: Option<&mut crate::seL4_Word>, msg2: Option<&mut crate::seL4_Word>, msg3: Option<&mut crate::seL4_Word>, reply_authority: crate::ReplyAuthority) -> (crate::seL4_MessageInfo, crate::seL4_Word)
```

### `seL4_CallWithMRsWithoutIPCBuffer`

```rust
fn seL4_CallWithMRsWithoutIPCBuffer(dest: crate::seL4_CPtr, msg_info: crate::seL4_MessageInfo, msg0: Option<&mut crate::seL4_Word>, msg1: Option<&mut crate::seL4_Word>, msg2: Option<&mut crate::seL4_Word>, msg3: Option<&mut crate::seL4_Word>) -> crate::seL4_MessageInfo
```

### `seL4_Yield`

```rust
fn seL4_Yield()
```

### `seL4_DebugPutChar`

```rust
fn seL4_DebugPutChar(c: u8)
```

### `seL4_DebugHalt`

```rust
fn seL4_DebugHalt()
```

### `seL4_DebugSnapshot`

```rust
fn seL4_DebugSnapshot()
```

### `seL4_DebugCapIdentify`

```rust
fn seL4_DebugCapIdentify(cap: crate::seL4_CPtr) -> crate::seL4_Uint32
```

### `seL4_DebugNameThread`

```rust
fn seL4_DebugNameThread(tcb: crate::seL4_CPtr, name: &[u8], ipc_buffer: &mut crate::seL4_IPCBuffer)
```

## Constants

### `UNUSED_REPLY_ARG`
```rust
const UNUSED_REPLY_ARG: crate::seL4_Word = 0u64;
```

## Macros

### `fill_mrs_from_ipc_buffer!`

### `empty_mrs_to_ipc_buffer!`

### `fill_mrs_from_args!`

### `fill_mrs_from_in_args!`

### `empty_mrs_to_args!`

### `fence!`

