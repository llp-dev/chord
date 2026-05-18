**anstyle_parse**

# Module: anstyle_parse

## Contents

**Modules**

- [`state`](#state) - ANSI escape code parsing state machine

**Structs**

- [`AsciiParser`](#asciiparser) - Only allow parsing 7-bit ASCII
- [`Parser`](#parser) - Parser for raw _VTE_ protocol which delegates actions to a [`Perform`]
- [`Utf8Parser`](#utf8parser) - Allow parsing UTF-8

**Traits**

- [`CharAccumulator`](#characcumulator) - Build a `char` out of bytes
- [`Perform`](#perform) - Performs actions requested by the [`Parser`]

**Type Aliases**

- [`DefaultCharAccumulator`](#defaultcharaccumulator) - Most flexible [`CharAccumulator`] for [`Parser`] based on active features

---

## anstyle_parse::AsciiParser

*Struct*

Only allow parsing 7-bit ASCII

**Unit Struct**

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &AsciiParser) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **CharAccumulator**
  - `fn add(self: & mut Self, _byte: u8) -> Option<char>`
- **Clone**
  - `fn clone(self: &Self) -> AsciiParser`
- **Default**
  - `fn default() -> AsciiParser`



## anstyle_parse::CharAccumulator

*Trait*

Build a `char` out of bytes

**Methods:**

- `add`: Build a `char` out of bytes



## anstyle_parse::DefaultCharAccumulator

*Type Alias*: `Utf8Parser`

Most flexible [`CharAccumulator`] for [`Parser`] based on active features



## anstyle_parse::Parser

*Struct*

Parser for raw _VTE_ protocol which delegates actions to a [`Perform`]

**Generic Parameters:**
- C

**Methods:**

- `fn new() -> Parser` - Create a new Parser
- `fn advance<P>(self: & mut Self, performer: & mut P, byte: u8)` - Advance the parser state

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Parser<C>`
- **Default**
  - `fn default() -> Parser<C>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Parser<C>) -> bool`



## anstyle_parse::Perform

*Trait*

Performs actions requested by the [`Parser`]

Actions in this case mean, for example, handling a CSI escape sequence describing cursor
movement, or simply printing characters to the screen.

The methods on this type correspond to actions described in
<http://vt100.net/emu/dec_ansi_parser>. I've done my best to describe them in
a useful way in my own words for completeness, but the site should be
referenced if something isn't clear. If the site disappears at some point in
the future, consider checking archive.org.

**Methods:**

- `print`: Draw a character to the screen and update states.
- `execute`: Execute a C0 or C1 control function.
- `hook`: Invoked when a final character arrives in first part of device control string.
- `put`: Pass bytes as part of a device control string to the handle chosen in `hook`. C0 controls
- `unhook`: Called when a device control string is terminated.
- `osc_dispatch`: Dispatch an operating system command.
- `csi_dispatch`: A final character has arrived for a CSI sequence
- `esc_dispatch`: The final character of an escape sequence has arrived.



## anstyle_parse::Utf8Parser

*Struct*

Allow parsing UTF-8

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Utf8Parser) -> bool`
- **CharAccumulator**
  - `fn add(self: & mut Self, byte: u8) -> Option<char>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Utf8Parser`
- **Default**
  - `fn default() -> Utf8Parser`



## Module: state

ANSI escape code parsing state machine



