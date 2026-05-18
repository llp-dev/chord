**verus_state_machines_macros > lemmas**

# Module: lemmas

## Contents

**Functions**

- [`check_each_lemma_valid`](#check_each_lemma_valid) - Check that each lemma is valid by making sure it has the right arguments.
- [`check_lemmas`](#check_lemmas) - Check that the declarations of 'inductive' lemmas are well-formed.
- [`check_lemmas_cover_all_cases`](#check_lemmas_cover_all_cases) - Check that every transition has a corresponding 'inductive' lemma.
- [`check_no_explicit_conditions`](#check_no_explicit_conditions) - Error if the user tried to add 'requires' or 'ensures' to an inductiveness lemma.
- [`get_expected_params`](#get_expected_params) - For the lemma about an 'init' routine,
- [`get_transition`](#get_transition)
- [`params_match`](#params_match) - If the params match, return None.
- [`params_to_string`](#params_to_string)
- [`pat_is_ident`](#pat_is_ident) - Check if the `pat` is for the given ident, with no extra stuff.
- [`transition_params_to_string`](#transition_params_to_string)
- [`ty_to_string`](#ty_to_string)

---

## verus_state_machines_macros::lemmas::check_each_lemma_valid

*Function*

Check that each lemma is valid by making sure it has the right arguments.
They should match token-by-token (since at this point we are incapable of more complex
type analysis) and be named the same.

Naturally, in the process, we check that each lemma actually names a transition
that exists. We also check that there are no duplicate lemmas.

Make sure the error message is helpful. On error, just tell the user exactly
what params they can copy-paste in.

```rust
fn check_each_lemma_valid(bundle: &crate::parse_token_stream::SMBundle) -> parse::Result<()>
```



## verus_state_machines_macros::lemmas::check_lemmas

*Function*

Check that the declarations of 'inductive' lemmas are well-formed.

```rust
fn check_lemmas(bundle: &crate::parse_token_stream::SMBundle) -> parse::Result<()>
```



## verus_state_machines_macros::lemmas::check_lemmas_cover_all_cases

*Function*

Check that every transition has a corresponding 'inductive' lemma.
On error, print out a list of stubs that the user can directly copy-paste into their source.

```rust
fn check_lemmas_cover_all_cases(bundle: &crate::parse_token_stream::SMBundle) -> parse::Result<()>
```



## verus_state_machines_macros::lemmas::check_no_explicit_conditions

*Function*

Error if the user tried to add 'requires' or 'ensures' to an inductiveness lemma.

```rust
fn check_no_explicit_conditions(bundle: &crate::parse_token_stream::SMBundle) -> parse::Result<()>
```



## verus_state_machines_macros::lemmas::get_expected_params

*Function*

For the lemma about an 'init' routine,
we expect params: `post: X, ...` where `...` are the transition params and X is the Self type.
For a 'transition' routine,
we expect params: `pre: X, post: X, ...`

```rust
fn get_expected_params(t: &crate::ast::Transition) -> Vec<crate::ast::TransitionParam>
```



## verus_state_machines_macros::lemmas::get_transition

*Function*

```rust
fn get_transition<'a>(transitions: &'a Vec<crate::ast::Transition>, name: &String) -> Option<&'a crate::ast::Transition>
```



## verus_state_machines_macros::lemmas::params_match

*Function*

If the params match, return None.
If not, return a span to error at. Pick the earliest span where a discrepancy is found.

```rust
fn params_match(expected: &Vec<crate::ast::TransitionParam>, actual: &verus_syn::punctuated::Punctuated<verus_syn::FnArg, token::Comma>, sig_span: proc_macro2::Span) -> Option<proc_macro2::Span>
```



## verus_state_machines_macros::lemmas::params_to_string

*Function*

```rust
fn params_to_string(params: &Vec<crate::ast::TransitionParam>) -> String
```



## verus_state_machines_macros::lemmas::pat_is_ident

*Function*

Check if the `pat` is for the given ident, with no extra stuff.

```rust
fn pat_is_ident(pat: &verus_syn::Pat, ident: &verus_syn::Ident) -> bool
```



## verus_state_machines_macros::lemmas::transition_params_to_string

*Function*

```rust
fn transition_params_to_string(is_init: bool, params: &Vec<crate::ast::TransitionParam>) -> String
```



## verus_state_machines_macros::lemmas::ty_to_string

*Function*

```rust
fn ty_to_string(ty: &verus_syn::Type) -> String
```



