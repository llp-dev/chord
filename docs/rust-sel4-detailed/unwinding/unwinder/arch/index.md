*[unwinding](../../index.md) / [unwinder](../index.md) / [arch](index.md)*

---

# Module `arch`

## Contents

- [Modules](#modules)
  - [`x86_64`](#x86-64)
- [Structs](#structs)
  - [`Context`](#context)
- [Functions](#functions)
  - [`save_context`](#save-context)
  - [`restore_context`](#restore-context)
- [Constants](#constants)
  - [`MAX_REG_RULES`](#max-reg-rules)
- [Macros](#macros)
  - [`maybe_cfi!`](#maybe-cfi)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`x86_64`](#x86-64) | mod |  |
| [`Context`](#context) | struct |  |
| [`save_context`](#save-context) | fn |  |
| [`restore_context`](#restore-context) | fn |  |
| [`MAX_REG_RULES`](#max-reg-rules) | const |  |
| [`maybe_cfi!`](#maybe-cfi) | macro |  |

## Modules

- [`x86_64`](x86_64/index.md)

## Structs

### `Context`

```rust
struct Context {
    pub registers: [usize; 16],
    pub ra: usize,
    pub mcxsr: usize,
    pub fcw: usize,
}
```

#### Trait Implementations

##### `impl Clone for Context`

- <span id="context-clone"></span>`fn clone(&self) -> Context` — [`Context`](#context)

##### `impl Debug for Context`

- <span id="context-debug-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Context`

- <span id="context-default"></span>`fn default() -> Context` — [`Context`](#context)

##### `impl Index for Context`

- <span id="context-index-type-output"></span>`type Output = usize`

- <span id="context-index"></span>`fn index(&self, reg: Register) -> &usize`

##### `impl IndexMut for Context`

- <span id="context-indexmut-index-mut"></span>`fn index_mut(&mut self, reg: Register) -> &mut usize`

## Functions

### `save_context`

```rust
fn save_context(f: fn(&mut Context, *mut ()), ptr: *mut ())
```

### `restore_context`

```rust
unsafe fn restore_context(ctx: &Context) -> never
```

## Constants

### `MAX_REG_RULES`
```rust
const MAX_REG_RULES: usize = 17usize;
```

## Macros

### `maybe_cfi!`

