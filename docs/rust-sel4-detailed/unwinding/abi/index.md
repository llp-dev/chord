*[unwinding](../index.md) / [abi](index.md)*

---

# Module `abi`

## Contents

- [Modules](#modules)
  - [`arch`](#arch)
  - [`find_fde`](#find-fde)
  - [`frame`](#frame)
- [Structs](#structs)
  - [`UnwindReasonCode`](#unwindreasoncode)
  - [`UnwindAction`](#unwindaction)
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
- [Type Aliases](#type-aliases)
  - [`UnwindExceptionCleanupFn`](#unwindexceptioncleanupfn)
  - [`UnwindStopFn`](#unwindstopfn)
  - [`UnwindTraceFn`](#unwindtracefn)
  - [`PersonalityRoutine`](#personalityroutine)
- [Macros](#macros)
  - [`binding!`](#binding)
  - [`try1!`](#try1)
  - [`try2!`](#try2)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`arch`](#arch) | mod |  |
| [`find_fde`](#find-fde) | mod |  |
| [`frame`](#frame) | mod |  |
| [`UnwindReasonCode`](#unwindreasoncode) | struct |  |
| [`UnwindAction`](#unwindaction) | struct |  |
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
| [`UnwindExceptionCleanupFn`](#unwindexceptioncleanupfn) | type |  |
| [`UnwindStopFn`](#unwindstopfn) | type |  |
| [`UnwindTraceFn`](#unwindtracefn) | type |  |
| [`PersonalityRoutine`](#personalityroutine) | type |  |
| [`binding!`](#binding) | macro |  |
| [`try1!`](#try1) | macro |  |
| [`try2!`](#try2) | macro |  |

## Modules

- [`arch`](arch/index.md)
- [`find_fde`](find_fde/index.md)
- [`frame`](frame/index.md)

## Structs

### `UnwindReasonCode`

```rust
struct UnwindReasonCode(i32);
```

#### Implementations

- <span id="unwindreasoncode-const-no-reason"></span>`const NO_REASON: Self`

- <span id="unwindreasoncode-const-foreign-exception-caught"></span>`const FOREIGN_EXCEPTION_CAUGHT: Self`

- <span id="unwindreasoncode-const-fatal-phase2-error"></span>`const FATAL_PHASE2_ERROR: Self`

- <span id="unwindreasoncode-const-fatal-phase1-error"></span>`const FATAL_PHASE1_ERROR: Self`

- <span id="unwindreasoncode-const-normal-stop"></span>`const NORMAL_STOP: Self`

- <span id="unwindreasoncode-const-end-of-stack"></span>`const END_OF_STACK: Self`

- <span id="unwindreasoncode-const-handler-found"></span>`const HANDLER_FOUND: Self`

- <span id="unwindreasoncode-const-install-context"></span>`const INSTALL_CONTEXT: Self`

- <span id="unwindreasoncode-const-continue-unwind"></span>`const CONTINUE_UNWIND: Self`

#### Trait Implementations

##### `impl Clone for UnwindReasonCode`

- <span id="unwindreasoncode-clone"></span>`fn clone(&self) -> UnwindReasonCode` — [`UnwindReasonCode`](#unwindreasoncode)

##### `impl Copy for UnwindReasonCode`

##### `impl Eq for UnwindReasonCode`

##### `impl PartialEq for UnwindReasonCode`

- <span id="unwindreasoncode-partialeq-eq"></span>`fn eq(&self, other: &UnwindReasonCode) -> bool` — [`UnwindReasonCode`](#unwindreasoncode)

##### `impl StructuralPartialEq for UnwindReasonCode`

### `UnwindAction`

```rust
struct UnwindAction(i32);
```

#### Implementations

- <span id="unwindaction-const-search-phase"></span>`const SEARCH_PHASE: Self`

- <span id="unwindaction-const-cleanup-phase"></span>`const CLEANUP_PHASE: Self`

- <span id="unwindaction-const-handler-frame"></span>`const HANDLER_FRAME: Self`

- <span id="unwindaction-const-force-unwind"></span>`const FORCE_UNWIND: Self`

- <span id="unwindaction-const-end-of-stack"></span>`const END_OF_STACK: Self`

#### Trait Implementations

##### `impl BitOr for UnwindAction`

- <span id="unwindaction-bitor-type-output"></span>`type Output = UnwindAction`

- <span id="unwindaction-bitor"></span>`fn bitor(self, rhs: Self) -> Self`

##### `impl Clone for UnwindAction`

- <span id="unwindaction-clone"></span>`fn clone(&self) -> UnwindAction` — [`UnwindAction`](#unwindaction)

##### `impl Copy for UnwindAction`

##### `impl Eq for UnwindAction`

##### `impl PartialEq for UnwindAction`

- <span id="unwindaction-partialeq-eq"></span>`fn eq(&self, other: &UnwindAction) -> bool` — [`UnwindAction`](#unwindaction)

##### `impl StructuralPartialEq for UnwindAction`

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

## Type Aliases

### `UnwindExceptionCleanupFn`

```rust
type UnwindExceptionCleanupFn = fn(UnwindReasonCode, *mut UnwindException);
```

### `UnwindStopFn`

```rust
type UnwindStopFn = fn(i32, UnwindAction, u64, *mut UnwindException, &mut UnwindContext<'_>, *mut core::ffi::c_void) -> UnwindReasonCode;
```

### `UnwindTraceFn`

```rust
type UnwindTraceFn = fn(&UnwindContext<'_>, *mut core::ffi::c_void) -> UnwindReasonCode;
```

### `PersonalityRoutine`

```rust
type PersonalityRoutine = fn(i32, UnwindAction, u64, *mut UnwindException, &mut UnwindContext<'_>) -> UnwindReasonCode;
```

## Macros

### `binding!`

### `try1!`

### `try2!`

