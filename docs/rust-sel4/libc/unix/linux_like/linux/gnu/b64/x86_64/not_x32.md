**libc > unix > linux_like > linux > gnu > b64 > x86_64 > not_x32**

# Module: unix::linux_like::linux::gnu::b64::x86_64::not_x32

## Contents

**Structs**

- [`statvfs`](#statvfs)

**Functions**

- [`sysctl`](#sysctl)

**Constants**

- [`PTHREAD_ADAPTIVE_MUTEX_INITIALIZER_NP`](#pthread_adaptive_mutex_initializer_np)
- [`PTHREAD_ERRORCHECK_MUTEX_INITIALIZER_NP`](#pthread_errorcheck_mutex_initializer_np)
- [`PTHREAD_RECURSIVE_MUTEX_INITIALIZER_NP`](#pthread_recursive_mutex_initializer_np)
- [`SYS__sysctl`](#sys__sysctl)
- [`SYS_accept`](#sys_accept)
- [`SYS_accept4`](#sys_accept4)
- [`SYS_access`](#sys_access)
- [`SYS_acct`](#sys_acct)
- [`SYS_add_key`](#sys_add_key)
- [`SYS_adjtimex`](#sys_adjtimex)
- [`SYS_afs_syscall`](#sys_afs_syscall)
- [`SYS_alarm`](#sys_alarm)
- [`SYS_arch_prctl`](#sys_arch_prctl)
- [`SYS_bind`](#sys_bind)
- [`SYS_bpf`](#sys_bpf)
- [`SYS_brk`](#sys_brk)
- [`SYS_capget`](#sys_capget)
- [`SYS_capset`](#sys_capset)
- [`SYS_chdir`](#sys_chdir)
- [`SYS_chmod`](#sys_chmod)
- [`SYS_chown`](#sys_chown)
- [`SYS_chroot`](#sys_chroot)
- [`SYS_clock_adjtime`](#sys_clock_adjtime)
- [`SYS_clock_getres`](#sys_clock_getres)
- [`SYS_clock_gettime`](#sys_clock_gettime)
- [`SYS_clock_nanosleep`](#sys_clock_nanosleep)
- [`SYS_clock_settime`](#sys_clock_settime)
- [`SYS_clone`](#sys_clone)
- [`SYS_clone3`](#sys_clone3)
- [`SYS_close`](#sys_close)
- [`SYS_close_range`](#sys_close_range)
- [`SYS_connect`](#sys_connect)
- [`SYS_copy_file_range`](#sys_copy_file_range)
- [`SYS_creat`](#sys_creat)
- [`SYS_create_module`](#sys_create_module)
- [`SYS_delete_module`](#sys_delete_module)
- [`SYS_dup`](#sys_dup)
- [`SYS_dup2`](#sys_dup2)
- [`SYS_dup3`](#sys_dup3)
- [`SYS_epoll_create`](#sys_epoll_create)
- [`SYS_epoll_create1`](#sys_epoll_create1)
- [`SYS_epoll_ctl`](#sys_epoll_ctl)
- [`SYS_epoll_ctl_old`](#sys_epoll_ctl_old)
- [`SYS_epoll_pwait`](#sys_epoll_pwait)
- [`SYS_epoll_pwait2`](#sys_epoll_pwait2)
- [`SYS_epoll_wait`](#sys_epoll_wait)
- [`SYS_epoll_wait_old`](#sys_epoll_wait_old)
- [`SYS_eventfd`](#sys_eventfd)
- [`SYS_eventfd2`](#sys_eventfd2)
- [`SYS_execve`](#sys_execve)
- [`SYS_execveat`](#sys_execveat)
- [`SYS_exit`](#sys_exit)
- [`SYS_exit_group`](#sys_exit_group)
- [`SYS_faccessat`](#sys_faccessat)
- [`SYS_faccessat2`](#sys_faccessat2)
- [`SYS_fadvise64`](#sys_fadvise64)
- [`SYS_fallocate`](#sys_fallocate)
- [`SYS_fanotify_init`](#sys_fanotify_init)
- [`SYS_fanotify_mark`](#sys_fanotify_mark)
- [`SYS_fchdir`](#sys_fchdir)
- [`SYS_fchmod`](#sys_fchmod)
- [`SYS_fchmodat`](#sys_fchmodat)
- [`SYS_fchmodat2`](#sys_fchmodat2)
- [`SYS_fchown`](#sys_fchown)
- [`SYS_fchownat`](#sys_fchownat)
- [`SYS_fcntl`](#sys_fcntl)
- [`SYS_fdatasync`](#sys_fdatasync)
- [`SYS_fgetxattr`](#sys_fgetxattr)
- [`SYS_finit_module`](#sys_finit_module)
- [`SYS_flistxattr`](#sys_flistxattr)
- [`SYS_flock`](#sys_flock)
- [`SYS_fork`](#sys_fork)
- [`SYS_fremovexattr`](#sys_fremovexattr)
- [`SYS_fsconfig`](#sys_fsconfig)
- [`SYS_fsetxattr`](#sys_fsetxattr)
- [`SYS_fsmount`](#sys_fsmount)
- [`SYS_fsopen`](#sys_fsopen)
- [`SYS_fspick`](#sys_fspick)
- [`SYS_fstat`](#sys_fstat)
- [`SYS_fstatfs`](#sys_fstatfs)
- [`SYS_fsync`](#sys_fsync)
- [`SYS_ftruncate`](#sys_ftruncate)
- [`SYS_futex`](#sys_futex)
- [`SYS_futex_waitv`](#sys_futex_waitv)
- [`SYS_futimesat`](#sys_futimesat)
- [`SYS_get_kernel_syms`](#sys_get_kernel_syms)
- [`SYS_get_mempolicy`](#sys_get_mempolicy)
- [`SYS_get_robust_list`](#sys_get_robust_list)
- [`SYS_get_thread_area`](#sys_get_thread_area)
- [`SYS_getcpu`](#sys_getcpu)
- [`SYS_getcwd`](#sys_getcwd)
- [`SYS_getdents`](#sys_getdents)
- [`SYS_getdents64`](#sys_getdents64)
- [`SYS_getegid`](#sys_getegid)
- [`SYS_geteuid`](#sys_geteuid)
- [`SYS_getgid`](#sys_getgid)
- [`SYS_getgroups`](#sys_getgroups)
- [`SYS_getitimer`](#sys_getitimer)
- [`SYS_getpeername`](#sys_getpeername)
- [`SYS_getpgid`](#sys_getpgid)
- [`SYS_getpgrp`](#sys_getpgrp)
- [`SYS_getpid`](#sys_getpid)
- [`SYS_getpmsg`](#sys_getpmsg)
- [`SYS_getppid`](#sys_getppid)
- [`SYS_getpriority`](#sys_getpriority)
- [`SYS_getrandom`](#sys_getrandom)
- [`SYS_getresgid`](#sys_getresgid)
- [`SYS_getresuid`](#sys_getresuid)
- [`SYS_getrlimit`](#sys_getrlimit)
- [`SYS_getrusage`](#sys_getrusage)
- [`SYS_getsid`](#sys_getsid)
- [`SYS_getsockname`](#sys_getsockname)
- [`SYS_getsockopt`](#sys_getsockopt)
- [`SYS_gettid`](#sys_gettid)
- [`SYS_gettimeofday`](#sys_gettimeofday)
- [`SYS_getuid`](#sys_getuid)
- [`SYS_getxattr`](#sys_getxattr)
- [`SYS_init_module`](#sys_init_module)
- [`SYS_inotify_add_watch`](#sys_inotify_add_watch)
- [`SYS_inotify_init`](#sys_inotify_init)
- [`SYS_inotify_init1`](#sys_inotify_init1)
- [`SYS_inotify_rm_watch`](#sys_inotify_rm_watch)
- [`SYS_io_cancel`](#sys_io_cancel)
- [`SYS_io_destroy`](#sys_io_destroy)
- [`SYS_io_getevents`](#sys_io_getevents)
- [`SYS_io_setup`](#sys_io_setup)
- [`SYS_io_submit`](#sys_io_submit)
- [`SYS_io_uring_enter`](#sys_io_uring_enter)
- [`SYS_io_uring_register`](#sys_io_uring_register)
- [`SYS_io_uring_setup`](#sys_io_uring_setup)
- [`SYS_ioctl`](#sys_ioctl)
- [`SYS_ioperm`](#sys_ioperm)
- [`SYS_iopl`](#sys_iopl)
- [`SYS_ioprio_get`](#sys_ioprio_get)
- [`SYS_ioprio_set`](#sys_ioprio_set)
- [`SYS_kcmp`](#sys_kcmp)
- [`SYS_kexec_file_load`](#sys_kexec_file_load)
- [`SYS_kexec_load`](#sys_kexec_load)
- [`SYS_keyctl`](#sys_keyctl)
- [`SYS_kill`](#sys_kill)
- [`SYS_landlock_add_rule`](#sys_landlock_add_rule)
- [`SYS_landlock_create_ruleset`](#sys_landlock_create_ruleset)
- [`SYS_landlock_restrict_self`](#sys_landlock_restrict_self)
- [`SYS_lchown`](#sys_lchown)
- [`SYS_lgetxattr`](#sys_lgetxattr)
- [`SYS_link`](#sys_link)
- [`SYS_linkat`](#sys_linkat)
- [`SYS_listen`](#sys_listen)
- [`SYS_listxattr`](#sys_listxattr)
- [`SYS_llistxattr`](#sys_llistxattr)
- [`SYS_lookup_dcookie`](#sys_lookup_dcookie)
- [`SYS_lremovexattr`](#sys_lremovexattr)
- [`SYS_lseek`](#sys_lseek)
- [`SYS_lsetxattr`](#sys_lsetxattr)
- [`SYS_lstat`](#sys_lstat)
- [`SYS_madvise`](#sys_madvise)
- [`SYS_mbind`](#sys_mbind)
- [`SYS_membarrier`](#sys_membarrier)
- [`SYS_memfd_create`](#sys_memfd_create)
- [`SYS_memfd_secret`](#sys_memfd_secret)
- [`SYS_migrate_pages`](#sys_migrate_pages)
- [`SYS_mincore`](#sys_mincore)
- [`SYS_mkdir`](#sys_mkdir)
- [`SYS_mkdirat`](#sys_mkdirat)
- [`SYS_mknod`](#sys_mknod)
- [`SYS_mknodat`](#sys_mknodat)
- [`SYS_mlock`](#sys_mlock)
- [`SYS_mlock2`](#sys_mlock2)
- [`SYS_mlockall`](#sys_mlockall)
- [`SYS_mmap`](#sys_mmap)
- [`SYS_modify_ldt`](#sys_modify_ldt)
- [`SYS_mount`](#sys_mount)
- [`SYS_mount_setattr`](#sys_mount_setattr)
- [`SYS_move_mount`](#sys_move_mount)
- [`SYS_move_pages`](#sys_move_pages)
- [`SYS_mprotect`](#sys_mprotect)
- [`SYS_mq_getsetattr`](#sys_mq_getsetattr)
- [`SYS_mq_notify`](#sys_mq_notify)
- [`SYS_mq_open`](#sys_mq_open)
- [`SYS_mq_timedreceive`](#sys_mq_timedreceive)
- [`SYS_mq_timedsend`](#sys_mq_timedsend)
- [`SYS_mq_unlink`](#sys_mq_unlink)
- [`SYS_mremap`](#sys_mremap)
- [`SYS_mseal`](#sys_mseal)
- [`SYS_msgctl`](#sys_msgctl)
- [`SYS_msgget`](#sys_msgget)
- [`SYS_msgrcv`](#sys_msgrcv)
- [`SYS_msgsnd`](#sys_msgsnd)
- [`SYS_msync`](#sys_msync)
- [`SYS_munlock`](#sys_munlock)
- [`SYS_munlockall`](#sys_munlockall)
- [`SYS_munmap`](#sys_munmap)
- [`SYS_name_to_handle_at`](#sys_name_to_handle_at)
- [`SYS_nanosleep`](#sys_nanosleep)
- [`SYS_newfstatat`](#sys_newfstatat)
- [`SYS_nfsservctl`](#sys_nfsservctl)
- [`SYS_open`](#sys_open)
- [`SYS_open_by_handle_at`](#sys_open_by_handle_at)
- [`SYS_open_tree`](#sys_open_tree)
- [`SYS_openat`](#sys_openat)
- [`SYS_openat2`](#sys_openat2)
- [`SYS_pause`](#sys_pause)
- [`SYS_perf_event_open`](#sys_perf_event_open)
- [`SYS_personality`](#sys_personality)
- [`SYS_pidfd_getfd`](#sys_pidfd_getfd)
- [`SYS_pidfd_open`](#sys_pidfd_open)
- [`SYS_pidfd_send_signal`](#sys_pidfd_send_signal)
- [`SYS_pipe`](#sys_pipe)
- [`SYS_pipe2`](#sys_pipe2)
- [`SYS_pivot_root`](#sys_pivot_root)
- [`SYS_pkey_alloc`](#sys_pkey_alloc)
- [`SYS_pkey_free`](#sys_pkey_free)
- [`SYS_pkey_mprotect`](#sys_pkey_mprotect)
- [`SYS_poll`](#sys_poll)
- [`SYS_ppoll`](#sys_ppoll)
- [`SYS_prctl`](#sys_prctl)
- [`SYS_pread64`](#sys_pread64)
- [`SYS_preadv`](#sys_preadv)
- [`SYS_preadv2`](#sys_preadv2)
- [`SYS_prlimit64`](#sys_prlimit64)
- [`SYS_process_madvise`](#sys_process_madvise)
- [`SYS_process_mrelease`](#sys_process_mrelease)
- [`SYS_process_vm_readv`](#sys_process_vm_readv)
- [`SYS_process_vm_writev`](#sys_process_vm_writev)
- [`SYS_pselect6`](#sys_pselect6)
- [`SYS_ptrace`](#sys_ptrace)
- [`SYS_putpmsg`](#sys_putpmsg)
- [`SYS_pwrite64`](#sys_pwrite64)
- [`SYS_pwritev`](#sys_pwritev)
- [`SYS_pwritev2`](#sys_pwritev2)
- [`SYS_query_module`](#sys_query_module)
- [`SYS_quotactl`](#sys_quotactl)
- [`SYS_quotactl_fd`](#sys_quotactl_fd)
- [`SYS_read`](#sys_read)
- [`SYS_readahead`](#sys_readahead)
- [`SYS_readlink`](#sys_readlink)
- [`SYS_readlinkat`](#sys_readlinkat)
- [`SYS_readv`](#sys_readv)
- [`SYS_reboot`](#sys_reboot)
- [`SYS_recvfrom`](#sys_recvfrom)
- [`SYS_recvmmsg`](#sys_recvmmsg)
- [`SYS_recvmsg`](#sys_recvmsg)
- [`SYS_remap_file_pages`](#sys_remap_file_pages)
- [`SYS_removexattr`](#sys_removexattr)
- [`SYS_rename`](#sys_rename)
- [`SYS_renameat`](#sys_renameat)
- [`SYS_renameat2`](#sys_renameat2)
- [`SYS_request_key`](#sys_request_key)
- [`SYS_restart_syscall`](#sys_restart_syscall)
- [`SYS_rmdir`](#sys_rmdir)
- [`SYS_rseq`](#sys_rseq)
- [`SYS_rt_sigaction`](#sys_rt_sigaction)
- [`SYS_rt_sigpending`](#sys_rt_sigpending)
- [`SYS_rt_sigprocmask`](#sys_rt_sigprocmask)
- [`SYS_rt_sigqueueinfo`](#sys_rt_sigqueueinfo)
- [`SYS_rt_sigreturn`](#sys_rt_sigreturn)
- [`SYS_rt_sigsuspend`](#sys_rt_sigsuspend)
- [`SYS_rt_sigtimedwait`](#sys_rt_sigtimedwait)
- [`SYS_rt_tgsigqueueinfo`](#sys_rt_tgsigqueueinfo)
- [`SYS_sched_get_priority_max`](#sys_sched_get_priority_max)
- [`SYS_sched_get_priority_min`](#sys_sched_get_priority_min)
- [`SYS_sched_getaffinity`](#sys_sched_getaffinity)
- [`SYS_sched_getattr`](#sys_sched_getattr)
- [`SYS_sched_getparam`](#sys_sched_getparam)
- [`SYS_sched_getscheduler`](#sys_sched_getscheduler)
- [`SYS_sched_rr_get_interval`](#sys_sched_rr_get_interval)
- [`SYS_sched_setaffinity`](#sys_sched_setaffinity)
- [`SYS_sched_setattr`](#sys_sched_setattr)
- [`SYS_sched_setparam`](#sys_sched_setparam)
- [`SYS_sched_setscheduler`](#sys_sched_setscheduler)
- [`SYS_sched_yield`](#sys_sched_yield)
- [`SYS_seccomp`](#sys_seccomp)
- [`SYS_security`](#sys_security)
- [`SYS_select`](#sys_select)
- [`SYS_semctl`](#sys_semctl)
- [`SYS_semget`](#sys_semget)
- [`SYS_semop`](#sys_semop)
- [`SYS_semtimedop`](#sys_semtimedop)
- [`SYS_sendfile`](#sys_sendfile)
- [`SYS_sendmmsg`](#sys_sendmmsg)
- [`SYS_sendmsg`](#sys_sendmsg)
- [`SYS_sendto`](#sys_sendto)
- [`SYS_set_mempolicy`](#sys_set_mempolicy)
- [`SYS_set_mempolicy_home_node`](#sys_set_mempolicy_home_node)
- [`SYS_set_robust_list`](#sys_set_robust_list)
- [`SYS_set_thread_area`](#sys_set_thread_area)
- [`SYS_set_tid_address`](#sys_set_tid_address)
- [`SYS_setdomainname`](#sys_setdomainname)
- [`SYS_setfsgid`](#sys_setfsgid)
- [`SYS_setfsuid`](#sys_setfsuid)
- [`SYS_setgid`](#sys_setgid)
- [`SYS_setgroups`](#sys_setgroups)
- [`SYS_sethostname`](#sys_sethostname)
- [`SYS_setitimer`](#sys_setitimer)
- [`SYS_setns`](#sys_setns)
- [`SYS_setpgid`](#sys_setpgid)
- [`SYS_setpriority`](#sys_setpriority)
- [`SYS_setregid`](#sys_setregid)
- [`SYS_setresgid`](#sys_setresgid)
- [`SYS_setresuid`](#sys_setresuid)
- [`SYS_setreuid`](#sys_setreuid)
- [`SYS_setrlimit`](#sys_setrlimit)
- [`SYS_setsid`](#sys_setsid)
- [`SYS_setsockopt`](#sys_setsockopt)
- [`SYS_settimeofday`](#sys_settimeofday)
- [`SYS_setuid`](#sys_setuid)
- [`SYS_setxattr`](#sys_setxattr)
- [`SYS_shmat`](#sys_shmat)
- [`SYS_shmctl`](#sys_shmctl)
- [`SYS_shmdt`](#sys_shmdt)
- [`SYS_shmget`](#sys_shmget)
- [`SYS_shutdown`](#sys_shutdown)
- [`SYS_sigaltstack`](#sys_sigaltstack)
- [`SYS_signalfd`](#sys_signalfd)
- [`SYS_signalfd4`](#sys_signalfd4)
- [`SYS_socket`](#sys_socket)
- [`SYS_socketpair`](#sys_socketpair)
- [`SYS_splice`](#sys_splice)
- [`SYS_stat`](#sys_stat)
- [`SYS_statfs`](#sys_statfs)
- [`SYS_statx`](#sys_statx)
- [`SYS_swapoff`](#sys_swapoff)
- [`SYS_swapon`](#sys_swapon)
- [`SYS_symlink`](#sys_symlink)
- [`SYS_symlinkat`](#sys_symlinkat)
- [`SYS_sync`](#sys_sync)
- [`SYS_sync_file_range`](#sys_sync_file_range)
- [`SYS_syncfs`](#sys_syncfs)
- [`SYS_sysfs`](#sys_sysfs)
- [`SYS_sysinfo`](#sys_sysinfo)
- [`SYS_syslog`](#sys_syslog)
- [`SYS_tee`](#sys_tee)
- [`SYS_tgkill`](#sys_tgkill)
- [`SYS_time`](#sys_time)
- [`SYS_timer_create`](#sys_timer_create)
- [`SYS_timer_delete`](#sys_timer_delete)
- [`SYS_timer_getoverrun`](#sys_timer_getoverrun)
- [`SYS_timer_gettime`](#sys_timer_gettime)
- [`SYS_timer_settime`](#sys_timer_settime)
- [`SYS_timerfd_create`](#sys_timerfd_create)
- [`SYS_timerfd_gettime`](#sys_timerfd_gettime)
- [`SYS_timerfd_settime`](#sys_timerfd_settime)
- [`SYS_times`](#sys_times)
- [`SYS_tkill`](#sys_tkill)
- [`SYS_truncate`](#sys_truncate)
- [`SYS_tuxcall`](#sys_tuxcall)
- [`SYS_umask`](#sys_umask)
- [`SYS_umount2`](#sys_umount2)
- [`SYS_uname`](#sys_uname)
- [`SYS_unlink`](#sys_unlink)
- [`SYS_unlinkat`](#sys_unlinkat)
- [`SYS_unshare`](#sys_unshare)
- [`SYS_uselib`](#sys_uselib)
- [`SYS_userfaultfd`](#sys_userfaultfd)
- [`SYS_ustat`](#sys_ustat)
- [`SYS_utime`](#sys_utime)
- [`SYS_utimensat`](#sys_utimensat)
- [`SYS_utimes`](#sys_utimes)
- [`SYS_vfork`](#sys_vfork)
- [`SYS_vhangup`](#sys_vhangup)
- [`SYS_vmsplice`](#sys_vmsplice)
- [`SYS_vserver`](#sys_vserver)
- [`SYS_wait4`](#sys_wait4)
- [`SYS_waitid`](#sys_waitid)
- [`SYS_write`](#sys_write)
- [`SYS_writev`](#sys_writev)
- [`__SIZEOF_PTHREAD_BARRIER_T`](#__sizeof_pthread_barrier_t)
- [`__SIZEOF_PTHREAD_MUTEX_T`](#__sizeof_pthread_mutex_t)
- [`__SIZEOF_PTHREAD_RWLOCK_T`](#__sizeof_pthread_rwlock_t)

---

## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::PTHREAD_ADAPTIVE_MUTEX_INITIALIZER_NP

*Constant*: `crate::pthread_mutex_t`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::PTHREAD_ERRORCHECK_MUTEX_INITIALIZER_NP

*Constant*: `crate::pthread_mutex_t`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::PTHREAD_RECURSIVE_MUTEX_INITIALIZER_NP

*Constant*: `crate::pthread_mutex_t`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS__sysctl

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_accept

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_accept4

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_access

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_acct

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_add_key

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_adjtimex

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_afs_syscall

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_alarm

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_arch_prctl

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_bind

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_bpf

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_brk

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_capget

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_capset

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_chdir

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_chmod

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_chown

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_chroot

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_clock_adjtime

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_clock_getres

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_clock_gettime

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_clock_nanosleep

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_clock_settime

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_clone

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_clone3

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_close

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_close_range

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_connect

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_copy_file_range

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_creat

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_create_module

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_delete_module

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_dup

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_dup2

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_dup3

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_epoll_create

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_epoll_create1

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_epoll_ctl

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_epoll_ctl_old

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_epoll_pwait

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_epoll_pwait2

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_epoll_wait

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_epoll_wait_old

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_eventfd

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_eventfd2

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_execve

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_execveat

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_exit

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_exit_group

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_faccessat

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_faccessat2

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_fadvise64

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_fallocate

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_fanotify_init

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_fanotify_mark

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_fchdir

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_fchmod

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_fchmodat

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_fchmodat2

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_fchown

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_fchownat

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_fcntl

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_fdatasync

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_fgetxattr

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_finit_module

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_flistxattr

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_flock

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_fork

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_fremovexattr

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_fsconfig

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_fsetxattr

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_fsmount

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_fsopen

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_fspick

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_fstat

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_fstatfs

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_fsync

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_ftruncate

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_futex

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_futex_waitv

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_futimesat

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_get_kernel_syms

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_get_mempolicy

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_get_robust_list

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_get_thread_area

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_getcpu

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_getcwd

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_getdents

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_getdents64

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_getegid

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_geteuid

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_getgid

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_getgroups

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_getitimer

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_getpeername

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_getpgid

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_getpgrp

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_getpid

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_getpmsg

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_getppid

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_getpriority

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_getrandom

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_getresgid

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_getresuid

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_getrlimit

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_getrusage

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_getsid

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_getsockname

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_getsockopt

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_gettid

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_gettimeofday

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_getuid

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_getxattr

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_init_module

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_inotify_add_watch

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_inotify_init

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_inotify_init1

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_inotify_rm_watch

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_io_cancel

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_io_destroy

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_io_getevents

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_io_setup

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_io_submit

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_io_uring_enter

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_io_uring_register

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_io_uring_setup

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_ioctl

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_ioperm

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_iopl

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_ioprio_get

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_ioprio_set

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_kcmp

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_kexec_file_load

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_kexec_load

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_keyctl

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_kill

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_landlock_add_rule

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_landlock_create_ruleset

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_landlock_restrict_self

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_lchown

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_lgetxattr

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_link

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_linkat

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_listen

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_listxattr

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_llistxattr

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_lookup_dcookie

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_lremovexattr

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_lseek

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_lsetxattr

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_lstat

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_madvise

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_mbind

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_membarrier

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_memfd_create

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_memfd_secret

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_migrate_pages

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_mincore

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_mkdir

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_mkdirat

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_mknod

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_mknodat

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_mlock

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_mlock2

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_mlockall

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_mmap

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_modify_ldt

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_mount

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_mount_setattr

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_move_mount

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_move_pages

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_mprotect

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_mq_getsetattr

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_mq_notify

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_mq_open

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_mq_timedreceive

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_mq_timedsend

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_mq_unlink

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_mremap

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_mseal

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_msgctl

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_msgget

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_msgrcv

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_msgsnd

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_msync

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_munlock

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_munlockall

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_munmap

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_name_to_handle_at

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_nanosleep

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_newfstatat

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_nfsservctl

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_open

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_open_by_handle_at

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_open_tree

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_openat

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_openat2

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_pause

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_perf_event_open

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_personality

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_pidfd_getfd

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_pidfd_open

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_pidfd_send_signal

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_pipe

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_pipe2

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_pivot_root

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_pkey_alloc

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_pkey_free

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_pkey_mprotect

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_poll

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_ppoll

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_prctl

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_pread64

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_preadv

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_preadv2

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_prlimit64

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_process_madvise

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_process_mrelease

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_process_vm_readv

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_process_vm_writev

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_pselect6

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_ptrace

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_putpmsg

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_pwrite64

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_pwritev

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_pwritev2

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_query_module

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_quotactl

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_quotactl_fd

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_read

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_readahead

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_readlink

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_readlinkat

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_readv

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_reboot

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_recvfrom

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_recvmmsg

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_recvmsg

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_remap_file_pages

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_removexattr

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_rename

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_renameat

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_renameat2

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_request_key

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_restart_syscall

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_rmdir

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_rseq

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_rt_sigaction

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_rt_sigpending

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_rt_sigprocmask

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_rt_sigqueueinfo

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_rt_sigreturn

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_rt_sigsuspend

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_rt_sigtimedwait

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_rt_tgsigqueueinfo

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_sched_get_priority_max

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_sched_get_priority_min

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_sched_getaffinity

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_sched_getattr

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_sched_getparam

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_sched_getscheduler

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_sched_rr_get_interval

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_sched_setaffinity

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_sched_setattr

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_sched_setparam

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_sched_setscheduler

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_sched_yield

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_seccomp

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_security

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_select

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_semctl

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_semget

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_semop

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_semtimedop

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_sendfile

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_sendmmsg

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_sendmsg

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_sendto

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_set_mempolicy

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_set_mempolicy_home_node

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_set_robust_list

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_set_thread_area

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_set_tid_address

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_setdomainname

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_setfsgid

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_setfsuid

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_setgid

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_setgroups

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_sethostname

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_setitimer

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_setns

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_setpgid

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_setpriority

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_setregid

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_setresgid

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_setresuid

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_setreuid

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_setrlimit

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_setsid

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_setsockopt

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_settimeofday

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_setuid

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_setxattr

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_shmat

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_shmctl

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_shmdt

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_shmget

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_shutdown

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_sigaltstack

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_signalfd

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_signalfd4

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_socket

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_socketpair

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_splice

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_stat

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_statfs

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_statx

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_swapoff

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_swapon

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_symlink

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_symlinkat

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_sync

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_sync_file_range

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_syncfs

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_sysfs

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_sysinfo

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_syslog

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_tee

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_tgkill

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_time

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_timer_create

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_timer_delete

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_timer_getoverrun

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_timer_gettime

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_timer_settime

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_timerfd_create

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_timerfd_gettime

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_timerfd_settime

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_times

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_tkill

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_truncate

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_tuxcall

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_umask

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_umount2

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_uname

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_unlink

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_unlinkat

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_unshare

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_uselib

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_userfaultfd

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_ustat

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_utime

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_utimensat

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_utimes

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_vfork

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_vhangup

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_vmsplice

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_vserver

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_wait4

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_waitid

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_write

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::SYS_writev

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::__SIZEOF_PTHREAD_BARRIER_T

*Constant*: `usize`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::__SIZEOF_PTHREAD_MUTEX_T

*Constant*: `usize`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::__SIZEOF_PTHREAD_RWLOCK_T

*Constant*: `usize`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::statvfs

*Struct*

**Fields:**
- `f_bsize: c_ulong`
- `f_frsize: c_ulong`
- `f_blocks: crate::fsblkcnt_t`
- `f_bfree: crate::fsblkcnt_t`
- `f_bavail: crate::fsblkcnt_t`
- `f_files: crate::fsfilcnt_t`
- `f_ffree: crate::fsfilcnt_t`
- `f_favail: crate::fsfilcnt_t`
- `f_fsid: c_ulong`
- `f_flag: c_ulong`
- `f_namemax: c_ulong`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> statvfs`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::gnu::b64::x86_64::not_x32::sysctl

*Function*

```rust
fn sysctl(name: *mut c_int, namelen: c_int, oldp: *mut c_void, oldlenp: *mut size_t, newp: *mut c_void, newlen: size_t) -> c_int
```



