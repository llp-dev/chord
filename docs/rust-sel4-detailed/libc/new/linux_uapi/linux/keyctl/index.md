*[libc](../../../../index.md) / [new](../../../index.md) / [linux_uapi](../../index.md) / [linux](../index.md) / [keyctl](index.md)*

---

# Module `keyctl`

Header: `linux/keyctl.h`

## Contents

- [Constants](#constants)
  - [`KEY_SPEC_THREAD_KEYRING`](#key-spec-thread-keyring)
  - [`KEY_SPEC_PROCESS_KEYRING`](#key-spec-process-keyring)
  - [`KEY_SPEC_SESSION_KEYRING`](#key-spec-session-keyring)
  - [`KEY_SPEC_USER_KEYRING`](#key-spec-user-keyring)
  - [`KEY_SPEC_USER_SESSION_KEYRING`](#key-spec-user-session-keyring)
  - [`KEY_SPEC_GROUP_KEYRING`](#key-spec-group-keyring)
  - [`KEY_SPEC_REQKEY_AUTH_KEY`](#key-spec-reqkey-auth-key)
  - [`KEY_SPEC_REQUESTOR_KEYRING`](#key-spec-requestor-keyring)
  - [`KEY_REQKEY_DEFL_NO_CHANGE`](#key-reqkey-defl-no-change)
  - [`KEY_REQKEY_DEFL_DEFAULT`](#key-reqkey-defl-default)
  - [`KEY_REQKEY_DEFL_THREAD_KEYRING`](#key-reqkey-defl-thread-keyring)
  - [`KEY_REQKEY_DEFL_PROCESS_KEYRING`](#key-reqkey-defl-process-keyring)
  - [`KEY_REQKEY_DEFL_SESSION_KEYRING`](#key-reqkey-defl-session-keyring)
  - [`KEY_REQKEY_DEFL_USER_KEYRING`](#key-reqkey-defl-user-keyring)
  - [`KEY_REQKEY_DEFL_USER_SESSION_KEYRING`](#key-reqkey-defl-user-session-keyring)
  - [`KEY_REQKEY_DEFL_GROUP_KEYRING`](#key-reqkey-defl-group-keyring)
  - [`KEY_REQKEY_DEFL_REQUESTOR_KEYRING`](#key-reqkey-defl-requestor-keyring)
  - [`KEYCTL_GET_KEYRING_ID`](#keyctl-get-keyring-id)
  - [`KEYCTL_JOIN_SESSION_KEYRING`](#keyctl-join-session-keyring)
  - [`KEYCTL_UPDATE`](#keyctl-update)
  - [`KEYCTL_REVOKE`](#keyctl-revoke)
  - [`KEYCTL_CHOWN`](#keyctl-chown)
  - [`KEYCTL_SETPERM`](#keyctl-setperm)
  - [`KEYCTL_DESCRIBE`](#keyctl-describe)
  - [`KEYCTL_CLEAR`](#keyctl-clear)
  - [`KEYCTL_LINK`](#keyctl-link)
  - [`KEYCTL_UNLINK`](#keyctl-unlink)
  - [`KEYCTL_SEARCH`](#keyctl-search)
  - [`KEYCTL_READ`](#keyctl-read)
  - [`KEYCTL_INSTANTIATE`](#keyctl-instantiate)
  - [`KEYCTL_NEGATE`](#keyctl-negate)
  - [`KEYCTL_SET_REQKEY_KEYRING`](#keyctl-set-reqkey-keyring)
  - [`KEYCTL_SET_TIMEOUT`](#keyctl-set-timeout)
  - [`KEYCTL_ASSUME_AUTHORITY`](#keyctl-assume-authority)
  - [`KEYCTL_GET_SECURITY`](#keyctl-get-security)
  - [`KEYCTL_SESSION_TO_PARENT`](#keyctl-session-to-parent)
  - [`KEYCTL_REJECT`](#keyctl-reject)
  - [`KEYCTL_INSTANTIATE_IOV`](#keyctl-instantiate-iov)
  - [`KEYCTL_INVALIDATE`](#keyctl-invalidate)
  - [`KEYCTL_GET_PERSISTENT`](#keyctl-get-persistent)
  - [`KEYCTL_DH_COMPUTE`](#keyctl-dh-compute)
  - [`KEYCTL_PKEY_QUERY`](#keyctl-pkey-query)
  - [`KEYCTL_PKEY_ENCRYPT`](#keyctl-pkey-encrypt)
  - [`KEYCTL_PKEY_DECRYPT`](#keyctl-pkey-decrypt)
  - [`KEYCTL_PKEY_SIGN`](#keyctl-pkey-sign)
  - [`KEYCTL_PKEY_VERIFY`](#keyctl-pkey-verify)
  - [`KEYCTL_RESTRICT_KEYRING`](#keyctl-restrict-keyring)
  - [`KEYCTL_MOVE`](#keyctl-move)
  - [`KEYCTL_CAPABILITIES`](#keyctl-capabilities)
  - [`KEYCTL_SUPPORTS_ENCRYPT`](#keyctl-supports-encrypt)
  - [`KEYCTL_SUPPORTS_DECRYPT`](#keyctl-supports-decrypt)
  - [`KEYCTL_SUPPORTS_SIGN`](#keyctl-supports-sign)
  - [`KEYCTL_SUPPORTS_VERIFY`](#keyctl-supports-verify)
  - [`KEYCTL_CAPS0_CAPABILITIES`](#keyctl-caps0-capabilities)
  - [`KEYCTL_CAPS0_PERSISTENT_KEYRINGS`](#keyctl-caps0-persistent-keyrings)
  - [`KEYCTL_CAPS0_DIFFIE_HELLMAN`](#keyctl-caps0-diffie-hellman)
  - [`KEYCTL_CAPS0_PUBLIC_KEY`](#keyctl-caps0-public-key)
  - [`KEYCTL_CAPS0_BIG_KEY`](#keyctl-caps0-big-key)
  - [`KEYCTL_CAPS0_INVALIDATE`](#keyctl-caps0-invalidate)
  - [`KEYCTL_CAPS0_RESTRICT_KEYRING`](#keyctl-caps0-restrict-keyring)
  - [`KEYCTL_CAPS0_MOVE`](#keyctl-caps0-move)
  - [`KEYCTL_CAPS1_NS_KEYRING_NAME`](#keyctl-caps1-ns-keyring-name)
  - [`KEYCTL_CAPS1_NS_KEY_TAG`](#keyctl-caps1-ns-key-tag)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`KEY_SPEC_THREAD_KEYRING`](#key-spec-thread-keyring) | const |  |
| [`KEY_SPEC_PROCESS_KEYRING`](#key-spec-process-keyring) | const |  |
| [`KEY_SPEC_SESSION_KEYRING`](#key-spec-session-keyring) | const |  |
| [`KEY_SPEC_USER_KEYRING`](#key-spec-user-keyring) | const |  |
| [`KEY_SPEC_USER_SESSION_KEYRING`](#key-spec-user-session-keyring) | const |  |
| [`KEY_SPEC_GROUP_KEYRING`](#key-spec-group-keyring) | const |  |
| [`KEY_SPEC_REQKEY_AUTH_KEY`](#key-spec-reqkey-auth-key) | const |  |
| [`KEY_SPEC_REQUESTOR_KEYRING`](#key-spec-requestor-keyring) | const |  |
| [`KEY_REQKEY_DEFL_NO_CHANGE`](#key-reqkey-defl-no-change) | const |  |
| [`KEY_REQKEY_DEFL_DEFAULT`](#key-reqkey-defl-default) | const |  |
| [`KEY_REQKEY_DEFL_THREAD_KEYRING`](#key-reqkey-defl-thread-keyring) | const |  |
| [`KEY_REQKEY_DEFL_PROCESS_KEYRING`](#key-reqkey-defl-process-keyring) | const |  |
| [`KEY_REQKEY_DEFL_SESSION_KEYRING`](#key-reqkey-defl-session-keyring) | const |  |
| [`KEY_REQKEY_DEFL_USER_KEYRING`](#key-reqkey-defl-user-keyring) | const |  |
| [`KEY_REQKEY_DEFL_USER_SESSION_KEYRING`](#key-reqkey-defl-user-session-keyring) | const |  |
| [`KEY_REQKEY_DEFL_GROUP_KEYRING`](#key-reqkey-defl-group-keyring) | const |  |
| [`KEY_REQKEY_DEFL_REQUESTOR_KEYRING`](#key-reqkey-defl-requestor-keyring) | const |  |
| [`KEYCTL_GET_KEYRING_ID`](#keyctl-get-keyring-id) | const |  |
| [`KEYCTL_JOIN_SESSION_KEYRING`](#keyctl-join-session-keyring) | const |  |
| [`KEYCTL_UPDATE`](#keyctl-update) | const |  |
| [`KEYCTL_REVOKE`](#keyctl-revoke) | const |  |
| [`KEYCTL_CHOWN`](#keyctl-chown) | const |  |
| [`KEYCTL_SETPERM`](#keyctl-setperm) | const |  |
| [`KEYCTL_DESCRIBE`](#keyctl-describe) | const |  |
| [`KEYCTL_CLEAR`](#keyctl-clear) | const |  |
| [`KEYCTL_LINK`](#keyctl-link) | const |  |
| [`KEYCTL_UNLINK`](#keyctl-unlink) | const |  |
| [`KEYCTL_SEARCH`](#keyctl-search) | const |  |
| [`KEYCTL_READ`](#keyctl-read) | const |  |
| [`KEYCTL_INSTANTIATE`](#keyctl-instantiate) | const |  |
| [`KEYCTL_NEGATE`](#keyctl-negate) | const |  |
| [`KEYCTL_SET_REQKEY_KEYRING`](#keyctl-set-reqkey-keyring) | const |  |
| [`KEYCTL_SET_TIMEOUT`](#keyctl-set-timeout) | const |  |
| [`KEYCTL_ASSUME_AUTHORITY`](#keyctl-assume-authority) | const |  |
| [`KEYCTL_GET_SECURITY`](#keyctl-get-security) | const |  |
| [`KEYCTL_SESSION_TO_PARENT`](#keyctl-session-to-parent) | const |  |
| [`KEYCTL_REJECT`](#keyctl-reject) | const |  |
| [`KEYCTL_INSTANTIATE_IOV`](#keyctl-instantiate-iov) | const |  |
| [`KEYCTL_INVALIDATE`](#keyctl-invalidate) | const |  |
| [`KEYCTL_GET_PERSISTENT`](#keyctl-get-persistent) | const |  |
| [`KEYCTL_DH_COMPUTE`](#keyctl-dh-compute) | const |  |
| [`KEYCTL_PKEY_QUERY`](#keyctl-pkey-query) | const |  |
| [`KEYCTL_PKEY_ENCRYPT`](#keyctl-pkey-encrypt) | const |  |
| [`KEYCTL_PKEY_DECRYPT`](#keyctl-pkey-decrypt) | const |  |
| [`KEYCTL_PKEY_SIGN`](#keyctl-pkey-sign) | const |  |
| [`KEYCTL_PKEY_VERIFY`](#keyctl-pkey-verify) | const |  |
| [`KEYCTL_RESTRICT_KEYRING`](#keyctl-restrict-keyring) | const |  |
| [`KEYCTL_MOVE`](#keyctl-move) | const |  |
| [`KEYCTL_CAPABILITIES`](#keyctl-capabilities) | const |  |
| [`KEYCTL_SUPPORTS_ENCRYPT`](#keyctl-supports-encrypt) | const |  |
| [`KEYCTL_SUPPORTS_DECRYPT`](#keyctl-supports-decrypt) | const |  |
| [`KEYCTL_SUPPORTS_SIGN`](#keyctl-supports-sign) | const |  |
| [`KEYCTL_SUPPORTS_VERIFY`](#keyctl-supports-verify) | const |  |
| [`KEYCTL_CAPS0_CAPABILITIES`](#keyctl-caps0-capabilities) | const |  |
| [`KEYCTL_CAPS0_PERSISTENT_KEYRINGS`](#keyctl-caps0-persistent-keyrings) | const |  |
| [`KEYCTL_CAPS0_DIFFIE_HELLMAN`](#keyctl-caps0-diffie-hellman) | const |  |
| [`KEYCTL_CAPS0_PUBLIC_KEY`](#keyctl-caps0-public-key) | const |  |
| [`KEYCTL_CAPS0_BIG_KEY`](#keyctl-caps0-big-key) | const |  |
| [`KEYCTL_CAPS0_INVALIDATE`](#keyctl-caps0-invalidate) | const |  |
| [`KEYCTL_CAPS0_RESTRICT_KEYRING`](#keyctl-caps0-restrict-keyring) | const |  |
| [`KEYCTL_CAPS0_MOVE`](#keyctl-caps0-move) | const |  |
| [`KEYCTL_CAPS1_NS_KEYRING_NAME`](#keyctl-caps1-ns-keyring-name) | const |  |
| [`KEYCTL_CAPS1_NS_KEY_TAG`](#keyctl-caps1-ns-key-tag) | const |  |

## Constants

### `KEY_SPEC_THREAD_KEYRING`
```rust
const KEY_SPEC_THREAD_KEYRING: i32 = -1i32;
```

### `KEY_SPEC_PROCESS_KEYRING`
```rust
const KEY_SPEC_PROCESS_KEYRING: i32 = -2i32;
```

### `KEY_SPEC_SESSION_KEYRING`
```rust
const KEY_SPEC_SESSION_KEYRING: i32 = -3i32;
```

### `KEY_SPEC_USER_KEYRING`
```rust
const KEY_SPEC_USER_KEYRING: i32 = -4i32;
```

### `KEY_SPEC_USER_SESSION_KEYRING`
```rust
const KEY_SPEC_USER_SESSION_KEYRING: i32 = -5i32;
```

### `KEY_SPEC_GROUP_KEYRING`
```rust
const KEY_SPEC_GROUP_KEYRING: i32 = -6i32;
```

### `KEY_SPEC_REQKEY_AUTH_KEY`
```rust
const KEY_SPEC_REQKEY_AUTH_KEY: i32 = -7i32;
```

### `KEY_SPEC_REQUESTOR_KEYRING`
```rust
const KEY_SPEC_REQUESTOR_KEYRING: i32 = -8i32;
```

### `KEY_REQKEY_DEFL_NO_CHANGE`
```rust
const KEY_REQKEY_DEFL_NO_CHANGE: i32 = -1i32;
```

### `KEY_REQKEY_DEFL_DEFAULT`
```rust
const KEY_REQKEY_DEFL_DEFAULT: i32 = 0i32;
```

### `KEY_REQKEY_DEFL_THREAD_KEYRING`
```rust
const KEY_REQKEY_DEFL_THREAD_KEYRING: i32 = 1i32;
```

### `KEY_REQKEY_DEFL_PROCESS_KEYRING`
```rust
const KEY_REQKEY_DEFL_PROCESS_KEYRING: i32 = 2i32;
```

### `KEY_REQKEY_DEFL_SESSION_KEYRING`
```rust
const KEY_REQKEY_DEFL_SESSION_KEYRING: i32 = 3i32;
```

### `KEY_REQKEY_DEFL_USER_KEYRING`
```rust
const KEY_REQKEY_DEFL_USER_KEYRING: i32 = 4i32;
```

### `KEY_REQKEY_DEFL_USER_SESSION_KEYRING`
```rust
const KEY_REQKEY_DEFL_USER_SESSION_KEYRING: i32 = 5i32;
```

### `KEY_REQKEY_DEFL_GROUP_KEYRING`
```rust
const KEY_REQKEY_DEFL_GROUP_KEYRING: i32 = 6i32;
```

### `KEY_REQKEY_DEFL_REQUESTOR_KEYRING`
```rust
const KEY_REQKEY_DEFL_REQUESTOR_KEYRING: i32 = 7i32;
```

### `KEYCTL_GET_KEYRING_ID`
```rust
const KEYCTL_GET_KEYRING_ID: u32 = 0u32;
```

### `KEYCTL_JOIN_SESSION_KEYRING`
```rust
const KEYCTL_JOIN_SESSION_KEYRING: u32 = 1u32;
```

### `KEYCTL_UPDATE`
```rust
const KEYCTL_UPDATE: u32 = 2u32;
```

### `KEYCTL_REVOKE`
```rust
const KEYCTL_REVOKE: u32 = 3u32;
```

### `KEYCTL_CHOWN`
```rust
const KEYCTL_CHOWN: u32 = 4u32;
```

### `KEYCTL_SETPERM`
```rust
const KEYCTL_SETPERM: u32 = 5u32;
```

### `KEYCTL_DESCRIBE`
```rust
const KEYCTL_DESCRIBE: u32 = 6u32;
```

### `KEYCTL_CLEAR`
```rust
const KEYCTL_CLEAR: u32 = 7u32;
```

### `KEYCTL_LINK`
```rust
const KEYCTL_LINK: u32 = 8u32;
```

### `KEYCTL_UNLINK`
```rust
const KEYCTL_UNLINK: u32 = 9u32;
```

### `KEYCTL_SEARCH`
```rust
const KEYCTL_SEARCH: u32 = 10u32;
```

### `KEYCTL_READ`
```rust
const KEYCTL_READ: u32 = 11u32;
```

### `KEYCTL_INSTANTIATE`
```rust
const KEYCTL_INSTANTIATE: u32 = 12u32;
```

### `KEYCTL_NEGATE`
```rust
const KEYCTL_NEGATE: u32 = 13u32;
```

### `KEYCTL_SET_REQKEY_KEYRING`
```rust
const KEYCTL_SET_REQKEY_KEYRING: u32 = 14u32;
```

### `KEYCTL_SET_TIMEOUT`
```rust
const KEYCTL_SET_TIMEOUT: u32 = 15u32;
```

### `KEYCTL_ASSUME_AUTHORITY`
```rust
const KEYCTL_ASSUME_AUTHORITY: u32 = 16u32;
```

### `KEYCTL_GET_SECURITY`
```rust
const KEYCTL_GET_SECURITY: u32 = 17u32;
```

### `KEYCTL_SESSION_TO_PARENT`
```rust
const KEYCTL_SESSION_TO_PARENT: u32 = 18u32;
```

### `KEYCTL_REJECT`
```rust
const KEYCTL_REJECT: u32 = 19u32;
```

### `KEYCTL_INSTANTIATE_IOV`
```rust
const KEYCTL_INSTANTIATE_IOV: u32 = 20u32;
```

### `KEYCTL_INVALIDATE`
```rust
const KEYCTL_INVALIDATE: u32 = 21u32;
```

### `KEYCTL_GET_PERSISTENT`
```rust
const KEYCTL_GET_PERSISTENT: u32 = 22u32;
```

### `KEYCTL_DH_COMPUTE`
```rust
const KEYCTL_DH_COMPUTE: u32 = 23u32;
```

### `KEYCTL_PKEY_QUERY`
```rust
const KEYCTL_PKEY_QUERY: u32 = 24u32;
```

### `KEYCTL_PKEY_ENCRYPT`
```rust
const KEYCTL_PKEY_ENCRYPT: u32 = 25u32;
```

### `KEYCTL_PKEY_DECRYPT`
```rust
const KEYCTL_PKEY_DECRYPT: u32 = 26u32;
```

### `KEYCTL_PKEY_SIGN`
```rust
const KEYCTL_PKEY_SIGN: u32 = 27u32;
```

### `KEYCTL_PKEY_VERIFY`
```rust
const KEYCTL_PKEY_VERIFY: u32 = 28u32;
```

### `KEYCTL_RESTRICT_KEYRING`
```rust
const KEYCTL_RESTRICT_KEYRING: u32 = 29u32;
```

### `KEYCTL_MOVE`
```rust
const KEYCTL_MOVE: u32 = 30u32;
```

### `KEYCTL_CAPABILITIES`
```rust
const KEYCTL_CAPABILITIES: u32 = 31u32;
```

### `KEYCTL_SUPPORTS_ENCRYPT`
```rust
const KEYCTL_SUPPORTS_ENCRYPT: u32 = 1u32;
```

### `KEYCTL_SUPPORTS_DECRYPT`
```rust
const KEYCTL_SUPPORTS_DECRYPT: u32 = 2u32;
```

### `KEYCTL_SUPPORTS_SIGN`
```rust
const KEYCTL_SUPPORTS_SIGN: u32 = 4u32;
```

### `KEYCTL_SUPPORTS_VERIFY`
```rust
const KEYCTL_SUPPORTS_VERIFY: u32 = 8u32;
```

### `KEYCTL_CAPS0_CAPABILITIES`
```rust
const KEYCTL_CAPS0_CAPABILITIES: u32 = 1u32;
```

### `KEYCTL_CAPS0_PERSISTENT_KEYRINGS`
```rust
const KEYCTL_CAPS0_PERSISTENT_KEYRINGS: u32 = 2u32;
```

### `KEYCTL_CAPS0_DIFFIE_HELLMAN`
```rust
const KEYCTL_CAPS0_DIFFIE_HELLMAN: u32 = 4u32;
```

### `KEYCTL_CAPS0_PUBLIC_KEY`
```rust
const KEYCTL_CAPS0_PUBLIC_KEY: u32 = 8u32;
```

### `KEYCTL_CAPS0_BIG_KEY`
```rust
const KEYCTL_CAPS0_BIG_KEY: u32 = 16u32;
```

### `KEYCTL_CAPS0_INVALIDATE`
```rust
const KEYCTL_CAPS0_INVALIDATE: u32 = 32u32;
```

### `KEYCTL_CAPS0_RESTRICT_KEYRING`
```rust
const KEYCTL_CAPS0_RESTRICT_KEYRING: u32 = 64u32;
```

### `KEYCTL_CAPS0_MOVE`
```rust
const KEYCTL_CAPS0_MOVE: u32 = 128u32;
```

### `KEYCTL_CAPS1_NS_KEYRING_NAME`
```rust
const KEYCTL_CAPS1_NS_KEYRING_NAME: u32 = 1u32;
```

### `KEYCTL_CAPS1_NS_KEY_TAG`
```rust
const KEYCTL_CAPS1_NS_KEY_TAG: u32 = 2u32;
```

