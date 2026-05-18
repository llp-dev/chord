**tock_registers > interfaces**

# Module: interfaces

## Contents

**Traits**

- [`Debuggable`](#debuggable) - [`Debuggable`] is a trait for registers that support human-readable debug
- [`ReadWriteable`](#readwriteable) - [`Readable`] and [`Writeable`] register, over the same [`RegisterLongName`]
- [`Readable`](#readable) - Readable register
- [`Writeable`](#writeable) - Writeable register

---

## tock_registers::interfaces::Debuggable

*Trait*

[`Debuggable`] is a trait for registers that support human-readable debug
output with [`core::fmt::Debug`].  It extends the [`Readable`] trait and
doesn't require manual implementation.

This is implemented for the register when using the [`register_bitfields`]
macro.

The `debug` method returns a value that implements [`core::fmt::Debug`].

[`register_bitfields`]: crate::register_bitfields

**Methods:**

- `debug`: Returns a [`RegisterDebugValue`](crate::debug::RegisterDebugValue) that



## tock_registers::interfaces::ReadWriteable

*Trait*

[`Readable`] and [`Writeable`] register, over the same [`RegisterLongName`]

Register which supports both reading and setting a value.

**This trait does not have to be implemented manually!** It is automatically
implemented for every type that is both [`Readable`] and [`Writeable`], as
long as [`Readable::R`] == [`Writeable::R`] (i.e. not for
[`Aliased`](crate::registers::Aliased) registers).

**Methods:**

- `T`
- `R`
- `modify`: Write the value of one or more fields, leaving the other fields



## tock_registers::interfaces::Readable

*Trait*

Readable register

Register which at least supports reading the current value. Only
[`Readable::get`] must be implemented, as for other methods a default
implementation is provided.

A register that is both [`Readable`] and [`Writeable`] will also
automatically be [`ReadWriteable`], if the [`RegisterLongName`] of
[`Readable`] is the same as that of [`Writeable`] (i.e. not for
[`Aliased`](crate::registers::Aliased) registers).

**Methods:**

- `T`
- `R`
- `get`: Get the raw register value
- `read`: Read the value of the given field
- `read_as_enum`: Set the raw register value
- `extract`: Make a local copy of the register
- `is_set`: Check if one or more bits in a field are set
- `any_matching_bits_set`: Check if any bits corresponding to the mask in the passed `FieldValue`
- `matches_all`: Check if all specified parts of a field match
- `matches_any`: Check if any of the passed parts of a field exactly match the contained



## tock_registers::interfaces::Writeable

*Trait*

Writeable register

Register which at least supports setting a value. Only [`Writeable::set`]
must be implemented, as for other methods a default implementation is
provided.

A register that is both [`Readable`] and [`Writeable`] will also
automatically be [`ReadWriteable`], if the [`RegisterLongName`] of
[`Readable`] is the same as that of [`Writeable`] (i.e. not for
[`Aliased`](crate::registers::Aliased) registers).

**Methods:**

- `T`
- `R`
- `set`: Set the raw register value
- `write`: Write the value of one or more fields, overwriting the other fields with
- `modify_no_read`: Write the value of one or more fields, maintaining the value of



