**tock_registers > fields**

# Module: fields

## Contents

**Macros**

- [`FieldValue_impl_for`](#fieldvalue_impl_for)
- [`Field_impl_for`](#field_impl_for)

**Structs**

- [`Field`](#field) - Specific section of a register.
- [`FieldValue`](#fieldvalue) - Values for the specific register fields.

**Traits**

- [`TryFromValue`](#tryfromvalue) - Conversion of raw register value into enumerated values member. Implemented

---

## tock_registers::fields::Field

*Struct*

Specific section of a register.

For the Field, the mask is unshifted, ie. the LSB should always be set.

**Generic Parameters:**
- T
- R

**Fields:**
- `mask: T`
- `shift: usize`
- `associated_register: core::marker::PhantomData<R>`

**Methods:**

- `fn val(self: &Self, value: u64) -> FieldValue<u64, R>`
- `fn val(self: &Self, value: u32) -> FieldValue<u32, R>`
- `fn val(self: &Self, value: u16) -> FieldValue<u16, R>`
- `fn val(self: &Self, value: u8) -> FieldValue<u8, R>`
- `fn new(mask: T, shift: usize) -> Field<T, R>`
- `fn read(self: Self, val: T) -> T`
- `fn is_set(self: Self, val: T) -> bool` - Check if one or more bits in a field are set
- `fn read_as_enum<E>(self: Self, val: T) -> Option<E>` - Read value of the field as an enum member
- `fn val(self: &Self, value: usize) -> FieldValue<usize, R>`
- `fn val(self: &Self, value: u128) -> FieldValue<u128, R>`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`



## tock_registers::fields::FieldValue

*Struct*

Values for the specific register fields.

For the FieldValue, the masks and values are shifted into their actual
location in the register.

**Generic Parameters:**
- T
- R

**Fields:**
- `mask: T`
- `value: T`
- `associated_register: core::marker::PhantomData<R>`

**Methods:**

- `fn none() -> Self`
- `fn mask(self: &Self) -> T` - Get the raw bitmask represented by this FieldValue.
- `fn read(self: &Self, field: Field<T, R>) -> T`
- `fn modify(self: Self, val: T) -> T` - Modify fields in a register value
- `fn any_matching_bits_set(self: &Self, val: T) -> bool` - Check if any of the bits covered by the mask for this `FieldValue` and
- `fn matches_all(self: &Self, val: T) -> bool` - Check if all specified parts of a field match
- `fn new(mask: usize, shift: usize, value: usize) -> Self`
- `fn new(mask: u128, shift: usize, value: u128) -> Self`
- `fn new(mask: u64, shift: usize, value: u64) -> Self`
- `fn new(mask: u32, shift: usize, value: u32) -> Self`
- `fn new(mask: u16, shift: usize, value: u16) -> Self`
- `fn new(mask: u8, shift: usize, value: u8) -> Self`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> FieldValue<T, R>`
- **AddAssign**
  - `fn add_assign(self: & mut Self, rhs: FieldValue<T, R>)`
- **Add**
  - `fn add(self: Self, rhs: Self) -> Self`



## tock_registers::fields::FieldValue_impl_for

*Declarative Macro*

```rust
macro_rules! FieldValue_impl_for {
    ($type:ty) => { ... };
}
```



## tock_registers::fields::Field_impl_for

*Declarative Macro*

```rust
macro_rules! Field_impl_for {
    ($type:ty) => { ... };
}
```



## tock_registers::fields::TryFromValue

*Trait*

Conversion of raw register value into enumerated values member. Implemented
inside register_bitfields! macro for each bit field.

**Methods:**

- `EnumType`
- `try_from_value`



