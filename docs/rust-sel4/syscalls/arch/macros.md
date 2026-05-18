**syscalls > arch > macros**

# Module: arch::macros

## Contents

**Macros**

- [`syscall_enum`](#syscall_enum) - Helper for generating support code for a list of syscalls.

---

## syscalls::arch::macros::syscall_enum

*Declarative Macro*

Helper for generating support code for a list of syscalls.

```rust
macro_rules! syscall_enum {
    (
        $(#[$outer:meta])*
        $vis:vis enum $Name:ident {
            $(#[$first_inner:meta])*
            $first_syscall:ident = $first_num:expr,
            $(
                $(#[$inner:meta])*
                $syscall:ident = $num:expr,
            )*
        }

        LAST: $last_syscall:ident;
    ) => { ... };
}
```



