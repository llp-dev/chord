**sel4_linux_syscall_types > syscall_registers**

# Module: syscall_registers

## Contents

**Structs**

- [`IteratorAsSyscallArgs`](#iteratorassyscallargs)
- [`VaListAsSyscallArgs`](#valistassyscallargs)

**Traits**

- [`SyscallArg`](#syscallarg)
- [`SyscallArgs`](#syscallargs)

**Type Aliases**

- [`SyscallWordArg`](#syscallwordarg)

---

## sel4_linux_syscall_types::syscall_registers::IteratorAsSyscallArgs

*Struct*

**Generic Parameters:**
- T

**Tuple Struct**: `()`

**Methods:**

- `fn new(it: T) -> Self`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &IteratorAsSyscallArgs<T>) -> $crate::cmp::Ordering`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **SyscallArgs**
  - `fn next_word_arg(self: & mut Self) -> Option<SyscallWordArg>`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &IteratorAsSyscallArgs<T>) -> $crate::option::Option<$crate::cmp::Ordering>`
- **PartialEq**
  - `fn eq(self: &Self, other: &IteratorAsSyscallArgs<T>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> IteratorAsSyscallArgs<T>`



## sel4_linux_syscall_types::syscall_registers::SyscallArg

*Trait*

**Methods:**

- `from_word`



## sel4_linux_syscall_types::syscall_registers::SyscallArgs

*Trait*

**Methods:**

- `next_word_arg`
- `next_arg`



## sel4_linux_syscall_types::syscall_registers::SyscallWordArg

*Type Alias*: `usize`



## sel4_linux_syscall_types::syscall_registers::VaListAsSyscallArgs

*Struct*

**Generic Parameters:**
- 'a

**Tuple Struct**: `()`

**Methods:**

- `fn new(va_list: VaList<'a>) -> Self`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **SyscallArgs**
  - `fn next_word_arg(self: & mut Self) -> Option<SyscallWordArg>`



