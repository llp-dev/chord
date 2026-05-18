*[tock_registers](../index.md) / [fields](index.md)*

---

# Module `fields`

Register bitfield types and macros

To conveniently access and manipulate fields of a register, this library
provides types and macros to describe and access bitfields of a
register. This can be especially useful in conjunction with the APIs defined
in [`interfaces`](crate::interfaces), which make use of these types and
hence allow to access and manipulate bitfields of proper registers directly.

A specific section (bitfield) in a register is described by the [`Field`](#field)
type, consisting of an unshifted bitmask over the base register [`UIntLike`](../index.md)
type, and a shift parameter. It is further associated with a specific
[`RegisterLongName`](../index.md), which can prevent its use with incompatible registers.

A value of a section of a register is described by the [`FieldValue`](#fieldvalue)
type. It stores the information of the respective section in the register,
as well as the associated value. A [`FieldValue`](#fieldvalue) can be created from a
[`Field`](#field) through the [`val`](Field::val) method.

## `register_bitfields` macro

For defining register layouts with an associated [`RegisterLongName`](../index.md), along
with [`Field`](#field)s and matching [`FieldValue`](#fieldvalue)s, a convenient macro-based
interface can be used.

The following example demonstrates how two registers can be defined, over a
`u32` base type:

```rust
use tock_registers::register_bitfields;
use tock_registers::registers::InMemoryRegister;
use tock_registers::interfaces::{Readable, ReadWriteable};
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

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Field`](#field) | struct | Specific section of a register. |
| [`FieldValue`](#fieldvalue) | struct | Values for the specific register fields. |
| [`TryFromValue`](#tryfromvalue) | trait | Conversion of raw register value into enumerated values member. |
| [`Field_impl_for!`](#field-impl-for) | macro |  |
| [`FieldValue_impl_for!`](#fieldvalue-impl-for) | macro |  |

## Structs

### `Field<T: UIntLike, R: RegisterLongName>`

```rust
struct Field<T: UIntLike, R: RegisterLongName> {
    pub mask: T,
    pub shift: usize,
    associated_register: core::marker::PhantomData<R>,
}
```

Specific section of a register.

For the Field, the mask is unshifted, ie. the LSB should always be set.

#### Implementations

- <span id="field-new"></span>`const fn new(mask: T, shift: usize) -> Field<T, R>` — [`Field`](#field)

- <span id="field-read"></span>`fn read(self, val: T) -> T`

- <span id="field-is-set"></span>`fn is_set(self, val: T) -> bool`

  Check if one or more bits in a field are set

- <span id="field-read-as-enum"></span>`fn read_as_enum<E: TryFromValue<T, EnumType = E>>(self, val: T) -> Option<E>`

  Read value of the field as an enum member

  

  This method expects to be passed the unasked and unshifted register

  value, extracts the field value by calling `Field::read` and

  subsequently passes that value to the [`TryFromValue`](#tryfromvalue) implementation on

  the passed enum type.

  

  The [`register_bitfields!`](crate::register_bitfields) macro will

  generate an enum containing the various named field variants and

  implementing the required [`TryFromValue`](#tryfromvalue) trait. It is accessible as

  `$REGISTER_NAME::$FIELD_NAME::Value`.

  

  This method can be useful to symbolically represent read register field

  states throughout the codebase and to enforce exhaustive matches over

  all defined valid register field values.

  

  ## Usage Example

  

  ```rust

  use tock_registers::interfaces::Readable;

  use tock_registers::registers::InMemoryRegister;

  use tock_registers::register_bitfields;

  register_bitfields![u8,

      EXAMPLEREG [

          TESTFIELD OFFSET(3) NUMBITS(3) [

              Foo = 2,

              Bar = 3,

              Baz = 6,

          ],

      ],

  ];

  

  assert_eq!(

      EXAMPLEREG::TESTFIELD.read_as_enum::<EXAMPLEREG::TESTFIELD::Value>(0x9C).unwrap(),

      EXAMPLEREG::TESTFIELD::Value::Bar

  );

  ```

#### Trait Implementations

##### `impl<T: UIntLike, R: RegisterLongName> Clone for Field<T, R>`

- <span id="field-clone"></span>`fn clone(&self) -> Self`

##### `impl<T: UIntLike, R: RegisterLongName> Copy for Field<T, R>`

### `FieldValue<T: UIntLike, R: RegisterLongName>`

```rust
struct FieldValue<T: UIntLike, R: RegisterLongName> {
    mask: T,
    pub value: T,
    associated_register: core::marker::PhantomData<R>,
}
```

Values for the specific register fields.

For the FieldValue, the masks and values are shifted into their actual
location in the register.

#### Implementations

- <span id="fieldvalue-new"></span>`const fn new(mask: u8, shift: usize, value: u8) -> Self`

#### Trait Implementations

##### `impl<T: UIntLike, R: RegisterLongName> Add for FieldValue<T, R>`

- <span id="fieldvalue-add-type-output"></span>`type Output = FieldValue<T, R>`

- <span id="fieldvalue-add"></span>`fn add(self, rhs: Self) -> Self`

##### `impl<T: UIntLike, R: RegisterLongName> AddAssign for FieldValue<T, R>`

- <span id="fieldvalue-addassign-add-assign"></span>`fn add_assign(&mut self, rhs: FieldValue<T, R>)` — [`FieldValue`](#fieldvalue)

##### `impl<T: clone::Clone + UIntLike, R: clone::Clone + RegisterLongName> Clone for FieldValue<T, R>`

- <span id="fieldvalue-clone"></span>`fn clone(&self) -> FieldValue<T, R>` — [`FieldValue`](#fieldvalue)

##### `impl<T: marker::Copy + UIntLike, R: marker::Copy + RegisterLongName> Copy for FieldValue<T, R>`

## Traits

### `TryFromValue<V>`

```rust
trait TryFromValue<V> { ... }
```

Conversion of raw register value into enumerated values member. Implemented
inside register_bitfields! macro for each bit field.

#### Associated Types

- `type EnumType`

#### Required Methods

- `fn try_from_value(v: V) -> Option<<Self as >::EnumType>`

## Macros

### `Field_impl_for!`

### `FieldValue_impl_for!`

