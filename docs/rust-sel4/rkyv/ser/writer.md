**rkyv > ser > writer**

# Module: ser::writer

## Contents

**Traits**

- [`Positional`](#positional) - A writer that knows its current position.
- [`Writer`](#writer) - A type that writes bytes to some output.
- [`WriterExt`](#writerext) - Helper methods for [`Writer`].

---

## rkyv::ser::writer::Positional

*Trait*

A writer that knows its current position.

**Methods:**

- `pos`: Returns the current position of the writer.



## rkyv::ser::writer::Writer

*Trait*

A type that writes bytes to some output.

A type that is [`Write`](::std::io::Write) can be wrapped in an [`IoWriter`]
to equip it with `Writer`.

It's important that the memory for archived objects is properly aligned
before attempting to read objects out of it; use an
[`AlignedVec`](crate::util::AlignedVec) or the [`Align`](crate::util::Align)
wrapper as appropriate.

**Methods:**

- `write`: Attempts to write the given bytes to the serializer.



## rkyv::ser::writer::WriterExt

*Trait*

Helper methods for [`Writer`].

**Methods:**

- `pad`: Advances the given number of bytes as padding.
- `align`: Aligns the position of the serializer to the given alignment.
- `align_for`: Aligns the position of the serializer to be suitable to write the given
- `resolve_aligned`: Resolves the given value with its resolver and writes the archived type.
- `resolve_unsized_aligned`: Resolves the given reference with its resolver and writes the archived



