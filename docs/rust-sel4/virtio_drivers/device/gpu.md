**virtio_drivers > device > gpu**

# Module: device::gpu

## Contents

**Modules**

- [`edid`](#edid) - EDID (Extended Display Identification Data) parsing.

**Structs**

- [`CmdGetEdid`](#cmdgetedid)
- [`Command`](#command)
- [`Config`](#config)
- [`CtrlHeader`](#ctrlheader)
- [`CursorPos`](#cursorpos)
- [`Features`](#features)
- [`Rect`](#rect)
- [`ResourceAttachBacking`](#resourceattachbacking)
- [`ResourceCreate2D`](#resourcecreate2d)
- [`ResourceDetachBacking`](#resourcedetachbacking)
- [`ResourceFlush`](#resourceflush)
- [`ResourceUnref`](#resourceunref)
- [`RespDisplayInfo`](#respdisplayinfo)
- [`RespEdid`](#respedid)
- [`SetScanout`](#setscanout)
- [`TransferToHost2D`](#transfertohost2d)
- [`UpdateCursor`](#updatecursor)
- [`VirtIOGpu`](#virtiogpu) - A virtio based graphics adapter.

**Enums**

- [`Format`](#format)

**Constants**

- [`CURSOR_RECT`](#cursor_rect)
- [`EVENT_DISPLAY`](#event_display) - Display configuration has changed.
- [`GPU_FLAG_FENCE`](#gpu_flag_fence)
- [`QUEUE_CURSOR`](#queue_cursor)
- [`QUEUE_SIZE`](#queue_size)
- [`QUEUE_TRANSMIT`](#queue_transmit)
- [`RESOURCE_ID_CURSOR`](#resource_id_cursor)
- [`RESOURCE_ID_FB`](#resource_id_fb)
- [`SCANOUT_ID`](#scanout_id)
- [`SUPPORTED_FEATURES`](#supported_features)

---

## virtio_drivers::device::gpu::CURSOR_RECT

*Constant*: `Rect`



## virtio_drivers::device::gpu::CmdGetEdid

*Struct*

**Fields:**
- `header: CtrlHeader`
- `scanout: u32`
- `_padding: u32`

**Traits:** Immutable, KnownLayout, IntoBytes

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## virtio_drivers::device::gpu::Command

*Struct*

**Tuple Struct**: `(u32)`

**Methods:**


**Traits:** FromZeros, IntoBytes, TryFromBytes, Immutable, KnownLayout, Copy, Eq, FromBytes

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Command) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Command`



## virtio_drivers::device::gpu::Config

*Struct*

**Fields:**
- `events_read: crate::config::ReadOnly<u32>` - Signals pending events to the driver。
- `events_clear: crate::config::WriteOnly<u32>` - Clears pending events in the device.
- `num_scanouts: crate::config::ReadOnly<u32>` - Specifies the maximum number of scanouts supported by the device.



## virtio_drivers::device::gpu::CtrlHeader

*Struct*

**Fields:**
- `hdr_type: Command`
- `flags: u32`
- `fence_id: u64`
- `ctx_id: u32`
- `_padding: u32`

**Methods:**

- `fn with_type(hdr_type: Command) -> CtrlHeader`
- `fn check_type(self: &Self, expected: Command) -> Result` - Return error if the type is not same as expected.

**Traits:** FromZeros, IntoBytes, TryFromBytes, Immutable, KnownLayout, Copy, FromBytes

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> CtrlHeader`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## virtio_drivers::device::gpu::CursorPos

*Struct*

**Fields:**
- `scanout_id: u32`
- `x: u32`
- `y: u32`
- `_padding: u32`

**Traits:** IntoBytes, Immutable, KnownLayout, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> CursorPos`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## virtio_drivers::device::gpu::EVENT_DISPLAY

*Constant*: `u32`

Display configuration has changed.



## virtio_drivers::device::gpu::Features

*Struct*

**Tuple Struct**: `(<Features as $crate::__private::PublicFlags>::Internal)`

**Methods:**

- `fn empty() -> Self` - Get a flags value with all bits unset.
- `fn all() -> Self` - Get a flags value with all known bits set.
- `fn bits(self: &Self) -> u64` - Get the underlying bits value.
- `fn from_bits(bits: u64) -> $crate::__private::core::option::Option<Self>` - Convert from a bits value.
- `fn from_bits_truncate(bits: u64) -> Self` - Convert from a bits value, unsetting any unknown bits.
- `fn from_bits_retain(bits: u64) -> Self` - Convert from a bits value exactly.
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
- `fn iter(self: &Self) -> $crate::iter::Iter<Features>` - Yield a set of contained flags values.
- `fn iter_names(self: &Self) -> $crate::iter::IterNames<Features>` - Yield a set of contained named flags values.

**Traits:** Eq, Copy, PublicFlags

**Trait Implementations:**

- **BitOr**
  - `fn bitor(self: Self, other: Features) -> Self` - The bitwise or (`|`) of the bits in two flags values.
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
  - `fn eq(self: &Self, other: &Features) -> bool`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: Self)` - The bitwise or (`|`) of the bits in two flags values.
- **BitXor**
  - `fn bitxor(self: Self, other: Self) -> Self` - The bitwise exclusive-or (`^`) of the bits in two flags values.
- **FromIterator**
  - `fn from_iter<T>(iterator: T) -> Self` - The bitwise or (`|`) of the bits in each flags value.
- **Default**
  - `fn default() -> Features`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: Self)` - The intersection of a source flags value with the complement of a target flags value (`&!`).
- **Not**
  - `fn not(self: Self) -> Self` - The bitwise negation (`!`) of the bits in a flags value, truncating the result.
- **Flags**
  - `fn bits(self: &Self) -> u64`
  - `fn from_bits_retain(bits: u64) -> Features`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: Self)` - The bitwise exclusive-or (`^`) of the bits in two flags values.
- **Clone**
  - `fn clone(self: &Self) -> Features`
- **Binary**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **BitAnd**
  - `fn bitand(self: Self, other: Self) -> Self` - The bitwise and (`&`) of the bits in two flags values.
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`



## virtio_drivers::device::gpu::Format

*Enum*

**Variants:**
- `B8G8R8A8UNORM`

**Traits:** IntoBytes, Immutable, KnownLayout

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## virtio_drivers::device::gpu::GPU_FLAG_FENCE

*Constant*: `u32`



## virtio_drivers::device::gpu::QUEUE_CURSOR

*Constant*: `u16`



## virtio_drivers::device::gpu::QUEUE_SIZE

*Constant*: `u16`



## virtio_drivers::device::gpu::QUEUE_TRANSMIT

*Constant*: `u16`



## virtio_drivers::device::gpu::RESOURCE_ID_CURSOR

*Constant*: `u32`



## virtio_drivers::device::gpu::RESOURCE_ID_FB

*Constant*: `u32`



## virtio_drivers::device::gpu::Rect

*Struct*

**Fields:**
- `x: u32`
- `y: u32`
- `width: u32`
- `height: u32`

**Traits:** FromBytes, Copy, FromZeros, IntoBytes, TryFromBytes, Immutable, KnownLayout

**Trait Implementations:**

- **Default**
  - `fn default() -> Rect`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Rect`



## virtio_drivers::device::gpu::ResourceAttachBacking

*Struct*

**Fields:**
- `header: CtrlHeader`
- `resource_id: u32`
- `nr_entries: u32`
- `addr: u64`
- `length: u32`
- `_padding: u32`

**Traits:** IntoBytes, Immutable, KnownLayout

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## virtio_drivers::device::gpu::ResourceCreate2D

*Struct*

**Fields:**
- `header: CtrlHeader`
- `resource_id: u32`
- `format: Format`
- `width: u32`
- `height: u32`

**Traits:** Immutable, KnownLayout, IntoBytes

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## virtio_drivers::device::gpu::ResourceDetachBacking

*Struct*

**Fields:**
- `header: CtrlHeader`
- `resource_id: u32`
- `_padding: u32`

**Traits:** IntoBytes, Immutable, KnownLayout

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## virtio_drivers::device::gpu::ResourceFlush

*Struct*

**Fields:**
- `header: CtrlHeader`
- `rect: Rect`
- `resource_id: u32`
- `_padding: u32`

**Traits:** IntoBytes, Immutable, KnownLayout

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## virtio_drivers::device::gpu::ResourceUnref

*Struct*

**Fields:**
- `header: CtrlHeader`
- `resource_id: u32`
- `_padding: u32`

**Traits:** IntoBytes, Immutable, KnownLayout

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## virtio_drivers::device::gpu::RespDisplayInfo

*Struct*

**Fields:**
- `header: CtrlHeader`
- `rect: Rect`
- `enabled: u32`
- `flags: u32`

**Traits:** TryFromBytes, Immutable, FromBytes, KnownLayout, FromZeros

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## virtio_drivers::device::gpu::RespEdid

*Struct*

**Fields:**
- `header: CtrlHeader`
- `size: u32`
- `_padding: u32`
- `edid: [u8; 1024]`

**Traits:** TryFromBytes, Immutable, FromBytes, KnownLayout, FromZeros

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## virtio_drivers::device::gpu::SCANOUT_ID

*Constant*: `u32`



## virtio_drivers::device::gpu::SUPPORTED_FEATURES

*Constant*: `Features`



## virtio_drivers::device::gpu::SetScanout

*Struct*

**Fields:**
- `header: CtrlHeader`
- `rect: Rect`
- `scanout_id: u32`
- `resource_id: u32`

**Traits:** IntoBytes, Immutable, KnownLayout

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## virtio_drivers::device::gpu::TransferToHost2D

*Struct*

**Fields:**
- `header: CtrlHeader`
- `rect: Rect`
- `offset: u64`
- `resource_id: u32`
- `_padding: u32`

**Traits:** IntoBytes, Immutable, KnownLayout

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## virtio_drivers::device::gpu::UpdateCursor

*Struct*

**Fields:**
- `header: CtrlHeader`
- `pos: CursorPos`
- `resource_id: u32`
- `hot_x: u32`
- `hot_y: u32`
- `_padding: u32`

**Traits:** IntoBytes, Immutable, KnownLayout, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> UpdateCursor`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## virtio_drivers::device::gpu::VirtIOGpu

*Struct*

A virtio based graphics adapter.

It can operate in 2D mode and in 3D (virgl) mode.
3D mode will offload rendering ops to the host gpu and therefore requires
a gpu with 3D support on the host machine.
In 2D mode the virtio-gpu device provides support for ARGB Hardware cursors
and multiple scanouts (aka heads).

**Generic Parameters:**
- H
- T

**Fields:**
- `transport: T`
- `rect: Option<Rect>`
- `frame_buffer_dma: Option<crate::hal::Dma<H>>` - DMA area of frame buffer.
- `cursor_buffer_dma: Option<crate::hal::Dma<H>>` - DMA area of cursor image buffer.
- `control_queue: crate::queue::VirtQueue<H, { _ }>` - Queue for sending control commands.
- `cursor_queue: crate::queue::VirtQueue<H, { _ }>` - Queue for sending cursor commands.
- `queue_buf_send: alloc::boxed::Box<[u8]>` - Send buffer for queue.
- `queue_buf_recv: alloc::boxed::Box<[u8]>` - Recv buffer for queue.
- `has_edid: bool` - Whether EDID feature was negotiated.

**Methods:**

- `fn new(transport: T) -> Result<Self>` - Create a new VirtIO-Gpu driver.
- `fn ack_interrupt(self: & mut Self) -> InterruptStatus` - Acknowledge interrupt.
- `fn resolution(self: & mut Self) -> Result<(u32, u32)>` - Get the resolution (width, height).
- `fn get_edid(self: & mut Self, scanout: u32) -> Result<Edid>` - Get the EDID data for the specified scanout.
- `fn edid_preferred_resolution(self: & mut Self) -> Result<(u32, u32)>` - Get the preferred resolution from the EDID data.
- `fn edid_supported_resolutions(self: & mut Self) -> Result<Vec<(u32, u32)>>` - Get the list of supported resolutions from EDID data.
- `fn setup_framebuffer(self: & mut Self) -> Result<& mut [u8]>` - Setup framebuffer at the display's default resolution.
- `fn change_resolution(self: & mut Self, width: u32, height: u32) -> Result<& mut [u8]>` - Set or change the framebuffer resolution. If a framebuffer already exists, tears down the
- `fn flush(self: & mut Self) -> Result` - Flush framebuffer to screen.
- `fn setup_cursor(self: & mut Self, cursor_image: &[u8], pos_x: u32, pos_y: u32, hot_x: u32, hot_y: u32) -> Result` - Set the pointer shape and position.
- `fn move_cursor(self: & mut Self, pos_x: u32, pos_y: u32) -> Result` - Move the pointer without updating the shape.
- `fn request<Req, Rsp>(self: & mut Self, req: Req) -> Result<Rsp>` - Send a request to the device and block for a response.
- `fn cursor_request<Req>(self: & mut Self, req: Req) -> Result` - Send a mouse cursor operation request to the device and block for a response.
- `fn get_display_info(self: & mut Self) -> Result<RespDisplayInfo>`
- `fn resource_create_2d(self: & mut Self, resource_id: u32, width: u32, height: u32) -> Result`
- `fn set_scanout(self: & mut Self, rect: Rect, scanout_id: u32, resource_id: u32) -> Result`
- `fn resource_flush(self: & mut Self, rect: Rect, resource_id: u32) -> Result`
- `fn transfer_to_host_2d(self: & mut Self, rect: Rect, offset: u64, resource_id: u32) -> Result`
- `fn resource_attach_backing(self: & mut Self, resource_id: u32, paddr: u64, length: u32) -> Result`
- `fn resource_detach_backing(self: & mut Self, resource_id: u32) -> Result`
- `fn resource_unref(self: & mut Self, resource_id: u32) -> Result`
- `fn update_cursor(self: & mut Self, resource_id: u32, scanout_id: u32, pos_x: u32, pos_y: u32, hot_x: u32, hot_y: u32, is_move: bool) -> Result`

**Trait Implementations:**

- **Drop**
  - `fn drop(self: & mut Self)`



## Module: edid

EDID (Extended Display Identification Data) parsing.

Provides types for extracting display information from raw EDID blobs,
including preferred resolution and standard timing modes.

Reference: VESA Enhanced EDID Standard (E-EDID), Release A, Rev. 2.



