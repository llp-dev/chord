**anstream > stream**

# Module: stream

## Contents

**Traits**

- [`AsLockedWrite`](#aslockedwrite) - Lock a stream
- [`IsTerminal`](#isterminal) - Trait to determine if a descriptor/handle refers to a terminal/tty.
- [`RawStream`](#rawstream) - Required functionality for underlying [`std::io::Write`] for adaptation

---

## anstream::stream::AsLockedWrite

*Trait*

Lock a stream

**Methods:**

- `Write`: Locked writer type
- `as_locked_write`: Lock a stream



## anstream::stream::IsTerminal

*Trait*

Trait to determine if a descriptor/handle refers to a terminal/tty.

**Methods:**

- `is_terminal`: Returns `true` if the descriptor/handle refers to a terminal/tty.



## anstream::stream::RawStream

*Trait*

Required functionality for underlying [`std::io::Write`] for adaptation



