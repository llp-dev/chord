**verus_state_machines_macros > case_macro**

# Module: case_macro

## Contents

**Structs**

- [`Arm`](#arm)
- [`MatchPost`](#matchpost)
- [`MatchPrePost`](#matchprepost)

**Functions**

- [`case_on`](#case_on)
- [`parse_arm`](#parse_arm)

---

## verus_state_machines_macros::case_macro::Arm

*Struct*

**Fields:**
- `step_name: verus_syn::Ident`
- `params: Vec<verus_syn::Ident>`
- `block: verus_syn::ExprBlock`



## verus_state_machines_macros::case_macro::MatchPost

*Struct*

**Fields:**
- `post: verus_syn::Expr`
- `name: verus_syn::Path`
- `arms: Vec<Arm>`
- `label: Option<verus_syn::Expr>`

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> parse::Result<MatchPost>`



## verus_state_machines_macros::case_macro::MatchPrePost

*Struct*

**Fields:**
- `pre: verus_syn::Expr`
- `post: verus_syn::Expr`
- `name: verus_syn::Path`
- `arms: Vec<Arm>`
- `label: Option<verus_syn::Expr>`

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> parse::Result<MatchPrePost>`



## verus_state_machines_macros::case_macro::case_on

*Function*

```rust
fn case_on(input: proc_macro::TokenStream, is_init: bool, is_strong: bool) -> proc_macro::TokenStream
```



## verus_state_machines_macros::case_macro::parse_arm

*Function*

```rust
fn parse_arm(input: verus_syn::parse::ParseStream) -> parse::Result<Arm>
```



