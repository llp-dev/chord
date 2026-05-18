*[sel4](../index.md) / [ipc_buffer](index.md)*

---

# Module `ipc_buffer`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`IpcBuffer`](#ipcbuffer) | struct | Corresponds to `seL4_IPCBuffer`. |

## Structs

### `IpcBuffer`

```rust
struct IpcBuffer(sys::seL4_IPCBuffer);
```

Corresponds to `seL4_IPCBuffer`.

#### Implementations

- <span id="ipcbuffer-from-inner"></span>`const fn from_inner(inner: sys::seL4_IPCBuffer) -> Self`

- <span id="ipcbuffer-into-inner"></span>`const fn into_inner(self) -> sys::seL4_IPCBuffer`

- <span id="ipcbuffer-inner"></span>`const fn inner(&self) -> &sys::seL4_IPCBuffer`

- <span id="ipcbuffer-inner-mut"></span>`fn inner_mut(&mut self) -> &mut sys::seL4_IPCBuffer`

- <span id="ipcbuffer-msg-regs"></span>`fn msg_regs(&self) -> &[Word]` — [`Word`](../index.md#word)

- <span id="ipcbuffer-msg-regs-mut"></span>`fn msg_regs_mut(&mut self) -> &mut [Word]` — [`Word`](../index.md#word)

- <span id="ipcbuffer-msg-bytes"></span>`fn msg_bytes(&self) -> &[u8]`

- <span id="ipcbuffer-msg-bytes-mut"></span>`fn msg_bytes_mut(&mut self) -> &mut [u8]`

- <span id="ipcbuffer-user-data"></span>`fn user_data(&self) -> Word` — [`Word`](../index.md#word)

- <span id="ipcbuffer-set-user-data"></span>`fn set_user_data(&mut self, data: Word)` — [`Word`](../index.md#word)

- <span id="ipcbuffer-caps-or-badges"></span>`fn caps_or_badges(&self) -> &[Word]` — [`Word`](../index.md#word)

- <span id="ipcbuffer-caps-or-badges-mut"></span>`fn caps_or_badges_mut(&mut self) -> &mut [Word]` — [`Word`](../index.md#word)

- <span id="ipcbuffer-recv-slot"></span>`fn recv_slot(&self) -> AbsoluteCPtr` — [`AbsoluteCPtr`](../cptr/index.md#absolutecptr)

- <span id="ipcbuffer-set-recv-slot"></span>`fn set_recv_slot(&mut self, slot: &AbsoluteCPtr)` — [`AbsoluteCPtr`](../cptr/index.md#absolutecptr)

#### Trait Implementations

##### `impl Default for IpcBuffer`

- <span id="ipcbuffer-default"></span>`fn default() -> IpcBuffer` — [`IpcBuffer`](#ipcbuffer)

##### `impl InvocationContext for &mut crate::IpcBuffer`

- <span id="mut-crateipcbuffer-invocationcontext-with-context"></span>`fn with_context<T>(&mut self, f: impl FnOnce(&mut IpcBuffer) -> T) -> T` — [`IpcBuffer`](#ipcbuffer)

