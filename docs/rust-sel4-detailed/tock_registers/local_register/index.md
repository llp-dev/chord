*[tock_registers](../index.md) / [local_register](index.md)*

---

# Module `local_register`

Module containing the [`LocalRegisterCopy`](#localregistercopy) type. Please refer to
its documentation.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`LocalRegisterCopy`](#localregistercopy) | struct | A read-write copy of register contents. |
| [`From_impl_for!`](#from-impl-for) | macro |  |

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

- <span id="localregistercopy-read"></span>`fn read(&self, field: Field<T, R>) -> T` — [`Field`](../fields/index.md#field)

  Read the value of the given field

- <span id="localregistercopy-read-as-enum"></span>`fn read_as_enum<E: TryFromValue<T, EnumType = E>>(&self, field: Field<T, R>) -> Option<E>` — [`Field`](../fields/index.md#field)

  Read value of the given field as an enum member

- <span id="localregistercopy-write"></span>`fn write(&mut self, field: FieldValue<T, R>)` — [`FieldValue`](../fields/index.md#fieldvalue)

  Write the value of one or more fields, overwriting the other fields with

  zero

- <span id="localregistercopy-modify"></span>`fn modify(&mut self, field: FieldValue<T, R>)` — [`FieldValue`](../fields/index.md#fieldvalue)

  Write the value of one or more fields, leaving the other fields

  unchanged

- <span id="localregistercopy-is-set"></span>`fn is_set(&self, field: Field<T, R>) -> bool` — [`Field`](../fields/index.md#field)

  Check if one or more bits in a field are set

- <span id="localregistercopy-any-matching-bits-set"></span>`fn any_matching_bits_set(&self, field: FieldValue<T, R>) -> bool` — [`FieldValue`](../fields/index.md#fieldvalue)

  Check if any bits corresponding to the mask in the passed `FieldValue`

  are set.

- <span id="localregistercopy-matches-all"></span>`fn matches_all(&self, field: FieldValue<T, R>) -> bool` — [`FieldValue`](../fields/index.md#fieldvalue)

  Check if all specified parts of a field match

- <span id="localregistercopy-matches-any"></span>`fn matches_any(&self, fields: &[FieldValue<T, R>]) -> bool` — [`FieldValue`](../fields/index.md#fieldvalue)

  Check if any of the passed parts of a field exactly match the contained

  value. This allows for matching on unset bits, or matching on specific

  values in multi-bit fields.

- <span id="localregistercopy-bitand"></span>`fn bitand(&self, rhs: T) -> LocalRegisterCopy<T, R>` — [`LocalRegisterCopy`](#localregistercopy)

  Do a bitwise AND operation of the stored value and the passed in value

  and return a new LocalRegisterCopy.

- <span id="localregistercopy-debug"></span>`fn debug(&self) -> crate::debug::RegisterDebugValue<T, R>` — [`RegisterDebugValue`](../debug/index.md#registerdebugvalue)

#### Trait Implementations

##### `impl<T: clone::Clone + UIntLike, R: clone::Clone + RegisterLongName> Clone for LocalRegisterCopy<T, R>`

- <span id="localregistercopy-clone"></span>`fn clone(&self) -> LocalRegisterCopy<T, R>` — [`LocalRegisterCopy`](#localregistercopy)

##### `impl<T: marker::Copy + UIntLike, R: marker::Copy + RegisterLongName> Copy for LocalRegisterCopy<T, R>`

##### `impl<T: UIntLike + fmt::Debug, R: RegisterLongName> Debug for LocalRegisterCopy<T, R>`

- <span id="localregistercopy-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Macros

### `From_impl_for!`

