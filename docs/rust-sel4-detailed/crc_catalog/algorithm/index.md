*[crc_catalog](../index.md) / [algorithm](index.md)*

---

# Module `algorithm`

CRC algorithms as structs.

## Contents

- [Constants](#constants)
  - [`CRC_3_GSM`](#crc-3-gsm)
  - [`CRC_3_ROHC`](#crc-3-rohc)
  - [`CRC_4_G_704`](#crc-4-g-704)
  - [`CRC_4_INTERLAKEN`](#crc-4-interlaken)
  - [`CRC_5_EPC_C1G2`](#crc-5-epc-c1g2)
  - [`CRC_5_G_704`](#crc-5-g-704)
  - [`CRC_5_USB`](#crc-5-usb)
  - [`CRC_6_CDMA2000_A`](#crc-6-cdma2000-a)
  - [`CRC_6_CDMA2000_B`](#crc-6-cdma2000-b)
  - [`CRC_6_DARC`](#crc-6-darc)
  - [`CRC_6_G_704`](#crc-6-g-704)
  - [`CRC_6_GSM`](#crc-6-gsm)
  - [`CRC_7_MMC`](#crc-7-mmc)
  - [`CRC_7_ROHC`](#crc-7-rohc)
  - [`CRC_7_UMTS`](#crc-7-umts)
  - [`CRC_8_AUTOSAR`](#crc-8-autosar)
  - [`CRC_8_BLUETOOTH`](#crc-8-bluetooth)
  - [`CRC_8_CDMA2000`](#crc-8-cdma2000)
  - [`CRC_8_DARC`](#crc-8-darc)
  - [`CRC_8_DVB_S2`](#crc-8-dvb-s2)
  - [`CRC_8_GSM_A`](#crc-8-gsm-a)
  - [`CRC_8_GSM_B`](#crc-8-gsm-b)
  - [`CRC_8_HITAG`](#crc-8-hitag)
  - [`CRC_8_I_432_1`](#crc-8-i-432-1)
  - [`CRC_8_I_CODE`](#crc-8-i-code)
  - [`CRC_8_LTE`](#crc-8-lte)
  - [`CRC_8_MAXIM_DOW`](#crc-8-maxim-dow)
  - [`CRC_8_MIFARE_MAD`](#crc-8-mifare-mad)
  - [`CRC_8_NRSC_5`](#crc-8-nrsc-5)
  - [`CRC_8_OPENSAFETY`](#crc-8-opensafety)
  - [`CRC_8_ROHC`](#crc-8-rohc)
  - [`CRC_8_SAE_J1850`](#crc-8-sae-j1850)
  - [`CRC_8_SMBUS`](#crc-8-smbus)
  - [`CRC_8_TECH_3250`](#crc-8-tech-3250)
  - [`CRC_8_WCDMA`](#crc-8-wcdma)
  - [`CRC_10_ATM`](#crc-10-atm)
  - [`CRC_10_CDMA2000`](#crc-10-cdma2000)
  - [`CRC_10_GSM`](#crc-10-gsm)
  - [`CRC_11_FLEXRAY`](#crc-11-flexray)
  - [`CRC_11_UMTS`](#crc-11-umts)
  - [`CRC_12_CDMA2000`](#crc-12-cdma2000)
  - [`CRC_12_DECT`](#crc-12-dect)
  - [`CRC_12_GSM`](#crc-12-gsm)
  - [`CRC_12_UMTS`](#crc-12-umts)
  - [`CRC_13_BBC`](#crc-13-bbc)
  - [`CRC_14_DARC`](#crc-14-darc)
  - [`CRC_14_GSM`](#crc-14-gsm)
  - [`CRC_15_CAN`](#crc-15-can)
  - [`CRC_15_MPT1327`](#crc-15-mpt1327)
  - [`CRC_16_ARC`](#crc-16-arc)
  - [`CRC_16_CDMA2000`](#crc-16-cdma2000)
  - [`CRC_16_CMS`](#crc-16-cms)
  - [`CRC_16_DDS_110`](#crc-16-dds-110)
  - [`CRC_16_DECT_R`](#crc-16-dect-r)
  - [`CRC_16_DECT_X`](#crc-16-dect-x)
  - [`CRC_16_DNP`](#crc-16-dnp)
  - [`CRC_16_EN_13757`](#crc-16-en-13757)
  - [`CRC_16_GENIBUS`](#crc-16-genibus)
  - [`CRC_16_GSM`](#crc-16-gsm)
  - [`CRC_16_IBM_3740`](#crc-16-ibm-3740)
  - [`CRC_16_IBM_SDLC`](#crc-16-ibm-sdlc)
  - [`CRC_16_ISO_IEC_14443_3_A`](#crc-16-iso-iec-14443-3-a)
  - [`CRC_16_KERMIT`](#crc-16-kermit)
  - [`CRC_16_LJ1200`](#crc-16-lj1200)
  - [`CRC_16_M17`](#crc-16-m17)
  - [`CRC_16_MAXIM_DOW`](#crc-16-maxim-dow)
  - [`CRC_16_MCRF4XX`](#crc-16-mcrf4xx)
  - [`CRC_16_MODBUS`](#crc-16-modbus)
  - [`CRC_16_NRSC_5`](#crc-16-nrsc-5)
  - [`CRC_16_OPENSAFETY_A`](#crc-16-opensafety-a)
  - [`CRC_16_OPENSAFETY_B`](#crc-16-opensafety-b)
  - [`CRC_16_PROFIBUS`](#crc-16-profibus)
  - [`CRC_16_RIELLO`](#crc-16-riello)
  - [`CRC_16_SPI_FUJITSU`](#crc-16-spi-fujitsu)
  - [`CRC_16_T10_DIF`](#crc-16-t10-dif)
  - [`CRC_16_TELEDISK`](#crc-16-teledisk)
  - [`CRC_16_TMS37157`](#crc-16-tms37157)
  - [`CRC_16_UMTS`](#crc-16-umts)
  - [`CRC_16_USB`](#crc-16-usb)
  - [`CRC_16_XMODEM`](#crc-16-xmodem)
  - [`CRC_17_CAN_FD`](#crc-17-can-fd)
  - [`CRC_21_CAN_FD`](#crc-21-can-fd)
  - [`CRC_24_BLE`](#crc-24-ble)
  - [`CRC_24_FLEXRAY_A`](#crc-24-flexray-a)
  - [`CRC_24_FLEXRAY_B`](#crc-24-flexray-b)
  - [`CRC_24_INTERLAKEN`](#crc-24-interlaken)
  - [`CRC_24_LTE_A`](#crc-24-lte-a)
  - [`CRC_24_LTE_B`](#crc-24-lte-b)
  - [`CRC_24_OPENPGP`](#crc-24-openpgp)
  - [`CRC_24_OS_9`](#crc-24-os-9)
  - [`CRC_30_CDMA`](#crc-30-cdma)
  - [`CRC_31_PHILIPS`](#crc-31-philips)
  - [`CRC_32_AIXM`](#crc-32-aixm)
  - [`CRC_32_AUTOSAR`](#crc-32-autosar)
  - [`CRC_32_BASE91_D`](#crc-32-base91-d)
  - [`CRC_32_BZIP2`](#crc-32-bzip2)
  - [`CRC_32_CD_ROM_EDC`](#crc-32-cd-rom-edc)
  - [`CRC_32_CKSUM`](#crc-32-cksum)
  - [`CRC_32_ISCSI`](#crc-32-iscsi)
  - [`CRC_32_ISO_HDLC`](#crc-32-iso-hdlc)
  - [`CRC_32_JAMCRC`](#crc-32-jamcrc)
  - [`CRC_32_MEF`](#crc-32-mef)
  - [`CRC_32_MPEG_2`](#crc-32-mpeg-2)
  - [`CRC_32_XFER`](#crc-32-xfer)
  - [`CRC_40_GSM`](#crc-40-gsm)
  - [`CRC_64_ECMA_182`](#crc-64-ecma-182)
  - [`CRC_64_GO_ISO`](#crc-64-go-iso)
  - [`CRC_64_MS`](#crc-64-ms)
  - [`CRC_64_REDIS`](#crc-64-redis)
  - [`CRC_64_WE`](#crc-64-we)
  - [`CRC_64_XZ`](#crc-64-xz)
  - [`CRC_82_DARC`](#crc-82-darc)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`CRC_3_GSM`](#crc-3-gsm) | const |  |
| [`CRC_3_ROHC`](#crc-3-rohc) | const |  |
| [`CRC_4_G_704`](#crc-4-g-704) | const |  |
| [`CRC_4_INTERLAKEN`](#crc-4-interlaken) | const |  |
| [`CRC_5_EPC_C1G2`](#crc-5-epc-c1g2) | const |  |
| [`CRC_5_G_704`](#crc-5-g-704) | const |  |
| [`CRC_5_USB`](#crc-5-usb) | const |  |
| [`CRC_6_CDMA2000_A`](#crc-6-cdma2000-a) | const |  |
| [`CRC_6_CDMA2000_B`](#crc-6-cdma2000-b) | const |  |
| [`CRC_6_DARC`](#crc-6-darc) | const |  |
| [`CRC_6_G_704`](#crc-6-g-704) | const |  |
| [`CRC_6_GSM`](#crc-6-gsm) | const |  |
| [`CRC_7_MMC`](#crc-7-mmc) | const |  |
| [`CRC_7_ROHC`](#crc-7-rohc) | const |  |
| [`CRC_7_UMTS`](#crc-7-umts) | const |  |
| [`CRC_8_AUTOSAR`](#crc-8-autosar) | const |  |
| [`CRC_8_BLUETOOTH`](#crc-8-bluetooth) | const |  |
| [`CRC_8_CDMA2000`](#crc-8-cdma2000) | const |  |
| [`CRC_8_DARC`](#crc-8-darc) | const |  |
| [`CRC_8_DVB_S2`](#crc-8-dvb-s2) | const |  |
| [`CRC_8_GSM_A`](#crc-8-gsm-a) | const |  |
| [`CRC_8_GSM_B`](#crc-8-gsm-b) | const |  |
| [`CRC_8_HITAG`](#crc-8-hitag) | const |  |
| [`CRC_8_I_432_1`](#crc-8-i-432-1) | const |  |
| [`CRC_8_I_CODE`](#crc-8-i-code) | const |  |
| [`CRC_8_LTE`](#crc-8-lte) | const |  |
| [`CRC_8_MAXIM_DOW`](#crc-8-maxim-dow) | const |  |
| [`CRC_8_MIFARE_MAD`](#crc-8-mifare-mad) | const |  |
| [`CRC_8_NRSC_5`](#crc-8-nrsc-5) | const |  |
| [`CRC_8_OPENSAFETY`](#crc-8-opensafety) | const |  |
| [`CRC_8_ROHC`](#crc-8-rohc) | const |  |
| [`CRC_8_SAE_J1850`](#crc-8-sae-j1850) | const |  |
| [`CRC_8_SMBUS`](#crc-8-smbus) | const |  |
| [`CRC_8_TECH_3250`](#crc-8-tech-3250) | const |  |
| [`CRC_8_WCDMA`](#crc-8-wcdma) | const |  |
| [`CRC_10_ATM`](#crc-10-atm) | const |  |
| [`CRC_10_CDMA2000`](#crc-10-cdma2000) | const |  |
| [`CRC_10_GSM`](#crc-10-gsm) | const |  |
| [`CRC_11_FLEXRAY`](#crc-11-flexray) | const |  |
| [`CRC_11_UMTS`](#crc-11-umts) | const |  |
| [`CRC_12_CDMA2000`](#crc-12-cdma2000) | const |  |
| [`CRC_12_DECT`](#crc-12-dect) | const |  |
| [`CRC_12_GSM`](#crc-12-gsm) | const |  |
| [`CRC_12_UMTS`](#crc-12-umts) | const |  |
| [`CRC_13_BBC`](#crc-13-bbc) | const |  |
| [`CRC_14_DARC`](#crc-14-darc) | const |  |
| [`CRC_14_GSM`](#crc-14-gsm) | const |  |
| [`CRC_15_CAN`](#crc-15-can) | const |  |
| [`CRC_15_MPT1327`](#crc-15-mpt1327) | const |  |
| [`CRC_16_ARC`](#crc-16-arc) | const |  |
| [`CRC_16_CDMA2000`](#crc-16-cdma2000) | const |  |
| [`CRC_16_CMS`](#crc-16-cms) | const |  |
| [`CRC_16_DDS_110`](#crc-16-dds-110) | const |  |
| [`CRC_16_DECT_R`](#crc-16-dect-r) | const |  |
| [`CRC_16_DECT_X`](#crc-16-dect-x) | const |  |
| [`CRC_16_DNP`](#crc-16-dnp) | const |  |
| [`CRC_16_EN_13757`](#crc-16-en-13757) | const |  |
| [`CRC_16_GENIBUS`](#crc-16-genibus) | const |  |
| [`CRC_16_GSM`](#crc-16-gsm) | const |  |
| [`CRC_16_IBM_3740`](#crc-16-ibm-3740) | const |  |
| [`CRC_16_IBM_SDLC`](#crc-16-ibm-sdlc) | const |  |
| [`CRC_16_ISO_IEC_14443_3_A`](#crc-16-iso-iec-14443-3-a) | const |  |
| [`CRC_16_KERMIT`](#crc-16-kermit) | const |  |
| [`CRC_16_LJ1200`](#crc-16-lj1200) | const |  |
| [`CRC_16_M17`](#crc-16-m17) | const |  |
| [`CRC_16_MAXIM_DOW`](#crc-16-maxim-dow) | const |  |
| [`CRC_16_MCRF4XX`](#crc-16-mcrf4xx) | const |  |
| [`CRC_16_MODBUS`](#crc-16-modbus) | const |  |
| [`CRC_16_NRSC_5`](#crc-16-nrsc-5) | const |  |
| [`CRC_16_OPENSAFETY_A`](#crc-16-opensafety-a) | const |  |
| [`CRC_16_OPENSAFETY_B`](#crc-16-opensafety-b) | const |  |
| [`CRC_16_PROFIBUS`](#crc-16-profibus) | const |  |
| [`CRC_16_RIELLO`](#crc-16-riello) | const |  |
| [`CRC_16_SPI_FUJITSU`](#crc-16-spi-fujitsu) | const |  |
| [`CRC_16_T10_DIF`](#crc-16-t10-dif) | const |  |
| [`CRC_16_TELEDISK`](#crc-16-teledisk) | const |  |
| [`CRC_16_TMS37157`](#crc-16-tms37157) | const |  |
| [`CRC_16_UMTS`](#crc-16-umts) | const |  |
| [`CRC_16_USB`](#crc-16-usb) | const |  |
| [`CRC_16_XMODEM`](#crc-16-xmodem) | const |  |
| [`CRC_17_CAN_FD`](#crc-17-can-fd) | const |  |
| [`CRC_21_CAN_FD`](#crc-21-can-fd) | const |  |
| [`CRC_24_BLE`](#crc-24-ble) | const |  |
| [`CRC_24_FLEXRAY_A`](#crc-24-flexray-a) | const |  |
| [`CRC_24_FLEXRAY_B`](#crc-24-flexray-b) | const |  |
| [`CRC_24_INTERLAKEN`](#crc-24-interlaken) | const |  |
| [`CRC_24_LTE_A`](#crc-24-lte-a) | const |  |
| [`CRC_24_LTE_B`](#crc-24-lte-b) | const |  |
| [`CRC_24_OPENPGP`](#crc-24-openpgp) | const |  |
| [`CRC_24_OS_9`](#crc-24-os-9) | const |  |
| [`CRC_30_CDMA`](#crc-30-cdma) | const |  |
| [`CRC_31_PHILIPS`](#crc-31-philips) | const |  |
| [`CRC_32_AIXM`](#crc-32-aixm) | const |  |
| [`CRC_32_AUTOSAR`](#crc-32-autosar) | const |  |
| [`CRC_32_BASE91_D`](#crc-32-base91-d) | const |  |
| [`CRC_32_BZIP2`](#crc-32-bzip2) | const |  |
| [`CRC_32_CD_ROM_EDC`](#crc-32-cd-rom-edc) | const |  |
| [`CRC_32_CKSUM`](#crc-32-cksum) | const |  |
| [`CRC_32_ISCSI`](#crc-32-iscsi) | const |  |
| [`CRC_32_ISO_HDLC`](#crc-32-iso-hdlc) | const |  |
| [`CRC_32_JAMCRC`](#crc-32-jamcrc) | const |  |
| [`CRC_32_MEF`](#crc-32-mef) | const |  |
| [`CRC_32_MPEG_2`](#crc-32-mpeg-2) | const |  |
| [`CRC_32_XFER`](#crc-32-xfer) | const |  |
| [`CRC_40_GSM`](#crc-40-gsm) | const |  |
| [`CRC_64_ECMA_182`](#crc-64-ecma-182) | const |  |
| [`CRC_64_GO_ISO`](#crc-64-go-iso) | const |  |
| [`CRC_64_MS`](#crc-64-ms) | const |  |
| [`CRC_64_REDIS`](#crc-64-redis) | const |  |
| [`CRC_64_WE`](#crc-64-we) | const |  |
| [`CRC_64_XZ`](#crc-64-xz) | const |  |
| [`CRC_82_DARC`](#crc-82-darc) | const |  |

## Constants

### `CRC_3_GSM`
```rust
const CRC_3_GSM: crate::Algorithm<u8>;
```

### `CRC_3_ROHC`
```rust
const CRC_3_ROHC: crate::Algorithm<u8>;
```

### `CRC_4_G_704`
```rust
const CRC_4_G_704: crate::Algorithm<u8>;
```

### `CRC_4_INTERLAKEN`
```rust
const CRC_4_INTERLAKEN: crate::Algorithm<u8>;
```

### `CRC_5_EPC_C1G2`
```rust
const CRC_5_EPC_C1G2: crate::Algorithm<u8>;
```

### `CRC_5_G_704`
```rust
const CRC_5_G_704: crate::Algorithm<u8>;
```

### `CRC_5_USB`
```rust
const CRC_5_USB: crate::Algorithm<u8>;
```

### `CRC_6_CDMA2000_A`
```rust
const CRC_6_CDMA2000_A: crate::Algorithm<u8>;
```

### `CRC_6_CDMA2000_B`
```rust
const CRC_6_CDMA2000_B: crate::Algorithm<u8>;
```

### `CRC_6_DARC`
```rust
const CRC_6_DARC: crate::Algorithm<u8>;
```

### `CRC_6_G_704`
```rust
const CRC_6_G_704: crate::Algorithm<u8>;
```

### `CRC_6_GSM`
```rust
const CRC_6_GSM: crate::Algorithm<u8>;
```

### `CRC_7_MMC`
```rust
const CRC_7_MMC: crate::Algorithm<u8>;
```

### `CRC_7_ROHC`
```rust
const CRC_7_ROHC: crate::Algorithm<u8>;
```

### `CRC_7_UMTS`
```rust
const CRC_7_UMTS: crate::Algorithm<u8>;
```

### `CRC_8_AUTOSAR`
```rust
const CRC_8_AUTOSAR: crate::Algorithm<u8>;
```

### `CRC_8_BLUETOOTH`
```rust
const CRC_8_BLUETOOTH: crate::Algorithm<u8>;
```

### `CRC_8_CDMA2000`
```rust
const CRC_8_CDMA2000: crate::Algorithm<u8>;
```

### `CRC_8_DARC`
```rust
const CRC_8_DARC: crate::Algorithm<u8>;
```

### `CRC_8_DVB_S2`
```rust
const CRC_8_DVB_S2: crate::Algorithm<u8>;
```

### `CRC_8_GSM_A`
```rust
const CRC_8_GSM_A: crate::Algorithm<u8>;
```

### `CRC_8_GSM_B`
```rust
const CRC_8_GSM_B: crate::Algorithm<u8>;
```

### `CRC_8_HITAG`
```rust
const CRC_8_HITAG: crate::Algorithm<u8>;
```

### `CRC_8_I_432_1`
```rust
const CRC_8_I_432_1: crate::Algorithm<u8>;
```

### `CRC_8_I_CODE`
```rust
const CRC_8_I_CODE: crate::Algorithm<u8>;
```

### `CRC_8_LTE`
```rust
const CRC_8_LTE: crate::Algorithm<u8>;
```

### `CRC_8_MAXIM_DOW`
```rust
const CRC_8_MAXIM_DOW: crate::Algorithm<u8>;
```

### `CRC_8_MIFARE_MAD`
```rust
const CRC_8_MIFARE_MAD: crate::Algorithm<u8>;
```

### `CRC_8_NRSC_5`
```rust
const CRC_8_NRSC_5: crate::Algorithm<u8>;
```

### `CRC_8_OPENSAFETY`
```rust
const CRC_8_OPENSAFETY: crate::Algorithm<u8>;
```

### `CRC_8_ROHC`
```rust
const CRC_8_ROHC: crate::Algorithm<u8>;
```

### `CRC_8_SAE_J1850`
```rust
const CRC_8_SAE_J1850: crate::Algorithm<u8>;
```

### `CRC_8_SMBUS`
```rust
const CRC_8_SMBUS: crate::Algorithm<u8>;
```

### `CRC_8_TECH_3250`
```rust
const CRC_8_TECH_3250: crate::Algorithm<u8>;
```

### `CRC_8_WCDMA`
```rust
const CRC_8_WCDMA: crate::Algorithm<u8>;
```

### `CRC_10_ATM`
```rust
const CRC_10_ATM: crate::Algorithm<u16>;
```

### `CRC_10_CDMA2000`
```rust
const CRC_10_CDMA2000: crate::Algorithm<u16>;
```

### `CRC_10_GSM`
```rust
const CRC_10_GSM: crate::Algorithm<u16>;
```

### `CRC_11_FLEXRAY`
```rust
const CRC_11_FLEXRAY: crate::Algorithm<u16>;
```

### `CRC_11_UMTS`
```rust
const CRC_11_UMTS: crate::Algorithm<u16>;
```

### `CRC_12_CDMA2000`
```rust
const CRC_12_CDMA2000: crate::Algorithm<u16>;
```

### `CRC_12_DECT`
```rust
const CRC_12_DECT: crate::Algorithm<u16>;
```

### `CRC_12_GSM`
```rust
const CRC_12_GSM: crate::Algorithm<u16>;
```

### `CRC_12_UMTS`
```rust
const CRC_12_UMTS: crate::Algorithm<u16>;
```

### `CRC_13_BBC`
```rust
const CRC_13_BBC: crate::Algorithm<u16>;
```

### `CRC_14_DARC`
```rust
const CRC_14_DARC: crate::Algorithm<u16>;
```

### `CRC_14_GSM`
```rust
const CRC_14_GSM: crate::Algorithm<u16>;
```

### `CRC_15_CAN`
```rust
const CRC_15_CAN: crate::Algorithm<u16>;
```

### `CRC_15_MPT1327`
```rust
const CRC_15_MPT1327: crate::Algorithm<u16>;
```

### `CRC_16_ARC`
```rust
const CRC_16_ARC: crate::Algorithm<u16>;
```

### `CRC_16_CDMA2000`
```rust
const CRC_16_CDMA2000: crate::Algorithm<u16>;
```

### `CRC_16_CMS`
```rust
const CRC_16_CMS: crate::Algorithm<u16>;
```

### `CRC_16_DDS_110`
```rust
const CRC_16_DDS_110: crate::Algorithm<u16>;
```

### `CRC_16_DECT_R`
```rust
const CRC_16_DECT_R: crate::Algorithm<u16>;
```

### `CRC_16_DECT_X`
```rust
const CRC_16_DECT_X: crate::Algorithm<u16>;
```

### `CRC_16_DNP`
```rust
const CRC_16_DNP: crate::Algorithm<u16>;
```

### `CRC_16_EN_13757`
```rust
const CRC_16_EN_13757: crate::Algorithm<u16>;
```

### `CRC_16_GENIBUS`
```rust
const CRC_16_GENIBUS: crate::Algorithm<u16>;
```

### `CRC_16_GSM`
```rust
const CRC_16_GSM: crate::Algorithm<u16>;
```

### `CRC_16_IBM_3740`
```rust
const CRC_16_IBM_3740: crate::Algorithm<u16>;
```

### `CRC_16_IBM_SDLC`
```rust
const CRC_16_IBM_SDLC: crate::Algorithm<u16>;
```

### `CRC_16_ISO_IEC_14443_3_A`
```rust
const CRC_16_ISO_IEC_14443_3_A: crate::Algorithm<u16>;
```

### `CRC_16_KERMIT`
```rust
const CRC_16_KERMIT: crate::Algorithm<u16>;
```

### `CRC_16_LJ1200`
```rust
const CRC_16_LJ1200: crate::Algorithm<u16>;
```

### `CRC_16_M17`
```rust
const CRC_16_M17: crate::Algorithm<u16>;
```

### `CRC_16_MAXIM_DOW`
```rust
const CRC_16_MAXIM_DOW: crate::Algorithm<u16>;
```

### `CRC_16_MCRF4XX`
```rust
const CRC_16_MCRF4XX: crate::Algorithm<u16>;
```

### `CRC_16_MODBUS`
```rust
const CRC_16_MODBUS: crate::Algorithm<u16>;
```

### `CRC_16_NRSC_5`
```rust
const CRC_16_NRSC_5: crate::Algorithm<u16>;
```

### `CRC_16_OPENSAFETY_A`
```rust
const CRC_16_OPENSAFETY_A: crate::Algorithm<u16>;
```

### `CRC_16_OPENSAFETY_B`
```rust
const CRC_16_OPENSAFETY_B: crate::Algorithm<u16>;
```

### `CRC_16_PROFIBUS`
```rust
const CRC_16_PROFIBUS: crate::Algorithm<u16>;
```

### `CRC_16_RIELLO`
```rust
const CRC_16_RIELLO: crate::Algorithm<u16>;
```

### `CRC_16_SPI_FUJITSU`
```rust
const CRC_16_SPI_FUJITSU: crate::Algorithm<u16>;
```

### `CRC_16_T10_DIF`
```rust
const CRC_16_T10_DIF: crate::Algorithm<u16>;
```

### `CRC_16_TELEDISK`
```rust
const CRC_16_TELEDISK: crate::Algorithm<u16>;
```

### `CRC_16_TMS37157`
```rust
const CRC_16_TMS37157: crate::Algorithm<u16>;
```

### `CRC_16_UMTS`
```rust
const CRC_16_UMTS: crate::Algorithm<u16>;
```

### `CRC_16_USB`
```rust
const CRC_16_USB: crate::Algorithm<u16>;
```

### `CRC_16_XMODEM`
```rust
const CRC_16_XMODEM: crate::Algorithm<u16>;
```

### `CRC_17_CAN_FD`
```rust
const CRC_17_CAN_FD: crate::Algorithm<u32>;
```

### `CRC_21_CAN_FD`
```rust
const CRC_21_CAN_FD: crate::Algorithm<u32>;
```

### `CRC_24_BLE`
```rust
const CRC_24_BLE: crate::Algorithm<u32>;
```

### `CRC_24_FLEXRAY_A`
```rust
const CRC_24_FLEXRAY_A: crate::Algorithm<u32>;
```

### `CRC_24_FLEXRAY_B`
```rust
const CRC_24_FLEXRAY_B: crate::Algorithm<u32>;
```

### `CRC_24_INTERLAKEN`
```rust
const CRC_24_INTERLAKEN: crate::Algorithm<u32>;
```

### `CRC_24_LTE_A`
```rust
const CRC_24_LTE_A: crate::Algorithm<u32>;
```

### `CRC_24_LTE_B`
```rust
const CRC_24_LTE_B: crate::Algorithm<u32>;
```

### `CRC_24_OPENPGP`
```rust
const CRC_24_OPENPGP: crate::Algorithm<u32>;
```

### `CRC_24_OS_9`
```rust
const CRC_24_OS_9: crate::Algorithm<u32>;
```

### `CRC_30_CDMA`
```rust
const CRC_30_CDMA: crate::Algorithm<u32>;
```

### `CRC_31_PHILIPS`
```rust
const CRC_31_PHILIPS: crate::Algorithm<u32>;
```

### `CRC_32_AIXM`
```rust
const CRC_32_AIXM: crate::Algorithm<u32>;
```

### `CRC_32_AUTOSAR`
```rust
const CRC_32_AUTOSAR: crate::Algorithm<u32>;
```

### `CRC_32_BASE91_D`
```rust
const CRC_32_BASE91_D: crate::Algorithm<u32>;
```

### `CRC_32_BZIP2`
```rust
const CRC_32_BZIP2: crate::Algorithm<u32>;
```

### `CRC_32_CD_ROM_EDC`
```rust
const CRC_32_CD_ROM_EDC: crate::Algorithm<u32>;
```

### `CRC_32_CKSUM`
```rust
const CRC_32_CKSUM: crate::Algorithm<u32>;
```

### `CRC_32_ISCSI`
```rust
const CRC_32_ISCSI: crate::Algorithm<u32>;
```

### `CRC_32_ISO_HDLC`
```rust
const CRC_32_ISO_HDLC: crate::Algorithm<u32>;
```

### `CRC_32_JAMCRC`
```rust
const CRC_32_JAMCRC: crate::Algorithm<u32>;
```

### `CRC_32_MEF`
```rust
const CRC_32_MEF: crate::Algorithm<u32>;
```

### `CRC_32_MPEG_2`
```rust
const CRC_32_MPEG_2: crate::Algorithm<u32>;
```

### `CRC_32_XFER`
```rust
const CRC_32_XFER: crate::Algorithm<u32>;
```

### `CRC_40_GSM`
```rust
const CRC_40_GSM: crate::Algorithm<u64>;
```

### `CRC_64_ECMA_182`
```rust
const CRC_64_ECMA_182: crate::Algorithm<u64>;
```

### `CRC_64_GO_ISO`
```rust
const CRC_64_GO_ISO: crate::Algorithm<u64>;
```

### `CRC_64_MS`
```rust
const CRC_64_MS: crate::Algorithm<u64>;
```

### `CRC_64_REDIS`
```rust
const CRC_64_REDIS: crate::Algorithm<u64>;
```

### `CRC_64_WE`
```rust
const CRC_64_WE: crate::Algorithm<u64>;
```

### `CRC_64_XZ`
```rust
const CRC_64_XZ: crate::Algorithm<u64>;
```

### `CRC_82_DARC`
```rust
const CRC_82_DARC: crate::Algorithm<u128>;
```

