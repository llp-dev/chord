**unwinding > abi**

# Module: abi

## Contents

**Macros**

- [`binding`](#binding)

**Structs**

- [`UnwindAction`](#unwindaction)
- [`UnwindReasonCode`](#unwindreasoncode)

**Type Aliases**

- [`PersonalityRoutine`](#personalityroutine)
- [`UnwindExceptionCleanupFn`](#unwindexceptioncleanupfn)
- [`UnwindStopFn`](#unwindstopfn)
- [`UnwindTraceFn`](#unwindtracefn)

---

## unwinding::abi::PersonalityRoutine

*Type Alias*: `fn(...)`



## unwinding::abi::UnwindAction

*Struct*

**Tuple Struct**: `(i32)`

**Methods:**

- `fn empty() -> Self`
- `fn contains(self: &Self, other: Self) -> bool`

**Traits:** Eq, Copy

**Trait Implementations:**

- **BitOr**
  - `fn bitor(self: Self, rhs: Self) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &UnwindAction) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> UnwindAction`



## unwinding::abi::UnwindExceptionCleanupFn

*Type Alias*: `fn(...)`



## unwinding::abi::UnwindReasonCode

*Struct*

**Tuple Struct**: `(i32)`

**Methods:**


**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &UnwindReasonCode) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> UnwindReasonCode`



## unwinding::abi::UnwindStopFn

*Type Alias*: `fn(...)`



## unwinding::abi::UnwindTraceFn

*Type Alias*: `fn(...)`



## unwinding::abi::binding

*Declarative Macro*

```rust
macro_rules! binding {
    () => { ... };
    (unsafe extern $abi: literal fn $name: ident ($($arg: ident : $arg_ty: ty),*$(,)?) $(-> $ret: ty)?; $($rest: tt)*) => { ... };
    (extern $abi: literal fn $name: ident ($($arg: ident : $arg_ty: ty),*$(,)?) $(-> $ret: ty)?; $($rest: tt)*) => { ... };
}
```



