**rustls > time_provider**

# Module: time_provider

## Contents

**Traits**

- [`TimeProvider`](#timeprovider) - An object that provides the current time.

---

## rustls::time_provider::TimeProvider

*Trait*

An object that provides the current time.

This is used to, for example, check if a certificate has expired during
certificate validation, or to check the age of a ticket.

**Methods:**

- `current_time`: Returns the current wall time.



