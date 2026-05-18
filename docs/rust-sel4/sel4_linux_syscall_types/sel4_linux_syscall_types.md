**sel4_linux_syscall_types**

# Module: sel4_linux_syscall_types

## Contents

**Structs**

- [`IOVec`](#iovec)

**Enums**

- [`ParseSyscallError`](#parsesyscallerror)
- [`ParseSyscallErrorInner`](#parsesyscallerrorinner)
- [`Syscall`](#syscall)

**Constants**

- [`ENOMEM`](#enomem)
- [`ENOSYS`](#enosys)
- [`MAP_ANONYMOUS`](#map_anonymous)
- [`SEEK_CUR`](#seek_cur)

**Type Aliases**

- [`SyscallNumber`](#syscallnumber)
- [`SyscallReturnValue`](#syscallreturnvalue)

---

## sel4_linux_syscall_types::ENOMEM

*Constant*: `SyscallReturnValue`



## sel4_linux_syscall_types::ENOSYS

*Constant*: `SyscallReturnValue`



## sel4_linux_syscall_types::IOVec

*Struct*

**Fields:**
- `iov_base: *const core::ffi::c_void`
- `iov_len: usize`

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &IOVec) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Ord**
  - `fn cmp(self: &Self, other: &IOVec) -> $crate::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &IOVec) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> IOVec`



## sel4_linux_syscall_types::MAP_ANONYMOUS

*Constant*: `i32`



## sel4_linux_syscall_types::ParseSyscallError

*Enum*

**Generic Parameters:**
- T

**Variants:**
- `UnrecognizedSyscallNumber{ sysnum: SyscallNumber, args: T }`
- `TooFewValues{ sysnum: SyscallNumber }`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ParseSyscallError<T>`
- **PartialEq**
  - `fn eq(self: &Self, other: &ParseSyscallError<T>) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Ord**
  - `fn cmp(self: &Self, other: &ParseSyscallError<T>) -> $crate::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &ParseSyscallError<T>) -> $crate::option::Option<$crate::cmp::Ordering>`



## sel4_linux_syscall_types::ParseSyscallErrorInner

*Enum*

**Variants:**
- `UnrecognizedSyscallNumber`
- `TooFewValues`

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &ParseSyscallErrorInner) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Ord**
  - `fn cmp(self: &Self, other: &ParseSyscallErrorInner) -> $crate::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &ParseSyscallErrorInner) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> ParseSyscallErrorInner`



## sel4_linux_syscall_types::SEEK_CUR

*Constant*: `i32`



## sel4_linux_syscall_types::Syscall

*Enum*

**Variants:**
- `Lseek{ fd: core::ffi::c_int, offset: usize, whence: core::ffi::c_int }`
- `Write{ fd: core::ffi::c_int, buf: *const core::ffi::c_char, count: usize }`
- `Writev{ fd: core::ffi::c_int, iov: *const IOVec, iovcnt: core::ffi::c_int }`
- `Getuid`
- `Geteuid`
- `Getgid`
- `Getegid`
- `Brk{ addr: *mut core::ffi::c_void }`
- `Mmap{ addr: *mut core::ffi::c_void, len: usize, prot: core::ffi::c_int, flag: core::ffi::c_int, fd: core::ffi::c_int, offset: usize }`

**Methods:**

- `fn parse<T>(sysnum: SyscallNumber, args: T) -> Result<Self, ParseSyscallError<T>>`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &Syscall) -> $crate::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Syscall) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Clone**
  - `fn clone(self: &Self) -> Syscall`
- **PartialEq**
  - `fn eq(self: &Self, other: &Syscall) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4_linux_syscall_types::SyscallNumber

*Type Alias*: `isize`



## sel4_linux_syscall_types::SyscallReturnValue

*Type Alias*: `isize`



