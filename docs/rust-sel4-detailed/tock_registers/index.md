# Crate `tock_registers`

Tock Register Interface

Provides efficient mechanisms to express and use type-checked memory mapped
registers and bitfields.

```rust
fn main() {}

use tock_registers::registers::{ReadOnly, ReadWrite};
use tock_registers::register_bitfields;

// Register maps are specified like this:
#[repr(C)]
struct Registers {
    // Control register: read-write
    cr: ReadWrite<u32, Control::Register>,
    // Status register: read-only
    s: ReadOnly<u32, Status::Register>,
}

// Register fields and definitions look like this:
register_bitfields![u32,
    // Simpler bitfields are expressed concisely:
    Control [
        /// Stop the Current Transfer
        STOP 8,
        /// Software Reset
        SWRST 7,
        /// Master Disable
        MDIS 1,
        /// Master Enable
        MEN 0
    ],

    // More complex registers can express subtypes:
    Status [
        TXCOMPLETE  OFFSET(0) NUMBITS(1) [],
        TXINTERRUPT OFFSET(1) NUMBITS(1) [],
        RXCOMPLETE  OFFSET(2) NUMBITS(1) [],
        RXINTERRUPT OFFSET(3) NUMBITS(1) [],
        MODE        OFFSET(4) NUMBITS(3) [
            FullDuplex = 0,
            HalfDuplex = 1,
            Loopback = 2,
            Disabled = 3
        ],
        ERRORCOUNT OFFSET(6) NUMBITS(3) []
    ]
];
```

Author
------
- Shane Leonard <shanel@stanford.edu>

## Contents

