**virtio_drivers > device > socket > connectionmanager**

# Module: device::socket::connectionmanager

## Contents

**Structs**

- [`Connection`](#connection)
- [`RingBuffer`](#ringbuffer)
- [`VsockConnectionManager`](#vsockconnectionmanager) - A higher level interface for VirtIO socket (vsock) devices.

**Functions**

- [`get_connection`](#get_connection) - Returns the connection from the given list matching the given peer address and local port, and
- [`get_connection_for_event`](#get_connection_for_event) - Returns the connection from the given list matching the event, if any, and its index.

**Constants**

- [`DEFAULT_PER_CONNECTION_BUFFER_CAPACITY`](#default_per_connection_buffer_capacity)

---

## virtio_drivers::device::socket::connectionmanager::Connection

*Struct*

**Fields:**
- `info: super::vsock::ConnectionInfo`
- `buffer: RingBuffer`
- `peer_requested_shutdown: bool` - The peer sent a SHUTDOWN request, but we haven't yet responded with a RST because there is

**Methods:**

- `fn new(peer: VsockAddr, local_port: u32, buffer_capacity: u32) -> Self`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## virtio_drivers::device::socket::connectionmanager::DEFAULT_PER_CONNECTION_BUFFER_CAPACITY

*Constant*: `u32`



## virtio_drivers::device::socket::connectionmanager::RingBuffer

*Struct*

**Fields:**
- `buffer: alloc::boxed::Box<[u8]>`
- `used: usize` - The number of bytes currently in the buffer.
- `start: usize` - The index of the first used byte in the buffer.

**Methods:**

- `fn new(capacity: usize) -> Self`
- `fn used(self: &Self) -> usize` - Returns the number of bytes currently used in the buffer.
- `fn is_empty(self: &Self) -> bool` - Returns true iff there are currently no bytes in the buffer.
- `fn free(self: &Self) -> usize` - Returns the number of bytes currently free in the buffer.
- `fn add(self: & mut Self, bytes: &[u8]) -> bool` - Adds the given bytes to the buffer if there is enough capacity for them all.
- `fn drain(self: & mut Self, out: & mut [u8]) -> usize` - Reads and removes as many bytes as possible from the buffer, up to the length of the given

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## virtio_drivers::device::socket::connectionmanager::VsockConnectionManager

*Struct*

A higher level interface for VirtIO socket (vsock) devices.

This keeps track of multiple vsock connections.

`RX_BUFFER_SIZE` is the size in bytes of each buffer used in the RX virtqueue. This must be
bigger than `size_of::<VirtioVsockHdr>()`.

# Example

```
# use virtio_drivers::{Error, Hal};
# use virtio_drivers::transport::Transport;
use virtio_drivers::device::socket::{VirtIOSocket, VsockAddr, VsockConnectionManager};

# fn example<HalImpl: Hal, T: Transport>(transport: T) -> Result<(), Error> {
let mut socket = VsockConnectionManager::new(VirtIOSocket::<HalImpl, _>::new(transport)?);

// Start a thread to call `socket.poll()` and handle events.

let remote_address = VsockAddr { cid: 2, port: 42 };
let local_port = 1234;
socket.connect(remote_address, local_port)?;

// Wait until `socket.poll()` returns an event indicating that the socket is connected.

socket.send(remote_address, local_port, "Hello world".as_bytes())?;

socket.shutdown(remote_address, local_port)?;
# Ok(())
# }
```

**Generic Parameters:**
- H
- T
- const RX_BUFFER_SIZE

**Fields:**
- `driver: super::VirtIOSocket<H, T, RX_BUFFER_SIZE>`
- `per_connection_buffer_capacity: u32`
- `connections: alloc::vec::Vec<Connection>`
- `listening_ports: alloc::vec::Vec<u32>`

**Methods:**

- `fn new(driver: VirtIOSocket<H, T, RX_BUFFER_SIZE>) -> Self` - Construct a new connection manager wrapping the given low-level VirtIO socket driver.
- `fn new_with_capacity(driver: VirtIOSocket<H, T, RX_BUFFER_SIZE>, per_connection_buffer_capacity: u32) -> Self` - Construct a new connection manager wrapping the given low-level VirtIO socket driver, with
- `fn guest_cid(self: &Self) -> u64` - Returns the CID which has been assigned to this guest.
- `fn listen(self: & mut Self, port: u32)` - Allows incoming connections on the given port number.
- `fn unlisten(self: & mut Self, port: u32)` - Stops allowing incoming connections on the given port number.
- `fn connect(self: & mut Self, destination: VsockAddr, src_port: u32) -> Result` - Sends a request to connect to the given destination.
- `fn send(self: & mut Self, destination: VsockAddr, src_port: u32, buffer: &[u8]) -> Result` - Sends the buffer to the destination.
- `fn poll(self: & mut Self) -> Result<Option<VsockEvent>>` - Polls the vsock device to receive data or other updates.
- `fn recv(self: & mut Self, peer: VsockAddr, src_port: u32, buffer: & mut [u8]) -> Result<usize>` - Reads data received from the given connection.
- `fn recv_buffer_available_bytes(self: & mut Self, peer: VsockAddr, src_port: u32) -> Result<usize>` - Returns the number of bytes in the receive buffer available to be read by `recv`.
- `fn update_credit(self: & mut Self, peer: VsockAddr, src_port: u32) -> Result` - Sends a credit update to the given peer.
- `fn wait_for_event(self: & mut Self) -> Result<VsockEvent>` - Blocks until we get some event from the vsock device.
- `fn shutdown(self: & mut Self, destination: VsockAddr, src_port: u32) -> Result` - Requests to shut down the connection cleanly, telling the peer that we won't send or receive
- `fn force_close(self: & mut Self, destination: VsockAddr, src_port: u32) -> Result` - Forcibly closes the connection without waiting for the peer.



## virtio_drivers::device::socket::connectionmanager::get_connection

*Function*

Returns the connection from the given list matching the given peer address and local port, and
its index.

Returns `Err(SocketError::NotConnected)` if there is no matching connection in the list.

```rust
fn get_connection(connections: & mut [Connection], peer: super::protocol::VsockAddr, local_port: u32) -> core::result::Result<(usize, & mut Connection), super::SocketError>
```



## virtio_drivers::device::socket::connectionmanager::get_connection_for_event

*Function*

Returns the connection from the given list matching the event, if any, and its index.

```rust
fn get_connection_for_event<'a>(connections: &'a  mut [Connection], event: &super::VsockEvent, local_cid: u64) -> Option<(usize, &'a  mut Connection)>
```



