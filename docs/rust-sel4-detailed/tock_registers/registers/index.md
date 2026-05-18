*[tock_registers](../index.md) / [registers](index.md)*

---

# Module `registers`

Implementation of included register types.

This module provides a standard set of register types, which can describe
different access levels:

- [`ReadWrite`](#readwrite) for registers which can be read and written to
- [`ReadOnly`](#readonly) for registers which can only be read
- [`WriteOnly`](#writeonly) for registers which can only be written to
- [`Aliased`](#aliased) for registers which can be both read and written, but
  represent different registers depending on the operation
- [`InMemoryRegister`](#inmemoryregister) provide a register-type in RAM using volatile
  operations

These types can be disabled by removing the `register_types` crate feature
(part of the default features). This is useful if this crate should be used
only as an interface library, or if all unsafe code should be disabled.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ReadWrite`](#readwrite) | struct | Read/Write registers. |
| [`ReadOnly`](#readonly) | struct | Read-only registers. |
| [`WriteOnly`](#writeonly) | struct | Write-only registers. |
| [`Aliased`](#aliased) | struct | Read-only and write-only registers aliased to the same address. |
| [`InMemoryRegister`](#inmemoryregister) | struct | In memory volatile register. |

## Structs

### `ReadWrite<T: UIntLike, R: RegisterLongName>`

```rust
struct ReadWrite<T: UIntLike, R: RegisterLongName> {
    value: core::cell::UnsafeCell<T>,
    associated_register: core::marker::PhantomData<R>,
}
```

Read/Write registers.

For accessing and manipulating the register contents, the [`Readable`](../interfaces/index.md),
[`Writeable`](../interfaces/index.md) and [`ReadWriteable`](crate::interfaces::ReadWriteable) traits
are implemented.

#### Trait Implementations

##### `impl<T> Debuggable for ReadWrite<T, R>`

##### `impl<T, R> ReadWriteable for ReadWrite<T, R>`

- <span id="readwrite-readwriteable-type-t"></span>`type T = T`

- <span id="readwrite-readwriteable-type-r"></span>`type R = R`

- <span id="readwrite-readwriteable-modify"></span>`fn modify(&self, field: FieldValue<<S as ReadWriteable>::T, <S as ReadWriteable>::R>)` — [`FieldValue`](../fields/index.md#fieldvalue), [`ReadWriteable`](../interfaces/index.md#readwriteable)

##### `impl<T: UIntLike, R: RegisterLongName> Readable for ReadWrite<T, R>`

- <span id="readwrite-readable-type-t"></span>`type T = T`

- <span id="readwrite-readable-type-r"></span>`type R = R`

- <span id="readwrite-readable-get"></span>`fn get(&self) -> <Self as >::T` — [`Readable`](../interfaces/index.md#readable)

##### `impl<T: UIntLike, R: RegisterLongName> Writeable for ReadWrite<T, R>`

- <span id="readwrite-writeable-type-t"></span>`type T = T`

- <span id="readwrite-writeable-type-r"></span>`type R = R`

- <span id="readwrite-writeable-set"></span>`fn set(&self, value: T)`

### `ReadOnly<T: UIntLike, R: RegisterLongName>`

```rust
struct ReadOnly<T: UIntLike, R: RegisterLongName> {
    value: T,
    associated_register: core::marker::PhantomData<R>,
}
```

Read-only registers.

For accessing the register contents the [`Readable`](../interfaces/index.md) trait is implemented.

#### Trait Implementations

##### `impl<T> Debuggable for ReadOnly<T, R>`

##### `impl<T: UIntLike, R: RegisterLongName> Readable for ReadOnly<T, R>`

- <span id="readonly-readable-type-t"></span>`type T = T`

- <span id="readonly-readable-type-r"></span>`type R = R`

- <span id="readonly-readable-get"></span>`fn get(&self) -> T`

### `WriteOnly<T: UIntLike, R: RegisterLongName>`

```rust
struct WriteOnly<T: UIntLike, R: RegisterLongName> {
    value: core::cell::UnsafeCell<T>,
    associated_register: core::marker::PhantomData<R>,
}
```

Write-only registers.

For setting the register contents the [`Writeable`](../interfaces/index.md) trait is implemented.

#### Trait Implementations

##### `impl<T: UIntLike, R: RegisterLongName> Writeable for WriteOnly<T, R>`

- <span id="writeonly-writeable-type-t"></span>`type T = T`

- <span id="writeonly-writeable-type-r"></span>`type R = R`

- <span id="writeonly-writeable-set"></span>`fn set(&self, value: T)`

### `Aliased<T: UIntLike, R: RegisterLongName, W: RegisterLongName>`

```rust
struct Aliased<T: UIntLike, R: RegisterLongName, W: RegisterLongName> {
    value: core::cell::UnsafeCell<T>,
    associated_register: core::marker::PhantomData<(R, W)>,
}
```

Read-only and write-only registers aliased to the same address.

Unlike the [`ReadWrite`](#readwrite) register, this represents a register which has
different meanings based on if it is written or read.  This might be found
on a device where control and status registers are accessed via the same
memory address via writes and reads, respectively.

This register implements [`Readable`](../interfaces/index.md) and [`Writeable`](../interfaces/index.md), but in general does
not implement [`ReadWriteable`](crate::interfaces::ReadWriteable) (only if
the type parameters `R` and `W` are identical, in which case a [`ReadWrite`](#readwrite)
register might be a better choice).

#### Trait Implementations

##### `impl<T> Debuggable for Aliased<T, R, W>`

##### `impl<T, R> ReadWriteable for Aliased<T, R, W>`

- <span id="aliased-readwriteable-type-t"></span>`type T = T`

- <span id="aliased-readwriteable-type-r"></span>`type R = R`

- <span id="aliased-readwriteable-modify"></span>`fn modify(&self, field: FieldValue<<S as ReadWriteable>::T, <S as ReadWriteable>::R>)` — [`FieldValue`](../fields/index.md#fieldvalue), [`ReadWriteable`](../interfaces/index.md#readwriteable)

##### `impl<T: UIntLike, R: RegisterLongName, W: RegisterLongName> Readable for Aliased<T, R, W>`

- <span id="aliased-readable-type-t"></span>`type T = T`

- <span id="aliased-readable-type-r"></span>`type R = R`

- <span id="aliased-readable-get"></span>`fn get(&self) -> <Self as >::T` — [`Readable`](../interfaces/index.md#readable)

##### `impl<T: UIntLike, R: RegisterLongName, W: RegisterLongName> Writeable for Aliased<T, R, W>`

- <span id="aliased-writeable-type-t"></span>`type T = T`

- <span id="aliased-writeable-type-r"></span>`type R = W`

- <span id="aliased-writeable-set"></span>`fn set(&self, value: <Self as >::T)` — [`Writeable`](../interfaces/index.md#writeable)

### `InMemoryRegister<T: UIntLike, R: RegisterLongName>`

```rust
struct InMemoryRegister<T: UIntLike, R: RegisterLongName> {
    value: core::cell::UnsafeCell<T>,
    associated_register: core::marker::PhantomData<R>,
}
```

In memory volatile register.

Like [`ReadWrite`](#readwrite), but can be safely constructed using the
`InMemoryRegister::new` method. It will always be initialized to the
passed in, well-defined initial value.

For accessing and manipulating the register contents, the [`Readable`](../interfaces/index.md),
[`Writeable`](../interfaces/index.md) and [`ReadWriteable`](crate::interfaces::ReadWriteable) traits
are implemented.

#### Implementations

- <span id="inmemoryregister-new"></span>`const fn new(value: T) -> Self`

#### Trait Implementations

##### `impl<T> Debuggable for InMemoryRegister<T, R>`

##### `impl<T, R> ReadWriteable for InMemoryRegister<T, R>`

- <span id="inmemoryregister-readwriteable-type-t"></span>`type T = T`

- <span id="inmemoryregister-readwriteable-type-r"></span>`type R = R`

- <span id="inmemoryregister-readwriteable-modify"></span>`fn modify(&self, field: FieldValue<<S as ReadWriteable>::T, <S as ReadWriteable>::R>)` — [`FieldValue`](../fields/index.md#fieldvalue), [`ReadWriteable`](../interfaces/index.md#readwriteable)

##### `impl<T: UIntLike, R: RegisterLongName> Readable for InMemoryRegister<T, R>`

- <span id="inmemoryregister-readable-type-t"></span>`type T = T`

- <span id="inmemoryregister-readable-type-r"></span>`type R = R`

- <span id="inmemoryregister-readable-get"></span>`fn get(&self) -> <Self as >::T` — [`Readable`](../interfaces/index.md#readable)

##### `impl<T: UIntLike, R: RegisterLongName> Writeable for InMemoryRegister<T, R>`

- <span id="inmemoryregister-writeable-type-t"></span>`type T = T`

- <span id="inmemoryregister-writeable-type-r"></span>`type R = R`

- <span id="inmemoryregister-writeable-set"></span>`fn set(&self, value: T)`

