**sel4_async_block_io > access**

# Module: access

## Contents

**Enums**

- [`Absurdity`](#absurdity)
- [`ReadOnly`](#readonly)
- [`ReadWrite`](#readwrite)
- [`WriteOnly`](#writeonly)

**Traits**

- [`Access`](#access)
- [`ReadAccess`](#readaccess)
- [`Witness`](#witness)
- [`WriteAccess`](#writeaccess)

---

## sel4_async_block_io::access::Absurdity

*Enum*

**Traits:** Witness, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Absurdity`



## sel4_async_block_io::access::Access

*Trait*

**Methods:**

- `ReadWitness`
- `WriteWitness`



## sel4_async_block_io::access::ReadAccess

*Trait*

**Methods:**

- `READ_WITNESS`



## sel4_async_block_io::access::ReadOnly

*Enum*

**Traits:** Access, ReadAccess



## sel4_async_block_io::access::ReadWrite

*Enum*

**Traits:** Access, WriteAccess, ReadAccess



## sel4_async_block_io::access::Witness

*Trait*

**Methods:**

- `TRY_WITNESS`



## sel4_async_block_io::access::WriteAccess

*Trait*

**Methods:**

- `WRITE_WITNESS`



## sel4_async_block_io::access::WriteOnly

*Enum*

**Traits:** Access, WriteAccess



