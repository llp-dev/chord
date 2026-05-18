**rustls > msgs > base**

# Module: msgs::base

## Contents

**Structs**

- [`MaybeEmpty`](#maybeempty)
- [`NonEmpty`](#nonempty)
- [`PayloadU16`](#payloadu16) - An arbitrary, unknown-content, u16-length-prefixed payload

**Enums**

- [`Payload`](#payload) - An externally length'd payload

**Traits**

- [`Cardinality`](#cardinality)

---

## rustls::msgs::base::Cardinality

*Trait*

**Methods:**

- `MIN`



## rustls::msgs::base::MaybeEmpty

*Struct*

**Unit Struct**



## rustls::msgs::base::NonEmpty

*Struct*

**Unit Struct**



## rustls::msgs::base::Payload

*Enum*

An externally length'd payload

**Generic Parameters:**
- 'a

**Variants:**
- `Borrowed(&'a [u8])`
- `Owned(alloc::vec::Vec<u8>)`

**Methods:**

- `fn bytes(self: &Self) -> &[u8]`
- `fn into_owned(self: Self) -> Payload<'static>`
- `fn into_vec(self: Self) -> Vec<u8>`
- `fn read(r: & mut Reader<'a>) -> Self`
- `fn new<impl Into<Vec<u8>>>(bytes: impl Trait) -> Self`
- `fn empty() -> Self`

**Traits:** Eq

**Trait Implementations:**

- **Codec**
  - `fn encode(self: &Self, bytes: & mut Vec<u8>)`
  - `fn read(r: & mut Reader<'a>) -> Result<Self, InvalidMessage>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Payload<'a>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Payload<'a>) -> bool`



## rustls::msgs::base::PayloadU16

*Struct*

An arbitrary, unknown-content, u16-length-prefixed payload

The `C` type parameter controls whether decoded values may
be empty.

**Generic Parameters:**
- C

**Tuple Struct**: `()`

**Methods:**

- `fn new(bytes: Vec<u8>) -> Self`

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &PayloadU16<C>) -> bool`
- **Codec**
  - `fn encode(self: &Self, bytes: & mut Vec<u8>)`
  - `fn read(r: & mut Reader) -> Result<Self, InvalidMessage>`
- **Clone**
  - `fn clone(self: &Self) -> PayloadU16<C>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



