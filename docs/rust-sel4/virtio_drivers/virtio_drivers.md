**virtio_drivers**

# Module: virtio_drivers

## Contents

**Modules**

- [`config`](#config) - Types and macros for VirtIO device configuration space.
- [`device`](#device) - Drivers for specific VirtIO devices.
- [`hal`](#hal)
- [`queue`](#queue) - Support for virt queues, the main mechanism for data transport on VirtIO devices.
- [`transport`](#transport) - VirtIO transports.

**Macros**

- [`read_config`](#read_config) - Reads the given field of the given struct from the device config space via the given transport.
- [`write_config`](#write_config) - Writes the given field of the given struct from the device config space via the given transport.

**Enums**

- [`Error`](#error) - The error type of VirtIO drivers.

**Functions**

- [`align_up`](#align_up) - Align `size` up to a page.
- [`align_up_phys`](#align_up_phys) - Align `size` up to a page.
- [`pages`](#pages) - The number of pages required to store `size` bytes, rounded up to a whole number of pages.

**Constants**

- [`PAGE_SIZE`](#page_size) - The page size in bytes supported by the library (4 KiB).
- [`PAGE_SIZE_PHYS`](#page_size_phys)

**Type Aliases**

- [`Result`](#result) - The type returned by driver methods.

---

## virtio_drivers::Error

*Enum*

The error type of VirtIO drivers.

**Variants:**
- `QueueFull` - There are not enough descriptors available in the virtqueue, try again later.
- `NotReady` - The device is not ready.
- `WrongToken` - The device used a different descriptor chain to the one we were expecting.
- `AlreadyUsed` - The queue is already in use.
- `InvalidParam` - Invalid parameter.
- `DmaError` - Failed to allocate DMA memory.
- `IoError` - I/O error
- `Unsupported` - The request was not supported by the device.
- `ConfigSpaceTooSmall` - The config space advertised by the device is smaller than the driver expected.
- `ConfigSpaceMissing` - The device doesn't have any config space, but the driver expects some.
- `SocketDeviceError(device::socket::SocketError)` - Error from the socket device.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Error**
  - `fn source(self: &Self) -> ::core::option::Option<&dyn ::thiserror::__private18::Error>`
- **Display**
  - `fn fmt(self: &Self, __formatter: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Error) -> bool`
- **From**
  - `fn from(_value: alloc::string::FromUtf8Error) -> Self`
- **From**
  - `fn from(source: SocketError) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Error`



## virtio_drivers::PAGE_SIZE

*Constant*: `usize`

The page size in bytes supported by the library (4 KiB).



## virtio_drivers::PAGE_SIZE_PHYS

*Constant*: `PhysAddr`



## virtio_drivers::Result

*Type Alias*: `core::result::Result<T, Error>`

The type returned by driver methods.



## virtio_drivers::align_up

*Function*

Align `size` up to a page.

```rust
fn align_up(size: usize) -> usize
```



## virtio_drivers::align_up_phys

*Function*

Align `size` up to a page.

```rust
fn align_up_phys(size: PhysAddr) -> PhysAddr
```



## Module: config

Types and macros for VirtIO device configuration space.



## Module: device

Drivers for specific VirtIO devices.



## Module: hal



## virtio_drivers::pages

*Function*

The number of pages required to store `size` bytes, rounded up to a whole number of pages.

```rust
fn pages(size: usize) -> usize
```



## Module: queue

Support for virt queues, the main mechanism for data transport on VirtIO devices.

Types from this module are used to implement VirtIO device drivers. If you just want to use the
drivers provided (rather than implementing drivers for other devices) then you shouldn't need to
use anything from this module.



## virtio_drivers::read_config

*Declarative Macro*

Reads the given field of the given struct from the device config space via the given transport.

```rust
macro_rules! read_config {
    ($transport:expr, $struct:ty, $field:ident) => { ... };
}
```



## Module: transport

VirtIO transports.



## virtio_drivers::write_config

*Declarative Macro*

Writes the given field of the given struct from the device config space via the given transport.

```rust
macro_rules! write_config {
    ($transport:expr, $struct:ty, $field:ident, $value:expr) => { ... };
}
```



