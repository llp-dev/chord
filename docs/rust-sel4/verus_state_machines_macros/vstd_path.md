**verus_state_machines_macros > vstd_path**

# Module: vstd_path

## Contents

**Macros**

- [`quote_spanned_vstd`](#quote_spanned_vstd)
- [`quote_vstd`](#quote_vstd)

**Structs**

- [`Vstd`](#vstd)

**Functions**

- [`set_is_core`](#set_is_core)
- [`set_is_vstd`](#set_is_vstd)

**Statics**

- [`IS_CORE`](#is_core)
- [`IS_VSTD`](#is_vstd)

---

## verus_state_machines_macros::vstd_path::IS_CORE

*Static*

```rust
static IS_CORE: std::sync::atomic::AtomicBool
```



## verus_state_machines_macros::vstd_path::IS_VSTD

*Static*

```rust
static IS_VSTD: std::sync::atomic::AtomicBool
```



## verus_state_machines_macros::vstd_path::Vstd

*Struct*

**Tuple Struct**: `(proc_macro2::Span)`

**Trait Implementations:**

- **ToTokens**
  - `fn to_tokens(self: &Self, tokens: & mut TokenStream)`



## verus_state_machines_macros::vstd_path::quote_spanned_vstd

*Declarative Macro*

```rust
macro_rules! quote_spanned_vstd {
    ($b:ident, $span:expr => $($tt:tt)*) => { ... };
}
```



## verus_state_machines_macros::vstd_path::quote_vstd

*Declarative Macro*

```rust
macro_rules! quote_vstd {
    ($b:ident => $($tt:tt)*) => { ... };
}
```



## verus_state_machines_macros::vstd_path::set_is_core

*Function*

```rust
fn set_is_core(b: bool)
```



## verus_state_machines_macros::vstd_path::set_is_vstd

*Function*

```rust
fn set_is_vstd(b: bool)
```



