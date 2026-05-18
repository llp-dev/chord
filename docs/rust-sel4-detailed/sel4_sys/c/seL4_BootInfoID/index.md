*[sel4_sys](../../index.md) / [c](../index.md) / [seL4_BootInfoID](index.md)*

---

# Module `seL4_BootInfoID`

## Contents

- [Type Aliases](#type-aliases)
  - [`Type`](#type)
- [Constants](#constants)
  - [`SEL4_BOOTINFO_HEADER_PADDING`](#sel4-bootinfo-header-padding)
  - [`SEL4_BOOTINFO_HEADER_X86_VBE`](#sel4-bootinfo-header-x86-vbe)
  - [`SEL4_BOOTINFO_HEADER_X86_MBMMAP`](#sel4-bootinfo-header-x86-mbmmap)
  - [`SEL4_BOOTINFO_HEADER_X86_ACPI_RSDP`](#sel4-bootinfo-header-x86-acpi-rsdp)
  - [`SEL4_BOOTINFO_HEADER_X86_FRAMEBUFFER`](#sel4-bootinfo-header-x86-framebuffer)
  - [`SEL4_BOOTINFO_HEADER_X86_TSC_FREQ`](#sel4-bootinfo-header-x86-tsc-freq)
  - [`SEL4_BOOTINFO_HEADER_FDT`](#sel4-bootinfo-header-fdt)
  - [`SEL4_BOOTINFO_HEADER_NUM`](#sel4-bootinfo-header-num)
  - [`_enum_pad_seL4_BootInfoID`](#enum-pad-sel4-bootinfoid)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Type`](#type) | type |  |
| [`SEL4_BOOTINFO_HEADER_PADDING`](#sel4-bootinfo-header-padding) | const |  |
| [`SEL4_BOOTINFO_HEADER_X86_VBE`](#sel4-bootinfo-header-x86-vbe) | const |  |
| [`SEL4_BOOTINFO_HEADER_X86_MBMMAP`](#sel4-bootinfo-header-x86-mbmmap) | const |  |
| [`SEL4_BOOTINFO_HEADER_X86_ACPI_RSDP`](#sel4-bootinfo-header-x86-acpi-rsdp) | const |  |
| [`SEL4_BOOTINFO_HEADER_X86_FRAMEBUFFER`](#sel4-bootinfo-header-x86-framebuffer) | const |  |
| [`SEL4_BOOTINFO_HEADER_X86_TSC_FREQ`](#sel4-bootinfo-header-x86-tsc-freq) | const |  |
| [`SEL4_BOOTINFO_HEADER_FDT`](#sel4-bootinfo-header-fdt) | const |  |
| [`SEL4_BOOTINFO_HEADER_NUM`](#sel4-bootinfo-header-num) | const |  |
| [`_enum_pad_seL4_BootInfoID`](#enum-pad-sel4-bootinfoid) | const |  |

## Type Aliases

### `Type`

```rust
type Type = ::core::ffi::c_ulong;
```

## Constants

### `SEL4_BOOTINFO_HEADER_PADDING`
```rust
const SEL4_BOOTINFO_HEADER_PADDING: Type = 0u64;
```

### `SEL4_BOOTINFO_HEADER_X86_VBE`
```rust
const SEL4_BOOTINFO_HEADER_X86_VBE: Type = 1u64;
```

### `SEL4_BOOTINFO_HEADER_X86_MBMMAP`
```rust
const SEL4_BOOTINFO_HEADER_X86_MBMMAP: Type = 2u64;
```

### `SEL4_BOOTINFO_HEADER_X86_ACPI_RSDP`
```rust
const SEL4_BOOTINFO_HEADER_X86_ACPI_RSDP: Type = 3u64;
```

### `SEL4_BOOTINFO_HEADER_X86_FRAMEBUFFER`
```rust
const SEL4_BOOTINFO_HEADER_X86_FRAMEBUFFER: Type = 4u64;
```

### `SEL4_BOOTINFO_HEADER_X86_TSC_FREQ`
```rust
const SEL4_BOOTINFO_HEADER_X86_TSC_FREQ: Type = 5u64;
```

### `SEL4_BOOTINFO_HEADER_FDT`
```rust
const SEL4_BOOTINFO_HEADER_FDT: Type = 6u64;
```

### `SEL4_BOOTINFO_HEADER_NUM`
```rust
const SEL4_BOOTINFO_HEADER_NUM: Type = 7u64;
```

### `_enum_pad_seL4_BootInfoID`
```rust
const _enum_pad_seL4_BootInfoID: Type = 9_223_372_036_854_775_807u64;
```

