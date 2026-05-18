**spin > relax**

# Module: relax

## Contents

**Structs**

- [`Loop`](#loop) - A strategy that rapidly spins, without telling the CPU to do any powering down.
- [`Spin`](#spin) - A strategy that rapidly spins while informing the CPU that it should power down non-essential components via

**Traits**

- [`RelaxStrategy`](#relaxstrategy) - A trait implemented by spinning relax strategies.

---

## spin::relax::Loop

*Struct*

A strategy that rapidly spins, without telling the CPU to do any powering down.

You almost certainly do not want to use this. Use [`Spin`] instead. It exists for completeness and for targets
that, for some reason, miscompile or do not support spin hint intrinsics despite attempting to generate code for
them (i.e: this is a workaround for possible compiler bugs).

**Unit Struct**

**Trait Implementations:**

- **RelaxStrategy**
  - `fn relax()`



## spin::relax::RelaxStrategy

*Trait*

A trait implemented by spinning relax strategies.

**Methods:**

- `relax`: Perform the relaxing operation during a period of contention.



## spin::relax::Spin

*Struct*

A strategy that rapidly spins while informing the CPU that it should power down non-essential components via
[`core::hint::spin_loop`].

Note that spinning is a 'dumb' strategy and most schedulers cannot correctly differentiate it from useful work,
thereby misallocating even more CPU time to the spinning process. This is known as
['priority inversion'](https://matklad.github.io/2020/01/02/spinlocks-considered-harmful.html).

If you see signs that priority inversion is occurring, consider switching to [`Yield`] or, even better, not using a
spinlock at all and opting for a proper scheduler-aware lock. Remember also that different targets, operating
systems, schedulers, and even the same scheduler with different workloads will exhibit different behaviour. Just
because priority inversion isn't occurring in your tests does not mean that it will not occur. Use a scheduler-
aware lock if at all possible.

**Unit Struct**

**Trait Implementations:**

- **RelaxStrategy**
  - `fn relax()`



