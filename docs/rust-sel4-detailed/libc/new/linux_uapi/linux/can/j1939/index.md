*[libc](../../../../../index.md) / [new](../../../../index.md) / [linux_uapi](../../../index.md) / [linux](../../index.md) / [can](../index.md) / [j1939](index.md)*

---

# Module `j1939`

`linux/can/j1939.h`

## Contents

- [Modules](#modules)
  - [`bcm`](#bcm)
  - [`error`](#error)
  - [`j1939`](#j1939)
  - [`raw`](#raw)
- [Structs](#structs)
  - [`j1939_filter`](#j1939-filter)
  - [`can_frame`](#can-frame)
  - [`canfd_frame`](#canfd-frame)
  - [`canxl_frame`](#canxl-frame)
  - [`sockaddr_can`](#sockaddr-can)
  - [`__c_anonymous_sockaddr_can_tp`](#c-anonymous-sockaddr-can-tp)
  - [`__c_anonymous_sockaddr_can_j1939`](#c-anonymous-sockaddr-can-j1939)
  - [`can_filter`](#can-filter)
- [Type Aliases](#type-aliases)
  - [`pgn_t`](#pgn-t)
  - [`priority_t`](#priority-t)
  - [`name_t`](#name-t)
  - [`canid_t`](#canid-t)
  - [`can_err_mask_t`](#can-err-mask-t)
- [Constants](#constants)
  - [`J1939_MAX_UNICAST_ADDR`](#j1939-max-unicast-addr)
  - [`J1939_IDLE_ADDR`](#j1939-idle-addr)
  - [`J1939_NO_ADDR`](#j1939-no-addr)
  - [`J1939_NO_NAME`](#j1939-no-name)
  - [`J1939_PGN_REQUEST`](#j1939-pgn-request)
  - [`J1939_PGN_ADDRESS_CLAIMED`](#j1939-pgn-address-claimed)
  - [`J1939_PGN_ADDRESS_COMMANDED`](#j1939-pgn-address-commanded)
  - [`J1939_PGN_PDU1_MAX`](#j1939-pgn-pdu1-max)
  - [`J1939_PGN_MAX`](#j1939-pgn-max)
  - [`J1939_NO_PGN`](#j1939-no-pgn)
  - [`SOL_CAN_J1939`](#sol-can-j1939)
  - [`SO_J1939_FILTER`](#so-j1939-filter)
  - [`SO_J1939_PROMISC`](#so-j1939-promisc)
  - [`SO_J1939_SEND_PRIO`](#so-j1939-send-prio)
  - [`SO_J1939_ERRQUEUE`](#so-j1939-errqueue)
  - [`SCM_J1939_DEST_ADDR`](#scm-j1939-dest-addr)
  - [`SCM_J1939_DEST_NAME`](#scm-j1939-dest-name)
  - [`SCM_J1939_PRIO`](#scm-j1939-prio)
  - [`SCM_J1939_ERRQUEUE`](#scm-j1939-errqueue)
  - [`J1939_NLA_PAD`](#j1939-nla-pad)
  - [`J1939_NLA_BYTES_ACKED`](#j1939-nla-bytes-acked)
  - [`J1939_NLA_TOTAL_SIZE`](#j1939-nla-total-size)
  - [`J1939_NLA_PGN`](#j1939-nla-pgn)
  - [`J1939_NLA_SRC_NAME`](#j1939-nla-src-name)
  - [`J1939_NLA_DEST_NAME`](#j1939-nla-dest-name)
  - [`J1939_NLA_SRC_ADDR`](#j1939-nla-src-addr)
  - [`J1939_NLA_DEST_ADDR`](#j1939-nla-dest-addr)
  - [`J1939_EE_INFO_NONE`](#j1939-ee-info-none)
  - [`J1939_EE_INFO_TX_ABORT`](#j1939-ee-info-tx-abort)
  - [`J1939_EE_INFO_RX_RTS`](#j1939-ee-info-rx-rts)
  - [`J1939_EE_INFO_RX_DPO`](#j1939-ee-info-rx-dpo)
  - [`J1939_EE_INFO_RX_ABORT`](#j1939-ee-info-rx-abort)
  - [`J1939_FILTER_MAX`](#j1939-filter-max)
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
| [`j1939_filter`](#j1939-filter) | struct |  |
| [`can_frame`](#can-frame) | struct |  |
| [`canfd_frame`](#canfd-frame) | struct |  |
| [`canxl_frame`](#canxl-frame) | struct |  |
| [`sockaddr_can`](#sockaddr-can) | struct |  |
| [`__c_anonymous_sockaddr_can_tp`](#c-anonymous-sockaddr-can-tp) | struct |  |
| [`__c_anonymous_sockaddr_can_j1939`](#c-anonymous-sockaddr-can-j1939) | struct |  |
| [`can_filter`](#can-filter) | struct |  |
| [`pgn_t`](#pgn-t) | type |  |
| [`priority_t`](#priority-t) | type |  |
| [`name_t`](#name-t) | type |  |
| [`canid_t`](#canid-t) | type |  |
| [`can_err_mask_t`](#can-err-mask-t) | type |  |
| [`J1939_MAX_UNICAST_ADDR`](#j1939-max-unicast-addr) | const |  |
| [`J1939_IDLE_ADDR`](#j1939-idle-addr) | const |  |
| [`J1939_NO_ADDR`](#j1939-no-addr) | const |  |
| [`J1939_NO_NAME`](#j1939-no-name) | const |  |
| [`J1939_PGN_REQUEST`](#j1939-pgn-request) | const |  |
| [`J1939_PGN_ADDRESS_CLAIMED`](#j1939-pgn-address-claimed) | const |  |
| [`J1939_PGN_ADDRESS_COMMANDED`](#j1939-pgn-address-commanded) | const |  |
| [`J1939_PGN_PDU1_MAX`](#j1939-pgn-pdu1-max) | const |  |
| [`J1939_PGN_MAX`](#j1939-pgn-max) | const |  |
| [`J1939_NO_PGN`](#j1939-no-pgn) | const |  |
| [`SOL_CAN_J1939`](#sol-can-j1939) | const |  |
| [`SO_J1939_FILTER`](#so-j1939-filter) | const |  |
| [`SO_J1939_PROMISC`](#so-j1939-promisc) | const |  |
| [`SO_J1939_SEND_PRIO`](#so-j1939-send-prio) | const |  |
| [`SO_J1939_ERRQUEUE`](#so-j1939-errqueue) | const |  |
| [`SCM_J1939_DEST_ADDR`](#scm-j1939-dest-addr) | const |  |
| [`SCM_J1939_DEST_NAME`](#scm-j1939-dest-name) | const |  |
| [`SCM_J1939_PRIO`](#scm-j1939-prio) | const |  |
| [`SCM_J1939_ERRQUEUE`](#scm-j1939-errqueue) | const |  |
| [`J1939_NLA_PAD`](#j1939-nla-pad) | const |  |
| [`J1939_NLA_BYTES_ACKED`](#j1939-nla-bytes-acked) | const |  |
| [`J1939_NLA_TOTAL_SIZE`](#j1939-nla-total-size) | const |  |
| [`J1939_NLA_PGN`](#j1939-nla-pgn) | const |  |
| [`J1939_NLA_SRC_NAME`](#j1939-nla-src-name) | const |  |
| [`J1939_NLA_DEST_NAME`](#j1939-nla-dest-name) | const |  |
| [`J1939_NLA_SRC_ADDR`](#j1939-nla-src-addr) | const |  |
| [`J1939_NLA_DEST_ADDR`](#j1939-nla-dest-addr) | const |  |
| [`J1939_EE_INFO_NONE`](#j1939-ee-info-none) | const |  |
| [`J1939_EE_INFO_TX_ABORT`](#j1939-ee-info-tx-abort) | const |  |
| [`J1939_EE_INFO_RX_RTS`](#j1939-ee-info-rx-rts) | const |  |
| [`J1939_EE_INFO_RX_DPO`](#j1939-ee-info-rx-dpo) | const |  |
| [`J1939_EE_INFO_RX_ABORT`](#j1939-ee-info-rx-abort) | const |  |
| [`J1939_FILTER_MAX`](#j1939-filter-max) | const |  |
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

### `j1939_filter`

```rust
struct j1939_filter {
    pub name: name_t,
    pub name_mask: name_t,
    pub pgn: pgn_t,
    pub pgn_mask: pgn_t,
    pub addr: u8,
    pub addr_mask: u8,
}
```

#### Trait Implementations

##### `impl Clone for j1939_filter`

- <span id="j1939-filter-clone"></span>`fn clone(&self) -> j1939_filter` — [`j1939_filter`](../../../../index.md#j1939-filter)

##### `impl Copy for j1939_filter`

##### `impl Debug for j1939_filter`

- <span id="j1939-filter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

### `pgn_t`

```rust
type pgn_t = u32;
```

### `priority_t`

```rust
type priority_t = u8;
```

### `name_t`

```rust
type name_t = u64;
```

### `canid_t`

```rust
type canid_t = u32;
```

### `can_err_mask_t`

```rust
type can_err_mask_t = u32;
```

## Constants

### `J1939_MAX_UNICAST_ADDR`
```rust
const J1939_MAX_UNICAST_ADDR: c_uchar = 253u8;
```

### `J1939_IDLE_ADDR`
```rust
const J1939_IDLE_ADDR: c_uchar = 254u8;
```

### `J1939_NO_ADDR`
```rust
const J1939_NO_ADDR: c_uchar = 255u8;
```

### `J1939_NO_NAME`
```rust
const J1939_NO_NAME: c_ulong = 0u64;
```

### `J1939_PGN_REQUEST`
```rust
const J1939_PGN_REQUEST: c_uint = 59_904u32;
```

### `J1939_PGN_ADDRESS_CLAIMED`
```rust
const J1939_PGN_ADDRESS_CLAIMED: c_uint = 60_928u32;
```

### `J1939_PGN_ADDRESS_COMMANDED`
```rust
const J1939_PGN_ADDRESS_COMMANDED: c_uint = 65_240u32;
```

### `J1939_PGN_PDU1_MAX`
```rust
const J1939_PGN_PDU1_MAX: c_uint = 261_888u32;
```

### `J1939_PGN_MAX`
```rust
const J1939_PGN_MAX: c_uint = 262_143u32;
```

### `J1939_NO_PGN`
```rust
const J1939_NO_PGN: c_uint = 262_144u32;
```

### `SOL_CAN_J1939`
```rust
const SOL_CAN_J1939: c_int = 107i32;
```

### `SO_J1939_FILTER`
```rust
const SO_J1939_FILTER: c_int = 1i32;
```

### `SO_J1939_PROMISC`
```rust
const SO_J1939_PROMISC: c_int = 2i32;
```

### `SO_J1939_SEND_PRIO`
```rust
const SO_J1939_SEND_PRIO: c_int = 3i32;
```

### `SO_J1939_ERRQUEUE`
```rust
const SO_J1939_ERRQUEUE: c_int = 4i32;
```

### `SCM_J1939_DEST_ADDR`
```rust
const SCM_J1939_DEST_ADDR: c_int = 1i32;
```

### `SCM_J1939_DEST_NAME`
```rust
const SCM_J1939_DEST_NAME: c_int = 2i32;
```

### `SCM_J1939_PRIO`
```rust
const SCM_J1939_PRIO: c_int = 3i32;
```

### `SCM_J1939_ERRQUEUE`
```rust
const SCM_J1939_ERRQUEUE: c_int = 4i32;
```

### `J1939_NLA_PAD`
```rust
const J1939_NLA_PAD: c_int = 0i32;
```

### `J1939_NLA_BYTES_ACKED`
```rust
const J1939_NLA_BYTES_ACKED: c_int = 1i32;
```

### `J1939_NLA_TOTAL_SIZE`
```rust
const J1939_NLA_TOTAL_SIZE: c_int = 2i32;
```

### `J1939_NLA_PGN`
```rust
const J1939_NLA_PGN: c_int = 3i32;
```

### `J1939_NLA_SRC_NAME`
```rust
const J1939_NLA_SRC_NAME: c_int = 4i32;
```

### `J1939_NLA_DEST_NAME`
```rust
const J1939_NLA_DEST_NAME: c_int = 5i32;
```

### `J1939_NLA_SRC_ADDR`
```rust
const J1939_NLA_SRC_ADDR: c_int = 6i32;
```

### `J1939_NLA_DEST_ADDR`
```rust
const J1939_NLA_DEST_ADDR: c_int = 7i32;
```

### `J1939_EE_INFO_NONE`
```rust
const J1939_EE_INFO_NONE: c_int = 0i32;
```

### `J1939_EE_INFO_TX_ABORT`
```rust
const J1939_EE_INFO_TX_ABORT: c_int = 1i32;
```

### `J1939_EE_INFO_RX_RTS`
```rust
const J1939_EE_INFO_RX_RTS: c_int = 2i32;
```

### `J1939_EE_INFO_RX_DPO`
```rust
const J1939_EE_INFO_RX_DPO: c_int = 3i32;
```

### `J1939_EE_INFO_RX_ABORT`
```rust
const J1939_EE_INFO_RX_ABORT: c_int = 4i32;
```

### `J1939_FILTER_MAX`
```rust
const J1939_FILTER_MAX: c_int = 512i32;
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

