**sel4 > reply_authority**

# Module: reply_authority

## Contents

**Structs**

- [`ImplicitReplyAuthority`](#implicitreplyauthority) - Under this configuration, no reply authority is required.

**Traits**

- [`ConveysReplyAuthority`](#conveysreplyauthority) - Trait for types from which [`ReplyAuthority`] can be derived.

**Type Aliases**

- [`ReplyAuthority`](#replyauthority) - Configuration-dependant alias for conveying reply authority to syscalls.

---

## sel4::reply_authority::ConveysReplyAuthority

*Trait*

Trait for types from which [`ReplyAuthority`] can be derived.

**Methods:**

- `C`
- `into_reply_authority`



## sel4::reply_authority::ImplicitReplyAuthority

*Struct*

Under this configuration, no reply authority is required.

**Generic Parameters:**
- C

**Methods:**

- `fn reply(self: Self, info: MessageInfo)` - Corresponds to `seL4_Reply`.
- `fn new(invocation_context: C) -> Self`
- `fn into_invocation_context(self: Self) -> C`

**Trait Implementations:**

- **Default**
  - `fn default() -> ImplicitReplyAuthority<C>`



## sel4::reply_authority::ReplyAuthority

*Type Alias*: `ImplicitReplyAuthority<C>`

Configuration-dependant alias for conveying reply authority to syscalls.



