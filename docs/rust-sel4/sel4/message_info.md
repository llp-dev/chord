**sel4 > message_info**

# Module: message_info

## Contents

**Structs**

- [`MessageInfo`](#messageinfo) - Corresponds to `seL4_MessageInfo_t`.
- [`MessageInfoBuilder`](#messageinfobuilder) - Helper for constructing [`MessageInfo`].

---

## sel4::message_info::MessageInfo

*Struct*

Corresponds to `seL4_MessageInfo_t`.

**Tuple Struct**: `()`

**Methods:**

- `fn from_inner(inner: sys::seL4_MessageInfo) -> Self`
- `fn into_inner(self: Self) -> sys::seL4_MessageInfo`
- `fn inner(self: &Self) -> &sys::seL4_MessageInfo`
- `fn inner_mut(self: & mut Self) -> & mut sys::seL4_MessageInfo`
- `fn new(label: Word, caps_unwrapped: usize, extra_caps: usize, length: usize) -> Self`
- `fn label(self: &Self) -> Word`
- `fn label_width() -> usize`
- `fn caps_unwrapped(self: &Self) -> usize`
- `fn extra_caps(self: &Self) -> usize`
- `fn length(self: &Self) -> usize`

**Traits:** Eq

**Trait Implementations:**

- **From**
  - `fn from(builder: MessageInfoBuilder) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> MessageInfo`
- **PartialEq**
  - `fn eq(self: &Self, other: &MessageInfo) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4::message_info::MessageInfoBuilder

*Struct*

Helper for constructing [`MessageInfo`].

**Methods:**

- `fn build(self: Self) -> MessageInfo`
- `fn label(self: Self, label: Word) -> Self`
- `fn caps_unwrapped(self: Self, caps_unwrapped: usize) -> Self`
- `fn extra_caps(self: Self, extra_caps: usize) -> Self`
- `fn length(self: Self, length: usize) -> Self`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> MessageInfoBuilder`
- **PartialEq**
  - `fn eq(self: &Self, other: &MessageInfoBuilder) -> bool`
- **Default**
  - `fn default() -> MessageInfoBuilder`



