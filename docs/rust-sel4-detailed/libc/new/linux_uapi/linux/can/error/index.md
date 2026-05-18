*[libc](../../../../../index.md) / [new](../../../../index.md) / [linux_uapi](../../../index.md) / [linux](../../index.md) / [can](../index.md) / [error](index.md)*

---

# Module `error`

Header: `linux/can/error.h`

## Contents

- [Modules](#modules)
  - [`bcm`](#bcm)
  - [`error`](#error)
  - [`j1939`](#j1939)
  - [`raw`](#raw)
- [Structs](#structs)
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
  - [`CAN_ERR_DLC`](#can-err-dlc)
  - [`CAN_ERR_TX_TIMEOUT`](#can-err-tx-timeout)
  - [`CAN_ERR_LOSTARB`](#can-err-lostarb)
  - [`CAN_ERR_CRTL`](#can-err-crtl)
  - [`CAN_ERR_PROT`](#can-err-prot)
  - [`CAN_ERR_TRX`](#can-err-trx)
  - [`CAN_ERR_ACK`](#can-err-ack)
  - [`CAN_ERR_BUSOFF`](#can-err-busoff)
  - [`CAN_ERR_BUSERROR`](#can-err-buserror)
  - [`CAN_ERR_RESTARTED`](#can-err-restarted)
  - [`CAN_ERR_CNT`](#can-err-cnt)
  - [`CAN_ERR_LOSTARB_UNSPEC`](#can-err-lostarb-unspec)
  - [`CAN_ERR_CRTL_UNSPEC`](#can-err-crtl-unspec)
  - [`CAN_ERR_CRTL_RX_OVERFLOW`](#can-err-crtl-rx-overflow)
  - [`CAN_ERR_CRTL_TX_OVERFLOW`](#can-err-crtl-tx-overflow)
  - [`CAN_ERR_CRTL_RX_WARNING`](#can-err-crtl-rx-warning)
  - [`CAN_ERR_CRTL_TX_WARNING`](#can-err-crtl-tx-warning)
  - [`CAN_ERR_CRTL_RX_PASSIVE`](#can-err-crtl-rx-passive)
  - [`CAN_ERR_CRTL_TX_PASSIVE`](#can-err-crtl-tx-passive)
  - [`CAN_ERR_CRTL_ACTIVE`](#can-err-crtl-active)
  - [`CAN_ERR_PROT_UNSPEC`](#can-err-prot-unspec)
  - [`CAN_ERR_PROT_BIT`](#can-err-prot-bit)
  - [`CAN_ERR_PROT_FORM`](#can-err-prot-form)
  - [`CAN_ERR_PROT_STUFF`](#can-err-prot-stuff)
  - [`CAN_ERR_PROT_BIT0`](#can-err-prot-bit0)
  - [`CAN_ERR_PROT_BIT1`](#can-err-prot-bit1)
  - [`CAN_ERR_PROT_OVERLOAD`](#can-err-prot-overload)
  - [`CAN_ERR_PROT_ACTIVE`](#can-err-prot-active)
  - [`CAN_ERR_PROT_TX`](#can-err-prot-tx)
  - [`CAN_ERR_PROT_LOC_UNSPEC`](#can-err-prot-loc-unspec)
  - [`CAN_ERR_PROT_LOC_SOF`](#can-err-prot-loc-sof)
  - [`CAN_ERR_PROT_LOC_ID28_21`](#can-err-prot-loc-id28-21)
  - [`CAN_ERR_PROT_LOC_ID20_18`](#can-err-prot-loc-id20-18)
  - [`CAN_ERR_PROT_LOC_SRTR`](#can-err-prot-loc-srtr)
  - [`CAN_ERR_PROT_LOC_IDE`](#can-err-prot-loc-ide)
  - [`CAN_ERR_PROT_LOC_ID17_13`](#can-err-prot-loc-id17-13)
  - [`CAN_ERR_PROT_LOC_ID12_05`](#can-err-prot-loc-id12-05)
  - [`CAN_ERR_PROT_LOC_ID04_00`](#can-err-prot-loc-id04-00)
  - [`CAN_ERR_PROT_LOC_RTR`](#can-err-prot-loc-rtr)
  - [`CAN_ERR_PROT_LOC_RES1`](#can-err-prot-loc-res1)
  - [`CAN_ERR_PROT_LOC_RES0`](#can-err-prot-loc-res0)
  - [`CAN_ERR_PROT_LOC_DLC`](#can-err-prot-loc-dlc)
  - [`CAN_ERR_PROT_LOC_DATA`](#can-err-prot-loc-data)
  - [`CAN_ERR_PROT_LOC_CRC_SEQ`](#can-err-prot-loc-crc-seq)
  - [`CAN_ERR_PROT_LOC_CRC_DEL`](#can-err-prot-loc-crc-del)
  - [`CAN_ERR_PROT_LOC_ACK`](#can-err-prot-loc-ack)
  - [`CAN_ERR_PROT_LOC_ACK_DEL`](#can-err-prot-loc-ack-del)
  - [`CAN_ERR_PROT_LOC_EOF`](#can-err-prot-loc-eof)
  - [`CAN_ERR_PROT_LOC_INTERM`](#can-err-prot-loc-interm)
  - [`CAN_ERR_TRX_UNSPEC`](#can-err-trx-unspec)
  - [`CAN_ERR_TRX_CANH_NO_WIRE`](#can-err-trx-canh-no-wire)
  - [`CAN_ERR_TRX_CANH_SHORT_TO_BAT`](#can-err-trx-canh-short-to-bat)
  - [`CAN_ERR_TRX_CANH_SHORT_TO_VCC`](#can-err-trx-canh-short-to-vcc)
  - [`CAN_ERR_TRX_CANH_SHORT_TO_GND`](#can-err-trx-canh-short-to-gnd)
  - [`CAN_ERR_TRX_CANL_NO_WIRE`](#can-err-trx-canl-no-wire)
  - [`CAN_ERR_TRX_CANL_SHORT_TO_BAT`](#can-err-trx-canl-short-to-bat)
  - [`CAN_ERR_TRX_CANL_SHORT_TO_VCC`](#can-err-trx-canl-short-to-vcc)
  - [`CAN_ERR_TRX_CANL_SHORT_TO_GND`](#can-err-trx-canl-short-to-gnd)
  - [`CAN_ERR_TRX_CANL_SHORT_TO_CANH`](#can-err-trx-canl-short-to-canh)
  - [`CAN_ERROR_WARNING_THRESHOLD`](#can-error-warning-threshold)
  - [`CAN_ERROR_PASSIVE_THRESHOLD`](#can-error-passive-threshold)
  - [`CAN_BUS_OFF_THRESHOLD`](#can-bus-off-threshold)
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
| [`can_frame`](#can-frame) | struct |  |
| [`canfd_frame`](#canfd-frame) | struct |  |
| [`canxl_frame`](#canxl-frame) | struct |  |
| [`sockaddr_can`](#sockaddr-can) | struct |  |
| [`__c_anonymous_sockaddr_can_tp`](#c-anonymous-sockaddr-can-tp) | struct |  |
| [`__c_anonymous_sockaddr_can_j1939`](#c-anonymous-sockaddr-can-j1939) | struct |  |
| [`can_filter`](#can-filter) | struct |  |
| [`canid_t`](#canid-t) | type |  |
| [`can_err_mask_t`](#can-err-mask-t) | type |  |
| [`CAN_ERR_DLC`](#can-err-dlc) | const |  |
| [`CAN_ERR_TX_TIMEOUT`](#can-err-tx-timeout) | const |  |
| [`CAN_ERR_LOSTARB`](#can-err-lostarb) | const |  |
| [`CAN_ERR_CRTL`](#can-err-crtl) | const |  |
| [`CAN_ERR_PROT`](#can-err-prot) | const |  |
| [`CAN_ERR_TRX`](#can-err-trx) | const |  |
| [`CAN_ERR_ACK`](#can-err-ack) | const |  |
| [`CAN_ERR_BUSOFF`](#can-err-busoff) | const |  |
| [`CAN_ERR_BUSERROR`](#can-err-buserror) | const |  |
| [`CAN_ERR_RESTARTED`](#can-err-restarted) | const |  |
| [`CAN_ERR_CNT`](#can-err-cnt) | const |  |
| [`CAN_ERR_LOSTARB_UNSPEC`](#can-err-lostarb-unspec) | const |  |
| [`CAN_ERR_CRTL_UNSPEC`](#can-err-crtl-unspec) | const |  |
| [`CAN_ERR_CRTL_RX_OVERFLOW`](#can-err-crtl-rx-overflow) | const |  |
| [`CAN_ERR_CRTL_TX_OVERFLOW`](#can-err-crtl-tx-overflow) | const |  |
| [`CAN_ERR_CRTL_RX_WARNING`](#can-err-crtl-rx-warning) | const |  |
| [`CAN_ERR_CRTL_TX_WARNING`](#can-err-crtl-tx-warning) | const |  |
| [`CAN_ERR_CRTL_RX_PASSIVE`](#can-err-crtl-rx-passive) | const |  |
| [`CAN_ERR_CRTL_TX_PASSIVE`](#can-err-crtl-tx-passive) | const |  |
| [`CAN_ERR_CRTL_ACTIVE`](#can-err-crtl-active) | const |  |
| [`CAN_ERR_PROT_UNSPEC`](#can-err-prot-unspec) | const |  |
| [`CAN_ERR_PROT_BIT`](#can-err-prot-bit) | const |  |
| [`CAN_ERR_PROT_FORM`](#can-err-prot-form) | const |  |
| [`CAN_ERR_PROT_STUFF`](#can-err-prot-stuff) | const |  |
| [`CAN_ERR_PROT_BIT0`](#can-err-prot-bit0) | const |  |
| [`CAN_ERR_PROT_BIT1`](#can-err-prot-bit1) | const |  |
| [`CAN_ERR_PROT_OVERLOAD`](#can-err-prot-overload) | const |  |
| [`CAN_ERR_PROT_ACTIVE`](#can-err-prot-active) | const |  |
| [`CAN_ERR_PROT_TX`](#can-err-prot-tx) | const |  |
| [`CAN_ERR_PROT_LOC_UNSPEC`](#can-err-prot-loc-unspec) | const |  |
| [`CAN_ERR_PROT_LOC_SOF`](#can-err-prot-loc-sof) | const |  |
| [`CAN_ERR_PROT_LOC_ID28_21`](#can-err-prot-loc-id28-21) | const |  |
| [`CAN_ERR_PROT_LOC_ID20_18`](#can-err-prot-loc-id20-18) | const |  |
| [`CAN_ERR_PROT_LOC_SRTR`](#can-err-prot-loc-srtr) | const |  |
| [`CAN_ERR_PROT_LOC_IDE`](#can-err-prot-loc-ide) | const |  |
| [`CAN_ERR_PROT_LOC_ID17_13`](#can-err-prot-loc-id17-13) | const |  |
| [`CAN_ERR_PROT_LOC_ID12_05`](#can-err-prot-loc-id12-05) | const |  |
| [`CAN_ERR_PROT_LOC_ID04_00`](#can-err-prot-loc-id04-00) | const |  |
| [`CAN_ERR_PROT_LOC_RTR`](#can-err-prot-loc-rtr) | const |  |
| [`CAN_ERR_PROT_LOC_RES1`](#can-err-prot-loc-res1) | const |  |
| [`CAN_ERR_PROT_LOC_RES0`](#can-err-prot-loc-res0) | const |  |
| [`CAN_ERR_PROT_LOC_DLC`](#can-err-prot-loc-dlc) | const |  |
| [`CAN_ERR_PROT_LOC_DATA`](#can-err-prot-loc-data) | const |  |
| [`CAN_ERR_PROT_LOC_CRC_SEQ`](#can-err-prot-loc-crc-seq) | const |  |
| [`CAN_ERR_PROT_LOC_CRC_DEL`](#can-err-prot-loc-crc-del) | const |  |
| [`CAN_ERR_PROT_LOC_ACK`](#can-err-prot-loc-ack) | const |  |
| [`CAN_ERR_PROT_LOC_ACK_DEL`](#can-err-prot-loc-ack-del) | const |  |
| [`CAN_ERR_PROT_LOC_EOF`](#can-err-prot-loc-eof) | const |  |
| [`CAN_ERR_PROT_LOC_INTERM`](#can-err-prot-loc-interm) | const |  |
| [`CAN_ERR_TRX_UNSPEC`](#can-err-trx-unspec) | const |  |
| [`CAN_ERR_TRX_CANH_NO_WIRE`](#can-err-trx-canh-no-wire) | const |  |
| [`CAN_ERR_TRX_CANH_SHORT_TO_BAT`](#can-err-trx-canh-short-to-bat) | const |  |
| [`CAN_ERR_TRX_CANH_SHORT_TO_VCC`](#can-err-trx-canh-short-to-vcc) | const |  |
| [`CAN_ERR_TRX_CANH_SHORT_TO_GND`](#can-err-trx-canh-short-to-gnd) | const |  |
| [`CAN_ERR_TRX_CANL_NO_WIRE`](#can-err-trx-canl-no-wire) | const |  |
| [`CAN_ERR_TRX_CANL_SHORT_TO_BAT`](#can-err-trx-canl-short-to-bat) | const |  |
| [`CAN_ERR_TRX_CANL_SHORT_TO_VCC`](#can-err-trx-canl-short-to-vcc) | const |  |
| [`CAN_ERR_TRX_CANL_SHORT_TO_GND`](#can-err-trx-canl-short-to-gnd) | const |  |
| [`CAN_ERR_TRX_CANL_SHORT_TO_CANH`](#can-err-trx-canl-short-to-canh) | const |  |
| [`CAN_ERROR_WARNING_THRESHOLD`](#can-error-warning-threshold) | const |  |
| [`CAN_ERROR_PASSIVE_THRESHOLD`](#can-error-passive-threshold) | const |  |
| [`CAN_BUS_OFF_THRESHOLD`](#can-bus-off-threshold) | const |  |
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

### `CAN_ERR_DLC`
```rust
const CAN_ERR_DLC: c_int = 8i32;
```

### `CAN_ERR_TX_TIMEOUT`
```rust
const CAN_ERR_TX_TIMEOUT: c_uint = 1u32;
```

### `CAN_ERR_LOSTARB`
```rust
const CAN_ERR_LOSTARB: c_uint = 2u32;
```

### `CAN_ERR_CRTL`
```rust
const CAN_ERR_CRTL: c_uint = 4u32;
```

### `CAN_ERR_PROT`
```rust
const CAN_ERR_PROT: c_uint = 8u32;
```

### `CAN_ERR_TRX`
```rust
const CAN_ERR_TRX: c_uint = 16u32;
```

### `CAN_ERR_ACK`
```rust
const CAN_ERR_ACK: c_uint = 32u32;
```

### `CAN_ERR_BUSOFF`
```rust
const CAN_ERR_BUSOFF: c_uint = 64u32;
```

### `CAN_ERR_BUSERROR`
```rust
const CAN_ERR_BUSERROR: c_uint = 128u32;
```

### `CAN_ERR_RESTARTED`
```rust
const CAN_ERR_RESTARTED: c_uint = 256u32;
```

### `CAN_ERR_CNT`
```rust
const CAN_ERR_CNT: c_uint = 512u32;
```

### `CAN_ERR_LOSTARB_UNSPEC`
```rust
const CAN_ERR_LOSTARB_UNSPEC: c_int = 0i32;
```

### `CAN_ERR_CRTL_UNSPEC`
```rust
const CAN_ERR_CRTL_UNSPEC: c_int = 0i32;
```

### `CAN_ERR_CRTL_RX_OVERFLOW`
```rust
const CAN_ERR_CRTL_RX_OVERFLOW: c_int = 1i32;
```

### `CAN_ERR_CRTL_TX_OVERFLOW`
```rust
const CAN_ERR_CRTL_TX_OVERFLOW: c_int = 2i32;
```

### `CAN_ERR_CRTL_RX_WARNING`
```rust
const CAN_ERR_CRTL_RX_WARNING: c_int = 4i32;
```

### `CAN_ERR_CRTL_TX_WARNING`
```rust
const CAN_ERR_CRTL_TX_WARNING: c_int = 8i32;
```

### `CAN_ERR_CRTL_RX_PASSIVE`
```rust
const CAN_ERR_CRTL_RX_PASSIVE: c_int = 16i32;
```

### `CAN_ERR_CRTL_TX_PASSIVE`
```rust
const CAN_ERR_CRTL_TX_PASSIVE: c_int = 32i32;
```

### `CAN_ERR_CRTL_ACTIVE`
```rust
const CAN_ERR_CRTL_ACTIVE: c_int = 64i32;
```

### `CAN_ERR_PROT_UNSPEC`
```rust
const CAN_ERR_PROT_UNSPEC: c_int = 0i32;
```

### `CAN_ERR_PROT_BIT`
```rust
const CAN_ERR_PROT_BIT: c_int = 1i32;
```

### `CAN_ERR_PROT_FORM`
```rust
const CAN_ERR_PROT_FORM: c_int = 2i32;
```

### `CAN_ERR_PROT_STUFF`
```rust
const CAN_ERR_PROT_STUFF: c_int = 4i32;
```

### `CAN_ERR_PROT_BIT0`
```rust
const CAN_ERR_PROT_BIT0: c_int = 8i32;
```

### `CAN_ERR_PROT_BIT1`
```rust
const CAN_ERR_PROT_BIT1: c_int = 16i32;
```

### `CAN_ERR_PROT_OVERLOAD`
```rust
const CAN_ERR_PROT_OVERLOAD: c_int = 32i32;
```

### `CAN_ERR_PROT_ACTIVE`
```rust
const CAN_ERR_PROT_ACTIVE: c_int = 64i32;
```

### `CAN_ERR_PROT_TX`
```rust
const CAN_ERR_PROT_TX: c_int = 128i32;
```

### `CAN_ERR_PROT_LOC_UNSPEC`
```rust
const CAN_ERR_PROT_LOC_UNSPEC: c_int = 0i32;
```

### `CAN_ERR_PROT_LOC_SOF`
```rust
const CAN_ERR_PROT_LOC_SOF: c_int = 3i32;
```

### `CAN_ERR_PROT_LOC_ID28_21`
```rust
const CAN_ERR_PROT_LOC_ID28_21: c_int = 2i32;
```

### `CAN_ERR_PROT_LOC_ID20_18`
```rust
const CAN_ERR_PROT_LOC_ID20_18: c_int = 6i32;
```

### `CAN_ERR_PROT_LOC_SRTR`
```rust
const CAN_ERR_PROT_LOC_SRTR: c_int = 4i32;
```

### `CAN_ERR_PROT_LOC_IDE`
```rust
const CAN_ERR_PROT_LOC_IDE: c_int = 5i32;
```

### `CAN_ERR_PROT_LOC_ID17_13`
```rust
const CAN_ERR_PROT_LOC_ID17_13: c_int = 7i32;
```

### `CAN_ERR_PROT_LOC_ID12_05`
```rust
const CAN_ERR_PROT_LOC_ID12_05: c_int = 15i32;
```

### `CAN_ERR_PROT_LOC_ID04_00`
```rust
const CAN_ERR_PROT_LOC_ID04_00: c_int = 14i32;
```

### `CAN_ERR_PROT_LOC_RTR`
```rust
const CAN_ERR_PROT_LOC_RTR: c_int = 12i32;
```

### `CAN_ERR_PROT_LOC_RES1`
```rust
const CAN_ERR_PROT_LOC_RES1: c_int = 13i32;
```

### `CAN_ERR_PROT_LOC_RES0`
```rust
const CAN_ERR_PROT_LOC_RES0: c_int = 9i32;
```

### `CAN_ERR_PROT_LOC_DLC`
```rust
const CAN_ERR_PROT_LOC_DLC: c_int = 11i32;
```

### `CAN_ERR_PROT_LOC_DATA`
```rust
const CAN_ERR_PROT_LOC_DATA: c_int = 10i32;
```

### `CAN_ERR_PROT_LOC_CRC_SEQ`
```rust
const CAN_ERR_PROT_LOC_CRC_SEQ: c_int = 8i32;
```

### `CAN_ERR_PROT_LOC_CRC_DEL`
```rust
const CAN_ERR_PROT_LOC_CRC_DEL: c_int = 24i32;
```

### `CAN_ERR_PROT_LOC_ACK`
```rust
const CAN_ERR_PROT_LOC_ACK: c_int = 25i32;
```

### `CAN_ERR_PROT_LOC_ACK_DEL`
```rust
const CAN_ERR_PROT_LOC_ACK_DEL: c_int = 27i32;
```

### `CAN_ERR_PROT_LOC_EOF`
```rust
const CAN_ERR_PROT_LOC_EOF: c_int = 26i32;
```

### `CAN_ERR_PROT_LOC_INTERM`
```rust
const CAN_ERR_PROT_LOC_INTERM: c_int = 18i32;
```

### `CAN_ERR_TRX_UNSPEC`
```rust
const CAN_ERR_TRX_UNSPEC: c_int = 0i32;
```

### `CAN_ERR_TRX_CANH_NO_WIRE`
```rust
const CAN_ERR_TRX_CANH_NO_WIRE: c_int = 4i32;
```

### `CAN_ERR_TRX_CANH_SHORT_TO_BAT`
```rust
const CAN_ERR_TRX_CANH_SHORT_TO_BAT: c_int = 5i32;
```

### `CAN_ERR_TRX_CANH_SHORT_TO_VCC`
```rust
const CAN_ERR_TRX_CANH_SHORT_TO_VCC: c_int = 6i32;
```

### `CAN_ERR_TRX_CANH_SHORT_TO_GND`
```rust
const CAN_ERR_TRX_CANH_SHORT_TO_GND: c_int = 7i32;
```

### `CAN_ERR_TRX_CANL_NO_WIRE`
```rust
const CAN_ERR_TRX_CANL_NO_WIRE: c_int = 64i32;
```

### `CAN_ERR_TRX_CANL_SHORT_TO_BAT`
```rust
const CAN_ERR_TRX_CANL_SHORT_TO_BAT: c_int = 80i32;
```

### `CAN_ERR_TRX_CANL_SHORT_TO_VCC`
```rust
const CAN_ERR_TRX_CANL_SHORT_TO_VCC: c_int = 96i32;
```

### `CAN_ERR_TRX_CANL_SHORT_TO_GND`
```rust
const CAN_ERR_TRX_CANL_SHORT_TO_GND: c_int = 112i32;
```

### `CAN_ERR_TRX_CANL_SHORT_TO_CANH`
```rust
const CAN_ERR_TRX_CANL_SHORT_TO_CANH: c_int = 128i32;
```

### `CAN_ERROR_WARNING_THRESHOLD`
```rust
const CAN_ERROR_WARNING_THRESHOLD: c_int = 96i32;
```

### `CAN_ERROR_PASSIVE_THRESHOLD`
```rust
const CAN_ERROR_PASSIVE_THRESHOLD: c_int = 128i32;
```

### `CAN_BUS_OFF_THRESHOLD`
```rust
const CAN_BUS_OFF_THRESHOLD: c_int = 256i32;
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

