**virtio_drivers > device > socket > error**

# Module: device::socket::error

## Contents

**Enums**

- [`SocketError`](#socketerror) - The error type of VirtIO socket driver.

**Type Aliases**

- [`Result`](#result)

---

## virtio_drivers::device::socket::error::Result

*Type Alias*: `result::Result<T, SocketError>`



## virtio_drivers::device::socket::error::SocketError

*Enum*

The error type of VirtIO socket driver.

**Variants:**
- `ConnectionExists` - There is an existing connection.
- `NotConnected` - The device is not connected to any peer.
- `PeerSocketShutdown` - Peer socket is shutdown.
- `BufferTooShort` - The given buffer is shorter than expected.
- `OutputBufferTooShort(usize)` - The given buffer for output is shorter than expected.
- `BufferTooLong(usize, usize)` - The given buffer has exceeded the maximum buffer size.
- `UnknownOperation(u16)` - Unknown operation.
- `InvalidOperation` - Invalid operation,
- `InvalidNumber` - Invalid number.
- `UnexpectedDataInPacket` - Unexpected data in packet.
- `InsufficientBufferSpaceInPeer` - Peer has insufficient buffer space, try again later.
- `RecycledWrongBuffer` - Recycled a wrong buffer.

**Traits:** Copy, Error, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, __formatter: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &SocketError) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> SocketError`



