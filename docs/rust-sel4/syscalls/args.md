**syscalls > args**

# Module: args

## Contents

**Structs**

- [`SyscallArgs`](#syscallargs) - The 6 arguments of a syscall, raw untyped version.

---

## syscalls::args::SyscallArgs

*Struct*

The 6 arguments of a syscall, raw untyped version.

**Fields:**
- `arg0: usize`
- `arg1: usize`
- `arg2: usize`
- `arg3: usize`
- `arg4: usize`
- `arg5: usize`

**Methods:**

- `fn new(a0: usize, a1: usize, a2: usize, a3: usize, a4: usize, a5: usize) -> Self`

**Traits:** Copy, Eq

**Trait Implementations:**

- **From**
  - `fn from(args: &[usize; 1]) -> Self`
- **From**
  - `fn from(args: &[usize; 3]) -> Self`
- **From**
  - `fn from(args: &[usize; 5]) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> SyscallArgs`
- **From**
  - `fn from(_args: &[usize; 0]) -> Self`
- **From**
  - `fn from(args: &[usize; 2]) -> Self`
- **From**
  - `fn from(args: &[usize; 4]) -> Self`
- **From**
  - `fn from(args: &[usize; 6]) -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &SyscallArgs) -> bool`



