**libc > new > linux_uapi > linux > pidfd**

# Module: new::linux_uapi::linux::pidfd

## Contents

**Structs**

- [`pidfd_info`](#pidfd_info)

**Constants**

- [`PIDFD_GET_CGROUP_NAMESPACE`](#pidfd_get_cgroup_namespace)
- [`PIDFD_GET_INFO`](#pidfd_get_info)
- [`PIDFD_GET_IPC_NAMESPACE`](#pidfd_get_ipc_namespace)
- [`PIDFD_GET_MNT_NAMESPACE`](#pidfd_get_mnt_namespace)
- [`PIDFD_GET_NET_NAMESPACE`](#pidfd_get_net_namespace)
- [`PIDFD_GET_PID_FOR_CHILDREN_NAMESPACE`](#pidfd_get_pid_for_children_namespace)
- [`PIDFD_GET_PID_NAMESPACE`](#pidfd_get_pid_namespace)
- [`PIDFD_GET_TIME_FOR_CHILDREN_NAMESPACE`](#pidfd_get_time_for_children_namespace)
- [`PIDFD_GET_TIME_NAMESPACE`](#pidfd_get_time_namespace)
- [`PIDFD_GET_USER_NAMESPACE`](#pidfd_get_user_namespace)
- [`PIDFD_GET_UTS_NAMESPACE`](#pidfd_get_uts_namespace)
- [`PIDFD_INFO_CGROUPID`](#pidfd_info_cgroupid)
- [`PIDFD_INFO_CREDS`](#pidfd_info_creds)
- [`PIDFD_INFO_EXIT`](#pidfd_info_exit)
- [`PIDFD_INFO_PID`](#pidfd_info_pid)
- [`PIDFD_INFO_SIZE_VER0`](#pidfd_info_size_ver0)
- [`PIDFD_NONBLOCK`](#pidfd_nonblock)
- [`PIDFD_SIGNAL_PROCESS_GROUP`](#pidfd_signal_process_group)
- [`PIDFD_SIGNAL_THREAD`](#pidfd_signal_thread)
- [`PIDFD_SIGNAL_THREAD_GROUP`](#pidfd_signal_thread_group)
- [`PIDFD_THREAD`](#pidfd_thread)

---

## libc::new::linux_uapi::linux::pidfd::PIDFD_GET_CGROUP_NAMESPACE

*Constant*: `c_ulong`



## libc::new::linux_uapi::linux::pidfd::PIDFD_GET_INFO

*Constant*: `c_ulong`



## libc::new::linux_uapi::linux::pidfd::PIDFD_GET_IPC_NAMESPACE

*Constant*: `c_ulong`



## libc::new::linux_uapi::linux::pidfd::PIDFD_GET_MNT_NAMESPACE

*Constant*: `c_ulong`



## libc::new::linux_uapi::linux::pidfd::PIDFD_GET_NET_NAMESPACE

*Constant*: `c_ulong`



## libc::new::linux_uapi::linux::pidfd::PIDFD_GET_PID_FOR_CHILDREN_NAMESPACE

*Constant*: `c_ulong`



## libc::new::linux_uapi::linux::pidfd::PIDFD_GET_PID_NAMESPACE

*Constant*: `c_ulong`



## libc::new::linux_uapi::linux::pidfd::PIDFD_GET_TIME_FOR_CHILDREN_NAMESPACE

*Constant*: `c_ulong`



## libc::new::linux_uapi::linux::pidfd::PIDFD_GET_TIME_NAMESPACE

*Constant*: `c_ulong`



## libc::new::linux_uapi::linux::pidfd::PIDFD_GET_USER_NAMESPACE

*Constant*: `c_ulong`



## libc::new::linux_uapi::linux::pidfd::PIDFD_GET_UTS_NAMESPACE

*Constant*: `c_ulong`



## libc::new::linux_uapi::linux::pidfd::PIDFD_INFO_CGROUPID

*Constant*: `c_uint`



## libc::new::linux_uapi::linux::pidfd::PIDFD_INFO_CREDS

*Constant*: `c_uint`



## libc::new::linux_uapi::linux::pidfd::PIDFD_INFO_EXIT

*Constant*: `c_uint`



## libc::new::linux_uapi::linux::pidfd::PIDFD_INFO_PID

*Constant*: `c_uint`



## libc::new::linux_uapi::linux::pidfd::PIDFD_INFO_SIZE_VER0

*Constant*: `c_uint`



## libc::new::linux_uapi::linux::pidfd::PIDFD_NONBLOCK

*Constant*: `c_uint`



## libc::new::linux_uapi::linux::pidfd::PIDFD_SIGNAL_PROCESS_GROUP

*Constant*: `c_uint`



## libc::new::linux_uapi::linux::pidfd::PIDFD_SIGNAL_THREAD

*Constant*: `c_uint`



## libc::new::linux_uapi::linux::pidfd::PIDFD_SIGNAL_THREAD_GROUP

*Constant*: `c_uint`



## libc::new::linux_uapi::linux::pidfd::PIDFD_THREAD

*Constant*: `c_uint`



## libc::new::linux_uapi::linux::pidfd::pidfd_info

*Struct*

**Fields:**
- `mask: crate::__u64`
- `cgroupid: crate::__u64`
- `pid: crate::__u32`
- `tgid: crate::__u32`
- `ppid: crate::__u32`
- `ruid: crate::__u32`
- `rgid: crate::__u32`
- `euid: crate::__u32`
- `egid: crate::__u32`
- `suid: crate::__u32`
- `sgid: crate::__u32`
- `fsuid: crate::__u32`
- `fsgid: crate::__u32`
- `exit_code: crate::__s32`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> pidfd_info`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



