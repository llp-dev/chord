**sddf_ipc_types > empty_message**

# Module: empty_message

## Contents

**Structs**

- [`EmptyMessage`](#emptymessage)

---

## sddf_ipc_types::empty_message::EmptyMessage

*Struct*

**Tuple Struct**: `()`

**Methods:**

- `fn new() -> Self`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &EmptyMessage) -> bool`
- **MessageWriter**
  - `fn write_message(self: &Self, buf: & mut [MessageRegisterValue]) -> Result<(MessageLabel, usize), <Self as >::Error>`
- **Ord**
  - `fn cmp(self: &Self, other: &EmptyMessage) -> $crate::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &EmptyMessage) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> EmptyMessage`
- **Default**
  - `fn default() -> EmptyMessage`
- **ReadFromMessage**
  - `fn read_from_message(label: MessageLabel, buf: &[MessageRegisterValue]) -> Result<Self, <Self as >::Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



