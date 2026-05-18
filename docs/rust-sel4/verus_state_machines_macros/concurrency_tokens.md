**verus_state_machines_macros > concurrency_tokens**

# Module: concurrency_tokens

## Contents

**Structs**

- [`Ctxt`](#ctxt) - Context object for the complex task of translating a single
- [`TokenParam`](#tokenparam)

**Enums**

- [`InoutType`](#inouttype)
- [`MainTrait`](#maintrait)
- [`Mode`](#mode)
- [`PrequelElement`](#prequelelement)

**Functions**

- [`add_initialization_input_conditions`](#add_initialization_input_conditions)
- [`add_initialization_output_conditions`](#add_initialization_output_conditions)
- [`add_token_param_in_out`](#add_token_param_in_out)
- [`asserts_to_single_predicate`](#asserts_to_single_predicate)
- [`assign_pat_or_arbitrary`](#assign_pat_or_arbitrary)
- [`bool_not_expr`](#bool_not_expr)
- [`const_fn_stream`](#const_fn_stream) - For a given sharding(constant) field, add that constant
- [`determine_outputs`](#determine_outputs)
- [`exchange_collect`](#exchange_collect)
- [`exchange_name`](#exchange_name)
- [`exchange_stream`](#exchange_stream) - Primary method to build an exchange method for a given transition.
- [`field_token_collection_type`](#field_token_collection_type)
- [`field_token_type`](#field_token_type)
- [`field_token_type_name`](#field_token_type_name)
- [`get_all_lemmas_for_transition`](#get_all_lemmas_for_transition) - Gets the lemmas that prove validity of the given transition.
- [`get_const_field_value`](#get_const_field_value)
- [`get_extra_deps`](#get_extra_deps)
- [`get_init_param_input_type`](#get_init_param_input_type)
- [`get_init_param_output_type`](#get_init_param_output_type)
- [`get_inst_object_value`](#get_inst_object_value)
- [`get_inst_value`](#get_inst_value)
- [`get_main_lemma_for_transition_opt`](#get_main_lemma_for_transition_opt)
- [`get_new_field_value`](#get_new_field_value)
- [`get_nondeterministic_out_value`](#get_nondeterministic_out_value)
- [`get_old_field_value`](#get_old_field_value)
- [`get_post_value_for_variable`](#get_post_value_for_variable) - Get the expression E that a given field gets updated to.
- [`get_storage_type_tuple`](#get_storage_type_tuple) - If the SM has storage types T1, T2, ...
- [`inst_type`](#inst_type)
- [`inst_type_name`](#inst_type_name)
- [`instance_struct_stream`](#instance_struct_stream) - Print declaration for the Instance type.
- [`mk_and`](#mk_and)
- [`mk_eq`](#mk_eq)
- [`nondeterministic_read_spec_out_name`](#nondeterministic_read_spec_out_name)
- [`output_token_types_and_fns`](#output_token_types_and_fns)
- [`prune_irrelevant_ops`](#prune_irrelevant_ops) - Prune out ops that update a NotTokenized field.
- [`prune_irrelevant_ops_rec`](#prune_irrelevant_ops_rec)
- [`relation_for_collection_of_internal_tokens`](#relation_for_collection_of_internal_tokens)
- [`stored_object_type`](#stored_object_type)
- [`token_matches_elt`](#token_matches_elt)
- [`token_struct_stream`](#token_struct_stream) - Create the struct for a Token.
- [`token_trait_impl_main`](#token_trait_impl_main)
- [`token_trait_impls`](#token_trait_impls)
- [`traits_stream`](#traits_stream) - Add some helper functions that are useful to call from other
- [`transition_arg_name`](#transition_arg_name)
- [`translate_elt`](#translate_elt)
- [`translate_expr`](#translate_expr) - Perform translation on the Rust AST Exprs.
- [`translate_special_assignment`](#translate_special_assignment)
- [`translate_special_condition`](#translate_special_condition)
- [`translate_split_kind`](#translate_split_kind)
- [`translate_transition`](#translate_transition) - The function has several purposes:
- [`translate_value_expr`](#translate_value_expr) - Gets the "value" expression from the MonoidElt
- [`trusted_clone`](#trusted_clone) - Add a `clone()` method to the Instance type (or any other type which is cloneable).
- [`trusted_copy`](#trusted_copy)
- [`with_prequel`](#with_prequel)

---

## verus_state_machines_macros::concurrency_tokens::Ctxt

*Struct*

Context object for the complex task of translating a single
transition into a token-exchange method.

**Fields:**
- `fields_written: std::collections::HashSet<String>`
- `fields_read: std::collections::HashSet<String>`
- `fields_read_birds_eye: std::collections::HashSet<String>`
- `params: std::collections::HashMap<String, Vec<TokenParam>>`
- `requires: Vec<verus_syn::Expr>`
- `ensures: Vec<verus_syn::Expr>`
- `ident_to_field: std::collections::HashMap<String, crate::ast::Field>`
- `is_init: bool`
- `sm: crate::ast::SM`
- `fresh_num_counter: usize`

**Methods:**

- `fn get_field_or_panic(self: &Self, ident: &Ident) -> Field`
- `fn get_numbered_token_ident(self: & mut Self, base_id: &Ident) -> Ident`
- `fn get_explicit_lifetime(self: &Self) -> bool` - Determines if we need to add an explicit lifetime parameter



## verus_state_machines_macros::concurrency_tokens::InoutType

*Enum*

**Variants:**
- `In`
- `Out`
- `InOut`
- `BorrowIn`
- `BorrowOut`

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &InoutType) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> InoutType`



## verus_state_machines_macros::concurrency_tokens::MainTrait

*Enum*

**Generic Parameters:**
- 'a

**Variants:**
- `Simple`
- `Value(&'a verus_syn::Type)`
- `KeyValue(&'a verus_syn::Type, &'a verus_syn::Type)`
- `Element(&'a verus_syn::Type)`
- `Count`
- `MonotonicCount`



## verus_state_machines_macros::concurrency_tokens::Mode

*Enum*

**Variants:**
- `Ghost`
- `Tracked`



## verus_state_machines_macros::concurrency_tokens::PrequelElement

*Enum*

**Variants:**
- `AssertCondition(verus_syn::Expr)`
- `Condition(verus_syn::Expr)`
- `Let(Box<verus_syn::Pat>, Option<Box<verus_syn::Type>>, verus_syn::Expr)`
- `Match(verus_syn::Expr, Vec<crate::ast::Arm>, usize)`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> PrequelElement`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## verus_state_machines_macros::concurrency_tokens::TokenParam

*Struct*

**Fields:**
- `inout_type: InoutType`
- `name: verus_syn::Ident`
- `ty: verus_syn::Type`
- `is_collection: bool` - Is this param a collection (option, map, multiset) of tokens



## verus_state_machines_macros::concurrency_tokens::add_initialization_input_conditions

*Function*

```rust
fn add_initialization_input_conditions(field: &crate::ast::Field, requires: & mut Vec<verus_syn::Expr>, init_value: verus_syn::Expr, param_value: verus_syn::Expr)
```



## verus_state_machines_macros::concurrency_tokens::add_initialization_output_conditions

*Function*

```rust
fn add_initialization_output_conditions(sm: &crate::ast::SM, field: &crate::ast::Field, ensures: & mut Vec<verus_syn::Expr>, inst_eq_enss: & mut Vec<verus_syn::Expr>, init_value: verus_syn::Expr, inst_value: verus_syn::Expr, param_value: verus_syn::Expr)
```



## verus_state_machines_macros::concurrency_tokens::add_token_param_in_out

*Function*

```rust
fn add_token_param_in_out(ctxt: &Ctxt, in_params: & mut Vec<proc_macro2::TokenStream>, out_params: & mut Vec<(proc_macro2::TokenStream, proc_macro2::TokenStream, Mode)>, inst_eq_enss: & mut Vec<verus_syn::Expr>, inst_eq_reqs: & mut Vec<verus_syn::Expr>, param_name: &verus_syn::Ident, param_type: &verus_syn::Type, inout_type: InoutType, apply_instance_condition: bool, use_explicit_lifetime: bool)
```



## verus_state_machines_macros::concurrency_tokens::asserts_to_single_predicate

*Function*

```rust
fn asserts_to_single_predicate(ts: &crate::ast::TransitionStmt) -> Option<proc_macro2::TokenStream>
```



## verus_state_machines_macros::concurrency_tokens::assign_pat_or_arbitrary

*Function*

```rust
fn assign_pat_or_arbitrary(pat: &verus_syn::Pat, init_e: &verus_syn::Expr) -> Option<(verus_syn::Pat, verus_syn::Expr)>
```



## verus_state_machines_macros::concurrency_tokens::bool_not_expr

*Function*

```rust
fn bool_not_expr(e: &verus_syn::Expr) -> verus_syn::Expr
```



## verus_state_machines_macros::concurrency_tokens::const_fn_stream

*Function*

For a given sharding(constant) field, add that constant
as a #[verifier::spec] fn on the Instance type. (The field is constant
for the entire instance.)

```rust
fn const_fn_stream(field: &crate::ast::Field) -> proc_macro2::TokenStream
```



## verus_state_machines_macros::concurrency_tokens::determine_outputs

*Function*

```rust
fn determine_outputs(ctxt: & mut Ctxt, ts: &crate::ast::TransitionStmt) -> parse::Result<()>
```



## verus_state_machines_macros::concurrency_tokens::exchange_collect

*Function*

```rust
fn exchange_collect(ctxt: & mut Ctxt, ts: &crate::ast::TransitionStmt, prequel: Vec<PrequelElement>) -> parse::Result<Vec<PrequelElement>>
```



## verus_state_machines_macros::concurrency_tokens::exchange_name

*Function*

```rust
fn exchange_name(tr: &crate::ast::Transition) -> verus_syn::Ident
```



## verus_state_machines_macros::concurrency_tokens::exchange_stream

*Function*

Primary method to build an exchange method for a given transition.

To build an exchange method, we need to collect 4 key pieces
of information:

 * input arguments (may include spec & proof tokens)
 * outputs (may include spec & proof tokens)
 * pre-conditions (mostly follow from 'require' statements)
 * post-conditions (mostly follow from 'update' and 'assert' statements)

When possible, we use &mut arguments (i.e., if a given token is
both an input and an output).

```rust
fn exchange_stream(bundle: &crate::parse_token_stream::SMBundle, tr: &crate::ast::Transition, safety_condition_lemmas: &std::collections::HashMap<String, verus_syn::Ident>) -> parse::Result<proc_macro2::TokenStream>
```



## verus_state_machines_macros::concurrency_tokens::field_token_collection_type

*Function*

```rust
fn field_token_collection_type(sm: &crate::ast::SM, field: &crate::ast::Field) -> verus_syn::Type
```



## verus_state_machines_macros::concurrency_tokens::field_token_type

*Function*

```rust
fn field_token_type(sm: &crate::ast::SM, field: &crate::ast::Field) -> verus_syn::Type
```



## verus_state_machines_macros::concurrency_tokens::field_token_type_name

*Function*

```rust
fn field_token_type_name(field: &crate::ast::Field) -> verus_syn::Ident
```



## verus_state_machines_macros::concurrency_tokens::get_all_lemmas_for_transition

*Function*

Gets the lemmas that prove validity of the given transition.

```rust
fn get_all_lemmas_for_transition(bundle: &crate::parse_token_stream::SMBundle, trans: &crate::ast::Transition, safety_condition_lemmas: &std::collections::HashMap<String, verus_syn::Ident>) -> Vec<verus_syn::Ident>
```



## verus_state_machines_macros::concurrency_tokens::get_const_field_value

*Function*

```rust
fn get_const_field_value(ctxt: &Ctxt, field: &crate::ast::Field, span: proc_macro2::Span) -> verus_syn::Expr
```



## verus_state_machines_macros::concurrency_tokens::get_extra_deps

*Function*

```rust
fn get_extra_deps(bundle: &crate::parse_token_stream::SMBundle, trans: &crate::ast::Transition, safety_condition_lemmas: &std::collections::HashMap<String, verus_syn::Ident>) -> proc_macro2::TokenStream
```



## verus_state_machines_macros::concurrency_tokens::get_init_param_input_type

*Function*

```rust
fn get_init_param_input_type(_sm: &crate::ast::SM, field: &crate::ast::Field) -> Option<verus_syn::Type>
```



## verus_state_machines_macros::concurrency_tokens::get_init_param_output_type

*Function*

```rust
fn get_init_param_output_type(sm: &crate::ast::SM, field: &crate::ast::Field) -> Option<verus_syn::Type>
```



## verus_state_machines_macros::concurrency_tokens::get_inst_object_value

*Function*

```rust
fn get_inst_object_value(ctxt: &Ctxt) -> verus_syn::Expr
```



## verus_state_machines_macros::concurrency_tokens::get_inst_value

*Function*

```rust
fn get_inst_value(ctxt: &Ctxt) -> verus_syn::Expr
```



## verus_state_machines_macros::concurrency_tokens::get_main_lemma_for_transition_opt

*Function*

```rust
fn get_main_lemma_for_transition_opt<'a>(lemmas: &'a Vec<crate::ast::Lemma>, trans_name: &verus_syn::Ident) -> Option<&'a crate::ast::Lemma>
```



## verus_state_machines_macros::concurrency_tokens::get_new_field_value

*Function*

```rust
fn get_new_field_value(field: &crate::ast::Field) -> verus_syn::Expr
```



## verus_state_machines_macros::concurrency_tokens::get_nondeterministic_out_value

*Function*

```rust
fn get_nondeterministic_out_value(_ctxt: &Ctxt, field: &crate::ast::Field, span: proc_macro2::Span) -> verus_syn::Expr
```



## verus_state_machines_macros::concurrency_tokens::get_old_field_value

*Function*

```rust
fn get_old_field_value(ctxt: &Ctxt, field: &crate::ast::Field, span: proc_macro2::Span) -> verus_syn::Expr
```



## verus_state_machines_macros::concurrency_tokens::get_post_value_for_variable

*Function*

Get the expression E that a given field gets updated to.
This is used in the post-condition of the exchange method.
Since the 'update' might be inside a conditional, we may need to build
a conditional expression.

Returns 'None' if it doesn't find an 'update' for the given variable.

Also handles 'init' statements in the same way as 'update' statements,
so it can be used for initialization as well.

Ignores all special ops.

```rust
fn get_post_value_for_variable(ctxt: &Ctxt, ts: &crate::ast::TransitionStmt, field: &crate::ast::Field) -> Option<verus_syn::Expr>
```



## verus_state_machines_macros::concurrency_tokens::get_storage_type_tuple

*Function*

If the SM has storage types T1, T2, ...
this returns the tuple type (T1, T2, ...).
Uses the "inner" type (e.g., the X in Option<X>, not Option<X> itself)

```rust
fn get_storage_type_tuple(sm: &crate::ast::SM) -> verus_syn::Type
```



## verus_state_machines_macros::concurrency_tokens::inst_type

*Function*

```rust
fn inst_type(sm: &crate::ast::SM) -> verus_syn::Type
```



## verus_state_machines_macros::concurrency_tokens::inst_type_name

*Function*

```rust
fn inst_type_name(sm_name: &verus_syn::Ident) -> verus_syn::Ident
```



## verus_state_machines_macros::concurrency_tokens::instance_struct_stream

*Function*

Print declaration for the Instance type.

From the user's perspective, this should just be an opaque, unforgeable token type
that serves as an identifier for a protocol instance and contains the constants
for the instance.

Formally, this token should be derived in terms of a monoid derived from the state
machine state. (See `monoid-formalism.md`.) Since that formalism is not
implemented mechanically
in Verus, we instead make the state itself a field. This is kind of meaningless, but it
serves as a placeholder and does result in the necessary dependency edge between this
struct and the type struct. The fields are private, so they shouldn't matter to the user.

Another thing is that we need the `Instance` struct to have the correct Send/Sync traits.
(All the other token types will then inherit those traits. The Senc/Synd traits of
the Instance should depend on the Sync/Send traits of the storage types.
In particular, the Instance should be Sync+Send iff the storage types are all Sync+Send.
Otherwise, the Instance type should have neither.
Initially, this rule might seem a little extra-restrictive, but we need this restriction
because if it is possible for the instance to interact with another thread at all, it
could conceivably do arbitrary withdraw/deposits (thus 'sending' the storage objects
to the other thread) or arbitrary guards (thus 'syncing' the storage objects to the
other thread). Due to the flexibility of the guard protocol, this could theoretically
happen regardless of whether the instance/tokens are synced or sent to the other thread.

```rust
fn instance_struct_stream(sm: &crate::ast::SM) -> proc_macro2::TokenStream
```



## verus_state_machines_macros::concurrency_tokens::mk_and

*Function*

```rust
fn mk_and(span: proc_macro2::Span, lhs: verus_syn::Expr, rhs: verus_syn::Expr) -> verus_syn::Expr
```



## verus_state_machines_macros::concurrency_tokens::mk_eq

*Function*

```rust
fn mk_eq(span: proc_macro2::Span, lhs: &verus_syn::Expr, rhs: &verus_syn::Expr) -> verus_syn::Expr
```



## verus_state_machines_macros::concurrency_tokens::nondeterministic_read_spec_out_name

*Function*

```rust
fn nondeterministic_read_spec_out_name(field: &crate::ast::Field) -> verus_syn::Ident
```



## verus_state_machines_macros::concurrency_tokens::output_token_types_and_fns

*Function*

```rust
fn output_token_types_and_fns(token_stream: & mut proc_macro2::TokenStream, bundle: &crate::parse_token_stream::SMBundle, safety_condition_lemmas: &std::collections::HashMap<String, verus_syn::Ident>) -> parse::Result<()>
```



## verus_state_machines_macros::concurrency_tokens::prune_irrelevant_ops

*Function*

Prune out ops that update a NotTokenized field.
The resulting transition is, of course, not equivalent to the original in the
sense of its action on the global state, but for the purpose of the exchange method,
it doesn't matter. Updates to a NotTokenized field aren't observed by the exchange method.
(Also, if there's just any dead code for any other reason, that will also get pruned
as a byproduct.)

```rust
fn prune_irrelevant_ops(ctxt: &Ctxt, ts: crate::ast::TransitionStmt) -> crate::ast::TransitionStmt
```



## verus_state_machines_macros::concurrency_tokens::prune_irrelevant_ops_rec

*Function*

```rust
fn prune_irrelevant_ops_rec(ctxt: &Ctxt, ts: crate::ast::TransitionStmt) -> Option<crate::ast::TransitionStmt>
```



## verus_state_machines_macros::concurrency_tokens::relation_for_collection_of_internal_tokens

*Function*

```rust
fn relation_for_collection_of_internal_tokens(_sm: &crate::ast::SM, field: &crate::ast::Field, param_value: verus_syn::Expr, given_value: verus_syn::Expr, inst_value: verus_syn::Expr, output: bool, span: proc_macro2::Span) -> verus_syn::Expr
```



## verus_state_machines_macros::concurrency_tokens::stored_object_type

*Function*

```rust
fn stored_object_type(field: &crate::ast::Field) -> verus_syn::Type
```



## verus_state_machines_macros::concurrency_tokens::token_matches_elt

*Function*

```rust
fn token_matches_elt(span: proc_macro2::Span, token_is_ref: bool, token_name: &verus_syn::Ident, elt: &crate::ast::MonoidElt, pat_opt: &Option<Box<verus_syn::Pat>>, ctxt: &Ctxt, field: &crate::ast::Field, output: bool) -> verus_syn::Expr
```



## verus_state_machines_macros::concurrency_tokens::token_struct_stream

*Function*

Create the struct for a Token.
Can create any combination of three fields: key, value, count.
The `count` field, when present, always has type `nat`;
for the other two, when present, the types are provided as arguments.

In general the 'key' field is used when something is unique
And the 'count' is for nats that are additive.

For example:
  'option' only has a value
  'map' strategy has a key and a value
  'set' only has a key,
  'count' only has a count field
  'multiset' has key and count

The 'multiset' is kind of tricky but the idea is that a Multiset<V>
is basically a Map<V, nat>. So the V is treated as a 'key' here
and if you have multiple elements of the same value, that's represented
by having a higher counter.

```rust
fn token_struct_stream(sm: &crate::ast::SM, field: &crate::ast::Field) -> proc_macro2::TokenStream
```



## verus_state_machines_macros::concurrency_tokens::token_trait_impl_main

*Function*

```rust
fn token_trait_impl_main(token_ty: &verus_syn::Type, generics: &Option<verus_syn::Generics>, main_trait: &MainTrait) -> proc_macro2::TokenStream
```



## verus_state_machines_macros::concurrency_tokens::token_trait_impls

*Function*

```rust
fn token_trait_impls(token_ty: &verus_syn::Type, generics: &Option<verus_syn::Generics>, main_trait: MainTrait, unique: bool) -> proc_macro2::TokenStream
```



## verus_state_machines_macros::concurrency_tokens::traits_stream

*Function*

Add some helper functions that are useful to call from other
generated conditions (e.g., see `add_initialization_output_conditions`)

```rust
fn traits_stream(sm: &crate::ast::SM, field: &crate::ast::Field) -> proc_macro2::TokenStream
```



## verus_state_machines_macros::concurrency_tokens::transition_arg_name

*Function*

```rust
fn transition_arg_name(field: &crate::ast::Field) -> verus_syn::Ident
```



## verus_state_machines_macros::concurrency_tokens::translate_elt

*Function*

```rust
fn translate_elt(ctxt: &Ctxt, elt: &crate::ast::MonoidElt, birds_eye: bool, errors: & mut Vec<verus_syn::parse::Error>) -> crate::ast::MonoidElt
```



## verus_state_machines_macros::concurrency_tokens::translate_expr

*Function*

Perform translation on the Rust AST Exprs.
We need to find expressions like `pre.foo` that refer to individual fields and replace
them with some expression in terms of the parameters of the exchange function.
There are a few ways this can happen, depending on the context:

 * Reading a 'Constant' field. In this case, we replace it with the constant value
   from the instance object, e.g., `instance.foo()`.

 * Reading a 'Variable' field. Replace it with the value from the input token
   e.g., `token_foo.value()` or `old(token_foo).value()`.

 * A nondeterministic read. In this case, we replace it with the value of one of the
   _out_ parameters. Previous well-formedness checks (`check_birds_eye`) should ensure
   that this will only happen in contexts that will ultimately appears in the
   post-conditions, never pre-conditions.

```rust
fn translate_expr(ctxt: &Ctxt, expr: &verus_syn::Expr, birds_eye: bool, errors: & mut Vec<verus_syn::parse::Error>) -> verus_syn::Expr
```



## verus_state_machines_macros::concurrency_tokens::translate_special_assignment

*Function*

```rust
fn translate_special_assignment(op: &crate::ast::SpecialOp, param: &TokenParam) -> verus_syn::Expr
```



## verus_state_machines_macros::concurrency_tokens::translate_special_condition

*Function*

```rust
fn translate_special_condition(ctxt: & mut Ctxt, span: proc_macro2::Span, field: &crate::ast::Field, op: &crate::ast::SpecialOp, pat_opt: &Option<Box<verus_syn::Pat>>, errors: & mut Vec<verus_syn::parse::Error>) -> Option<crate::ast::TransitionStmt>
```



## verus_state_machines_macros::concurrency_tokens::translate_split_kind

*Function*

```rust
fn translate_split_kind(ctxt: & mut Ctxt, sk: & mut crate::ast::SplitKind, errors: & mut Vec<verus_syn::parse::Error>)
```



## verus_state_machines_macros::concurrency_tokens::translate_transition

*Function*

The function has several purposes:

  1. Replace `pre.foo` subexpression with the corresponding value from the
     input tokens, e.g., `token_foo.value`
  2. Expand the definitions for any SpecialOps.

For point (1), we call `translate_expr` on each Rust Expr found in the transition AST.

For point (2), we translate all the SpecialOp into Require and PostCondition statements.
The Require statements, of course, will become pre-conditions on the exchange method,
while the PostCondition statements become post-conditions (and thus they can refer to
the out-params). Also, when processing the SpecialOps, we determine what params we need
to add to the exchange method (e.g., a 'remove' statement corresponds to an in-param,
an 'add' statement corresponds to an out-param, and so on). We collect that information
in `ctxt.params`.

Also note that the translation looks slightly different than the translation done in
`simplification.rs` which is used for the relations describing the atomic transitions.
The reason for that is that the translation of `simplification.rs` often creates
conditions that refer to the entire state (e.g., the safety conditions for 'add').
On the other hand, the conditions we produce here must refer to local state (although
they must also _imply_ the enabling conditions of the atomic transition relation).
Further, we do not create new 'Assert' statements here at all. If the user wants
the exchange postcondition to contain any additional predicates, they can always
add an 'assert' explicitly.

```rust
fn translate_transition(ctxt: & mut Ctxt, ts: & mut crate::ast::TransitionStmt, errors: & mut Vec<verus_syn::parse::Error>) -> parse::Result<()>
```



## verus_state_machines_macros::concurrency_tokens::translate_value_expr

*Function*

Gets the "value" expression from the MonoidElt
(whichever corresponds to the 'value' field of the token struct).
(For a map element, it's the value of the key-value pair;
for anything else, where there's only expression, it's that one.)

```rust
fn translate_value_expr(ctxt: &Ctxt, elt: &crate::ast::MonoidElt, birds_eye: bool, errors: & mut Vec<verus_syn::parse::Error>) -> Option<verus_syn::Expr>
```



## verus_state_machines_macros::concurrency_tokens::trusted_clone

*Function*

Add a `clone()` method to the Instance type (or any other type which is cloneable).
This is safe, because the Instance object effectively just represents

  * the fact that the protocol exists, and has been initialized
  * any constants associated to the protocol

TODO it would be even better to add a Copy instance as well; however, this
currently runs into Verus limitations with deriving instances.

```rust
fn trusted_clone() -> proc_macro2::TokenStream
```



## verus_state_machines_macros::concurrency_tokens::trusted_copy

*Function*

```rust
fn trusted_copy(self_ty: &verus_syn::Type, generics: &Option<verus_syn::Generics>) -> proc_macro2::TokenStream
```



## verus_state_machines_macros::concurrency_tokens::with_prequel

*Function*

```rust
fn with_prequel(pre: &Vec<PrequelElement>, include_assert_conditions: bool, e: verus_syn::Expr) -> verus_syn::Expr
```



