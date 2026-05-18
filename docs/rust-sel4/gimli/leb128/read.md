**gimli > leb128 > read**

# Module: leb128::read

## Contents

**Functions**

- [`signed`](#signed) - Read a signed LEB128 number from the given `Reader` and
- [`skip`](#skip) - Read bytes until the LEB128 continuation bit is not set.
- [`u16`](#u16) - Read an LEB128 u16 from the given `Reader` and
- [`unsigned`](#unsigned) - Read an unsigned LEB128 number from the given `Reader` and

---

## gimli::leb128::read::signed

*Function*

Read a signed LEB128 number from the given `Reader` and
return it or an error if reading failed.

```rust
fn signed<R>(r: & mut R) -> crate::read::Result<i64>
```



## gimli::leb128::read::skip

*Function*

Read bytes until the LEB128 continuation bit is not set.

```rust
fn skip<R>(r: & mut R) -> crate::read::Result<()>
```



## gimli::leb128::read::u16

*Function*

Read an LEB128 u16 from the given `Reader` and
return it or an error if reading failed.

```rust
fn u16<R>(r: & mut R) -> crate::read::Result<u16>
```



## gimli::leb128::read::unsigned

*Function*

Read an unsigned LEB128 number from the given `Reader` and
return it or an error if reading failed.

```rust
fn unsigned<R>(r: & mut R) -> crate::read::Result<u64>
```



