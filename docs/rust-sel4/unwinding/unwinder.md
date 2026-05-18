**unwinding > unwinder**

# Module: unwinder

## Contents

**Modules**

- [`arch`](#arch)
- [`find_fde`](#find_fde)
- [`frame`](#frame)

**Macros**

- [`try1`](#try1)
- [`try2`](#try2)

**Structs**

- [`UnwindContext`](#unwindcontext)
- [`UnwindException`](#unwindexception)

**Functions**

- [`_Unwind_Backtrace`](#_unwind_backtrace)
- [`_Unwind_DeleteException`](#_unwind_deleteexception)
- [`_Unwind_FindEnclosingFunction`](#_unwind_findenclosingfunction)
- [`_Unwind_ForcedUnwind`](#_unwind_forcedunwind)
- [`_Unwind_GetCFA`](#_unwind_getcfa)
- [`_Unwind_GetDataRelBase`](#_unwind_getdatarelbase)
- [`_Unwind_GetGR`](#_unwind_getgr)
- [`_Unwind_GetIP`](#_unwind_getip)
- [`_Unwind_GetIPInfo`](#_unwind_getipinfo)
- [`_Unwind_GetLanguageSpecificData`](#_unwind_getlanguagespecificdata)
- [`_Unwind_GetRegionStart`](#_unwind_getregionstart)
- [`_Unwind_GetTextRelBase`](#_unwind_gettextrelbase)
- [`_Unwind_RaiseException`](#_unwind_raiseexception)
- [`_Unwind_Resume`](#_unwind_resume)
- [`_Unwind_Resume_or_Rethrow`](#_unwind_resume_or_rethrow)
- [`_Unwind_SetGR`](#_unwind_setgr)
- [`_Unwind_SetIP`](#_unwind_setip)
- [`force_unwind_phase2`](#force_unwind_phase2)
- [`raise_exception_phase2`](#raise_exception_phase2)
- [`with_context`](#with_context)

---

## unwinding::unwinder::UnwindContext

*Struct*

**Generic Parameters:**
- 'a

**Fields:**
- `frame: Option<&'a frame::Frame>`
- `ctx: &'a  mut Context`
- `signal: bool`



## unwinding::unwinder::UnwindException

*Struct*

**Fields:**
- `exception_class: u64`
- `exception_cleanup: Option<UnwindExceptionCleanupFn>`
- `private_1: Option<UnwindStopFn>`
- `private_2: usize`
- `private_unused: [usize; 4]`



## unwinding::unwinder::_Unwind_Backtrace

*Function*

```rust
fn _Unwind_Backtrace(trace: UnwindTraceFn, trace_argument: *mut core::ffi::c_void) -> UnwindReasonCode
```



## unwinding::unwinder::_Unwind_DeleteException

*Function*

```rust
fn _Unwind_DeleteException(exception: *mut UnwindException)
```



## unwinding::unwinder::_Unwind_FindEnclosingFunction

*Function*

```rust
fn _Unwind_FindEnclosingFunction(pc: *mut core::ffi::c_void) -> *mut core::ffi::c_void
```



## unwinding::unwinder::_Unwind_ForcedUnwind

*Function*

```rust
fn _Unwind_ForcedUnwind(exception: *mut UnwindException, stop: UnwindStopFn, stop_arg: *mut core::ffi::c_void) -> UnwindReasonCode
```



## unwinding::unwinder::_Unwind_GetCFA

*Function*

```rust
fn _Unwind_GetCFA(unwind_ctx: &UnwindContext) -> usize
```



## unwinding::unwinder::_Unwind_GetDataRelBase

*Function*

```rust
fn _Unwind_GetDataRelBase(unwind_ctx: &UnwindContext) -> usize
```



## unwinding::unwinder::_Unwind_GetGR

*Function*

```rust
fn _Unwind_GetGR(unwind_ctx: &UnwindContext, index: i32) -> usize
```



## unwinding::unwinder::_Unwind_GetIP

*Function*

```rust
fn _Unwind_GetIP(unwind_ctx: &UnwindContext) -> usize
```



## unwinding::unwinder::_Unwind_GetIPInfo

*Function*

```rust
fn _Unwind_GetIPInfo(unwind_ctx: &UnwindContext, ip_before_insn: & mut i32) -> usize
```



## unwinding::unwinder::_Unwind_GetLanguageSpecificData

*Function*

```rust
fn _Unwind_GetLanguageSpecificData(unwind_ctx: &UnwindContext) -> *mut core::ffi::c_void
```



## unwinding::unwinder::_Unwind_GetRegionStart

*Function*

```rust
fn _Unwind_GetRegionStart(unwind_ctx: &UnwindContext) -> usize
```



## unwinding::unwinder::_Unwind_GetTextRelBase

*Function*

```rust
fn _Unwind_GetTextRelBase(unwind_ctx: &UnwindContext) -> usize
```



## unwinding::unwinder::_Unwind_RaiseException

*Function*

```rust
fn _Unwind_RaiseException(exception: *mut UnwindException) -> UnwindReasonCode
```



## unwinding::unwinder::_Unwind_Resume

*Function*

```rust
fn _Unwind_Resume(exception: *mut UnwindException) -> never
```



## unwinding::unwinder::_Unwind_Resume_or_Rethrow

*Function*

```rust
fn _Unwind_Resume_or_Rethrow(exception: *mut UnwindException) -> UnwindReasonCode
```



## unwinding::unwinder::_Unwind_SetGR

*Function*

```rust
fn _Unwind_SetGR(unwind_ctx: & mut UnwindContext, index: i32, value: usize)
```



## unwinding::unwinder::_Unwind_SetIP

*Function*

```rust
fn _Unwind_SetIP(unwind_ctx: & mut UnwindContext, value: usize)
```



## Module: arch



## Module: find_fde



## unwinding::unwinder::force_unwind_phase2

*Function*

```rust
fn force_unwind_phase2(exception: *mut UnwindException, ctx: & mut Context, stop: UnwindStopFn, stop_arg: *mut core::ffi::c_void) -> UnwindReasonCode
```



## Module: frame



## unwinding::unwinder::raise_exception_phase2

*Function*

```rust
fn raise_exception_phase2(exception: *mut UnwindException, ctx: & mut Context, handler_cfa: usize) -> UnwindReasonCode
```



## unwinding::unwinder::try1

*Declarative Macro*

```rust
macro_rules! try1 {
    ($e: expr) => { ... };
}
```



## unwinding::unwinder::try2

*Declarative Macro*

```rust
macro_rules! try2 {
    ($e: expr) => { ... };
}
```



## unwinding::unwinder::with_context

*Function*

```rust
fn with_context<T, F>(f: F) -> T
```



