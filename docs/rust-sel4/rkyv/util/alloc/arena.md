**rkyv > util > alloc > arena**

# Module: util::alloc::arena

## Contents

**Functions**

- [`clear_arena`](#clear_arena) - Clears the builtin arena allocator.
- [`with_arena`](#with_arena) - Calls the given function with the builtin arena allocator.

---

## rkyv::util::alloc::arena::clear_arena

*Function*

Clears the builtin arena allocator.

When the `std` feature is enabled, this only clears the allocator for the
current thread. When atomic pointers are supported, this will clear the
allocator for all threads. Otherwise, this function does nothing.

```rust
fn clear_arena()
```



## rkyv::util::alloc::arena::with_arena

*Function*

Calls the given function with the builtin arena allocator.

When the `std` feature is enabled, the builtin arena allocator is a
thread-local variable, with one allocator per thread. When atomic pointers
are supported, it is a global static and all threads share the same arena.
Otherwise, this will create and drop a new arena each time it is called.

```rust
fn with_arena<T, impl FnOnce(&mut Arena) -> T>(f: impl Trait) -> T
```



