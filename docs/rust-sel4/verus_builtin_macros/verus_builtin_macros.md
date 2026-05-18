**verus_builtin_macros**

# Module: verus_builtin_macros

## Contents

**Modules**

- [`atomic_ghost`](#atomic_ghost) - Helper proc-macro for the atomic_ghost lib
- [`attr_block_trait`](#attr_block_trait)
- [`attr_rewrite`](#attr_rewrite)
- [`calc_macro`](#calc_macro)
- [`contrib`](#contrib)
- [`enum_synthesize`](#enum_synthesize)
- [`fndecl`](#fndecl)
- [`is_variant`](#is_variant)
- [`rustdoc`](#rustdoc)
- [`struct_decl_inv`](#struct_decl_inv)
- [`structural`](#structural)
- [`syntax`](#syntax)
- [`syntax_trait`](#syntax_trait)
- [`topological_sort`](#topological_sort)
- [`unerased_proxies`](#unerased_proxies) - For many items, we need to "split them in two":

**Proc Macros**

- [`Structural`](#structural)
- [`StructuralEq`](#structuraleq)
- [`atomic_with_ghost_helper`](#atomic_with_ghost_helper)
- [`auto_spec`](#auto_spec) - This copies the body of an exec function into a "returns" clause,
- [`calc_proc_macro`](#calc_proc_macro)
- [`exec_spec_unverified`](#exec_spec_unverified) - Automatically compiles spec items to exec items, but without proofs of functional correctness.
- [`exec_spec_verified`](#exec_spec_verified) - Automatically compiles spec items to exec items, with proofs of functional correctness.
- [`fndecl`](#fndecl)
- [`is_variant`](#is_variant) - Add `is_<VARIANT>` and `get_<VARIANT>` functions to an enum
- [`is_variant_no_deprecation_warning`](#is_variant_no_deprecation_warning) - Add `is_<VARIANT>` and `get_<VARIANT>` functions to an enum
- [`make_spec_type`](#make_spec_type) - Automate generating spec types and their View/DeepView implementations
- [`proof`](#proof) - Add a verus proof block.
- [`proof_decl`](#proof_decl) - proof_decl add extra stmts that are used only
- [`proof_with`](#proof_with) - proof_with add ghost input/output to the next function call.
- [`self_view`](#self_view)
- [`set_build`](#set_build) - This macro takes an expresion of the form:
- [`set_build_debug`](#set_build_debug) - Same as set_build, but prints the generated set builder to stderr
- [`struct_with_invariants`](#struct_with_invariants)
- [`verus`](#verus)
- [`verus_enum_synthesize`](#verus_enum_synthesize)
- [`verus_erase_ghost`](#verus_erase_ghost)
- [`verus_exec_expr`](#verus_exec_expr)
- [`verus_exec_expr_erase_ghost`](#verus_exec_expr_erase_ghost)
- [`verus_exec_expr_keep_ghost`](#verus_exec_expr_keep_ghost)
- [`verus_exec_inv_macro_exprs`](#verus_exec_inv_macro_exprs)
- [`verus_exec_macro_exprs`](#verus_exec_macro_exprs)
- [`verus_ghost_inv_macro_exprs`](#verus_ghost_inv_macro_exprs)
- [`verus_impl`](#verus_impl) - Like verus!, but for use inside a (non-trait) impl
- [`verus_keep_ghost`](#verus_keep_ghost)
- [`verus_proof_expr`](#verus_proof_expr)
- [`verus_proof_macro_explicit_exprs`](#verus_proof_macro_explicit_exprs) - `verus_proof_macro_explicit_exprs!(f!(tts))` applies verus syntax to transform `tts` into
- [`verus_proof_macro_exprs`](#verus_proof_macro_exprs) - verus_proof_macro_exprs!(f!(exprs)) applies verus syntax to transform exprs into exprs',
- [`verus_spec`](#verus_spec)
- [`verus_trait_impl`](#verus_trait_impl) - Like verus!, but for use inside a trait impl
- [`verus_verify`](#verus_verify)

**Enums**

- [`EraseGhost`](#eraseghost)
- [`VstdKind`](#vstdkind)

**Functions**

- [`cfg_erase`](#cfg_erase)
- [`cfg_no_vstd`](#cfg_no_vstd)
- [`cfg_verify_core`](#cfg_verify_core)
- [`vstd_kind`](#vstd_kind)

---

## verus_builtin_macros::EraseGhost

*Enum*

**Variants:**
- `Keep` - keep all ghost code
- `Erase` - erase ghost code, but leave ghost stubs
- `EraseAll` - erase all ghost code

**Methods:**

- `fn keep(self: &Self) -> bool`
- `fn erase(self: &Self) -> bool`
- `fn erase_all(self: &Self) -> bool`

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &EraseGhost) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> EraseGhost`



## verus_builtin_macros::Structural

*Derive Macro*

```rust
#[derive(Structural)]
```



## verus_builtin_macros::StructuralEq

*Derive Macro*

```rust
#[derive(StructuralEq)]
```



## verus_builtin_macros::VstdKind

*Enum*

**Variants:**
- `IsVstd` - The current crate is vstd.
- `NoVstd` - There is no vstd (only verus_builtin). Really only used for testing.
- `Imported` - Imports the vstd crate like usual.
- `IsCore` - Embed vstd and verus_builtin as modules, necessary for verifying the `core` library.
- `ImportedViaCore` - For other crates in stdlib verification that import core

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> VstdKind`



## Module: atomic_ghost

Helper proc-macro for the atomic_ghost lib



## verus_builtin_macros::atomic_with_ghost_helper

*Function-like Macro*

```rust
atomic_with_ghost_helper!(...)
```



## Module: attr_block_trait



## Module: attr_rewrite



## verus_builtin_macros::auto_spec

*Attribute Macro*

This copies the body of an exec function into a "returns" clause,
so that the exec function will be also usable as a spec function.
For example,
  `#[vstd::contrib::auto_spec] fn f(u: u8) -> u8 { u / 2 }`
becomes:
  `#[verifier::allow_in_spec] fn f(u: u8) -> u8 returns (u / 2) { u / 2 }`
The macro performs some limited fixups, such as removing proof blocks
and turning +, -, and * into add, sub, mul.
However, only a few such fixups are currently implemented and not all exec bodies
will be usable as return clauses, so this macro will not work on all exec functions.

```rust
#[auto_spec]
```



## Module: calc_macro



## verus_builtin_macros::calc_proc_macro

*Function-like Macro*

```rust
calc_proc_macro!(...)
```



## verus_builtin_macros::cfg_erase

*Function*

```rust
fn cfg_erase() -> EraseGhost
```



## verus_builtin_macros::cfg_no_vstd

*Function*

```rust
fn cfg_no_vstd() -> bool
```



## verus_builtin_macros::cfg_verify_core

*Function*

```rust
fn cfg_verify_core() -> bool
```



## Module: contrib



## Module: enum_synthesize



## verus_builtin_macros::exec_spec_unverified

*Function-like Macro*

Automatically compiles spec items to exec items, but without proofs of functional correctness.
This means that,iIn contrast to `exec_spec_verified!`, all specifications on compiled executable code generated by `exec_spec_unverified!` are trusted.

Supports all of the features supported by `exec_spec_verified!`, as well as these additional features:
  - More general bounded quantifiers. Quantifier expressions must match this form: `forall |x1: <type1>, x2: <type2>, ..., xN: <typeN>| <guard1> && <guard2> && ... && <guardN> ==> <body>` or `exists |x1: <type1>, x2: <type2>, ..., xN: <typeN>| <guard1> && <guard2> && ... && <guardN> && <body>`, where:
    - `<guardI>` is of the form: `<lowerI> <op> xI <op> <upperI>`, where:
        - `<op>` is either `<=` or `<`
        - `<lowerI>` and `<upperI>` can mention `xJ` for all `J < I`
    - `<typeI>` is a Rust primitive integer (`i<N>`, `isize`, `u<N>`, `usize`) or `char`. Note that `char` is not supported by quantifiers in `exec_spec_verified!`.

```rust
exec_spec_unverified!(...)
```



## verus_builtin_macros::exec_spec_verified

*Function-like Macro*

Automatically compiles spec items to exec items, with proofs of functional correctness.

This macro takes a list of spec items, and generates a corresponding list of exec items:
- For every struct/enum `A`, we generate `ExecA`, which implements `DeepView<V = A>`
- For every spec function `spec fn f(T) -> U`, we generate
  ```rust,ignore
  fn exec_f(a: exec(T)) -> (r: exec(U))
  ensures r.deep_view() == f(a.deep_view()) { ... }
  ```
  where `exec(T)` maps a subset of supported spec types to exec types, including
  `Seq` (translated to `Vec`) and `Option`.

Below is a non-exhaustive list of supported spec expressions. Items marked with a \* utilize unverified translations from spec to exec code internally.
  - Basic arithmetic operations
  - Logical operators (&&, ||, &&&, |||, not, ==>)
  - If, match and "matches"
  - Field expressions
  - Bounded quantifiers of the form `forall |i: <type>| <lower> <op> i <op> <upper> ==> <expr>` and `exists |i: <type>| <lower> <op> i <op> <upper> && <expr>`, where:
    - `<op>` is either `<=` or `<`
    - `<type>` is a Rust primitive integer (`i<N>`, `isize`, `u<N>`, `usize`)
  - `SpecString` (an alias to `Seq<char>` to syntactically indicate that we want `String`/`&str`), equality\*, indexing, len, string literals
  - `Option<T>` with these functions:
    - equality, `unwrap`
  - `Seq<T>` (compiled to `Vec<T>` or `&[T]` depending on the context), `seq!` literals, and these `Seq` functions:
    - equality\*, `len`, indexing, `subrange`\*, `add`\*, `push`\*, `update`\*, `empty`, `to_multiset`\*, `drop_first`\*, `drop_last`\*, `take`\*, `skip`\*, `first`, `last`, `is_suffix_of`\*, `is_prefix_of`\*, `contains`\*, `index_of`\*, `index_of_first`\*, `index_of_last`\*
  - `Map<K, V>` (compiled to `HashMap<K, V>`), and these `Map` functions:
    - equality\*, `len`\*, indexing\*, `empty`, `dom`\*, `insert`\*, `remove`\*, `get`\*
    - Note: indexing is only supported on `Map<K, V>` where `K` is a primitive type (e.g. `usize`); for other types `K`, use `get` instead.
  - `Set<T>` (compiled to `HashSet<T>`), and these `Set` functions:
    - equality\*, `len`\*, `empty`, `contains`\*, `insert`\*, `remove`\*, `union`\*, `intersect`\*, `difference`\*
  - `Multiset<T>` (compiled to `ExecMultiset<T>`, a type implemented in `vstd::contrib::exec_spec` whose internal representation is a `HashMap`), and these `Multiset` functions:
    - equality\*, `len`\*, `count`\*, `empty`\*, `singleton`\*, `add`\*, `sub`\*
  - User-defined structs and enums. These types should be defined within the macro using spec-compatible types for the fields (e.g. `Seq`). Such types are then compiled to their `Exec-` versions, which use the exec versions of each field's type (e.g. `Vec`/slice).
  - Primitive integer/boolean types (`i<N>`, `isize`, `u<N>`, `usize`, `char`, etc.). Note that `int` and `nat` cannot be used in `exec_spec_verified!`.

```rust
exec_spec_verified!(...)
```



## Module: fndecl



## verus_builtin_macros::fndecl

*Function-like Macro*

```rust
fndecl!(...)
```



## Module: is_variant



## verus_builtin_macros::is_variant

*Attribute Macro*

Add `is_<VARIANT>` and `get_<VARIANT>` functions to an enum

```rust
#[is_variant]
```



## verus_builtin_macros::is_variant_no_deprecation_warning

*Attribute Macro*

Add `is_<VARIANT>` and `get_<VARIANT>` functions to an enum

```rust
#[is_variant_no_deprecation_warning]
```



## verus_builtin_macros::make_spec_type

*Attribute Macro*

Automate generating spec types and their View/DeepView implementations
https://github.com/verus-lang/verus/pull/1798

```rust
#[make_spec_type]
```



## verus_builtin_macros::proof

*Function-like Macro*

Add a verus proof block.

```rust
proof!(...)
```



## verus_builtin_macros::proof_decl

*Function-like Macro*

proof_decl add extra stmts that are used only
for verification.
For example, declare a ghost/tracked variable.
To avoid confusion, let stmts without ghost/tracked is not supported.
Non-local stmts inside proof_decl! are treated similar to those in proof!

```rust
proof_decl!(...)
```



## verus_builtin_macros::proof_with

*Function-like Macro*

proof_with add ghost input/output to the next function call.
In stable rust, we cannot add attribute-based macro to expr/statement.
Using proof_with! to tell #[verus_spec] to add ghost input/output.
Using proof_with outside of #[verus_spec] does not have any side effects.

```rust
proof_with!(...)
```



## Module: rustdoc



## verus_builtin_macros::self_view

*Attribute Macro*

```rust
#[self_view]
```



## verus_builtin_macros::set_build

*Function-like Macro*

This macro takes an expresion of the form:
`set_build!{ elem_expr: optional_typ | x1: typ1 in ..., ..., xn: typn in ..., cond1, ..., condm }`
or just:
`set_build!{ x: typ in ..., cond1, ..., condm }`
where each `xk: typk in ...` has one of the following forms:
- `xk: typk` for finite types (implementing FiniteFull)
- `xk: typk in expr`, where expr has type Set<typk>
- `xk: typk in lo..hi`, where lo and hi have type typk, where typk implements FiniteRange
- `xk: typk in lo..=hi`, where lo and hi have type typk, where typk implements FiniteRange
and each condk is a boolean expression.
From this, the setbuild macro uses map_by, map_flatten_by, filter, etc. to build a set of
elements specified by elem_expr.

(Note: to see the code generated by set_build, use set_build_debug,
which is like set_build, but also prints the generated builder to stderr as a side effect.)

Important restriction: by default, the elem_expr must be a variable, tuple, or datatype such
that all of the variables x1, ..., xn can be easily found with nothing more that tuple/datatype
field accesses.  In exchange for this restriction, set_build guarantees not to introduce
any extra existential quantifiers into to constructed set.  This makes it easy for proofs
to use sets constructed with set_build, when compared to other forms of Set construction
(like Set::map or Set::flatten) that do introduce existential quantifiers.
To override this default and remove this restriction, you can mark one or more variables as
`exists x: typ` rather than just `x: typ`, and set_build will use map/flatten for these
variables. This will, however, make proofs about the constructed set more difficult.

## Example: finite types
```
proof fn test() {
    use vstd::contrib::set_build;

    let s: Set<u8> = set_build!{ x: u8 };
    assert(s.finite());
    assert(forall|u: u8| s.contains(u));
}
```
This generates the set `Set::<u8>::from_finite_type(|x: u8| true)`.

## Example: filtering
```
let s: Set<u8> = set_build!{ x: u8, x < 100 };
assert(forall|u: u8| s.contains(u) <==> u < 100);
```
This generates the set `Set::<u8>::from_finite_type(|x: u8| true).filter(|x: u8| (x < 100))`.

## Example: ranges
```
let s: Set<u8> = set_build!{ x: u8 in 10..20 };
assert(forall|u: u8| s.contains(u) <==> 10 <= u < 20);
```
This generates the set `Set::<u8>::range(10, 20)`.

## Example: tuples, mapping, and "exists"
```
let s1: Set<(u8, u8)> = set_build!{ (x, x): (u8, u8) | exists x: u8, x < 100 };
let s2: Set<(u8, u8)> = set_build!{ (x, x): (u8, u8) | x: u8, x < 100 };
```
Here, s1 generates a set based on `filter` and `map`:
```
Set::<u8>::from_finite_type(|x: u8| true)
    .filter(|x: u8| (x < 100))
    .map(|x: u8| ((x, x)))
```
while s2 generates the same set, but uses `map_by` rather than `map`:
```
Set::<u8>::from_finite_type(|x: u8| true)
    .filter(|x: u8| (x < 100))
    .map_by(|x: u8| ((x, x)), |__VERUS_x: (u8, u8)| (__VERUS_x.0))
```
Although s1 looks simpler, it is harder to work with in proofs.
For example, an obvious assertion about s1 fails, while succeeding for s2:
```
assert(forall|p: (u8, u8)| s1.contains(p) <==> p.0 == p.1 && p.0 < 100); // FAILS
assert(forall|p: (u8, u8)| s2.contains(p) <==> p.0 == p.1 && p.0 < 100);
```
Nevertheless, s1 and s2 are equal to each other.
In fact one way to prove the assertion above about s1 is to first use `=~=` to establish
that s1 and s2 are equal:
```
assert(s1 =~= s2);
assert(forall|p: (u8, u8)| s1.contains(p) <==> p.0 == p.1 && p.0 < 100);
```

## Example: datatypes
```
struct Address {
    page: int,
    offset: int,
}

proof fn test() {
    use vstd::contrib::set_build;

    let s: Set<Address> = set_build!{
        Address { page, offset }: Address |
        page: int in 10..20,
        offset: int in 0..4096,
    };
    assert(s.finite());
    assert(s.contains(Address { page: 15, offset: 1024 }));
    assert(!s.contains(Address { page: 5, offset: 1024 }));
}
```
This generates:
```
Set::<int>::range(10, 20)
    .map_flatten_by(
        |__VERUS_x: int| {
            let page = __VERUS_x;
            Set::<int>::range(0, 4096)
                .map_by(
                    |offset: int| (Address { page, offset }),
                    |__VERUS_x: Address| (__VERUS_x.offset),
                )
        },
        |__VERUS_x: Address| __VERUS_x.page,
    )
```

## Example: complex expressions
```
let s = set_build!{ (x, y, y - x): (int, int, int) | x: int in 10..20, y: int in x..20, x + y != 25 };
assert(s.contains((12, 14, 2)));
assert(!s.contains((14, 12, 2))); // because y = 12 is not in 14..20
assert(s.contains((10, 13, 3)));
assert(s.contains((10, 14, 4)));
assert(!s.contains((10, 15, 5))); // because of x + y != 25
assert(s.contains((10, 16, 6)));
```
From this, set_build generates:
```
Set::<int>::range(10, 20)
    .map_flatten_by(
        |__VERUS_x: int| {
            let x = __VERUS_x;
            Set::<int>::range(x, 20)
                .filter(|y: int| (x + y != 25))
                .map_by(
                    |y: int| ((x, y, y - x)),
                    |__VERUS_x: (int, int, int)| (__VERUS_x.1),
                )
        },
        |__VERUS_x: (int, int, int)| __VERUS_x.0,
    )
```

```rust
set_build!(...)
```



## verus_builtin_macros::set_build_debug

*Function-like Macro*

Same as set_build, but prints the generated set builder to stderr

```rust
set_build_debug!(...)
```



## Module: struct_decl_inv



## verus_builtin_macros::struct_with_invariants

*Function-like Macro*

```rust
struct_with_invariants!(...)
```



## Module: structural



## Module: syntax



## Module: syntax_trait



## Module: topological_sort



## Module: unerased_proxies

For many items, we need to "split them in two":

 - The erased version of the item, which retains the syntactic item kind, the name, and has
   ghost code erased.

 - The "unerased_proxy", which has a name prefixed with "VERUS_UNERASED_PROXY__",
   is always a function item, and has all the ghost code.

We do this for all 'const' items and 'const fn' items because any const code might be executed
during rustc's type-checking, but we still need a place for all the ghost code to go.

We also do this for 'static' items for consistency with const. Besides the above reason, this
method avoids the need for bespoke signature-encoding mechanisms for consts and statics.

Specifics:

```
const X: u64 = {
   assert(true);
   0
};
```

becomes

```
#[verus::internal(unerased_proxy)]
#[verus::internal(encoded_const)]
fn VERUS_UNERASED_PROXY__X() -> u64 {
   assert(true);
   0
}

#[verifier::external]
#[verus::internal(has_unerased_proxy)]
const X: u64 = { 0 };
```

Statics are similar, but use the `unerased_static` attribute.

Likewise `const fn` items get split in two:

```
const fn x(t: u64): u64 = {
    assert(true);
    x
}
```

becomes

```
#[verus::internal(unerased_proxy)]
fn VERUS_UNERASED_PROXY__x(t: u64): u64 = {
    assert(true);
    x
}

#[verifier::external]
#[verus::internal(has_unerased_proxy)]
const fn x(t: u64): u64 = {
    x
}
```



## verus_builtin_macros::verus

*Function-like Macro*

```rust
verus!(...)
```



## verus_builtin_macros::verus_enum_synthesize

*Attribute Macro*

```rust
#[verus_enum_synthesize]
```



## verus_builtin_macros::verus_erase_ghost

*Function-like Macro*

```rust
verus_erase_ghost!(...)
```



## verus_builtin_macros::verus_exec_expr

*Function-like Macro*

```rust
verus_exec_expr!(...)
```



## verus_builtin_macros::verus_exec_expr_erase_ghost

*Function-like Macro*

```rust
verus_exec_expr_erase_ghost!(...)
```



## verus_builtin_macros::verus_exec_expr_keep_ghost

*Function-like Macro*

```rust
verus_exec_expr_keep_ghost!(...)
```



## verus_builtin_macros::verus_exec_inv_macro_exprs

*Function-like Macro*

```rust
verus_exec_inv_macro_exprs!(...)
```



## verus_builtin_macros::verus_exec_macro_exprs

*Function-like Macro*

```rust
verus_exec_macro_exprs!(...)
```



## verus_builtin_macros::verus_ghost_inv_macro_exprs

*Function-like Macro*

```rust
verus_ghost_inv_macro_exprs!(...)
```



## verus_builtin_macros::verus_impl

*Function-like Macro*

Like verus!, but for use inside a (non-trait) impl

```rust
verus_impl!(...)
```



## verus_builtin_macros::verus_keep_ghost

*Function-like Macro*

```rust
verus_keep_ghost!(...)
```



## verus_builtin_macros::verus_proof_expr

*Function-like Macro*

```rust
verus_proof_expr!(...)
```



## verus_builtin_macros::verus_proof_macro_explicit_exprs

*Function-like Macro*

`verus_proof_macro_explicit_exprs!(f!(tts))` applies verus syntax to transform `tts` into
`tts'`, then returns `f!(tts')`, only applying the transform to any of the exprs within it that
are explicitly prefixed with `@@`, leaving the rest as-is. Contrast this to
[`verus_proof_macro_exprs`] which is likely what you want to try first to see if it satisfies
your needs.

```rust
verus_proof_macro_explicit_exprs!(...)
```



## verus_builtin_macros::verus_proof_macro_exprs

*Function-like Macro*

verus_proof_macro_exprs!(f!(exprs)) applies verus syntax to transform exprs into exprs',
then returns f!(exprs'),
where exprs is a sequence of expressions separated by ",", ";", and/or "=>".

```rust
verus_proof_macro_exprs!(...)
```



## verus_builtin_macros::verus_spec

*Attribute Macro*

```rust
#[verus_spec]
```



## verus_builtin_macros::verus_trait_impl

*Function-like Macro*

Like verus!, but for use inside a trait impl

```rust
verus_trait_impl!(...)
```



## verus_builtin_macros::verus_verify

*Attribute Macro*

```rust
#[verus_verify]
```



## verus_builtin_macros::vstd_kind

*Function*

```rust
fn vstd_kind() -> VstdKind
```



