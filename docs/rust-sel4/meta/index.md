# meta

### General crates

- [`sel4`]: Straightforward, pure-Rust bindings to the seL4 API.
- [`sel4_sys`]: Raw bindings to the seL4 API, generated from the libsel4 headers and interface
  definition files. This crate is not intended to be used directly by application code, but
  rather serves as a basis for the `sel4` crate's implementation.
- [`sel4_config`]: Macros and constants corresponding to the seL4 kernel configuration. Can be
  used by all targets (i.e. in all of: application code, build scripts, and build-time tools).
- [`sel4_platform_info`]: Constants corresponding to the contents of `platform_info.h`. Can be
  used by all targets, on configurations where this file exists.
- [`sel4_sync`]: Synchronization constructs using seL4 IPC. Currently only supports
  notification-based mutexes.
- [`sel4_logging`]: [`Log`](log::Log) implementation for the [`log`] crate.
- [`sel4_shared_memory`]: Abstractions for interacting with data in shared memory.
- [`sel4_shared_ring_buffer`] and `sel4_shared_ring_buffer_*`: Implementation of shared data structures used in the [seL4 Device
  Driver Framework](https://github.com/au-ts/sddf).
- `sel4_async_*`: Crates for leveraging async Rust in seL4 userspace.
- `sel4_*_driver`: Crates implementing drivers for use with `sel4_shared_ring_buffer_*` and the orthogonal [`sel4_driver_interfaces`].
- ...and many more, including lower-level crates for implementing additional runtimes.

### Runtime crates

- **Root task**:
  - [`sel4_root_task`]: A runtime for root tasks that supports thread-local storage and
    unwinding, and provides a global allocator.
- **seL4 Microkit**:
  - [`sel4_microkit`]: A runtime for [seL4 Microkit](https://github.com/seL4/microkit)
    protection domains, including an implementation of libmicrokit and abstractions for IPC.

## Modules

### [`meta`](meta.md)

*1 module*

