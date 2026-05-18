**sel4_root_task > heap**

# Module: heap

## Contents

**Modules**

- [`_private`](#_private)

**Functions**

- [`set_global_allocator_mutex_notification`](#set_global_allocator_mutex_notification) - Provides the global allocator with a [`sel4::cap::Notification`] to use as a mutex..

---

## Module: _private



## sel4_root_task::heap::set_global_allocator_mutex_notification

*Function*

Provides the global allocator with a [`sel4::cap::Notification`] to use as a mutex..

Until this function is used, contention in the global allocator will result in a panic. This is
only useful for multi-threaded root tasks.

```rust
fn set_global_allocator_mutex_notification(nfn: sel4::cap::Notification)
```



