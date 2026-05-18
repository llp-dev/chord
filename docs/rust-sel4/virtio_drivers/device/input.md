**virtio_drivers > device > input**

# Module: device::input

## Contents

**Structs**

- [`AbsInfo`](#absinfo) - Information about an axis of an input device, typically a joystick.
- [`Config`](#config)
- [`DevIDs`](#devids) - The identifiers of a VirtIO input device.
- [`InputEvent`](#inputevent) - Both queues use the same `virtio_input_event` struct. `type`, `code` and `value`
- [`VirtIOInput`](#virtioinput) - Virtual human interface devices such as keyboards, mice and tablets.

**Enums**

- [`InputConfigSelect`](#inputconfigselect) - Select value used for [`VirtIOInput::query_config_select()`].

**Constants**

- [`CONFIG_DATA_MAX_LENGTH`](#config_data_max_length)
- [`QUEUE_EVENT`](#queue_event)
- [`QUEUE_SIZE`](#queue_size)
- [`QUEUE_STATUS`](#queue_status)
- [`SUPPORTED_FEATURES`](#supported_features)

---

## virtio_drivers::device::input::AbsInfo

*Struct*

Information about an axis of an input device, typically a joystick.

**Fields:**
- `min: u32` - The minimum value for the axis.
- `max: u32` - The maximum value for the axis.
- `fuzz: u32` - The fuzz value used to filter noise from the event stream.
- `flat: u32` - The size of the dead zone; values less than this will be reported as 0.
- `res: u32` - The resolution for values reported for the axis.

**Traits:** TryFromBytes, Immutable, KnownLayout, Eq, FromBytes, FromZeros, IntoBytes

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &AbsInfo) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> AbsInfo`
- **Clone**
  - `fn clone(self: &Self) -> AbsInfo`



## virtio_drivers::device::input::CONFIG_DATA_MAX_LENGTH

*Constant*: `usize`



## virtio_drivers::device::input::Config

*Struct*

**Fields:**
- `select: crate::config::WriteOnly<u8>`
- `subsel: crate::config::WriteOnly<u8>`
- `size: crate::config::ReadOnly<u8>`
- `_reserved: [crate::config::ReadOnly<u8>; 5]`
- `data: [crate::config::ReadOnly<u8>; 128]`

**Traits:** FromZeros, IntoBytes, TryFromBytes, Immutable, FromBytes



## virtio_drivers::device::input::DevIDs

*Struct*

The identifiers of a VirtIO input device.

**Fields:**
- `bustype: u16` - The bustype identifier.
- `vendor: u16` - The vendor identifier.
- `product: u16` - The product identifier.
- `version: u16` - The version identifier.

**Traits:** FromZeros, IntoBytes, TryFromBytes, Immutable, KnownLayout, Eq, FromBytes

**Trait Implementations:**

- **Default**
  - `fn default() -> DevIDs`
- **Clone**
  - `fn clone(self: &Self) -> DevIDs`
- **PartialEq**
  - `fn eq(self: &Self, other: &DevIDs) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## virtio_drivers::device::input::InputConfigSelect

*Enum*

Select value used for [`VirtIOInput::query_config_select()`].

**Variants:**
- `IdName` - Returns the name of the device, in u.string. subsel is zero.
- `IdSerial` - Returns the serial number of the device, in u.string. subsel is zero.
- `IdDevids` - Returns ID information of the device, in u.ids. subsel is zero.
- `PropBits` - Returns input properties of the device, in u.bitmap. subsel is zero.
- `EvBits` - subsel specifies the event type using EV_* constants in the underlying
- `AbsInfo` - subsel specifies the absolute axis using ABS_* constants in the underlying

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> InputConfigSelect`



## virtio_drivers::device::input::InputEvent

*Struct*

Both queues use the same `virtio_input_event` struct. `type`, `code` and `value`
are filled according to the Linux input layer (evdev) interface.

**Fields:**
- `event_type: u16` - Event type.
- `code: u16` - Event code.
- `value: u32` - Event value.

**Traits:** TryFromBytes, Immutable, KnownLayout, FromBytes, Copy, FromZeros, IntoBytes

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> InputEvent`
- **Clone**
  - `fn clone(self: &Self) -> InputEvent`



## virtio_drivers::device::input::QUEUE_EVENT

*Constant*: `u16`



## virtio_drivers::device::input::QUEUE_SIZE

*Constant*: `usize`



## virtio_drivers::device::input::QUEUE_STATUS

*Constant*: `u16`



## virtio_drivers::device::input::SUPPORTED_FEATURES

*Constant*: `super::common::Feature`



## virtio_drivers::device::input::VirtIOInput

*Struct*

Virtual human interface devices such as keyboards, mice and tablets.

An instance of the virtio device represents one such input device.
Device behavior mirrors that of the evdev layer in Linux,
making pass-through implementations on top of evdev easy.

**Generic Parameters:**
- H
- T

**Fields:**
- `transport: T`
- `event_queue: crate::queue::VirtQueue<H, QUEUE_SIZE>`
- `status_queue: crate::queue::VirtQueue<H, QUEUE_SIZE>`
- `event_buf: alloc::boxed::Box<[InputEvent; 32]>`

**Methods:**

- `fn new(transport: T) -> Result<Self, Error>` - Create a new VirtIO-Input driver.
- `fn ack_interrupt(self: & mut Self) -> InterruptStatus` - Acknowledge interrupt and process events.
- `fn pop_pending_event(self: & mut Self) -> Option<InputEvent>` - Pop the pending event.
- `fn query_config_select(self: & mut Self, select: InputConfigSelect, subsel: u8, out: & mut [u8]) -> Result<u8, Error>` - Query a specific piece of information by `select` and `subsel`, and write
- `fn query_config_select_alloc(self: & mut Self, select: InputConfigSelect, subsel: u8) -> Result<Box<[u8]>, Error>` - Queries a specific piece of information by `select` and `subsel`, allocates a sufficiently
- `fn query_config_string(self: & mut Self, select: InputConfigSelect, subsel: u8) -> Result<String, Error>` - Queries a specific piece of information by `select` and `subsel` into a newly-allocated
- `fn name(self: & mut Self) -> Result<String, Error>` - Queries and returns the name of the device, or an error if it is not valid UTF-8.
- `fn serial_number(self: & mut Self) -> Result<String, Error>` - Queries and returns the serial number of the device, or an error if it is not valid UTF-8.
- `fn ids(self: & mut Self) -> Result<DevIDs, Error>` - Queries and returns the ID information of the device.
- `fn prop_bits(self: & mut Self) -> Result<Box<[u8]>, Error>` - Queries and returns the input properties of the device.
- `fn ev_bits(self: & mut Self, event_type: u8) -> Result<Box<[u8]>, Error>` - Queries and returns a bitmap of supported event codes for the given event type.
- `fn abs_info(self: & mut Self, axis: u8) -> Result<AbsInfo, Error>` - Queries and returns information about the given axis of the device.

**Traits:** Sync, Send

**Trait Implementations:**

- **Drop**
  - `fn drop(self: & mut Self)`



