*[sel4_config_macros](../index.md) / [generic](index.md)*

---

# Module `generic`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`attr_macros`](#attr-macros) | mod |  |
| [`cfg_if`](#cfg-if) | mod |  |
| [`common_helpers`](#common-helpers) | mod |  |
| [`condition`](#condition) | mod |  |
| [`expr_macros`](#expr-macros) | mod |  |
| [`MacroImpls`](#macroimpls) | struct |  |

## Modules

- [`attr_macros`](attr_macros/index.md)
- [`cfg_if`](cfg_if/index.md)
- [`common_helpers`](common_helpers/index.md)
- [`condition`](condition/index.md)
- [`expr_macros`](expr_macros/index.md)

## Structs

### `MacroImpls<'a>`

```rust
struct MacroImpls<'a> {
    config: &'a sel4_config_types::Configuration,
    synthetic_attr: &'a str,
}
```

#### Implementations

- <span id="supermacroimpls-cfg-impl"></span>`fn cfg_impl(&self, input: TokenStream, item: TokenStream) -> TokenStream`

- <span id="supermacroimpls-cfg-attr-impl"></span>`fn cfg_attr_impl(&self, input: TokenStream, item: TokenStream) -> TokenStream`

- <span id="supermacroimpls-cfg-struct-impl"></span>`fn cfg_struct_impl(&self, input: TokenStream, item: TokenStream) -> TokenStream`

- <span id="supermacroimpls-cfg-enum-impl"></span>`fn cfg_enum_impl(&self, input: TokenStream, item: TokenStream) -> TokenStream`

- <span id="supermacroimpls-cfg-match-impl"></span>`fn cfg_match_impl(&self, input: TokenStream, item: TokenStream) -> TokenStream`

- <span id="supermacroimpls-cfg-wrap-match-impl"></span>`fn cfg_wrap_match_impl(&self, item: TokenStream) -> TokenStream`

- <span id="supermacroimpls-cfg-match-impl-inner"></span>`fn cfg_match_impl_inner(&self, item: TokenStream) -> TokenStream`

