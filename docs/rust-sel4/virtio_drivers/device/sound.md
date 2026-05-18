**virtio_drivers > device > sound**

# Module: device::sound

## Contents

**Structs**

- [`JackFeatures`](#jackfeatures)
- [`Notification`](#notification) - Notification from sound device.
- [`PcmFeatures`](#pcmfeatures) - Supported PCM stream features.
- [`PcmFormats`](#pcmformats) - Supported PCM sample formats.
- [`PcmParameters`](#pcmparameters)
- [`PcmRates`](#pcmrates) - Supported PCM frame rates.
- [`VirtIOSndChmapInfo`](#virtiosndchmapinfo)
- [`VirtIOSndEvent`](#virtiosndevent) - An event notification
- [`VirtIOSndHdr`](#virtiosndhdr) - A common header
- [`VirtIOSndInfo`](#virtiosndinfo) - Field `hda_fn_nid` indicates a function group node identifier.
- [`VirtIOSndJackHdr`](#virtiosndjackhdr)
- [`VirtIOSndJackInfo`](#virtiosndjackinfo) - Jack information.
- [`VirtIOSndJackInfoRsp`](#virtiosndjackinforsp)
- [`VirtIOSndJackRemap`](#virtiosndjackremap)
- [`VirtIOSndPcmHdr`](#virtiosndpcmhdr)
- [`VirtIOSndPcmInfo`](#virtiosndpcminfo) - PCM information.
- [`VirtIOSndPcmSetParams`](#virtiosndpcmsetparams)
- [`VirtIOSndPcmStatus`](#virtiosndpcmstatus) - An I/O status
- [`VirtIOSndPcmXfer`](#virtiosndpcmxfer) - An I/O header
- [`VirtIOSndQueryInfo`](#virtiosndqueryinfo)
- [`VirtIOSndQueryInfoRsp`](#virtiosndqueryinforsp)
- [`VirtIOSound`](#virtiosound) - Audio driver based on virtio v1.2.
- [`VirtIOSoundConfig`](#virtiosoundconfig)

**Enums**

- [`ChannelPosition`](#channelposition)
- [`CommandCode`](#commandcode) - In virtIO v1.2, this enum should be called "Code".
- [`ItemInformationRequestType`](#iteminformationrequesttype) - Enum representing the types of item information requests.
- [`NotificationType`](#notificationtype) - The notification type.
- [`PCMState`](#pcmstate) - The status of the PCM stream.
- [`PcmFormat`](#pcmformat) - A single PCM sample format.
- [`PcmRate`](#pcmrate) - A PCM frame rate.
- [`PcmSampleFormat`](#pcmsampleformat)
- [`PcmStreamFeatures`](#pcmstreamfeatures)
- [`RequestStatusCode`](#requeststatuscode)

**Constants**

- [`CONTROL_QUEUE_IDX`](#control_queue_idx)
- [`EVENT_QUEUE_IDX`](#event_queue_idx)
- [`QUEUE_SIZE`](#queue_size)
- [`RX_QUEUE_IDX`](#rx_queue_idx)
- [`SUPPORTED_FEATURES`](#supported_features)
- [`TX_QUEUE_IDX`](#tx_queue_idx)
- [`VIRTIO_SND_CHMAP_MAX_SIZE`](#virtio_snd_chmap_max_size) - maximum possible number of channels
- [`VIRTIO_SND_D_INPUT`](#virtio_snd_d_input)
- [`VIRTIO_SND_D_OUTPUT`](#virtio_snd_d_output)

---

## virtio_drivers::device::sound::CONTROL_QUEUE_IDX

*Constant*: `u16`



## virtio_drivers::device::sound::ChannelPosition

*Enum*

**Variants:**
- `None` - undefined
- `Na` - silent
- `Mono` - mono stream
- `Fl` - front left
- `Fr` - front right
- `Rl` - rear left
- `Rr` - rear right
- `Fc` - front center
- `Lfe` - low frequency (LFE)
- `Sl` - side left
- `Sr` - side right
- `Rc` - rear center
- `Flc` - front left center
- `Frc` - front right center
- `Rlc` - rear left center
- `Rrc` - rear right center
- `Flw` - front left wide
- `Frw` - front right wide
- `Flh` - front left high
- `Fch` - front center high
- `Frh` - front right high
- `Tc` - top center
- `Tfl` - top front left
- `Tfr` - top front right
- `Tfc` - top front center
- `Trl` - top rear left
- `Trr` - top rear right
- `Trc` - top rear center
- `Tflc` - top front left center
- `Tfrc` - top front right center
- `Tsl` - top side left
- `Tsr` - top side right
- `Llfe` - left LFE
- `Rlfe` - right LFE
- `Bc` - bottom center
- `Blc` - bottom left center
- `Brc` - bottom right center

**Methods:**

- `fn n(value: u8) -> Option<Self>`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ChannelPosition`
- **PartialEq**
  - `fn eq(self: &Self, other: &ChannelPosition) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## virtio_drivers::device::sound::CommandCode

*Enum*

In virtIO v1.2, this enum should be called "Code".

To avoid ambiguity in its meaning, I use the term "CommandCode" here.

**Variants:**
- `RJackInfo`
- `RJackRemap`
- `RPcmInfo`
- `RPcmSetParams`
- `RPcmPrepare`
- `RPcmRelease`
- `RPcmStart`
- `RPcmStop`
- `RChmapInfo`
- `EvtJackConnected`
- `EvtJackDisconnected`
- `EvtPcmPeriodElapsed`
- `EvtPcmXrun`
- `SOk` - success
- `SBadMsg` - a control message is malformed or contains invalid parameters
- `SNotSupp` - requested operation or parameters are not supported
- `SIoErr` - an I/O error occurred

**Methods:**

- `fn n(value: u32) -> Option<Self>`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> CommandCode`
- **PartialEq**
  - `fn eq(self: &Self, other: &CommandCode) -> bool`



## virtio_drivers::device::sound::EVENT_QUEUE_IDX

*Constant*: `u16`



## virtio_drivers::device::sound::ItemInformationRequestType

*Enum*

Enum representing the types of item information requests.

**Variants:**
- `RJackInfo` - Represents a jack information request.
- `RPcmInfo` - Represents a PCM information request.
- `RChmapInfo` - Represents a channel map information request.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ItemInformationRequestType`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &ItemInformationRequestType) -> bool`



## virtio_drivers::device::sound::JackFeatures

*Struct*

**Tuple Struct**: `(<JackFeatures as $crate::__private::PublicFlags>::Internal)`

**Methods:**

- `fn empty() -> Self` - Get a flags value with all bits unset.
- `fn all() -> Self` - Get a flags value with all known bits set.
- `fn bits(self: &Self) -> u32` - Get the underlying bits value.
- `fn from_bits(bits: u32) -> $crate::__private::core::option::Option<Self>` - Convert from a bits value.
- `fn from_bits_truncate(bits: u32) -> Self` - Convert from a bits value, unsetting any unknown bits.
- `fn from_bits_retain(bits: u32) -> Self` - Convert from a bits value exactly.
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
- `fn iter(self: &Self) -> $crate::iter::Iter<JackFeatures>` - Yield a set of contained flags values.
- `fn iter_names(self: &Self) -> $crate::iter::IterNames<JackFeatures>` - Yield a set of contained named flags values.

**Traits:** Eq, Copy, PublicFlags

**Trait Implementations:**

- **Binary**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **BitAnd**
  - `fn bitand(self: Self, other: Self) -> Self` - The bitwise and (`&`) of the bits in two flags values.
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: JackFeatures) -> Self` - The bitwise or (`|`) of the bits in two flags values.
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
  - `fn eq(self: &Self, other: &JackFeatures) -> bool`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: Self)` - The bitwise or (`|`) of the bits in two flags values.
- **BitXor**
  - `fn bitxor(self: Self, other: Self) -> Self` - The bitwise exclusive-or (`^`) of the bits in two flags values.
- **FromIterator**
  - `fn from_iter<T>(iterator: T) -> Self` - The bitwise or (`|`) of the bits in each flags value.
- **Default**
  - `fn default() -> JackFeatures`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: Self)` - The intersection of a source flags value with the complement of a target flags value (`&!`).
- **Not**
  - `fn not(self: Self) -> Self` - The bitwise negation (`!`) of the bits in a flags value, truncating the result.
- **Flags**
  - `fn bits(self: &Self) -> u32`
  - `fn from_bits_retain(bits: u32) -> JackFeatures`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: Self)` - The bitwise exclusive-or (`^`) of the bits in two flags values.
- **Clone**
  - `fn clone(self: &Self) -> JackFeatures`



## virtio_drivers::device::sound::Notification

*Struct*

Notification from sound device.

**Fields:**
- `notification_type: NotificationType`
- `data: u32`

**Methods:**

- `fn data(self: &Self) -> u32` - Get the resource index.
- `fn notification_type(self: &Self) -> NotificationType` - Get the notification type.

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Notification`
- **PartialEq**
  - `fn eq(self: &Self, other: &Notification) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## virtio_drivers::device::sound::NotificationType

*Enum*

The notification type.

**Variants:**
- `JackConnected` - An external device has been connected to the jack.
- `JackDisconnected` - An external device has been disconnected from the jack.
- `PcmPeriodElapsed` - A hardware buffer period has elapsed, the period size is controlled using the `period_bytes` field.
- `PcmXrun` - An underflow for the output stream or an overflow for the inputstream has occurred.

**Methods:**

- `fn n(value: u32) -> Option<Self>` - Converts the given value to a variant of this enum, if any matches.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> NotificationType`
- **PartialEq**
  - `fn eq(self: &Self, other: &NotificationType) -> bool`



## virtio_drivers::device::sound::PCMState

*Enum*

The status of the PCM stream.

**Variants:**
- `SetParams`
- `Prepare`
- `Release`
- `Start`
- `Stop`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> PCMState`
- **Clone**
  - `fn clone(self: &Self) -> PCMState`
- **PartialEq**
  - `fn eq(self: &Self, other: &PCMState) -> bool`



## virtio_drivers::device::sound::PcmFeatures

*Struct*

Supported PCM stream features.

**Tuple Struct**: `(<PcmFeatures as $crate::__private::PublicFlags>::Internal)`

**Methods:**

- `fn empty() -> Self` - Get a flags value with all bits unset.
- `fn all() -> Self` - Get a flags value with all known bits set.
- `fn bits(self: &Self) -> u32` - Get the underlying bits value.
- `fn from_bits(bits: u32) -> $crate::__private::core::option::Option<Self>` - Convert from a bits value.
- `fn from_bits_truncate(bits: u32) -> Self` - Convert from a bits value, unsetting any unknown bits.
- `fn from_bits_retain(bits: u32) -> Self` - Convert from a bits value exactly.
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
- `fn iter(self: &Self) -> $crate::iter::Iter<PcmFeatures>` - Yield a set of contained flags values.
- `fn iter_names(self: &Self) -> $crate::iter::IterNames<PcmFeatures>` - Yield a set of contained named flags values.

**Traits:** Eq, Copy, PublicFlags

**Trait Implementations:**

- **Sub**
  - `fn sub(self: Self, other: Self) -> Self` - The intersection of a source flags value with the complement of a target flags value (`&!`).
- **PartialEq**
  - `fn eq(self: &Self, other: &PcmFeatures) -> bool`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: Self)` - The bitwise or (`|`) of the bits in two flags values.
- **BitXor**
  - `fn bitxor(self: Self, other: Self) -> Self` - The bitwise exclusive-or (`^`) of the bits in two flags values.
- **FromIterator**
  - `fn from_iter<T>(iterator: T) -> Self` - The bitwise or (`|`) of the bits in each flags value.
- **Default**
  - `fn default() -> PcmFeatures`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: Self)` - The intersection of a source flags value with the complement of a target flags value (`&!`).
- **Not**
  - `fn not(self: Self) -> Self` - The bitwise negation (`!`) of the bits in a flags value, truncating the result.
- **Flags**
  - `fn bits(self: &Self) -> u32`
  - `fn from_bits_retain(bits: u32) -> PcmFeatures`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: Self)` - The bitwise exclusive-or (`^`) of the bits in two flags values.
- **Clone**
  - `fn clone(self: &Self) -> PcmFeatures`
- **Binary**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **BitAnd**
  - `fn bitand(self: Self, other: Self) -> Self` - The bitwise and (`&`) of the bits in two flags values.
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: PcmFeatures) -> Self` - The bitwise or (`|`) of the bits in two flags values.
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



## virtio_drivers::device::sound::PcmFormat

*Enum*

A single PCM sample format.

**Variants:**
- `ImaAdpcm` - IMA ADPCM format.
- `MuLaw` - Mu-law format.
- `ALaw` - A-law format.
- `S8` - Signed 8-bit format.
- `U8` - Unsigned 8-bit format.
- `S16` - Signed 16-bit format.
- `U16` - Unsigned 16-bit format.
- `S18_3` - Signed 18.3-bit format.
- `U18_3` - Unsigned 18.3-bit format.
- `S20_3` - Signed 20.3-bit format.
- `U20_3` - Unsigned 20.3-bit format.
- `S24_3` - Signed 24.3-bit format.
- `U24_3` - Unsigned 24.3-bit format.
- `S20` - Signed 20-bit format.
- `U20` - Unsigned 20-bit format.
- `S24` - Signed 24-bit format.
- `U24` - Unsigned 24-bit format.
- `S32` - Signed 32-bit format.
- `U32` - Unsigned 32-bit format.
- `FLOAT` - 32-bit floating-point format.
- `FLOAT64` - 64-bit floating-point format.
- `DsdU8` - DSD unsigned 8-bit format.
- `DsdU16` - DSD unsigned 16-bit format.
- `DsdU32` - DSD unsigned 32-bit format.
- `Iec958Subframe` - IEC958 subframe format.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &PcmFormat) -> bool`
- **Default**
  - `fn default() -> PcmFormat`
- **Clone**
  - `fn clone(self: &Self) -> PcmFormat`



## virtio_drivers::device::sound::PcmFormats

*Struct*

Supported PCM sample formats.

**Tuple Struct**: `(<PcmFormats as $crate::__private::PublicFlags>::Internal)`

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
- `fn iter(self: &Self) -> $crate::iter::Iter<PcmFormats>` - Yield a set of contained flags values.
- `fn iter_names(self: &Self) -> $crate::iter::IterNames<PcmFormats>` - Yield a set of contained named flags values.

**Traits:** Eq, Copy, PublicFlags

**Trait Implementations:**

- **UpperHex**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: PcmFormats) -> Self` - The bitwise or (`|`) of the bits in two flags values.
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
  - `fn eq(self: &Self, other: &PcmFormats) -> bool`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: Self)` - The bitwise or (`|`) of the bits in two flags values.
- **BitXor**
  - `fn bitxor(self: Self, other: Self) -> Self` - The bitwise exclusive-or (`^`) of the bits in two flags values.
- **FromIterator**
  - `fn from_iter<T>(iterator: T) -> Self` - The bitwise or (`|`) of the bits in each flags value.
- **From**
  - `fn from(format: PcmFormat) -> Self`
- **Default**
  - `fn default() -> PcmFormats`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: Self)` - The intersection of a source flags value with the complement of a target flags value (`&!`).
- **Not**
  - `fn not(self: Self) -> Self` - The bitwise negation (`!`) of the bits in a flags value, truncating the result.
- **Flags**
  - `fn bits(self: &Self) -> u64`
  - `fn from_bits_retain(bits: u64) -> PcmFormats`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: Self)` - The bitwise exclusive-or (`^`) of the bits in two flags values.
- **Clone**
  - `fn clone(self: &Self) -> PcmFormats`
- **Binary**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **BitAnd**
  - `fn bitand(self: Self, other: Self) -> Self` - The bitwise and (`&`) of the bits in two flags values.



## virtio_drivers::device::sound::PcmParameters

*Struct*

**Fields:**
- `setup: bool`
- `buffer_bytes: u32`
- `period_bytes: u32`
- `features: PcmFeatures`
- `channels: u8`
- `format: PcmFormat`
- `rate: PcmRate`

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> PcmParameters`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &PcmParameters) -> bool`
- **Default**
  - `fn default() -> PcmParameters`



## virtio_drivers::device::sound::PcmRate

*Enum*

A PCM frame rate.

**Variants:**
- `Rate5512` - 5512 Hz PCM rate.
- `Rate8000` - 8000 Hz PCM rate.
- `Rate11025` - 11025 Hz PCM rate.
- `Rate16000` - 16000 Hz PCM rate.
- `Rate22050` - 22050 Hz PCM rate.
- `Rate32000` - 32000 Hz PCM rate.
- `Rate44100` - 44100 Hz PCM rate.
- `Rate48000` - 48000 Hz PCM rate.
- `Rate64000` - 64000 Hz PCM rate.
- `Rate88200` - 88200 Hz PCM rate.
- `Rate96000` - 96000 Hz PCM rate.
- `Rate176400` - 176400 Hz PCM rate.
- `Rate192000` - 192000 Hz PCM rate.
- `Rate384000` - 384000 Hz PCM rate.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Default**
  - `fn default() -> PcmRate`
- **Clone**
  - `fn clone(self: &Self) -> PcmRate`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &PcmRate) -> bool`



## virtio_drivers::device::sound::PcmRates

*Struct*

Supported PCM frame rates.

**Tuple Struct**: `(<PcmRates as $crate::__private::PublicFlags>::Internal)`

**Methods:**

- `fn iter(self: &Self) -> $crate::iter::Iter<PcmRates>` - Yield a set of contained flags values.
- `fn iter_names(self: &Self) -> $crate::iter::IterNames<PcmRates>` - Yield a set of contained named flags values.
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

**Traits:** Eq, Copy, PublicFlags

**Trait Implementations:**

- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: Self)` - The bitwise exclusive-or (`^`) of the bits in two flags values.
- **Clone**
  - `fn clone(self: &Self) -> PcmRates`
- **Binary**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **BitAnd**
  - `fn bitand(self: Self, other: Self) -> Self` - The bitwise and (`&`) of the bits in two flags values.
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **BitOr**
  - `fn bitor(self: Self, other: PcmRates) -> Self` - The bitwise or (`|`) of the bits in two flags values.
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
  - `fn eq(self: &Self, other: &PcmRates) -> bool`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: Self)` - The bitwise or (`|`) of the bits in two flags values.
- **BitXor**
  - `fn bitxor(self: Self, other: Self) -> Self` - The bitwise exclusive-or (`^`) of the bits in two flags values.
- **FromIterator**
  - `fn from_iter<T>(iterator: T) -> Self` - The bitwise or (`|`) of the bits in each flags value.
- **From**
  - `fn from(rate: PcmRate) -> Self`
- **Default**
  - `fn default() -> PcmRates`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: Self)` - The intersection of a source flags value with the complement of a target flags value (`&!`).
- **Not**
  - `fn not(self: Self) -> Self` - The bitwise negation (`!`) of the bits in a flags value, truncating the result.
- **Flags**
  - `fn bits(self: &Self) -> u64`
  - `fn from_bits_retain(bits: u64) -> PcmRates`



## virtio_drivers::device::sound::PcmSampleFormat

*Enum*

**Variants:**
- `ImaAdpcm`
- `MuLaw`
- `ALaw`
- `S8`
- `U8`
- `S16`
- `U16`
- `S18_3`
- `U18_3`
- `S20_3`
- `U20_3`
- `S24_3`
- `U24_3`
- `S20`
- `U20`
- `S24`
- `U24`
- `S32`
- `U32`
- `Float`
- `Float64`
- `DsdU8`
- `DsdU16`
- `DsdU32`
- `Iec958Subframe`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> PcmSampleFormat`
- **PartialEq**
  - `fn eq(self: &Self, other: &PcmSampleFormat) -> bool`



## virtio_drivers::device::sound::PcmStreamFeatures

*Enum*

**Variants:**
- `ShmemHost`
- `ShmemGuest`
- `MsgPolling`
- `EvtShmemPeriods`
- `EvtXruns`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> PcmStreamFeatures`
- **PartialEq**
  - `fn eq(self: &Self, other: &PcmStreamFeatures) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## virtio_drivers::device::sound::QUEUE_SIZE

*Constant*: `u16`



## virtio_drivers::device::sound::RX_QUEUE_IDX

*Constant*: `u16`



## virtio_drivers::device::sound::RequestStatusCode

*Enum*

**Variants:**
- `Ok`
- `BadMsg`
- `NotSupp`
- `IoErr`

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &RequestStatusCode) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> RequestStatusCode`



## virtio_drivers::device::sound::SUPPORTED_FEATURES

*Constant*: `super::common::Feature`



## virtio_drivers::device::sound::TX_QUEUE_IDX

*Constant*: `u16`



## virtio_drivers::device::sound::VIRTIO_SND_CHMAP_MAX_SIZE

*Constant*: `usize`

maximum possible number of channels



## virtio_drivers::device::sound::VIRTIO_SND_D_INPUT

*Constant*: `u8`



## virtio_drivers::device::sound::VIRTIO_SND_D_OUTPUT

*Constant*: `u8`



## virtio_drivers::device::sound::VirtIOSndChmapInfo

*Struct*

**Fields:**
- `hdr: VirtIOSndInfo`
- `direction: u8`
- `channels: u8`
- `positions: [u8; 18]`

**Traits:** TryFromBytes, Immutable, KnownLayout, FromBytes, FromZeros, IntoBytes

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> VirtIOSndChmapInfo`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`



## virtio_drivers::device::sound::VirtIOSndEvent

*Struct*

An event notification

**Fields:**
- `hdr: VirtIOSndHdr`
- `data: u32`

**Traits:** Immutable, FromBytes, KnownLayout, FromZeros, TryFromBytes



## virtio_drivers::device::sound::VirtIOSndHdr

*Struct*

A common header

**Fields:**
- `command_code: u32`

**Traits:** FromZeros, IntoBytes, TryFromBytes, Immutable, KnownLayout, Eq, FromBytes

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(value: CommandCode) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &VirtIOSndHdr) -> bool`
- **From**
  - `fn from(value: RequestStatusCode) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> VirtIOSndHdr`
- **From**
  - `fn from(value: ItemInformationRequestType) -> Self`



## virtio_drivers::device::sound::VirtIOSndInfo

*Struct*

Field `hda_fn_nid` indicates a function group node identifier.

**Fields:**
- `hda_fn_nid: u32`

**Traits:** Immutable, KnownLayout, Eq, FromBytes, FromZeros, IntoBytes, TryFromBytes

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &VirtIOSndInfo) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> VirtIOSndInfo`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## virtio_drivers::device::sound::VirtIOSndJackHdr

*Struct*

**Fields:**
- `hdr: VirtIOSndHdr`
- `jack_id: u32` - specifies a jack identifier from 0 to jacks - 1

**Traits:** FromZeros, IntoBytes, TryFromBytes, Immutable, KnownLayout, FromBytes

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> VirtIOSndJackHdr`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## virtio_drivers::device::sound::VirtIOSndJackInfo

*Struct*

Jack information.

**Fields:**
- `hdr: VirtIOSndInfo`
- `features: u32`
- `hda_reg_defconf: u32` - indicates a pin default configuration value
- `hda_reg_caps: u32` - indicates a pin capabilities value
- `connected: u8` - indicates the current jack connection status (1 - connected, 0 - disconnected)
- `_padding: [u8; 7]`

**Traits:** TryFromBytes, Immutable, KnownLayout, Eq, FromBytes, FromZeros, IntoBytes

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &VirtIOSndJackInfo) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> VirtIOSndJackInfo`



## virtio_drivers::device::sound::VirtIOSndJackInfoRsp

*Struct*

**Fields:**
- `hdr: VirtIOSndHdr`
- `body: VirtIOSndJackInfo`

**Traits:** FromZeros, IntoBytes, TryFromBytes, Immutable, KnownLayout, FromBytes



## virtio_drivers::device::sound::VirtIOSndJackRemap

*Struct*

**Fields:**
- `hdr: VirtIOSndJackHdr`
- `association: u32`
- `sequence: u32`

**Traits:** FromZeros, IntoBytes, TryFromBytes, Immutable, KnownLayout, FromBytes



## virtio_drivers::device::sound::VirtIOSndPcmHdr

*Struct*

**Fields:**
- `hdr: VirtIOSndHdr` - specifies request type (VIRTIO_SND_R_PCM_*)
- `stream_id: u32` - specifies a PCM stream identifier from 0 to streams - 1

**Traits:** TryFromBytes, Immutable, KnownLayout, Eq, FromBytes, FromZeros, IntoBytes

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &VirtIOSndPcmHdr) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## virtio_drivers::device::sound::VirtIOSndPcmInfo

*Struct*

PCM information.

**Fields:**
- `hdr: VirtIOSndInfo`
- `features: u32`
- `formats: u64`
- `rates: u64`
- `direction: u8` - indicates the direction of data flow (VIRTIO_SND_D_*)
- `channels_min: u8` - indicates a minimum number of supported channels
- `channels_max: u8` - indicates a maximum number of supported channels
- `_padding: [u8; 5]`

**Traits:** Immutable, KnownLayout, Eq, FromBytes, FromZeros, IntoBytes, TryFromBytes

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &VirtIOSndPcmInfo) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> VirtIOSndPcmInfo`
- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



## virtio_drivers::device::sound::VirtIOSndPcmSetParams

*Struct*

**Fields:**
- `hdr: VirtIOSndPcmHdr`
- `buffer_bytes: u32`
- `period_bytes: u32`
- `features: u32`
- `channels: u8`
- `format: u8`
- `rate: u8`
- `_padding: u8`

**Traits:** Immutable, KnownLayout, Eq, FromBytes, FromZeros, IntoBytes, TryFromBytes

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &VirtIOSndPcmSetParams) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## virtio_drivers::device::sound::VirtIOSndPcmStatus

*Struct*

An I/O status

**Fields:**
- `status: u32`
- `latency_bytes: u32`

**Traits:** KnownLayout, FromBytes, FromZeros, IntoBytes, TryFromBytes, Immutable

**Trait Implementations:**

- **Default**
  - `fn default() -> VirtIOSndPcmStatus`



## virtio_drivers::device::sound::VirtIOSndPcmXfer

*Struct*

An I/O header

**Fields:**
- `stream_id: u32`

**Traits:** FromBytes, FromZeros, IntoBytes, TryFromBytes, Immutable, KnownLayout



## virtio_drivers::device::sound::VirtIOSndQueryInfo

*Struct*

**Fields:**
- `hdr: VirtIOSndHdr` - specifies a particular item request type (VIRTIO_SND_R_*_INFO)
- `start_id: u32` - specifies the starting identifier for the item
- `count: u32` - specifies the number of items for which information is requested
- `size: u32` - specifies the size of the structure containing information for one item

**Traits:** FromZeros, IntoBytes, TryFromBytes, Immutable, KnownLayout, FromBytes

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## virtio_drivers::device::sound::VirtIOSndQueryInfoRsp

*Struct*

**Fields:**
- `hdr: VirtIOSndHdr`
- `info: VirtIOSndInfo`

**Traits:** TryFromBytes, Immutable, KnownLayout, FromBytes, FromZeros, IntoBytes

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## virtio_drivers::device::sound::VirtIOSound

*Struct*

Audio driver based on virtio v1.2.

Supports synchronous blocking and asynchronous non-blocking audio playback.

Currently, only audio playback functionality has been implemented.

**Generic Parameters:**
- H
- T

**Fields:**
- `transport: T`
- `control_queue: crate::queue::VirtQueue<H, { _ }>`
- `event_queue: crate::queue::OwningQueue<H, { _ }, { _ }>`
- `tx_queue: crate::queue::VirtQueue<H, { _ }>`
- `rx_queue: crate::queue::VirtQueue<H, { _ }>`
- `negotiated_features: super::common::Feature`
- `jacks: u32`
- `streams: u32`
- `chmaps: u32`
- `pcm_infos: Option<alloc::vec::Vec<VirtIOSndPcmInfo>>`
- `jack_infos: Option<alloc::vec::Vec<VirtIOSndJackInfo>>`
- `chmap_infos: Option<alloc::vec::Vec<VirtIOSndChmapInfo>>`
- `pcm_parameters: alloc::vec::Vec<PcmParameters>`
- `queue_buf_send: alloc::boxed::Box<[u8]>`
- `queue_buf_recv: alloc::boxed::Box<[u8]>`
- `set_up: bool`
- `token_rsp: alloc::collections::BTreeMap<u16, alloc::boxed::Box<VirtIOSndPcmStatus>>`
- `pcm_states: alloc::vec::Vec<PCMState>`
- `token_buf: alloc::collections::BTreeMap<u16, alloc::vec::Vec<u8>>`

**Methods:**

- `fn new(transport: T) -> Result<Self>` - Create a new VirtIO-Sound driver.
- `fn jacks(self: &Self) -> u32` - Total jack num.
- `fn streams(self: &Self) -> u32` - Total stream num.
- `fn chmaps(self: &Self) -> u32` - Total chmap num.
- `fn ack_interrupt(self: & mut Self) -> InterruptStatus` - Acknowledge interrupt.
- `fn request<Req>(self: & mut Self, req: Req) -> Result<VirtIOSndHdr>`
- `fn set_up(self: & mut Self) -> Result<()>` - Set up the driver, initiate pcm_infos and jacks_infos
- `fn enable_interrupts(self: & mut Self, enable: bool)` - Enables interrupts from the device.
- `fn jack_info(self: & mut Self, jack_start_id: u32, jack_count: u32) -> Result<Vec<VirtIOSndJackInfo>>` - Query information about the available jacks.
- `fn pcm_info(self: & mut Self, stream_start_id: u32, stream_count: u32) -> Result<Vec<VirtIOSndPcmInfo>>` - Query information about the available streams.
- `fn chmap_info(self: & mut Self, chmaps_start_id: u32, chmaps_count: u32) -> Result<Vec<VirtIOSndChmapInfo>>` - Query information about the available chmaps.
- `fn jack_remap(self: & mut Self, jack_id: u32, association: u32, sequence: u32) -> Result` - If the VIRTIO_SND_JACK_F_REMAP feature bit is set in the jack information, then the driver can send a
- `fn pcm_set_params(self: & mut Self, stream_id: u32, buffer_bytes: u32, period_bytes: u32, features: PcmFeatures, channels: u8, format: PcmFormat, rate: PcmRate) -> Result` - Set selected stream parameters for the specified stream ID.
- `fn pcm_prepare(self: & mut Self, stream_id: u32) -> Result` - Prepare a stream with specified stream ID.
- `fn pcm_release(self: & mut Self, stream_id: u32) -> Result` - Release a stream with specified stream ID.
- `fn pcm_start(self: & mut Self, stream_id: u32) -> Result` - Start a stream with specified stream ID.
- `fn pcm_stop(self: & mut Self, stream_id: u32) -> Result` - Stop a stream with specified stream ID.
- `fn pcm_xfer(self: & mut Self, stream_id: u32, frames: &[u8]) -> Result` - Transfer PCM frame to device, based on the stream type(OUTPUT/INPUT).
- `fn pcm_xfer_nb(self: & mut Self, stream_id: u32, frames: &[u8]) -> Result<u16>` - Transfer PCM frame to device, based on the stream type(OUTPUT/INPUT).
- `fn pcm_xfer_ok(self: & mut Self, token: u16) -> Result` - The PCM frame transmission corresponding to the given token has been completed.
- `fn output_streams(self: & mut Self) -> Result<Vec<u32>>` - Get all output streams.
- `fn input_streams(self: & mut Self) -> Result<Vec<u32>>` - Get all input streams.
- `fn rates_supported(self: & mut Self, stream_id: u32) -> Result<PcmRates>` - Get the rates that a stream supports.
- `fn formats_supported(self: & mut Self, stream_id: u32) -> Result<PcmFormats>` - Get the formats that a stream supports.
- `fn channel_range_supported(self: & mut Self, stream_id: u32) -> Result<RangeInclusive<u8>>` - Get channel range that a stream supports.
- `fn features_supported(self: & mut Self, stream_id: u32) -> Result<PcmFeatures>` - Get features that a stream supports.
- `fn latest_notification(self: & mut Self) -> Result<Option<Notification>>` - Get the latest notification.



## virtio_drivers::device::sound::VirtIOSoundConfig

*Struct*

**Fields:**
- `jacks: crate::config::ReadOnly<u32>`
- `streams: crate::config::ReadOnly<u32>`
- `chmaps: crate::config::ReadOnly<u32>`

**Traits:** TryFromBytes, Immutable, FromBytes, FromZeros, IntoBytes



