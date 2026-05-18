**verus_builtin_macros > contrib**

# Module: contrib

## Contents

**Modules**

- [`auto_spec`](#auto_spec)
- [`exec_spec`](#exec_spec)
- [`set_build`](#set_build)
- [`spec_derive`](#spec_derive)

**Functions**

- [`collect_attrs`](#collect_attrs)
- [`contrib_preprocess_impl_item`](#contrib_preprocess_impl_item)
- [`contrib_preprocess_impl_items`](#contrib_preprocess_impl_items)
- [`contrib_preprocess_item`](#contrib_preprocess_item)
- [`contrib_preprocess_items`](#contrib_preprocess_items)
- [`impl_item_attrs`](#impl_item_attrs)
- [`item_attrs`](#item_attrs)
- [`traverse_path`](#traverse_path)

---

## Module: auto_spec



## verus_builtin_macros::contrib::collect_attrs

*Function*

```rust
fn collect_attrs(attrs: Option<&Vec<verus_syn::Attribute>>) -> Vec<(String, Option<proc_macro2::TokenStream>)>
```



## verus_builtin_macros::contrib::contrib_preprocess_impl_item

*Function*

```rust
fn contrib_preprocess_impl_item(item: & mut verus_syn::ImplItem, new_items: & mut Vec<verus_syn::ImplItem>)
```



## verus_builtin_macros::contrib::contrib_preprocess_impl_items

*Function*

```rust
fn contrib_preprocess_impl_items(items: & mut Vec<verus_syn::ImplItem>)
```



## verus_builtin_macros::contrib::contrib_preprocess_item

*Function*

```rust
fn contrib_preprocess_item(item: & mut verus_syn::Item, new_items: & mut Vec<verus_syn::Item>)
```



## verus_builtin_macros::contrib::contrib_preprocess_items

*Function*

```rust
fn contrib_preprocess_items(items: & mut Vec<verus_syn::Item>)
```



## Module: exec_spec



## verus_builtin_macros::contrib::impl_item_attrs

*Function*

```rust
fn impl_item_attrs(item: &verus_syn::ImplItem) -> Option<&Vec<verus_syn::Attribute>>
```



## verus_builtin_macros::contrib::item_attrs

*Function*

```rust
fn item_attrs(item: &verus_syn::Item) -> Option<&Vec<verus_syn::Attribute>>
```



## Module: set_build



## Module: spec_derive



## verus_builtin_macros::contrib::traverse_path

*Function*

```rust
fn traverse_path(path: &verus_syn::Path) -> Option<String>
```



