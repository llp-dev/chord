**sel4_shared_ring_buffer_smoltcp**

# Module: sel4_shared_ring_buffer_smoltcp

## Contents

**Structs**

- [`DeviceImpl`](#deviceimpl)
- [`RxToken`](#rxtoken)
- [`TxToken`](#txtoken)

---

## sel4_shared_ring_buffer_smoltcp::DeviceImpl

*Struct*

**Generic Parameters:**
- A
- R
- P

**Methods:**

- `fn new(raw_mutex: R, dma_region: SharedMemoryRef<'static, [u8]>, bounce_buffer_allocator: A, rx_ring_buffers: RingBuffers<'static, Provide, fn(...)>, tx_ring_buffers: RingBuffers<'static, Provide, fn(...)>, num_rx_buffers: usize, rx_buffer_size: usize, caps: DeviceCapabilities) -> Result<Self, Error>`
- `fn poll(self: &Self) -> bool`
- `fn can_receive(self: &Self) -> bool`
- `fn can_transmit(self: &Self) -> bool`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Device**
  - `fn capabilities(self: &Self) -> DeviceCapabilities`
  - `fn receive(self: & mut Self, _timestamp: Instant) -> Option<(<Self as >::RxToken, <Self as >::TxToken)>`
  - `fn transmit(self: & mut Self, _timestamp: Instant) -> Option<<Self as >::TxToken>`



## sel4_shared_ring_buffer_smoltcp::RxToken

*Struct*

**Generic Parameters:**
- A
- R
- P

**Trait Implementations:**

- **Drop**
  - `fn drop(self: & mut Self)`
- **RxToken**
  - `fn consume<T, F>(self: Self, f: F) -> T`



## sel4_shared_ring_buffer_smoltcp::TxToken

*Struct*

**Generic Parameters:**
- A
- R
- P

**Trait Implementations:**

- **Drop**
  - `fn drop(self: & mut Self)`
- **TxToken**
  - `fn consume<T, F>(self: Self, len: usize, f: F) -> T`



