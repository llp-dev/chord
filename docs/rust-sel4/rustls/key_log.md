**rustls > key_log**

# Module: key_log

## Contents

**Structs**

- [`NoKeyLog`](#nokeylog) - KeyLog that does exactly nothing.

**Traits**

- [`KeyLog`](#keylog) - This trait represents the ability to do something useful

---

## rustls::key_log::KeyLog

*Trait*

This trait represents the ability to do something useful
with key material, such as logging it to a file for debugging.

Naturally, secrets passed over the interface are *extremely*
sensitive and can break the security of past, present and
future sessions.

You'll likely want some interior mutability in your
implementation to make this useful.

See [`KeyLogFile`] that implements the standard
`SSLKEYLOGFILE` environment variable behaviour.

**Methods:**

- `log`: Log the given `secret`.  `client_random` is provided for
- `will_log`: Indicates whether the secret with label `label` will be logged.



## rustls::key_log::NoKeyLog

*Struct*

KeyLog that does exactly nothing.

**Unit Struct**

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **KeyLog**
  - `fn log(self: &Self, _: &str, _: &[u8], _: &[u8])`
  - `fn will_log(self: &Self, _label: &str) -> bool`



