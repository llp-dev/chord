**syn > ext**

# Module: ext

## Contents

**Modules**

- [`private`](#private)

**Traits**

- [`IdentExt`](#identext) - Additional methods for `Ident` not provided by proc-macro2 or libproc_macro.
- [`PunctExt`](#punctext)
- [`TokenStreamExt`](#tokenstreamext)

---

## syn::ext::IdentExt

*Trait*

Additional methods for `Ident` not provided by proc-macro2 or libproc_macro.

This trait is sealed and cannot be implemented for types outside of Syn. It
is implemented only for `proc_macro2::Ident`.

**Methods:**

- `parse_any`: Parses any identifier including keywords.
- `peek_any`: Peeks any identifier including keywords. Usage:
- `unraw`: Strips the raw marker `r#`, if any, from the beginning of an ident.



## syn::ext::PunctExt

*Trait*

**Methods:**

- `new_spanned`



## syn::ext::TokenStreamExt

*Trait*

**Methods:**

- `append`



## Module: private



