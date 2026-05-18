*[anstyle](../index.md) / [effect](index.md)*

---

# Module `effect`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Effects`](#effects) | struct | A set of text effects |
| [`Metadata`](#metadata) | struct |  |
| [`EffectsDisplay`](#effectsdisplay) | struct |  |
| [`EffectIter`](#effectiter) | struct | Enumerate each enabled value in [`Effects`] |
| [`EffectIndexIter`](#effectindexiter) | struct |  |
| [`METADATA`](#metadata) | const |  |

## Structs

### `Effects`

```rust
struct Effects(u16);
```

A set of text effects

# Examples

```rust
let effects = anstyle::Effects::BOLD | anstyle::Effects::UNDERLINE;
```

#### Implementations

- <span id="effects-const-plain"></span>`const PLAIN: Self`

- <span id="effects-const-bold"></span>`const BOLD: Self`

- <span id="effects-const-dimmed"></span>`const DIMMED: Self`

- <span id="effects-const-italic"></span>`const ITALIC: Self`

- <span id="effects-const-underline"></span>`const UNDERLINE: Self`

- <span id="effects-const-double-underline"></span>`const DOUBLE_UNDERLINE: Self`

- <span id="effects-const-curly-underline"></span>`const CURLY_UNDERLINE: Self`

- <span id="effects-const-dotted-underline"></span>`const DOTTED_UNDERLINE: Self`

- <span id="effects-const-dashed-underline"></span>`const DASHED_UNDERLINE: Self`

- <span id="effects-const-blink"></span>`const BLINK: Self`

- <span id="effects-const-invert"></span>`const INVERT: Self`

- <span id="effects-const-hidden"></span>`const HIDDEN: Self`

- <span id="effects-const-strikethrough"></span>`const STRIKETHROUGH: Self`

- <span id="effects-new"></span>`const fn new() -> Self`

  No effects enabled

  

  # Examples

  

  ```rust

  let effects = anstyle::Effects::new();

  ```

- <span id="effects-is-plain"></span>`const fn is_plain(self) -> bool`

  Check if no effects are enabled

  

  # Examples

  

  ```rust

  let effects = anstyle::Effects::new();

  assert!(effects.is_plain());

  

  let effects = anstyle::Effects::BOLD | anstyle::Effects::UNDERLINE;

  assert!(!effects.is_plain());

  ```

- <span id="effects-contains"></span>`const fn contains(self, other: Effects) -> bool` — [`Effects`](../index.md#effects)

  Returns `true` if all of the effects in `other` are contained within `self`.

  

  # Examples

  

  ```rust

  let effects = anstyle::Effects::BOLD | anstyle::Effects::UNDERLINE;

  assert!(effects.contains(anstyle::Effects::BOLD));

  

  let effects = anstyle::Effects::new();

  assert!(!effects.contains(anstyle::Effects::BOLD));

  ```

- <span id="effects-insert"></span>`const fn insert(self, other: Effects) -> Self` — [`Effects`](../index.md#effects)

  Inserts the specified effects in-place.

  

  # Examples

  

  ```rust

  let effects = anstyle::Effects::new().insert(anstyle::Effects::new());

  assert!(effects.is_plain());

  

  let effects = anstyle::Effects::new().insert(anstyle::Effects::BOLD);

  assert!(effects.contains(anstyle::Effects::BOLD));

  ```

- <span id="effects-remove"></span>`const fn remove(self, other: Effects) -> Self` — [`Effects`](../index.md#effects)

  Removes the specified effects in-place.

  

  # Examples

  

  ```rust

  let effects = (anstyle::Effects::BOLD | anstyle::Effects::UNDERLINE).remove(anstyle::Effects::BOLD);

  assert!(!effects.contains(anstyle::Effects::BOLD));

  assert!(effects.contains(anstyle::Effects::UNDERLINE));

  ```

- <span id="effects-clear"></span>`const fn clear(self) -> Self`

  Reset all effects in-place

  ```rust

  let effects = (anstyle::Effects::BOLD | anstyle::Effects::UNDERLINE).clear();

  assert!(!effects.contains(anstyle::Effects::BOLD));

  assert!(!effects.contains(anstyle::Effects::UNDERLINE));

  ```

- <span id="effects-set"></span>`const fn set(self, other: Self, enable: bool) -> Self`

  Enable or disable the specified effects depending on the passed value.

  

  # Examples

  

  ```rust

  let effects = anstyle::Effects::new().set(anstyle::Effects::BOLD, true);

  assert!(effects.contains(anstyle::Effects::BOLD));

  ```

- <span id="effects-iter"></span>`fn iter(self) -> EffectIter` — [`EffectIter`](../index.md#effectiter)

  Iterate over enabled effects

- <span id="effects-index-iter"></span>`fn index_iter(self) -> EffectIndexIter` — [`EffectIndexIter`](#effectindexiter)

  Iterate over enabled effect indices

- <span id="effects-render"></span>`fn render(self) -> impl core::fmt::Display + Copy`

  Render the ANSI code

- <span id="effects-write-to"></span>`fn write_to(self, write: &mut dyn std::io::Write) -> std::io::Result<()>`

#### Trait Implementations

##### `impl BitOr for Effects`

- <span id="effects-bitor-type-output"></span>`type Output = Effects`

- <span id="effects-bitor"></span>`fn bitor(self, rhs: Self) -> Self`

##### `impl BitOrAssign for Effects`

- <span id="effects-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: Self)`

##### `impl Clone for Effects`

- <span id="effects-clone"></span>`fn clone(&self) -> Effects` — [`Effects`](../index.md#effects)

##### `impl Copy for Effects`

##### `impl Debug for Effects`

- <span id="effects-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for Effects`

- <span id="effects-default"></span>`fn default() -> Effects` — [`Effects`](../index.md#effects)

##### `impl Eq for Effects`

##### `impl Hash for Effects`

- <span id="effects-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Effects`

- <span id="effects-ord-cmp"></span>`fn cmp(&self, other: &Effects) -> cmp::Ordering` — [`Effects`](../index.md#effects)

##### `impl PartialEq for Effects`

- <span id="effects-partialeq-eq"></span>`fn eq(&self, other: &Effects) -> bool` — [`Effects`](../index.md#effects)

##### `impl PartialOrd for Effects`

- <span id="effects-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Effects) -> option::Option<cmp::Ordering>` — [`Effects`](../index.md#effects)

##### `impl StructuralPartialEq for Effects`

##### `impl Sub for Effects`

- <span id="effects-sub-type-output"></span>`type Output = Effects`

- <span id="effects-sub"></span>`fn sub(self, other: Self) -> Self`

##### `impl SubAssign for Effects`

- <span id="effects-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: Self)`

### `Metadata`

```rust
struct Metadata {
    name: &'static str,
    escape: &'static str,
}
```

### `EffectsDisplay`

```rust
struct EffectsDisplay(Effects);
```

#### Trait Implementations

##### `impl Clone for EffectsDisplay`

- <span id="effectsdisplay-clone"></span>`fn clone(&self) -> EffectsDisplay` — [`EffectsDisplay`](#effectsdisplay)

##### `impl Copy for EffectsDisplay`

##### `impl Debug for EffectsDisplay`

- <span id="effectsdisplay-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for EffectsDisplay`

- <span id="effectsdisplay-default"></span>`fn default() -> EffectsDisplay` — [`EffectsDisplay`](#effectsdisplay)

##### `impl Display for EffectsDisplay`

- <span id="effectsdisplay-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl ToString for EffectsDisplay`

- <span id="effectsdisplay-tostring-to-string"></span>`fn to_string(&self) -> String`

### `EffectIter`

```rust
struct EffectIter {
    index: usize,
    effects: Effects,
}
```

Enumerate each enabled value in [`Effects`](../index.md)

#### Trait Implementations

##### `impl Clone for EffectIter`

- <span id="effectiter-clone"></span>`fn clone(&self) -> EffectIter` — [`EffectIter`](../index.md#effectiter)

##### `impl Debug for EffectIter`

- <span id="effectiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for EffectIter`

##### `impl IntoIterator for EffectIter`

- <span id="effectiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="effectiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="effectiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for EffectIter`

- <span id="effectiter-iterator-type-item"></span>`type Item = Effects`

- <span id="effectiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl PartialEq for EffectIter`

- <span id="effectiter-partialeq-eq"></span>`fn eq(&self, other: &EffectIter) -> bool` — [`EffectIter`](../index.md#effectiter)

##### `impl StructuralPartialEq for EffectIter`

### `EffectIndexIter`

```rust
struct EffectIndexIter {
    index: usize,
    effects: Effects,
}
```

#### Trait Implementations

##### `impl Clone for EffectIndexIter`

- <span id="effectindexiter-clone"></span>`fn clone(&self) -> EffectIndexIter` — [`EffectIndexIter`](#effectindexiter)

##### `impl Debug for EffectIndexIter`

- <span id="effectindexiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for EffectIndexIter`

##### `impl IntoIterator for EffectIndexIter`

- <span id="effectindexiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="effectindexiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="effectindexiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for EffectIndexIter`

- <span id="effectindexiter-iterator-type-item"></span>`type Item = usize`

- <span id="effectindexiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl PartialEq for EffectIndexIter`

- <span id="effectindexiter-partialeq-eq"></span>`fn eq(&self, other: &EffectIndexIter) -> bool` — [`EffectIndexIter`](#effectindexiter)

##### `impl StructuralPartialEq for EffectIndexIter`

## Constants

### `METADATA`
```rust
const METADATA: [Metadata; 12];
```

