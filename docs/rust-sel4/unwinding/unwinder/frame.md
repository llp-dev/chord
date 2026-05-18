**unwinding > unwinder > frame**

# Module: unwinder::frame

## Contents

**Structs**

- [`Frame`](#frame)
- [`StoreOnStack`](#storeonstack)

**Functions**

- [`next_value`](#next_value)

---

## unwinding::unwinder::frame::Frame

*Struct*

**Fields:**
- `fde_result: super::find_fde::FDESearchResult`
- `row: gimli::UnwindTableRow<usize, StoreOnStack>`

**Methods:**

- `fn from_context(ctx: &Context, signal: bool) -> Result<Option<Self>, gimli::Error>`
- `fn evaluate_expression(self: &Self, _ctx: &Context, _expr: UnwindExpression<usize>) -> Result<usize, gimli::Error>`
- `fn adjust_stack_for_args(self: &Self, ctx: & mut Context)`
- `fn unwind(self: &Self, ctx: &Context) -> Result<Context, gimli::Error>`
- `fn bases(self: &Self) -> &BaseAddresses`
- `fn personality(self: &Self) -> Option<PersonalityRoutine>`
- `fn lsda(self: &Self) -> usize`
- `fn initial_address(self: &Self) -> usize`
- `fn is_signal_trampoline(self: &Self) -> bool`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## unwinding::unwinder::frame::StoreOnStack

*Struct*

**Unit Struct**

**Traits:** UnwindContextStorage



## unwinding::unwinder::frame::next_value

*Function*

```rust
fn next_value(x: usize) -> usize
```



