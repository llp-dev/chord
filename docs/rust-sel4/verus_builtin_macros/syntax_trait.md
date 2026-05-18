**verus_builtin_macros > syntax_trait**

# Module: syntax_trait

## Contents

**Macros**

- [`do_split_trait_method`](#do_split_trait_method)

**Functions**

- [`expand_extension_trait`](#expand_extension_trait)
- [`expand_extension_traits`](#expand_extension_traits)
- [`is_sizedness_bound`](#is_sizedness_bound) - Heuristically determines whether `tp` is a Size-related bound.  We cannot do
- [`new_impl_for_trait`](#new_impl_for_trait)
- [`new_trait_from`](#new_trait_from)
- [`split_trait_method`](#split_trait_method)
- [`split_trait_method_syn`](#split_trait_method_syn)

---

## verus_builtin_macros::syntax_trait::do_split_trait_method

*Declarative Macro*

```rust
macro_rules! do_split_trait_method {
    ($s:ident, $fun:ident, $spec_fun:ident, $mk_rust_attr:ident, $recv:ident, $pred:ident) => { ... };
}
```



## verus_builtin_macros::syntax_trait::expand_extension_trait

*Function*

```rust
fn expand_extension_trait<'tcx>(erase_all: bool, new_items: & mut Vec<verus_syn::Item>, t: &verus_syn::Path, tr: &verus_syn::ItemTrait, tr_spec: &verus_syn::Ident, tr_impl: &verus_syn::Ident)
```



## verus_builtin_macros::syntax_trait::expand_extension_traits

*Function*

```rust
fn expand_extension_traits(erase_all: bool, items: & mut Vec<verus_syn::Item>)
```



## verus_builtin_macros::syntax_trait::is_sizedness_bound

*Function*

Heuristically determines whether `tp` is a Size-related bound.  We cannot do
this correctly in a macro since identifiers are not yet resolved and traits
may be renamed when imported. However, this should be sufficiently rare in
practice, so we ignore this issue here. Otherwise, the user would have to
provide the external_trait_extension macro with an additional parameter.

```rust
fn is_sizedness_bound(tp: &verus_syn::TypeParamBound) -> bool
```



## verus_builtin_macros::syntax_trait::new_impl_for_trait

*Function*

```rust
fn new_impl_for_trait(tr: &verus_syn::ItemTrait, tr_spec: &verus_syn::Path, self_ty: Box<verus_syn::Type>) -> verus_syn::ItemImpl
```



## verus_builtin_macros::syntax_trait::new_trait_from

*Function*

```rust
fn new_trait_from(tr: &verus_syn::ItemTrait, ident: verus_syn::Ident) -> verus_syn::ItemTrait
```



## verus_builtin_macros::syntax_trait::split_trait_method

*Function*

```rust
fn split_trait_method(spec_items: & mut Vec<verus_syn::TraitItem>, fun: & mut verus_syn::TraitItemFn, erase_ghost: bool)
```



## verus_builtin_macros::syntax_trait::split_trait_method_syn

*Function*

```rust
fn split_trait_method_syn(fun: &syn::TraitItemFn, erase_ghost: bool) -> Option<syn::TraitItemFn>
```



