**gimli > read > value**

# Module: read::value

## Contents

**Enums**

- [`Value`](#value) - The value of an entry on the DWARF stack.
- [`ValueType`](#valuetype) - The type of an entry on the DWARF stack.

---

## gimli::read::value::Value

*Enum*

The value of an entry on the DWARF stack.

**Variants:**
- `Generic(u64)` - A generic value, which is address-sized and of unspecified sign.
- `I8(i8)` - A signed 8-bit integer value.
- `U8(u8)` - An unsigned 8-bit integer value.
- `I16(i16)` - A signed 16-bit integer value.
- `U16(u16)` - An unsigned 16-bit integer value.
- `I32(i32)` - A signed 32-bit integer value.
- `U32(u32)` - An unsigned 32-bit integer value.
- `I64(i64)` - A signed 64-bit integer value.
- `U64(u64)` - An unsigned 64-bit integer value.
- `F32(f32)` - A 32-bit floating point value.
- `F64(f64)` - A 64-bit floating point value.

**Methods:**

- `fn value_type(self: &Self) -> ValueType` - Return the `ValueType` corresponding to this `Value`.
- `fn parse<R>(value_type: ValueType, bytes: R) -> Result<Value>` - Read a `Value` with the given `value_type` from a `Reader`.
- `fn to_u64(self: Self, addr_mask: u64) -> Result<u64>` - Convert a `Value` to a `u64`.
- `fn from_u64(value_type: ValueType, value: u64) -> Result<Value>` - Create a `Value` with the given `value_type` from a `u64` value.
- `fn convert(self: Self, value_type: ValueType, addr_mask: u64) -> Result<Value>` - Convert a `Value` to the given `value_type`.
- `fn reinterpret(self: Self, value_type: ValueType, addr_mask: u64) -> Result<Value>` - Reinterpret the bits in a `Value` as the given `value_type`.
- `fn abs(self: Self, addr_mask: u64) -> Result<Value>` - Perform an absolute value operation.
- `fn neg(self: Self, addr_mask: u64) -> Result<Value>` - Perform a negation operation.
- `fn add(self: Self, rhs: Value, addr_mask: u64) -> Result<Value>` - Perform an addition operation.
- `fn sub(self: Self, rhs: Value, addr_mask: u64) -> Result<Value>` - Perform a subtraction operation.
- `fn mul(self: Self, rhs: Value, addr_mask: u64) -> Result<Value>` - Perform a multiplication operation.
- `fn div(self: Self, rhs: Value, addr_mask: u64) -> Result<Value>` - Perform a division operation.
- `fn rem(self: Self, rhs: Value, addr_mask: u64) -> Result<Value>` - Perform a remainder operation.
- `fn not(self: Self, addr_mask: u64) -> Result<Value>` - Perform a bitwise not operation.
- `fn and(self: Self, rhs: Value, addr_mask: u64) -> Result<Value>` - Perform a bitwise and operation.
- `fn or(self: Self, rhs: Value, addr_mask: u64) -> Result<Value>` - Perform a bitwise or operation.
- `fn xor(self: Self, rhs: Value, addr_mask: u64) -> Result<Value>` - Perform a bitwise exclusive-or operation.
- `fn shl(self: Self, rhs: Value, addr_mask: u64) -> Result<Value>` - Perform a shift left operation.
- `fn shr(self: Self, rhs: Value, addr_mask: u64) -> Result<Value>` - Perform a logical shift right operation.
- `fn shra(self: Self, rhs: Value, addr_mask: u64) -> Result<Value>` - Perform an arithmetic shift right operation.
- `fn eq(self: Self, rhs: Value, addr_mask: u64) -> Result<Value>` - Perform the `==` relational operation.
- `fn ge(self: Self, rhs: Value, addr_mask: u64) -> Result<Value>` - Perform the `>=` relational operation.
- `fn gt(self: Self, rhs: Value, addr_mask: u64) -> Result<Value>` - Perform the `>` relational operation.
- `fn le(self: Self, rhs: Value, addr_mask: u64) -> Result<Value>` - Perform the `<= relational operation.
- `fn lt(self: Self, rhs: Value, addr_mask: u64) -> Result<Value>` - Perform the `< relational operation.
- `fn ne(self: Self, rhs: Value, addr_mask: u64) -> Result<Value>` - Perform the `!= relational operation.

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Value) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Value`



## gimli::read::value::ValueType

*Enum*

The type of an entry on the DWARF stack.

**Variants:**
- `Generic` - The generic type, which is address-sized and of unspecified sign,
- `I8` - Signed 8-bit integer type.
- `U8` - Unsigned 8-bit integer type.
- `I16` - Signed 16-bit integer type.
- `U16` - Unsigned 16-bit integer type.
- `I32` - Signed 32-bit integer type.
- `U32` - Unsigned 32-bit integer type.
- `I64` - Signed 64-bit integer type.
- `U64` - Unsigned 64-bit integer type.
- `F32` - 32-bit floating point type.
- `F64` - 64-bit floating point type.

**Methods:**

- `fn bit_size(self: Self, addr_mask: u64) -> u32` - The size in bits of a value for this type.
- `fn from_encoding(encoding: constants::DwAte, byte_size: u64) -> Option<ValueType>` - Construct a `ValueType` from the attributes of a base type DIE.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &ValueType) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> ValueType`



