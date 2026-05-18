**spin > barrier**

# Module: barrier

## Contents

**Structs**

- [`Barrier`](#barrier) - A primitive that synchronizes the execution of multiple threads.
- [`BarrierState`](#barrierstate)
- [`BarrierWaitResult`](#barrierwaitresult) - A `BarrierWaitResult` is returned by [`wait`] when all threads in the [`Barrier`]

---

## spin::barrier::Barrier

*Struct*

A primitive that synchronizes the execution of multiple threads.

# Example

```
use spin;
use std::sync::Arc;
use std::thread;

let mut handles = Vec::with_capacity(10);
let barrier = Arc::new(spin::Barrier::new(10));
for _ in 0..10 {
    let c = barrier.clone();
    // The same messages will be printed together.
    // You will NOT see any interleaving.
    handles.push(thread::spawn(move|| {
        println!("before wait");
        c.wait();
        println!("after wait");
    }));
}
// Wait for other threads to finish.
for handle in handles {
    handle.join().unwrap();
}
```

**Generic Parameters:**
- R

**Fields:**
- `lock: crate::mutex::Mutex<BarrierState, R>`
- `num_threads: usize`

**Methods:**

- `fn wait(self: &Self) -> BarrierWaitResult` - Blocks the current thread until all threads have rendezvoused here.
- `fn new(n: usize) -> Self` - Creates a new barrier that can block a given number of threads.



## spin::barrier::BarrierState

*Struct*

**Fields:**
- `count: usize`
- `generation_id: usize`



## spin::barrier::BarrierWaitResult

*Struct*

A `BarrierWaitResult` is returned by [`wait`] when all threads in the [`Barrier`]
have rendezvoused.

[`wait`]: struct.Barrier.html#method.wait
[`Barrier`]: struct.Barrier.html

# Examples

```
use spin;

let barrier = spin::Barrier::new(1);
let barrier_wait_result = barrier.wait();
```

**Tuple Struct**: `(bool)`

**Methods:**

- `fn is_leader(self: &Self) -> bool` - Returns whether this thread from [`wait`] is the "leader thread".



