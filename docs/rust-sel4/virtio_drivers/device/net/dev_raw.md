**virtio_drivers > device > net > dev_raw**

# Module: device::net::dev_raw

## Contents

**Structs**

- [`VirtIONetRaw`](#virtionetraw) - Raw driver for a VirtIO network device.

---

## virtio_drivers::device::net::dev_raw::VirtIONetRaw

*Struct*

Raw driver for a VirtIO network device.

This is a raw version of the VirtIONet driver. It provides non-blocking
methods for transmitting and receiving raw slices, without the buffer
management. For more higher-level functions such as receive buffer backing,
see [`VirtIONet`].

[`VirtIONet`]: super::VirtIONet

**Generic Parameters:**
- H
- T
- const QUEUE_SIZE

**Fields:**
- `transport: T`
- `mac: [u8; 6]`
- `recv_queue: crate::queue::VirtQueue<H, QUEUE_SIZE>`
- `send_queue: crate::queue::VirtQueue<H, QUEUE_SIZE>`
- `legacy_header: bool` - Whether `num_buffers` is missing in the `virtio_net_hdr` struct.

**Methods:**

- `fn new(transport: T) -> Result<Self>` - Create a new VirtIO-Net driver.
- `fn ack_interrupt(self: & mut Self) -> InterruptStatus` - Acknowledge interrupt.
- `fn disable_interrupts(self: & mut Self)` - Disable interrupts.
- `fn enable_interrupts(self: & mut Self)` - Enable interrupts.
- `fn mac_address(self: &Self) -> [u8; 6]` - Get MAC address.
- `fn can_send(self: &Self) -> bool` - Whether can send packet.
- `fn check_rx_buf_len(rx_buf: &[u8]) -> Result<()>` - Whether the length of the receive buffer is valid.
- `fn check_tx_buf_len(self: &Self, tx_buf: &[u8]) -> Result<()>` - Whether the length of the transmit buffer is valid.
- `fn fill_buffer_header(self: &Self, buffer: & mut [u8]) -> Result<usize>` - Fill the header of the `buffer` with [`VirtioNetHdr`].
- `fn transmit_begin(self: & mut Self, tx_buf: &[u8]) -> Result<u16>` - Submits a request to transmit a buffer immediately without waiting for
- `fn poll_transmit(self: & mut Self) -> Option<u16>` - Fetches the token of the next completed transmission request from the
- `fn transmit_complete(self: & mut Self, token: u16, tx_buf: &[u8]) -> Result<usize>` - Completes a transmission operation which was started by [`transmit_begin`].
- `fn receive_begin(self: & mut Self, rx_buf: & mut [u8]) -> Result<u16>` - Submits a request to receive a buffer immediately without waiting for
- `fn poll_receive(self: &Self) -> Option<u16>` - Fetches the token of the next completed reception request from the
- `fn receive_complete(self: & mut Self, token: u16, rx_buf: & mut [u8]) -> Result<(usize, usize)>` - Completes a transmission operation which was started by [`receive_begin`].
- `fn send(self: & mut Self, tx_buf: &[u8]) -> Result` - Sends a packet to the network, and blocks until the request completed.
- `fn receive_wait(self: & mut Self, rx_buf: & mut [u8]) -> Result<(usize, usize)>` - Blocks and waits for a packet to be received.

**Trait Implementations:**

- **Drop**
  - `fn drop(self: & mut Self)`



