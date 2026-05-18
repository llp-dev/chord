*[unwinding](../../index.md) / [unwinder](../index.md) / [frame](index.md)*

---

# Module `frame`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`StoreOnStack`](#storeonstack) | struct |  |
| [`Frame`](#frame) | struct |  |
| [`next_value`](#next-value) | fn |  |

## Structs

### `StoreOnStack`

```rust
struct StoreOnStack;
```

#### Trait Implementations

##### `impl<O: gimli::ReaderOffset> UnwindContextStorage for StoreOnStack`

- <span id="storeonstack-unwindcontextstorage-type-rules"></span>`type Rules = [(Register, RegisterRule<O>); 32]`

- <span id="storeonstack-unwindcontextstorage-type-stack"></span>`type Stack = [UnwindTableRow<O, StoreOnStack>; 2]`

### `Frame`

```rust
struct Frame {
    fde_result: super::find_fde::FDESearchResult,
    row: gimli::UnwindTableRow<usize, StoreOnStack>,
}
```

#### Implementations

- <span id="frame-from-context"></span>`fn from_context(ctx: &Context, signal: bool) -> Result<Option<Self>, gimli::Error>` — [`Context`](../arch/index.md#context)

- <span id="frame-evaluate-expression"></span>`fn evaluate_expression(&self, _ctx: &Context, _expr: UnwindExpression<usize>) -> Result<usize, gimli::Error>` — [`Context`](../arch/index.md#context)

- <span id="frame-adjust-stack-for-args"></span>`fn adjust_stack_for_args(&self, ctx: &mut Context)` — [`Context`](../arch/index.md#context)

- <span id="frame-unwind"></span>`fn unwind(&self, ctx: &Context) -> Result<Context, gimli::Error>` — [`Context`](../arch/index.md#context)

- <span id="frame-bases"></span>`fn bases(&self) -> &BaseAddresses`

- <span id="frame-personality"></span>`fn personality(&self) -> Option<PersonalityRoutine>` — [`PersonalityRoutine`](../../abi/index.md#personalityroutine)

- <span id="frame-lsda"></span>`fn lsda(&self) -> usize`

- <span id="frame-initial-address"></span>`fn initial_address(&self) -> usize`

- <span id="frame-is-signal-trampoline"></span>`fn is_signal_trampoline(&self) -> bool`

#### Trait Implementations

##### `impl Debug for Frame`

- <span id="frame-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

### `next_value`

```rust
const fn next_value(x: usize) -> usize
```

