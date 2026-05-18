**sel4_simple_task_runtime_config_types > zerocopy_helpers**

# Module: zerocopy_helpers

## Contents

**Structs**

- [`InvalidZerocopyOptionTag`](#invalidzerocopyoptiontag)
- [`ZerocopyOptionWord`](#zerocopyoptionword)
- [`ZerocopyOptionWordRange`](#zerocopyoptionwordrange)
- [`ZerocopyWordRange`](#zerocopywordrange)

**Type Aliases**

- [`NativeWord`](#nativeword)
- [`ZerocopyWord`](#zerocopyword)

---

## sel4_simple_task_runtime_config_types::zerocopy_helpers::InvalidZerocopyOptionTag

*Struct*

**Fields:**
- `tag: u8`

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> InvalidZerocopyOptionTag`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_simple_task_runtime_config_types::zerocopy_helpers::NativeWord

*Type Alias*: `u64`



## sel4_simple_task_runtime_config_types::zerocopy_helpers::ZerocopyOptionWord

*Struct*

**Traits:** TryFromBytes, Eq, KnownLayout, Immutable, IntoBytes, FromBytes, FromZeros

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &ZerocopyOptionWord) -> bool`
- **From**
  - `fn from(option: Option<&ZerocopyWord>) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> ZerocopyOptionWord`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_simple_task_runtime_config_types::zerocopy_helpers::ZerocopyOptionWordRange

*Struct*

**Traits:** Eq, KnownLayout, Immutable, IntoBytes, FromBytes, FromZeros, TryFromBytes

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &ZerocopyOptionWordRange) -> bool`
- **From**
  - `fn from(option: Option<&ZerocopyWordRange>) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> ZerocopyOptionWordRange`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_simple_task_runtime_config_types::zerocopy_helpers::ZerocopyWord

*Type Alias*: `zerocopy::U64<zerocopy::BigEndian>`



## sel4_simple_task_runtime_config_types::zerocopy_helpers::ZerocopyWordRange

*Struct*

**Traits:** Eq, Immutable, IntoBytes, FromBytes, FromZeros, KnownLayout, TryFromBytes

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &ZerocopyWordRange) -> bool`
- **TryFrom**
  - `fn try_from(range: &Range<T>) -> Result<Self, <Self as >::Error>`
- **Clone**
  - `fn clone(self: &Self) -> ZerocopyWordRange`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



