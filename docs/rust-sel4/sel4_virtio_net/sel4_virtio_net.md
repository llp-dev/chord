**sel4_virtio_net**

# Module: sel4_virtio_net

## Contents

**Structs**

- [`DeviceWrapper`](#devicewrapper)
- [`VirtioRxToken`](#virtiorxtoken)
- [`VirtioTxToken`](#virtiotxtoken)

**Constants**

- [`NET_QUEUE_SIZE`](#net_queue_size)

**Type Aliases**

- [`DeviceImpl`](#deviceimpl)

---

## sel4_virtio_net::DeviceImpl

*Type Alias*: `virtio_drivers::device::net::VirtIONet<H, T, NET_QUEUE_SIZE>`



## sel4_virtio_net::DeviceWrapper

*Struct*

**Generic Parameters:**
- H
- T

**Methods:**

- `fn new(dev: DeviceImpl<H, T>) -> Self`

**Trait Implementations:**

- **Device**
  - `fn receive(self: & mut Self, _timestamp: Instant) -> Option<(<Self as >::RxToken, <Self as >::TxToken)>`
  - `fn transmit(self: & mut Self, _timestamp: Instant) -> Option<<Self as >::TxToken>`
  - `fn capabilities(self: &Self) -> DeviceCapabilities`
- **HandleInterrupt**
  - `fn handle_interrupt(self: & mut Self)`
- **GetNetDeviceMeta**
  - `fn get_mac_address(self: & mut Self) -> Result<MacAddress, <Self as >::Error>`



## sel4_virtio_net::NET_QUEUE_SIZE

*Constant*: `usize`



## sel4_virtio_net::VirtioRxToken

*Struct*

**Generic Parameters:**
- H
- T

**Tuple Struct**: `()`

**Trait Implementations:**

- **RxToken**
  - `fn consume<R, F>(self: Self, f: F) -> R`



## sel4_virtio_net::VirtioTxToken

*Struct*

**Generic Parameters:**
- H
- T

**Tuple Struct**: `()`

**Trait Implementations:**

- **TxToken**
  - `fn consume<R, F>(self: Self, len: usize, f: F) -> R`



