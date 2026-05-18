**syn > macros**

# Module: macros

## Contents

**Macros**

- [`ast_enum`](#ast_enum)
- [`ast_enum_from_struct`](#ast_enum_from_struct)
- [`ast_enum_of_structs`](#ast_enum_of_structs)
- [`ast_enum_of_structs_impl`](#ast_enum_of_structs_impl)
- [`ast_struct`](#ast_struct)
- [`check_keyword_matches`](#check_keyword_matches)
- [`generate_to_tokens`](#generate_to_tokens)
- [`pub_if_not_doc`](#pub_if_not_doc)
- [`return_impl_trait`](#return_impl_trait)

---

## syn::macros::ast_enum

*Declarative Macro*

```rust
macro_rules! ast_enum {
    (
        $(#[$enum_attr:meta])*
        $pub:ident $enum:ident $name:ident $body:tt
    ) => { ... };
}
```



## syn::macros::ast_enum_from_struct

*Declarative Macro*

```rust
macro_rules! ast_enum_from_struct {
    ($name:ident::Verbatim, $member:ident) => { ... };
    ($name:ident::$variant:ident, $member:ident) => { ... };
}
```



## syn::macros::ast_enum_of_structs

*Declarative Macro*

```rust
macro_rules! ast_enum_of_structs {
    (
        $(#[$enum_attr:meta])*
        $pub:ident $enum:ident $name:ident $body:tt
    ) => { ... };
}
```



## syn::macros::ast_enum_of_structs_impl

*Declarative Macro*

```rust
macro_rules! ast_enum_of_structs_impl {
    (
        $name:ident {
            $(
                $(#[cfg $cfg_attr:tt])*
                $(#[doc $($doc_attr:tt)*])*
                $variant:ident $( ($member:ident) )*,
            )*
        }
    ) => { ... };
}
```



## syn::macros::ast_struct

*Declarative Macro*

```rust
macro_rules! ast_struct {
    (
        $(#[$attr:meta])*
        $pub:ident $struct:ident $name:ident #full $body:tt
    ) => { ... };
    (
        $(#[$attr:meta])*
        $pub:ident $struct:ident $name:ident $body:tt
    ) => { ... };
}
```



## syn::macros::check_keyword_matches

*Declarative Macro*

```rust
macro_rules! check_keyword_matches {
    (enum enum) => { ... };
    (pub pub) => { ... };
    (struct struct) => { ... };
}
```



## syn::macros::generate_to_tokens

*Declarative Macro*

```rust
macro_rules! generate_to_tokens {
    (
        ($($arms:tt)*) $tokens:ident $name:ident {
            $(#[cfg $cfg_attr:tt])*
            $(#[doc $($doc_attr:tt)*])*
            $variant:ident,
            $($next:tt)*
        }
    ) => { ... };
    (
        ($($arms:tt)*) $tokens:ident $name:ident {
            $(#[cfg $cfg_attr:tt])*
            $(#[doc $($doc_attr:tt)*])*
            $variant:ident($member:ident),
            $($next:tt)*
        }
    ) => { ... };
    (($($arms:tt)*) $tokens:ident $name:ident {}) => { ... };
}
```



## syn::macros::pub_if_not_doc

*Declarative Macro*

```rust
macro_rules! pub_if_not_doc {
    ($(#[$m:meta])* $pub:ident $($item:tt)*) => { ... };
}
```



## syn::macros::return_impl_trait

*Declarative Macro*

```rust
macro_rules! return_impl_trait {
    (
        $(#[$attr:meta])*
        $vis:vis fn $name:ident $args:tt -> $impl_trait:ty [$concrete:ty] $body:block
    ) => { ... };
}
```



