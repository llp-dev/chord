*[wasmparser](../index.md) / [arity](index.md)*

---

# Module `arity`

## Contents

- [Traits](#traits)
  - [`ModuleArity`](#modulearity)
- [Functions](#functions)
  - [`visit_block`](#visit-block)
  - [`visit_loop`](#visit-loop)
  - [`visit_if`](#visit-if)
  - [`visit_else`](#visit-else)
  - [`visit_end`](#visit-end)
  - [`visit_br`](#visit-br)
  - [`visit_br_if`](#visit-br-if)
  - [`visit_br_table`](#visit-br-table)
  - [`visit_return`](#visit-return)
  - [`visit_call`](#visit-call)
  - [`visit_call_indirect`](#visit-call-indirect)
  - [`visit_struct_new`](#visit-struct-new)
  - [`visit_struct_new_desc`](#visit-struct-new-desc)
  - [`visit_array_new_fixed`](#visit-array-new-fixed)
  - [`visit_br_on_cast`](#visit-br-on-cast)
  - [`visit_br_on_cast_fail`](#visit-br-on-cast-fail)
  - [`visit_typed_select_multi`](#visit-typed-select-multi)
  - [`visit_return_call`](#visit-return-call)
  - [`visit_return_call_indirect`](#visit-return-call-indirect)
  - [`visit_try_table`](#visit-try-table)
  - [`visit_throw`](#visit-throw)
  - [`visit_try`](#visit-try)
  - [`visit_catch`](#visit-catch)
  - [`visit_delegate`](#visit-delegate)
  - [`visit_catch_all`](#visit-catch-all)
  - [`visit_call_ref`](#visit-call-ref)
  - [`visit_return_call_ref`](#visit-return-call-ref)
  - [`visit_br_on_null`](#visit-br-on-null)
  - [`visit_br_on_non_null`](#visit-br-on-non-null)
  - [`visit_cont_bind`](#visit-cont-bind)
  - [`visit_suspend`](#visit-suspend)
  - [`visit_resume`](#visit-resume)
  - [`visit_resume_throw`](#visit-resume-throw)
  - [`visit_switch`](#visit-switch)
  - [`visit_br_on_cast_desc`](#visit-br-on-cast-desc)
  - [`visit_br_on_cast_desc_fail`](#visit-br-on-cast-desc-fail)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ModuleArity`](#modulearity) | trait | To compute the arity (param and result counts) of "variable-arity" operators, the operator_arity macro needs information about the module's types and the current control stack. |
| [`visit_block`](#visit-block) | fn |  |
| [`visit_loop`](#visit-loop) | fn |  |
| [`visit_if`](#visit-if) | fn |  |
| [`visit_else`](#visit-else) | fn |  |
| [`visit_end`](#visit-end) | fn |  |
| [`visit_br`](#visit-br) | fn |  |
| [`visit_br_if`](#visit-br-if) | fn |  |
| [`visit_br_table`](#visit-br-table) | fn |  |
| [`visit_return`](#visit-return) | fn |  |
| [`visit_call`](#visit-call) | fn |  |
| [`visit_call_indirect`](#visit-call-indirect) | fn |  |
| [`visit_struct_new`](#visit-struct-new) | fn |  |
| [`visit_struct_new_desc`](#visit-struct-new-desc) | fn |  |
| [`visit_array_new_fixed`](#visit-array-new-fixed) | fn |  |
| [`visit_br_on_cast`](#visit-br-on-cast) | fn |  |
| [`visit_br_on_cast_fail`](#visit-br-on-cast-fail) | fn |  |
| [`visit_typed_select_multi`](#visit-typed-select-multi) | fn |  |
| [`visit_return_call`](#visit-return-call) | fn |  |
| [`visit_return_call_indirect`](#visit-return-call-indirect) | fn |  |
| [`visit_try_table`](#visit-try-table) | fn |  |
| [`visit_throw`](#visit-throw) | fn |  |
| [`visit_try`](#visit-try) | fn |  |
| [`visit_catch`](#visit-catch) | fn |  |
| [`visit_delegate`](#visit-delegate) | fn |  |
| [`visit_catch_all`](#visit-catch-all) | fn |  |
| [`visit_call_ref`](#visit-call-ref) | fn |  |
| [`visit_return_call_ref`](#visit-return-call-ref) | fn |  |
| [`visit_br_on_null`](#visit-br-on-null) | fn |  |
| [`visit_br_on_non_null`](#visit-br-on-non-null) | fn |  |
| [`visit_cont_bind`](#visit-cont-bind) | fn |  |
| [`visit_suspend`](#visit-suspend) | fn |  |
| [`visit_resume`](#visit-resume) | fn |  |
| [`visit_resume_throw`](#visit-resume-throw) | fn |  |
| [`visit_switch`](#visit-switch) | fn |  |
| [`visit_br_on_cast_desc`](#visit-br-on-cast-desc) | fn |  |
| [`visit_br_on_cast_desc_fail`](#visit-br-on-cast-desc-fail) | fn |  |

## Traits

### `ModuleArity`

```rust
trait ModuleArity { ... }
```

To compute the arity (param and result counts) of "variable-arity"
operators, the operator_arity macro needs information about the
module's types and the current control stack. The ModuleArity
trait exposes this information.

#### Required Methods

- `fn sub_type_at(&self, type_idx: u32) -> Option<&SubType>`

  Type with given index

- `fn tag_type_arity(&self, at: u32) -> Option<(u32, u32)>`

  Arity (param and result counts) of tag with given index

- `fn type_index_of_function(&self, function_idx: u32) -> Option<u32>`

  Type index of function with given index

- `fn func_type_of_cont_type(&self, c: &ContType) -> Option<&FuncType>`

  Function type for a given continuation type

- `fn sub_type_of_ref_type(&self, rt: &RefType) -> Option<&SubType>`

  Sub type for a given reference type

- `fn control_stack_height(&self) -> u32`

  Current height of control stack

- `fn label_block(&self, depth: u32) -> Option<(BlockType, FrameKind)>`

  BlockType and FrameKind of label with given index

#### Provided Methods

- `fn sub_type_arity(&self, t: &SubType) -> Option<(u32, u32)>`

  Computes arity of given SubType

- `fn block_type_arity(&self, ty: BlockType) -> Option<(u32, u32)>`

  Computes arity of given BlockType

## Functions

### `visit_block`

```rust
fn visit_block(module: &dyn ModuleArity, block: crate::BlockType) -> Option<(u32, u32)>
```

### `visit_loop`

```rust
fn visit_loop(module: &dyn ModuleArity, block: crate::BlockType) -> Option<(u32, u32)>
```

### `visit_if`

```rust
fn visit_if(module: &dyn ModuleArity, block: crate::BlockType) -> Option<(u32, u32)>
```

### `visit_else`

```rust
fn visit_else(module: &dyn ModuleArity) -> Option<(u32, u32)>
```

### `visit_end`

```rust
fn visit_end(module: &dyn ModuleArity) -> Option<(u32, u32)>
```

### `visit_br`

```rust
fn visit_br(module: &dyn ModuleArity, depth: u32) -> Option<(u32, u32)>
```

### `visit_br_if`

```rust
fn visit_br_if(module: &dyn ModuleArity, depth: u32) -> Option<(u32, u32)>
```

### `visit_br_table`

```rust
fn visit_br_table(module: &dyn ModuleArity, table: crate::BrTable<'_>) -> Option<(u32, u32)>
```

### `visit_return`

```rust
fn visit_return(module: &dyn ModuleArity) -> Option<(u32, u32)>
```

### `visit_call`

```rust
fn visit_call(module: &dyn ModuleArity, func: u32) -> Option<(u32, u32)>
```

### `visit_call_indirect`

```rust
fn visit_call_indirect(module: &dyn ModuleArity, ty: u32, _table: u32) -> Option<(u32, u32)>
```

### `visit_struct_new`

```rust
fn visit_struct_new(module: &dyn ModuleArity, ty: u32) -> Option<(u32, u32)>
```

### `visit_struct_new_desc`

```rust
fn visit_struct_new_desc(module: &dyn ModuleArity, ty: u32) -> Option<(u32, u32)>
```

### `visit_array_new_fixed`

```rust
fn visit_array_new_fixed(_module: &dyn ModuleArity, _ty: u32, size: u32) -> Option<(u32, u32)>
```

### `visit_br_on_cast`

```rust
fn visit_br_on_cast(module: &dyn ModuleArity, depth: u32, _from: crate::RefType, _to: crate::RefType) -> Option<(u32, u32)>
```

### `visit_br_on_cast_fail`

```rust
fn visit_br_on_cast_fail(module: &dyn ModuleArity, depth: u32, _from: crate::RefType, _to: crate::RefType) -> Option<(u32, u32)>
```

### `visit_typed_select_multi`

```rust
fn visit_typed_select_multi(_module: &dyn ModuleArity, tys: Vec<crate::ValType>) -> Option<(u32, u32)>
```

### `visit_return_call`

```rust
fn visit_return_call(module: &dyn ModuleArity, func: u32) -> Option<(u32, u32)>
```

### `visit_return_call_indirect`

```rust
fn visit_return_call_indirect(module: &dyn ModuleArity, ty: u32, table: u32) -> Option<(u32, u32)>
```

### `visit_try_table`

```rust
fn visit_try_table(module: &dyn ModuleArity, table: crate::TryTable) -> Option<(u32, u32)>
```

### `visit_throw`

```rust
fn visit_throw(module: &dyn ModuleArity, tag: u32) -> Option<(u32, u32)>
```

### `visit_try`

```rust
fn visit_try(module: &dyn ModuleArity, ty: crate::BlockType) -> Option<(u32, u32)>
```

### `visit_catch`

```rust
fn visit_catch(module: &dyn ModuleArity, tag: u32) -> Option<(u32, u32)>
```

### `visit_delegate`

```rust
fn visit_delegate(module: &dyn ModuleArity, _depth: u32) -> Option<(u32, u32)>
```

### `visit_catch_all`

```rust
fn visit_catch_all(module: &dyn ModuleArity) -> Option<(u32, u32)>
```

### `visit_call_ref`

```rust
fn visit_call_ref(module: &dyn ModuleArity, ty: u32) -> Option<(u32, u32)>
```

### `visit_return_call_ref`

```rust
fn visit_return_call_ref(module: &dyn ModuleArity, ty: u32) -> Option<(u32, u32)>
```

### `visit_br_on_null`

```rust
fn visit_br_on_null(module: &dyn ModuleArity, depth: u32) -> Option<(u32, u32)>
```

### `visit_br_on_non_null`

```rust
fn visit_br_on_non_null(module: &dyn ModuleArity, depth: u32) -> Option<(u32, u32)>
```

### `visit_cont_bind`

```rust
fn visit_cont_bind(module: &dyn ModuleArity, arg: u32, result: u32) -> Option<(u32, u32)>
```

### `visit_suspend`

```rust
fn visit_suspend(module: &dyn ModuleArity, tag: u32) -> Option<(u32, u32)>
```

### `visit_resume`

```rust
fn visit_resume(module: &dyn ModuleArity, cont: u32, _table: crate::ResumeTable) -> Option<(u32, u32)>
```

### `visit_resume_throw`

```rust
fn visit_resume_throw(module: &dyn ModuleArity, cont: u32, tag: u32, _table: crate::ResumeTable) -> Option<(u32, u32)>
```

### `visit_switch`

```rust
fn visit_switch(module: &dyn ModuleArity, cont: u32, _tag: u32) -> Option<(u32, u32)>
```

### `visit_br_on_cast_desc`

```rust
fn visit_br_on_cast_desc(module: &dyn ModuleArity, depth: u32, _from: crate::RefType, _to: crate::RefType) -> Option<(u32, u32)>
```

### `visit_br_on_cast_desc_fail`

```rust
fn visit_br_on_cast_desc_fail(module: &dyn ModuleArity, depth: u32, _from: crate::RefType, _to: crate::RefType) -> Option<(u32, u32)>
```

