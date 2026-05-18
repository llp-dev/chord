*[tock_registers](../index.md) / [interfaces](index.md)*

---

# Module `interfaces`

Interfaces (traits) to register types

This module contains traits which reflect standardized interfaces to
different types of registers. Examples of registers implementing these
interfaces are [`ReadWrite`](crate::registers::ReadWrite) or
[`InMemoryRegister`](crate::registers::InMemoryRegister).

Each trait has two associated type parameters, namely:

- `T`: [`UIntLike`](../index.md), indicating the underlying integer type used to
  represent the register's raw contents.

- `R`: [`RegisterLongName`](../index.md), functioning as a type to identify this
  register's descriptive name and semantic meaning. It is further used to
  impose type constraints on values passed through the API, such as
  [`FieldValue`](../fields/index.md).

Registers can have different access levels, which are mapped to different
traits respectively:

- [`Readable`](#readable): indicates that the current value of this register can be
  read. Implementations will need to provide the
  [`get`](crate::interfaces::Readable::get) method.

- [`Writeable`](#writeable): indicates that the value of this register can be
  set. Implementations will need to provide the
  [`set`](crate::interfaces::Writeable::set) method.

- [`ReadWriteable`](#readwriteable): indicates that this register can be _modified_. It is
  not sufficient for registers to be both read- and writable, they must also
  have the same semantic meaning when read from and written to. This is not
  true in general, for example a memory-mapped UART register might transmit
  when writing and receive when reading.

  If a type implements both [`Readable`](#readable) and [`Writeable`](#writeable), and the
  associated [`RegisterLongName`](../index.md) type parameters are identical, it will
  automatically implement [`ReadWriteable`](#readwriteable). In particular, for
  [`Aliased`](crate::registers::Aliased) this is -- in general -- not the
  case, so

  ```rust
  use tock_registers::interfaces::{Readable, Writeable, ReadWriteable};
  use tock_registers::registers::ReadWrite;
  use tock_registers::register_bitfields;
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
  use tock_registers::interfaces::{Readable, Writeable, ReadWriteable};
  use tock_registers::registers::Aliased;
  use tock_registers::register_bitfields;
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

- [`Debuggable`](#debuggable): indicates that the register supports producing
  human-readable debug output using the `RegisterDebugValue` type. This type
  can be produced with the [`debug`](crate::interfaces::Debuggable::debug)
  method. This will return a value that implements
  [`Debug`](core::fmt::Debug). It is automticaly implemented for any
  register implementing [`Readable`](#readable).


## Example: implementing a custom register type

These traits can be used to implement custom register types, which are
compatible to the ones shipped in this crate. For example, to define a
register which sets a `u8` value using a Cell reference, always reads the
bitwise-negated vale and prints every written value to the console:

```rust
use core::cell::Cell;
use core::marker::PhantomData;

use tock_registers::interfaces::{Readable, Writeable, ReadWriteable};
use tock_registers::RegisterLongName;
use tock_registers::register_bitfields;

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

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Readable`](#readable) | trait | Readable register |
| [`Debuggable`](#debuggable) | trait | [`Debuggable`] is a trait for registers that support human-readable debug output with [`core::fmt::Debug`]. |
| [`Writeable`](#writeable) | trait | Writeable register |
| [`ReadWriteable`](#readwriteable) | trait | [`Readable`] and [`Writeable`] register, over the same [`RegisterLongName`] |

## Traits

### `Readable`

```rust
trait Readable { ... }
```

Readable register

Register which at least supports reading the current value. Only
`Readable::get` must be implemented, as for other methods a default
implementation is provided.

A register that is both [`Readable`](#readable) and [`Writeable`](#writeable) will also
automatically be [`ReadWriteable`](#readwriteable), if the [`RegisterLongName`](../index.md) of
[`Readable`](#readable) is the same as that of [`Writeable`](#writeable) (i.e. not for
[`Aliased`](crate::registers::Aliased) registers).

#### Associated Types

- `type T: 1`

- `type R: 1`

#### Required Methods

- `fn get(&self) -> <Self as >::T`

  Get the raw register value

#### Provided Methods

- `fn read(&self, field: Field<<Self as >::T, <Self as >::R>) -> <Self as >::T`

  Read the value of the given field

- `fn read_as_enum<E: TryFromValue<<Self as >::T, EnumType = E>>(&self, field: Field<<Self as >::T, <Self as >::R>) -> Option<E>`

  Set the raw register value

- `fn extract(&self) -> LocalRegisterCopy<<Self as >::T, <Self as >::R>`

  Make a local copy of the register

- `fn is_set(&self, field: Field<<Self as >::T, <Self as >::R>) -> bool`

  Check if one or more bits in a field are set

- `fn any_matching_bits_set(&self, field: FieldValue<<Self as >::T, <Self as >::R>) -> bool`

  Check if any bits corresponding to the mask in the passed `FieldValue`

- `fn matches_all(&self, field: FieldValue<<Self as >::T, <Self as >::R>) -> bool`

  Check if all specified parts of a field match

- `fn matches_any(&self, fields: &[FieldValue<<Self as >::T, <Self as >::R>]) -> bool`

  Check if any of the passed parts of a field exactly match the contained

#### Implementors

- [`Aliased`](../registers/index.md#aliased)
- [`InMemoryRegister`](../registers/index.md#inmemoryregister)
- [`ReadOnly`](../registers/index.md#readonly)
- [`ReadWrite`](../registers/index.md#readwrite)

### `Debuggable`

```rust
trait Debuggable: Readable { ... }
```

[`Debuggable`](#debuggable) is a trait for registers that support human-readable debug
output with [`core::fmt::Debug`](../../log/index.md).  It extends the [`Readable`](#readable) trait and
doesn't require manual implementation.

This is implemented for the register when using the [`register_bitfields`](../index.md)
macro.

The `debug` method returns a value that implements [`core::fmt::Debug`](../../log/index.md).


#### Provided Methods

- `fn debug(&self) -> crate::debug::RegisterDebugValue<<Self as >::T, <Self as >::R>`

  Returns a [`RegisterDebugValue`](crate::debug::RegisterDebugValue) that

#### Implementors

- `T`

### `Writeable`

```rust
trait Writeable { ... }
```

Writeable register

Register which at least supports setting a value. Only `Writeable::set`
must be implemented, as for other methods a default implementation is
provided.

A register that is both [`Readable`](#readable) and [`Writeable`](#writeable) will also
automatically be [`ReadWriteable`](#readwriteable), if the [`RegisterLongName`](../index.md) of
[`Readable`](#readable) is the same as that of [`Writeable`](#writeable) (i.e. not for
[`Aliased`](crate::registers::Aliased) registers).

#### Associated Types

- `type T: 1`

- `type R: 1`

#### Required Methods

- `fn set(&self, value: <Self as >::T)`

  Set the raw register value

#### Provided Methods

- `fn write(&self, field: FieldValue<<Self as >::T, <Self as >::R>)`

  Write the value of one or more fields, overwriting the other fields with

- `fn modify_no_read(&self, original: LocalRegisterCopy<<Self as >::T, <Self as >::R>, field: FieldValue<<Self as >::T, <Self as >::R>)`

  Write the value of one or more fields, maintaining the value of

#### Implementors

- [`Aliased`](../registers/index.md#aliased)
- [`InMemoryRegister`](../registers/index.md#inmemoryregister)
- [`ReadWrite`](../registers/index.md#readwrite)
- [`WriteOnly`](../registers/index.md#writeonly)

### `ReadWriteable`

```rust
trait ReadWriteable { ... }
```

[`Readable`](#readable) and [`Writeable`](#writeable) register, over the same [`RegisterLongName`](../index.md)

Register which supports both reading and setting a value.

**This trait does not have to be implemented manually!** It is automatically
implemented for every type that is both [`Readable`](#readable) and [`Writeable`](#writeable), as
long as `Readable::R` == `Writeable::R` (i.e. not for
[`Aliased`](crate::registers::Aliased) registers).

#### Associated Types

- `type T: 1`

- `type R: 1`

#### Required Methods

- `fn modify(&self, field: FieldValue<<Self as >::T, <Self as >::R>)`

  Write the value of one or more fields, leaving the other fields

#### Implementors

- `S`

