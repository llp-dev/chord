**unwinding > personality**

# Module: personality

## Contents

**Enums**

- [`EHAction`](#ehaction)

**Functions**

- [`find_eh_action`](#find_eh_action)
- [`parse_encoded_pointer`](#parse_encoded_pointer)
- [`parse_pointer_encoding`](#parse_pointer_encoding)
- [`rust_eh_personality`](#rust_eh_personality)

---

## unwinding::personality::EHAction

*Enum*

**Variants:**
- `None`
- `Cleanup(usize)`
- `Catch(usize)`
- `Filter(usize)`
- `Terminate`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## unwinding::personality::find_eh_action

*Function*

```rust
fn find_eh_action(reader: & mut gimli::EndianSlice<'static, gimli::NativeEndian>, unwind_ctx: &UnwindContext) -> gimli::Result<EHAction>
```



## unwinding::personality::parse_encoded_pointer

*Function*

```rust
fn parse_encoded_pointer(encoding: constants::DwEhPe, unwind_ctx: &UnwindContext, input: & mut gimli::EndianSlice<'static, gimli::NativeEndian>) -> gimli::Result<gimli::Pointer>
```



## unwinding::personality::parse_pointer_encoding

*Function*

```rust
fn parse_pointer_encoding(input: & mut gimli::EndianSlice<'static, gimli::NativeEndian>) -> gimli::Result<constants::DwEhPe>
```



## unwinding::personality::rust_eh_personality

*Function*

```rust
fn rust_eh_personality(version: i32, actions: UnwindAction, _exception_class: u64, exception: *mut UnwindException, unwind_ctx: & mut UnwindContext) -> UnwindReasonCode
```



