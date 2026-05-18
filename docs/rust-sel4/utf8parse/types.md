**utf8parse > types**

# Module: types

## Contents

**Enums**

- [`Action`](#action) - Action to take when receiving a byte
- [`State`](#state) - States the parser can be in.

---

## utf8parse::types::Action

*Enum*

Action to take when receiving a byte

**Variants:**
- `InvalidSequence` - Unexpected byte; sequence is invalid
- `EmitByte` - Received valid 7-bit ASCII byte which can be directly emitted.
- `SetByte1` - Set the bottom continuation byte
- `SetByte2` - Set the 2nd-from-last continuation byte
- `SetByte2Top` - Set the 2nd-from-last byte which is part of a two byte sequence
- `SetByte3` - Set the 3rd-from-last continuation byte
- `SetByte3Top` - Set the 3rd-from-last byte which is part of a three byte sequence
- `SetByte4` - Set the top byte of a four byte sequence.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Action`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## utf8parse::types::State

*Enum*

States the parser can be in.

There is a state for each initial input of the 3 and 4 byte sequences since
the following bytes are subject to different conditions than a tail byte.

**Variants:**
- `Ground` - Ground state; expect anything
- `Tail3` - 3 tail bytes
- `Tail2` - 2 tail bytes
- `Tail1` - 1 tail byte
- `U3_2_e0` - UTF8-3 starting with E0
- `U3_2_ed` - UTF8-3 starting with ED
- `Utf8_4_3_f0` - UTF8-4 starting with F0
- `Utf8_4_3_f4` - UTF8-4 starting with F4

**Methods:**

- `fn advance(self: Self, byte: u8) -> (State, Action)` - Advance the parser state.

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &State) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> State`
- **Default**
  - `fn default() -> State`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



