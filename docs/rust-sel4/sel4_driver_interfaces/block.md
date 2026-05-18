**sel4_driver_interfaces > block**

# Module: block

## Contents

**Structs**

- [`MacAddress`](#macaddress)

**Traits**

- [`GetBlockDeviceLayout`](#getblockdevicelayout)

---

## sel4_driver_interfaces::block::GetBlockDeviceLayout

*Trait*

**Methods:**

- `Error`
- `get_block_size`
- `get_num_blocks`



## sel4_driver_interfaces::block::MacAddress

*Struct*

**Tuple Struct**: `([u8; 6])`

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &MacAddress) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &MacAddress) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &MacAddress) -> bool`
- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`
- **Clone**
  - `fn clone(self: &Self) -> MacAddress`
- **Serialize**
  - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



