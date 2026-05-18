*[getrandom](../index.md) / [use_file](index.md)*

---

# Module `use_file`

Implementations that just need to read from a file

## Contents

- [Structs](#structs)
  - [`Mutex`](#mutex)
  - [`DropGuard`](#dropguard)
- [Functions](#functions)
  - [`getrandom_inner`](#getrandom-inner)
  - [`get_rng_fd`](#get-rng-fd)
  - [`wait_until_rng_ready`](#wait-until-rng-ready)
- [Constants](#constants)
  - [`FILE_PATH`](#file-path)
  - [`FD_UNINIT`](#fd-uninit)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Mutex`](#mutex) | struct |  |
| [`DropGuard`](#dropguard) | struct |  |
| [`getrandom_inner`](#getrandom-inner) | fn |  |
| [`get_rng_fd`](#get-rng-fd) | fn |  |
| [`wait_until_rng_ready`](#wait-until-rng-ready) | fn |  |
| [`FILE_PATH`](#file-path) | const | For all platforms, we use `/dev/urandom` rather than `/dev/random`. |
| [`FD_UNINIT`](#fd-uninit) | const |  |

## Structs

### `Mutex`

```rust
struct Mutex(core::cell::UnsafeCell<libc::pthread_mutex_t>);
```

#### Implementations

- <span id="mutex-new"></span>`const fn new() -> Self`

- <span id="mutex-lock"></span>`unsafe fn lock(&self)`

- <span id="mutex-unlock"></span>`unsafe fn unlock(&self)`

#### Trait Implementations

##### `impl Sync for Mutex`

### `DropGuard<F: FnMut()>`

```rust
struct DropGuard<F: FnMut()>(F);
```

#### Trait Implementations

##### `impl<F: FnMut()> Drop for DropGuard<F>`

- <span id="dropguard-drop"></span>`fn drop(&mut self)`

## Functions

### `getrandom_inner`

```rust
fn getrandom_inner(dest: &mut [core::mem::MaybeUninit<u8>]) -> Result<(), crate::Error>
```

### `get_rng_fd`

```rust
fn get_rng_fd() -> Result<libc::c_int, crate::Error>
```

### `wait_until_rng_ready`

```rust
fn wait_until_rng_ready() -> Result<(), crate::Error>
```

## Constants

### `FILE_PATH`
```rust
const FILE_PATH: &str;
```

For all platforms, we use `/dev/urandom` rather than `/dev/random`.
For more information see the linked man pages in lib.rs.
  - On Linux, "/dev/urandom is preferred and sufficient in all use cases".
  - On Redox, only /dev/urandom is provided.
  - On AIX, /dev/urandom will "provide cryptographically secure output".
  - On Haiku and QNX Neutrino they are identical.

### `FD_UNINIT`
```rust
const FD_UNINIT: usize = 18_446_744_073_709_551_615usize;
```

