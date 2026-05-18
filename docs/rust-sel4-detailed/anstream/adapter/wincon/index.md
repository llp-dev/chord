*[anstream](../../index.md) / [adapter](../index.md) / [wincon](index.md)*

---

# Module `wincon`

## Contents

- [Structs](#structs)
  - [`WinconBytes`](#winconbytes)
  - [`WinconBytesIter`](#winconbytesiter)
  - [`WinconCapture`](#winconcapture)
- [Enums](#enums)
  - [`CsiState`](#csistate)
  - [`ColorTarget`](#colortarget)
- [Functions](#functions)
  - [`next_bytes`](#next-bytes)
  - [`to_ansi_color`](#to-ansi-color)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`WinconBytes`](#winconbytes) | struct | Incrementally convert to wincon calls for non-contiguous data |
| [`WinconBytesIter`](#winconbytesiter) | struct | See [`WinconBytes`] |
| [`WinconCapture`](#winconcapture) | struct |  |
| [`CsiState`](#csistate) | enum |  |
| [`ColorTarget`](#colortarget) | enum |  |
| [`next_bytes`](#next-bytes) | fn |  |
| [`to_ansi_color`](#to-ansi-color) | fn |  |

## Structs

### `WinconBytes`

```rust
struct WinconBytes {
    parser: anstyle_parse::Parser,
    capture: WinconCapture,
}
```

Incrementally convert to wincon calls for non-contiguous data

#### Implementations

- <span id="winconbytes-new"></span>`fn new() -> Self`

  Initial state

- <span id="winconbytes-extract-next"></span>`fn extract_next<'s>(self: &'s mut Self, bytes: &'s [u8]) -> WinconBytesIter<'s>` — [`WinconBytesIter`](#winconbytesiter)

  Strip the next segment of data

#### Trait Implementations

##### `impl Clone for WinconBytes`

- <span id="winconbytes-clone"></span>`fn clone(&self) -> WinconBytes` — [`WinconBytes`](#winconbytes)

##### `impl Debug for WinconBytes`

- <span id="winconbytes-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for WinconBytes`

- <span id="winconbytes-default"></span>`fn default() -> WinconBytes` — [`WinconBytes`](#winconbytes)

##### `impl Eq for WinconBytes`

##### `impl PartialEq for WinconBytes`

- <span id="winconbytes-partialeq-eq"></span>`fn eq(&self, other: &WinconBytes) -> bool` — [`WinconBytes`](#winconbytes)

##### `impl StructuralPartialEq for WinconBytes`

### `WinconBytesIter<'s>`

```rust
struct WinconBytesIter<'s> {
    bytes: &'s [u8],
    parser: &'s mut anstyle_parse::Parser,
    capture: &'s mut WinconCapture,
}
```

See [`WinconBytes`](#winconbytes)

#### Trait Implementations

##### `impl Debug for WinconBytesIter<'s>`

- <span id="winconbytesiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for WinconBytesIter<'s>`

##### `impl IntoIterator for WinconBytesIter<'s>`

- <span id="winconbytesiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="winconbytesiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="winconbytesiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for WinconBytesIter<'_>`

- <span id="winconbytesiter-iterator-type-item"></span>`type Item = (Style, String)`

- <span id="winconbytesiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl PartialEq for WinconBytesIter<'s>`

- <span id="winconbytesiter-partialeq-eq"></span>`fn eq(&self, other: &WinconBytesIter<'s>) -> bool` — [`WinconBytesIter`](#winconbytesiter)

##### `impl StructuralPartialEq for WinconBytesIter<'s>`

### `WinconCapture`

```rust
struct WinconCapture {
    style: anstyle::Style,
    printable: String,
    ready: Option<anstyle::Style>,
}
```

#### Implementations

- <span id="winconcapture-reset"></span>`fn reset(&mut self)`

#### Trait Implementations

##### `impl Clone for WinconCapture`

- <span id="winconcapture-clone"></span>`fn clone(&self) -> WinconCapture` — [`WinconCapture`](#winconcapture)

##### `impl Debug for WinconCapture`

- <span id="winconcapture-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for WinconCapture`

- <span id="winconcapture-default"></span>`fn default() -> WinconCapture` — [`WinconCapture`](#winconcapture)

##### `impl Eq for WinconCapture`

##### `impl PartialEq for WinconCapture`

- <span id="winconcapture-partialeq-eq"></span>`fn eq(&self, other: &WinconCapture) -> bool` — [`WinconCapture`](#winconcapture)

##### `impl Perform for WinconCapture`

- <span id="winconcapture-perform-print"></span>`fn print(&mut self, c: char)`

  Draw a character to the screen and update states.

- <span id="winconcapture-perform-execute"></span>`fn execute(&mut self, byte: u8)`

  Execute a C0 or C1 control function.

- <span id="winconcapture-perform-csi-dispatch"></span>`fn csi_dispatch(&mut self, params: &anstyle_parse::Params, _intermediates: &[u8], ignore: bool, action: u8)`

##### `impl StructuralPartialEq for WinconCapture`

## Enums

### `CsiState`

```rust
enum CsiState {
    Normal,
    PrepareCustomColor,
    Ansi256,
    Rgb,
    Underline,
}
```

#### Trait Implementations

##### `impl Clone for CsiState`

- <span id="csistate-clone"></span>`fn clone(&self) -> CsiState` — [`CsiState`](#csistate)

##### `impl Copy for CsiState`

##### `impl Debug for CsiState`

- <span id="csistate-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CsiState`

##### `impl PartialEq for CsiState`

- <span id="csistate-partialeq-eq"></span>`fn eq(&self, other: &CsiState) -> bool` — [`CsiState`](#csistate)

##### `impl StructuralPartialEq for CsiState`

### `ColorTarget`

```rust
enum ColorTarget {
    Fg,
    Bg,
    Underline,
}
```

#### Trait Implementations

##### `impl Clone for ColorTarget`

- <span id="colortarget-clone"></span>`fn clone(&self) -> ColorTarget` — [`ColorTarget`](#colortarget)

##### `impl Copy for ColorTarget`

##### `impl Debug for ColorTarget`

- <span id="colortarget-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ColorTarget`

##### `impl PartialEq for ColorTarget`

- <span id="colortarget-partialeq-eq"></span>`fn eq(&self, other: &ColorTarget) -> bool` — [`ColorTarget`](#colortarget)

##### `impl StructuralPartialEq for ColorTarget`

## Functions

### `next_bytes`

```rust
fn next_bytes(bytes: &mut &[u8], parser: &mut anstyle_parse::Parser, capture: &mut WinconCapture) -> Option<(anstyle::Style, String)>
```

### `to_ansi_color`

```rust
fn to_ansi_color(digit: u16) -> Option<anstyle::AnsiColor>
```

