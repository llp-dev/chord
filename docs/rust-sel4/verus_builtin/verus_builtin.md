**verus_builtin**

# Module: verus_builtin

## Contents

**Macros**

- [`decreases_to`](#decreases_to) - decreases_to!(b => a) means that height(a) < height(b), so that b can decrease to a
- [`decreases_to_internal`](#decreases_to_internal)
- [`impl_binary_op`](#impl_binary_op)
- [`impl_binary_op_nat`](#impl_binary_op_nat)
- [`impl_binary_op_rhs`](#impl_binary_op_rhs)
- [`impl_ieee_float`](#impl_ieee_float)
- [`impl_ieee_float_cast`](#impl_ieee_float_cast)
- [`impl_ord`](#impl_ord)
- [`impl_ord_self_rhs`](#impl_ord_self_rhs)
- [`impl_structural`](#impl_structural)
- [`impl_unary_op`](#impl_unary_op)
- [`with_triggers`](#with_triggers)

**Structs**

- [`FnProof`](#fnproof) - FnProof is the type of proof closures; the syntax proof_fn is used to wrap FnProof
- [`FnSpec`](#fnspec)
- [`Ghost`](#ghost)
- [`NoCopy`](#nocopy)
- [`NoSyncSend`](#nosyncsend)
- [`ProofFnConfirm`](#prooffnconfirm)
- [`SpecChain`](#specchain)
- [`Tracked`](#tracked)
- [`int`](#int)
- [`nat`](#nat)
- [`real`](#real)

**Traits**

- [`Boolean`](#boolean)
- [`Chainable`](#chainable)
- [`Decimal`](#decimal)
- [`IeeeFloat`](#ieeefloat)
- [`IeeeFloatCast`](#ieeefloatcast)
- [`Integer`](#integer)
- [`ProofFn`](#prooffn)
- [`ProofFnMut`](#prooffnmut)
- [`ProofFnOnce`](#prooffnonce)
- [`ProofFnReqEns`](#prooffnreqens)
- [`ProofFnReqEnsDef`](#prooffnreqensdef)
- [`SpecAdd`](#specadd)
- [`SpecBitAnd`](#specbitand)
- [`SpecBitOr`](#specbitor)
- [`SpecBitXor`](#specbitxor)
- [`SpecEuclideanMod`](#speceuclideanmod)
- [`SpecEuclideanOrRealDiv`](#speceuclideanorrealdiv)
- [`SpecMul`](#specmul)
- [`SpecNeg`](#specneg)
- [`SpecOrd`](#specord)
- [`SpecShl`](#specshl)
- [`SpecShr`](#specshr)
- [`SpecSub`](#specsub)
- [`Structural`](#structural) - derive(Structural) means that exec-mode == and ghost == always yield the same result.

**Constants**

- [`PROOF_FN`](#proof_fn)
- [`PROOF_FN_COPY`](#proof_fn_copy)
- [`PROOF_FN_MUT`](#proof_fn_mut)
- [`PROOF_FN_ONCE`](#proof_fn_once)
- [`PROOF_FN_SEND`](#proof_fn_send)
- [`PROOF_FN_SYNC`](#proof_fn_sync)

---

## verus_builtin::Boolean

*Trait*

**Methods:**

- `CONST_DEFAULT`



## verus_builtin::Chainable

*Trait*



## verus_builtin::Decimal

*Trait*

**Methods:**

- `CONST_DEFAULT`



## verus_builtin::FnProof

*Struct*

FnProof is the type of proof closures; the syntax proof_fn is used to wrap FnProof

**Generic Parameters:**
- 'a
- Options
- ArgModes
- OutMode
- Args
- Output

**Fields:**
- `_no_sync_send: NoSyncSend`
- `_lifetime: core::marker::PhantomData<&'a fn(...)>`
- `_options: core::marker::PhantomData<Options>`
- `_arg_modes: core::marker::PhantomData<ArgModes>`
- `_out_mode: core::marker::PhantomData<OutMode>`

**Traits:** Sync, Copy, Send

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`



## verus_builtin::FnSpec

*Struct*

**Generic Parameters:**
- Args
- Output

**Fields:**
- `phantom: core::marker::PhantomData<(Args, Output)>`



## verus_builtin::Ghost

*Struct*

**Generic Parameters:**
- A

**Fields:**
- `phantom: core::marker::PhantomData<A>`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`



## verus_builtin::IeeeFloat

*Trait*



## verus_builtin::IeeeFloatCast

*Trait*



## verus_builtin::Integer

*Trait*

**Methods:**

- `CONST_DEFAULT`



## verus_builtin::NoCopy

*Struct*



## verus_builtin::NoSyncSend

*Struct*

**Fields:**
- `_no_send_sync: core::marker::PhantomData<*const ()>`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> NoSyncSend`



## verus_builtin::PROOF_FN

*Constant*: `u8`



## verus_builtin::PROOF_FN_COPY

*Constant*: `u8`



## verus_builtin::PROOF_FN_MUT

*Constant*: `u8`



## verus_builtin::PROOF_FN_ONCE

*Constant*: `u8`



## verus_builtin::PROOF_FN_SEND

*Constant*: `u8`



## verus_builtin::PROOF_FN_SYNC

*Constant*: `u8`



## verus_builtin::ProofFn

*Trait*



## verus_builtin::ProofFnConfirm

*Struct*

**Unit Struct**



## verus_builtin::ProofFnMut

*Trait*



## verus_builtin::ProofFnOnce

*Trait*



## verus_builtin::ProofFnReqEns

*Trait*



## verus_builtin::ProofFnReqEnsDef

*Trait*



## verus_builtin::SpecAdd

*Trait*

**Methods:**

- `Output`



## verus_builtin::SpecBitAnd

*Trait*

**Methods:**

- `Output`



## verus_builtin::SpecBitOr

*Trait*

**Methods:**

- `Output`



## verus_builtin::SpecBitXor

*Trait*

**Methods:**

- `Output`



## verus_builtin::SpecChain

*Struct*

**Fields:**
- `data: core::marker::PhantomData<int>`



## verus_builtin::SpecEuclideanMod

*Trait*

**Methods:**

- `Output`



## verus_builtin::SpecEuclideanOrRealDiv

*Trait*

**Methods:**

- `Output`



## verus_builtin::SpecMul

*Trait*

**Methods:**

- `Output`



## verus_builtin::SpecNeg

*Trait*

**Methods:**

- `Output`



## verus_builtin::SpecOrd

*Trait*



## verus_builtin::SpecShl

*Trait*

**Methods:**

- `Output`



## verus_builtin::SpecShr

*Trait*

**Methods:**

- `Output`



## verus_builtin::SpecSub

*Trait*

**Methods:**

- `Output`



## verus_builtin::Structural

*Trait*

derive(Structural) means that exec-mode == and ghost == always yield the same result.
derive(Structural) is only allowed when all the fields of a type are also Structural.
derive(StructuralEq) means derive(Structural) and also implement PartialEqSpec,
setting eq_spec to == and obeys_eq_spec to true.



## verus_builtin::Tracked

*Struct*

**Generic Parameters:**
- A

**Fields:**
- `phantom: core::marker::PhantomData<A>`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, _: & mut core::fmt::Formatter) -> core::fmt::Result`



## verus_builtin::decreases_to

*Declarative Macro*

decreases_to!(b => a) means that height(a) < height(b), so that b can decrease to a
in decreases clauses.
decreases_to!(b1, ..., bn => a1, ..., am) can compare lexicographically ordered values,
which can be useful when making assertions about decreases clauses.
Notes:
- decreases_to! desugars to a call to is_smaller_than_lexicographic.
- you can write #[trigger](decreases_to!(b => a)) to trigger on height(a).
  (in the SMT encoding, height is a function call and is a useful trigger,
  while is_smaller_than/is_smaller_than_lexicographic is not a function call
  and is not a useful trigger.)

```rust
macro_rules! decreases_to {
    ($($x:tt)*) => { ... };
}
```



## verus_builtin::decreases_to_internal

*Declarative Macro*

```rust
macro_rules! decreases_to_internal {
    ($($x:expr),* $(,)? => $($y:expr),* $(,)?) => { ... };
}
```



## verus_builtin::impl_binary_op

*Declarative Macro*

```rust
macro_rules! impl_binary_op {
    ($trt:ident, $fun:ident, $ret:ty, [$($t:ty)*]) => { ... };
}
```



## verus_builtin::impl_binary_op_nat

*Declarative Macro*

```rust
macro_rules! impl_binary_op_nat {
    ($trt:ident, $fun:ident, $ret:ty, [$($t:ty)*]) => { ... };
}
```



## verus_builtin::impl_binary_op_rhs

*Declarative Macro*

```rust
macro_rules! impl_binary_op_rhs {
    ($trt:ident, $fun:ident, $rhs: ty, $ret:ty, [$($t:ty)*]) => { ... };
}
```



## verus_builtin::impl_ieee_float

*Declarative Macro*

```rust
macro_rules! impl_ieee_float {
    ([$($t:ty)*]) => { ... };
}
```



## verus_builtin::impl_ieee_float_cast

*Declarative Macro*

```rust
macro_rules! impl_ieee_float_cast {
    ($from:ty, [$($to:ty)*]) => { ... };
}
```



## verus_builtin::impl_ord

*Declarative Macro*

```rust
macro_rules! impl_ord {
    ([$($t:ty)*]) => { ... };
}
```



## verus_builtin::impl_ord_self_rhs

*Declarative Macro*

```rust
macro_rules! impl_ord_self_rhs {
    ([$($t:ty)*]) => { ... };
}
```



## verus_builtin::impl_structural

*Declarative Macro*

```rust
macro_rules! impl_structural {
    ($($t:ty)*) => { ... };
}
```



## verus_builtin::impl_unary_op

*Declarative Macro*

```rust
macro_rules! impl_unary_op {
    ($trt:ident, $fun:ident, $ret:ty, [$($t:ty)*]) => { ... };
}
```



## verus_builtin::int

*Struct*

**Unit Struct**

**Traits:** Copy, SpecAdd, SpecOrd, SpecEuclideanMod, Structural, SpecMul, Eq, SpecNeg, Integer, SpecEuclideanOrRealDiv, SpecSub

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, _other: &Self) -> core::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, _other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> int`
- **Neg**
  - `fn neg(self: Self) -> <Self as >::Output`
- **Rem**
  - `fn rem(self: Self, _other: Self) -> <Self as >::Output`
- **Div**
  - `fn div(self: Self, _other: Self) -> <Self as >::Output`
- **Mul**
  - `fn mul(self: Self, _other: Self) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, _other: Self) -> <Self as >::Output`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, _other: &Self) -> Option<core::cmp::Ordering>`
- **Add**
  - `fn add(self: Self, _other: Self) -> <Self as >::Output`



## verus_builtin::nat

*Struct*

**Unit Struct**

**Traits:** SpecAdd, SpecAdd, SpecAdd, SpecAdd, SpecEuclideanMod, SpecAdd, SpecEuclideanOrRealDiv, SpecAdd, SpecMul, SpecAdd, SpecMul, SpecNeg, SpecMul, SpecOrd, SpecMul, SpecMul, SpecMul, SpecMul, Eq, SpecMul, SpecAdd, SpecMul, SpecMul, SpecMul, SpecMul, SpecMul, SpecSub, SpecAdd, SpecAdd, SpecMul, Copy, Integer, SpecAdd, Structural, SpecAdd, SpecAdd, SpecAdd

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, _other: &Self) -> bool`
- **Rem**
  - `fn rem(self: Self, _other: Self) -> <Self as >::Output`
- **Mul**
  - `fn mul(self: Self, _other: Self) -> <Self as >::Output`
- **Add**
  - `fn add(self: Self, _other: Self) -> <Self as >::Output`
- **Clone**
  - `fn clone(self: &Self) -> nat`
- **Ord**
  - `fn cmp(self: &Self, _other: &Self) -> core::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, _other: &Self) -> Option<core::cmp::Ordering>`
- **Div**
  - `fn div(self: Self, _other: Self) -> <Self as >::Output`
- **Sub**
  - `fn sub(self: Self, _other: Self) -> <Self as >::Output`



## verus_builtin::real

*Struct*

**Unit Struct**

**Traits:** Chainable, Decimal, Copy, IeeeFloatCast, SpecEuclideanOrRealDiv, SpecMul, SpecSub, SpecAdd, SpecNeg, SpecOrd, IeeeFloatCast

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> real`



## verus_builtin::with_triggers

*Declarative Macro*

```rust
macro_rules! with_triggers {
    ( $([ $($term:expr),* ]),* => $body:expr) => { ... };
}
```



