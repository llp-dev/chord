**allocator_api2 > stable**

# Module: stable

## Contents

**Modules**

- [`alloc`](#alloc) - Memory allocation APIs
- [`boxed`](#boxed) - The `Box<T>` type for heap allocation.
- [`collections`](#collections)
- [`vec`](#vec) - A contiguous growable array type with heap-allocated contents, written

---

## Module: alloc

Memory allocation APIs



## Module: boxed

The `Box<T>` type for heap allocation.

[`Box<T>`], casually referred to as a 'box', provides the simplest form of
heap allocation in Rust. Boxes provide ownership for this allocation, and
drop their contents when they go out of scope. Boxes also ensure that they
never allocate more than `isize::MAX` bytes.

# Examples

Move a value from the stack to the heap by creating a [`Box`]:

```
let val: u8 = 5;
let boxed: Box<u8> = Box::new(val);
```

Move a value from a [`Box`] back to the stack by [dereferencing]:

```
let boxed: Box<u8> = Box::new(5);
let val: u8 = *boxed;
```

Creating a recursive data structure:

```
#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

let list: List<i32> = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
println!("{list:?}");
```

This will print `Cons(1, Cons(2, Nil))`.

Recursive structures must be boxed, because if the definition of `Cons`
looked like this:

```compile_fail,E0072
# enum List<T> {
Cons(T, List<T>),
# }
```

It wouldn't work. This is because the size of a `List` depends on how many
elements are in the list, and so we don't know how much memory to allocate
for a `Cons`. By introducing a [`Box<T>`], which has a defined size, we know how
big `Cons` needs to be.

# Memory layout

For non-zero-sized values, a [`Box`] will use the [`Global`] allocator for
its allocation. It is valid to convert both ways between a [`Box`] and a
raw pointer allocated with the [`Global`] allocator, given that the
[`Layout`] used with the allocator is correct for the type. More precisely,
a `value: *mut T` that has been allocated with the [`Global`] allocator
with `Layout::for_value(&*value)` may be converted into a box using
[`Box::<T>::from_raw(value)`]. Conversely, the memory backing a `value: *mut
T` obtained from [`Box::<T>::into_raw`] may be deallocated using the
[`Global`] allocator with [`Layout::for_value(&*value)`].

For zero-sized values, the `Box` pointer still has to be [valid] for reads
and writes and sufficiently aligned. In particular, casting any aligned
non-zero integer literal to a raw pointer produces a valid pointer, but a
pointer pointing into previously allocated memory that since got freed is
not valid. The recommended way to build a Box to a ZST if `Box::new` cannot
be used is to use [`ptr::NonNull::dangling`].

So long as `T: Sized`, a `Box<T>` is guaranteed to be represented
as a single pointer and is also ABI-compatible with C pointers
(i.e. the C type `T*`). This means that if you have extern "C"
Rust functions that will be called from C, you can define those
Rust functions using `Box<T>` types, and use `T*` as corresponding
type on the C side. As an example, consider this C header which
declares functions that create and destroy some kind of `Foo`
value:

```c
/* C header */

/* Returns ownership to the caller */
struct Foo* foo_new(void);

/* Takes ownership from the caller; no-op when invoked with null */
void foo_delete(struct Foo*);
```

These two functions might be implemented in Rust as follows. Here, the
`struct Foo*` type from C is translated to `Box<Foo>`, which captures
the ownership constraints. Note also that the nullable argument to
`foo_delete` is represented in Rust as `Option<Box<Foo>>`, since `Box<Foo>`
cannot be null.

```
#[repr(C)]
pub struct Foo;

#[no_mangle]
pub extern "C" fn foo_new() -> Box<Foo> {
    Box::new(Foo)
}

#[no_mangle]
pub extern "C" fn foo_delete(_: Option<Box<Foo>>) {}
```

Even though `Box<T>` has the same representation and C ABI as a C pointer,
this does not mean that you can convert an arbitrary `T*` into a `Box<T>`
and expect things to work. `Box<T>` values will always be fully aligned,
non-null pointers. Moreover, the destructor for `Box<T>` will attempt to
free the value with the global allocator. In general, the best practice
is to only use `Box<T>` for pointers that originated from the global
allocator.

**Important.** At least at present, you should avoid using
`Box<T>` types for functions that are defined in C but invoked
from Rust. In those cases, you should directly mirror the C types
as closely as possible. Using types like `Box<T>` where the C
definition is just using `T*` can lead to undefined behavior, as
described in [rust-lang/unsafe-code-guidelines#198][ucg#198].

# Considerations for unsafe code

**Warning: This section is not normative and is subject to change, possibly
being relaxed in the future! It is a simplified summary of the rules
currently implemented in the compiler.**

The aliasing rules for `Box<T>` are the same as for `&mut T`. `Box<T>`
asserts uniqueness over its content. Using raw pointers derived from a box
after that box has been mutated through, moved or borrowed as `&mut T`
is not allowed. For more guidance on working with box from unsafe code, see
[rust-lang/unsafe-code-guidelines#326][ucg#326].


[ucg#198]: https://github.com/rust-lang/unsafe-code-guidelines/issues/198
[ucg#326]: https://github.com/rust-lang/unsafe-code-guidelines/issues/326
[dereferencing]: core::ops::Deref
[`Box::<T>::from_raw(value)`]: Box::from_raw
[`Global`]: crate::alloc::Global
[`Layout`]: crate::alloc::Layout
[`Layout::for_value(&*value)`]: crate::alloc::Layout::for_value
[valid]: ptr#safety



## Module: collections



## Module: vec

A contiguous growable array type with heap-allocated contents, written
`Vec<T>`.

Vectors have *O*(1) indexing, amortized *O*(1) push (to the end) and
*O*(1) pop (from the end).

Vectors ensure they never allocate more than `isize::MAX` bytes.

# Examples

You can explicitly create a [`Vec`] with [`Vec::new`]:

```
let v: Vec<i32> = Vec::new();
```

...or by using the [`vec!`] macro:

```
let v: Vec<i32> = vec![];

let v = vec![1, 2, 3, 4, 5];

let v = vec![0; 10]; // ten zeroes
```

You can [`push`] values onto the end of a vector (which will grow the vector
as needed):

```
let mut v = vec![1, 2];

v.push(3);
```

Popping values works in much the same way:

```
let mut v = vec![1, 2];

let two = v.pop();
```

Vectors also support indexing (through the [`Index`] and [`IndexMut`] traits):

```
let mut v = vec![1, 2, 3];
let three = v[2];
v[1] = v[1] + 5;
```

[`push`]: Vec::push



