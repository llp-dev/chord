**cfg_if**

# Module: cfg_if

## Contents

**Macros**

- [`cfg_if`](#cfg_if) - The main macro provided by this crate. See crate documentation for more

---

## cfg_if::cfg_if

*Declarative Macro*

The main macro provided by this crate. See crate documentation for more
information.

```rust
macro_rules! cfg_if {
    (
        if #[cfg( $($i_meta:tt)+ )] { $( $i_tokens:tt )* }
        $(
            else if #[cfg( $($ei_meta:tt)+ )] { $( $ei_tokens:tt )* }
        )*
        $(
            else { $( $e_tokens:tt )* }
        )?
    ) => { ... };
    (@__items ( $( ($($_:tt)*) , )* ) ; ) => { ... };
    (
        @__items ( $( ($($no:tt)+) , )* ) ;
        (( $( $($yes:tt)+ )? ) ( $( $tokens:tt )* )),
        $( $rest:tt , )*
    ) => { ... };
    (@__temp_group $( $tokens:tt )* ) => { ... };
}
```



