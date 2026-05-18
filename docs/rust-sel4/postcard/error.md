**postcard > error**

# Module: error

## Contents

**Enums**

- [`Error`](#error) - This is the error type used by Postcard

**Type Aliases**

- [`Result`](#result) - This is the Result type used by Postcard.

---

## postcard::error::Error

*Enum*

This is the error type used by Postcard

**Variants:**
- `WontImplement` - This is a feature that postcard will never implement
- `NotYetImplemented` - This is a feature that postcard intends to support, but does not yet
- `SerializeBufferFull` - The serialize buffer is full
- `SerializeSeqLengthUnknown` - The length of a sequence must be known
- `DeserializeUnexpectedEnd` - Hit the end of buffer, expected more data
- `DeserializeBadVarint` - Found a varint that didn't terminate. Is the usize too big for this platform?
- `DeserializeBadBool` - Found a bool that wasn't 0 or 1
- `DeserializeBadChar` - Found an invalid unicode char
- `DeserializeBadUtf8` - Tried to parse invalid utf-8
- `DeserializeBadOption` - Found an Option discriminant that wasn't 0 or 1
- `DeserializeBadEnum` - Found an enum discriminant that was > `u32::MAX`
- `DeserializeBadEncoding` - The original data was not well encoded
- `DeserializeBadCrc` - Bad CRC while deserializing
- `SerdeSerCustom` - Serde Serialization Error
- `SerdeDeCustom` - Serde Deserialization Error
- `CollectStrError` - Error while processing `collect_str` during serialization

**Traits:** Eq, Error

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Error`
- **Error**
  - `fn custom<T>(_msg: T) -> Self`
- **Serialize**
  - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Error) -> bool`
- **Error**
  - `fn custom<T>(_msg: T) -> Self`
- **Display**
  - `fn fmt(self: &Self, f: & mut Formatter) -> core::fmt::Result`
- **Deserialize**
  - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`



## postcard::error::Result

*Type Alias*: `::core::result::Result<T, Error>`

This is the Result type used by Postcard.



