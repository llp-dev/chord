**tock_registers > registers**

# Module: registers

## Contents

**Structs**

- [`Aliased`](#aliased) - Read-only and write-only registers aliased to the same address.
- [`InMemoryRegister`](#inmemoryregister) - In memory volatile register.
- [`ReadOnly`](#readonly) - Read-only registers.
- [`ReadWrite`](#readwrite) - Read/Write registers.
- [`WriteOnly`](#writeonly) - Write-only registers.

---

## tock_registers::registers::Aliased

*Struct*

Read-only and write-only registers aliased to the same address.

Unlike the [`ReadWrite`] register, this represents a register which has
different meanings based on if it is written or read.  This might be found
on a device where control and status registers are accessed via the same
memory address via writes and reads, respectively.

This register implements [`Readable`] and [`Writeable`], but in general does
not implement [`ReadWriteable`](crate::interfaces::ReadWriteable) (only if
the type parameters `R` and `W` are identical, in which case a [`ReadWrite`]
register might be a better choice).

**Generic Parameters:**
- T
- R
- W

**Fields:**
- `value: core::cell::UnsafeCell<T>`
- `associated_register: core::marker::PhantomData<(R, W)>`

**Trait Implementations:**

- **Writeable**
  - `fn set(self: &Self, value: <Self as >::T)`
- **Readable**
  - `fn get(self: &Self) -> <Self as >::T`



## tock_registers::registers::InMemoryRegister

*Struct*

In memory volatile register.

Like [`ReadWrite`], but can be safely constructed using the
[`InMemoryRegister::new`] method. It will always be initialized to the
passed in, well-defined initial value.

For accessing and manipulating the register contents, the [`Readable`],
[`Writeable`] and [`ReadWriteable`](crate::interfaces::ReadWriteable) traits
are implemented.

**Generic Parameters:**
- T
- R

**Fields:**
- `value: core::cell::UnsafeCell<T>`
- `associated_register: core::marker::PhantomData<R>`

**Methods:**

- `fn new(value: T) -> Self`

**Trait Implementations:**

- **Readable**
  - `fn get(self: &Self) -> <Self as >::T`
- **Writeable**
  - `fn set(self: &Self, value: T)`



## tock_registers::registers::ReadOnly

*Struct*

Read-only registers.

For accessing the register contents the [`Readable`] trait is implemented.

**Generic Parameters:**
- T
- R

**Fields:**
- `value: T`
- `associated_register: core::marker::PhantomData<R>`

**Trait Implementations:**

- **Readable**
  - `fn get(self: &Self) -> T`



## tock_registers::registers::ReadWrite

*Struct*

Read/Write registers.

For accessing and manipulating the register contents, the [`Readable`],
[`Writeable`] and [`ReadWriteable`](crate::interfaces::ReadWriteable) traits
are implemented.

**Generic Parameters:**
- T
- R

**Fields:**
- `value: core::cell::UnsafeCell<T>`
- `associated_register: core::marker::PhantomData<R>`

**Trait Implementations:**

- **Writeable**
  - `fn set(self: &Self, value: T)`
- **Readable**
  - `fn get(self: &Self) -> <Self as >::T`



## tock_registers::registers::WriteOnly

*Struct*

Write-only registers.

For setting the register contents the [`Writeable`] trait is implemented.

**Generic Parameters:**
- T
- R

**Fields:**
- `value: core::cell::UnsafeCell<T>`
- `associated_register: core::marker::PhantomData<R>`

**Trait Implementations:**

- **Writeable**
  - `fn set(self: &Self, value: T)`



