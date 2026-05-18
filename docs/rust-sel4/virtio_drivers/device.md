**virtio_drivers > device**

# Module: device

## Contents

**Modules**

- [`blk`](#blk) - Driver for VirtIO block devices.
- [`common`](#common) - Common part shared across all the devices.
- [`console`](#console) - Driver for VirtIO console devices.
- [`gpu`](#gpu) - Driver for VirtIO GPU devices.
- [`input`](#input) - Driver for VirtIO input devices.
- [`net`](#net) - Driver for VirtIO network devices.
- [`rng`](#rng) - Driver for VirtIO random number generator devices.
- [`socket`](#socket) - Driver for VirtIO socket devices.
- [`sound`](#sound) - Driver for VirtIO Sound devices.
- [`virtio_9p`](#virtio_9p) - Driver for VirtIO 9p devices.

---

## Module: blk

Driver for VirtIO block devices.



## Module: common

Common part shared across all the devices.



## Module: console

Driver for VirtIO console devices.



## Module: gpu

Driver for VirtIO GPU devices.



## Module: input

Driver for VirtIO input devices.



## Module: net

Driver for VirtIO network devices.



## Module: rng

Driver for VirtIO random number generator devices.



## Module: socket

Driver for VirtIO socket devices.

To use the driver, you should first create a [`VirtIOSocket`] instance with your VirtIO
transport, and then create a [`VsockConnectionManager`] wrapping it to keep track of
connections. If you want to manage connections yourself you can use the `VirtIOSocket` directly
for a lower-level interface.

See [`VsockConnectionManager`] for a usage example.



## Module: sound

Driver for VirtIO Sound devices.



## Module: virtio_9p

Driver for VirtIO 9p devices.



