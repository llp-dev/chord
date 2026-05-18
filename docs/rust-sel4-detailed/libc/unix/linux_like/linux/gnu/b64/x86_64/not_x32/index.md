*[libc](../../../../../../../index.md) / [unix](../../../../../../index.md) / [linux_like](../../../../../index.md) / [linux](../../../../index.md) / [gnu](../../../index.md) / [b64](../../index.md) / [x86_64](../index.md) / [not_x32](index.md)*

---

# Module `not_x32`

## Contents

- [Structs](#structs)
  - [`statvfs`](#statvfs)
- [Functions](#functions)
  - [`sysctl`](#sysctl)
- [Constants](#constants)
  - [`__SIZEOF_PTHREAD_MUTEX_T`](#sizeof-pthread-mutex-t)
  - [`__SIZEOF_PTHREAD_RWLOCK_T`](#sizeof-pthread-rwlock-t)
  - [`__SIZEOF_PTHREAD_BARRIER_T`](#sizeof-pthread-barrier-t)
  - [`PTHREAD_RECURSIVE_MUTEX_INITIALIZER_NP`](#pthread-recursive-mutex-initializer-np)
  - [`PTHREAD_ERRORCHECK_MUTEX_INITIALIZER_NP`](#pthread-errorcheck-mutex-initializer-np)
  - [`PTHREAD_ADAPTIVE_MUTEX_INITIALIZER_NP`](#pthread-adaptive-mutex-initializer-np)
  - [`SYS_read`](#sys-read)
  - [`SYS_write`](#sys-write)
  - [`SYS_open`](#sys-open)
  - [`SYS_close`](#sys-close)
  - [`SYS_stat`](#sys-stat)
  - [`SYS_fstat`](#sys-fstat)
  - [`SYS_lstat`](#sys-lstat)
  - [`SYS_poll`](#sys-poll)
  - [`SYS_lseek`](#sys-lseek)
  - [`SYS_mmap`](#sys-mmap)
  - [`SYS_mprotect`](#sys-mprotect)
  - [`SYS_munmap`](#sys-munmap)
  - [`SYS_brk`](#sys-brk)
  - [`SYS_rt_sigaction`](#sys-rt-sigaction)
  - [`SYS_rt_sigprocmask`](#sys-rt-sigprocmask)
  - [`SYS_rt_sigreturn`](#sys-rt-sigreturn)
  - [`SYS_ioctl`](#sys-ioctl)
  - [`SYS_pread64`](#sys-pread64)
  - [`SYS_pwrite64`](#sys-pwrite64)
  - [`SYS_readv`](#sys-readv)
  - [`SYS_writev`](#sys-writev)
  - [`SYS_access`](#sys-access)
  - [`SYS_pipe`](#sys-pipe)
  - [`SYS_select`](#sys-select)
  - [`SYS_sched_yield`](#sys-sched-yield)
  - [`SYS_mremap`](#sys-mremap)
  - [`SYS_msync`](#sys-msync)
  - [`SYS_mincore`](#sys-mincore)
  - [`SYS_madvise`](#sys-madvise)
  - [`SYS_shmget`](#sys-shmget)
  - [`SYS_shmat`](#sys-shmat)
  - [`SYS_shmctl`](#sys-shmctl)
  - [`SYS_dup`](#sys-dup)
  - [`SYS_dup2`](#sys-dup2)
  - [`SYS_pause`](#sys-pause)
  - [`SYS_nanosleep`](#sys-nanosleep)
  - [`SYS_getitimer`](#sys-getitimer)
  - [`SYS_alarm`](#sys-alarm)
  - [`SYS_setitimer`](#sys-setitimer)
  - [`SYS_getpid`](#sys-getpid)
  - [`SYS_sendfile`](#sys-sendfile)
  - [`SYS_socket`](#sys-socket)
  - [`SYS_connect`](#sys-connect)
  - [`SYS_accept`](#sys-accept)
  - [`SYS_sendto`](#sys-sendto)
  - [`SYS_recvfrom`](#sys-recvfrom)
  - [`SYS_sendmsg`](#sys-sendmsg)
  - [`SYS_recvmsg`](#sys-recvmsg)
  - [`SYS_shutdown`](#sys-shutdown)
  - [`SYS_bind`](#sys-bind)
  - [`SYS_listen`](#sys-listen)
  - [`SYS_getsockname`](#sys-getsockname)
  - [`SYS_getpeername`](#sys-getpeername)
  - [`SYS_socketpair`](#sys-socketpair)
  - [`SYS_setsockopt`](#sys-setsockopt)
  - [`SYS_getsockopt`](#sys-getsockopt)
  - [`SYS_clone`](#sys-clone)
  - [`SYS_fork`](#sys-fork)
  - [`SYS_vfork`](#sys-vfork)
  - [`SYS_execve`](#sys-execve)
  - [`SYS_exit`](#sys-exit)
  - [`SYS_wait4`](#sys-wait4)
  - [`SYS_kill`](#sys-kill)
  - [`SYS_uname`](#sys-uname)
  - [`SYS_semget`](#sys-semget)
  - [`SYS_semop`](#sys-semop)
  - [`SYS_semctl`](#sys-semctl)
  - [`SYS_shmdt`](#sys-shmdt)
  - [`SYS_msgget`](#sys-msgget)
  - [`SYS_msgsnd`](#sys-msgsnd)
  - [`SYS_msgrcv`](#sys-msgrcv)
  - [`SYS_msgctl`](#sys-msgctl)
  - [`SYS_fcntl`](#sys-fcntl)
  - [`SYS_flock`](#sys-flock)
  - [`SYS_fsync`](#sys-fsync)
  - [`SYS_fdatasync`](#sys-fdatasync)
  - [`SYS_truncate`](#sys-truncate)
  - [`SYS_ftruncate`](#sys-ftruncate)
  - [`SYS_getdents`](#sys-getdents)
  - [`SYS_getcwd`](#sys-getcwd)
  - [`SYS_chdir`](#sys-chdir)
  - [`SYS_fchdir`](#sys-fchdir)
  - [`SYS_rename`](#sys-rename)
  - [`SYS_mkdir`](#sys-mkdir)
  - [`SYS_rmdir`](#sys-rmdir)
  - [`SYS_creat`](#sys-creat)
  - [`SYS_link`](#sys-link)
  - [`SYS_unlink`](#sys-unlink)
  - [`SYS_symlink`](#sys-symlink)
  - [`SYS_readlink`](#sys-readlink)
  - [`SYS_chmod`](#sys-chmod)
  - [`SYS_fchmod`](#sys-fchmod)
  - [`SYS_chown`](#sys-chown)
  - [`SYS_fchown`](#sys-fchown)
  - [`SYS_lchown`](#sys-lchown)
  - [`SYS_umask`](#sys-umask)
  - [`SYS_gettimeofday`](#sys-gettimeofday)
  - [`SYS_getrlimit`](#sys-getrlimit)
  - [`SYS_getrusage`](#sys-getrusage)
  - [`SYS_sysinfo`](#sys-sysinfo)
  - [`SYS_times`](#sys-times)
  - [`SYS_ptrace`](#sys-ptrace)
  - [`SYS_getuid`](#sys-getuid)
  - [`SYS_syslog`](#sys-syslog)
  - [`SYS_getgid`](#sys-getgid)
  - [`SYS_setuid`](#sys-setuid)
  - [`SYS_setgid`](#sys-setgid)
  - [`SYS_geteuid`](#sys-geteuid)
  - [`SYS_getegid`](#sys-getegid)
  - [`SYS_setpgid`](#sys-setpgid)
  - [`SYS_getppid`](#sys-getppid)
  - [`SYS_getpgrp`](#sys-getpgrp)
  - [`SYS_setsid`](#sys-setsid)
  - [`SYS_setreuid`](#sys-setreuid)
  - [`SYS_setregid`](#sys-setregid)
  - [`SYS_getgroups`](#sys-getgroups)
  - [`SYS_setgroups`](#sys-setgroups)
  - [`SYS_setresuid`](#sys-setresuid)
  - [`SYS_getresuid`](#sys-getresuid)
  - [`SYS_setresgid`](#sys-setresgid)
  - [`SYS_getresgid`](#sys-getresgid)
  - [`SYS_getpgid`](#sys-getpgid)
  - [`SYS_setfsuid`](#sys-setfsuid)
  - [`SYS_setfsgid`](#sys-setfsgid)
  - [`SYS_getsid`](#sys-getsid)
  - [`SYS_capget`](#sys-capget)
  - [`SYS_capset`](#sys-capset)
  - [`SYS_rt_sigpending`](#sys-rt-sigpending)
  - [`SYS_rt_sigtimedwait`](#sys-rt-sigtimedwait)
  - [`SYS_rt_sigqueueinfo`](#sys-rt-sigqueueinfo)
  - [`SYS_rt_sigsuspend`](#sys-rt-sigsuspend)
  - [`SYS_sigaltstack`](#sys-sigaltstack)
  - [`SYS_utime`](#sys-utime)
  - [`SYS_mknod`](#sys-mknod)
  - [`SYS_uselib`](#sys-uselib)
  - [`SYS_personality`](#sys-personality)
  - [`SYS_ustat`](#sys-ustat)
  - [`SYS_statfs`](#sys-statfs)
  - [`SYS_fstatfs`](#sys-fstatfs)
  - [`SYS_sysfs`](#sys-sysfs)
  - [`SYS_getpriority`](#sys-getpriority)
  - [`SYS_setpriority`](#sys-setpriority)
  - [`SYS_sched_setparam`](#sys-sched-setparam)
  - [`SYS_sched_getparam`](#sys-sched-getparam)
  - [`SYS_sched_setscheduler`](#sys-sched-setscheduler)
  - [`SYS_sched_getscheduler`](#sys-sched-getscheduler)
  - [`SYS_sched_get_priority_max`](#sys-sched-get-priority-max)
  - [`SYS_sched_get_priority_min`](#sys-sched-get-priority-min)
  - [`SYS_sched_rr_get_interval`](#sys-sched-rr-get-interval)
  - [`SYS_mlock`](#sys-mlock)
  - [`SYS_munlock`](#sys-munlock)
  - [`SYS_mlockall`](#sys-mlockall)
  - [`SYS_munlockall`](#sys-munlockall)
  - [`SYS_vhangup`](#sys-vhangup)
  - [`SYS_modify_ldt`](#sys-modify-ldt)
  - [`SYS_pivot_root`](#sys-pivot-root)
  - [`SYS__sysctl`](#sys-sysctl)
  - [`SYS_prctl`](#sys-prctl)
  - [`SYS_arch_prctl`](#sys-arch-prctl)
  - [`SYS_adjtimex`](#sys-adjtimex)
  - [`SYS_setrlimit`](#sys-setrlimit)
  - [`SYS_chroot`](#sys-chroot)
  - [`SYS_sync`](#sys-sync)
  - [`SYS_acct`](#sys-acct)
  - [`SYS_settimeofday`](#sys-settimeofday)
  - [`SYS_mount`](#sys-mount)
  - [`SYS_umount2`](#sys-umount2)
  - [`SYS_swapon`](#sys-swapon)
  - [`SYS_swapoff`](#sys-swapoff)
  - [`SYS_reboot`](#sys-reboot)
  - [`SYS_sethostname`](#sys-sethostname)
  - [`SYS_setdomainname`](#sys-setdomainname)
  - [`SYS_iopl`](#sys-iopl)
  - [`SYS_ioperm`](#sys-ioperm)
  - [`SYS_create_module`](#sys-create-module)
  - [`SYS_init_module`](#sys-init-module)
  - [`SYS_delete_module`](#sys-delete-module)
  - [`SYS_get_kernel_syms`](#sys-get-kernel-syms)
  - [`SYS_query_module`](#sys-query-module)
  - [`SYS_quotactl`](#sys-quotactl)
  - [`SYS_nfsservctl`](#sys-nfsservctl)
  - [`SYS_getpmsg`](#sys-getpmsg)
  - [`SYS_putpmsg`](#sys-putpmsg)
  - [`SYS_afs_syscall`](#sys-afs-syscall)
  - [`SYS_tuxcall`](#sys-tuxcall)
  - [`SYS_security`](#sys-security)
  - [`SYS_gettid`](#sys-gettid)
  - [`SYS_readahead`](#sys-readahead)
  - [`SYS_setxattr`](#sys-setxattr)
  - [`SYS_lsetxattr`](#sys-lsetxattr)
  - [`SYS_fsetxattr`](#sys-fsetxattr)
  - [`SYS_getxattr`](#sys-getxattr)
  - [`SYS_lgetxattr`](#sys-lgetxattr)
  - [`SYS_fgetxattr`](#sys-fgetxattr)
  - [`SYS_listxattr`](#sys-listxattr)
  - [`SYS_llistxattr`](#sys-llistxattr)
  - [`SYS_flistxattr`](#sys-flistxattr)
  - [`SYS_removexattr`](#sys-removexattr)
  - [`SYS_lremovexattr`](#sys-lremovexattr)
  - [`SYS_fremovexattr`](#sys-fremovexattr)
  - [`SYS_tkill`](#sys-tkill)
  - [`SYS_time`](#sys-time)
  - [`SYS_futex`](#sys-futex)
  - [`SYS_sched_setaffinity`](#sys-sched-setaffinity)
  - [`SYS_sched_getaffinity`](#sys-sched-getaffinity)
  - [`SYS_set_thread_area`](#sys-set-thread-area)
  - [`SYS_io_setup`](#sys-io-setup)
  - [`SYS_io_destroy`](#sys-io-destroy)
  - [`SYS_io_getevents`](#sys-io-getevents)
  - [`SYS_io_submit`](#sys-io-submit)
  - [`SYS_io_cancel`](#sys-io-cancel)
  - [`SYS_get_thread_area`](#sys-get-thread-area)
  - [`SYS_lookup_dcookie`](#sys-lookup-dcookie)
  - [`SYS_epoll_create`](#sys-epoll-create)
  - [`SYS_epoll_ctl_old`](#sys-epoll-ctl-old)
  - [`SYS_epoll_wait_old`](#sys-epoll-wait-old)
  - [`SYS_remap_file_pages`](#sys-remap-file-pages)
  - [`SYS_getdents64`](#sys-getdents64)
  - [`SYS_set_tid_address`](#sys-set-tid-address)
  - [`SYS_restart_syscall`](#sys-restart-syscall)
  - [`SYS_semtimedop`](#sys-semtimedop)
  - [`SYS_fadvise64`](#sys-fadvise64)
  - [`SYS_timer_create`](#sys-timer-create)
  - [`SYS_timer_settime`](#sys-timer-settime)
  - [`SYS_timer_gettime`](#sys-timer-gettime)
  - [`SYS_timer_getoverrun`](#sys-timer-getoverrun)
  - [`SYS_timer_delete`](#sys-timer-delete)
  - [`SYS_clock_settime`](#sys-clock-settime)
  - [`SYS_clock_gettime`](#sys-clock-gettime)
  - [`SYS_clock_getres`](#sys-clock-getres)
  - [`SYS_clock_nanosleep`](#sys-clock-nanosleep)
  - [`SYS_exit_group`](#sys-exit-group)
  - [`SYS_epoll_wait`](#sys-epoll-wait)
  - [`SYS_epoll_ctl`](#sys-epoll-ctl)
  - [`SYS_tgkill`](#sys-tgkill)
  - [`SYS_utimes`](#sys-utimes)
  - [`SYS_vserver`](#sys-vserver)
  - [`SYS_mbind`](#sys-mbind)
  - [`SYS_set_mempolicy`](#sys-set-mempolicy)
  - [`SYS_get_mempolicy`](#sys-get-mempolicy)
  - [`SYS_mq_open`](#sys-mq-open)
  - [`SYS_mq_unlink`](#sys-mq-unlink)
  - [`SYS_mq_timedsend`](#sys-mq-timedsend)
  - [`SYS_mq_timedreceive`](#sys-mq-timedreceive)
  - [`SYS_mq_notify`](#sys-mq-notify)
  - [`SYS_mq_getsetattr`](#sys-mq-getsetattr)
  - [`SYS_kexec_load`](#sys-kexec-load)
  - [`SYS_waitid`](#sys-waitid)
  - [`SYS_add_key`](#sys-add-key)
  - [`SYS_request_key`](#sys-request-key)
  - [`SYS_keyctl`](#sys-keyctl)
  - [`SYS_ioprio_set`](#sys-ioprio-set)
  - [`SYS_ioprio_get`](#sys-ioprio-get)
  - [`SYS_inotify_init`](#sys-inotify-init)
  - [`SYS_inotify_add_watch`](#sys-inotify-add-watch)
  - [`SYS_inotify_rm_watch`](#sys-inotify-rm-watch)
  - [`SYS_migrate_pages`](#sys-migrate-pages)
  - [`SYS_openat`](#sys-openat)
  - [`SYS_mkdirat`](#sys-mkdirat)
  - [`SYS_mknodat`](#sys-mknodat)
  - [`SYS_fchownat`](#sys-fchownat)
  - [`SYS_futimesat`](#sys-futimesat)
  - [`SYS_newfstatat`](#sys-newfstatat)
  - [`SYS_unlinkat`](#sys-unlinkat)
  - [`SYS_renameat`](#sys-renameat)
  - [`SYS_linkat`](#sys-linkat)
  - [`SYS_symlinkat`](#sys-symlinkat)
  - [`SYS_readlinkat`](#sys-readlinkat)
  - [`SYS_fchmodat`](#sys-fchmodat)
  - [`SYS_faccessat`](#sys-faccessat)
  - [`SYS_pselect6`](#sys-pselect6)
  - [`SYS_ppoll`](#sys-ppoll)
  - [`SYS_unshare`](#sys-unshare)
  - [`SYS_set_robust_list`](#sys-set-robust-list)
  - [`SYS_get_robust_list`](#sys-get-robust-list)
  - [`SYS_splice`](#sys-splice)
  - [`SYS_tee`](#sys-tee)
  - [`SYS_sync_file_range`](#sys-sync-file-range)
  - [`SYS_vmsplice`](#sys-vmsplice)
  - [`SYS_move_pages`](#sys-move-pages)
  - [`SYS_utimensat`](#sys-utimensat)
  - [`SYS_epoll_pwait`](#sys-epoll-pwait)
  - [`SYS_signalfd`](#sys-signalfd)
  - [`SYS_timerfd_create`](#sys-timerfd-create)
  - [`SYS_eventfd`](#sys-eventfd)
  - [`SYS_fallocate`](#sys-fallocate)
  - [`SYS_timerfd_settime`](#sys-timerfd-settime)
  - [`SYS_timerfd_gettime`](#sys-timerfd-gettime)
  - [`SYS_accept4`](#sys-accept4)
  - [`SYS_signalfd4`](#sys-signalfd4)
  - [`SYS_eventfd2`](#sys-eventfd2)
  - [`SYS_epoll_create1`](#sys-epoll-create1)
  - [`SYS_dup3`](#sys-dup3)
  - [`SYS_pipe2`](#sys-pipe2)
  - [`SYS_inotify_init1`](#sys-inotify-init1)
  - [`SYS_preadv`](#sys-preadv)
  - [`SYS_pwritev`](#sys-pwritev)
  - [`SYS_rt_tgsigqueueinfo`](#sys-rt-tgsigqueueinfo)
  - [`SYS_perf_event_open`](#sys-perf-event-open)
  - [`SYS_recvmmsg`](#sys-recvmmsg)
  - [`SYS_fanotify_init`](#sys-fanotify-init)
  - [`SYS_fanotify_mark`](#sys-fanotify-mark)
  - [`SYS_prlimit64`](#sys-prlimit64)
  - [`SYS_name_to_handle_at`](#sys-name-to-handle-at)
  - [`SYS_open_by_handle_at`](#sys-open-by-handle-at)
  - [`SYS_clock_adjtime`](#sys-clock-adjtime)
  - [`SYS_syncfs`](#sys-syncfs)
  - [`SYS_sendmmsg`](#sys-sendmmsg)
  - [`SYS_setns`](#sys-setns)
  - [`SYS_getcpu`](#sys-getcpu)
  - [`SYS_process_vm_readv`](#sys-process-vm-readv)
  - [`SYS_process_vm_writev`](#sys-process-vm-writev)
  - [`SYS_kcmp`](#sys-kcmp)
  - [`SYS_finit_module`](#sys-finit-module)
  - [`SYS_sched_setattr`](#sys-sched-setattr)
  - [`SYS_sched_getattr`](#sys-sched-getattr)
  - [`SYS_renameat2`](#sys-renameat2)
  - [`SYS_seccomp`](#sys-seccomp)
  - [`SYS_getrandom`](#sys-getrandom)
  - [`SYS_memfd_create`](#sys-memfd-create)
  - [`SYS_kexec_file_load`](#sys-kexec-file-load)
  - [`SYS_bpf`](#sys-bpf)
  - [`SYS_execveat`](#sys-execveat)
  - [`SYS_userfaultfd`](#sys-userfaultfd)
  - [`SYS_membarrier`](#sys-membarrier)
  - [`SYS_mlock2`](#sys-mlock2)
  - [`SYS_copy_file_range`](#sys-copy-file-range)
  - [`SYS_preadv2`](#sys-preadv2)
  - [`SYS_pwritev2`](#sys-pwritev2)
  - [`SYS_pkey_mprotect`](#sys-pkey-mprotect)
  - [`SYS_pkey_alloc`](#sys-pkey-alloc)
  - [`SYS_pkey_free`](#sys-pkey-free)
  - [`SYS_statx`](#sys-statx)
  - [`SYS_rseq`](#sys-rseq)
  - [`SYS_pidfd_send_signal`](#sys-pidfd-send-signal)
  - [`SYS_io_uring_setup`](#sys-io-uring-setup)
  - [`SYS_io_uring_enter`](#sys-io-uring-enter)
  - [`SYS_io_uring_register`](#sys-io-uring-register)
  - [`SYS_open_tree`](#sys-open-tree)
  - [`SYS_move_mount`](#sys-move-mount)
  - [`SYS_fsopen`](#sys-fsopen)
  - [`SYS_fsconfig`](#sys-fsconfig)
  - [`SYS_fsmount`](#sys-fsmount)
  - [`SYS_fspick`](#sys-fspick)
  - [`SYS_pidfd_open`](#sys-pidfd-open)
  - [`SYS_clone3`](#sys-clone3)
  - [`SYS_close_range`](#sys-close-range)
  - [`SYS_openat2`](#sys-openat2)
  - [`SYS_pidfd_getfd`](#sys-pidfd-getfd)
  - [`SYS_faccessat2`](#sys-faccessat2)
  - [`SYS_process_madvise`](#sys-process-madvise)
  - [`SYS_epoll_pwait2`](#sys-epoll-pwait2)
  - [`SYS_mount_setattr`](#sys-mount-setattr)
  - [`SYS_quotactl_fd`](#sys-quotactl-fd)
  - [`SYS_landlock_create_ruleset`](#sys-landlock-create-ruleset)
  - [`SYS_landlock_add_rule`](#sys-landlock-add-rule)
  - [`SYS_landlock_restrict_self`](#sys-landlock-restrict-self)
  - [`SYS_memfd_secret`](#sys-memfd-secret)
  - [`SYS_process_mrelease`](#sys-process-mrelease)
  - [`SYS_futex_waitv`](#sys-futex-waitv)
  - [`SYS_set_mempolicy_home_node`](#sys-set-mempolicy-home-node)
  - [`SYS_fchmodat2`](#sys-fchmodat2)
  - [`SYS_mseal`](#sys-mseal)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`statvfs`](#statvfs) | struct |  |
| [`sysctl`](#sysctl) | fn |  |
| [`__SIZEOF_PTHREAD_MUTEX_T`](#sizeof-pthread-mutex-t) | const |  |
| [`__SIZEOF_PTHREAD_RWLOCK_T`](#sizeof-pthread-rwlock-t) | const |  |
| [`__SIZEOF_PTHREAD_BARRIER_T`](#sizeof-pthread-barrier-t) | const |  |
| [`PTHREAD_RECURSIVE_MUTEX_INITIALIZER_NP`](#pthread-recursive-mutex-initializer-np) | const |  |
| [`PTHREAD_ERRORCHECK_MUTEX_INITIALIZER_NP`](#pthread-errorcheck-mutex-initializer-np) | const |  |
| [`PTHREAD_ADAPTIVE_MUTEX_INITIALIZER_NP`](#pthread-adaptive-mutex-initializer-np) | const |  |
| [`SYS_read`](#sys-read) | const |  |
| [`SYS_write`](#sys-write) | const |  |
| [`SYS_open`](#sys-open) | const |  |
| [`SYS_close`](#sys-close) | const |  |
| [`SYS_stat`](#sys-stat) | const |  |
| [`SYS_fstat`](#sys-fstat) | const |  |
| [`SYS_lstat`](#sys-lstat) | const |  |
| [`SYS_poll`](#sys-poll) | const |  |
| [`SYS_lseek`](#sys-lseek) | const |  |
| [`SYS_mmap`](#sys-mmap) | const |  |
| [`SYS_mprotect`](#sys-mprotect) | const |  |
| [`SYS_munmap`](#sys-munmap) | const |  |
| [`SYS_brk`](#sys-brk) | const |  |
| [`SYS_rt_sigaction`](#sys-rt-sigaction) | const |  |
| [`SYS_rt_sigprocmask`](#sys-rt-sigprocmask) | const |  |
| [`SYS_rt_sigreturn`](#sys-rt-sigreturn) | const |  |
| [`SYS_ioctl`](#sys-ioctl) | const |  |
| [`SYS_pread64`](#sys-pread64) | const |  |
| [`SYS_pwrite64`](#sys-pwrite64) | const |  |
| [`SYS_readv`](#sys-readv) | const |  |
| [`SYS_writev`](#sys-writev) | const |  |
| [`SYS_access`](#sys-access) | const |  |
| [`SYS_pipe`](#sys-pipe) | const |  |
| [`SYS_select`](#sys-select) | const |  |
| [`SYS_sched_yield`](#sys-sched-yield) | const |  |
| [`SYS_mremap`](#sys-mremap) | const |  |
| [`SYS_msync`](#sys-msync) | const |  |
| [`SYS_mincore`](#sys-mincore) | const |  |
| [`SYS_madvise`](#sys-madvise) | const |  |
| [`SYS_shmget`](#sys-shmget) | const |  |
| [`SYS_shmat`](#sys-shmat) | const |  |
| [`SYS_shmctl`](#sys-shmctl) | const |  |
| [`SYS_dup`](#sys-dup) | const |  |
| [`SYS_dup2`](#sys-dup2) | const |  |
| [`SYS_pause`](#sys-pause) | const |  |
| [`SYS_nanosleep`](#sys-nanosleep) | const |  |
| [`SYS_getitimer`](#sys-getitimer) | const |  |
| [`SYS_alarm`](#sys-alarm) | const |  |
| [`SYS_setitimer`](#sys-setitimer) | const |  |
| [`SYS_getpid`](#sys-getpid) | const |  |
| [`SYS_sendfile`](#sys-sendfile) | const |  |
| [`SYS_socket`](#sys-socket) | const |  |
| [`SYS_connect`](#sys-connect) | const |  |
| [`SYS_accept`](#sys-accept) | const |  |
| [`SYS_sendto`](#sys-sendto) | const |  |
| [`SYS_recvfrom`](#sys-recvfrom) | const |  |
| [`SYS_sendmsg`](#sys-sendmsg) | const |  |
| [`SYS_recvmsg`](#sys-recvmsg) | const |  |
| [`SYS_shutdown`](#sys-shutdown) | const |  |
| [`SYS_bind`](#sys-bind) | const |  |
| [`SYS_listen`](#sys-listen) | const |  |
| [`SYS_getsockname`](#sys-getsockname) | const |  |
| [`SYS_getpeername`](#sys-getpeername) | const |  |
| [`SYS_socketpair`](#sys-socketpair) | const |  |
| [`SYS_setsockopt`](#sys-setsockopt) | const |  |
| [`SYS_getsockopt`](#sys-getsockopt) | const |  |
| [`SYS_clone`](#sys-clone) | const |  |
| [`SYS_fork`](#sys-fork) | const |  |
| [`SYS_vfork`](#sys-vfork) | const |  |
| [`SYS_execve`](#sys-execve) | const |  |
| [`SYS_exit`](#sys-exit) | const |  |
| [`SYS_wait4`](#sys-wait4) | const |  |
| [`SYS_kill`](#sys-kill) | const |  |
| [`SYS_uname`](#sys-uname) | const |  |
| [`SYS_semget`](#sys-semget) | const |  |
| [`SYS_semop`](#sys-semop) | const |  |
| [`SYS_semctl`](#sys-semctl) | const |  |
| [`SYS_shmdt`](#sys-shmdt) | const |  |
| [`SYS_msgget`](#sys-msgget) | const |  |
| [`SYS_msgsnd`](#sys-msgsnd) | const |  |
| [`SYS_msgrcv`](#sys-msgrcv) | const |  |
| [`SYS_msgctl`](#sys-msgctl) | const |  |
| [`SYS_fcntl`](#sys-fcntl) | const |  |
| [`SYS_flock`](#sys-flock) | const |  |
| [`SYS_fsync`](#sys-fsync) | const |  |
| [`SYS_fdatasync`](#sys-fdatasync) | const |  |
| [`SYS_truncate`](#sys-truncate) | const |  |
| [`SYS_ftruncate`](#sys-ftruncate) | const |  |
| [`SYS_getdents`](#sys-getdents) | const |  |
| [`SYS_getcwd`](#sys-getcwd) | const |  |
| [`SYS_chdir`](#sys-chdir) | const |  |
| [`SYS_fchdir`](#sys-fchdir) | const |  |
| [`SYS_rename`](#sys-rename) | const |  |
| [`SYS_mkdir`](#sys-mkdir) | const |  |
| [`SYS_rmdir`](#sys-rmdir) | const |  |
| [`SYS_creat`](#sys-creat) | const |  |
| [`SYS_link`](#sys-link) | const |  |
| [`SYS_unlink`](#sys-unlink) | const |  |
| [`SYS_symlink`](#sys-symlink) | const |  |
| [`SYS_readlink`](#sys-readlink) | const |  |
| [`SYS_chmod`](#sys-chmod) | const |  |
| [`SYS_fchmod`](#sys-fchmod) | const |  |
| [`SYS_chown`](#sys-chown) | const |  |
| [`SYS_fchown`](#sys-fchown) | const |  |
| [`SYS_lchown`](#sys-lchown) | const |  |
| [`SYS_umask`](#sys-umask) | const |  |
| [`SYS_gettimeofday`](#sys-gettimeofday) | const |  |
| [`SYS_getrlimit`](#sys-getrlimit) | const |  |
| [`SYS_getrusage`](#sys-getrusage) | const |  |
| [`SYS_sysinfo`](#sys-sysinfo) | const |  |
| [`SYS_times`](#sys-times) | const |  |
| [`SYS_ptrace`](#sys-ptrace) | const |  |
| [`SYS_getuid`](#sys-getuid) | const |  |
| [`SYS_syslog`](#sys-syslog) | const |  |
| [`SYS_getgid`](#sys-getgid) | const |  |
| [`SYS_setuid`](#sys-setuid) | const |  |
| [`SYS_setgid`](#sys-setgid) | const |  |
| [`SYS_geteuid`](#sys-geteuid) | const |  |
| [`SYS_getegid`](#sys-getegid) | const |  |
| [`SYS_setpgid`](#sys-setpgid) | const |  |
| [`SYS_getppid`](#sys-getppid) | const |  |
| [`SYS_getpgrp`](#sys-getpgrp) | const |  |
| [`SYS_setsid`](#sys-setsid) | const |  |
| [`SYS_setreuid`](#sys-setreuid) | const |  |
| [`SYS_setregid`](#sys-setregid) | const |  |
| [`SYS_getgroups`](#sys-getgroups) | const |  |
| [`SYS_setgroups`](#sys-setgroups) | const |  |
| [`SYS_setresuid`](#sys-setresuid) | const |  |
| [`SYS_getresuid`](#sys-getresuid) | const |  |
| [`SYS_setresgid`](#sys-setresgid) | const |  |
| [`SYS_getresgid`](#sys-getresgid) | const |  |
| [`SYS_getpgid`](#sys-getpgid) | const |  |
| [`SYS_setfsuid`](#sys-setfsuid) | const |  |
| [`SYS_setfsgid`](#sys-setfsgid) | const |  |
| [`SYS_getsid`](#sys-getsid) | const |  |
| [`SYS_capget`](#sys-capget) | const |  |
| [`SYS_capset`](#sys-capset) | const |  |
| [`SYS_rt_sigpending`](#sys-rt-sigpending) | const |  |
| [`SYS_rt_sigtimedwait`](#sys-rt-sigtimedwait) | const |  |
| [`SYS_rt_sigqueueinfo`](#sys-rt-sigqueueinfo) | const |  |
| [`SYS_rt_sigsuspend`](#sys-rt-sigsuspend) | const |  |
| [`SYS_sigaltstack`](#sys-sigaltstack) | const |  |
| [`SYS_utime`](#sys-utime) | const |  |
| [`SYS_mknod`](#sys-mknod) | const |  |
| [`SYS_uselib`](#sys-uselib) | const |  |
| [`SYS_personality`](#sys-personality) | const |  |
| [`SYS_ustat`](#sys-ustat) | const |  |
| [`SYS_statfs`](#sys-statfs) | const |  |
| [`SYS_fstatfs`](#sys-fstatfs) | const |  |
| [`SYS_sysfs`](#sys-sysfs) | const |  |
| [`SYS_getpriority`](#sys-getpriority) | const |  |
| [`SYS_setpriority`](#sys-setpriority) | const |  |
| [`SYS_sched_setparam`](#sys-sched-setparam) | const |  |
| [`SYS_sched_getparam`](#sys-sched-getparam) | const |  |
| [`SYS_sched_setscheduler`](#sys-sched-setscheduler) | const |  |
| [`SYS_sched_getscheduler`](#sys-sched-getscheduler) | const |  |
| [`SYS_sched_get_priority_max`](#sys-sched-get-priority-max) | const |  |
| [`SYS_sched_get_priority_min`](#sys-sched-get-priority-min) | const |  |
| [`SYS_sched_rr_get_interval`](#sys-sched-rr-get-interval) | const |  |
| [`SYS_mlock`](#sys-mlock) | const |  |
| [`SYS_munlock`](#sys-munlock) | const |  |
| [`SYS_mlockall`](#sys-mlockall) | const |  |
| [`SYS_munlockall`](#sys-munlockall) | const |  |
| [`SYS_vhangup`](#sys-vhangup) | const |  |
| [`SYS_modify_ldt`](#sys-modify-ldt) | const |  |
| [`SYS_pivot_root`](#sys-pivot-root) | const |  |
| [`SYS__sysctl`](#sys-sysctl) | const |  |
| [`SYS_prctl`](#sys-prctl) | const |  |
| [`SYS_arch_prctl`](#sys-arch-prctl) | const |  |
| [`SYS_adjtimex`](#sys-adjtimex) | const |  |
| [`SYS_setrlimit`](#sys-setrlimit) | const |  |
| [`SYS_chroot`](#sys-chroot) | const |  |
| [`SYS_sync`](#sys-sync) | const |  |
| [`SYS_acct`](#sys-acct) | const |  |
| [`SYS_settimeofday`](#sys-settimeofday) | const |  |
| [`SYS_mount`](#sys-mount) | const |  |
| [`SYS_umount2`](#sys-umount2) | const |  |
| [`SYS_swapon`](#sys-swapon) | const |  |
| [`SYS_swapoff`](#sys-swapoff) | const |  |
| [`SYS_reboot`](#sys-reboot) | const |  |
| [`SYS_sethostname`](#sys-sethostname) | const |  |
| [`SYS_setdomainname`](#sys-setdomainname) | const |  |
| [`SYS_iopl`](#sys-iopl) | const |  |
| [`SYS_ioperm`](#sys-ioperm) | const |  |
| [`SYS_create_module`](#sys-create-module) | const |  |
| [`SYS_init_module`](#sys-init-module) | const |  |
| [`SYS_delete_module`](#sys-delete-module) | const |  |
| [`SYS_get_kernel_syms`](#sys-get-kernel-syms) | const |  |
| [`SYS_query_module`](#sys-query-module) | const |  |
| [`SYS_quotactl`](#sys-quotactl) | const |  |
| [`SYS_nfsservctl`](#sys-nfsservctl) | const |  |
| [`SYS_getpmsg`](#sys-getpmsg) | const |  |
| [`SYS_putpmsg`](#sys-putpmsg) | const |  |
| [`SYS_afs_syscall`](#sys-afs-syscall) | const |  |
| [`SYS_tuxcall`](#sys-tuxcall) | const |  |
| [`SYS_security`](#sys-security) | const |  |
| [`SYS_gettid`](#sys-gettid) | const |  |
| [`SYS_readahead`](#sys-readahead) | const |  |
| [`SYS_setxattr`](#sys-setxattr) | const |  |
| [`SYS_lsetxattr`](#sys-lsetxattr) | const |  |
| [`SYS_fsetxattr`](#sys-fsetxattr) | const |  |
| [`SYS_getxattr`](#sys-getxattr) | const |  |
| [`SYS_lgetxattr`](#sys-lgetxattr) | const |  |
| [`SYS_fgetxattr`](#sys-fgetxattr) | const |  |
| [`SYS_listxattr`](#sys-listxattr) | const |  |
| [`SYS_llistxattr`](#sys-llistxattr) | const |  |
| [`SYS_flistxattr`](#sys-flistxattr) | const |  |
| [`SYS_removexattr`](#sys-removexattr) | const |  |
| [`SYS_lremovexattr`](#sys-lremovexattr) | const |  |
| [`SYS_fremovexattr`](#sys-fremovexattr) | const |  |
| [`SYS_tkill`](#sys-tkill) | const |  |
| [`SYS_time`](#sys-time) | const |  |
| [`SYS_futex`](#sys-futex) | const |  |
| [`SYS_sched_setaffinity`](#sys-sched-setaffinity) | const |  |
| [`SYS_sched_getaffinity`](#sys-sched-getaffinity) | const |  |
| [`SYS_set_thread_area`](#sys-set-thread-area) | const |  |
| [`SYS_io_setup`](#sys-io-setup) | const |  |
| [`SYS_io_destroy`](#sys-io-destroy) | const |  |
| [`SYS_io_getevents`](#sys-io-getevents) | const |  |
| [`SYS_io_submit`](#sys-io-submit) | const |  |
| [`SYS_io_cancel`](#sys-io-cancel) | const |  |
| [`SYS_get_thread_area`](#sys-get-thread-area) | const |  |
| [`SYS_lookup_dcookie`](#sys-lookup-dcookie) | const |  |
| [`SYS_epoll_create`](#sys-epoll-create) | const |  |
| [`SYS_epoll_ctl_old`](#sys-epoll-ctl-old) | const |  |
| [`SYS_epoll_wait_old`](#sys-epoll-wait-old) | const |  |
| [`SYS_remap_file_pages`](#sys-remap-file-pages) | const |  |
| [`SYS_getdents64`](#sys-getdents64) | const |  |
| [`SYS_set_tid_address`](#sys-set-tid-address) | const |  |
| [`SYS_restart_syscall`](#sys-restart-syscall) | const |  |
| [`SYS_semtimedop`](#sys-semtimedop) | const |  |
| [`SYS_fadvise64`](#sys-fadvise64) | const |  |
| [`SYS_timer_create`](#sys-timer-create) | const |  |
| [`SYS_timer_settime`](#sys-timer-settime) | const |  |
| [`SYS_timer_gettime`](#sys-timer-gettime) | const |  |
| [`SYS_timer_getoverrun`](#sys-timer-getoverrun) | const |  |
| [`SYS_timer_delete`](#sys-timer-delete) | const |  |
| [`SYS_clock_settime`](#sys-clock-settime) | const |  |
| [`SYS_clock_gettime`](#sys-clock-gettime) | const |  |
| [`SYS_clock_getres`](#sys-clock-getres) | const |  |
| [`SYS_clock_nanosleep`](#sys-clock-nanosleep) | const |  |
| [`SYS_exit_group`](#sys-exit-group) | const |  |
| [`SYS_epoll_wait`](#sys-epoll-wait) | const |  |
| [`SYS_epoll_ctl`](#sys-epoll-ctl) | const |  |
| [`SYS_tgkill`](#sys-tgkill) | const |  |
| [`SYS_utimes`](#sys-utimes) | const |  |
| [`SYS_vserver`](#sys-vserver) | const |  |
| [`SYS_mbind`](#sys-mbind) | const |  |
| [`SYS_set_mempolicy`](#sys-set-mempolicy) | const |  |
| [`SYS_get_mempolicy`](#sys-get-mempolicy) | const |  |
| [`SYS_mq_open`](#sys-mq-open) | const |  |
| [`SYS_mq_unlink`](#sys-mq-unlink) | const |  |
| [`SYS_mq_timedsend`](#sys-mq-timedsend) | const |  |
| [`SYS_mq_timedreceive`](#sys-mq-timedreceive) | const |  |
| [`SYS_mq_notify`](#sys-mq-notify) | const |  |
| [`SYS_mq_getsetattr`](#sys-mq-getsetattr) | const |  |
| [`SYS_kexec_load`](#sys-kexec-load) | const |  |
| [`SYS_waitid`](#sys-waitid) | const |  |
| [`SYS_add_key`](#sys-add-key) | const |  |
| [`SYS_request_key`](#sys-request-key) | const |  |
| [`SYS_keyctl`](#sys-keyctl) | const |  |
| [`SYS_ioprio_set`](#sys-ioprio-set) | const |  |
| [`SYS_ioprio_get`](#sys-ioprio-get) | const |  |
| [`SYS_inotify_init`](#sys-inotify-init) | const |  |
| [`SYS_inotify_add_watch`](#sys-inotify-add-watch) | const |  |
| [`SYS_inotify_rm_watch`](#sys-inotify-rm-watch) | const |  |
| [`SYS_migrate_pages`](#sys-migrate-pages) | const |  |
| [`SYS_openat`](#sys-openat) | const |  |
| [`SYS_mkdirat`](#sys-mkdirat) | const |  |
| [`SYS_mknodat`](#sys-mknodat) | const |  |
| [`SYS_fchownat`](#sys-fchownat) | const |  |
| [`SYS_futimesat`](#sys-futimesat) | const |  |
| [`SYS_newfstatat`](#sys-newfstatat) | const |  |
| [`SYS_unlinkat`](#sys-unlinkat) | const |  |
| [`SYS_renameat`](#sys-renameat) | const |  |
| [`SYS_linkat`](#sys-linkat) | const |  |
| [`SYS_symlinkat`](#sys-symlinkat) | const |  |
| [`SYS_readlinkat`](#sys-readlinkat) | const |  |
| [`SYS_fchmodat`](#sys-fchmodat) | const |  |
| [`SYS_faccessat`](#sys-faccessat) | const |  |
| [`SYS_pselect6`](#sys-pselect6) | const |  |
| [`SYS_ppoll`](#sys-ppoll) | const |  |
| [`SYS_unshare`](#sys-unshare) | const |  |
| [`SYS_set_robust_list`](#sys-set-robust-list) | const |  |
| [`SYS_get_robust_list`](#sys-get-robust-list) | const |  |
| [`SYS_splice`](#sys-splice) | const |  |
| [`SYS_tee`](#sys-tee) | const |  |
| [`SYS_sync_file_range`](#sys-sync-file-range) | const |  |
| [`SYS_vmsplice`](#sys-vmsplice) | const |  |
| [`SYS_move_pages`](#sys-move-pages) | const |  |
| [`SYS_utimensat`](#sys-utimensat) | const |  |
| [`SYS_epoll_pwait`](#sys-epoll-pwait) | const |  |
| [`SYS_signalfd`](#sys-signalfd) | const |  |
| [`SYS_timerfd_create`](#sys-timerfd-create) | const |  |
| [`SYS_eventfd`](#sys-eventfd) | const |  |
| [`SYS_fallocate`](#sys-fallocate) | const |  |
| [`SYS_timerfd_settime`](#sys-timerfd-settime) | const |  |
| [`SYS_timerfd_gettime`](#sys-timerfd-gettime) | const |  |
| [`SYS_accept4`](#sys-accept4) | const |  |
| [`SYS_signalfd4`](#sys-signalfd4) | const |  |
| [`SYS_eventfd2`](#sys-eventfd2) | const |  |
| [`SYS_epoll_create1`](#sys-epoll-create1) | const |  |
| [`SYS_dup3`](#sys-dup3) | const |  |
| [`SYS_pipe2`](#sys-pipe2) | const |  |
| [`SYS_inotify_init1`](#sys-inotify-init1) | const |  |
| [`SYS_preadv`](#sys-preadv) | const |  |
| [`SYS_pwritev`](#sys-pwritev) | const |  |
| [`SYS_rt_tgsigqueueinfo`](#sys-rt-tgsigqueueinfo) | const |  |
| [`SYS_perf_event_open`](#sys-perf-event-open) | const |  |
| [`SYS_recvmmsg`](#sys-recvmmsg) | const |  |
| [`SYS_fanotify_init`](#sys-fanotify-init) | const |  |
| [`SYS_fanotify_mark`](#sys-fanotify-mark) | const |  |
| [`SYS_prlimit64`](#sys-prlimit64) | const |  |
| [`SYS_name_to_handle_at`](#sys-name-to-handle-at) | const |  |
| [`SYS_open_by_handle_at`](#sys-open-by-handle-at) | const |  |
| [`SYS_clock_adjtime`](#sys-clock-adjtime) | const |  |
| [`SYS_syncfs`](#sys-syncfs) | const |  |
| [`SYS_sendmmsg`](#sys-sendmmsg) | const |  |
| [`SYS_setns`](#sys-setns) | const |  |
| [`SYS_getcpu`](#sys-getcpu) | const |  |
| [`SYS_process_vm_readv`](#sys-process-vm-readv) | const |  |
| [`SYS_process_vm_writev`](#sys-process-vm-writev) | const |  |
| [`SYS_kcmp`](#sys-kcmp) | const |  |
| [`SYS_finit_module`](#sys-finit-module) | const |  |
| [`SYS_sched_setattr`](#sys-sched-setattr) | const |  |
| [`SYS_sched_getattr`](#sys-sched-getattr) | const |  |
| [`SYS_renameat2`](#sys-renameat2) | const |  |
| [`SYS_seccomp`](#sys-seccomp) | const |  |
| [`SYS_getrandom`](#sys-getrandom) | const |  |
| [`SYS_memfd_create`](#sys-memfd-create) | const |  |
| [`SYS_kexec_file_load`](#sys-kexec-file-load) | const |  |
| [`SYS_bpf`](#sys-bpf) | const |  |
| [`SYS_execveat`](#sys-execveat) | const |  |
| [`SYS_userfaultfd`](#sys-userfaultfd) | const |  |
| [`SYS_membarrier`](#sys-membarrier) | const |  |
| [`SYS_mlock2`](#sys-mlock2) | const |  |
| [`SYS_copy_file_range`](#sys-copy-file-range) | const |  |
| [`SYS_preadv2`](#sys-preadv2) | const |  |
| [`SYS_pwritev2`](#sys-pwritev2) | const |  |
| [`SYS_pkey_mprotect`](#sys-pkey-mprotect) | const |  |
| [`SYS_pkey_alloc`](#sys-pkey-alloc) | const |  |
| [`SYS_pkey_free`](#sys-pkey-free) | const |  |
| [`SYS_statx`](#sys-statx) | const |  |
| [`SYS_rseq`](#sys-rseq) | const |  |
| [`SYS_pidfd_send_signal`](#sys-pidfd-send-signal) | const |  |
| [`SYS_io_uring_setup`](#sys-io-uring-setup) | const |  |
| [`SYS_io_uring_enter`](#sys-io-uring-enter) | const |  |
| [`SYS_io_uring_register`](#sys-io-uring-register) | const |  |
| [`SYS_open_tree`](#sys-open-tree) | const |  |
| [`SYS_move_mount`](#sys-move-mount) | const |  |
| [`SYS_fsopen`](#sys-fsopen) | const |  |
| [`SYS_fsconfig`](#sys-fsconfig) | const |  |
| [`SYS_fsmount`](#sys-fsmount) | const |  |
| [`SYS_fspick`](#sys-fspick) | const |  |
| [`SYS_pidfd_open`](#sys-pidfd-open) | const |  |
| [`SYS_clone3`](#sys-clone3) | const |  |
| [`SYS_close_range`](#sys-close-range) | const |  |
| [`SYS_openat2`](#sys-openat2) | const |  |
| [`SYS_pidfd_getfd`](#sys-pidfd-getfd) | const |  |
| [`SYS_faccessat2`](#sys-faccessat2) | const |  |
| [`SYS_process_madvise`](#sys-process-madvise) | const |  |
| [`SYS_epoll_pwait2`](#sys-epoll-pwait2) | const |  |
| [`SYS_mount_setattr`](#sys-mount-setattr) | const |  |
| [`SYS_quotactl_fd`](#sys-quotactl-fd) | const |  |
| [`SYS_landlock_create_ruleset`](#sys-landlock-create-ruleset) | const |  |
| [`SYS_landlock_add_rule`](#sys-landlock-add-rule) | const |  |
| [`SYS_landlock_restrict_self`](#sys-landlock-restrict-self) | const |  |
| [`SYS_memfd_secret`](#sys-memfd-secret) | const |  |
| [`SYS_process_mrelease`](#sys-process-mrelease) | const |  |
| [`SYS_futex_waitv`](#sys-futex-waitv) | const |  |
| [`SYS_set_mempolicy_home_node`](#sys-set-mempolicy-home-node) | const |  |
| [`SYS_fchmodat2`](#sys-fchmodat2) | const |  |
| [`SYS_mseal`](#sys-mseal) | const |  |

## Structs

### `statvfs`

```rust
struct statvfs {
    pub f_bsize: c_ulong,
    pub f_frsize: c_ulong,
    pub f_blocks: crate::fsblkcnt_t,
    pub f_bfree: crate::fsblkcnt_t,
    pub f_bavail: crate::fsblkcnt_t,
    pub f_files: crate::fsfilcnt_t,
    pub f_ffree: crate::fsfilcnt_t,
    pub f_favail: crate::fsfilcnt_t,
    pub f_fsid: c_ulong,
    pub f_flag: c_ulong,
    pub f_namemax: c_ulong,
    __f_spare: [c_int; 6],
}
```

#### Trait Implementations

##### `impl Clone for statvfs`

- <span id="statvfs-clone"></span>`fn clone(&self) -> statvfs` — [`statvfs`](../index.md#statvfs)

##### `impl Copy for statvfs`

##### `impl Debug for statvfs`

- <span id="statvfs-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

### `sysctl`

```rust
unsafe fn sysctl(name: *mut c_int, namelen: c_int, oldp: *mut c_void, oldlenp: *mut size_t, newp: *mut c_void, newlen: size_t) -> c_int
```

## Constants

### `__SIZEOF_PTHREAD_MUTEX_T`
```rust
const __SIZEOF_PTHREAD_MUTEX_T: usize = 40usize;
```

### `__SIZEOF_PTHREAD_RWLOCK_T`
```rust
const __SIZEOF_PTHREAD_RWLOCK_T: usize = 56usize;
```

### `__SIZEOF_PTHREAD_BARRIER_T`
```rust
const __SIZEOF_PTHREAD_BARRIER_T: usize = 32usize;
```

### `PTHREAD_RECURSIVE_MUTEX_INITIALIZER_NP`
```rust
const PTHREAD_RECURSIVE_MUTEX_INITIALIZER_NP: crate::pthread_mutex_t;
```

### `PTHREAD_ERRORCHECK_MUTEX_INITIALIZER_NP`
```rust
const PTHREAD_ERRORCHECK_MUTEX_INITIALIZER_NP: crate::pthread_mutex_t;
```

### `PTHREAD_ADAPTIVE_MUTEX_INITIALIZER_NP`
```rust
const PTHREAD_ADAPTIVE_MUTEX_INITIALIZER_NP: crate::pthread_mutex_t;
```

### `SYS_read`
```rust
const SYS_read: c_long = 0i64;
```

### `SYS_write`
```rust
const SYS_write: c_long = 1i64;
```

### `SYS_open`
```rust
const SYS_open: c_long = 2i64;
```

### `SYS_close`
```rust
const SYS_close: c_long = 3i64;
```

### `SYS_stat`
```rust
const SYS_stat: c_long = 4i64;
```

### `SYS_fstat`
```rust
const SYS_fstat: c_long = 5i64;
```

### `SYS_lstat`
```rust
const SYS_lstat: c_long = 6i64;
```

### `SYS_poll`
```rust
const SYS_poll: c_long = 7i64;
```

### `SYS_lseek`
```rust
const SYS_lseek: c_long = 8i64;
```

### `SYS_mmap`
```rust
const SYS_mmap: c_long = 9i64;
```

### `SYS_mprotect`
```rust
const SYS_mprotect: c_long = 10i64;
```

### `SYS_munmap`
```rust
const SYS_munmap: c_long = 11i64;
```

### `SYS_brk`
```rust
const SYS_brk: c_long = 12i64;
```

### `SYS_rt_sigaction`
```rust
const SYS_rt_sigaction: c_long = 13i64;
```

### `SYS_rt_sigprocmask`
```rust
const SYS_rt_sigprocmask: c_long = 14i64;
```

### `SYS_rt_sigreturn`
```rust
const SYS_rt_sigreturn: c_long = 15i64;
```

### `SYS_ioctl`
```rust
const SYS_ioctl: c_long = 16i64;
```

### `SYS_pread64`
```rust
const SYS_pread64: c_long = 17i64;
```

### `SYS_pwrite64`
```rust
const SYS_pwrite64: c_long = 18i64;
```

### `SYS_readv`
```rust
const SYS_readv: c_long = 19i64;
```

### `SYS_writev`
```rust
const SYS_writev: c_long = 20i64;
```

### `SYS_access`
```rust
const SYS_access: c_long = 21i64;
```

### `SYS_pipe`
```rust
const SYS_pipe: c_long = 22i64;
```

### `SYS_select`
```rust
const SYS_select: c_long = 23i64;
```

### `SYS_sched_yield`
```rust
const SYS_sched_yield: c_long = 24i64;
```

### `SYS_mremap`
```rust
const SYS_mremap: c_long = 25i64;
```

### `SYS_msync`
```rust
const SYS_msync: c_long = 26i64;
```

### `SYS_mincore`
```rust
const SYS_mincore: c_long = 27i64;
```

### `SYS_madvise`
```rust
const SYS_madvise: c_long = 28i64;
```

### `SYS_shmget`
```rust
const SYS_shmget: c_long = 29i64;
```

### `SYS_shmat`
```rust
const SYS_shmat: c_long = 30i64;
```

### `SYS_shmctl`
```rust
const SYS_shmctl: c_long = 31i64;
```

### `SYS_dup`
```rust
const SYS_dup: c_long = 32i64;
```

### `SYS_dup2`
```rust
const SYS_dup2: c_long = 33i64;
```

### `SYS_pause`
```rust
const SYS_pause: c_long = 34i64;
```

### `SYS_nanosleep`
```rust
const SYS_nanosleep: c_long = 35i64;
```

### `SYS_getitimer`
```rust
const SYS_getitimer: c_long = 36i64;
```

### `SYS_alarm`
```rust
const SYS_alarm: c_long = 37i64;
```

### `SYS_setitimer`
```rust
const SYS_setitimer: c_long = 38i64;
```

### `SYS_getpid`
```rust
const SYS_getpid: c_long = 39i64;
```

### `SYS_sendfile`
```rust
const SYS_sendfile: c_long = 40i64;
```

### `SYS_socket`
```rust
const SYS_socket: c_long = 41i64;
```

### `SYS_connect`
```rust
const SYS_connect: c_long = 42i64;
```

### `SYS_accept`
```rust
const SYS_accept: c_long = 43i64;
```

### `SYS_sendto`
```rust
const SYS_sendto: c_long = 44i64;
```

### `SYS_recvfrom`
```rust
const SYS_recvfrom: c_long = 45i64;
```

### `SYS_sendmsg`
```rust
const SYS_sendmsg: c_long = 46i64;
```

### `SYS_recvmsg`
```rust
const SYS_recvmsg: c_long = 47i64;
```

### `SYS_shutdown`
```rust
const SYS_shutdown: c_long = 48i64;
```

### `SYS_bind`
```rust
const SYS_bind: c_long = 49i64;
```

### `SYS_listen`
```rust
const SYS_listen: c_long = 50i64;
```

### `SYS_getsockname`
```rust
const SYS_getsockname: c_long = 51i64;
```

### `SYS_getpeername`
```rust
const SYS_getpeername: c_long = 52i64;
```

### `SYS_socketpair`
```rust
const SYS_socketpair: c_long = 53i64;
```

### `SYS_setsockopt`
```rust
const SYS_setsockopt: c_long = 54i64;
```

### `SYS_getsockopt`
```rust
const SYS_getsockopt: c_long = 55i64;
```

### `SYS_clone`
```rust
const SYS_clone: c_long = 56i64;
```

### `SYS_fork`
```rust
const SYS_fork: c_long = 57i64;
```

### `SYS_vfork`
```rust
const SYS_vfork: c_long = 58i64;
```

### `SYS_execve`
```rust
const SYS_execve: c_long = 59i64;
```

### `SYS_exit`
```rust
const SYS_exit: c_long = 60i64;
```

### `SYS_wait4`
```rust
const SYS_wait4: c_long = 61i64;
```

### `SYS_kill`
```rust
const SYS_kill: c_long = 62i64;
```

### `SYS_uname`
```rust
const SYS_uname: c_long = 63i64;
```

### `SYS_semget`
```rust
const SYS_semget: c_long = 64i64;
```

### `SYS_semop`
```rust
const SYS_semop: c_long = 65i64;
```

### `SYS_semctl`
```rust
const SYS_semctl: c_long = 66i64;
```

### `SYS_shmdt`
```rust
const SYS_shmdt: c_long = 67i64;
```

### `SYS_msgget`
```rust
const SYS_msgget: c_long = 68i64;
```

### `SYS_msgsnd`
```rust
const SYS_msgsnd: c_long = 69i64;
```

### `SYS_msgrcv`
```rust
const SYS_msgrcv: c_long = 70i64;
```

### `SYS_msgctl`
```rust
const SYS_msgctl: c_long = 71i64;
```

### `SYS_fcntl`
```rust
const SYS_fcntl: c_long = 72i64;
```

### `SYS_flock`
```rust
const SYS_flock: c_long = 73i64;
```

### `SYS_fsync`
```rust
const SYS_fsync: c_long = 74i64;
```

### `SYS_fdatasync`
```rust
const SYS_fdatasync: c_long = 75i64;
```

### `SYS_truncate`
```rust
const SYS_truncate: c_long = 76i64;
```

### `SYS_ftruncate`
```rust
const SYS_ftruncate: c_long = 77i64;
```

### `SYS_getdents`
```rust
const SYS_getdents: c_long = 78i64;
```

### `SYS_getcwd`
```rust
const SYS_getcwd: c_long = 79i64;
```

### `SYS_chdir`
```rust
const SYS_chdir: c_long = 80i64;
```

### `SYS_fchdir`
```rust
const SYS_fchdir: c_long = 81i64;
```

### `SYS_rename`
```rust
const SYS_rename: c_long = 82i64;
```

### `SYS_mkdir`
```rust
const SYS_mkdir: c_long = 83i64;
```

### `SYS_rmdir`
```rust
const SYS_rmdir: c_long = 84i64;
```

### `SYS_creat`
```rust
const SYS_creat: c_long = 85i64;
```

### `SYS_link`
```rust
const SYS_link: c_long = 86i64;
```

### `SYS_unlink`
```rust
const SYS_unlink: c_long = 87i64;
```

### `SYS_symlink`
```rust
const SYS_symlink: c_long = 88i64;
```

### `SYS_readlink`
```rust
const SYS_readlink: c_long = 89i64;
```

### `SYS_chmod`
```rust
const SYS_chmod: c_long = 90i64;
```

### `SYS_fchmod`
```rust
const SYS_fchmod: c_long = 91i64;
```

### `SYS_chown`
```rust
const SYS_chown: c_long = 92i64;
```

### `SYS_fchown`
```rust
const SYS_fchown: c_long = 93i64;
```

### `SYS_lchown`
```rust
const SYS_lchown: c_long = 94i64;
```

### `SYS_umask`
```rust
const SYS_umask: c_long = 95i64;
```

### `SYS_gettimeofday`
```rust
const SYS_gettimeofday: c_long = 96i64;
```

### `SYS_getrlimit`
```rust
const SYS_getrlimit: c_long = 97i64;
```

### `SYS_getrusage`
```rust
const SYS_getrusage: c_long = 98i64;
```

### `SYS_sysinfo`
```rust
const SYS_sysinfo: c_long = 99i64;
```

### `SYS_times`
```rust
const SYS_times: c_long = 100i64;
```

### `SYS_ptrace`
```rust
const SYS_ptrace: c_long = 101i64;
```

### `SYS_getuid`
```rust
const SYS_getuid: c_long = 102i64;
```

### `SYS_syslog`
```rust
const SYS_syslog: c_long = 103i64;
```

### `SYS_getgid`
```rust
const SYS_getgid: c_long = 104i64;
```

### `SYS_setuid`
```rust
const SYS_setuid: c_long = 105i64;
```

### `SYS_setgid`
```rust
const SYS_setgid: c_long = 106i64;
```

### `SYS_geteuid`
```rust
const SYS_geteuid: c_long = 107i64;
```

### `SYS_getegid`
```rust
const SYS_getegid: c_long = 108i64;
```

### `SYS_setpgid`
```rust
const SYS_setpgid: c_long = 109i64;
```

### `SYS_getppid`
```rust
const SYS_getppid: c_long = 110i64;
```

### `SYS_getpgrp`
```rust
const SYS_getpgrp: c_long = 111i64;
```

### `SYS_setsid`
```rust
const SYS_setsid: c_long = 112i64;
```

### `SYS_setreuid`
```rust
const SYS_setreuid: c_long = 113i64;
```

### `SYS_setregid`
```rust
const SYS_setregid: c_long = 114i64;
```

### `SYS_getgroups`
```rust
const SYS_getgroups: c_long = 115i64;
```

### `SYS_setgroups`
```rust
const SYS_setgroups: c_long = 116i64;
```

### `SYS_setresuid`
```rust
const SYS_setresuid: c_long = 117i64;
```

### `SYS_getresuid`
```rust
const SYS_getresuid: c_long = 118i64;
```

### `SYS_setresgid`
```rust
const SYS_setresgid: c_long = 119i64;
```

### `SYS_getresgid`
```rust
const SYS_getresgid: c_long = 120i64;
```

### `SYS_getpgid`
```rust
const SYS_getpgid: c_long = 121i64;
```

### `SYS_setfsuid`
```rust
const SYS_setfsuid: c_long = 122i64;
```

### `SYS_setfsgid`
```rust
const SYS_setfsgid: c_long = 123i64;
```

### `SYS_getsid`
```rust
const SYS_getsid: c_long = 124i64;
```

### `SYS_capget`
```rust
const SYS_capget: c_long = 125i64;
```

### `SYS_capset`
```rust
const SYS_capset: c_long = 126i64;
```

### `SYS_rt_sigpending`
```rust
const SYS_rt_sigpending: c_long = 127i64;
```

### `SYS_rt_sigtimedwait`
```rust
const SYS_rt_sigtimedwait: c_long = 128i64;
```

### `SYS_rt_sigqueueinfo`
```rust
const SYS_rt_sigqueueinfo: c_long = 129i64;
```

### `SYS_rt_sigsuspend`
```rust
const SYS_rt_sigsuspend: c_long = 130i64;
```

### `SYS_sigaltstack`
```rust
const SYS_sigaltstack: c_long = 131i64;
```

### `SYS_utime`
```rust
const SYS_utime: c_long = 132i64;
```

### `SYS_mknod`
```rust
const SYS_mknod: c_long = 133i64;
```

### `SYS_uselib`
```rust
const SYS_uselib: c_long = 134i64;
```

### `SYS_personality`
```rust
const SYS_personality: c_long = 135i64;
```

### `SYS_ustat`
```rust
const SYS_ustat: c_long = 136i64;
```

### `SYS_statfs`
```rust
const SYS_statfs: c_long = 137i64;
```

### `SYS_fstatfs`
```rust
const SYS_fstatfs: c_long = 138i64;
```

### `SYS_sysfs`
```rust
const SYS_sysfs: c_long = 139i64;
```

### `SYS_getpriority`
```rust
const SYS_getpriority: c_long = 140i64;
```

### `SYS_setpriority`
```rust
const SYS_setpriority: c_long = 141i64;
```

### `SYS_sched_setparam`
```rust
const SYS_sched_setparam: c_long = 142i64;
```

### `SYS_sched_getparam`
```rust
const SYS_sched_getparam: c_long = 143i64;
```

### `SYS_sched_setscheduler`
```rust
const SYS_sched_setscheduler: c_long = 144i64;
```

### `SYS_sched_getscheduler`
```rust
const SYS_sched_getscheduler: c_long = 145i64;
```

### `SYS_sched_get_priority_max`
```rust
const SYS_sched_get_priority_max: c_long = 146i64;
```

### `SYS_sched_get_priority_min`
```rust
const SYS_sched_get_priority_min: c_long = 147i64;
```

### `SYS_sched_rr_get_interval`
```rust
const SYS_sched_rr_get_interval: c_long = 148i64;
```

### `SYS_mlock`
```rust
const SYS_mlock: c_long = 149i64;
```

### `SYS_munlock`
```rust
const SYS_munlock: c_long = 150i64;
```

### `SYS_mlockall`
```rust
const SYS_mlockall: c_long = 151i64;
```

### `SYS_munlockall`
```rust
const SYS_munlockall: c_long = 152i64;
```

### `SYS_vhangup`
```rust
const SYS_vhangup: c_long = 153i64;
```

### `SYS_modify_ldt`
```rust
const SYS_modify_ldt: c_long = 154i64;
```

### `SYS_pivot_root`
```rust
const SYS_pivot_root: c_long = 155i64;
```

### `SYS__sysctl`
```rust
const SYS__sysctl: c_long = 156i64;
```

### `SYS_prctl`
```rust
const SYS_prctl: c_long = 157i64;
```

### `SYS_arch_prctl`
```rust
const SYS_arch_prctl: c_long = 158i64;
```

### `SYS_adjtimex`
```rust
const SYS_adjtimex: c_long = 159i64;
```

### `SYS_setrlimit`
```rust
const SYS_setrlimit: c_long = 160i64;
```

### `SYS_chroot`
```rust
const SYS_chroot: c_long = 161i64;
```

### `SYS_sync`
```rust
const SYS_sync: c_long = 162i64;
```

### `SYS_acct`
```rust
const SYS_acct: c_long = 163i64;
```

### `SYS_settimeofday`
```rust
const SYS_settimeofday: c_long = 164i64;
```

### `SYS_mount`
```rust
const SYS_mount: c_long = 165i64;
```

### `SYS_umount2`
```rust
const SYS_umount2: c_long = 166i64;
```

### `SYS_swapon`
```rust
const SYS_swapon: c_long = 167i64;
```

### `SYS_swapoff`
```rust
const SYS_swapoff: c_long = 168i64;
```

### `SYS_reboot`
```rust
const SYS_reboot: c_long = 169i64;
```

### `SYS_sethostname`
```rust
const SYS_sethostname: c_long = 170i64;
```

### `SYS_setdomainname`
```rust
const SYS_setdomainname: c_long = 171i64;
```

### `SYS_iopl`
```rust
const SYS_iopl: c_long = 172i64;
```

### `SYS_ioperm`
```rust
const SYS_ioperm: c_long = 173i64;
```

### `SYS_create_module`
```rust
const SYS_create_module: c_long = 174i64;
```

### `SYS_init_module`
```rust
const SYS_init_module: c_long = 175i64;
```

### `SYS_delete_module`
```rust
const SYS_delete_module: c_long = 176i64;
```

### `SYS_get_kernel_syms`
```rust
const SYS_get_kernel_syms: c_long = 177i64;
```

### `SYS_query_module`
```rust
const SYS_query_module: c_long = 178i64;
```

### `SYS_quotactl`
```rust
const SYS_quotactl: c_long = 179i64;
```

### `SYS_nfsservctl`
```rust
const SYS_nfsservctl: c_long = 180i64;
```

### `SYS_getpmsg`
```rust
const SYS_getpmsg: c_long = 181i64;
```

### `SYS_putpmsg`
```rust
const SYS_putpmsg: c_long = 182i64;
```

### `SYS_afs_syscall`
```rust
const SYS_afs_syscall: c_long = 183i64;
```

### `SYS_tuxcall`
```rust
const SYS_tuxcall: c_long = 184i64;
```

### `SYS_security`
```rust
const SYS_security: c_long = 185i64;
```

### `SYS_gettid`
```rust
const SYS_gettid: c_long = 186i64;
```

### `SYS_readahead`
```rust
const SYS_readahead: c_long = 187i64;
```

### `SYS_setxattr`
```rust
const SYS_setxattr: c_long = 188i64;
```

### `SYS_lsetxattr`
```rust
const SYS_lsetxattr: c_long = 189i64;
```

### `SYS_fsetxattr`
```rust
const SYS_fsetxattr: c_long = 190i64;
```

### `SYS_getxattr`
```rust
const SYS_getxattr: c_long = 191i64;
```

### `SYS_lgetxattr`
```rust
const SYS_lgetxattr: c_long = 192i64;
```

### `SYS_fgetxattr`
```rust
const SYS_fgetxattr: c_long = 193i64;
```

### `SYS_listxattr`
```rust
const SYS_listxattr: c_long = 194i64;
```

### `SYS_llistxattr`
```rust
const SYS_llistxattr: c_long = 195i64;
```

### `SYS_flistxattr`
```rust
const SYS_flistxattr: c_long = 196i64;
```

### `SYS_removexattr`
```rust
const SYS_removexattr: c_long = 197i64;
```

### `SYS_lremovexattr`
```rust
const SYS_lremovexattr: c_long = 198i64;
```

### `SYS_fremovexattr`
```rust
const SYS_fremovexattr: c_long = 199i64;
```

### `SYS_tkill`
```rust
const SYS_tkill: c_long = 200i64;
```

### `SYS_time`
```rust
const SYS_time: c_long = 201i64;
```

### `SYS_futex`
```rust
const SYS_futex: c_long = 202i64;
```

### `SYS_sched_setaffinity`
```rust
const SYS_sched_setaffinity: c_long = 203i64;
```

### `SYS_sched_getaffinity`
```rust
const SYS_sched_getaffinity: c_long = 204i64;
```

### `SYS_set_thread_area`
```rust
const SYS_set_thread_area: c_long = 205i64;
```

### `SYS_io_setup`
```rust
const SYS_io_setup: c_long = 206i64;
```

### `SYS_io_destroy`
```rust
const SYS_io_destroy: c_long = 207i64;
```

### `SYS_io_getevents`
```rust
const SYS_io_getevents: c_long = 208i64;
```

### `SYS_io_submit`
```rust
const SYS_io_submit: c_long = 209i64;
```

### `SYS_io_cancel`
```rust
const SYS_io_cancel: c_long = 210i64;
```

### `SYS_get_thread_area`
```rust
const SYS_get_thread_area: c_long = 211i64;
```

### `SYS_lookup_dcookie`
```rust
const SYS_lookup_dcookie: c_long = 212i64;
```

### `SYS_epoll_create`
```rust
const SYS_epoll_create: c_long = 213i64;
```

### `SYS_epoll_ctl_old`
```rust
const SYS_epoll_ctl_old: c_long = 214i64;
```

### `SYS_epoll_wait_old`
```rust
const SYS_epoll_wait_old: c_long = 215i64;
```

### `SYS_remap_file_pages`
```rust
const SYS_remap_file_pages: c_long = 216i64;
```

### `SYS_getdents64`
```rust
const SYS_getdents64: c_long = 217i64;
```

### `SYS_set_tid_address`
```rust
const SYS_set_tid_address: c_long = 218i64;
```

### `SYS_restart_syscall`
```rust
const SYS_restart_syscall: c_long = 219i64;
```

### `SYS_semtimedop`
```rust
const SYS_semtimedop: c_long = 220i64;
```

### `SYS_fadvise64`
```rust
const SYS_fadvise64: c_long = 221i64;
```

### `SYS_timer_create`
```rust
const SYS_timer_create: c_long = 222i64;
```

### `SYS_timer_settime`
```rust
const SYS_timer_settime: c_long = 223i64;
```

### `SYS_timer_gettime`
```rust
const SYS_timer_gettime: c_long = 224i64;
```

### `SYS_timer_getoverrun`
```rust
const SYS_timer_getoverrun: c_long = 225i64;
```

### `SYS_timer_delete`
```rust
const SYS_timer_delete: c_long = 226i64;
```

### `SYS_clock_settime`
```rust
const SYS_clock_settime: c_long = 227i64;
```

### `SYS_clock_gettime`
```rust
const SYS_clock_gettime: c_long = 228i64;
```

### `SYS_clock_getres`
```rust
const SYS_clock_getres: c_long = 229i64;
```

### `SYS_clock_nanosleep`
```rust
const SYS_clock_nanosleep: c_long = 230i64;
```

### `SYS_exit_group`
```rust
const SYS_exit_group: c_long = 231i64;
```

### `SYS_epoll_wait`
```rust
const SYS_epoll_wait: c_long = 232i64;
```

### `SYS_epoll_ctl`
```rust
const SYS_epoll_ctl: c_long = 233i64;
```

### `SYS_tgkill`
```rust
const SYS_tgkill: c_long = 234i64;
```

### `SYS_utimes`
```rust
const SYS_utimes: c_long = 235i64;
```

### `SYS_vserver`
```rust
const SYS_vserver: c_long = 236i64;
```

### `SYS_mbind`
```rust
const SYS_mbind: c_long = 237i64;
```

### `SYS_set_mempolicy`
```rust
const SYS_set_mempolicy: c_long = 238i64;
```

### `SYS_get_mempolicy`
```rust
const SYS_get_mempolicy: c_long = 239i64;
```

### `SYS_mq_open`
```rust
const SYS_mq_open: c_long = 240i64;
```

### `SYS_mq_unlink`
```rust
const SYS_mq_unlink: c_long = 241i64;
```

### `SYS_mq_timedsend`
```rust
const SYS_mq_timedsend: c_long = 242i64;
```

### `SYS_mq_timedreceive`
```rust
const SYS_mq_timedreceive: c_long = 243i64;
```

### `SYS_mq_notify`
```rust
const SYS_mq_notify: c_long = 244i64;
```

### `SYS_mq_getsetattr`
```rust
const SYS_mq_getsetattr: c_long = 245i64;
```

### `SYS_kexec_load`
```rust
const SYS_kexec_load: c_long = 246i64;
```

### `SYS_waitid`
```rust
const SYS_waitid: c_long = 247i64;
```

### `SYS_add_key`
```rust
const SYS_add_key: c_long = 248i64;
```

### `SYS_request_key`
```rust
const SYS_request_key: c_long = 249i64;
```

### `SYS_keyctl`
```rust
const SYS_keyctl: c_long = 250i64;
```

### `SYS_ioprio_set`
```rust
const SYS_ioprio_set: c_long = 251i64;
```

### `SYS_ioprio_get`
```rust
const SYS_ioprio_get: c_long = 252i64;
```

### `SYS_inotify_init`
```rust
const SYS_inotify_init: c_long = 253i64;
```

### `SYS_inotify_add_watch`
```rust
const SYS_inotify_add_watch: c_long = 254i64;
```

### `SYS_inotify_rm_watch`
```rust
const SYS_inotify_rm_watch: c_long = 255i64;
```

### `SYS_migrate_pages`
```rust
const SYS_migrate_pages: c_long = 256i64;
```

### `SYS_openat`
```rust
const SYS_openat: c_long = 257i64;
```

### `SYS_mkdirat`
```rust
const SYS_mkdirat: c_long = 258i64;
```

### `SYS_mknodat`
```rust
const SYS_mknodat: c_long = 259i64;
```

### `SYS_fchownat`
```rust
const SYS_fchownat: c_long = 260i64;
```

### `SYS_futimesat`
```rust
const SYS_futimesat: c_long = 261i64;
```

### `SYS_newfstatat`
```rust
const SYS_newfstatat: c_long = 262i64;
```

### `SYS_unlinkat`
```rust
const SYS_unlinkat: c_long = 263i64;
```

### `SYS_renameat`
```rust
const SYS_renameat: c_long = 264i64;
```

### `SYS_linkat`
```rust
const SYS_linkat: c_long = 265i64;
```

### `SYS_symlinkat`
```rust
const SYS_symlinkat: c_long = 266i64;
```

### `SYS_readlinkat`
```rust
const SYS_readlinkat: c_long = 267i64;
```

### `SYS_fchmodat`
```rust
const SYS_fchmodat: c_long = 268i64;
```

### `SYS_faccessat`
```rust
const SYS_faccessat: c_long = 269i64;
```

### `SYS_pselect6`
```rust
const SYS_pselect6: c_long = 270i64;
```

### `SYS_ppoll`
```rust
const SYS_ppoll: c_long = 271i64;
```

### `SYS_unshare`
```rust
const SYS_unshare: c_long = 272i64;
```

### `SYS_set_robust_list`
```rust
const SYS_set_robust_list: c_long = 273i64;
```

### `SYS_get_robust_list`
```rust
const SYS_get_robust_list: c_long = 274i64;
```

### `SYS_splice`
```rust
const SYS_splice: c_long = 275i64;
```

### `SYS_tee`
```rust
const SYS_tee: c_long = 276i64;
```

### `SYS_sync_file_range`
```rust
const SYS_sync_file_range: c_long = 277i64;
```

### `SYS_vmsplice`
```rust
const SYS_vmsplice: c_long = 278i64;
```

### `SYS_move_pages`
```rust
const SYS_move_pages: c_long = 279i64;
```

### `SYS_utimensat`
```rust
const SYS_utimensat: c_long = 280i64;
```

### `SYS_epoll_pwait`
```rust
const SYS_epoll_pwait: c_long = 281i64;
```

### `SYS_signalfd`
```rust
const SYS_signalfd: c_long = 282i64;
```

### `SYS_timerfd_create`
```rust
const SYS_timerfd_create: c_long = 283i64;
```

### `SYS_eventfd`
```rust
const SYS_eventfd: c_long = 284i64;
```

### `SYS_fallocate`
```rust
const SYS_fallocate: c_long = 285i64;
```

### `SYS_timerfd_settime`
```rust
const SYS_timerfd_settime: c_long = 286i64;
```

### `SYS_timerfd_gettime`
```rust
const SYS_timerfd_gettime: c_long = 287i64;
```

### `SYS_accept4`
```rust
const SYS_accept4: c_long = 288i64;
```

### `SYS_signalfd4`
```rust
const SYS_signalfd4: c_long = 289i64;
```

### `SYS_eventfd2`
```rust
const SYS_eventfd2: c_long = 290i64;
```

### `SYS_epoll_create1`
```rust
const SYS_epoll_create1: c_long = 291i64;
```

### `SYS_dup3`
```rust
const SYS_dup3: c_long = 292i64;
```

### `SYS_pipe2`
```rust
const SYS_pipe2: c_long = 293i64;
```

### `SYS_inotify_init1`
```rust
const SYS_inotify_init1: c_long = 294i64;
```

### `SYS_preadv`
```rust
const SYS_preadv: c_long = 295i64;
```

### `SYS_pwritev`
```rust
const SYS_pwritev: c_long = 296i64;
```

### `SYS_rt_tgsigqueueinfo`
```rust
const SYS_rt_tgsigqueueinfo: c_long = 297i64;
```

### `SYS_perf_event_open`
```rust
const SYS_perf_event_open: c_long = 298i64;
```

### `SYS_recvmmsg`
```rust
const SYS_recvmmsg: c_long = 299i64;
```

### `SYS_fanotify_init`
```rust
const SYS_fanotify_init: c_long = 300i64;
```

### `SYS_fanotify_mark`
```rust
const SYS_fanotify_mark: c_long = 301i64;
```

### `SYS_prlimit64`
```rust
const SYS_prlimit64: c_long = 302i64;
```

### `SYS_name_to_handle_at`
```rust
const SYS_name_to_handle_at: c_long = 303i64;
```

### `SYS_open_by_handle_at`
```rust
const SYS_open_by_handle_at: c_long = 304i64;
```

### `SYS_clock_adjtime`
```rust
const SYS_clock_adjtime: c_long = 305i64;
```

### `SYS_syncfs`
```rust
const SYS_syncfs: c_long = 306i64;
```

### `SYS_sendmmsg`
```rust
const SYS_sendmmsg: c_long = 307i64;
```

### `SYS_setns`
```rust
const SYS_setns: c_long = 308i64;
```

### `SYS_getcpu`
```rust
const SYS_getcpu: c_long = 309i64;
```

### `SYS_process_vm_readv`
```rust
const SYS_process_vm_readv: c_long = 310i64;
```

### `SYS_process_vm_writev`
```rust
const SYS_process_vm_writev: c_long = 311i64;
```

### `SYS_kcmp`
```rust
const SYS_kcmp: c_long = 312i64;
```

### `SYS_finit_module`
```rust
const SYS_finit_module: c_long = 313i64;
```

### `SYS_sched_setattr`
```rust
const SYS_sched_setattr: c_long = 314i64;
```

### `SYS_sched_getattr`
```rust
const SYS_sched_getattr: c_long = 315i64;
```

### `SYS_renameat2`
```rust
const SYS_renameat2: c_long = 316i64;
```

### `SYS_seccomp`
```rust
const SYS_seccomp: c_long = 317i64;
```

### `SYS_getrandom`
```rust
const SYS_getrandom: c_long = 318i64;
```

### `SYS_memfd_create`
```rust
const SYS_memfd_create: c_long = 319i64;
```

### `SYS_kexec_file_load`
```rust
const SYS_kexec_file_load: c_long = 320i64;
```

### `SYS_bpf`
```rust
const SYS_bpf: c_long = 321i64;
```

### `SYS_execveat`
```rust
const SYS_execveat: c_long = 322i64;
```

### `SYS_userfaultfd`
```rust
const SYS_userfaultfd: c_long = 323i64;
```

### `SYS_membarrier`
```rust
const SYS_membarrier: c_long = 324i64;
```

### `SYS_mlock2`
```rust
const SYS_mlock2: c_long = 325i64;
```

### `SYS_copy_file_range`
```rust
const SYS_copy_file_range: c_long = 326i64;
```

### `SYS_preadv2`
```rust
const SYS_preadv2: c_long = 327i64;
```

### `SYS_pwritev2`
```rust
const SYS_pwritev2: c_long = 328i64;
```

### `SYS_pkey_mprotect`
```rust
const SYS_pkey_mprotect: c_long = 329i64;
```

### `SYS_pkey_alloc`
```rust
const SYS_pkey_alloc: c_long = 330i64;
```

### `SYS_pkey_free`
```rust
const SYS_pkey_free: c_long = 331i64;
```

### `SYS_statx`
```rust
const SYS_statx: c_long = 332i64;
```

### `SYS_rseq`
```rust
const SYS_rseq: c_long = 334i64;
```

### `SYS_pidfd_send_signal`
```rust
const SYS_pidfd_send_signal: c_long = 424i64;
```

### `SYS_io_uring_setup`
```rust
const SYS_io_uring_setup: c_long = 425i64;
```

### `SYS_io_uring_enter`
```rust
const SYS_io_uring_enter: c_long = 426i64;
```

### `SYS_io_uring_register`
```rust
const SYS_io_uring_register: c_long = 427i64;
```

### `SYS_open_tree`
```rust
const SYS_open_tree: c_long = 428i64;
```

### `SYS_move_mount`
```rust
const SYS_move_mount: c_long = 429i64;
```

### `SYS_fsopen`
```rust
const SYS_fsopen: c_long = 430i64;
```

### `SYS_fsconfig`
```rust
const SYS_fsconfig: c_long = 431i64;
```

### `SYS_fsmount`
```rust
const SYS_fsmount: c_long = 432i64;
```

### `SYS_fspick`
```rust
const SYS_fspick: c_long = 433i64;
```

### `SYS_pidfd_open`
```rust
const SYS_pidfd_open: c_long = 434i64;
```

### `SYS_clone3`
```rust
const SYS_clone3: c_long = 435i64;
```

### `SYS_close_range`
```rust
const SYS_close_range: c_long = 436i64;
```

### `SYS_openat2`
```rust
const SYS_openat2: c_long = 437i64;
```

### `SYS_pidfd_getfd`
```rust
const SYS_pidfd_getfd: c_long = 438i64;
```

### `SYS_faccessat2`
```rust
const SYS_faccessat2: c_long = 439i64;
```

### `SYS_process_madvise`
```rust
const SYS_process_madvise: c_long = 440i64;
```

### `SYS_epoll_pwait2`
```rust
const SYS_epoll_pwait2: c_long = 441i64;
```

### `SYS_mount_setattr`
```rust
const SYS_mount_setattr: c_long = 442i64;
```

### `SYS_quotactl_fd`
```rust
const SYS_quotactl_fd: c_long = 443i64;
```

### `SYS_landlock_create_ruleset`
```rust
const SYS_landlock_create_ruleset: c_long = 444i64;
```

### `SYS_landlock_add_rule`
```rust
const SYS_landlock_add_rule: c_long = 445i64;
```

### `SYS_landlock_restrict_self`
```rust
const SYS_landlock_restrict_self: c_long = 446i64;
```

### `SYS_memfd_secret`
```rust
const SYS_memfd_secret: c_long = 447i64;
```

### `SYS_process_mrelease`
```rust
const SYS_process_mrelease: c_long = 448i64;
```

### `SYS_futex_waitv`
```rust
const SYS_futex_waitv: c_long = 449i64;
```

### `SYS_set_mempolicy_home_node`
```rust
const SYS_set_mempolicy_home_node: c_long = 450i64;
```

### `SYS_fchmodat2`
```rust
const SYS_fchmodat2: c_long = 452i64;
```

### `SYS_mseal`
```rust
const SYS_mseal: c_long = 462i64;
```

