**virtio_drivers > device > socket**

# Module: device::socket

## Contents

**Modules**

- [`connectionmanager`](#connectionmanager)
- [`error`](#error) - This module contain the error from the VirtIO socket driver.
- [`protocol`](#protocol) - This module defines the socket device protocol according to the virtio spec v1.1 5.10 Socket Device
- [`vsock`](#vsock) - Driver for VirtIO socket devices.

**Constants**

- [`DEFAULT_RX_BUFFER_SIZE`](#default_rx_buffer_size) - The size in bytes of each buffer used in the RX virtqueue. This must be bigger than

---

## virtio_drivers::device::socket::DEFAULT_RX_BUFFER_SIZE

*Constant*: `usize`

The size in bytes of each buffer used in the RX virtqueue. This must be bigger than
`size_of::<VirtioVsockHdr>()`.



## Module: connectionmanager



## Module: error

This module contain the error from the VirtIO socket driver.



## Module: protocol

This module defines the socket device protocol according to the virtio spec v1.1 5.10 Socket Device



## Module: vsock

Driver for VirtIO socket devices.



