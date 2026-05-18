**virtio_drivers > transport > pci > bus**

# Module: transport::pci::bus

## Contents

**Structs**

- [`BusDeviceIterator`](#busdeviceiterator) - An iterator which enumerates PCI devices and functions on a given bus.
- [`CapabilityInfo`](#capabilityinfo) - Information about a PCI device capability.
- [`CapabilityIterator`](#capabilityiterator) - Iterator over capabilities for a device.
- [`Command`](#command) - The command register in PCI configuration space.
- [`DeviceFunction`](#devicefunction) - An identifier for a PCI bus, device and function.
- [`DeviceFunctionInfo`](#devicefunctioninfo) - Information about a PCI device function.
- [`MmioCam`](#mmiocam) - `ConfigurationAccess` implementation for memory-mapped access to a PCI root complex, via either
- [`PciRoot`](#pciroot) - The root complex of a PCI bus.
- [`Status`](#status) - The status register in PCI configuration space.

**Enums**

- [`BarInfo`](#barinfo) - Information about a PCI Base Address Register.
- [`Cam`](#cam) - A PCI Configuration Access Mechanism.
- [`HeaderType`](#headertype) - The type of a PCI device function header.
- [`MemoryBarType`](#memorybartype) - The location allowed for a memory BAR.
- [`PciError`](#pcierror) - Errors accessing a PCI device.

**Traits**

- [`ConfigurationAccess`](#configurationaccess) - A method to access PCI configuration space for a particular PCI bus.

**Constants**

- [`BAR0_OFFSET`](#bar0_offset) - The offset in bytes to BAR0 within PCI configuration space.
- [`INVALID_READ`](#invalid_read)
- [`MAX_DEVICES`](#max_devices) - The maximum number of devices on a bus.
- [`MAX_FUNCTIONS`](#max_functions) - The maximum number of functions on a device.
- [`PCI_CAP_ID_VNDR`](#pci_cap_id_vndr) - ID for vendor-specific PCI capabilities.
- [`STATUS_COMMAND_OFFSET`](#status_command_offset) - The offset in bytes to the status and command fields within PCI configuration space.

---

## virtio_drivers::transport::pci::bus::BAR0_OFFSET

*Constant*: `u8`

The offset in bytes to BAR0 within PCI configuration space.



## virtio_drivers::transport::pci::bus::BarInfo

*Enum*

Information about a PCI Base Address Register.

**Variants:**
- `Memory{ address_type: MemoryBarType, prefetchable: bool, address: u64, size: u64 }` - The BAR is for a memory region.
- `IO{ address: u32, size: u32 }` - The BAR is for an I/O region.

**Methods:**

- `fn takes_two_entries(self: &Self) -> bool` - Returns whether this BAR is a 64-bit memory region, and so takes two entries in the table in
- `fn memory_address_size(self: &Self) -> Option<(u64, u64)>` - Returns the address and size of this BAR if it is a memory bar, or `None` if it is an IO

**Traits:** Eq

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> BarInfo`
- **PartialEq**
  - `fn eq(self: &Self, other: &BarInfo) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## virtio_drivers::transport::pci::bus::BusDeviceIterator

*Struct*

An iterator which enumerates PCI devices and functions on a given bus.

**Generic Parameters:**
- C

**Fields:**
- `configuration_access: C` - This must only be used to read read-only fields, and must not be exposed outside this
- `next: DeviceFunction`

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## virtio_drivers::transport::pci::bus::Cam

*Enum*

A PCI Configuration Access Mechanism.

**Variants:**
- `MmioCam` - The PCI memory-mapped Configuration Access Mechanism.
- `Ecam` - The PCIe memory-mapped Enhanced Configuration Access Mechanism.

**Methods:**

- `fn size(self: Self) -> u32` - Returns the total size in bytes of the memory-mapped region.
- `fn cam_offset(self: Self, device_function: DeviceFunction, register_offset: u8) -> u32` - Returns the offset in bytes within the CAM region for the given device, function and

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Cam) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Cam`



## virtio_drivers::transport::pci::bus::CapabilityInfo

*Struct*

Information about a PCI device capability.

**Fields:**
- `offset: u8` - The offset of the capability in the PCI configuration space of the device function.
- `id: u8` - The ID of the capability.
- `private_header: u16` - The third and fourth bytes of the capability, to save reading them again.

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &CapabilityInfo) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> CapabilityInfo`



## virtio_drivers::transport::pci::bus::CapabilityIterator

*Struct*

Iterator over capabilities for a device.

**Generic Parameters:**
- 'a
- C

**Fields:**
- `configuration_access: &'a C`
- `device_function: DeviceFunction`
- `next_capability_offset: Option<u8>`

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## virtio_drivers::transport::pci::bus::Command

*Struct*

The command register in PCI configuration space.

**Tuple Struct**: `(<Command as $crate::__private::PublicFlags>::Internal)`

**Methods:**

- `fn empty() -> Self` - Get a flags value with all bits unset.
- `fn all() -> Self` - Get a flags value with all known bits set.
- `fn bits(self: &Self) -> u16` - Get the underlying bits value.
- `fn from_bits(bits: u16) -> $crate::__private::core::option::Option<Self>` - Convert from a bits value.
- `fn from_bits_truncate(bits: u16) -> Self` - Convert from a bits value, unsetting any unknown bits.
- `fn from_bits_retain(bits: u16) -> Self` - Convert from a bits value exactly.
- `fn from_name(name: &str) -> $crate::__private::core::option::Option<Self>` - Get a flags value with the bits of a flag with the given name set.
- `fn is_empty(self: &Self) -> bool` - Whether all bits in this flags value are unset.
- `fn is_all(self: &Self) -> bool` - Whether all known bits in this flags value are set.
- `fn intersects(self: &Self, other: Self) -> bool` - Whether any set bits in a source flags value are also set in a target flags value.
- `fn contains(self: &Self, other: Self) -> bool` - Whether all set bits in a source flags value are also set in a target flags value.
- `fn insert(self: & mut Self, other: Self)` - The bitwise or (`|`) of the bits in two flags values.
- `fn remove(self: & mut Self, other: Self)` - The intersection of a source flags value with the complement of a target flags
- `fn toggle(self: & mut Self, other: Self)` - The bitwise exclusive-or (`^`) of the bits in two flags values.
- `fn set(self: & mut Self, other: Self, value: bool)` - Call `insert` when `value` is `true` or `remove` when `value` is `false`.
- `fn intersection(self: Self, other: Self) -> Self` - The bitwise and (`&`) of the bits in two flags values.
- `fn union(self: Self, other: Self) -> Self` - The bitwise or (`|`) of the bits in two flags values.
- `fn difference(self: Self, other: Self) -> Self` - The intersection of a source flags value with the complement of a target flags
- `fn symmetric_difference(self: Self, other: Self) -> Self` - The bitwise exclusive-or (`^`) of the bits in two flags values.
- `fn complement(self: Self) -> Self` - The bitwise negation (`!`) of the bits in a flags value, truncating the result.
- `fn iter(self: &Self) -> $crate::iter::Iter<Command>` - Yield a set of contained flags values.
- `fn iter_names(self: &Self) -> $crate::iter::IterNames<Command>` - Yield a set of contained named flags values.

**Traits:** Eq, Copy, PublicFlags

**Trait Implementations:**

- **UpperHex**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: Command) -> Self` - The bitwise or (`|`) of the bits in two flags values.
- **Extend**
  - `fn extend<T>(self: & mut Self, iterator: T)` - The bitwise or (`|`) of the bits in each flags value.
- **IntoIterator**
  - `fn into_iter(self: Self) -> <Self as >::IntoIter`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Octal**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: Self)` - The bitwise and (`&`) of the bits in two flags values.
- **Sub**
  - `fn sub(self: Self, other: Self) -> Self` - The intersection of a source flags value with the complement of a target flags value (`&!`).
- **PartialEq**
  - `fn eq(self: &Self, other: &Command) -> bool`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: Self)` - The bitwise or (`|`) of the bits in two flags values.
- **BitXor**
  - `fn bitxor(self: Self, other: Self) -> Self` - The bitwise exclusive-or (`^`) of the bits in two flags values.
- **FromIterator**
  - `fn from_iter<T>(iterator: T) -> Self` - The bitwise or (`|`) of the bits in each flags value.
- **Default**
  - `fn default() -> Command`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: Self)` - The intersection of a source flags value with the complement of a target flags value (`&!`).
- **Not**
  - `fn not(self: Self) -> Self` - The bitwise negation (`!`) of the bits in a flags value, truncating the result.
- **Flags**
  - `fn bits(self: &Self) -> u16`
  - `fn from_bits_retain(bits: u16) -> Command`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: Self)` - The bitwise exclusive-or (`^`) of the bits in two flags values.
- **Clone**
  - `fn clone(self: &Self) -> Command`
- **Binary**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **BitAnd**
  - `fn bitand(self: Self, other: Self) -> Self` - The bitwise and (`&`) of the bits in two flags values.



## virtio_drivers::transport::pci::bus::ConfigurationAccess

*Trait*

A method to access PCI configuration space for a particular PCI bus.

**Methods:**

- `read_word`: Reads 4 bytes from the configuration space.
- `write_word`: Writes 4 bytes to the configuration space.
- `unsafe_clone`: Makes a clone of the `ConfigurationAccess`, accessing the same PCI bus.



## virtio_drivers::transport::pci::bus::DeviceFunction

*Struct*

An identifier for a PCI bus, device and function.

**Fields:**
- `bus: u8` - The PCI bus number, between 0 and 255.
- `device: u8` - The device number on the bus, between 0 and 31.
- `function: u8` - The function number of the device, between 0 and 7.

**Methods:**

- `fn valid(self: &Self) -> bool` - Returns whether the device and function numbers are valid, i.e. the device is between 0 and

**Traits:** Eq, Copy

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> DeviceFunction`
- **Display**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`
- **Ord**
  - `fn cmp(self: &Self, other: &DeviceFunction) -> $crate::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &DeviceFunction) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &DeviceFunction) -> bool`



## virtio_drivers::transport::pci::bus::DeviceFunctionInfo

*Struct*

Information about a PCI device function.

**Fields:**
- `vendor_id: u16` - The PCI vendor ID.
- `device_id: u16` - The PCI device ID.
- `class: u8` - The PCI class.
- `subclass: u8` - The PCI subclass.
- `prog_if: u8` - The PCI programming interface byte.
- `revision: u8` - The PCI revision ID.
- `header_type: HeaderType` - The type of PCI device.

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> DeviceFunctionInfo`
- **PartialEq**
  - `fn eq(self: &Self, other: &DeviceFunctionInfo) -> bool`



## virtio_drivers::transport::pci::bus::HeaderType

*Enum*

The type of a PCI device function header.

**Variants:**
- `Standard` - A normal PCI device.
- `PciPciBridge` - A PCI to PCI bridge.
- `PciCardbusBridge` - A PCI to CardBus bridge.
- `Unrecognised(u8)` - Unrecognised header type.

**Traits:** Copy, Eq

**Trait Implementations:**

- **From**
  - `fn from(value: u8) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> HeaderType`
- **PartialEq**
  - `fn eq(self: &Self, other: &HeaderType) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## virtio_drivers::transport::pci::bus::INVALID_READ

*Constant*: `u32`



## virtio_drivers::transport::pci::bus::MAX_DEVICES

*Constant*: `u8`

The maximum number of devices on a bus.



## virtio_drivers::transport::pci::bus::MAX_FUNCTIONS

*Constant*: `u8`

The maximum number of functions on a device.



## virtio_drivers::transport::pci::bus::MemoryBarType

*Enum*

The location allowed for a memory BAR.

**Variants:**
- `Width32` - The BAR has a 32-bit address and can be mapped anywhere in 32-bit address space.
- `Below1MiB` - The BAR must be mapped below 1MiB.
- `Width64` - The BAR has a 64-bit address and can be mapped anywhere in 64-bit address space.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> MemoryBarType`
- **PartialEq**
  - `fn eq(self: &Self, other: &MemoryBarType) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **TryFrom**
  - `fn try_from(value: u8) -> Result<Self, <Self as >::Error>`



## virtio_drivers::transport::pci::bus::MmioCam

*Struct*

`ConfigurationAccess` implementation for memory-mapped access to a PCI root complex, via either
a 16 MiB region for the PCI Configuration Access Mechanism or a 256 MiB region for the PCIe
Enhanced Configuration Access Mechanism.

**Generic Parameters:**
- 'a

**Fields:**
- `mmio: safe_mmio::UniqueMmioPointer<'a, [safe_mmio::fields::ReadPureWrite<u32>]>`
- `cam: Cam`

**Methods:**

- `fn new(mmio_base: *mut u8, cam: Cam) -> Self` - Wraps the PCI root complex with the given MMIO base address.

**Traits:** Sync

**Trait Implementations:**

- **ConfigurationAccess**
  - `fn read_word(self: &Self, device_function: DeviceFunction, register_offset: u8) -> u32`
  - `fn write_word(self: & mut Self, device_function: DeviceFunction, register_offset: u8, data: u32)`
  - `fn unsafe_clone(self: &Self) -> Self`



## virtio_drivers::transport::pci::bus::PCI_CAP_ID_VNDR

*Constant*: `u8`

ID for vendor-specific PCI capabilities.



## virtio_drivers::transport::pci::bus::PciError

*Enum*

Errors accessing a PCI device.

**Variants:**
- `InvalidBarType` - The device reported an invalid BAR type.

**Traits:** Error, Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> PciError`
- **Display**
  - `fn fmt(self: &Self, __formatter: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &PciError) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## virtio_drivers::transport::pci::bus::PciRoot

*Struct*

The root complex of a PCI bus.

**Generic Parameters:**
- C

**Fields:**
- `configuration_access: C`

**Methods:**

- `fn new(configuration_access: C) -> Self` - Creates a new `PciRoot` to access a PCI root complex through the given configuration access
- `fn enumerate_bus(self: &Self, bus: u8) -> BusDeviceIterator<C>` - Enumerates PCI devices on the given bus.
- `fn get_status_command(self: &Self, device_function: DeviceFunction) -> (Status, Command)` - Reads the status and command registers of the given device function.
- `fn set_command(self: & mut Self, device_function: DeviceFunction, command: Command)` - Sets the command register of the given device function.
- `fn capabilities(self: &Self, device_function: DeviceFunction) -> CapabilityIterator<C>` - Gets an iterator over the capabilities of the given device function.
- `fn bars(self: & mut Self, device_function: DeviceFunction) -> Result<[Option<BarInfo>; 6], PciError>` - Returns information about all the given device function's BARs.
- `fn bar_info(self: & mut Self, device_function: DeviceFunction, bar_index: u8) -> Result<Option<BarInfo>, PciError>` - Gets information about the given BAR of the given device function.
- `fn set_bar_32(self: & mut Self, device_function: DeviceFunction, bar_index: u8, address: u32)` - Sets the address of the given 32-bit memory or I/O BAR of the given device function.
- `fn set_bar_64(self: & mut Self, device_function: DeviceFunction, bar_index: u8, address: u64)` - Sets the address of the given 64-bit memory BAR of the given device function.
- `fn capabilities_offset(self: &Self, device_function: DeviceFunction) -> Option<u8>` - Gets the capabilities 'pointer' for the device function, if any.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## virtio_drivers::transport::pci::bus::STATUS_COMMAND_OFFSET

*Constant*: `u8`

The offset in bytes to the status and command fields within PCI configuration space.



## virtio_drivers::transport::pci::bus::Status

*Struct*

The status register in PCI configuration space.

**Tuple Struct**: `(<Status as $crate::__private::PublicFlags>::Internal)`

**Methods:**

- `fn empty() -> Self` - Get a flags value with all bits unset.
- `fn all() -> Self` - Get a flags value with all known bits set.
- `fn bits(self: &Self) -> u16` - Get the underlying bits value.
- `fn from_bits(bits: u16) -> $crate::__private::core::option::Option<Self>` - Convert from a bits value.
- `fn from_bits_truncate(bits: u16) -> Self` - Convert from a bits value, unsetting any unknown bits.
- `fn from_bits_retain(bits: u16) -> Self` - Convert from a bits value exactly.
- `fn from_name(name: &str) -> $crate::__private::core::option::Option<Self>` - Get a flags value with the bits of a flag with the given name set.
- `fn is_empty(self: &Self) -> bool` - Whether all bits in this flags value are unset.
- `fn is_all(self: &Self) -> bool` - Whether all known bits in this flags value are set.
- `fn intersects(self: &Self, other: Self) -> bool` - Whether any set bits in a source flags value are also set in a target flags value.
- `fn contains(self: &Self, other: Self) -> bool` - Whether all set bits in a source flags value are also set in a target flags value.
- `fn insert(self: & mut Self, other: Self)` - The bitwise or (`|`) of the bits in two flags values.
- `fn remove(self: & mut Self, other: Self)` - The intersection of a source flags value with the complement of a target flags
- `fn toggle(self: & mut Self, other: Self)` - The bitwise exclusive-or (`^`) of the bits in two flags values.
- `fn set(self: & mut Self, other: Self, value: bool)` - Call `insert` when `value` is `true` or `remove` when `value` is `false`.
- `fn intersection(self: Self, other: Self) -> Self` - The bitwise and (`&`) of the bits in two flags values.
- `fn union(self: Self, other: Self) -> Self` - The bitwise or (`|`) of the bits in two flags values.
- `fn difference(self: Self, other: Self) -> Self` - The intersection of a source flags value with the complement of a target flags
- `fn symmetric_difference(self: Self, other: Self) -> Self` - The bitwise exclusive-or (`^`) of the bits in two flags values.
- `fn complement(self: Self) -> Self` - The bitwise negation (`!`) of the bits in a flags value, truncating the result.
- `fn iter(self: &Self) -> $crate::iter::Iter<Status>` - Yield a set of contained flags values.
- `fn iter_names(self: &Self) -> $crate::iter::IterNames<Status>` - Yield a set of contained named flags values.

**Traits:** Eq, Copy, PublicFlags

**Trait Implementations:**

- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: Self)` - The intersection of a source flags value with the complement of a target flags value (`&!`).
- **Not**
  - `fn not(self: Self) -> Self` - The bitwise negation (`!`) of the bits in a flags value, truncating the result.
- **Flags**
  - `fn bits(self: &Self) -> u16`
  - `fn from_bits_retain(bits: u16) -> Status`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: Self)` - The bitwise exclusive-or (`^`) of the bits in two flags values.
- **Clone**
  - `fn clone(self: &Self) -> Status`
- **Binary**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **BitAnd**
  - `fn bitand(self: Self, other: Self) -> Self` - The bitwise and (`&`) of the bits in two flags values.
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: Status) -> Self` - The bitwise or (`|`) of the bits in two flags values.
- **Extend**
  - `fn extend<T>(self: & mut Self, iterator: T)` - The bitwise or (`|`) of the bits in each flags value.
- **IntoIterator**
  - `fn into_iter(self: Self) -> <Self as >::IntoIter`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Octal**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: Self)` - The bitwise and (`&`) of the bits in two flags values.
- **Sub**
  - `fn sub(self: Self, other: Self) -> Self` - The intersection of a source flags value with the complement of a target flags value (`&!`).
- **PartialEq**
  - `fn eq(self: &Self, other: &Status) -> bool`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: Self)` - The bitwise or (`|`) of the bits in two flags values.
- **BitXor**
  - `fn bitxor(self: Self, other: Self) -> Self` - The bitwise exclusive-or (`^`) of the bits in two flags values.
- **FromIterator**
  - `fn from_iter<T>(iterator: T) -> Self` - The bitwise or (`|`) of the bits in each flags value.
- **Default**
  - `fn default() -> Status`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`



