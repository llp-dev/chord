# virtio_drivers

VirtIO guest drivers.

These drivers can be used by bare-metal code (such as a bootloader or OS kernel) running in a VM
to interact with VirtIO devices provided by the VMM (such as QEMU or crosvm).

# Usage

You must first implement the [`Hal`] trait, to allocate DMA regions and translate between
physical addresses (as seen by devices) and virtual addresses (as seen by your program). You can
then construct the appropriate transport for the VirtIO device, e.g. for an MMIO device (perhaps
discovered from the device tree):

```
use core::ptr::NonNull;
use virtio_drivers::transport::mmio::{MmioTransport, VirtIOHeader};

# fn example(mmio_device_address: usize, mmio_size: usize) {
let header = NonNull::new(mmio_device_address as *mut VirtIOHeader).unwrap();
let transport = unsafe { MmioTransport::new(header, mmio_size) }.unwrap();
# }
```

You can then check what kind of VirtIO device it is and construct the appropriate driver:

```
# use virtio_drivers::Hal;
# #[cfg(feature = "alloc")]
use virtio_drivers::{
    device::console::VirtIOConsole,
    transport::{mmio::MmioTransport, DeviceType, Transport},
};

# #[cfg(feature = "alloc")]
# fn example<HalImpl: Hal>(transport: MmioTransport) {
if transport.device_type() == DeviceType::Console {
    let mut console = VirtIOConsole::<HalImpl, _>::new(transport).unwrap();
    // Send a byte to the console.
    console.send(b'H').unwrap();
}
# }
```

## Modules

### [`virtio_drivers`](virtio_drivers.md)

*1 enum, 1 type alias, 2 constants, 2 macros, 3 functions, 5 modules*

### [`config`](config.md)

*2 traits, 3 structs*

### [`device`](device.md)

*10 modules*

### [`device::blk`](device/blk.md)

*1 enum, 4 constants, 6 structs*

### [`device::common`](device/common.md)

*1 struct*

### [`device::console`](device/console.md)

*4 constants, 4 structs*

### [`device::gpu`](device/gpu.md)

*1 enum, 1 module, 10 constants, 18 structs*

### [`device::gpu::edid`](device/gpu/edid.md)

*1 enum, 3 structs, 5 constants*

### [`device::input`](device/input.md)

*1 enum, 5 constants, 5 structs*

### [`device::net`](device/net.md)

*1 type alias, 3 modules, 5 constants, 8 structs*

### [`device::net::dev`](device/net/dev.md)

*1 struct*

### [`device::net::dev_raw`](device/net/dev_raw.md)

*1 struct*

### [`device::net::net_buf`](device/net/net_buf.md)

*2 structs*

### [`device::rng`](device/rng.md)

*1 struct, 3 constants*

### [`device::socket`](device/socket.md)

*1 constant, 4 modules*

### [`device::socket::connectionmanager`](device/socket/connectionmanager.md)

*1 constant, 2 functions, 3 structs*

### [`device::socket::error`](device/socket/error.md)

*1 enum, 1 type alias*

### [`device::socket::protocol`](device/socket/protocol.md)

*1 constant, 2 enums, 6 structs*

### [`device::socket::vsock`](device/socket/vsock.md)

*1 function, 2 enums, 4 structs, 5 constants*

### [`device::sound`](device/sound.md)

*10 enums, 23 structs, 9 constants*

### [`device::virtio_9p`](device/virtio_9p.md)

*1 function, 1 struct, 4 constants*

### [`hal`](hal.md)

*1 enum, 1 struct, 1 trait, 1 type alias*

### [`queue`](queue.md)

*1 enum, 1 module, 3 functions, 7 structs*

### [`queue::owning`](queue/owning.md)

*1 struct*

### [`transport`](transport.md)

*1 trait, 2 enums, 2 structs, 4 modules*

### [`transport::mmio`](transport/mmio.md)

*2 enums, 2 structs, 4 constants*

### [`transport::pci`](transport/pci.md)

*1 enum, 1 module, 17 constants, 3 structs, 4 functions*

### [`transport::pci::bus`](transport/pci/bus.md)

*1 trait, 5 enums, 6 constants, 9 structs*

### [`transport::some`](transport/some.md)

*1 enum*

### [`transport::x86_64`](transport/x86_64.md)

*1 function, 1 struct, 2 macros, 2 modules*

### [`transport::x86_64::cam`](transport/x86_64/cam.md)

*1 constant, 1 struct*

### [`transport::x86_64::hypercalls`](transport/x86_64/hypercalls.md)

*1 macro, 1 struct, 4 functions, 5 constants*

