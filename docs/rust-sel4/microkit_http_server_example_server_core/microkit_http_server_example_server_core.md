**microkit_http_server_example_server_core**

# Module: microkit_http_server_example_server_core

## Contents

**Functions**

- [`run_server`](#run_server)

---

## microkit_http_server_example_server_core::run_server

*Function*

```rust
fn run_server<T, impl 'static + Send + Sync + Fn() -> Instant>(now_unix_time: core::time::Duration, now_fn: impl Trait, _timers_ctx: sel4_async_time::TimerManager, network_ctx: sel4_async_network::ManagedInterface, fs_block_io: T, spawner: sel4_async_single_threaded_executor::LocalSpawner, cert_pem: &str, priv_pem: &str, max_num_simultaneous_connections: usize) -> never
```



