**syscalls**

# Module: syscalls

## Contents

**Modules**

- [`arch`](#arch)
- [`args`](#args) - Provide helper functions/trait impls to pack/unpack
- [`errno`](#errno)
- [`macros`](#macros)
- [`map`](#map)
- [`raw`](#raw) - Exposes raw syscalls that simply return a `usize` instead of a `Result`.
- [`set`](#set) - Enables the creation of a syscall bitset.
- [`syscall`](#syscall)

**Macros**

- [`raw_syscall`](#raw_syscall) - Performs a raw syscall and returns a `usize`. Use [`syscall`] if you wish to
- [`syscall`](#syscall) - Performs a syscall and returns a `Result<usize, Errno>`.
- [`syscall_args`](#syscall_args)

**Functions**

- [`syscall`](#syscall) - Does a raw syscall.
- [`syscall0`](#syscall0) - Issues a system call with 0 arguments.
- [`syscall1`](#syscall1) - Issues a system call with 1 argument.
- [`syscall2`](#syscall2) - Issues a system call with 2 arguments.
- [`syscall3`](#syscall3) - Issues a system call with 3 arguments.
- [`syscall4`](#syscall4) - Issues a system call with 4 arguments.
- [`syscall5`](#syscall5) - Issues a system call with 5 arguments.
- [`syscall6`](#syscall6) - Issues a system call with 6 arguments.

---

## Module: arch



## Module: args

Provide helper functions/trait impls to pack/unpack
[`SyscallArgs`].

`io:Error` is not implemented for better `no_std` support.



## Module: errno



## Module: macros



## Module: map



## Module: raw

Exposes raw syscalls that simply return a `usize` instead of a `Result`.



## syscalls::raw_syscall

*Declarative Macro*

Performs a raw syscall and returns a `usize`. Use [`syscall`] if you wish to
get a `Result` as a return value.

Accepts a syscall number and a variable number of arguments (0 to 6).

# Example
```
use syscalls::{Sysno, raw_syscall};

// gettid is guaranteed to never fail, so we don't need a `Result` return
// value.
let tid = unsafe { raw_syscall!(Sysno::gettid) };
println!("My thread ID is {}", tid);
```

```rust
macro_rules! raw_syscall {
    ($nr:expr) => { ... };
    ($nr:expr, $a1:expr) => { ... };
    ($nr:expr, $a1:expr, $a2:expr) => { ... };
    ($nr:expr, $a1:expr, $a2:expr, $a3:expr) => { ... };
    ($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr) => { ... };
    ($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr) => { ... };
    ($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => { ... };
}
```



## Module: set

Enables the creation of a syscall bitset.



## syscalls::syscall

*Function*

Does a raw syscall.

# Arguments
 - `nr`: The syscall number.
 - `args`: packed arguments

# Returns
 - `Ok` on success,
 - `Err` when the syscall failed (with errno).

# Safety

Running a system call is inherently unsafe. It is the caller's
responsibility to ensure safety.

```rust
fn syscall(nr: Sysno, args: &SyscallArgs) -> Result<usize, Errno>
```



## Module: syscall



## syscalls::syscall

*Declarative Macro*

Performs a syscall and returns a `Result<usize, Errno>`.

Accepts a syscall number and a variable number of arguments (0 to 6).

# Returns
 - `Ok` on success, or
 - `Err(errno)` if the syscall failed.

# Example
```
use syscalls::{Sysno, syscall};

match unsafe { syscall!(Sysno::clone) } {
    Ok(0) => {
        // Child process
    }
    Ok(pid) => {
        // Parent process
    }
    Err(err) => {
        eprintln!("clone() failed: {}", err);
    }
}
```

```rust
macro_rules! syscall {
    ($nr:expr) => { ... };
    ($nr:expr, $a1:expr) => { ... };
    ($nr:expr, $a1:expr, $a2:expr) => { ... };
    ($nr:expr, $a1:expr, $a2:expr, $a3:expr) => { ... };
    ($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr) => { ... };
    ($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr) => { ... };
    ($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => { ... };
}
```



## syscalls::syscall0

*Function*

Issues a system call with 0 arguments.

# Safety

Running a system call is inherently unsafe. It is the caller's
responsibility to ensure safety.

```rust
fn syscall0(nr: Sysno) -> Result<usize, Errno>
```



## syscalls::syscall1

*Function*

Issues a system call with 1 argument.

# Safety

Running a system call is inherently unsafe. It is the caller's
responsibility to ensure safety.

```rust
fn syscall1(nr: Sysno, a1: usize) -> Result<usize, Errno>
```



## syscalls::syscall2

*Function*

Issues a system call with 2 arguments.

# Safety

Running a system call is inherently unsafe. It is the caller's
responsibility to ensure safety.

```rust
fn syscall2(nr: Sysno, a1: usize, a2: usize) -> Result<usize, Errno>
```



## syscalls::syscall3

*Function*

Issues a system call with 3 arguments.

# Safety

Running a system call is inherently unsafe. It is the caller's
responsibility to ensure safety.

```rust
fn syscall3(nr: Sysno, a1: usize, a2: usize, a3: usize) -> Result<usize, Errno>
```



## syscalls::syscall4

*Function*

Issues a system call with 4 arguments.

# Safety

Running a system call is inherently unsafe. It is the caller's
responsibility to ensure safety.

```rust
fn syscall4(nr: Sysno, a1: usize, a2: usize, a3: usize, a4: usize) -> Result<usize, Errno>
```



## syscalls::syscall5

*Function*

Issues a system call with 5 arguments.

# Safety

Running a system call is inherently unsafe. It is the caller's
responsibility to ensure safety.

```rust
fn syscall5(nr: Sysno, a1: usize, a2: usize, a3: usize, a4: usize, a5: usize) -> Result<usize, Errno>
```



## syscalls::syscall6

*Function*

Issues a system call with 6 arguments.

# Safety

Running a system call is inherently unsafe. It is the caller's
responsibility to ensure safety.

```rust
fn syscall6(nr: Sysno, a1: usize, a2: usize, a3: usize, a4: usize, a5: usize, a6: usize) -> Result<usize, Errno>
```



## syscalls::syscall_args

*Declarative Macro*

```rust
macro_rules! syscall_args {
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr) => { ... };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr) => { ... };
    ($a:expr, $b:expr, $c:expr, $d:expr) => { ... };
    ($a:expr, $b:expr, $c:expr) => { ... };
    ($a:expr, $b:expr) => { ... };
    ($a:expr) => { ... };
    () => { ... };
}
```



