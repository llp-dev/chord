# Crate `sel4_stack`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Stack`](#stack) | struct |  |
| [`StackBottom`](#stackbottom) | struct |  |

## Structs

### `Stack<const N: usize>`

```rust
struct Stack<const N: usize>(core::cell::UnsafeCell<[u8; N]>);
```

#### Implementations

- <span id="stack-new"></span>`const fn new() -> Self`

- <span id="stack-bottom"></span>`const fn bottom(&self) -> StackBottom` — [`StackBottom`](#stackbottom)

#### Trait Implementations

##### `impl Default for Stack<N>`

- <span id="stack-default"></span>`fn default() -> Self`

##### `impl Sync for Stack<N>`

### `StackBottom`

```rust
struct StackBottom(*mut u8);
```

#### Implementations

- <span id="stackbottom-ptr"></span>`fn ptr(&self) -> *mut u8`

#### Trait Implementations

##### `impl Sync for StackBottom`

