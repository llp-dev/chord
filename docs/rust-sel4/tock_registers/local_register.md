**tock_registers > local_register**

# Module: local_register

## Contents

**Macros**

- [`From_impl_for`](#from_impl_for)

**Structs**

- [`LocalRegisterCopy`](#localregistercopy) - A read-write copy of register contents.

---

## tock_registers::local_register::From_impl_for

*Declarative Macro*

```rust
macro_rules! From_impl_for {
    ($type:ty) => { ... };
}
```



## tock_registers::local_register::LocalRegisterCopy

*Struct*

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

**Generic Parameters:**
- T
- R

**Fields:**
- `value: T`
- `associated_register: core::marker::PhantomData<R>`

**Methods:**

- `fn new(value: T) -> Self`
- `fn get(self: &Self) -> T` - Get the raw register value
- `fn set(self: & mut Self, value: T)` - Set the raw register value
- `fn read(self: &Self, field: Field<T, R>) -> T` - Read the value of the given field
- `fn read_as_enum<E>(self: &Self, field: Field<T, R>) -> Option<E>` - Read value of the given field as an enum member
- `fn write(self: & mut Self, field: FieldValue<T, R>)` - Write the value of one or more fields, overwriting the other fields with
- `fn modify(self: & mut Self, field: FieldValue<T, R>)` - Write the value of one or more fields, leaving the other fields
- `fn is_set(self: &Self, field: Field<T, R>) -> bool` - Check if one or more bits in a field are set
- `fn any_matching_bits_set(self: &Self, field: FieldValue<T, R>) -> bool` - Check if any bits corresponding to the mask in the passed `FieldValue`
- `fn matches_all(self: &Self, field: FieldValue<T, R>) -> bool` - Check if all specified parts of a field match
- `fn matches_any(self: &Self, fields: &[FieldValue<T, R>]) -> bool` - Check if any of the passed parts of a field exactly match the contained
- `fn bitand(self: &Self, rhs: T) -> LocalRegisterCopy<T, R>` - Do a bitwise AND operation of the stored value and the passed in value
- `fn debug(self: &Self) -> crate::debug::RegisterDebugValue<T, R>`

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> LocalRegisterCopy<T, R>`



