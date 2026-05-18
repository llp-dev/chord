**verus_builtin_macros > enum_synthesize**

# Module: enum_synthesize

## Contents

**Functions**

- [`attribute_verus_enum_synthesize`](#attribute_verus_enum_synthesize)
- [`visit_item_enum_synthesize`](#visit_item_enum_synthesize)

---

## verus_builtin_macros::enum_synthesize::attribute_verus_enum_synthesize

*Function*

```rust
fn attribute_verus_enum_synthesize(_erase_ghost: &crate::EraseGhost, _attr: proc_macro::TokenStream, stream: proc_macro::TokenStream) -> proc_macro::TokenStream
```



## verus_builtin_macros::enum_synthesize::visit_item_enum_synthesize

*Function*

```rust
fn visit_item_enum_synthesize(erase_ghost: &crate::EraseGhost, enum_: & mut verus_syn::ItemEnum) -> Option<verus_syn::Item>
```



