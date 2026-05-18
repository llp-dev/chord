**libc > new > linux_uapi > linux > can**

# Module: new::linux_uapi::linux::can

## Contents

**Structs**

- [`__c_anonymous_sockaddr_can_j1939`](#__c_anonymous_sockaddr_can_j1939)
- [`__c_anonymous_sockaddr_can_tp`](#__c_anonymous_sockaddr_can_tp)
- [`can_filter`](#can_filter)
- [`can_frame`](#can_frame)
- [`canfd_frame`](#canfd_frame)
- [`canxl_frame`](#canxl_frame)
- [`sockaddr_can`](#sockaddr_can)

**Unions**

- [`__c_anonymous_sockaddr_can_can_addr`](#__c_anonymous_sockaddr_can_can_addr)

**Constants**

- [`CANFD_BRS`](#canfd_brs)
- [`CANFD_ESI`](#canfd_esi)
- [`CANFD_FDF`](#canfd_fdf)
- [`CANFD_MAX_DLC`](#canfd_max_dlc)
- [`CANFD_MAX_DLEN`](#canfd_max_dlen)
- [`CANFD_MTU`](#canfd_mtu)
- [`CANXL_HDR_SIZE`](#canxl_hdr_size)
- [`CANXL_MAX_DLC`](#canxl_max_dlc)
- [`CANXL_MAX_DLC_MASK`](#canxl_max_dlc_mask)
- [`CANXL_MAX_DLEN`](#canxl_max_dlen)
- [`CANXL_MAX_MTU`](#canxl_max_mtu)
- [`CANXL_MIN_DLC`](#canxl_min_dlc)
- [`CANXL_MIN_DLEN`](#canxl_min_dlen)
- [`CANXL_MIN_MTU`](#canxl_min_mtu)
- [`CANXL_MTU`](#canxl_mtu)
- [`CANXL_PRIO_BITS`](#canxl_prio_bits)
- [`CANXL_PRIO_MASK`](#canxl_prio_mask)
- [`CANXL_SEC`](#canxl_sec)
- [`CANXL_XLF`](#canxl_xlf)
- [`CAN_BCM`](#can_bcm)
- [`CAN_EFF_FLAG`](#can_eff_flag)
- [`CAN_EFF_ID_BITS`](#can_eff_id_bits)
- [`CAN_EFF_MASK`](#can_eff_mask)
- [`CAN_ERR_FLAG`](#can_err_flag)
- [`CAN_ERR_MASK`](#can_err_mask)
- [`CAN_INV_FILTER`](#can_inv_filter)
- [`CAN_ISOTP`](#can_isotp)
- [`CAN_J1939`](#can_j1939)
- [`CAN_MAX_DLC`](#can_max_dlc)
- [`CAN_MAX_DLEN`](#can_max_dlen)
- [`CAN_MCNET`](#can_mcnet)
- [`CAN_MTU`](#can_mtu)
- [`CAN_NPROTO`](#can_nproto)
- [`CAN_RAW`](#can_raw)
- [`CAN_RTR_FLAG`](#can_rtr_flag)
- [`CAN_SFF_ID_BITS`](#can_sff_id_bits)
- [`CAN_SFF_MASK`](#can_sff_mask)
- [`CAN_TP16`](#can_tp16)
- [`CAN_TP20`](#can_tp20)
- [`SOL_CAN_BASE`](#sol_can_base)

**Type Aliases**

- [`can_err_mask_t`](#can_err_mask_t)
- [`canid_t`](#canid_t)

---

## libc::new::linux_uapi::linux::can::CANFD_BRS

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::CANFD_ESI

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::CANFD_FDF

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::CANFD_MAX_DLC

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::CANFD_MAX_DLEN

*Constant*: `usize`



## libc::new::linux_uapi::linux::can::CANFD_MTU

*Constant*: `usize`



## libc::new::linux_uapi::linux::can::CANXL_HDR_SIZE

*Constant*: `usize`



## libc::new::linux_uapi::linux::can::CANXL_MAX_DLC

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::CANXL_MAX_DLC_MASK

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::CANXL_MAX_DLEN

*Constant*: `usize`



## libc::new::linux_uapi::linux::can::CANXL_MAX_MTU

*Constant*: `usize`



## libc::new::linux_uapi::linux::can::CANXL_MIN_DLC

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::CANXL_MIN_DLEN

*Constant*: `usize`



## libc::new::linux_uapi::linux::can::CANXL_MIN_MTU

*Constant*: `usize`



## libc::new::linux_uapi::linux::can::CANXL_MTU

*Constant*: `usize`



## libc::new::linux_uapi::linux::can::CANXL_PRIO_BITS

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::CANXL_PRIO_MASK

*Constant*: `crate::canid_t`



## libc::new::linux_uapi::linux::can::CANXL_SEC

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::CANXL_XLF

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::CAN_BCM

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::CAN_EFF_FLAG

*Constant*: `canid_t`



## libc::new::linux_uapi::linux::can::CAN_EFF_ID_BITS

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::CAN_EFF_MASK

*Constant*: `canid_t`



## libc::new::linux_uapi::linux::can::CAN_ERR_FLAG

*Constant*: `canid_t`



## libc::new::linux_uapi::linux::can::CAN_ERR_MASK

*Constant*: `canid_t`



## libc::new::linux_uapi::linux::can::CAN_INV_FILTER

*Constant*: `canid_t`



## libc::new::linux_uapi::linux::can::CAN_ISOTP

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::CAN_J1939

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::CAN_MAX_DLC

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::CAN_MAX_DLEN

*Constant*: `usize`



## libc::new::linux_uapi::linux::can::CAN_MCNET

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::CAN_MTU

*Constant*: `usize`



## libc::new::linux_uapi::linux::can::CAN_NPROTO

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::CAN_RAW

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::CAN_RTR_FLAG

*Constant*: `canid_t`



## libc::new::linux_uapi::linux::can::CAN_SFF_ID_BITS

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::CAN_SFF_MASK

*Constant*: `canid_t`



## libc::new::linux_uapi::linux::can::CAN_TP16

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::CAN_TP20

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::SOL_CAN_BASE

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::__c_anonymous_sockaddr_can_can_addr

*Union*

**Fields:**
- `tp: __c_anonymous_sockaddr_can_tp`
- `j1939: __c_anonymous_sockaddr_can_j1939`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> __c_anonymous_sockaddr_can_can_addr`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`



## libc::new::linux_uapi::linux::can::__c_anonymous_sockaddr_can_j1939

*Struct*

**Fields:**
- `name: u64`
- `pgn: u32`
- `addr: u8`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> __c_anonymous_sockaddr_can_j1939`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::new::linux_uapi::linux::can::__c_anonymous_sockaddr_can_tp

*Struct*

**Fields:**
- `rx_id: canid_t`
- `tx_id: canid_t`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> __c_anonymous_sockaddr_can_tp`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::new::linux_uapi::linux::can::can_err_mask_t

*Type Alias*: `u32`



## libc::new::linux_uapi::linux::can::can_filter

*Struct*

**Fields:**
- `can_id: canid_t`
- `can_mask: canid_t`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> can_filter`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::new::linux_uapi::linux::can::can_frame

*Struct*

**Fields:**
- `can_id: canid_t`
- `can_dlc: u8`
- `len8_dlc: u8`
- `data: [u8; 8]`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> can_frame`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::new::linux_uapi::linux::can::canfd_frame

*Struct*

**Fields:**
- `can_id: canid_t`
- `len: u8`
- `flags: u8`
- `data: [u8; 64]`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> canfd_frame`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::new::linux_uapi::linux::can::canid_t

*Type Alias*: `u32`



## libc::new::linux_uapi::linux::can::canxl_frame

*Struct*

**Fields:**
- `prio: canid_t`
- `flags: u8`
- `sdt: u8`
- `len: u16`
- `af: u32`
- `data: [u8; 2048]`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> canxl_frame`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::new::linux_uapi::linux::can::sockaddr_can

*Struct*

**Fields:**
- `can_family: crate::sa_family_t`
- `can_ifindex: c_int`
- `can_addr: __c_anonymous_sockaddr_can_can_addr`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> sockaddr_can`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



