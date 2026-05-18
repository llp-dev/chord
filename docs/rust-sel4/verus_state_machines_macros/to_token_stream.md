**verus_state_machines_macros > to_token_stream**

# Module: to_token_stream

## Contents

**Functions**

- [`expect_first_param_is_label`](#expect_first_param_is_label)
- [`fix_attrs`](#fix_attrs)
- [`generic_components_for_fn`](#generic_components_for_fn)
- [`generics_for_decl`](#generics_for_decl)
- [`get_generic_args`](#get_generic_args)
- [`get_self_ty`](#get_self_ty)
- [`get_self_ty_turbofish`](#get_self_ty_turbofish)
- [`get_self_ty_turbofish_path`](#get_self_ty_turbofish_path)
- [`get_step_ty`](#get_step_ty)
- [`impl_decl_stream`](#impl_decl_stream)
- [`impl_decl_stream_for`](#impl_decl_stream_for)
- [`just_args`](#just_args)
- [`just_params`](#just_params)
- [`left_of_colon`](#left_of_colon)
- [`lemma_update_body`](#lemma_update_body) - Add pre-conditions and post-conditions to the inductiveness lemma.
- [`name_with_type_args`](#name_with_type_args)
- [`name_with_type_args_turbofish`](#name_with_type_args_turbofish)
- [`name_with_type_args_turbofish_path`](#name_with_type_args_turbofish_path)
- [`output_other_fns`](#output_other_fns)
- [`output_primary_stuff`](#output_primary_stuff)
- [`output_step_datatype`](#output_step_datatype)
- [`output_take_step_fns`](#output_take_step_fns)
- [`output_token_stream`](#output_token_stream)
- [`post_args`](#post_args)
- [`post_assoc_params`](#post_assoc_params)
- [`post_label_args`](#post_label_args)
- [`post_params`](#post_params)
- [`pre_args`](#pre_args)
- [`pre_assoc_params`](#pre_assoc_params)
- [`pre_params`](#pre_params) - pre: Self, post: Self, params...
- [`pre_post_args`](#pre_post_args)
- [`pre_post_assoc_params`](#pre_post_assoc_params)
- [`pre_post_label_args`](#pre_post_label_args)
- [`pre_post_params`](#pre_post_params) - pre: Self, post: Self, params...
- [`set_mode_proof`](#set_mode_proof)
- [`shardable_type_to_type`](#shardable_type_to_type)
- [`should_delete_attr`](#should_delete_attr)
- [`step_params`](#step_params)

---

## verus_state_machines_macros::to_token_stream::expect_first_param_is_label

*Function*

```rust
fn expect_first_param_is_label(sm: &crate::ast::SM, tr: &crate::ast::Transition) -> bool
```



## verus_state_machines_macros::to_token_stream::fix_attrs

*Function*

```rust
fn fix_attrs(attrs: & mut Vec<verus_syn::Attribute>)
```



## verus_state_machines_macros::to_token_stream::generic_components_for_fn

*Function*

```rust
fn generic_components_for_fn(generics: &Option<verus_syn::Generics>) -> (proc_macro2::TokenStream, proc_macro2::TokenStream)
```



## verus_state_machines_macros::to_token_stream::generics_for_decl

*Function*

```rust
fn generics_for_decl(generics: &Option<verus_syn::Generics>) -> (proc_macro2::TokenStream, proc_macro2::TokenStream)
```



## verus_state_machines_macros::to_token_stream::get_generic_args

*Function*

```rust
fn get_generic_args(params: &verus_syn::punctuated::Punctuated<verus_syn::GenericParam, token::Comma>) -> verus_syn::punctuated::Punctuated<verus_syn::GenericArgument, token::Comma>
```



## verus_state_machines_macros::to_token_stream::get_self_ty

*Function*

```rust
fn get_self_ty(sm: &crate::ast::SM) -> verus_syn::Type
```



## verus_state_machines_macros::to_token_stream::get_self_ty_turbofish

*Function*

```rust
fn get_self_ty_turbofish(sm: &crate::ast::SM) -> verus_syn::Type
```



## verus_state_machines_macros::to_token_stream::get_self_ty_turbofish_path

*Function*

```rust
fn get_self_ty_turbofish_path(sm: &crate::ast::SM) -> verus_syn::Path
```



## verus_state_machines_macros::to_token_stream::get_step_ty

*Function*

```rust
fn get_step_ty(sm: &crate::ast::SM, is_init: bool) -> verus_syn::Type
```



## verus_state_machines_macros::to_token_stream::impl_decl_stream

*Function*

```rust
fn impl_decl_stream(self_ty: &verus_syn::Type, generics: &Option<verus_syn::Generics>) -> proc_macro2::TokenStream
```



## verus_state_machines_macros::to_token_stream::impl_decl_stream_for

*Function*

```rust
fn impl_decl_stream_for(self_ty: &verus_syn::Type, generics: &Option<verus_syn::Generics>, for_trait: proc_macro2::TokenStream) -> proc_macro2::TokenStream
```



## verus_state_machines_macros::to_token_stream::just_args

*Function*

```rust
fn just_args(params: &Vec<crate::ast::TransitionParam>, skip_first: bool) -> proc_macro2::TokenStream
```



## verus_state_machines_macros::to_token_stream::just_params

*Function*

```rust
fn just_params(params: &Vec<crate::ast::TransitionParam>) -> proc_macro2::TokenStream
```



## verus_state_machines_macros::to_token_stream::left_of_colon

*Function*

```rust
fn left_of_colon<'a>(fn_arg: &'a verus_syn::FnArg) -> &'a verus_syn::Pat
```



## verus_state_machines_macros::to_token_stream::lemma_update_body

*Function*

Add pre-conditions and post-conditions to the inductiveness lemma.

For 'init' routines:
  requires(initialized(post, ...));
  ensures(post.invariant());

For normal transitions:
  requires(pre.invariant() && transition(pre, post, ...));
  ensures(post.invariant());

For 'readonly' transitions, there is no need to prove inductiveness.
We should have already ruled out the existence of such lemmas.

```rust
fn lemma_update_body(bundle: &crate::parse_token_stream::SMBundle, l: &crate::ast::Lemma, func: & mut verus_syn::ImplItemFn)
```



## verus_state_machines_macros::to_token_stream::name_with_type_args

*Function*

```rust
fn name_with_type_args(name: &verus_syn::Ident, sm: &crate::ast::SM) -> verus_syn::Type
```



## verus_state_machines_macros::to_token_stream::name_with_type_args_turbofish

*Function*

```rust
fn name_with_type_args_turbofish(name: &verus_syn::Ident, sm: &crate::ast::SM) -> verus_syn::Type
```



## verus_state_machines_macros::to_token_stream::name_with_type_args_turbofish_path

*Function*

```rust
fn name_with_type_args_turbofish_path(name: &verus_syn::Ident, sm: &crate::ast::SM) -> verus_syn::Path
```



## verus_state_machines_macros::to_token_stream::output_other_fns

*Function*

```rust
fn output_other_fns(bundle: &crate::parse_token_stream::SMBundle, impl_stream: & mut proc_macro2::TokenStream, invariants: &Vec<crate::ast::Invariant>, lemmas: &Vec<crate::ast::Lemma>, normal_fns: &Vec<verus_syn::ImplItemFn>)
```



## verus_state_machines_macros::to_token_stream::output_primary_stuff

*Function*

```rust
fn output_primary_stuff(root_stream: & mut proc_macro2::TokenStream, impl_stream: & mut proc_macro2::TokenStream, bundle: &crate::parse_token_stream::SMBundle) -> std::collections::HashMap<String, verus_syn::Ident>
```



## verus_state_machines_macros::to_token_stream::output_step_datatype

*Function*

```rust
fn output_step_datatype(root_stream: & mut proc_macro2::TokenStream, show_stream: & mut proc_macro2::TokenStream, impl_stream: & mut proc_macro2::TokenStream, sm: &crate::ast::SM, is_init: bool)
```



## verus_state_machines_macros::to_token_stream::output_take_step_fns

*Function*

```rust
fn output_take_step_fns(stream: & mut proc_macro2::TokenStream, bundle: &crate::parse_token_stream::SMBundle, safety_condition_lemmas: &std::collections::HashMap<String, verus_syn::Ident>)
```



## verus_state_machines_macros::to_token_stream::output_token_stream

*Function*

```rust
fn output_token_stream(bundle: crate::parse_token_stream::SMBundle, concurrent: bool) -> parse::Result<proc_macro2::TokenStream>
```



## verus_state_machines_macros::to_token_stream::post_args

*Function*

```rust
fn post_args(params: &Vec<crate::ast::TransitionParam>) -> proc_macro2::TokenStream
```



## verus_state_machines_macros::to_token_stream::post_assoc_params

*Function*

```rust
fn post_assoc_params(ty: &verus_syn::Type, params: &Vec<crate::ast::TransitionParam>) -> proc_macro2::TokenStream
```



## verus_state_machines_macros::to_token_stream::post_label_args

*Function*

```rust
fn post_label_args(params: &Vec<crate::ast::TransitionParam>) -> proc_macro2::TokenStream
```



## verus_state_machines_macros::to_token_stream::post_params

*Function*

```rust
fn post_params(params: &Vec<crate::ast::TransitionParam>) -> proc_macro2::TokenStream
```



## verus_state_machines_macros::to_token_stream::pre_args

*Function*

```rust
fn pre_args(params: &Vec<crate::ast::TransitionParam>) -> proc_macro2::TokenStream
```



## verus_state_machines_macros::to_token_stream::pre_assoc_params

*Function*

```rust
fn pre_assoc_params(ty: &verus_syn::Type, params: &Vec<crate::ast::TransitionParam>) -> proc_macro2::TokenStream
```



## verus_state_machines_macros::to_token_stream::pre_params

*Function*

pre: Self, post: Self, params...

```rust
fn pre_params(params: &Vec<crate::ast::TransitionParam>) -> proc_macro2::TokenStream
```



## verus_state_machines_macros::to_token_stream::pre_post_args

*Function*

```rust
fn pre_post_args(params: &Vec<crate::ast::TransitionParam>) -> proc_macro2::TokenStream
```



## verus_state_machines_macros::to_token_stream::pre_post_assoc_params

*Function*

```rust
fn pre_post_assoc_params(ty: &verus_syn::Type, params: &Vec<crate::ast::TransitionParam>) -> proc_macro2::TokenStream
```



## verus_state_machines_macros::to_token_stream::pre_post_label_args

*Function*

```rust
fn pre_post_label_args(params: &Vec<crate::ast::TransitionParam>) -> proc_macro2::TokenStream
```



## verus_state_machines_macros::to_token_stream::pre_post_params

*Function*

pre: Self, post: Self, params...

```rust
fn pre_post_params(params: &Vec<crate::ast::TransitionParam>) -> proc_macro2::TokenStream
```



## verus_state_machines_macros::to_token_stream::set_mode_proof

*Function*

```rust
fn set_mode_proof(sig: & mut verus_syn::Signature, span: proc_macro2::Span)
```



## verus_state_machines_macros::to_token_stream::shardable_type_to_type

*Function*

```rust
fn shardable_type_to_type(span: proc_macro2::Span, stype: &crate::ast::ShardableType) -> verus_syn::Type
```



## verus_state_machines_macros::to_token_stream::should_delete_attr

*Function*

```rust
fn should_delete_attr(attr: &verus_syn::Attribute) -> bool
```



## verus_state_machines_macros::to_token_stream::step_params

*Function*

```rust
fn step_params(sm: &crate::ast::SM, tr: &crate::ast::Transition) -> proc_macro2::TokenStream
```



