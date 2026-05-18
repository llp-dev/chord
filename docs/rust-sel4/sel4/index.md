# sel4

This crate provides straightforward, pure-Rust bindings to the [seL4
API](https://sel4.systems/Info/Docs/seL4-manual-latest.pdf).

Most items in this crate correspond to types, constants, and functions in `libsel4`. See the
[seL4 manual](https://sel4.systems/Info/Docs/seL4-manual-latest.pdf) for more information about
the seL4 concepts and `libsel4` items references in this crate's documentation.

This crate's implementation is based on the lower-level [`sel4-sys`](::sel4_sys) crate, which is
generated from the libsel4 headers and interface definition files.

### Features

#### `"state"`

Functions in the C libsel4 use the thread-local variable `__sel4_ipc_buffer` to obtain a pointer
to the current thread's IPC buffer:

```C
extern __thread seL4_IPCBuffer *__sel4_ipc_buffer;
```

[libmicrokit](https://github.com/seL4/microkit/tree/main/libmicrokit), which does not support
thread-local storage uses the following snippet to force `__sel4_ipc_buffer` to global rather
than thread-local:

```C
#define __thread
#include <sel4/sel4.h>
```

For the sake of flexibility and applicability, this crate can be configured to use no state at
all. Users can opt out of state and explicitly pass around references to the active IPC buffer
instead of relying on the implementation to obtain such a reference using thread-local or global
state. Such a paradigm is useful in certain uncommon circumstances, but most users will benefit
from the convenience of an implicit IPC buffer. The `"state"` feature, enabled by default, uses
state to allow one to make seL4 API calls without having to explicitly specify an IPC buffer.

For the sake of interoperability with C, the state looks something like: `static mut
__sel4_ipc_buffer: *mut IpcBuffer`. If the `"state-exposed"` feature is enabled, it is exposed
with `#![no_mangle]`. If the `"state-extern"` feature is enabled, it is wrapped in an `extern
"C"` block. Whether it is thread-local is determined by the following pseudocode:

```rust
cfg_if! {
    if #[cfg(all(any(target_thread_local, feature = "tls"), not(feature = "non-thread-local-state")))] {
        // thread-local
    } else if #[cfg(not(feature = "thread-local-state"))] {
        // not thread-local
    } else {
        compile_error!(r#"invalid configuration"#);
    }
}
```

The non-thread-local configuration should only be used in cases where the language runtime does
not support thread-local storage. In those cases without thread-local storage where this crate
will only ever run in a single thread, use the `"single-threaded"` feature to enable a more
efficient implementation. Note that enabling the `"single-threaded"` feature in a case where
this crate runs in more than one thread is unsafe.

At the API level, the `"state"` feature causes [`NoExplicitInvocationContext`] to be an alias
for [`ImplicitInvocationContext`], which implements [`InvocationContext`] by accessing the
thread-local pointer to an IPC buffer. When the `"state"` feature is not enabled,
[`NoExplicitInvocationContext`] is an alias for [`NoInvocationContext`], which does not
implement [`InvocationContext`]. The thread-local IPC buffer pointer is modified and accessed by
[`set_ipc_buffer`], [`with_ipc_buffer`], and [`with_ipc_buffer_mut`]. The lower-level
[`try_with_ipc_buffer_slot`] and [`try_with_ipc_buffer_slot_mut`] are provided as well.

### Building

This crate and its dependencies depend, at build time, on the libsel4 headers. The location of
these headers is provided to this crate at build time by environment variables. If
`SEL4_INCLUDE_DIRS` is set, then its value is interpreted as a colon-separated list of include
paths for the libsel4 headers. Otherwise, if `SEL4_PREFIX` is set, then
`$SEL4_PREFIX/libsel4/include` is used.

## Modules

### [`sel4`](sel4.md)

*1 constant, 1 module, 1 type alias, 2 macros*

### [`arch::imp`](arch/imp.md)

*1 constant*

### [`arch::imp::arch::imp::object`](arch/imp/arch/imp/object.md)

*2 enums, 2 type aliases*

### [`arch::imp::arch::imp::user_context`](arch/imp/arch/imp/user_context.md)

*1 struct*

### [`arch::imp::cap_arch`](arch/imp/cap_arch.md)

*8 type aliases*

### [`arch::imp::cap_type_arch`](arch/imp/cap_type_arch.md)

*2 type aliases, 8 structs*

### [`arch::imp::fault`](arch/imp/fault.md)

*1 enum, 5 structs*

### [`arch::imp::object`](arch/imp/object.md)

*2 enums, 2 type aliases*

### [`arch::imp::vm_attributes`](arch/imp/vm_attributes.md)

*1 struct*

### [`arch::imp::vspace`](arch/imp/vspace.md)

*2 enums*

### [`arch::imp::vspace::vspace_levels`](arch/imp/vspace/vspace_levels.md)

*2 constants*

### [`bootinfo`](bootinfo.md)

*1 enum, 5 structs*

### [`cap_rights`](cap_rights.md)

*2 structs*

### [`cnode_cap_data`](cnode_cap_data.md)

*1 struct*

### [`cptr`](cptr.md)

*1 type alias, 2 modules, 2 traits, 4 structs*

### [`cptr::cap`](cptr/cap.md)

*16 type aliases*

### [`cptr::cap_type`](cptr/cap_type.md)

*14 structs*

### [`debug`](debug.md)

*2 functions*

### [`error`](error.md)

*1 enum, 1 type alias*

### [`init_thread`](init_thread.md)

*1 function, 1 module, 2 structs*

### [`init_thread::slot`](init_thread/slot.md)

*11 constants*

### [`invocation_context`](invocation_context.md)

*1 struct, 1 trait, 1 type alias*

### [`invocations`](invocations.md)

*1 struct*

### [`ipc_buffer`](ipc_buffer.md)

*1 struct*

### [`message_info`](message_info.md)

*2 structs*

### [`object`](object.md)

*2 enums, 3 traits*

### [`printing`](printing.md)

*1 function, 1 struct*

### [`reply_authority`](reply_authority.md)

*1 struct, 1 trait, 1 type alias*

### [`state`](state.md)

*1 struct, 6 functions*

### [`state::token`](state/token.md)

*2 structs*

### [`syscalls`](syscalls.md)

*1 constant, 1 type alias, 2 functions, 2 structs, 2 traits*

### [`syscalls::fast_messages_sealing`](syscalls/fast_messages_sealing.md)

*1 trait*

### [`vspace`](vspace.md)

*1 module, 3 traits*

### [`vspace::vspace_levels`](vspace/vspace_levels.md)

*2 functions*

