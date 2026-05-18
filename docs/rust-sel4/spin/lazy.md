**spin > lazy**

# Module: lazy

## Contents

**Structs**

- [`Lazy`](#lazy) - A value which is initialized on the first access.

---

## spin::lazy::Lazy

*Struct*

A value which is initialized on the first access.

This type is a thread-safe `Lazy`, and can be used in statics.

# Examples

```
use std::collections::HashMap;
use spin::Lazy;

static HASHMAP: Lazy<HashMap<i32, String>> = Lazy::new(|| {
    println!("initializing");
    let mut m = HashMap::new();
    m.insert(13, "Spica".to_string());
    m.insert(74, "Hoyten".to_string());
    m
});

fn main() {
    println!("ready");
    std::thread::spawn(|| {
        println!("{:?}", HASHMAP.get(&13));
    }).join().unwrap();
    println!("{:?}", HASHMAP.get(&74));

    // Prints:
    //   ready
    //   initializing
    //   Some("Spica")
    //   Some("Hoyten")
}
```

**Generic Parameters:**
- T
- F
- R

**Fields:**
- `cell: crate::once::Once<T, R>`
- `init: core::cell::Cell<Option<F>>`

**Methods:**

- `fn force(this: &Self) -> &T` - Forces the evaluation of this lazy value and
- `fn new(f: F) -> Self` - Creates a new lazy value with the given initializing
- `fn as_mut_ptr(self: &Self) -> *mut T` - Retrieves a mutable pointer to the inner data.

**Traits:** Sync

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Default**
  - `fn default() -> Self` - Creates a new lazy value using `Default` as the initializing function.
- **Deref**
  - `fn deref(self: &Self) -> &T`



