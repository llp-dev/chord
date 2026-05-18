**verus_state_machines_macros > util**

# Module: util

## Contents

**Functions**

- [`combine_errors_or_ok`](#combine_errors_or_ok)
- [`combine_results`](#combine_results)
- [`expr_from_tokens`](#expr_from_tokens)
- [`is_definitely_irrefutable`](#is_definitely_irrefutable)
- [`pat_from_tokens`](#pat_from_tokens)

---

## verus_state_machines_macros::util::combine_errors_or_ok

*Function*

```rust
fn combine_errors_or_ok(errors: Vec<verus_syn::Error>) -> parse::Result<()>
```



## verus_state_machines_macros::util::combine_results

*Function*

```rust
fn combine_results(errors: Vec<parse::Result<()>>) -> parse::Result<()>
```



## verus_state_machines_macros::util::expr_from_tokens

*Function*

```rust
fn expr_from_tokens(t: proc_macro2::TokenStream) -> verus_syn::Expr
```



## verus_state_machines_macros::util::is_definitely_irrefutable

*Function*

```rust
fn is_definitely_irrefutable(pat: &verus_syn::Pat) -> bool
```



## verus_state_machines_macros::util::pat_from_tokens

*Function*

```rust
fn pat_from_tokens(t: proc_macro2::TokenStream) -> verus_syn::Pat
```



