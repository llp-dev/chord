*[clap_builder](../../index.md) / [builder](../index.md) / [possible_value](index.md)*

---

# Module `possible_value`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`PossibleValue`](#possiblevalue) | struct | A possible value of an argument. |

## Structs

### `PossibleValue`

```rust
struct PossibleValue {
    name: crate::builder::Str,
    help: Option<crate::builder::StyledStr>,
    aliases: Vec<crate::builder::Str>,
    hide: bool,
}
```

A possible value of an argument.

This is used for specifying [possible values] of [Args].

See also `PossibleValuesParser`

<div class="warning">

**NOTE:** Most likely you can use strings, rather than `PossibleValue` as it is only required
to [hide] single values from help messages and shell completions or to attach [`help`](../../output/help/index.md) to
possible values.

</div>

# Examples

```rust
use clap_builder as clap;
use clap::{Arg, builder::PossibleValue, ArgAction};
let cfg = Arg::new("config")
    .action(ArgAction::Set)
    .value_name("FILE")
    .value_parser([
        PossibleValue::new("fast"),
        PossibleValue::new("slow").help("slower than fast"),
        PossibleValue::new("secret speed").hide(true)
    ]);
```





#### Implementations

- <span id="possiblevalue-new"></span>`fn new(name: impl Into<Str>) -> Self` — [`Str`](../str/index.md#str)

  Create a [`PossibleValue`](#possiblevalue) with its name.

  

  The name will be used to decide whether this value was provided by the user to an argument.

  

  <div class="warning">

  

  **NOTE:** In case it is not [hidden] it will also be shown in help messages for arguments

  that use it as a [possible value] and have not hidden them through `Arg::hide_possible_values(true)`.

  

  </div>

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::builder::PossibleValue;

  PossibleValue::new("fast")

  ;

  ```

  

  

- <span id="possiblevalue-help"></span>`fn help(self, help: impl IntoResettable<StyledStr>) -> Self` — [`IntoResettable`](../resettable/index.md#intoresettable), [`StyledStr`](../styled_str/index.md#styledstr)

  Sets the help description of the value.

  

  This is typically displayed in completions (where supported) and should be a short, one-line

  description.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::builder::PossibleValue;

  PossibleValue::new("slow")

      .help("not fast")

  ;

  ```

- <span id="possiblevalue-hide"></span>`fn hide(self, yes: bool) -> Self`

  Hides this value from help and shell completions.

  

  This is an alternative to hiding through `Arg::hide_possible_values(true)`, if you only

  want to hide some values.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::builder::PossibleValue;

  PossibleValue::new("secret")

      .hide(true)

  ;

  ```

- <span id="possiblevalue-alias"></span>`fn alias(self, name: impl IntoResettable<Str>) -> Self` — [`IntoResettable`](../resettable/index.md#intoresettable), [`Str`](../str/index.md#str)

  Sets a *hidden* alias for this argument value.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::builder::PossibleValue;

  PossibleValue::new("slow")

      .alias("not-fast")

  ;

  ```

- <span id="possiblevalue-aliases"></span>`fn aliases(self, names: impl IntoIterator<Item = impl Into<Str>>) -> Self` — [`Str`](../str/index.md#str)

  Sets multiple *hidden* aliases for this argument value.

  

  # Examples

  

  ```rust

  use clap_builder as clap;

  use clap::builder::PossibleValue;

  PossibleValue::new("slow")

      .aliases(["not-fast", "snake-like"])

  ;

  ```

#### Trait Implementations

##### `impl Clone for PossibleValue`

- <span id="possiblevalue-clone"></span>`fn clone(&self) -> PossibleValue` — [`PossibleValue`](#possiblevalue)

##### `impl Debug for PossibleValue`

- <span id="possiblevalue-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for PossibleValue`

- <span id="possiblevalue-default"></span>`fn default() -> PossibleValue` — [`PossibleValue`](#possiblevalue)

##### `impl Eq for PossibleValue`

##### `impl PartialEq for PossibleValue`

- <span id="possiblevalue-partialeq-eq"></span>`fn eq(&self, other: &PossibleValue) -> bool` — [`PossibleValue`](#possiblevalue)

##### `impl StructuralPartialEq for PossibleValue`

