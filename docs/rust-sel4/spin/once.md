**spin > once**

# Module: once

## Contents

**Modules**

- [`status`](#status)

**Structs**

- [`Finish`](#finish)
- [`Once`](#once) - A primitive that provides lazy one-time initialization.

---

## spin::once::Finish

*Struct*

**Generic Parameters:**
- 'a

**Fields:**
- `status: &'a self::status::AtomicStatus`

**Trait Implementations:**

- **Drop**
  - `fn drop(self: & mut Self)`



## spin::once::Once

*Struct*

A primitive that provides lazy one-time initialization.

Unlike its `std::sync` equivalent, this is generalized such that the closure returns a
value to be stored by the [`Once`] (`std::sync::Once` can be trivially emulated with
`Once`).

Because [`Once::new`] is `const`, this primitive may be used to safely initialize statics.

# Examples

```
use spin;

static START: spin::Once = spin::Once::new();

START.call_once(|| {
    // run initialization here
});
```

**Generic Parameters:**
- T
- R

**Fields:**
- `phantom: core::marker::PhantomData<R>`
- `status: self::status::AtomicStatus`
- `data: core::cell::UnsafeCell<core::mem::MaybeUninit<T>>`

**Methods:**

- `fn new() -> Self` - Creates a new [`Once`].
- `fn initialized(data: T) -> Self` - Creates a new initialized [`Once`].
- `fn as_mut_ptr(self: &Self) -> *mut T` - Retrieve a pointer to the inner data.
- `fn force_get(self: &Self) -> &T` - Get a reference to the initialized instance. Must only be called once COMPLETE.
- `fn force_get_mut(self: & mut Self) -> & mut T` - Get a reference to the initialized instance. Must only be called once COMPLETE.
- `fn force_into_inner(self: Self) -> T` - Get a reference to the initialized instance. Must only be called once COMPLETE.
- `fn get(self: &Self) -> Option<&T>` - Returns a reference to the inner value if the [`Once`] has been initialized.
- `fn get_unchecked(self: &Self) -> &T` - Returns a reference to the inner value on the unchecked assumption that the  [`Once`] has been initialized.
- `fn get_mut(self: & mut Self) -> Option<& mut T>` - Returns a mutable reference to the inner value if the [`Once`] has been initialized.
- `fn get_mut_unchecked(self: & mut Self) -> & mut T` - Returns a mutable reference to the inner value
- `fn try_into_inner(self: Self) -> Option<T>` - Returns a the inner value if the [`Once`] has been initialized.
- `fn into_inner_unchecked(self: Self) -> T` - Returns a the inner value if the [`Once`] has been initialized.
- `fn is_completed(self: &Self) -> bool` - Checks whether the value has been initialized.
- `fn call_once<F>(self: &Self, f: F) -> &T` - Performs an initialization routine once and only once. The given closure
- `fn try_call_once<F, E>(self: &Self, f: F) -> Result<&T, E>` - This method is similar to `call_once`, but allows the given closure to
- `fn try_call_once_slow<F, E>(self: &Self, f: F) -> Result<&T, E>`
- `fn wait(self: &Self) -> &T` - Spins until the [`Once`] contains a value.
- `fn poll(self: &Self) -> Option<&T>` - Like [`Once::get`], but will spin if the [`Once`] is in the process of being

**Traits:** Send, Sync

**Trait Implementations:**

- **Drop**
  - `fn drop(self: & mut Self)`
- **From**
  - `fn from(data: T) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Default**
  - `fn default() -> Self`



## Module: status



