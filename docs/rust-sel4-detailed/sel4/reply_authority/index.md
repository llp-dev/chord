*[sel4](../index.md) / [reply_authority](index.md)*

---

# Module `reply_authority`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ImplicitReplyAuthority`](#implicitreplyauthority) | struct | Under this configuration, no reply authority is required. |
| [`ConveysReplyAuthority`](#conveysreplyauthority) | trait | Trait for types from which [`ReplyAuthority`] can be derived. |
| [`ReplyAuthority`](#replyauthority) | type | Configuration-dependant alias for conveying reply authority to syscalls. |
| [`ReplyAuthorityImpl`](#replyauthorityimpl) | type |  |

## Structs

### `ImplicitReplyAuthority<C>`

```rust
struct ImplicitReplyAuthority<C> {
    invocation_context: C,
}
```

Under this configuration, no reply authority is required.

#### Implementations

- <span id="implicitreplyauthority-into-sys-reply-authority"></span>`fn into_sys_reply_authority(self) -> sys::ReplyAuthority`

#### Trait Implementations

##### `impl<C: default::Default> Default for ImplicitReplyAuthority<C>`

- <span id="implicitreplyauthority-default"></span>`fn default() -> ImplicitReplyAuthority<C>` — [`ImplicitReplyAuthority`](#implicitreplyauthority)

## Traits

### `ConveysReplyAuthority`

```rust
trait ConveysReplyAuthority { ... }
```

Trait for types from which [`ReplyAuthority`](#replyauthority) can be derived.

#### Associated Types

- `type C`

#### Required Methods

- `fn into_reply_authority(self) -> ReplyAuthority<<Self as >::C>`

#### Implementors

- [`ReplyAuthority`](#replyauthority)
- `()`

## Type Aliases

### `ReplyAuthority<C>`

```rust
type ReplyAuthority<C> = ImplicitReplyAuthority<C>;
```

Configuration-dependant alias for conveying reply authority to syscalls.

### `ReplyAuthorityImpl<C>`

```rust
type ReplyAuthorityImpl<C> = ImplicitReplyAuthority<C>;
```

