**once_cell > unsync**

# Module: unsync

## Contents

**Structs**

- [`Lazy`](#lazy) - A value which is initialized on the first access.
- [`OnceCell`](#oncecell) - A cell which can be written to only once. It is not thread safe.

---

## once_cell::unsync::Lazy

*Struct*

A value which is initialized on the first access.

# Example
```
use once_cell::unsync::Lazy;

let lazy: Lazy<i32> = Lazy::new(|| {
    println!("initializing");
    92
});
println!("ready");
println!("{}", *lazy);
println!("{}", *lazy);

// Prints:
//   ready
//   initializing
//   92
//   92
```

**Generic Parameters:**
- T
- F

**Methods:**

- `fn force(this: &Lazy<T, F>) -> &T` - Forces the evaluation of this lazy value and returns a reference to
- `fn force_mut(this: & mut Lazy<T, F>) -> & mut T` - Forces the evaluation of this lazy value and returns a mutable reference to
- `fn get(this: &Lazy<T, F>) -> Option<&T>` - Gets the reference to the result of this lazy value if
- `fn get_mut(this: & mut Lazy<T, F>) -> Option<& mut T>` - Gets the mutable reference to the result of this lazy value if
- `fn new(init: F) -> Lazy<T, F>` - Creates a new lazy value with the given initializing function.
- `fn into_value(this: Lazy<T, F>) -> Result<T, F>` - Consumes this `Lazy` returning the stored value.

**Trait Implementations:**

- **Default**
  - `fn default() -> Lazy<T>` - Creates a new lazy value using `Default` as the initializing function.
- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut T`
- **Deref**
  - `fn deref(self: &Self) -> &T`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## once_cell::unsync::OnceCell

*Struct*

A cell which can be written to only once. It is not thread safe.

Unlike [`std::cell::RefCell`], a `OnceCell` provides simple `&`
references to the contents.

[`std::cell::RefCell`]: https://doc.rust-lang.org/std/cell/struct.RefCell.html

# Example
```
use once_cell::unsync::OnceCell;

let cell = OnceCell::new();
assert!(cell.get().is_none());

let value: &String = cell.get_or_init(|| {
    "Hello, World!".to_string()
});
assert_eq!(value, "Hello, World!");
assert!(cell.get().is_some());
```

**Generic Parameters:**
- T

**Methods:**

- `fn new() -> OnceCell<T>` - Creates a new empty cell.
- `fn with_value(value: T) -> OnceCell<T>` - Creates a new initialized cell.
- `fn get(self: &Self) -> Option<&T>` - Gets a reference to the underlying value.
- `fn get_mut(self: & mut Self) -> Option<& mut T>` - Gets a mutable reference to the underlying value.
- `fn set(self: &Self, value: T) -> Result<(), T>` - Sets the contents of this cell to `value`.
- `fn try_insert(self: &Self, value: T) -> Result<&T, (&T, T)>` - Like [`set`](Self::set), but also returns a reference to the final cell value.
- `fn get_or_init<F>(self: &Self, f: F) -> &T` - Gets the contents of the cell, initializing it with `f`
- `fn get_or_try_init<F, E>(self: &Self, f: F) -> Result<&T, E>` - Gets the contents of the cell, initializing it with `f` if
- `fn take(self: & mut Self) -> Option<T>` - Takes the value out of this `OnceCell`, moving it back to an uninitialized state.
- `fn into_inner(self: Self) -> Option<T>` - Consumes the `OnceCell`, returning the wrapped value.

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **From**
  - `fn from(value: T) -> Self`
- **Default**
  - `fn default() -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> OnceCell<T>`
  - `fn clone_from(self: & mut Self, source: &Self)`



