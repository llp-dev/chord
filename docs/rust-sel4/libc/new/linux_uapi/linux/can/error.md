**libc > new > linux_uapi > linux > can > error**

# Module: new::linux_uapi::linux::can::error

## Contents

**Constants**

- [`CAN_BUS_OFF_THRESHOLD`](#can_bus_off_threshold)
- [`CAN_ERROR_PASSIVE_THRESHOLD`](#can_error_passive_threshold)
- [`CAN_ERROR_WARNING_THRESHOLD`](#can_error_warning_threshold)
- [`CAN_ERR_ACK`](#can_err_ack)
- [`CAN_ERR_BUSERROR`](#can_err_buserror)
- [`CAN_ERR_BUSOFF`](#can_err_busoff)
- [`CAN_ERR_CNT`](#can_err_cnt)
- [`CAN_ERR_CRTL`](#can_err_crtl)
- [`CAN_ERR_CRTL_ACTIVE`](#can_err_crtl_active)
- [`CAN_ERR_CRTL_RX_OVERFLOW`](#can_err_crtl_rx_overflow)
- [`CAN_ERR_CRTL_RX_PASSIVE`](#can_err_crtl_rx_passive)
- [`CAN_ERR_CRTL_RX_WARNING`](#can_err_crtl_rx_warning)
- [`CAN_ERR_CRTL_TX_OVERFLOW`](#can_err_crtl_tx_overflow)
- [`CAN_ERR_CRTL_TX_PASSIVE`](#can_err_crtl_tx_passive)
- [`CAN_ERR_CRTL_TX_WARNING`](#can_err_crtl_tx_warning)
- [`CAN_ERR_CRTL_UNSPEC`](#can_err_crtl_unspec)
- [`CAN_ERR_DLC`](#can_err_dlc)
- [`CAN_ERR_LOSTARB`](#can_err_lostarb)
- [`CAN_ERR_LOSTARB_UNSPEC`](#can_err_lostarb_unspec)
- [`CAN_ERR_PROT`](#can_err_prot)
- [`CAN_ERR_PROT_ACTIVE`](#can_err_prot_active)
- [`CAN_ERR_PROT_BIT`](#can_err_prot_bit)
- [`CAN_ERR_PROT_BIT0`](#can_err_prot_bit0)
- [`CAN_ERR_PROT_BIT1`](#can_err_prot_bit1)
- [`CAN_ERR_PROT_FORM`](#can_err_prot_form)
- [`CAN_ERR_PROT_LOC_ACK`](#can_err_prot_loc_ack)
- [`CAN_ERR_PROT_LOC_ACK_DEL`](#can_err_prot_loc_ack_del)
- [`CAN_ERR_PROT_LOC_CRC_DEL`](#can_err_prot_loc_crc_del)
- [`CAN_ERR_PROT_LOC_CRC_SEQ`](#can_err_prot_loc_crc_seq)
- [`CAN_ERR_PROT_LOC_DATA`](#can_err_prot_loc_data)
- [`CAN_ERR_PROT_LOC_DLC`](#can_err_prot_loc_dlc)
- [`CAN_ERR_PROT_LOC_EOF`](#can_err_prot_loc_eof)
- [`CAN_ERR_PROT_LOC_ID04_00`](#can_err_prot_loc_id04_00)
- [`CAN_ERR_PROT_LOC_ID12_05`](#can_err_prot_loc_id12_05)
- [`CAN_ERR_PROT_LOC_ID17_13`](#can_err_prot_loc_id17_13)
- [`CAN_ERR_PROT_LOC_ID20_18`](#can_err_prot_loc_id20_18)
- [`CAN_ERR_PROT_LOC_ID28_21`](#can_err_prot_loc_id28_21)
- [`CAN_ERR_PROT_LOC_IDE`](#can_err_prot_loc_ide)
- [`CAN_ERR_PROT_LOC_INTERM`](#can_err_prot_loc_interm)
- [`CAN_ERR_PROT_LOC_RES0`](#can_err_prot_loc_res0)
- [`CAN_ERR_PROT_LOC_RES1`](#can_err_prot_loc_res1)
- [`CAN_ERR_PROT_LOC_RTR`](#can_err_prot_loc_rtr)
- [`CAN_ERR_PROT_LOC_SOF`](#can_err_prot_loc_sof)
- [`CAN_ERR_PROT_LOC_SRTR`](#can_err_prot_loc_srtr)
- [`CAN_ERR_PROT_LOC_UNSPEC`](#can_err_prot_loc_unspec)
- [`CAN_ERR_PROT_OVERLOAD`](#can_err_prot_overload)
- [`CAN_ERR_PROT_STUFF`](#can_err_prot_stuff)
- [`CAN_ERR_PROT_TX`](#can_err_prot_tx)
- [`CAN_ERR_PROT_UNSPEC`](#can_err_prot_unspec)
- [`CAN_ERR_RESTARTED`](#can_err_restarted)
- [`CAN_ERR_TRX`](#can_err_trx)
- [`CAN_ERR_TRX_CANH_NO_WIRE`](#can_err_trx_canh_no_wire)
- [`CAN_ERR_TRX_CANH_SHORT_TO_BAT`](#can_err_trx_canh_short_to_bat)
- [`CAN_ERR_TRX_CANH_SHORT_TO_GND`](#can_err_trx_canh_short_to_gnd)
- [`CAN_ERR_TRX_CANH_SHORT_TO_VCC`](#can_err_trx_canh_short_to_vcc)
- [`CAN_ERR_TRX_CANL_NO_WIRE`](#can_err_trx_canl_no_wire)
- [`CAN_ERR_TRX_CANL_SHORT_TO_BAT`](#can_err_trx_canl_short_to_bat)
- [`CAN_ERR_TRX_CANL_SHORT_TO_CANH`](#can_err_trx_canl_short_to_canh)
- [`CAN_ERR_TRX_CANL_SHORT_TO_GND`](#can_err_trx_canl_short_to_gnd)
- [`CAN_ERR_TRX_CANL_SHORT_TO_VCC`](#can_err_trx_canl_short_to_vcc)
- [`CAN_ERR_TRX_UNSPEC`](#can_err_trx_unspec)
- [`CAN_ERR_TX_TIMEOUT`](#can_err_tx_timeout)

---

## libc::new::linux_uapi::linux::can::error::CAN_BUS_OFF_THRESHOLD

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::error::CAN_ERROR_PASSIVE_THRESHOLD

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::error::CAN_ERROR_WARNING_THRESHOLD

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_ACK

*Constant*: `c_uint`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_BUSERROR

*Constant*: `c_uint`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_BUSOFF

*Constant*: `c_uint`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_CNT

*Constant*: `c_uint`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_CRTL

*Constant*: `c_uint`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_CRTL_ACTIVE

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_CRTL_RX_OVERFLOW

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_CRTL_RX_PASSIVE

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_CRTL_RX_WARNING

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_CRTL_TX_OVERFLOW

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_CRTL_TX_PASSIVE

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_CRTL_TX_WARNING

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_CRTL_UNSPEC

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_DLC

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_LOSTARB

*Constant*: `c_uint`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_LOSTARB_UNSPEC

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_PROT

*Constant*: `c_uint`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_PROT_ACTIVE

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_PROT_BIT

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_PROT_BIT0

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_PROT_BIT1

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_PROT_FORM

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_PROT_LOC_ACK

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_PROT_LOC_ACK_DEL

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_PROT_LOC_CRC_DEL

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_PROT_LOC_CRC_SEQ

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_PROT_LOC_DATA

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_PROT_LOC_DLC

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_PROT_LOC_EOF

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_PROT_LOC_ID04_00

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_PROT_LOC_ID12_05

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_PROT_LOC_ID17_13

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_PROT_LOC_ID20_18

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_PROT_LOC_ID28_21

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_PROT_LOC_IDE

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_PROT_LOC_INTERM

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_PROT_LOC_RES0

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_PROT_LOC_RES1

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_PROT_LOC_RTR

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_PROT_LOC_SOF

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_PROT_LOC_SRTR

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_PROT_LOC_UNSPEC

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_PROT_OVERLOAD

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_PROT_STUFF

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_PROT_TX

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_PROT_UNSPEC

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_RESTARTED

*Constant*: `c_uint`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_TRX

*Constant*: `c_uint`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_TRX_CANH_NO_WIRE

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_TRX_CANH_SHORT_TO_BAT

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_TRX_CANH_SHORT_TO_GND

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_TRX_CANH_SHORT_TO_VCC

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_TRX_CANL_NO_WIRE

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_TRX_CANL_SHORT_TO_BAT

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_TRX_CANL_SHORT_TO_CANH

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_TRX_CANL_SHORT_TO_GND

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_TRX_CANL_SHORT_TO_VCC

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_TRX_UNSPEC

*Constant*: `c_int`



## libc::new::linux_uapi::linux::can::error::CAN_ERR_TX_TIMEOUT

*Constant*: `c_uint`



