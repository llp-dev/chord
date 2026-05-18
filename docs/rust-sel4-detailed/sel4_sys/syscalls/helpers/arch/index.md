*[sel4_sys](../../../index.md) / [syscalls](../../index.md) / [helpers](../index.md) / [arch](index.md)*

---

# Module `arch`

## Contents

- [Modules](#modules)
  - [`imp`](#imp)
- [Functions](#functions)
  - [`sys_id_to_word`](#sys-id-to-word)
  - [`sys_send`](#sys-send)
  - [`sys_reply`](#sys-reply)
  - [`sys_send_null`](#sys-send-null)
  - [`sys_recv`](#sys-recv)
  - [`sys_send_recv`](#sys-send-recv)
  - [`sys_null`](#sys-null)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`imp`](#imp) | mod |  |
| [`sys_id_to_word`](#sys-id-to-word) | fn |  |
| [`sys_send`](#sys-send) | fn |  |
| [`sys_reply`](#sys-reply) | fn |  |
| [`sys_send_null`](#sys-send-null) | fn |  |
| [`sys_recv`](#sys-recv) | fn |  |
| [`sys_send_recv`](#sys-send-recv) | fn |  |
| [`sys_null`](#sys-null) | fn |  |

## Modules

- [`imp`](imp/index.md)

## Functions

### `sys_id_to_word`

```rust
fn sys_id_to_word(sys_id: core::ffi::c_int) -> crate::seL4_Word
```

### `sys_send`

```rust
fn sys_send(sys: core::ffi::c_int, dest: crate::seL4_Word, info_arg: crate::seL4_MessageInfo, mr0: crate::seL4_Word, mr1: crate::seL4_Word, mr2: crate::seL4_Word, mr3: crate::seL4_Word)
```

### `sys_reply`

```rust
fn sys_reply(sys: core::ffi::c_int, info_arg: crate::seL4_MessageInfo, mr0: crate::seL4_Word, mr1: crate::seL4_Word, mr2: crate::seL4_Word, mr3: crate::seL4_Word)
```

### `sys_send_null`

```rust
fn sys_send_null(sys: core::ffi::c_int, src: crate::seL4_Word, info_arg: crate::seL4_MessageInfo)
```

### `sys_recv`

```rust
fn sys_recv(sys: core::ffi::c_int, src: crate::seL4_Word, out_mr0: &mut crate::seL4_Word, out_mr1: &mut crate::seL4_Word, out_mr2: &mut crate::seL4_Word, out_mr3: &mut crate::seL4_Word, reply: crate::seL4_Word) -> (crate::seL4_MessageInfo, crate::seL4_Word)
```

### `sys_send_recv`

```rust
fn sys_send_recv(sys: core::ffi::c_int, dest: crate::seL4_Word, info_arg: crate::seL4_MessageInfo, in_out_mr0: &mut crate::seL4_Word, in_out_mr1: &mut crate::seL4_Word, in_out_mr2: &mut crate::seL4_Word, in_out_mr3: &mut crate::seL4_Word, reply: crate::seL4_Word) -> (crate::seL4_MessageInfo, crate::seL4_Word)
```

### `sys_null`

```rust
fn sys_null(sys: core::ffi::c_int)
```

