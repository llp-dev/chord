*[sel4_config_macros](../../index.md) / [generic](../index.md) / [cfg_if](index.md)*

---

# Module `cfg_if`

## Contents

- [Structs](#structs)
  - [`CfgIfInput`](#cfgifinput)
  - [`BranchWithCondition`](#branchwithcondition)
- [Functions](#functions)
  - [`parse_cfg_if_input`](#parse-cfg-if-input)
  - [`parse_branch_with_condition`](#parse-branch-with-condition)
  - [`parse_condition`](#parse-condition)
  - [`parse_branch`](#parse-branch)
- [Type Aliases](#type-aliases)
  - [`Branch`](#branch)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`CfgIfInput`](#cfgifinput) | struct |  |
| [`BranchWithCondition`](#branchwithcondition) | struct |  |
| [`parse_cfg_if_input`](#parse-cfg-if-input) | fn |  |
| [`parse_branch_with_condition`](#parse-branch-with-condition) | fn |  |
| [`parse_condition`](#parse-condition) | fn |  |
| [`parse_branch`](#parse-branch) | fn |  |
| [`Branch`](#branch) | type |  |

## Structs

### `CfgIfInput`

```rust
struct CfgIfInput {
    branches_with_conditions: Vec<BranchWithCondition>,
    trailing_branch_without_condition: Option<proc_macro2::TokenStream>,
}
```

### `BranchWithCondition`

```rust
struct BranchWithCondition {
    condition: condition::Condition,
    branch: proc_macro2::TokenStream,
}
```

## Functions

### `parse_cfg_if_input`

```rust
fn parse_cfg_if_input(synthetic_attr: &str, input: syn::parse::ParseStream<'_>) -> syn::Result<CfgIfInput>
```

### `parse_branch_with_condition`

```rust
fn parse_branch_with_condition(synthetic_attr: &str, input: syn::parse::ParseStream<'_>) -> syn::Result<BranchWithCondition>
```

### `parse_condition`

```rust
fn parse_condition(synthetic_attr: &str, input: syn::parse::ParseStream<'_>) -> syn::Result<condition::Condition>
```

### `parse_branch`

```rust
fn parse_branch(input: syn::parse::ParseStream<'_>) -> syn::Result<proc_macro2::TokenStream>
```

## Type Aliases

### `Branch`

```rust
type Branch = proc_macro2::TokenStream;
```

