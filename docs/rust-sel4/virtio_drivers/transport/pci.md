**virtio_drivers > transport > pci**

# Module: transport::pci

## Contents

**Modules**

- [`bus`](#bus) - Module for dealing with a PCI bus in general, without anything specific to VirtIO.

**Structs**

- [`CommonCfg`](#commoncfg) - `virtio_pci_common_cfg`, see 4.1.4.3 "Common configuration structure layout".
- [`PciTransport`](#pcitransport) - PCI transport for VirtIO.
- [`VirtioCapabilityInfo`](#virtiocapabilityinfo) - Information about a VirtIO structure within some BAR, as provided by a `virtio_pci_cap`.

**Enums**

- [`VirtioPciError`](#virtiopcierror) - An error encountered initialising a VirtIO PCI transport.

**Functions**

- [`device_type`](#device_type)
- [`get_bar_region`](#get_bar_region)
- [`get_bar_region_slice`](#get_bar_region_slice)
- [`virtio_device_type`](#virtio_device_type) - Returns the type of VirtIO device to which the given PCI vendor and device ID corresponds, or

**Constants**

- [`CAP_BAR_OFFSET`](#cap_bar_offset) - The offset of the bar field within `virtio_pci_cap`.
- [`CAP_BAR_OFFSET_OFFSET`](#cap_bar_offset_offset) - The offset of the offset field with `virtio_pci_cap`.
- [`CAP_LENGTH_OFFSET`](#cap_length_offset) - The offset of the `length` field within `virtio_pci_cap`.
- [`CAP_NOTIFY_OFF_MULTIPLIER_OFFSET`](#cap_notify_off_multiplier_offset) - The offset of the`notify_off_multiplier` field within `virtio_pci_notify_cap`.
- [`PCI_DEVICE_ID_OFFSET`](#pci_device_id_offset) - The offset to add to a VirtIO device ID to get the corresponding PCI device ID.
- [`TRANSITIONAL_9P_TRANSPORT`](#transitional_9p_transport)
- [`TRANSITIONAL_BLOCK`](#transitional_block)
- [`TRANSITIONAL_CONSOLE`](#transitional_console)
- [`TRANSITIONAL_ENTROPY_SOURCE`](#transitional_entropy_source)
- [`TRANSITIONAL_MEMORY_BALLOONING`](#transitional_memory_ballooning)
- [`TRANSITIONAL_NETWORK`](#transitional_network)
- [`TRANSITIONAL_SCSI_HOST`](#transitional_scsi_host)
- [`VIRTIO_PCI_CAP_COMMON_CFG`](#virtio_pci_cap_common_cfg) - Common configuration.
- [`VIRTIO_PCI_CAP_DEVICE_CFG`](#virtio_pci_cap_device_cfg) - Device specific configuration.
- [`VIRTIO_PCI_CAP_ISR_CFG`](#virtio_pci_cap_isr_cfg) - ISR Status.
- [`VIRTIO_PCI_CAP_NOTIFY_CFG`](#virtio_pci_cap_notify_cfg) - Notifications.
- [`VIRTIO_VENDOR_ID`](#virtio_vendor_id) - The PCI vendor ID for VirtIO devices.

---

## virtio_drivers::transport::pci::CAP_BAR_OFFSET

*Constant*: `u8`

The offset of the bar field within `virtio_pci_cap`.



## virtio_drivers::transport::pci::CAP_BAR_OFFSET_OFFSET

*Constant*: `u8`

The offset of the offset field with `virtio_pci_cap`.



## virtio_drivers::transport::pci::CAP_LENGTH_OFFSET

*Constant*: `u8`

The offset of the `length` field within `virtio_pci_cap`.



## virtio_drivers::transport::pci::CAP_NOTIFY_OFF_MULTIPLIER_OFFSET

*Constant*: `u8`

The offset of the`notify_off_multiplier` field within `virtio_pci_notify_cap`.



## virtio_drivers::transport::pci::CommonCfg

*Struct*

`virtio_pci_common_cfg`, see 4.1.4.3 "Common configuration structure layout".

**Fields:**
- `device_feature_select: safe_mmio::fields::ReadPureWrite<u32>`
- `device_feature: safe_mmio::fields::ReadPure<u32>`
- `driver_feature_select: safe_mmio::fields::ReadPureWrite<u32>`
- `driver_feature: safe_mmio::fields::ReadPureWrite<u32>`
- `msix_config: safe_mmio::fields::ReadPureWrite<u16>`
- `num_queues: safe_mmio::fields::ReadPure<u16>`
- `device_status: safe_mmio::fields::ReadPureWrite<u8>`
- `config_generation: safe_mmio::fields::ReadPure<u8>`
- `queue_select: safe_mmio::fields::ReadPureWrite<u16>`
- `queue_size: safe_mmio::fields::ReadPureWrite<u16>`
- `queue_msix_vector: safe_mmio::fields::ReadPureWrite<u16>`
- `queue_enable: safe_mmio::fields::ReadPureWrite<u16>`
- `queue_notify_off: safe_mmio::fields::ReadPureWrite<u16>`
- `queue_desc: safe_mmio::fields::ReadPureWrite<u64>`
- `queue_driver: safe_mmio::fields::ReadPureWrite<u64>`
- `queue_device: safe_mmio::fields::ReadPureWrite<u64>`



## virtio_drivers::transport::pci::PCI_DEVICE_ID_OFFSET

*Constant*: `u16`

The offset to add to a VirtIO device ID to get the corresponding PCI device ID.



## virtio_drivers::transport::pci::PciTransport

*Struct*

PCI transport for VirtIO.

Ref: 4.1 Virtio Over PCI Bus

**Fields:**
- `device_type: super::DeviceType`
- `device_function: self::bus::DeviceFunction` - The bus, device and function identifier for the VirtIO device.
- `common_cfg: safe_mmio::UniqueMmioPointer<'static, CommonCfg>` - The common configuration structure within some BAR.
- `notify_region: safe_mmio::UniqueMmioPointer<'static, [safe_mmio::fields::WriteOnly<u16>]>` - The start of the queue notification region within some BAR.
- `notify_off_multiplier: u32`
- `isr_status: safe_mmio::UniqueMmioPointer<'static, safe_mmio::fields::ReadOnly<u8>>` - The ISR status register within some BAR.
- `config_space: Option<safe_mmio::UniqueMmioPointer<'static, [u32]>>` - The VirtIO device-specific configuration within some BAR.

**Methods:**

- `fn new<H, C>(root: & mut PciRoot<C>, device_function: DeviceFunction) -> Result<Self, VirtioPciError>` - Construct a new PCI VirtIO device driver for the given device function on the given PCI

**Traits:** Sync, Send

**Trait Implementations:**

- **Transport**
  - `fn device_type(self: &Self) -> DeviceType`
  - `fn read_device_features(self: & mut Self) -> u64`
  - `fn write_driver_features(self: & mut Self, driver_features: u64)`
  - `fn max_queue_size(self: & mut Self, queue: u16) -> u32`
  - `fn notify(self: & mut Self, queue: u16)`
  - `fn get_status(self: &Self) -> DeviceStatus`
  - `fn set_status(self: & mut Self, status: DeviceStatus)`
  - `fn set_guest_page_size(self: & mut Self, _guest_page_size: u32)`
  - `fn requires_legacy_layout(self: &Self) -> bool`
  - `fn queue_set(self: & mut Self, queue: u16, size: u32, descriptors: PhysAddr, driver_area: PhysAddr, device_area: PhysAddr)`
  - `fn queue_unset(self: & mut Self, _queue: u16)`
  - `fn queue_used(self: & mut Self, queue: u16) -> bool`
  - `fn ack_interrupt(self: & mut Self) -> InterruptStatus`
  - `fn read_config_generation(self: &Self) -> u32`
  - `fn read_config_space<T>(self: &Self, offset: usize) -> Result<T, Error>`
  - `fn write_config_space<T>(self: & mut Self, offset: usize, value: T) -> Result<(), Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Drop**
  - `fn drop(self: & mut Self)`



## virtio_drivers::transport::pci::TRANSITIONAL_9P_TRANSPORT

*Constant*: `u16`



## virtio_drivers::transport::pci::TRANSITIONAL_BLOCK

*Constant*: `u16`



## virtio_drivers::transport::pci::TRANSITIONAL_CONSOLE

*Constant*: `u16`



## virtio_drivers::transport::pci::TRANSITIONAL_ENTROPY_SOURCE

*Constant*: `u16`



## virtio_drivers::transport::pci::TRANSITIONAL_MEMORY_BALLOONING

*Constant*: `u16`



## virtio_drivers::transport::pci::TRANSITIONAL_NETWORK

*Constant*: `u16`



## virtio_drivers::transport::pci::TRANSITIONAL_SCSI_HOST

*Constant*: `u16`



## virtio_drivers::transport::pci::VIRTIO_PCI_CAP_COMMON_CFG

*Constant*: `u8`

Common configuration.



## virtio_drivers::transport::pci::VIRTIO_PCI_CAP_DEVICE_CFG

*Constant*: `u8`

Device specific configuration.



## virtio_drivers::transport::pci::VIRTIO_PCI_CAP_ISR_CFG

*Constant*: `u8`

ISR Status.



## virtio_drivers::transport::pci::VIRTIO_PCI_CAP_NOTIFY_CFG

*Constant*: `u8`

Notifications.



## virtio_drivers::transport::pci::VIRTIO_VENDOR_ID

*Constant*: `u16`

The PCI vendor ID for VirtIO devices.



## virtio_drivers::transport::pci::VirtioCapabilityInfo

*Struct*

Information about a VirtIO structure within some BAR, as provided by a `virtio_pci_cap`.

**Fields:**
- `bar: u8` - The bar in which the structure can be found.
- `offset: u32` - The offset within the bar.
- `length: u32` - The length in bytes of the structure within the bar.

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &VirtioCapabilityInfo) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> VirtioCapabilityInfo`



## virtio_drivers::transport::pci::VirtioPciError

*Enum*

An error encountered initialising a VirtIO PCI transport.

**Variants:**
- `InvalidDeviceId(u16)` - PCI device ID was not a valid VirtIO device ID.
- `InvalidVendorId(u16)` - PCI device vender ID was not the VirtIO vendor ID.
- `MissingCommonConfig` - No valid `VIRTIO_PCI_CAP_COMMON_CFG` capability was found.
- `MissingNotifyConfig` - No valid `VIRTIO_PCI_CAP_NOTIFY_CFG` capability was found.
- `InvalidNotifyOffMultiplier(u32)` - `VIRTIO_PCI_CAP_NOTIFY_CFG` capability has a `notify_off_multiplier` that is not a multiple
- `MissingIsrConfig` - No valid `VIRTIO_PCI_CAP_ISR_CFG` capability was found.
- `UnexpectedIoBar` - An IO BAR was provided rather than a memory BAR.
- `BarNotAllocated(u8)` - A BAR which we need was not allocated an address.
- `BarOffsetOutOfRange` - The offset for some capability was greater than the length of the BAR.
- `Misaligned{ address: usize, alignment: usize }` - The address was not aligned as expected.
- `Pci(self::bus::PciError)` - A generic PCI error,

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Error**
  - `fn source(self: &Self) -> ::core::option::Option<&dyn ::thiserror::__private18::Error>`
- **Display**
  - `fn fmt(self: &Self, __formatter: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &VirtioPciError) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> VirtioPciError`
- **From**
  - `fn from(error: PciError) -> Self`



## Module: bus

Module for dealing with a PCI bus in general, without anything specific to VirtIO.



## virtio_drivers::transport::pci::device_type

*Function*

```rust
fn device_type(pci_device_id: u16) -> Option<super::DeviceType>
```



## virtio_drivers::transport::pci::get_bar_region

*Function*

```rust
fn get_bar_region<H, T, C>(root: & mut self::bus::PciRoot<C>, device_function: self::bus::DeviceFunction, struct_info: &VirtioCapabilityInfo) -> Result<core::ptr::NonNull<T>, VirtioPciError>
```



## virtio_drivers::transport::pci::get_bar_region_slice

*Function*

```rust
fn get_bar_region_slice<H, T, C>(root: & mut self::bus::PciRoot<C>, device_function: self::bus::DeviceFunction, struct_info: &VirtioCapabilityInfo) -> Result<core::ptr::NonNull<[T]>, VirtioPciError>
```



## virtio_drivers::transport::pci::virtio_device_type

*Function*

Returns the type of VirtIO device to which the given PCI vendor and device ID corresponds, or
`None` if it is not a recognised VirtIO device.

```rust
fn virtio_device_type(device_function_info: &self::bus::DeviceFunctionInfo) -> Option<super::DeviceType>
```



