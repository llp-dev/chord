**smoltcp > wire > dns**

# Module: wire::dns

## Contents

**Structs**

- [`Flags`](#flags)
- [`Packet`](#packet) - A read/write wrapper around a DNS packet buffer.
- [`Question`](#question)
- [`Record`](#record)
- [`Repr`](#repr) - High-level DNS packet representation.

**Enums**

- [`Opcode`](#opcode) - DNS OpCodes
- [`Rcode`](#rcode) - DNS OpCodes
- [`RecordData`](#recorddata)
- [`Type`](#type) - DNS record types

---

## smoltcp::wire::dns::Flags

*Struct*

**Methods:**

- `fn empty() -> Self` - Returns an empty set of flags.
- `fn all() -> Self` - Returns the set containing all flags.
- `fn bits(self: &Self) -> u16` - Returns the raw value of the flags currently stored.
- `fn from_bits(bits: u16) -> $crate::_core::option::Option<Self>` - Convert from underlying bit representation, unless that
- `fn from_bits_truncate(bits: u16) -> Self` - Convert from underlying bit representation, dropping any bits
- `fn from_bits_unchecked(bits: u16) -> Self` - Convert from underlying bit representation, preserving all
- `fn is_empty(self: &Self) -> bool` - Returns `true` if no flags are currently stored.
- `fn is_all(self: &Self) -> bool` - Returns `true` if all flags are currently set.
- `fn intersects(self: &Self, other: Self) -> bool` - Returns `true` if there are flags common to both `self` and `other`.
- `fn contains(self: &Self, other: Self) -> bool` - Returns `true` if all of the flags in `other` are contained within `self`.
- `fn insert(self: & mut Self, other: Self)` - Inserts the specified flags in-place.
- `fn remove(self: & mut Self, other: Self)` - Removes the specified flags in-place.
- `fn toggle(self: & mut Self, other: Self)` - Toggles the specified flags in-place.
- `fn set(self: & mut Self, other: Self, value: bool)` - Inserts or removes the specified flags depending on the passed value.
- `fn intersection(self: Self, other: Self) -> Self` - Returns the intersection between the flags in `self` and
- `fn union(self: Self, other: Self) -> Self` - Returns the union of between the flags in `self` and `other`.
- `fn difference(self: Self, other: Self) -> Self` - Returns the difference between the flags in `self` and `other`.
- `fn symmetric_difference(self: Self, other: Self) -> Self` - Returns the [symmetric difference][sym-diff] between the flags
- `fn complement(self: Self) -> Self` - Returns the complement of this set of flags.

**Traits:** Copy, Eq

**Trait Implementations:**

- **BitAnd**
  - `fn bitand(self: Self, other: Self) -> Self` - Returns the intersection between the two sets of flags.
- **PartialEq**
  - `fn eq(self: &Self, other: &Flags) -> bool`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut $crate::_core::fmt::Formatter) -> $crate::_core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Flags`
- **BitOr**
  - `fn bitor(self: Self, other: Flags) -> Self` - Returns the union of the two sets of flags.
- **Extend**
  - `fn extend<T>(self: & mut Self, iterator: T)`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Octal**
  - `fn fmt(self: &Self, f: & mut $crate::_core::fmt::Formatter) -> $crate::_core::fmt::Result`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: Self)` - Disables all flags disabled in the set.
- **Sub**
  - `fn sub(self: Self, other: Self) -> Self` - Returns the set difference of the two sets of flags.
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Flags) -> $crate::option::Option<$crate::cmp::Ordering>`
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: Self)` - Adds the set of flags.
- **BitXor**
  - `fn bitxor(self: Self, other: Self) -> Self` - Returns the left flags, but with all the right flags toggled.
- **FromIterator**
  - `fn from_iter<T>(iterator: T) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::_core::fmt::Formatter) -> $crate::_core::fmt::Result`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut $crate::_core::fmt::Formatter) -> $crate::_core::fmt::Result`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: Self)` - Disables all flags enabled in the set.
- **Not**
  - `fn not(self: Self) -> Self` - Returns the complement of this set of flags.
- **Ord**
  - `fn cmp(self: &Self, other: &Flags) -> $crate::cmp::Ordering`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: Self)` - Toggles the set of flags.
- **Binary**
  - `fn fmt(self: &Self, f: & mut $crate::_core::fmt::Formatter) -> $crate::_core::fmt::Result`



## smoltcp::wire::dns::Opcode

*Enum*

DNS OpCodes

**Variants:**
- `Query`
- `Status`
- `Unknown(u8)`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &Opcode) -> $crate::cmp::Ordering`
- **Clone**
  - `fn clone(self: &Self) -> Opcode`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Opcode) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Opcode) -> $crate::option::Option<$crate::cmp::Ordering>`
- **From**
  - `fn from(value: u8) -> Self`



## smoltcp::wire::dns::Packet

*Struct*

A read/write wrapper around a DNS packet buffer.

**Generic Parameters:**
- T

**Methods:**

- `fn new_unchecked(buffer: T) -> Packet<T>` - Imbue a raw octet buffer with DNS packet structure.
- `fn new_checked(buffer: T) -> Result<Packet<T>>` - Shorthand for a combination of [new_unchecked] and [check_len].
- `fn check_len(self: &Self) -> Result<()>` - Ensure that no accessor method will panic if called.
- `fn into_inner(self: Self) -> T` - Consume the packet, returning the underlying buffer.
- `fn payload(self: &Self) -> &[u8]`
- `fn transaction_id(self: &Self) -> u16`
- `fn flags(self: &Self) -> Flags`
- `fn opcode(self: &Self) -> Opcode`
- `fn rcode(self: &Self) -> Rcode`
- `fn question_count(self: &Self) -> u16`
- `fn answer_record_count(self: &Self) -> u16`
- `fn authority_record_count(self: &Self) -> u16`
- `fn additional_record_count(self: &Self) -> u16`
- `fn parse_name<'a>(self: &'a Self, bytes: &'a [u8]) -> impl Trait` - Parse part of a name from `bytes`, following pointers if any.
- `fn payload_mut(self: & mut Self) -> & mut [u8]`
- `fn set_transaction_id(self: & mut Self, val: u16)`
- `fn set_flags(self: & mut Self, val: Flags)`
- `fn set_opcode(self: & mut Self, val: Opcode)`
- `fn set_question_count(self: & mut Self, val: u16)`
- `fn set_answer_record_count(self: & mut Self, val: u16)`
- `fn set_authority_record_count(self: & mut Self, val: u16)`
- `fn set_additional_record_count(self: & mut Self, val: u16)`

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Packet<T>) -> bool`



## smoltcp::wire::dns::Question

*Struct*

**Generic Parameters:**
- 'a

**Fields:**
- `name: &'a [u8]`
- `type_: Type`

**Methods:**

- `fn parse(buffer: &'a [u8]) -> Result<(&'a [u8], Question<'a>)>`
- `fn buffer_len(self: &Self) -> usize` - Return the length of a packet that will be emitted from this high-level representation.
- `fn emit(self: &Self, packet: & mut [u8])` - Emit a high-level representation into a DNS packet.

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Question<'a>) -> bool`



## smoltcp::wire::dns::Rcode

*Enum*

DNS OpCodes

**Variants:**
- `NoError`
- `FormErr`
- `ServFail`
- `NXDomain`
- `NotImp`
- `Refused`
- `YXDomain`
- `YXRRSet`
- `NXRRSet`
- `NotAuth`
- `NotZone`
- `Unknown(u8)`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Rcode) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Rcode) -> $crate::option::Option<$crate::cmp::Ordering>`
- **From**
  - `fn from(value: u8) -> Self`
- **Ord**
  - `fn cmp(self: &Self, other: &Rcode) -> $crate::cmp::Ordering`
- **Clone**
  - `fn clone(self: &Self) -> Rcode`



## smoltcp::wire::dns::Record

*Struct*

**Generic Parameters:**
- 'a

**Fields:**
- `name: &'a [u8]`
- `ttl: u32`
- `data: RecordData<'a>`

