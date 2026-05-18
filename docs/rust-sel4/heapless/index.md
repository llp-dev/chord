# heapless

`static` friendly data structures that don't require dynamic memory allocation

The core principle behind `heapless` is that its data structures are backed by a *static* memory
allocation. For example, you can think of `heapless::Vec` as an alternative version of
`std::Vec` with fixed capacity and that can't be re-allocated on the fly (e.g. via `push`).

All `heapless` data structures store their memory allocation *inline* and specify their capacity
via their type parameter `N`. This means that you can instantiate a `heapless` data structure on
the stack, in a `static` variable, or even in the heap.

```
use heapless::Vec; // fixed capacity `std::Vec`

// on the stack
let mut xs: Vec<u8, 8> = Vec::new(); // can hold up to 8 elements
xs.push(42)?;
assert_eq!(xs.pop(), Some(42));

// in a `static` variable
static mut XS: Vec<u8, 8> = Vec::new();

let xs = unsafe { &mut XS };

xs.push(42)?;
assert_eq!(xs.pop(), Some(42));

// in the heap (though kind of pointless because no reallocation)
let mut ys: Box<Vec<u8, 8>> = Box::new(Vec::new());
ys.push(42)?;
assert_eq!(ys.pop(), Some(42));
# Ok::<(), u8>(())
```

Because they have fixed capacity `heapless` data structures don't implicitly reallocate. This
means that operations like `heapless::Vec.push` are *truly* constant time rather than amortized
constant time with potentially unbounded (depends on the allocator) worst case execution time
(which is bad/unacceptable for hard real time applications).

`heapless` data structures don't use a memory allocator which means no risk of an uncatchable
Out Of Memory (OOM) condition while performing operations on them. It's certainly possible to
run out of capacity while growing `heapless` data structures, but the API lets you handle this
possibility by returning a `Result` on operations that may exhaust the capacity of the data
structure.

List of currently implemented data structures:
- [`BinaryHeap`]: A priority queue.
- [`Deque`]: A double-ended queue.
- [`HistoryBuf`]: A “history buffer”, similar to a write-only ring buffer.
- [`IndexMap`]: A hash table.
- [`IndexSet`]: A hash set.
- [`LinearMap`]: A linear map.
- [`SortedLinkedList`](sorted_linked_list::SortedLinkedList): A sorted linked list.
- [`String`]: A string.
- [`Vec`]: A vector.
- [`mpmc::MpMcQueue`](mpmc): A lock-free multiple-producer, multiple-consumer queue.
- [`spsc::Queue`](spsc): A lock-free single-producer, single-consumer queue.

# Minimum Supported Rust Version (MSRV)

This crate does *not* have a Minimum Supported Rust Version (MSRV) and may make use of language
features and API in the standard library available in the latest stable Rust version.

In other words, changes in the Rust version requirement of this crate are not considered semver
breaking change and may occur in patch version releases.

## Modules

### [`heapless`](heapless.md)

*1 macro, 1 struct, 13 modules*

### [`binary_heap`](binary_heap.md)

*1 trait, 2 enums, 2 structs, 4 type aliases*

### [`binary_heap::private`](binary_heap/private.md)

*1 trait*

### [`c_string`](c_string.md)

*1 enum, 1 struct*

### [`deque`](deque.md)

*2 type aliases, 4 structs*

### [`history_buf`](history_buf.md)

*2 structs, 2 type aliases*

### [`history_buf::storage`](history_buf/storage.md)

*1 struct, 2 traits, 2 type aliases*

### [`index_map`](index_map.md)

*1 enum, 1 type alias, 9 structs*

### [`index_set`](index_set.md)

*1 type alias, 4 structs*

### [`len_type`](len_type.md)

*2 traits*

### [`linear_map`](linear_map.md)

*4 structs, 4 type aliases*

### [`linear_map::storage`](linear_map/storage.md)

*2 traits*

### [`mpmc`](mpmc.md)

*1 struct, 2 type aliases*

### [`sorted_linked_list`](sorted_linked_list.md)

*1 trait, 2 type aliases, 6 structs*

### [`sorted_linked_list::private`](sorted_linked_list/private.md)

*1 trait*

### [`sorted_linked_list::storage`](sorted_linked_list/storage.md)

*1 struct, 2 traits, 2 type aliases*

### [`spsc`](spsc.md)

*2 type aliases, 5 structs*

### [`storage`](storage.md)

*1 trait, 2 enums*

### [`string`](string.md)

*1 enum, 1 struct, 4 type aliases*

### [`string::drain`](string/drain.md)

*1 struct*

### [`string::storage`](string/storage.md)

*2 traits*

### [`vec`](vec.md)

*2 structs, 2 type aliases*

### [`vec::drain`](vec/drain.md)

*1 struct*

### [`vec::storage`](vec/storage.md)

*1 struct, 2 traits, 2 type aliases*

