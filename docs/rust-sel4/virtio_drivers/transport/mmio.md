**virtio_drivers > transport > mmio**

# Module: transport::mmio

## Contents

**Structs**

- [`MmioTransport`](#mmiotransport) - MMIO Device Register Interface.
- [`VirtIOHeader`](#virtioheader) - MMIO Device Register Interface, both legacy and modern.

**Enums**

- [`MmioError`](#mmioerror) - An error encountered initialising a VirtIO MMIO transport.
- [`MmioVersion`](#mmioversion) - The version of the VirtIO MMIO transport supported by a device.

**Constants**

- [`CONFIG_SPACE_OFFSET`](#config_space_offset)
- [`LEGACY_VERSION`](#legacy_version)
- [`MAGIC_VALUE`](#magic_value)
- [`MODERN_VERSION`](#modern_version)

---

## virtio_drivers::transport::mmio::CONFIG_SPACE_OFFSET

*Constant*: `usize`



## virtio_drivers::transport::mmio::LEGACY_VERSION

*Constant*: `u32`



## virtio_drivers::transport::mmio::MAGIC_VALUE

*Constant*: `u32`



## virtio_drivers::transport::mmio::MODERN_VERSION

*Constant*: `u32`



## virtio_drivers::transport::mmio::MmioError

*Enum*

An error encountered initialising a VirtIO MMIO transport.

**Variants:**
- `BadMagic(u32)` - The header doesn't start with the expected magic value 0x74726976.
- `UnsupportedVersion(u32)` - The header reports a version number that is neither 1 (legacy) nor 2 (modern).
- `InvalidDeviceID(super::DeviceTypeError)` - The header reports a device ID of 0.
- `MmioRegionTooSmall` - The MMIO region size was smaller than the header size we expect.

**Traits:** Eq, Error

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> MmioError`
- **Display**
  - `fn fmt(self: &Self, __formatter: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &MmioError) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## virtio_drivers::transport::mmio::MmioTransport

*Struct*

MMIO Device Register Interface.

Ref: 4.2.2 MMIO Device Register Layout and 4.2.4 Legacy interface

**Generic Parameters:**
- 'a

**Fields:**
- `header: safe_mmio::UniqueMmioPointer<'a, VirtIOHeader>`
- `config_space: safe_mmio::UniqueMmioPointer<'a, [u8]>`
- `version: MmioVersion`
- `device_type: super::DeviceType`

**Methods:**

- `fn new(header: NonNull<VirtIOHeader>, mmio_size: usize) -> Result<Self, MmioError>` - Constructs a new VirtIO MMIO transport, or returns an error if the header reports an
- `fn new_from_unique(header: UniqueMmioPointer<'a, VirtIOHeader>, config_space: UniqueMmioPointer<'a, [u8]>) -> Result<Self, MmioError>` - Constructs a new VirtIO MMIO transport, or returns an error if the header reports an
- `fn version(self: &Self) -> MmioVersion` - Gets the version of the VirtIO MMIO transport.
- `fn vendor_id(self: &Self) -> u32` - Gets the vendor ID.

**Traits:** Sync

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
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
  - `fn read_config_space<T>(self: &Self, offset: usize) -> Result<T, Error>`
  - `fn write_config_space<T>(self: & mut Self, offset: usize, value: T) -> Result<(), Error>`
- **Drop**
  - `fn drop(self: & mut Self)`



## virtio_drivers::transport::mmio::MmioVersion

*Enum*

The version of the VirtIO MMIO transport supported by a device.

**Variants:**
- `Legacy` - Legacy MMIO transport with page-based addressing.
- `Modern` - Modern MMIO transport.

**Traits:** Copy, Eq

**Trait Implementations:**

- **TryFrom**
  - `fn try_from(version: u32) -> Result<Self, <Self as >::Error>`
- **Clone**
  - `fn clone(self: &Self) -> MmioVersion`
- **PartialEq**
  - `fn eq(self: &Self, other: &MmioVersion) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## virtio_drivers::transport::mmio::VirtIOHeader

*Struct*

MMIO Device Register Interface, both legacy and modern.

Ref: 4.2.2 MMIO Device Register Layout and 4.2.4 Legacy interface

**Fields:**
- `magic: safe_mmio::fields::ReadPure<u32>` - Magic value
- `version: safe_mmio::fields::ReadPure<u32>` - Device version number
- `device_id: safe_mmio::fields::ReadPure<u32>` - Virtio Subsystem Device ID
- `vendor_id: safe_mmio::fields::ReadPure<u32>` - Virtio Subsystem Vendor ID
- `device_features: safe_mmio::fields::ReadPure<u32>` - Flags representing features the device supports
- `device_features_sel: safe_mmio::fields::WriteOnly<u32>` - Device (host) features word selection
- `__r1: [u32; 2]` - Reserved
- `driver_features: safe_mmio::fields::WriteOnly<u32>` - Flags representing device features understood and activated by the driver
- `driver_features_sel: safe_mmio::fields::WriteOnly<u32>` - Activated (guest) features word selection
- `legacy_guest_page_size: safe_mmio::fields::WriteOnly<u32>` - Guest page size
- `__r2: u32` - Reserved
- `queue_sel: safe_mmio::fields::WriteOnly<u32>` - Virtual queue index
- `queue_num_max: safe_mmio::fields::ReadPure<u32>` - Maximum virtual queue size
- `queue_num: safe_mmio::fields::WriteOnly<u32>` - Virtual queue size
- `legacy_queue_align: safe_mmio::fields::WriteOnly<u32>` - Used Ring alignment in the virtual queue
- `legacy_queue_pfn: safe_mmio::fields::ReadPureWrite<u32>` - Guest physical page number of the virtual queue
- `queue_ready: safe_mmio::fields::ReadPureWrite<u32>` - new interface only
- `__r3: [u32; 2]` - Reserved
- `queue_notify: safe_mmio::fields::WriteOnly<u32>` - Queue notifier
- `__r4: [u32; 3]` - Reserved
- `interrupt_status: safe_mmio::fields::ReadPure<u32>` - Interrupt status
- `interrupt_ack: safe_mmio::fields::WriteOnly<u32>` - Interrupt acknowledge
- `__r5: [u32; 2]` - Reserved
- `status: safe_mmio::fields::ReadPureWrite<super::DeviceStatus>` - Device status
- `__r6: [u32; 3]` - Reserved
- `queue_desc_low: safe_mmio::fields::WriteOnly<u32>`
- `queue_desc_high: safe_mmio::fields::WriteOnly<u32>`
- `__r7: [u32; 2]` - Reserved
- `queue_driver_low: safe_mmio::fields::WriteOnly<u32>`
- `queue_driver_high: safe_mmio::fields::WriteOnly<u32>`
- `__r8: [u32; 2]` - Reserved
- `queue_device_low: safe_mmio::fields::WriteOnly<u32>`
- `queue_device_high: safe_mmio::fields::WriteOnly<u32>`
- `__r9: [u32; 21]` - Reserved
- `config_generation: safe_mmio::fields::ReadPure<u32>`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



