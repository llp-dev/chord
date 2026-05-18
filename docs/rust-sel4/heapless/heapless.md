**heapless**

# Module: heapless

## Contents

**Modules**

- [`binary_heap`](#binary_heap) - A priority queue implemented with a binary heap.
- [`c_string`](#c_string) - A fixed capacity [`CString`](https://doc.rust-lang.org/std/ffi/struct.CString.html).
- [`deque`](#deque) - A fixed capacity double-ended queue.
- [`history_buf`](#history_buf) - A "history buffer", similar to a write-only ring buffer of fixed length.
- [`index_map`](#index_map) - A fixed-capacity hash table where the iteration order is independent of the hash of the keys.
- [`index_set`](#index_set) - A fixed-capacity hash set where the iteration order is independent of the hash values.
- [`linear_map`](#linear_map) - A fixed capacity map/dictionary that performs lookups via linear search.
- [`mpmc`](#mpmc) - A fixed capacity multiple-producer, multiple-consumer (MPMC) lock-free queue.
- [`sorted_linked_list`](#sorted_linked_list) - A fixed sorted priority linked list, similar to [`BinaryHeap`] but with different properties
- [`spsc`](#spsc) - A fixed capacity single-producer, single-consumer (SPSC) lock-free queue.
- [`storage`](#storage) - `Storage` trait defining how data is stored in a container.
- [`string`](#string) - A fixed capacity [`String`](https://doc.rust-lang.org/std/string/struct.String.html).
- [`vec`](#vec) - A fixed capacity [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html).

**Macros**

- [`format`](#format) - Macro that creates a fixed capacity [`String`]. Equivalent to [`format!`](https://doc.rust-lang.org/std/macro.format.html).

**Structs**

- [`CapacityError`](#capacityerror) - The error type for fallible [`Vec`] and [`String`] methods.

---

## heapless::CapacityError

*Struct*

The error type for fallible [`Vec`] and [`String`] methods.

**Unit Struct**

**Traits:** Error

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



## Module: binary_heap

A priority queue implemented with a binary heap.

Insertion and popping the largest element have *O*(log n) time complexity.
Checking the smallest/largest element is *O*(1).



## Module: c_string

A fixed capacity [`CString`](https://doc.rust-lang.org/std/ffi/struct.CString.html).



## Module: deque

A fixed capacity double-ended queue.

# Examples

```
use heapless::Deque;

// A deque with a fixed capacity of 8 elements allocated on the stack
let mut deque = Deque::<_, 8>::new();

// You can use it as a good old FIFO queue.
deque.push_back(1);
deque.push_back(2);
assert_eq!(deque.len(), 2);

assert_eq!(deque.pop_front(), Some(1));
assert_eq!(deque.pop_front(), Some(2));
assert_eq!(deque.len(), 0);

// Deque is double-ended, you can push and pop from the front and back.
deque.push_back(1);
deque.push_front(2);
deque.push_back(3);
deque.push_front(4);
assert_eq!(deque.pop_front(), Some(4));
assert_eq!(deque.pop_front(), Some(2));
assert_eq!(deque.pop_front(), Some(1));
assert_eq!(deque.pop_front(), Some(3));

// You can iterate it, yielding all the elements front-to-back.
for x in &deque {
    println!("{}", x);
}
```



## heapless::format

*Declarative Macro*

Macro that creates a fixed capacity [`String`]. Equivalent to [`format!`](https://doc.rust-lang.org/std/macro.format.html).

The macro's arguments work in the same way as the regular macro.

It is possible to explicitly specify the capacity of the returned string as the first argument.
In this case it is necessary to disambiguate by separating the capacity with a semicolon.

# Errors

There are two possible error cases. Both return the unit type [`core::fmt::Error`].

- In case the formatting exceeds the string's capacity. This error does not exist in
  the standard library as the string would just grow.
- If a formatting trait implementation returns an error. The standard library panics
  in this case.

# Examples

```
# fn main() -> Result<(), core::fmt::Error> {
use heapless::{format, String};

// Notice semicolon instead of comma!
format!(4; "test")?;
format!(15; "hello {}", "world!")?;
format!(20; "x = {}, y = {y}", 10, y = 30)?;
let (x, y) = (1, 2);
format!(12; "{x} + {y} = 3")?;

let implicit: String<10> = format!("speed = {}", 7)?;
# Ok(())
# }
```

```rust
macro_rules! format {
    ($max:expr; $lenT:path; $($arg:tt)*) => { ... };
    ($max:expr; $($arg:tt)*) => { ... };
    ($($arg:tt)*) => { ... };
}
```



## Module: history_buf

A "history buffer", similar to a write-only ring buffer of fixed length.

This buffer keeps a fixed number of elements.  On write, the oldest element
is overwritten. Thus, the buffer is useful to keep a history of values with
some desired depth, and for example calculate a rolling average.

# Examples
```
use heapless::HistoryBuf;

// Initialize a new buffer with 8 elements.
let mut buf = HistoryBuf::<_, 8>::new();

// Starts with no data
assert_eq!(buf.recent(), None);

buf.write(3);
buf.write(5);
buf.extend(&[4, 4]);

// The most recent written element is a four.
assert_eq!(buf.recent(), Some(&4));

// To access all elements in an unspecified order, use `as_slice()`.
for el in buf.as_slice() {
    println!("{:?}", el);
}

// Now we can prepare an average of all values, which comes out to 4.
let avg = buf.as_slice().iter().sum::<usize>() / buf.len();
assert_eq!(avg, 4);
```



## Module: index_map

A fixed-capacity hash table where the iteration order is independent of the hash of the keys.



## Module: index_set

A fixed-capacity hash set where the iteration order is independent of the hash values.



## Module: linear_map

A fixed capacity map/dictionary that performs lookups via linear search.

Note that as this map doesn't use hashing so most operations are *O*(n) instead of *O*(1).



## Module: mpmc

A fixed capacity multiple-producer, multiple-consumer (MPMC) lock-free queue.

**Note:** This module requires atomic compare-and-swap (CAS) instructions. On
targets where they're not natively available, they are emulated by the
[`portable-atomic`](https://crates.io/crates/portable-atomic) crate.

# Example

This queue can be constructed in `const` context. Placing it in a `static` variable lets *all*
contexts (interrupts/threads/`main`) safely enqueue and dequeue items.

```
use core::sync::atomic::{AtomicU8, Ordering};

use heapless::mpmc::Queue;

static Q: Queue<u8, 2> = Queue::new();

fn main() {
    // Configure systick interrupt.

    loop {
        if let Some(x) = Q.dequeue() {
            println!("{}", x);
        } else {
            // Wait for interrupt.
        }
#       break
    }
}

fn systick() {
    static COUNT: AtomicU8 = AtomicU8::new(0);
    let count = COUNT.fetch_add(1, Ordering::SeqCst);

#   let _ =
    Q.enqueue(count);
}
```

# Benchmark

Measured on an ARM Cortex-M3 core running at 8 MHz and with zero flash wait cycles, compiled with `-C opt-level=z`:

| Method                      | Time | N  |
|:----------------------------|-----:|---:|
| `Queue::<u8, 8>::enqueue()` |   34 |  0 |
| `Queue::<u8, 8>::enqueue()` |   52 |  1 |
| `Queue::<u8, 8>::enqueue()` |   69 |  2 |
| `Queue::<u8, 8>::dequeue()` |   35 |  0 |
| `Queue::<u8, 8>::dequeue()` |   53 |  1 |
| `Queue::<u8, 8>::dequeue()` |   71 |  2 |

- N denotes the number of interruptions. On Cortex-M, an interruption consists of an
  interrupt handler preempting the would-be atomic section of the `enqueue`/`dequeue`
  operation. Note that it does *not* matter if the higher priority handler uses the queue or
  not.
- All execution times are in clock cycles (1 clock cycle = 125 ns).
- Execution time is *dependent* on `mem::size_of::<T>()`, as both operations include
  `ptr::read::<T>()` or `ptr::write::<T>()` in their successful path.
- The numbers reported correspond to the successful path, i.e. `dequeue` returning `Some`
  and `enqueue` returning `Ok`.

# References

This is an implementation of Dmitry Vyukov's [bounded MPMC queue], minus the
cache padding.

[bounded MPMC queue]: http://www.1024cores.net/home/lock-free-algorithms/queues/bounded-mpmc-queue



## Module: sorted_linked_list

A fixed sorted priority linked list, similar to [`BinaryHeap`] but with different properties
on `push`, `pop`, etc.

For example, the sorting of the list will never `memcpy` the underlying value, so having large
objects in the list will not cause a performance hit.

# Examples

```
use heapless::sorted_linked_list::{Max, SortedLinkedList};
let mut ll: SortedLinkedList<_, Max, 3, u8> = SortedLinkedList::new_u8();

// The largest value will always be first
ll.push(1).unwrap();
assert_eq!(ll.peek(), Some(&1));

ll.push(2).unwrap();
assert_eq!(ll.peek(), Some(&2));

ll.push(3).unwrap();
assert_eq!(ll.peek(), Some(&3));

// This will not fit in the queue.
assert_eq!(ll.push(4), Err(4));
```

[`BinaryHeap`]: `crate::binary_heap::BinaryHeap`



## Module: spsc

A fixed capacity single-producer, single-consumer (SPSC) lock-free queue.

*Note:* This module requires atomic load and store instructions. On
targets where they're not natively available, they are emulated by the
[`portable-atomic`](https://crates.io/crates/portable-atomic) crate.

# Examples

[`Queue`] can be used as a plain queue:

```
use heapless::spsc::Queue;

let mut queue: Queue<u8, 4> = Queue::new();

assert_eq!(queue.enqueue(0), Ok(()));
assert_eq!(queue.enqueue(1), Ok(()));
assert_eq!(queue.enqueue(2), Ok(()));
assert_eq!(queue.enqueue(3), Err(3)); // Queue is full.

assert_eq!(queue.dequeue(), Some(0));
```

[`Queue::split`] can be used to split the queue into a [`Producer`]/[`Consumer`] pair.

After splitting a `&'static mut Queue`, the resulting [`Producer`] and [`Consumer`]
can be moved into different execution contexts, e.g. threads, interrupt handlers, etc.


```
use heapless::spsc::{Producer, Queue};

#[derive(Debug)]
enum Event {
    A,
    B,
}

fn main() {
    let queue: &'static mut Queue<Event, 4> = {
        static mut Q: Queue<Event, 4> = Queue::new();
        // SAFETY: `Q` is only accessible in this scope
        // and `main` is only called once.
        unsafe { &mut Q }
    };

    let (producer, mut consumer) = queue.split();

    // `producer` can be moved into `interrupt_handler` using a static mutex or the mechanism
    // provided by the concurrency framework you are using, e.g. a resource in RTIC.
#   let mut producer = producer;
#   interrupt_handler(&mut producer);

    loop {
        match consumer.dequeue() {
            Some(Event::A) => { /* .. */ }
            Some(Event::B) => { /* .. */ }
            None => { /* Sleep. */ }
        }
#       break
    }
}

// This is a different execution context that can preempt `main`.
fn interrupt_handler(producer: &mut Producer<'static, Event>) {
#   let condition = true;

    // ..

    if condition {
        producer.enqueue(Event::A).unwrap();
    } else {
        producer.enqueue(Event::B).unwrap();
    }

    // ..
}
```

# Benchmarks

Measured on an ARM Cortex-M3 core running at 8 MHz and with zero flash wait cycles, compiled with `-C opt-level=3`:

| Method                         | Time |
|:-------------------------------|-----:|
| `Producer::<u8, _>::enqueue()` |   16 |
| `Queue::<u8, _>::enqueue()`    |   14 |
| `Consumer::<u8, _>::dequeue()` |   15 |
| `Queue::<u8, _>::dequeue()`    |   12 |

- All execution times are in clock cycles (1 clock cycle = 125 ns).
- Execution time is *dependent* on `mem::size_of::<T>()`, as both operations include
  `ptr::read::<T>()` or `ptr::write::<T>()` in their successful path.
- The numbers reported correspond to the successful path, i.e. `dequeue` returning `Some`
  and `enqueue` returning `Ok`.

# References

This is an implementation based on [https://www.codeproject.com/Articles/43510/Lock-Free-Single-Producer-Single-Consumer-Circular](
  https://web.archive.org/web/20250117082625/https://www.codeproject.com/Articles/43510/Lock-Free-Single-Producer-Single-Consumer-Circular
).



## Module: storage

`Storage` trait defining how data is stored in a container.



## Module: string

A fixed capacity [`String`](https://doc.rust-lang.org/std/string/struct.String.html).



## Module: vec

A fixed capacity [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html).



