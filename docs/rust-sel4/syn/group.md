**syn > group**

# Module: group

## Contents

**Functions**

- [`parse_delimited`](#parse_delimited)
- [`parse_group`](#parse_group)

---

## syn::group::parse_delimited

*Function*

```rust
fn parse_delimited<'a>(input: &crate::parse::ParseBuffer<'a>, delimiter: proc_macro2::Delimiter) -> crate::error::Result<(proc_macro2::extra::DelimSpan, crate::parse::ParseBuffer<'a>)>
```



## syn::group::parse_group

*Function*

```rust
fn parse_group<'a>(input: &crate::parse::ParseBuffer<'a>) -> crate::error::Result<Group<'a>>
```



