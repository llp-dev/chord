*[sel4_sys](../index.md) / [invocations](index.md)*

---

# Module `invocations`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`invocation_label`](#invocation-label) | mod |  |
| [`seL4_TCB_SetFlags_ret`](#sel4-tcb-setflags-ret) | struct |  |
| [`seL4_X86_Page_GetAddress_ret`](#sel4-x86-page-getaddress-ret) | struct |  |
| [`seL4_X86_IOPort_In8_ret`](#sel4-x86-ioport-in8-ret) | struct |  |
| [`seL4_X86_IOPort_In16_ret`](#sel4-x86-ioport-in16-ret) | struct |  |
| [`seL4_X86_IOPort_In32_ret`](#sel4-x86-ioport-in32-ret) | struct |  |

## Modules

- [`invocation_label`](invocation_label/index.md)

## Structs

### `seL4_TCB_SetFlags_ret`

```rust
struct seL4_TCB_SetFlags_ret {
    pub error: seL4_Error::Type,
    pub flags: seL4_Word,
}
```

#### Trait Implementations

##### `impl Default for seL4_TCB_SetFlags_ret`

- <span id="sel4-tcb-setflags-ret-default"></span>`fn default() -> seL4_TCB_SetFlags_ret` — [`seL4_TCB_SetFlags_ret`](../index.md#sel4-tcb-setflags-ret)

### `seL4_X86_Page_GetAddress_ret`

```rust
struct seL4_X86_Page_GetAddress_ret {
    pub error: seL4_Error::Type,
    pub paddr: seL4_Word,
}
```

#### Trait Implementations

##### `impl Default for seL4_X86_Page_GetAddress_ret`

- <span id="sel4-x86-page-getaddress-ret-default"></span>`fn default() -> seL4_X86_Page_GetAddress_ret` — [`seL4_X86_Page_GetAddress_ret`](../index.md#sel4-x86-page-getaddress-ret)

### `seL4_X86_IOPort_In8_ret`

```rust
struct seL4_X86_IOPort_In8_ret {
    pub error: seL4_Error::Type,
    pub result: seL4_Uint8,
}
```

#### Trait Implementations

##### `impl Default for seL4_X86_IOPort_In8_ret`

- <span id="sel4-x86-ioport-in8-ret-default"></span>`fn default() -> seL4_X86_IOPort_In8_ret` — [`seL4_X86_IOPort_In8_ret`](../index.md#sel4-x86-ioport-in8-ret)

### `seL4_X86_IOPort_In16_ret`

```rust
struct seL4_X86_IOPort_In16_ret {
    pub error: seL4_Error::Type,
    pub result: seL4_Uint16,
}
```

#### Trait Implementations

##### `impl Default for seL4_X86_IOPort_In16_ret`

- <span id="sel4-x86-ioport-in16-ret-default"></span>`fn default() -> seL4_X86_IOPort_In16_ret` — [`seL4_X86_IOPort_In16_ret`](../index.md#sel4-x86-ioport-in16-ret)

### `seL4_X86_IOPort_In32_ret`

```rust
struct seL4_X86_IOPort_In32_ret {
    pub error: seL4_Error::Type,
    pub result: seL4_Uint32,
}
```

#### Trait Implementations

##### `impl Default for seL4_X86_IOPort_In32_ret`

- <span id="sel4-x86-ioport-in32-ret-default"></span>`fn default() -> seL4_X86_IOPort_In32_ret` — [`seL4_X86_IOPort_In32_ret`](../index.md#sel4-x86-ioport-in32-ret)

