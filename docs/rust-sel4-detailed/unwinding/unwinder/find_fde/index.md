*[unwinding](../../index.md) / [unwinder](../index.md) / [find_fde](index.md)*

---

# Module `find_fde`

## Contents

- [Modules](#modules)
  - [`custom`](#custom)
  - [`custom_eh_frame_finder`](#custom-eh-frame-finder)
- [Structs](#structs)
  - [`FDESearchResult`](#fdesearchresult)
  - [`GlobalFinder`](#globalfinder)
- [Traits](#traits)
  - [`FDEFinder`](#fdefinder)
- [Functions](#functions)
  - [`get_finder`](#get-finder)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`custom`](#custom) | mod |  |
| [`custom_eh_frame_finder`](#custom-eh-frame-finder) | mod |  |
| [`FDESearchResult`](#fdesearchresult) | struct |  |
| [`GlobalFinder`](#globalfinder) | struct |  |
| [`FDEFinder`](#fdefinder) | trait |  |
| [`get_finder`](#get-finder) | fn |  |

## Modules

- [`custom`](custom/index.md)
- [`custom_eh_frame_finder`](custom_eh_frame_finder/index.md)

## Structs

### `FDESearchResult`

```rust
struct FDESearchResult {
    pub fde: gimli::FrameDescriptionEntry<gimli::EndianSlice<'static, gimli::NativeEndian>>,
    pub bases: gimli::BaseAddresses,
    pub eh_frame: gimli::EhFrame<gimli::EndianSlice<'static, gimli::NativeEndian>>,
}
```

#### Trait Implementations

##### `impl Debug for FDESearchResult`

- <span id="fdesearchresult-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `GlobalFinder`

```rust
struct GlobalFinder(());
```

#### Trait Implementations

##### `impl FDEFinder for GlobalFinder`

- <span id="globalfinder-fdefinder-find-fde"></span>`fn find_fde(&self, pc: usize) -> Option<FDESearchResult>` — [`FDESearchResult`](#fdesearchresult)

## Traits

### `FDEFinder`

```rust
trait FDEFinder { ... }
```

#### Required Methods

- `fn find_fde(&self, pc: usize) -> Option<FDESearchResult>`

#### Implementors

- [`CustomFinder`](custom/index.md#customfinder)
- [`GlobalFinder`](#globalfinder)

## Functions

### `get_finder`

```rust
fn get_finder() -> &'static GlobalFinder
```

