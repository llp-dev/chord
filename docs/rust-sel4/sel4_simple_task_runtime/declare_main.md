**sel4_simple_task_runtime > declare_main**

# Module: declare_main

## Contents

**Modules**

- [`_private`](#_private)

**Functions**

- [`run_main`](#run_main)
- [`run_main_json`](#run_main_json)
- [`run_main_postcard`](#run_main_postcard)
- [`wrap`](#wrap)

---

## Module: _private



## sel4_simple_task_runtime::declare_main::run_main

*Function*

```rust
fn run_main<T, impl Fn(&[u8]) -> T>(f: impl Trait, arg: &[u8])
```



## sel4_simple_task_runtime::declare_main::run_main_json

*Function*

```rust
fn run_main_json<T, U, impl Fn(U) -> T>(f: impl Trait, arg: &[u8])
```



## sel4_simple_task_runtime::declare_main::run_main_postcard

*Function*

```rust
fn run_main_postcard<T, U, impl Fn(U) -> T>(f: impl Trait, arg: &[u8])
```



## sel4_simple_task_runtime::declare_main::wrap

*Function*

```rust
fn wrap<impl FnOnce() + UnwindSafe>(f: impl Trait)
```



