# sel4_root_task

A runtime for root tasks.

This crate defines an entrypoint at `_start` that performs the following Rust language runtime
and [`libsel4`](sel4) initialization:
- Set up the stack
- Initialize thread-local storage (using stack memory)
- Set up exception handling
- Set the seL4 IPC buffer for `libsel4` (using [`sel4::set_ipc_buffer`])
- Run C-style constructors (from `__init_array_start`/`__init_array_end`)

After initialization, the entrypoint calls the root task main function, which must be declared
with [`#[root_task]`](root_task). For example:

```rust
#[root_task]
fn main(bootinfo: &sel4::BootInfo) -> ! {
    todo!()
}
```

Building root tasks using this crate does not require a custom linker script when using `LLD`.
In particular, this crate is tested with the `LLD` binary that ships with `rustc` (`rust-lld`).
Using a GNU linker will likely require a custom linker script.

## Modules

### [`sel4_root_task`](sel4_root_task.md)

*1 constant*

### [`heap`](heap.md)

*1 function, 1 module*

### [`termination`](termination.md)

*1 enum, 1 trait*

