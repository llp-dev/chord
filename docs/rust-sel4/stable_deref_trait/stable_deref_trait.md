**stable_deref_trait**

# Module: stable_deref_trait

## Contents

**Traits**

- [`CloneStableDeref`](#clonestablederef) - An unsafe marker trait for types where clones deref to the same address. This has all the requirements of StableDeref, and additionally requires that after calling clone(), both the old and new value deref to the same address. For example, Rc and Arc implement CloneStableDeref, but Box and Vec do not.
- [`StableDeref`](#stablederef) - An unsafe marker trait for types that deref to a stable address, even when moved. For example, this is implemented by Box, Vec, Rc, Arc and String, among others. Even when a Box is moved, the underlying storage remains at a fixed location.

---

## stable_deref_trait::CloneStableDeref

*Trait*

An unsafe marker trait for types where clones deref to the same address. This has all the requirements of StableDeref, and additionally requires that after calling clone(), both the old and new value deref to the same address. For example, Rc and Arc implement CloneStableDeref, but Box and Vec do not.

Note that a single type should never implement both DerefMut and CloneStableDeref. If it did, this would let you get two mutable references to the same location, by cloning and then calling deref_mut() on both values.



## stable_deref_trait::StableDeref

*Trait*

An unsafe marker trait for types that deref to a stable address, even when moved. For example, this is implemented by Box, Vec, Rc, Arc and String, among others. Even when a Box is moved, the underlying storage remains at a fixed location.

More specifically, implementors must ensure that the result of calling deref() is valid for the lifetime of the object, not just the lifetime of the borrow, and that the deref is valid even if the object is moved. Also, it must be valid even after invoking arbitrary &self methods or doing anything transitively accessible from &Self. If Self also implements DerefMut, the same restrictions apply to deref_mut() and it must remain valid if anything transitively accessible from the result of deref_mut() is mutated/called. Additionally, multiple calls to deref, (and deref_mut if implemented) must return the same address. No requirements are placed on &mut self methods other than deref_mut() and drop(), if applicable.

Basically, it must be valid to convert the result of deref() to a pointer, and later dereference that pointer, as long as the original object is still live, even if it has been moved or &self methods have been called on it. If DerefMut is also implemented, it must be valid to get pointers from deref() and deref_mut() and dereference them while the object is live, as long as you don't simultaneously dereference both of them.

Additionally, Deref and DerefMut implementations must not panic, but users of the trait are not allowed to rely on this fact (so that this restriction can be removed later without breaking backwards compatibility, should the need arise).

Here are some examples to help illustrate the requirements for implementing this trait:

```
# use std::ops::Deref;
struct Foo(u8);
impl Deref for Foo {
    type Target = u8;
    fn deref(&self) -> &Self::Target { &self.0 }
}
```

Foo cannot implement StableDeref because the int will move when Foo is moved, invalidating the result of deref().

```
# use std::ops::Deref;
struct Foo(Box<u8>);
impl Deref for Foo {
    type Target = u8;
    fn deref(&self) -> &Self::Target { &*self.0 }
}
```

Foo can safely implement StableDeref, due to the use of Box.


```
# use std::ops::Deref;
# use std::ops::DerefMut;
# use std::rc::Rc;
#[derive(Clone)]
struct Foo(Rc<u8>);
impl Deref for Foo {
    type Target = u8;
    fn deref(&self) -> &Self::Target { &*self.0 }
}
impl DerefMut for Foo {
    fn deref_mut(&mut self) -> &mut Self::Target { Rc::make_mut(&mut self.0) }
}
```

This is a simple implementation of copy-on-write: Foo's deref_mut will copy the underlying int if it is not uniquely owned, ensuring unique access at the point where deref_mut() returns. However, Foo cannot implement StableDeref because calling deref_mut(), followed by clone().deref() will result in mutable and immutable references to the same location. Note that if the DerefMut implementation were removed, Foo could safely implement StableDeref. Likewise, if the Clone implementation were removed, it would be safe to implement StableDeref, although Foo would not be very useful in that case, (without clones, the rc will always be uniquely owned).


```
# use std::ops::Deref;
struct Foo;
impl Deref for Foo {
    type Target = str;
    fn deref(&self) -> &Self::Target { &"Hello" }
}
```
Foo can safely implement StableDeref. It doesn't own the data being derefed, but the data is gaurenteed to live long enough, due to it being 'static.

```
# use std::ops::Deref;
# use std::cell::Cell;
struct Foo(Cell<bool>);
impl Deref for Foo {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        let b = self.0.get();
        self.0.set(!b);
        if b { &"Hello" } else { &"World" }
    }
}
```
Foo cannot safely implement StableDeref, even though every possible result of deref lives long enough. In order to safely implement StableAddress, multiple calls to deref must return the same result.

```
# use std::ops::Deref;
# use std::ops::DerefMut;
struct Foo(Box<(u8, u8)>);
impl Deref for Foo {
    type Target = u8;
    fn deref(&self) -> &Self::Target { &self.0.deref().0 }
}
impl DerefMut for Foo {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0.deref_mut().1 }
}
```

Foo cannot implement StableDeref because deref and deref_mut return different addresses.




