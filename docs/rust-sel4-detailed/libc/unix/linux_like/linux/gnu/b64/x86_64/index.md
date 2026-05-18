*[libc](../../../../../../index.md) / [unix](../../../../../index.md) / [linux_like](../../../../index.md) / [linux](../../../index.md) / [gnu](../../index.md) / [b64](../index.md) / [x86_64](index.md)*

---

# Module `x86_64`

x86_64-specific definitions for 64-bit linux-like values

## Contents

- [Modules](#modules)
  - [`not_x32`](#not-x32)
- [Structs](#structs)
  - [`sigaction`](#sigaction)
  - [`statfs`](#statfs)
  - [`flock`](#flock)
  - [`flock64`](#flock64)
  - [`siginfo_t`](#siginfo-t)
  - [`stack_t`](#stack-t)
  - [`stat`](#stat)
  - [`stat64`](#stat64)
  - [`statfs64`](#statfs64)
  - [`statvfs64`](#statvfs64)
  - [`pthread_attr_t`](#pthread-attr-t)
  - [`_libc_fpxreg`](#libc-fpxreg)
  - [`_libc_xmmreg`](#libc-xmmreg)
  - [`_libc_fpstate`](#libc-fpstate)
  - [`user_regs_struct`](#user-regs-struct)
  - [`user`](#user)
  - [`mcontext_t`](#mcontext-t)
  - [`ipc_perm`](#ipc-perm)
  - [`shmid_ds`](#shmid-ds)
  - [`ptrace_rseq_configuration`](#ptrace-rseq-configuration)
  - [`clone_args`](#clone-args)
  - [`user_fpregs_struct`](#user-fpregs-struct)
  - [`ucontext_t`](#ucontext-t)
  - [`max_align_t`](#max-align-t)
  - [`statvfs`](#statvfs)
- [Functions](#functions)
  - [`getcontext`](#getcontext)
  - [`setcontext`](#setcontext)
  - [`makecontext`](#makecontext)
  - [`swapcontext`](#swapcontext)
  - [`sysctl`](#sysctl)
- [Type Aliases](#type-aliases)
  - [`wchar_t`](#wchar-t)
  - [`nlink_t`](#nlink-t)
  - [`blksize_t`](#blksize-t)
  - [`greg_t`](#greg-t)
  - [`suseconds_t`](#suseconds-t)
  - [`__u64`](#u64)
  - [`__s64`](#s64)
- [Constants](#constants)
  - [`POSIX_FADV_DONTNEED`](#posix-fadv-dontneed)
  - [`POSIX_FADV_NOREUSE`](#posix-fadv-noreuse)
  - [`VEOF`](#veof)
  - [`RTLD_DEEPBIND`](#rtld-deepbind)
  - [`RTLD_GLOBAL`](#rtld-global)
  - [`RTLD_NOLOAD`](#rtld-noload)
  - [`O_APPEND`](#o-append)
  - [`O_CREAT`](#o-creat)
  - [`O_EXCL`](#o-excl)
  - [`O_NOCTTY`](#o-noctty)
  - [`O_NONBLOCK`](#o-nonblock)
  - [`O_SYNC`](#o-sync)
  - [`O_RSYNC`](#o-rsync)
  - [`O_DSYNC`](#o-dsync)
  - [`O_FSYNC`](#o-fsync)
  - [`O_NOATIME`](#o-noatime)
  - [`O_PATH`](#o-path)
  - [`O_TMPFILE`](#o-tmpfile)
  - [`MADV_SOFT_OFFLINE`](#madv-soft-offline)
  - [`MAP_GROWSDOWN`](#map-growsdown)
  - [`EDEADLK`](#edeadlk)
  - [`ENAMETOOLONG`](#enametoolong)
  - [`ENOLCK`](#enolck)
  - [`ENOSYS`](#enosys)
  - [`ENOTEMPTY`](#enotempty)
  - [`ELOOP`](#eloop)
  - [`ENOMSG`](#enomsg)
  - [`EIDRM`](#eidrm)
  - [`ECHRNG`](#echrng)
  - [`EL2NSYNC`](#el2nsync)
  - [`EL3HLT`](#el3hlt)
  - [`EL3RST`](#el3rst)
  - [`ELNRNG`](#elnrng)
  - [`EUNATCH`](#eunatch)
  - [`ENOCSI`](#enocsi)
  - [`EL2HLT`](#el2hlt)
  - [`EBADE`](#ebade)
  - [`EBADR`](#ebadr)
  - [`EXFULL`](#exfull)
  - [`ENOANO`](#enoano)
  - [`EBADRQC`](#ebadrqc)
  - [`EBADSLT`](#ebadslt)
  - [`EMULTIHOP`](#emultihop)
  - [`EOVERFLOW`](#eoverflow)
  - [`ENOTUNIQ`](#enotuniq)
  - [`EBADFD`](#ebadfd)
  - [`EBADMSG`](#ebadmsg)
  - [`EREMCHG`](#eremchg)
  - [`ELIBACC`](#elibacc)
  - [`ELIBBAD`](#elibbad)
  - [`ELIBSCN`](#elibscn)
  - [`ELIBMAX`](#elibmax)
  - [`ELIBEXEC`](#elibexec)
  - [`EILSEQ`](#eilseq)
  - [`ERESTART`](#erestart)
  - [`ESTRPIPE`](#estrpipe)
  - [`EUSERS`](#eusers)
  - [`ENOTSOCK`](#enotsock)
  - [`EDESTADDRREQ`](#edestaddrreq)
  - [`EMSGSIZE`](#emsgsize)
  - [`EPROTOTYPE`](#eprototype)
  - [`ENOPROTOOPT`](#enoprotoopt)
  - [`EPROTONOSUPPORT`](#eprotonosupport)
  - [`ESOCKTNOSUPPORT`](#esocktnosupport)
  - [`EOPNOTSUPP`](#eopnotsupp)
  - [`EPFNOSUPPORT`](#epfnosupport)
  - [`EAFNOSUPPORT`](#eafnosupport)
  - [`EADDRINUSE`](#eaddrinuse)
  - [`EADDRNOTAVAIL`](#eaddrnotavail)
  - [`ENETDOWN`](#enetdown)
  - [`ENETUNREACH`](#enetunreach)
  - [`ENETRESET`](#enetreset)
  - [`ECONNABORTED`](#econnaborted)
  - [`ECONNRESET`](#econnreset)
  - [`ENOBUFS`](#enobufs)
  - [`EISCONN`](#eisconn)
  - [`ENOTCONN`](#enotconn)
  - [`ESHUTDOWN`](#eshutdown)
  - [`ETOOMANYREFS`](#etoomanyrefs)
  - [`ETIMEDOUT`](#etimedout)
  - [`ECONNREFUSED`](#econnrefused)
  - [`EHOSTDOWN`](#ehostdown)
  - [`EHOSTUNREACH`](#ehostunreach)
  - [`EALREADY`](#ealready)
  - [`EINPROGRESS`](#einprogress)
  - [`ESTALE`](#estale)
  - [`EDQUOT`](#edquot)
  - [`ENOMEDIUM`](#enomedium)
  - [`EMEDIUMTYPE`](#emediumtype)
  - [`ECANCELED`](#ecanceled)
  - [`ENOKEY`](#enokey)
  - [`EKEYEXPIRED`](#ekeyexpired)
  - [`EKEYREVOKED`](#ekeyrevoked)
  - [`EKEYREJECTED`](#ekeyrejected)
  - [`EOWNERDEAD`](#eownerdead)
  - [`ENOTRECOVERABLE`](#enotrecoverable)
  - [`EHWPOISON`](#ehwpoison)
  - [`ERFKILL`](#erfkill)
  - [`SOCK_STREAM`](#sock-stream)
  - [`SOCK_DGRAM`](#sock-dgram)
  - [`SA_ONSTACK`](#sa-onstack)
  - [`SA_SIGINFO`](#sa-siginfo)
  - [`SA_NOCLDWAIT`](#sa-nocldwait)
  - [`SIGTTIN`](#sigttin)
  - [`SIGTTOU`](#sigttou)
  - [`SIGXCPU`](#sigxcpu)
  - [`SIGXFSZ`](#sigxfsz)
  - [`SIGVTALRM`](#sigvtalrm)
  - [`SIGPROF`](#sigprof)
  - [`SIGWINCH`](#sigwinch)
  - [`SIGCHLD`](#sigchld)
  - [`SIGBUS`](#sigbus)
  - [`SIGUSR1`](#sigusr1)
  - [`SIGUSR2`](#sigusr2)
  - [`SIGCONT`](#sigcont)
  - [`SIGSTOP`](#sigstop)
  - [`SIGTSTP`](#sigtstp)
  - [`SIGURG`](#sigurg)
  - [`SIGIO`](#sigio)
  - [`SIGSYS`](#sigsys)
  - [`SIGSTKFLT`](#sigstkflt)
  - [`SIGUNUSED`](#sigunused)
  - [`SIGPOLL`](#sigpoll)
  - [`SIGPWR`](#sigpwr)
  - [`SIG_SETMASK`](#sig-setmask)
  - [`SIG_BLOCK`](#sig-block)
  - [`SIG_UNBLOCK`](#sig-unblock)
  - [`POLLWRNORM`](#pollwrnorm)
  - [`POLLWRBAND`](#pollwrband)
  - [`O_ASYNC`](#o-async)
  - [`O_NDELAY`](#o-ndelay)
  - [`PTRACE_DETACH`](#ptrace-detach)
  - [`PTRACE_GET_RSEQ_CONFIGURATION`](#ptrace-get-rseq-configuration)
  - [`EFD_NONBLOCK`](#efd-nonblock)
  - [`F_GETLK`](#f-getlk)
  - [`F_GETOWN`](#f-getown)
  - [`F_SETOWN`](#f-setown)
  - [`F_SETLK`](#f-setlk)
  - [`F_SETLKW`](#f-setlkw)
  - [`F_OFD_GETLK`](#f-ofd-getlk)
  - [`F_OFD_SETLK`](#f-ofd-setlk)
  - [`F_OFD_SETLKW`](#f-ofd-setlkw)
  - [`F_RDLCK`](#f-rdlck)
  - [`F_WRLCK`](#f-wrlck)
  - [`F_UNLCK`](#f-unlck)
  - [`SFD_NONBLOCK`](#sfd-nonblock)
  - [`TCSANOW`](#tcsanow)
  - [`TCSADRAIN`](#tcsadrain)
  - [`TCSAFLUSH`](#tcsaflush)
  - [`SFD_CLOEXEC`](#sfd-cloexec)
  - [`NCCS`](#nccs)
  - [`O_TRUNC`](#o-trunc)
  - [`O_CLOEXEC`](#o-cloexec)
  - [`EBFONT`](#ebfont)
  - [`ENOSTR`](#enostr)
  - [`ENODATA`](#enodata)
  - [`ETIME`](#etime)
  - [`ENOSR`](#enosr)
  - [`ENONET`](#enonet)
  - [`ENOPKG`](#enopkg)
  - [`EREMOTE`](#eremote)
  - [`ENOLINK`](#enolink)
  - [`EADV`](#eadv)
  - [`ESRMNT`](#esrmnt)
  - [`ECOMM`](#ecomm)
  - [`EPROTO`](#eproto)
  - [`EDOTDOT`](#edotdot)
  - [`SA_NODEFER`](#sa-nodefer)
  - [`SA_RESETHAND`](#sa-resethand)
  - [`SA_RESTART`](#sa-restart)
  - [`SA_NOCLDSTOP`](#sa-nocldstop)
  - [`EPOLL_CLOEXEC`](#epoll-cloexec)
  - [`EFD_CLOEXEC`](#efd-cloexec)
  - [`__SIZEOF_PTHREAD_CONDATTR_T`](#sizeof-pthread-condattr-t)
  - [`__SIZEOF_PTHREAD_MUTEXATTR_T`](#sizeof-pthread-mutexattr-t)
  - [`__SIZEOF_PTHREAD_BARRIERATTR_T`](#sizeof-pthread-barrierattr-t)
  - [`O_DIRECT`](#o-direct)
  - [`O_DIRECTORY`](#o-directory)
  - [`O_NOFOLLOW`](#o-nofollow)
  - [`MAP_HUGETLB`](#map-hugetlb)
  - [`MAP_LOCKED`](#map-locked)
  - [`MAP_NORESERVE`](#map-noreserve)
  - [`MAP_32BIT`](#map-32bit)
  - [`MAP_ANON`](#map-anon)
  - [`MAP_ANONYMOUS`](#map-anonymous)
  - [`MAP_DENYWRITE`](#map-denywrite)
  - [`MAP_EXECUTABLE`](#map-executable)
  - [`MAP_POPULATE`](#map-populate)
  - [`MAP_NONBLOCK`](#map-nonblock)
  - [`MAP_STACK`](#map-stack)
  - [`MAP_SYNC`](#map-sync)
  - [`EDEADLOCK`](#edeadlock)
  - [`EUCLEAN`](#euclean)
  - [`ENOTNAM`](#enotnam)
  - [`ENAVAIL`](#enavail)
  - [`EISNAM`](#eisnam)
  - [`EREMOTEIO`](#eremoteio)
  - [`PTRACE_GETFPREGS`](#ptrace-getfpregs)
  - [`PTRACE_SETFPREGS`](#ptrace-setfpregs)
  - [`PTRACE_GETFPXREGS`](#ptrace-getfpxregs)
  - [`PTRACE_SETFPXREGS`](#ptrace-setfpxregs)
  - [`PTRACE_GETREGS`](#ptrace-getregs)
  - [`PTRACE_SETREGS`](#ptrace-setregs)
  - [`PTRACE_PEEKSIGINFO_SHARED`](#ptrace-peeksiginfo-shared)
  - [`PTRACE_SYSEMU`](#ptrace-sysemu)
  - [`PTRACE_SYSEMU_SINGLESTEP`](#ptrace-sysemu-singlestep)
  - [`PR_GET_SPECULATION_CTRL`](#pr-get-speculation-ctrl)
  - [`PR_SET_SPECULATION_CTRL`](#pr-set-speculation-ctrl)
  - [`PR_SPEC_NOT_AFFECTED`](#pr-spec-not-affected)
  - [`PR_SPEC_PRCTL`](#pr-spec-prctl)
  - [`PR_SPEC_ENABLE`](#pr-spec-enable)
  - [`PR_SPEC_DISABLE`](#pr-spec-disable)
  - [`PR_SPEC_FORCE_DISABLE`](#pr-spec-force-disable)
  - [`PR_SPEC_DISABLE_NOEXEC`](#pr-spec-disable-noexec)
  - [`PR_SPEC_STORE_BYPASS`](#pr-spec-store-bypass)
  - [`PR_SPEC_INDIRECT_BRANCH`](#pr-spec-indirect-branch)
  - [`MCL_CURRENT`](#mcl-current)
  - [`MCL_FUTURE`](#mcl-future)
  - [`MCL_ONFAULT`](#mcl-onfault)
  - [`SIGSTKSZ`](#sigstksz)
  - [`MINSIGSTKSZ`](#minsigstksz)
  - [`CBAUD`](#cbaud)
  - [`TAB1`](#tab1)
  - [`TAB2`](#tab2)
  - [`TAB3`](#tab3)
  - [`CR1`](#cr1)
  - [`CR2`](#cr2)
  - [`CR3`](#cr3)
  - [`FF1`](#ff1)
  - [`BS1`](#bs1)
  - [`VT1`](#vt1)
  - [`VWERASE`](#vwerase)
  - [`VREPRINT`](#vreprint)
  - [`VSUSP`](#vsusp)
  - [`VSTART`](#vstart)
  - [`VSTOP`](#vstop)
  - [`VDISCARD`](#vdiscard)
  - [`VTIME`](#vtime)
  - [`IXON`](#ixon)
  - [`IXOFF`](#ixoff)
  - [`ONLCR`](#onlcr)
  - [`CSIZE`](#csize)
  - [`CS6`](#cs6)
  - [`CS7`](#cs7)
  - [`CS8`](#cs8)
  - [`CSTOPB`](#cstopb)
  - [`CREAD`](#cread)
  - [`PARENB`](#parenb)
  - [`PARODD`](#parodd)
  - [`HUPCL`](#hupcl)
  - [`CLOCAL`](#clocal)
  - [`ECHOKE`](#echoke)
  - [`ECHOE`](#echoe)
  - [`ECHOK`](#echok)
  - [`ECHONL`](#echonl)
  - [`ECHOPRT`](#echoprt)
  - [`ECHOCTL`](#echoctl)
  - [`ISIG`](#isig)
  - [`ICANON`](#icanon)
  - [`PENDIN`](#pendin)
  - [`NOFLSH`](#noflsh)
  - [`CIBAUD`](#cibaud)
  - [`CBAUDEX`](#cbaudex)
  - [`VSWTC`](#vswtc)
  - [`OLCUC`](#olcuc)
  - [`NLDLY`](#nldly)
  - [`CRDLY`](#crdly)
  - [`TABDLY`](#tabdly)
  - [`BSDLY`](#bsdly)
  - [`FFDLY`](#ffdly)
  - [`VTDLY`](#vtdly)
  - [`XTABS`](#xtabs)
  - [`B0`](#b0)
  - [`B50`](#b50)
  - [`B75`](#b75)
  - [`B110`](#b110)
  - [`B134`](#b134)
  - [`B150`](#b150)
  - [`B200`](#b200)
  - [`B300`](#b300)
  - [`B600`](#b600)
  - [`B1200`](#b1200)
  - [`B1800`](#b1800)
  - [`B2400`](#b2400)
  - [`B4800`](#b4800)
  - [`B9600`](#b9600)
  - [`B19200`](#b19200)
  - [`B38400`](#b38400)
  - [`EXTA`](#exta)
  - [`EXTB`](#extb)
  - [`B57600`](#b57600)
  - [`B115200`](#b115200)
  - [`B230400`](#b230400)
  - [`B460800`](#b460800)
  - [`B500000`](#b500000)
  - [`B576000`](#b576000)
  - [`B921600`](#b921600)
  - [`B1000000`](#b1000000)
  - [`B1152000`](#b1152000)
  - [`B1500000`](#b1500000)
  - [`B2000000`](#b2000000)
  - [`B2500000`](#b2500000)
  - [`B3000000`](#b3000000)
  - [`B3500000`](#b3500000)
  - [`B4000000`](#b4000000)
  - [`VEOL`](#veol)
  - [`VEOL2`](#veol2)
  - [`VMIN`](#vmin)
  - [`IEXTEN`](#iexten)
  - [`TOSTOP`](#tostop)
  - [`FLUSHO`](#flusho)
  - [`EXTPROC`](#extproc)
  - [`R15`](#r15)
  - [`R14`](#r14)
  - [`R13`](#r13)
  - [`R12`](#r12)
  - [`RBP`](#rbp)
  - [`RBX`](#rbx)
  - [`R11`](#r11)
  - [`R10`](#r10)
  - [`R9`](#r9)
  - [`R8`](#r8)
  - [`RAX`](#rax)
  - [`RCX`](#rcx)
  - [`RDX`](#rdx)
  - [`RSI`](#rsi)
  - [`RDI`](#rdi)
  - [`ORIG_RAX`](#orig-rax)
  - [`RIP`](#rip)
  - [`CS`](#cs)
  - [`EFLAGS`](#eflags)
  - [`RSP`](#rsp)
  - [`SS`](#ss)
  - [`FS_BASE`](#fs-base)
  - [`GS_BASE`](#gs-base)
  - [`DS`](#ds)
  - [`ES`](#es)
  - [`FS`](#fs)
  - [`GS`](#gs)
  - [`REG_R8`](#reg-r8)
  - [`REG_R9`](#reg-r9)
  - [`REG_R10`](#reg-r10)
  - [`REG_R11`](#reg-r11)
  - [`REG_R12`](#reg-r12)
  - [`REG_R13`](#reg-r13)
  - [`REG_R14`](#reg-r14)
  - [`REG_R15`](#reg-r15)
  - [`REG_RDI`](#reg-rdi)
  - [`REG_RSI`](#reg-rsi)
  - [`REG_RBP`](#reg-rbp)
  - [`REG_RBX`](#reg-rbx)
  - [`REG_RDX`](#reg-rdx)
  - [`REG_RAX`](#reg-rax)
  - [`REG_RCX`](#reg-rcx)
  - [`REG_RSP`](#reg-rsp)
  - [`REG_RIP`](#reg-rip)
  - [`REG_EFL`](#reg-efl)
  - [`REG_CSGSFS`](#reg-csgsfs)
  - [`REG_ERR`](#reg-err)
  - [`REG_TRAPNO`](#reg-trapno)
  - [`REG_OLDMASK`](#reg-oldmask)
  - [`REG_CR2`](#reg-cr2)
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
| [`not_x32`](#not-x32) | mod |  |
| [`sigaction`](#sigaction) | struct |  |
| [`statfs`](#statfs) | struct |  |
| [`flock`](#flock) | struct |  |
| [`flock64`](#flock64) | struct |  |
| [`siginfo_t`](#siginfo-t) | struct |  |
| [`stack_t`](#stack-t) | struct |  |
| [`stat`](#stat) | struct |  |
| [`stat64`](#stat64) | struct |  |
| [`statfs64`](#statfs64) | struct |  |
| [`statvfs64`](#statvfs64) | struct |  |
| [`pthread_attr_t`](#pthread-attr-t) | struct |  |
| [`_libc_fpxreg`](#libc-fpxreg) | struct |  |
| [`_libc_xmmreg`](#libc-xmmreg) | struct |  |
| [`_libc_fpstate`](#libc-fpstate) | struct |  |
| [`user_regs_struct`](#user-regs-struct) | struct |  |
| [`user`](#user) | struct |  |
| [`mcontext_t`](#mcontext-t) | struct |  |
| [`ipc_perm`](#ipc-perm) | struct |  |
| [`shmid_ds`](#shmid-ds) | struct |  |
| [`ptrace_rseq_configuration`](#ptrace-rseq-configuration) | struct |  |
| [`clone_args`](#clone-args) | struct |  |
| [`user_fpregs_struct`](#user-fpregs-struct) | struct |  |
| [`ucontext_t`](#ucontext-t) | struct |  |
| [`max_align_t`](#max-align-t) | struct |  |
| [`statvfs`](#statvfs) | struct |  |
| [`getcontext`](#getcontext) | fn |  |
| [`setcontext`](#setcontext) | fn |  |
| [`makecontext`](#makecontext) | fn |  |
| [`swapcontext`](#swapcontext) | fn |  |
| [`sysctl`](#sysctl) | fn |  |
| [`wchar_t`](#wchar-t) | type |  |
| [`nlink_t`](#nlink-t) | type |  |
| [`blksize_t`](#blksize-t) | type |  |
| [`greg_t`](#greg-t) | type |  |
| [`suseconds_t`](#suseconds-t) | type |  |
| [`__u64`](#u64) | type |  |
| [`__s64`](#s64) | type |  |
| [`POSIX_FADV_DONTNEED`](#posix-fadv-dontneed) | const |  |
| [`POSIX_FADV_NOREUSE`](#posix-fadv-noreuse) | const |  |
| [`VEOF`](#veof) | const |  |
| [`RTLD_DEEPBIND`](#rtld-deepbind) | const |  |
| [`RTLD_GLOBAL`](#rtld-global) | const |  |
| [`RTLD_NOLOAD`](#rtld-noload) | const |  |
| [`O_APPEND`](#o-append) | const |  |
| [`O_CREAT`](#o-creat) | const |  |
| [`O_EXCL`](#o-excl) | const |  |
| [`O_NOCTTY`](#o-noctty) | const |  |
| [`O_NONBLOCK`](#o-nonblock) | const |  |
| [`O_SYNC`](#o-sync) | const |  |
| [`O_RSYNC`](#o-rsync) | const |  |
| [`O_DSYNC`](#o-dsync) | const |  |
| [`O_FSYNC`](#o-fsync) | const |  |
| [`O_NOATIME`](#o-noatime) | const |  |
| [`O_PATH`](#o-path) | const |  |
| [`O_TMPFILE`](#o-tmpfile) | const |  |
| [`MADV_SOFT_OFFLINE`](#madv-soft-offline) | const |  |
| [`MAP_GROWSDOWN`](#map-growsdown) | const |  |
| [`EDEADLK`](#edeadlk) | const |  |
| [`ENAMETOOLONG`](#enametoolong) | const |  |
| [`ENOLCK`](#enolck) | const |  |
| [`ENOSYS`](#enosys) | const |  |
| [`ENOTEMPTY`](#enotempty) | const |  |
| [`ELOOP`](#eloop) | const |  |
| [`ENOMSG`](#enomsg) | const |  |
| [`EIDRM`](#eidrm) | const |  |
| [`ECHRNG`](#echrng) | const |  |
| [`EL2NSYNC`](#el2nsync) | const |  |
| [`EL3HLT`](#el3hlt) | const |  |
| [`EL3RST`](#el3rst) | const |  |
| [`ELNRNG`](#elnrng) | const |  |
| [`EUNATCH`](#eunatch) | const |  |
| [`ENOCSI`](#enocsi) | const |  |
| [`EL2HLT`](#el2hlt) | const |  |
| [`EBADE`](#ebade) | const |  |
| [`EBADR`](#ebadr) | const |  |
| [`EXFULL`](#exfull) | const |  |
| [`ENOANO`](#enoano) | const |  |
| [`EBADRQC`](#ebadrqc) | const |  |
| [`EBADSLT`](#ebadslt) | const |  |
| [`EMULTIHOP`](#emultihop) | const |  |
| [`EOVERFLOW`](#eoverflow) | const |  |
| [`ENOTUNIQ`](#enotuniq) | const |  |
| [`EBADFD`](#ebadfd) | const |  |
| [`EBADMSG`](#ebadmsg) | const |  |
| [`EREMCHG`](#eremchg) | const |  |
| [`ELIBACC`](#elibacc) | const |  |
| [`ELIBBAD`](#elibbad) | const |  |
| [`ELIBSCN`](#elibscn) | const |  |
| [`ELIBMAX`](#elibmax) | const |  |
| [`ELIBEXEC`](#elibexec) | const |  |
| [`EILSEQ`](#eilseq) | const |  |
| [`ERESTART`](#erestart) | const |  |
| [`ESTRPIPE`](#estrpipe) | const |  |
| [`EUSERS`](#eusers) | const |  |
| [`ENOTSOCK`](#enotsock) | const |  |
| [`EDESTADDRREQ`](#edestaddrreq) | const |  |
| [`EMSGSIZE`](#emsgsize) | const |  |
| [`EPROTOTYPE`](#eprototype) | const |  |
| [`ENOPROTOOPT`](#enoprotoopt) | const |  |
| [`EPROTONOSUPPORT`](#eprotonosupport) | const |  |
| [`ESOCKTNOSUPPORT`](#esocktnosupport) | const |  |
| [`EOPNOTSUPP`](#eopnotsupp) | const |  |
| [`EPFNOSUPPORT`](#epfnosupport) | const |  |
| [`EAFNOSUPPORT`](#eafnosupport) | const |  |
| [`EADDRINUSE`](#eaddrinuse) | const |  |
| [`EADDRNOTAVAIL`](#eaddrnotavail) | const |  |
| [`ENETDOWN`](#enetdown) | const |  |
| [`ENETUNREACH`](#enetunreach) | const |  |
| [`ENETRESET`](#enetreset) | const |  |
| [`ECONNABORTED`](#econnaborted) | const |  |
| [`ECONNRESET`](#econnreset) | const |  |
| [`ENOBUFS`](#enobufs) | const |  |
| [`EISCONN`](#eisconn) | const |  |
| [`ENOTCONN`](#enotconn) | const |  |
| [`ESHUTDOWN`](#eshutdown) | const |  |
| [`ETOOMANYREFS`](#etoomanyrefs) | const |  |
| [`ETIMEDOUT`](#etimedout) | const |  |
| [`ECONNREFUSED`](#econnrefused) | const |  |
| [`EHOSTDOWN`](#ehostdown) | const |  |
| [`EHOSTUNREACH`](#ehostunreach) | const |  |
| [`EALREADY`](#ealready) | const |  |
| [`EINPROGRESS`](#einprogress) | const |  |
| [`ESTALE`](#estale) | const |  |
| [`EDQUOT`](#edquot) | const |  |
| [`ENOMEDIUM`](#enomedium) | const |  |
| [`EMEDIUMTYPE`](#emediumtype) | const |  |
| [`ECANCELED`](#ecanceled) | const |  |
| [`ENOKEY`](#enokey) | const |  |
| [`EKEYEXPIRED`](#ekeyexpired) | const |  |
| [`EKEYREVOKED`](#ekeyrevoked) | const |  |
| [`EKEYREJECTED`](#ekeyrejected) | const |  |
| [`EOWNERDEAD`](#eownerdead) | const |  |
| [`ENOTRECOVERABLE`](#enotrecoverable) | const |  |
| [`EHWPOISON`](#ehwpoison) | const |  |
| [`ERFKILL`](#erfkill) | const |  |
| [`SOCK_STREAM`](#sock-stream) | const |  |
| [`SOCK_DGRAM`](#sock-dgram) | const |  |
| [`SA_ONSTACK`](#sa-onstack) | const |  |
| [`SA_SIGINFO`](#sa-siginfo) | const |  |
| [`SA_NOCLDWAIT`](#sa-nocldwait) | const |  |
| [`SIGTTIN`](#sigttin) | const |  |
| [`SIGTTOU`](#sigttou) | const |  |
| [`SIGXCPU`](#sigxcpu) | const |  |
| [`SIGXFSZ`](#sigxfsz) | const |  |
| [`SIGVTALRM`](#sigvtalrm) | const |  |
| [`SIGPROF`](#sigprof) | const |  |
| [`SIGWINCH`](#sigwinch) | const |  |
| [`SIGCHLD`](#sigchld) | const |  |
| [`SIGBUS`](#sigbus) | const |  |
| [`SIGUSR1`](#sigusr1) | const |  |
| [`SIGUSR2`](#sigusr2) | const |  |
| [`SIGCONT`](#sigcont) | const |  |
| [`SIGSTOP`](#sigstop) | const |  |
| [`SIGTSTP`](#sigtstp) | const |  |
| [`SIGURG`](#sigurg) | const |  |
| [`SIGIO`](#sigio) | const |  |
| [`SIGSYS`](#sigsys) | const |  |
| [`SIGSTKFLT`](#sigstkflt) | const |  |
| [`SIGUNUSED`](#sigunused) | const |  |
| [`SIGPOLL`](#sigpoll) | const |  |
| [`SIGPWR`](#sigpwr) | const |  |
| [`SIG_SETMASK`](#sig-setmask) | const |  |
| [`SIG_BLOCK`](#sig-block) | const |  |
| [`SIG_UNBLOCK`](#sig-unblock) | const |  |
| [`POLLWRNORM`](#pollwrnorm) | const |  |
| [`POLLWRBAND`](#pollwrband) | const |  |
| [`O_ASYNC`](#o-async) | const |  |
| [`O_NDELAY`](#o-ndelay) | const |  |
| [`PTRACE_DETACH`](#ptrace-detach) | const |  |
| [`PTRACE_GET_RSEQ_CONFIGURATION`](#ptrace-get-rseq-configuration) | const |  |
| [`EFD_NONBLOCK`](#efd-nonblock) | const |  |
| [`F_GETLK`](#f-getlk) | const |  |
| [`F_GETOWN`](#f-getown) | const |  |
| [`F_SETOWN`](#f-setown) | const |  |
| [`F_SETLK`](#f-setlk) | const |  |
| [`F_SETLKW`](#f-setlkw) | const |  |
| [`F_OFD_GETLK`](#f-ofd-getlk) | const |  |
| [`F_OFD_SETLK`](#f-ofd-setlk) | const |  |
| [`F_OFD_SETLKW`](#f-ofd-setlkw) | const |  |
| [`F_RDLCK`](#f-rdlck) | const |  |
| [`F_WRLCK`](#f-wrlck) | const |  |
| [`F_UNLCK`](#f-unlck) | const |  |
| [`SFD_NONBLOCK`](#sfd-nonblock) | const |  |
| [`TCSANOW`](#tcsanow) | const |  |
| [`TCSADRAIN`](#tcsadrain) | const |  |
| [`TCSAFLUSH`](#tcsaflush) | const |  |
| [`SFD_CLOEXEC`](#sfd-cloexec) | const |  |
| [`NCCS`](#nccs) | const |  |
| [`O_TRUNC`](#o-trunc) | const |  |
| [`O_CLOEXEC`](#o-cloexec) | const |  |
| [`EBFONT`](#ebfont) | const |  |
| [`ENOSTR`](#enostr) | const |  |
| [`ENODATA`](#enodata) | const |  |
| [`ETIME`](#etime) | const |  |
| [`ENOSR`](#enosr) | const |  |
| [`ENONET`](#enonet) | const |  |
| [`ENOPKG`](#enopkg) | const |  |
| [`EREMOTE`](#eremote) | const |  |
| [`ENOLINK`](#enolink) | const |  |
| [`EADV`](#eadv) | const |  |
| [`ESRMNT`](#esrmnt) | const |  |
| [`ECOMM`](#ecomm) | const |  |
| [`EPROTO`](#eproto) | const |  |
| [`EDOTDOT`](#edotdot) | const |  |
| [`SA_NODEFER`](#sa-nodefer) | const |  |
| [`SA_RESETHAND`](#sa-resethand) | const |  |
| [`SA_RESTART`](#sa-restart) | const |  |
| [`SA_NOCLDSTOP`](#sa-nocldstop) | const |  |
| [`EPOLL_CLOEXEC`](#epoll-cloexec) | const |  |
| [`EFD_CLOEXEC`](#efd-cloexec) | const |  |
| [`__SIZEOF_PTHREAD_CONDATTR_T`](#sizeof-pthread-condattr-t) | const |  |
| [`__SIZEOF_PTHREAD_MUTEXATTR_T`](#sizeof-pthread-mutexattr-t) | const |  |
| [`__SIZEOF_PTHREAD_BARRIERATTR_T`](#sizeof-pthread-barrierattr-t) | const |  |
| [`O_DIRECT`](#o-direct) | const |  |
| [`O_DIRECTORY`](#o-directory) | const |  |
| [`O_NOFOLLOW`](#o-nofollow) | const |  |
| [`MAP_HUGETLB`](#map-hugetlb) | const |  |
| [`MAP_LOCKED`](#map-locked) | const |  |
| [`MAP_NORESERVE`](#map-noreserve) | const |  |
| [`MAP_32BIT`](#map-32bit) | const |  |
| [`MAP_ANON`](#map-anon) | const |  |
| [`MAP_ANONYMOUS`](#map-anonymous) | const |  |
| [`MAP_DENYWRITE`](#map-denywrite) | const |  |
| [`MAP_EXECUTABLE`](#map-executable) | const |  |
| [`MAP_POPULATE`](#map-populate) | const |  |
| [`MAP_NONBLOCK`](#map-nonblock) | const |  |
| [`MAP_STACK`](#map-stack) | const |  |
| [`MAP_SYNC`](#map-sync) | const |  |
| [`EDEADLOCK`](#edeadlock) | const |  |
| [`EUCLEAN`](#euclean) | const |  |
| [`ENOTNAM`](#enotnam) | const |  |
| [`ENAVAIL`](#enavail) | const |  |
| [`EISNAM`](#eisnam) | const |  |
| [`EREMOTEIO`](#eremoteio) | const |  |
| [`PTRACE_GETFPREGS`](#ptrace-getfpregs) | const |  |
| [`PTRACE_SETFPREGS`](#ptrace-setfpregs) | const |  |
| [`PTRACE_GETFPXREGS`](#ptrace-getfpxregs) | const |  |
| [`PTRACE_SETFPXREGS`](#ptrace-setfpxregs) | const |  |
| [`PTRACE_GETREGS`](#ptrace-getregs) | const |  |
| [`PTRACE_SETREGS`](#ptrace-setregs) | const |  |
| [`PTRACE_PEEKSIGINFO_SHARED`](#ptrace-peeksiginfo-shared) | const |  |
| [`PTRACE_SYSEMU`](#ptrace-sysemu) | const |  |
| [`PTRACE_SYSEMU_SINGLESTEP`](#ptrace-sysemu-singlestep) | const |  |
| [`PR_GET_SPECULATION_CTRL`](#pr-get-speculation-ctrl) | const |  |
| [`PR_SET_SPECULATION_CTRL`](#pr-set-speculation-ctrl) | const |  |
| [`PR_SPEC_NOT_AFFECTED`](#pr-spec-not-affected) | const |  |
| [`PR_SPEC_PRCTL`](#pr-spec-prctl) | const |  |
| [`PR_SPEC_ENABLE`](#pr-spec-enable) | const |  |
| [`PR_SPEC_DISABLE`](#pr-spec-disable) | const |  |
| [`PR_SPEC_FORCE_DISABLE`](#pr-spec-force-disable) | const |  |
| [`PR_SPEC_DISABLE_NOEXEC`](#pr-spec-disable-noexec) | const |  |
| [`PR_SPEC_STORE_BYPASS`](#pr-spec-store-bypass) | const |  |
| [`PR_SPEC_INDIRECT_BRANCH`](#pr-spec-indirect-branch) | const |  |
| [`MCL_CURRENT`](#mcl-current) | const |  |
| [`MCL_FUTURE`](#mcl-future) | const |  |
| [`MCL_ONFAULT`](#mcl-onfault) | const |  |
| [`SIGSTKSZ`](#sigstksz) | const |  |
| [`MINSIGSTKSZ`](#minsigstksz) | const |  |
| [`CBAUD`](#cbaud) | const |  |
| [`TAB1`](#tab1) | const |  |
| [`TAB2`](#tab2) | const |  |
| [`TAB3`](#tab3) | const |  |
| [`CR1`](#cr1) | const |  |
| [`CR2`](#cr2) | const |  |
| [`CR3`](#cr3) | const |  |
| [`FF1`](#ff1) | const |  |
| [`BS1`](#bs1) | const |  |
| [`VT1`](#vt1) | const |  |
| [`VWERASE`](#vwerase) | const |  |
| [`VREPRINT`](#vreprint) | const |  |
| [`VSUSP`](#vsusp) | const |  |
| [`VSTART`](#vstart) | const |  |
| [`VSTOP`](#vstop) | const |  |
| [`VDISCARD`](#vdiscard) | const |  |
| [`VTIME`](#vtime) | const |  |
| [`IXON`](#ixon) | const |  |
| [`IXOFF`](#ixoff) | const |  |
| [`ONLCR`](#onlcr) | const |  |
| [`CSIZE`](#csize) | const |  |
| [`CS6`](#cs6) | const |  |
| [`CS7`](#cs7) | const |  |
| [`CS8`](#cs8) | const |  |
| [`CSTOPB`](#cstopb) | const |  |
| [`CREAD`](#cread) | const |  |
| [`PARENB`](#parenb) | const |  |
| [`PARODD`](#parodd) | const |  |
| [`HUPCL`](#hupcl) | const |  |
| [`CLOCAL`](#clocal) | const |  |
| [`ECHOKE`](#echoke) | const |  |
| [`ECHOE`](#echoe) | const |  |
| [`ECHOK`](#echok) | const |  |
| [`ECHONL`](#echonl) | const |  |
| [`ECHOPRT`](#echoprt) | const |  |
| [`ECHOCTL`](#echoctl) | const |  |
| [`ISIG`](#isig) | const |  |
| [`ICANON`](#icanon) | const |  |
| [`PENDIN`](#pendin) | const |  |
| [`NOFLSH`](#noflsh) | const |  |
| [`CIBAUD`](#cibaud) | const |  |
| [`CBAUDEX`](#cbaudex) | const |  |
| [`VSWTC`](#vswtc) | const |  |
| [`OLCUC`](#olcuc) | const |  |
| [`NLDLY`](#nldly) | const |  |
| [`CRDLY`](#crdly) | const |  |
| [`TABDLY`](#tabdly) | const |  |
| [`BSDLY`](#bsdly) | const |  |
| [`FFDLY`](#ffdly) | const |  |
| [`VTDLY`](#vtdly) | const |  |
| [`XTABS`](#xtabs) | const |  |
| [`B0`](#b0) | const |  |
| [`B50`](#b50) | const |  |
| [`B75`](#b75) | const |  |
| [`B110`](#b110) | const |  |
| [`B134`](#b134) | const |  |
| [`B150`](#b150) | const |  |
| [`B200`](#b200) | const |  |
| [`B300`](#b300) | const |  |
| [`B600`](#b600) | const |  |
| [`B1200`](#b1200) | const |  |
| [`B1800`](#b1800) | const |  |
| [`B2400`](#b2400) | const |  |
| [`B4800`](#b4800) | const |  |
| [`B9600`](#b9600) | const |  |
| [`B19200`](#b19200) | const |  |
| [`B38400`](#b38400) | const |  |
| [`EXTA`](#exta) | const |  |
| [`EXTB`](#extb) | const |  |
| [`B57600`](#b57600) | const |  |
| [`B115200`](#b115200) | const |  |
| [`B230400`](#b230400) | const |  |
| [`B460800`](#b460800) | const |  |
| [`B500000`](#b500000) | const |  |
| [`B576000`](#b576000) | const |  |
| [`B921600`](#b921600) | const |  |
| [`B1000000`](#b1000000) | const |  |
| [`B1152000`](#b1152000) | const |  |
| [`B1500000`](#b1500000) | const |  |
| [`B2000000`](#b2000000) | const |  |
| [`B2500000`](#b2500000) | const |  |
| [`B3000000`](#b3000000) | const |  |
| [`B3500000`](#b3500000) | const |  |
| [`B4000000`](#b4000000) | const |  |
| [`VEOL`](#veol) | const |  |
| [`VEOL2`](#veol2) | const |  |
| [`VMIN`](#vmin) | const |  |
| [`IEXTEN`](#iexten) | const |  |
| [`TOSTOP`](#tostop) | const |  |
| [`FLUSHO`](#flusho) | const |  |
| [`EXTPROC`](#extproc) | const |  |
| [`R15`](#r15) | const |  |
| [`R14`](#r14) | const |  |
| [`R13`](#r13) | const |  |
| [`R12`](#r12) | const |  |
| [`RBP`](#rbp) | const |  |
| [`RBX`](#rbx) | const |  |
| [`R11`](#r11) | const |  |
| [`R10`](#r10) | const |  |
| [`R9`](#r9) | const |  |
| [`R8`](#r8) | const |  |
| [`RAX`](#rax) | const |  |
| [`RCX`](#rcx) | const |  |
| [`RDX`](#rdx) | const |  |
| [`RSI`](#rsi) | const |  |
| [`RDI`](#rdi) | const |  |
| [`ORIG_RAX`](#orig-rax) | const |  |
| [`RIP`](#rip) | const |  |
| [`CS`](#cs) | const |  |
| [`EFLAGS`](#eflags) | const |  |
| [`RSP`](#rsp) | const |  |
| [`SS`](#ss) | const |  |
| [`FS_BASE`](#fs-base) | const |  |
| [`GS_BASE`](#gs-base) | const |  |
| [`DS`](#ds) | const |  |
| [`ES`](#es) | const |  |
| [`FS`](#fs) | const |  |
| [`GS`](#gs) | const |  |
| [`REG_R8`](#reg-r8) | const |  |
| [`REG_R9`](#reg-r9) | const |  |
| [`REG_R10`](#reg-r10) | const |  |
| [`REG_R11`](#reg-r11) | const |  |
| [`REG_R12`](#reg-r12) | const |  |
| [`REG_R13`](#reg-r13) | const |  |
| [`REG_R14`](#reg-r14) | const |  |
| [`REG_R15`](#reg-r15) | const |  |
| [`REG_RDI`](#reg-rdi) | const |  |
| [`REG_RSI`](#reg-rsi) | const |  |
| [`REG_RBP`](#reg-rbp) | const |  |
| [`REG_RBX`](#reg-rbx) | const |  |
| [`REG_RDX`](#reg-rdx) | const |  |
| [`REG_RAX`](#reg-rax) | const |  |
| [`REG_RCX`](#reg-rcx) | const |  |
| [`REG_RSP`](#reg-rsp) | const |  |
| [`REG_RIP`](#reg-rip) | const |  |
| [`REG_EFL`](#reg-efl) | const |  |
| [`REG_CSGSFS`](#reg-csgsfs) | const |  |
| [`REG_ERR`](#reg-err) | const |  |
| [`REG_TRAPNO`](#reg-trapno) | const |  |
| [`REG_OLDMASK`](#reg-oldmask) | const |  |
| [`REG_CR2`](#reg-cr2) | const |  |
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

## Modules

- [`not_x32`](not_x32/index.md)

## Structs

### `sigaction`

```rust
struct sigaction {
    pub sa_sigaction: crate::sighandler_t,
    pub sa_mask: crate::sigset_t,
    pub sa_flags: c_int,
    pub sa_restorer: Option<fn()>,
}
```

#### Trait Implementations

##### `impl Clone for sigaction`

- <span id="sigaction-clone"></span>`fn clone(&self) -> sigaction` — [`sigaction`](../index.md#sigaction)

##### `impl Copy for sigaction`

##### `impl Debug for sigaction`

- <span id="sigaction-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `statfs`

```rust
struct statfs {
    pub f_type: crate::__fsword_t,
    pub f_bsize: crate::__fsword_t,
    pub f_blocks: crate::fsblkcnt_t,
    pub f_bfree: crate::fsblkcnt_t,
    pub f_bavail: crate::fsblkcnt_t,
    pub f_files: crate::fsfilcnt_t,
    pub f_ffree: crate::fsfilcnt_t,
    pub f_fsid: crate::fsid_t,
    pub f_namelen: crate::__fsword_t,
    pub f_frsize: crate::__fsword_t,
    f_spare: [crate::__fsword_t; 5],
}
```

#### Trait Implementations

##### `impl Clone for statfs`

- <span id="statfs-clone"></span>`fn clone(&self) -> statfs` — [`statfs`](../index.md#statfs)

##### `impl Copy for statfs`

##### `impl Debug for statfs`

- <span id="statfs-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `flock`

```rust
struct flock {
    pub l_type: c_short,
    pub l_whence: c_short,
    pub l_start: crate::off_t,
    pub l_len: crate::off_t,
    pub l_pid: crate::pid_t,
}
```

#### Trait Implementations

##### `impl Clone for flock`

- <span id="flock-clone"></span>`fn clone(&self) -> flock` — [`flock`](../index.md#flock)

##### `impl Copy for flock`

##### `impl Debug for flock`

- <span id="flock-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `flock64`

```rust
struct flock64 {
    pub l_type: c_short,
    pub l_whence: c_short,
    pub l_start: crate::off64_t,
    pub l_len: crate::off64_t,
    pub l_pid: crate::pid_t,
}
```

#### Trait Implementations

##### `impl Clone for flock64`

- <span id="flock64-clone"></span>`fn clone(&self) -> flock64` — [`flock64`](../index.md#flock64)

##### `impl Copy for flock64`

##### `impl Debug for flock64`

- <span id="flock64-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `siginfo_t`

```rust
struct siginfo_t {
    pub si_signo: c_int,
    pub si_errno: c_int,
    pub si_code: c_int,
    _align: [u64; 0],
    // [REDACTED: Private Fields]
}
```

#### Implementations

- <span id="siginfo-t-si-addr"></span>`unsafe fn si_addr(&self) -> *mut c_void` — [`c_void`](../../../../../../index.md#c-void)

- <span id="siginfo-t-si-value"></span>`unsafe fn si_value(&self) -> crate::sigval` — [`sigval`](../../../../../../index.md#sigval)

#### Trait Implementations

##### `impl Clone for siginfo_t`

- <span id="siginfo-t-clone"></span>`fn clone(&self) -> siginfo_t` — [`siginfo_t`](../index.md#siginfo-t)

##### `impl Copy for siginfo_t`

##### `impl Debug for siginfo_t`

- <span id="siginfo-t-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `stack_t`

```rust
struct stack_t {
    pub ss_sp: *mut c_void,
    pub ss_flags: c_int,
    pub ss_size: size_t,
}
```

#### Trait Implementations

##### `impl Clone for stack_t`

- <span id="stack-t-clone"></span>`fn clone(&self) -> stack_t` — [`stack_t`](../index.md#stack-t)

##### `impl Copy for stack_t`

##### `impl Debug for stack_t`

- <span id="stack-t-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `stat`

```rust
struct stat {
    pub st_dev: crate::dev_t,
    pub st_ino: crate::ino_t,
    pub st_nlink: crate::nlink_t,
    pub st_mode: crate::mode_t,
    pub st_uid: crate::uid_t,
    pub st_gid: crate::gid_t,
    __pad0: Padding<c_int>,
    pub st_rdev: crate::dev_t,
    pub st_size: crate::off_t,
    pub st_blksize: crate::blksize_t,
    pub st_blocks: crate::blkcnt_t,
    pub st_atime: crate::time_t,
    pub st_atime_nsec: i64,
    pub st_mtime: crate::time_t,
    pub st_mtime_nsec: i64,
    pub st_ctime: crate::time_t,
    pub st_ctime_nsec: i64,
    __unused: Padding<[i64; 3]>,
}
```

#### Trait Implementations

##### `impl Clone for stat`

- <span id="stat-clone"></span>`fn clone(&self) -> stat` — [`stat`](../index.md#stat)

##### `impl Copy for stat`

##### `impl Debug for stat`

- <span id="stat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `stat64`

```rust
struct stat64 {
    pub st_dev: crate::dev_t,
    pub st_ino: crate::ino64_t,
    pub st_nlink: crate::nlink_t,
    pub st_mode: crate::mode_t,
    pub st_uid: crate::uid_t,
    pub st_gid: crate::gid_t,
    __pad0: Padding<c_int>,
    pub st_rdev: crate::dev_t,
    pub st_size: crate::off_t,
    pub st_blksize: crate::blksize_t,
    pub st_blocks: crate::blkcnt64_t,
    pub st_atime: crate::time_t,
    pub st_atime_nsec: i64,
    pub st_mtime: crate::time_t,
    pub st_mtime_nsec: i64,
    pub st_ctime: crate::time_t,
    pub st_ctime_nsec: i64,
    __reserved: Padding<[i64; 3]>,
}
```

#### Trait Implementations

##### `impl Clone for stat64`

- <span id="stat64-clone"></span>`fn clone(&self) -> stat64` — [`stat64`](../index.md#stat64)

##### `impl Copy for stat64`

##### `impl Debug for stat64`

- <span id="stat64-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `statfs64`

```rust
struct statfs64 {
    pub f_type: crate::__fsword_t,
    pub f_bsize: crate::__fsword_t,
    pub f_blocks: u64,
    pub f_bfree: u64,
    pub f_bavail: u64,
    pub f_files: u64,
    pub f_ffree: u64,
    pub f_fsid: crate::fsid_t,
    pub f_namelen: crate::__fsword_t,
    pub f_frsize: crate::__fsword_t,
    pub f_flags: crate::__fsword_t,
    pub f_spare: [crate::__fsword_t; 4],
}
```

#### Trait Implementations

##### `impl Clone for statfs64`

- <span id="statfs64-clone"></span>`fn clone(&self) -> statfs64` — [`statfs64`](../index.md#statfs64)

##### `impl Copy for statfs64`

##### `impl Debug for statfs64`

- <span id="statfs64-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `statvfs64`

```rust
struct statvfs64 {
    pub f_bsize: c_ulong,
    pub f_frsize: c_ulong,
    pub f_blocks: u64,
    pub f_bfree: u64,
    pub f_bavail: u64,
    pub f_files: u64,
    pub f_ffree: u64,
    pub f_favail: u64,
    pub f_fsid: c_ulong,
    pub f_flag: c_ulong,
    pub f_namemax: c_ulong,
    __f_spare: [c_int; 6],
}
```

#### Trait Implementations

##### `impl Clone for statvfs64`

- <span id="statvfs64-clone"></span>`fn clone(&self) -> statvfs64` — [`statvfs64`](../index.md#statvfs64)

##### `impl Copy for statvfs64`

##### `impl Debug for statvfs64`

- <span id="statvfs64-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `pthread_attr_t`

```rust
struct pthread_attr_t {
    __size: [u64; 7],
}
```

#### Trait Implementations

##### `impl Clone for pthread_attr_t`

- <span id="pthread-attr-t-clone"></span>`fn clone(&self) -> pthread_attr_t` — [`pthread_attr_t`](../index.md#pthread-attr-t)

##### `impl Copy for pthread_attr_t`

##### `impl Debug for pthread_attr_t`

- <span id="pthread-attr-t-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `_libc_fpxreg`

```rust
struct _libc_fpxreg {
    pub significand: [u16; 4],
    pub exponent: u16,
    __private: [u16; 3],
}
```

#### Trait Implementations

##### `impl Clone for _libc_fpxreg`

- <span id="libc-fpxreg-clone"></span>`fn clone(&self) -> _libc_fpxreg` — [`_libc_fpxreg`](../index.md#libc-fpxreg)

##### `impl Copy for _libc_fpxreg`

##### `impl Debug for _libc_fpxreg`

- <span id="libc-fpxreg-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `_libc_xmmreg`

```rust
struct _libc_xmmreg {
    pub element: [u32; 4],
}
```

#### Trait Implementations

##### `impl Clone for _libc_xmmreg`

- <span id="libc-xmmreg-clone"></span>`fn clone(&self) -> _libc_xmmreg` — [`_libc_xmmreg`](../index.md#libc-xmmreg)

##### `impl Copy for _libc_xmmreg`

##### `impl Debug for _libc_xmmreg`

- <span id="libc-xmmreg-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `_libc_fpstate`

```rust
struct _libc_fpstate {
    pub cwd: u16,
    pub swd: u16,
    pub ftw: u16,
    pub fop: u16,
    pub rip: u64,
    pub rdp: u64,
    pub mxcsr: u32,
    pub mxcr_mask: u32,
    pub _st: [_libc_fpxreg; 8],
    pub _xmm: [_libc_xmmreg; 16],
    __private: [u64; 12],
}
```

#### Trait Implementations

##### `impl Clone for _libc_fpstate`

- <span id="libc-fpstate-clone"></span>`fn clone(&self) -> _libc_fpstate` — [`_libc_fpstate`](../index.md#libc-fpstate)

##### `impl Copy for _libc_fpstate`

##### `impl Debug for _libc_fpstate`

- <span id="libc-fpstate-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `user_regs_struct`

```rust
struct user_regs_struct {
    pub r15: c_ulonglong,
    pub r14: c_ulonglong,
    pub r13: c_ulonglong,
    pub r12: c_ulonglong,
    pub rbp: c_ulonglong,
    pub rbx: c_ulonglong,
    pub r11: c_ulonglong,
    pub r10: c_ulonglong,
    pub r9: c_ulonglong,
    pub r8: c_ulonglong,
    pub rax: c_ulonglong,
    pub rcx: c_ulonglong,
    pub rdx: c_ulonglong,
    pub rsi: c_ulonglong,
    pub rdi: c_ulonglong,
    pub orig_rax: c_ulonglong,
    pub rip: c_ulonglong,
    pub cs: c_ulonglong,
    pub eflags: c_ulonglong,
    pub rsp: c_ulonglong,
    pub ss: c_ulonglong,
    pub fs_base: c_ulonglong,
    pub gs_base: c_ulonglong,
    pub ds: c_ulonglong,
    pub es: c_ulonglong,
    pub fs: c_ulonglong,
    pub gs: c_ulonglong,
}
```

#### Trait Implementations

##### `impl Clone for user_regs_struct`

- <span id="user-regs-struct-clone"></span>`fn clone(&self) -> user_regs_struct` — [`user_regs_struct`](../index.md#user-regs-struct)

##### `impl Copy for user_regs_struct`

##### `impl Debug for user_regs_struct`

- <span id="user-regs-struct-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `user`

```rust
struct user {
    pub regs: user_regs_struct,
    pub u_fpvalid: c_int,
    pub i387: user_fpregs_struct,
    pub u_tsize: c_ulonglong,
    pub u_dsize: c_ulonglong,
    pub u_ssize: c_ulonglong,
    pub start_code: c_ulonglong,
    pub start_stack: c_ulonglong,
    pub signal: c_longlong,
    __reserved: Padding<c_int>,
    pub u_ar0: *mut user_regs_struct,
    pub u_fpstate: *mut user_fpregs_struct,
    pub magic: c_ulonglong,
    pub u_comm: [c_char; 32],
    pub u_debugreg: [c_ulonglong; 8],
}
```

#### Trait Implementations

##### `impl Clone for user`

- <span id="user-clone"></span>`fn clone(&self) -> user` — [`user`](../index.md#user)

##### `impl Copy for user`

##### `impl Debug for user`

- <span id="user-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `mcontext_t`

```rust
struct mcontext_t {
    pub gregs: [greg_t; 23],
    pub fpregs: *mut _libc_fpstate,
    __private: [u64; 8],
}
```

#### Trait Implementations

##### `impl Clone for mcontext_t`

- <span id="mcontext-t-clone"></span>`fn clone(&self) -> mcontext_t` — [`mcontext_t`](../index.md#mcontext-t)

##### `impl Copy for mcontext_t`

##### `impl Debug for mcontext_t`

- <span id="mcontext-t-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ipc_perm`

```rust
struct ipc_perm {
    pub __key: crate::key_t,
    pub uid: crate::uid_t,
    pub gid: crate::gid_t,
    pub cuid: crate::uid_t,
    pub cgid: crate::gid_t,
    pub mode: c_ushort,
    __pad1: Padding<c_ushort>,
    pub __seq: c_ushort,
    __pad2: Padding<c_ushort>,
    __unused1: Padding<u64>,
    __unused2: Padding<u64>,
}
```

#### Trait Implementations

##### `impl Clone for ipc_perm`

- <span id="ipc-perm-clone"></span>`fn clone(&self) -> ipc_perm` — [`ipc_perm`](../index.md#ipc-perm)

##### `impl Copy for ipc_perm`

##### `impl Debug for ipc_perm`

- <span id="ipc-perm-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `shmid_ds`

```rust
struct shmid_ds {
    pub shm_perm: crate::ipc_perm,
    pub shm_segsz: size_t,
    pub shm_atime: crate::time_t,
    pub shm_dtime: crate::time_t,
    pub shm_ctime: crate::time_t,
    pub shm_cpid: crate::pid_t,
    pub shm_lpid: crate::pid_t,
    pub shm_nattch: crate::shmatt_t,
    __unused4: Padding<u64>,
    __unused5: Padding<u64>,
}
```

#### Trait Implementations

##### `impl Clone for shmid_ds`

- <span id="shmid-ds-clone"></span>`fn clone(&self) -> shmid_ds` — [`shmid_ds`](../index.md#shmid-ds)

##### `impl Copy for shmid_ds`

##### `impl Debug for shmid_ds`

- <span id="shmid-ds-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ptrace_rseq_configuration`

```rust
struct ptrace_rseq_configuration {
    pub rseq_abi_pointer: crate::__u64,
    pub rseq_abi_size: crate::__u32,
    pub signature: crate::__u32,
    pub flags: crate::__u32,
    pub pad: crate::__u32,
}
```

#### Trait Implementations

##### `impl Clone for ptrace_rseq_configuration`

- <span id="ptrace-rseq-configuration-clone"></span>`fn clone(&self) -> ptrace_rseq_configuration` — [`ptrace_rseq_configuration`](../index.md#ptrace-rseq-configuration)

##### `impl Copy for ptrace_rseq_configuration`

##### `impl Debug for ptrace_rseq_configuration`

- <span id="ptrace-rseq-configuration-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `clone_args`

```rust
struct clone_args {
    pub flags: c_ulonglong,
    pub pidfd: c_ulonglong,
    pub child_tid: c_ulonglong,
    pub parent_tid: c_ulonglong,
    pub exit_signal: c_ulonglong,
    pub stack: c_ulonglong,
    pub stack_size: c_ulonglong,
    pub tls: c_ulonglong,
    pub set_tid: c_ulonglong,
    pub set_tid_size: c_ulonglong,
    pub cgroup: c_ulonglong,
}
```

#### Trait Implementations

##### `impl Clone for clone_args`

- <span id="clone-args-clone"></span>`fn clone(&self) -> clone_args` — [`clone_args`](../index.md#clone-args)

##### `impl Copy for clone_args`

##### `impl Debug for clone_args`

- <span id="clone-args-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `user_fpregs_struct`

```rust
struct user_fpregs_struct {
    pub cwd: c_ushort,
    pub swd: c_ushort,
    pub ftw: c_ushort,
    pub fop: c_ushort,
    pub rip: c_ulonglong,
    pub rdp: c_ulonglong,
    pub mxcsr: c_uint,
    pub mxcr_mask: c_uint,
    pub st_space: [c_uint; 32],
    pub xmm_space: [c_uint; 64],
    padding: Padding<[c_uint; 24]>,
}
```

#### Trait Implementations

##### `impl Clone for user_fpregs_struct`

- <span id="user-fpregs-struct-clone"></span>`fn clone(&self) -> user_fpregs_struct` — [`user_fpregs_struct`](../index.md#user-fpregs-struct)

##### `impl Copy for user_fpregs_struct`

##### `impl Debug for user_fpregs_struct`

- <span id="user-fpregs-struct-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ucontext_t`

```rust
struct ucontext_t {
    pub uc_flags: c_ulong,
    pub uc_link: *mut ucontext_t,
    pub uc_stack: crate::stack_t,
    pub uc_mcontext: mcontext_t,
    pub uc_sigmask: crate::sigset_t,
    __private: [u8; 512],
    __ssp: [c_ulonglong; 4],
}
```

#### Trait Implementations

##### `impl Clone for ucontext_t`

- <span id="ucontext-t-clone"></span>`fn clone(&self) -> ucontext_t` — [`ucontext_t`](../index.md#ucontext-t)

##### `impl Copy for ucontext_t`

##### `impl Debug for ucontext_t`

- <span id="ucontext-t-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `max_align_t`

```rust
struct max_align_t {
    priv_: [f64; 4],
}
```

#### Trait Implementations

##### `impl Clone for max_align_t`

- <span id="max-align-t-clone"></span>`fn clone(&self) -> max_align_t` — [`max_align_t`](../index.md#max-align-t)

##### `impl Copy for max_align_t`

##### `impl Debug for max_align_t`

- <span id="max-align-t-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="statvfs-clone"></span>`fn clone(&self) -> statvfs` — [`statvfs`](#statvfs)

##### `impl Copy for statvfs`

##### `impl Debug for statvfs`

- <span id="statvfs-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

### `getcontext`

```rust
unsafe fn getcontext(ucp: *mut ucontext_t) -> c_int
```

### `setcontext`

```rust
unsafe fn setcontext(ucp: *const ucontext_t) -> c_int
```

### `makecontext`

```rust
unsafe fn makecontext(ucp: *mut ucontext_t, func: fn(), argc: c_int)
```

### `swapcontext`

```rust
unsafe fn swapcontext(uocp: *mut ucontext_t, ucp: *const ucontext_t) -> c_int
```

### `sysctl`

```rust
unsafe fn sysctl(name: *mut c_int, namelen: c_int, oldp: *mut c_void, oldlenp: *mut size_t, newp: *mut c_void, newlen: size_t) -> c_int
```

## Type Aliases

### `wchar_t`

```rust
type wchar_t = i32;
```

### `nlink_t`

```rust
type nlink_t = u64;
```

### `blksize_t`

```rust
type blksize_t = i64;
```

### `greg_t`

```rust
type greg_t = i64;
```

### `suseconds_t`

```rust
type suseconds_t = i64;
```

### `__u64`

```rust
type __u64 = c_ulonglong;
```

### `__s64`

```rust
type __s64 = c_longlong;
```

## Constants

### `POSIX_FADV_DONTNEED`
```rust
const POSIX_FADV_DONTNEED: c_int = 4i32;
```

### `POSIX_FADV_NOREUSE`
```rust
const POSIX_FADV_NOREUSE: c_int = 5i32;
```

### `VEOF`
```rust
const VEOF: usize = 4usize;
```

### `RTLD_DEEPBIND`
```rust
const RTLD_DEEPBIND: c_int = 8i32;
```

### `RTLD_GLOBAL`
```rust
const RTLD_GLOBAL: c_int = 256i32;
```

### `RTLD_NOLOAD`
```rust
const RTLD_NOLOAD: c_int = 4i32;
```

### `O_APPEND`
```rust
const O_APPEND: c_int = 1_024i32;
```

### `O_CREAT`
```rust
const O_CREAT: c_int = 64i32;
```

### `O_EXCL`
```rust
const O_EXCL: c_int = 128i32;
```

### `O_NOCTTY`
```rust
const O_NOCTTY: c_int = 256i32;
```

### `O_NONBLOCK`
```rust
const O_NONBLOCK: c_int = 2_048i32;
```

### `O_SYNC`
```rust
const O_SYNC: c_int = 1_052_672i32;
```

### `O_RSYNC`
```rust
const O_RSYNC: c_int = 1_052_672i32;
```

### `O_DSYNC`
```rust
const O_DSYNC: c_int = 4_096i32;
```

### `O_FSYNC`
```rust
const O_FSYNC: c_int = 1_052_672i32;
```

### `O_NOATIME`
```rust
const O_NOATIME: c_int = 262_144i32;
```

### `O_PATH`
```rust
const O_PATH: c_int = 2_097_152i32;
```

### `O_TMPFILE`
```rust
const O_TMPFILE: c_int = 4_259_840i32;
```

### `MADV_SOFT_OFFLINE`
```rust
const MADV_SOFT_OFFLINE: c_int = 101i32;
```

### `MAP_GROWSDOWN`
```rust
const MAP_GROWSDOWN: c_int = 256i32;
```

### `EDEADLK`
```rust
const EDEADLK: c_int = 35i32;
```

### `ENAMETOOLONG`
```rust
const ENAMETOOLONG: c_int = 36i32;
```

### `ENOLCK`
```rust
const ENOLCK: c_int = 37i32;
```

### `ENOSYS`
```rust
const ENOSYS: c_int = 38i32;
```

### `ENOTEMPTY`
```rust
const ENOTEMPTY: c_int = 39i32;
```

### `ELOOP`
```rust
const ELOOP: c_int = 40i32;
```

### `ENOMSG`
```rust
const ENOMSG: c_int = 42i32;
```

### `EIDRM`
```rust
const EIDRM: c_int = 43i32;
```

### `ECHRNG`
```rust
const ECHRNG: c_int = 44i32;
```

### `EL2NSYNC`
```rust
const EL2NSYNC: c_int = 45i32;
```

### `EL3HLT`
```rust
const EL3HLT: c_int = 46i32;
```

### `EL3RST`
```rust
const EL3RST: c_int = 47i32;
```

### `ELNRNG`
```rust
const ELNRNG: c_int = 48i32;
```

### `EUNATCH`
```rust
const EUNATCH: c_int = 49i32;
```

### `ENOCSI`
```rust
const ENOCSI: c_int = 50i32;
```

### `EL2HLT`
```rust
const EL2HLT: c_int = 51i32;
```

### `EBADE`
```rust
const EBADE: c_int = 52i32;
```

### `EBADR`
```rust
const EBADR: c_int = 53i32;
```

### `EXFULL`
```rust
const EXFULL: c_int = 54i32;
```

### `ENOANO`
```rust
const ENOANO: c_int = 55i32;
```

### `EBADRQC`
```rust
const EBADRQC: c_int = 56i32;
```

### `EBADSLT`
```rust
const EBADSLT: c_int = 57i32;
```

### `EMULTIHOP`
```rust
const EMULTIHOP: c_int = 72i32;
```

### `EOVERFLOW`
```rust
const EOVERFLOW: c_int = 75i32;
```

### `ENOTUNIQ`
```rust
const ENOTUNIQ: c_int = 76i32;
```

### `EBADFD`
```rust
const EBADFD: c_int = 77i32;
```

### `EBADMSG`
```rust
const EBADMSG: c_int = 74i32;
```

### `EREMCHG`
```rust
const EREMCHG: c_int = 78i32;
```

### `ELIBACC`
```rust
const ELIBACC: c_int = 79i32;
```

### `ELIBBAD`
```rust
const ELIBBAD: c_int = 80i32;
```

### `ELIBSCN`
```rust
const ELIBSCN: c_int = 81i32;
```

### `ELIBMAX`
```rust
const ELIBMAX: c_int = 82i32;
```

### `ELIBEXEC`
```rust
const ELIBEXEC: c_int = 83i32;
```

### `EILSEQ`
```rust
const EILSEQ: c_int = 84i32;
```

### `ERESTART`
```rust
const ERESTART: c_int = 85i32;
```

### `ESTRPIPE`
```rust
const ESTRPIPE: c_int = 86i32;
```

### `EUSERS`
```rust
const EUSERS: c_int = 87i32;
```

### `ENOTSOCK`
```rust
const ENOTSOCK: c_int = 88i32;
```

### `EDESTADDRREQ`
```rust
const EDESTADDRREQ: c_int = 89i32;
```

### `EMSGSIZE`
```rust
const EMSGSIZE: c_int = 90i32;
```

### `EPROTOTYPE`
```rust
const EPROTOTYPE: c_int = 91i32;
```

### `ENOPROTOOPT`
```rust
const ENOPROTOOPT: c_int = 92i32;
```

### `EPROTONOSUPPORT`
```rust
const EPROTONOSUPPORT: c_int = 93i32;
```

### `ESOCKTNOSUPPORT`
```rust
const ESOCKTNOSUPPORT: c_int = 94i32;
```

### `EOPNOTSUPP`
```rust
const EOPNOTSUPP: c_int = 95i32;
```

### `EPFNOSUPPORT`
```rust
const EPFNOSUPPORT: c_int = 96i32;
```

### `EAFNOSUPPORT`
```rust
const EAFNOSUPPORT: c_int = 97i32;
```

### `EADDRINUSE`
```rust
const EADDRINUSE: c_int = 98i32;
```

### `EADDRNOTAVAIL`
```rust
const EADDRNOTAVAIL: c_int = 99i32;
```

### `ENETDOWN`
```rust
const ENETDOWN: c_int = 100i32;
```

### `ENETUNREACH`
```rust
const ENETUNREACH: c_int = 101i32;
```

### `ENETRESET`
```rust
const ENETRESET: c_int = 102i32;
```

### `ECONNABORTED`
```rust
const ECONNABORTED: c_int = 103i32;
```

### `ECONNRESET`
```rust
const ECONNRESET: c_int = 104i32;
```

### `ENOBUFS`
```rust
const ENOBUFS: c_int = 105i32;
```

### `EISCONN`
```rust
const EISCONN: c_int = 106i32;
```

### `ENOTCONN`
```rust
const ENOTCONN: c_int = 107i32;
```

### `ESHUTDOWN`
```rust
const ESHUTDOWN: c_int = 108i32;
```

### `ETOOMANYREFS`
```rust
const ETOOMANYREFS: c_int = 109i32;
```

### `ETIMEDOUT`
```rust
const ETIMEDOUT: c_int = 110i32;
```

### `ECONNREFUSED`
```rust
const ECONNREFUSED: c_int = 111i32;
```

### `EHOSTDOWN`
```rust
const EHOSTDOWN: c_int = 112i32;
```

### `EHOSTUNREACH`
```rust
const EHOSTUNREACH: c_int = 113i32;
```

### `EALREADY`
```rust
const EALREADY: c_int = 114i32;
```

### `EINPROGRESS`
```rust
const EINPROGRESS: c_int = 115i32;
```

### `ESTALE`
```rust
const ESTALE: c_int = 116i32;
```

### `EDQUOT`
```rust
const EDQUOT: c_int = 122i32;
```

### `ENOMEDIUM`
```rust
const ENOMEDIUM: c_int = 123i32;
```

### `EMEDIUMTYPE`
```rust
const EMEDIUMTYPE: c_int = 124i32;
```

### `ECANCELED`
```rust
const ECANCELED: c_int = 125i32;
```

### `ENOKEY`
```rust
const ENOKEY: c_int = 126i32;
```

### `EKEYEXPIRED`
```rust
const EKEYEXPIRED: c_int = 127i32;
```

### `EKEYREVOKED`
```rust
const EKEYREVOKED: c_int = 128i32;
```

### `EKEYREJECTED`
```rust
const EKEYREJECTED: c_int = 129i32;
```

### `EOWNERDEAD`
```rust
const EOWNERDEAD: c_int = 130i32;
```

### `ENOTRECOVERABLE`
```rust
const ENOTRECOVERABLE: c_int = 131i32;
```

### `EHWPOISON`
```rust
const EHWPOISON: c_int = 133i32;
```

### `ERFKILL`
```rust
const ERFKILL: c_int = 132i32;
```

### `SOCK_STREAM`
```rust
const SOCK_STREAM: c_int = 1i32;
```

### `SOCK_DGRAM`
```rust
const SOCK_DGRAM: c_int = 2i32;
```

### `SA_ONSTACK`
```rust
const SA_ONSTACK: c_int = 134_217_728i32;
```

### `SA_SIGINFO`
```rust
const SA_SIGINFO: c_int = 4i32;
```

### `SA_NOCLDWAIT`
```rust
const SA_NOCLDWAIT: c_int = 2i32;
```

### `SIGTTIN`
```rust
const SIGTTIN: c_int = 21i32;
```

### `SIGTTOU`
```rust
const SIGTTOU: c_int = 22i32;
```

### `SIGXCPU`
```rust
const SIGXCPU: c_int = 24i32;
```

### `SIGXFSZ`
```rust
const SIGXFSZ: c_int = 25i32;
```

### `SIGVTALRM`
```rust
const SIGVTALRM: c_int = 26i32;
```

### `SIGPROF`
```rust
const SIGPROF: c_int = 27i32;
```

### `SIGWINCH`
```rust
const SIGWINCH: c_int = 28i32;
```

### `SIGCHLD`
```rust
const SIGCHLD: c_int = 17i32;
```

### `SIGBUS`
```rust
const SIGBUS: c_int = 7i32;
```

### `SIGUSR1`
```rust
const SIGUSR1: c_int = 10i32;
```

### `SIGUSR2`
```rust
const SIGUSR2: c_int = 12i32;
```

### `SIGCONT`
```rust
const SIGCONT: c_int = 18i32;
```

### `SIGSTOP`
```rust
const SIGSTOP: c_int = 19i32;
```

### `SIGTSTP`
```rust
const SIGTSTP: c_int = 20i32;
```

### `SIGURG`
```rust
const SIGURG: c_int = 23i32;
```

### `SIGIO`
```rust
const SIGIO: c_int = 29i32;
```

### `SIGSYS`
```rust
const SIGSYS: c_int = 31i32;
```

### `SIGSTKFLT`
```rust
const SIGSTKFLT: c_int = 16i32;
```

### `SIGUNUSED`
```rust
const SIGUNUSED: c_int = 31i32;
```

### `SIGPOLL`
```rust
const SIGPOLL: c_int = 29i32;
```

### `SIGPWR`
```rust
const SIGPWR: c_int = 30i32;
```

### `SIG_SETMASK`
```rust
const SIG_SETMASK: c_int = 2i32;
```

### `SIG_BLOCK`
```rust
const SIG_BLOCK: c_int = 0i32;
```

### `SIG_UNBLOCK`
```rust
const SIG_UNBLOCK: c_int = 1i32;
```

### `POLLWRNORM`
```rust
const POLLWRNORM: c_short = 256i16;
```

### `POLLWRBAND`
```rust
const POLLWRBAND: c_short = 512i16;
```

### `O_ASYNC`
```rust
const O_ASYNC: c_int = 8_192i32;
```

### `O_NDELAY`
```rust
const O_NDELAY: c_int = 2_048i32;
```

### `PTRACE_DETACH`
```rust
const PTRACE_DETACH: c_uint = 17u32;
```

### `PTRACE_GET_RSEQ_CONFIGURATION`
```rust
const PTRACE_GET_RSEQ_CONFIGURATION: c_uint = 16_911u32;
```

### `EFD_NONBLOCK`
```rust
const EFD_NONBLOCK: c_int = 2_048i32;
```

### `F_GETLK`
```rust
const F_GETLK: c_int = 5i32;
```

### `F_GETOWN`
```rust
const F_GETOWN: c_int = 9i32;
```

### `F_SETOWN`
```rust
const F_SETOWN: c_int = 8i32;
```

### `F_SETLK`
```rust
const F_SETLK: c_int = 6i32;
```

### `F_SETLKW`
```rust
const F_SETLKW: c_int = 7i32;
```

### `F_OFD_GETLK`
```rust
const F_OFD_GETLK: c_int = 36i32;
```

### `F_OFD_SETLK`
```rust
const F_OFD_SETLK: c_int = 37i32;
```

### `F_OFD_SETLKW`
```rust
const F_OFD_SETLKW: c_int = 38i32;
```

### `F_RDLCK`
```rust
const F_RDLCK: c_int = 0i32;
```

### `F_WRLCK`
```rust
const F_WRLCK: c_int = 1i32;
```

### `F_UNLCK`
```rust
const F_UNLCK: c_int = 2i32;
```

### `SFD_NONBLOCK`
```rust
const SFD_NONBLOCK: c_int = 2_048i32;
```

### `TCSANOW`
```rust
const TCSANOW: c_int = 0i32;
```

### `TCSADRAIN`
```rust
const TCSADRAIN: c_int = 1i32;
```

### `TCSAFLUSH`
```rust
const TCSAFLUSH: c_int = 2i32;
```

### `SFD_CLOEXEC`
```rust
const SFD_CLOEXEC: c_int = 524_288i32;
```

### `NCCS`
```rust
const NCCS: usize = 32usize;
```

### `O_TRUNC`
```rust
const O_TRUNC: c_int = 512i32;
```

### `O_CLOEXEC`
```rust
const O_CLOEXEC: c_int = 524_288i32;
```

### `EBFONT`
```rust
const EBFONT: c_int = 59i32;
```

### `ENOSTR`
```rust
const ENOSTR: c_int = 60i32;
```

### `ENODATA`
```rust
const ENODATA: c_int = 61i32;
```

### `ETIME`
```rust
const ETIME: c_int = 62i32;
```

### `ENOSR`
```rust
const ENOSR: c_int = 63i32;
```

### `ENONET`
```rust
const ENONET: c_int = 64i32;
```

### `ENOPKG`
```rust
const ENOPKG: c_int = 65i32;
```

### `EREMOTE`
```rust
const EREMOTE: c_int = 66i32;
```

### `ENOLINK`
```rust
const ENOLINK: c_int = 67i32;
```

### `EADV`
```rust
const EADV: c_int = 68i32;
```

### `ESRMNT`
```rust
const ESRMNT: c_int = 69i32;
```

### `ECOMM`
```rust
const ECOMM: c_int = 70i32;
```

### `EPROTO`
```rust
const EPROTO: c_int = 71i32;
```

### `EDOTDOT`
```rust
const EDOTDOT: c_int = 73i32;
```

### `SA_NODEFER`
```rust
const SA_NODEFER: c_int = 1_073_741_824i32;
```

### `SA_RESETHAND`
```rust
const SA_RESETHAND: c_int = -2_147_483_648i32;
```

### `SA_RESTART`
```rust
const SA_RESTART: c_int = 268_435_456i32;
```

### `SA_NOCLDSTOP`
```rust
const SA_NOCLDSTOP: c_int = 1i32;
```

### `EPOLL_CLOEXEC`
```rust
const EPOLL_CLOEXEC: c_int = 524_288i32;
```

### `EFD_CLOEXEC`
```rust
const EFD_CLOEXEC: c_int = 524_288i32;
```

### `__SIZEOF_PTHREAD_CONDATTR_T`
```rust
const __SIZEOF_PTHREAD_CONDATTR_T: usize = 4usize;
```

### `__SIZEOF_PTHREAD_MUTEXATTR_T`
```rust
const __SIZEOF_PTHREAD_MUTEXATTR_T: usize = 4usize;
```

### `__SIZEOF_PTHREAD_BARRIERATTR_T`
```rust
const __SIZEOF_PTHREAD_BARRIERATTR_T: usize = 4usize;
```

### `O_DIRECT`
```rust
const O_DIRECT: c_int = 16_384i32;
```

### `O_DIRECTORY`
```rust
const O_DIRECTORY: c_int = 65_536i32;
```

### `O_NOFOLLOW`
```rust
const O_NOFOLLOW: c_int = 131_072i32;
```

### `MAP_HUGETLB`
```rust
const MAP_HUGETLB: c_int = 262_144i32;
```

### `MAP_LOCKED`
```rust
const MAP_LOCKED: c_int = 8_192i32;
```

### `MAP_NORESERVE`
```rust
const MAP_NORESERVE: c_int = 16_384i32;
```

### `MAP_32BIT`
```rust
const MAP_32BIT: c_int = 64i32;
```

### `MAP_ANON`
```rust
const MAP_ANON: c_int = 32i32;
```

### `MAP_ANONYMOUS`
```rust
const MAP_ANONYMOUS: c_int = 32i32;
```

### `MAP_DENYWRITE`
```rust
const MAP_DENYWRITE: c_int = 2_048i32;
```

### `MAP_EXECUTABLE`
```rust
const MAP_EXECUTABLE: c_int = 4_096i32;
```

### `MAP_POPULATE`
```rust
const MAP_POPULATE: c_int = 32_768i32;
```

### `MAP_NONBLOCK`
```rust
const MAP_NONBLOCK: c_int = 65_536i32;
```

### `MAP_STACK`
```rust
const MAP_STACK: c_int = 131_072i32;
```

### `MAP_SYNC`
```rust
const MAP_SYNC: c_int = 524_288i32;
```

### `EDEADLOCK`
```rust
const EDEADLOCK: c_int = 35i32;
```

### `EUCLEAN`
```rust
const EUCLEAN: c_int = 117i32;
```

### `ENOTNAM`
```rust
const ENOTNAM: c_int = 118i32;
```

### `ENAVAIL`
```rust
const ENAVAIL: c_int = 119i32;
```

### `EISNAM`
```rust
const EISNAM: c_int = 120i32;
```

### `EREMOTEIO`
```rust
const EREMOTEIO: c_int = 121i32;
```

### `PTRACE_GETFPREGS`
```rust
const PTRACE_GETFPREGS: c_uint = 14u32;
```

### `PTRACE_SETFPREGS`
```rust
const PTRACE_SETFPREGS: c_uint = 15u32;
```

### `PTRACE_GETFPXREGS`
```rust
const PTRACE_GETFPXREGS: c_uint = 18u32;
```

### `PTRACE_SETFPXREGS`
```rust
const PTRACE_SETFPXREGS: c_uint = 19u32;
```

### `PTRACE_GETREGS`
```rust
const PTRACE_GETREGS: c_uint = 12u32;
```

### `PTRACE_SETREGS`
```rust
const PTRACE_SETREGS: c_uint = 13u32;
```

### `PTRACE_PEEKSIGINFO_SHARED`
```rust
const PTRACE_PEEKSIGINFO_SHARED: c_uint = 1u32;
```

### `PTRACE_SYSEMU`
```rust
const PTRACE_SYSEMU: c_uint = 31u32;
```

### `PTRACE_SYSEMU_SINGLESTEP`
```rust
const PTRACE_SYSEMU_SINGLESTEP: c_uint = 32u32;
```

### `PR_GET_SPECULATION_CTRL`
```rust
const PR_GET_SPECULATION_CTRL: c_int = 52i32;
```

### `PR_SET_SPECULATION_CTRL`
```rust
const PR_SET_SPECULATION_CTRL: c_int = 53i32;
```

### `PR_SPEC_NOT_AFFECTED`
```rust
const PR_SPEC_NOT_AFFECTED: c_uint = 0u32;
```

### `PR_SPEC_PRCTL`
```rust
const PR_SPEC_PRCTL: c_uint = 1u32;
```

### `PR_SPEC_ENABLE`
```rust
const PR_SPEC_ENABLE: c_uint = 2u32;
```

### `PR_SPEC_DISABLE`
```rust
const PR_SPEC_DISABLE: c_uint = 4u32;
```

### `PR_SPEC_FORCE_DISABLE`
```rust
const PR_SPEC_FORCE_DISABLE: c_uint = 8u32;
```

### `PR_SPEC_DISABLE_NOEXEC`
```rust
const PR_SPEC_DISABLE_NOEXEC: c_uint = 16u32;
```

### `PR_SPEC_STORE_BYPASS`
```rust
const PR_SPEC_STORE_BYPASS: c_int = 0i32;
```

### `PR_SPEC_INDIRECT_BRANCH`
```rust
const PR_SPEC_INDIRECT_BRANCH: c_int = 1i32;
```

### `MCL_CURRENT`
```rust
const MCL_CURRENT: c_int = 1i32;
```

### `MCL_FUTURE`
```rust
const MCL_FUTURE: c_int = 2i32;
```

### `MCL_ONFAULT`
```rust
const MCL_ONFAULT: c_int = 4i32;
```

### `SIGSTKSZ`
```rust
const SIGSTKSZ: size_t = 8_192usize;
```

### `MINSIGSTKSZ`
```rust
const MINSIGSTKSZ: size_t = 2_048usize;
```

### `CBAUD`
```rust
const CBAUD: crate::tcflag_t = 4_111u32;
```

### `TAB1`
```rust
const TAB1: crate::tcflag_t = 2_048u32;
```

### `TAB2`
```rust
const TAB2: crate::tcflag_t = 4_096u32;
```

### `TAB3`
```rust
const TAB3: crate::tcflag_t = 6_144u32;
```

### `CR1`
```rust
const CR1: crate::tcflag_t = 512u32;
```

### `CR2`
```rust
const CR2: crate::tcflag_t = 1_024u32;
```

### `CR3`
```rust
const CR3: crate::tcflag_t = 1_536u32;
```

### `FF1`
```rust
const FF1: crate::tcflag_t = 32_768u32;
```

### `BS1`
```rust
const BS1: crate::tcflag_t = 8_192u32;
```

### `VT1`
```rust
const VT1: crate::tcflag_t = 16_384u32;
```

### `VWERASE`
```rust
const VWERASE: usize = 14usize;
```

### `VREPRINT`
```rust
const VREPRINT: usize = 12usize;
```

### `VSUSP`
```rust
const VSUSP: usize = 10usize;
```

### `VSTART`
```rust
const VSTART: usize = 8usize;
```

### `VSTOP`
```rust
const VSTOP: usize = 9usize;
```

### `VDISCARD`
```rust
const VDISCARD: usize = 13usize;
```

### `VTIME`
```rust
const VTIME: usize = 5usize;
```

### `IXON`
```rust
const IXON: crate::tcflag_t = 1_024u32;
```

### `IXOFF`
```rust
const IXOFF: crate::tcflag_t = 4_096u32;
```

### `ONLCR`
```rust
const ONLCR: crate::tcflag_t = 4u32;
```

### `CSIZE`
```rust
const CSIZE: crate::tcflag_t = 48u32;
```

### `CS6`
```rust
const CS6: crate::tcflag_t = 16u32;
```

### `CS7`
```rust
const CS7: crate::tcflag_t = 32u32;
```

### `CS8`
```rust
const CS8: crate::tcflag_t = 48u32;
```

### `CSTOPB`
```rust
const CSTOPB: crate::tcflag_t = 64u32;
```

### `CREAD`
```rust
const CREAD: crate::tcflag_t = 128u32;
```

### `PARENB`
```rust
const PARENB: crate::tcflag_t = 256u32;
```

### `PARODD`
```rust
const PARODD: crate::tcflag_t = 512u32;
```

### `HUPCL`
```rust
const HUPCL: crate::tcflag_t = 1_024u32;
```

### `CLOCAL`
```rust
const CLOCAL: crate::tcflag_t = 2_048u32;
```

### `ECHOKE`
```rust
const ECHOKE: crate::tcflag_t = 2_048u32;
```

### `ECHOE`
```rust
const ECHOE: crate::tcflag_t = 16u32;
```

### `ECHOK`
```rust
const ECHOK: crate::tcflag_t = 32u32;
```

### `ECHONL`
```rust
const ECHONL: crate::tcflag_t = 64u32;
```

### `ECHOPRT`
```rust
const ECHOPRT: crate::tcflag_t = 1_024u32;
```

### `ECHOCTL`
```rust
const ECHOCTL: crate::tcflag_t = 512u32;
```

### `ISIG`
```rust
const ISIG: crate::tcflag_t = 1u32;
```

### `ICANON`
```rust
const ICANON: crate::tcflag_t = 2u32;
```

### `PENDIN`
```rust
const PENDIN: crate::tcflag_t = 16_384u32;
```

### `NOFLSH`
```rust
const NOFLSH: crate::tcflag_t = 128u32;
```

### `CIBAUD`
```rust
const CIBAUD: crate::tcflag_t = 269_418_496u32;
```

### `CBAUDEX`
```rust
const CBAUDEX: crate::tcflag_t = 4_096u32;
```

### `VSWTC`
```rust
const VSWTC: usize = 7usize;
```

### `OLCUC`
```rust
const OLCUC: crate::tcflag_t = 2u32;
```

### `NLDLY`
```rust
const NLDLY: crate::tcflag_t = 256u32;
```

### `CRDLY`
```rust
const CRDLY: crate::tcflag_t = 1_536u32;
```

### `TABDLY`
```rust
const TABDLY: crate::tcflag_t = 6_144u32;
```

### `BSDLY`
```rust
const BSDLY: crate::tcflag_t = 8_192u32;
```

### `FFDLY`
```rust
const FFDLY: crate::tcflag_t = 32_768u32;
```

### `VTDLY`
```rust
const VTDLY: crate::tcflag_t = 16_384u32;
```

### `XTABS`
```rust
const XTABS: crate::tcflag_t = 6_144u32;
```

### `B0`
```rust
const B0: crate::speed_t = 0u32;
```

### `B50`
```rust
const B50: crate::speed_t = 1u32;
```

### `B75`
```rust
const B75: crate::speed_t = 2u32;
```

### `B110`
```rust
const B110: crate::speed_t = 3u32;
```

### `B134`
```rust
const B134: crate::speed_t = 4u32;
```

### `B150`
```rust
const B150: crate::speed_t = 5u32;
```

### `B200`
```rust
const B200: crate::speed_t = 6u32;
```

### `B300`
```rust
const B300: crate::speed_t = 7u32;
```

### `B600`
```rust
const B600: crate::speed_t = 8u32;
```

### `B1200`
```rust
const B1200: crate::speed_t = 9u32;
```

### `B1800`
```rust
const B1800: crate::speed_t = 10u32;
```

### `B2400`
```rust
const B2400: crate::speed_t = 11u32;
```

### `B4800`
```rust
const B4800: crate::speed_t = 12u32;
```

### `B9600`
```rust
const B9600: crate::speed_t = 13u32;
```

### `B19200`
```rust
const B19200: crate::speed_t = 14u32;
```

### `B38400`
```rust
const B38400: crate::speed_t = 15u32;
```

### `EXTA`
```rust
const EXTA: crate::speed_t = 14u32;
```

### `EXTB`
```rust
const EXTB: crate::speed_t = 15u32;
```

### `B57600`
```rust
const B57600: crate::speed_t = 4_097u32;
```

### `B115200`
```rust
const B115200: crate::speed_t = 4_098u32;
```

### `B230400`
```rust
const B230400: crate::speed_t = 4_099u32;
```

### `B460800`
```rust
const B460800: crate::speed_t = 4_100u32;
```

### `B500000`
```rust
const B500000: crate::speed_t = 4_101u32;
```

### `B576000`
```rust
const B576000: crate::speed_t = 4_102u32;
```

### `B921600`
```rust
const B921600: crate::speed_t = 4_103u32;
```

### `B1000000`
```rust
const B1000000: crate::speed_t = 4_104u32;
```

### `B1152000`
```rust
const B1152000: crate::speed_t = 4_105u32;
```

### `B1500000`
```rust
const B1500000: crate::speed_t = 4_106u32;
```

### `B2000000`
```rust
const B2000000: crate::speed_t = 4_107u32;
```

### `B2500000`
```rust
const B2500000: crate::speed_t = 4_108u32;
```

### `B3000000`
```rust
const B3000000: crate::speed_t = 4_109u32;
```

### `B3500000`
```rust
const B3500000: crate::speed_t = 4_110u32;
```

### `B4000000`
```rust
const B4000000: crate::speed_t = 4_111u32;
```

### `VEOL`
```rust
const VEOL: usize = 11usize;
```

### `VEOL2`
```rust
const VEOL2: usize = 16usize;
```

### `VMIN`
```rust
const VMIN: usize = 6usize;
```

### `IEXTEN`
```rust
const IEXTEN: crate::tcflag_t = 32_768u32;
```

### `TOSTOP`
```rust
const TOSTOP: crate::tcflag_t = 256u32;
```

### `FLUSHO`
```rust
const FLUSHO: crate::tcflag_t = 4_096u32;
```

### `EXTPROC`
```rust
const EXTPROC: crate::tcflag_t = 65_536u32;
```

### `R15`
```rust
const R15: c_int = 0i32;
```

### `R14`
```rust
const R14: c_int = 1i32;
```

### `R13`
```rust
const R13: c_int = 2i32;
```

### `R12`
```rust
const R12: c_int = 3i32;
```

### `RBP`
```rust
const RBP: c_int = 4i32;
```

### `RBX`
```rust
const RBX: c_int = 5i32;
```

### `R11`
```rust
const R11: c_int = 6i32;
```

### `R10`
```rust
const R10: c_int = 7i32;
```

### `R9`
```rust
const R9: c_int = 8i32;
```

### `R8`
```rust
const R8: c_int = 9i32;
```

### `RAX`
```rust
const RAX: c_int = 10i32;
```

### `RCX`
```rust
const RCX: c_int = 11i32;
```

### `RDX`
```rust
const RDX: c_int = 12i32;
```

### `RSI`
```rust
const RSI: c_int = 13i32;
```

### `RDI`
```rust
const RDI: c_int = 14i32;
```

### `ORIG_RAX`
```rust
const ORIG_RAX: c_int = 15i32;
```

### `RIP`
```rust
const RIP: c_int = 16i32;
```

### `CS`
```rust
const CS: c_int = 17i32;
```

### `EFLAGS`
```rust
const EFLAGS: c_int = 18i32;
```

### `RSP`
```rust
const RSP: c_int = 19i32;
```

### `SS`
```rust
const SS: c_int = 20i32;
```

### `FS_BASE`
```rust
const FS_BASE: c_int = 21i32;
```

### `GS_BASE`
```rust
const GS_BASE: c_int = 22i32;
```

### `DS`
```rust
const DS: c_int = 23i32;
```

### `ES`
```rust
const ES: c_int = 24i32;
```

### `FS`
```rust
const FS: c_int = 25i32;
```

### `GS`
```rust
const GS: c_int = 26i32;
```

### `REG_R8`
```rust
const REG_R8: c_int = 0i32;
```

### `REG_R9`
```rust
const REG_R9: c_int = 1i32;
```

### `REG_R10`
```rust
const REG_R10: c_int = 2i32;
```

### `REG_R11`
```rust
const REG_R11: c_int = 3i32;
```

### `REG_R12`
```rust
const REG_R12: c_int = 4i32;
```

### `REG_R13`
```rust
const REG_R13: c_int = 5i32;
```

### `REG_R14`
```rust
const REG_R14: c_int = 6i32;
```

### `REG_R15`
```rust
const REG_R15: c_int = 7i32;
```

### `REG_RDI`
```rust
const REG_RDI: c_int = 8i32;
```

### `REG_RSI`
```rust
const REG_RSI: c_int = 9i32;
```

### `REG_RBP`
```rust
const REG_RBP: c_int = 10i32;
```

### `REG_RBX`
```rust
const REG_RBX: c_int = 11i32;
```

### `REG_RDX`
```rust
const REG_RDX: c_int = 12i32;
```

### `REG_RAX`
```rust
const REG_RAX: c_int = 13i32;
```

### `REG_RCX`
```rust
const REG_RCX: c_int = 14i32;
```

### `REG_RSP`
```rust
const REG_RSP: c_int = 15i32;
```

### `REG_RIP`
```rust
const REG_RIP: c_int = 16i32;
```

### `REG_EFL`
```rust
const REG_EFL: c_int = 17i32;
```

### `REG_CSGSFS`
```rust
const REG_CSGSFS: c_int = 18i32;
```

### `REG_ERR`
```rust
const REG_ERR: c_int = 19i32;
```

### `REG_TRAPNO`
```rust
const REG_TRAPNO: c_int = 20i32;
```

### `REG_OLDMASK`
```rust
const REG_OLDMASK: c_int = 21i32;
```

### `REG_CR2`
```rust
const REG_CR2: c_int = 22i32;
```

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

