**sel4_runtime_common**

# Module: sel4_runtime_common

## Contents

**Macros**

- [`declare_entrypoint`](#declare_entrypoint)
- [`declare_entrypoint_with_stack_init`](#declare_entrypoint_with_stack_init)
- [`declare_stack`](#declare_stack)

**Functions**

- [`global_init_complete`](#global_init_complete)
- [`with_local_init`](#with_local_init)

---

## sel4_runtime_common::declare_entrypoint

*Declarative Macro*

```rust
macro_rules! declare_entrypoint {
    {
        $f:ident($( $i:ident: $t:ty ),* $(,)?)
     } => { ... };
    {
        $f:ident($( $i:ident: $t:ty ),* $(,)?)
        global_init if $global_init_cond:expr
     } => { ... };
}
```



## sel4_runtime_common::declare_entrypoint_with_stack_init

*Declarative Macro*

```rust
macro_rules! declare_entrypoint_with_stack_init {
    ($f:ident($( $i:ident: $t:ty ),* $(,)?)) => { ... };
}
```



## sel4_runtime_common::declare_stack

*Declarative Macro*

```rust
macro_rules! declare_stack {
    ($size:expr) => { ... };
}
```



## sel4_runtime_common::global_init_complete

*Function*

```rust
fn global_init_complete() -> bool
```



## sel4_runtime_common::with_local_init

*Function*

```rust
fn with_local_init<impl FnOnce() -> !>(f: impl Trait) -> never
```



