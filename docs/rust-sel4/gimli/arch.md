**gimli > arch**

# Module: arch

## Contents

**Structs**

- [`AArch64`](#aarch64) - ARM 64-bit (AArch64) architecture specific definitions.
- [`Arm`](#arm) - ARM architecture specific definitions.
- [`LoongArch`](#loongarch) - LoongArch architecture specific definitions.
- [`MIPS`](#mips) - MIPS architecture specific definitions.
- [`PowerPc64`](#powerpc64) - PowerPC 64bit
- [`RiscV`](#riscv) - RISC-V architecture specific definitions.
- [`X86`](#x86) - Intel i386 architecture specific definitions.
- [`X86_64`](#x86_64) - AMD64 architecture specific definitions.

---

## gimli::arch::AArch64

*Struct*

ARM 64-bit (AArch64) architecture specific definitions.

See [DWARF for the ARM 64-bit Architecture](
https://github.com/ARM-software/abi-aa/blob/main/aadwarf64/aadwarf64.rst).

**Unit Struct**

**Methods:**

- `fn register_name(register: Register) -> Option<&'static str>` - The name of a register, or `None` if the register number is unknown.
- `fn name_to_register(value: &str) -> Option<Register>` - Converts a register name into a register number.

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> AArch64`



## gimli::arch::Arm

*Struct*

ARM architecture specific definitions.

See [DWARF for the ARM Architecture](
https://github.com/ARM-software/abi-aa/blob/main/aadwarf32/aadwarf32.rst).

**Unit Struct**

**Methods:**

- `fn register_name(register: Register) -> Option<&'static str>` - The name of a register, or `None` if the register number is unknown.
- `fn name_to_register(value: &str) -> Option<Register>` - Converts a register name into a register number.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Arm`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## gimli::arch::LoongArch

*Struct*

LoongArch architecture specific definitions.

See [LoongArch ELF psABI specification](https://loongson.github.io/LoongArch-Documentation/LoongArch-ELF-ABI-EN.html).

**Unit Struct**

**Methods:**

- `fn register_name(register: Register) -> Option<&'static str>` - The name of a register, or `None` if the register number is unknown.
- `fn name_to_register(value: &str) -> Option<Register>` - Converts a register name into a register number.

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> LoongArch`



## gimli::arch::MIPS

*Struct*

MIPS architecture specific definitions.

See [MIPS Details](https://en.wikibooks.org/wiki/MIPS_Assembly/MIPS_Details).

**Unit Struct**

**Methods:**

- `fn register_name(register: Register) -> Option<&'static str>` - The name of a register, or `None` if the register number is unknown.
- `fn name_to_register(value: &str) -> Option<Register>` - Converts a register name into a register number.

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> MIPS`



## gimli::arch::PowerPc64

*Struct*

PowerPC 64bit

See [64-bit ELF ABI Specification for OpenPOWER Architecture](https://openpowerfoundation.org/specifications/64bitelfabi/).

**Unit Struct**

**Methods:**

- `fn register_name(register: Register) -> Option<&'static str>` - The name of a register, or `None` if the register number is unknown.
- `fn name_to_register(value: &str) -> Option<Register>` - Converts a register name into a register number.

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> PowerPc64`



## gimli::arch::RiscV

*Struct*

RISC-V architecture specific definitions.

See [RISC-V ELF psABI specification](https://github.com/riscv/riscv-elf-psabi-doc).

**Unit Struct**

**Methods:**

- `fn register_name(register: Register) -> Option<&'static str>` - The name of a register, or `None` if the register number is unknown.
- `fn name_to_register(value: &str) -> Option<Register>` - Converts a register name into a register number.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> RiscV`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## gimli::arch::X86

*Struct*

Intel i386 architecture specific definitions.

See section 2.4.2 of the [i386 psABI](https://gitlab.com/x86-psABIs/i386-ABI).

**Unit Struct**

**Methods:**

- `fn register_name(register: Register) -> Option<&'static str>` - The name of a register, or `None` if the register number is unknown.
- `fn name_to_register(value: &str) -> Option<Register>` - Converts a register name into a register number.

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> X86`



## gimli::arch::X86_64

*Struct*

AMD64 architecture specific definitions.

See section 3.6.2 of the [x86-64 psABI](https://gitlab.com/x86-psABIs/x86-64-ABI).

**Unit Struct**

**Methods:**

- `fn register_name(register: Register) -> Option<&'static str>` - The name of a register, or `None` if the register number is unknown.
- `fn name_to_register(value: &str) -> Option<Register>` - Converts a register name into a register number.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> X86_64`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



