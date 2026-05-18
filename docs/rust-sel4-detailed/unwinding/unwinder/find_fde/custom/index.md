*[unwinding](../../../index.md) / [unwinder](../../index.md) / [find_fde](../index.md) / [custom](index.md)*

---

# Module `custom`

## Contents

- [Structs](#structs)
  - [`CustomFinder`](#customfinder)
  - [`FrameInfo`](#frameinfo)
  - [`SetCustomEhFrameFinderError`](#setcustomehframefindererror)
- [Enums](#enums)
  - [`FrameInfoKind`](#frameinfokind)
- [Traits](#traits)
  - [`EhFrameFinder`](#ehframefinder)
- [Functions](#functions)
  - [`get_finder`](#get-finder)
  - [`set_custom_eh_frame_finder`](#set-custom-eh-frame-finder)
  - [`get_custom_eh_frame_finder`](#get-custom-eh-frame-finder)
  - [`find_fde`](#find-fde)
  - [`find_fde_with_eh_frame_hdr`](#find-fde-with-eh-frame-hdr)
  - [`find_fde_with_eh_frame`](#find-fde-with-eh-frame)
- [Constants](#constants)
  - [`UNINITIALIZED`](#uninitialized)
  - [`INITIALIZING`](#initializing)
  - [`INITIALIZED`](#initialized)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`CustomFinder`](#customfinder) | struct |  |
| [`FrameInfo`](#frameinfo) | struct |  |
| [`SetCustomEhFrameFinderError`](#setcustomehframefindererror) | struct | The type returned by [`set_custom_eh_frame_finder`] if [`set_custom_eh_frame_finder`] has already been called. |
| [`FrameInfoKind`](#frameinfokind) | enum |  |
| [`EhFrameFinder`](#ehframefinder) | trait | A trait for types whose values can be used as the global EH frame finder set by [`set_custom_eh_frame_finder`]. |
| [`get_finder`](#get-finder) | fn |  |
| [`set_custom_eh_frame_finder`](#set-custom-eh-frame-finder) | fn | Sets the global EH frame finder. |
| [`get_custom_eh_frame_finder`](#get-custom-eh-frame-finder) | fn |  |
| [`find_fde`](#find-fde) | fn |  |
| [`find_fde_with_eh_frame_hdr`](#find-fde-with-eh-frame-hdr) | fn |  |
| [`find_fde_with_eh_frame`](#find-fde-with-eh-frame) | fn |  |
| [`UNINITIALIZED`](#uninitialized) | const |  |
| [`INITIALIZING`](#initializing) | const |  |
| [`INITIALIZED`](#initialized) | const |  |

## Structs

### `CustomFinder`

```rust
struct CustomFinder(());
```

#### Trait Implementations

##### `impl FDEFinder for CustomFinder`

- <span id="customfinder-fdefinder-find-fde"></span>`fn find_fde(&self, pc: usize) -> Option<FDESearchResult>` — [`FDESearchResult`](../index.md#fdesearchresult)

### `FrameInfo`

```rust
struct FrameInfo {
    pub text_base: Option<usize>,
    pub kind: FrameInfoKind,
}
```

### `SetCustomEhFrameFinderError`

```rust
struct SetCustomEhFrameFinderError(());
```

The type returned by [`set_custom_eh_frame_finder`](#set-custom-eh-frame-finder) if [`set_custom_eh_frame_finder`](#set-custom-eh-frame-finder) has
already been called.

#### Trait Implementations

##### `impl Debug for SetCustomEhFrameFinderError`

- <span id="setcustomehframefindererror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Enums

### `FrameInfoKind`

```rust
enum FrameInfoKind {
    EhFrameHdr(usize),
    EhFrame(usize),
}
```

## Traits

### `EhFrameFinder`

```rust
trait EhFrameFinder { ... }
```

A trait for types whose values can be used as the global EH frame finder set by [`set_custom_eh_frame_finder`](#set-custom-eh-frame-finder).

#### Required Methods

- `fn find(&self, pc: usize) -> Option<FrameInfo>`

## Functions

### `get_finder`

```rust
fn get_finder() -> &'static CustomFinder
```

### `set_custom_eh_frame_finder`

```rust
fn set_custom_eh_frame_finder(fde_finder: &'static dyn EhFrameFinder + Sync) -> Result<(), SetCustomEhFrameFinderError>
```

Sets the global EH frame finder.

This function should only be called once during the lifetime of the program.

# Errors

An error is returned if this function has already been called during the lifetime of the
program.

### `get_custom_eh_frame_finder`

```rust
fn get_custom_eh_frame_finder() -> Option<&'static dyn EhFrameFinder>
```

### `find_fde`

```rust
fn find_fde<T: EhFrameFinder + ?Sized>(eh_frame_finder: &T, pc: usize) -> Option<super::FDESearchResult>
```

### `find_fde_with_eh_frame_hdr`

```rust
fn find_fde_with_eh_frame_hdr(pc: usize, text_base: Option<usize>, eh_frame_hdr: usize) -> Option<super::FDESearchResult>
```

### `find_fde_with_eh_frame`

```rust
fn find_fde_with_eh_frame(pc: usize, text_base: Option<usize>, eh_frame: usize) -> Option<super::FDESearchResult>
```

## Constants

### `UNINITIALIZED`
```rust
const UNINITIALIZED: u32 = 0u32;
```

### `INITIALIZING`
```rust
const INITIALIZING: u32 = 1u32;
```

### `INITIALIZED`
```rust
const INITIALIZED: u32 = 2u32;
```

