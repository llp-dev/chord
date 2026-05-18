**verus_builtin_macros > contrib > exec_spec**

# Module: contrib::exec_spec

## Contents

**Structs**

- [`Exprs`](#exprs) - Custom parser for a comma-separated list of expressions.
- [`GuardedQuantifierBounds`](#guardedquantifierbounds)
- [`GuardedQuantifierUnverified`](#guardedquantifierunverified)
- [`GuardedQuantifierVar`](#guardedquantifiervar)
- [`GuardedQuantifierVerified`](#guardedquantifierverified)
- [`Items`](#items) - Custom parser for a list of items.
- [`LocalCtx`](#localctx) - Records the locals and their modes.

**Enums**

- [`ExprPathKind`](#exprpathkind)
- [`TypeKind`](#typekind)
- [`VarMode`](#varmode) - Each variable is marked with a mode indicating

**Functions**

- [`check_quant_type`](#check_quant_type) - Returns true when the given type is supported in a quantified expression for the given mode
- [`compile_block`](#compile_block) - Compiles a block.
- [`compile_enum`](#compile_enum) - Compiles an enum item.
- [`compile_expr`](#compile_expr) - Compiles an expression
- [`compile_expr_path`](#compile_expr_path) - Similar to `compile_pat_path`, but for paths occurring in expressions.
- [`compile_guarded_quant_loops_unverified`](#compile_guarded_quant_loops_unverified) - Compiles nested loops for the guarded variables, given the quantifier op (exists/forall), body expression, and guard operator (&&/==>)
- [`compile_guarded_quant_unverified`](#compile_guarded_quant_unverified) - Compiles some forms of forall/exists quantifiers to loops.
- [`compile_guarded_quant_verified`](#compile_guarded_quant_verified) - Compiles some forms of forall/exists quantifiers to loops.
- [`compile_item`](#compile_item) - Compiles a fn/struct/enum item.
- [`compile_match_arm`](#compile_match_arm) - Compiles a match arm.
- [`compile_pat_path`](#compile_pat_path) - Maps a spec mode path to the corresponding exec mode path
- [`compile_pattern`](#compile_pattern) - Compiles a spec mode pattern to an exec mode pattern,
- [`compile_sig`](#compile_sig) - Compiles a spec fn to the exec fn signature.
- [`compile_single_quant_var`](#compile_single_quant_var) - Compiles the initialization, update statement, initial condition, and while loop condition for a single variable
- [`compile_spec_fn`](#compile_spec_fn) - Compiles a spec function into an exec function.
- [`compile_struct`](#compile_struct) - Compiles a struct item.
- [`compile_type`](#compile_type) - Converts a spec type to exec type via `<T as ExecSpecType>::Exec`.
- [`exec_spec`](#exec_spec) - Parses and compiles a list of items.
- [`get_guarded_range_quant_unverified`](#get_guarded_range_quant_unverified) - Matches the closure to the form
- [`get_guarded_range_quant_verified`](#get_guarded_range_quant_verified) - Matches the closure to the form
- [`get_seg_type_arg`](#get_seg_type_arg) - Gets the n-th (angle-bracket) argument as a type.
- [`get_simple_pat`](#get_simple_pat) - A simple pattern is either a variable (`a`) or a typed variable (`a: T`).
- [`get_single_guard`](#get_single_guard) - Extracts a single guard expression
- [`infer_expr_path_kind`](#infer_expr_path_kind) - Infers the kind of path based on context and the form of the path.
- [`is_path_eq`](#is_path_eq) - Checks if the given path is of the form
- [`prefix_nth_segment`](#prefix_nth_segment) - Appends a `prefix` to the n-th segment of the path.
- [`respan`](#respan) - Recursively sets the span of all tokens in a token stream to the given one.

**Constants**

- [`UNSUPPORTED_QUANTIFIED_TYPE_ERROR_MSG`](#unsupported_quantified_type_error_msg)
- [`UNSUPPORTED_QUANTIFIER_ERROR_MSG`](#unsupported_quantifier_error_msg)
- [`UNTRUSTED_UNSUPPORTED_QUANTIFIED_TYPE_ERROR_MSG`](#untrusted_unsupported_quantified_type_error_msg)
- [`UNTRUSTED_UNSUPPORTED_QUANTIFIER_ERROR_MSG`](#untrusted_unsupported_quantifier_error_msg)

---

## verus_builtin_macros::contrib::exec_spec::ExprPathKind

*Enum*

**Variants:**
- `Local(VarMode)`
- `FnName`
- `StructOrEnum`
- `Constant`
- `Unknown`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ExprPathKind`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &ExprPathKind) -> bool`



## verus_builtin_macros::contrib::exec_spec::Exprs

*Struct*

Custom parser for a comma-separated list of expressions.

**Tuple Struct**: `(Vec<verus_syn::Expr>)`

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> verus_syn::parse::Result<Exprs>`



## verus_builtin_macros::contrib::exec_spec::GuardedQuantifierBounds

*Struct*

**Fields:**
- `lower: Box<verus_syn::Expr>`
- `upper: Box<verus_syn::Expr>`
- `lower_op: verus_syn::BinOp`
- `upper_op: verus_syn::BinOp`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> GuardedQuantifierBounds`



## verus_builtin_macros::contrib::exec_spec::GuardedQuantifierUnverified

*Struct*

**Fields:**
- `guard_op: verus_syn::BinOp`
- `body: Box<verus_syn::Expr>`
- `guarded_vars: Vec<GuardedQuantifierVar>`



## verus_builtin_macros::contrib::exec_spec::GuardedQuantifierVar

*Struct*

**Fields:**
- `quant_var: verus_syn::Ident`
- `quant_type: Box<verus_syn::Type>`
- `bounds: GuardedQuantifierBounds`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> GuardedQuantifierVar`



## verus_builtin_macros::contrib::exec_spec::GuardedQuantifierVerified

*Struct*

**Fields:**
- `quant_var: GuardedQuantifierVar`
- `guard_op: verus_syn::BinOp`
- `body: Box<verus_syn::Expr>`



## verus_builtin_macros::contrib::exec_spec::Items

*Struct*

Custom parser for a list of items.

**Tuple Struct**: `(Vec<verus_syn::Item>)`

**Trait Implementations:**

- **Parse**
  - `fn parse(input: ParseStream) -> verus_syn::parse::Result<Items>`



## verus_builtin_macros::contrib::exec_spec::LocalCtx

*Struct*

Records the locals and their modes.

**Fields:**
- `cur_fn: verus_syn::Ident` - Name of the current spec function
- `vars: std::collections::HashMap<verus_syn::Ident, VarMode>` - Mapping local variables to their modes
- `trigger_fns: std::rc::Rc<std::cell::RefCell<std::collections::HashMap<verus_syn::Ident, verus_syn::Type>>>` - A global counter for generating fresh trigger

**Methods:**

- `fn new(cur_fn: &Ident) -> Self`
- `fn add(self: & mut Self, ident: Ident, mode: VarMode)`
- `fn gen_fresh_trigger_fn(self: &Self, typ: &Type) -> Ident` - Generates a fresh trigger function name

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> LocalCtx`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## verus_builtin_macros::contrib::exec_spec::TypeKind

*Enum*

**Variants:**
- `Owned`
- `Ref`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> TypeKind`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## verus_builtin_macros::contrib::exec_spec::UNSUPPORTED_QUANTIFIED_TYPE_ERROR_MSG

*Constant*: `&str`



## verus_builtin_macros::contrib::exec_spec::UNSUPPORTED_QUANTIFIER_ERROR_MSG

*Constant*: `&str`



## verus_builtin_macros::contrib::exec_spec::UNTRUSTED_UNSUPPORTED_QUANTIFIED_TYPE_ERROR_MSG

*Constant*: `&str`



## verus_builtin_macros::contrib::exec_spec::UNTRUSTED_UNSUPPORTED_QUANTIFIER_ERROR_MSG

*Constant*: `&str`



## verus_builtin_macros::contrib::exec_spec::VarMode

*Enum*

Each variable is marked with a mode indicating
whether it is the owned or the borrowed version
of the spec type.

**Variants:**
- `Owned`
- `Ref`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> VarMode`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &VarMode) -> bool`



## verus_builtin_macros::contrib::exec_spec::check_quant_type

*Function*

Returns true when the given type is supported in a quantified expression for the given mode

```rust
fn check_quant_type(quant_type: &verus_syn::Type, unverified: bool) -> Result<(), verus_syn::Error>
```



## verus_builtin_macros::contrib::exec_spec::compile_block

*Function*

Compiles a block.

TODO: to avoid issues of `temporary value dropped while borrowed`
the return value of a block has the owned type instead of the ref type
This might incur some performance overhead.

```rust
fn compile_block(ctx: &LocalCtx, block: &verus_syn::Block, unverified: bool) -> Result<proc_macro2::TokenStream, verus_syn::Error>
```



## verus_builtin_macros::contrib::exec_spec::compile_enum

*Function*

Compiles an enum item.

```rust
fn compile_enum(item_enum: &verus_syn::ItemEnum) -> Result<proc_macro2::TokenStream, verus_syn::Error>
```



## verus_builtin_macros::contrib::exec_spec::compile_expr

*Function*

Compiles an expression

Suppose the original expression has (spec) type `T`
the exec expression returned from this function should
have the type
- `T::ExecRefType<'_>` if mode is `VarMode::Ref`
- `T::ExecOwnedType` if mode is `VarMode::Owned`

```rust
fn compile_expr(ctx: &LocalCtx, expr: &verus_syn::Expr, mode: VarMode, unverified: bool) -> Result<proc_macro2::TokenStream, verus_syn::Error>
```



## verus_builtin_macros::contrib::exec_spec::compile_expr_path

*Function*

Similar to `compile_pat_path`, but for paths occurring in expressions.
TODO: find ways to make this more reliable

```rust
fn compile_expr_path(ctx: &LocalCtx, path: &verus_syn::Path, known_kind: Option<ExprPathKind>) -> Result<(verus_syn::Path, ExprPathKind), verus_syn::Error>
```



## verus_builtin_macros::contrib::exec_spec::compile_guarded_quant_loops_unverified

*Function*

Compiles nested loops for the guarded variables, given the quantifier op (exists/forall), body expression, and guard operator (&&/==>)

```rust
fn compile_guarded_quant_loops_unverified(ctx: &LocalCtx, op: &verus_syn::UnOp, expr: &verus_syn::Expr, guard_op: &verus_syn::BinOp, body: &verus_syn::Expr, guarded_vars: &Vec<GuardedQuantifierVar>) -> Result<proc_macro2::TokenStream, verus_syn::Error>
```



## verus_builtin_macros::contrib::exec_spec::compile_guarded_quant_unverified

*Function*

Compiles some forms of forall/exists quantifiers to loops.

```rust
fn compile_guarded_quant_unverified(ctx: &LocalCtx, op: &verus_syn::UnOp, expr: &verus_syn::Expr) -> Result<proc_macro2::TokenStream, verus_syn::Error>
```



## verus_builtin_macros::contrib::exec_spec::compile_guarded_quant_verified

*Function*

Compiles some forms of forall/exists quantifiers to loops.

```rust
fn compile_guarded_quant_verified(ctx: &LocalCtx, op: &verus_syn::UnOp, expr: &verus_syn::Expr) -> Result<proc_macro2::TokenStream, verus_syn::Error>
```



## verus_builtin_macros::contrib::exec_spec::compile_item

*Function*

Compiles a fn/struct/enum item.

```rust
fn compile_item(item: verus_syn::Item, unverified: bool) -> Result<proc_macro2::TokenStream, verus_syn::Error>
```



## verus_builtin_macros::contrib::exec_spec::compile_match_arm

*Function*

Compiles a match arm.

```rust
fn compile_match_arm(ctx: &LocalCtx, arm: &verus_syn::Arm, unverified: bool) -> Result<proc_macro2::TokenStream, verus_syn::Error>
```



## verus_builtin_macros::contrib::exec_spec::compile_pat_path

*Function*

Maps a spec mode path to the corresponding exec mode path
Assuming that it is already checked that path is not a
local variable.

```rust
fn compile_pat_path(path: &verus_syn::Path) -> Result<verus_syn::Path, verus_syn::Error>
```



## verus_builtin_macros::contrib::exec_spec::compile_pattern

*Function*

Compiles a spec mode pattern to an exec mode pattern,
potentially shadowing some local variables.

For paths occurring in the patterns,
we assume that they are only used in two ways:
  - SpecEnumName::Variant => ExecSpecEnumName::ExecVariant
  - SpecStructName => ExecSpecStructName

i.e. for paths of length 2, we prefix the first segment with `Exec`
and for paths of length 1, we prefix the last segment with `Exec`.

```rust
fn compile_pattern(ctx: & mut LocalCtx, pat: &verus_syn::Pat, new_locals: & mut std::collections::HashSet<verus_syn::Ident>) -> Result<proc_macro2::TokenStream, verus_syn::Error>
```



## verus_builtin_macros::contrib::exec_spec::compile_sig

*Function*

Compiles a spec fn to the exec fn signature.

```rust
fn compile_sig(ctx: & mut LocalCtx, item_fn: &verus_syn::ItemFn, unverified: bool) -> Result<proc_macro2::TokenStream, verus_syn::Error>
```



## verus_builtin_macros::contrib::exec_spec::compile_single_quant_var

*Function*

Compiles the initialization, update statement, initial condition, and while loop condition for a single variable

```rust
fn compile_single_quant_var(ctx: &LocalCtx, var: &GuardedQuantifierVar) -> Result<(proc_macro2::TokenStream, proc_macro2::TokenStream, proc_macro2::TokenStream, proc_macro2::TokenStream), verus_syn::Error>
```



## verus_builtin_macros::contrib::exec_spec::compile_spec_fn

*Function*

Compiles a spec function into an exec function.

```rust
fn compile_spec_fn(item_fn: &verus_syn::ItemFn, unverified: bool) -> Result<proc_macro2::TokenStream, verus_syn::Error>
```



## verus_builtin_macros::contrib::exec_spec::compile_struct

*Function*

Compiles a struct item.

```rust
fn compile_struct(item_struct: &verus_syn::ItemStruct) -> Result<proc_macro2::TokenStream, verus_syn::Error>
```



## verus_builtin_macros::contrib::exec_spec::compile_type

*Function*

Converts a spec type to exec type via `<T as ExecSpecType>::Exec`.

```rust
fn compile_type(typ: &verus_syn::Type, ctx: TypeKind) -> Result<proc_macro2::TokenStream, verus_syn::Error>
```



## verus_builtin_macros::contrib::exec_spec::exec_spec

*Function*

Parses and compiles a list of items.

```rust
fn exec_spec(input: proc_macro::TokenStream, unverified: bool) -> proc_macro::TokenStream
```



## verus_builtin_macros::contrib::exec_spec::get_guarded_range_quant_unverified

*Function*

Matches the closure to the form
|x1: <type1>, x2: <type2>, ..., xN: <typeN>| <guard1> && <guard2> && ... && <guardN> ==> <body>
or
|x1: <type1>, x2: <type2>, ..., xN: <typeN>| <guard1> && <guard2> && ... && <guardN> && <body>

```rust
fn get_guarded_range_quant_unverified(closure: &verus_syn::ExprClosure) -> Result<GuardedQuantifierUnverified, verus_syn::Error>
```



## verus_builtin_macros::contrib::exec_spec::get_guarded_range_quant_verified

*Function*

Matches the closure to the form
  `|x| <guard> ==> <body>`
or
  `|x| <guard> && <body>`
where <guard> is one of:
  `<lower> <= x < <upper>`
  `<lower> <= x <= <upper>`
  `<lower> < x < <upper>`
  `<lower> < x <= <upper>`

```rust
fn get_guarded_range_quant_verified(closure: &verus_syn::ExprClosure) -> Result<GuardedQuantifierVerified, verus_syn::Error>
```



## verus_builtin_macros::contrib::exec_spec::get_seg_type_arg

*Function*

Gets the n-th (angle-bracket) argument as a type.

```rust
fn get_seg_type_arg(seg: &verus_syn::PathSegment, n: usize) -> Result<&verus_syn::Type, verus_syn::Error>
```



## verus_builtin_macros::contrib::exec_spec::get_simple_pat

*Function*

A simple pattern is either a variable (`a`) or a typed variable (`a: T`).

```rust
fn get_simple_pat(pat: &verus_syn::Pat) -> Result<(&verus_syn::Ident, Option<Box<verus_syn::Type>>), verus_syn::Error>
```



## verus_builtin_macros::contrib::exec_spec::get_single_guard

*Function*

Extracts a single guard expression

```rust
fn get_single_guard(guard: &verus_syn::Expr, quant_var: &verus_syn::Ident, unverified: bool) -> Result<GuardedQuantifierBounds, verus_syn::Error>
```



## verus_builtin_macros::contrib::exec_spec::infer_expr_path_kind

*Function*

Infers the kind of path based on context and the form of the path.
TODO: a bit ad-hoc

```rust
fn infer_expr_path_kind(ctx: &LocalCtx, path: &verus_syn::Path) -> ExprPathKind
```



## verus_builtin_macros::contrib::exec_spec::is_path_eq

*Function*

Checks if the given path is of the form
`idents[0]::idents[1]::...::idents[n]`,
ignoring any path arguments.

```rust
fn is_path_eq(path: &verus_syn::Path, idents: &[&str]) -> bool
```



## verus_builtin_macros::contrib::exec_spec::prefix_nth_segment

*Function*

Appends a `prefix` to the n-th segment of the path.

```rust
fn prefix_nth_segment(path: &verus_syn::Path, prefix: &str, n: usize) -> Result<verus_syn::Path, verus_syn::Error>
```



## verus_builtin_macros::contrib::exec_spec::respan

*Function*

Recursively sets the span of all tokens in a token stream to the given one.

```rust
fn respan(input: proc_macro2::TokenStream, span: proc_macro2::Span) -> proc_macro2::TokenStream
```



