**sel4_async_network_rustls_utils > time_provider_impl**

# Module: time_provider_impl

## Contents

**Structs**

- [`TimeProviderImpl`](#timeproviderimpl)

---

## sel4_async_network_rustls_utils::time_provider_impl::TimeProviderImpl

*Struct*

**Generic Parameters:**
- F

**Methods:**

- `fn new(now_global: UnixTime, now_fn: F) -> Self`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **TimeProvider**
  - `fn current_time(self: &Self) -> Option<UnixTime>`



