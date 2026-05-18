**unwinding > unwinder > find_fde > custom**

# Module: unwinder::find_fde::custom

## Contents

**Structs**

- [`CustomFinder`](#customfinder)
- [`FrameInfo`](#frameinfo)
- [`SetCustomEhFrameFinderError`](#setcustomehframefindererror) - The type returned by [`set_custom_eh_frame_finder`] if [`set_custom_eh_frame_finder`] has

**Enums**

- [`FrameInfoKind`](#frameinfokind)

**Functions**

- [`find_fde`](#find_fde)
- [`find_fde_with_eh_frame`](#find_fde_with_eh_frame)
- [`find_fde_with_eh_frame_hdr`](#find_fde_with_eh_frame_hdr)
- [`get_custom_eh_frame_finder`](#get_custom_eh_frame_finder)
- [`get_finder`](#get_finder)
- [`set_custom_eh_frame_finder`](#set_custom_eh_frame_finder) - Sets the global EH frame finder.

**Statics**

- [`CUSTOM_EH_FRAME_FINDER`](#custom_eh_frame_finder)
- [`CUSTOM_EH_FRAME_FINDER_STATE`](#custom_eh_frame_finder_state)

**Traits**

- [`EhFrameFinder`](#ehframefinder) - A trait for types whose values can be used as the global EH frame finder set by [`set_custom_eh_frame_finder`].

**Constants**

- [`INITIALIZED`](#initialized)
- [`INITIALIZING`](#initializing)
- [`UNINITIALIZED`](#uninitialized)

---

## unwinding::unwinder::find_fde::custom::CUSTOM_EH_FRAME_FINDER

*Static*

```rust
static mut CUSTOM_EH_FRAME_FINDER: Option<&dyn EhFrameFinder>
```



## unwinding::unwinder::find_fde::custom::CUSTOM_EH_FRAME_FINDER_STATE

*Static*

```rust
static CUSTOM_EH_FRAME_FINDER_STATE: core::sync::atomic::AtomicU32
```



## unwinding::unwinder::find_fde::custom::CustomFinder

*Struct*

**Tuple Struct**: `(())`

**Trait Implementations:**

- **FDEFinder**
  - `fn find_fde(self: &Self, pc: usize) -> Option<FDESearchResult>`



## unwinding::unwinder::find_fde::custom::EhFrameFinder

*Trait*

A trait for types whose values can be used as the global EH frame finder set by [`set_custom_eh_frame_finder`].

**Methods:**

- `find`



## unwinding::unwinder::find_fde::custom::FrameInfo

*Struct*

**Fields:**
- `text_base: Option<usize>`
- `kind: FrameInfoKind`



## unwinding::unwinder::find_fde::custom::FrameInfoKind

*Enum*

**Variants:**
- `EhFrameHdr(usize)`
- `EhFrame(usize)`



## unwinding::unwinder::find_fde::custom::INITIALIZED

*Constant*: `u32`



## unwinding::unwinder::find_fde::custom::INITIALIZING

*Constant*: `u32`



## unwinding::unwinder::find_fde::custom::SetCustomEhFrameFinderError

*Struct*

The type returned by [`set_custom_eh_frame_finder`] if [`set_custom_eh_frame_finder`] has
already been called.

**Tuple Struct**: `(())`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## unwinding::unwinder::find_fde::custom::UNINITIALIZED

*Constant*: `u32`



## unwinding::unwinder::find_fde::custom::find_fde

*Function*

```rust
fn find_fde<T>(eh_frame_finder: &T, pc: usize) -> Option<super::FDESearchResult>
```



## unwinding::unwinder::find_fde::custom::find_fde_with_eh_frame

*Function*

```rust
fn find_fde_with_eh_frame(pc: usize, text_base: Option<usize>, eh_frame: usize) -> Option<super::FDESearchResult>
```



## unwinding::unwinder::find_fde::custom::find_fde_with_eh_frame_hdr

*Function*

```rust
fn find_fde_with_eh_frame_hdr(pc: usize, text_base: Option<usize>, eh_frame_hdr: usize) -> Option<super::FDESearchResult>
```



## unwinding::unwinder::find_fde::custom::get_custom_eh_frame_finder

*Function*

```rust
fn get_custom_eh_frame_finder() -> Option<&'static dyn EhFrameFinder>
```



## unwinding::unwinder::find_fde::custom::get_finder

*Function*

```rust
fn get_finder() -> &'static CustomFinder
```



## unwinding::unwinder::find_fde::custom::set_custom_eh_frame_finder

*Function*

Sets the global EH frame finder.

This function should only be called once during the lifetime of the program.

# Errors

An error is returned if this function has already been called during the lifetime of the
program.

```rust
fn set_custom_eh_frame_finder(fde_finder: &'static dyn EhFrameFinder) -> Result<(), SetCustomEhFrameFinderError>
```



