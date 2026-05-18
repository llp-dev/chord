**rkyv > ser > writer > std**

# Module: ser::writer::std

## Contents

**Structs**

- [`IoWriter`](#iowriter) - Wraps a type that implements [`io::Write`](std::io::Write) and equips it

---

## rkyv::ser::writer::std::IoWriter

*Struct*

Wraps a type that implements [`io::Write`](std::io::Write) and equips it
with [`Writer`].

# Examples
```
# use rkyv::ser::{Writer, Positional, writer::IoWriter};
use rkyv::rancor::{Error, Strategy};
let mut io_writer = IoWriter::new(Vec::new());
// In most cases, calling a method like `serialize` will wrap the writer in
// a Strategy for us.
let mut writer = Strategy::<_, Error>::wrap(&mut io_writer);
assert_eq!(writer.pos(), 0);
writer.write(&[0u8, 1u8, 2u8, 3u8]);
assert_eq!(writer.pos(), 4);
let buf = io_writer.into_inner();
assert_eq!(buf.len(), 4);
assert_eq!(buf, vec![0u8, 1u8, 2u8, 3u8]);
```

**Generic Parameters:**
- W

**Methods:**

- `fn new(inner: W) -> Self` - Creates a new serializer from a writer.
- `fn with_pos(inner: W, pos: usize) -> Self` - Creates a new serializer from a writer, and assumes that the underlying
- `fn into_inner(self: Self) -> W` - Consumes the serializer and returns the internal writer used to create

**Trait Implementations:**

- **Writer**
  - `fn write(self: & mut Self, bytes: &[u8]) -> Result<(), E>`
- **Positional**
  - `fn pos(self: &Self) -> usize`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



