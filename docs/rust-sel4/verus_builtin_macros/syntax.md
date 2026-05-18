**verus_builtin_macros > syntax**

# Module: syntax

## Contents

**Macros**

- [`declare_mk_rust_attr`](#declare_mk_rust_attr) - Constructs #[name(tokens)]
- [`declare_mk_verus_attr`](#declare_mk_verus_attr) - Constructs #[verus::internal(tokens)] and #[verifier::tokens]
- [`parse_quote_spanned_builtin`](#parse_quote_spanned_builtin)
- [`parse_quote_spanned_vstd`](#parse_quote_spanned_vstd)
- [`quote_builtin`](#quote_builtin)
- [`quote_spanned_builtin`](#quote_spanned_builtin)
- [`quote_spanned_builtin_builtin_macros`](#quote_spanned_builtin_builtin_macros)
- [`quote_spanned_builtin_builtin_macros_vstd`](#quote_spanned_builtin_builtin_macros_vstd)
- [`quote_spanned_vstd`](#quote_spanned_vstd)
- [`quote_verbatim`](#quote_verbatim)
- [`quote_vstd`](#quote_vstd)
- [`stmt_with_semi`](#stmt_with_semi)

**Structs**

- [`Builtin`](#builtin)
- [`BuiltinMacros`](#builtinmacros)
- [`ExecGhostPatVisitor`](#execghostpatvisitor)
- [`ImplItems`](#implitems)
- [`Items`](#items)
- [`MacroElements`](#macroelements)
- [`MacroElementsExplicitExpr`](#macroelementsexplicitexpr)
- [`MacroInvoke`](#macroinvoke)
- [`MacroInvokeExplicitExpr`](#macroinvokeexplicitexpr)
- [`ProofFnOptions`](#prooffnoptions)
- [`Stmts`](#stmts)
- [`Visitor`](#visitor)
- [`Vstd`](#vstd)

**Enums**

- [`Delimiter`](#delimiter)
- [`ExtractQuantTriggersFound`](#extractquanttriggersfound)
- [`InsideArith`](#insidearith)
- [`MacroElement`](#macroelement)
- [`MacroElementExplicitExpr`](#macroelementexplicitexpr)
- [`ProofFnTypeArg`](#prooffntypearg)
- [`ProofFnUsage`](#prooffnusage)

**Functions**

- [`chain_count`](#chain_count)
- [`check_return_ident`](#check_return_ident) - In VIR there's the same check, but Rustc will complain first, and throw out
- [`check_verus_return_ident`](#check_verus_return_ident) - In VIR there's the same check, but Rustc will complain first, and throw out
- [`data_mode_attrs`](#data_mode_attrs)
- [`for_loop_spec_attr`](#for_loop_spec_attr)
- [`generic_to_tokens`](#generic_to_tokens)
- [`get_ex_ident_mangle_path`](#get_ex_ident_mangle_path)
- [`has_external_code`](#has_external_code)
- [`inputs_to_tokens`](#inputs_to_tokens)
- [`into_spans`](#into_spans)
- [`inv_macro_exprs`](#inv_macro_exprs)
- [`is_encoded_const`](#is_encoded_const)
- [`is_external`](#is_external)
- [`is_probably_float_type`](#is_probably_float_type)
- [`is_probably_nat_or_int_type`](#is_probably_nat_or_int_type)
- [`is_probably_real_type`](#is_probably_real_type)
- [`is_ptr_type`](#is_ptr_type)
- [`mk_rust_attr`](#mk_rust_attr)
- [`mk_rust_attr_syn`](#mk_rust_attr_syn)
- [`mk_verifier_attr`](#mk_verifier_attr)
- [`mk_verifier_attr_syn`](#mk_verifier_attr_syn)
- [`mk_verus_attr`](#mk_verus_attr)
- [`mk_verus_attr_syn`](#mk_verus_attr_syn)
- [`path_is_ident`](#path_is_ident)
- [`path_matches_idents`](#path_matches_idents)
- [`proof_block`](#proof_block)
- [`proof_fn_track_to_type`](#proof_fn_track_to_type)
- [`proof_fn_tracks_to_type`](#proof_fn_tracks_to_type)
- [`proof_macro_explicit_exprs`](#proof_macro_explicit_exprs)
- [`proof_macro_exprs`](#proof_macro_exprs)
- [`rejoin_tokens`](#rejoin_tokens)
- [`rewrite_args_unwrap_ghost_tracked`](#rewrite_args_unwrap_ghost_tracked)
- [`rewrite_exe_pat`](#rewrite_exe_pat)
- [`rewrite_expr`](#rewrite_expr)
- [`rewrite_expr_node`](#rewrite_expr_node)
- [`rewrite_impl_items`](#rewrite_impl_items)
- [`rewrite_items`](#rewrite_items)
- [`rewrite_proof_decl`](#rewrite_proof_decl)
- [`sig_specs_attr`](#sig_specs_attr)
- [`split_off_proof_note_attrs`](#split_off_proof_note_attrs)
- [`take_expr`](#take_expr)
- [`take_ghost`](#take_ghost)
- [`take_pat`](#take_pat)
- [`take_sig_with_spec`](#take_sig_with_spec)
- [`take_type`](#take_type)
- [`verus_generic_to_tokens`](#verus_generic_to_tokens)
- [`verus_inputs_to_tokens`](#verus_inputs_to_tokens)
- [`while_loop_spec_attr`](#while_loop_spec_attr)
- [`wrap_expr_with_attrs`](#wrap_expr_with_attrs)

**Constants**

- [`ILLEGAL_CALLEES`](#illegal_callees)
- [`PROOF_FN`](#proof_fn)
- [`PROOF_FN_COPY`](#proof_fn_copy)
- [`PROOF_FN_MUT`](#proof_fn_mut)
- [`PROOF_FN_ONCE`](#proof_fn_once)
- [`PROOF_FN_SEND`](#proof_fn_send)
- [`PROOF_FN_SYNC`](#proof_fn_sync)
- [`VERUS_SPEC`](#verus_spec)

---

## verus_builtin_macros::syntax::Builtin

*Struct*

**Tuple Struct**: `(proc_macro2::Span)`

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## verus_builtin_macros::syntax::BuiltinMacros

*Struct*

**Tuple Struct**: `(proc_macro2::Span)`

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## verus_builtin_macros::syntax::Delimiter

*Enum*

**Variants:**
- `Paren(verus_syn::token::Paren)`
- `Bracket(verus_syn::token::Bracket)`
- `Brace(verus_syn::token::Brace)`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## verus_builtin_macros::syntax::ExecGhostPatVisitor

*Struct*

**Fields:**
- `inside_ghost: u32`
- `tracked: Option<$crate::token::Tracked>`
- `ghost: Option<$crate::token::Ghost>`
- `x_decls: Vec<verus_syn::Stmt>`
- `x_assigns: Vec<verus_syn::Stmt>`

**Trait Implementations:**

- **VisitMut**
  - `fn visit_pat_mut(self: & mut Self, pat: & mut Pat)`



## verus_builtin_macros::syntax::ExtractQuantTriggersFound

*Enum*

**Variants:**
- `Auto`
- `AllTriggers`
- `Triggers(verus_syn::ExprTuple)`
- `None`



## verus_builtin_macros::syntax::ILLEGAL_CALLEES

*Constant*: `&[&str]`



## verus_builtin_macros::syntax::ImplItems

*Struct*

**Fields:**
- `items: Vec<verus_syn::ImplItem>`

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> verus_syn::parse::Result<ImplItems>`



## verus_builtin_macros::syntax::InsideArith

*Enum*

**Variants:**
- `None`
- `Widen`
- `Fixed`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> InsideArith`
- **PartialEq**
  - `fn eq(self: &Self, other: &InsideArith) -> bool`



## verus_builtin_macros::syntax::Items

*Struct*

**Fields:**
- `items: Vec<verus_syn::Item>`

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> verus_syn::parse::Result<Items>`



## verus_builtin_macros::syntax::MacroElement

*Enum*

**Variants:**
- `Comma($crate::token::Comma)`
- `Semi($crate::token::Semi)`
- `FatArrow($crate::token::FatArrow)`
- `Colon($crate::token::Colon)`
- `Expr(Box<verus_syn::Expr>)`

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Parse**
  - `fn parse(input: ParseStream) -> verus_syn::parse::Result<MacroElement>`



## verus_builtin_macros::syntax::MacroElementExplicitExpr

*Enum*

**Variants:**
- `Comma($crate::token::Comma)`
- `Semi($crate::token::Semi)`
- `FatArrow($crate::token::FatArrow)`
- `Colon($crate::token::Colon)`
- `ExplicitExpr($crate::token::At, $crate::token::At, Box<verus_syn::Expr>)`
- `TT(proc_macro2::TokenTree)`

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Parse**
  - `fn parse(input: ParseStream) -> verus_syn::parse::Result<MacroElementExplicitExpr>`



## verus_builtin_macros::syntax::MacroElements

*Struct*

**Fields:**
- `elements: Vec<MacroElement>`

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> verus_syn::parse::Result<MacroElements>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## verus_builtin_macros::syntax::MacroElementsExplicitExpr

*Struct*

**Fields:**
- `elements: Vec<MacroElementExplicitExpr>`

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> verus_syn::parse::Result<MacroElementsExplicitExpr>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## verus_builtin_macros::syntax::MacroInvoke

*Struct*

**Fields:**
- `path: verus_syn::Path`
- `bang: $crate::token::Not`
- `delimiter: Delimiter`
- `elements: MacroElements`

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> verus_syn::parse::Result<MacroInvoke>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## verus_builtin_macros::syntax::MacroInvokeExplicitExpr

*Struct*

**Fields:**
- `path: verus_syn::Path`
- `bang: $crate::token::Not`
- `delimiter: Delimiter`
- `elements: MacroElementsExplicitExpr`

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> verus_syn::parse::Result<MacroInvokeExplicitExpr>`
- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## verus_builtin_macros::syntax::PROOF_FN

*Constant*: `u8`



## verus_builtin_macros::syntax::PROOF_FN_COPY

*Constant*: `u8`



## verus_builtin_macros::syntax::PROOF_FN_MUT

*Constant*: `u8`



## verus_builtin_macros::syntax::PROOF_FN_ONCE

*Constant*: `u8`



## verus_builtin_macros::syntax::PROOF_FN_SEND

*Constant*: `u8`



## verus_builtin_macros::syntax::PROOF_FN_SYNC

*Constant*: `u8`



## verus_builtin_macros::syntax::ProofFnOptions

*Struct*

**Fields:**
- `usage: ProofFnUsage`
- `req_ens: Option<verus_syn::Type>`
- `copy: bool`
- `send: bool`
- `sync: bool`

**Methods:**

- `fn parse<'a, impl Iterator<Item = &'a verus_syn::PathSegment>>(iter: impl Trait) -> Result<Self, String>`
- `fn parse_opt(opt: &Option<verus_syn::FnProofOptions>) -> Result<Self, String>`
- `fn to_types(self: &Self, span: Span) -> (Type, Type, Type, Type, Type)`

**Trait Implementations:**

- **Default**
  - `fn default() -> ProofFnOptions`



## verus_builtin_macros::syntax::ProofFnTypeArg

*Enum*

**Variants:**
- `Usage(ProofFnUsage)`
- `ReqEns(Option<Box<verus_syn::Type>>)`
- `Copy`
- `Send`
- `Sync`
- `Tracked`
- `Ghost`
- `Zero`

**Methods:**

- `fn to_type(self: &Self, span: Span) -> Type`



## verus_builtin_macros::syntax::ProofFnUsage

*Enum*

**Variants:**
- `FnOnce`
- `FnMut`
- `Fn`

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &ProofFnUsage) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> ProofFnUsage`
- **Default**
  - `fn default() -> Self`



## verus_builtin_macros::syntax::Stmts

*Struct*

**Tuple Struct**: `(Vec<verus_syn::Stmt>)`

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> verus_syn::Result<Self>`



## verus_builtin_macros::syntax::VERUS_SPEC

*Constant*: `&str`



## verus_builtin_macros::syntax::Visitor

*Struct*

**Fields:**
- `erase_ghost: crate::EraseGhost`
- `use_spec_traits: bool`
- `inside_ghost: u32`
- `inside_type: u32`
- `inside_external_code: u32`
- `inside_const: bool`
- `inside_arith: InsideArith`
- `assign_to: bool`
- `rustdoc: bool`

**Methods:**

- `fn chain_operators(self: & mut Self, expr: & mut Expr) -> bool`
- `fn closure_quant_operators(self: & mut Self, expr: & mut Expr) -> bool` - Turn `forall|x| ...`
- `fn handle_big_and_big_or(self: & mut Self, expr: & mut Expr) -> bool` - Handle &&& and |||
- `fn handle_spec_operators(self: & mut Self, expr: & mut Expr) -> bool`
- `fn handle_unary_ops(self: & mut Self, expr: & mut Expr) -> bool` - Handle UnaryOp expressions Neg and Sub
- `fn handle_binary_ops(self: & mut Self, expr: & mut Expr) -> bool` - Handle all BinaryOp expressions, transforming them if necessary
- `fn handle_cast(self: & mut Self, expr: & mut Expr) -> bool` - Handle `as` casts. These need to turn into `spec_cast_integer` calls in spec contexts.
- `fn handle_lit_int(self: & mut Self, expr: & mut Expr) -> bool` - Handle integer literals.
- `fn handle_lit_float(self: & mut Self, expr: & mut Expr) -> bool` - Handle float/real literals.
- `fn handle_assume(self: & mut Self, expr: & mut Expr) -> bool` - Handle `assume` statements. Automatically wrap them in a proof block.
- `fn handle_assert(self: & mut Self, expr: & mut Expr) -> bool` - Handle `assert` statements. Automatically wrap them in a proof block.
- `fn handle_assert_forall(self: & mut Self, expr: & mut Expr) -> bool` - Handle `assert forall` statements. Automatically wrap them in a proof block.
- `fn handle_reveal_hide(self: & mut Self, expr: & mut Expr) -> bool` - Handle `reveal` and `hide` statements.
- `fn auto_proof_block(self: & mut Self, expr: & mut Expr, span: Span)`
- `fn handle_mode_blocks(self: & mut Self, expr: & mut Expr) -> bool` - Handle:
- `fn handle_closures(self: & mut Self, expr: & mut Expr) -> bool` - Handle closures
- `fn add_loop_specs(self: & mut Self, stmts: & mut Vec<Stmt>, invariant_except_breaks: Option<InvariantExceptBreak>, invariants: Option<Invariant>, invariant_ensures: Option<InvariantEnsures>, ensures: Option<Ensures>, decreases: Option<Decreases>)`
- `fn desugar_for_loop(self: & mut Self, for_loop: verus_syn::ExprForLoop) -> Expr`
- `fn extract_quant_triggers(self: & mut Self, inner_attrs: Vec<Attribute>, span: Span) -> Result<ExtractQuantTriggersFound, Box<Expr>>`
- `fn visit_expr_with_arith(self: & mut Self, expr: & mut Expr, arith_mode: InsideArith)`
- `fn normalize_expr_proof_note_attrs(self: & mut Self, expr: & mut Expr)`
- `fn take_ghost<T>(self: &Self, dest: & mut T) -> T`
- `fn maybe_erase_expr(self: &Self, span: Span, e: Expr) -> Expr`
- `fn filter_attrs(self: & mut Self, attrs: & mut Vec<Attribute>)`
- `fn visit_loop_spec(self: & mut Self, spec: & mut verus_syn::LoopSpec)`
- `fn take_sig_specs<TType, impl ToTokens, impl ToTokens, impl ToTokens, impl ToTokens>(self: & mut Self, spec: & mut SignatureSpec, ret_pat: Option<(Pat, TType)>, final_ret_pat: Option<Pat>, _span: Span, is_impl_fn: bool, is_closure: bool, ident: impl Trait, generics: Option<impl Trait>, inputs: (Option<impl Trait>, impl Trait)) -> Vec<Stmt>`
- `fn visit_fn(self: & mut Self, attrs: & mut Vec<Attribute>, vis: Option<&Visibility>, sig: & mut Signature, semi_token: Option<$crate::token::Semi>, is_trait: bool, is_impl_fn: bool) -> Vec<Stmt>`
- `fn desugar_const_or_static(self: & mut Self, con_mode: &FnMode, con_ensures: & mut Option<Ensures>, con_block: & mut Option<Box<Block>>, con_expr: & mut Option<Box<Expr>>, con_eq_token: & mut Option<$crate::token::Eq>, con_semi_token: & mut Option<$crate::token::Semi>, _con_ty: &Type, con_span: Span)`
- `fn visit_const_or_static(self: & mut Self, span: proc_macro2::Span, attrs: & mut Vec<Attribute>, vis: Option<&Visibility>, publish: & mut Publish, mode: & mut FnMode) -> FnMode`
- `fn visit_local_extend(self: & mut Self, local: & mut Local) -> (bool, Vec<Stmt>)`
- `fn visit_stmt_extend(self: & mut Self, stmt: & mut Stmt) -> (bool, Vec<Stmt>)`
- `fn visit_stream_expr(self: & mut Self, stream: proc_macro::TokenStream) -> proc_macro::TokenStream`
- `fn visit_items_prefilter(self: & mut Self, items: & mut Vec<Item>)`
- `fn handle_assume_specification(self: & mut Self, assume_specification: &AssumeSpecification, span: Span, assume_spec_extra_impl_items: & mut Vec<ImplItem>) -> Item`
- `fn handle_broadcast_group(self: & mut Self, item_broadcast_group: &ItemBroadcastGroup, span: Span) -> TokenStream`
- `fn visit_items_post(self: & mut Self, items: & mut Vec<Item>)`
- `fn visit_impl_items_post(self: & mut Self, _items: & mut Vec<ImplItem>)`
- `fn visit_impl_items_prefilter(self: & mut Self, items: & mut Vec<ImplItem>, for_trait: bool)`
- `fn visit_trait_items_prefilter(self: & mut Self, items: & mut Vec<TraitItem>)`
- `fn needs_unerased_proxies(self: &Self) -> bool`
- `fn item_needs_unerased_proxy(self: &Self, item: &Item) -> bool`
- `fn impl_item_needs_unerased_proxy(self: &Self, impl_item: &ImplItem, for_trait: bool) -> bool`
- `fn item_translate_const_to_0_arg_fn(self: &Self, item: Item) -> Item`
- `fn impl_item_translate_const_to_0_arg_fn(self: &Self, impl_item: ImplItem) -> ImplItem`
- `fn item_make_proxy(self: &Self, item: Item) -> Item`
- `fn impl_item_make_proxy(self: &Self, impl_item: ImplItem) -> ImplItem`
- `fn attribute_cannot_be_on_two_different_functions(attr: &verus_syn::Attribute) -> bool`
- `fn item_make_external_and_erased(self: & mut Self, item: Item) -> Item`
- `fn impl_item_make_external_and_erased(self: & mut Self, impl_item: ImplItem) -> ImplItem`
- `fn visit_items_make_unerased_proxies(self: & mut Self, items: & mut Vec<Item>)`
- `fn visit_impl_items_make_unerased_proxies(self: & mut Self, impl_items: & mut Vec<ImplItem>, for_trait: bool)`

**Trait Implementations:**

- **VisitMut**
  - `fn visit_expr_mut(self: & mut Self, expr: & mut Expr)`
  - `fn visit_attribute_mut(self: & mut Self, attr: & mut Attribute)`
  - `fn visit_expr_while_mut(self: & mut Self, expr_while: & mut ExprWhile)`
  - `fn visit_expr_loop_mut(self: & mut Self, expr_loop: & mut ExprLoop)`
  - `fn visit_specification_mut(self: & mut Self, spec: & mut verus_syn::Specification)`
  - `fn visit_local_mut(self: & mut Self, local: & mut Local)`
  - `fn visit_block_mut(self: & mut Self, block: & mut Block)`
  - `fn visit_type_param_mut(self: & mut Self, p: & mut verus_syn::TypeParam)`
  - `fn visit_item_fn_mut(self: & mut Self, fun: & mut ItemFn)`
  - `fn visit_impl_item_fn_mut(self: & mut Self, method: & mut ImplItemFn)`
  - `fn visit_trait_item_fn_mut(self: & mut Self, method: & mut TraitItemFn)`
  - `fn visit_item_const_mut(self: & mut Self, con: & mut ItemConst)`
  - `fn visit_item_static_mut(self: & mut Self, sta: & mut ItemStatic)`
  - `fn visit_impl_item_const_mut(self: & mut Self, con: & mut verus_syn::ImplItemConst)`
  - `fn visit_field_mut(self: & mut Self, field: & mut Field)`
  - `fn visit_item_enum_mut(self: & mut Self, item: & mut ItemEnum)`
  - `fn visit_item_union_mut(self: & mut Self, item: & mut ItemUnion)`
  - `fn visit_item_struct_mut(self: & mut Self, item: & mut ItemStruct)`
  - `fn visit_type_mut(self: & mut Self, ty: & mut Type)`
  - `fn visit_path_mut(self: & mut Self, path: & mut Path)`
  - `fn visit_generic_argument_mut(self: & mut Self, arg: & mut verus_syn::GenericArgument)`
  - `fn visit_item_mod_mut(self: & mut Self, item: & mut ItemMod)`
  - `fn visit_item_impl_mut(self: & mut Self, imp: & mut ItemImpl)`
  - `fn visit_item_trait_mut(self: & mut Self, tr: & mut ItemTrait)`
  - `fn visit_reveal_hide_mut(self: & mut Self, _i: & mut verus_syn::RevealHide)`
  - `fn visit_item_broadcast_group_mut(self: & mut Self, _i: & mut ItemBroadcastGroup)`



## verus_builtin_macros::syntax::Vstd

*Struct*

**Tuple Struct**: `(proc_macro2::Span)`

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## verus_builtin_macros::syntax::chain_count

*Function*

```rust
fn chain_count(expr: &verus_syn::Expr) -> u32
```



## verus_builtin_macros::syntax::check_return_ident

*Function*

In VIR there's the same check, but Rustc will complain first, and throw out
some errors about "constrain_type", which ar confusing and the users should not see.
Instead we give an early error with nice error msg here.

```rust
fn check_return_ident(ret_pat: &verus_syn::Pat, input_args: &syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma>) -> Option<verus_syn::Stmt>
```



## verus_builtin_macros::syntax::check_verus_return_ident

*Function*

In VIR there's the same check, but Rustc will complain first, and throw out
some errors about "constrain_type", which ar confusing and the users should not see.
Instead we give an early error with nice error msg here.

```rust
fn check_verus_return_ident(ret_pat: &verus_syn::Pat, input_args: &verus_syn::punctuated::Punctuated<verus_syn::FnArg, verus_syn::token::Comma>) -> Option<verus_syn::Stmt>
```



## verus_builtin_macros::syntax::data_mode_attrs

*Function*

```rust
fn data_mode_attrs(mode: &verus_syn::DataMode) -> Vec<verus_syn::Attribute>
```



## verus_builtin_macros::syntax::declare_mk_rust_attr

*Declarative Macro*

Constructs #[name(tokens)]

```rust
macro_rules! declare_mk_rust_attr {
    ($name:ident, $s:ident) => { ... };
}
```



## verus_builtin_macros::syntax::declare_mk_verus_attr

*Declarative Macro*

Constructs #[verus::internal(tokens)] and #[verifier::tokens]

```rust
macro_rules! declare_mk_verus_attr {
    ($name:ident, $name2:ident, $s:ident) => { ... };
}
```



## verus_builtin_macros::syntax::for_loop_spec_attr

*Function*

```rust
fn for_loop_spec_attr(erase_ghost: crate::EraseGhost, spec_attr: verus_syn::LoopSpec, forloop: syn::ExprForLoop) -> verus_syn::Expr
```



## verus_builtin_macros::syntax::generic_to_tokens

*Function*

```rust
fn generic_to_tokens(generic: &syn::Generics) -> Option<proc_macro2::TokenStream>
```



## verus_builtin_macros::syntax::get_ex_ident_mangle_path

*Function*

```rust
fn get_ex_ident_mangle_path(qself: &Option<verus_syn::QSelf>, path: &verus_syn::Path) -> verus_syn::Ident
```



## verus_builtin_macros::syntax::has_external_code

*Function*

```rust
fn has_external_code(attrs: &Vec<verus_syn::Attribute>) -> bool
```



## verus_builtin_macros::syntax::inputs_to_tokens

*Function*

```rust
fn inputs_to_tokens(inputs: &syn::punctuated::Punctuated<syn::FnArg, $crate::token::Comma>) -> (Option<proc_macro2::TokenStream>, proc_macro2::TokenStream)
```



## verus_builtin_macros::syntax::into_spans

*Function*

```rust
fn into_spans(span: proc_macro2::Span) -> proc_macro2::extra::DelimSpan
```



## verus_builtin_macros::syntax::inv_macro_exprs

*Function*

```rust
fn inv_macro_exprs(erase_ghost: crate::EraseGhost, treat_elements_as_ghost: bool, stream: proc_macro::TokenStream) -> proc_macro::TokenStream
```



## verus_builtin_macros::syntax::is_encoded_const

*Function*

```rust
fn is_encoded_const(attrs: &Vec<verus_syn::Attribute>) -> bool
```



## verus_builtin_macros::syntax::is_external

*Function*

```rust
fn is_external(attrs: &Vec<verus_syn::Attribute>) -> bool
```



## verus_builtin_macros::syntax::is_probably_float_type

*Function*

```rust
fn is_probably_float_type(typ: &verus_syn::Type) -> bool
```



## verus_builtin_macros::syntax::is_probably_nat_or_int_type

*Function*

```rust
fn is_probably_nat_or_int_type(typ: &verus_syn::Type) -> bool
```



## verus_builtin_macros::syntax::is_probably_real_type

*Function*

```rust
fn is_probably_real_type(typ: &verus_syn::Type) -> bool
```



## verus_builtin_macros::syntax::is_ptr_type

*Function*

```rust
fn is_ptr_type(typ: &verus_syn::Type) -> bool
```



## verus_builtin_macros::syntax::mk_rust_attr

*Function*

```rust
fn mk_rust_attr(span: Span, name: &str, tokens: TokenStream) -> verus_syn::Attribute
```



## verus_builtin_macros::syntax::mk_rust_attr_syn

*Function*

```rust
fn mk_rust_attr_syn(span: Span, name: &str, tokens: TokenStream) -> syn::Attribute
```



## verus_builtin_macros::syntax::mk_verifier_attr

*Function*

```rust
fn mk_verifier_attr(span: Span, tokens: TokenStream) -> verus_syn::Attribute
```



## verus_builtin_macros::syntax::mk_verifier_attr_syn

*Function*

```rust
fn mk_verifier_attr_syn(span: Span, tokens: TokenStream) -> syn::Attribute
```



## verus_builtin_macros::syntax::mk_verus_attr

*Function*

```rust
fn mk_verus_attr(span: Span, tokens: TokenStream) -> verus_syn::Attribute
```



## verus_builtin_macros::syntax::mk_verus_attr_syn

*Function*

```rust
fn mk_verus_attr_syn(span: Span, tokens: TokenStream) -> syn::Attribute
```



## verus_builtin_macros::syntax::parse_quote_spanned_builtin

*Declarative Macro*

```rust
macro_rules! parse_quote_spanned_builtin {
    ($b:ident, $span:expr => $($tt:tt)*) => { ... };
}
```



## verus_builtin_macros::syntax::parse_quote_spanned_vstd

*Declarative Macro*

```rust
macro_rules! parse_quote_spanned_vstd {
    ($b:ident, $span:expr => $($tt:tt)*) => { ... };
}
```



## verus_builtin_macros::syntax::path_is_ident

*Function*

```rust
fn path_is_ident(path: &verus_syn::Path, s: &str) -> bool
```



## verus_builtin_macros::syntax::path_matches_idents

*Function*

```rust
fn path_matches_idents(path: &verus_syn::Path, expected: &[&str]) -> bool
```



## verus_builtin_macros::syntax::proof_block

*Function*

```rust
fn proof_block(erase_ghost: crate::EraseGhost, stream: proc_macro::TokenStream) -> proc_macro::TokenStream
```



## verus_builtin_macros::syntax::proof_fn_track_to_type

*Function*

```rust
fn proof_fn_track_to_type(span: proc_macro2::Span, is_tracked: bool) -> verus_syn::Type
```



## verus_builtin_macros::syntax::proof_fn_tracks_to_type

*Function*

```rust
fn proof_fn_tracks_to_type<impl Iterator<Item = bool>>(span: proc_macro2::Span, tracks: impl Trait) -> verus_syn::Type
```



## verus_builtin_macros::syntax::proof_macro_explicit_exprs

*Function*

```rust
fn proof_macro_explicit_exprs(erase_ghost: crate::EraseGhost, inside_ghost: bool, stream: proc_macro::TokenStream) -> proc_macro::TokenStream
```



## verus_builtin_macros::syntax::proof_macro_exprs

*Function*

```rust
fn proof_macro_exprs(erase_ghost: crate::EraseGhost, inside_ghost: bool, stream: proc_macro::TokenStream) -> proc_macro::TokenStream
```



## verus_builtin_macros::syntax::quote_builtin

*Declarative Macro*

```rust
macro_rules! quote_builtin {
    ($b:ident => $($tt:tt)*) => { ... };
}
```



## verus_builtin_macros::syntax::quote_spanned_builtin

*Declarative Macro*

```rust
macro_rules! quote_spanned_builtin {
    ($b:ident, $span:expr => $($tt:tt)*) => { ... };
}
```



## verus_builtin_macros::syntax::quote_spanned_builtin_builtin_macros

*Declarative Macro*

```rust
macro_rules! quote_spanned_builtin_builtin_macros {
    ($b:ident, $m:ident, $span:expr => $($tt:tt)*) => { ... };
}
```



## verus_builtin_macros::syntax::quote_spanned_builtin_builtin_macros_vstd

*Declarative Macro*

```rust
macro_rules! quote_spanned_builtin_builtin_macros_vstd {
    ($b:ident, $m:ident, $v:ident, $span:expr => $($tt:tt)*) => { ... };
}
```



## verus_builtin_macros::syntax::quote_spanned_vstd

*Declarative Macro*

```rust
macro_rules! quote_spanned_vstd {
    ($b:ident, $span:expr => $($tt:tt)*) => { ... };
}
```



## verus_builtin_macros::syntax::quote_verbatim

*Declarative Macro*

```rust
macro_rules! quote_verbatim {
    ($b: ident, $span:expr, $attrs:tt => $($tok:tt)*) => { ... };
    ($span:expr, $attrs:tt => $($tok:tt)*) => { ... };
}
```



## verus_builtin_macros::syntax::quote_vstd

*Declarative Macro*

```rust
macro_rules! quote_vstd {
    ($b:ident => $($tt:tt)*) => { ... };
}
```



## verus_builtin_macros::syntax::rejoin_tokens

*Function*

```rust
fn rejoin_tokens(stream: proc_macro::TokenStream) -> proc_macro::TokenStream
```



## verus_builtin_macros::syntax::rewrite_args_unwrap_ghost_tracked

*Function*

```rust
fn rewrite_args_unwrap_ghost_tracked(erase_ghost: &crate::EraseGhost, arg: & mut verus_syn::FnArg) -> Vec<verus_syn::Stmt>
```



## verus_builtin_macros::syntax::rewrite_exe_pat

*Function*

```rust
fn rewrite_exe_pat(pat: & mut verus_syn::Pat) -> (Vec<verus_syn::Stmt>, Vec<verus_syn::Stmt>)
```



## verus_builtin_macros::syntax::rewrite_expr

*Function*

```rust
fn rewrite_expr(erase_ghost: crate::EraseGhost, inside_ghost: bool, stream: proc_macro::TokenStream) -> proc_macro::TokenStream
```



## verus_builtin_macros::syntax::rewrite_expr_node

*Function*

```rust
fn rewrite_expr_node(erase_ghost: crate::EraseGhost, inside_ghost: bool, expr: & mut verus_syn::Expr)
```



## verus_builtin_macros::syntax::rewrite_impl_items

*Function*

```rust
fn rewrite_impl_items(stream: proc_macro::TokenStream, erase_ghost: crate::EraseGhost, use_spec_traits: bool, for_trait: bool) -> proc_macro::TokenStream
```



## verus_builtin_macros::syntax::rewrite_items

*Function*

```rust
fn rewrite_items(stream: proc_macro::TokenStream, erase_ghost: crate::EraseGhost, use_spec_traits: bool) -> proc_macro::TokenStream
```



## verus_builtin_macros::syntax::rewrite_proof_decl

*Function*

```rust
fn rewrite_proof_decl(erase_ghost: crate::EraseGhost, stream: proc_macro::TokenStream) -> proc_macro::TokenStream
```



## verus_builtin_macros::syntax::sig_specs_attr

*Function*

```rust
fn sig_specs_attr(erase_ghost: crate::EraseGhost, spec_attr: verus_syn::SignatureSpecAttr, sig: & mut syn::Signature, is_impl_fn: bool, is_closure: bool) -> Vec<verus_syn::Stmt>
```



## verus_builtin_macros::syntax::split_off_proof_note_attrs

*Function*

```rust
fn split_off_proof_note_attrs(attrs: Vec<verus_syn::Attribute>) -> (Vec<verus_syn::Attribute>, Vec<verus_syn::Attribute>)
```



## verus_builtin_macros::syntax::stmt_with_semi

*Declarative Macro*

```rust
macro_rules! stmt_with_semi {
    ($b:ident, $span:expr => $($tok:tt)*) => { ... };
    ($span:expr => $($tok:tt)*) => { ... };
}
```



## verus_builtin_macros::syntax::take_expr

*Function*

```rust
fn take_expr(expr: & mut verus_syn::Expr) -> verus_syn::Expr
```



## verus_builtin_macros::syntax::take_ghost

*Function*

```rust
fn take_ghost<T>(erase_ghost: crate::EraseGhost, dest: & mut T) -> T
```



## verus_builtin_macros::syntax::take_pat

*Function*

```rust
fn take_pat(pat: & mut verus_syn::Pat) -> verus_syn::Pat
```



## verus_builtin_macros::syntax::take_sig_with_spec

*Function*

```rust
fn take_sig_with_spec(erase_ghost: crate::EraseGhost, with: verus_syn::WithSpecOnFn, sig: & mut syn::Signature, ret_pat: & mut Option<verus_syn::Pat>) -> Vec<verus_syn::Stmt>
```



## verus_builtin_macros::syntax::take_type

*Function*

```rust
fn take_type(expr: & mut verus_syn::Type) -> verus_syn::Type
```



## verus_builtin_macros::syntax::verus_generic_to_tokens

*Function*

```rust
fn verus_generic_to_tokens(generic: &verus_syn::Generics) -> Option<proc_macro2::TokenStream>
```



## verus_builtin_macros::syntax::verus_inputs_to_tokens

*Function*

```rust
fn verus_inputs_to_tokens(inputs: &verus_syn::punctuated::Punctuated<verus_syn::FnArg, $crate::token::Comma>) -> (Option<proc_macro2::TokenStream>, proc_macro2::TokenStream)
```



## verus_builtin_macros::syntax::while_loop_spec_attr

*Function*

```rust
fn while_loop_spec_attr(erase_ghost: crate::EraseGhost, spec_attr: verus_syn::LoopSpec) -> Vec<verus_syn::Stmt>
```



## verus_builtin_macros::syntax::wrap_expr_with_attrs

*Function*

```rust
fn wrap_expr_with_attrs(expr: verus_syn::Expr, attrs: Vec<verus_syn::Attribute>) -> verus_syn::Expr
```



