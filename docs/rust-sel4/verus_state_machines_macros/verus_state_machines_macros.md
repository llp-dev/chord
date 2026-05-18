**verus_state_machines_macros**

# Module: verus_state_machines_macros

## Contents

**Modules**

- [`ast`](#ast)
- [`case_macro`](#case_macro)
- [`check_bind_stmts`](#check_bind_stmts)
- [`check_birds_eye`](#check_birds_eye)
- [`concurrency_tokens`](#concurrency_tokens) - Output all the generated code specific to tokenized state machines.
- [`field_access_visitor`](#field_access_visitor) - This the module with utilities for processing a Rust Expr.
- [`ident_visitor`](#ident_visitor)
- [`inherent_safety_conditions`](#inherent_safety_conditions)
- [`lemmas`](#lemmas)
- [`parse_token_stream`](#parse_token_stream) - Module for the initial processing of the macro tokens, to return an SM AST
- [`parse_transition`](#parse_transition)
- [`safety_conditions`](#safety_conditions)
- [`self_type_visitor`](#self_type_visitor)
- [`simplification`](#simplification)
- [`simplify_asserts`](#simplify_asserts)
- [`to_relation`](#to_relation)
- [`to_token_stream`](#to_token_stream) - Primary module for outputting the generated code.
- [`token_transition_checks`](#token_transition_checks) - More sanity checks on transitions, checking properties specifically for
- [`transitions`](#transitions)
- [`util`](#util)
- [`vstd_path`](#vstd_path)

**Proc Macros**

- [`case_on_init`](#case_on_init)
- [`case_on_next`](#case_on_next)
- [`case_on_next_strong`](#case_on_next_strong)
- [`state_machine`](#state_machine)
- [`tokenized_state_machine`](#tokenized_state_machine)
- [`tokenized_state_machine_vstd`](#tokenized_state_machine_vstd)

**Functions**

- [`cfg_verify_core`](#cfg_verify_core)
- [`construct_state_machine`](#construct_state_machine)

---

## Module: ast



## Module: case_macro



## verus_state_machines_macros::case_on_init

*Function-like Macro*

```rust
case_on_init!(...)
```



## verus_state_machines_macros::case_on_next

*Function-like Macro*

```rust
case_on_next!(...)
```



## verus_state_machines_macros::case_on_next_strong

*Function-like Macro*

```rust
case_on_next_strong!(...)
```



## verus_state_machines_macros::cfg_verify_core

*Function*

```rust
fn cfg_verify_core() -> bool
```



## Module: check_bind_stmts



## Module: check_birds_eye



## Module: concurrency_tokens

Output all the generated code specific to tokenized state machines.
We print declarations for:

 * The Instance type
 * All the Token types for shardable fields
 * Proof methods for each transition (including init and readonly transitions)



## verus_state_machines_macros::construct_state_machine

*Function*

```rust
fn construct_state_machine(input: proc_macro::TokenStream, concurrent: bool) -> proc_macro::TokenStream
```



## Module: field_access_visitor

This the module with utilities for processing a Rust Expr.
Formally, the codegen for the token exchange methods needs to:

 1. Look for all `pre.field` subexpressions to determine which fields are read.
 2. Perform substitutions of the `pre.field` subexpressions for other expressions
    we construct.

Unfortunately, attempting to treat a Rust Expr as anything other than a completely
opaque expression comes with a variety of technical challenges, which have to do with
the fact that this has to run before type-resolution and even before macro-expansion.

In order to ensure this transformation is done correctly, we need to:

  * Make sure that reserved identifiers like `token_foo` are not shadowed in the expressions
  * Disallow macros entirely, which could interfere in a number of ways.
    (Note: there seems to be an experimental API for expanding macros inside proc-macros,
    which we could look into to possibly remove this restriction!)

Both these things are done in ident_visitor.rs.

This is all very awkward, and it's also hard to be sure we've really handled every
case. The awkwardness here suggests that it would be more principled to do this
in VIR, or with VIR support. Unfortunately, this plan has its own problems: namely,
the type signatures we generate (namely the input tokens) actually depend on the
results of analysis (1).

If this becomes a problem, there a few things we could do. We could avoid
the need for analysis (1) by requiring the user to be more explicit about which fields
are read. Then we could move the trickiest parts of the analysis into VIR, or at least
use VIR to help us enforce extra constraints we need to hold. However, I intend
to experiment with the current method for now, since generating all the conditions
in the macro has a lot of advantages for usability.



## Module: ident_visitor



## Module: inherent_safety_conditions



## Module: lemmas



## Module: parse_token_stream

Module for the initial processing of the macro tokens, to return an SM AST



## Module: parse_transition



## Module: safety_conditions



## Module: self_type_visitor



## Module: simplification



## Module: simplify_asserts



## verus_state_machines_macros::state_machine

*Function-like Macro*

```rust
state_machine!(...)
```



## Module: to_relation



## Module: to_token_stream

Primary module for outputting the generated code.
This includes: the primary struct, the transition definitions,
invariant predicates, lemmas that prove inductiveness, and lemmas
that prove safety conditions (as given by the 'assert' statements).

Concurrent-state-machine-specific stuff is in concurrency_tokens.rs



## Module: token_transition_checks

More sanity checks on transitions, checking properties specifically for
concurrency_tokens.rs



## verus_state_machines_macros::tokenized_state_machine

*Function-like Macro*

```rust
tokenized_state_machine!(...)
```



## verus_state_machines_macros::tokenized_state_machine_vstd

*Function-like Macro*

```rust
tokenized_state_machine_vstd!(...)
```



## Module: transitions



## Module: util



## Module: vstd_path



