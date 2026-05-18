**smoltcp > storage > packet_buffer**

# Module: storage::packet_buffer

## Contents

**Structs**

- [`PacketBuffer`](#packetbuffer) - An UDP packet ring buffer.
- [`PacketMetadata`](#packetmetadata) - Size and header of a packet.

---

## smoltcp::storage::packet_buffer::PacketBuffer

*Struct*

An UDP packet ring buffer.

**Generic Parameters:**
- 'a
- H

**Methods:**

- `fn new<MS, PS>(metadata_storage: MS, payload_storage: PS) -> PacketBuffer<'a, H>` - Create a new packet buffer with the provided metadata and payload storage.
- `fn is_empty(self: &Self) -> bool` - Query whether the buffer is empty.
- `fn is_full(self: &Self) -> bool` - Query whether the buffer is full.
- `fn enqueue(self: & mut Self, size: usize, header: H) -> Result<& mut [u8], Full>` - Enqueue a single packet with the given header into the buffer, and
- `fn enqueue_with_infallible<'b, F>(self: &'b  mut Self, max_size: usize, header: H, f: F) -> Result<usize, Full>` - Call `f` with a packet from the buffer large enough to fit `max_size` bytes. The packet
- `fn dequeue_with<'c, R, E, F>(self: &'c  mut Self, f: F) -> Result<Result<R, E>, Empty>` - Call `f` with a single packet from the buffer, and dequeue the packet if `f`
- `fn dequeue(self: & mut Self) -> Result<(H, & mut [u8]), Empty>` - Dequeue a single packet from the buffer, and return a reference to its payload
- `fn peek(self: & mut Self) -> Result<(&H, &[u8]), Empty>` - Peek at a single packet from the buffer without removing it, and return a reference to
- `fn packet_capacity(self: &Self) -> usize` - Return the maximum number packets that can be stored.
- `fn payload_capacity(self: &Self) -> usize` - Return the maximum number of bytes in the payload ring buffer.
- `fn payload_bytes_count(self: &Self) -> usize` - Return the current number of bytes in the payload ring buffer.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## smoltcp::storage::packet_buffer::PacketMetadata

*Struct*

Size and header of a packet.

**Generic Parameters:**
- H

**Methods:**


**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> PacketMetadata<H>`



