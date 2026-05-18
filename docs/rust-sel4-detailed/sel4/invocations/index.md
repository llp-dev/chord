*[sel4](../index.md) / [invocations](index.md)*

---

# Module `invocations`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TcbFlagsBuilder`](#tcbflagsbuilder) | struct |  |
| [`USER_CONTEXT_MAX_REG_COUNT`](#user-context-max-reg-count) | const |  |

## Structs

### `TcbFlagsBuilder`

```rust
struct TcbFlagsBuilder(crate::Word);
```

#### Implementations

- <span id="tcbflagsbuilder-new"></span>`fn new() -> Self`

- <span id="tcbflagsbuilder-build"></span>`fn build(self) -> Word` — [`Word`](../index.md#word)

- <span id="tcbflagsbuilder-fpu-disabled"></span>`fn fpu_disabled(self, val: bool) -> Self`

- <span id="tcbflagsbuilder-apply-flag-val"></span>`fn apply_flag_val(self, flag: Word, val: bool) -> Self` — [`Word`](../index.md#word)

#### Trait Implementations

##### `impl Clone for TcbFlagsBuilder`

- <span id="tcbflagsbuilder-clone"></span>`fn clone(&self) -> TcbFlagsBuilder` — [`TcbFlagsBuilder`](#tcbflagsbuilder)

##### `impl Debug for TcbFlagsBuilder`

- <span id="tcbflagsbuilder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for TcbFlagsBuilder`

- <span id="tcbflagsbuilder-default"></span>`fn default() -> Self`

##### `impl PartialEq for TcbFlagsBuilder`

- <span id="tcbflagsbuilder-partialeq-eq"></span>`fn eq(&self, other: &TcbFlagsBuilder) -> bool` — [`TcbFlagsBuilder`](#tcbflagsbuilder)

##### `impl StructuralPartialEq for TcbFlagsBuilder`

## Constants

### `USER_CONTEXT_MAX_REG_COUNT`
```rust
const USER_CONTEXT_MAX_REG_COUNT: usize = 20usize;
```