**Methods:**

- `fn parse(buffer: &'a [u8]) -> Result<(&'a [u8], Record<'a>)>`

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Record<'a>) -> bool`



## smoltcp::wire::dns::RecordData

*Enum*

**Generic Parameters:**
- 'a

**Variants:**
- `A(crate::wire::Ipv4Address)`
- `Cname(&'a [u8])`
- `Other(Type, &'a [u8])`

**Methods:**

- `fn parse(type_: Type, data: &'a [u8]) -> Result<RecordData<'a>>`

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &RecordData<'a>) -> bool`



## smoltcp::wire::dns::Repr

*Struct*

High-level DNS packet representation.

Currently only supports query packets.

**Generic Parameters:**
- 'a

**Fields:**
- `transaction_id: u16`
- `opcode: Opcode`
- `flags: Flags`
- `question: Question<'a>`

**Methods:**

- `fn buffer_len(self: &Self) -> usize` - Return the length of a packet that will be emitted from this high-level representation.
- `fn emit<T>(self: &Self, packet: & mut Packet<& mut T>)` - Emit a high-level representation into a DNS packet.

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Repr<'a>) -> bool`



## smoltcp::wire::dns::Type

*Enum*

DNS record types

**Variants:**
- `A`
- `Ns`
- `Cname`
- `Soa`
- `Aaaa`
- `Unknown(u16)`

**Traits:** Copy, Eq

**Trait Implementations:**

- **From**
  - `fn from(value: u16) -> Self`
- **Ord**
  - `fn cmp(self: &Self, other: &Type) -> $crate::cmp::Ordering`
- **Clone**
  - `fn clone(self: &Self) -> Type`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Type) -> bool`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Type) -> $crate::option::Option<$crate::cmp::Ordering>`



