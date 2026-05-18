**embedded_io_async**

# Module: embedded_io_async

## Contents

**Traits**

- [`BufRead`](#bufread) - Async buffered reader.
- [`Read`](#read) - Async reader.
- [`Seek`](#seek) - Async seek within streams.
- [`Write`](#write) - Async writer.

---

## embedded_io_async::BufRead

*Trait*

Async buffered reader.

This trait is the `embedded-io-async` equivalent of [`std::io::BufRead`].

**Methods:**

- `fill_buf`: Return the contents of the internal buffer, filling it with more data from the inner reader if it is empty.
- `consume`: Tell this buffer that `amt` bytes have been consumed from the buffer, so they should no longer be returned in calls to `fill_buf`.



## embedded_io_async::Read

*Trait*

Async reader.

This trait is the `embedded-io-async` equivalent of [`std::io::Read`].

**Methods:**

- `read`: Read some bytes from this source into the specified buffer, returning how many bytes were read.
- `read_exact`: Read the exact number of bytes required to fill `buf`.



## embedded_io_async::Seek

*Trait*

Async seek within streams.

This trait is the `embedded-io-async` equivalent of [`std::io::Seek`].

**Methods:**

- `seek`: Seek to an offset, in bytes, in a stream.
- `rewind`: Rewind to the beginning of a stream.
- `stream_position`: Returns the current seek position from the start of the stream.



## embedded_io_async::Write

*Trait*

Async writer.

This trait is the `embedded-io-async` equivalent of [`std::io::Write`].

**Methods:**

- `write`: Write a buffer into this writer, returning how many bytes were written.
- `flush`: Flush this output stream, ensuring that all intermediately buffered contents reach their destination.
- `write_all`: Write an entire buffer into this writer.



