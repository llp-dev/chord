**verus_builtin_macros > attr_rewrite**

# Module: attr_rewrite

## Contents

**Structs**

- [`ExecReplacer`](#execreplacer)
- [`ImplItemReplacer`](#implitemreplacer)

**Enums**

- [`VerusIOTarget`](#verusiotarget)
- [`VerusSpecTarget`](#verusspectarget)

**Functions**

- [`closure_to_fn_sig`](#closure_to_fn_sig)
- [`get_verus_spec`](#get_verus_spec)
- [`is_verus_proof_stmt`](#is_verus_proof_stmt)
- [`proof_rewrite`](#proof_rewrite)
- [`replace_block`](#replace_block)
- [`replace_expr`](#replace_expr)
- [`rewrite_const_ret_proxy`](#rewrite_const_ret_proxy) - Rewrite the const function and return a proxy function.
- [`rewrite_unverified_func`](#rewrite_unverified_func)
- [`rewrite_verus_attribute`](#rewrite_verus_attribute)
- [`rewrite_verus_spec`](#rewrite_verus_spec)
- [`rewrite_verus_spec_on_expr_local`](#rewrite_verus_spec_on_expr_local) - The `verus_spec(with)` annotation can be applied to either a local statement or an expression.
- [`rewrite_verus_spec_on_fun_or_loop`](#rewrite_verus_spec_on_fun_or_loop)
- [`rewrite_verus_spec_on_item_const`](#rewrite_verus_spec_on_item_const)
- [`rewrite_with_expr`](#rewrite_with_expr)
- [`syn_to_verus_syn`](#syn_to_verus_syn)

**Constants**

- [`DUAL_SPEC_PREFIX`](#dual_spec_prefix)
- [`VERIFIED`](#verified)
- [`VERUS_SPEC`](#verus_spec)

---

## verus_builtin_macros::attr_rewrite::DUAL_SPEC_PREFIX

*Constant*: `&str`



## verus_builtin_macros::attr_rewrite::ExecReplacer

*Struct*

**Fields:**
- `erase: crate::EraseGhost`

**Trait Implementations:**

- **VisitMut**
  - `fn visit_attribute_mut(self: & mut Self, node: & mut syn::Attribute)`
  - `fn visit_block_mut(self: & mut Self, block: & mut syn::Block)` - convert proof_with macro to functin with ghost/tracked argumemts.



## verus_builtin_macros::attr_rewrite::ImplItemReplacer

*Struct*

**Fields:**
- `verify_const: bool`

**Trait Implementations:**

- **VisitMut**
  - `fn visit_impl_item_fn_mut(self: & mut Self, method: & mut syn::ImplItemFn)`
  - `fn visit_impl_item_const_mut(self: & mut Self, i: & mut syn::ImplItemConst)`



## verus_builtin_macros::attr_rewrite::VERIFIED

*Constant*: `&str`



## verus_builtin_macros::attr_rewrite::VERUS_SPEC

*Constant*: `&str`



## verus_builtin_macros::attr_rewrite::VerusIOTarget

*Enum*

**Variants:**
- `Local(syn::Local)`
- `Expr(syn::Expr)`

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut proc_macro2::TokenStream)`



## verus_builtin_macros::attr_rewrite::VerusSpecTarget

*Enum*

**Variants:**
- `IOTarget(VerusIOTarget)`
- `FnOrLoop(crate::attr_block_trait::AnyFnOrLoop)`
- `ItemConst(syn::ItemConst)`

**Trait Implementations:**

- **Parse**
  - `fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<VerusSpecTarget>`



## verus_builtin_macros::attr_rewrite::closure_to_fn_sig

*Function*

```rust
fn closure_to_fn_sig(closure: &syn::ExprClosure) -> syn::Signature
```



## verus_builtin_macros::attr_rewrite::get_verus_spec

*Function*

```rust
fn get_verus_spec(attrs: &[syn::Attribute]) -> Option<&syn::Attribute>
```



## verus_builtin_macros::attr_rewrite::is_verus_proof_stmt

*Function*

```rust
fn is_verus_proof_stmt(stmt: &syn::Stmt) -> bool
```



## verus_builtin_macros::attr_rewrite::proof_rewrite

*Function*

```rust
fn proof_rewrite(erase: crate::EraseGhost, input: proc_macro2::TokenStream) -> proc_macro::TokenStream
```



## verus_builtin_macros::attr_rewrite::replace_block

*Function*

```rust
fn replace_block(erase: crate::EraseGhost, fblock: & mut syn::Block)
```



## verus_builtin_macros::attr_rewrite::replace_expr

*Function*

```rust
fn replace_expr(erase: crate::EraseGhost, expr: & mut syn::Expr)
```



## verus_builtin_macros::attr_rewrite::rewrite_const_ret_proxy

*Function*

Rewrite the const function and return a proxy function.

```rust
fn rewrite_const_ret_proxy(const_fun: & mut syn::ItemFn) -> syn::ItemFn
```



## verus_builtin_macros::attr_rewrite::rewrite_unverified_func

*Function*

```rust
fn rewrite_unverified_func(fun: & mut syn::ItemFn, span: proc_macro2::Span, erase: crate::EraseGhost) -> Vec<syn::ItemFn>
```



## verus_builtin_macros::attr_rewrite::rewrite_verus_attribute

*Function*

```rust
fn rewrite_verus_attribute(erase: &crate::EraseGhost, attr_args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream
```



## verus_builtin_macros::attr_rewrite::rewrite_verus_spec

*Function*

```rust
fn rewrite_verus_spec(erase: crate::EraseGhost, outer_attr_tokens: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream
```



## verus_builtin_macros::attr_rewrite::rewrite_verus_spec_on_expr_local

*Function*

The `verus_spec(with)` annotation can be applied to either a local statement or an expression.

- When applied to an expression (`expr`), the trailing semicolon (`;`) is ignored due to limitations of the procedure macro.
  To include the semicolon, developers must use the following syntax:
  ```rust
  {#[verus_spec(with ..)] expr};
  ```

- When used with an expression, developers must explicitly declare the returned ghost or tracked patterns.
  This is because the additional declarations cannot be automatically added in a meaningful way.

Example:
```rust
if #[verus_io(with Tracked(arg1), Ghost(arg2) -> Tracked(out) |= Tracked(extra))]
call(arg0) == something {
}
```
This will be transformed to the following:
```rust
{
    let (tmp, tmp_out) = call(arg0, Tracked(arg1), Tracked(arg2));
    proof!{out = tmp_out.get();}  // Ensuring `out` is properly assigned.
    (tmp, Tracked(extra))  // Returning the transformed values.
}
```

The recommended approach for handling returned ghost/tracked outputs is to use a local statement:

Example:
```rust
#[verus_spec(with Tracked(arg1), Ghost(arg2) -> Tracked(out) |= Tracked(extra))]
let out0 = call(arg0);
```
This will be transformed to:
```rust
let tracked mut out;
let out0 = {
    let (tmp, tmp_out) = call(arg0, Tracked(arg1), Tracked(arg2));
    proof!{out = tmp_out.get();}  // Ensure proper assignment of the ghost value.
    (tmp, Tracked(extra))  // Returning the transformed values.
};
```

```rust
fn rewrite_verus_spec_on_expr_local(erase: crate::EraseGhost, attr_input: proc_macro::TokenStream, io_target: VerusIOTarget) -> proc_macro::TokenStream
```



## verus_builtin_macros::attr_rewrite::rewrite_verus_spec_on_fun_or_loop

*Function*

```rust
fn rewrite_verus_spec_on_fun_or_loop(erase: crate::EraseGhost, outer_attr_tokens: proc_macro::TokenStream, f: crate::attr_block_trait::AnyFnOrLoop) -> proc_macro::TokenStream
```



## verus_builtin_macros::attr_rewrite::rewrite_verus_spec_on_item_const

*Function*

```rust
fn rewrite_verus_spec_on_item_const(erase_ghost: crate::EraseGhost, outer_attr_tokens: proc_macro::TokenStream, item_const: syn::ItemConst) -> proc_macro::TokenStream
```



## verus_builtin_macros::attr_rewrite::rewrite_with_expr

*Function*

```rust
fn rewrite_with_expr(erase: crate::EraseGhost, expr: & mut syn::Expr, call_with_spec: verus_syn::WithSpecOnExpr) -> Vec<verus_syn::Stmt>
```



## verus_builtin_macros::attr_rewrite::syn_to_verus_syn

*Function*

```rust
fn syn_to_verus_syn<V, impl ToTokens>(input: impl Trait) -> V
```



