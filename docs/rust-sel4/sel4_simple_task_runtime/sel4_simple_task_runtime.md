**sel4_simple_task_runtime**

# Module: sel4_simple_task_runtime

## Contents

**Macros**

- [`declare_main_with`](#declare_main_with)

**Functions**

- [`get_backtracing`](#get_backtracing)
- [`idle`](#idle)
- [`try_idle`](#try_idle)

---

## sel4_simple_task_runtime::declare_main_with

*Declarative Macro*

```rust
macro_rules! declare_main_with {
    ($f:ident, $main:path) => { ... };
}
```



## sel4_simple_task_runtime::get_backtracing

*Function*

```rust
fn get_backtracing() -> sel4_backtrace_simple::SimpleBacktracing
```



## sel4_simple_task_runtime::idle

*Function*

```rust
fn idle() -> never
```



## sel4_simple_task_runtime::try_idle

*Function*

```rust
fn try_idle()
```



