**sel4_alloca**

# Module: sel4_alloca

## Contents

**Functions**

- [`with_alloca`](#with_alloca)
- [`with_alloca_bytes`](#with_alloca_bytes)
- [`with_alloca_ptr`](#with_alloca_ptr)
- [`with_alloca_slice`](#with_alloca_slice)

---

## sel4_alloca::with_alloca

*Function*

```rust
fn with_alloca<R, T, F>(f: F) -> R
```



## sel4_alloca::with_alloca_bytes

*Function*

```rust
fn with_alloca_bytes<R, F>(layout: core::alloc::Layout, f: F) -> R
```



## sel4_alloca::with_alloca_ptr

*Function*

```rust
fn with_alloca_ptr<R, F>(layout: core::alloc::Layout, f: F) -> R
```



## sel4_alloca::with_alloca_slice

*Function*

```rust
fn with_alloca_slice<R, T, F>(n: usize, f: F) -> R
```



