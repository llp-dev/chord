**tock_registers**

# Module: tock_registers

## Contents

**Modules**

- [`debug`](#debug) - Register Debug Support Infrastructure
- [`fields`](#fields) - Register bitfield types and macros
- [`interfaces`](#interfaces) - Interfaces (traits) to register types
- [`local_register`](#local_register) - Module containing the [`LocalRegisterCopy`] type. Please refer to
- [`macros`](#macros) - Macros for cleanly defining peripheral registers.
- [`registers`](#registers) - Implementation of included register types.

**Macros**

- [`UIntLike_impl_for`](#uintlike_impl_for)
- [`bitmask`](#bitmask) - Helper macro for computing bitmask of variable number of bits
- [`register_bitfields`](#register_bitfields) - Define register types and fields.
- [`register_bitmasks`](#register_bitmasks) - Helper macro for defining register fields.
- [`register_fields`](#register_fields)
- [`register_structs`](#register_structs) - Define a peripheral memory map containing registers.
- [`test_fields`](#test_fields) - Statically validate the size and offsets of the fields defined within the

**Traits**

- [`RegisterLongName`](#registerlongname) - Descriptive name for each register.
- [`UIntLike`](#uintlike) - Trait representing the base type of registers.

---

## tock_registers::RegisterLongName

*Trait*

Descriptive name for each register.



## tock_registers::UIntLike

*Trait*

Trait representing the base type of registers.

UIntLike defines basic properties of types required to read/write/modify a
register through its methods and supertrait requirements.

It features a range of default implementations for common unsigned integer
types, such as [`u8`], [`u16`], [`u32`], [`u64`], [`u128`], and [`usize`].

**Methods:**

- `zero`: Return the representation of the value `0` in the implementing type.



## tock_registers::UIntLike_impl_for

*Declarative Macro*

```rust
macro_rules! UIntLike_impl_for {
    ($type:ty) => { ... };
}
```



## tock_registers::bitmask

*Declarative Macro*

Helper macro for computing bitmask of variable number of bits

```rust
macro_rules! bitmask {
    ($numbits:expr) => { ... };
}
```



## Module: debug

Register Debug Support Infrastructure

This module provides optional infrastructure to query debug information from
register types implementing the [`RegisterDebugInfo`] trait. This
information can then be used by the [`RegisterDebugValue`] type to produce a
human-readable representation of a register's fields and values.



## Module: fields

Register bitfield types and macros

To conveniently access and manipulate fields of a register, this library
provides types and macros to describe and access bitfields of a
register. This can be especially useful in conjunction with the APIs defined
in [`interfaces`](crate::interfaces), which make use of these types and
hence allow to access and manipulate bitfields of proper registers directly.

A specific section (bitfield) in a register is described by the [`Field`]
type, consisting of an unshifted bitmask over the base register [`UIntLike`]
type, and a shift parameter. It is further associated with a specific
[`RegisterLongName`], which can prevent its use with incompatible registers.

A value of a section of a register is described by the [`FieldValue`]
type. It stores the information of the respective section in the register,
as well as the associated value. A [`FieldValue`] can be created from a
[`Field`] through the [`val`](Field::val) method.

## `register_bitfields` macro

For defining register layouts with an associated [`RegisterLongName`], along
with [`Field`]s and matching [`FieldValue`]s, a convenient macro-based
interface can be used.

The following example demonstrates how two registers can be defined, over a
`u32` base type:

```rust
# use tock_registers::register_bitfields;
# use tock_registers::registers::InMemoryRegister;
# use tock_registers::interfaces::{Readable, ReadWriteable};
register_bitfields![u32,
    Uart [
        ENABLE OFFSET(0) NUMBITS(4) [
            ON = 8,
            OFF = 0
        ]
    ],
    Psel [
        PIN OFFSET(0) NUMBITS(6),
        CONNECT OFFSET(31) NUMBITS(1)
    ],
];

// In this scope, `Uart` is a module, representing the register and its
// fields. `Uart::Register` is a `RegisterLongName` type identifying this
// register. `Uart::ENABLE` is a field covering the first 4 bits of this
// register. `Uart::ENABLE::ON` is a `FieldValue` over that field, with the
// associated value 8. We can now use the types like so:
let reg: InMemoryRegister<u32, Uart::Register> = InMemoryRegister::new(0);
assert!(reg.read(Uart::ENABLE) == 0x00000000);
reg.modify(Uart::ENABLE::ON);
assert!(reg.get() == 0x00000008);

use tock_registers::interfaces::Debuggable;
assert!(
    &format!("{:?}", reg.debug())
    == "Uart { ENABLE: ON }"
);
```



## Module: interfaces

Interfaces (traits) to register types

This module contains traits which reflect standardized interfaces to
different types of registers. Examples of registers implementing these
interfaces are [`ReadWrite`](crate::registers::ReadWrite) or
[`InMemoryRegister`](crate::registers::InMemoryRegister).

Each trait has two associated type parameters, namely:

- `T`: [`UIntLike`], indicating the underlying integer type used to
  represent the register's raw contents.

- `R`: [`RegisterLongName`], functioning as a type to identify this
  register's descriptive name and semantic meaning. It is further used to
  impose type constraints on values passed through the API, such as
  [`FieldValue`].

Registers can have different access levels, which are mapped to different
traits respectively:

- [`Readable`]: indicates that the current value of this register can be
  read. Implementations will need to provide the
  [`get`](crate::interfaces::Readable::get) method.

- [`Writeable`]: indicates that the value of this register can be
  set. Implementations will need to provide the
  [`set`](crate::interfaces::Writeable::set) method.

- [`ReadWriteable`]: indicates that this register can be _modified_. It is
  not sufficient for registers to be both read- and writable, they must also
  have the same semantic meaning when read from and written to. This is not
  true in general, for example a memory-mapped UART register might transmit
  when writing and receive when reading.

  If a type implements both [`Readable`] and [`Writeable`], and the
  associated [`RegisterLongName`] type parameters are identical, it will
  automatically implement [`ReadWriteable`]. In particular, for
  [`Aliased`](crate::registers::Aliased) this is -- in general -- not the
  case, so

  ```rust
  # use tock_registers::interfaces::{Readable, Writeable, ReadWriteable};
  # use tock_registers::registers::ReadWrite;
  # use tock_registers::register_bitfields;
  register_bitfields![u8,
      A [
          DUMMY OFFSET(0) NUMBITS(1) [],
      ],
  ];
  let mut register_memory: u8 = 0;
  let read_write_reg: &ReadWrite<u8, A::Register> = unsafe {
      core::mem::transmute(&mut register_memory)
  };
  ReadWriteable::modify(read_write_reg, A::DUMMY::SET);
  ```

  works, but not

  ```compile_fail
  # use tock_registers::interfaces::{Readable, Writeable, ReadWriteable};
  # use tock_registers::registers::Aliased;
  # use tock_registers::register_bitfields;
  register_bitfields![u8,
      A [
          DUMMY OFFSET(0) NUMBITS(1) [],
      ],
      B [
          DUMMY OFFSET(0) NUMBITS(1) [],
      ],
  ];
  let aliased_reg: &Aliased<u8, A::Register, B::Register> = unsafe {
      core::mem::transmute(Box::leak(Box::new(0_u8)))
  };
  ReadWriteable::modify(aliased_reg, A::DUMMY::SET);
  ```

- [`Debuggable`]: indicates that the register supports producing
  human-readable debug output using the `RegisterDebugValue` type. This type
  can be produced with the [`debug`](crate::interfaces::Debuggable::debug)
  method. This will return a value that implements
  [`Debug`](core::fmt::Debug). It is automticaly implemented for any
  register implementing [`Readable`].


## Example: implementing a custom register type

These traits can be used to implement custom register types, which are
compatible to the ones shipped in this crate. For example, to define a
register which sets a `u8` value using a Cell reference, always reads the
bitwise-negated vale and prints every written value to the console:

```rust
# use core::cell::Cell;
# use core::marker::PhantomData;
#
# use tock_registers::interfaces::{Readable, Writeable, ReadWriteable};
# use tock_registers::RegisterLongName;
# use tock_registers::register_bitfields;
#
struct DummyRegister<'a, R: RegisterLongName> {
    cell_ref: &'a Cell<u8>,
    _register_long_name: PhantomData<R>,
}

impl<'a, R: RegisterLongName> Readable for DummyRegister<'a, R> {
    type T = u8;
    type R = R;

    fn get(&self) -> u8 {
        // Return the bitwise-inverse of the current value
        !self.cell_ref.get()
    }
}

impl<'a, R: RegisterLongName> Writeable for DummyRegister<'a, R> {
    type T = u8;
    type R = R;

    fn set(&self, value: u8) {
        println!("Setting Cell to {:02x?}!", value);
        self.cell_ref.set(value);
    }
}

register_bitfields![u8,
    DummyReg [
        HIGH OFFSET(4) NUMBITS(4) [
            A = 0b0001,
            B = 0b0010,
            C = 0b0100,
            D = 0b1000,
        ],
        LOW OFFSET(0) NUMBITS(4) [],
    ],
];

// Create a new DummyRegister over some Cell<u8>
let cell = Cell::new(0);
let dummy = DummyRegister {
    cell_ref: &cell,
    _register_long_name: PhantomData,
};

// Set a value and read it back. This demonstrates the raw getters and
// setters of Writeable and Readable
dummy.set(0xFA);
assert!(dummy.get() == 0x05);

// Use some of the automatically derived APIs, such as ReadWriteable::modify
// and Readable::read
dummy.modify(DummyReg::HIGH::C);
assert!(dummy.read(DummyReg::HIGH) == 0xb);
```



## Module: local_register

Module containing the [`LocalRegisterCopy`] type. Please refer to
its documentation.



## Module: macros

Macros for cleanly defining peripheral registers.



## tock_registers::register_bitfields

*Declarative Macro*

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

```rust
macro_rules! register_bitfields {
    {
        $valtype:ident, $( $(#[$inner:meta])* $vis:vis $reg:ident $fields:tt ),* $(,)?
    } => { ... };
}
```



## tock_registers::register_bitmasks

*Declarative Macro*

Helper macro for defining register fields.

```rust
macro_rules! register_bitmasks {
    {
        // BITFIELD_NAME OFFSET(x)
        $(#[$outer:meta])*
        $valtype:ident, $reg_mod:ident, $reg_desc:ident, [
            $( $(#[$inner:meta])* $field:ident OFFSET($offset:expr)),+ $(,)?
        ]
    } => { ... };
    {
        // BITFIELD_NAME OFFSET
        // All fields are 1 bit
        $(#[$outer:meta])*
        $valtype:ident, $reg_mod:ident, $reg_desc:ident, [
            $( $(#[$inner:meta])* $field:ident $offset:expr ),+ $(,)?
        ]
    } => { ... };
    {
        // BITFIELD_NAME OFFSET(x) NUMBITS(y)
        $(#[$outer:meta])*
        $valtype:ident, $reg_mod:ident, $reg_desc:ident, [
            $( $(#[$inner:meta])* $field:ident OFFSET($offset:expr) NUMBITS($numbits:expr) ),+ $(,)?
        ]
    } => { ... };
    {
        // BITFIELD_NAME OFFSET(x) NUMBITS(y) []
        $(#[$outer:meta])*
        $valtype:ident, $reg_mod:ident, $reg_desc:ident, [
            $( $(#[$inner:meta])* $field:ident OFFSET($offset:expr) NUMBITS($numbits:expr)
               $values:tt ),+ $(,)?
        ]
    } => { ... };
    {
        $valtype:ident, $reg_desc:ident, $(#[$outer:meta])* $field:ident,
                    $offset:expr, $numbits:expr,
                    [$( $(#[$inner:meta])* $valname:ident = $value:expr ),+ $(,)?]
    } => { ... };
    {
        $valtype:ident, $reg_desc:ident, $(#[$outer:meta])* $field:ident,
                    $offset:expr, $numbits:expr,
                    []
    } => { ... };
    (
        // final implementation of the macro
        @debug $valtype:ident, $reg_mod:ident, $reg_desc:ident, [$($field:ident),*]
    ) => { ... };
    (
        @fv_enum_type_seq $valtype:ident, $enum_val:path $(, $($rest:path),+)?
    ) => { ... };
    (
        @fv_enum_type_seq $valtype:ident $(,)?
    ) => { ... };
}
```



## tock_registers::register_fields

*Declarative Macro*

```rust
macro_rules! register_fields {
    (@root $(#[$attr_struct:meta])* $vis_struct:vis $name:ident $(<$life:lifetime>)? { $($input:tt)* } ) => { ... };
    (@munch
        (
            $(#[$attr_end:meta])*
            ($offset:expr => @END),
        )
        -> {$vis_struct:vis struct $(#[$attr_struct:meta])* $name:ident $(<$life:lifetime>)? $(
                $(#[$attr:meta])*
                ($vis:vis $id:ident: $ty:ty)
            )*}
    ) => { ... };
    (@munch
        (
            $(#[$attr:meta])*
            ($offset_start:expr => $vis:vis $field:ident: $ty:ty),
            $($after:tt)*
        )
        -> {$($output:tt)*}
    ) => { ... };
    (@munch
        (
            $(#[$attr:meta])*
            ($offset_start:expr => $padding:ident),
            $(#[$attr_next:meta])*
            ($offset_end:expr => $($next:tt)*),
            $($after:tt)*
        )
        -> {$($output:tt)*}
    ) => { ... };
}
```



## tock_registers::register_structs

*Declarative Macro*

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

```rust
macro_rules! register_structs {
    {
        $(
            $(#[$attr:meta])*
            $vis_struct:vis $name:ident $(<$life:lifetime>)? {
                $( $fields:tt )*
            }
        ),*
    } => { ... };
}
```



## Module: registers

Implementation of included register types.

This module provides a standard set of register types, which can describe
different access levels:

- [`ReadWrite`] for registers which can be read and written to
- [`ReadOnly`] for registers which can only be read
- [`WriteOnly`] for registers which can only be written to
- [`Aliased`] for registers which can be both read and written, but
  represent different registers depending on the operation
- [`InMemoryRegister`] provide a register-type in RAM using volatile
  operations

These types can be disabled by removing the `register_types` crate feature
(part of the default features). This is useful if this crate should be used
only as an interface library, or if all unsafe code should be disabled.



## tock_registers::test_fields

*Declarative Macro*

Statically validate the size and offsets of the fields defined within the
register struct through the `register_structs!()` macro.

This macro expands to an expression which contains static assertions about
various parameters of the individual fields in the register struct
definition. It will test for:

- Proper start offset of padding fields. It will fail in cases such as

  ```should_fail
  # #[macro_use]
  # extern crate tock_registers;
  # use tock_registers::register_structs;
  # use tock_registers::registers::ReadWrite;
  register_structs! {
      UartRegisters {
          (0x04 => _reserved),
          (0x08 => foo: ReadWrite<u32>),
          (0x0C => @END),
      }
  }
  # // This is required for rustdoc to not place this code snipped into an
  # // fn main() {...} function.
  # fn main() { }
  ```

  In this example, the start offset of `_reserved` should have been `0x00`
  instead of `0x04`.

- Correct start offset and end offset (start offset of next field) in actual
  fields. It will fail in cases such as

  ```should_fail
  # #[macro_use]
  # extern crate tock_registers;
  # use tock_registers::register_structs;
  # use tock_registers::registers::ReadWrite;
  register_structs! {
      UartRegisters {
          (0x00 => foo: ReadWrite<u32>),
          (0x05 => bar: ReadWrite<u32>),
          (0x08 => @END),
      }
  }
  # // This is required for rustdoc to not place this code snipped into an
  # // fn main() {...} function.
  # fn main() { }
  ```

  In this example, the start offset of `bar` and thus the end offset of
  `foo` should have been `0x04` instead of `0x05`.

- Invalid alignment of fields.

- That the end marker matches the actual generated struct size. This will
fail in cases such as

  ```should_fail
  # #[macro_use]
  # extern crate tock_registers;
  # use tock_registers::register_structs;
  # use tock_registers::registers::ReadWrite;
  register_structs! {
      UartRegisters {
          (0x00 => foo: ReadWrite<u32>),
          (0x04 => bar: ReadWrite<u32>),
          (0x10 => @END),
      }
  }
  # // This is required for rustdoc to not place this code snipped into an
  # // fn main() {...} function.
  # fn main() { }
  ```

```rust
macro_rules! test_fields {
    (@root $struct:ident $(<$life:lifetime>)? { $($input:tt)* } ) => { ... };
    (@munch $struct:ident $(<$life:lifetime>)?
        (
            $(#[$attr_end:meta])*
            ($size:expr => @END),
        )
        : $stmts:expr
    ) => { ... };
    (@munch $struct:ident $(<$life:lifetime>)?
        (
            $(#[$attr:meta])*
            ($offset_start:expr => $vis:vis $field:ident: $ty:ty),
            $(#[$attr_next:meta])*
            ($offset_end:expr => $($next:tt)*),
            $($after:tt)*
        )
        : $output:expr
    ) => { ... };
    (@munch $struct:ident $(<$life:lifetime>)?
        (
            $(#[$attr:meta])*
            ($offset_start:expr => $padding:ident),
            $(#[$attr_next:meta])*
            ($offset_end:expr => $($next:tt)*),
            $($after:tt)*
        )
        : $output:expr
    ) => { ... };
}
```



