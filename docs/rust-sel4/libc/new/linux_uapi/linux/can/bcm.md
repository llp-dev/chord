**libc > new > linux_uapi > linux > can > bcm**

# Module: new::linux_uapi::linux::can::bcm

## Contents

**Structs**

- [`bcm_msg_head`](#bcm_msg_head)
- [`bcm_timeval`](#bcm_timeval)

**Constants**

- [`CAN_FD_FRAME`](#can_fd_frame)
- [`RX_ANNOUNCE_RESUME`](#rx_announce_resume)
- [`RX_CHANGED`](#rx_changed)
- [`RX_CHECK_DLC`](#rx_check_dlc)
- [`RX_DELETE`](#rx_delete)
- [`RX_FILTER_ID`](#rx_filter_id)
- [`RX_NO_AUTOTIMER`](#rx_no_autotimer)
- [`RX_READ`](#rx_read)
- [`RX_RTR_FRAME`](#rx_rtr_frame)
- [`RX_SETUP`](#rx_setup)
- [`RX_STATUS`](#rx_status)
- [`RX_TIMEOUT`](#rx_timeout)
- [`SETTIMER`](#settimer)
- [`STARTTIMER`](#starttimer)
- [`TX_ANNOUNCE`](#tx_announce)
- [`TX_COUNTEVT`](#tx_countevt)
- [`TX_CP_CAN_ID`](#tx_cp_can_id)
- [`TX_DELETE`](#tx_delete)
- [`TX_EXPIRED`](#tx_expired)
- [`TX_READ`](#tx_read)
- [`TX_RESET_MULTI_IDX`](#tx_reset_multi_idx)
- [`TX_SEND`](#tx_send)
- [`TX_SETUP`](#tx_setup)
- [`TX_STATUS`](#tx_status)

---

## libc::new::linux_uapi::linux::can::bcm::CAN_FD_FRAME

*Constant*: `u32`



## libc::new::linux_uapi::linux::can::bcm::RX_ANNOUNCE_RESUME

*Constant*: `u32`



## libc::new::linux_uapi::linux::can::bcm::RX_CHANGED

*Constant*: `u32`



## libc::new::linux_uapi::linux::can::bcm::RX_CHECK_DLC

*Constant*: `u32`



## libc::new::linux_uapi::linux::can::bcm::RX_DELETE

*Constant*: `u32`



## libc::new::linux_uapi::linux::can::bcm::RX_FILTER_ID

*Constant*: `u32`



## libc::new::linux_uapi::linux::can::bcm::RX_NO_AUTOTIMER

*Constant*: `u32`



## libc::new::linux_uapi::linux::can::bcm::RX_READ

*Constant*: `u32`



## libc::new::linux_uapi::linux::can::bcm::RX_RTR_FRAME

*Constant*: `u32`



## libc::new::linux_uapi::linux::can::bcm::RX_SETUP

*Constant*: `u32`



## libc::new::linux_uapi::linux::can::bcm::RX_STATUS

*Constant*: `u32`



## libc::new::linux_uapi::linux::can::bcm::RX_TIMEOUT

*Constant*: `u32`



## libc::new::linux_uapi::linux::can::bcm::SETTIMER

*Constant*: `u32`



## libc::new::linux_uapi::linux::can::bcm::STARTTIMER

*Constant*: `u32`



## libc::new::linux_uapi::linux::can::bcm::TX_ANNOUNCE

*Constant*: `u32`



## libc::new::linux_uapi::linux::can::bcm::TX_COUNTEVT

*Constant*: `u32`



## libc::new::linux_uapi::linux::can::bcm::TX_CP_CAN_ID

*Constant*: `u32`



## libc::new::linux_uapi::linux::can::bcm::TX_DELETE

*Constant*: `u32`



## libc::new::linux_uapi::linux::can::bcm::TX_EXPIRED

*Constant*: `u32`



## libc::new::linux_uapi::linux::can::bcm::TX_READ

*Constant*: `u32`



## libc::new::linux_uapi::linux::can::bcm::TX_RESET_MULTI_IDX

*Constant*: `u32`



## libc::new::linux_uapi::linux::can::bcm::TX_SEND

*Constant*: `u32`



## libc::new::linux_uapi::linux::can::bcm::TX_SETUP

*Constant*: `u32`



## libc::new::linux_uapi::linux::can::bcm::TX_STATUS

*Constant*: `u32`



## libc::new::linux_uapi::linux::can::bcm::bcm_msg_head

*Struct*

**Fields:**
- `opcode: u32`
- `flags: u32`
- `count: u32`
- `ival1: bcm_timeval`
- `ival2: bcm_timeval`
- `can_id: canid_t`
- `nframes: u32`
- `frames: [can_frame; 0]`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> bcm_msg_head`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::new::linux_uapi::linux::can::bcm::bcm_timeval

*Struct*

**Fields:**
- `tv_sec: c_long`
- `tv_usec: c_long`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> bcm_timeval`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



