**verus_builtin_macros > is_variant**

# Module: is_variant

## Contents

**Functions**

- [`attribute_is_variant`](#attribute_is_variant)
- [`attribute_is_variant_internal`](#attribute_is_variant_internal)
- [`attribute_is_variant_no_deprecation_warning`](#attribute_is_variant_no_deprecation_warning)

**Statics**

- [`IS_VARIANT_DEPRECATION_EMITTED`](#is_variant_deprecation_emitted)

---

## verus_builtin_macros::is_variant::IS_VARIANT_DEPRECATION_EMITTED

*Static*

```rust
static IS_VARIANT_DEPRECATION_EMITTED: std::sync::atomic::AtomicBool
```



## verus_builtin_macros::is_variant::attribute_is_variant

*Function*

```rust
fn attribute_is_variant(attr: proc_macro2::TokenStream, s: synstructure::Structure) -> proc_macro2::TokenStream
```



## verus_builtin_macros::is_variant::attribute_is_variant_internal

*Function*

```rust
fn attribute_is_variant_internal(_attr: proc_macro2::TokenStream, s: synstructure::Structure, no_deprecation_warning: bool) -> proc_macro2::TokenStream
```



## verus_builtin_macros::is_variant::attribute_is_variant_no_deprecation_warning

*Function*

```rust
fn attribute_is_variant_no_deprecation_warning(attr: proc_macro2::TokenStream, s: synstructure::Structure) -> proc_macro2::TokenStream
```



