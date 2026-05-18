**virtio_drivers > device > net > net_buf**

# Module: device::net::net_buf

## Contents

**Structs**

- [`RxBuffer`](#rxbuffer) - A buffer used for receiving.
- [`TxBuffer`](#txbuffer) - A buffer used for transmitting.

---

## virtio_drivers::device::net::net_buf::RxBuffer

*Struct*

A buffer used for receiving.

**Fields:**
- `buf: alloc::vec::Vec<usize>`
- `packet_len: usize`
- `idx: u16`
- `legacy_header: bool` - Whether `num_buffers` is missing in the `virtio_net_hdr` struct.

**Methods:**

- `fn new(idx: usize, buf_len: usize, legacy_header: bool) -> Self` - Allocates a new buffer with length `buf_len`.
- `fn set_packet_len(self: & mut Self, packet_len: usize)` - Set the network packet length.
- `fn packet_len(self: &Self) -> usize` - Returns the network packet length (without header).
- `fn as_bytes(self: &Self) -> &[u8]` - Returns all data in the buffer, including both the header and the packet.
- `fn as_bytes_mut(self: & mut Self) -> & mut [u8]` - Returns all data in the buffer with the mutable reference,
- `fn header(self: &Self) -> VirtioNetHdr` - Returns a copy of the header.
- `fn packet(self: &Self) -> &[u8]` - Returns the network packet as a slice.
- `fn packet_mut(self: & mut Self) -> & mut [u8]` - Returns the network packet as a mutable slice.



## virtio_drivers::device::net::net_buf::TxBuffer

*Struct*

A buffer used for transmitting.

**Tuple Struct**: `(alloc::vec::Vec<u8>)`

**Methods:**

- `fn from(buf: &[u8]) -> Self` - Constructs the buffer from the given slice.
- `fn packet_len(self: &Self) -> usize` - Returns the network packet length.
- `fn packet(self: &Self) -> &[u8]` - Returns the network packet as a slice.
- `fn packet_mut(self: & mut Self) -> & mut [u8]` - Returns the network packet as a mutable slice.



