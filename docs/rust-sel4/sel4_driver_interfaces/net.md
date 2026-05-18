**sel4_driver_interfaces > net**

# Module: net

## Contents

**Structs**

- [`MacAddress`](#macaddress)

**Traits**

- [`GetNetDeviceMeta`](#getnetdevicemeta)

---

## sel4_driver_interfaces::net::GetNetDeviceMeta

*Trait*

**Methods:**

- `Error`
- `get_mac_address`



## sel4_driver_interfaces::net::MacAddress

*Struct*

**Tuple Struct**: `([u8; 6])`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`
- **Serialize**
  - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &MacAddress) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Ord**
  - `fn cmp(self: &Self, other: &MacAddress) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &MacAddress) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> MacAddress`



