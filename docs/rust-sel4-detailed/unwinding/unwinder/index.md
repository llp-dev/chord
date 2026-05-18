*[unwinding](../index.md) / [unwinder](index.md)*

---

# Module `unwinder`

## Contents

- [Modules](#modules)
  - [`arch`](#arch)
  - [`find_fde`](#find-fde)
  - [`frame`](#frame)
  - [`custom_eh_frame_finder`](#custom-eh-frame-finder)
- [Structs](#structs)
  - [`UnwindException`](#unwindexception)
  - [`UnwindContext`](#unwindcontext)
- [Functions](#functions)
  - [`with_context`](#with-context)
  - [`_Unwind_GetGR`](#unwind-getgr)
  - [`_Unwind_GetCFA`](#unwind-getcfa)
  - [`_Unwind_SetGR`](#unwind-setgr)
  - [`_Unwind_GetIP`](#unwind-getip)
  - [`_Unwind_GetIPInfo`](#unwind-getipinfo)
  - [`_Unwind_SetIP`](#unwind-setip)
  - [`_Unwind_GetLanguageSpecificData`](#unwind-getlanguagespecificdata)
  - [`_Unwind_GetRegionStart`](#unwind-getregionstart)
  - [`_Unwind_GetTextRelBase`](#unwind-gettextrelbase)
  - [`_Unwind_GetDataRelBase`](#unwind-getdatarelbase)
  - [`_Unwind_FindEnclosingFunction`](#unwind-findenclosingfunction)
  - [`_Unwind_RaiseException`](#unwind-raiseexception)
  - [`raise_exception_phase2`](#raise-exception-phase2)
  - [`_Unwind_ForcedUnwind`](#unwind-forcedunwind)
  - [`force_unwind_phase2`](#force-unwind-phase2)
  - [`_Unwind_Resume`](#unwind-resume)
  - [`_Unwind_Resume_or_Rethrow`](#unwind-resume-or-rethrow)
  - [`_Unwind_DeleteException`](#unwind-deleteexception)
  - [`_Unwind_Backtrace`](#unwind-backtrace)
- [Macros](#macros)
  - [`try1!`](#try1)
  - [`try2!`](#try2)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`arch`](#arch) | mod |  |
| [`find_fde`](#find-fde) | mod |  |
| [`frame`](#frame) | mod |  |
| [`custom_eh_frame_finder`](#custom-eh-frame-finder) | mod |  |
| [`UnwindException`](#unwindexception) | struct |  |
| [`UnwindContext`](#unwindcontext) | struct |  |
| [`with_context`](#with-context) | fn |  |
| [`_Unwind_GetGR`](#unwind-getgr) | fn |  |
| [`_Unwind_GetCFA`](#unwind-getcfa) | fn |  |
| [`_Unwind_SetGR`](#unwind-setgr) | fn |  |
| [`_Unwind_GetIP`](#unwind-getip) | fn |  |
| [`_Unwind_GetIPInfo`](#unwind-getipinfo) | fn |  |
| [`_Unwind_SetIP`](#unwind-setip) | fn |  |
| [`_Unwind_GetLanguageSpecificData`](#unwind-getlanguagespecificdata) | fn |  |
| [`_Unwind_GetRegionStart`](#unwind-getregionstart) | fn |  |
| [`_Unwind_GetTextRelBase`](#unwind-gettextrelbase) | fn |  |
| [`_Unwind_GetDataRelBase`](#unwind-getdatarelbase) | fn |  |
| [`_Unwind_FindEnclosingFunction`](#unwind-findenclosingfunction) | fn |  |
| [`_Unwind_RaiseException`](#unwind-raiseexception) | fn |  |
| [`raise_exception_phase2`](#raise-exception-phase2) | fn |  |
| [`_Unwind_ForcedUnwind`](#unwind-forcedunwind) | fn |  |
| [`force_unwind_phase2`](#force-unwind-phase2) | fn |  |
| [`_Unwind_Resume`](#unwind-resume) | fn |  |
| [`_Unwind_Resume_or_Rethrow`](#unwind-resume-or-rethrow) | fn |  |
| [`_Unwind_DeleteException`](#unwind-deleteexception) | fn |  |
| [`_Unwind_Backtrace`](#unwind-backtrace) | fn |  |
| [`try1!`](#try1) | macro |  |
| [`try2!`](#try2) | macro |  |

## Modules

- [`arch`](arch/index.md)
- [`find_fde`](find_fde/index.md)
- [`frame`](frame/index.md)
- [`custom_eh_frame_finder`](custom_eh_frame_finder/index.md)

## Structs

### `UnwindException`

```rust
struct UnwindException {
    pub exception_class: u64,
    pub exception_cleanup: Option<UnwindExceptionCleanupFn>,
    private_1: Option<UnwindStopFn>,
    private_2: usize,
    private_unused: [usize; 4],
}
```

### `UnwindContext<'a>`

```rust
struct UnwindContext<'a> {
    frame: Option<&'a frame::Frame>,
    ctx: &'a mut Context,
    signal: bool,
}
```

## Functions

### `with_context`

```rust
fn with_context<T, F: FnOnce(&mut Context) -> T>(f: F) -> T
```

### `_Unwind_GetGR`

```rust
fn _Unwind_GetGR(unwind_ctx: &UnwindContext<'_>, index: i32) -> usize
```

### `_Unwind_GetCFA`

```rust
fn _Unwind_GetCFA(unwind_ctx: &UnwindContext<'_>) -> usize
```

### `_Unwind_SetGR`

```rust
fn _Unwind_SetGR(unwind_ctx: &mut UnwindContext<'_>, index: i32, value: usize)
```

### `_Unwind_GetIP`

```rust
fn _Unwind_GetIP(unwind_ctx: &UnwindContext<'_>) -> usize
```

### `_Unwind_GetIPInfo`

```rust
fn _Unwind_GetIPInfo(unwind_ctx: &UnwindContext<'_>, ip_before_insn: &mut i32) -> usize
```

### `_Unwind_SetIP`

```rust
fn _Unwind_SetIP(unwind_ctx: &mut UnwindContext<'_>, value: usize)
```

### `_Unwind_GetLanguageSpecificData`

```rust
fn _Unwind_GetLanguageSpecificData(unwind_ctx: &UnwindContext<'_>) -> *mut core::ffi::c_void
```

### `_Unwind_GetRegionStart`

```rust
fn _Unwind_GetRegionStart(unwind_ctx: &UnwindContext<'_>) -> usize
```

### `_Unwind_GetTextRelBase`

```rust
fn _Unwind_GetTextRelBase(unwind_ctx: &UnwindContext<'_>) -> usize
```

### `_Unwind_GetDataRelBase`

```rust
fn _Unwind_GetDataRelBase(unwind_ctx: &UnwindContext<'_>) -> usize
```

### `_Unwind_FindEnclosingFunction`

```rust
fn _Unwind_FindEnclosingFunction(pc: *mut core::ffi::c_void) -> *mut core::ffi::c_void
```

### `_Unwind_RaiseException`

```rust
unsafe fn _Unwind_RaiseException(exception: *mut UnwindException) -> UnwindReasonCode
```

### `raise_exception_phase2`

```rust
fn raise_exception_phase2(exception: *mut UnwindException, ctx: &mut Context, handler_cfa: usize) -> UnwindReasonCode
```

### `_Unwind_ForcedUnwind`

```rust
unsafe fn _Unwind_ForcedUnwind(exception: *mut UnwindException, stop: UnwindStopFn, stop_arg: *mut core::ffi::c_void) -> UnwindReasonCode
```

### `force_unwind_phase2`

```rust
fn force_unwind_phase2(exception: *mut UnwindException, ctx: &mut Context, stop: UnwindStopFn, stop_arg: *mut core::ffi::c_void) -> UnwindReasonCode
```

### `_Unwind_Resume`

```rust
unsafe fn _Unwind_Resume(exception: *mut UnwindException) -> never
```

### `_Unwind_Resume_or_Rethrow`

```rust
unsafe fn _Unwind_Resume_or_Rethrow(exception: *mut UnwindException) -> UnwindReasonCode
```

### `_Unwind_DeleteException`

```rust
unsafe fn _Unwind_DeleteException(exception: *mut UnwindException)
```

### `_Unwind_Backtrace`

```rust
fn _Unwind_Backtrace(trace: UnwindTraceFn, trace_argument: *mut core::ffi::c_void) -> UnwindReasonCode
```

## Macros

### `try1!`

### `try2!`

