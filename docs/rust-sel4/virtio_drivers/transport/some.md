**virtio_drivers > transport > some**

# Module: transport::some

## Contents

**Enums**

- [`SomeTransport`](#sometransport) - A wrapper for an arbitrary VirtIO transport, either MMIO or PCI.

---

## virtio_drivers::transport::some::SomeTransport

*Enum*

A wrapper for an arbitrary VirtIO transport, either MMIO or PCI.

**Generic Parameters:**
- 'a

**Variants:**
- `Mmio(super::mmio::MmioTransport<'a>)` - An MMIO transport.
- `Pci(super::pci::PciTransport)` - A PCI transport.
- `HypPci(super::x86_64::HypPciTransport)` - An x86-64 pKVM PCI transport.

**Trait Implementations:**

- **Transport**
  - `fn device_type(self: &Self) -> DeviceType`
  - `fn read_device_features(self: & mut Self) -> u64`
  - `fn write_driver_features(self: & mut Self, driver_features: u64)`
  - `fn max_queue_size(self: & mut Self, queue: u16) -> u32`
  - `fn notify(self: & mut Self, queue: u16)`
  - `fn get_status(self: &Self) -> DeviceStatus`
  - `fn set_status(self: & mut Self, status: DeviceStatus)`
  - `fn set_guest_page_size(self: & mut Self, guest_page_size: u32)`
  - `fn requires_legacy_layout(self: &Self) -> bool`
  - `fn queue_set(self: & mut Self, queue: u16, size: u32, descriptors: PhysAddr, driver_area: PhysAddr, device_area: PhysAddr)`
  - `fn queue_unset(self: & mut Self, queue: u16)`
  - `fn queue_used(self: & mut Self, queue: u16) -> bool`
  - `fn ack_interrupt(self: & mut Self) -> InterruptStatus`
  - `fn read_config_generation(self: &Self) -> u32`
  - `fn read_config_space<T>(self: &Self, offset: usize) -> Result<T>`
  - `fn write_config_space<T>(self: & mut Self, offset: usize, value: T) -> Result<()>`
- **From**
  - `fn from(pci: PciTransport) -> Self`
- **From**
  - `fn from(mmio: MmioTransport<'a>) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



