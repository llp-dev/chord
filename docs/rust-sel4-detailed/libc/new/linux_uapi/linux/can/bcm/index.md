*[libc](../../../../../index.md) / [new](../../../../index.md) / [linux_uapi](../../../index.md) / [linux](../../index.md) / [can](../index.md) / [bcm](index.md)*

---

# Module `bcm`

Header: `linux/can/bcm.h`

## Contents

- [Modules](#modules)
  - [`bcm`](#bcm)
  - [`error`](#error)
  - [`j1939`](#j1939)
  - [`raw`](#raw)
- [Structs](#structs)
  - [`bcm_timeval`](#bcm-timeval)
  - [`bcm_msg_head`](#bcm-msg-head)
  - [`can_frame`](#can-frame)
  - [`canfd_frame`](#canfd-frame)
  - [`canxl_frame`](#canxl-frame)
  - [`sockaddr_can`](#sockaddr-can)
  - [`__c_anonymous_sockaddr_can_tp`](#c-anonymous-sockaddr-can-tp)
  - [`__c_anonymous_sockaddr_can_j1939`](#c-anonymous-sockaddr-can-j1939)
  - [`can_filter`](#can-filter)
- [Type Aliases](#type-aliases)
  - [`canid_t`](#canid-t)
  - [`can_err_mask_t`](#can-err-mask-t)
- [Constants](#constants)
  - [`TX_SETUP`](#tx-setup)
  - [`TX_DELETE`](#tx-delete)
  - [`TX_READ`](#tx-read)
  - [`TX_SEND`](#tx-send)
  - [`RX_SETUP`](#rx-setup)
  - [`RX_DELETE`](#rx-delete)
  - [`RX_READ`](#rx-read)
  - [`TX_STATUS`](#tx-status)
  - [`TX_EXPIRED`](#tx-expired)
  - [`RX_STATUS`](#rx-status)
  - [`RX_TIMEOUT`](#rx-timeout)
  - [`RX_CHANGED`](#rx-changed)
  - [`SETTIMER`](#settimer)
  - [`STARTTIMER`](#starttimer)
  - [`TX_COUNTEVT`](#tx-countevt)
  - [`TX_ANNOUNCE`](#tx-announce)
  - [`TX_CP_CAN_ID`](#tx-cp-can-id)
  - [`RX_FILTER_ID`](#rx-filter-id)
  - [`RX_CHECK_DLC`](#rx-check-dlc)
  - [`RX_NO_AUTOTIMER`](#rx-no-autotimer)
  - [`RX_ANNOUNCE_RESUME`](#rx-announce-resume)
  - [`TX_RESET_MULTI_IDX`](#tx-reset-multi-idx)
  - [`RX_RTR_FRAME`](#rx-rtr-frame)
  - [`CAN_FD_FRAME`](#can-fd-frame)
  - [`CAN_EFF_FLAG`](#can-eff-flag)
  - [`CAN_RTR_FLAG`](#can-rtr-flag)
  - [`CAN_ERR_FLAG`](#can-err-flag)
  - [`CAN_SFF_MASK`](#can-sff-mask)
  - [`CAN_EFF_MASK`](#can-eff-mask)
  - [`CAN_ERR_MASK`](#can-err-mask)
  - [`CANXL_PRIO_MASK`](#canxl-prio-mask)
  - [`CAN_SFF_ID_BITS`](#can-sff-id-bits)
  - [`CAN_EFF_ID_BITS`](#can-eff-id-bits)
  - [`CANXL_PRIO_BITS`](#canxl-prio-bits)
  - [`CAN_MAX_DLC`](#can-max-dlc)
  - [`CAN_MAX_DLEN`](#can-max-dlen)
  - [`CANFD_MAX_DLC`](#canfd-max-dlc)
  - [`CANFD_MAX_DLEN`](#canfd-max-dlen)
  - [`CANXL_MIN_DLC`](#canxl-min-dlc)
  - [`CANXL_MAX_DLC`](#canxl-max-dlc)
  - [`CANXL_MAX_DLC_MASK`](#canxl-max-dlc-mask)
  - [`CANXL_MIN_DLEN`](#canxl-min-dlen)
  - [`CANXL_MAX_DLEN`](#canxl-max-dlen)
  - [`CANFD_BRS`](#canfd-brs)
  - [`CANFD_ESI`](#canfd-esi)
  - [`CANFD_FDF`](#canfd-fdf)
  - [`CANXL_XLF`](#canxl-xlf)
  - [`CANXL_SEC`](#canxl-sec)
  - [`CAN_MTU`](#can-mtu)
  - [`CANFD_MTU`](#canfd-mtu)
  - [`CANXL_MTU`](#canxl-mtu)
  - [`CANXL_HDR_SIZE`](#canxl-hdr-size)
  - [`CANXL_MIN_MTU`](#canxl-min-mtu)
  - [`CANXL_MAX_MTU`](#canxl-max-mtu)
  - [`CAN_RAW`](#can-raw)
  - [`CAN_BCM`](#can-bcm)
  - [`CAN_TP16`](#can-tp16)
  - [`CAN_TP20`](#can-tp20)
  - [`CAN_MCNET`](#can-mcnet)
  - [`CAN_ISOTP`](#can-isotp)
  - [`CAN_J1939`](#can-j1939)
  - [`CAN_NPROTO`](#can-nproto)
  - [`SOL_CAN_BASE`](#sol-can-base)
  - [`CAN_INV_FILTER`](#can-inv-filter)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`bcm`](#bcm) | mod | Header: `linux/can/bcm.h` |
| [`error`](#error) | mod | Header: `linux/can/error.h` |
| [`j1939`](#j1939) | mod | `linux/can/j1939.h` |
| [`raw`](#raw) | mod | Header: `linux/can/raw.h` |
| [`bcm_timeval`](#bcm-timeval) | struct |  |
| [`bcm_msg_head`](#bcm-msg-head) | struct |  |
| [`can_frame`](#can-frame) | struct |  |
| [`canfd_frame`](#canfd-frame) | struct |  |
| [`canxl_frame`](#canxl-frame) | struct |  |
| [`sockaddr_can`](#sockaddr-can) | struct |  |
| [`__c_anonymous_sockaddr_can_tp`](#c-anonymous-sockaddr-can-tp) | struct |  |
| [`__c_anonymous_sockaddr_can_j1939`](#c-anonymous-sockaddr-can-j1939) | struct |  |
| [`can_filter`](#can-filter) | struct |  |
| [`canid_t`](#canid-t) | type |  |
| [`can_err_mask_t`](#can-err-mask-t) | type |  |
| [`TX_SETUP`](#tx-setup) | const |  |
| [`TX_DELETE`](#tx-delete) | const |  |
| [`TX_READ`](#tx-read) | const |  |
| [`TX_SEND`](#tx-send) | const |  |
| [`RX_SETUP`](#rx-setup) | const |  |
| [`RX_DELETE`](#rx-delete) | const |  |
| [`RX_READ`](#rx-read) | const |  |
| [`TX_STATUS`](#tx-status) | const |  |
| [`TX_EXPIRED`](#tx-expired) | const |  |
| [`RX_STATUS`](#rx-status) | const |  |
| [`RX_TIMEOUT`](#rx-timeout) | const |  |
| [`RX_CHANGED`](#rx-changed) | const |  |
| [`SETTIMER`](#settimer) | const |  |
| [`STARTTIMER`](#starttimer) | const |  |
| [`TX_COUNTEVT`](#tx-countevt) | const |  |
| [`TX_ANNOUNCE`](#tx-announce) | const |  |
| [`TX_CP_CAN_ID`](#tx-cp-can-id) | const |  |
| [`RX_FILTER_ID`](#rx-filter-id) | const |  |
| [`RX_CHECK_DLC`](#rx-check-dlc) | const |  |
| [`RX_NO_AUTOTIMER`](#rx-no-autotimer) | const |  |
| [`RX_ANNOUNCE_RESUME`](#rx-announce-resume) | const |  |
| [`TX_RESET_MULTI_IDX`](#tx-reset-multi-idx) | const |  |
| [`RX_RTR_FRAME`](#rx-rtr-frame) | const |  |
| [`CAN_FD_FRAME`](#can-fd-frame) | const |  |
| [`CAN_EFF_FLAG`](#can-eff-flag) | const |  |
| [`CAN_RTR_FLAG`](#can-rtr-flag) | const |  |
| [`CAN_ERR_FLAG`](#can-err-flag) | const |  |
| [`CAN_SFF_MASK`](#can-sff-mask) | const |  |
| [`CAN_EFF_MASK`](#can-eff-mask) | const |  |
| [`CAN_ERR_MASK`](#can-err-mask) | const |  |
| [`CANXL_PRIO_MASK`](#canxl-prio-mask) | const |  |
| [`CAN_SFF_ID_BITS`](#can-sff-id-bits) | const |  |
| [`CAN_EFF_ID_BITS`](#can-eff-id-bits) | const |  |
| [`CANXL_PRIO_BITS`](#canxl-prio-bits) | const |  |
| [`CAN_MAX_DLC`](#can-max-dlc) | const |  |
| [`CAN_MAX_DLEN`](#can-max-dlen) | const |  |
| [`CANFD_MAX_DLC`](#canfd-max-dlc) | const |  |
| [`CANFD_MAX_DLEN`](#canfd-max-dlen) | const |  |
| [`CANXL_MIN_DLC`](#canxl-min-dlc) | const |  |
| [`CANXL_MAX_DLC`](#canxl-max-dlc) | const |  |
| [`CANXL_MAX_DLC_MASK`](#canxl-max-dlc-mask) | const |  |
| [`CANXL_MIN_DLEN`](#canxl-min-dlen) | const |  |
| [`CANXL_MAX_DLEN`](#canxl-max-dlen) | const |  |
| [`CANFD_BRS`](#canfd-brs) | const |  |
| [`CANFD_ESI`](#canfd-esi) | const |  |
| [`CANFD_FDF`](#canfd-fdf) | const |  |
| [`CANXL_XLF`](#canxl-xlf) | const |  |
| [`CANXL_SEC`](#canxl-sec) | const |  |
| [`CAN_MTU`](#can-mtu) | const |  |
| [`CANFD_MTU`](#canfd-mtu) | const |  |
| [`CANXL_MTU`](#canxl-mtu) | const |  |
| [`CANXL_HDR_SIZE`](#canxl-hdr-size) | const |  |
| [`CANXL_MIN_MTU`](#canxl-min-mtu) | const |  |
| [`CANXL_MAX_MTU`](#canxl-max-mtu) | const |  |
| [`CAN_RAW`](#can-raw) | const |  |
| [`CAN_BCM`](#can-bcm) | const |  |
| [`CAN_TP16`](#can-tp16) | const |  |
| [`CAN_TP20`](#can-tp20) | const |  |
| [`CAN_MCNET`](#can-mcnet) | const |  |
| [`CAN_ISOTP`](#can-isotp) | const |  |
| [`CAN_J1939`](#can-j1939) | const |  |
| [`CAN_NPROTO`](#can-nproto) | const |  |
| [`SOL_CAN_BASE`](#sol-can-base) | const |  |
| [`CAN_INV_FILTER`](#can-inv-filter) | const |  |

## Modules

- [`bcm`](bcm/index.md) — Header: `linux/can/bcm.h`
- [`error`](error/index.md) — Header: `linux/can/error.h`
- [`j1939`](j1939/index.md) — `linux/can/j1939.h`
- [`raw`](raw/index.md) — Header: `linux/can/raw.h`

## Structs

### `bcm_timeval`

```rust
struct bcm_timeval {
    pub tv_sec: c_long,
    pub tv_usec: c_long,
}
```

#### Trait Implementations

##### `impl Clone for bcm_timeval`

- <span id="bcm-timeval-clone"></span>`fn clone(&self) -> bcm_timeval` — [`bcm_timeval`](../../../../index.md#bcm-timeval)

##### `impl Copy for bcm_timeval`

##### `impl Debug for bcm_timeval`

- <span id="bcm-timeval-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `bcm_msg_head`

```rust
struct bcm_msg_head {
    pub opcode: u32,
    pub flags: u32,
    pub count: u32,
    pub ival1: bcm_timeval,
    pub ival2: bcm_timeval,
    pub can_id: canid_t,
    pub nframes: u32,
    pub frames: [can_frame; 0],
}
```

#### Trait Implementations

##### `impl Clone for bcm_msg_head`

- <span id="bcm-msg-head-clone"></span>`fn clone(&self) -> bcm_msg_head` — [`bcm_msg_head`](../../../../index.md#bcm-msg-head)

##### `impl Copy for bcm_msg_head`

##### `impl Debug for bcm_msg_head`

- <span id="bcm-msg-head-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `can_frame`

```rust
struct can_frame {
    pub can_id: canid_t,
    pub can_dlc: u8,
    __pad: Padding<u8>,
    __res0: u8,
    pub len8_dlc: u8,
    pub data: [u8; 8],
}
```

#### Trait Implementations

##### `impl Clone for can_frame`

- <span id="can-frame-clone"></span>`fn clone(&self) -> can_frame` — [`can_frame`](../../../../index.md#can-frame)

##### `impl Copy for can_frame`

##### `impl Debug for can_frame`

- <span id="can-frame-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `canfd_frame`

```rust
struct canfd_frame {
    pub can_id: canid_t,
    pub len: u8,
    pub flags: u8,
    __res0: u8,
    __res1: u8,
    pub data: [u8; 64],
}
```

#### Trait Implementations

##### `impl Clone for canfd_frame`

- <span id="canfd-frame-clone"></span>`fn clone(&self) -> canfd_frame` — [`canfd_frame`](../../../../index.md#canfd-frame)

##### `impl Copy for canfd_frame`

##### `impl Debug for canfd_frame`

- <span id="canfd-frame-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `canxl_frame`

```rust
struct canxl_frame {
    pub prio: canid_t,
    pub flags: u8,
    pub sdt: u8,
    pub len: u16,
    pub af: u32,
    pub data: [u8; 2048],
}
```

#### Trait Implementations

##### `impl Clone for canxl_frame`

- <span id="canxl-frame-clone"></span>`fn clone(&self) -> canxl_frame` — [`canxl_frame`](../../../../index.md#canxl-frame)

##### `impl Copy for canxl_frame`

##### `impl Debug for canxl_frame`

- <span id="canxl-frame-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `sockaddr_can`

```rust
struct sockaddr_can {
    pub can_family: crate::sa_family_t,
    pub can_ifindex: c_int,
    pub can_addr: __c_anonymous_sockaddr_can_can_addr,
}
```

#### Trait Implementations

##### `impl Clone for sockaddr_can`

- <span id="sockaddr-can-clone"></span>`fn clone(&self) -> sockaddr_can` — [`sockaddr_can`](../../../../index.md#sockaddr-can)

##### `impl Copy for sockaddr_can`

##### `impl Debug for sockaddr_can`

- <span id="sockaddr-can-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `__c_anonymous_sockaddr_can_tp`

```rust
struct __c_anonymous_sockaddr_can_tp {
    pub rx_id: canid_t,
    pub tx_id: canid_t,
}
```

#### Trait Implementations

##### `impl Clone for __c_anonymous_sockaddr_can_tp`

- <span id="c-anonymous-sockaddr-can-tp-clone"></span>`fn clone(&self) -> __c_anonymous_sockaddr_can_tp` — [`__c_anonymous_sockaddr_can_tp`](../../../../index.md#c-anonymous-sockaddr-can-tp)

##### `impl Copy for __c_anonymous_sockaddr_can_tp`

##### `impl Debug for __c_anonymous_sockaddr_can_tp`

- <span id="c-anonymous-sockaddr-can-tp-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `__c_anonymous_sockaddr_can_j1939`

```rust
struct __c_anonymous_sockaddr_can_j1939 {
    pub name: u64,
    pub pgn: u32,
    pub addr: u8,
}
```

#### Trait Implementations

##### `impl Clone for __c_anonymous_sockaddr_can_j1939`

- <span id="c-anonymous-sockaddr-can-j1939-clone"></span>`fn clone(&self) -> __c_anonymous_sockaddr_can_j1939` — [`__c_anonymous_sockaddr_can_j1939`](../../../../index.md#c-anonymous-sockaddr-can-j1939)

##### `impl Copy for __c_anonymous_sockaddr_can_j1939`

##### `impl Debug for __c_anonymous_sockaddr_can_j1939`

- <span id="c-anonymous-sockaddr-can-j1939-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `can_filter`

```rust
struct can_filter {
    pub can_id: canid_t,
    pub can_mask: canid_t,
}
```

#### Trait Implementations

##### `impl Clone for can_filter`

- <span id="can-filter-clone"></span>`fn clone(&self) -> can_filter` — [`can_filter`](../../../../index.md#can-filter)

##### `impl Copy for can_filter`

##### `impl Debug for can_filter`

- <span id="can-filter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Type Aliases

### `canid_t`

```rust
type canid_t = u32;
```

### `can_err_mask_t`

```rust
type can_err_mask_t = u32;
```

## Constants

### `TX_SETUP`
```rust
const TX_SETUP: u32 = 1u32;
```

### `TX_DELETE`
```rust
const TX_DELETE: u32 = 2u32;
```

### `TX_READ`
```rust
const TX_READ: u32 = 3u32;
```

### `TX_SEND`
```rust
const TX_SEND: u32 = 4u32;
```

### `RX_SETUP`
```rust
const RX_SETUP: u32 = 5u32;
```

### `RX_DELETE`
```rust
const RX_DELETE: u32 = 6u32;
```

### `RX_READ`
```rust
const RX_READ: u32 = 7u32;
```

### `TX_STATUS`
```rust
const TX_STATUS: u32 = 8u32;
```

### `TX_EXPIRED`
```rust
const TX_EXPIRED: u32 = 9u32;
```

### `RX_STATUS`
```rust
const RX_STATUS: u32 = 10u32;
```

### `RX_TIMEOUT`
```rust
const RX_TIMEOUT: u32 = 11u32;
```

### `RX_CHANGED`
```rust
const RX_CHANGED: u32 = 12u32;
```

### `SETTIMER`
```rust
const SETTIMER: u32 = 1u32;
```

### `STARTTIMER`
```rust
const STARTTIMER: u32 = 2u32;
```

### `TX_COUNTEVT`
```rust
const TX_COUNTEVT: u32 = 4u32;
```

### `TX_ANNOUNCE`
```rust
const TX_ANNOUNCE: u32 = 8u32;
```

### `TX_CP_CAN_ID`
```rust
const TX_CP_CAN_ID: u32 = 16u32;
```

### `RX_FILTER_ID`
```rust
const RX_FILTER_ID: u32 = 32u32;
```

### `RX_CHECK_DLC`
```rust
const RX_CHECK_DLC: u32 = 64u32;
```

### `RX_NO_AUTOTIMER`
```rust
const RX_NO_AUTOTIMER: u32 = 128u32;
```

### `RX_ANNOUNCE_RESUME`
```rust
const RX_ANNOUNCE_RESUME: u32 = 256u32;
```

### `TX_RESET_MULTI_IDX`
```rust
const TX_RESET_MULTI_IDX: u32 = 512u32;
```

### `RX_RTR_FRAME`
```rust
const RX_RTR_FRAME: u32 = 1_024u32;
```

### `CAN_FD_FRAME`
```rust
const CAN_FD_FRAME: u32 = 2_048u32;
```

### `CAN_EFF_FLAG`
```rust
const CAN_EFF_FLAG: canid_t = 2_147_483_648u32;
```

### `CAN_RTR_FLAG`
```rust
const CAN_RTR_FLAG: canid_t = 1_073_741_824u32;
```

### `CAN_ERR_FLAG`
```rust
const CAN_ERR_FLAG: canid_t = 536_870_912u32;
```

### `CAN_SFF_MASK`
```rust
const CAN_SFF_MASK: canid_t = 2_047u32;
```

### `CAN_EFF_MASK`
```rust
const CAN_EFF_MASK: canid_t = 536_870_911u32;
```

### `CAN_ERR_MASK`
```rust
const CAN_ERR_MASK: canid_t = 536_870_911u32;
```

### `CANXL_PRIO_MASK`
```rust
const CANXL_PRIO_MASK: crate::canid_t = 2_047u32;
```

### `CAN_SFF_ID_BITS`
```rust
const CAN_SFF_ID_BITS: c_int = 11i32;
```

### `CAN_EFF_ID_BITS`
```rust
const CAN_EFF_ID_BITS: c_int = 29i32;
```

### `CANXL_PRIO_BITS`
```rust
const CANXL_PRIO_BITS: c_int = 11i32;
```

### `CAN_MAX_DLC`
```rust
const CAN_MAX_DLC: c_int = 8i32;
```

### `CAN_MAX_DLEN`
```rust
const CAN_MAX_DLEN: usize = 8usize;
```

### `CANFD_MAX_DLC`
```rust
const CANFD_MAX_DLC: c_int = 15i32;
```

### `CANFD_MAX_DLEN`
```rust
const CANFD_MAX_DLEN: usize = 64usize;
```

### `CANXL_MIN_DLC`
```rust
const CANXL_MIN_DLC: c_int = 0i32;
```

### `CANXL_MAX_DLC`
```rust
const CANXL_MAX_DLC: c_int = 2_047i32;
```

### `CANXL_MAX_DLC_MASK`
```rust
const CANXL_MAX_DLC_MASK: c_int = 2_047i32;
```

### `CANXL_MIN_DLEN`
```rust
const CANXL_MIN_DLEN: usize = 1usize;
```

### `CANXL_MAX_DLEN`
```rust
const CANXL_MAX_DLEN: usize = 2_048usize;
```

### `CANFD_BRS`
```rust
const CANFD_BRS: c_int = 1i32;
```

### `CANFD_ESI`
```rust
const CANFD_ESI: c_int = 2i32;
```

### `CANFD_FDF`
```rust
const CANFD_FDF: c_int = 4i32;
```

### `CANXL_XLF`
```rust
const CANXL_XLF: c_int = 128i32;
```

### `CANXL_SEC`
```rust
const CANXL_SEC: c_int = 1i32;
```

### `CAN_MTU`
```rust
const CAN_MTU: usize = 16usize;
```

### `CANFD_MTU`
```rust
const CANFD_MTU: usize = 72usize;
```

### `CANXL_MTU`
```rust
const CANXL_MTU: usize = 2_060usize;
```

### `CANXL_HDR_SIZE`
```rust
const CANXL_HDR_SIZE: usize = 12usize;
```

### `CANXL_MIN_MTU`
```rust
const CANXL_MIN_MTU: usize = 76usize;
```

### `CANXL_MAX_MTU`
```rust
const CANXL_MAX_MTU: usize = 2_060usize;
```

### `CAN_RAW`
```rust
const CAN_RAW: c_int = 1i32;
```

### `CAN_BCM`
```rust
const CAN_BCM: c_int = 2i32;
```

### `CAN_TP16`
```rust
const CAN_TP16: c_int = 3i32;
```

### `CAN_TP20`
```rust
const CAN_TP20: c_int = 4i32;
```

### `CAN_MCNET`
```rust
const CAN_MCNET: c_int = 5i32;
```

### `CAN_ISOTP`
```rust
const CAN_ISOTP: c_int = 6i32;
```

### `CAN_J1939`
```rust
const CAN_J1939: c_int = 7i32;
```

### `CAN_NPROTO`
```rust
const CAN_NPROTO: c_int = 8i32;
```

### `SOL_CAN_BASE`
```rust
const SOL_CAN_BASE: c_int = 100i32;
```

### `CAN_INV_FILTER`
```rust
const CAN_INV_FILTER: canid_t = 536_870_912u32;
```

