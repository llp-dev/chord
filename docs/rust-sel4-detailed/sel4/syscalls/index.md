*[sel4](../index.md) / [syscalls](index.md)*

---

# Module `syscalls`

## Contents

- [Modules](#modules)
  - [`fast_messages_sealing`](#fast-messages-sealing)
- [Structs](#structs)
  - [`RecvWithMRs`](#recvwithmrs)
  - [`CallWithMRs`](#callwithmrs)
- [Traits](#traits)
  - [`IpcCapType`](#ipccaptype)
  - [`FastMessages`](#fastmessages)
- [Functions](#functions)
  - [`wait_message_info_from_sys`](#wait-message-info-from-sys)
  - [`reply`](#reply)
  - [`yield`](#yield)
- [Type Aliases](#type-aliases)
  - [`Badge`](#badge)
  - [`WaitMessageInfo`](#waitmessageinfo)
  - [`ConcreteFastMessagesForIn`](#concretefastmessagesforin)
  - [`ConcreteFastMessagesForInOut`](#concretefastmessagesforinout)
- [Constants](#constants)
  - [`NUM_MESSAGE_REGISTERS`](#num-message-registers)
  - [`UNUSED_FOR_IN`](#unused-for-in)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`fast_messages_sealing`](#fast-messages-sealing) | mod |  |
| [`RecvWithMRs`](#recvwithmrs) | struct | The result of [`cap::Endpoint::recv_with_mrs`]. |
| [`CallWithMRs`](#callwithmrs) | struct | The result of [`cap::Endpoint::call_with_mrs`]. |
| [`IpcCapType`](#ipccaptype) | trait | Trait for [`CapType`]s which are used as targets of IPC syscalls. |
| [`FastMessages`](#fastmessages) | trait | Trait for types which can hold the contents of a set of inline message registers. |
| [`wait_message_info_from_sys`](#wait-message-info-from-sys) | fn |  |
| [`reply`](#reply) | fn | Corresponds to `seL4_Reply`. |
| [`yield`](#yield) | fn | Corresponds to `seL4_Yield`. |
| [`Badge`](#badge) | type | A capability badge. |
| [`WaitMessageInfo`](#waitmessageinfo) | type |  |
| [`ConcreteFastMessagesForIn`](#concretefastmessagesforin) | type |  |
| [`ConcreteFastMessagesForInOut`](#concretefastmessagesforinout) | type |  |
| [`NUM_MESSAGE_REGISTERS`](#num-message-registers) | const | Number of message registers in the IPC buffer. |
| [`UNUSED_FOR_IN`](#unused-for-in) | const |  |

## Modules

- [`fast_messages_sealing`](fast_messages_sealing/index.md)

## Structs

### `RecvWithMRs`

```rust
struct RecvWithMRs {
    pub info: crate::MessageInfo,
    pub badge: Badge,
    pub msg: [crate::Word; 4],
}
```

The result of `cap::Endpoint::recv_with_mrs`.

### `CallWithMRs`

```rust
struct CallWithMRs {
    pub info: crate::MessageInfo,
    pub msg: [crate::Word; 4],
}
```

The result of `cap::Endpoint::call_with_mrs`.

## Traits

### `IpcCapType`

```rust
trait IpcCapType: CapType { ... }
```

Trait for [`CapType`](../cptr/index.md)s which are used as targets of IPC syscalls.

#### Implementors

- [`Endpoint`](../cptr/cap_type/index.md#endpoint)
- [`Notification`](../cptr/cap_type/index.md#notification)
- [`Unspecified`](../cptr/cap_type/index.md#unspecified)

### `FastMessages`

```rust
trait FastMessages: fast_messages_sealing::FastMessagesSealed { ... }
```

Trait for types which can hold the contents of a set of inline message registers.

#### Required Methods

- `fn prepare_in(self) -> [Option<crate::Word>; 4]`

- `fn prepare_in_out(self) -> [crate::Word; 4]`

#### Implementors

- `&[crate::Word]`
- `[crate::Word; N]`

## Functions

### `wait_message_info_from_sys`

```rust
fn wait_message_info_from_sys(info: sys::WaitMessageInfo)
```

### `reply`

```rust
fn reply(ipc_buffer: &mut crate::IpcBuffer, info: crate::MessageInfo)
```

Corresponds to `seL4_Reply`.

### `yield`

```rust
fn yield()
```

Corresponds to `seL4_Yield`.

## Type Aliases

### `Badge`

```rust
type Badge = crate::Word;
```

A capability badge.

### `WaitMessageInfo`

```rust
type WaitMessageInfo = ();
```

### `ConcreteFastMessagesForIn`

```rust
type ConcreteFastMessagesForIn = [Option<crate::Word>; 4];
```

### `ConcreteFastMessagesForInOut`

```rust
type ConcreteFastMessagesForInOut = [crate::Word; 4];
```

## Constants

### `NUM_MESSAGE_REGISTERS`
```rust
const NUM_MESSAGE_REGISTERS: usize = 120usize;
```

Number of message registers in the IPC buffer.

### `UNUSED_FOR_IN`
```rust
const UNUSED_FOR_IN: crate::Word = 0u64;
```

