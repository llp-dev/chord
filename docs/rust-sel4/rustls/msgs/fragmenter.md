**rustls > msgs > fragmenter**

# Module: msgs::fragmenter

## Contents

**Structs**

- [`MessageFragmenter`](#messagefragmenter)

---

## rustls::msgs::fragmenter::MessageFragmenter

*Struct*

**Methods:**

- `fn fragment_message<'a>(self: &Self, msg: &'a PlainMessage) -> impl Trait` - Take `msg` and fragment it into new messages with the same type and version.
- `fn set_max_fragment_size(self: & mut Self, max_fragment_size: Option<usize>) -> Result<(), Error>` - Set the maximum fragment size that will be produced.

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`



