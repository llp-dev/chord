**libc > new > common > linux_like > pthread**

# Module: new::common::linux_like::pthread

## Contents

**Functions**

- [`pthread_getaffinity_np`](#pthread_getaffinity_np)
- [`pthread_getattr_np`](#pthread_getattr_np)
- [`pthread_getname_np`](#pthread_getname_np)
- [`pthread_setaffinity_np`](#pthread_setaffinity_np)
- [`pthread_setname_np`](#pthread_setname_np)

---

## libc::new::common::linux_like::pthread::pthread_getaffinity_np

*Function*

```rust
fn pthread_getaffinity_np(thread: crate::pthread_t, cpusetsize: size_t, cpuset: *mut crate::cpu_set_t) -> c_int
```



## libc::new::common::linux_like::pthread::pthread_getattr_np

*Function*

```rust
fn pthread_getattr_np(native: crate::pthread_t, attr: *mut crate::pthread_attr_t) -> c_int
```



## libc::new::common::linux_like::pthread::pthread_getname_np

*Function*

```rust
fn pthread_getname_np(thread: crate::pthread_t, name: *mut c_char, len: size_t) -> c_int
```



## libc::new::common::linux_like::pthread::pthread_setaffinity_np

*Function*

```rust
fn pthread_setaffinity_np(thread: crate::pthread_t, cpusetsize: size_t, cpuset: *const crate::cpu_set_t) -> c_int
```



## libc::new::common::linux_like::pthread::pthread_setname_np

*Function*

```rust
fn pthread_setname_np(thread: crate::pthread_t, name: *const c_char) -> c_int
```



