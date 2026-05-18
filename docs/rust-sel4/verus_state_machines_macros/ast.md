**verus_state_machines_macros > ast**

# Module: ast

## Contents

**Structs**

- [`Arm`](#arm) - like syn::Arm, without the body
- [`AssertProof`](#assertproof) - Extra info for generating the verification condition of a safety condition
- [`Extras`](#extras)
- [`Field`](#field)
- [`Invariant`](#invariant)
- [`Lemma`](#lemma)
- [`LemmaPurpose`](#lemmapurpose)
- [`SM`](#sm)
- [`SpecialOp`](#specialop)
- [`Transition`](#transition)
- [`TransitionParam`](#transitionparam)

**Enums**

- [`LemmaPurposeKind`](#lemmapurposekind)
- [`LetKind`](#letkind)
- [`MonoidElt`](#monoidelt)
- [`MonoidStmtType`](#monoidstmttype)
- [`PostConditionReason`](#postconditionreason)
- [`PostConditionReasonField`](#postconditionreasonfield)
- [`ShardableType`](#shardabletype) - These represent the types of the state machine fields,
- [`SimplStmt`](#simplstmt)
- [`SplitKind`](#splitkind) - The 'Split' is a generic node that handles any kind of control flow or variable binding.
- [`SubIdx`](#subidx)
- [`TransitionKind`](#transitionkind)
- [`TransitionStmt`](#transitionstmt)

**Constants**

- [`INIT_LABEL_TYPE_NAME`](#init_label_type_name)
- [`TRANSITION_LABEL_TYPE_NAME`](#transition_label_type_name)

---

## verus_state_machines_macros::ast::Arm

*Struct*

like syn::Arm, without the body

**Fields:**
- `pat: verus_syn::Pat`
- `guard: Option<(token::If, Box<verus_syn::Expr>)>`
- `fat_arrow_token: token::FatArrow`
- `comma: Option<token::Comma>`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Arm`



## verus_state_machines_macros::ast::AssertProof

*Struct*

Extra info for generating the verification condition of a safety condition

**Fields:**
- `proof: Option<std::rc::Rc<verus_syn::Block>>`
- `error_msg: String`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> AssertProof`



## verus_state_machines_macros::ast::Extras

*Struct*

**Fields:**
- `invariants: Vec<Invariant>`
- `lemmas: Vec<Lemma>`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Extras`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## verus_state_machines_macros::ast::Field

*Struct*

**Fields:**
- `name: verus_syn::Ident`
- `stype: ShardableType`
- `type_span: proc_macro2::Span`

**Methods:**

- `fn get_type(self: &Self) -> Type`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Field`



## verus_state_machines_macros::ast::INIT_LABEL_TYPE_NAME

*Constant*: `&str`



## verus_state_machines_macros::ast::Invariant

*Struct*

**Fields:**
- `func: verus_syn::ImplItemFn`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Invariant`



## verus_state_machines_macros::ast::Lemma

*Struct*

**Fields:**
- `purpose: LemmaPurpose`
- `func: verus_syn::ImplItemFn`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Lemma`



## verus_state_machines_macros::ast::LemmaPurpose

*Struct*

**Fields:**
- `transition: verus_syn::Ident`
- `kind: LemmaPurposeKind`

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> LemmaPurpose`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## verus_state_machines_macros::ast::LemmaPurposeKind

*Enum*

**Variants:**
- `PreservesInvariant`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> LemmaPurposeKind`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## verus_state_machines_macros::ast::LetKind

*Enum*

**Variants:**
- `Normal`
- `BirdsEye`

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &LetKind) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> LetKind`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## verus_state_machines_macros::ast::MonoidElt

*Enum*

**Variants:**
- `True` - Represents the element `true`
- `OptionSome(Option<verus_syn::Expr>)` - Represents the element Some(e)
- `SingletonKV(verus_syn::Expr, Option<Box<verus_syn::Expr>>)` - Represents the singleton map [k => v]
- `SingletonMultiset(verus_syn::Expr)` - Represents the singleton multiset {e}
- `SingletonSet(verus_syn::Expr)` - Represents the set multiset {e}
- `General(verus_syn::Expr)` - Represents e

**Methods:**

- `fn syntax(self: &Self) -> &'static str`
- `fn type_name(self: &Self) -> &'static str`
- `fn is_general(self: &Self) -> bool`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> MonoidElt`



## verus_state_machines_macros::ast::MonoidStmtType

*Enum*

**Variants:**
- `Have`
- `Add(bool)`
- `Remove`
- `Guard`
- `Deposit`
- `Withdraw`

**Methods:**

- `fn name(self: Self) -> &'static str`
- `fn is_for_storage(self: Self) -> bool`
- `fn is_modifier(self: Self) -> bool`
- `fn is_have(self: Self) -> bool`
- `fn is_deposit(self: Self) -> bool`
- `fn is_remove(self: Self) -> bool`
- `fn is_add(self: Self) -> bool`
- `fn is_guard(self: Self) -> bool`
- `fn is_withdraw(self: Self) -> bool`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> MonoidStmtType`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## verus_state_machines_macros::ast::PostConditionReason

*Enum*

**Variants:**
- `FieldValue(PostConditionReasonField, String)`

**Methods:**

- `fn to_err_msg(self: &Self) -> String`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> PostConditionReason`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## verus_state_machines_macros::ast::PostConditionReasonField

*Enum*

**Variants:**
- `Update`
- `NoUpdateConditional`
- `NoUpdateTopLevel`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> PostConditionReasonField`



## verus_state_machines_macros::ast::SM

*Struct*

**Fields:**
- `attrs: Vec<verus_syn::Attribute>`
- `name: verus_syn::Ident`
- `generics: Option<verus_syn::Generics>`
- `fields: Vec<Field>`
- `fields_named_ast: verus_syn::FieldsNamed`
- `transitions: Vec<Transition>`
- `concurrent: bool`
- `transition_label: Option<verus_syn::Item>`
- `init_label: Option<verus_syn::Item>`

**Methods:**

- `fn get_label_generics<'a>(self: &'a Self, is_init: bool) -> &'a Generics`
- `fn get_label_generics_opt<'a>(self: &'a Self, is_init: bool) -> Option<&'a Generics>`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> SM`



## verus_state_machines_macros::ast::ShardableType

*Enum*

These represent the types of the state machine fields,
along with their sharding strategies.
(For non-tokenized state machines, we just say everything
is 'Variable'.)

Be aware of the relationship between the enum representation here
and the user's field declarations. As an example, a user's declaration
might look like `#[sharding(option)] foo: Option<Foo>`.
Recall the user is expected to declare a type of a certain form which
depends on the sharding strategy; e.g., in `#[sharding(option)]`, the
user is required to declare a type of the form `Option<something>`.

In the representation here, we actually "destruct" user-provided type,
and just represent it as `Option(Foo)` (not `Option(Option<Foo>)`).
This way, we can easily talk about `Foo` directly when necessary,
while we can easily reconstruct the user-provided type (see `shardable_type_to_type`).

**Variants:**
- `Variable(verus_syn::Type)`
- `Constant(verus_syn::Type)`
- `NotTokenized(verus_syn::Type)`
- `Option(verus_syn::Type)`
- `Map(verus_syn::Type, verus_syn::Type)`
- `Multiset(verus_syn::Type)`
- `Set(verus_syn::Type)`
- `Count`
- `Bool`
- `PersistentMap(verus_syn::Type, verus_syn::Type)`
- `PersistentOption(verus_syn::Type)`
- `PersistentSet(verus_syn::Type)`
- `PersistentCount`
- `PersistentBool`
- `StorageOption(verus_syn::Type)`
- `StorageMap(verus_syn::Type, verus_syn::Type)`

**Methods:**

- `fn strategy_name(self: &Self) -> &str` - get the name the user uses in the field declarations to declare the given strategy
- `fn is_count(self: &Self) -> bool`
- `fn is_storage(self: &Self) -> bool`
- `fn is_persistent(self: &Self) -> bool`
- `fn get_option_param_type(self: &Self) -> Type`
- `fn get_multiset_param_type(self: &Self) -> Type`
- `fn get_map_key_type(self: &Self) -> Type`
- `fn get_map_value_type(self: &Self) -> Type`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ShardableType`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## verus_state_machines_macros::ast::SimplStmt

*Enum*

**Variants:**
- `Let(proc_macro2::Span, Box<verus_syn::Pat>, Option<Box<verus_syn::Type>>, verus_syn::Expr, Vec<SimplStmt>, Vec<verus_syn::Ident>)`
- `Split(proc_macro2::Span, Box<SplitKind>, Vec<(proc_macro2::Span, Vec<SimplStmt>)>, Vec<verus_syn::Ident>)`
- `Require(proc_macro2::Span, verus_syn::Expr)`
- `PostCondition(proc_macro2::Span, verus_syn::Expr, PostConditionReason)`
- `Assert(proc_macro2::Span, verus_syn::Expr, AssertProof)`
- `Assign(proc_macro2::Span, verus_syn::Ident, Box<verus_syn::Type>, verus_syn::Expr, bool)`

**Methods:**

- `fn get_span(self: &Self) -> Span`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> SimplStmt`



## verus_state_machines_macros::ast::SpecialOp

*Struct*

**Fields:**
- `stmt: MonoidStmtType`
- `elt: MonoidElt`

**Methods:**

- `fn is_modifier(self: &Self) -> bool` - returns 'true' if the operational definition of the operation
- `fn is_only_allowed_in_property_or_readonly(self: &Self) -> bool`
- `fn is_have(self: &Self) -> bool`
- `fn is_deposit(self: &Self) -> bool`
- `fn is_remove(self: &Self) -> bool`
- `fn is_add(self: &Self) -> bool`
- `fn is_guard(self: &Self) -> bool`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> SpecialOp`



## verus_state_machines_macros::ast::SplitKind

*Enum*

The 'Split' is a generic node that handles any kind of control flow or variable binding.
The variables being bound are determined by any Pats in the contents of the SplitKind.
Meanwhile each node has n children depending on the SplitKind.

  * If - 2 children
  * Match - 1 child per arm
  * Let - 1 child
  * Special - 1 child
     * Note that many (most, in fact) special nodes don't bind any variables at all.
       If the special node doesn't have a Pat then we always construct the node
       so that its child is an empty block.

**Variants:**
- `If(verus_syn::Expr)`
- `Match(verus_syn::Expr, Vec<Arm>)`
- `Let(Box<verus_syn::Pat>, Option<Box<verus_syn::Type>>, LetKind, verus_syn::Expr)`
- `Special(verus_syn::Ident, SpecialOp, AssertProof, Option<Box<verus_syn::Pat>>)` - concurrent-state-machine-specific stuff

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> SplitKind`



## verus_state_machines_macros::ast::SubIdx

*Enum*

**Variants:**
- `Field(verus_syn::Ident)`
- `Idx(Box<verus_syn::Expr>)`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> SubIdx`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## verus_state_machines_macros::ast::TRANSITION_LABEL_TYPE_NAME

*Constant*: `&str`



## verus_state_machines_macros::ast::Transition

*Struct*

**Fields:**
- `name: verus_syn::Ident`
- `kind: TransitionKind`
- `params: Vec<TransitionParam>`
- `body: TransitionStmt`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Transition`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## verus_state_machines_macros::ast::TransitionKind

*Enum*

**Variants:**
- `Init`
- `Transition`
- `ReadonlyTransition` - Like a transition, but it can't update anything.
- `Property` - This is sort of like a readonly transition, but it

**Methods:**

- `fn requires_invariant_lemma(self: &Self) -> bool`

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &TransitionKind) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> TransitionKind`



## verus_state_machines_macros::ast::TransitionParam

*Struct*

**Fields:**
- `name: verus_syn::Ident`
- `ty: verus_syn::Type`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> TransitionParam`



## verus_state_machines_macros::ast::TransitionStmt

*Enum*

**Variants:**
- `Block(proc_macro2::Span, Vec<TransitionStmt>)`
- `Split(proc_macro2::Span, Box<SplitKind>, Vec<TransitionStmt>)`
- `Require(proc_macro2::Span, verus_syn::Expr)`
- `Assert(proc_macro2::Span, verus_syn::Expr, AssertProof)`
- `Update(proc_macro2::Span, verus_syn::Ident, verus_syn::Expr)`
- `SubUpdate(proc_macro2::Span, verus_syn::Ident, Vec<SubIdx>, verus_syn::Expr)`
- `Initialize(proc_macro2::Span, verus_syn::Ident, verus_syn::Expr)`
- `PostCondition(proc_macro2::Span, verus_syn::Expr)` - Different than an Assert - this statement is allowed to depend on output values.

**Methods:**

- `fn get_span<'a>(self: &'a Self) -> &'a Span`
- `fn statement_name(self: &Self) -> &'static str`
- `fn is_trivial(self: &Self) -> bool`

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> TransitionStmt`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



