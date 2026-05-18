**virtio_drivers > transport > x86_64 > cam**

# Module: transport::x86_64::cam

## Contents

**Structs**

- [`HypCam`](#hypcam) - A PCI configuration access mechanism using hypercalls implemented by the x86-64 pKVM hypervisor.

**Constants**

- [`PKVM_SIGNATURE`](#pkvm_signature)

---

## virtio_drivers::transport::x86_64::cam::HypCam

*Struct*

A PCI configuration access mechanism using hypercalls implemented by the x86-64 pKVM hypervisor.

**Fields:**
- `phys_base: crate::hal::PhysAddr` - The physical base address of the PCI root complex.
- `cam: crate::transport::pci::bus::Cam`

**Methods:**

- `fn new(phys_base: PhysAddr, cam: Cam) -> Self` - Creates a new `HypCam` for the PCI root complex at the given physical base address.
- `fn is_pkvm() -> bool` - Returns whether we are running under pKVM by checking the CPU ID signature.

**Trait Implementations:**

- **ConfigurationAccess**
  - `fn read_word(self: &Self, device_function: DeviceFunction, register_offset: u8) -> u32`
  - `fn write_word(self: & mut Self, device_function: DeviceFunction, register_offset: u8, data: u32)`
  - `fn unsafe_clone(self: &Self) -> Self`



## virtio_drivers::transport::x86_64::cam::PKVM_SIGNATURE

*Constant*: `&[u8]`



