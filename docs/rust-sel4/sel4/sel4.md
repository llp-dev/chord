**sel4**

# Module: sel4

## Contents

**Modules**

- [`init_thread`](#init_thread) - Items that are applicable within the context of the root task's initial thread's CSpace.

**Macros**

- [`debug_print`](#debug_print) - Like `std::debug_print!`, except backed by `seL4_DebugPutChar`.
- [`debug_println`](#debug_println) - Like `std::debug_println!`, except backed by `seL4_DebugPutChar`.

**Constants**

- [`WORD_SIZE`](#word_size) - The size of [`Word`] in bits.

**Type Aliases**

- [`Word`](#word) - Corresponds to `seL4_Word`.

---

## sel4::WORD_SIZE

*Constant*: `usize`

The size of [`Word`] in bits.



## sel4::Word

*Type Alias*: `sys::seL4_Word`

Corresponds to `seL4_Word`.



## sel4::debug_print

*Declarative Macro*

Like `std::debug_print!`, except backed by `seL4_DebugPutChar`.

```rust
macro_rules! debug_print {
    ($($arg:tt)*) => { ... };
}
```



## sel4::debug_println

*Declarative Macro*

Like `std::debug_println!`, except backed by `seL4_DebugPutChar`.

```rust
macro_rules! debug_println {
    () => { ... };
    ($($arg:tt)*) => { ... };
}
```



## Module: init_thread

Items that are applicable within the context of the root task's initial thread's CSpace.



