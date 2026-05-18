**sel4_config_macros**

# Module: sel4_config_macros

## Contents

**Proc Macros**

- [`sel4_cfg`](#sel4_cfg) - Make the attached code conditional on a seL4 kernel configuration expression.
- [`sel4_cfg_attr`](#sel4_cfg_attr) - Make the associated attribute expression conditional on a seL4 kernel configuration expression.
- [`sel4_cfg_bool`](#sel4_cfg_bool) - Like `core::cfg!`, except using the seL4 kernel configuration.
- [`sel4_cfg_enum`](#sel4_cfg_enum) - Allows an `enum`'s variants to use the [`macro@sel4_cfg`] attribute.
- [`sel4_cfg_if`](#sel4_cfg_if) - Like `cfg_if::cfg_if!`, except with [`macro@sel4_cfg`] instead of `#[cfg]`.
- [`sel4_cfg_match`](#sel4_cfg_match) - Allows a `match` expression's variants to use the [`macro@sel4_cfg`] attribute.
- [`sel4_cfg_str`](#sel4_cfg_str) - Like `core::cfg!`, except using the seL4 kernel configuration.
- [`sel4_cfg_struct`](#sel4_cfg_struct) - Allows a `struct`'s fields to use the [`macro@sel4_cfg`] attribute.
- [`sel4_cfg_usize`](#sel4_cfg_usize) - Like `core::cfg!`, except using the seL4 kernel configuration.
- [`sel4_cfg_word`](#sel4_cfg_word) - Like `core::cfg!`, except using the seL4 kernel configuration.
- [`sel4_cfg_wrap_match`](#sel4_cfg_wrap_match) - Like [`macro@sel4_cfg_match`], except it works on stable, at the expense of not being an

---

## sel4_config_macros::sel4_cfg

*Attribute Macro*

Make the attached code conditional on a seL4 kernel configuration expression.

Supports the same syntax as `#[cfg]`, except primitive expressions are based on seL4 kernel
configuration rather than `rustc` configuration.

Suppose `$SEL4_PREFIX/libsel4/include/kernel/gen_config.json` contains:

```ignore
{
    "KERNEL_MCS": false,
    "HAVE_FPU": true
    "ARM_PLAT": "bcm2711",
    "MAX_NUM_NODES": "4",
    ...
}
```

Note that values are either booleans or strings. Configuration keys corresponding to boolean
values are used as in `#[sel4_cfg(KERNEL_MCS)]`, whereas those corresponding to strings are used
in equalities as in `#[sel4_cfg(MAX_NUM_NODES = 4)]`. Expressions can be combined using `not()`,
`any()`, and `all()`, just like expressions in `#[cfg]`.

Unlike in `#[cfg]`, using a configuration key that is not present in the seL4 kernel
configuration will result in an error rather than just evaluating to false. That is, a key that
is not part of an equality expression evaluates to its boolean value, whereas in `#[cfg]` a key
that is not part of an equality expression evaluates to whether it is present in the
configuration.

```rust
#[sel4_cfg]
```



## sel4_config_macros::sel4_cfg_attr

*Attribute Macro*

Make the associated attribute expression conditional on a seL4 kernel configuration expression.

Supports the same syntax as `#[cfg_attr]`, except primitive expressions are based on seL4 kernel
configuration rather than `rustc` configuration.

See [`macro@sel4_cfg`].

```rust
#[sel4_cfg_attr]
```



## sel4_config_macros::sel4_cfg_bool

*Function-like Macro*

Like `core::cfg!`, except using the seL4 kernel configuration.

Unlike `core::cfg!`, this macro requires the configuration key to correspond to a boolean
value.

See [`macro@sel4_cfg`] for documentation on the configuration expression syntax.

# Example

```ignore
if sel4_cfg_bool!(KERNEL_MCS) {
    ...
}
```

```rust
sel4_cfg_bool!(...)
```



## sel4_config_macros::sel4_cfg_enum

*Attribute Macro*

Allows an `enum`'s variants to use the [`macro@sel4_cfg`] attribute.

# Example

```ignore
#[sel4_cfg_enum]
enum Foo {
    #[sel4_cfg(KERNEL_MCS)]
    Bar,
}
```

```rust
#[sel4_cfg_enum]
```



## sel4_config_macros::sel4_cfg_if

*Function-like Macro*

Like `cfg_if::cfg_if!`, except with [`macro@sel4_cfg`] instead of `#[cfg]`.

# Example

```ignore
sel4_cfg_if! {
    if #[sel4_cfg(KERNEL_MCS)] {
        ...
    } else if #[sel4_cfg(MAX_NUM_NODES = "1")] {
        ...
    } else {
        ...
    }
}
```

```rust
sel4_cfg_if!(...)
```



## sel4_config_macros::sel4_cfg_match

*Attribute Macro*

Allows a `match` expression's variants to use the [`macro@sel4_cfg`] attribute.

Using this attribute macro currently requires nightly (`#![feature(proc_macro_hygiene)]` and
`#![feature(stmt_expr_attributes)]`). The less elegant [`macro@sel4_cfg_wrap_match`] serves the
same purpose and works on stable.

# Example

```ignore
#[sel4_cfg_match]
match foo {
    #[sel4_cfg(KERNEL_MCS)]
    1337 => bar,
}
```

```rust
#[sel4_cfg_match]
```



## sel4_config_macros::sel4_cfg_str

*Function-like Macro*

Like `core::cfg!`, except using the seL4 kernel configuration.

This macro requires the configuration key to correspond to a string value. It parses that value
into an integer at compile-time, and assignes to it the type `usize`.

See [`macro@sel4_cfg`] for documentation on the configuration expression syntax.

# Example

```ignore
assert_eq!(1usize, sel4_cfg_usize!(MAX_NUM_NODES));
```

```rust
sel4_cfg_str!(...)
```



## sel4_config_macros::sel4_cfg_struct

*Attribute Macro*

Allows a `struct`'s fields to use the [`macro@sel4_cfg`] attribute.

# Example

```ignore
#[sel4_cfg_struct]
struct Foo {
    #[sel4_cfg(KERNEL_MCS)]
    bar: bool,
}
```

```rust
#[sel4_cfg_struct]
```



## sel4_config_macros::sel4_cfg_usize

*Function-like Macro*

Like `core::cfg!`, except using the seL4 kernel configuration.

This macro requires the configuration key to correspond to a string value. It parses that value
into an integer at compile-time, and assigns it type `usize`.

See [`macro@sel4_cfg`] for documentation on the configuration expression syntax.

# Example

```ignore
assert_eq!(1usize, sel4_cfg_usize!(MAX_NUM_NODES));
```

```rust
sel4_cfg_usize!(...)
```



## sel4_config_macros::sel4_cfg_word

*Function-like Macro*

Like `core::cfg!`, except using the seL4 kernel configuration.

This macro requires the configuration key to correspond to a string value. It parses that value
into an integer at compile-time, and assigns to it the one of the types `u32` or `u64`,
depending on the value of `WORD_SIZE` configuration key.

See [`macro@sel4_cfg`] for documentation on the configuration expression syntax.

# Example

```ignore
assert_eq!(1u64, sel4_cfg_word!(MAX_NUM_NODES));
```

```rust
sel4_cfg_word!(...)
```



## sel4_config_macros::sel4_cfg_wrap_match

*Function-like Macro*

Like [`macro@sel4_cfg_match`], except it works on stable, at the expense of not being an
attribute macro.

# Example

```ignore
sel4_cfg_wrap_match! {
    match foo {
        #[sel4_cfg(KERNEL_MCS)]
        1337 => bar,
    }
}
```

```rust
sel4_cfg_wrap_match!(...)
```



