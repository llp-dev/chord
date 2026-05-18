**clap_builder > error > format**

# Module: error::format

## Contents

**Structs**

- [`KindFormatter`](#kindformatter) - Report [`ErrorKind`]
- [`RichFormatter`](#richformatter) - Richly formatted error context

**Traits**

- [`ErrorFormatter`](#errorformatter) - Defines how to format an error for displaying to the user

---

## clap_builder::error::format::ErrorFormatter

*Trait*

Defines how to format an error for displaying to the user

**Methods:**

- `format_error`: Stylize the error for the terminal



## clap_builder::error::format::KindFormatter

*Struct*

Report [`ErrorKind`]

No context is included.

<div class="warning">

**NOTE:** Consider removing the `error-context` default feature if using this to remove all
overhead for [`RichFormatter`].

</div>

**Unit Struct**

**Trait Implementations:**

- **ErrorFormatter**
  - `fn format_error(error: &crate::error::Error<Self>) -> StyledStr`



## clap_builder::error::format::RichFormatter

*Struct*

Richly formatted error context

This follows the [rustc diagnostic style guide](https://rustc-dev-guide.rust-lang.org/diagnostics.html#suggestion-style-guide).

**Unit Struct**

**Trait Implementations:**

- **ErrorFormatter**
  - `fn format_error(error: &crate::error::Error<Self>) -> StyledStr`



