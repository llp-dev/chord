**virtio_drivers > transport > x86_64 > hypercalls**

# Module: transport::x86_64::hypercalls

## Contents

**Macros**

- [`vmcall`](#vmcall)

**Structs**

- [`HypIoRegion`](#hypioregion) - A region of physical address space which may be accessed by IO read and/or write hypercalls.

**Functions**

- [`__vmcall_impl`](#__vmcall_impl)
- [`cpuid_signature`](#cpuid_signature) - Gets the signature CPU ID.
- [`hyp_io_read`](#hyp_io_read) - Asks the hypervisor to perform an IO read at the given physical address.
- [`hyp_io_write`](#hyp_io_write) - Asks the hypervisor to perform an IO write at the given physical address.

**Constants**

- [`HYP_IO_MAX`](#hyp_io_max) - The maximum number of bytes that can be read or written by a single IO hypercall.
- [`KVM_CPUID_SIGNATURE`](#kvm_cpuid_signature) - This CPUID returns the signature and should be used to determine if VM is running under pKVM,
- [`KVM_HC_PKVM_OP`](#kvm_hc_pkvm_op)
- [`PKVM_GHC_IOREAD`](#pkvm_ghc_ioread)
- [`PKVM_GHC_IOWRITE`](#pkvm_ghc_iowrite)

---

## virtio_drivers::transport::x86_64::hypercalls::HYP_IO_MAX

*Constant*: `usize`

The maximum number of bytes that can be read or written by a single IO hypercall.



## virtio_drivers::transport::x86_64::hypercalls::HypIoRegion

*Struct*

A region of physical address space which may be accessed by IO read and/or write hypercalls.

**Fields:**
- `paddr: u64` - The physical address of the start of the IO region.
- `size: usize` - The size of the IO region in bytes.

**Methods:**

- `fn read<T>(self: Self, offset: usize) -> T`
- `fn write<T>(self: Self, offset: usize, value: T)`

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &HypIoRegion) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> HypIoRegion`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## virtio_drivers::transport::x86_64::hypercalls::KVM_CPUID_SIGNATURE

*Constant*: `u32`

This CPUID returns the signature and should be used to determine if VM is running under pKVM,
KVM or not. See the Linux header `arch/x86/include/uapi/asm/kvm_para.h`.



## virtio_drivers::transport::x86_64::hypercalls::KVM_HC_PKVM_OP

*Constant*: `u64`



## virtio_drivers::transport::x86_64::hypercalls::PKVM_GHC_IOREAD

*Constant*: `u64`



## virtio_drivers::transport::x86_64::hypercalls::PKVM_GHC_IOWRITE

*Constant*: `u64`



## virtio_drivers::transport::x86_64::hypercalls::__vmcall_impl

*Function*

```rust
fn __vmcall_impl(hypcall: u64, a1: u64, a2: u64, a3: u64, a4: u64) -> u64
```



## virtio_drivers::transport::x86_64::hypercalls::cpuid_signature

*Function*

Gets the signature CPU ID.

```rust
fn cpuid_signature() -> [u8; 4]
```



## virtio_drivers::transport::x86_64::hypercalls::hyp_io_read

*Function*

Asks the hypervisor to perform an IO read at the given physical address.

```rust
fn hyp_io_read(address: u64, size: usize) -> u64
```



## virtio_drivers::transport::x86_64::hypercalls::hyp_io_write

*Function*

Asks the hypervisor to perform an IO write at the given physical address.

```rust
fn hyp_io_write(address: u64, size: usize, data: u64)
```



## virtio_drivers::transport::x86_64::hypercalls::vmcall

*Declarative Macro*

```rust
macro_rules! vmcall {
    ($hypcall:expr) => { ... };
    ($hypcall:expr, $a1:expr) => { ... };
    ($hypcall:expr, $a1:expr, $a2:expr) => { ... };
    ($hypcall:expr, $a1:expr, $a2:expr, $a3:expr) => { ... };
    ($hypcall:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr) => { ... };
}
```



