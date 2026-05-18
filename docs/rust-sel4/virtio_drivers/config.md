**virtio_drivers > config**

# Module: config

## Contents

**Structs**

- [`ReadOnly`](#readonly) - A configuration space register from which the driver can only read.
- [`ReadWrite`](#readwrite) - A configuration space register which the driver may both read and write.
- [`WriteOnly`](#writeonly) - A configuration space register to which the driver can only write.

**Traits**

- [`ConfigReadable`](#configreadable) - Marker trait for configuration space registers from which the driver may read.
- [`ConfigWritable`](#configwritable) - Marker trait for configuration space registers to which the driver may write.

---

## virtio_drivers::config::ConfigReadable

*Trait*

Marker trait for configuration space registers from which the driver may read.



## virtio_drivers::config::ConfigWritable

*Trait*

Marker trait for configuration space registers to which the driver may write.



## virtio_drivers::config::ReadOnly

*Struct*

A configuration space register from which the driver can only read.

**Generic Parameters:**
- T

**Tuple Struct**: `(T)`

**Methods:**

- `fn new(value: T) -> Self` - Constructs a new instance for testing.

**Traits:** IntoBytes, TryFromBytes, ConfigReadable, FromZeros, Immutable, FromBytes

**Trait Implementations:**

- **Default**
  - `fn default() -> ReadOnly<T>`



## virtio_drivers::config::ReadWrite

*Struct*

A configuration space register which the driver may both read and write.

**Generic Parameters:**
- T

**Tuple Struct**: `(T)`

**Methods:**

- `fn new(value: T) -> Self` - Constructs a new instance for testing.

**Traits:** Immutable, ConfigWritable, FromBytes, ConfigReadable, FromZeros, IntoBytes, TryFromBytes

**Trait Implementations:**

- **Default**
  - `fn default() -> ReadWrite<T>`



## virtio_drivers::config::WriteOnly

*Struct*

A configuration space register to which the driver can only write.

**Generic Parameters:**
- T

**Tuple Struct**: `(T)`

**Methods:**

- `fn new(value: T) -> Self` - Constructs a new instance for testing.

**Traits:** FromZeros, IntoBytes, TryFromBytes, Immutable, FromBytes, ConfigWritable

**Trait Implementations:**

- **Default**
  - `fn default() -> WriteOnly<T>`



