**anstyle_parse > state**

# Module: state

## Contents

**Functions**

- [`state_change`](#state_change) - Transition to next [`State`]

---

## anstyle_parse::state::state_change

*Function*

Transition to next [`State`]

Note: This does not directly support UTF-8.
- If the data is validated as UTF-8 (e.g. `str`) or single-byte C1 control codes are
  unsupported, then treat [`Action::BeginUtf8`] and [`Action::Execute`] for UTF-8 continuations
  as [`Action::Print`].
- If the data is not validated, then a UTF-8 state machine will need to be implemented on top,
  starting with [`Action::BeginUtf8`].

Note: When [`State::Anywhere`] is returned, revert back to the prior state.

```rust
fn state_change(state: State, byte: u8) -> (State, Action)
```



