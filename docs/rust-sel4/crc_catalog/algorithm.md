**crc_catalog > algorithm**

# Module: algorithm

## Contents

**Constants**

- [`CRC_10_ATM`](#crc_10_atm)
- [`CRC_10_CDMA2000`](#crc_10_cdma2000)
- [`CRC_10_GSM`](#crc_10_gsm)
- [`CRC_11_FLEXRAY`](#crc_11_flexray)
- [`CRC_11_UMTS`](#crc_11_umts)
- [`CRC_12_CDMA2000`](#crc_12_cdma2000)
- [`CRC_12_DECT`](#crc_12_dect)
- [`CRC_12_GSM`](#crc_12_gsm)
- [`CRC_12_UMTS`](#crc_12_umts)
- [`CRC_13_BBC`](#crc_13_bbc)
- [`CRC_14_DARC`](#crc_14_darc)
- [`CRC_14_GSM`](#crc_14_gsm)
- [`CRC_15_CAN`](#crc_15_can)
- [`CRC_15_MPT1327`](#crc_15_mpt1327)
- [`CRC_16_ARC`](#crc_16_arc)
- [`CRC_16_CDMA2000`](#crc_16_cdma2000)
- [`CRC_16_CMS`](#crc_16_cms)
- [`CRC_16_DDS_110`](#crc_16_dds_110)
- [`CRC_16_DECT_R`](#crc_16_dect_r)
- [`CRC_16_DECT_X`](#crc_16_dect_x)
- [`CRC_16_DNP`](#crc_16_dnp)
- [`CRC_16_EN_13757`](#crc_16_en_13757)
- [`CRC_16_GENIBUS`](#crc_16_genibus)
- [`CRC_16_GSM`](#crc_16_gsm)
- [`CRC_16_IBM_3740`](#crc_16_ibm_3740)
- [`CRC_16_IBM_SDLC`](#crc_16_ibm_sdlc)
- [`CRC_16_ISO_IEC_14443_3_A`](#crc_16_iso_iec_14443_3_a)
- [`CRC_16_KERMIT`](#crc_16_kermit)
- [`CRC_16_LJ1200`](#crc_16_lj1200)
- [`CRC_16_M17`](#crc_16_m17)
- [`CRC_16_MAXIM_DOW`](#crc_16_maxim_dow)
- [`CRC_16_MCRF4XX`](#crc_16_mcrf4xx)
- [`CRC_16_MODBUS`](#crc_16_modbus)
- [`CRC_16_NRSC_5`](#crc_16_nrsc_5)
- [`CRC_16_OPENSAFETY_A`](#crc_16_opensafety_a)
- [`CRC_16_OPENSAFETY_B`](#crc_16_opensafety_b)
- [`CRC_16_PROFIBUS`](#crc_16_profibus)
- [`CRC_16_RIELLO`](#crc_16_riello)
- [`CRC_16_SPI_FUJITSU`](#crc_16_spi_fujitsu)
- [`CRC_16_T10_DIF`](#crc_16_t10_dif)
- [`CRC_16_TELEDISK`](#crc_16_teledisk)
- [`CRC_16_TMS37157`](#crc_16_tms37157)
- [`CRC_16_UMTS`](#crc_16_umts)
- [`CRC_16_USB`](#crc_16_usb)
- [`CRC_16_XMODEM`](#crc_16_xmodem)
- [`CRC_17_CAN_FD`](#crc_17_can_fd)
- [`CRC_21_CAN_FD`](#crc_21_can_fd)
- [`CRC_24_BLE`](#crc_24_ble)
- [`CRC_24_FLEXRAY_A`](#crc_24_flexray_a)
- [`CRC_24_FLEXRAY_B`](#crc_24_flexray_b)
- [`CRC_24_INTERLAKEN`](#crc_24_interlaken)
- [`CRC_24_LTE_A`](#crc_24_lte_a)
- [`CRC_24_LTE_B`](#crc_24_lte_b)
- [`CRC_24_OPENPGP`](#crc_24_openpgp)
- [`CRC_24_OS_9`](#crc_24_os_9)
- [`CRC_30_CDMA`](#crc_30_cdma)
- [`CRC_31_PHILIPS`](#crc_31_philips)
- [`CRC_32_AIXM`](#crc_32_aixm)
- [`CRC_32_AUTOSAR`](#crc_32_autosar)
- [`CRC_32_BASE91_D`](#crc_32_base91_d)
- [`CRC_32_BZIP2`](#crc_32_bzip2)
- [`CRC_32_CD_ROM_EDC`](#crc_32_cd_rom_edc)
- [`CRC_32_CKSUM`](#crc_32_cksum)
- [`CRC_32_ISCSI`](#crc_32_iscsi)
- [`CRC_32_ISO_HDLC`](#crc_32_iso_hdlc)
- [`CRC_32_JAMCRC`](#crc_32_jamcrc)
- [`CRC_32_MEF`](#crc_32_mef)
- [`CRC_32_MPEG_2`](#crc_32_mpeg_2)
- [`CRC_32_XFER`](#crc_32_xfer)
- [`CRC_3_GSM`](#crc_3_gsm)
- [`CRC_3_ROHC`](#crc_3_rohc)
- [`CRC_40_GSM`](#crc_40_gsm)
- [`CRC_4_G_704`](#crc_4_g_704)
- [`CRC_4_INTERLAKEN`](#crc_4_interlaken)
- [`CRC_5_EPC_C1G2`](#crc_5_epc_c1g2)
- [`CRC_5_G_704`](#crc_5_g_704)
- [`CRC_5_USB`](#crc_5_usb)
- [`CRC_64_ECMA_182`](#crc_64_ecma_182)
- [`CRC_64_GO_ISO`](#crc_64_go_iso)
- [`CRC_64_MS`](#crc_64_ms)
- [`CRC_64_REDIS`](#crc_64_redis)
- [`CRC_64_WE`](#crc_64_we)
- [`CRC_64_XZ`](#crc_64_xz)
- [`CRC_6_CDMA2000_A`](#crc_6_cdma2000_a)
- [`CRC_6_CDMA2000_B`](#crc_6_cdma2000_b)
- [`CRC_6_DARC`](#crc_6_darc)
- [`CRC_6_GSM`](#crc_6_gsm)
- [`CRC_6_G_704`](#crc_6_g_704)
- [`CRC_7_MMC`](#crc_7_mmc)
- [`CRC_7_ROHC`](#crc_7_rohc)
- [`CRC_7_UMTS`](#crc_7_umts)
- [`CRC_82_DARC`](#crc_82_darc)
- [`CRC_8_AUTOSAR`](#crc_8_autosar)
- [`CRC_8_BLUETOOTH`](#crc_8_bluetooth)
- [`CRC_8_CDMA2000`](#crc_8_cdma2000)
- [`CRC_8_DARC`](#crc_8_darc)
- [`CRC_8_DVB_S2`](#crc_8_dvb_s2)
- [`CRC_8_GSM_A`](#crc_8_gsm_a)
- [`CRC_8_GSM_B`](#crc_8_gsm_b)
- [`CRC_8_HITAG`](#crc_8_hitag)
- [`CRC_8_I_432_1`](#crc_8_i_432_1)
- [`CRC_8_I_CODE`](#crc_8_i_code)
- [`CRC_8_LTE`](#crc_8_lte)
- [`CRC_8_MAXIM_DOW`](#crc_8_maxim_dow)
- [`CRC_8_MIFARE_MAD`](#crc_8_mifare_mad)
- [`CRC_8_NRSC_5`](#crc_8_nrsc_5)
- [`CRC_8_OPENSAFETY`](#crc_8_opensafety)
- [`CRC_8_ROHC`](#crc_8_rohc)
- [`CRC_8_SAE_J1850`](#crc_8_sae_j1850)
- [`CRC_8_SMBUS`](#crc_8_smbus)
- [`CRC_8_TECH_3250`](#crc_8_tech_3250)
- [`CRC_8_WCDMA`](#crc_8_wcdma)

---

## crc_catalog::algorithm::CRC_10_ATM

*Constant*: `crate::Algorithm<u16>`



## crc_catalog::algorithm::CRC_10_CDMA2000

*Constant*: `crate::Algorithm<u16>`



## crc_catalog::algorithm::CRC_10_GSM

*Constant*: `crate::Algorithm<u16>`



## crc_catalog::algorithm::CRC_11_FLEXRAY

*Constant*: `crate::Algorithm<u16>`



## crc_catalog::algorithm::CRC_11_UMTS

*Constant*: `crate::Algorithm<u16>`



## crc_catalog::algorithm::CRC_12_CDMA2000

*Constant*: `crate::Algorithm<u16>`



## crc_catalog::algorithm::CRC_12_DECT

*Constant*: `crate::Algorithm<u16>`



## crc_catalog::algorithm::CRC_12_GSM

*Constant*: `crate::Algorithm<u16>`



## crc_catalog::algorithm::CRC_12_UMTS

*Constant*: `crate::Algorithm<u16>`



## crc_catalog::algorithm::CRC_13_BBC

*Constant*: `crate::Algorithm<u16>`



## crc_catalog::algorithm::CRC_14_DARC

*Constant*: `crate::Algorithm<u16>`



## crc_catalog::algorithm::CRC_14_GSM

*Constant*: `crate::Algorithm<u16>`



## crc_catalog::algorithm::CRC_15_CAN

*Constant*: `crate::Algorithm<u16>`



## crc_catalog::algorithm::CRC_15_MPT1327

*Constant*: `crate::Algorithm<u16>`



## crc_catalog::algorithm::CRC_16_ARC

*Constant*: `crate::Algorithm<u16>`



## crc_catalog::algorithm::CRC_16_CDMA2000

*Constant*: `crate::Algorithm<u16>`



## crc_catalog::algorithm::CRC_16_CMS

*Constant*: `crate::Algorithm<u16>`



## crc_catalog::algorithm::CRC_16_DDS_110

*Constant*: `crate::Algorithm<u16>`



## crc_catalog::algorithm::CRC_16_DECT_R

*Constant*: `crate::Algorithm<u16>`



## crc_catalog::algorithm::CRC_16_DECT_X

*Constant*: `crate::Algorithm<u16>`



## crc_catalog::algorithm::CRC_16_DNP

*Constant*: `crate::Algorithm<u16>`



## crc_catalog::algorithm::CRC_16_EN_13757

*Constant*: `crate::Algorithm<u16>`



## crc_catalog::algorithm::CRC_16_GENIBUS

*Constant*: `crate::Algorithm<u16>`



## crc_catalog::algorithm::CRC_16_GSM

*Constant*: `crate::Algorithm<u16>`



## crc_catalog::algorithm::CRC_16_IBM_3740

*Constant*: `crate::Algorithm<u16>`



## crc_catalog::algorithm::CRC_16_IBM_SDLC

*Constant*: `crate::Algorithm<u16>`



## crc_catalog::algorithm::CRC_16_ISO_IEC_14443_3_A

*Constant*: `crate::Algorithm<u16>`



## crc_catalog::algorithm::CRC_16_KERMIT

*Constant*: `crate::Algorithm<u16>`



## crc_catalog::algorithm::CRC_16_LJ1200

*Constant*: `crate::Algorithm<u16>`



## crc_catalog::algorithm::CRC_16_M17

*Constant*: `crate::Algorithm<u16>`



## crc_catalog::algorithm::CRC_16_MAXIM_DOW

*Constant*: `crate::Algorithm<u16>`



## crc_catalog::algorithm::CRC_16_MCRF4XX

*Constant*: `crate::Algorithm<u16>`



## crc_catalog::algorithm::CRC_16_MODBUS

*Constant*: `crate::Algorithm<u16>`



## crc_catalog::algorithm::CRC_16_NRSC_5

*Constant*: `crate::Algorithm<u16>`



## crc_catalog::algorithm::CRC_16_OPENSAFETY_A

*Constant*: `crate::Algorithm<u16>`



## crc_catalog::algorithm::CRC_16_OPENSAFETY_B

*Constant*: `crate::Algorithm<u16>`



## crc_catalog::algorithm::CRC_16_PROFIBUS

*Constant*: `crate::Algorithm<u16>`



## crc_catalog::algorithm::CRC_16_RIELLO

*Constant*: `crate::Algorithm<u16>`



## crc_catalog::algorithm::CRC_16_SPI_FUJITSU

*Constant*: `crate::Algorithm<u16>`



## crc_catalog::algorithm::CRC_16_T10_DIF

*Constant*: `crate::Algorithm<u16>`



## crc_catalog::algorithm::CRC_16_TELEDISK

*Constant*: `crate::Algorithm<u16>`



## crc_catalog::algorithm::CRC_16_TMS37157

*Constant*: `crate::Algorithm<u16>`



## crc_catalog::algorithm::CRC_16_UMTS

*Constant*: `crate::Algorithm<u16>`



## crc_catalog::algorithm::CRC_16_USB

*Constant*: `crate::Algorithm<u16>`



## crc_catalog::algorithm::CRC_16_XMODEM

*Constant*: `crate::Algorithm<u16>`



## crc_catalog::algorithm::CRC_17_CAN_FD

*Constant*: `crate::Algorithm<u32>`



## crc_catalog::algorithm::CRC_21_CAN_FD

*Constant*: `crate::Algorithm<u32>`



## crc_catalog::algorithm::CRC_24_BLE

*Constant*: `crate::Algorithm<u32>`



## crc_catalog::algorithm::CRC_24_FLEXRAY_A

*Constant*: `crate::Algorithm<u32>`



## crc_catalog::algorithm::CRC_24_FLEXRAY_B

*Constant*: `crate::Algorithm<u32>`



## crc_catalog::algorithm::CRC_24_INTERLAKEN

*Constant*: `crate::Algorithm<u32>`



## crc_catalog::algorithm::CRC_24_LTE_A

*Constant*: `crate::Algorithm<u32>`



## crc_catalog::algorithm::CRC_24_LTE_B

*Constant*: `crate::Algorithm<u32>`



## crc_catalog::algorithm::CRC_24_OPENPGP

*Constant*: `crate::Algorithm<u32>`



## crc_catalog::algorithm::CRC_24_OS_9

*Constant*: `crate::Algorithm<u32>`



## crc_catalog::algorithm::CRC_30_CDMA

*Constant*: `crate::Algorithm<u32>`



## crc_catalog::algorithm::CRC_31_PHILIPS

*Constant*: `crate::Algorithm<u32>`



## crc_catalog::algorithm::CRC_32_AIXM

*Constant*: `crate::Algorithm<u32>`



## crc_catalog::algorithm::CRC_32_AUTOSAR

*Constant*: `crate::Algorithm<u32>`



## crc_catalog::algorithm::CRC_32_BASE91_D

*Constant*: `crate::Algorithm<u32>`



## crc_catalog::algorithm::CRC_32_BZIP2

*Constant*: `crate::Algorithm<u32>`



## crc_catalog::algorithm::CRC_32_CD_ROM_EDC

*Constant*: `crate::Algorithm<u32>`



## crc_catalog::algorithm::CRC_32_CKSUM

*Constant*: `crate::Algorithm<u32>`



## crc_catalog::algorithm::CRC_32_ISCSI

*Constant*: `crate::Algorithm<u32>`



## crc_catalog::algorithm::CRC_32_ISO_HDLC

*Constant*: `crate::Algorithm<u32>`



## crc_catalog::algorithm::CRC_32_JAMCRC

*Constant*: `crate::Algorithm<u32>`



## crc_catalog::algorithm::CRC_32_MEF

*Constant*: `crate::Algorithm<u32>`



## crc_catalog::algorithm::CRC_32_MPEG_2

*Constant*: `crate::Algorithm<u32>`



## crc_catalog::algorithm::CRC_32_XFER

*Constant*: `crate::Algorithm<u32>`



## crc_catalog::algorithm::CRC_3_GSM

*Constant*: `crate::Algorithm<u8>`



## crc_catalog::algorithm::CRC_3_ROHC

*Constant*: `crate::Algorithm<u8>`



## crc_catalog::algorithm::CRC_40_GSM

*Constant*: `crate::Algorithm<u64>`



## crc_catalog::algorithm::CRC_4_G_704

*Constant*: `crate::Algorithm<u8>`



## crc_catalog::algorithm::CRC_4_INTERLAKEN

*Constant*: `crate::Algorithm<u8>`



## crc_catalog::algorithm::CRC_5_EPC_C1G2

*Constant*: `crate::Algorithm<u8>`



## crc_catalog::algorithm::CRC_5_G_704

*Constant*: `crate::Algorithm<u8>`



## crc_catalog::algorithm::CRC_5_USB

*Constant*: `crate::Algorithm<u8>`



## crc_catalog::algorithm::CRC_64_ECMA_182

*Constant*: `crate::Algorithm<u64>`



## crc_catalog::algorithm::CRC_64_GO_ISO

*Constant*: `crate::Algorithm<u64>`



## crc_catalog::algorithm::CRC_64_MS

*Constant*: `crate::Algorithm<u64>`



## crc_catalog::algorithm::CRC_64_REDIS

*Constant*: `crate::Algorithm<u64>`



## crc_catalog::algorithm::CRC_64_WE

*Constant*: `crate::Algorithm<u64>`



## crc_catalog::algorithm::CRC_64_XZ

*Constant*: `crate::Algorithm<u64>`



## crc_catalog::algorithm::CRC_6_CDMA2000_A

*Constant*: `crate::Algorithm<u8>`



## crc_catalog::algorithm::CRC_6_CDMA2000_B

*Constant*: `crate::Algorithm<u8>`



## crc_catalog::algorithm::CRC_6_DARC

*Constant*: `crate::Algorithm<u8>`



## crc_catalog::algorithm::CRC_6_GSM

*Constant*: `crate::Algorithm<u8>`



## crc_catalog::algorithm::CRC_6_G_704

*Constant*: `crate::Algorithm<u8>`



## crc_catalog::algorithm::CRC_7_MMC

*Constant*: `crate::Algorithm<u8>`



## crc_catalog::algorithm::CRC_7_ROHC

*Constant*: `crate::Algorithm<u8>`



## crc_catalog::algorithm::CRC_7_UMTS

*Constant*: `crate::Algorithm<u8>`



## crc_catalog::algorithm::CRC_82_DARC

*Constant*: `crate::Algorithm<u128>`



## crc_catalog::algorithm::CRC_8_AUTOSAR

*Constant*: `crate::Algorithm<u8>`



## crc_catalog::algorithm::CRC_8_BLUETOOTH

*Constant*: `crate::Algorithm<u8>`



## crc_catalog::algorithm::CRC_8_CDMA2000

*Constant*: `crate::Algorithm<u8>`



## crc_catalog::algorithm::CRC_8_DARC

*Constant*: `crate::Algorithm<u8>`



## crc_catalog::algorithm::CRC_8_DVB_S2

*Constant*: `crate::Algorithm<u8>`



## crc_catalog::algorithm::CRC_8_GSM_A

*Constant*: `crate::Algorithm<u8>`



## crc_catalog::algorithm::CRC_8_GSM_B

*Constant*: `crate::Algorithm<u8>`



## crc_catalog::algorithm::CRC_8_HITAG

*Constant*: `crate::Algorithm<u8>`



## crc_catalog::algorithm::CRC_8_I_432_1

*Constant*: `crate::Algorithm<u8>`



## crc_catalog::algorithm::CRC_8_I_CODE

*Constant*: `crate::Algorithm<u8>`



## crc_catalog::algorithm::CRC_8_LTE

*Constant*: `crate::Algorithm<u8>`



## crc_catalog::algorithm::CRC_8_MAXIM_DOW

*Constant*: `crate::Algorithm<u8>`



## crc_catalog::algorithm::CRC_8_MIFARE_MAD

*Constant*: `crate::Algorithm<u8>`



## crc_catalog::algorithm::CRC_8_NRSC_5

*Constant*: `crate::Algorithm<u8>`



## crc_catalog::algorithm::CRC_8_OPENSAFETY

*Constant*: `crate::Algorithm<u8>`



## crc_catalog::algorithm::CRC_8_ROHC

*Constant*: `crate::Algorithm<u8>`



## crc_catalog::algorithm::CRC_8_SAE_J1850

*Constant*: `crate::Algorithm<u8>`



## crc_catalog::algorithm::CRC_8_SMBUS

*Constant*: `crate::Algorithm<u8>`



## crc_catalog::algorithm::CRC_8_TECH_3250

*Constant*: `crate::Algorithm<u8>`



## crc_catalog::algorithm::CRC_8_WCDMA

*Constant*: `crate::Algorithm<u8>`



