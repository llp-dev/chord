**unwinding > panicking**

# Module: panicking

## Contents

**Functions**

- [`begin_panic`](#begin_panic)
- [`catch_unwind`](#catch_unwind)

**Traits**

- [`Exception`](#exception)

---

## unwinding::panicking::Exception

*Trait*

**Methods:**

- `CLASS`
- `wrap`
- `unwrap`



## unwinding::panicking::begin_panic

*Function*

```rust
fn begin_panic<E>(exception: E) -> UnwindReasonCode
```



## unwinding::panicking::catch_unwind

*Function*

```rust
fn catch_unwind<E, R, F>(f: F) -> Result<R, Option<E>>
```



