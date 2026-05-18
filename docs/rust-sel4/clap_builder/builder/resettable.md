**clap_builder > builder > resettable**

# Module: builder::resettable

## Contents

**Enums**

- [`Resettable`](#resettable) - Clearable builder value

**Traits**

- [`IntoResettable`](#intoresettable) - Convert to the intended resettable type

---

## clap_builder::builder::resettable::IntoResettable

*Trait*

Convert to the intended resettable type

**Methods:**

- `into_resettable`: Convert to the intended resettable type



## clap_builder::builder::resettable::Resettable

*Enum*

Clearable builder value

This allows a builder function to both accept any value that can [`Into::into`] `T` (like
`&str` into `OsStr`) as well as `None` to reset it to the default.  This is needed to
workaround a limitation where you can't have a function argument that is `impl Into<Option<T>>`
where `T` is `impl Into<S>` accept `None` as its type is ambiguous.

# Example

```rust
# use clap_builder as clap;
# use clap::Command;
# use clap::Arg;
fn common() -> Command {
    Command::new("cli")
        .arg(Arg::new("input").short('i').long("input"))
}
let mut command = common();
command.mut_arg("input", |arg| arg.short(None));
```

**Generic Parameters:**
- T

**Variants:**
- `Value(T)` - Overwrite builder value
- `Reset` - Reset builder value

**Traits:** Eq, Copy

**Trait Implementations:**

- **From**
  - `fn from(other: T) -> Self`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **IntoResettable**
  - `fn into_resettable(self: Self) -> Resettable<T>`
- **Ord**
  - `fn cmp(self: &Self, other: &Resettable<T>) -> $crate::cmp::Ordering`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Resettable<T>) -> bool`
- **From**
  - `fn from(other: Option<T>) -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Resettable<T>) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> Resettable<T>`



