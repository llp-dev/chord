*[unwinding](../../../index.md) / [unwinder](../../index.md) / [find_fde](../index.md) / [custom_eh_frame_finder](index.md)*

---

# Module `custom_eh_frame_finder`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FrameInfo`](#frameinfo) | struct |  |
| [`SetCustomEhFrameFinderError`](#setcustomehframefindererror) | struct |  |
| [`FrameInfoKind`](#frameinfokind) | enum |  |
| [`EhFrameFinder`](#ehframefinder) | trait |  |
| [`set_custom_eh_frame_finder`](#set-custom-eh-frame-finder) | fn |  |

## Structs

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

The type returned by [`set_custom_eh_frame_finder`](../custom/index.md) if [`set_custom_eh_frame_finder`](../custom/index.md) has
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

A trait for types whose values can be used as the global EH frame finder set by [`set_custom_eh_frame_finder`](../custom/index.md).

#### Required Methods

- `fn find(&self, pc: usize) -> Option<FrameInfo>`

## Functions

### `set_custom_eh_frame_finder`

```rust
fn set_custom_eh_frame_finder(fde_finder: &'static dyn EhFrameFinder + Sync) -> Result<(), SetCustomEhFrameFinderError>
```

Sets the global EH frame finder.

This function should only be called once during the lifetime of the program.

# Errors

An error is returned if this function has already been called during the lifetime of the
program.

