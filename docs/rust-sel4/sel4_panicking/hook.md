**sel4_panicking > hook**

# Module: hook

## Contents

**Functions**

- [`set_hook`](#set_hook) - Like `std::panic::set_hook`.

**Type Aliases**

- [`PanicHook`](#panichook) - Type for panic hooks.

---

## sel4_panicking::hook::PanicHook

*Type Alias*: `&'static dyn Fn`

Type for panic hooks.

See [`set_hook`].



## sel4_panicking::hook::set_hook

*Function*

Like `std::panic::set_hook`.

```rust
fn set_hook(hook: PanicHook)
```



