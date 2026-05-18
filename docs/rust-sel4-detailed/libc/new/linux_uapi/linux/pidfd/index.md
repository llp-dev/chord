*[libc](../../../../index.md) / [new](../../../index.md) / [linux_uapi](../../index.md) / [linux](../index.md) / [pidfd](index.md)*

---

# Module `pidfd`

Header: `uapi/linux/pidfd.h`

## Contents

- [Structs](#structs)
  - [`pidfd_info`](#pidfd-info)
- [Constants](#constants)
  - [`PIDFD_NONBLOCK`](#pidfd-nonblock)
  - [`PIDFD_THREAD`](#pidfd-thread)
  - [`PIDFD_SIGNAL_THREAD`](#pidfd-signal-thread)
  - [`PIDFD_SIGNAL_THREAD_GROUP`](#pidfd-signal-thread-group)
  - [`PIDFD_SIGNAL_PROCESS_GROUP`](#pidfd-signal-process-group)
  - [`PIDFD_INFO_PID`](#pidfd-info-pid)
  - [`PIDFD_INFO_CREDS`](#pidfd-info-creds)
  - [`PIDFD_INFO_CGROUPID`](#pidfd-info-cgroupid)
  - [`PIDFD_INFO_EXIT`](#pidfd-info-exit)
  - [`PIDFD_INFO_SIZE_VER0`](#pidfd-info-size-ver0)
  - [`PIDFS_IOCTL_MAGIC`](#pidfs-ioctl-magic)
  - [`PIDFD_GET_CGROUP_NAMESPACE`](#pidfd-get-cgroup-namespace)
  - [`PIDFD_GET_IPC_NAMESPACE`](#pidfd-get-ipc-namespace)
  - [`PIDFD_GET_MNT_NAMESPACE`](#pidfd-get-mnt-namespace)
  - [`PIDFD_GET_NET_NAMESPACE`](#pidfd-get-net-namespace)
  - [`PIDFD_GET_PID_NAMESPACE`](#pidfd-get-pid-namespace)
  - [`PIDFD_GET_PID_FOR_CHILDREN_NAMESPACE`](#pidfd-get-pid-for-children-namespace)
  - [`PIDFD_GET_TIME_NAMESPACE`](#pidfd-get-time-namespace)
  - [`PIDFD_GET_TIME_FOR_CHILDREN_NAMESPACE`](#pidfd-get-time-for-children-namespace)
  - [`PIDFD_GET_USER_NAMESPACE`](#pidfd-get-user-namespace)
  - [`PIDFD_GET_UTS_NAMESPACE`](#pidfd-get-uts-namespace)
  - [`PIDFD_GET_INFO`](#pidfd-get-info)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`pidfd_info`](#pidfd-info) | struct |  |
| [`PIDFD_NONBLOCK`](#pidfd-nonblock) | const |  |
| [`PIDFD_THREAD`](#pidfd-thread) | const |  |
| [`PIDFD_SIGNAL_THREAD`](#pidfd-signal-thread) | const |  |
| [`PIDFD_SIGNAL_THREAD_GROUP`](#pidfd-signal-thread-group) | const |  |
| [`PIDFD_SIGNAL_PROCESS_GROUP`](#pidfd-signal-process-group) | const |  |
| [`PIDFD_INFO_PID`](#pidfd-info-pid) | const |  |
| [`PIDFD_INFO_CREDS`](#pidfd-info-creds) | const |  |
| [`PIDFD_INFO_CGROUPID`](#pidfd-info-cgroupid) | const |  |
| [`PIDFD_INFO_EXIT`](#pidfd-info-exit) | const |  |
| [`PIDFD_INFO_SIZE_VER0`](#pidfd-info-size-ver0) | const |  |
| [`PIDFS_IOCTL_MAGIC`](#pidfs-ioctl-magic) | const |  |
| [`PIDFD_GET_CGROUP_NAMESPACE`](#pidfd-get-cgroup-namespace) | const |  |
| [`PIDFD_GET_IPC_NAMESPACE`](#pidfd-get-ipc-namespace) | const |  |
| [`PIDFD_GET_MNT_NAMESPACE`](#pidfd-get-mnt-namespace) | const |  |
| [`PIDFD_GET_NET_NAMESPACE`](#pidfd-get-net-namespace) | const |  |
| [`PIDFD_GET_PID_NAMESPACE`](#pidfd-get-pid-namespace) | const |  |
| [`PIDFD_GET_PID_FOR_CHILDREN_NAMESPACE`](#pidfd-get-pid-for-children-namespace) | const |  |
| [`PIDFD_GET_TIME_NAMESPACE`](#pidfd-get-time-namespace) | const |  |
| [`PIDFD_GET_TIME_FOR_CHILDREN_NAMESPACE`](#pidfd-get-time-for-children-namespace) | const |  |
| [`PIDFD_GET_USER_NAMESPACE`](#pidfd-get-user-namespace) | const |  |
| [`PIDFD_GET_UTS_NAMESPACE`](#pidfd-get-uts-namespace) | const |  |
| [`PIDFD_GET_INFO`](#pidfd-get-info) | const |  |

## Structs

### `pidfd_info`

```rust
struct pidfd_info {
    pub mask: crate::__u64,
    pub cgroupid: crate::__u64,
    pub pid: crate::__u32,
    pub tgid: crate::__u32,
    pub ppid: crate::__u32,
    pub ruid: crate::__u32,
    pub rgid: crate::__u32,
    pub euid: crate::__u32,
    pub egid: crate::__u32,
    pub suid: crate::__u32,
    pub sgid: crate::__u32,
    pub fsuid: crate::__u32,
    pub fsgid: crate::__u32,
    pub exit_code: crate::__s32,
}
```

#### Trait Implementations

##### `impl Clone for pidfd_info`

- <span id="pidfd-info-clone"></span>`fn clone(&self) -> pidfd_info` — [`pidfd_info`](../../../index.md#pidfd-info)

##### `impl Copy for pidfd_info`

##### `impl Debug for pidfd_info`

- <span id="pidfd-info-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Constants

### `PIDFD_NONBLOCK`
```rust
const PIDFD_NONBLOCK: c_uint = 2_048u32;
```

### `PIDFD_THREAD`
```rust
const PIDFD_THREAD: c_uint = 128u32;
```

### `PIDFD_SIGNAL_THREAD`
```rust
const PIDFD_SIGNAL_THREAD: c_uint = 1u32;
```

### `PIDFD_SIGNAL_THREAD_GROUP`
```rust
const PIDFD_SIGNAL_THREAD_GROUP: c_uint = 2u32;
```

### `PIDFD_SIGNAL_PROCESS_GROUP`
```rust
const PIDFD_SIGNAL_PROCESS_GROUP: c_uint = 4u32;
```

### `PIDFD_INFO_PID`
```rust
const PIDFD_INFO_PID: c_uint = 1u32;
```

### `PIDFD_INFO_CREDS`
```rust
const PIDFD_INFO_CREDS: c_uint = 2u32;
```

### `PIDFD_INFO_CGROUPID`
```rust
const PIDFD_INFO_CGROUPID: c_uint = 4u32;
```

### `PIDFD_INFO_EXIT`
```rust
const PIDFD_INFO_EXIT: c_uint = 8u32;
```

### `PIDFD_INFO_SIZE_VER0`
```rust
const PIDFD_INFO_SIZE_VER0: c_uint = 64u32;
```

### `PIDFS_IOCTL_MAGIC`
```rust
const PIDFS_IOCTL_MAGIC: c_uint = 255u32;
```

### `PIDFD_GET_CGROUP_NAMESPACE`
```rust
const PIDFD_GET_CGROUP_NAMESPACE: c_ulong = 65_281u64;
```

### `PIDFD_GET_IPC_NAMESPACE`
```rust
const PIDFD_GET_IPC_NAMESPACE: c_ulong = 65_282u64;
```

### `PIDFD_GET_MNT_NAMESPACE`
```rust
const PIDFD_GET_MNT_NAMESPACE: c_ulong = 65_283u64;
```

### `PIDFD_GET_NET_NAMESPACE`
```rust
const PIDFD_GET_NET_NAMESPACE: c_ulong = 65_284u64;
```

### `PIDFD_GET_PID_NAMESPACE`
```rust
const PIDFD_GET_PID_NAMESPACE: c_ulong = 65_285u64;
```

### `PIDFD_GET_PID_FOR_CHILDREN_NAMESPACE`
```rust
const PIDFD_GET_PID_FOR_CHILDREN_NAMESPACE: c_ulong = 65_286u64;
```

### `PIDFD_GET_TIME_NAMESPACE`
```rust
const PIDFD_GET_TIME_NAMESPACE: c_ulong = 65_287u64;
```

### `PIDFD_GET_TIME_FOR_CHILDREN_NAMESPACE`
```rust
const PIDFD_GET_TIME_FOR_CHILDREN_NAMESPACE: c_ulong = 65_288u64;
```

### `PIDFD_GET_USER_NAMESPACE`
```rust
const PIDFD_GET_USER_NAMESPACE: c_ulong = 65_289u64;
```

### `PIDFD_GET_UTS_NAMESPACE`
```rust
const PIDFD_GET_UTS_NAMESPACE: c_ulong = 65_290u64;
```

### `PIDFD_GET_INFO`
```rust
const PIDFD_GET_INFO: c_ulong = 3_225_485_067u64;
```

