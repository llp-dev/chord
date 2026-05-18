**sel4_linux_syscall_types > arch > x86_64 > syscall_number**

# Module: arch::x86_64::syscall_number

## Contents

**Constants**

- [`ACCEPT`](#accept)
- [`ACCEPT4`](#accept4)
- [`ACCESS`](#access)
- [`ACCT`](#acct)
- [`ADD_KEY`](#add_key)
- [`ADJTIMEX`](#adjtimex)
- [`AFS_SYSCALL`](#afs_syscall)
- [`ALARM`](#alarm)
- [`ARCH_PRCTL`](#arch_prctl)
- [`BIND`](#bind)
- [`BPF`](#bpf)
- [`BRK`](#brk)
- [`CACHESTAT`](#cachestat)
- [`CAPGET`](#capget)
- [`CAPSET`](#capset)
- [`CHDIR`](#chdir)
- [`CHMOD`](#chmod)
- [`CHOWN`](#chown)
- [`CHROOT`](#chroot)
- [`CLOCK_ADJTIME`](#clock_adjtime)
- [`CLOCK_GETRES`](#clock_getres)
- [`CLOCK_GETTIME`](#clock_gettime)
- [`CLOCK_NANOSLEEP`](#clock_nanosleep)
- [`CLOCK_SETTIME`](#clock_settime)
- [`CLONE`](#clone)
- [`CLONE3`](#clone3)
- [`CLOSE`](#close)
- [`CLOSE_RANGE`](#close_range)
- [`CONNECT`](#connect)
- [`COPY_FILE_RANGE`](#copy_file_range)
- [`CREAT`](#creat)
- [`CREATE_MODULE`](#create_module)
- [`DELETE_MODULE`](#delete_module)
- [`DUP`](#dup)
- [`DUP2`](#dup2)
- [`DUP3`](#dup3)
- [`EPOLL_CREATE`](#epoll_create)
- [`EPOLL_CREATE1`](#epoll_create1)
- [`EPOLL_CTL`](#epoll_ctl)
- [`EPOLL_CTL_OLD`](#epoll_ctl_old)
- [`EPOLL_PWAIT`](#epoll_pwait)
- [`EPOLL_PWAIT2`](#epoll_pwait2)
- [`EPOLL_WAIT`](#epoll_wait)
- [`EPOLL_WAIT_OLD`](#epoll_wait_old)
- [`EVENTFD`](#eventfd)
- [`EVENTFD2`](#eventfd2)
- [`EXECVE`](#execve)
- [`EXECVEAT`](#execveat)
- [`EXIT`](#exit)
- [`EXIT_GROUP`](#exit_group)
- [`FACCESSAT`](#faccessat)
- [`FACCESSAT2`](#faccessat2)
- [`FADVISE64`](#fadvise64)
- [`FALLOCATE`](#fallocate)
- [`FANOTIFY_INIT`](#fanotify_init)
- [`FANOTIFY_MARK`](#fanotify_mark)
- [`FCHDIR`](#fchdir)
- [`FCHMOD`](#fchmod)
- [`FCHMODAT`](#fchmodat)
- [`FCHMODAT2`](#fchmodat2)
- [`FCHOWN`](#fchown)
- [`FCHOWNAT`](#fchownat)
- [`FCNTL`](#fcntl)
- [`FDATASYNC`](#fdatasync)
- [`FGETXATTR`](#fgetxattr)
- [`FINIT_MODULE`](#finit_module)
- [`FLISTXATTR`](#flistxattr)
- [`FLOCK`](#flock)
- [`FORK`](#fork)
- [`FREMOVEXATTR`](#fremovexattr)
- [`FSCONFIG`](#fsconfig)
- [`FSETXATTR`](#fsetxattr)
- [`FSMOUNT`](#fsmount)
- [`FSOPEN`](#fsopen)
- [`FSPICK`](#fspick)
- [`FSTAT`](#fstat)
- [`FSTATFS`](#fstatfs)
- [`FSYNC`](#fsync)
- [`FTRUNCATE`](#ftruncate)
- [`FUTEX`](#futex)
- [`FUTEX_WAITV`](#futex_waitv)
- [`FUTIMESAT`](#futimesat)
- [`GETCPU`](#getcpu)
- [`GETCWD`](#getcwd)
- [`GETDENTS`](#getdents)
- [`GETDENTS64`](#getdents64)
- [`GETEGID`](#getegid)
- [`GETEUID`](#geteuid)
- [`GETGID`](#getgid)
- [`GETGROUPS`](#getgroups)
- [`GETITIMER`](#getitimer)
- [`GETPEERNAME`](#getpeername)
- [`GETPGID`](#getpgid)
- [`GETPGRP`](#getpgrp)
- [`GETPID`](#getpid)
- [`GETPMSG`](#getpmsg)
- [`GETPPID`](#getppid)
- [`GETPRIORITY`](#getpriority)
- [`GETRANDOM`](#getrandom)
- [`GETRESGID`](#getresgid)
- [`GETRESUID`](#getresuid)
- [`GETRLIMIT`](#getrlimit)
- [`GETRUSAGE`](#getrusage)
- [`GETSID`](#getsid)
- [`GETSOCKNAME`](#getsockname)
- [`GETSOCKOPT`](#getsockopt)
- [`GETTID`](#gettid)
- [`GETTIMEOFDAY`](#gettimeofday)
- [`GETUID`](#getuid)
- [`GETXATTR`](#getxattr)
- [`GET_KERNEL_SYMS`](#get_kernel_syms)
- [`GET_MEMPOLICY`](#get_mempolicy)
- [`GET_ROBUST_LIST`](#get_robust_list)
- [`GET_THREAD_AREA`](#get_thread_area)
- [`INIT_MODULE`](#init_module)
- [`INOTIFY_ADD_WATCH`](#inotify_add_watch)
- [`INOTIFY_INIT`](#inotify_init)
- [`INOTIFY_INIT1`](#inotify_init1)
- [`INOTIFY_RM_WATCH`](#inotify_rm_watch)
- [`IOCTL`](#ioctl)
- [`IOPERM`](#ioperm)
- [`IOPL`](#iopl)
- [`IOPRIO_GET`](#ioprio_get)
- [`IOPRIO_SET`](#ioprio_set)
- [`IO_CANCEL`](#io_cancel)
- [`IO_DESTROY`](#io_destroy)
- [`IO_GETEVENTS`](#io_getevents)
- [`IO_PGETEVENTS`](#io_pgetevents)
- [`IO_SETUP`](#io_setup)
- [`IO_SUBMIT`](#io_submit)
- [`IO_URING_ENTER`](#io_uring_enter)
- [`IO_URING_REGISTER`](#io_uring_register)
- [`IO_URING_SETUP`](#io_uring_setup)
- [`KCMP`](#kcmp)
- [`KEXEC_FILE_LOAD`](#kexec_file_load)
- [`KEXEC_LOAD`](#kexec_load)
- [`KEYCTL`](#keyctl)
- [`KILL`](#kill)
- [`LANDLOCK_ADD_RULE`](#landlock_add_rule)
- [`LANDLOCK_CREATE_RULESET`](#landlock_create_ruleset)
- [`LANDLOCK_RESTRICT_SELF`](#landlock_restrict_self)
- [`LCHOWN`](#lchown)
- [`LGETXATTR`](#lgetxattr)
- [`LINK`](#link)
- [`LINKAT`](#linkat)
- [`LISTEN`](#listen)
- [`LISTXATTR`](#listxattr)
- [`LLISTXATTR`](#llistxattr)
- [`LOOKUP_DCOOKIE`](#lookup_dcookie)
- [`LREMOVEXATTR`](#lremovexattr)
- [`LSEEK`](#lseek)
- [`LSETXATTR`](#lsetxattr)
- [`LSTAT`](#lstat)
- [`MADVISE`](#madvise)
- [`MBIND`](#mbind)
- [`MEMBARRIER`](#membarrier)
- [`MEMFD_CREATE`](#memfd_create)
- [`MEMFD_SECRET`](#memfd_secret)
- [`MIGRATE_PAGES`](#migrate_pages)
- [`MINCORE`](#mincore)
- [`MKDIR`](#mkdir)
- [`MKDIRAT`](#mkdirat)
- [`MKNOD`](#mknod)
- [`MKNODAT`](#mknodat)
- [`MLOCK`](#mlock)
- [`MLOCK2`](#mlock2)
- [`MLOCKALL`](#mlockall)
- [`MMAP`](#mmap)
- [`MODIFY_LDT`](#modify_ldt)
- [`MOUNT`](#mount)
- [`MOUNT_SETATTR`](#mount_setattr)
- [`MOVE_MOUNT`](#move_mount)
- [`MOVE_PAGES`](#move_pages)
- [`MPROTECT`](#mprotect)
- [`MQ_GETSETATTR`](#mq_getsetattr)
- [`MQ_NOTIFY`](#mq_notify)
- [`MQ_OPEN`](#mq_open)
- [`MQ_TIMEDRECEIVE`](#mq_timedreceive)
- [`MQ_TIMEDSEND`](#mq_timedsend)
- [`MQ_UNLINK`](#mq_unlink)
- [`MREMAP`](#mremap)
- [`MSGCTL`](#msgctl)
- [`MSGGET`](#msgget)
- [`MSGRCV`](#msgrcv)
- [`MSGSND`](#msgsnd)
- [`MSYNC`](#msync)
- [`MUNLOCK`](#munlock)
- [`MUNLOCKALL`](#munlockall)
- [`MUNMAP`](#munmap)
- [`NAME_TO_HANDLE_AT`](#name_to_handle_at)
- [`NANOSLEEP`](#nanosleep)
- [`NEWFSTATAT`](#newfstatat)
- [`NFSSERVCTL`](#nfsservctl)
- [`OPEN`](#open)
- [`OPENAT`](#openat)
- [`OPENAT2`](#openat2)
- [`OPEN_BY_HANDLE_AT`](#open_by_handle_at)
- [`OPEN_TREE`](#open_tree)
- [`PAUSE`](#pause)
- [`PERF_EVENT_OPEN`](#perf_event_open)
- [`PERSONALITY`](#personality)
- [`PIDFD_GETFD`](#pidfd_getfd)
- [`PIDFD_OPEN`](#pidfd_open)
- [`PIDFD_SEND_SIGNAL`](#pidfd_send_signal)
- [`PIPE`](#pipe)
- [`PIPE2`](#pipe2)
- [`PIVOT_ROOT`](#pivot_root)
- [`PKEY_ALLOC`](#pkey_alloc)
- [`PKEY_FREE`](#pkey_free)
- [`PKEY_MPROTECT`](#pkey_mprotect)
- [`POLL`](#poll)
- [`PPOLL`](#ppoll)
- [`PRCTL`](#prctl)
- [`PREAD64`](#pread64)
- [`PREADV`](#preadv)
- [`PREADV2`](#preadv2)
- [`PRLIMIT64`](#prlimit64)
- [`PROCESS_MADVISE`](#process_madvise)
- [`PROCESS_MRELEASE`](#process_mrelease)
- [`PROCESS_VM_READV`](#process_vm_readv)
- [`PROCESS_VM_WRITEV`](#process_vm_writev)
- [`PSELECT6`](#pselect6)
- [`PTRACE`](#ptrace)
- [`PUTPMSG`](#putpmsg)
- [`PWRITE64`](#pwrite64)
- [`PWRITEV`](#pwritev)
- [`PWRITEV2`](#pwritev2)
- [`QUERY_MODULE`](#query_module)
- [`QUOTACTL`](#quotactl)
- [`READ`](#read)
- [`READAHEAD`](#readahead)
- [`READLINK`](#readlink)
- [`READLINKAT`](#readlinkat)
- [`READV`](#readv)
- [`REBOOT`](#reboot)
- [`RECVFROM`](#recvfrom)
- [`RECVMMSG`](#recvmmsg)
- [`RECVMSG`](#recvmsg)
- [`REMAP_FILE_PAGES`](#remap_file_pages)
- [`REMOVEXATTR`](#removexattr)
- [`RENAME`](#rename)
- [`RENAMEAT`](#renameat)
- [`RENAMEAT2`](#renameat2)
- [`REQUEST_KEY`](#request_key)
- [`RESTART_SYSCALL`](#restart_syscall)
- [`RMDIR`](#rmdir)
- [`RSEQ`](#rseq)
- [`RT_SIGACTION`](#rt_sigaction)
- [`RT_SIGPENDING`](#rt_sigpending)
- [`RT_SIGPROCMASK`](#rt_sigprocmask)
- [`RT_SIGQUEUEINFO`](#rt_sigqueueinfo)
- [`RT_SIGRETURN`](#rt_sigreturn)
- [`RT_SIGSUSPEND`](#rt_sigsuspend)
- [`RT_SIGTIMEDWAIT`](#rt_sigtimedwait)
- [`RT_TGSIGQUEUEINFO`](#rt_tgsigqueueinfo)
- [`SCHED_GETAFFINITY`](#sched_getaffinity)
- [`SCHED_GETATTR`](#sched_getattr)
- [`SCHED_GETPARAM`](#sched_getparam)
- [`SCHED_GETSCHEDULER`](#sched_getscheduler)
- [`SCHED_GET_PRIORITY_MAX`](#sched_get_priority_max)
- [`SCHED_GET_PRIORITY_MIN`](#sched_get_priority_min)
- [`SCHED_RR_GET_INTERVAL`](#sched_rr_get_interval)
- [`SCHED_SETAFFINITY`](#sched_setaffinity)
- [`SCHED_SETATTR`](#sched_setattr)
- [`SCHED_SETPARAM`](#sched_setparam)
- [`SCHED_SETSCHEDULER`](#sched_setscheduler)
- [`SCHED_YIELD`](#sched_yield)
- [`SECCOMP`](#seccomp)
- [`SECURITY`](#security)
- [`SELECT`](#select)
- [`SEMCTL`](#semctl)
- [`SEMGET`](#semget)
- [`SEMOP`](#semop)
- [`SEMTIMEDOP`](#semtimedop)
- [`SENDFILE`](#sendfile)
- [`SENDMMSG`](#sendmmsg)
- [`SENDMSG`](#sendmsg)
- [`SENDTO`](#sendto)
- [`SETDOMAINNAME`](#setdomainname)
- [`SETFSGID`](#setfsgid)
- [`SETFSUID`](#setfsuid)
- [`SETGID`](#setgid)
- [`SETGROUPS`](#setgroups)
- [`SETHOSTNAME`](#sethostname)
- [`SETITIMER`](#setitimer)
- [`SETNS`](#setns)
- [`SETPGID`](#setpgid)
- [`SETPRIORITY`](#setpriority)
- [`SETREGID`](#setregid)
- [`SETRESGID`](#setresgid)
- [`SETRESUID`](#setresuid)
- [`SETREUID`](#setreuid)
- [`SETRLIMIT`](#setrlimit)
- [`SETSID`](#setsid)
- [`SETSOCKOPT`](#setsockopt)
- [`SETTIMEOFDAY`](#settimeofday)
- [`SETUID`](#setuid)
- [`SETXATTR`](#setxattr)
- [`SET_MEMPOLICY`](#set_mempolicy)
- [`SET_MEMPOLICY_HOME_NODE`](#set_mempolicy_home_node)
- [`SET_ROBUST_LIST`](#set_robust_list)
- [`SET_THREAD_AREA`](#set_thread_area)
- [`SET_TID_ADDRESS`](#set_tid_address)
- [`SHMAT`](#shmat)
- [`SHMCTL`](#shmctl)
- [`SHMDT`](#shmdt)
- [`SHMGET`](#shmget)
- [`SHUTDOWN`](#shutdown)
- [`SIGALTSTACK`](#sigaltstack)
- [`SIGNALFD`](#signalfd)
- [`SIGNALFD4`](#signalfd4)
- [`SOCKET`](#socket)
- [`SOCKETPAIR`](#socketpair)
- [`SPLICE`](#splice)
- [`STAT`](#stat)
- [`STATFS`](#statfs)
- [`STATX`](#statx)
- [`SWAPOFF`](#swapoff)
- [`SWAPON`](#swapon)
- [`SYMLINK`](#symlink)
- [`SYMLINKAT`](#symlinkat)
- [`SYNC`](#sync)
- [`SYNCFS`](#syncfs)
- [`SYNC_FILE_RANGE`](#sync_file_range)
- [`SYSFS`](#sysfs)
- [`SYSINFO`](#sysinfo)
- [`SYSLOG`](#syslog)
- [`TEE`](#tee)
- [`TGKILL`](#tgkill)
- [`TIME`](#time)
- [`TIMERFD_CREATE`](#timerfd_create)
- [`TIMERFD_GETTIME`](#timerfd_gettime)
- [`TIMERFD_SETTIME`](#timerfd_settime)
- [`TIMER_CREATE`](#timer_create)
- [`TIMER_DELETE`](#timer_delete)
- [`TIMER_GETOVERRUN`](#timer_getoverrun)
- [`TIMER_GETTIME`](#timer_gettime)
- [`TIMER_SETTIME`](#timer_settime)
- [`TIMES`](#times)
- [`TKILL`](#tkill)
- [`TRUNCATE`](#truncate)
- [`TUXCALL`](#tuxcall)
- [`UMASK`](#umask)
- [`UMOUNT2`](#umount2)
- [`UNAME`](#uname)
- [`UNLINK`](#unlink)
- [`UNLINKAT`](#unlinkat)
- [`UNSHARE`](#unshare)
- [`USELIB`](#uselib)
- [`USERFAULTFD`](#userfaultfd)
- [`USTAT`](#ustat)
- [`UTIME`](#utime)
- [`UTIMENSAT`](#utimensat)
- [`UTIMES`](#utimes)
- [`VFORK`](#vfork)
- [`VHANGUP`](#vhangup)
- [`VMSPLICE`](#vmsplice)
- [`VSERVER`](#vserver)
- [`WAIT4`](#wait4)
- [`WAITID`](#waitid)
- [`WRITE`](#write)
- [`WRITEV`](#writev)
- [`_SYSCTL`](#_sysctl)

---

## sel4_linux_syscall_types::arch::x86_64::syscall_number::ACCEPT

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::ACCEPT4

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::ACCESS

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::ACCT

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::ADD_KEY

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::ADJTIMEX

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::AFS_SYSCALL

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::ALARM

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::ARCH_PRCTL

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::BIND

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::BPF

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::BRK

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::CACHESTAT

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::CAPGET

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::CAPSET

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::CHDIR

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::CHMOD

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::CHOWN

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::CHROOT

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::CLOCK_ADJTIME

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::CLOCK_GETRES

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::CLOCK_GETTIME

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::CLOCK_NANOSLEEP

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::CLOCK_SETTIME

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::CLONE

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::CLONE3

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::CLOSE

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::CLOSE_RANGE

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::CONNECT

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::COPY_FILE_RANGE

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::CREAT

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::CREATE_MODULE

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::DELETE_MODULE

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::DUP

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::DUP2

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::DUP3

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::EPOLL_CREATE

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::EPOLL_CREATE1

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::EPOLL_CTL

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::EPOLL_CTL_OLD

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::EPOLL_PWAIT

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::EPOLL_PWAIT2

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::EPOLL_WAIT

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::EPOLL_WAIT_OLD

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::EVENTFD

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::EVENTFD2

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::EXECVE

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::EXECVEAT

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::EXIT

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::EXIT_GROUP

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::FACCESSAT

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::FACCESSAT2

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::FADVISE64

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::FALLOCATE

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::FANOTIFY_INIT

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::FANOTIFY_MARK

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::FCHDIR

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::FCHMOD

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::FCHMODAT

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::FCHMODAT2

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::FCHOWN

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::FCHOWNAT

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::FCNTL

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::FDATASYNC

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::FGETXATTR

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::FINIT_MODULE

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::FLISTXATTR

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::FLOCK

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::FORK

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::FREMOVEXATTR

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::FSCONFIG

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::FSETXATTR

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::FSMOUNT

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::FSOPEN

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::FSPICK

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::FSTAT

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::FSTATFS

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::FSYNC

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::FTRUNCATE

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::FUTEX

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::FUTEX_WAITV

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::FUTIMESAT

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::GETCPU

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::GETCWD

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::GETDENTS

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::GETDENTS64

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::GETEGID

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::GETEUID

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::GETGID

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::GETGROUPS

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::GETITIMER

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::GETPEERNAME

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::GETPGID

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::GETPGRP

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::GETPID

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::GETPMSG

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::GETPPID

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::GETPRIORITY

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::GETRANDOM

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::GETRESGID

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::GETRESUID

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::GETRLIMIT

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::GETRUSAGE

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::GETSID

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::GETSOCKNAME

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::GETSOCKOPT

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::GETTID

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::GETTIMEOFDAY

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::GETUID

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::GETXATTR

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::GET_KERNEL_SYMS

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::GET_MEMPOLICY

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::GET_ROBUST_LIST

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::GET_THREAD_AREA

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::INIT_MODULE

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::INOTIFY_ADD_WATCH

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::INOTIFY_INIT

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::INOTIFY_INIT1

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::INOTIFY_RM_WATCH

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::IOCTL

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::IOPERM

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::IOPL

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::IOPRIO_GET

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::IOPRIO_SET

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::IO_CANCEL

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::IO_DESTROY

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::IO_GETEVENTS

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::IO_PGETEVENTS

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::IO_SETUP

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::IO_SUBMIT

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::IO_URING_ENTER

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::IO_URING_REGISTER

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::IO_URING_SETUP

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::KCMP

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::KEXEC_FILE_LOAD

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::KEXEC_LOAD

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::KEYCTL

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::KILL

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::LANDLOCK_ADD_RULE

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::LANDLOCK_CREATE_RULESET

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::LANDLOCK_RESTRICT_SELF

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::LCHOWN

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::LGETXATTR

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::LINK

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::LINKAT

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::LISTEN

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::LISTXATTR

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::LLISTXATTR

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::LOOKUP_DCOOKIE

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::LREMOVEXATTR

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::LSEEK

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::LSETXATTR

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::LSTAT

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::MADVISE

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::MBIND

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::MEMBARRIER

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::MEMFD_CREATE

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::MEMFD_SECRET

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::MIGRATE_PAGES

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::MINCORE

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::MKDIR

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::MKDIRAT

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::MKNOD

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::MKNODAT

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::MLOCK

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::MLOCK2

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::MLOCKALL

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::MMAP

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::MODIFY_LDT

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::MOUNT

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::MOUNT_SETATTR

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::MOVE_MOUNT

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::MOVE_PAGES

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::MPROTECT

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::MQ_GETSETATTR

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::MQ_NOTIFY

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::MQ_OPEN

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::MQ_TIMEDRECEIVE

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::MQ_TIMEDSEND

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::MQ_UNLINK

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::MREMAP

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::MSGCTL

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::MSGGET

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::MSGRCV

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::MSGSND

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::MSYNC

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::MUNLOCK

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::MUNLOCKALL

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::MUNMAP

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::NAME_TO_HANDLE_AT

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::NANOSLEEP

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::NEWFSTATAT

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::NFSSERVCTL

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::OPEN

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::OPENAT

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::OPENAT2

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::OPEN_BY_HANDLE_AT

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::OPEN_TREE

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::PAUSE

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::PERF_EVENT_OPEN

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::PERSONALITY

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::PIDFD_GETFD

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::PIDFD_OPEN

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::PIDFD_SEND_SIGNAL

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::PIPE

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::PIPE2

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::PIVOT_ROOT

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::PKEY_ALLOC

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::PKEY_FREE

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::PKEY_MPROTECT

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::POLL

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::PPOLL

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::PRCTL

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::PREAD64

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::PREADV

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::PREADV2

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::PRLIMIT64

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::PROCESS_MADVISE

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::PROCESS_MRELEASE

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::PROCESS_VM_READV

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::PROCESS_VM_WRITEV

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::PSELECT6

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::PTRACE

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::PUTPMSG

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::PWRITE64

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::PWRITEV

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::PWRITEV2

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::QUERY_MODULE

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::QUOTACTL

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::READ

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::READAHEAD

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::READLINK

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::READLINKAT

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::READV

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::REBOOT

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::RECVFROM

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::RECVMMSG

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::RECVMSG

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::REMAP_FILE_PAGES

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::REMOVEXATTR

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::RENAME

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::RENAMEAT

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::RENAMEAT2

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::REQUEST_KEY

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::RESTART_SYSCALL

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::RMDIR

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::RSEQ

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::RT_SIGACTION

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::RT_SIGPENDING

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::RT_SIGPROCMASK

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::RT_SIGQUEUEINFO

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::RT_SIGRETURN

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::RT_SIGSUSPEND

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::RT_SIGTIMEDWAIT

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::RT_TGSIGQUEUEINFO

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SCHED_GETAFFINITY

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SCHED_GETATTR

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SCHED_GETPARAM

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SCHED_GETSCHEDULER

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SCHED_GET_PRIORITY_MAX

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SCHED_GET_PRIORITY_MIN

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SCHED_RR_GET_INTERVAL

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SCHED_SETAFFINITY

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SCHED_SETATTR

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SCHED_SETPARAM

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SCHED_SETSCHEDULER

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SCHED_YIELD

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SECCOMP

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SECURITY

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SELECT

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SEMCTL

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SEMGET

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SEMOP

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SEMTIMEDOP

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SENDFILE

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SENDMMSG

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SENDMSG

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SENDTO

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SETDOMAINNAME

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SETFSGID

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SETFSUID

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SETGID

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SETGROUPS

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SETHOSTNAME

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SETITIMER

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SETNS

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SETPGID

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SETPRIORITY

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SETREGID

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SETRESGID

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SETRESUID

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SETREUID

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SETRLIMIT

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SETSID

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SETSOCKOPT

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SETTIMEOFDAY

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SETUID

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SETXATTR

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SET_MEMPOLICY

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SET_MEMPOLICY_HOME_NODE

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SET_ROBUST_LIST

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SET_THREAD_AREA

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SET_TID_ADDRESS

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SHMAT

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SHMCTL

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SHMDT

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SHMGET

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SHUTDOWN

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SIGALTSTACK

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SIGNALFD

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SIGNALFD4

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SOCKET

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SOCKETPAIR

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SPLICE

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::STAT

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::STATFS

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::STATX

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SWAPOFF

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SWAPON

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SYMLINK

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SYMLINKAT

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SYNC

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SYNCFS

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SYNC_FILE_RANGE

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SYSFS

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SYSINFO

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::SYSLOG

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::TEE

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::TGKILL

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::TIME

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::TIMERFD_CREATE

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::TIMERFD_GETTIME

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::TIMERFD_SETTIME

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::TIMER_CREATE

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::TIMER_DELETE

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::TIMER_GETOVERRUN

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::TIMER_GETTIME

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::TIMER_SETTIME

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::TIMES

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::TKILL

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::TRUNCATE

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::TUXCALL

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::UMASK

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::UMOUNT2

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::UNAME

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::UNLINK

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::UNLINKAT

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::UNSHARE

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::USELIB

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::USERFAULTFD

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::USTAT

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::UTIME

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::UTIMENSAT

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::UTIMES

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::VFORK

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::VHANGUP

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::VMSPLICE

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::VSERVER

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::WAIT4

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::WAITID

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::WRITE

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::WRITEV

*Constant*: `crate::SyscallNumber`



## sel4_linux_syscall_types::arch::x86_64::syscall_number::_SYSCTL

*Constant*: `crate::SyscallNumber`



