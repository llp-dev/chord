**anstream > strip**

# Module: strip

## Contents

**Structs**

- [`StripStream`](#stripstream) - Only pass printable data to the inner `Write`

---

## anstream::strip::StripStream

*Struct*

Only pass printable data to the inner `Write`

**Generic Parameters:**
- S

**Methods:**

- `fn lock(self: Self) -> StripStream<std::io::StderrLock<'static>>` - Get exclusive access to the `StripStream`
- `fn lock(self: Self) -> StripStream<std::io::StdoutLock<'static>>` - Get exclusive access to the `StripStream`
- `fn is_terminal(self: &Self) -> bool` - Returns `true` if the descriptor/handle refers to a terminal/tty.
- `fn new(raw: S) -> Self` - Only pass printable data to the inner `Write`
- `fn into_inner(self: Self) -> S` - Get the wrapped [`std::io::Write`]
- `fn as_inner(self: &Self) -> &S` - Get the wrapped [`std::io::Write`]

**Trait Implementations:**

- **Write**
  - `fn write(self: & mut Self, buf: &[u8]) -> std::io::Result<usize>`
  - `fn write_vectored(self: & mut Self, bufs: &[std::io::IoSlice]) -> std::io::Result<usize>`
  - `fn flush(self: & mut Self) -> std::io::Result<()>`
  - `fn write_all(self: & mut Self, buf: &[u8]) -> std::io::Result<()>`
  - `fn write_fmt(self: & mut Self, args: std::fmt::Arguments) -> std::io::Result<()>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



