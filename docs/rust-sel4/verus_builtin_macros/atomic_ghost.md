**verus_builtin_macros > atomic_ghost**

# Module: atomic_ghost

## Contents

**Structs**

- [`AG`](#ag)

**Functions**

- [`atomic_ghost`](#atomic_ghost)
- [`atomic_ghost_main`](#atomic_ghost_main)

---

## verus_builtin_macros::atomic_ghost::AG

*Struct*

**Fields:**
- `inner_macro_path: verus_syn::Path`
- `atomic: verus_syn::Expr`
- `op_name: verus_syn::Ident`
- `operands: Vec<verus_syn::Expr>`
- `prev_next: Option<(verus_syn::Ident, verus_syn::Ident)>`
- `ret: Option<verus_syn::Ident>`
- `ghost_name: verus_syn::Ident`
- `block: verus_syn::Block`

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> parse::Result<AG>`



## verus_builtin_macros::atomic_ghost::atomic_ghost

*Function*

```rust
fn atomic_ghost(input: proc_macro::TokenStream) -> proc_macro::TokenStream
```



## verus_builtin_macros::atomic_ghost::atomic_ghost_main

*Function*

```rust
fn atomic_ghost_main(ag: AG) -> parse::Result<proc_macro2::TokenStream>
```



