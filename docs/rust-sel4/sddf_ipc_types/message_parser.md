**sddf_ipc_types > message_parser**

# Module: message_parser

## Contents

**Structs**

- [`MessageParser`](#messageparser)

**Enums**

- [`MessagParseError`](#messagparseerror)

**Traits**

- [`TryFromMessageRegisterValue`](#tryfrommessageregistervalue)

---

## sddf_ipc_types::message_parser::MessagParseError

*Enum*

**Variants:**
- `UnexpectedLabel{ label: crate::MessageLabel }`
- `MessageTooShort{ index: usize, length: usize }`
- `InvalidMessageRegisterValue{ index: usize, value: crate::MessageRegisterValue }`

**Traits:** Error

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> MessagParseError`



## sddf_ipc_types::message_parser::MessageParser

*Struct*

**Generic Parameters:**
- 'a

**Methods:**

- `fn new(label: MessageLabel, buf: &'a [MessageRegisterValue]) -> Self`
- `fn label(self: &Self) -> MessageLabel`
- `fn label_try_into<T>(self: &Self) -> Result<T, MessagParseError>`
- `fn ensure_label_eq(self: &Self, expected: MessageLabel) -> Result<(), MessagParseError>`
- `fn get_mr<T>(self: &Self, i: usize) -> Result<T, MessagParseError>`



## sddf_ipc_types::message_parser::TryFromMessageRegisterValue

*Trait*

**Methods:**

- `try_from_message_register_value`



