**virtio_drivers > device > socket > vsock**

# Module: device::socket::vsock

## Contents

**Structs**

- [`ConnectionInfo`](#connectioninfo) - Information about a particular vsock connection.
- [`VirtIOSocket`](#virtiosocket) - Low-level driver for a VirtIO socket device.
- [`VsockBufferStatus`](#vsockbufferstatus)
- [`VsockEvent`](#vsockevent) - An event received from a VirtIO socket device.

**Enums**

- [`DisconnectReason`](#disconnectreason) - The reason why a vsock connection was closed.
- [`VsockEventType`](#vsockeventtype) - Details of the type of an event received from a VirtIO socket.

**Functions**

- [`read_header_and_body`](#read_header_and_body)

**Constants**

- [`EVENT_QUEUE_IDX`](#event_queue_idx)
- [`QUEUE_SIZE`](#queue_size)
- [`RX_QUEUE_IDX`](#rx_queue_idx)
- [`SUPPORTED_FEATURES`](#supported_features)
- [`TX_QUEUE_IDX`](#tx_queue_idx)

---

## virtio_drivers::device::socket::vsock::ConnectionInfo

*Struct*

Information about a particular vsock connection.

**Fields:**
- `dst: super::protocol::VsockAddr` - The address of the peer.
- `src_port: u32` - The local port number associated with the connection.
- `peer_buf_alloc: u32` - The last `buf_alloc` value the peer sent to us, indicating how much receive buffer space in
- `peer_fwd_cnt: u32` - The last `fwd_cnt` value the peer sent to us, indicating how many bytes of packet bodies it
- `tx_cnt: u32` - The number of bytes of packet bodies which we have sent to the peer.
- `buf_alloc: u32` - The number of bytes of buffer space we have allocated to receive packet bodies from the
- `fwd_cnt: u32` - The number of bytes of packet bodies which we have received from the peer and handled.
- `has_pending_credit_request: bool` - Whether we have recently requested credit from the peer.

**Methods:**

- `fn new(destination: VsockAddr, src_port: u32) -> Self` - Creates a new `ConnectionInfo` for the given peer address and local port, and default values
- `fn update_for_event(self: & mut Self, event: &VsockEvent)` - Updates this connection info with the peer buffer allocation and forwarded count from the
- `fn done_forwarding(self: & mut Self, length: usize)` - Increases the forwarded count recorded for this connection by the given number of bytes.
- `fn peer_free(self: &Self) -> u32` - Returns the number of bytes of RX buffer space the peer has available to receive packet body
- `fn new_header(self: &Self, src_cid: u64) -> VirtioVsockHdr`

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> ConnectionInfo`
- **PartialEq**
  - `fn eq(self: &Self, other: &ConnectionInfo) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> ConnectionInfo`



## virtio_drivers::device::socket::vsock::DisconnectReason

*Enum*

The reason why a vsock connection was closed.

**Variants:**
- `Reset` - The peer has either closed the connection in response to our shutdown request, or forcibly
- `Shutdown` - The peer asked to shut down the connection.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> DisconnectReason`
- **PartialEq**
  - `fn eq(self: &Self, other: &DisconnectReason) -> bool`



## virtio_drivers::device::socket::vsock::EVENT_QUEUE_IDX

*Constant*: `u16`



## virtio_drivers::device::socket::vsock::QUEUE_SIZE

*Constant*: `usize`



## virtio_drivers::device::socket::vsock::RX_QUEUE_IDX

*Constant*: `u16`



## virtio_drivers::device::socket::vsock::SUPPORTED_FEATURES

*Constant*: `super::protocol::Feature`



## virtio_drivers::device::socket::vsock::TX_QUEUE_IDX

*Constant*: `u16`



## virtio_drivers::device::socket::vsock::VirtIOSocket

*Struct*

Low-level driver for a VirtIO socket device.

You probably want to use [`VsockConnectionManager`](super::VsockConnectionManager) rather than
using this directly.

`RX_BUFFER_SIZE` is the size in bytes of each buffer used in the RX virtqueue. This must be
bigger than `size_of::<VirtioVsockHdr>()`.

**Generic Parameters:**
- H
- T
- const RX_BUFFER_SIZE

**Fields:**
- `transport: T`
- `rx: crate::queue::OwningQueue<H, QUEUE_SIZE, RX_BUFFER_SIZE>` - Virtqueue to receive packets.
- `tx: crate::queue::VirtQueue<H, { QUEUE_SIZE }>`
- `event: crate::queue::VirtQueue<H, { QUEUE_SIZE }>` - Virtqueue to receive events from the device.
- `guest_cid: u64` - The guest_cid field contains the guest’s context ID, which uniquely identifies

**Methods:**

- `fn new(transport: T) -> Result<Self>` - Create a new VirtIO Vsock driver.
- `fn guest_cid(self: &Self) -> u64` - Returns the CID which has been assigned to this guest.
- `fn connect(self: & mut Self, connection_info: &ConnectionInfo) -> Result` - Sends a request to connect to the given destination.
- `fn accept(self: & mut Self, connection_info: &ConnectionInfo) -> Result` - Accepts the given connection from a peer.
- `fn request_credit(self: & mut Self, connection_info: &ConnectionInfo) -> Result` - Requests the peer to send us a credit update for the given connection.
- `fn send(self: & mut Self, buffer: &[u8], connection_info: & mut ConnectionInfo) -> Result` - Sends the buffer to the destination.
- `fn check_peer_buffer_is_sufficient(self: & mut Self, connection_info: & mut ConnectionInfo, buffer_len: usize) -> Result`
- `fn credit_update(self: & mut Self, connection_info: &ConnectionInfo) -> Result` - Tells the peer how much buffer space we have to receive data.
- `fn poll<impl FnOnce(VsockEvent, &[u8]) -> Result<Option<VsockEvent>>>(self: & mut Self, handler: impl Trait) -> Result<Option<VsockEvent>>` - Polls the RX virtqueue for the next event, and calls the given handler function to handle
- `fn shutdown_with_hints(self: & mut Self, connection_info: &ConnectionInfo, hints: StreamShutdown) -> Result` - Requests to shut down the connection cleanly, sending hints about whether we will send or
- `fn shutdown(self: & mut Self, connection_info: &ConnectionInfo) -> Result` - Requests to shut down the connection cleanly, telling the peer that we won't send or receive
- `fn force_close(self: & mut Self, connection_info: &ConnectionInfo) -> Result` - Forcibly closes the connection without waiting for the peer.
- `fn send_packet_to_tx_queue(self: & mut Self, header: &VirtioVsockHdr, buffer: &[u8]) -> Result`

**Trait Implementations:**

- **Drop**
  - `fn drop(self: & mut Self)`



## virtio_drivers::device::socket::vsock::VsockBufferStatus

*Struct*

**Fields:**
- `buffer_allocation: u32`
- `forward_count: u32`

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> VsockBufferStatus`
- **PartialEq**
  - `fn eq(self: &Self, other: &VsockBufferStatus) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## virtio_drivers::device::socket::vsock::VsockEvent

*Struct*

An event received from a VirtIO socket device.

**Fields:**
- `source: super::protocol::VsockAddr` - The source of the event, i.e. the peer who sent it.
- `destination: super::protocol::VsockAddr` - The destination of the event, i.e. the CID and port on our side.
- `buffer_status: VsockBufferStatus` - The peer's buffer status for the connection.
- `event_type: VsockEventType` - The type of event.

**Methods:**

- `fn matches_connection(self: &Self, connection_info: &ConnectionInfo, guest_cid: u64) -> bool` - Returns whether the event matches the given connection.
- `fn from_header(header: &VirtioVsockHdr) -> Result<Self>`

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> VsockEvent`
- **PartialEq**
  - `fn eq(self: &Self, other: &VsockEvent) -> bool`



## virtio_drivers::device::socket::vsock::VsockEventType

*Enum*

Details of the type of an event received from a VirtIO socket.

**Variants:**
- `ConnectionRequest` - The peer requests to establish a connection with us.
- `Connected` - The connection was successfully established.
- `Disconnected{ reason: DisconnectReason }` - The connection was closed.
- `Received{ length: usize }` - Data was received on the connection.
- `CreditRequest` - The peer requests us to send a credit update.
- `CreditUpdate` - The peer just sent us a credit update with nothing else.

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> VsockEventType`
- **PartialEq**
  - `fn eq(self: &Self, other: &VsockEventType) -> bool`



## virtio_drivers::device::socket::vsock::read_header_and_body

*Function*

```rust
fn read_header_and_body(buffer: &[u8]) -> crate::Result<(super::protocol::VirtioVsockHdr, &[u8])>
```