- [Modules](#modules)
  - [`fields`](#fields)
  - [`interfaces`](#interfaces)
  - [`macros`](#macros)
  - [`registers`](#registers)
  - [`debug`](#debug)
  - [`local_register`](#local-register)
- [Structs](#structs)
  - [`LocalRegisterCopy`](#localregistercopy)
- [Traits](#traits)
  - [`UIntLike`](#uintlike)
  - [`RegisterLongName`](#registerlongname)
- [Macros](#macros)
  - [`UIntLike_impl_for!`](#uintlike-impl-for)
  - [`bitmask!`](#bitmask)
  - [`register_bitmasks!`](#register-bitmasks)
  - [`register_bitfields!`](#register-bitfields)
  - [`register_fields!`](#register-fields)
  - [`test_fields!`](#test-fields)
  - [`register_structs!`](#register-structs)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`fields`](#fields) | mod | Register bitfield types and macros |
| [`interfaces`](#interfaces) | mod | Interfaces (traits) to register types |
| [`macros`](#macros) | mod | Macros for cleanly defining peripheral registers. |
| [`registers`](#registers) | mod | Implementation of included register types. |
| [`debug`](#debug) | mod | Register Debug Support Infrastructure |
| [`local_register`](#local-register) | mod | Module containing the [`LocalRegisterCopy`] type. |
| [`LocalRegisterCopy`](#localregistercopy) | struct |  |
| [`UIntLike`](#uintlike) | trait | Trait representing the base type of registers. |
| [`RegisterLongName`](#registerlongname) | trait | Descriptive name for each register. |
| [`UIntLike_impl_for!`](#uintlike-impl-for) | macro |  |
| [`bitmask!`](#bitmask) | macro | Helper macro for computing bitmask of variable number of bits |
| [`register_bitmasks!`](#register-bitmasks) | macro | Helper macro for defining register fields. |
| [`register_bitfields!`](#register-bitfields) | macro | Define register types and fields. |
| [`register_fields!`](#register-fields) | macro |  |
| [`test_fields!`](#test-fields) | macro | Statically validate the size and offsets of the fields defined within the register struct through the `register_structs!()` macro. |
| [`register_structs!`](#register-structs) | macro | Define a peripheral memory map containing registers. |

## Modules

- [`fields`](fields/index.md) — Register bitfield types and macros
- [`interfaces`](interfaces/index.md) — Interfaces (traits) to register types
- [`macros`](macros/index.md) — Macros for cleanly defining peripheral registers.
- [`registers`](registers/index.md) — Implementation of included register types.
- [`debug`](debug/index.md) — Register Debug Support Infrastructure
- [`local_register`](local_register/index.md) — Module containing the [`LocalRegisterCopy`] type. Please refer to

## Structs

### `LocalRegisterCopy<T: UIntLike, R: RegisterLongName>`

```rust
struct LocalRegisterCopy<T: UIntLike, R: RegisterLongName> {
    value: T,
    associated_register: core::marker::PhantomData<R>,
}
```

A read-write copy of register contents.

This behaves very similarly to a read-write register, but instead of doing a
volatile read to MMIO to get the value for each function call, a copy of the
register contents are stored locally in memory. This allows a peripheral to
do a single read on a register, and then check which bits are set without
having to do a full MMIO read each time. It also allows the value of the
register to be "cached" in case the peripheral driver needs to clear the
register in hardware yet still be able to check the bits.  You can write to
a local register, which will modify the stored value, but will not modify
any hardware because it operates only on local copy.

This type does not implement the [`Readable`](crate::interfaces::Readable)
and [`Writeable`](crate::interfaces::Writeable) traits because it requires a
mutable reference to modify the contained value. It still mirrors the
interface which would be exposed by a type implementing
[`Readable`](crate::interfaces::Readable),
[`Writeable`](crate::interfaces::Writeable) and
[`ReadWriteable`](crate::interfaces::ReadWriteable).

#### Implementations

- <span id="localregistercopy-new"></span>`const fn new(value: T) -> Self`

- <span id="localregistercopy-get"></span>`fn get(&self) -> T`

  Get the raw register value

- <span id="localregistercopy-set"></span>`fn set(&mut self, value: T)`

  Set the raw register value

- <span id="localregistercopy-read"></span>`fn read(&self, field: Field<T, R>) -> T` — [`Field`](fields/index.md#field)

  Read the value of the given field

- <span id="localregistercopy-read-as-enum"></span>`fn read_as_enum<E: TryFromValue<T, EnumType = E>>(&self, field: Field<T, R>) -> Option<E>` — [`Field`](fields/index.md#field)

  Read value of the given field as an enum member

- <span id="localregistercopy-write"></span>`fn write(&mut self, field: FieldValue<T, R>)` — [`FieldValue`](fields/index.md#fieldvalue)

  Write the value of one or more fields, overwriting the other fields with

  zero

- <span id="localregistercopy-modify"></span>`fn modify(&mut self, field: FieldValue<T, R>)` — [`FieldValue`](fields/index.md#fieldvalue)

  Write the value of one or more fields, leaving the other fields

  unchanged

- <span id="localregistercopy-is-set"></span>`fn is_set(&self, field: Field<T, R>) -> bool` — [`Field`](fields/index.md#field)

  Check if one or more bits in a field are set

- <span id="localregistercopy-any-matching-bits-set"></span>`fn any_matching_bits_set(&self, field: FieldValue<T, R>) -> bool` — [`FieldValue`](fields/index.md#fieldvalue)

  Check if any bits corresponding to the mask in the passed `FieldValue`

  are set.

- <span id="localregistercopy-matches-all"></span>`fn matches_all(&self, field: FieldValue<T, R>) -> bool` — [`FieldValue`](fields/index.md#fieldvalue)

  Check if all specified parts of a field match

- <span id="localregistercopy-matches-any"></span>`fn matches_any(&self, fields: &[FieldValue<T, R>]) -> bool` — [`FieldValue`](fields/index.md#fieldvalue)

  Check if any of the passed parts of a field exactly match the contained

  value. This allows for matching on unset bits, or matching on specific

  values in multi-bit fields.

- <span id="localregistercopy-bitand"></span>`fn bitand(&self, rhs: T) -> LocalRegisterCopy<T, R>` — [`LocalRegisterCopy`](local_register/index.md#localregistercopy)

  Do a bitwise AND operation of the stored value and the passed in value

  and return a new LocalRegisterCopy.

- <span id="localregistercopy-debug"></span>`fn debug(&self) -> crate::debug::RegisterDebugValue<T, R>` — [`RegisterDebugValue`](debug/index.md#registerdebugvalue)

#### Trait Implementations

##### `impl<T: clone::Clone + UIntLike, R: clone::Clone + RegisterLongName> Clone for LocalRegisterCopy<T, R>`

- <span id="localregistercopy-clone"></span>`fn clone(&self) -> LocalRegisterCopy<T, R>` — [`LocalRegisterCopy`](local_register/index.md#localregistercopy)

##### `impl<T: marker::Copy + UIntLike, R: marker::Copy + RegisterLongName> Copy for LocalRegisterCopy<T, R>`

##### `impl<T: UIntLike + fmt::Debug, R: RegisterLongName> Debug for LocalRegisterCopy<T, R>`

- <span id="localregistercopy-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Traits

### `UIntLike`

```rust
trait UIntLike: BitAnd<Output = Self> + BitOr<Output = Self> + BitOrAssign + Not<Output = Self> + Eq + Shr<usize, Output = Self> + Shl<usize, Output = Self> + Copy + Clone + Debug { ... }
```

Trait representing the base type of registers.

UIntLike defines basic properties of types required to read/write/modify a
register through its methods and supertrait requirements.

It features a range of default implementations for common unsigned integer
types, such as `u8`, [`u16`](../gimli/leb128/read/index.md), `u32`, `u64`, [`u128`](../ring/aead/gcm/index.md), and `usize`.

#### Required Methods

- `fn zero() -> Self`

  Return the representation of the value `0` in the implementing type.

#### Implementors

- `u128`
- `u16`
- `u32`
- `u64`
- `u8`
- `usize`

### `RegisterLongName`

```rust
trait RegisterLongName { ... }
```

Descriptive name for each register.

#### Implementors

- `()`

## Macros

### `UIntLike_impl_for!`

### `bitmask!`

Helper macro for computing bitmask of variable number of bits

### `register_bitmasks!`

Helper macro for defining register fields.

### `register_bitfields!`

Define register types and fields.

Implementations of memory-mapped registers can use this macro to define the
structure and bitwise meaning of individual registers in the peripheral. An
example use for a hypothetical UART driver might look like:

```rust,ignore
register_bitfields![u32,
    CONTROL [
        ENABLE OFFSET(0) NUMBITS(1),
        STOP_BITS OFFSET(1) NUMBITS(2) [
            StopBits1 = 0,
            StopBits2 = 1,
            StopBits0 = 2
        ]
    ],
    BYTE [
        CHARACTER OFFSET(0) NUMBITS(8)
    ],
    INTERRUPT [
        TRANSMITTED OFFSET(0) NUMBITS(1),
        RECEIVED OFFSET(1) NUMBITS(1),
        FIFO_FULL OFFSET(2) NUMBITS(1)
    ]
];
```

Each field in the register can be identified by its offset within the
register and its bitwidth. Fields that have discrete options with semantic
meaning can be enumerated.

### `register_fields!`

### `test_fields!`

Statically validate the size and offsets of the fields defined within the
register struct through the `register_structs!()` macro.

This macro expands to an expression which contains static assertions about
various parameters of the individual fields in the register struct
definition. It will test for:

- Proper start offset of padding fields. It will fail in cases such as

  ```should_fail
  #[macro_use]
  extern crate tock_registers;
  use tock_registers::register_structs;
  use tock_registers::registers::ReadWrite;
  register_structs! {
      UartRegisters {
          (0x04 => _reserved),
          (0x08 => foo: ReadWrite<u32>),
          (0x0C => @END),
      }
  }
  // This is required for rustdoc to not place this code snipped into an
  // fn main() {...} function.
  fn main() { }
  ```

  In this example, the start offset of `_reserved` should have been `0x00`
  instead of `0x04`.

- Correct start offset and end offset (start offset of next field) in actual
  fields. It will fail in cases such as

  ```should_fail
  #[macro_use]
  extern crate tock_registers;
  use tock_registers::register_structs;
  use tock_registers::registers::ReadWrite;
  register_structs! {
      UartRegisters {
          (0x00 => foo: ReadWrite<u32>),
          (0x05 => bar: ReadWrite<u32>),
          (0x08 => @END),
      }
  }
  // This is required for rustdoc to not place this code snipped into an
  // fn main() {...} function.
  fn main() { }
  ```

  In this example, the start offset of `bar` and thus the end offset of
  `foo` should have been `0x04` instead of `0x05`.

- Invalid alignment of fields.

- That the end marker matches the actual generated struct size. This will
fail in cases such as

  ```should_fail
  #[macro_use]
  extern crate tock_registers;
  use tock_registers::register_structs;
  use tock_registers::registers::ReadWrite;
  register_structs! {
      UartRegisters {
          (0x00 => foo: ReadWrite<u32>),
          (0x04 => bar: ReadWrite<u32>),
          (0x10 => @END),
      }
  }
  // This is required for rustdoc to not place this code snipped into an
  // fn main() {...} function.
  fn main() { }
  ```

### `register_structs!`

Define a peripheral memory map containing registers.

Implementations of memory-mapped registers can use this macro to define the
individual registers in the peripheral and their relative address offset
from the start of the peripheral's mapped address. An example use for a
hypothetical UART driver might look like:

```rust,ignore
register_structs! {
    pub UartRegisters {
        (0x00 => control: ReadWrite<u32, CONTROL::Register>),
        (0x04 => write_byte: ReadWrite<u32, BYTE::Register>),
        (0x08 => _reserved1),
        (0x20 => interrupt_enable: ReadWrite<u32, INTERRUPT::Register>),
        (0x24 => interrupt_status: ReadWrite<u32, INTERRUPT::Register>),
        (0x28 => @END),
    }
}
```

By convention, gaps in the register memory map are named `_reserved`. The
macro will automatically compute the size of the reserved field so that the
next register is at the correct address.

The size of the register is denoted by the first parameter in the
[`ReadWrite`](crate::registers::ReadWrite) type. The second parameter in the
[`ReadWrite`](crate::registers::ReadWrite) type is a register definition
which is specified with the
[`register_bitfields!()`](crate::register_bitfields) macro.

