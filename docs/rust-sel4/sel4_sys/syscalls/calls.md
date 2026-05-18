**sel4_sys > syscalls > calls**

# Module: syscalls::calls

## Contents

**Functions**

- [`seL4_CallWithMRsWithoutIPCBuffer`](#sel4_callwithmrswithoutipcbuffer)
- [`seL4_DebugCapIdentify`](#sel4_debugcapidentify)
- [`seL4_DebugHalt`](#sel4_debughalt)
- [`seL4_DebugNameThread`](#sel4_debugnamethread)
- [`seL4_DebugPutChar`](#sel4_debugputchar)
- [`seL4_DebugSnapshot`](#sel4_debugsnapshot)
- [`seL4_NBSendWithMRsWithoutIPCBuffer`](#sel4_nbsendwithmrswithoutipcbuffer)
- [`seL4_RecvWithMRsWithoutIPCBuffer`](#sel4_recvwithmrswithoutipcbuffer)
- [`seL4_ReplyWithMRsWithoutIPCBuffer`](#sel4_replywithmrswithoutipcbuffer)
- [`seL4_SendWithMRsWithoutIPCBuffer`](#sel4_sendwithmrswithoutipcbuffer)
- [`seL4_Yield`](#sel4_yield)

---

## sel4_sys::syscalls::calls::seL4_CallWithMRsWithoutIPCBuffer

*Function*

```rust
fn seL4_CallWithMRsWithoutIPCBuffer(dest: crate::seL4_CPtr, msg_info: crate::seL4_MessageInfo, msg0: Option<& mut crate::seL4_Word>, msg1: Option<& mut crate::seL4_Word>, msg2: Option<& mut crate::seL4_Word>, msg3: Option<& mut crate::seL4_Word>) -> crate::seL4_MessageInfo
```



## sel4_sys::syscalls::calls::seL4_DebugCapIdentify

*Function*

```rust
fn seL4_DebugCapIdentify(cap: crate::seL4_CPtr) -> crate::seL4_Uint32
```



## sel4_sys::syscalls::calls::seL4_DebugHalt

*Function*

```rust
fn seL4_DebugHalt()
```



## sel4_sys::syscalls::calls::seL4_DebugNameThread

*Function*

```rust
fn seL4_DebugNameThread(tcb: crate::seL4_CPtr, name: &[u8], ipc_buffer: & mut crate::seL4_IPCBuffer)
```



## sel4_sys::syscalls::calls::seL4_DebugPutChar

*Function*

```rust
fn seL4_DebugPutChar(c: u8)
```



## sel4_sys::syscalls::calls::seL4_DebugSnapshot

*Function*

```rust
fn seL4_DebugSnapshot()
```



## sel4_sys::syscalls::calls::seL4_NBSendWithMRsWithoutIPCBuffer

*Function*

```rust
fn seL4_NBSendWithMRsWithoutIPCBuffer(dest: crate::seL4_CPtr, msg_info: crate::seL4_MessageInfo, msg0: Option<crate::seL4_Word>, msg1: Option<crate::seL4_Word>, msg2: Option<crate::seL4_Word>, msg3: Option<crate::seL4_Word>)
```



## sel4_sys::syscalls::calls::seL4_RecvWithMRsWithoutIPCBuffer

*Function*

```rust
fn seL4_RecvWithMRsWithoutIPCBuffer(src: crate::seL4_CPtr, msg0: Option<& mut crate::seL4_Word>, msg1: Option<& mut crate::seL4_Word>, msg2: Option<& mut crate::seL4_Word>, msg3: Option<& mut crate::seL4_Word>, reply_authority: crate::ReplyAuthority) -> (crate::seL4_MessageInfo, crate::seL4_Word)
```



## sel4_sys::syscalls::calls::seL4_ReplyWithMRsWithoutIPCBuffer

*Function*

```rust
fn seL4_ReplyWithMRsWithoutIPCBuffer(msg_info: crate::seL4_MessageInfo, msg0: Option<crate::seL4_Word>, msg1: Option<crate::seL4_Word>, msg2: Option<crate::seL4_Word>, msg3: Option<crate::seL4_Word>)
```



## sel4_sys::syscalls::calls::seL4_SendWithMRsWithoutIPCBuffer

*Function*

```rust
fn seL4_SendWithMRsWithoutIPCBuffer(dest: crate::seL4_CPtr, msg_info: crate::seL4_MessageInfo, msg0: Option<crate::seL4_Word>, msg1: Option<crate::seL4_Word>, msg2: Option<crate::seL4_Word>, msg3: Option<crate::seL4_Word>)
```



## sel4_sys::syscalls::calls::seL4_Yield

*Function*

```rust
fn seL4_Yield()
```



