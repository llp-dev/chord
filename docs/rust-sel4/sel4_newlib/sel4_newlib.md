**sel4_newlib**

# Module: sel4_newlib

## Contents

**Macros**

- [`declare_sbrk_with_static_heap`](#declare_sbrk_with_static_heap)

**Functions**

- [`srand`](#srand)

---

## sel4_newlib::declare_sbrk_with_static_heap

*Declarative Macro*

```rust
macro_rules! declare_sbrk_with_static_heap {
    ($n:expr) => { ... };
}
```



## sel4_newlib::srand

*Function*

```rust
fn srand(seed: core::ffi::c_uint)
```



