**verus_builtin_macros > contrib > auto_spec**

# Module: contrib::auto_spec

## Contents

**Structs**

- [`Visitor`](#visitor)

**Functions**

- [`auto_spec_fn`](#auto_spec_fn)
- [`auto_spec_impl_item`](#auto_spec_impl_item)
- [`auto_spec_item`](#auto_spec_item)
- [`is_verus_proof_stmt`](#is_verus_proof_stmt)

---

## verus_builtin_macros::contrib::auto_spec::Visitor

*Struct*

**Unit Struct**

**Trait Implementations:**

- **VisitMut**
  - `fn visit_expr_mut(self: & mut Self, expr: & mut Expr)`
  - `fn visit_block_mut(self: & mut Self, block: & mut Block)`



## verus_builtin_macros::contrib::auto_spec::auto_spec_fn

*Function*

```rust
fn auto_spec_fn(span: proc_macro2::Span, attrs: & mut Vec<verus_syn::Attribute>, sig: & mut verus_syn::Signature, block: verus_syn::Block)
```



## verus_builtin_macros::contrib::auto_spec::auto_spec_impl_item

*Function*

```rust
fn auto_spec_impl_item(item: & mut verus_syn::ImplItem, _args: Option<proc_macro2::TokenStream>, _new_items: & mut Vec<verus_syn::ImplItem>)
```



## verus_builtin_macros::contrib::auto_spec::auto_spec_item

*Function*

```rust
fn auto_spec_item(item: & mut verus_syn::Item, _args: Option<proc_macro2::TokenStream>, _new_items: & mut Vec<verus_syn::Item>)
```



## verus_builtin_macros::contrib::auto_spec::is_verus_proof_stmt

*Function*

```rust
fn is_verus_proof_stmt(stmt: &verus_syn::Stmt) -> bool
```



