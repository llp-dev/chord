**sel4 > ipc_buffer**

# Module: ipc_buffer

## Contents

**Structs**

- [`IpcBuffer`](#ipcbuffer) - Corresponds to `seL4_IPCBuffer`.

---

## sel4::ipc_buffer::IpcBuffer

*Struct*

Corresponds to `seL4_IPCBuffer`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_inner(inner: sys::seL4_IPCBuffer) -> Self`
- `fn into_inner(self: Self) -> sys::seL4_IPCBuffer`
- `fn inner(self: &Self) -> &sys::seL4_IPCBuffer`
- `fn inner_mut(self: & mut Self) -> & mut sys::seL4_IPCBuffer`
- `fn msg_regs(self: &Self) -> &[Word]`
- `fn msg_regs_mut(self: & mut Self) -> & mut [Word]`
- `fn msg_bytes(self: &Self) -> &[u8]`
- `fn msg_bytes_mut(self: & mut Self) -> & mut [u8]`
- `fn user_data(self: &Self) -> Word`
- `fn set_user_data(self: & mut Self, data: Word)`
- `fn caps_or_badges(self: &Self) -> &[Word]`
- `fn caps_or_badges_mut(self: & mut Self) -> & mut [Word]`
- `fn recv_slot(self: &Self) -> AbsoluteCPtr`
- `fn set_recv_slot(self: & mut Self, slot: &AbsoluteCPtr)`

**Trait Implementations:**

- **Default**
  - `fn default() -> IpcBuffer`



