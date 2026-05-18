**syscalls > errno > macros**

# Module: errno::macros

## Contents

**Macros**

- [`errno_enum`](#errno_enum)

---

## syscalls::errno::macros::errno_enum

*Declarative Macro*

```rust
macro_rules! errno_enum {
    (
        $(#[$meta:meta])*
        $vis:vis enum $Name:ident {
            $(
                $(#[$attrs:meta])*
                $item:ident($code:expr) = $doc:expr,
            )*
        }
    ) => { ... };
}
```



