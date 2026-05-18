**smoltcp > wire > pretty_print**

# Module: wire::pretty_print

## Contents

**Structs**

- [`PrettyIndent`](#prettyindent) - Indentation state.
- [`PrettyPrinter`](#prettyprinter) - Wrapper for using a `PrettyPrint` where a `Display` is expected.

**Traits**

- [`PrettyPrint`](#prettyprint) - Interface for printing listings.

---

## smoltcp::wire::pretty_print::PrettyIndent

*Struct*

Indentation state.

**Methods:**

- `fn new(prefix: &'static str) -> PrettyIndent` - Create an indentation state. The entire listing will be indented by the width
- `fn increase(self: & mut Self, f: & mut fmt::Formatter) -> fmt::Result` - Increase indentation level.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## smoltcp::wire::pretty_print::PrettyPrint

*Trait*

Interface for printing listings.

**Methods:**

- `pretty_print`: Write a concise, formatted representation of a packet contained in the provided



## smoltcp::wire::pretty_print::PrettyPrinter

*Struct*

Wrapper for using a `PrettyPrint` where a `Display` is expected.

**Generic Parameters:**
- 'a
- T

**Methods:**

- `fn new(prefix: &'static str, buffer: &'a dyn AsRef) -> PrettyPrinter<'a, T>` - Format the listing with the recorded parameters when Display::fmt is called.
- `fn print(printable: &'a T) -> PrettyPrinter<'a, T>` - Create a `PrettyPrinter` which prints the given object.

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



