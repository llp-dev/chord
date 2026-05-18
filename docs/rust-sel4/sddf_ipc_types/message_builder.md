**sddf_ipc_types > message_builder**

# Module: message_builder

## Contents

**Structs**

- [`MessageBuilder`](#messagebuilder)

**Traits**

- [`IntoMessageRegisterValue`](#intomessageregistervalue)

---

## sddf_ipc_types::message_builder::IntoMessageRegisterValue

*Trait*

**Methods:**

- `into_message_register_value`



## sddf_ipc_types::message_builder::MessageBuilder

*Struct*

**Generic Parameters:**
- 'a

**Methods:**

- `fn new(buf: &'a  mut [MessageRegisterValue]) -> Self`
- `fn set_label(self: & mut Self, label: MessageLabel)`
- `fn set_mr<impl IntoMessageRegisterValue>(self: & mut Self, i: usize, val: impl Trait)`
- `fn set_next_mr<impl IntoMessageRegisterValue>(self: & mut Self, val: impl Trait)`
- `fn build(self: Self) -> (MessageLabel, usize)`



