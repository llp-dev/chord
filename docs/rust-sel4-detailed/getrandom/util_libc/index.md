*[getrandom](../index.md) / [util_libc](index.md)*

---

# Module `util_libc`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Weak`](#weak) | struct |  |
| [`get_errno`](#get-errno) | fn |  |
| [`last_os_error`](#last-os-error) | fn |  |
| [`sys_fill_exact`](#sys-fill-exact) | fn |  |
| [`open_readonly`](#open-readonly) | fn |  |
| [`getrandom_syscall`](#getrandom-syscall) | fn | Thin wrapper around the `getrandom()` Linux system call |

## Structs

### `Weak`

```rust
struct Weak {
    name: &'static str,
    addr: core::sync::atomic::AtomicPtr<libc::c_void>,
}
```

#### Implementations

- <span id="weak-const-uninit"></span>`const UNINIT: *mut c_void`

- <span id="weak-new"></span>`const unsafe fn new(name: &'static str) -> Self`

- <span id="weak-ptr"></span>`fn ptr(&self) -> Option<NonNull<c_void>>`

## Functions

### `get_errno`

```rust
unsafe fn get_errno() -> libc::c_int
```

### `last_os_error`

```rust
fn last_os_error() -> crate::Error
```

### `sys_fill_exact`

```rust
fn sys_fill_exact(buf: &mut [core::mem::MaybeUninit<u8>], sys_fill: impl Fn(&mut [core::mem::MaybeUninit<u8>]) -> libc::ssize_t) -> Result<(), crate::Error>
```

### `open_readonly`

```rust
unsafe fn open_readonly(path: &str) -> Result<libc::c_int, crate::Error>
```

### `getrandom_syscall`

```rust
fn getrandom_syscall(buf: &mut [core::mem::MaybeUninit<u8>]) -> libc::ssize_t
```

Thin wrapper around the `getrandom()` Linux system call

