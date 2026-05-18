**virtio_drivers > device > net > dev**

# Module: device::net::dev

## Contents

**Structs**

- [`VirtIONet`](#virtionet) - Driver for a VirtIO network device.

---

## virtio_drivers::device::net::dev::VirtIONet

*Struct*

Driver for a VirtIO network device.

Unlike [`VirtIONetRaw`], it uses [`RxBuffer`]s for transmission and
reception rather than the raw slices. On initialization, it pre-allocates
all receive buffers and puts them all in the receive queue.

The virtio network device is a virtual ethernet card.

It has enhanced rapidly and demonstrates clearly how support for new
features are added to an existing device.
Empty buffers are placed in one virtqueue for receiving packets, and
outgoing packets are enqueued into another for transmission in that order.
A third command queue is used to control advanced filtering features.

**Generic Parameters:**
- H
- T
- const QUEUE_SIZE

**Fields:**
- `inner: super::VirtIONetRaw<H, T, QUEUE_SIZE>`
- `rx_buffers: [Option<super::net_buf::RxBuffer>; QUEUE_SIZE]`

**Methods:**

- `fn new(transport: T, buf_len: usize) -> Result<Self>` - Create a new VirtIO-Net driver.
- `fn ack_interrupt(self: & mut Self) -> InterruptStatus` - Acknowledge interrupt.
- `fn disable_interrupts(self: & mut Self)` - Disable interrupts.
- `fn enable_interrupts(self: & mut Self)` - Enable interrupts.
- `fn mac_address(self: &Self) -> [u8; 6]` - Get MAC address.
- `fn can_send(self: &Self) -> bool` - Whether can send packet.
- `fn can_recv(self: &Self) -> bool` - Whether can receive packet.
- `fn receive(self: & mut Self) -> Result<RxBuffer>` - Receives a [`RxBuffer`] from network. If currently no data, returns an
- `fn recycle_rx_buffer(self: & mut Self, rx_buf: RxBuffer) -> Result` - Gives back the ownership of `rx_buf`, and recycles it for next use.
- `fn new_tx_buffer(self: &Self, buf_len: usize) -> TxBuffer` - Allocate a new buffer for transmitting.
- `fn send(self: & mut Self, tx_buf: TxBuffer) -> Result` - Sends a [`TxBuffer`] to the network, and blocks until the request



