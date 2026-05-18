*[libc](../index.md) / [unix](index.md)*

---

# Module `unix`

Definitions found commonly among almost all Unix derivatives

More functions and definitions can be found in the more specific modules
according to the platform in question.

## Contents

- [Modules](#modules)
  - [`linux_like`](#linux-like)
  - [`linux`](#linux)
  - [`linux_l4re_shared`](#linux-l4re-shared)
- [Structs](#structs)
  - [`group`](#group)
  - [`utimbuf`](#utimbuf)
  - [`timeval`](#timeval)
  - [`rlimit`](#rlimit)
  - [`rusage`](#rusage)
  - [`ipv6_mreq`](#ipv6-mreq)
  - [`hostent`](#hostent)
  - [`iovec`](#iovec)
  - [`pollfd`](#pollfd)
  - [`winsize`](#winsize)
  - [`linger`](#linger)
  - [`sigval`](#sigval)
  - [`itimerval`](#itimerval)
  - [`tms`](#tms)
  - [`servent`](#servent)
  - [`protoent`](#protoent)
  - [`in6_addr`](#in6-addr)
  - [`in_addr`](#in-addr)
  - [`ip_mreq`](#ip-mreq)
  - [`ip_mreqn`](#ip-mreqn)
  - [`ip_mreq_source`](#ip-mreq-source)
  - [`sockaddr`](#sockaddr)
  - [`sockaddr_in`](#sockaddr-in)
  - [`sockaddr_in6`](#sockaddr-in6)
  - [`addrinfo`](#addrinfo)
  - [`sockaddr_ll`](#sockaddr-ll)
  - [`fd_set`](#fd-set)
  - [`tm`](#tm)
  - [`sched_param`](#sched-param)
  - [`Dl_info`](#dl-info)
  - [`lconv`](#lconv)
  - [`in_pktinfo`](#in-pktinfo)
  - [`ifaddrs`](#ifaddrs)
  - [`in6_rtmsg`](#in6-rtmsg)
  - [`arpreq`](#arpreq)
  - [`arpreq_old`](#arpreq-old)
  - [`arphdr`](#arphdr)
  - [`mmsghdr`](#mmsghdr)
  - [`sockaddr_un`](#sockaddr-un)
  - [`sockaddr_storage`](#sockaddr-storage)
  - [`utsname`](#utsname)
  - [`if_nameindex`](#if-nameindex)
  - [`file_clone_range`](#file-clone-range)
  - [`sock_filter`](#sock-filter)
  - [`sock_fprog`](#sock-fprog)
  - [`statx`](#statx)
  - [`statx_timestamp`](#statx-timestamp)
  - [`epoll_event`](#epoll-event)
  - [`sigevent`](#sigevent)
- [Enums](#enums)
  - [`DIR`](#dir)
  - [`FILE`](#file)
  - [`timezone`](#timezone)
- [Functions](#functions)
  - [`isalnum`](#isalnum)
  - [`isalpha`](#isalpha)
  - [`iscntrl`](#iscntrl)
  - [`isdigit`](#isdigit)
  - [`isgraph`](#isgraph)
  - [`islower`](#islower)
  - [`isprint`](#isprint)
  - [`ispunct`](#ispunct)
  - [`isspace`](#isspace)
  - [`isupper`](#isupper)
  - [`isxdigit`](#isxdigit)
  - [`isblank`](#isblank)
  - [`tolower`](#tolower)
  - [`toupper`](#toupper)
  - [`qsort`](#qsort)
  - [`bsearch`](#bsearch)
  - [`fopen`](#fopen)
  - [`freopen`](#freopen)
  - [`fflush`](#fflush)
  - [`fclose`](#fclose)
  - [`remove`](#remove)
  - [`rename`](#rename)
  - [`tmpfile`](#tmpfile)
  - [`setvbuf`](#setvbuf)
  - [`setbuf`](#setbuf)
  - [`getchar`](#getchar)
  - [`putchar`](#putchar)
  - [`fgetc`](#fgetc)
  - [`fgets`](#fgets)
  - [`fputc`](#fputc)
  - [`fputs`](#fputs)
  - [`puts`](#puts)
  - [`ungetc`](#ungetc)
  - [`fread`](#fread)
  - [`fwrite`](#fwrite)
  - [`fseek`](#fseek)
  - [`ftell`](#ftell)
  - [`rewind`](#rewind)
  - [`fgetpos`](#fgetpos)
  - [`fsetpos`](#fsetpos)
  - [`feof`](#feof)
  - [`ferror`](#ferror)
  - [`clearerr`](#clearerr)
  - [`perror`](#perror)
  - [`atof`](#atof)
  - [`atoi`](#atoi)
  - [`atol`](#atol)
  - [`atoll`](#atoll)
  - [`strtod`](#strtod)
  - [`strtof`](#strtof)
  - [`strtol`](#strtol)
  - [`strtoll`](#strtoll)
  - [`strtoul`](#strtoul)
  - [`strtoull`](#strtoull)
  - [`calloc`](#calloc)
  - [`malloc`](#malloc)
  - [`realloc`](#realloc)
  - [`free`](#free)
  - [`abort`](#abort)
  - [`exit`](#exit)
  - [`_exit`](#exit)
  - [`system`](#system)
  - [`getenv`](#getenv)
  - [`strcpy`](#strcpy)
  - [`strncpy`](#strncpy)
  - [`stpcpy`](#stpcpy)
  - [`strcat`](#strcat)
  - [`strncat`](#strncat)
  - [`strcmp`](#strcmp)
  - [`strncmp`](#strncmp)
  - [`strcoll`](#strcoll)
  - [`strchr`](#strchr)
  - [`strrchr`](#strrchr)
  - [`strspn`](#strspn)
  - [`strcspn`](#strcspn)
  - [`strdup`](#strdup)
  - [`strndup`](#strndup)
  - [`strpbrk`](#strpbrk)
  - [`strstr`](#strstr)
  - [`strcasecmp`](#strcasecmp)
  - [`strncasecmp`](#strncasecmp)
  - [`strlen`](#strlen)
  - [`strnlen`](#strnlen)
  - [`strerror`](#strerror)
  - [`strtok`](#strtok)
  - [`strtok_r`](#strtok-r)
  - [`strxfrm`](#strxfrm)
  - [`strsignal`](#strsignal)
  - [`wcslen`](#wcslen)
  - [`wcstombs`](#wcstombs)
  - [`memchr`](#memchr)
  - [`wmemchr`](#wmemchr)
  - [`memcmp`](#memcmp)
  - [`memcpy`](#memcpy)
  - [`memmove`](#memmove)
  - [`memset`](#memset)
  - [`memccpy`](#memccpy)
  - [`getpwnam`](#getpwnam)
  - [`getpwuid`](#getpwuid)
  - [`fprintf`](#fprintf)
  - [`printf`](#printf)
  - [`snprintf`](#snprintf)
  - [`sprintf`](#sprintf)
  - [`fscanf`](#fscanf)
  - [`scanf`](#scanf)
  - [`sscanf`](#sscanf)
  - [`getchar_unlocked`](#getchar-unlocked)
  - [`putchar_unlocked`](#putchar-unlocked)
  - [`socket`](#socket)
  - [`connect`](#connect)
  - [`listen`](#listen)
  - [`accept`](#accept)
  - [`getpeername`](#getpeername)
  - [`getsockname`](#getsockname)
  - [`setsockopt`](#setsockopt)
  - [`socketpair`](#socketpair)
  - [`sendto`](#sendto)
  - [`shutdown`](#shutdown)
  - [`chmod`](#chmod)
  - [`fchmod`](#fchmod)
  - [`fstat`](#fstat)
  - [`mkdir`](#mkdir)
  - [`stat`](#stat)
  - [`pclose`](#pclose)
  - [`fdopen`](#fdopen)
  - [`fileno`](#fileno)
  - [`open`](#open)
  - [`creat`](#creat)
  - [`fcntl`](#fcntl)
  - [`opendir`](#opendir)
  - [`readdir`](#readdir)
  - [`closedir`](#closedir)
  - [`rewinddir`](#rewinddir)
  - [`fchmodat`](#fchmodat)
  - [`fchown`](#fchown)
  - [`fchownat`](#fchownat)
  - [`fstatat`](#fstatat)
  - [`linkat`](#linkat)
  - [`renameat`](#renameat)
  - [`symlinkat`](#symlinkat)
  - [`unlinkat`](#unlinkat)
  - [`access`](#access)
  - [`alarm`](#alarm)
  - [`chdir`](#chdir)
  - [`fchdir`](#fchdir)
  - [`chown`](#chown)
  - [`lchown`](#lchown)
  - [`close`](#close)
  - [`dup`](#dup)
  - [`dup2`](#dup2)
  - [`execl`](#execl)
  - [`execle`](#execle)
  - [`execlp`](#execlp)
  - [`execv`](#execv)
  - [`execve`](#execve)
  - [`execvp`](#execvp)
  - [`fork`](#fork)
  - [`fpathconf`](#fpathconf)
  - [`getcwd`](#getcwd)
  - [`getegid`](#getegid)
  - [`geteuid`](#geteuid)
  - [`getgid`](#getgid)
  - [`getgroups`](#getgroups)
  - [`getlogin`](#getlogin)
  - [`getopt`](#getopt)
  - [`getpgid`](#getpgid)
  - [`getpgrp`](#getpgrp)
  - [`getpid`](#getpid)
  - [`getppid`](#getppid)
  - [`getuid`](#getuid)
  - [`isatty`](#isatty)
  - [`link`](#link)
  - [`lseek`](#lseek)
  - [`pathconf`](#pathconf)
  - [`pipe`](#pipe)
  - [`posix_memalign`](#posix-memalign)
  - [`aligned_alloc`](#aligned-alloc)
  - [`read`](#read)
  - [`rmdir`](#rmdir)
  - [`seteuid`](#seteuid)
  - [`setegid`](#setegid)
  - [`setgid`](#setgid)
  - [`setpgid`](#setpgid)
  - [`setsid`](#setsid)
  - [`setuid`](#setuid)
  - [`setreuid`](#setreuid)
  - [`setregid`](#setregid)
  - [`sleep`](#sleep)
  - [`nanosleep`](#nanosleep)
  - [`tcgetpgrp`](#tcgetpgrp)
  - [`tcsetpgrp`](#tcsetpgrp)
  - [`ttyname`](#ttyname)
  - [`ttyname_r`](#ttyname-r)
  - [`unlink`](#unlink)
  - [`wait`](#wait)
  - [`waitpid`](#waitpid)
  - [`write`](#write)
  - [`pread`](#pread)
  - [`pwrite`](#pwrite)
  - [`umask`](#umask)
  - [`utime`](#utime)
  - [`kill`](#kill)
  - [`killpg`](#killpg)
  - [`mlock`](#mlock)
  - [`munlock`](#munlock)
  - [`mlockall`](#mlockall)
  - [`munlockall`](#munlockall)
  - [`mmap`](#mmap)
  - [`munmap`](#munmap)
  - [`if_nametoindex`](#if-nametoindex)
  - [`if_indextoname`](#if-indextoname)
  - [`lstat`](#lstat)
  - [`fsync`](#fsync)
  - [`setenv`](#setenv)
  - [`unsetenv`](#unsetenv)
  - [`symlink`](#symlink)
  - [`truncate`](#truncate)
  - [`ftruncate`](#ftruncate)
  - [`signal`](#signal)
  - [`getrusage`](#getrusage)
  - [`realpath`](#realpath)
  - [`times`](#times)
  - [`pthread_self`](#pthread-self)
  - [`pthread_equal`](#pthread-equal)
  - [`pthread_join`](#pthread-join)
  - [`pthread_exit`](#pthread-exit)
  - [`pthread_attr_init`](#pthread-attr-init)
  - [`pthread_attr_destroy`](#pthread-attr-destroy)
  - [`pthread_attr_getstacksize`](#pthread-attr-getstacksize)
  - [`pthread_attr_setstacksize`](#pthread-attr-setstacksize)
  - [`pthread_attr_setdetachstate`](#pthread-attr-setdetachstate)
  - [`pthread_detach`](#pthread-detach)
  - [`sched_yield`](#sched-yield)
  - [`pthread_key_create`](#pthread-key-create)
  - [`pthread_key_delete`](#pthread-key-delete)
  - [`pthread_getspecific`](#pthread-getspecific)
  - [`pthread_setspecific`](#pthread-setspecific)
  - [`pthread_mutex_init`](#pthread-mutex-init)
  - [`pthread_mutex_destroy`](#pthread-mutex-destroy)
  - [`pthread_mutex_lock`](#pthread-mutex-lock)
  - [`pthread_mutex_trylock`](#pthread-mutex-trylock)
  - [`pthread_mutex_unlock`](#pthread-mutex-unlock)
  - [`pthread_mutexattr_init`](#pthread-mutexattr-init)
  - [`pthread_mutexattr_destroy`](#pthread-mutexattr-destroy)
  - [`pthread_mutexattr_settype`](#pthread-mutexattr-settype)
  - [`pthread_cond_init`](#pthread-cond-init)
  - [`pthread_cond_wait`](#pthread-cond-wait)
  - [`pthread_cond_timedwait`](#pthread-cond-timedwait)
  - [`pthread_cond_signal`](#pthread-cond-signal)
  - [`pthread_cond_broadcast`](#pthread-cond-broadcast)
  - [`pthread_cond_destroy`](#pthread-cond-destroy)
  - [`pthread_condattr_init`](#pthread-condattr-init)
  - [`pthread_condattr_destroy`](#pthread-condattr-destroy)
  - [`pthread_rwlock_init`](#pthread-rwlock-init)
  - [`pthread_rwlock_destroy`](#pthread-rwlock-destroy)
  - [`pthread_rwlock_rdlock`](#pthread-rwlock-rdlock)
  - [`pthread_rwlock_tryrdlock`](#pthread-rwlock-tryrdlock)
  - [`pthread_rwlock_wrlock`](#pthread-rwlock-wrlock)
  - [`pthread_rwlock_trywrlock`](#pthread-rwlock-trywrlock)
  - [`pthread_rwlock_unlock`](#pthread-rwlock-unlock)
  - [`pthread_rwlockattr_init`](#pthread-rwlockattr-init)
  - [`pthread_rwlockattr_destroy`](#pthread-rwlockattr-destroy)
  - [`getsockopt`](#getsockopt)
  - [`raise`](#raise)
  - [`utimes`](#utimes)
  - [`dlopen`](#dlopen)
  - [`dlerror`](#dlerror)
  - [`dlsym`](#dlsym)
  - [`dlclose`](#dlclose)
  - [`getaddrinfo`](#getaddrinfo)
  - [`freeaddrinfo`](#freeaddrinfo)
  - [`hstrerror`](#hstrerror)
  - [`gai_strerror`](#gai-strerror)
  - [`res_init`](#res-init)
  - [`gmtime_r`](#gmtime-r)
  - [`localtime_r`](#localtime-r)
  - [`mktime`](#mktime)
  - [`time`](#time)
  - [`gmtime`](#gmtime)
  - [`localtime`](#localtime)
  - [`difftime`](#difftime)
  - [`timegm`](#timegm)
  - [`mknod`](#mknod)
  - [`gethostname`](#gethostname)
  - [`endservent`](#endservent)
  - [`getservbyname`](#getservbyname)
  - [`getservbyport`](#getservbyport)
  - [`getservent`](#getservent)
  - [`setservent`](#setservent)
  - [`getprotobyname`](#getprotobyname)
  - [`getprotobynumber`](#getprotobynumber)
  - [`chroot`](#chroot)
  - [`usleep`](#usleep)
  - [`send`](#send)
  - [`recv`](#recv)
  - [`putenv`](#putenv)
  - [`poll`](#poll)
  - [`select`](#select)
  - [`setlocale`](#setlocale)
  - [`localeconv`](#localeconv)
  - [`sem_wait`](#sem-wait)
  - [`sem_trywait`](#sem-trywait)
  - [`sem_post`](#sem-post)
  - [`statvfs`](#statvfs)
  - [`fstatvfs`](#fstatvfs)
  - [`sigemptyset`](#sigemptyset)
  - [`sigaddset`](#sigaddset)
  - [`sigfillset`](#sigfillset)
  - [`sigdelset`](#sigdelset)
  - [`sigismember`](#sigismember)
  - [`sigprocmask`](#sigprocmask)
  - [`sigpending`](#sigpending)
  - [`sysconf`](#sysconf)
  - [`mkfifo`](#mkfifo)
  - [`fseeko`](#fseeko)
  - [`ftello`](#ftello)
  - [`tcdrain`](#tcdrain)
  - [`cfgetispeed`](#cfgetispeed)
  - [`cfgetospeed`](#cfgetospeed)
  - [`cfsetispeed`](#cfsetispeed)
  - [`cfsetospeed`](#cfsetospeed)
  - [`tcgetattr`](#tcgetattr)
  - [`tcsetattr`](#tcsetattr)
  - [`tcflow`](#tcflow)
  - [`tcflush`](#tcflush)
  - [`tcgetsid`](#tcgetsid)
  - [`tcsendbreak`](#tcsendbreak)
  - [`mkstemp`](#mkstemp)
  - [`mkdtemp`](#mkdtemp)
  - [`tmpnam`](#tmpnam)
  - [`openlog`](#openlog)
  - [`closelog`](#closelog)
  - [`setlogmask`](#setlogmask)
  - [`syslog`](#syslog)
  - [`nice`](#nice)
  - [`grantpt`](#grantpt)
  - [`posix_openpt`](#posix-openpt)
  - [`ptsname`](#ptsname)
  - [`unlockpt`](#unlockpt)
  - [`strcasestr`](#strcasestr)
  - [`getline`](#getline)
  - [`lockf`](#lockf)
  - [`adjtime`](#adjtime)
  - [`stpncpy`](#stpncpy)
  - [`sigqueue`](#sigqueue)
  - [`confstr`](#confstr)
  - [`dladdr`](#dladdr)
  - [`flock`](#flock)
  - [`open_wmemstream`](#open-wmemstream)
  - [`getsid`](#getsid)
  - [`pause`](#pause)
  - [`mkdirat`](#mkdirat)
  - [`openat`](#openat)
  - [`fdopendir`](#fdopendir)
  - [`readdir_r`](#readdir-r)
  - [`readlinkat`](#readlinkat)
  - [`fmemopen`](#fmemopen)
  - [`open_memstream`](#open-memstream)
  - [`atexit`](#atexit)
  - [`sigaction`](#sigaction)
  - [`readlink`](#readlink)
  - [`pselect`](#pselect)
  - [`cfmakeraw`](#cfmakeraw)
  - [`cfsetspeed`](#cfsetspeed)
  - [`fnmatch`](#fnmatch)
  - [`htonl`](#htonl)
  - [`htons`](#htons)
  - [`ntohl`](#ntohl)
  - [`ntohs`](#ntohs)
  - [`ioctl`](#ioctl)
  - [`sem_destroy`](#sem-destroy)
  - [`sem_init`](#sem-init)
  - [`fdatasync`](#fdatasync)
  - [`mincore`](#mincore)
  - [`clock_getres`](#clock-getres)
  - [`clock_gettime`](#clock-gettime)
  - [`clock_settime`](#clock-settime)
  - [`clock_getcpuclockid`](#clock-getcpuclockid)
  - [`getitimer`](#getitimer)
  - [`setitimer`](#setitimer)
  - [`dirfd`](#dirfd)
  - [`memalign`](#memalign)
  - [`setgroups`](#setgroups)
  - [`pipe2`](#pipe2)
  - [`statfs`](#statfs)
  - [`fstatfs`](#fstatfs)
  - [`memrchr`](#memrchr)
  - [`posix_fadvise`](#posix-fadvise)
  - [`futimens`](#futimens)
  - [`utimensat`](#utimensat)
  - [`duplocale`](#duplocale)
  - [`freelocale`](#freelocale)
  - [`newlocale`](#newlocale)
  - [`uselocale`](#uselocale)
  - [`mknodat`](#mknodat)
  - [`ptsname_r`](#ptsname-r)
  - [`clearenv`](#clearenv)
  - [`waitid`](#waitid)
  - [`getresuid`](#getresuid)
  - [`getresgid`](#getresgid)
  - [`acct`](#acct)
  - [`brk`](#brk)
  - [`sbrk`](#sbrk)
  - [`vfork`](#vfork)
  - [`setresgid`](#setresgid)
  - [`setresuid`](#setresuid)
  - [`wait4`](#wait4)
  - [`login_tty`](#login-tty)
  - [`execvpe`](#execvpe)
  - [`fexecve`](#fexecve)
  - [`getifaddrs`](#getifaddrs)
  - [`freeifaddrs`](#freeifaddrs)
  - [`bind`](#bind)
  - [`writev`](#writev)
  - [`readv`](#readv)
  - [`sendmsg`](#sendmsg)
  - [`recvmsg`](#recvmsg)
  - [`uname`](#uname)
  - [`strchrnul`](#strchrnul)
  - [`strftime`](#strftime)
  - [`strftime_l`](#strftime-l)
  - [`strptime`](#strptime)
  - [`mkostemp`](#mkostemp)
  - [`mkostemps`](#mkostemps)
  - [`getdomainname`](#getdomainname)
  - [`setdomainname`](#setdomainname)
  - [`if_nameindex`](#if-nameindex)
  - [`if_freenameindex`](#if-freenameindex)
  - [`getpwnam_r`](#getpwnam-r)
  - [`getpwuid_r`](#getpwuid-r)
  - [`fstatfs64`](#fstatfs64)
  - [`statvfs64`](#statvfs64)
  - [`fstatvfs64`](#fstatvfs64)
  - [`statfs64`](#statfs64)
  - [`creat64`](#creat64)
  - [`fstat64`](#fstat64)
  - [`fstatat64`](#fstatat64)
  - [`ftruncate64`](#ftruncate64)
  - [`lseek64`](#lseek64)
  - [`lstat64`](#lstat64)
  - [`mmap64`](#mmap64)
  - [`open64`](#open64)
  - [`openat64`](#openat64)
  - [`posix_fadvise64`](#posix-fadvise64)
  - [`pread64`](#pread64)
  - [`pwrite64`](#pwrite64)
  - [`readdir64`](#readdir64)
  - [`readdir64_r`](#readdir64-r)
  - [`stat64`](#stat64)
  - [`truncate64`](#truncate64)
  - [`preadv64`](#preadv64)
  - [`pwritev64`](#pwritev64)
  - [`forkpty`](#forkpty)
  - [`openpty`](#openpty)
  - [`statx`](#statx)
  - [`_IOC`](#ioc)
  - [`_IO`](#io)
  - [`_IOR`](#ior)
  - [`_IOW`](#iow)
  - [`_IOWR`](#iowr)
  - [`CMSG_ALIGN`](#cmsg-align)
  - [`CMSG_FIRSTHDR`](#cmsg-firsthdr)
  - [`CMSG_DATA`](#cmsg-data)
  - [`CMSG_SPACE`](#cmsg-space)
  - [`CMSG_LEN`](#cmsg-len)
  - [`FD_CLR`](#fd-clr)
  - [`FD_ISSET`](#fd-isset)
  - [`FD_SET`](#fd-set)
  - [`FD_ZERO`](#fd-zero)
  - [`SIGRTMAX`](#sigrtmax)
  - [`SIGRTMIN`](#sigrtmin)
  - [`WIFSTOPPED`](#wifstopped)
  - [`WSTOPSIG`](#wstopsig)
  - [`WIFCONTINUED`](#wifcontinued)
  - [`WIFSIGNALED`](#wifsignaled)
  - [`WTERMSIG`](#wtermsig)
  - [`WIFEXITED`](#wifexited)
  - [`WEXITSTATUS`](#wexitstatus)
  - [`WCOREDUMP`](#wcoredump)
  - [`W_EXITCODE`](#w-exitcode)
  - [`W_STOPCODE`](#w-stopcode)
  - [`QCMD`](#qcmd)
  - [`IPOPT_COPIED`](#ipopt-copied)
  - [`IPOPT_CLASS`](#ipopt-class)
  - [`IPOPT_NUMBER`](#ipopt-number)
  - [`IPTOS_ECN`](#iptos-ecn)
  - [`KERNEL_VERSION`](#kernel-version)
- [Type Aliases](#type-aliases)
  - [`intmax_t`](#intmax-t)
  - [`uintmax_t`](#uintmax-t)
  - [`size_t`](#size-t)
  - [`ptrdiff_t`](#ptrdiff-t)
  - [`intptr_t`](#intptr-t)
  - [`uintptr_t`](#uintptr-t)
  - [`ssize_t`](#ssize-t)
  - [`pid_t`](#pid-t)
  - [`in_addr_t`](#in-addr-t)
  - [`in_port_t`](#in-port-t)
  - [`sighandler_t`](#sighandler-t)
  - [`cc_t`](#cc-t)
  - [`uid_t`](#uid-t)
  - [`gid_t`](#gid-t)
  - [`locale_t`](#locale-t)
  - [`sa_family_t`](#sa-family-t)
  - [`speed_t`](#speed-t)
  - [`tcflag_t`](#tcflag-t)
  - [`clockid_t`](#clockid-t)
  - [`timer_t`](#timer-t)
  - [`useconds_t`](#useconds-t)
  - [`key_t`](#key-t)
  - [`id_t`](#id-t)
- [Constants](#constants)
  - [`INT_MIN`](#int-min)
  - [`INT_MAX`](#int-max)
  - [`SIG_DFL`](#sig-dfl)
  - [`SIG_IGN`](#sig-ign)
  - [`SIG_ERR`](#sig-err)
  - [`DT_UNKNOWN`](#dt-unknown)
  - [`DT_FIFO`](#dt-fifo)
  - [`DT_CHR`](#dt-chr)
  - [`DT_DIR`](#dt-dir)
  - [`DT_BLK`](#dt-blk)
  - [`DT_REG`](#dt-reg)
  - [`DT_LNK`](#dt-lnk)
  - [`DT_SOCK`](#dt-sock)
  - [`FD_CLOEXEC`](#fd-cloexec)
  - [`USRQUOTA`](#usrquota)
  - [`GRPQUOTA`](#grpquota)
  - [`SIGIOT`](#sigiot)
  - [`S_ISUID`](#s-isuid)
  - [`S_ISGID`](#s-isgid)
  - [`S_ISVTX`](#s-isvtx)
  - [`IF_NAMESIZE`](#if-namesize)
  - [`IFNAMSIZ`](#ifnamsiz)
  - [`LOG_EMERG`](#log-emerg)
  - [`LOG_ALERT`](#log-alert)
  - [`LOG_CRIT`](#log-crit)
  - [`LOG_ERR`](#log-err)
  - [`LOG_WARNING`](#log-warning)
  - [`LOG_NOTICE`](#log-notice)
  - [`LOG_INFO`](#log-info)
  - [`LOG_DEBUG`](#log-debug)
  - [`LOG_KERN`](#log-kern)
  - [`LOG_USER`](#log-user)
  - [`LOG_MAIL`](#log-mail)
  - [`LOG_DAEMON`](#log-daemon)
  - [`LOG_AUTH`](#log-auth)
  - [`LOG_SYSLOG`](#log-syslog)
  - [`LOG_LPR`](#log-lpr)
  - [`LOG_NEWS`](#log-news)
  - [`LOG_UUCP`](#log-uucp)
  - [`LOG_LOCAL0`](#log-local0)
  - [`LOG_LOCAL1`](#log-local1)
  - [`LOG_LOCAL2`](#log-local2)
  - [`LOG_LOCAL3`](#log-local3)
  - [`LOG_LOCAL4`](#log-local4)
  - [`LOG_LOCAL5`](#log-local5)
  - [`LOG_LOCAL6`](#log-local6)
  - [`LOG_LOCAL7`](#log-local7)
  - [`LOG_PID`](#log-pid)
  - [`LOG_CONS`](#log-cons)
  - [`LOG_ODELAY`](#log-odelay)
  - [`LOG_NDELAY`](#log-ndelay)
  - [`LOG_NOWAIT`](#log-nowait)
  - [`LOG_PRIMASK`](#log-primask)
  - [`LOG_FACMASK`](#log-facmask)
  - [`PRIO_MIN`](#prio-min)
  - [`PRIO_MAX`](#prio-max)
  - [`IPPROTO_ICMP`](#ipproto-icmp)
  - [`IPPROTO_ICMPV6`](#ipproto-icmpv6)
  - [`IPPROTO_TCP`](#ipproto-tcp)
  - [`IPPROTO_UDP`](#ipproto-udp)
  - [`IPPROTO_IP`](#ipproto-ip)
  - [`IPPROTO_IPV6`](#ipproto-ipv6)
  - [`INADDR_LOOPBACK`](#inaddr-loopback)
  - [`INADDR_ANY`](#inaddr-any)
  - [`INADDR_BROADCAST`](#inaddr-broadcast)
  - [`INADDR_NONE`](#inaddr-none)
  - [`IN6ADDR_LOOPBACK_INIT`](#in6addr-loopback-init)
  - [`IN6ADDR_ANY_INIT`](#in6addr-any-init)
  - [`ARPOP_REQUEST`](#arpop-request)
  - [`ARPOP_REPLY`](#arpop-reply)
  - [`ATF_COM`](#atf-com)
  - [`ATF_PERM`](#atf-perm)
  - [`ATF_PUBL`](#atf-publ)
  - [`ATF_USETRAILERS`](#atf-usetrailers)
  - [`FNM_PERIOD`](#fnm-period)
  - [`FNM_NOMATCH`](#fnm-nomatch)
  - [`FNM_CASEFOLD`](#fnm-casefold)
  - [`FNM_PATHNAME`](#fnm-pathname)
  - [`FNM_NOESCAPE`](#fnm-noescape)
  - [`ULONG_SIZE`](#ulong-size)
  - [`EXIT_FAILURE`](#exit-failure)
  - [`EXIT_SUCCESS`](#exit-success)
  - [`RAND_MAX`](#rand-max)
  - [`EOF`](#eof)
  - [`SEEK_SET`](#seek-set)
  - [`SEEK_CUR`](#seek-cur)
  - [`SEEK_END`](#seek-end)
  - [`_IOFBF`](#iofbf)
  - [`_IONBF`](#ionbf)
  - [`_IOLBF`](#iolbf)
  - [`F_DUPFD`](#f-dupfd)
  - [`F_GETFD`](#f-getfd)
  - [`F_SETFD`](#f-setfd)
  - [`F_GETFL`](#f-getfl)
  - [`F_SETFL`](#f-setfl)
  - [`F_SETLEASE`](#f-setlease)
  - [`F_GETLEASE`](#f-getlease)
  - [`F_NOTIFY`](#f-notify)
  - [`F_CANCELLK`](#f-cancellk)
  - [`F_DUPFD_CLOEXEC`](#f-dupfd-cloexec)
  - [`F_SETPIPE_SZ`](#f-setpipe-sz)
  - [`F_GETPIPE_SZ`](#f-getpipe-sz)
  - [`F_ADD_SEALS`](#f-add-seals)
  - [`F_GET_SEALS`](#f-get-seals)
  - [`F_SEAL_SEAL`](#f-seal-seal)
  - [`F_SEAL_SHRINK`](#f-seal-shrink)
  - [`F_SEAL_GROW`](#f-seal-grow)
  - [`F_SEAL_WRITE`](#f-seal-write)
  - [`SIGTRAP`](#sigtrap)
  - [`PTHREAD_CREATE_JOINABLE`](#pthread-create-joinable)
  - [`PTHREAD_CREATE_DETACHED`](#pthread-create-detached)
  - [`CLOCK_REALTIME`](#clock-realtime)
  - [`CLOCK_MONOTONIC`](#clock-monotonic)
  - [`CLOCK_PROCESS_CPUTIME_ID`](#clock-process-cputime-id)
  - [`CLOCK_THREAD_CPUTIME_ID`](#clock-thread-cputime-id)
  - [`CLOCK_MONOTONIC_RAW`](#clock-monotonic-raw)
  - [`CLOCK_REALTIME_COARSE`](#clock-realtime-coarse)
  - [`CLOCK_MONOTONIC_COARSE`](#clock-monotonic-coarse)
  - [`CLOCK_BOOTTIME`](#clock-boottime)
  - [`CLOCK_REALTIME_ALARM`](#clock-realtime-alarm)
  - [`CLOCK_BOOTTIME_ALARM`](#clock-boottime-alarm)
  - [`CLOCK_TAI`](#clock-tai)
  - [`TIMER_ABSTIME`](#timer-abstime)
  - [`RUSAGE_SELF`](#rusage-self)
  - [`O_RDONLY`](#o-rdonly)
  - [`O_WRONLY`](#o-wronly)
  - [`O_RDWR`](#o-rdwr)
  - [`SOCK_CLOEXEC`](#sock-cloexec)
  - [`S_IFIFO`](#s-ififo)
  - [`S_IFCHR`](#s-ifchr)
  - [`S_IFBLK`](#s-ifblk)
  - [`S_IFDIR`](#s-ifdir)
  - [`S_IFREG`](#s-ifreg)
  - [`S_IFLNK`](#s-iflnk)
  - [`S_IFSOCK`](#s-ifsock)
  - [`S_IFMT`](#s-ifmt)
  - [`S_IRWXU`](#s-irwxu)
  - [`S_IXUSR`](#s-ixusr)
  - [`S_IWUSR`](#s-iwusr)
  - [`S_IRUSR`](#s-irusr)
  - [`S_IRWXG`](#s-irwxg)
  - [`S_IXGRP`](#s-ixgrp)
  - [`S_IWGRP`](#s-iwgrp)
  - [`S_IRGRP`](#s-irgrp)
  - [`S_IRWXO`](#s-irwxo)
  - [`S_IXOTH`](#s-ixoth)
  - [`S_IWOTH`](#s-iwoth)
  - [`S_IROTH`](#s-iroth)
  - [`F_OK`](#f-ok)
  - [`R_OK`](#r-ok)
  - [`W_OK`](#w-ok)
  - [`X_OK`](#x-ok)
  - [`SIGHUP`](#sighup)
  - [`SIGINT`](#sigint)
  - [`SIGQUIT`](#sigquit)
  - [`SIGILL`](#sigill)
  - [`SIGABRT`](#sigabrt)
  - [`SIGFPE`](#sigfpe)
  - [`SIGKILL`](#sigkill)
  - [`SIGSEGV`](#sigsegv)
  - [`SIGPIPE`](#sigpipe)
  - [`SIGALRM`](#sigalrm)
  - [`SIGTERM`](#sigterm)
  - [`PROT_NONE`](#prot-none)
  - [`PROT_READ`](#prot-read)
  - [`PROT_WRITE`](#prot-write)
  - [`PROT_EXEC`](#prot-exec)
  - [`XATTR_CREATE`](#xattr-create)
  - [`XATTR_REPLACE`](#xattr-replace)
  - [`RLIM64_INFINITY`](#rlim64-infinity)
  - [`LC_CTYPE`](#lc-ctype)
  - [`LC_NUMERIC`](#lc-numeric)
  - [`LC_TIME`](#lc-time)
  - [`LC_COLLATE`](#lc-collate)
  - [`LC_MONETARY`](#lc-monetary)
  - [`LC_MESSAGES`](#lc-messages)
  - [`LC_ALL`](#lc-all)
  - [`LC_CTYPE_MASK`](#lc-ctype-mask)
  - [`LC_NUMERIC_MASK`](#lc-numeric-mask)
  - [`LC_TIME_MASK`](#lc-time-mask)
  - [`LC_COLLATE_MASK`](#lc-collate-mask)
  - [`LC_MONETARY_MASK`](#lc-monetary-mask)
  - [`LC_MESSAGES_MASK`](#lc-messages-mask)
  - [`MAP_FILE`](#map-file)
  - [`MAP_SHARED`](#map-shared)
  - [`MAP_PRIVATE`](#map-private)
  - [`MAP_FIXED`](#map-fixed)
  - [`MAP_FAILED`](#map-failed)
  - [`MS_ASYNC`](#ms-async)
  - [`MS_INVALIDATE`](#ms-invalidate)
  - [`MS_SYNC`](#ms-sync)
  - [`MS_RDONLY`](#ms-rdonly)
  - [`MS_NOSUID`](#ms-nosuid)
  - [`MS_NODEV`](#ms-nodev)
  - [`MS_NOEXEC`](#ms-noexec)
  - [`MS_SYNCHRONOUS`](#ms-synchronous)
  - [`MS_REMOUNT`](#ms-remount)
  - [`MS_MANDLOCK`](#ms-mandlock)
  - [`MS_DIRSYNC`](#ms-dirsync)
  - [`MS_NOSYMFOLLOW`](#ms-nosymfollow)
  - [`MS_NOATIME`](#ms-noatime)
  - [`MS_NODIRATIME`](#ms-nodiratime)
  - [`MS_BIND`](#ms-bind)
  - [`MS_MOVE`](#ms-move)
  - [`MS_REC`](#ms-rec)
  - [`MS_SILENT`](#ms-silent)
  - [`MS_POSIXACL`](#ms-posixacl)
  - [`MS_UNBINDABLE`](#ms-unbindable)
  - [`MS_PRIVATE`](#ms-private)
  - [`MS_SLAVE`](#ms-slave)
  - [`MS_SHARED`](#ms-shared)
  - [`MS_RELATIME`](#ms-relatime)
  - [`MS_KERNMOUNT`](#ms-kernmount)
  - [`MS_I_VERSION`](#ms-i-version)
  - [`MS_STRICTATIME`](#ms-strictatime)
  - [`MS_LAZYTIME`](#ms-lazytime)
  - [`MS_ACTIVE`](#ms-active)
  - [`MS_MGC_VAL`](#ms-mgc-val)
  - [`MS_MGC_MSK`](#ms-mgc-msk)
  - [`SCM_RIGHTS`](#scm-rights)
  - [`SCM_CREDENTIALS`](#scm-credentials)
  - [`PROT_GROWSDOWN`](#prot-growsdown)
  - [`PROT_GROWSUP`](#prot-growsup)
  - [`MAP_TYPE`](#map-type)
  - [`MADV_NORMAL`](#madv-normal)
  - [`MADV_RANDOM`](#madv-random)
  - [`MADV_SEQUENTIAL`](#madv-sequential)
  - [`MADV_WILLNEED`](#madv-willneed)
  - [`MADV_DONTNEED`](#madv-dontneed)
  - [`MADV_FREE`](#madv-free)
  - [`MADV_REMOVE`](#madv-remove)
  - [`MADV_DONTFORK`](#madv-dontfork)
  - [`MADV_DOFORK`](#madv-dofork)
  - [`MADV_MERGEABLE`](#madv-mergeable)
  - [`MADV_UNMERGEABLE`](#madv-unmergeable)
  - [`MADV_HUGEPAGE`](#madv-hugepage)
  - [`MADV_NOHUGEPAGE`](#madv-nohugepage)
  - [`MADV_DONTDUMP`](#madv-dontdump)
  - [`MADV_DODUMP`](#madv-dodump)
  - [`MADV_WIPEONFORK`](#madv-wipeonfork)
  - [`MADV_KEEPONFORK`](#madv-keeponfork)
  - [`MADV_COLD`](#madv-cold)
  - [`MADV_PAGEOUT`](#madv-pageout)
  - [`MADV_HWPOISON`](#madv-hwpoison)
  - [`MADV_POPULATE_READ`](#madv-populate-read)
  - [`MADV_POPULATE_WRITE`](#madv-populate-write)
  - [`MADV_DONTNEED_LOCKED`](#madv-dontneed-locked)
  - [`IFF_UP`](#iff-up)
  - [`IFF_BROADCAST`](#iff-broadcast)
  - [`IFF_DEBUG`](#iff-debug)
  - [`IFF_LOOPBACK`](#iff-loopback)
  - [`IFF_POINTOPOINT`](#iff-pointopoint)
  - [`IFF_NOTRAILERS`](#iff-notrailers)
  - [`IFF_RUNNING`](#iff-running)
  - [`IFF_NOARP`](#iff-noarp)
  - [`IFF_PROMISC`](#iff-promisc)
  - [`IFF_ALLMULTI`](#iff-allmulti)
  - [`IFF_MASTER`](#iff-master)
  - [`IFF_SLAVE`](#iff-slave)
  - [`IFF_MULTICAST`](#iff-multicast)
  - [`IFF_PORTSEL`](#iff-portsel)
  - [`IFF_AUTOMEDIA`](#iff-automedia)
  - [`IFF_DYNAMIC`](#iff-dynamic)
  - [`SOL_IP`](#sol-ip)
  - [`SOL_TCP`](#sol-tcp)
  - [`SOL_UDP`](#sol-udp)
  - [`SOL_IPV6`](#sol-ipv6)
  - [`SOL_ICMPV6`](#sol-icmpv6)
  - [`SOL_RAW`](#sol-raw)
  - [`SOL_DECNET`](#sol-decnet)
  - [`SOL_X25`](#sol-x25)
  - [`SOL_PACKET`](#sol-packet)
  - [`SOL_ATM`](#sol-atm)
  - [`SOL_AAL`](#sol-aal)
  - [`SOL_IRDA`](#sol-irda)
  - [`SOL_NETBEUI`](#sol-netbeui)
  - [`SOL_LLC`](#sol-llc)
  - [`SOL_DCCP`](#sol-dccp)
  - [`SOL_NETLINK`](#sol-netlink)
  - [`SOL_TIPC`](#sol-tipc)
  - [`SOL_BLUETOOTH`](#sol-bluetooth)
  - [`SOL_ALG`](#sol-alg)
  - [`AF_UNSPEC`](#af-unspec)
  - [`AF_UNIX`](#af-unix)
  - [`AF_LOCAL`](#af-local)
  - [`AF_INET`](#af-inet)
  - [`AF_AX25`](#af-ax25)
  - [`AF_IPX`](#af-ipx)
  - [`AF_APPLETALK`](#af-appletalk)
  - [`AF_NETROM`](#af-netrom)
  - [`AF_BRIDGE`](#af-bridge)
  - [`AF_ATMPVC`](#af-atmpvc)
  - [`AF_X25`](#af-x25)
  - [`AF_INET6`](#af-inet6)
  - [`AF_ROSE`](#af-rose)
  - [`AF_DECnet`](#af-decnet)
  - [`AF_NETBEUI`](#af-netbeui)
  - [`AF_SECURITY`](#af-security)
  - [`AF_KEY`](#af-key)
  - [`AF_NETLINK`](#af-netlink)
  - [`AF_ROUTE`](#af-route)
  - [`AF_PACKET`](#af-packet)
  - [`AF_ASH`](#af-ash)
  - [`AF_ECONET`](#af-econet)
  - [`AF_ATMSVC`](#af-atmsvc)
  - [`AF_RDS`](#af-rds)
  - [`AF_SNA`](#af-sna)
  - [`AF_IRDA`](#af-irda)
  - [`AF_PPPOX`](#af-pppox)
  - [`AF_WANPIPE`](#af-wanpipe)
  - [`AF_LLC`](#af-llc)
  - [`AF_CAN`](#af-can)
  - [`AF_TIPC`](#af-tipc)
  - [`AF_BLUETOOTH`](#af-bluetooth)
  - [`AF_IUCV`](#af-iucv)
  - [`AF_RXRPC`](#af-rxrpc)
  - [`AF_ISDN`](#af-isdn)
  - [`AF_PHONET`](#af-phonet)
  - [`AF_IEEE802154`](#af-ieee802154)
  - [`AF_CAIF`](#af-caif)
  - [`AF_ALG`](#af-alg)
  - [`PF_UNSPEC`](#pf-unspec)
  - [`PF_UNIX`](#pf-unix)
  - [`PF_LOCAL`](#pf-local)
  - [`PF_INET`](#pf-inet)
  - [`PF_AX25`](#pf-ax25)
  - [`PF_IPX`](#pf-ipx)
  - [`PF_APPLETALK`](#pf-appletalk)
  - [`PF_NETROM`](#pf-netrom)
  - [`PF_BRIDGE`](#pf-bridge)
  - [`PF_ATMPVC`](#pf-atmpvc)
  - [`PF_X25`](#pf-x25)
  - [`PF_INET6`](#pf-inet6)
  - [`PF_ROSE`](#pf-rose)
  - [`PF_DECnet`](#pf-decnet)
  - [`PF_NETBEUI`](#pf-netbeui)
  - [`PF_SECURITY`](#pf-security)
  - [`PF_KEY`](#pf-key)
  - [`PF_NETLINK`](#pf-netlink)
  - [`PF_ROUTE`](#pf-route)
  - [`PF_PACKET`](#pf-packet)
  - [`PF_ASH`](#pf-ash)
  - [`PF_ECONET`](#pf-econet)
  - [`PF_ATMSVC`](#pf-atmsvc)
  - [`PF_RDS`](#pf-rds)
  - [`PF_SNA`](#pf-sna)
  - [`PF_IRDA`](#pf-irda)
  - [`PF_PPPOX`](#pf-pppox)
  - [`PF_WANPIPE`](#pf-wanpipe)
  - [`PF_LLC`](#pf-llc)
  - [`PF_CAN`](#pf-can)
  - [`PF_TIPC`](#pf-tipc)
  - [`PF_BLUETOOTH`](#pf-bluetooth)
  - [`PF_IUCV`](#pf-iucv)
  - [`PF_RXRPC`](#pf-rxrpc)
  - [`PF_ISDN`](#pf-isdn)
  - [`PF_PHONET`](#pf-phonet)
  - [`PF_IEEE802154`](#pf-ieee802154)
  - [`PF_CAIF`](#pf-caif)
  - [`PF_ALG`](#pf-alg)
  - [`MSG_OOB`](#msg-oob)
  - [`MSG_PEEK`](#msg-peek)
  - [`MSG_DONTROUTE`](#msg-dontroute)
  - [`MSG_CTRUNC`](#msg-ctrunc)
  - [`MSG_TRUNC`](#msg-trunc)
  - [`MSG_DONTWAIT`](#msg-dontwait)
  - [`MSG_EOR`](#msg-eor)
  - [`MSG_WAITALL`](#msg-waitall)
  - [`MSG_FIN`](#msg-fin)
  - [`MSG_SYN`](#msg-syn)
  - [`MSG_CONFIRM`](#msg-confirm)
  - [`MSG_RST`](#msg-rst)
  - [`MSG_ERRQUEUE`](#msg-errqueue)
  - [`MSG_NOSIGNAL`](#msg-nosignal)
  - [`MSG_MORE`](#msg-more)
  - [`MSG_WAITFORONE`](#msg-waitforone)
  - [`MSG_FASTOPEN`](#msg-fastopen)
  - [`MSG_CMSG_CLOEXEC`](#msg-cmsg-cloexec)
  - [`SCM_TIMESTAMP`](#scm-timestamp)
  - [`SOCK_RAW`](#sock-raw)
  - [`SOCK_RDM`](#sock-rdm)
  - [`IP_TOS`](#ip-tos)
  - [`IP_TTL`](#ip-ttl)
  - [`IP_HDRINCL`](#ip-hdrincl)
  - [`IP_OPTIONS`](#ip-options)
  - [`IP_ROUTER_ALERT`](#ip-router-alert)
  - [`IP_RECVOPTS`](#ip-recvopts)
  - [`IP_RETOPTS`](#ip-retopts)
  - [`IP_PKTINFO`](#ip-pktinfo)
  - [`IP_PKTOPTIONS`](#ip-pktoptions)
  - [`IP_MTU_DISCOVER`](#ip-mtu-discover)
  - [`IP_RECVERR`](#ip-recverr)
  - [`IP_RECVTTL`](#ip-recvttl)
  - [`IP_RECVTOS`](#ip-recvtos)
  - [`IP_MTU`](#ip-mtu)
  - [`IP_FREEBIND`](#ip-freebind)
  - [`IP_IPSEC_POLICY`](#ip-ipsec-policy)
  - [`IP_XFRM_POLICY`](#ip-xfrm-policy)
  - [`IP_PASSSEC`](#ip-passsec)
  - [`IP_TRANSPARENT`](#ip-transparent)
  - [`IP_ORIGDSTADDR`](#ip-origdstaddr)
  - [`IP_RECVORIGDSTADDR`](#ip-recvorigdstaddr)
  - [`IP_MINTTL`](#ip-minttl)
  - [`IP_NODEFRAG`](#ip-nodefrag)
  - [`IP_CHECKSUM`](#ip-checksum)
  - [`IP_BIND_ADDRESS_NO_PORT`](#ip-bind-address-no-port)
  - [`IP_MULTICAST_IF`](#ip-multicast-if)
  - [`IP_MULTICAST_TTL`](#ip-multicast-ttl)
  - [`IP_MULTICAST_LOOP`](#ip-multicast-loop)
  - [`IP_ADD_MEMBERSHIP`](#ip-add-membership)
  - [`IP_DROP_MEMBERSHIP`](#ip-drop-membership)
  - [`IP_UNBLOCK_SOURCE`](#ip-unblock-source)
  - [`IP_BLOCK_SOURCE`](#ip-block-source)
  - [`IP_ADD_SOURCE_MEMBERSHIP`](#ip-add-source-membership)
  - [`IP_DROP_SOURCE_MEMBERSHIP`](#ip-drop-source-membership)
  - [`IP_MSFILTER`](#ip-msfilter)
  - [`IP_MULTICAST_ALL`](#ip-multicast-all)
  - [`IP_UNICAST_IF`](#ip-unicast-if)
  - [`IP_DEFAULT_MULTICAST_TTL`](#ip-default-multicast-ttl)
  - [`IP_DEFAULT_MULTICAST_LOOP`](#ip-default-multicast-loop)
  - [`IP_PMTUDISC_DONT`](#ip-pmtudisc-dont)
  - [`IP_PMTUDISC_WANT`](#ip-pmtudisc-want)
  - [`IP_PMTUDISC_DO`](#ip-pmtudisc-do)
  - [`IP_PMTUDISC_PROBE`](#ip-pmtudisc-probe)
  - [`IP_PMTUDISC_INTERFACE`](#ip-pmtudisc-interface)
  - [`IP_PMTUDISC_OMIT`](#ip-pmtudisc-omit)
  - [`IPPROTO_HOPOPTS`](#ipproto-hopopts)
  - [`IPPROTO_IGMP`](#ipproto-igmp)
  - [`IPPROTO_IPIP`](#ipproto-ipip)
  - [`IPPROTO_EGP`](#ipproto-egp)
  - [`IPPROTO_PUP`](#ipproto-pup)
  - [`IPPROTO_IDP`](#ipproto-idp)
  - [`IPPROTO_TP`](#ipproto-tp)
  - [`IPPROTO_DCCP`](#ipproto-dccp)
  - [`IPPROTO_ROUTING`](#ipproto-routing)
  - [`IPPROTO_FRAGMENT`](#ipproto-fragment)
  - [`IPPROTO_RSVP`](#ipproto-rsvp)
  - [`IPPROTO_GRE`](#ipproto-gre)
  - [`IPPROTO_ESP`](#ipproto-esp)
  - [`IPPROTO_AH`](#ipproto-ah)
  - [`IPPROTO_NONE`](#ipproto-none)
  - [`IPPROTO_DSTOPTS`](#ipproto-dstopts)
  - [`IPPROTO_MTP`](#ipproto-mtp)
  - [`IPPROTO_ENCAP`](#ipproto-encap)
  - [`IPPROTO_PIM`](#ipproto-pim)
  - [`IPPROTO_COMP`](#ipproto-comp)
  - [`IPPROTO_SCTP`](#ipproto-sctp)
  - [`IPPROTO_MH`](#ipproto-mh)
  - [`IPPROTO_UDPLITE`](#ipproto-udplite)
  - [`IPPROTO_RAW`](#ipproto-raw)
  - [`IPPROTO_BEETPH`](#ipproto-beetph)
  - [`IPPROTO_MPLS`](#ipproto-mpls)
  - [`IPPROTO_MPTCP`](#ipproto-mptcp)
  - [`IPPROTO_ETHERNET`](#ipproto-ethernet)
  - [`MCAST_EXCLUDE`](#mcast-exclude)
  - [`MCAST_INCLUDE`](#mcast-include)
  - [`MCAST_JOIN_GROUP`](#mcast-join-group)
  - [`MCAST_BLOCK_SOURCE`](#mcast-block-source)
  - [`MCAST_UNBLOCK_SOURCE`](#mcast-unblock-source)
  - [`MCAST_LEAVE_GROUP`](#mcast-leave-group)
  - [`MCAST_JOIN_SOURCE_GROUP`](#mcast-join-source-group)
  - [`MCAST_LEAVE_SOURCE_GROUP`](#mcast-leave-source-group)
  - [`MCAST_MSFILTER`](#mcast-msfilter)
  - [`IPV6_ADDRFORM`](#ipv6-addrform)
  - [`IPV6_2292PKTINFO`](#ipv6-2292pktinfo)
  - [`IPV6_2292HOPOPTS`](#ipv6-2292hopopts)
  - [`IPV6_2292DSTOPTS`](#ipv6-2292dstopts)
  - [`IPV6_2292RTHDR`](#ipv6-2292rthdr)
  - [`IPV6_2292PKTOPTIONS`](#ipv6-2292pktoptions)
  - [`IPV6_CHECKSUM`](#ipv6-checksum)
  - [`IPV6_2292HOPLIMIT`](#ipv6-2292hoplimit)
  - [`IPV6_NEXTHOP`](#ipv6-nexthop)
  - [`IPV6_AUTHHDR`](#ipv6-authhdr)
  - [`IPV6_UNICAST_HOPS`](#ipv6-unicast-hops)
  - [`IPV6_MULTICAST_IF`](#ipv6-multicast-if)
  - [`IPV6_MULTICAST_HOPS`](#ipv6-multicast-hops)
  - [`IPV6_MULTICAST_LOOP`](#ipv6-multicast-loop)
  - [`IPV6_ADD_MEMBERSHIP`](#ipv6-add-membership)
  - [`IPV6_DROP_MEMBERSHIP`](#ipv6-drop-membership)
  - [`IPV6_ROUTER_ALERT`](#ipv6-router-alert)
  - [`IPV6_MTU_DISCOVER`](#ipv6-mtu-discover)
  - [`IPV6_MTU`](#ipv6-mtu)
  - [`IPV6_RECVERR`](#ipv6-recverr)
  - [`IPV6_V6ONLY`](#ipv6-v6only)
  - [`IPV6_JOIN_ANYCAST`](#ipv6-join-anycast)
  - [`IPV6_LEAVE_ANYCAST`](#ipv6-leave-anycast)
  - [`IPV6_IPSEC_POLICY`](#ipv6-ipsec-policy)
  - [`IPV6_XFRM_POLICY`](#ipv6-xfrm-policy)
  - [`IPV6_HDRINCL`](#ipv6-hdrincl)
  - [`IPV6_RECVPKTINFO`](#ipv6-recvpktinfo)
  - [`IPV6_PKTINFO`](#ipv6-pktinfo)
  - [`IPV6_RECVHOPLIMIT`](#ipv6-recvhoplimit)
  - [`IPV6_HOPLIMIT`](#ipv6-hoplimit)
  - [`IPV6_RECVHOPOPTS`](#ipv6-recvhopopts)
  - [`IPV6_HOPOPTS`](#ipv6-hopopts)
  - [`IPV6_RTHDRDSTOPTS`](#ipv6-rthdrdstopts)
  - [`IPV6_RECVRTHDR`](#ipv6-recvrthdr)
  - [`IPV6_RTHDR`](#ipv6-rthdr)
  - [`IPV6_RECVDSTOPTS`](#ipv6-recvdstopts)
  - [`IPV6_DSTOPTS`](#ipv6-dstopts)
  - [`IPV6_RECVPATHMTU`](#ipv6-recvpathmtu)
  - [`IPV6_PATHMTU`](#ipv6-pathmtu)
  - [`IPV6_DONTFRAG`](#ipv6-dontfrag)
  - [`IPV6_RECVTCLASS`](#ipv6-recvtclass)
  - [`IPV6_TCLASS`](#ipv6-tclass)
  - [`IPV6_AUTOFLOWLABEL`](#ipv6-autoflowlabel)
  - [`IPV6_ADDR_PREFERENCES`](#ipv6-addr-preferences)
  - [`IPV6_MINHOPCOUNT`](#ipv6-minhopcount)
  - [`IPV6_ORIGDSTADDR`](#ipv6-origdstaddr)
  - [`IPV6_RECVORIGDSTADDR`](#ipv6-recvorigdstaddr)
  - [`IPV6_TRANSPARENT`](#ipv6-transparent)
  - [`IPV6_UNICAST_IF`](#ipv6-unicast-if)
  - [`IPV6_PREFER_SRC_TMP`](#ipv6-prefer-src-tmp)
  - [`IPV6_PREFER_SRC_PUBLIC`](#ipv6-prefer-src-public)
  - [`IPV6_PREFER_SRC_PUBTMP_DEFAULT`](#ipv6-prefer-src-pubtmp-default)
  - [`IPV6_PREFER_SRC_COA`](#ipv6-prefer-src-coa)
  - [`IPV6_PREFER_SRC_HOME`](#ipv6-prefer-src-home)
  - [`IPV6_PREFER_SRC_CGA`](#ipv6-prefer-src-cga)
  - [`IPV6_PREFER_SRC_NONCGA`](#ipv6-prefer-src-noncga)
  - [`IPV6_PMTUDISC_DONT`](#ipv6-pmtudisc-dont)
  - [`IPV6_PMTUDISC_WANT`](#ipv6-pmtudisc-want)
  - [`IPV6_PMTUDISC_DO`](#ipv6-pmtudisc-do)
  - [`IPV6_PMTUDISC_PROBE`](#ipv6-pmtudisc-probe)
  - [`IPV6_PMTUDISC_INTERFACE`](#ipv6-pmtudisc-interface)
  - [`IPV6_PMTUDISC_OMIT`](#ipv6-pmtudisc-omit)
  - [`TCP_NODELAY`](#tcp-nodelay)
  - [`TCP_MAXSEG`](#tcp-maxseg)
  - [`TCP_CORK`](#tcp-cork)
  - [`TCP_KEEPIDLE`](#tcp-keepidle)
  - [`TCP_KEEPINTVL`](#tcp-keepintvl)
  - [`TCP_KEEPCNT`](#tcp-keepcnt)
  - [`TCP_SYNCNT`](#tcp-syncnt)
  - [`TCP_LINGER2`](#tcp-linger2)
  - [`TCP_DEFER_ACCEPT`](#tcp-defer-accept)
  - [`TCP_WINDOW_CLAMP`](#tcp-window-clamp)
  - [`TCP_INFO`](#tcp-info)
  - [`TCP_QUICKACK`](#tcp-quickack)
  - [`TCP_CONGESTION`](#tcp-congestion)
  - [`TCP_MD5SIG`](#tcp-md5sig)
  - [`TCP_COOKIE_TRANSACTIONS`](#tcp-cookie-transactions)
  - [`TCP_THIN_LINEAR_TIMEOUTS`](#tcp-thin-linear-timeouts)
  - [`TCP_THIN_DUPACK`](#tcp-thin-dupack)
  - [`TCP_USER_TIMEOUT`](#tcp-user-timeout)
  - [`TCP_REPAIR`](#tcp-repair)
  - [`TCP_REPAIR_QUEUE`](#tcp-repair-queue)
  - [`TCP_QUEUE_SEQ`](#tcp-queue-seq)
  - [`TCP_REPAIR_OPTIONS`](#tcp-repair-options)
  - [`TCP_FASTOPEN`](#tcp-fastopen)
  - [`TCP_TIMESTAMP`](#tcp-timestamp)
  - [`TCP_NOTSENT_LOWAT`](#tcp-notsent-lowat)
  - [`TCP_CC_INFO`](#tcp-cc-info)
  - [`TCP_SAVE_SYN`](#tcp-save-syn)
  - [`TCP_SAVED_SYN`](#tcp-saved-syn)
  - [`TCP_REPAIR_WINDOW`](#tcp-repair-window)
  - [`TCP_FASTOPEN_CONNECT`](#tcp-fastopen-connect)
  - [`TCP_ULP`](#tcp-ulp)
  - [`TCP_MD5SIG_EXT`](#tcp-md5sig-ext)
  - [`TCP_FASTOPEN_KEY`](#tcp-fastopen-key)
  - [`TCP_FASTOPEN_NO_COOKIE`](#tcp-fastopen-no-cookie)
  - [`TCP_ZEROCOPY_RECEIVE`](#tcp-zerocopy-receive)
  - [`TCP_INQ`](#tcp-inq)
  - [`TCP_CM_INQ`](#tcp-cm-inq)
  - [`TCP_MD5SIG_MAXKEYLEN`](#tcp-md5sig-maxkeylen)
  - [`SO_DEBUG`](#so-debug)
  - [`SHUT_RD`](#shut-rd)
  - [`SHUT_WR`](#shut-wr)
  - [`SHUT_RDWR`](#shut-rdwr)
  - [`LOCK_SH`](#lock-sh)
  - [`LOCK_EX`](#lock-ex)
  - [`LOCK_NB`](#lock-nb)
  - [`LOCK_UN`](#lock-un)
  - [`SS_ONSTACK`](#ss-onstack)
  - [`SS_DISABLE`](#ss-disable)
  - [`NAME_MAX`](#name-max)
  - [`PATH_MAX`](#path-max)
  - [`UIO_MAXIOV`](#uio-maxiov)
  - [`FD_SETSIZE`](#fd-setsize)
  - [`EPOLLIN`](#epollin)
  - [`EPOLLPRI`](#epollpri)
  - [`EPOLLOUT`](#epollout)
  - [`EPOLLERR`](#epollerr)
  - [`EPOLLHUP`](#epollhup)
  - [`EPOLLRDNORM`](#epollrdnorm)
  - [`EPOLLRDBAND`](#epollrdband)
  - [`EPOLLWRNORM`](#epollwrnorm)
  - [`EPOLLWRBAND`](#epollwrband)
  - [`EPOLLMSG`](#epollmsg)
  - [`EPOLLRDHUP`](#epollrdhup)
  - [`EPOLLEXCLUSIVE`](#epollexclusive)
  - [`EPOLLWAKEUP`](#epollwakeup)
  - [`EPOLLONESHOT`](#epolloneshot)
  - [`EPOLLET`](#epollet)
  - [`EPOLL_CTL_ADD`](#epoll-ctl-add)
  - [`EPOLL_CTL_MOD`](#epoll-ctl-mod)
  - [`EPOLL_CTL_DEL`](#epoll-ctl-del)
  - [`MNT_FORCE`](#mnt-force)
  - [`MNT_DETACH`](#mnt-detach)
  - [`MNT_EXPIRE`](#mnt-expire)
  - [`UMOUNT_NOFOLLOW`](#umount-nofollow)
  - [`Q_GETFMT`](#q-getfmt)
  - [`Q_GETINFO`](#q-getinfo)
  - [`Q_SETINFO`](#q-setinfo)
  - [`QIF_BLIMITS`](#qif-blimits)
  - [`QIF_SPACE`](#qif-space)
  - [`QIF_ILIMITS`](#qif-ilimits)
  - [`QIF_INODES`](#qif-inodes)
  - [`QIF_BTIME`](#qif-btime)
  - [`QIF_ITIME`](#qif-itime)
  - [`QIF_LIMITS`](#qif-limits)
  - [`QIF_USAGE`](#qif-usage)
  - [`QIF_TIMES`](#qif-times)
  - [`QIF_ALL`](#qif-all)
  - [`Q_SYNC`](#q-sync)
  - [`Q_QUOTAON`](#q-quotaon)
  - [`Q_QUOTAOFF`](#q-quotaoff)
  - [`Q_GETQUOTA`](#q-getquota)
  - [`Q_SETQUOTA`](#q-setquota)
  - [`TCIOFF`](#tcioff)
  - [`TCION`](#tcion)
  - [`TCOOFF`](#tcooff)
  - [`TCOON`](#tcoon)
  - [`TCIFLUSH`](#tciflush)
  - [`TCOFLUSH`](#tcoflush)
  - [`TCIOFLUSH`](#tcioflush)
  - [`NL0`](#nl0)
  - [`NL1`](#nl1)
  - [`TAB0`](#tab0)
  - [`CR0`](#cr0)
  - [`FF0`](#ff0)
  - [`BS0`](#bs0)
  - [`VT0`](#vt0)
  - [`VERASE`](#verase)
  - [`VKILL`](#vkill)
  - [`VINTR`](#vintr)
  - [`VQUIT`](#vquit)
  - [`VLNEXT`](#vlnext)
  - [`IGNBRK`](#ignbrk)
  - [`BRKINT`](#brkint)
  - [`IGNPAR`](#ignpar)
  - [`PARMRK`](#parmrk)
  - [`INPCK`](#inpck)
  - [`ISTRIP`](#istrip)
  - [`INLCR`](#inlcr)
  - [`IGNCR`](#igncr)
  - [`ICRNL`](#icrnl)
  - [`IXANY`](#ixany)
  - [`IMAXBEL`](#imaxbel)
  - [`OPOST`](#opost)
  - [`CS5`](#cs5)
  - [`CRTSCTS`](#crtscts)
  - [`ECHO`](#echo)
  - [`OCRNL`](#ocrnl)
  - [`ONOCR`](#onocr)
  - [`ONLRET`](#onlret)
  - [`OFILL`](#ofill)
  - [`OFDEL`](#ofdel)
  - [`CLONE_VM`](#clone-vm)
  - [`CLONE_FS`](#clone-fs)
  - [`CLONE_FILES`](#clone-files)
  - [`CLONE_SIGHAND`](#clone-sighand)
  - [`CLONE_PTRACE`](#clone-ptrace)
  - [`CLONE_VFORK`](#clone-vfork)
  - [`CLONE_PARENT`](#clone-parent)
  - [`CLONE_THREAD`](#clone-thread)
  - [`CLONE_NEWNS`](#clone-newns)
  - [`CLONE_SYSVSEM`](#clone-sysvsem)
  - [`CLONE_SETTLS`](#clone-settls)
  - [`CLONE_PARENT_SETTID`](#clone-parent-settid)
  - [`CLONE_CHILD_CLEARTID`](#clone-child-cleartid)
  - [`CLONE_DETACHED`](#clone-detached)
  - [`CLONE_UNTRACED`](#clone-untraced)
  - [`CLONE_CHILD_SETTID`](#clone-child-settid)
  - [`CLONE_NEWCGROUP`](#clone-newcgroup)
  - [`CLONE_NEWUTS`](#clone-newuts)
  - [`CLONE_NEWIPC`](#clone-newipc)
  - [`CLONE_NEWUSER`](#clone-newuser)
  - [`CLONE_NEWPID`](#clone-newpid)
  - [`CLONE_NEWNET`](#clone-newnet)
  - [`CLONE_IO`](#clone-io)
  - [`WNOHANG`](#wnohang)
  - [`WUNTRACED`](#wuntraced)
  - [`WSTOPPED`](#wstopped)
  - [`WEXITED`](#wexited)
  - [`WCONTINUED`](#wcontinued)
  - [`WNOWAIT`](#wnowait)
  - [`ADDR_NO_RANDOMIZE`](#addr-no-randomize)
  - [`MMAP_PAGE_ZERO`](#mmap-page-zero)
  - [`ADDR_COMPAT_LAYOUT`](#addr-compat-layout)
  - [`READ_IMPLIES_EXEC`](#read-implies-exec)
  - [`ADDR_LIMIT_32BIT`](#addr-limit-32bit)
  - [`SHORT_INODE`](#short-inode)
  - [`WHOLE_SECONDS`](#whole-seconds)
  - [`STICKY_TIMEOUTS`](#sticky-timeouts)
  - [`ADDR_LIMIT_3GB`](#addr-limit-3gb)
  - [`PTRACE_O_TRACESYSGOOD`](#ptrace-o-tracesysgood)
  - [`PTRACE_O_TRACEFORK`](#ptrace-o-tracefork)
  - [`PTRACE_O_TRACEVFORK`](#ptrace-o-tracevfork)
  - [`PTRACE_O_TRACECLONE`](#ptrace-o-traceclone)
  - [`PTRACE_O_TRACEEXEC`](#ptrace-o-traceexec)
  - [`PTRACE_O_TRACEVFORKDONE`](#ptrace-o-tracevforkdone)
  - [`PTRACE_O_TRACEEXIT`](#ptrace-o-traceexit)
  - [`PTRACE_O_TRACESECCOMP`](#ptrace-o-traceseccomp)
  - [`PTRACE_O_SUSPEND_SECCOMP`](#ptrace-o-suspend-seccomp)
  - [`PTRACE_O_EXITKILL`](#ptrace-o-exitkill)
  - [`PTRACE_O_MASK`](#ptrace-o-mask)
  - [`PTRACE_EVENT_FORK`](#ptrace-event-fork)
  - [`PTRACE_EVENT_VFORK`](#ptrace-event-vfork)
  - [`PTRACE_EVENT_CLONE`](#ptrace-event-clone)
  - [`PTRACE_EVENT_EXEC`](#ptrace-event-exec)
  - [`PTRACE_EVENT_VFORK_DONE`](#ptrace-event-vfork-done)
  - [`PTRACE_EVENT_EXIT`](#ptrace-event-exit)
  - [`PTRACE_EVENT_SECCOMP`](#ptrace-event-seccomp)
  - [`__WNOTHREAD`](#wnothread)
  - [`__WALL`](#wall)
  - [`__WCLONE`](#wclone)
  - [`SPLICE_F_MOVE`](#splice-f-move)
  - [`SPLICE_F_NONBLOCK`](#splice-f-nonblock)
  - [`SPLICE_F_MORE`](#splice-f-more)
  - [`SPLICE_F_GIFT`](#splice-f-gift)
  - [`RTLD_LOCAL`](#rtld-local)
  - [`RTLD_LAZY`](#rtld-lazy)
  - [`POSIX_FADV_NORMAL`](#posix-fadv-normal)
  - [`POSIX_FADV_RANDOM`](#posix-fadv-random)
  - [`POSIX_FADV_SEQUENTIAL`](#posix-fadv-sequential)
  - [`POSIX_FADV_WILLNEED`](#posix-fadv-willneed)
  - [`AT_FDCWD`](#at-fdcwd)
  - [`AT_SYMLINK_NOFOLLOW`](#at-symlink-nofollow)
  - [`AT_REMOVEDIR`](#at-removedir)
  - [`AT_SYMLINK_FOLLOW`](#at-symlink-follow)
  - [`AT_NO_AUTOMOUNT`](#at-no-automount)
  - [`AT_EMPTY_PATH`](#at-empty-path)
  - [`AT_RECURSIVE`](#at-recursive)
  - [`LOG_CRON`](#log-cron)
  - [`LOG_AUTHPRIV`](#log-authpriv)
  - [`LOG_FTP`](#log-ftp)
  - [`LOG_PERROR`](#log-perror)
  - [`PIPE_BUF`](#pipe-buf)
  - [`SI_LOAD_SHIFT`](#si-load-shift)
  - [`SI_USER`](#si-user)
  - [`SI_KERNEL`](#si-kernel)
  - [`SI_QUEUE`](#si-queue)
  - [`SI_TIMER`](#si-timer)
  - [`SI_MESGQ`](#si-mesgq)
  - [`SI_ASYNCIO`](#si-asyncio)
  - [`SI_SIGIO`](#si-sigio)
  - [`SI_TKILL`](#si-tkill)
  - [`SI_ASYNCNL`](#si-asyncnl)
  - [`BUS_ADRALN`](#bus-adraln)
  - [`BUS_ADRERR`](#bus-adrerr)
  - [`BUS_OBJERR`](#bus-objerr)
  - [`BUS_MCEERR_AR`](#bus-mceerr-ar)
  - [`BUS_MCEERR_AO`](#bus-mceerr-ao)
  - [`TRAP_BRKPT`](#trap-brkpt)
  - [`TRAP_TRACE`](#trap-trace)
  - [`TRAP_BRANCH`](#trap-branch)
  - [`TRAP_HWBKPT`](#trap-hwbkpt)
  - [`TRAP_UNK`](#trap-unk)
  - [`CLD_EXITED`](#cld-exited)
  - [`CLD_KILLED`](#cld-killed)
  - [`CLD_DUMPED`](#cld-dumped)
  - [`CLD_TRAPPED`](#cld-trapped)
  - [`CLD_STOPPED`](#cld-stopped)
  - [`CLD_CONTINUED`](#cld-continued)
  - [`SIGEV_SIGNAL`](#sigev-signal)
  - [`SIGEV_NONE`](#sigev-none)
  - [`SIGEV_THREAD`](#sigev-thread)
  - [`P_ALL`](#p-all)
  - [`P_PID`](#p-pid)
  - [`P_PGID`](#p-pgid)
  - [`P_PIDFD`](#p-pidfd)
  - [`UTIME_OMIT`](#utime-omit)
  - [`UTIME_NOW`](#utime-now)
  - [`POLLIN`](#pollin)
  - [`POLLPRI`](#pollpri)
  - [`POLLOUT`](#pollout)
  - [`POLLERR`](#pollerr)
  - [`POLLHUP`](#pollhup)
  - [`POLLNVAL`](#pollnval)
  - [`POLLRDNORM`](#pollrdnorm)
  - [`POLLRDBAND`](#pollrdband)
  - [`POLLRDHUP`](#pollrdhup)
  - [`IPTOS_LOWDELAY`](#iptos-lowdelay)
  - [`IPTOS_THROUGHPUT`](#iptos-throughput)
  - [`IPTOS_RELIABILITY`](#iptos-reliability)
  - [`IPTOS_MINCOST`](#iptos-mincost)
  - [`IPTOS_PREC_NETCONTROL`](#iptos-prec-netcontrol)
  - [`IPTOS_PREC_INTERNETCONTROL`](#iptos-prec-internetcontrol)
  - [`IPTOS_PREC_CRITIC_ECP`](#iptos-prec-critic-ecp)
  - [`IPTOS_PREC_FLASHOVERRIDE`](#iptos-prec-flashoverride)
  - [`IPTOS_PREC_FLASH`](#iptos-prec-flash)
  - [`IPTOS_PREC_IMMEDIATE`](#iptos-prec-immediate)
  - [`IPTOS_PREC_PRIORITY`](#iptos-prec-priority)
  - [`IPTOS_PREC_ROUTINE`](#iptos-prec-routine)
  - [`IPTOS_ECN_MASK`](#iptos-ecn-mask)
  - [`IPTOS_ECN_ECT1`](#iptos-ecn-ect1)
  - [`IPTOS_ECN_ECT0`](#iptos-ecn-ect0)
  - [`IPTOS_ECN_CE`](#iptos-ecn-ce)
  - [`IPOPT_COPY`](#ipopt-copy)
  - [`IPOPT_CLASS_MASK`](#ipopt-class-mask)
  - [`IPOPT_NUMBER_MASK`](#ipopt-number-mask)
  - [`IPOPT_CONTROL`](#ipopt-control)
  - [`IPOPT_RESERVED1`](#ipopt-reserved1)
  - [`IPOPT_MEASUREMENT`](#ipopt-measurement)
  - [`IPOPT_RESERVED2`](#ipopt-reserved2)
  - [`IPOPT_END`](#ipopt-end)
  - [`IPOPT_NOOP`](#ipopt-noop)
  - [`IPOPT_SEC`](#ipopt-sec)
  - [`IPOPT_LSRR`](#ipopt-lsrr)
  - [`IPOPT_TIMESTAMP`](#ipopt-timestamp)
  - [`IPOPT_RR`](#ipopt-rr)
  - [`IPOPT_SID`](#ipopt-sid)
  - [`IPOPT_SSRR`](#ipopt-ssrr)
  - [`IPOPT_RA`](#ipopt-ra)
  - [`IPVERSION`](#ipversion)
  - [`MAXTTL`](#maxttl)
  - [`IPDEFTTL`](#ipdefttl)
  - [`IPOPT_OPTVAL`](#ipopt-optval)
  - [`IPOPT_OLEN`](#ipopt-olen)
  - [`IPOPT_OFFSET`](#ipopt-offset)
  - [`IPOPT_MINOFF`](#ipopt-minoff)
  - [`MAX_IPOPTLEN`](#max-ipoptlen)
  - [`IPOPT_NOP`](#ipopt-nop)
  - [`IPOPT_EOL`](#ipopt-eol)
  - [`IPOPT_TS`](#ipopt-ts)
  - [`IPOPT_TS_TSONLY`](#ipopt-ts-tsonly)
  - [`IPOPT_TS_TSANDADDR`](#ipopt-ts-tsandaddr)
  - [`IPOPT_TS_PRESPEC`](#ipopt-ts-prespec)
  - [`ARPOP_RREQUEST`](#arpop-rrequest)
  - [`ARPOP_RREPLY`](#arpop-rreply)
  - [`ARPOP_InREQUEST`](#arpop-inrequest)
  - [`ARPOP_InREPLY`](#arpop-inreply)
  - [`ARPOP_NAK`](#arpop-nak)
  - [`ATF_NETMASK`](#atf-netmask)
  - [`ATF_DONTPUB`](#atf-dontpub)
  - [`ARPHRD_NETROM`](#arphrd-netrom)
  - [`ARPHRD_ETHER`](#arphrd-ether)
  - [`ARPHRD_EETHER`](#arphrd-eether)
  - [`ARPHRD_AX25`](#arphrd-ax25)
  - [`ARPHRD_PRONET`](#arphrd-pronet)
  - [`ARPHRD_CHAOS`](#arphrd-chaos)
  - [`ARPHRD_IEEE802`](#arphrd-ieee802)
  - [`ARPHRD_ARCNET`](#arphrd-arcnet)
  - [`ARPHRD_APPLETLK`](#arphrd-appletlk)
  - [`ARPHRD_DLCI`](#arphrd-dlci)
  - [`ARPHRD_ATM`](#arphrd-atm)
  - [`ARPHRD_METRICOM`](#arphrd-metricom)
  - [`ARPHRD_IEEE1394`](#arphrd-ieee1394)
  - [`ARPHRD_EUI64`](#arphrd-eui64)
  - [`ARPHRD_INFINIBAND`](#arphrd-infiniband)
  - [`ARPHRD_SLIP`](#arphrd-slip)
  - [`ARPHRD_CSLIP`](#arphrd-cslip)
  - [`ARPHRD_SLIP6`](#arphrd-slip6)
  - [`ARPHRD_CSLIP6`](#arphrd-cslip6)
  - [`ARPHRD_RSRVD`](#arphrd-rsrvd)
  - [`ARPHRD_ADAPT`](#arphrd-adapt)
  - [`ARPHRD_ROSE`](#arphrd-rose)
  - [`ARPHRD_X25`](#arphrd-x25)
  - [`ARPHRD_HWX25`](#arphrd-hwx25)
  - [`ARPHRD_CAN`](#arphrd-can)
  - [`ARPHRD_PPP`](#arphrd-ppp)
  - [`ARPHRD_CISCO`](#arphrd-cisco)
  - [`ARPHRD_HDLC`](#arphrd-hdlc)
  - [`ARPHRD_LAPB`](#arphrd-lapb)
  - [`ARPHRD_DDCMP`](#arphrd-ddcmp)
  - [`ARPHRD_RAWHDLC`](#arphrd-rawhdlc)
  - [`ARPHRD_TUNNEL`](#arphrd-tunnel)
  - [`ARPHRD_TUNNEL6`](#arphrd-tunnel6)
  - [`ARPHRD_FRAD`](#arphrd-frad)
  - [`ARPHRD_SKIP`](#arphrd-skip)
  - [`ARPHRD_LOOPBACK`](#arphrd-loopback)
  - [`ARPHRD_LOCALTLK`](#arphrd-localtlk)
  - [`ARPHRD_FDDI`](#arphrd-fddi)
  - [`ARPHRD_BIF`](#arphrd-bif)
  - [`ARPHRD_SIT`](#arphrd-sit)
  - [`ARPHRD_IPDDP`](#arphrd-ipddp)
  - [`ARPHRD_IPGRE`](#arphrd-ipgre)
  - [`ARPHRD_PIMREG`](#arphrd-pimreg)
  - [`ARPHRD_HIPPI`](#arphrd-hippi)
  - [`ARPHRD_ASH`](#arphrd-ash)
  - [`ARPHRD_ECONET`](#arphrd-econet)
  - [`ARPHRD_IRDA`](#arphrd-irda)
  - [`ARPHRD_FCPP`](#arphrd-fcpp)
  - [`ARPHRD_FCAL`](#arphrd-fcal)
  - [`ARPHRD_FCPL`](#arphrd-fcpl)
  - [`ARPHRD_FCFABRIC`](#arphrd-fcfabric)
  - [`ARPHRD_IEEE802_TR`](#arphrd-ieee802-tr)
  - [`ARPHRD_IEEE80211`](#arphrd-ieee80211)
  - [`ARPHRD_IEEE80211_PRISM`](#arphrd-ieee80211-prism)
  - [`ARPHRD_IEEE80211_RADIOTAP`](#arphrd-ieee80211-radiotap)
  - [`ARPHRD_IEEE802154`](#arphrd-ieee802154)
  - [`ARPHRD_VOID`](#arphrd-void)
  - [`ARPHRD_NONE`](#arphrd-none)
  - [`IFF_TUN`](#iff-tun)
  - [`IFF_TAP`](#iff-tap)
  - [`IFF_NAPI`](#iff-napi)
  - [`IFF_NAPI_FRAGS`](#iff-napi-frags)
  - [`IFF_NO_CARRIER`](#iff-no-carrier)
  - [`IFF_NO_PI`](#iff-no-pi)
  - [`TUN_READQ_SIZE`](#tun-readq-size)
  - [`TUN_TUN_DEV`](#tun-tun-dev)
  - [`TUN_TAP_DEV`](#tun-tap-dev)
  - [`TUN_TYPE_MASK`](#tun-type-mask)
  - [`IFF_ONE_QUEUE`](#iff-one-queue)
  - [`IFF_VNET_HDR`](#iff-vnet-hdr)
  - [`IFF_TUN_EXCL`](#iff-tun-excl)
  - [`IFF_MULTI_QUEUE`](#iff-multi-queue)
  - [`IFF_ATTACH_QUEUE`](#iff-attach-queue)
  - [`IFF_DETACH_QUEUE`](#iff-detach-queue)
  - [`IFF_PERSIST`](#iff-persist)
  - [`IFF_NOFILTER`](#iff-nofilter)
  - [`TUN_TX_TIMESTAMP`](#tun-tx-timestamp)
  - [`TUN_F_CSUM`](#tun-f-csum)
  - [`TUN_F_TSO4`](#tun-f-tso4)
  - [`TUN_F_TSO6`](#tun-f-tso6)
  - [`TUN_F_TSO_ECN`](#tun-f-tso-ecn)
  - [`TUN_F_UFO`](#tun-f-ufo)
  - [`TUN_F_USO4`](#tun-f-uso4)
  - [`TUN_F_USO6`](#tun-f-uso6)
  - [`TUN_PKT_STRIP`](#tun-pkt-strip)
  - [`TUN_FLT_ALLMULTI`](#tun-flt-allmulti)
  - [`T_TYPE`](#t-type)
  - [`TUNSETNOCSUM`](#tunsetnocsum)
  - [`TUNSETDEBUG`](#tunsetdebug)
  - [`TUNSETIFF`](#tunsetiff)
  - [`TUNSETPERSIST`](#tunsetpersist)
  - [`TUNSETOWNER`](#tunsetowner)
  - [`TUNSETLINK`](#tunsetlink)
  - [`TUNSETGROUP`](#tunsetgroup)
  - [`TUNGETFEATURES`](#tungetfeatures)
  - [`TUNSETOFFLOAD`](#tunsetoffload)
  - [`TUNSETTXFILTER`](#tunsettxfilter)
  - [`TUNGETIFF`](#tungetiff)
  - [`TUNGETSNDBUF`](#tungetsndbuf)
  - [`TUNSETSNDBUF`](#tunsetsndbuf)
  - [`TUNATTACHFILTER`](#tunattachfilter)
  - [`TUNDETACHFILTER`](#tundetachfilter)
  - [`TUNGETVNETHDRSZ`](#tungetvnethdrsz)
  - [`TUNSETVNETHDRSZ`](#tunsetvnethdrsz)
  - [`TUNSETQUEUE`](#tunsetqueue)
  - [`TUNSETIFINDEX`](#tunsetifindex)
  - [`TUNGETFILTER`](#tungetfilter)
  - [`TUNSETVNETLE`](#tunsetvnetle)
  - [`TUNGETVNETLE`](#tungetvnetle)
  - [`TUNSETVNETBE`](#tunsetvnetbe)
  - [`TUNGETVNETBE`](#tungetvnetbe)
  - [`TUNSETSTEERINGEBPF`](#tunsetsteeringebpf)
  - [`TUNSETFILTEREBPF`](#tunsetfilterebpf)
  - [`TUNSETCARRIER`](#tunsetcarrier)
  - [`TUNGETDEVNETNS`](#tungetdevnetns)
  - [`FS_IOC_GETFLAGS`](#fs-ioc-getflags)
  - [`FS_IOC_SETFLAGS`](#fs-ioc-setflags)
  - [`FS_IOC_GETVERSION`](#fs-ioc-getversion)
  - [`FS_IOC_SETVERSION`](#fs-ioc-setversion)
  - [`FS_IOC32_GETFLAGS`](#fs-ioc32-getflags)
  - [`FS_IOC32_SETFLAGS`](#fs-ioc32-setflags)
  - [`FS_IOC32_GETVERSION`](#fs-ioc32-getversion)
  - [`FS_IOC32_SETVERSION`](#fs-ioc32-setversion)
  - [`FICLONE`](#ficlone)
  - [`FICLONERANGE`](#ficlonerange)
  - [`ADFS_SUPER_MAGIC`](#adfs-super-magic)
  - [`AFFS_SUPER_MAGIC`](#affs-super-magic)
  - [`AFS_SUPER_MAGIC`](#afs-super-magic)
  - [`AUTOFS_SUPER_MAGIC`](#autofs-super-magic)
  - [`BPF_FS_MAGIC`](#bpf-fs-magic)
  - [`BTRFS_SUPER_MAGIC`](#btrfs-super-magic)
  - [`CGROUP2_SUPER_MAGIC`](#cgroup2-super-magic)
  - [`CGROUP_SUPER_MAGIC`](#cgroup-super-magic)
  - [`CODA_SUPER_MAGIC`](#coda-super-magic)
  - [`CRAMFS_MAGIC`](#cramfs-magic)
  - [`DEBUGFS_MAGIC`](#debugfs-magic)
  - [`DEVPTS_SUPER_MAGIC`](#devpts-super-magic)
  - [`ECRYPTFS_SUPER_MAGIC`](#ecryptfs-super-magic)
  - [`EFS_SUPER_MAGIC`](#efs-super-magic)
  - [`EXT2_SUPER_MAGIC`](#ext2-super-magic)
  - [`EXT3_SUPER_MAGIC`](#ext3-super-magic)
  - [`EXT4_SUPER_MAGIC`](#ext4-super-magic)
  - [`F2FS_SUPER_MAGIC`](#f2fs-super-magic)
  - [`FUSE_SUPER_MAGIC`](#fuse-super-magic)
  - [`FUTEXFS_SUPER_MAGIC`](#futexfs-super-magic)
  - [`HOSTFS_SUPER_MAGIC`](#hostfs-super-magic)
  - [`HPFS_SUPER_MAGIC`](#hpfs-super-magic)
  - [`HUGETLBFS_MAGIC`](#hugetlbfs-magic)
  - [`ISOFS_SUPER_MAGIC`](#isofs-super-magic)
  - [`JFFS2_SUPER_MAGIC`](#jffs2-super-magic)
  - [`MINIX2_SUPER_MAGIC2`](#minix2-super-magic2)
  - [`MINIX2_SUPER_MAGIC`](#minix2-super-magic)
  - [`MINIX3_SUPER_MAGIC`](#minix3-super-magic)
  - [`MINIX_SUPER_MAGIC2`](#minix-super-magic2)
  - [`MINIX_SUPER_MAGIC`](#minix-super-magic)
  - [`MSDOS_SUPER_MAGIC`](#msdos-super-magic)
  - [`NCP_SUPER_MAGIC`](#ncp-super-magic)
  - [`NFS_SUPER_MAGIC`](#nfs-super-magic)
  - [`NILFS_SUPER_MAGIC`](#nilfs-super-magic)
  - [`OCFS2_SUPER_MAGIC`](#ocfs2-super-magic)
  - [`OPENPROM_SUPER_MAGIC`](#openprom-super-magic)
  - [`OVERLAYFS_SUPER_MAGIC`](#overlayfs-super-magic)
  - [`PROC_SUPER_MAGIC`](#proc-super-magic)
  - [`QNX4_SUPER_MAGIC`](#qnx4-super-magic)
  - [`QNX6_SUPER_MAGIC`](#qnx6-super-magic)
  - [`RDTGROUP_SUPER_MAGIC`](#rdtgroup-super-magic)
  - [`REISERFS_SUPER_MAGIC`](#reiserfs-super-magic)
  - [`SECURITYFS_MAGIC`](#securityfs-magic)
  - [`SELINUX_MAGIC`](#selinux-magic)
  - [`SMACK_MAGIC`](#smack-magic)
  - [`SMB_SUPER_MAGIC`](#smb-super-magic)
  - [`SYSFS_MAGIC`](#sysfs-magic)
  - [`TMPFS_MAGIC`](#tmpfs-magic)
  - [`TRACEFS_MAGIC`](#tracefs-magic)
  - [`UDF_SUPER_MAGIC`](#udf-super-magic)
  - [`USBDEVICE_SUPER_MAGIC`](#usbdevice-super-magic)
  - [`XENFS_SUPER_MAGIC`](#xenfs-super-magic)
  - [`NSFS_MAGIC`](#nsfs-magic)
  - [`AT_STATX_SYNC_TYPE`](#at-statx-sync-type)
  - [`AT_STATX_SYNC_AS_STAT`](#at-statx-sync-as-stat)
  - [`AT_STATX_FORCE_SYNC`](#at-statx-force-sync)
  - [`AT_STATX_DONT_SYNC`](#at-statx-dont-sync)
  - [`STATX_TYPE`](#statx-type)
  - [`STATX_MODE`](#statx-mode)
  - [`STATX_NLINK`](#statx-nlink)
  - [`STATX_UID`](#statx-uid)
  - [`STATX_GID`](#statx-gid)
  - [`STATX_ATIME`](#statx-atime)
  - [`STATX_MTIME`](#statx-mtime)
  - [`STATX_CTIME`](#statx-ctime)
  - [`STATX_INO`](#statx-ino)
  - [`STATX_SIZE`](#statx-size)
  - [`STATX_BLOCKS`](#statx-blocks)
  - [`STATX_BASIC_STATS`](#statx-basic-stats)
  - [`STATX_BTIME`](#statx-btime)
  - [`STATX_ALL`](#statx-all)
  - [`STATX_MNT_ID`](#statx-mnt-id)
  - [`STATX_DIOALIGN`](#statx-dioalign)
  - [`STATX__RESERVED`](#statx-reserved)
  - [`STATX_ATTR_COMPRESSED`](#statx-attr-compressed)
  - [`STATX_ATTR_IMMUTABLE`](#statx-attr-immutable)
  - [`STATX_ATTR_APPEND`](#statx-attr-append)
  - [`STATX_ATTR_NODUMP`](#statx-attr-nodump)
  - [`STATX_ATTR_ENCRYPTED`](#statx-attr-encrypted)
  - [`STATX_ATTR_AUTOMOUNT`](#statx-attr-automount)
  - [`STATX_ATTR_MOUNT_ROOT`](#statx-attr-mount-root)
  - [`STATX_ATTR_VERITY`](#statx-attr-verity)
  - [`STATX_ATTR_DAX`](#statx-attr-dax)
  - [`_IOC_NRBITS`](#ioc-nrbits)
  - [`_IOC_TYPEBITS`](#ioc-typebits)
  - [`_IOC_SIZEBITS`](#ioc-sizebits)
  - [`_IOC_DIRBITS`](#ioc-dirbits)
  - [`_IOC_NONE`](#ioc-none)
  - [`_IOC_WRITE`](#ioc-write)
  - [`_IOC_READ`](#ioc-read)
  - [`_IOC_NRMASK`](#ioc-nrmask)
  - [`_IOC_TYPEMASK`](#ioc-typemask)
  - [`_IOC_SIZEMASK`](#ioc-sizemask)
  - [`_IOC_DIRMASK`](#ioc-dirmask)
  - [`_IOC_NRSHIFT`](#ioc-nrshift)
  - [`_IOC_TYPESHIFT`](#ioc-typeshift)
  - [`_IOC_SIZESHIFT`](#ioc-sizeshift)
  - [`_IOC_DIRSHIFT`](#ioc-dirshift)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`linux_like`](#linux-like) | mod |  |
| [`linux`](#linux) | mod | Linux-specific definitions for linux-like values |
| [`linux_l4re_shared`](#linux-l4re-shared) | mod |  |
| [`group`](#group) | struct |  |
| [`utimbuf`](#utimbuf) | struct |  |
| [`timeval`](#timeval) | struct |  |
| [`rlimit`](#rlimit) | struct |  |
| [`rusage`](#rusage) | struct |  |
| [`ipv6_mreq`](#ipv6-mreq) | struct |  |
| [`hostent`](#hostent) | struct |  |
| [`iovec`](#iovec) | struct |  |
| [`pollfd`](#pollfd) | struct |  |
| [`winsize`](#winsize) | struct |  |
| [`linger`](#linger) | struct |  |
| [`sigval`](#sigval) | struct |  |
| [`itimerval`](#itimerval) | struct |  |
| [`tms`](#tms) | struct |  |
| [`servent`](#servent) | struct |  |
| [`protoent`](#protoent) | struct |  |
| [`in6_addr`](#in6-addr) | struct |  |
| [`in_addr`](#in-addr) | struct |  |
| [`ip_mreq`](#ip-mreq) | struct |  |
| [`ip_mreqn`](#ip-mreqn) | struct |  |
| [`ip_mreq_source`](#ip-mreq-source) | struct |  |
| [`sockaddr`](#sockaddr) | struct |  |
| [`sockaddr_in`](#sockaddr-in) | struct |  |
| [`sockaddr_in6`](#sockaddr-in6) | struct |  |
| [`addrinfo`](#addrinfo) | struct |  |
| [`sockaddr_ll`](#sockaddr-ll) | struct |  |
| [`fd_set`](#fd-set) | struct |  |
| [`tm`](#tm) | struct |  |
| [`sched_param`](#sched-param) | struct |  |
| [`Dl_info`](#dl-info) | struct |  |
| [`lconv`](#lconv) | struct |  |
| [`in_pktinfo`](#in-pktinfo) | struct |  |
| [`ifaddrs`](#ifaddrs) | struct |  |
| [`in6_rtmsg`](#in6-rtmsg) | struct |  |
| [`arpreq`](#arpreq) | struct |  |
| [`arpreq_old`](#arpreq-old) | struct |  |
| [`arphdr`](#arphdr) | struct |  |
| [`mmsghdr`](#mmsghdr) | struct |  |
| [`sockaddr_un`](#sockaddr-un) | struct |  |
| [`sockaddr_storage`](#sockaddr-storage) | struct |  |
| [`utsname`](#utsname) | struct |  |
| [`if_nameindex`](#if-nameindex) | struct |  |
| [`file_clone_range`](#file-clone-range) | struct |  |
| [`sock_filter`](#sock-filter) | struct |  |
| [`sock_fprog`](#sock-fprog) | struct |  |
| [`statx`](#statx) | struct |  |
| [`statx_timestamp`](#statx-timestamp) | struct |  |
| [`epoll_event`](#epoll-event) | struct |  |
| [`sigevent`](#sigevent) | struct |  |
| [`DIR`](#dir) | enum |  |
| [`FILE`](#file) | enum |  |
| [`timezone`](#timezone) | enum |  |
| [`isalnum`](#isalnum) | fn |  |
| [`isalpha`](#isalpha) | fn |  |
| [`iscntrl`](#iscntrl) | fn |  |
| [`isdigit`](#isdigit) | fn |  |
| [`isgraph`](#isgraph) | fn |  |
| [`islower`](#islower) | fn |  |
| [`isprint`](#isprint) | fn |  |
| [`ispunct`](#ispunct) | fn |  |
| [`isspace`](#isspace) | fn |  |
| [`isupper`](#isupper) | fn |  |
| [`isxdigit`](#isxdigit) | fn |  |
| [`isblank`](#isblank) | fn |  |
| [`tolower`](#tolower) | fn |  |
| [`toupper`](#toupper) | fn |  |
| [`qsort`](#qsort) | fn |  |
| [`bsearch`](#bsearch) | fn |  |
| [`fopen`](#fopen) | fn |  |
| [`freopen`](#freopen) | fn |  |
| [`fflush`](#fflush) | fn |  |
| [`fclose`](#fclose) | fn |  |
| [`remove`](#remove) | fn |  |
| [`rename`](#rename) | fn |  |
| [`tmpfile`](#tmpfile) | fn |  |
| [`setvbuf`](#setvbuf) | fn |  |
| [`setbuf`](#setbuf) | fn |  |
| [`getchar`](#getchar) | fn |  |
| [`putchar`](#putchar) | fn |  |
| [`fgetc`](#fgetc) | fn |  |
| [`fgets`](#fgets) | fn |  |
| [`fputc`](#fputc) | fn |  |
| [`fputs`](#fputs) | fn |  |
| [`puts`](#puts) | fn |  |
| [`ungetc`](#ungetc) | fn |  |
| [`fread`](#fread) | fn |  |
| [`fwrite`](#fwrite) | fn |  |
| [`fseek`](#fseek) | fn |  |
| [`ftell`](#ftell) | fn |  |
| [`rewind`](#rewind) | fn |  |
| [`fgetpos`](#fgetpos) | fn |  |
| [`fsetpos`](#fsetpos) | fn |  |
| [`feof`](#feof) | fn |  |
| [`ferror`](#ferror) | fn |  |
| [`clearerr`](#clearerr) | fn |  |
| [`perror`](#perror) | fn |  |
| [`atof`](#atof) | fn |  |
| [`atoi`](#atoi) | fn |  |
| [`atol`](#atol) | fn |  |
| [`atoll`](#atoll) | fn |  |
| [`strtod`](#strtod) | fn |  |
| [`strtof`](#strtof) | fn |  |
| [`strtol`](#strtol) | fn |  |
| [`strtoll`](#strtoll) | fn |  |
| [`strtoul`](#strtoul) | fn |  |
| [`strtoull`](#strtoull) | fn |  |
| [`calloc`](#calloc) | fn |  |
| [`malloc`](#malloc) | fn |  |
| [`realloc`](#realloc) | fn |  |
| [`free`](#free) | fn |  |
| [`abort`](#abort) | fn |  |
| [`exit`](#exit) | fn |  |
| [`_exit`](#exit) | fn |  |
| [`system`](#system) | fn |  |
| [`getenv`](#getenv) | fn |  |
| [`strcpy`](#strcpy) | fn |  |
| [`strncpy`](#strncpy) | fn |  |
| [`stpcpy`](#stpcpy) | fn |  |
| [`strcat`](#strcat) | fn |  |
| [`strncat`](#strncat) | fn |  |
| [`strcmp`](#strcmp) | fn |  |
| [`strncmp`](#strncmp) | fn |  |
| [`strcoll`](#strcoll) | fn |  |
| [`strchr`](#strchr) | fn |  |
| [`strrchr`](#strrchr) | fn |  |
| [`strspn`](#strspn) | fn |  |
| [`strcspn`](#strcspn) | fn |  |
| [`strdup`](#strdup) | fn |  |
| [`strndup`](#strndup) | fn |  |
| [`strpbrk`](#strpbrk) | fn |  |
| [`strstr`](#strstr) | fn |  |
| [`strcasecmp`](#strcasecmp) | fn |  |
| [`strncasecmp`](#strncasecmp) | fn |  |
| [`strlen`](#strlen) | fn |  |
| [`strnlen`](#strnlen) | fn |  |
| [`strerror`](#strerror) | fn |  |
| [`strtok`](#strtok) | fn |  |
| [`strtok_r`](#strtok-r) | fn |  |
| [`strxfrm`](#strxfrm) | fn |  |
| [`strsignal`](#strsignal) | fn |  |
| [`wcslen`](#wcslen) | fn |  |
| [`wcstombs`](#wcstombs) | fn |  |
| [`memchr`](#memchr) | fn |  |
| [`wmemchr`](#wmemchr) | fn |  |
| [`memcmp`](#memcmp) | fn |  |
| [`memcpy`](#memcpy) | fn |  |
| [`memmove`](#memmove) | fn |  |
| [`memset`](#memset) | fn |  |
| [`memccpy`](#memccpy) | fn |  |
| [`getpwnam`](#getpwnam) | fn |  |
| [`getpwuid`](#getpwuid) | fn |  |
| [`fprintf`](#fprintf) | fn |  |
| [`printf`](#printf) | fn |  |
| [`snprintf`](#snprintf) | fn |  |
| [`sprintf`](#sprintf) | fn |  |
| [`fscanf`](#fscanf) | fn |  |
| [`scanf`](#scanf) | fn |  |
| [`sscanf`](#sscanf) | fn |  |
| [`getchar_unlocked`](#getchar-unlocked) | fn |  |
| [`putchar_unlocked`](#putchar-unlocked) | fn |  |
| [`socket`](#socket) | fn |  |
| [`connect`](#connect) | fn |  |
| [`listen`](#listen) | fn |  |
| [`accept`](#accept) | fn |  |
| [`getpeername`](#getpeername) | fn |  |
| [`getsockname`](#getsockname) | fn |  |
| [`setsockopt`](#setsockopt) | fn |  |
| [`socketpair`](#socketpair) | fn |  |
| [`sendto`](#sendto) | fn |  |
| [`shutdown`](#shutdown) | fn |  |
| [`chmod`](#chmod) | fn |  |
| [`fchmod`](#fchmod) | fn |  |
| [`fstat`](#fstat) | fn |  |
| [`mkdir`](#mkdir) | fn |  |
| [`stat`](#stat) | fn |  |
| [`pclose`](#pclose) | fn |  |
| [`fdopen`](#fdopen) | fn |  |
| [`fileno`](#fileno) | fn |  |
| [`open`](#open) | fn |  |
| [`creat`](#creat) | fn |  |
| [`fcntl`](#fcntl) | fn |  |
| [`opendir`](#opendir) | fn |  |
| [`readdir`](#readdir) | fn |  |
| [`closedir`](#closedir) | fn |  |
| [`rewinddir`](#rewinddir) | fn |  |
| [`fchmodat`](#fchmodat) | fn |  |
| [`fchown`](#fchown) | fn |  |
| [`fchownat`](#fchownat) | fn |  |
| [`fstatat`](#fstatat) | fn |  |
| [`linkat`](#linkat) | fn |  |
| [`renameat`](#renameat) | fn |  |
| [`symlinkat`](#symlinkat) | fn |  |
| [`unlinkat`](#unlinkat) | fn |  |
| [`access`](#access) | fn |  |
| [`alarm`](#alarm) | fn |  |
| [`chdir`](#chdir) | fn |  |
| [`fchdir`](#fchdir) | fn |  |
| [`chown`](#chown) | fn |  |
| [`lchown`](#lchown) | fn |  |
| [`close`](#close) | fn |  |
| [`dup`](#dup) | fn |  |
| [`dup2`](#dup2) | fn |  |
| [`execl`](#execl) | fn |  |
| [`execle`](#execle) | fn |  |
| [`execlp`](#execlp) | fn |  |
| [`execv`](#execv) | fn |  |
| [`execve`](#execve) | fn |  |
| [`execvp`](#execvp) | fn |  |
| [`fork`](#fork) | fn |  |
| [`fpathconf`](#fpathconf) | fn |  |
| [`getcwd`](#getcwd) | fn |  |
| [`getegid`](#getegid) | fn |  |
| [`geteuid`](#geteuid) | fn |  |
| [`getgid`](#getgid) | fn |  |
| [`getgroups`](#getgroups) | fn |  |
| [`getlogin`](#getlogin) | fn |  |
| [`getopt`](#getopt) | fn |  |
| [`getpgid`](#getpgid) | fn |  |
| [`getpgrp`](#getpgrp) | fn |  |
| [`getpid`](#getpid) | fn |  |
| [`getppid`](#getppid) | fn |  |
| [`getuid`](#getuid) | fn |  |
| [`isatty`](#isatty) | fn |  |
| [`link`](#link) | fn |  |
| [`lseek`](#lseek) | fn |  |
| [`pathconf`](#pathconf) | fn |  |
| [`pipe`](#pipe) | fn |  |
| [`posix_memalign`](#posix-memalign) | fn |  |
| [`aligned_alloc`](#aligned-alloc) | fn |  |
| [`read`](#read) | fn |  |
| [`rmdir`](#rmdir) | fn |  |
| [`seteuid`](#seteuid) | fn |  |
| [`setegid`](#setegid) | fn |  |
| [`setgid`](#setgid) | fn |  |
| [`setpgid`](#setpgid) | fn |  |
| [`setsid`](#setsid) | fn |  |
| [`setuid`](#setuid) | fn |  |
| [`setreuid`](#setreuid) | fn |  |
| [`setregid`](#setregid) | fn |  |
| [`sleep`](#sleep) | fn |  |
| [`nanosleep`](#nanosleep) | fn |  |
| [`tcgetpgrp`](#tcgetpgrp) | fn |  |
| [`tcsetpgrp`](#tcsetpgrp) | fn |  |
| [`ttyname`](#ttyname) | fn |  |
| [`ttyname_r`](#ttyname-r) | fn |  |
| [`unlink`](#unlink) | fn |  |
| [`wait`](#wait) | fn |  |
| [`waitpid`](#waitpid) | fn |  |
| [`write`](#write) | fn |  |
| [`pread`](#pread) | fn |  |
| [`pwrite`](#pwrite) | fn |  |
| [`umask`](#umask) | fn |  |
| [`utime`](#utime) | fn |  |
| [`kill`](#kill) | fn |  |
| [`killpg`](#killpg) | fn |  |
| [`mlock`](#mlock) | fn |  |
| [`munlock`](#munlock) | fn |  |
| [`mlockall`](#mlockall) | fn |  |
| [`munlockall`](#munlockall) | fn |  |
| [`mmap`](#mmap) | fn |  |
| [`munmap`](#munmap) | fn |  |
| [`if_nametoindex`](#if-nametoindex) | fn |  |
| [`if_indextoname`](#if-indextoname) | fn |  |
| [`lstat`](#lstat) | fn |  |
| [`fsync`](#fsync) | fn |  |
| [`setenv`](#setenv) | fn |  |
| [`unsetenv`](#unsetenv) | fn |  |
| [`symlink`](#symlink) | fn |  |
| [`truncate`](#truncate) | fn |  |
| [`ftruncate`](#ftruncate) | fn |  |
| [`signal`](#signal) | fn |  |
| [`getrusage`](#getrusage) | fn |  |
| [`realpath`](#realpath) | fn |  |
| [`times`](#times) | fn |  |
| [`pthread_self`](#pthread-self) | fn |  |
| [`pthread_equal`](#pthread-equal) | fn |  |
| [`pthread_join`](#pthread-join) | fn |  |
| [`pthread_exit`](#pthread-exit) | fn |  |
| [`pthread_attr_init`](#pthread-attr-init) | fn |  |
| [`pthread_attr_destroy`](#pthread-attr-destroy) | fn |  |
| [`pthread_attr_getstacksize`](#pthread-attr-getstacksize) | fn |  |
| [`pthread_attr_setstacksize`](#pthread-attr-setstacksize) | fn |  |
| [`pthread_attr_setdetachstate`](#pthread-attr-setdetachstate) | fn |  |
| [`pthread_detach`](#pthread-detach) | fn |  |
| [`sched_yield`](#sched-yield) | fn |  |
| [`pthread_key_create`](#pthread-key-create) | fn |  |
| [`pthread_key_delete`](#pthread-key-delete) | fn |  |
| [`pthread_getspecific`](#pthread-getspecific) | fn |  |
| [`pthread_setspecific`](#pthread-setspecific) | fn |  |
| [`pthread_mutex_init`](#pthread-mutex-init) | fn |  |
| [`pthread_mutex_destroy`](#pthread-mutex-destroy) | fn |  |
| [`pthread_mutex_lock`](#pthread-mutex-lock) | fn |  |
| [`pthread_mutex_trylock`](#pthread-mutex-trylock) | fn |  |
| [`pthread_mutex_unlock`](#pthread-mutex-unlock) | fn |  |
| [`pthread_mutexattr_init`](#pthread-mutexattr-init) | fn |  |
| [`pthread_mutexattr_destroy`](#pthread-mutexattr-destroy) | fn |  |
| [`pthread_mutexattr_settype`](#pthread-mutexattr-settype) | fn |  |
| [`pthread_cond_init`](#pthread-cond-init) | fn |  |
| [`pthread_cond_wait`](#pthread-cond-wait) | fn |  |
| [`pthread_cond_timedwait`](#pthread-cond-timedwait) | fn |  |
| [`pthread_cond_signal`](#pthread-cond-signal) | fn |  |
| [`pthread_cond_broadcast`](#pthread-cond-broadcast) | fn |  |
| [`pthread_cond_destroy`](#pthread-cond-destroy) | fn |  |
| [`pthread_condattr_init`](#pthread-condattr-init) | fn |  |
| [`pthread_condattr_destroy`](#pthread-condattr-destroy) | fn |  |
| [`pthread_rwlock_init`](#pthread-rwlock-init) | fn |  |
| [`pthread_rwlock_destroy`](#pthread-rwlock-destroy) | fn |  |
| [`pthread_rwlock_rdlock`](#pthread-rwlock-rdlock) | fn |  |
| [`pthread_rwlock_tryrdlock`](#pthread-rwlock-tryrdlock) | fn |  |
| [`pthread_rwlock_wrlock`](#pthread-rwlock-wrlock) | fn |  |
| [`pthread_rwlock_trywrlock`](#pthread-rwlock-trywrlock) | fn |  |
| [`pthread_rwlock_unlock`](#pthread-rwlock-unlock) | fn |  |
| [`pthread_rwlockattr_init`](#pthread-rwlockattr-init) | fn |  |
| [`pthread_rwlockattr_destroy`](#pthread-rwlockattr-destroy) | fn |  |
| [`getsockopt`](#getsockopt) | fn |  |
| [`raise`](#raise) | fn |  |
| [`utimes`](#utimes) | fn |  |
| [`dlopen`](#dlopen) | fn |  |
| [`dlerror`](#dlerror) | fn |  |
| [`dlsym`](#dlsym) | fn |  |
| [`dlclose`](#dlclose) | fn |  |
| [`getaddrinfo`](#getaddrinfo) | fn |  |
| [`freeaddrinfo`](#freeaddrinfo) | fn |  |
| [`hstrerror`](#hstrerror) | fn |  |
| [`gai_strerror`](#gai-strerror) | fn |  |
| [`res_init`](#res-init) | fn |  |
| [`gmtime_r`](#gmtime-r) | fn |  |
| [`localtime_r`](#localtime-r) | fn |  |
| [`mktime`](#mktime) | fn |  |
| [`time`](#time) | fn |  |
| [`gmtime`](#gmtime) | fn |  |
| [`localtime`](#localtime) | fn |  |
| [`difftime`](#difftime) | fn |  |
| [`timegm`](#timegm) | fn |  |
| [`mknod`](#mknod) | fn |  |
| [`gethostname`](#gethostname) | fn |  |
| [`endservent`](#endservent) | fn |  |
| [`getservbyname`](#getservbyname) | fn |  |
| [`getservbyport`](#getservbyport) | fn |  |
| [`getservent`](#getservent) | fn |  |
| [`setservent`](#setservent) | fn |  |
| [`getprotobyname`](#getprotobyname) | fn |  |
| [`getprotobynumber`](#getprotobynumber) | fn |  |
| [`chroot`](#chroot) | fn |  |
| [`usleep`](#usleep) | fn |  |
| [`send`](#send) | fn |  |
| [`recv`](#recv) | fn |  |
| [`putenv`](#putenv) | fn |  |
| [`poll`](#poll) | fn |  |
| [`select`](#select) | fn |  |
| [`setlocale`](#setlocale) | fn |  |
| [`localeconv`](#localeconv) | fn |  |
| [`sem_wait`](#sem-wait) | fn |  |
| [`sem_trywait`](#sem-trywait) | fn |  |
| [`sem_post`](#sem-post) | fn |  |
| [`statvfs`](#statvfs) | fn |  |
| [`fstatvfs`](#fstatvfs) | fn |  |
| [`sigemptyset`](#sigemptyset) | fn |  |
| [`sigaddset`](#sigaddset) | fn |  |
| [`sigfillset`](#sigfillset) | fn |  |
| [`sigdelset`](#sigdelset) | fn |  |
| [`sigismember`](#sigismember) | fn |  |
| [`sigprocmask`](#sigprocmask) | fn |  |
| [`sigpending`](#sigpending) | fn |  |
| [`sysconf`](#sysconf) | fn |  |
| [`mkfifo`](#mkfifo) | fn |  |
| [`fseeko`](#fseeko) | fn |  |
| [`ftello`](#ftello) | fn |  |
| [`tcdrain`](#tcdrain) | fn |  |
| [`cfgetispeed`](#cfgetispeed) | fn |  |
| [`cfgetospeed`](#cfgetospeed) | fn |  |
| [`cfsetispeed`](#cfsetispeed) | fn |  |
| [`cfsetospeed`](#cfsetospeed) | fn |  |
| [`tcgetattr`](#tcgetattr) | fn |  |
| [`tcsetattr`](#tcsetattr) | fn |  |
| [`tcflow`](#tcflow) | fn |  |
| [`tcflush`](#tcflush) | fn |  |
| [`tcgetsid`](#tcgetsid) | fn |  |
| [`tcsendbreak`](#tcsendbreak) | fn |  |
| [`mkstemp`](#mkstemp) | fn |  |
| [`mkdtemp`](#mkdtemp) | fn |  |
| [`tmpnam`](#tmpnam) | fn |  |
| [`openlog`](#openlog) | fn |  |
| [`closelog`](#closelog) | fn |  |
| [`setlogmask`](#setlogmask) | fn |  |
| [`syslog`](#syslog) | fn |  |
| [`nice`](#nice) | fn |  |
| [`grantpt`](#grantpt) | fn |  |
| [`posix_openpt`](#posix-openpt) | fn |  |
| [`ptsname`](#ptsname) | fn |  |
| [`unlockpt`](#unlockpt) | fn |  |
| [`strcasestr`](#strcasestr) | fn |  |
| [`getline`](#getline) | fn |  |
| [`lockf`](#lockf) | fn |  |
| [`adjtime`](#adjtime) | fn |  |
| [`stpncpy`](#stpncpy) | fn |  |
| [`sigqueue`](#sigqueue) | fn |  |
| [`confstr`](#confstr) | fn |  |
| [`dladdr`](#dladdr) | fn |  |
| [`flock`](#flock) | fn |  |
| [`open_wmemstream`](#open-wmemstream) | fn |  |
| [`getsid`](#getsid) | fn |  |
| [`pause`](#pause) | fn |  |
| [`mkdirat`](#mkdirat) | fn |  |
| [`openat`](#openat) | fn |  |
| [`fdopendir`](#fdopendir) | fn |  |
| [`readdir_r`](#readdir-r) | fn | The 64-bit libc on Solaris and illumos only has readdir_r. |
| [`readlinkat`](#readlinkat) | fn |  |
| [`fmemopen`](#fmemopen) | fn |  |
| [`open_memstream`](#open-memstream) | fn |  |
| [`atexit`](#atexit) | fn |  |
| [`sigaction`](#sigaction) | fn |  |
| [`readlink`](#readlink) | fn |  |
| [`pselect`](#pselect) | fn |  |
| [`cfmakeraw`](#cfmakeraw) | fn |  |
| [`cfsetspeed`](#cfsetspeed) | fn |  |
| [`fnmatch`](#fnmatch) | fn |  |
| [`htonl`](#htonl) | fn |  |
| [`htons`](#htons) | fn |  |
| [`ntohl`](#ntohl) | fn |  |
| [`ntohs`](#ntohs) | fn |  |
| [`ioctl`](#ioctl) | fn |  |
| [`sem_destroy`](#sem-destroy) | fn |  |
| [`sem_init`](#sem-init) | fn |  |
| [`fdatasync`](#fdatasync) | fn |  |
| [`mincore`](#mincore) | fn |  |
| [`clock_getres`](#clock-getres) | fn |  |
| [`clock_gettime`](#clock-gettime) | fn |  |
| [`clock_settime`](#clock-settime) | fn |  |
| [`clock_getcpuclockid`](#clock-getcpuclockid) | fn |  |
| [`getitimer`](#getitimer) | fn |  |
| [`setitimer`](#setitimer) | fn |  |
| [`dirfd`](#dirfd) | fn |  |
| [`memalign`](#memalign) | fn |  |
| [`setgroups`](#setgroups) | fn |  |
| [`pipe2`](#pipe2) | fn |  |
| [`statfs`](#statfs) | fn |  |
| [`fstatfs`](#fstatfs) | fn |  |
| [`memrchr`](#memrchr) | fn |  |
| [`posix_fadvise`](#posix-fadvise) | fn |  |
| [`futimens`](#futimens) | fn |  |
| [`utimensat`](#utimensat) | fn |  |
| [`duplocale`](#duplocale) | fn |  |
| [`freelocale`](#freelocale) | fn |  |
| [`newlocale`](#newlocale) | fn |  |
| [`uselocale`](#uselocale) | fn |  |
| [`mknodat`](#mknodat) | fn |  |
| [`ptsname_r`](#ptsname-r) | fn |  |
| [`clearenv`](#clearenv) | fn |  |
| [`waitid`](#waitid) | fn |  |
| [`getresuid`](#getresuid) | fn |  |
| [`getresgid`](#getresgid) | fn |  |
| [`acct`](#acct) | fn |  |
| [`brk`](#brk) | fn |  |
| [`sbrk`](#sbrk) | fn |  |
| [`vfork`](#vfork) | fn |  |
| [`setresgid`](#setresgid) | fn |  |
| [`setresuid`](#setresuid) | fn |  |
| [`wait4`](#wait4) | fn |  |
| [`login_tty`](#login-tty) | fn |  |
| [`execvpe`](#execvpe) | fn |  |
| [`fexecve`](#fexecve) | fn |  |
| [`getifaddrs`](#getifaddrs) | fn |  |
| [`freeifaddrs`](#freeifaddrs) | fn |  |
| [`bind`](#bind) | fn |  |
| [`writev`](#writev) | fn |  |
| [`readv`](#readv) | fn |  |
| [`sendmsg`](#sendmsg) | fn |  |
| [`recvmsg`](#recvmsg) | fn |  |
| [`uname`](#uname) | fn |  |
| [`strchrnul`](#strchrnul) | fn |  |
| [`strftime`](#strftime) | fn |  |
| [`strftime_l`](#strftime-l) | fn |  |
| [`strptime`](#strptime) | fn |  |
| [`mkostemp`](#mkostemp) | fn |  |
| [`mkostemps`](#mkostemps) | fn |  |
| [`getdomainname`](#getdomainname) | fn |  |
| [`setdomainname`](#setdomainname) | fn |  |
| [`if_nameindex`](#if-nameindex) | fn |  |
| [`if_freenameindex`](#if-freenameindex) | fn |  |
| [`getpwnam_r`](#getpwnam-r) | fn |  |
| [`getpwuid_r`](#getpwuid-r) | fn |  |
| [`fstatfs64`](#fstatfs64) | fn |  |
| [`statvfs64`](#statvfs64) | fn |  |
| [`fstatvfs64`](#fstatvfs64) | fn |  |
| [`statfs64`](#statfs64) | fn |  |
| [`creat64`](#creat64) | fn |  |
| [`fstat64`](#fstat64) | fn |  |
| [`fstatat64`](#fstatat64) | fn |  |
| [`ftruncate64`](#ftruncate64) | fn |  |
| [`lseek64`](#lseek64) | fn |  |
| [`lstat64`](#lstat64) | fn |  |
| [`mmap64`](#mmap64) | fn |  |
| [`open64`](#open64) | fn |  |
| [`openat64`](#openat64) | fn |  |
| [`posix_fadvise64`](#posix-fadvise64) | fn |  |
| [`pread64`](#pread64) | fn |  |
| [`pwrite64`](#pwrite64) | fn |  |
| [`readdir64`](#readdir64) | fn |  |
| [`readdir64_r`](#readdir64-r) | fn |  |
| [`stat64`](#stat64) | fn |  |
| [`truncate64`](#truncate64) | fn |  |
| [`preadv64`](#preadv64) | fn |  |
| [`pwritev64`](#pwritev64) | fn |  |
| [`forkpty`](#forkpty) | fn |  |
| [`openpty`](#openpty) | fn |  |
| [`statx`](#statx) | fn |  |
| [`_IOC`](#ioc) | fn | Build an ioctl number, analogous to the C macro of the same name. |
| [`_IO`](#io) | fn | Build an ioctl number for an argumentless ioctl. |
| [`_IOR`](#ior) | fn | Build an ioctl number for an read-only ioctl. |
| [`_IOW`](#iow) | fn | Build an ioctl number for an write-only ioctl. |
| [`_IOWR`](#iowr) | fn | Build an ioctl number for a read-write ioctl. |
| [`CMSG_ALIGN`](#cmsg-align) | fn |  |
| [`CMSG_FIRSTHDR`](#cmsg-firsthdr) | fn |  |
| [`CMSG_DATA`](#cmsg-data) | fn |  |
| [`CMSG_SPACE`](#cmsg-space) | fn |  |
| [`CMSG_LEN`](#cmsg-len) | fn |  |
| [`FD_CLR`](#fd-clr) | fn |  |
| [`FD_ISSET`](#fd-isset) | fn |  |
| [`FD_SET`](#fd-set) | fn |  |
| [`FD_ZERO`](#fd-zero) | fn |  |
| [`SIGRTMAX`](#sigrtmax) | fn |  |
| [`SIGRTMIN`](#sigrtmin) | fn |  |
| [`WIFSTOPPED`](#wifstopped) | fn |  |
| [`WSTOPSIG`](#wstopsig) | fn |  |
| [`WIFCONTINUED`](#wifcontinued) | fn |  |
| [`WIFSIGNALED`](#wifsignaled) | fn |  |
| [`WTERMSIG`](#wtermsig) | fn |  |
| [`WIFEXITED`](#wifexited) | fn |  |
| [`WEXITSTATUS`](#wexitstatus) | fn |  |
| [`WCOREDUMP`](#wcoredump) | fn |  |
| [`W_EXITCODE`](#w-exitcode) | fn |  |
| [`W_STOPCODE`](#w-stopcode) | fn |  |
| [`QCMD`](#qcmd) | fn |  |
| [`IPOPT_COPIED`](#ipopt-copied) | fn |  |
| [`IPOPT_CLASS`](#ipopt-class) | fn |  |
| [`IPOPT_NUMBER`](#ipopt-number) | fn |  |
| [`IPTOS_ECN`](#iptos-ecn) | fn |  |
| [`KERNEL_VERSION`](#kernel-version) | fn |  |
| [`intmax_t`](#intmax-t) | type |  |
| [`uintmax_t`](#uintmax-t) | type |  |
| [`size_t`](#size-t) | type |  |
| [`ptrdiff_t`](#ptrdiff-t) | type |  |
| [`intptr_t`](#intptr-t) | type |  |
| [`uintptr_t`](#uintptr-t) | type |  |
| [`ssize_t`](#ssize-t) | type |  |
| [`pid_t`](#pid-t) | type |  |
| [`in_addr_t`](#in-addr-t) | type |  |
| [`in_port_t`](#in-port-t) | type |  |
| [`sighandler_t`](#sighandler-t) | type |  |
| [`cc_t`](#cc-t) | type |  |
| [`uid_t`](#uid-t) | type |  |
| [`gid_t`](#gid-t) | type |  |
| [`locale_t`](#locale-t) | type |  |
| [`sa_family_t`](#sa-family-t) | type |  |
| [`speed_t`](#speed-t) | type |  |
| [`tcflag_t`](#tcflag-t) | type |  |
| [`clockid_t`](#clockid-t) | type |  |
| [`timer_t`](#timer-t) | type |  |
| [`useconds_t`](#useconds-t) | type |  |
| [`key_t`](#key-t) | type |  |
| [`id_t`](#id-t) | type |  |
| [`INT_MIN`](#int-min) | const |  |
| [`INT_MAX`](#int-max) | const |  |
| [`SIG_DFL`](#sig-dfl) | const |  |
| [`SIG_IGN`](#sig-ign) | const |  |
| [`SIG_ERR`](#sig-err) | const |  |
| [`DT_UNKNOWN`](#dt-unknown) | const |  |
| [`DT_FIFO`](#dt-fifo) | const |  |
| [`DT_CHR`](#dt-chr) | const |  |
| [`DT_DIR`](#dt-dir) | const |  |
| [`DT_BLK`](#dt-blk) | const |  |
| [`DT_REG`](#dt-reg) | const |  |
| [`DT_LNK`](#dt-lnk) | const |  |
| [`DT_SOCK`](#dt-sock) | const |  |
| [`FD_CLOEXEC`](#fd-cloexec) | const |  |
| [`USRQUOTA`](#usrquota) | const |  |
| [`GRPQUOTA`](#grpquota) | const |  |
| [`SIGIOT`](#sigiot) | const |  |
| [`S_ISUID`](#s-isuid) | const |  |
| [`S_ISGID`](#s-isgid) | const |  |
| [`S_ISVTX`](#s-isvtx) | const |  |
| [`IF_NAMESIZE`](#if-namesize) | const |  |
| [`IFNAMSIZ`](#ifnamsiz) | const |  |
| [`LOG_EMERG`](#log-emerg) | const |  |
| [`LOG_ALERT`](#log-alert) | const |  |
| [`LOG_CRIT`](#log-crit) | const |  |
| [`LOG_ERR`](#log-err) | const |  |
| [`LOG_WARNING`](#log-warning) | const |  |
| [`LOG_NOTICE`](#log-notice) | const |  |
| [`LOG_INFO`](#log-info) | const |  |
| [`LOG_DEBUG`](#log-debug) | const |  |
| [`LOG_KERN`](#log-kern) | const |  |
| [`LOG_USER`](#log-user) | const |  |
| [`LOG_MAIL`](#log-mail) | const |  |
| [`LOG_DAEMON`](#log-daemon) | const |  |
| [`LOG_AUTH`](#log-auth) | const |  |
| [`LOG_SYSLOG`](#log-syslog) | const |  |
| [`LOG_LPR`](#log-lpr) | const |  |
| [`LOG_NEWS`](#log-news) | const |  |
| [`LOG_UUCP`](#log-uucp) | const |  |
| [`LOG_LOCAL0`](#log-local0) | const |  |
| [`LOG_LOCAL1`](#log-local1) | const |  |
| [`LOG_LOCAL2`](#log-local2) | const |  |
| [`LOG_LOCAL3`](#log-local3) | const |  |
| [`LOG_LOCAL4`](#log-local4) | const |  |
| [`LOG_LOCAL5`](#log-local5) | const |  |
| [`LOG_LOCAL6`](#log-local6) | const |  |
| [`LOG_LOCAL7`](#log-local7) | const |  |
| [`LOG_PID`](#log-pid) | const |  |
| [`LOG_CONS`](#log-cons) | const |  |
| [`LOG_ODELAY`](#log-odelay) | const |  |
| [`LOG_NDELAY`](#log-ndelay) | const |  |
| [`LOG_NOWAIT`](#log-nowait) | const |  |
| [`LOG_PRIMASK`](#log-primask) | const |  |
| [`LOG_FACMASK`](#log-facmask) | const |  |
| [`PRIO_MIN`](#prio-min) | const |  |
| [`PRIO_MAX`](#prio-max) | const |  |
| [`IPPROTO_ICMP`](#ipproto-icmp) | const |  |
| [`IPPROTO_ICMPV6`](#ipproto-icmpv6) | const |  |
| [`IPPROTO_TCP`](#ipproto-tcp) | const |  |
| [`IPPROTO_UDP`](#ipproto-udp) | const |  |
| [`IPPROTO_IP`](#ipproto-ip) | const |  |
| [`IPPROTO_IPV6`](#ipproto-ipv6) | const |  |
| [`INADDR_LOOPBACK`](#inaddr-loopback) | const |  |
| [`INADDR_ANY`](#inaddr-any) | const |  |
| [`INADDR_BROADCAST`](#inaddr-broadcast) | const |  |
| [`INADDR_NONE`](#inaddr-none) | const |  |
| [`IN6ADDR_LOOPBACK_INIT`](#in6addr-loopback-init) | const |  |
| [`IN6ADDR_ANY_INIT`](#in6addr-any-init) | const |  |
| [`ARPOP_REQUEST`](#arpop-request) | const |  |
| [`ARPOP_REPLY`](#arpop-reply) | const |  |
| [`ATF_COM`](#atf-com) | const |  |
| [`ATF_PERM`](#atf-perm) | const |  |
| [`ATF_PUBL`](#atf-publ) | const |  |
| [`ATF_USETRAILERS`](#atf-usetrailers) | const |  |
| [`FNM_PERIOD`](#fnm-period) | const |  |
| [`FNM_NOMATCH`](#fnm-nomatch) | const |  |
| [`FNM_CASEFOLD`](#fnm-casefold) | const |  |
| [`FNM_PATHNAME`](#fnm-pathname) | const |  |
| [`FNM_NOESCAPE`](#fnm-noescape) | const |  |
| [`ULONG_SIZE`](#ulong-size) | const |  |
| [`EXIT_FAILURE`](#exit-failure) | const |  |
| [`EXIT_SUCCESS`](#exit-success) | const |  |
| [`RAND_MAX`](#rand-max) | const |  |
| [`EOF`](#eof) | const |  |
| [`SEEK_SET`](#seek-set) | const |  |
| [`SEEK_CUR`](#seek-cur) | const |  |
| [`SEEK_END`](#seek-end) | const |  |
| [`_IOFBF`](#iofbf) | const |  |
| [`_IONBF`](#ionbf) | const |  |
| [`_IOLBF`](#iolbf) | const |  |
| [`F_DUPFD`](#f-dupfd) | const |  |
| [`F_GETFD`](#f-getfd) | const |  |
| [`F_SETFD`](#f-setfd) | const |  |
| [`F_GETFL`](#f-getfl) | const |  |
| [`F_SETFL`](#f-setfl) | const |  |
| [`F_SETLEASE`](#f-setlease) | const |  |
| [`F_GETLEASE`](#f-getlease) | const |  |
| [`F_NOTIFY`](#f-notify) | const |  |
| [`F_CANCELLK`](#f-cancellk) | const |  |
| [`F_DUPFD_CLOEXEC`](#f-dupfd-cloexec) | const |  |
| [`F_SETPIPE_SZ`](#f-setpipe-sz) | const |  |
| [`F_GETPIPE_SZ`](#f-getpipe-sz) | const |  |
| [`F_ADD_SEALS`](#f-add-seals) | const |  |
| [`F_GET_SEALS`](#f-get-seals) | const |  |
| [`F_SEAL_SEAL`](#f-seal-seal) | const |  |
| [`F_SEAL_SHRINK`](#f-seal-shrink) | const |  |
| [`F_SEAL_GROW`](#f-seal-grow) | const |  |
| [`F_SEAL_WRITE`](#f-seal-write) | const |  |
| [`SIGTRAP`](#sigtrap) | const |  |
| [`PTHREAD_CREATE_JOINABLE`](#pthread-create-joinable) | const |  |
| [`PTHREAD_CREATE_DETACHED`](#pthread-create-detached) | const |  |
| [`CLOCK_REALTIME`](#clock-realtime) | const |  |
| [`CLOCK_MONOTONIC`](#clock-monotonic) | const |  |
| [`CLOCK_PROCESS_CPUTIME_ID`](#clock-process-cputime-id) | const |  |
| [`CLOCK_THREAD_CPUTIME_ID`](#clock-thread-cputime-id) | const |  |
| [`CLOCK_MONOTONIC_RAW`](#clock-monotonic-raw) | const |  |
| [`CLOCK_REALTIME_COARSE`](#clock-realtime-coarse) | const |  |
| [`CLOCK_MONOTONIC_COARSE`](#clock-monotonic-coarse) | const |  |
| [`CLOCK_BOOTTIME`](#clock-boottime) | const |  |
| [`CLOCK_REALTIME_ALARM`](#clock-realtime-alarm) | const |  |
| [`CLOCK_BOOTTIME_ALARM`](#clock-boottime-alarm) | const |  |
| [`CLOCK_TAI`](#clock-tai) | const |  |
| [`TIMER_ABSTIME`](#timer-abstime) | const |  |
| [`RUSAGE_SELF`](#rusage-self) | const |  |
| [`O_RDONLY`](#o-rdonly) | const |  |
| [`O_WRONLY`](#o-wronly) | const |  |
| [`O_RDWR`](#o-rdwr) | const |  |
| [`SOCK_CLOEXEC`](#sock-cloexec) | const |  |
| [`S_IFIFO`](#s-ififo) | const |  |
| [`S_IFCHR`](#s-ifchr) | const |  |
| [`S_IFBLK`](#s-ifblk) | const |  |
| [`S_IFDIR`](#s-ifdir) | const |  |
| [`S_IFREG`](#s-ifreg) | const |  |
| [`S_IFLNK`](#s-iflnk) | const |  |
| [`S_IFSOCK`](#s-ifsock) | const |  |
| [`S_IFMT`](#s-ifmt) | const |  |
| [`S_IRWXU`](#s-irwxu) | const |  |
| [`S_IXUSR`](#s-ixusr) | const |  |
| [`S_IWUSR`](#s-iwusr) | const |  |
| [`S_IRUSR`](#s-irusr) | const |  |
| [`S_IRWXG`](#s-irwxg) | const |  |
| [`S_IXGRP`](#s-ixgrp) | const |  |
| [`S_IWGRP`](#s-iwgrp) | const |  |
| [`S_IRGRP`](#s-irgrp) | const |  |
| [`S_IRWXO`](#s-irwxo) | const |  |
| [`S_IXOTH`](#s-ixoth) | const |  |
| [`S_IWOTH`](#s-iwoth) | const |  |
| [`S_IROTH`](#s-iroth) | const |  |
| [`F_OK`](#f-ok) | const |  |
| [`R_OK`](#r-ok) | const |  |
| [`W_OK`](#w-ok) | const |  |
| [`X_OK`](#x-ok) | const |  |
| [`SIGHUP`](#sighup) | const |  |
| [`SIGINT`](#sigint) | const |  |
| [`SIGQUIT`](#sigquit) | const |  |
| [`SIGILL`](#sigill) | const |  |
| [`SIGABRT`](#sigabrt) | const |  |
| [`SIGFPE`](#sigfpe) | const |  |
| [`SIGKILL`](#sigkill) | const |  |
| [`SIGSEGV`](#sigsegv) | const |  |
| [`SIGPIPE`](#sigpipe) | const |  |
| [`SIGALRM`](#sigalrm) | const |  |
| [`SIGTERM`](#sigterm) | const |  |
| [`PROT_NONE`](#prot-none) | const |  |
| [`PROT_READ`](#prot-read) | const |  |
| [`PROT_WRITE`](#prot-write) | const |  |
| [`PROT_EXEC`](#prot-exec) | const |  |
| [`XATTR_CREATE`](#xattr-create) | const |  |
| [`XATTR_REPLACE`](#xattr-replace) | const |  |
| [`RLIM64_INFINITY`](#rlim64-infinity) | const |  |
| [`LC_CTYPE`](#lc-ctype) | const |  |
| [`LC_NUMERIC`](#lc-numeric) | const |  |
| [`LC_TIME`](#lc-time) | const |  |
| [`LC_COLLATE`](#lc-collate) | const |  |
| [`LC_MONETARY`](#lc-monetary) | const |  |
| [`LC_MESSAGES`](#lc-messages) | const |  |
| [`LC_ALL`](#lc-all) | const |  |
| [`LC_CTYPE_MASK`](#lc-ctype-mask) | const |  |
| [`LC_NUMERIC_MASK`](#lc-numeric-mask) | const |  |
| [`LC_TIME_MASK`](#lc-time-mask) | const |  |
| [`LC_COLLATE_MASK`](#lc-collate-mask) | const |  |
| [`LC_MONETARY_MASK`](#lc-monetary-mask) | const |  |
| [`LC_MESSAGES_MASK`](#lc-messages-mask) | const |  |
| [`MAP_FILE`](#map-file) | const |  |
| [`MAP_SHARED`](#map-shared) | const |  |
| [`MAP_PRIVATE`](#map-private) | const |  |
| [`MAP_FIXED`](#map-fixed) | const |  |
| [`MAP_FAILED`](#map-failed) | const |  |
| [`MS_ASYNC`](#ms-async) | const |  |
| [`MS_INVALIDATE`](#ms-invalidate) | const |  |
| [`MS_SYNC`](#ms-sync) | const |  |
| [`MS_RDONLY`](#ms-rdonly) | const |  |
| [`MS_NOSUID`](#ms-nosuid) | const |  |
| [`MS_NODEV`](#ms-nodev) | const |  |
| [`MS_NOEXEC`](#ms-noexec) | const |  |
| [`MS_SYNCHRONOUS`](#ms-synchronous) | const |  |
| [`MS_REMOUNT`](#ms-remount) | const |  |
| [`MS_MANDLOCK`](#ms-mandlock) | const |  |
| [`MS_DIRSYNC`](#ms-dirsync) | const |  |
| [`MS_NOSYMFOLLOW`](#ms-nosymfollow) | const |  |
| [`MS_NOATIME`](#ms-noatime) | const |  |
| [`MS_NODIRATIME`](#ms-nodiratime) | const |  |
| [`MS_BIND`](#ms-bind) | const |  |
| [`MS_MOVE`](#ms-move) | const |  |
| [`MS_REC`](#ms-rec) | const |  |
| [`MS_SILENT`](#ms-silent) | const |  |
| [`MS_POSIXACL`](#ms-posixacl) | const |  |
| [`MS_UNBINDABLE`](#ms-unbindable) | const |  |
| [`MS_PRIVATE`](#ms-private) | const |  |
| [`MS_SLAVE`](#ms-slave) | const |  |
| [`MS_SHARED`](#ms-shared) | const |  |
| [`MS_RELATIME`](#ms-relatime) | const |  |
| [`MS_KERNMOUNT`](#ms-kernmount) | const |  |
| [`MS_I_VERSION`](#ms-i-version) | const |  |
| [`MS_STRICTATIME`](#ms-strictatime) | const |  |
| [`MS_LAZYTIME`](#ms-lazytime) | const |  |
| [`MS_ACTIVE`](#ms-active) | const |  |
| [`MS_MGC_VAL`](#ms-mgc-val) | const |  |
| [`MS_MGC_MSK`](#ms-mgc-msk) | const |  |
| [`SCM_RIGHTS`](#scm-rights) | const |  |
| [`SCM_CREDENTIALS`](#scm-credentials) | const |  |
| [`PROT_GROWSDOWN`](#prot-growsdown) | const |  |
| [`PROT_GROWSUP`](#prot-growsup) | const |  |
| [`MAP_TYPE`](#map-type) | const |  |
| [`MADV_NORMAL`](#madv-normal) | const |  |
| [`MADV_RANDOM`](#madv-random) | const |  |
| [`MADV_SEQUENTIAL`](#madv-sequential) | const |  |
| [`MADV_WILLNEED`](#madv-willneed) | const |  |
| [`MADV_DONTNEED`](#madv-dontneed) | const |  |
| [`MADV_FREE`](#madv-free) | const |  |
| [`MADV_REMOVE`](#madv-remove) | const |  |
| [`MADV_DONTFORK`](#madv-dontfork) | const |  |
| [`MADV_DOFORK`](#madv-dofork) | const |  |
| [`MADV_MERGEABLE`](#madv-mergeable) | const |  |
| [`MADV_UNMERGEABLE`](#madv-unmergeable) | const |  |
| [`MADV_HUGEPAGE`](#madv-hugepage) | const |  |
| [`MADV_NOHUGEPAGE`](#madv-nohugepage) | const |  |
| [`MADV_DONTDUMP`](#madv-dontdump) | const |  |
| [`MADV_DODUMP`](#madv-dodump) | const |  |
| [`MADV_WIPEONFORK`](#madv-wipeonfork) | const |  |
| [`MADV_KEEPONFORK`](#madv-keeponfork) | const |  |
| [`MADV_COLD`](#madv-cold) | const |  |
| [`MADV_PAGEOUT`](#madv-pageout) | const |  |
| [`MADV_HWPOISON`](#madv-hwpoison) | const |  |
| [`MADV_POPULATE_READ`](#madv-populate-read) | const |  |
| [`MADV_POPULATE_WRITE`](#madv-populate-write) | const |  |
| [`MADV_DONTNEED_LOCKED`](#madv-dontneed-locked) | const |  |
| [`IFF_UP`](#iff-up) | const |  |
| [`IFF_BROADCAST`](#iff-broadcast) | const |  |
| [`IFF_DEBUG`](#iff-debug) | const |  |
| [`IFF_LOOPBACK`](#iff-loopback) | const |  |
| [`IFF_POINTOPOINT`](#iff-pointopoint) | const |  |
| [`IFF_NOTRAILERS`](#iff-notrailers) | const |  |
| [`IFF_RUNNING`](#iff-running) | const |  |
| [`IFF_NOARP`](#iff-noarp) | const |  |
| [`IFF_PROMISC`](#iff-promisc) | const |  |
| [`IFF_ALLMULTI`](#iff-allmulti) | const |  |
| [`IFF_MASTER`](#iff-master) | const |  |
| [`IFF_SLAVE`](#iff-slave) | const |  |
| [`IFF_MULTICAST`](#iff-multicast) | const |  |
| [`IFF_PORTSEL`](#iff-portsel) | const |  |
| [`IFF_AUTOMEDIA`](#iff-automedia) | const |  |
| [`IFF_DYNAMIC`](#iff-dynamic) | const |  |
| [`SOL_IP`](#sol-ip) | const |  |
| [`SOL_TCP`](#sol-tcp) | const |  |
| [`SOL_UDP`](#sol-udp) | const |  |
| [`SOL_IPV6`](#sol-ipv6) | const |  |
| [`SOL_ICMPV6`](#sol-icmpv6) | const |  |
| [`SOL_RAW`](#sol-raw) | const |  |
| [`SOL_DECNET`](#sol-decnet) | const |  |
| [`SOL_X25`](#sol-x25) | const |  |
| [`SOL_PACKET`](#sol-packet) | const |  |
| [`SOL_ATM`](#sol-atm) | const |  |
| [`SOL_AAL`](#sol-aal) | const |  |
| [`SOL_IRDA`](#sol-irda) | const |  |
| [`SOL_NETBEUI`](#sol-netbeui) | const |  |
| [`SOL_LLC`](#sol-llc) | const |  |
| [`SOL_DCCP`](#sol-dccp) | const |  |
| [`SOL_NETLINK`](#sol-netlink) | const |  |
| [`SOL_TIPC`](#sol-tipc) | const |  |
| [`SOL_BLUETOOTH`](#sol-bluetooth) | const |  |
| [`SOL_ALG`](#sol-alg) | const |  |
| [`AF_UNSPEC`](#af-unspec) | const |  |
| [`AF_UNIX`](#af-unix) | const |  |
| [`AF_LOCAL`](#af-local) | const |  |
| [`AF_INET`](#af-inet) | const |  |
| [`AF_AX25`](#af-ax25) | const |  |
| [`AF_IPX`](#af-ipx) | const |  |
| [`AF_APPLETALK`](#af-appletalk) | const |  |
| [`AF_NETROM`](#af-netrom) | const |  |
| [`AF_BRIDGE`](#af-bridge) | const |  |
| [`AF_ATMPVC`](#af-atmpvc) | const |  |
| [`AF_X25`](#af-x25) | const |  |
| [`AF_INET6`](#af-inet6) | const |  |
| [`AF_ROSE`](#af-rose) | const |  |
| [`AF_DECnet`](#af-decnet) | const |  |
| [`AF_NETBEUI`](#af-netbeui) | const |  |
| [`AF_SECURITY`](#af-security) | const |  |
| [`AF_KEY`](#af-key) | const |  |
| [`AF_NETLINK`](#af-netlink) | const |  |
| [`AF_ROUTE`](#af-route) | const |  |
| [`AF_PACKET`](#af-packet) | const |  |
| [`AF_ASH`](#af-ash) | const |  |
| [`AF_ECONET`](#af-econet) | const |  |
| [`AF_ATMSVC`](#af-atmsvc) | const |  |
| [`AF_RDS`](#af-rds) | const |  |
| [`AF_SNA`](#af-sna) | const |  |
| [`AF_IRDA`](#af-irda) | const |  |
| [`AF_PPPOX`](#af-pppox) | const |  |
| [`AF_WANPIPE`](#af-wanpipe) | const |  |
| [`AF_LLC`](#af-llc) | const |  |
| [`AF_CAN`](#af-can) | const |  |
| [`AF_TIPC`](#af-tipc) | const |  |
| [`AF_BLUETOOTH`](#af-bluetooth) | const |  |
| [`AF_IUCV`](#af-iucv) | const |  |
| [`AF_RXRPC`](#af-rxrpc) | const |  |
| [`AF_ISDN`](#af-isdn) | const |  |
| [`AF_PHONET`](#af-phonet) | const |  |
| [`AF_IEEE802154`](#af-ieee802154) | const |  |
| [`AF_CAIF`](#af-caif) | const |  |
| [`AF_ALG`](#af-alg) | const |  |
| [`PF_UNSPEC`](#pf-unspec) | const |  |
| [`PF_UNIX`](#pf-unix) | const |  |
| [`PF_LOCAL`](#pf-local) | const |  |
| [`PF_INET`](#pf-inet) | const |  |
| [`PF_AX25`](#pf-ax25) | const |  |
| [`PF_IPX`](#pf-ipx) | const |  |
| [`PF_APPLETALK`](#pf-appletalk) | const |  |
| [`PF_NETROM`](#pf-netrom) | const |  |
| [`PF_BRIDGE`](#pf-bridge) | const |  |
| [`PF_ATMPVC`](#pf-atmpvc) | const |  |
| [`PF_X25`](#pf-x25) | const |  |
| [`PF_INET6`](#pf-inet6) | const |  |
| [`PF_ROSE`](#pf-rose) | const |  |
| [`PF_DECnet`](#pf-decnet) | const |  |
| [`PF_NETBEUI`](#pf-netbeui) | const |  |
| [`PF_SECURITY`](#pf-security) | const |  |
| [`PF_KEY`](#pf-key) | const |  |
| [`PF_NETLINK`](#pf-netlink) | const |  |
| [`PF_ROUTE`](#pf-route) | const |  |
| [`PF_PACKET`](#pf-packet) | const |  |
| [`PF_ASH`](#pf-ash) | const |  |
| [`PF_ECONET`](#pf-econet) | const |  |
| [`PF_ATMSVC`](#pf-atmsvc) | const |  |
| [`PF_RDS`](#pf-rds) | const |  |
| [`PF_SNA`](#pf-sna) | const |  |
| [`PF_IRDA`](#pf-irda) | const |  |
| [`PF_PPPOX`](#pf-pppox) | const |  |
| [`PF_WANPIPE`](#pf-wanpipe) | const |  |
| [`PF_LLC`](#pf-llc) | const |  |
| [`PF_CAN`](#pf-can) | const |  |
| [`PF_TIPC`](#pf-tipc) | const |  |
| [`PF_BLUETOOTH`](#pf-bluetooth) | const |  |
| [`PF_IUCV`](#pf-iucv) | const |  |
| [`PF_RXRPC`](#pf-rxrpc) | const |  |
| [`PF_ISDN`](#pf-isdn) | const |  |
| [`PF_PHONET`](#pf-phonet) | const |  |
| [`PF_IEEE802154`](#pf-ieee802154) | const |  |
| [`PF_CAIF`](#pf-caif) | const |  |
| [`PF_ALG`](#pf-alg) | const |  |
| [`MSG_OOB`](#msg-oob) | const |  |
| [`MSG_PEEK`](#msg-peek) | const |  |
| [`MSG_DONTROUTE`](#msg-dontroute) | const |  |
| [`MSG_CTRUNC`](#msg-ctrunc) | const |  |
| [`MSG_TRUNC`](#msg-trunc) | const |  |
| [`MSG_DONTWAIT`](#msg-dontwait) | const |  |
| [`MSG_EOR`](#msg-eor) | const |  |
| [`MSG_WAITALL`](#msg-waitall) | const |  |
| [`MSG_FIN`](#msg-fin) | const |  |
| [`MSG_SYN`](#msg-syn) | const |  |
| [`MSG_CONFIRM`](#msg-confirm) | const |  |
| [`MSG_RST`](#msg-rst) | const |  |
| [`MSG_ERRQUEUE`](#msg-errqueue) | const |  |
| [`MSG_NOSIGNAL`](#msg-nosignal) | const |  |
| [`MSG_MORE`](#msg-more) | const |  |
| [`MSG_WAITFORONE`](#msg-waitforone) | const |  |
| [`MSG_FASTOPEN`](#msg-fastopen) | const |  |
| [`MSG_CMSG_CLOEXEC`](#msg-cmsg-cloexec) | const |  |
| [`SCM_TIMESTAMP`](#scm-timestamp) | const |  |
| [`SOCK_RAW`](#sock-raw) | const |  |
| [`SOCK_RDM`](#sock-rdm) | const |  |
| [`IP_TOS`](#ip-tos) | const |  |
| [`IP_TTL`](#ip-ttl) | const |  |
| [`IP_HDRINCL`](#ip-hdrincl) | const |  |
| [`IP_OPTIONS`](#ip-options) | const |  |
| [`IP_ROUTER_ALERT`](#ip-router-alert) | const |  |
| [`IP_RECVOPTS`](#ip-recvopts) | const |  |
| [`IP_RETOPTS`](#ip-retopts) | const |  |
| [`IP_PKTINFO`](#ip-pktinfo) | const |  |
| [`IP_PKTOPTIONS`](#ip-pktoptions) | const |  |
| [`IP_MTU_DISCOVER`](#ip-mtu-discover) | const |  |
| [`IP_RECVERR`](#ip-recverr) | const |  |
| [`IP_RECVTTL`](#ip-recvttl) | const |  |
| [`IP_RECVTOS`](#ip-recvtos) | const |  |
| [`IP_MTU`](#ip-mtu) | const |  |
| [`IP_FREEBIND`](#ip-freebind) | const |  |
| [`IP_IPSEC_POLICY`](#ip-ipsec-policy) | const |  |
| [`IP_XFRM_POLICY`](#ip-xfrm-policy) | const |  |
| [`IP_PASSSEC`](#ip-passsec) | const |  |
| [`IP_TRANSPARENT`](#ip-transparent) | const |  |
| [`IP_ORIGDSTADDR`](#ip-origdstaddr) | const |  |
| [`IP_RECVORIGDSTADDR`](#ip-recvorigdstaddr) | const |  |
| [`IP_MINTTL`](#ip-minttl) | const |  |
| [`IP_NODEFRAG`](#ip-nodefrag) | const |  |
| [`IP_CHECKSUM`](#ip-checksum) | const |  |
| [`IP_BIND_ADDRESS_NO_PORT`](#ip-bind-address-no-port) | const |  |
| [`IP_MULTICAST_IF`](#ip-multicast-if) | const |  |
| [`IP_MULTICAST_TTL`](#ip-multicast-ttl) | const |  |
| [`IP_MULTICAST_LOOP`](#ip-multicast-loop) | const |  |
| [`IP_ADD_MEMBERSHIP`](#ip-add-membership) | const |  |
| [`IP_DROP_MEMBERSHIP`](#ip-drop-membership) | const |  |
| [`IP_UNBLOCK_SOURCE`](#ip-unblock-source) | const |  |
| [`IP_BLOCK_SOURCE`](#ip-block-source) | const |  |
| [`IP_ADD_SOURCE_MEMBERSHIP`](#ip-add-source-membership) | const |  |
| [`IP_DROP_SOURCE_MEMBERSHIP`](#ip-drop-source-membership) | const |  |
| [`IP_MSFILTER`](#ip-msfilter) | const |  |
| [`IP_MULTICAST_ALL`](#ip-multicast-all) | const |  |
| [`IP_UNICAST_IF`](#ip-unicast-if) | const |  |
| [`IP_DEFAULT_MULTICAST_TTL`](#ip-default-multicast-ttl) | const |  |
| [`IP_DEFAULT_MULTICAST_LOOP`](#ip-default-multicast-loop) | const |  |
| [`IP_PMTUDISC_DONT`](#ip-pmtudisc-dont) | const |  |
| [`IP_PMTUDISC_WANT`](#ip-pmtudisc-want) | const |  |
| [`IP_PMTUDISC_DO`](#ip-pmtudisc-do) | const |  |
| [`IP_PMTUDISC_PROBE`](#ip-pmtudisc-probe) | const |  |
| [`IP_PMTUDISC_INTERFACE`](#ip-pmtudisc-interface) | const |  |
| [`IP_PMTUDISC_OMIT`](#ip-pmtudisc-omit) | const |  |
| [`IPPROTO_HOPOPTS`](#ipproto-hopopts) | const | Hop-by-hop option header |
| [`IPPROTO_IGMP`](#ipproto-igmp) | const | group mgmt protocol |
| [`IPPROTO_IPIP`](#ipproto-ipip) | const | for compatibility |
| [`IPPROTO_EGP`](#ipproto-egp) | const | exterior gateway protocol |
| [`IPPROTO_PUP`](#ipproto-pup) | const | pup |
| [`IPPROTO_IDP`](#ipproto-idp) | const | xns idp |
| [`IPPROTO_TP`](#ipproto-tp) | const | tp-4 w/ class negotiation |
| [`IPPROTO_DCCP`](#ipproto-dccp) | const | DCCP |
| [`IPPROTO_ROUTING`](#ipproto-routing) | const | IP6 routing header |
| [`IPPROTO_FRAGMENT`](#ipproto-fragment) | const | IP6 fragmentation header |
| [`IPPROTO_RSVP`](#ipproto-rsvp) | const | resource reservation |
| [`IPPROTO_GRE`](#ipproto-gre) | const | General Routing Encap. |
| [`IPPROTO_ESP`](#ipproto-esp) | const | IP6 Encap Sec. |
| [`IPPROTO_AH`](#ipproto-ah) | const | IP6 Auth Header |
| [`IPPROTO_NONE`](#ipproto-none) | const | IP6 no next header |
| [`IPPROTO_DSTOPTS`](#ipproto-dstopts) | const | IP6 destination option |
| [`IPPROTO_MTP`](#ipproto-mtp) | const |  |
| [`IPPROTO_ENCAP`](#ipproto-encap) | const | encapsulation header |
| [`IPPROTO_PIM`](#ipproto-pim) | const | Protocol indep. |
| [`IPPROTO_COMP`](#ipproto-comp) | const | IP Payload Comp. |
| [`IPPROTO_SCTP`](#ipproto-sctp) | const | SCTP |
| [`IPPROTO_MH`](#ipproto-mh) | const |  |
| [`IPPROTO_UDPLITE`](#ipproto-udplite) | const |  |
| [`IPPROTO_RAW`](#ipproto-raw) | const | raw IP packet |
| [`IPPROTO_BEETPH`](#ipproto-beetph) | const |  |
| [`IPPROTO_MPLS`](#ipproto-mpls) | const |  |
| [`IPPROTO_MPTCP`](#ipproto-mptcp) | const | Multipath TCP |
| [`IPPROTO_ETHERNET`](#ipproto-ethernet) | const | Ethernet-within-IPv6 encapsulation. |
| [`MCAST_EXCLUDE`](#mcast-exclude) | const |  |
| [`MCAST_INCLUDE`](#mcast-include) | const |  |
| [`MCAST_JOIN_GROUP`](#mcast-join-group) | const |  |
| [`MCAST_BLOCK_SOURCE`](#mcast-block-source) | const |  |
| [`MCAST_UNBLOCK_SOURCE`](#mcast-unblock-source) | const |  |
| [`MCAST_LEAVE_GROUP`](#mcast-leave-group) | const |  |
| [`MCAST_JOIN_SOURCE_GROUP`](#mcast-join-source-group) | const |  |
| [`MCAST_LEAVE_SOURCE_GROUP`](#mcast-leave-source-group) | const |  |
| [`MCAST_MSFILTER`](#mcast-msfilter) | const |  |
| [`IPV6_ADDRFORM`](#ipv6-addrform) | const |  |
| [`IPV6_2292PKTINFO`](#ipv6-2292pktinfo) | const |  |
| [`IPV6_2292HOPOPTS`](#ipv6-2292hopopts) | const |  |
| [`IPV6_2292DSTOPTS`](#ipv6-2292dstopts) | const |  |
| [`IPV6_2292RTHDR`](#ipv6-2292rthdr) | const |  |
| [`IPV6_2292PKTOPTIONS`](#ipv6-2292pktoptions) | const |  |
| [`IPV6_CHECKSUM`](#ipv6-checksum) | const |  |
| [`IPV6_2292HOPLIMIT`](#ipv6-2292hoplimit) | const |  |
| [`IPV6_NEXTHOP`](#ipv6-nexthop) | const |  |
| [`IPV6_AUTHHDR`](#ipv6-authhdr) | const |  |
| [`IPV6_UNICAST_HOPS`](#ipv6-unicast-hops) | const |  |
| [`IPV6_MULTICAST_IF`](#ipv6-multicast-if) | const |  |
| [`IPV6_MULTICAST_HOPS`](#ipv6-multicast-hops) | const |  |
| [`IPV6_MULTICAST_LOOP`](#ipv6-multicast-loop) | const |  |
| [`IPV6_ADD_MEMBERSHIP`](#ipv6-add-membership) | const |  |
| [`IPV6_DROP_MEMBERSHIP`](#ipv6-drop-membership) | const |  |
| [`IPV6_ROUTER_ALERT`](#ipv6-router-alert) | const |  |
| [`IPV6_MTU_DISCOVER`](#ipv6-mtu-discover) | const |  |
| [`IPV6_MTU`](#ipv6-mtu) | const |  |
| [`IPV6_RECVERR`](#ipv6-recverr) | const |  |
| [`IPV6_V6ONLY`](#ipv6-v6only) | const |  |
| [`IPV6_JOIN_ANYCAST`](#ipv6-join-anycast) | const |  |
| [`IPV6_LEAVE_ANYCAST`](#ipv6-leave-anycast) | const |  |
| [`IPV6_IPSEC_POLICY`](#ipv6-ipsec-policy) | const |  |
| [`IPV6_XFRM_POLICY`](#ipv6-xfrm-policy) | const |  |
| [`IPV6_HDRINCL`](#ipv6-hdrincl) | const |  |
| [`IPV6_RECVPKTINFO`](#ipv6-recvpktinfo) | const |  |
| [`IPV6_PKTINFO`](#ipv6-pktinfo) | const |  |
| [`IPV6_RECVHOPLIMIT`](#ipv6-recvhoplimit) | const |  |
| [`IPV6_HOPLIMIT`](#ipv6-hoplimit) | const |  |
| [`IPV6_RECVHOPOPTS`](#ipv6-recvhopopts) | const |  |
| [`IPV6_HOPOPTS`](#ipv6-hopopts) | const |  |
| [`IPV6_RTHDRDSTOPTS`](#ipv6-rthdrdstopts) | const |  |
| [`IPV6_RECVRTHDR`](#ipv6-recvrthdr) | const |  |
| [`IPV6_RTHDR`](#ipv6-rthdr) | const |  |
| [`IPV6_RECVDSTOPTS`](#ipv6-recvdstopts) | const |  |
| [`IPV6_DSTOPTS`](#ipv6-dstopts) | const |  |
| [`IPV6_RECVPATHMTU`](#ipv6-recvpathmtu) | const |  |
| [`IPV6_PATHMTU`](#ipv6-pathmtu) | const |  |
| [`IPV6_DONTFRAG`](#ipv6-dontfrag) | const |  |
| [`IPV6_RECVTCLASS`](#ipv6-recvtclass) | const |  |
| [`IPV6_TCLASS`](#ipv6-tclass) | const |  |
| [`IPV6_AUTOFLOWLABEL`](#ipv6-autoflowlabel) | const |  |
| [`IPV6_ADDR_PREFERENCES`](#ipv6-addr-preferences) | const |  |
| [`IPV6_MINHOPCOUNT`](#ipv6-minhopcount) | const |  |
| [`IPV6_ORIGDSTADDR`](#ipv6-origdstaddr) | const |  |
| [`IPV6_RECVORIGDSTADDR`](#ipv6-recvorigdstaddr) | const |  |
| [`IPV6_TRANSPARENT`](#ipv6-transparent) | const |  |
| [`IPV6_UNICAST_IF`](#ipv6-unicast-if) | const |  |
| [`IPV6_PREFER_SRC_TMP`](#ipv6-prefer-src-tmp) | const |  |
| [`IPV6_PREFER_SRC_PUBLIC`](#ipv6-prefer-src-public) | const |  |
| [`IPV6_PREFER_SRC_PUBTMP_DEFAULT`](#ipv6-prefer-src-pubtmp-default) | const |  |
| [`IPV6_PREFER_SRC_COA`](#ipv6-prefer-src-coa) | const |  |
| [`IPV6_PREFER_SRC_HOME`](#ipv6-prefer-src-home) | const |  |
| [`IPV6_PREFER_SRC_CGA`](#ipv6-prefer-src-cga) | const |  |
| [`IPV6_PREFER_SRC_NONCGA`](#ipv6-prefer-src-noncga) | const |  |
| [`IPV6_PMTUDISC_DONT`](#ipv6-pmtudisc-dont) | const |  |
| [`IPV6_PMTUDISC_WANT`](#ipv6-pmtudisc-want) | const |  |
| [`IPV6_PMTUDISC_DO`](#ipv6-pmtudisc-do) | const |  |
| [`IPV6_PMTUDISC_PROBE`](#ipv6-pmtudisc-probe) | const |  |
| [`IPV6_PMTUDISC_INTERFACE`](#ipv6-pmtudisc-interface) | const |  |
| [`IPV6_PMTUDISC_OMIT`](#ipv6-pmtudisc-omit) | const |  |
| [`TCP_NODELAY`](#tcp-nodelay) | const |  |
| [`TCP_MAXSEG`](#tcp-maxseg) | const |  |
| [`TCP_CORK`](#tcp-cork) | const |  |
| [`TCP_KEEPIDLE`](#tcp-keepidle) | const |  |
| [`TCP_KEEPINTVL`](#tcp-keepintvl) | const |  |
| [`TCP_KEEPCNT`](#tcp-keepcnt) | const |  |
| [`TCP_SYNCNT`](#tcp-syncnt) | const |  |
| [`TCP_LINGER2`](#tcp-linger2) | const |  |
| [`TCP_DEFER_ACCEPT`](#tcp-defer-accept) | const |  |
| [`TCP_WINDOW_CLAMP`](#tcp-window-clamp) | const |  |
| [`TCP_INFO`](#tcp-info) | const |  |
| [`TCP_QUICKACK`](#tcp-quickack) | const |  |
| [`TCP_CONGESTION`](#tcp-congestion) | const |  |
| [`TCP_MD5SIG`](#tcp-md5sig) | const |  |
| [`TCP_COOKIE_TRANSACTIONS`](#tcp-cookie-transactions) | const |  |
| [`TCP_THIN_LINEAR_TIMEOUTS`](#tcp-thin-linear-timeouts) | const |  |
| [`TCP_THIN_DUPACK`](#tcp-thin-dupack) | const |  |
| [`TCP_USER_TIMEOUT`](#tcp-user-timeout) | const |  |
| [`TCP_REPAIR`](#tcp-repair) | const |  |
| [`TCP_REPAIR_QUEUE`](#tcp-repair-queue) | const |  |
| [`TCP_QUEUE_SEQ`](#tcp-queue-seq) | const |  |
| [`TCP_REPAIR_OPTIONS`](#tcp-repair-options) | const |  |
| [`TCP_FASTOPEN`](#tcp-fastopen) | const |  |
| [`TCP_TIMESTAMP`](#tcp-timestamp) | const |  |
| [`TCP_NOTSENT_LOWAT`](#tcp-notsent-lowat) | const |  |
| [`TCP_CC_INFO`](#tcp-cc-info) | const |  |
| [`TCP_SAVE_SYN`](#tcp-save-syn) | const |  |
| [`TCP_SAVED_SYN`](#tcp-saved-syn) | const |  |
| [`TCP_REPAIR_WINDOW`](#tcp-repair-window) | const |  |
| [`TCP_FASTOPEN_CONNECT`](#tcp-fastopen-connect) | const |  |
| [`TCP_ULP`](#tcp-ulp) | const |  |
| [`TCP_MD5SIG_EXT`](#tcp-md5sig-ext) | const |  |
| [`TCP_FASTOPEN_KEY`](#tcp-fastopen-key) | const |  |
| [`TCP_FASTOPEN_NO_COOKIE`](#tcp-fastopen-no-cookie) | const |  |
| [`TCP_ZEROCOPY_RECEIVE`](#tcp-zerocopy-receive) | const |  |
| [`TCP_INQ`](#tcp-inq) | const |  |
| [`TCP_CM_INQ`](#tcp-cm-inq) | const |  |
| [`TCP_MD5SIG_MAXKEYLEN`](#tcp-md5sig-maxkeylen) | const |  |
| [`SO_DEBUG`](#so-debug) | const |  |
| [`SHUT_RD`](#shut-rd) | const |  |
| [`SHUT_WR`](#shut-wr) | const |  |
| [`SHUT_RDWR`](#shut-rdwr) | const |  |
| [`LOCK_SH`](#lock-sh) | const |  |
| [`LOCK_EX`](#lock-ex) | const |  |
| [`LOCK_NB`](#lock-nb) | const |  |
| [`LOCK_UN`](#lock-un) | const |  |
| [`SS_ONSTACK`](#ss-onstack) | const |  |
| [`SS_DISABLE`](#ss-disable) | const |  |
| [`NAME_MAX`](#name-max) | const |  |
| [`PATH_MAX`](#path-max) | const |  |
| [`UIO_MAXIOV`](#uio-maxiov) | const |  |
| [`FD_SETSIZE`](#fd-setsize) | const |  |
| [`EPOLLIN`](#epollin) | const |  |
| [`EPOLLPRI`](#epollpri) | const |  |
| [`EPOLLOUT`](#epollout) | const |  |
| [`EPOLLERR`](#epollerr) | const |  |
| [`EPOLLHUP`](#epollhup) | const |  |
| [`EPOLLRDNORM`](#epollrdnorm) | const |  |
| [`EPOLLRDBAND`](#epollrdband) | const |  |
| [`EPOLLWRNORM`](#epollwrnorm) | const |  |
| [`EPOLLWRBAND`](#epollwrband) | const |  |
| [`EPOLLMSG`](#epollmsg) | const |  |
| [`EPOLLRDHUP`](#epollrdhup) | const |  |
| [`EPOLLEXCLUSIVE`](#epollexclusive) | const |  |
| [`EPOLLWAKEUP`](#epollwakeup) | const |  |
| [`EPOLLONESHOT`](#epolloneshot) | const |  |
| [`EPOLLET`](#epollet) | const |  |
| [`EPOLL_CTL_ADD`](#epoll-ctl-add) | const |  |
| [`EPOLL_CTL_MOD`](#epoll-ctl-mod) | const |  |
| [`EPOLL_CTL_DEL`](#epoll-ctl-del) | const |  |
| [`MNT_FORCE`](#mnt-force) | const |  |
| [`MNT_DETACH`](#mnt-detach) | const |  |
| [`MNT_EXPIRE`](#mnt-expire) | const |  |
| [`UMOUNT_NOFOLLOW`](#umount-nofollow) | const |  |
| [`Q_GETFMT`](#q-getfmt) | const |  |
| [`Q_GETINFO`](#q-getinfo) | const |  |
| [`Q_SETINFO`](#q-setinfo) | const |  |
| [`QIF_BLIMITS`](#qif-blimits) | const |  |
| [`QIF_SPACE`](#qif-space) | const |  |
| [`QIF_ILIMITS`](#qif-ilimits) | const |  |
| [`QIF_INODES`](#qif-inodes) | const |  |
| [`QIF_BTIME`](#qif-btime) | const |  |
| [`QIF_ITIME`](#qif-itime) | const |  |
| [`QIF_LIMITS`](#qif-limits) | const |  |
| [`QIF_USAGE`](#qif-usage) | const |  |
| [`QIF_TIMES`](#qif-times) | const |  |
| [`QIF_ALL`](#qif-all) | const |  |
| [`Q_SYNC`](#q-sync) | const |  |
| [`Q_QUOTAON`](#q-quotaon) | const |  |
| [`Q_QUOTAOFF`](#q-quotaoff) | const |  |
| [`Q_GETQUOTA`](#q-getquota) | const |  |
| [`Q_SETQUOTA`](#q-setquota) | const |  |
| [`TCIOFF`](#tcioff) | const |  |
| [`TCION`](#tcion) | const |  |
| [`TCOOFF`](#tcooff) | const |  |
| [`TCOON`](#tcoon) | const |  |
| [`TCIFLUSH`](#tciflush) | const |  |
| [`TCOFLUSH`](#tcoflush) | const |  |
| [`TCIOFLUSH`](#tcioflush) | const |  |
| [`NL0`](#nl0) | const |  |
| [`NL1`](#nl1) | const |  |
| [`TAB0`](#tab0) | const |  |
| [`CR0`](#cr0) | const |  |
| [`FF0`](#ff0) | const |  |
| [`BS0`](#bs0) | const |  |
| [`VT0`](#vt0) | const |  |
| [`VERASE`](#verase) | const |  |
| [`VKILL`](#vkill) | const |  |
| [`VINTR`](#vintr) | const |  |
| [`VQUIT`](#vquit) | const |  |
| [`VLNEXT`](#vlnext) | const |  |
| [`IGNBRK`](#ignbrk) | const |  |
| [`BRKINT`](#brkint) | const |  |
| [`IGNPAR`](#ignpar) | const |  |
| [`PARMRK`](#parmrk) | const |  |
| [`INPCK`](#inpck) | const |  |
| [`ISTRIP`](#istrip) | const |  |
| [`INLCR`](#inlcr) | const |  |
| [`IGNCR`](#igncr) | const |  |
| [`ICRNL`](#icrnl) | const |  |
| [`IXANY`](#ixany) | const |  |
| [`IMAXBEL`](#imaxbel) | const |  |
| [`OPOST`](#opost) | const |  |
| [`CS5`](#cs5) | const |  |
| [`CRTSCTS`](#crtscts) | const |  |
| [`ECHO`](#echo) | const |  |
| [`OCRNL`](#ocrnl) | const |  |
| [`ONOCR`](#onocr) | const |  |
| [`ONLRET`](#onlret) | const |  |
| [`OFILL`](#ofill) | const |  |
| [`OFDEL`](#ofdel) | const |  |
| [`CLONE_VM`](#clone-vm) | const |  |
| [`CLONE_FS`](#clone-fs) | const |  |
| [`CLONE_FILES`](#clone-files) | const |  |
| [`CLONE_SIGHAND`](#clone-sighand) | const |  |
| [`CLONE_PTRACE`](#clone-ptrace) | const |  |
| [`CLONE_VFORK`](#clone-vfork) | const |  |
| [`CLONE_PARENT`](#clone-parent) | const |  |
| [`CLONE_THREAD`](#clone-thread) | const |  |
| [`CLONE_NEWNS`](#clone-newns) | const |  |
| [`CLONE_SYSVSEM`](#clone-sysvsem) | const |  |
| [`CLONE_SETTLS`](#clone-settls) | const |  |
| [`CLONE_PARENT_SETTID`](#clone-parent-settid) | const |  |
| [`CLONE_CHILD_CLEARTID`](#clone-child-cleartid) | const |  |
| [`CLONE_DETACHED`](#clone-detached) | const |  |
| [`CLONE_UNTRACED`](#clone-untraced) | const |  |
| [`CLONE_CHILD_SETTID`](#clone-child-settid) | const |  |
| [`CLONE_NEWCGROUP`](#clone-newcgroup) | const |  |
| [`CLONE_NEWUTS`](#clone-newuts) | const |  |
| [`CLONE_NEWIPC`](#clone-newipc) | const |  |
| [`CLONE_NEWUSER`](#clone-newuser) | const |  |
| [`CLONE_NEWPID`](#clone-newpid) | const |  |
| [`CLONE_NEWNET`](#clone-newnet) | const |  |
| [`CLONE_IO`](#clone-io) | const |  |
| [`WNOHANG`](#wnohang) | const |  |
| [`WUNTRACED`](#wuntraced) | const |  |
| [`WSTOPPED`](#wstopped) | const |  |
| [`WEXITED`](#wexited) | const |  |
| [`WCONTINUED`](#wcontinued) | const |  |
| [`WNOWAIT`](#wnowait) | const |  |
| [`ADDR_NO_RANDOMIZE`](#addr-no-randomize) | const |  |
| [`MMAP_PAGE_ZERO`](#mmap-page-zero) | const |  |
| [`ADDR_COMPAT_LAYOUT`](#addr-compat-layout) | const |  |
| [`READ_IMPLIES_EXEC`](#read-implies-exec) | const |  |
| [`ADDR_LIMIT_32BIT`](#addr-limit-32bit) | const |  |
| [`SHORT_INODE`](#short-inode) | const |  |
| [`WHOLE_SECONDS`](#whole-seconds) | const |  |
| [`STICKY_TIMEOUTS`](#sticky-timeouts) | const |  |
| [`ADDR_LIMIT_3GB`](#addr-limit-3gb) | const |  |
| [`PTRACE_O_TRACESYSGOOD`](#ptrace-o-tracesysgood) | const |  |
| [`PTRACE_O_TRACEFORK`](#ptrace-o-tracefork) | const |  |
| [`PTRACE_O_TRACEVFORK`](#ptrace-o-tracevfork) | const |  |
| [`PTRACE_O_TRACECLONE`](#ptrace-o-traceclone) | const |  |
| [`PTRACE_O_TRACEEXEC`](#ptrace-o-traceexec) | const |  |
| [`PTRACE_O_TRACEVFORKDONE`](#ptrace-o-tracevforkdone) | const |  |
| [`PTRACE_O_TRACEEXIT`](#ptrace-o-traceexit) | const |  |
| [`PTRACE_O_TRACESECCOMP`](#ptrace-o-traceseccomp) | const |  |
| [`PTRACE_O_SUSPEND_SECCOMP`](#ptrace-o-suspend-seccomp) | const |  |
| [`PTRACE_O_EXITKILL`](#ptrace-o-exitkill) | const |  |
| [`PTRACE_O_MASK`](#ptrace-o-mask) | const |  |
| [`PTRACE_EVENT_FORK`](#ptrace-event-fork) | const |  |
| [`PTRACE_EVENT_VFORK`](#ptrace-event-vfork) | const |  |
| [`PTRACE_EVENT_CLONE`](#ptrace-event-clone) | const |  |
| [`PTRACE_EVENT_EXEC`](#ptrace-event-exec) | const |  |
| [`PTRACE_EVENT_VFORK_DONE`](#ptrace-event-vfork-done) | const |  |
| [`PTRACE_EVENT_EXIT`](#ptrace-event-exit) | const |  |
| [`PTRACE_EVENT_SECCOMP`](#ptrace-event-seccomp) | const |  |
| [`__WNOTHREAD`](#wnothread) | const |  |
| [`__WALL`](#wall) | const |  |
| [`__WCLONE`](#wclone) | const |  |
| [`SPLICE_F_MOVE`](#splice-f-move) | const |  |
| [`SPLICE_F_NONBLOCK`](#splice-f-nonblock) | const |  |
| [`SPLICE_F_MORE`](#splice-f-more) | const |  |
| [`SPLICE_F_GIFT`](#splice-f-gift) | const |  |
| [`RTLD_LOCAL`](#rtld-local) | const |  |
| [`RTLD_LAZY`](#rtld-lazy) | const |  |
| [`POSIX_FADV_NORMAL`](#posix-fadv-normal) | const |  |
| [`POSIX_FADV_RANDOM`](#posix-fadv-random) | const |  |
| [`POSIX_FADV_SEQUENTIAL`](#posix-fadv-sequential) | const |  |
| [`POSIX_FADV_WILLNEED`](#posix-fadv-willneed) | const |  |
| [`AT_FDCWD`](#at-fdcwd) | const |  |
| [`AT_SYMLINK_NOFOLLOW`](#at-symlink-nofollow) | const |  |
| [`AT_REMOVEDIR`](#at-removedir) | const |  |
| [`AT_SYMLINK_FOLLOW`](#at-symlink-follow) | const |  |
| [`AT_NO_AUTOMOUNT`](#at-no-automount) | const |  |
| [`AT_EMPTY_PATH`](#at-empty-path) | const |  |
| [`AT_RECURSIVE`](#at-recursive) | const |  |
| [`LOG_CRON`](#log-cron) | const |  |
| [`LOG_AUTHPRIV`](#log-authpriv) | const |  |
| [`LOG_FTP`](#log-ftp) | const |  |
| [`LOG_PERROR`](#log-perror) | const |  |
| [`PIPE_BUF`](#pipe-buf) | const |  |
| [`SI_LOAD_SHIFT`](#si-load-shift) | const |  |
| [`SI_USER`](#si-user) | const |  |
| [`SI_KERNEL`](#si-kernel) | const |  |
| [`SI_QUEUE`](#si-queue) | const |  |
| [`SI_TIMER`](#si-timer) | const |  |
| [`SI_MESGQ`](#si-mesgq) | const |  |
| [`SI_ASYNCIO`](#si-asyncio) | const |  |
| [`SI_SIGIO`](#si-sigio) | const |  |
| [`SI_TKILL`](#si-tkill) | const |  |
| [`SI_ASYNCNL`](#si-asyncnl) | const |  |
| [`BUS_ADRALN`](#bus-adraln) | const |  |
| [`BUS_ADRERR`](#bus-adrerr) | const |  |
| [`BUS_OBJERR`](#bus-objerr) | const |  |
| [`BUS_MCEERR_AR`](#bus-mceerr-ar) | const |  |
| [`BUS_MCEERR_AO`](#bus-mceerr-ao) | const |  |
| [`TRAP_BRKPT`](#trap-brkpt) | const |  |
| [`TRAP_TRACE`](#trap-trace) | const |  |
| [`TRAP_BRANCH`](#trap-branch) | const |  |
| [`TRAP_HWBKPT`](#trap-hwbkpt) | const |  |
| [`TRAP_UNK`](#trap-unk) | const |  |
| [`CLD_EXITED`](#cld-exited) | const |  |
| [`CLD_KILLED`](#cld-killed) | const |  |
| [`CLD_DUMPED`](#cld-dumped) | const |  |
| [`CLD_TRAPPED`](#cld-trapped) | const |  |
| [`CLD_STOPPED`](#cld-stopped) | const |  |
| [`CLD_CONTINUED`](#cld-continued) | const |  |
| [`SIGEV_SIGNAL`](#sigev-signal) | const |  |
| [`SIGEV_NONE`](#sigev-none) | const |  |
| [`SIGEV_THREAD`](#sigev-thread) | const |  |
| [`P_ALL`](#p-all) | const |  |
| [`P_PID`](#p-pid) | const |  |
| [`P_PGID`](#p-pgid) | const |  |
| [`P_PIDFD`](#p-pidfd) | const |  |
| [`UTIME_OMIT`](#utime-omit) | const |  |
| [`UTIME_NOW`](#utime-now) | const |  |
| [`POLLIN`](#pollin) | const |  |
| [`POLLPRI`](#pollpri) | const |  |
| [`POLLOUT`](#pollout) | const |  |
| [`POLLERR`](#pollerr) | const |  |
| [`POLLHUP`](#pollhup) | const |  |
| [`POLLNVAL`](#pollnval) | const |  |
| [`POLLRDNORM`](#pollrdnorm) | const |  |
| [`POLLRDBAND`](#pollrdband) | const |  |
| [`POLLRDHUP`](#pollrdhup) | const |  |
| [`IPTOS_LOWDELAY`](#iptos-lowdelay) | const |  |
| [`IPTOS_THROUGHPUT`](#iptos-throughput) | const |  |
| [`IPTOS_RELIABILITY`](#iptos-reliability) | const |  |
| [`IPTOS_MINCOST`](#iptos-mincost) | const |  |
| [`IPTOS_PREC_NETCONTROL`](#iptos-prec-netcontrol) | const |  |
| [`IPTOS_PREC_INTERNETCONTROL`](#iptos-prec-internetcontrol) | const |  |
| [`IPTOS_PREC_CRITIC_ECP`](#iptos-prec-critic-ecp) | const |  |
| [`IPTOS_PREC_FLASHOVERRIDE`](#iptos-prec-flashoverride) | const |  |
| [`IPTOS_PREC_FLASH`](#iptos-prec-flash) | const |  |
| [`IPTOS_PREC_IMMEDIATE`](#iptos-prec-immediate) | const |  |
| [`IPTOS_PREC_PRIORITY`](#iptos-prec-priority) | const |  |
| [`IPTOS_PREC_ROUTINE`](#iptos-prec-routine) | const |  |
| [`IPTOS_ECN_MASK`](#iptos-ecn-mask) | const |  |
| [`IPTOS_ECN_ECT1`](#iptos-ecn-ect1) | const |  |
| [`IPTOS_ECN_ECT0`](#iptos-ecn-ect0) | const |  |
| [`IPTOS_ECN_CE`](#iptos-ecn-ce) | const |  |
| [`IPOPT_COPY`](#ipopt-copy) | const |  |
| [`IPOPT_CLASS_MASK`](#ipopt-class-mask) | const |  |
| [`IPOPT_NUMBER_MASK`](#ipopt-number-mask) | const |  |
| [`IPOPT_CONTROL`](#ipopt-control) | const |  |
| [`IPOPT_RESERVED1`](#ipopt-reserved1) | const |  |
| [`IPOPT_MEASUREMENT`](#ipopt-measurement) | const |  |
| [`IPOPT_RESERVED2`](#ipopt-reserved2) | const |  |
| [`IPOPT_END`](#ipopt-end) | const |  |
| [`IPOPT_NOOP`](#ipopt-noop) | const |  |
| [`IPOPT_SEC`](#ipopt-sec) | const |  |
| [`IPOPT_LSRR`](#ipopt-lsrr) | const |  |
| [`IPOPT_TIMESTAMP`](#ipopt-timestamp) | const |  |
| [`IPOPT_RR`](#ipopt-rr) | const |  |
| [`IPOPT_SID`](#ipopt-sid) | const |  |
| [`IPOPT_SSRR`](#ipopt-ssrr) | const |  |
| [`IPOPT_RA`](#ipopt-ra) | const |  |
| [`IPVERSION`](#ipversion) | const |  |
| [`MAXTTL`](#maxttl) | const |  |
| [`IPDEFTTL`](#ipdefttl) | const |  |
| [`IPOPT_OPTVAL`](#ipopt-optval) | const |  |
| [`IPOPT_OLEN`](#ipopt-olen) | const |  |
| [`IPOPT_OFFSET`](#ipopt-offset) | const |  |
| [`IPOPT_MINOFF`](#ipopt-minoff) | const |  |
| [`MAX_IPOPTLEN`](#max-ipoptlen) | const |  |
| [`IPOPT_NOP`](#ipopt-nop) | const |  |
| [`IPOPT_EOL`](#ipopt-eol) | const |  |
| [`IPOPT_TS`](#ipopt-ts) | const |  |
| [`IPOPT_TS_TSONLY`](#ipopt-ts-tsonly) | const |  |
| [`IPOPT_TS_TSANDADDR`](#ipopt-ts-tsandaddr) | const |  |
| [`IPOPT_TS_PRESPEC`](#ipopt-ts-prespec) | const |  |
| [`ARPOP_RREQUEST`](#arpop-rrequest) | const |  |
| [`ARPOP_RREPLY`](#arpop-rreply) | const |  |
| [`ARPOP_InREQUEST`](#arpop-inrequest) | const |  |
| [`ARPOP_InREPLY`](#arpop-inreply) | const |  |
| [`ARPOP_NAK`](#arpop-nak) | const |  |
| [`ATF_NETMASK`](#atf-netmask) | const |  |
| [`ATF_DONTPUB`](#atf-dontpub) | const |  |
| [`ARPHRD_NETROM`](#arphrd-netrom) | const |  |
| [`ARPHRD_ETHER`](#arphrd-ether) | const |  |
| [`ARPHRD_EETHER`](#arphrd-eether) | const |  |
| [`ARPHRD_AX25`](#arphrd-ax25) | const |  |
| [`ARPHRD_PRONET`](#arphrd-pronet) | const |  |
| [`ARPHRD_CHAOS`](#arphrd-chaos) | const |  |
| [`ARPHRD_IEEE802`](#arphrd-ieee802) | const |  |
| [`ARPHRD_ARCNET`](#arphrd-arcnet) | const |  |
| [`ARPHRD_APPLETLK`](#arphrd-appletlk) | const |  |
| [`ARPHRD_DLCI`](#arphrd-dlci) | const |  |
| [`ARPHRD_ATM`](#arphrd-atm) | const |  |
| [`ARPHRD_METRICOM`](#arphrd-metricom) | const |  |
| [`ARPHRD_IEEE1394`](#arphrd-ieee1394) | const |  |
| [`ARPHRD_EUI64`](#arphrd-eui64) | const |  |
| [`ARPHRD_INFINIBAND`](#arphrd-infiniband) | const |  |
| [`ARPHRD_SLIP`](#arphrd-slip) | const |  |
| [`ARPHRD_CSLIP`](#arphrd-cslip) | const |  |
| [`ARPHRD_SLIP6`](#arphrd-slip6) | const |  |
| [`ARPHRD_CSLIP6`](#arphrd-cslip6) | const |  |
| [`ARPHRD_RSRVD`](#arphrd-rsrvd) | const |  |
| [`ARPHRD_ADAPT`](#arphrd-adapt) | const |  |
| [`ARPHRD_ROSE`](#arphrd-rose) | const |  |
| [`ARPHRD_X25`](#arphrd-x25) | const |  |
| [`ARPHRD_HWX25`](#arphrd-hwx25) | const |  |
| [`ARPHRD_CAN`](#arphrd-can) | const |  |
| [`ARPHRD_PPP`](#arphrd-ppp) | const |  |
| [`ARPHRD_CISCO`](#arphrd-cisco) | const |  |
| [`ARPHRD_HDLC`](#arphrd-hdlc) | const |  |
| [`ARPHRD_LAPB`](#arphrd-lapb) | const |  |
| [`ARPHRD_DDCMP`](#arphrd-ddcmp) | const |  |
| [`ARPHRD_RAWHDLC`](#arphrd-rawhdlc) | const |  |
| [`ARPHRD_TUNNEL`](#arphrd-tunnel) | const |  |
| [`ARPHRD_TUNNEL6`](#arphrd-tunnel6) | const |  |
| [`ARPHRD_FRAD`](#arphrd-frad) | const |  |
| [`ARPHRD_SKIP`](#arphrd-skip) | const |  |
| [`ARPHRD_LOOPBACK`](#arphrd-loopback) | const |  |
| [`ARPHRD_LOCALTLK`](#arphrd-localtlk) | const |  |
| [`ARPHRD_FDDI`](#arphrd-fddi) | const |  |
| [`ARPHRD_BIF`](#arphrd-bif) | const |  |
| [`ARPHRD_SIT`](#arphrd-sit) | const |  |
| [`ARPHRD_IPDDP`](#arphrd-ipddp) | const |  |
| [`ARPHRD_IPGRE`](#arphrd-ipgre) | const |  |
| [`ARPHRD_PIMREG`](#arphrd-pimreg) | const |  |
| [`ARPHRD_HIPPI`](#arphrd-hippi) | const |  |
| [`ARPHRD_ASH`](#arphrd-ash) | const |  |
| [`ARPHRD_ECONET`](#arphrd-econet) | const |  |
| [`ARPHRD_IRDA`](#arphrd-irda) | const |  |
| [`ARPHRD_FCPP`](#arphrd-fcpp) | const |  |
| [`ARPHRD_FCAL`](#arphrd-fcal) | const |  |
| [`ARPHRD_FCPL`](#arphrd-fcpl) | const |  |
| [`ARPHRD_FCFABRIC`](#arphrd-fcfabric) | const |  |
| [`ARPHRD_IEEE802_TR`](#arphrd-ieee802-tr) | const |  |
| [`ARPHRD_IEEE80211`](#arphrd-ieee80211) | const |  |
| [`ARPHRD_IEEE80211_PRISM`](#arphrd-ieee80211-prism) | const |  |
| [`ARPHRD_IEEE80211_RADIOTAP`](#arphrd-ieee80211-radiotap) | const |  |
| [`ARPHRD_IEEE802154`](#arphrd-ieee802154) | const |  |
| [`ARPHRD_VOID`](#arphrd-void) | const |  |
| [`ARPHRD_NONE`](#arphrd-none) | const |  |
| [`IFF_TUN`](#iff-tun) | const |  |
| [`IFF_TAP`](#iff-tap) | const |  |
| [`IFF_NAPI`](#iff-napi) | const |  |
| [`IFF_NAPI_FRAGS`](#iff-napi-frags) | const |  |
| [`IFF_NO_CARRIER`](#iff-no-carrier) | const |  |
| [`IFF_NO_PI`](#iff-no-pi) | const |  |
| [`TUN_READQ_SIZE`](#tun-readq-size) | const |  |
| [`TUN_TUN_DEV`](#tun-tun-dev) | const |  |
| [`TUN_TAP_DEV`](#tun-tap-dev) | const |  |
| [`TUN_TYPE_MASK`](#tun-type-mask) | const |  |
| [`IFF_ONE_QUEUE`](#iff-one-queue) | const |  |
| [`IFF_VNET_HDR`](#iff-vnet-hdr) | const |  |
| [`IFF_TUN_EXCL`](#iff-tun-excl) | const |  |
| [`IFF_MULTI_QUEUE`](#iff-multi-queue) | const |  |
| [`IFF_ATTACH_QUEUE`](#iff-attach-queue) | const |  |
| [`IFF_DETACH_QUEUE`](#iff-detach-queue) | const |  |
| [`IFF_PERSIST`](#iff-persist) | const |  |
| [`IFF_NOFILTER`](#iff-nofilter) | const |  |
| [`TUN_TX_TIMESTAMP`](#tun-tx-timestamp) | const |  |
| [`TUN_F_CSUM`](#tun-f-csum) | const |  |
| [`TUN_F_TSO4`](#tun-f-tso4) | const |  |
| [`TUN_F_TSO6`](#tun-f-tso6) | const |  |
| [`TUN_F_TSO_ECN`](#tun-f-tso-ecn) | const |  |
| [`TUN_F_UFO`](#tun-f-ufo) | const |  |
| [`TUN_F_USO4`](#tun-f-uso4) | const |  |
| [`TUN_F_USO6`](#tun-f-uso6) | const |  |
| [`TUN_PKT_STRIP`](#tun-pkt-strip) | const |  |
| [`TUN_FLT_ALLMULTI`](#tun-flt-allmulti) | const |  |
| [`T_TYPE`](#t-type) | const |  |
| [`TUNSETNOCSUM`](#tunsetnocsum) | const |  |
| [`TUNSETDEBUG`](#tunsetdebug) | const |  |
| [`TUNSETIFF`](#tunsetiff) | const |  |
| [`TUNSETPERSIST`](#tunsetpersist) | const |  |
| [`TUNSETOWNER`](#tunsetowner) | const |  |
| [`TUNSETLINK`](#tunsetlink) | const |  |
| [`TUNSETGROUP`](#tunsetgroup) | const |  |
| [`TUNGETFEATURES`](#tungetfeatures) | const |  |
| [`TUNSETOFFLOAD`](#tunsetoffload) | const |  |
| [`TUNSETTXFILTER`](#tunsettxfilter) | const |  |
| [`TUNGETIFF`](#tungetiff) | const |  |
| [`TUNGETSNDBUF`](#tungetsndbuf) | const |  |
| [`TUNSETSNDBUF`](#tunsetsndbuf) | const |  |
| [`TUNATTACHFILTER`](#tunattachfilter) | const |  |
| [`TUNDETACHFILTER`](#tundetachfilter) | const |  |
| [`TUNGETVNETHDRSZ`](#tungetvnethdrsz) | const |  |
| [`TUNSETVNETHDRSZ`](#tunsetvnethdrsz) | const |  |
| [`TUNSETQUEUE`](#tunsetqueue) | const |  |
| [`TUNSETIFINDEX`](#tunsetifindex) | const |  |
| [`TUNGETFILTER`](#tungetfilter) | const |  |
| [`TUNSETVNETLE`](#tunsetvnetle) | const |  |
| [`TUNGETVNETLE`](#tungetvnetle) | const |  |
| [`TUNSETVNETBE`](#tunsetvnetbe) | const |  |
| [`TUNGETVNETBE`](#tungetvnetbe) | const |  |
| [`TUNSETSTEERINGEBPF`](#tunsetsteeringebpf) | const |  |
| [`TUNSETFILTEREBPF`](#tunsetfilterebpf) | const |  |
| [`TUNSETCARRIER`](#tunsetcarrier) | const |  |
| [`TUNGETDEVNETNS`](#tungetdevnetns) | const |  |
| [`FS_IOC_GETFLAGS`](#fs-ioc-getflags) | const |  |
| [`FS_IOC_SETFLAGS`](#fs-ioc-setflags) | const |  |
| [`FS_IOC_GETVERSION`](#fs-ioc-getversion) | const |  |
| [`FS_IOC_SETVERSION`](#fs-ioc-setversion) | const |  |
| [`FS_IOC32_GETFLAGS`](#fs-ioc32-getflags) | const |  |
| [`FS_IOC32_SETFLAGS`](#fs-ioc32-setflags) | const |  |
| [`FS_IOC32_GETVERSION`](#fs-ioc32-getversion) | const |  |
| [`FS_IOC32_SETVERSION`](#fs-ioc32-setversion) | const |  |
| [`FICLONE`](#ficlone) | const |  |
| [`FICLONERANGE`](#ficlonerange) | const |  |
| [`ADFS_SUPER_MAGIC`](#adfs-super-magic) | const |  |
| [`AFFS_SUPER_MAGIC`](#affs-super-magic) | const |  |
| [`AFS_SUPER_MAGIC`](#afs-super-magic) | const |  |
| [`AUTOFS_SUPER_MAGIC`](#autofs-super-magic) | const |  |
| [`BPF_FS_MAGIC`](#bpf-fs-magic) | const |  |
| [`BTRFS_SUPER_MAGIC`](#btrfs-super-magic) | const |  |
| [`CGROUP2_SUPER_MAGIC`](#cgroup2-super-magic) | const |  |
| [`CGROUP_SUPER_MAGIC`](#cgroup-super-magic) | const |  |
| [`CODA_SUPER_MAGIC`](#coda-super-magic) | const |  |
| [`CRAMFS_MAGIC`](#cramfs-magic) | const |  |
| [`DEBUGFS_MAGIC`](#debugfs-magic) | const |  |
| [`DEVPTS_SUPER_MAGIC`](#devpts-super-magic) | const |  |
| [`ECRYPTFS_SUPER_MAGIC`](#ecryptfs-super-magic) | const |  |
| [`EFS_SUPER_MAGIC`](#efs-super-magic) | const |  |
| [`EXT2_SUPER_MAGIC`](#ext2-super-magic) | const |  |
| [`EXT3_SUPER_MAGIC`](#ext3-super-magic) | const |  |
| [`EXT4_SUPER_MAGIC`](#ext4-super-magic) | const |  |
| [`F2FS_SUPER_MAGIC`](#f2fs-super-magic) | const |  |
| [`FUSE_SUPER_MAGIC`](#fuse-super-magic) | const |  |
| [`FUTEXFS_SUPER_MAGIC`](#futexfs-super-magic) | const |  |
| [`HOSTFS_SUPER_MAGIC`](#hostfs-super-magic) | const |  |
| [`HPFS_SUPER_MAGIC`](#hpfs-super-magic) | const |  |
| [`HUGETLBFS_MAGIC`](#hugetlbfs-magic) | const |  |
| [`ISOFS_SUPER_MAGIC`](#isofs-super-magic) | const |  |
| [`JFFS2_SUPER_MAGIC`](#jffs2-super-magic) | const |  |
| [`MINIX2_SUPER_MAGIC2`](#minix2-super-magic2) | const |  |
| [`MINIX2_SUPER_MAGIC`](#minix2-super-magic) | const |  |
| [`MINIX3_SUPER_MAGIC`](#minix3-super-magic) | const |  |
| [`MINIX_SUPER_MAGIC2`](#minix-super-magic2) | const |  |
| [`MINIX_SUPER_MAGIC`](#minix-super-magic) | const |  |
| [`MSDOS_SUPER_MAGIC`](#msdos-super-magic) | const |  |
| [`NCP_SUPER_MAGIC`](#ncp-super-magic) | const |  |
| [`NFS_SUPER_MAGIC`](#nfs-super-magic) | const |  |
| [`NILFS_SUPER_MAGIC`](#nilfs-super-magic) | const |  |
| [`OCFS2_SUPER_MAGIC`](#ocfs2-super-magic) | const |  |
| [`OPENPROM_SUPER_MAGIC`](#openprom-super-magic) | const |  |
| [`OVERLAYFS_SUPER_MAGIC`](#overlayfs-super-magic) | const |  |
| [`PROC_SUPER_MAGIC`](#proc-super-magic) | const |  |
| [`QNX4_SUPER_MAGIC`](#qnx4-super-magic) | const |  |
| [`QNX6_SUPER_MAGIC`](#qnx6-super-magic) | const |  |
| [`RDTGROUP_SUPER_MAGIC`](#rdtgroup-super-magic) | const |  |
| [`REISERFS_SUPER_MAGIC`](#reiserfs-super-magic) | const |  |
| [`SECURITYFS_MAGIC`](#securityfs-magic) | const |  |
| [`SELINUX_MAGIC`](#selinux-magic) | const |  |
| [`SMACK_MAGIC`](#smack-magic) | const |  |
| [`SMB_SUPER_MAGIC`](#smb-super-magic) | const |  |
| [`SYSFS_MAGIC`](#sysfs-magic) | const |  |
| [`TMPFS_MAGIC`](#tmpfs-magic) | const |  |
| [`TRACEFS_MAGIC`](#tracefs-magic) | const |  |
| [`UDF_SUPER_MAGIC`](#udf-super-magic) | const |  |
| [`USBDEVICE_SUPER_MAGIC`](#usbdevice-super-magic) | const |  |
| [`XENFS_SUPER_MAGIC`](#xenfs-super-magic) | const |  |
| [`NSFS_MAGIC`](#nsfs-magic) | const |  |
| [`AT_STATX_SYNC_TYPE`](#at-statx-sync-type) | const |  |
| [`AT_STATX_SYNC_AS_STAT`](#at-statx-sync-as-stat) | const |  |
| [`AT_STATX_FORCE_SYNC`](#at-statx-force-sync) | const |  |
| [`AT_STATX_DONT_SYNC`](#at-statx-dont-sync) | const |  |
| [`STATX_TYPE`](#statx-type) | const |  |
| [`STATX_MODE`](#statx-mode) | const |  |
| [`STATX_NLINK`](#statx-nlink) | const |  |
| [`STATX_UID`](#statx-uid) | const |  |
| [`STATX_GID`](#statx-gid) | const |  |
| [`STATX_ATIME`](#statx-atime) | const |  |
| [`STATX_MTIME`](#statx-mtime) | const |  |
| [`STATX_CTIME`](#statx-ctime) | const |  |
| [`STATX_INO`](#statx-ino) | const |  |
| [`STATX_SIZE`](#statx-size) | const |  |
| [`STATX_BLOCKS`](#statx-blocks) | const |  |
| [`STATX_BASIC_STATS`](#statx-basic-stats) | const |  |
| [`STATX_BTIME`](#statx-btime) | const |  |
| [`STATX_ALL`](#statx-all) | const |  |
| [`STATX_MNT_ID`](#statx-mnt-id) | const |  |
| [`STATX_DIOALIGN`](#statx-dioalign) | const |  |
| [`STATX__RESERVED`](#statx-reserved) | const |  |
| [`STATX_ATTR_COMPRESSED`](#statx-attr-compressed) | const |  |
| [`STATX_ATTR_IMMUTABLE`](#statx-attr-immutable) | const |  |
| [`STATX_ATTR_APPEND`](#statx-attr-append) | const |  |
| [`STATX_ATTR_NODUMP`](#statx-attr-nodump) | const |  |
| [`STATX_ATTR_ENCRYPTED`](#statx-attr-encrypted) | const |  |
| [`STATX_ATTR_AUTOMOUNT`](#statx-attr-automount) | const |  |
| [`STATX_ATTR_MOUNT_ROOT`](#statx-attr-mount-root) | const |  |
| [`STATX_ATTR_VERITY`](#statx-attr-verity) | const |  |
| [`STATX_ATTR_DAX`](#statx-attr-dax) | const |  |
| [`_IOC_NRBITS`](#ioc-nrbits) | const |  |
| [`_IOC_TYPEBITS`](#ioc-typebits) | const |  |
| [`_IOC_SIZEBITS`](#ioc-sizebits) | const |  |
| [`_IOC_DIRBITS`](#ioc-dirbits) | const |  |
| [`_IOC_NONE`](#ioc-none) | const |  |
| [`_IOC_WRITE`](#ioc-write) | const |  |
| [`_IOC_READ`](#ioc-read) | const |  |
| [`_IOC_NRMASK`](#ioc-nrmask) | const |  |
| [`_IOC_TYPEMASK`](#ioc-typemask) | const |  |
| [`_IOC_SIZEMASK`](#ioc-sizemask) | const |  |
| [`_IOC_DIRMASK`](#ioc-dirmask) | const |  |
| [`_IOC_NRSHIFT`](#ioc-nrshift) | const |  |
| [`_IOC_TYPESHIFT`](#ioc-typeshift) | const |  |
| [`_IOC_SIZESHIFT`](#ioc-sizeshift) | const |  |
| [`_IOC_DIRSHIFT`](#ioc-dirshift) | const |  |

## Modules

- [`linux_like`](linux_like/index.md)
- [`linux`](linux/index.md) — Linux-specific definitions for linux-like values
- [`linux_l4re_shared`](linux_l4re_shared/index.md)

## Structs

### `group`

```rust
struct group {
    pub gr_name: *mut c_char,
    pub gr_passwd: *mut c_char,
    pub gr_gid: crate::gid_t,
    pub gr_mem: *mut *mut c_char,
}
```

#### Trait Implementations

##### `impl Clone for group`

- <span id="group-clone"></span>`fn clone(&self) -> group` — [`group`](../index.md#group)

##### `impl Copy for group`

##### `impl Debug for group`

- <span id="group-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `utimbuf`

```rust
struct utimbuf {
    pub actime: time_t,
    pub modtime: time_t,
}
```

#### Trait Implementations

##### `impl Clone for utimbuf`

- <span id="utimbuf-clone"></span>`fn clone(&self) -> utimbuf` — [`utimbuf`](../index.md#utimbuf)

##### `impl Copy for utimbuf`

##### `impl Debug for utimbuf`

- <span id="utimbuf-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `timeval`

```rust
struct timeval {
    pub tv_sec: time_t,
    pub tv_usec: crate::suseconds_t,
}
```

#### Trait Implementations

##### `impl Clone for timeval`

- <span id="timeval-clone"></span>`fn clone(&self) -> timeval` — [`timeval`](../index.md#timeval)

##### `impl Copy for timeval`

##### `impl Debug for timeval`

- <span id="timeval-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for timeval`

- <span id="timeval-default"></span>`fn default() -> timeval` — [`timeval`](../index.md#timeval)

### `rlimit`

```rust
struct rlimit {
    pub rlim_cur: rlim_t,
    pub rlim_max: rlim_t,
}
```

#### Trait Implementations

##### `impl Clone for rlimit`

- <span id="rlimit-clone"></span>`fn clone(&self) -> rlimit` — [`rlimit`](../index.md#rlimit)

##### `impl Copy for rlimit`

##### `impl Debug for rlimit`

- <span id="rlimit-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `rusage`

```rust
struct rusage {
    pub ru_utime: timeval,
    pub ru_stime: timeval,
    pub ru_maxrss: c_long,
    pub ru_ixrss: c_long,
    pub ru_idrss: c_long,
    pub ru_isrss: c_long,
    pub ru_minflt: c_long,
    pub ru_majflt: c_long,
    pub ru_nswap: c_long,
    pub ru_inblock: c_long,
    pub ru_oublock: c_long,
    pub ru_msgsnd: c_long,
    pub ru_msgrcv: c_long,
    pub ru_nsignals: c_long,
    pub ru_nvcsw: c_long,
    pub ru_nivcsw: c_long,
}
```

#### Trait Implementations

##### `impl Clone for rusage`

- <span id="rusage-clone"></span>`fn clone(&self) -> rusage` — [`rusage`](../index.md#rusage)

##### `impl Copy for rusage`

##### `impl Debug for rusage`

- <span id="rusage-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ipv6_mreq`

```rust
struct ipv6_mreq {
    pub ipv6mr_multiaddr: in6_addr,
    pub ipv6mr_interface: c_uint,
}
```

#### Trait Implementations

##### `impl Clone for ipv6_mreq`

- <span id="ipv6-mreq-clone"></span>`fn clone(&self) -> ipv6_mreq` — [`ipv6_mreq`](../index.md#ipv6-mreq)

##### `impl Copy for ipv6_mreq`

##### `impl Debug for ipv6_mreq`

- <span id="ipv6-mreq-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `hostent`

```rust
struct hostent {
    pub h_name: *mut c_char,
    pub h_aliases: *mut *mut c_char,
    pub h_addrtype: c_int,
    pub h_length: c_int,
    pub h_addr_list: *mut *mut c_char,
}
```

#### Trait Implementations

##### `impl Clone for hostent`

- <span id="hostent-clone"></span>`fn clone(&self) -> hostent` — [`hostent`](../index.md#hostent)

##### `impl Copy for hostent`

##### `impl Debug for hostent`

- <span id="hostent-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `iovec`

```rust
struct iovec {
    pub iov_base: *mut c_void,
    pub iov_len: size_t,
}
```

#### Trait Implementations

##### `impl Clone for iovec`

- <span id="iovec-clone"></span>`fn clone(&self) -> iovec` — [`iovec`](../index.md#iovec)

##### `impl Copy for iovec`

##### `impl Debug for iovec`

- <span id="iovec-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `pollfd`

```rust
struct pollfd {
    pub fd: c_int,
    pub events: c_short,
    pub revents: c_short,
}
```

#### Trait Implementations

##### `impl Clone for pollfd`

- <span id="pollfd-clone"></span>`fn clone(&self) -> pollfd` — [`pollfd`](../index.md#pollfd)

##### `impl Copy for pollfd`

##### `impl Debug for pollfd`

- <span id="pollfd-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `winsize`

```rust
struct winsize {
    pub ws_row: c_ushort,
    pub ws_col: c_ushort,
    pub ws_xpixel: c_ushort,
    pub ws_ypixel: c_ushort,
}
```

#### Trait Implementations

##### `impl Clone for winsize`

- <span id="winsize-clone"></span>`fn clone(&self) -> winsize` — [`winsize`](../index.md#winsize)

##### `impl Copy for winsize`

##### `impl Debug for winsize`

- <span id="winsize-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `linger`

```rust
struct linger {
    pub l_onoff: c_int,
    pub l_linger: c_int,
}
```

#### Trait Implementations

##### `impl Clone for linger`

- <span id="linger-clone"></span>`fn clone(&self) -> linger` — [`linger`](../index.md#linger)

##### `impl Copy for linger`

##### `impl Debug for linger`

- <span id="linger-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `sigval`

```rust
struct sigval {
    pub sival_ptr: *mut c_void,
}
```

#### Trait Implementations

##### `impl Clone for sigval`

- <span id="sigval-clone"></span>`fn clone(&self) -> sigval` — [`sigval`](../index.md#sigval)

##### `impl Copy for sigval`

##### `impl Debug for sigval`

- <span id="sigval-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `itimerval`

```rust
struct itimerval {
    pub it_interval: crate::timeval,
    pub it_value: crate::timeval,
}
```

#### Trait Implementations

##### `impl Clone for itimerval`

- <span id="itimerval-clone"></span>`fn clone(&self) -> itimerval` — [`itimerval`](../index.md#itimerval)

##### `impl Copy for itimerval`

##### `impl Debug for itimerval`

- <span id="itimerval-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `tms`

```rust
struct tms {
    pub tms_utime: crate::clock_t,
    pub tms_stime: crate::clock_t,
    pub tms_cutime: crate::clock_t,
    pub tms_cstime: crate::clock_t,
}
```

#### Trait Implementations

##### `impl Clone for tms`

- <span id="tms-clone"></span>`fn clone(&self) -> tms` — [`tms`](../index.md#tms)

##### `impl Copy for tms`

##### `impl Debug for tms`

- <span id="tms-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `servent`

```rust
struct servent {
    pub s_name: *mut c_char,
    pub s_aliases: *mut *mut c_char,
    pub s_port: c_int,
    pub s_proto: *mut c_char,
}
```

#### Trait Implementations

##### `impl Clone for servent`

- <span id="servent-clone"></span>`fn clone(&self) -> servent` — [`servent`](../index.md#servent)

##### `impl Copy for servent`

##### `impl Debug for servent`

- <span id="servent-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `protoent`

```rust
struct protoent {
    pub p_name: *mut c_char,
    pub p_aliases: *mut *mut c_char,
    pub p_proto: c_int,
}
```

#### Trait Implementations

##### `impl Clone for protoent`

- <span id="protoent-clone"></span>`fn clone(&self) -> protoent` — [`protoent`](../index.md#protoent)

##### `impl Copy for protoent`

##### `impl Debug for protoent`

- <span id="protoent-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `in6_addr`

```rust
struct in6_addr {
    pub s6_addr: [u8; 16],
}
```

#### Trait Implementations

##### `impl Clone for in6_addr`

- <span id="in6-addr-clone"></span>`fn clone(&self) -> in6_addr` — [`in6_addr`](../index.md#in6-addr)

##### `impl Copy for in6_addr`

##### `impl Debug for in6_addr`

- <span id="in6-addr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `in_addr`

```rust
struct in_addr {
    pub s_addr: crate::in_addr_t,
}
```

#### Trait Implementations

##### `impl Clone for in_addr`

- <span id="in-addr-clone"></span>`fn clone(&self) -> in_addr` — [`in_addr`](#in-addr)

##### `impl Copy for in_addr`

##### `impl Debug for in_addr`

- <span id="in-addr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ip_mreq`

```rust
struct ip_mreq {
    pub imr_multiaddr: in_addr,
    pub imr_interface: in_addr,
}
```

#### Trait Implementations

##### `impl Clone for ip_mreq`

- <span id="ip-mreq-clone"></span>`fn clone(&self) -> ip_mreq` — [`ip_mreq`](#ip-mreq)

##### `impl Copy for ip_mreq`

##### `impl Debug for ip_mreq`

- <span id="ip-mreq-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ip_mreqn`

```rust
struct ip_mreqn {
    pub imr_multiaddr: in_addr,
    pub imr_address: in_addr,
    pub imr_ifindex: c_int,
}
```

#### Trait Implementations

##### `impl Clone for ip_mreqn`

- <span id="ip-mreqn-clone"></span>`fn clone(&self) -> ip_mreqn` — [`ip_mreqn`](#ip-mreqn)

##### `impl Copy for ip_mreqn`

##### `impl Debug for ip_mreqn`

- <span id="ip-mreqn-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ip_mreq_source`

```rust
struct ip_mreq_source {
    pub imr_multiaddr: in_addr,
    pub imr_interface: in_addr,
    pub imr_sourceaddr: in_addr,
}
```

#### Trait Implementations

##### `impl Clone for ip_mreq_source`

- <span id="ip-mreq-source-clone"></span>`fn clone(&self) -> ip_mreq_source` — [`ip_mreq_source`](#ip-mreq-source)

##### `impl Copy for ip_mreq_source`

##### `impl Debug for ip_mreq_source`

- <span id="ip-mreq-source-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `sockaddr`

```rust
struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [c_char; 14],
}
```

#### Trait Implementations

##### `impl Clone for sockaddr`

- <span id="sockaddr-clone"></span>`fn clone(&self) -> sockaddr` — [`sockaddr`](#sockaddr)

##### `impl Copy for sockaddr`

##### `impl Debug for sockaddr`

- <span id="sockaddr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `sockaddr_in`

```rust
struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: crate::in_port_t,
    pub sin_addr: crate::in_addr,
    pub sin_zero: [u8; 8],
}
```

#### Trait Implementations

##### `impl Clone for sockaddr_in`

- <span id="sockaddr-in-clone"></span>`fn clone(&self) -> sockaddr_in` — [`sockaddr_in`](#sockaddr-in)

##### `impl Copy for sockaddr_in`

##### `impl Debug for sockaddr_in`

- <span id="sockaddr-in-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `sockaddr_in6`

```rust
struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: crate::in_port_t,
    pub sin6_flowinfo: u32,
    pub sin6_addr: crate::in6_addr,
    pub sin6_scope_id: u32,
}
```

#### Trait Implementations

##### `impl Clone for sockaddr_in6`

- <span id="sockaddr-in6-clone"></span>`fn clone(&self) -> sockaddr_in6` — [`sockaddr_in6`](#sockaddr-in6)

##### `impl Copy for sockaddr_in6`

##### `impl Debug for sockaddr_in6`

- <span id="sockaddr-in6-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `addrinfo`

```rust
struct addrinfo {
    pub ai_flags: c_int,
    pub ai_family: c_int,
    pub ai_socktype: c_int,
    pub ai_protocol: c_int,
    pub ai_addrlen: socklen_t,
    pub ai_addr: *mut crate::sockaddr,
    pub ai_canonname: *mut c_char,
    pub ai_next: *mut addrinfo,
}
```

#### Trait Implementations

##### `impl Clone for addrinfo`

- <span id="addrinfo-clone"></span>`fn clone(&self) -> addrinfo` — [`addrinfo`](#addrinfo)

##### `impl Copy for addrinfo`

##### `impl Debug for addrinfo`

- <span id="addrinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `sockaddr_ll`

```rust
struct sockaddr_ll {
    pub sll_family: c_ushort,
    pub sll_protocol: c_ushort,
    pub sll_ifindex: c_int,
    pub sll_hatype: c_ushort,
    pub sll_pkttype: c_uchar,
    pub sll_halen: c_uchar,
    pub sll_addr: [c_uchar; 8],
}
```

#### Trait Implementations

##### `impl Clone for sockaddr_ll`

- <span id="sockaddr-ll-clone"></span>`fn clone(&self) -> sockaddr_ll` — [`sockaddr_ll`](#sockaddr-ll)

##### `impl Copy for sockaddr_ll`

##### `impl Debug for sockaddr_ll`

- <span id="sockaddr-ll-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `fd_set`

```rust
struct fd_set {
    fds_bits: [c_ulong; 16],
}
```

#### Trait Implementations

##### `impl Clone for fd_set`

- <span id="fd-set-clone"></span>`fn clone(&self) -> fd_set` — [`fd_set`](#fd-set)

##### `impl Copy for fd_set`

##### `impl Debug for fd_set`

- <span id="fd-set-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `tm`

```rust
struct tm {
    pub tm_sec: c_int,
    pub tm_min: c_int,
    pub tm_hour: c_int,
    pub tm_mday: c_int,
    pub tm_mon: c_int,
    pub tm_year: c_int,
    pub tm_wday: c_int,
    pub tm_yday: c_int,
    pub tm_isdst: c_int,
    pub tm_gmtoff: c_long,
    pub tm_zone: *const c_char,
}
```

#### Trait Implementations

##### `impl Clone for tm`

- <span id="tm-clone"></span>`fn clone(&self) -> tm` — [`tm`](#tm)

##### `impl Copy for tm`

##### `impl Debug for tm`

- <span id="tm-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `sched_param`

```rust
struct sched_param {
    pub sched_priority: c_int,
}
```

#### Trait Implementations

##### `impl Clone for sched_param`

- <span id="sched-param-clone"></span>`fn clone(&self) -> sched_param` — [`sched_param`](#sched-param)

##### `impl Copy for sched_param`

##### `impl Debug for sched_param`

- <span id="sched-param-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Dl_info`

```rust
struct Dl_info {
    pub dli_fname: *const c_char,
    pub dli_fbase: *mut c_void,
    pub dli_sname: *const c_char,
    pub dli_saddr: *mut c_void,
}
```

#### Trait Implementations

##### `impl Clone for Dl_info`

- <span id="dl-info-clone"></span>`fn clone(&self) -> Dl_info` — [`Dl_info`](#dl-info)

##### `impl Copy for Dl_info`

##### `impl Debug for Dl_info`

- <span id="dl-info-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `lconv`

```rust
struct lconv {
    pub decimal_point: *mut c_char,
    pub thousands_sep: *mut c_char,
    pub grouping: *mut c_char,
    pub int_curr_symbol: *mut c_char,
    pub currency_symbol: *mut c_char,
    pub mon_decimal_point: *mut c_char,
    pub mon_thousands_sep: *mut c_char,
    pub mon_grouping: *mut c_char,
    pub positive_sign: *mut c_char,
    pub negative_sign: *mut c_char,
    pub int_frac_digits: c_char,
    pub frac_digits: c_char,
    pub p_cs_precedes: c_char,
    pub p_sep_by_space: c_char,
    pub n_cs_precedes: c_char,
    pub n_sep_by_space: c_char,
    pub p_sign_posn: c_char,
    pub n_sign_posn: c_char,
    pub int_p_cs_precedes: c_char,
    pub int_p_sep_by_space: c_char,
    pub int_n_cs_precedes: c_char,
    pub int_n_sep_by_space: c_char,
    pub int_p_sign_posn: c_char,
    pub int_n_sign_posn: c_char,
}
```

#### Trait Implementations

##### `impl Clone for lconv`

- <span id="lconv-clone"></span>`fn clone(&self) -> lconv` — [`lconv`](#lconv)

##### `impl Copy for lconv`

##### `impl Debug for lconv`

- <span id="lconv-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `in_pktinfo`

```rust
struct in_pktinfo {
    pub ipi_ifindex: c_int,
    pub ipi_spec_dst: crate::in_addr,
    pub ipi_addr: crate::in_addr,
}
```

#### Trait Implementations

##### `impl Clone for in_pktinfo`

- <span id="in-pktinfo-clone"></span>`fn clone(&self) -> in_pktinfo` — [`in_pktinfo`](#in-pktinfo)

##### `impl Copy for in_pktinfo`

##### `impl Debug for in_pktinfo`

- <span id="in-pktinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ifaddrs`

```rust
struct ifaddrs {
    pub ifa_next: *mut ifaddrs,
    pub ifa_name: *mut c_char,
    pub ifa_flags: c_uint,
    pub ifa_addr: *mut crate::sockaddr,
    pub ifa_netmask: *mut crate::sockaddr,
    pub ifa_ifu: *mut crate::sockaddr,
    pub ifa_data: *mut c_void,
}
```

#### Trait Implementations

##### `impl Clone for ifaddrs`

- <span id="ifaddrs-clone"></span>`fn clone(&self) -> ifaddrs` — [`ifaddrs`](#ifaddrs)

##### `impl Copy for ifaddrs`

##### `impl Debug for ifaddrs`

- <span id="ifaddrs-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `in6_rtmsg`

```rust
struct in6_rtmsg {
    rtmsg_dst: crate::in6_addr,
    rtmsg_src: crate::in6_addr,
    rtmsg_gateway: crate::in6_addr,
    rtmsg_type: u32,
    rtmsg_dst_len: u16,
    rtmsg_src_len: u16,
    rtmsg_metric: u32,
    rtmsg_info: c_ulong,
    rtmsg_flags: u32,
    rtmsg_ifindex: c_int,
}
```

#### Trait Implementations

##### `impl Clone for in6_rtmsg`

- <span id="in6-rtmsg-clone"></span>`fn clone(&self) -> in6_rtmsg` — [`in6_rtmsg`](#in6-rtmsg)

##### `impl Copy for in6_rtmsg`

##### `impl Debug for in6_rtmsg`

- <span id="in6-rtmsg-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `arpreq`

```rust
struct arpreq {
    pub arp_pa: crate::sockaddr,
    pub arp_ha: crate::sockaddr,
    pub arp_flags: c_int,
    pub arp_netmask: crate::sockaddr,
    pub arp_dev: [c_char; 16],
}
```

#### Trait Implementations

##### `impl Clone for arpreq`

- <span id="arpreq-clone"></span>`fn clone(&self) -> arpreq` — [`arpreq`](#arpreq)

##### `impl Copy for arpreq`

##### `impl Debug for arpreq`

- <span id="arpreq-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `arpreq_old`

```rust
struct arpreq_old {
    pub arp_pa: crate::sockaddr,
    pub arp_ha: crate::sockaddr,
    pub arp_flags: c_int,
    pub arp_netmask: crate::sockaddr,
}
```

#### Trait Implementations

##### `impl Clone for arpreq_old`

- <span id="arpreq-old-clone"></span>`fn clone(&self) -> arpreq_old` — [`arpreq_old`](#arpreq-old)

##### `impl Copy for arpreq_old`

##### `impl Debug for arpreq_old`

- <span id="arpreq-old-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `arphdr`

```rust
struct arphdr {
    pub ar_hrd: u16,
    pub ar_pro: u16,
    pub ar_hln: u8,
    pub ar_pln: u8,
    pub ar_op: u16,
}
```

#### Trait Implementations

##### `impl Clone for arphdr`

- <span id="arphdr-clone"></span>`fn clone(&self) -> arphdr` — [`arphdr`](#arphdr)

##### `impl Copy for arphdr`

##### `impl Debug for arphdr`

- <span id="arphdr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `mmsghdr`

```rust
struct mmsghdr {
    pub msg_hdr: crate::msghdr,
    pub msg_len: c_uint,
}
```

#### Trait Implementations

##### `impl Clone for mmsghdr`

- <span id="mmsghdr-clone"></span>`fn clone(&self) -> mmsghdr` — [`mmsghdr`](#mmsghdr)

##### `impl Copy for mmsghdr`

##### `impl Debug for mmsghdr`

- <span id="mmsghdr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `sockaddr_un`

```rust
struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [c_char; 108],
}
```

#### Trait Implementations

##### `impl Clone for sockaddr_un`

- <span id="sockaddr-un-clone"></span>`fn clone(&self) -> sockaddr_un` — [`sockaddr_un`](#sockaddr-un)

##### `impl Copy for sockaddr_un`

##### `impl Debug for sockaddr_un`

- <span id="sockaddr-un-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `sockaddr_storage`

```rust
struct sockaddr_storage {
    pub ss_family: sa_family_t,
    __ss_pad2: Padding<[u8; 118]>,
    __ss_align: size_t,
}
```

#### Trait Implementations

##### `impl Clone for sockaddr_storage`

- <span id="sockaddr-storage-clone"></span>`fn clone(&self) -> sockaddr_storage` — [`sockaddr_storage`](#sockaddr-storage)

##### `impl Copy for sockaddr_storage`

##### `impl Debug for sockaddr_storage`

- <span id="sockaddr-storage-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `utsname`

```rust
struct utsname {
    pub sysname: [c_char; 65],
    pub nodename: [c_char; 65],
    pub release: [c_char; 65],
    pub version: [c_char; 65],
    pub machine: [c_char; 65],
    pub domainname: [c_char; 65],
}
```

#### Trait Implementations

##### `impl Clone for utsname`

- <span id="utsname-clone"></span>`fn clone(&self) -> utsname` — [`utsname`](#utsname)

##### `impl Copy for utsname`

##### `impl Debug for utsname`

- <span id="utsname-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `if_nameindex`

```rust
struct if_nameindex {
    pub if_index: c_uint,
    pub if_name: *mut c_char,
}
```

#### Trait Implementations

##### `impl Clone for if_nameindex`

- <span id="if-nameindex-clone"></span>`fn clone(&self) -> if_nameindex` — [`if_nameindex`](#if-nameindex)

##### `impl Copy for if_nameindex`

##### `impl Debug for if_nameindex`

- <span id="if-nameindex-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `file_clone_range`

```rust
struct file_clone_range {
    pub src_fd: crate::__s64,
    pub src_offset: crate::__u64,
    pub src_length: crate::__u64,
    pub dest_offset: crate::__u64,
}
```

#### Trait Implementations

##### `impl Clone for file_clone_range`

- <span id="file-clone-range-clone"></span>`fn clone(&self) -> file_clone_range` — [`file_clone_range`](#file-clone-range)

##### `impl Copy for file_clone_range`

##### `impl Debug for file_clone_range`

- <span id="file-clone-range-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `sock_filter`

```rust
struct sock_filter {
    pub code: __u16,
    pub jt: __u8,
    pub jf: __u8,
    pub k: __u32,
}
```

#### Trait Implementations

##### `impl Clone for sock_filter`

- <span id="sock-filter-clone"></span>`fn clone(&self) -> sock_filter` — [`sock_filter`](#sock-filter)

##### `impl Copy for sock_filter`

##### `impl Debug for sock_filter`

- <span id="sock-filter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `sock_fprog`

```rust
struct sock_fprog {
    pub len: c_ushort,
    pub filter: *mut sock_filter,
}
```

#### Trait Implementations

##### `impl Clone for sock_fprog`

- <span id="sock-fprog-clone"></span>`fn clone(&self) -> sock_fprog` — [`sock_fprog`](#sock-fprog)

##### `impl Copy for sock_fprog`

##### `impl Debug for sock_fprog`

- <span id="sock-fprog-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `statx`

```rust
struct statx {
    pub stx_mask: crate::__u32,
    pub stx_blksize: crate::__u32,
    pub stx_attributes: crate::__u64,
    pub stx_nlink: crate::__u32,
    pub stx_uid: crate::__u32,
    pub stx_gid: crate::__u32,
    pub stx_mode: crate::__u16,
    __statx_pad1: Padding<[crate::__u16; 1]>,
    pub stx_ino: crate::__u64,
    pub stx_size: crate::__u64,
    pub stx_blocks: crate::__u64,
    pub stx_attributes_mask: crate::__u64,
    pub stx_atime: statx_timestamp,
    pub stx_btime: statx_timestamp,
    pub stx_ctime: statx_timestamp,
    pub stx_mtime: statx_timestamp,
    pub stx_rdev_major: crate::__u32,
    pub stx_rdev_minor: crate::__u32,
    pub stx_dev_major: crate::__u32,
    pub stx_dev_minor: crate::__u32,
    pub stx_mnt_id: crate::__u64,
    pub stx_dio_mem_align: crate::__u32,
    pub stx_dio_offset_align: crate::__u32,
    __statx_pad3: Padding<[crate::__u64; 12]>,
}
```

#### Trait Implementations

##### `impl Clone for statx`

- <span id="statx-clone"></span>`fn clone(&self) -> statx` — [`statx`](#statx)

##### `impl Copy for statx`

##### `impl Debug for statx`

- <span id="statx-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `statx_timestamp`

```rust
struct statx_timestamp {
    pub tv_sec: crate::__s64,
    pub tv_nsec: crate::__u32,
    __statx_timestamp_pad1: Padding<[crate::__s32; 1]>,
}
```

#### Trait Implementations

##### `impl Clone for statx_timestamp`

- <span id="statx-timestamp-clone"></span>`fn clone(&self) -> statx_timestamp` — [`statx_timestamp`](#statx-timestamp)

##### `impl Copy for statx_timestamp`

##### `impl Debug for statx_timestamp`

- <span id="statx-timestamp-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `epoll_event`

```rust
struct epoll_event {
    pub events: u32,
    pub u64: u64,
}
```

#### Trait Implementations

##### `impl Clone for epoll_event`

- <span id="epoll-event-clone"></span>`fn clone(&self) -> epoll_event` — [`epoll_event`](#epoll-event)

##### `impl Copy for epoll_event`

##### `impl Debug for epoll_event`

- <span id="epoll-event-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `sigevent`

```rust
struct sigevent {
    pub sigev_value: crate::sigval,
    pub sigev_signo: c_int,
    pub sigev_notify: c_int,
    pub sigev_notify_thread_id: c_int,
    __unused1: Padding<[c_int; 11]>,
}
```

#### Trait Implementations

##### `impl Clone for sigevent`

- <span id="sigevent-clone"></span>`fn clone(&self) -> sigevent` — [`sigevent`](#sigevent)

##### `impl Copy for sigevent`

##### `impl Debug for sigevent`

- <span id="sigevent-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Enums

### `DIR`

```rust
enum DIR {
}
```

#### Trait Implementations

##### `impl Clone for DIR`

- <span id="dir-clone"></span>`fn clone(&self) -> DIR` — [`DIR`](../index.md#dir)

##### `impl Copy for DIR`

##### `impl Debug for DIR`

- <span id="dir-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `FILE`

```rust
enum FILE {
}
```

#### Trait Implementations

##### `impl Clone for FILE`

- <span id="file-clone"></span>`fn clone(&self) -> FILE` — [`FILE`](../index.md#file)

##### `impl Copy for FILE`

##### `impl Debug for FILE`

- <span id="file-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `timezone`

```rust
enum timezone {
}
```

#### Trait Implementations

##### `impl Clone for timezone`

- <span id="timezone-clone"></span>`fn clone(&self) -> timezone` — [`timezone`](#timezone)

##### `impl Copy for timezone`

##### `impl Debug for timezone`

- <span id="timezone-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

### `isalnum`

```rust
unsafe fn isalnum(c: c_int) -> c_int
```

### `isalpha`

```rust
unsafe fn isalpha(c: c_int) -> c_int
```

### `iscntrl`

```rust
unsafe fn iscntrl(c: c_int) -> c_int
```

### `isdigit`

```rust
unsafe fn isdigit(c: c_int) -> c_int
```

### `isgraph`

```rust
unsafe fn isgraph(c: c_int) -> c_int
```

### `islower`

```rust
unsafe fn islower(c: c_int) -> c_int
```

### `isprint`

```rust
unsafe fn isprint(c: c_int) -> c_int
```

### `ispunct`

```rust
unsafe fn ispunct(c: c_int) -> c_int
```

### `isspace`

```rust
unsafe fn isspace(c: c_int) -> c_int
```

### `isupper`

```rust
unsafe fn isupper(c: c_int) -> c_int
```

### `isxdigit`

```rust
unsafe fn isxdigit(c: c_int) -> c_int
```

### `isblank`

```rust
unsafe fn isblank(c: c_int) -> c_int
```

### `tolower`

```rust
unsafe fn tolower(c: c_int) -> c_int
```

### `toupper`

```rust
unsafe fn toupper(c: c_int) -> c_int
```

### `qsort`

```rust
unsafe fn qsort(base: *mut c_void, num: size_t, size: size_t, compar: Option<fn(*const c_void, *const c_void) -> c_int>)
```

### `bsearch`

```rust
unsafe fn bsearch(key: *const c_void, base: *const c_void, num: size_t, size: size_t, compar: Option<fn(*const c_void, *const c_void) -> c_int>) -> *mut c_void
```

### `fopen`

```rust
unsafe fn fopen(filename: *const c_char, mode: *const c_char) -> *mut FILE
```

### `freopen`

```rust
unsafe fn freopen(filename: *const c_char, mode: *const c_char, file: *mut FILE) -> *mut FILE
```

### `fflush`

```rust
unsafe fn fflush(file: *mut FILE) -> c_int
```

### `fclose`

```rust
unsafe fn fclose(file: *mut FILE) -> c_int
```

### `remove`

```rust
unsafe fn remove(filename: *const c_char) -> c_int
```

### `rename`

```rust
unsafe fn rename(oldname: *const c_char, newname: *const c_char) -> c_int
```

### `tmpfile`

```rust
unsafe fn tmpfile() -> *mut FILE
```

### `setvbuf`

```rust
unsafe fn setvbuf(stream: *mut FILE, buffer: *mut c_char, mode: c_int, size: size_t) -> c_int
```

### `setbuf`

```rust
unsafe fn setbuf(stream: *mut FILE, buf: *mut c_char)
```

### `getchar`

```rust
unsafe fn getchar() -> c_int
```

### `putchar`

```rust
unsafe fn putchar(c: c_int) -> c_int
```

### `fgetc`

```rust
unsafe fn fgetc(stream: *mut FILE) -> c_int
```

### `fgets`

```rust
unsafe fn fgets(buf: *mut c_char, n: c_int, stream: *mut FILE) -> *mut c_char
```

### `fputc`

```rust
unsafe fn fputc(c: c_int, stream: *mut FILE) -> c_int
```

### `fputs`

```rust
unsafe fn fputs(s: *const c_char, stream: *mut FILE) -> c_int
```

### `puts`

```rust
unsafe fn puts(s: *const c_char) -> c_int
```

### `ungetc`

```rust
unsafe fn ungetc(c: c_int, stream: *mut FILE) -> c_int
```

### `fread`

```rust
unsafe fn fread(ptr: *mut c_void, size: size_t, nobj: size_t, stream: *mut FILE) -> size_t
```

### `fwrite`

```rust
unsafe fn fwrite(ptr: *const c_void, size: size_t, nobj: size_t, stream: *mut FILE) -> size_t
```

### `fseek`

```rust
unsafe fn fseek(stream: *mut FILE, offset: c_long, whence: c_int) -> c_int
```

### `ftell`

```rust
unsafe fn ftell(stream: *mut FILE) -> c_long
```

### `rewind`

```rust
unsafe fn rewind(stream: *mut FILE)
```

### `fgetpos`

```rust
unsafe fn fgetpos(stream: *mut FILE, ptr: *mut fpos_t) -> c_int
```

### `fsetpos`

```rust
unsafe fn fsetpos(stream: *mut FILE, ptr: *const fpos_t) -> c_int
```

### `feof`

```rust
unsafe fn feof(stream: *mut FILE) -> c_int
```

### `ferror`

```rust
unsafe fn ferror(stream: *mut FILE) -> c_int
```

### `clearerr`

```rust
unsafe fn clearerr(stream: *mut FILE)
```

### `perror`

```rust
unsafe fn perror(s: *const c_char)
```

### `atof`

```rust
unsafe fn atof(s: *const c_char) -> c_double
```

### `atoi`

```rust
unsafe fn atoi(s: *const c_char) -> c_int
```

### `atol`

```rust
unsafe fn atol(s: *const c_char) -> c_long
```

### `atoll`

```rust
unsafe fn atoll(s: *const c_char) -> c_longlong
```

### `strtod`

```rust
unsafe fn strtod(s: *const c_char, endp: *mut *mut c_char) -> c_double
```

### `strtof`

```rust
unsafe fn strtof(s: *const c_char, endp: *mut *mut c_char) -> c_float
```

### `strtol`

```rust
unsafe fn strtol(s: *const c_char, endp: *mut *mut c_char, base: c_int) -> c_long
```

### `strtoll`

```rust
unsafe fn strtoll(s: *const c_char, endp: *mut *mut c_char, base: c_int) -> c_longlong
```

### `strtoul`

```rust
unsafe fn strtoul(s: *const c_char, endp: *mut *mut c_char, base: c_int) -> c_ulong
```

### `strtoull`

```rust
unsafe fn strtoull(s: *const c_char, endp: *mut *mut c_char, base: c_int) -> c_ulonglong
```

### `calloc`

```rust
unsafe fn calloc(nobj: size_t, size: size_t) -> *mut c_void
```

### `malloc`

```rust
unsafe fn malloc(size: size_t) -> *mut c_void
```

### `realloc`

```rust
unsafe fn realloc(p: *mut c_void, size: size_t) -> *mut c_void
```

### `free`

```rust
unsafe fn free(p: *mut c_void)
```

### `abort`

```rust
unsafe fn abort() -> never
```

### `exit`

```rust
unsafe fn exit(status: c_int) -> never
```

### `_exit`

```rust
unsafe fn _exit(status: c_int) -> never
```

### `system`

```rust
unsafe fn system(s: *const c_char) -> c_int
```

### `getenv`

```rust
unsafe fn getenv(s: *const c_char) -> *mut c_char
```

### `strcpy`

```rust
unsafe fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char
```

### `strncpy`

```rust
unsafe fn strncpy(dst: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char
```

### `stpcpy`

```rust
unsafe fn stpcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char
```

### `strcat`

```rust
unsafe fn strcat(s: *mut c_char, ct: *const c_char) -> *mut c_char
```

### `strncat`

```rust
unsafe fn strncat(s: *mut c_char, ct: *const c_char, n: size_t) -> *mut c_char
```

### `strcmp`

```rust
unsafe fn strcmp(cs: *const c_char, ct: *const c_char) -> c_int
```

### `strncmp`

```rust
unsafe fn strncmp(cs: *const c_char, ct: *const c_char, n: size_t) -> c_int
```

### `strcoll`

```rust
unsafe fn strcoll(cs: *const c_char, ct: *const c_char) -> c_int
```

### `strchr`

```rust
unsafe fn strchr(cs: *const c_char, c: c_int) -> *mut c_char
```

### `strrchr`

```rust
unsafe fn strrchr(cs: *const c_char, c: c_int) -> *mut c_char
```

### `strspn`

```rust
unsafe fn strspn(cs: *const c_char, ct: *const c_char) -> size_t
```

### `strcspn`

```rust
unsafe fn strcspn(cs: *const c_char, ct: *const c_char) -> size_t
```

### `strdup`

```rust
unsafe fn strdup(cs: *const c_char) -> *mut c_char
```

### `strndup`

```rust
unsafe fn strndup(cs: *const c_char, n: size_t) -> *mut c_char
```

### `strpbrk`

```rust
unsafe fn strpbrk(cs: *const c_char, ct: *const c_char) -> *mut c_char
```

### `strstr`

```rust
unsafe fn strstr(cs: *const c_char, ct: *const c_char) -> *mut c_char
```

### `strcasecmp`

```rust
unsafe fn strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int
```

### `strncasecmp`

```rust
unsafe fn strncasecmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int
```

### `strlen`

```rust
unsafe fn strlen(cs: *const c_char) -> size_t
```

### `strnlen`

```rust
unsafe fn strnlen(cs: *const c_char, maxlen: size_t) -> size_t
```

### `strerror`

```rust
unsafe fn strerror(n: c_int) -> *mut c_char
```

### `strtok`

```rust
unsafe fn strtok(s: *mut c_char, t: *const c_char) -> *mut c_char
```

### `strtok_r`

```rust
unsafe fn strtok_r(s: *mut c_char, t: *const c_char, p: *mut *mut c_char) -> *mut c_char
```

### `strxfrm`

```rust
unsafe fn strxfrm(s: *mut c_char, ct: *const c_char, n: size_t) -> size_t
```

### `strsignal`

```rust
unsafe fn strsignal(sig: c_int) -> *mut c_char
```

### `wcslen`

```rust
unsafe fn wcslen(buf: *const wchar_t) -> size_t
```

### `wcstombs`

```rust
unsafe fn wcstombs(dest: *mut c_char, src: *const wchar_t, n: size_t) -> size_t
```

### `memchr`

```rust
unsafe fn memchr(cx: *const c_void, c: c_int, n: size_t) -> *mut c_void
```

### `wmemchr`

```rust
unsafe fn wmemchr(cx: *const wchar_t, c: wchar_t, n: size_t) -> *mut wchar_t
```

### `memcmp`

```rust
unsafe fn memcmp(cx: *const c_void, ct: *const c_void, n: size_t) -> c_int
```

### `memcpy`

```rust
unsafe fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void
```

### `memmove`

```rust
unsafe fn memmove(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void
```

### `memset`

```rust
unsafe fn memset(dest: *mut c_void, c: c_int, n: size_t) -> *mut c_void
```

### `memccpy`

```rust
unsafe fn memccpy(dest: *mut c_void, src: *const c_void, c: c_int, n: size_t) -> *mut c_void
```

### `getpwnam`

```rust
unsafe fn getpwnam(name: *const c_char) -> *mut passwd
```

### `getpwuid`

```rust
unsafe fn getpwuid(uid: crate::uid_t) -> *mut passwd
```

### `fprintf`

```rust
unsafe fn fprintf(stream: *mut crate::FILE, format: *const c_char) -> c_int
```

### `printf`

```rust
unsafe fn printf(format: *const c_char) -> c_int
```

### `snprintf`

```rust
unsafe fn snprintf(s: *mut c_char, n: size_t, format: *const c_char) -> c_int
```

### `sprintf`

```rust
unsafe fn sprintf(s: *mut c_char, format: *const c_char) -> c_int
```

### `fscanf`

```rust
unsafe fn fscanf(stream: *mut crate::FILE, format: *const c_char) -> c_int
```

### `scanf`

```rust
unsafe fn scanf(format: *const c_char) -> c_int
```

### `sscanf`

```rust
unsafe fn sscanf(s: *const c_char, format: *const c_char) -> c_int
```

### `getchar_unlocked`

```rust
unsafe fn getchar_unlocked() -> c_int
```

### `putchar_unlocked`

```rust
unsafe fn putchar_unlocked(c: c_int) -> c_int
```

### `socket`

```rust
unsafe fn socket(domain: c_int, ty: c_int, protocol: c_int) -> c_int
```

### `connect`

```rust
unsafe fn connect(socket: c_int, address: *const sockaddr, len: socklen_t) -> c_int
```

### `listen`

```rust
unsafe fn listen(socket: c_int, backlog: c_int) -> c_int
```

### `accept`

```rust
unsafe fn accept(socket: c_int, address: *mut sockaddr, address_len: *mut socklen_t) -> c_int
```

### `getpeername`

```rust
unsafe fn getpeername(socket: c_int, address: *mut sockaddr, address_len: *mut socklen_t) -> c_int
```

### `getsockname`

```rust
unsafe fn getsockname(socket: c_int, address: *mut sockaddr, address_len: *mut socklen_t) -> c_int
```

### `setsockopt`

```rust
unsafe fn setsockopt(socket: c_int, level: c_int, name: c_int, value: *const c_void, option_len: socklen_t) -> c_int
```

### `socketpair`

```rust
unsafe fn socketpair(domain: c_int, type_: c_int, protocol: c_int, socket_vector: *mut c_int) -> c_int
```

### `sendto`

```rust
unsafe fn sendto(socket: c_int, buf: *const c_void, len: size_t, flags: c_int, addr: *const sockaddr, addrlen: socklen_t) -> ssize_t
```

### `shutdown`

```rust
unsafe fn shutdown(socket: c_int, how: c_int) -> c_int
```

### `chmod`

```rust
unsafe fn chmod(path: *const c_char, mode: mode_t) -> c_int
```

### `fchmod`

```rust
unsafe fn fchmod(fd: c_int, mode: mode_t) -> c_int
```

### `fstat`

```rust
unsafe fn fstat(fildes: c_int, buf: *mut stat) -> c_int
```

### `mkdir`

```rust
unsafe fn mkdir(path: *const c_char, mode: mode_t) -> c_int
```

### `stat`

```rust
unsafe fn stat(path: *const c_char, buf: *mut stat) -> c_int
```

### `pclose`

```rust
unsafe fn pclose(stream: *mut crate::FILE) -> c_int
```

### `fdopen`

```rust
unsafe fn fdopen(fd: c_int, mode: *const c_char) -> *mut crate::FILE
```

### `fileno`

```rust
unsafe fn fileno(stream: *mut crate::FILE) -> c_int
```

### `open`

```rust
unsafe fn open(path: *const c_char, oflag: c_int) -> c_int
```

### `creat`

```rust
unsafe fn creat(path: *const c_char, mode: mode_t) -> c_int
```

### `fcntl`

```rust
unsafe fn fcntl(fd: c_int, cmd: c_int) -> c_int
```

### `opendir`

```rust
unsafe fn opendir(dirname: *const c_char) -> *mut crate::DIR
```

### `readdir`

```rust
unsafe fn readdir(dirp: *mut crate::DIR) -> *mut crate::dirent
```

### `closedir`

```rust
unsafe fn closedir(dirp: *mut crate::DIR) -> c_int
```

### `rewinddir`

```rust
unsafe fn rewinddir(dirp: *mut crate::DIR)
```

### `fchmodat`

```rust
unsafe fn fchmodat(dirfd: c_int, pathname: *const c_char, mode: mode_t, flags: c_int) -> c_int
```

### `fchown`

```rust
unsafe fn fchown(fd: c_int, owner: crate::uid_t, group: crate::gid_t) -> c_int
```

### `fchownat`

```rust
unsafe fn fchownat(dirfd: c_int, pathname: *const c_char, owner: crate::uid_t, group: crate::gid_t, flags: c_int) -> c_int
```

### `fstatat`

```rust
unsafe fn fstatat(dirfd: c_int, pathname: *const c_char, buf: *mut stat, flags: c_int) -> c_int
```

### `linkat`

```rust
unsafe fn linkat(olddirfd: c_int, oldpath: *const c_char, newdirfd: c_int, newpath: *const c_char, flags: c_int) -> c_int
```

### `renameat`

```rust
unsafe fn renameat(olddirfd: c_int, oldpath: *const c_char, newdirfd: c_int, newpath: *const c_char) -> c_int
```

### `symlinkat`

```rust
unsafe fn symlinkat(target: *const c_char, newdirfd: c_int, linkpath: *const c_char) -> c_int
```

### `unlinkat`

```rust
unsafe fn unlinkat(dirfd: c_int, pathname: *const c_char, flags: c_int) -> c_int
```

### `access`

```rust
unsafe fn access(path: *const c_char, amode: c_int) -> c_int
```

### `alarm`

```rust
unsafe fn alarm(seconds: c_uint) -> c_uint
```

### `chdir`

```rust
unsafe fn chdir(dir: *const c_char) -> c_int
```

### `fchdir`

```rust
unsafe fn fchdir(dirfd: c_int) -> c_int
```

### `chown`

```rust
unsafe fn chown(path: *const c_char, uid: uid_t, gid: gid_t) -> c_int
```

### `lchown`

```rust
unsafe fn lchown(path: *const c_char, uid: uid_t, gid: gid_t) -> c_int
```

### `close`

```rust
unsafe fn close(fd: c_int) -> c_int
```

### `dup`

```rust
unsafe fn dup(fd: c_int) -> c_int
```

### `dup2`

```rust
unsafe fn dup2(src: c_int, dst: c_int) -> c_int
```

### `execl`

```rust
unsafe fn execl(path: *const c_char, arg0: *const c_char) -> c_int
```

### `execle`

```rust
unsafe fn execle(path: *const c_char, arg0: *const c_char) -> c_int
```

### `execlp`

```rust
unsafe fn execlp(file: *const c_char, arg0: *const c_char) -> c_int
```

### `execv`

```rust
unsafe fn execv(prog: *const c_char, argv: *const *const c_char) -> c_int
```

### `execve`

```rust
unsafe fn execve(prog: *const c_char, argv: *const *const c_char, envp: *const *const c_char) -> c_int
```

### `execvp`

```rust
unsafe fn execvp(c: *const c_char, argv: *const *const c_char) -> c_int
```

### `fork`

```rust
unsafe fn fork() -> pid_t
```

### `fpathconf`

```rust
unsafe fn fpathconf(filedes: c_int, name: c_int) -> c_long
```

### `getcwd`

```rust
unsafe fn getcwd(buf: *mut c_char, size: size_t) -> *mut c_char
```

### `getegid`

```rust
unsafe fn getegid() -> gid_t
```

### `geteuid`

```rust
unsafe fn geteuid() -> uid_t
```

### `getgid`

```rust
unsafe fn getgid() -> gid_t
```

### `getgroups`

```rust
unsafe fn getgroups(ngroups_max: c_int, groups: *mut gid_t) -> c_int
```

### `getlogin`

```rust
unsafe fn getlogin() -> *mut c_char
```

### `getopt`

```rust
unsafe fn getopt(argc: c_int, argv: *const *mut c_char, optstr: *const c_char) -> c_int
```

### `getpgid`

```rust
unsafe fn getpgid(pid: pid_t) -> pid_t
```

### `getpgrp`

```rust
unsafe fn getpgrp() -> pid_t
```

### `getpid`

```rust
unsafe fn getpid() -> pid_t
```

### `getppid`

```rust
unsafe fn getppid() -> pid_t
```

### `getuid`

```rust
unsafe fn getuid() -> uid_t
```

### `isatty`

```rust
unsafe fn isatty(fd: c_int) -> c_int
```

### `link`

```rust
unsafe fn link(src: *const c_char, dst: *const c_char) -> c_int
```

### `lseek`

```rust
unsafe fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t
```

### `pathconf`

```rust
unsafe fn pathconf(path: *const c_char, name: c_int) -> c_long
```

### `pipe`

```rust
unsafe fn pipe(fds: *mut c_int) -> c_int
```

### `posix_memalign`

```rust
unsafe fn posix_memalign(memptr: *mut *mut c_void, align: size_t, size: size_t) -> c_int
```

### `aligned_alloc`

```rust
unsafe fn aligned_alloc(alignment: size_t, size: size_t) -> *mut c_void
```

### `read`

```rust
unsafe fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t
```

### `rmdir`

```rust
unsafe fn rmdir(path: *const c_char) -> c_int
```

### `seteuid`

```rust
unsafe fn seteuid(uid: uid_t) -> c_int
```

### `setegid`

```rust
unsafe fn setegid(gid: gid_t) -> c_int
```

### `setgid`

```rust
unsafe fn setgid(gid: gid_t) -> c_int
```

### `setpgid`

```rust
unsafe fn setpgid(pid: pid_t, pgid: pid_t) -> c_int
```

### `setsid`

```rust
unsafe fn setsid() -> pid_t
```

### `setuid`

```rust
unsafe fn setuid(uid: uid_t) -> c_int
```

### `setreuid`

```rust
unsafe fn setreuid(ruid: uid_t, euid: uid_t) -> c_int
```

### `setregid`

```rust
unsafe fn setregid(rgid: gid_t, egid: gid_t) -> c_int
```

### `sleep`

```rust
unsafe fn sleep(secs: c_uint) -> c_uint
```

### `nanosleep`

```rust
unsafe fn nanosleep(rqtp: *const timespec, rmtp: *mut timespec) -> c_int
```

### `tcgetpgrp`

```rust
unsafe fn tcgetpgrp(fd: c_int) -> pid_t
```

### `tcsetpgrp`

```rust
unsafe fn tcsetpgrp(fd: c_int, pgrp: crate::pid_t) -> c_int
```

### `ttyname`

```rust
unsafe fn ttyname(fd: c_int) -> *mut c_char
```

### `ttyname_r`

```rust
unsafe fn ttyname_r(fd: c_int, buf: *mut c_char, buflen: size_t) -> c_int
```

### `unlink`

```rust
unsafe fn unlink(c: *const c_char) -> c_int
```

### `wait`

```rust
unsafe fn wait(status: *mut c_int) -> pid_t
```

### `waitpid`

```rust
unsafe fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t
```

### `write`

```rust
unsafe fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t
```

### `pread`

```rust
unsafe fn pread(fd: c_int, buf: *mut c_void, count: size_t, offset: off_t) -> ssize_t
```

### `pwrite`

```rust
unsafe fn pwrite(fd: c_int, buf: *const c_void, count: size_t, offset: off_t) -> ssize_t
```

### `umask`

```rust
unsafe fn umask(mask: mode_t) -> mode_t
```

### `utime`

```rust
unsafe fn utime(file: *const c_char, buf: *const utimbuf) -> c_int
```

### `kill`

```rust
unsafe fn kill(pid: pid_t, sig: c_int) -> c_int
```

### `killpg`

```rust
unsafe fn killpg(pgrp: pid_t, sig: c_int) -> c_int
```

### `mlock`

```rust
unsafe fn mlock(addr: *const c_void, len: size_t) -> c_int
```

### `munlock`

```rust
unsafe fn munlock(addr: *const c_void, len: size_t) -> c_int
```

### `mlockall`

```rust
unsafe fn mlockall(flags: c_int) -> c_int
```

### `munlockall`

```rust
unsafe fn munlockall() -> c_int
```

### `mmap`

```rust
unsafe fn mmap(addr: *mut c_void, len: size_t, prot: c_int, flags: c_int, fd: c_int, offset: off_t) -> *mut c_void
```

### `munmap`

```rust
unsafe fn munmap(addr: *mut c_void, len: size_t) -> c_int
```

### `if_nametoindex`

```rust
unsafe fn if_nametoindex(ifname: *const c_char) -> c_uint
```

### `if_indextoname`

```rust
unsafe fn if_indextoname(ifindex: c_uint, ifname: *mut c_char) -> *mut c_char
```

### `lstat`

```rust
unsafe fn lstat(path: *const c_char, buf: *mut stat) -> c_int
```

### `fsync`

```rust
unsafe fn fsync(fd: c_int) -> c_int
```

### `setenv`

```rust
unsafe fn setenv(name: *const c_char, val: *const c_char, overwrite: c_int) -> c_int
```

### `unsetenv`

```rust
unsafe fn unsetenv(name: *const c_char) -> c_int
```

### `symlink`

```rust
unsafe fn symlink(path1: *const c_char, path2: *const c_char) -> c_int
```

### `truncate`

```rust
unsafe fn truncate(path: *const c_char, length: off_t) -> c_int
```

### `ftruncate`

```rust
unsafe fn ftruncate(fd: c_int, length: off_t) -> c_int
```

### `signal`

```rust
unsafe fn signal(signum: c_int, handler: sighandler_t) -> sighandler_t
```

### `getrusage`

```rust
unsafe fn getrusage(resource: c_int, usage: *mut rusage) -> c_int
```

### `realpath`

```rust
unsafe fn realpath(pathname: *const c_char, resolved: *mut c_char) -> *mut c_char
```

### `times`

```rust
unsafe fn times(buf: *mut crate::tms) -> crate::clock_t
```

### `pthread_self`

```rust
unsafe fn pthread_self() -> crate::pthread_t
```

### `pthread_equal`

```rust
unsafe fn pthread_equal(t1: crate::pthread_t, t2: crate::pthread_t) -> c_int
```

### `pthread_join`

```rust
unsafe fn pthread_join(native: crate::pthread_t, value: *mut *mut c_void) -> c_int
```

### `pthread_exit`

```rust
unsafe fn pthread_exit(value: *mut c_void) -> never
```

### `pthread_attr_init`

```rust
unsafe fn pthread_attr_init(attr: *mut crate::pthread_attr_t) -> c_int
```

### `pthread_attr_destroy`

```rust
unsafe fn pthread_attr_destroy(attr: *mut crate::pthread_attr_t) -> c_int
```

### `pthread_attr_getstacksize`

```rust
unsafe fn pthread_attr_getstacksize(attr: *const crate::pthread_attr_t, stacksize: *mut size_t) -> c_int
```

### `pthread_attr_setstacksize`

```rust
unsafe fn pthread_attr_setstacksize(attr: *mut crate::pthread_attr_t, stack_size: size_t) -> c_int
```

### `pthread_attr_setdetachstate`

```rust
unsafe fn pthread_attr_setdetachstate(attr: *mut crate::pthread_attr_t, state: c_int) -> c_int
```

### `pthread_detach`

```rust
unsafe fn pthread_detach(thread: crate::pthread_t) -> c_int
```

### `sched_yield`

```rust
unsafe fn sched_yield() -> c_int
```

### `pthread_key_create`

```rust
unsafe fn pthread_key_create(key: *mut crate::pthread_key_t, dtor: Option<fn(*mut c_void)>) -> c_int
```

### `pthread_key_delete`

```rust
unsafe fn pthread_key_delete(key: crate::pthread_key_t) -> c_int
```

### `pthread_getspecific`

```rust
unsafe fn pthread_getspecific(key: crate::pthread_key_t) -> *mut c_void
```

### `pthread_setspecific`

```rust
unsafe fn pthread_setspecific(key: crate::pthread_key_t, value: *const c_void) -> c_int
```

### `pthread_mutex_init`

```rust
unsafe fn pthread_mutex_init(lock: *mut crate::pthread_mutex_t, attr: *const crate::pthread_mutexattr_t) -> c_int
```

### `pthread_mutex_destroy`

```rust
unsafe fn pthread_mutex_destroy(lock: *mut crate::pthread_mutex_t) -> c_int
```

### `pthread_mutex_lock`

```rust
unsafe fn pthread_mutex_lock(lock: *mut crate::pthread_mutex_t) -> c_int
```

### `pthread_mutex_trylock`

```rust
unsafe fn pthread_mutex_trylock(lock: *mut crate::pthread_mutex_t) -> c_int
```

### `pthread_mutex_unlock`

```rust
unsafe fn pthread_mutex_unlock(lock: *mut crate::pthread_mutex_t) -> c_int
```

### `pthread_mutexattr_init`

```rust
unsafe fn pthread_mutexattr_init(attr: *mut crate::pthread_mutexattr_t) -> c_int
```

### `pthread_mutexattr_destroy`

```rust
unsafe fn pthread_mutexattr_destroy(attr: *mut crate::pthread_mutexattr_t) -> c_int
```

### `pthread_mutexattr_settype`

```rust
unsafe fn pthread_mutexattr_settype(attr: *mut crate::pthread_mutexattr_t, _type: c_int) -> c_int
```

### `pthread_cond_init`

```rust
unsafe fn pthread_cond_init(cond: *mut crate::pthread_cond_t, attr: *const crate::pthread_condattr_t) -> c_int
```

### `pthread_cond_wait`

```rust
unsafe fn pthread_cond_wait(cond: *mut crate::pthread_cond_t, lock: *mut crate::pthread_mutex_t) -> c_int
```

### `pthread_cond_timedwait`

```rust
unsafe fn pthread_cond_timedwait(cond: *mut crate::pthread_cond_t, lock: *mut crate::pthread_mutex_t, abstime: *const crate::timespec) -> c_int
```

### `pthread_cond_signal`

```rust
unsafe fn pthread_cond_signal(cond: *mut crate::pthread_cond_t) -> c_int
```

### `pthread_cond_broadcast`

```rust
unsafe fn pthread_cond_broadcast(cond: *mut crate::pthread_cond_t) -> c_int
```

### `pthread_cond_destroy`

```rust
unsafe fn pthread_cond_destroy(cond: *mut crate::pthread_cond_t) -> c_int
```

### `pthread_condattr_init`

```rust
unsafe fn pthread_condattr_init(attr: *mut crate::pthread_condattr_t) -> c_int
```

### `pthread_condattr_destroy`

```rust
unsafe fn pthread_condattr_destroy(attr: *mut crate::pthread_condattr_t) -> c_int
```

### `pthread_rwlock_init`

```rust
unsafe fn pthread_rwlock_init(lock: *mut crate::pthread_rwlock_t, attr: *const crate::pthread_rwlockattr_t) -> c_int
```

### `pthread_rwlock_destroy`

```rust
unsafe fn pthread_rwlock_destroy(lock: *mut crate::pthread_rwlock_t) -> c_int
```

### `pthread_rwlock_rdlock`

```rust
unsafe fn pthread_rwlock_rdlock(lock: *mut crate::pthread_rwlock_t) -> c_int
```

### `pthread_rwlock_tryrdlock`

```rust
unsafe fn pthread_rwlock_tryrdlock(lock: *mut crate::pthread_rwlock_t) -> c_int
```

### `pthread_rwlock_wrlock`

```rust
unsafe fn pthread_rwlock_wrlock(lock: *mut crate::pthread_rwlock_t) -> c_int
```

### `pthread_rwlock_trywrlock`

```rust
unsafe fn pthread_rwlock_trywrlock(lock: *mut crate::pthread_rwlock_t) -> c_int
```

### `pthread_rwlock_unlock`

```rust
unsafe fn pthread_rwlock_unlock(lock: *mut crate::pthread_rwlock_t) -> c_int
```

### `pthread_rwlockattr_init`

```rust
unsafe fn pthread_rwlockattr_init(attr: *mut crate::pthread_rwlockattr_t) -> c_int
```

### `pthread_rwlockattr_destroy`

```rust
unsafe fn pthread_rwlockattr_destroy(attr: *mut crate::pthread_rwlockattr_t) -> c_int
```

### `getsockopt`

```rust
unsafe fn getsockopt(sockfd: c_int, level: c_int, optname: c_int, optval: *mut c_void, optlen: *mut crate::socklen_t) -> c_int
```

### `raise`

```rust
unsafe fn raise(signum: c_int) -> c_int
```

### `utimes`

```rust
unsafe fn utimes(filename: *const c_char, times: *const crate::timeval) -> c_int
```

### `dlopen`

```rust
unsafe fn dlopen(filename: *const c_char, flag: c_int) -> *mut c_void
```

### `dlerror`

```rust
unsafe fn dlerror() -> *mut c_char
```

### `dlsym`

```rust
unsafe fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void
```

### `dlclose`

```rust
unsafe fn dlclose(handle: *mut c_void) -> c_int
```

### `getaddrinfo`

```rust
unsafe fn getaddrinfo(node: *const c_char, service: *const c_char, hints: *const addrinfo, res: *mut *mut addrinfo) -> c_int
```

### `freeaddrinfo`

```rust
unsafe fn freeaddrinfo(res: *mut addrinfo)
```

### `hstrerror`

```rust
unsafe fn hstrerror(errcode: c_int) -> *const c_char
```

### `gai_strerror`

```rust
unsafe fn gai_strerror(errcode: c_int) -> *const c_char
```

### `res_init`

```rust
unsafe fn res_init() -> c_int
```

### `gmtime_r`

```rust
unsafe fn gmtime_r(time_p: *const time_t, result: *mut tm) -> *mut tm
```

### `localtime_r`

```rust
unsafe fn localtime_r(time_p: *const time_t, result: *mut tm) -> *mut tm
```

### `mktime`

```rust
unsafe fn mktime(tm: *mut tm) -> time_t
```

### `time`

```rust
unsafe fn time(time: *mut time_t) -> time_t
```

### `gmtime`

```rust
unsafe fn gmtime(time_p: *const time_t) -> *mut tm
```

### `localtime`

```rust
unsafe fn localtime(time_p: *const time_t) -> *mut tm
```

### `difftime`

```rust
unsafe fn difftime(time1: time_t, time0: time_t) -> c_double
```

### `timegm`

```rust
unsafe fn timegm(tm: *mut crate::tm) -> time_t
```

### `mknod`

```rust
unsafe fn mknod(pathname: *const c_char, mode: mode_t, dev: crate::dev_t) -> c_int
```

### `gethostname`

```rust
unsafe fn gethostname(name: *mut c_char, len: size_t) -> c_int
```

### `endservent`

```rust
unsafe fn endservent()
```

### `getservbyname`

```rust
unsafe fn getservbyname(name: *const c_char, proto: *const c_char) -> *mut servent
```

### `getservbyport`

```rust
unsafe fn getservbyport(port: c_int, proto: *const c_char) -> *mut servent
```

### `getservent`

```rust
unsafe fn getservent() -> *mut servent
```

### `setservent`

```rust
unsafe fn setservent(stayopen: c_int)
```

### `getprotobyname`

```rust
unsafe fn getprotobyname(name: *const c_char) -> *mut protoent
```

### `getprotobynumber`

```rust
unsafe fn getprotobynumber(proto: c_int) -> *mut protoent
```

### `chroot`

```rust
unsafe fn chroot(name: *const c_char) -> c_int
```

### `usleep`

```rust
unsafe fn usleep(secs: c_uint) -> c_int
```

### `send`

```rust
unsafe fn send(socket: c_int, buf: *const c_void, len: size_t, flags: c_int) -> ssize_t
```

### `recv`

```rust
unsafe fn recv(socket: c_int, buf: *mut c_void, len: size_t, flags: c_int) -> ssize_t
```

### `putenv`

```rust
unsafe fn putenv(string: *mut c_char) -> c_int
```

### `poll`

```rust
unsafe fn poll(fds: *mut pollfd, nfds: nfds_t, timeout: c_int) -> c_int
```

### `select`

```rust
unsafe fn select(nfds: c_int, readfds: *mut fd_set, writefds: *mut fd_set, errorfds: *mut fd_set, timeout: *mut timeval) -> c_int
```

### `setlocale`

```rust
unsafe fn setlocale(category: c_int, locale: *const c_char) -> *mut c_char
```

### `localeconv`

```rust
unsafe fn localeconv() -> *mut lconv
```

### `sem_wait`

```rust
unsafe fn sem_wait(sem: *mut sem_t) -> c_int
```

### `sem_trywait`

```rust
unsafe fn sem_trywait(sem: *mut sem_t) -> c_int
```

### `sem_post`

```rust
unsafe fn sem_post(sem: *mut sem_t) -> c_int
```

### `statvfs`

```rust
unsafe fn statvfs(path: *const c_char, buf: *mut crate::statvfs) -> c_int
```

### `fstatvfs`

```rust
unsafe fn fstatvfs(fd: c_int, buf: *mut crate::statvfs) -> c_int
```

### `sigemptyset`

```rust
unsafe fn sigemptyset(set: *mut sigset_t) -> c_int
```

### `sigaddset`

```rust
unsafe fn sigaddset(set: *mut sigset_t, signum: c_int) -> c_int
```

### `sigfillset`

```rust
unsafe fn sigfillset(set: *mut sigset_t) -> c_int
```

### `sigdelset`

```rust
unsafe fn sigdelset(set: *mut sigset_t, signum: c_int) -> c_int
```

### `sigismember`

```rust
unsafe fn sigismember(set: *const sigset_t, signum: c_int) -> c_int
```

### `sigprocmask`

```rust
unsafe fn sigprocmask(how: c_int, set: *const sigset_t, oldset: *mut sigset_t) -> c_int
```

### `sigpending`

```rust
unsafe fn sigpending(set: *mut sigset_t) -> c_int
```

### `sysconf`

```rust
unsafe fn sysconf(name: c_int) -> c_long
```

### `mkfifo`

```rust
unsafe fn mkfifo(path: *const c_char, mode: mode_t) -> c_int
```

### `fseeko`

```rust
unsafe fn fseeko(stream: *mut crate::FILE, offset: off_t, whence: c_int) -> c_int
```

### `ftello`

```rust
unsafe fn ftello(stream: *mut crate::FILE) -> off_t
```

### `tcdrain`

```rust
unsafe fn tcdrain(fd: c_int) -> c_int
```

### `cfgetispeed`

```rust
unsafe fn cfgetispeed(termios: *const crate::termios) -> crate::speed_t
```

### `cfgetospeed`

```rust
unsafe fn cfgetospeed(termios: *const crate::termios) -> crate::speed_t
```

### `cfsetispeed`

```rust
unsafe fn cfsetispeed(termios: *mut crate::termios, speed: crate::speed_t) -> c_int
```

### `cfsetospeed`

```rust
unsafe fn cfsetospeed(termios: *mut crate::termios, speed: crate::speed_t) -> c_int
```

### `tcgetattr`

```rust
unsafe fn tcgetattr(fd: c_int, termios: *mut crate::termios) -> c_int
```

### `tcsetattr`

```rust
unsafe fn tcsetattr(fd: c_int, optional_actions: c_int, termios: *const crate::termios) -> c_int
```

### `tcflow`

```rust
unsafe fn tcflow(fd: c_int, action: c_int) -> c_int
```

### `tcflush`

```rust
unsafe fn tcflush(fd: c_int, action: c_int) -> c_int
```

### `tcgetsid`

```rust
unsafe fn tcgetsid(fd: c_int) -> crate::pid_t
```

### `tcsendbreak`

```rust
unsafe fn tcsendbreak(fd: c_int, duration: c_int) -> c_int
```

### `mkstemp`

```rust
unsafe fn mkstemp(template: *mut c_char) -> c_int
```

### `mkdtemp`

```rust
unsafe fn mkdtemp(template: *mut c_char) -> *mut c_char
```

### `tmpnam`

```rust
unsafe fn tmpnam(ptr: *mut c_char) -> *mut c_char
```

### `openlog`

```rust
unsafe fn openlog(ident: *const c_char, logopt: c_int, facility: c_int)
```

### `closelog`

```rust
unsafe fn closelog()
```

### `setlogmask`

```rust
unsafe fn setlogmask(maskpri: c_int) -> c_int
```

### `syslog`

```rust
unsafe fn syslog(priority: c_int, message: *const c_char)
```

### `nice`

```rust
unsafe fn nice(incr: c_int) -> c_int
```

### `grantpt`

```rust
unsafe fn grantpt(fd: c_int) -> c_int
```

### `posix_openpt`

```rust
unsafe fn posix_openpt(flags: c_int) -> c_int
```

### `ptsname`

```rust
unsafe fn ptsname(fd: c_int) -> *mut c_char
```

### `unlockpt`

```rust
unsafe fn unlockpt(fd: c_int) -> c_int
```

### `strcasestr`

```rust
unsafe fn strcasestr(cs: *const c_char, ct: *const c_char) -> *mut c_char
```

### `getline`

```rust
unsafe fn getline(lineptr: *mut *mut c_char, n: *mut size_t, stream: *mut FILE) -> ssize_t
```

### `lockf`

```rust
unsafe fn lockf(fd: c_int, cmd: c_int, len: off_t) -> c_int
```

### `adjtime`

```rust
unsafe fn adjtime(delta: *const timeval, olddelta: *mut timeval) -> c_int
```

### `stpncpy`

```rust
unsafe fn stpncpy(dst: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char
```

### `sigqueue`

```rust
unsafe fn sigqueue(pid: pid_t, sig: c_int, value: crate::sigval) -> c_int
```

### `confstr`

```rust
unsafe fn confstr(name: c_int, buf: *mut c_char, len: size_t) -> size_t
```

### `dladdr`

```rust
unsafe fn dladdr(addr: *const c_void, info: *mut Dl_info) -> c_int
```

### `flock`

```rust
unsafe fn flock(fd: c_int, operation: c_int) -> c_int
```

### `open_wmemstream`

```rust
unsafe fn open_wmemstream(ptr: *mut *mut wchar_t, sizeloc: *mut size_t) -> *mut FILE
```

### `getsid`

```rust
unsafe fn getsid(pid: pid_t) -> pid_t
```

### `pause`

```rust
unsafe fn pause() -> c_int
```

### `mkdirat`

```rust
unsafe fn mkdirat(dirfd: c_int, pathname: *const c_char, mode: mode_t) -> c_int
```

### `openat`

```rust
unsafe fn openat(dirfd: c_int, pathname: *const c_char, flags: c_int) -> c_int
```

### `fdopendir`

```rust
unsafe fn fdopendir(fd: c_int) -> *mut crate::DIR
```

### `readdir_r`

```rust
unsafe fn readdir_r(dirp: *mut crate::DIR, entry: *mut crate::dirent, result: *mut *mut crate::dirent) -> c_int
```

The 64-bit libc on Solaris and illumos only has readdir_r. If a
32-bit Solaris or illumos target is ever created, it should use
__posix_readdir_r. See libc(3LIB) on Solaris or illumos:
https://illumos.org/man/3lib/libc
https://docs.oracle.com/cd/E36784_01/html/E36873/libc-3lib.html
https://www.unix.com/man-page/opensolaris/3LIB/libc/

### `readlinkat`

```rust
unsafe fn readlinkat(dirfd: c_int, pathname: *const c_char, buf: *mut c_char, bufsiz: size_t) -> ssize_t
```

### `fmemopen`

```rust
unsafe fn fmemopen(buf: *mut c_void, size: size_t, mode: *const c_char) -> *mut FILE
```

### `open_memstream`

```rust
unsafe fn open_memstream(ptr: *mut *mut c_char, sizeloc: *mut size_t) -> *mut FILE
```

### `atexit`

```rust
unsafe fn atexit(cb: fn()) -> c_int
```

### `sigaction`

```rust
unsafe fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int
```

### `readlink`

```rust
unsafe fn readlink(path: *const c_char, buf: *mut c_char, bufsz: size_t) -> ssize_t
```

### `pselect`

```rust
unsafe fn pselect(nfds: c_int, readfds: *mut fd_set, writefds: *mut fd_set, errorfds: *mut fd_set, timeout: *const timespec, sigmask: *const sigset_t) -> c_int
```

### `cfmakeraw`

```rust
unsafe fn cfmakeraw(termios: *mut crate::termios)
```

### `cfsetspeed`

```rust
unsafe fn cfsetspeed(termios: *mut crate::termios, speed: crate::speed_t) -> c_int
```

### `fnmatch`

```rust
unsafe fn fnmatch(pattern: *const c_char, name: *const c_char, flags: c_int) -> c_int
```

### `htonl`

```rust
const fn htonl(hostlong: u32) -> u32
```

### `htons`

```rust
const fn htons(hostshort: u16) -> u16
```

### `ntohl`

```rust
const fn ntohl(netlong: u32) -> u32
```

### `ntohs`

```rust
const fn ntohs(netshort: u16) -> u16
```

### `ioctl`

```rust
unsafe fn ioctl(fd: c_int, request: c_ulong) -> c_int
```

### `sem_destroy`

```rust
unsafe fn sem_destroy(sem: *mut sem_t) -> c_int
```

### `sem_init`

```rust
unsafe fn sem_init(sem: *mut sem_t, pshared: c_int, value: c_uint) -> c_int
```

### `fdatasync`

```rust
unsafe fn fdatasync(fd: c_int) -> c_int
```

### `mincore`

```rust
unsafe fn mincore(addr: *mut c_void, len: size_t, vec: *mut c_uchar) -> c_int
```

### `clock_getres`

```rust
unsafe fn clock_getres(clk_id: crate::clockid_t, tp: *mut crate::timespec) -> c_int
```

### `clock_gettime`

```rust
unsafe fn clock_gettime(clk_id: crate::clockid_t, tp: *mut crate::timespec) -> c_int
```

### `clock_settime`

```rust
unsafe fn clock_settime(clk_id: crate::clockid_t, tp: *const crate::timespec) -> c_int
```

### `clock_getcpuclockid`

```rust
unsafe fn clock_getcpuclockid(pid: crate::pid_t, clk_id: *mut crate::clockid_t) -> c_int
```

### `getitimer`

```rust
unsafe fn getitimer(which: c_int, curr_value: *mut crate::itimerval) -> c_int
```

### `setitimer`

```rust
unsafe fn setitimer(which: c_int, new_value: *const crate::itimerval, old_value: *mut crate::itimerval) -> c_int
```

### `dirfd`

```rust
unsafe fn dirfd(dirp: *mut crate::DIR) -> c_int
```

### `memalign`

```rust
unsafe fn memalign(align: size_t, size: size_t) -> *mut c_void
```

### `setgroups`

```rust
unsafe fn setgroups(ngroups: size_t, ptr: *const crate::gid_t) -> c_int
```

### `pipe2`

```rust
unsafe fn pipe2(fds: *mut c_int, flags: c_int) -> c_int
```

### `statfs`

```rust
unsafe fn statfs(path: *const c_char, buf: *mut statfs) -> c_int
```

### `fstatfs`

```rust
unsafe fn fstatfs(fd: c_int, buf: *mut statfs) -> c_int
```

### `memrchr`

```rust
unsafe fn memrchr(cx: *const c_void, c: c_int, n: size_t) -> *mut c_void
```

### `posix_fadvise`

```rust
unsafe fn posix_fadvise(fd: c_int, offset: off_t, len: off_t, advise: c_int) -> c_int
```

### `futimens`

```rust
unsafe fn futimens(fd: c_int, times: *const crate::timespec) -> c_int
```

### `utimensat`

```rust
unsafe fn utimensat(dirfd: c_int, path: *const c_char, times: *const crate::timespec, flag: c_int) -> c_int
```

### `duplocale`

```rust
unsafe fn duplocale(base: crate::locale_t) -> crate::locale_t
```

### `freelocale`

```rust
unsafe fn freelocale(loc: crate::locale_t)
```

### `newlocale`

```rust
unsafe fn newlocale(mask: c_int, locale: *const c_char, base: crate::locale_t) -> crate::locale_t
```

### `uselocale`

```rust
unsafe fn uselocale(loc: crate::locale_t) -> crate::locale_t
```

### `mknodat`

```rust
unsafe fn mknodat(dirfd: c_int, pathname: *const c_char, mode: mode_t, dev: dev_t) -> c_int
```

### `ptsname_r`

```rust
unsafe fn ptsname_r(fd: c_int, buf: *mut c_char, buflen: size_t) -> c_int
```

### `clearenv`

```rust
unsafe fn clearenv() -> c_int
```

### `waitid`

```rust
unsafe fn waitid(idtype: idtype_t, id: id_t, infop: *mut crate::siginfo_t, options: c_int) -> c_int
```

### `getresuid`

```rust
unsafe fn getresuid(ruid: *mut crate::uid_t, euid: *mut crate::uid_t, suid: *mut crate::uid_t) -> c_int
```

### `getresgid`

```rust
unsafe fn getresgid(rgid: *mut crate::gid_t, egid: *mut crate::gid_t, sgid: *mut crate::gid_t) -> c_int
```

### `acct`

```rust
unsafe fn acct(filename: *const c_char) -> c_int
```

### `brk`

```rust
unsafe fn brk(addr: *mut c_void) -> c_int
```

### `sbrk`

```rust
unsafe fn sbrk(increment: intptr_t) -> *mut c_void
```

### `vfork`

```rust
unsafe fn vfork() -> crate::pid_t
```

### `setresgid`

```rust
unsafe fn setresgid(rgid: crate::gid_t, egid: crate::gid_t, sgid: crate::gid_t) -> c_int
```

### `setresuid`

```rust
unsafe fn setresuid(ruid: crate::uid_t, euid: crate::uid_t, suid: crate::uid_t) -> c_int
```

### `wait4`

```rust
unsafe fn wait4(pid: crate::pid_t, status: *mut c_int, options: c_int, rusage: *mut crate::rusage) -> crate::pid_t
```

### `login_tty`

```rust
unsafe fn login_tty(fd: c_int) -> c_int
```

### `execvpe`

```rust
unsafe fn execvpe(file: *const c_char, argv: *const *const c_char, envp: *const *const c_char) -> c_int
```

### `fexecve`

```rust
unsafe fn fexecve(fd: c_int, argv: *const *const c_char, envp: *const *const c_char) -> c_int
```

### `getifaddrs`

```rust
unsafe fn getifaddrs(ifap: *mut *mut crate::ifaddrs) -> c_int
```

### `freeifaddrs`

```rust
unsafe fn freeifaddrs(ifa: *mut crate::ifaddrs)
```

### `bind`

```rust
unsafe fn bind(socket: c_int, address: *const crate::sockaddr, address_len: crate::socklen_t) -> c_int
```

### `writev`

```rust
unsafe fn writev(fd: c_int, iov: *const crate::iovec, iovcnt: c_int) -> ssize_t
```

### `readv`

```rust
unsafe fn readv(fd: c_int, iov: *const crate::iovec, iovcnt: c_int) -> ssize_t
```

### `sendmsg`

```rust
unsafe fn sendmsg(fd: c_int, msg: *const crate::msghdr, flags: c_int) -> ssize_t
```

### `recvmsg`

```rust
unsafe fn recvmsg(fd: c_int, msg: *mut crate::msghdr, flags: c_int) -> ssize_t
```

### `uname`

```rust
unsafe fn uname(buf: *mut crate::utsname) -> c_int
```

### `strchrnul`

```rust
unsafe fn strchrnul(s: *const c_char, c: c_int) -> *mut c_char
```

### `strftime`

```rust
unsafe fn strftime(s: *mut c_char, max: size_t, format: *const c_char, tm: *const crate::tm) -> size_t
```

### `strftime_l`

```rust
unsafe fn strftime_l(s: *mut c_char, max: size_t, format: *const c_char, tm: *const crate::tm, locale: crate::locale_t) -> size_t
```

### `strptime`

```rust
unsafe fn strptime(s: *const c_char, format: *const c_char, tm: *mut crate::tm) -> *mut c_char
```

### `mkostemp`

```rust
unsafe fn mkostemp(template: *mut c_char, flags: c_int) -> c_int
```

### `mkostemps`

```rust
unsafe fn mkostemps(template: *mut c_char, suffixlen: c_int, flags: c_int) -> c_int
```

### `getdomainname`

```rust
unsafe fn getdomainname(name: *mut c_char, len: size_t) -> c_int
```

### `setdomainname`

```rust
unsafe fn setdomainname(name: *const c_char, len: size_t) -> c_int
```

### `if_nameindex`

```rust
unsafe fn if_nameindex() -> *mut if_nameindex
```

### `if_freenameindex`

```rust
unsafe fn if_freenameindex(ptr: *mut if_nameindex)
```

### `getpwnam_r`

```rust
unsafe fn getpwnam_r(name: *const c_char, pwd: *mut passwd, buf: *mut c_char, buflen: size_t, result: *mut *mut passwd) -> c_int
```

### `getpwuid_r`

```rust
unsafe fn getpwuid_r(uid: crate::uid_t, pwd: *mut passwd, buf: *mut c_char, buflen: size_t, result: *mut *mut passwd) -> c_int
```

### `fstatfs64`

```rust
unsafe fn fstatfs64(fd: c_int, buf: *mut statfs64) -> c_int
```

### `statvfs64`

```rust
unsafe fn statvfs64(path: *const c_char, buf: *mut statvfs64) -> c_int
```

### `fstatvfs64`

```rust
unsafe fn fstatvfs64(fd: c_int, buf: *mut statvfs64) -> c_int
```

### `statfs64`

```rust
unsafe fn statfs64(path: *const c_char, buf: *mut statfs64) -> c_int
```

### `creat64`

```rust
unsafe fn creat64(path: *const c_char, mode: mode_t) -> c_int
```

### `fstat64`

```rust
unsafe fn fstat64(fildes: c_int, buf: *mut stat64) -> c_int
```

### `fstatat64`

```rust
unsafe fn fstatat64(dirfd: c_int, pathname: *const c_char, buf: *mut stat64, flags: c_int) -> c_int
```

### `ftruncate64`

```rust
unsafe fn ftruncate64(fd: c_int, length: off64_t) -> c_int
```

### `lseek64`

```rust
unsafe fn lseek64(fd: c_int, offset: off64_t, whence: c_int) -> off64_t
```

### `lstat64`

```rust
unsafe fn lstat64(path: *const c_char, buf: *mut stat64) -> c_int
```

### `mmap64`

```rust
unsafe fn mmap64(addr: *mut c_void, len: size_t, prot: c_int, flags: c_int, fd: c_int, offset: off64_t) -> *mut c_void
```

### `open64`

```rust
unsafe fn open64(path: *const c_char, oflag: c_int) -> c_int
```

### `openat64`

```rust
unsafe fn openat64(fd: c_int, path: *const c_char, oflag: c_int) -> c_int
```

### `posix_fadvise64`

```rust
unsafe fn posix_fadvise64(fd: c_int, offset: off64_t, len: off64_t, advise: c_int) -> c_int
```

### `pread64`

```rust
unsafe fn pread64(fd: c_int, buf: *mut c_void, count: size_t, offset: off64_t) -> ssize_t
```

### `pwrite64`

```rust
unsafe fn pwrite64(fd: c_int, buf: *const c_void, count: size_t, offset: off64_t) -> ssize_t
```

### `readdir64`

```rust
unsafe fn readdir64(dirp: *mut crate::DIR) -> *mut crate::dirent64
```

### `readdir64_r`

```rust
unsafe fn readdir64_r(dirp: *mut crate::DIR, entry: *mut crate::dirent64, result: *mut *mut crate::dirent64) -> c_int
```

### `stat64`

```rust
unsafe fn stat64(path: *const c_char, buf: *mut stat64) -> c_int
```

### `truncate64`

```rust
unsafe fn truncate64(path: *const c_char, length: off64_t) -> c_int
```

### `preadv64`

```rust
unsafe fn preadv64(fd: c_int, iov: *const crate::iovec, iovcnt: c_int, offset: off64_t) -> ssize_t
```

### `pwritev64`

```rust
unsafe fn pwritev64(fd: c_int, iov: *const crate::iovec, iovcnt: c_int, offset: off64_t) -> ssize_t
```

### `forkpty`

```rust
unsafe fn forkpty(amaster: *mut c_int, name: *mut c_char, termp: *const termios, winp: *const crate::winsize) -> crate::pid_t
```

### `openpty`

```rust
unsafe fn openpty(amaster: *mut c_int, aslave: *mut c_int, name: *mut c_char, termp: *const termios, winp: *const crate::winsize) -> c_int
```

### `statx`

```rust
unsafe fn statx(dirfd: c_int, pathname: *const c_char, flags: c_int, mask: c_uint, statxbuf: *mut statx) -> c_int
```

### `_IOC`

```rust
const fn _IOC(dir: u32, ty: u32, nr: u32, size: usize) -> c_ulong
```

Build an ioctl number, analogous to the C macro of the same name.

### `_IO`

```rust
const fn _IO(ty: u32, nr: u32) -> c_ulong
```

Build an ioctl number for an argumentless ioctl.

### `_IOR`

```rust
const fn _IOR<T>(ty: u32, nr: u32) -> c_ulong
```

Build an ioctl number for an read-only ioctl.

### `_IOW`

```rust
const fn _IOW<T>(ty: u32, nr: u32) -> c_ulong
```

Build an ioctl number for an write-only ioctl.

### `_IOWR`

```rust
const fn _IOWR<T>(ty: u32, nr: u32) -> c_ulong
```

Build an ioctl number for a read-write ioctl.

### `CMSG_ALIGN`

```rust
const fn CMSG_ALIGN(len: usize) -> usize
```

### `CMSG_FIRSTHDR`

```rust
unsafe fn CMSG_FIRSTHDR(mhdr: *const crate::msghdr) -> *mut crate::cmsghdr
```

### `CMSG_DATA`

```rust
unsafe fn CMSG_DATA(cmsg: *const crate::cmsghdr) -> *mut c_uchar
```

### `CMSG_SPACE`

```rust
const unsafe fn CMSG_SPACE(length: c_uint) -> c_uint
```

### `CMSG_LEN`

```rust
const unsafe fn CMSG_LEN(length: c_uint) -> c_uint
```

### `FD_CLR`

```rust
unsafe fn FD_CLR(fd: c_int, set: *mut fd_set)
```

### `FD_ISSET`

```rust
unsafe fn FD_ISSET(fd: c_int, set: *const fd_set) -> bool
```

### `FD_SET`

```rust
unsafe fn FD_SET(fd: c_int, set: *mut fd_set)
```

### `FD_ZERO`

```rust
unsafe fn FD_ZERO(set: *mut fd_set)
```

### `SIGRTMAX`

```rust
fn SIGRTMAX() -> c_int
```

### `SIGRTMIN`

```rust
fn SIGRTMIN() -> c_int
```

### `WIFSTOPPED`

```rust
const fn WIFSTOPPED(status: c_int) -> bool
```

### `WSTOPSIG`

```rust
const fn WSTOPSIG(status: c_int) -> c_int
```

### `WIFCONTINUED`

```rust
const fn WIFCONTINUED(status: c_int) -> bool
```

### `WIFSIGNALED`

```rust
const fn WIFSIGNALED(status: c_int) -> bool
```

### `WTERMSIG`

```rust
const fn WTERMSIG(status: c_int) -> c_int
```

### `WIFEXITED`

```rust
const fn WIFEXITED(status: c_int) -> bool
```

### `WEXITSTATUS`

```rust
const fn WEXITSTATUS(status: c_int) -> c_int
```

### `WCOREDUMP`

```rust
const fn WCOREDUMP(status: c_int) -> bool
```

### `W_EXITCODE`

```rust
const fn W_EXITCODE(ret: c_int, sig: c_int) -> c_int
```

### `W_STOPCODE`

```rust
const fn W_STOPCODE(sig: c_int) -> c_int
```

### `QCMD`

```rust
const fn QCMD(cmd: c_int, type_: c_int) -> c_int
```

### `IPOPT_COPIED`

```rust
const fn IPOPT_COPIED(o: u8) -> u8
```

### `IPOPT_CLASS`

```rust
const fn IPOPT_CLASS(o: u8) -> u8
```

### `IPOPT_NUMBER`

```rust
const fn IPOPT_NUMBER(o: u8) -> u8
```

### `IPTOS_ECN`

```rust
const fn IPTOS_ECN(x: u8) -> u8
```

### `KERNEL_VERSION`

```rust
const fn KERNEL_VERSION(a: u32, b: u32, c: u32) -> u32
```

## Type Aliases

### `intmax_t`

```rust
type intmax_t = i64;
```

### `uintmax_t`

```rust
type uintmax_t = u64;
```

### `size_t`

```rust
type size_t = usize;
```

### `ptrdiff_t`

```rust
type ptrdiff_t = isize;
```

### `intptr_t`

```rust
type intptr_t = isize;
```

### `uintptr_t`

```rust
type uintptr_t = usize;
```

### `ssize_t`

```rust
type ssize_t = isize;
```

### `pid_t`

```rust
type pid_t = i32;
```

### `in_addr_t`

```rust
type in_addr_t = u32;
```

### `in_port_t`

```rust
type in_port_t = u16;
```

### `sighandler_t`

```rust
type sighandler_t = size_t;
```

### `cc_t`

```rust
type cc_t = c_uchar;
```

### `uid_t`

```rust
type uid_t = u32;
```

### `gid_t`

```rust
type gid_t = u32;
```

### `locale_t`

```rust
type locale_t = *mut c_void;
```

### `sa_family_t`

```rust
type sa_family_t = u16;
```

### `speed_t`

```rust
type speed_t = c_uint;
```

### `tcflag_t`

```rust
type tcflag_t = c_uint;
```

### `clockid_t`

```rust
type clockid_t = c_int;
```

### `timer_t`

```rust
type timer_t = *mut c_void;
```

### `useconds_t`

```rust
type useconds_t = u32;
```

### `key_t`

```rust
type key_t = c_int;
```

### `id_t`

```rust
type id_t = c_uint;
```

## Constants

### `INT_MIN`
```rust
const INT_MIN: c_int = -2_147_483_648i32;
```

### `INT_MAX`
```rust
const INT_MAX: c_int = 2_147_483_647i32;
```

### `SIG_DFL`
```rust
const SIG_DFL: sighandler_t = 0usize;
```

### `SIG_IGN`
```rust
const SIG_IGN: sighandler_t = 1usize;
```

### `SIG_ERR`
```rust
const SIG_ERR: sighandler_t = 18_446_744_073_709_551_615usize;
```

### `DT_UNKNOWN`
```rust
const DT_UNKNOWN: u8 = 0u8;
```

### `DT_FIFO`
```rust
const DT_FIFO: u8 = 1u8;
```

### `DT_CHR`
```rust
const DT_CHR: u8 = 2u8;
```

### `DT_DIR`
```rust
const DT_DIR: u8 = 4u8;
```

### `DT_BLK`
```rust
const DT_BLK: u8 = 6u8;
```

### `DT_REG`
```rust
const DT_REG: u8 = 8u8;
```

### `DT_LNK`
```rust
const DT_LNK: u8 = 10u8;
```

### `DT_SOCK`
```rust
const DT_SOCK: u8 = 12u8;
```

### `FD_CLOEXEC`
```rust
const FD_CLOEXEC: c_int = 1i32;
```

### `USRQUOTA`
```rust
const USRQUOTA: c_int = 0i32;
```

### `GRPQUOTA`
```rust
const GRPQUOTA: c_int = 1i32;
```

### `SIGIOT`
```rust
const SIGIOT: c_int = 6i32;
```

### `S_ISUID`
```rust
const S_ISUID: mode_t = 2_048u32;
```

### `S_ISGID`
```rust
const S_ISGID: mode_t = 1_024u32;
```

### `S_ISVTX`
```rust
const S_ISVTX: mode_t = 512u32;
```

### `IF_NAMESIZE`
```rust
const IF_NAMESIZE: size_t = 16usize;
```

### `IFNAMSIZ`
```rust
const IFNAMSIZ: size_t = 16usize;
```

### `LOG_EMERG`
```rust
const LOG_EMERG: c_int = 0i32;
```

### `LOG_ALERT`
```rust
const LOG_ALERT: c_int = 1i32;
```

### `LOG_CRIT`
```rust
const LOG_CRIT: c_int = 2i32;
```

### `LOG_ERR`
```rust
const LOG_ERR: c_int = 3i32;
```

### `LOG_WARNING`
```rust
const LOG_WARNING: c_int = 4i32;
```

### `LOG_NOTICE`
```rust
const LOG_NOTICE: c_int = 5i32;
```

### `LOG_INFO`
```rust
const LOG_INFO: c_int = 6i32;
```

### `LOG_DEBUG`
```rust
const LOG_DEBUG: c_int = 7i32;
```

### `LOG_KERN`
```rust
const LOG_KERN: c_int = 0i32;
```

### `LOG_USER`
```rust
const LOG_USER: c_int = 8i32;
```

### `LOG_MAIL`
```rust
const LOG_MAIL: c_int = 16i32;
```

### `LOG_DAEMON`
```rust
const LOG_DAEMON: c_int = 24i32;
```

### `LOG_AUTH`
```rust
const LOG_AUTH: c_int = 32i32;
```

### `LOG_SYSLOG`
```rust
const LOG_SYSLOG: c_int = 40i32;
```

### `LOG_LPR`
```rust
const LOG_LPR: c_int = 48i32;
```

### `LOG_NEWS`
```rust
const LOG_NEWS: c_int = 56i32;
```

### `LOG_UUCP`
```rust
const LOG_UUCP: c_int = 64i32;
```

### `LOG_LOCAL0`
```rust
const LOG_LOCAL0: c_int = 128i32;
```

### `LOG_LOCAL1`
```rust
const LOG_LOCAL1: c_int = 136i32;
```

### `LOG_LOCAL2`
```rust
const LOG_LOCAL2: c_int = 144i32;
```

### `LOG_LOCAL3`
```rust
const LOG_LOCAL3: c_int = 152i32;
```

### `LOG_LOCAL4`
```rust
const LOG_LOCAL4: c_int = 160i32;
```

### `LOG_LOCAL5`
```rust
const LOG_LOCAL5: c_int = 168i32;
```

### `LOG_LOCAL6`
```rust
const LOG_LOCAL6: c_int = 176i32;
```

### `LOG_LOCAL7`
```rust
const LOG_LOCAL7: c_int = 184i32;
```

### `LOG_PID`
```rust
const LOG_PID: c_int = 1i32;
```

### `LOG_CONS`
```rust
const LOG_CONS: c_int = 2i32;
```

### `LOG_ODELAY`
```rust
const LOG_ODELAY: c_int = 4i32;
```

### `LOG_NDELAY`
```rust
const LOG_NDELAY: c_int = 8i32;
```

### `LOG_NOWAIT`
```rust
const LOG_NOWAIT: c_int = 16i32;
```

### `LOG_PRIMASK`
```rust
const LOG_PRIMASK: c_int = 7i32;
```

### `LOG_FACMASK`
```rust
const LOG_FACMASK: c_int = 1_016i32;
```

### `PRIO_MIN`
```rust
const PRIO_MIN: c_int = -20i32;
```

### `PRIO_MAX`
```rust
const PRIO_MAX: c_int = 20i32;
```

### `IPPROTO_ICMP`
```rust
const IPPROTO_ICMP: c_int = 1i32;
```

### `IPPROTO_ICMPV6`
```rust
const IPPROTO_ICMPV6: c_int = 58i32;
```

### `IPPROTO_TCP`
```rust
const IPPROTO_TCP: c_int = 6i32;
```

### `IPPROTO_UDP`
```rust
const IPPROTO_UDP: c_int = 17i32;
```

### `IPPROTO_IP`
```rust
const IPPROTO_IP: c_int = 0i32;
```

### `IPPROTO_IPV6`
```rust
const IPPROTO_IPV6: c_int = 41i32;
```

### `INADDR_LOOPBACK`
```rust
const INADDR_LOOPBACK: in_addr_t = 2_130_706_433u32;
```

### `INADDR_ANY`
```rust
const INADDR_ANY: in_addr_t = 0u32;
```

### `INADDR_BROADCAST`
```rust
const INADDR_BROADCAST: in_addr_t = 4_294_967_295u32;
```

### `INADDR_NONE`
```rust
const INADDR_NONE: in_addr_t = 4_294_967_295u32;
```

### `IN6ADDR_LOOPBACK_INIT`
```rust
const IN6ADDR_LOOPBACK_INIT: in6_addr;
```

### `IN6ADDR_ANY_INIT`
```rust
const IN6ADDR_ANY_INIT: in6_addr;
```

### `ARPOP_REQUEST`
```rust
const ARPOP_REQUEST: u16 = 1u16;
```

### `ARPOP_REPLY`
```rust
const ARPOP_REPLY: u16 = 2u16;
```

### `ATF_COM`
```rust
const ATF_COM: c_int = 2i32;
```

### `ATF_PERM`
```rust
const ATF_PERM: c_int = 4i32;
```

### `ATF_PUBL`
```rust
const ATF_PUBL: c_int = 8i32;
```

### `ATF_USETRAILERS`
```rust
const ATF_USETRAILERS: c_int = 16i32;
```

### `FNM_PERIOD`
```rust
const FNM_PERIOD: c_int = 4i32;
```

### `FNM_NOMATCH`
```rust
const FNM_NOMATCH: c_int = 1i32;
```

### `FNM_CASEFOLD`
```rust
const FNM_CASEFOLD: c_int = 16i32;
```

### `FNM_PATHNAME`
```rust
const FNM_PATHNAME: c_int = 1i32;
```

### `FNM_NOESCAPE`
```rust
const FNM_NOESCAPE: c_int = 2i32;
```

### `ULONG_SIZE`
```rust
const ULONG_SIZE: usize = 64usize;
```

### `EXIT_FAILURE`
```rust
const EXIT_FAILURE: c_int = 1i32;
```

### `EXIT_SUCCESS`
```rust
const EXIT_SUCCESS: c_int = 0i32;
```

### `RAND_MAX`
```rust
const RAND_MAX: c_int = 2_147_483_647i32;
```

### `EOF`
```rust
const EOF: c_int = -1i32;
```

### `SEEK_SET`
```rust
const SEEK_SET: c_int = 0i32;
```

### `SEEK_CUR`
```rust
const SEEK_CUR: c_int = 1i32;
```

### `SEEK_END`
```rust
const SEEK_END: c_int = 2i32;
```

### `_IOFBF`
```rust
const _IOFBF: c_int = 0i32;
```

### `_IONBF`
```rust
const _IONBF: c_int = 2i32;
```

### `_IOLBF`
```rust
const _IOLBF: c_int = 1i32;
```

### `F_DUPFD`
```rust
const F_DUPFD: c_int = 0i32;
```

### `F_GETFD`
```rust
const F_GETFD: c_int = 1i32;
```

### `F_SETFD`
```rust
const F_SETFD: c_int = 2i32;
```

### `F_GETFL`
```rust
const F_GETFL: c_int = 3i32;
```

### `F_SETFL`
```rust
const F_SETFL: c_int = 4i32;
```

### `F_SETLEASE`
```rust
const F_SETLEASE: c_int = 1_024i32;
```

### `F_GETLEASE`
```rust
const F_GETLEASE: c_int = 1_025i32;
```

### `F_NOTIFY`
```rust
const F_NOTIFY: c_int = 1_026i32;
```

### `F_CANCELLK`
```rust
const F_CANCELLK: c_int = 1_029i32;
```

### `F_DUPFD_CLOEXEC`
```rust
const F_DUPFD_CLOEXEC: c_int = 1_030i32;
```

### `F_SETPIPE_SZ`
```rust
const F_SETPIPE_SZ: c_int = 1_031i32;
```

### `F_GETPIPE_SZ`
```rust
const F_GETPIPE_SZ: c_int = 1_032i32;
```

### `F_ADD_SEALS`
```rust
const F_ADD_SEALS: c_int = 1_033i32;
```

### `F_GET_SEALS`
```rust
const F_GET_SEALS: c_int = 1_034i32;
```

### `F_SEAL_SEAL`
```rust
const F_SEAL_SEAL: c_int = 1i32;
```

### `F_SEAL_SHRINK`
```rust
const F_SEAL_SHRINK: c_int = 2i32;
```

### `F_SEAL_GROW`
```rust
const F_SEAL_GROW: c_int = 4i32;
```

### `F_SEAL_WRITE`
```rust
const F_SEAL_WRITE: c_int = 8i32;
```

### `SIGTRAP`
```rust
const SIGTRAP: c_int = 5i32;
```

### `PTHREAD_CREATE_JOINABLE`
```rust
const PTHREAD_CREATE_JOINABLE: c_int = 0i32;
```

### `PTHREAD_CREATE_DETACHED`
```rust
const PTHREAD_CREATE_DETACHED: c_int = 1i32;
```

### `CLOCK_REALTIME`
```rust
const CLOCK_REALTIME: crate::clockid_t = 0i32;
```

### `CLOCK_MONOTONIC`
```rust
const CLOCK_MONOTONIC: crate::clockid_t = 1i32;
```

### `CLOCK_PROCESS_CPUTIME_ID`
```rust
const CLOCK_PROCESS_CPUTIME_ID: crate::clockid_t = 2i32;
```

### `CLOCK_THREAD_CPUTIME_ID`
```rust
const CLOCK_THREAD_CPUTIME_ID: crate::clockid_t = 3i32;
```

### `CLOCK_MONOTONIC_RAW`
```rust
const CLOCK_MONOTONIC_RAW: crate::clockid_t = 4i32;
```

### `CLOCK_REALTIME_COARSE`
```rust
const CLOCK_REALTIME_COARSE: crate::clockid_t = 5i32;
```

### `CLOCK_MONOTONIC_COARSE`
```rust
const CLOCK_MONOTONIC_COARSE: crate::clockid_t = 6i32;
```

### `CLOCK_BOOTTIME`
```rust
const CLOCK_BOOTTIME: crate::clockid_t = 7i32;
```

### `CLOCK_REALTIME_ALARM`
```rust
const CLOCK_REALTIME_ALARM: crate::clockid_t = 8i32;
```

### `CLOCK_BOOTTIME_ALARM`
```rust
const CLOCK_BOOTTIME_ALARM: crate::clockid_t = 9i32;
```

### `CLOCK_TAI`
```rust
const CLOCK_TAI: crate::clockid_t = 11i32;
```

### `TIMER_ABSTIME`
```rust
const TIMER_ABSTIME: c_int = 1i32;
```

### `RUSAGE_SELF`
```rust
const RUSAGE_SELF: c_int = 0i32;
```

### `O_RDONLY`
```rust
const O_RDONLY: c_int = 0i32;
```

### `O_WRONLY`
```rust
const O_WRONLY: c_int = 1i32;
```

### `O_RDWR`
```rust
const O_RDWR: c_int = 2i32;
```

### `SOCK_CLOEXEC`
```rust
const SOCK_CLOEXEC: c_int = 524_288i32;
```

### `S_IFIFO`
```rust
const S_IFIFO: mode_t = 4_096u32;
```

### `S_IFCHR`
```rust
const S_IFCHR: mode_t = 8_192u32;
```

### `S_IFBLK`
```rust
const S_IFBLK: mode_t = 24_576u32;
```

### `S_IFDIR`
```rust
const S_IFDIR: mode_t = 16_384u32;
```

### `S_IFREG`
```rust
const S_IFREG: mode_t = 32_768u32;
```

### `S_IFLNK`
```rust
const S_IFLNK: mode_t = 40_960u32;
```

### `S_IFSOCK`
```rust
const S_IFSOCK: mode_t = 49_152u32;
```

### `S_IFMT`
```rust
const S_IFMT: mode_t = 61_440u32;
```

### `S_IRWXU`
```rust
const S_IRWXU: mode_t = 448u32;
```

### `S_IXUSR`
```rust
const S_IXUSR: mode_t = 64u32;
```

### `S_IWUSR`
```rust
const S_IWUSR: mode_t = 128u32;
```

### `S_IRUSR`
```rust
const S_IRUSR: mode_t = 256u32;
```

### `S_IRWXG`
```rust
const S_IRWXG: mode_t = 56u32;
```

### `S_IXGRP`
```rust
const S_IXGRP: mode_t = 8u32;
```

### `S_IWGRP`
```rust
const S_IWGRP: mode_t = 16u32;
```

### `S_IRGRP`
```rust
const S_IRGRP: mode_t = 32u32;
```

### `S_IRWXO`
```rust
const S_IRWXO: mode_t = 7u32;
```

### `S_IXOTH`
```rust
const S_IXOTH: mode_t = 1u32;
```

### `S_IWOTH`
```rust
const S_IWOTH: mode_t = 2u32;
```

### `S_IROTH`
```rust
const S_IROTH: mode_t = 4u32;
```

### `F_OK`
```rust
const F_OK: c_int = 0i32;
```

### `R_OK`
```rust
const R_OK: c_int = 4i32;
```

### `W_OK`
```rust
const W_OK: c_int = 2i32;
```

### `X_OK`
```rust
const X_OK: c_int = 1i32;
```

### `SIGHUP`
```rust
const SIGHUP: c_int = 1i32;
```

### `SIGINT`
```rust
const SIGINT: c_int = 2i32;
```

### `SIGQUIT`
```rust
const SIGQUIT: c_int = 3i32;
```

### `SIGILL`
```rust
const SIGILL: c_int = 4i32;
```

### `SIGABRT`
```rust
const SIGABRT: c_int = 6i32;
```

### `SIGFPE`
```rust
const SIGFPE: c_int = 8i32;
```

### `SIGKILL`
```rust
const SIGKILL: c_int = 9i32;
```

### `SIGSEGV`
```rust
const SIGSEGV: c_int = 11i32;
```

### `SIGPIPE`
```rust
const SIGPIPE: c_int = 13i32;
```

### `SIGALRM`
```rust
const SIGALRM: c_int = 14i32;
```

### `SIGTERM`
```rust
const SIGTERM: c_int = 15i32;
```

### `PROT_NONE`
```rust
const PROT_NONE: c_int = 0i32;
```

### `PROT_READ`
```rust
const PROT_READ: c_int = 1i32;
```

### `PROT_WRITE`
```rust
const PROT_WRITE: c_int = 2i32;
```

### `PROT_EXEC`
```rust
const PROT_EXEC: c_int = 4i32;
```

### `XATTR_CREATE`
```rust
const XATTR_CREATE: c_int = 1i32;
```

### `XATTR_REPLACE`
```rust
const XATTR_REPLACE: c_int = 2i32;
```

### `RLIM64_INFINITY`
```rust
const RLIM64_INFINITY: crate::rlim64_t = 18_446_744_073_709_551_615u64;
```

### `LC_CTYPE`
```rust
const LC_CTYPE: c_int = 0i32;
```

### `LC_NUMERIC`
```rust
const LC_NUMERIC: c_int = 1i32;
```

### `LC_TIME`
```rust
const LC_TIME: c_int = 2i32;
```

### `LC_COLLATE`
```rust
const LC_COLLATE: c_int = 3i32;
```

### `LC_MONETARY`
```rust
const LC_MONETARY: c_int = 4i32;
```

### `LC_MESSAGES`
```rust
const LC_MESSAGES: c_int = 5i32;
```

### `LC_ALL`
```rust
const LC_ALL: c_int = 6i32;
```

### `LC_CTYPE_MASK`
```rust
const LC_CTYPE_MASK: c_int = 1i32;
```

### `LC_NUMERIC_MASK`
```rust
const LC_NUMERIC_MASK: c_int = 2i32;
```

### `LC_TIME_MASK`
```rust
const LC_TIME_MASK: c_int = 4i32;
```

### `LC_COLLATE_MASK`
```rust
const LC_COLLATE_MASK: c_int = 8i32;
```

### `LC_MONETARY_MASK`
```rust
const LC_MONETARY_MASK: c_int = 16i32;
```

### `LC_MESSAGES_MASK`
```rust
const LC_MESSAGES_MASK: c_int = 32i32;
```

### `MAP_FILE`
```rust
const MAP_FILE: c_int = 0i32;
```

### `MAP_SHARED`
```rust
const MAP_SHARED: c_int = 1i32;
```

### `MAP_PRIVATE`
```rust
const MAP_PRIVATE: c_int = 2i32;
```

### `MAP_FIXED`
```rust
const MAP_FIXED: c_int = 16i32;
```

### `MAP_FAILED`
```rust
const MAP_FAILED: *mut c_void = {0xffffffffffffffff as *mut core::ffi::c_void};
```

### `MS_ASYNC`
```rust
const MS_ASYNC: c_int = 1i32;
```

### `MS_INVALIDATE`
```rust
const MS_INVALIDATE: c_int = 2i32;
```

### `MS_SYNC`
```rust
const MS_SYNC: c_int = 4i32;
```

### `MS_RDONLY`
```rust
const MS_RDONLY: c_ulong = 1u64;
```

### `MS_NOSUID`
```rust
const MS_NOSUID: c_ulong = 2u64;
```

### `MS_NODEV`
```rust
const MS_NODEV: c_ulong = 4u64;
```

### `MS_NOEXEC`
```rust
const MS_NOEXEC: c_ulong = 8u64;
```

### `MS_SYNCHRONOUS`
```rust
const MS_SYNCHRONOUS: c_ulong = 16u64;
```

### `MS_REMOUNT`
```rust
const MS_REMOUNT: c_ulong = 32u64;
```

### `MS_MANDLOCK`
```rust
const MS_MANDLOCK: c_ulong = 64u64;
```

### `MS_DIRSYNC`
```rust
const MS_DIRSYNC: c_ulong = 128u64;
```

### `MS_NOSYMFOLLOW`
```rust
const MS_NOSYMFOLLOW: c_ulong = 256u64;
```

### `MS_NOATIME`
```rust
const MS_NOATIME: c_ulong = 1_024u64;
```

### `MS_NODIRATIME`
```rust
const MS_NODIRATIME: c_ulong = 2_048u64;
```

### `MS_BIND`
```rust
const MS_BIND: c_ulong = 4_096u64;
```

### `MS_MOVE`
```rust
const MS_MOVE: c_ulong = 8_192u64;
```

### `MS_REC`
```rust
const MS_REC: c_ulong = 16_384u64;
```

### `MS_SILENT`
```rust
const MS_SILENT: c_ulong = 32_768u64;
```

### `MS_POSIXACL`
```rust
const MS_POSIXACL: c_ulong = 65_536u64;
```

### `MS_UNBINDABLE`
```rust
const MS_UNBINDABLE: c_ulong = 131_072u64;
```

### `MS_PRIVATE`
```rust
const MS_PRIVATE: c_ulong = 262_144u64;
```

### `MS_SLAVE`
```rust
const MS_SLAVE: c_ulong = 524_288u64;
```

### `MS_SHARED`
```rust
const MS_SHARED: c_ulong = 1_048_576u64;
```

### `MS_RELATIME`
```rust
const MS_RELATIME: c_ulong = 2_097_152u64;
```

### `MS_KERNMOUNT`
```rust
const MS_KERNMOUNT: c_ulong = 4_194_304u64;
```

### `MS_I_VERSION`
```rust
const MS_I_VERSION: c_ulong = 8_388_608u64;
```

### `MS_STRICTATIME`
```rust
const MS_STRICTATIME: c_ulong = 16_777_216u64;
```

### `MS_LAZYTIME`
```rust
const MS_LAZYTIME: c_ulong = 33_554_432u64;
```

### `MS_ACTIVE`
```rust
const MS_ACTIVE: c_ulong = 1_073_741_824u64;
```

### `MS_MGC_VAL`
```rust
const MS_MGC_VAL: c_ulong = 3_236_757_504u64;
```

### `MS_MGC_MSK`
```rust
const MS_MGC_MSK: c_ulong = 4_294_901_760u64;
```

### `SCM_RIGHTS`
```rust
const SCM_RIGHTS: c_int = 1i32;
```

### `SCM_CREDENTIALS`
```rust
const SCM_CREDENTIALS: c_int = 2i32;
```

### `PROT_GROWSDOWN`
```rust
const PROT_GROWSDOWN: c_int = 16_777_216i32;
```

### `PROT_GROWSUP`
```rust
const PROT_GROWSUP: c_int = 33_554_432i32;
```

### `MAP_TYPE`
```rust
const MAP_TYPE: c_int = 15i32;
```

### `MADV_NORMAL`
```rust
const MADV_NORMAL: c_int = 0i32;
```

### `MADV_RANDOM`
```rust
const MADV_RANDOM: c_int = 1i32;
```

### `MADV_SEQUENTIAL`
```rust
const MADV_SEQUENTIAL: c_int = 2i32;
```

### `MADV_WILLNEED`
```rust
const MADV_WILLNEED: c_int = 3i32;
```

### `MADV_DONTNEED`
```rust
const MADV_DONTNEED: c_int = 4i32;
```

### `MADV_FREE`
```rust
const MADV_FREE: c_int = 8i32;
```

### `MADV_REMOVE`
```rust
const MADV_REMOVE: c_int = 9i32;
```

### `MADV_DONTFORK`
```rust
const MADV_DONTFORK: c_int = 10i32;
```

### `MADV_DOFORK`
```rust
const MADV_DOFORK: c_int = 11i32;
```

### `MADV_MERGEABLE`
```rust
const MADV_MERGEABLE: c_int = 12i32;
```

### `MADV_UNMERGEABLE`
```rust
const MADV_UNMERGEABLE: c_int = 13i32;
```

### `MADV_HUGEPAGE`
```rust
const MADV_HUGEPAGE: c_int = 14i32;
```

### `MADV_NOHUGEPAGE`
```rust
const MADV_NOHUGEPAGE: c_int = 15i32;
```

### `MADV_DONTDUMP`
```rust
const MADV_DONTDUMP: c_int = 16i32;
```

### `MADV_DODUMP`
```rust
const MADV_DODUMP: c_int = 17i32;
```

### `MADV_WIPEONFORK`
```rust
const MADV_WIPEONFORK: c_int = 18i32;
```

### `MADV_KEEPONFORK`
```rust
const MADV_KEEPONFORK: c_int = 19i32;
```

### `MADV_COLD`
```rust
const MADV_COLD: c_int = 20i32;
```

### `MADV_PAGEOUT`
```rust
const MADV_PAGEOUT: c_int = 21i32;
```

### `MADV_HWPOISON`
```rust
const MADV_HWPOISON: c_int = 100i32;
```

### `MADV_POPULATE_READ`
```rust
const MADV_POPULATE_READ: c_int = 22i32;
```

### `MADV_POPULATE_WRITE`
```rust
const MADV_POPULATE_WRITE: c_int = 23i32;
```

### `MADV_DONTNEED_LOCKED`
```rust
const MADV_DONTNEED_LOCKED: c_int = 24i32;
```

### `IFF_UP`
```rust
const IFF_UP: c_int = 1i32;
```

### `IFF_BROADCAST`
```rust
const IFF_BROADCAST: c_int = 2i32;
```

### `IFF_DEBUG`
```rust
const IFF_DEBUG: c_int = 4i32;
```

### `IFF_LOOPBACK`
```rust
const IFF_LOOPBACK: c_int = 8i32;
```

### `IFF_POINTOPOINT`
```rust
const IFF_POINTOPOINT: c_int = 16i32;
```

### `IFF_NOTRAILERS`
```rust
const IFF_NOTRAILERS: c_int = 32i32;
```

### `IFF_RUNNING`
```rust
const IFF_RUNNING: c_int = 64i32;
```

### `IFF_NOARP`
```rust
const IFF_NOARP: c_int = 128i32;
```

### `IFF_PROMISC`
```rust
const IFF_PROMISC: c_int = 256i32;
```

### `IFF_ALLMULTI`
```rust
const IFF_ALLMULTI: c_int = 512i32;
```

### `IFF_MASTER`
```rust
const IFF_MASTER: c_int = 1_024i32;
```

### `IFF_SLAVE`
```rust
const IFF_SLAVE: c_int = 2_048i32;
```

### `IFF_MULTICAST`
```rust
const IFF_MULTICAST: c_int = 4_096i32;
```

### `IFF_PORTSEL`
```rust
const IFF_PORTSEL: c_int = 8_192i32;
```

### `IFF_AUTOMEDIA`
```rust
const IFF_AUTOMEDIA: c_int = 16_384i32;
```

### `IFF_DYNAMIC`
```rust
const IFF_DYNAMIC: c_int = 32_768i32;
```

### `SOL_IP`
```rust
const SOL_IP: c_int = 0i32;
```

### `SOL_TCP`
```rust
const SOL_TCP: c_int = 6i32;
```

### `SOL_UDP`
```rust
const SOL_UDP: c_int = 17i32;
```

### `SOL_IPV6`
```rust
const SOL_IPV6: c_int = 41i32;
```

### `SOL_ICMPV6`
```rust
const SOL_ICMPV6: c_int = 58i32;
```

### `SOL_RAW`
```rust
const SOL_RAW: c_int = 255i32;
```

### `SOL_DECNET`
```rust
const SOL_DECNET: c_int = 261i32;
```

### `SOL_X25`
```rust
const SOL_X25: c_int = 262i32;
```

### `SOL_PACKET`
```rust
const SOL_PACKET: c_int = 263i32;
```

### `SOL_ATM`
```rust
const SOL_ATM: c_int = 264i32;
```

### `SOL_AAL`
```rust
const SOL_AAL: c_int = 265i32;
```

### `SOL_IRDA`
```rust
const SOL_IRDA: c_int = 266i32;
```

### `SOL_NETBEUI`
```rust
const SOL_NETBEUI: c_int = 267i32;
```

### `SOL_LLC`
```rust
const SOL_LLC: c_int = 268i32;
```

### `SOL_DCCP`
```rust
const SOL_DCCP: c_int = 269i32;
```

### `SOL_NETLINK`
```rust
const SOL_NETLINK: c_int = 270i32;
```

### `SOL_TIPC`
```rust
const SOL_TIPC: c_int = 271i32;
```

### `SOL_BLUETOOTH`
```rust
const SOL_BLUETOOTH: c_int = 274i32;
```

### `SOL_ALG`
```rust
const SOL_ALG: c_int = 279i32;
```

### `AF_UNSPEC`
```rust
const AF_UNSPEC: c_int = 0i32;
```

### `AF_UNIX`
```rust
const AF_UNIX: c_int = 1i32;
```

### `AF_LOCAL`
```rust
const AF_LOCAL: c_int = 1i32;
```

### `AF_INET`
```rust
const AF_INET: c_int = 2i32;
```

### `AF_AX25`
```rust
const AF_AX25: c_int = 3i32;
```

### `AF_IPX`
```rust
const AF_IPX: c_int = 4i32;
```

### `AF_APPLETALK`
```rust
const AF_APPLETALK: c_int = 5i32;
```

### `AF_NETROM`
```rust
const AF_NETROM: c_int = 6i32;
```

### `AF_BRIDGE`
```rust
const AF_BRIDGE: c_int = 7i32;
```

### `AF_ATMPVC`
```rust
const AF_ATMPVC: c_int = 8i32;
```

### `AF_X25`
```rust
const AF_X25: c_int = 9i32;
```

### `AF_INET6`
```rust
const AF_INET6: c_int = 10i32;
```

### `AF_ROSE`
```rust
const AF_ROSE: c_int = 11i32;
```

### `AF_DECnet`
```rust
const AF_DECnet: c_int = 12i32;
```

### `AF_NETBEUI`
```rust
const AF_NETBEUI: c_int = 13i32;
```

### `AF_SECURITY`
```rust
const AF_SECURITY: c_int = 14i32;
```

### `AF_KEY`
```rust
const AF_KEY: c_int = 15i32;
```

### `AF_NETLINK`
```rust
const AF_NETLINK: c_int = 16i32;
```

### `AF_ROUTE`
```rust
const AF_ROUTE: c_int = 16i32;
```

### `AF_PACKET`
```rust
const AF_PACKET: c_int = 17i32;
```

### `AF_ASH`
```rust
const AF_ASH: c_int = 18i32;
```

### `AF_ECONET`
```rust
const AF_ECONET: c_int = 19i32;
```

### `AF_ATMSVC`
```rust
const AF_ATMSVC: c_int = 20i32;
```

### `AF_RDS`
```rust
const AF_RDS: c_int = 21i32;
```

### `AF_SNA`
```rust
const AF_SNA: c_int = 22i32;
```

### `AF_IRDA`
```rust
const AF_IRDA: c_int = 23i32;
```

### `AF_PPPOX`
```rust
const AF_PPPOX: c_int = 24i32;
```

### `AF_WANPIPE`
```rust
const AF_WANPIPE: c_int = 25i32;
```

### `AF_LLC`
```rust
const AF_LLC: c_int = 26i32;
```

### `AF_CAN`
```rust
const AF_CAN: c_int = 29i32;
```

### `AF_TIPC`
```rust
const AF_TIPC: c_int = 30i32;
```

### `AF_BLUETOOTH`
```rust
const AF_BLUETOOTH: c_int = 31i32;
```

### `AF_IUCV`
```rust
const AF_IUCV: c_int = 32i32;
```

### `AF_RXRPC`
```rust
const AF_RXRPC: c_int = 33i32;
```

### `AF_ISDN`
```rust
const AF_ISDN: c_int = 34i32;
```

### `AF_PHONET`
```rust
const AF_PHONET: c_int = 35i32;
```

### `AF_IEEE802154`
```rust
const AF_IEEE802154: c_int = 36i32;
```

### `AF_CAIF`
```rust
const AF_CAIF: c_int = 37i32;
```

### `AF_ALG`
```rust
const AF_ALG: c_int = 38i32;
```

### `PF_UNSPEC`
```rust
const PF_UNSPEC: c_int = 0i32;
```

### `PF_UNIX`
```rust
const PF_UNIX: c_int = 1i32;
```

### `PF_LOCAL`
```rust
const PF_LOCAL: c_int = 1i32;
```

### `PF_INET`
```rust
const PF_INET: c_int = 2i32;
```

### `PF_AX25`
```rust
const PF_AX25: c_int = 3i32;
```

### `PF_IPX`
```rust
const PF_IPX: c_int = 4i32;
```

### `PF_APPLETALK`
```rust
const PF_APPLETALK: c_int = 5i32;
```

### `PF_NETROM`
```rust
const PF_NETROM: c_int = 6i32;
```

### `PF_BRIDGE`
```rust
const PF_BRIDGE: c_int = 7i32;
```

### `PF_ATMPVC`
```rust
const PF_ATMPVC: c_int = 8i32;
```

### `PF_X25`
```rust
const PF_X25: c_int = 9i32;
```

### `PF_INET6`
```rust
const PF_INET6: c_int = 10i32;
```

### `PF_ROSE`
```rust
const PF_ROSE: c_int = 11i32;
```

### `PF_DECnet`
```rust
const PF_DECnet: c_int = 12i32;
```

### `PF_NETBEUI`
```rust
const PF_NETBEUI: c_int = 13i32;
```

### `PF_SECURITY`
```rust
const PF_SECURITY: c_int = 14i32;
```

### `PF_KEY`
```rust
const PF_KEY: c_int = 15i32;
```

### `PF_NETLINK`
```rust
const PF_NETLINK: c_int = 16i32;
```

### `PF_ROUTE`
```rust
const PF_ROUTE: c_int = 16i32;
```

### `PF_PACKET`
```rust
const PF_PACKET: c_int = 17i32;
```

### `PF_ASH`
```rust
const PF_ASH: c_int = 18i32;
```

### `PF_ECONET`
```rust
const PF_ECONET: c_int = 19i32;
```

### `PF_ATMSVC`
```rust
const PF_ATMSVC: c_int = 20i32;
```

### `PF_RDS`
```rust
const PF_RDS: c_int = 21i32;
```

### `PF_SNA`
```rust
const PF_SNA: c_int = 22i32;
```

### `PF_IRDA`
```rust
const PF_IRDA: c_int = 23i32;
```

### `PF_PPPOX`
```rust
const PF_PPPOX: c_int = 24i32;
```

### `PF_WANPIPE`
```rust
const PF_WANPIPE: c_int = 25i32;
```

### `PF_LLC`
```rust
const PF_LLC: c_int = 26i32;
```

### `PF_CAN`
```rust
const PF_CAN: c_int = 29i32;
```

### `PF_TIPC`
```rust
const PF_TIPC: c_int = 30i32;
```

### `PF_BLUETOOTH`
```rust
const PF_BLUETOOTH: c_int = 31i32;
```

### `PF_IUCV`
```rust
const PF_IUCV: c_int = 32i32;
```

### `PF_RXRPC`
```rust
const PF_RXRPC: c_int = 33i32;
```

### `PF_ISDN`
```rust
const PF_ISDN: c_int = 34i32;
```

### `PF_PHONET`
```rust
const PF_PHONET: c_int = 35i32;
```

### `PF_IEEE802154`
```rust
const PF_IEEE802154: c_int = 36i32;
```

### `PF_CAIF`
```rust
const PF_CAIF: c_int = 37i32;
```

### `PF_ALG`
```rust
const PF_ALG: c_int = 38i32;
```

### `MSG_OOB`
```rust
const MSG_OOB: c_int = 1i32;
```

### `MSG_PEEK`
```rust
const MSG_PEEK: c_int = 2i32;
```

### `MSG_DONTROUTE`
```rust
const MSG_DONTROUTE: c_int = 4i32;
```

### `MSG_CTRUNC`
```rust
const MSG_CTRUNC: c_int = 8i32;
```

### `MSG_TRUNC`
```rust
const MSG_TRUNC: c_int = 32i32;
```

### `MSG_DONTWAIT`
```rust
const MSG_DONTWAIT: c_int = 64i32;
```

### `MSG_EOR`
```rust
const MSG_EOR: c_int = 128i32;
```

### `MSG_WAITALL`
```rust
const MSG_WAITALL: c_int = 256i32;
```

### `MSG_FIN`
```rust
const MSG_FIN: c_int = 512i32;
```

### `MSG_SYN`
```rust
const MSG_SYN: c_int = 1_024i32;
```

### `MSG_CONFIRM`
```rust
const MSG_CONFIRM: c_int = 2_048i32;
```

### `MSG_RST`
```rust
const MSG_RST: c_int = 4_096i32;
```

### `MSG_ERRQUEUE`
```rust
const MSG_ERRQUEUE: c_int = 8_192i32;
```

### `MSG_NOSIGNAL`
```rust
const MSG_NOSIGNAL: c_int = 16_384i32;
```

### `MSG_MORE`
```rust
const MSG_MORE: c_int = 32_768i32;
```

### `MSG_WAITFORONE`
```rust
const MSG_WAITFORONE: c_int = 65_536i32;
```

### `MSG_FASTOPEN`
```rust
const MSG_FASTOPEN: c_int = 536_870_912i32;
```

### `MSG_CMSG_CLOEXEC`
```rust
const MSG_CMSG_CLOEXEC: c_int = 1_073_741_824i32;
```

### `SCM_TIMESTAMP`
```rust
const SCM_TIMESTAMP: c_int = 29i32;
```

### `SOCK_RAW`
```rust
const SOCK_RAW: c_int = 3i32;
```

### `SOCK_RDM`
```rust
const SOCK_RDM: c_int = 4i32;
```

### `IP_TOS`
```rust
const IP_TOS: c_int = 1i32;
```

### `IP_TTL`
```rust
const IP_TTL: c_int = 2i32;
```

### `IP_HDRINCL`
```rust
const IP_HDRINCL: c_int = 3i32;
```

### `IP_OPTIONS`
```rust
const IP_OPTIONS: c_int = 4i32;
```

### `IP_ROUTER_ALERT`
```rust
const IP_ROUTER_ALERT: c_int = 5i32;
```

### `IP_RECVOPTS`
```rust
const IP_RECVOPTS: c_int = 6i32;
```

### `IP_RETOPTS`
```rust
const IP_RETOPTS: c_int = 7i32;
```

### `IP_PKTINFO`
```rust
const IP_PKTINFO: c_int = 8i32;
```

### `IP_PKTOPTIONS`
```rust
const IP_PKTOPTIONS: c_int = 9i32;
```

### `IP_MTU_DISCOVER`
```rust
const IP_MTU_DISCOVER: c_int = 10i32;
```

### `IP_RECVERR`
```rust
const IP_RECVERR: c_int = 11i32;
```

### `IP_RECVTTL`
```rust
const IP_RECVTTL: c_int = 12i32;
```

### `IP_RECVTOS`
```rust
const IP_RECVTOS: c_int = 13i32;
```

### `IP_MTU`
```rust
const IP_MTU: c_int = 14i32;
```

### `IP_FREEBIND`
```rust
const IP_FREEBIND: c_int = 15i32;
```

### `IP_IPSEC_POLICY`
```rust
const IP_IPSEC_POLICY: c_int = 16i32;
```

### `IP_XFRM_POLICY`
```rust
const IP_XFRM_POLICY: c_int = 17i32;
```

### `IP_PASSSEC`
```rust
const IP_PASSSEC: c_int = 18i32;
```

### `IP_TRANSPARENT`
```rust
const IP_TRANSPARENT: c_int = 19i32;
```

### `IP_ORIGDSTADDR`
```rust
const IP_ORIGDSTADDR: c_int = 20i32;
```

### `IP_RECVORIGDSTADDR`
```rust
const IP_RECVORIGDSTADDR: c_int = 20i32;
```

### `IP_MINTTL`
```rust
const IP_MINTTL: c_int = 21i32;
```

### `IP_NODEFRAG`
```rust
const IP_NODEFRAG: c_int = 22i32;
```

### `IP_CHECKSUM`
```rust
const IP_CHECKSUM: c_int = 23i32;
```

### `IP_BIND_ADDRESS_NO_PORT`
```rust
const IP_BIND_ADDRESS_NO_PORT: c_int = 24i32;
```

### `IP_MULTICAST_IF`
```rust
const IP_MULTICAST_IF: c_int = 32i32;
```

### `IP_MULTICAST_TTL`
```rust
const IP_MULTICAST_TTL: c_int = 33i32;
```

### `IP_MULTICAST_LOOP`
```rust
const IP_MULTICAST_LOOP: c_int = 34i32;
```

### `IP_ADD_MEMBERSHIP`
```rust
const IP_ADD_MEMBERSHIP: c_int = 35i32;
```

### `IP_DROP_MEMBERSHIP`
```rust
const IP_DROP_MEMBERSHIP: c_int = 36i32;
```

### `IP_UNBLOCK_SOURCE`
```rust
const IP_UNBLOCK_SOURCE: c_int = 37i32;
```

### `IP_BLOCK_SOURCE`
```rust
const IP_BLOCK_SOURCE: c_int = 38i32;
```

### `IP_ADD_SOURCE_MEMBERSHIP`
```rust
const IP_ADD_SOURCE_MEMBERSHIP: c_int = 39i32;
```

### `IP_DROP_SOURCE_MEMBERSHIP`
```rust
const IP_DROP_SOURCE_MEMBERSHIP: c_int = 40i32;
```

### `IP_MSFILTER`
```rust
const IP_MSFILTER: c_int = 41i32;
```

### `IP_MULTICAST_ALL`
```rust
const IP_MULTICAST_ALL: c_int = 49i32;
```

### `IP_UNICAST_IF`
```rust
const IP_UNICAST_IF: c_int = 50i32;
```

### `IP_DEFAULT_MULTICAST_TTL`
```rust
const IP_DEFAULT_MULTICAST_TTL: c_int = 1i32;
```

### `IP_DEFAULT_MULTICAST_LOOP`
```rust
const IP_DEFAULT_MULTICAST_LOOP: c_int = 1i32;
```

### `IP_PMTUDISC_DONT`
```rust
const IP_PMTUDISC_DONT: c_int = 0i32;
```

### `IP_PMTUDISC_WANT`
```rust
const IP_PMTUDISC_WANT: c_int = 1i32;
```

### `IP_PMTUDISC_DO`
```rust
const IP_PMTUDISC_DO: c_int = 2i32;
```

### `IP_PMTUDISC_PROBE`
```rust
const IP_PMTUDISC_PROBE: c_int = 3i32;
```

### `IP_PMTUDISC_INTERFACE`
```rust
const IP_PMTUDISC_INTERFACE: c_int = 4i32;
```

### `IP_PMTUDISC_OMIT`
```rust
const IP_PMTUDISC_OMIT: c_int = 5i32;
```

### `IPPROTO_HOPOPTS`
```rust
const IPPROTO_HOPOPTS: c_int = 0i32;
```

Hop-by-hop option header

### `IPPROTO_IGMP`
```rust
const IPPROTO_IGMP: c_int = 2i32;
```

group mgmt protocol

### `IPPROTO_IPIP`
```rust
const IPPROTO_IPIP: c_int = 4i32;
```

for compatibility

### `IPPROTO_EGP`
```rust
const IPPROTO_EGP: c_int = 8i32;
```

exterior gateway protocol

### `IPPROTO_PUP`
```rust
const IPPROTO_PUP: c_int = 12i32;
```

pup

### `IPPROTO_IDP`
```rust
const IPPROTO_IDP: c_int = 22i32;
```

xns idp

### `IPPROTO_TP`
```rust
const IPPROTO_TP: c_int = 29i32;
```

tp-4 w/ class negotiation

### `IPPROTO_DCCP`
```rust
const IPPROTO_DCCP: c_int = 33i32;
```

DCCP

### `IPPROTO_ROUTING`
```rust
const IPPROTO_ROUTING: c_int = 43i32;
```

IP6 routing header

### `IPPROTO_FRAGMENT`
```rust
const IPPROTO_FRAGMENT: c_int = 44i32;
```

IP6 fragmentation header

### `IPPROTO_RSVP`
```rust
const IPPROTO_RSVP: c_int = 46i32;
```

resource reservation

### `IPPROTO_GRE`
```rust
const IPPROTO_GRE: c_int = 47i32;
```

General Routing Encap.

### `IPPROTO_ESP`
```rust
const IPPROTO_ESP: c_int = 50i32;
```

IP6 Encap Sec. Payload

### `IPPROTO_AH`
```rust
const IPPROTO_AH: c_int = 51i32;
```

IP6 Auth Header

### `IPPROTO_NONE`
```rust
const IPPROTO_NONE: c_int = 59i32;
```

IP6 no next header

### `IPPROTO_DSTOPTS`
```rust
const IPPROTO_DSTOPTS: c_int = 60i32;
```

IP6 destination option

### `IPPROTO_MTP`
```rust
const IPPROTO_MTP: c_int = 92i32;
```

### `IPPROTO_ENCAP`
```rust
const IPPROTO_ENCAP: c_int = 98i32;
```

encapsulation header

### `IPPROTO_PIM`
```rust
const IPPROTO_PIM: c_int = 103i32;
```

Protocol indep. multicast

### `IPPROTO_COMP`
```rust
const IPPROTO_COMP: c_int = 108i32;
```

IP Payload Comp. Protocol

### `IPPROTO_SCTP`
```rust
const IPPROTO_SCTP: c_int = 132i32;
```

SCTP

### `IPPROTO_MH`
```rust
const IPPROTO_MH: c_int = 135i32;
```

### `IPPROTO_UDPLITE`
```rust
const IPPROTO_UDPLITE: c_int = 136i32;
```

### `IPPROTO_RAW`
```rust
const IPPROTO_RAW: c_int = 255i32;
```

raw IP packet

### `IPPROTO_BEETPH`
```rust
const IPPROTO_BEETPH: c_int = 94i32;
```

### `IPPROTO_MPLS`
```rust
const IPPROTO_MPLS: c_int = 137i32;
```

### `IPPROTO_MPTCP`
```rust
const IPPROTO_MPTCP: c_int = 262i32;
```

Multipath TCP

### `IPPROTO_ETHERNET`
```rust
const IPPROTO_ETHERNET: c_int = 143i32;
```

Ethernet-within-IPv6 encapsulation.

### `MCAST_EXCLUDE`
```rust
const MCAST_EXCLUDE: c_int = 0i32;
```

### `MCAST_INCLUDE`
```rust
const MCAST_INCLUDE: c_int = 1i32;
```

### `MCAST_JOIN_GROUP`
```rust
const MCAST_JOIN_GROUP: c_int = 42i32;
```

### `MCAST_BLOCK_SOURCE`
```rust
const MCAST_BLOCK_SOURCE: c_int = 43i32;
```

### `MCAST_UNBLOCK_SOURCE`
```rust
const MCAST_UNBLOCK_SOURCE: c_int = 44i32;
```

### `MCAST_LEAVE_GROUP`
```rust
const MCAST_LEAVE_GROUP: c_int = 45i32;
```

### `MCAST_JOIN_SOURCE_GROUP`
```rust
const MCAST_JOIN_SOURCE_GROUP: c_int = 46i32;
```

### `MCAST_LEAVE_SOURCE_GROUP`
```rust
const MCAST_LEAVE_SOURCE_GROUP: c_int = 47i32;
```

### `MCAST_MSFILTER`
```rust
const MCAST_MSFILTER: c_int = 48i32;
```

### `IPV6_ADDRFORM`
```rust
const IPV6_ADDRFORM: c_int = 1i32;
```

### `IPV6_2292PKTINFO`
```rust
const IPV6_2292PKTINFO: c_int = 2i32;
```

### `IPV6_2292HOPOPTS`
```rust
const IPV6_2292HOPOPTS: c_int = 3i32;
```

### `IPV6_2292DSTOPTS`
```rust
const IPV6_2292DSTOPTS: c_int = 4i32;
```

### `IPV6_2292RTHDR`
```rust
const IPV6_2292RTHDR: c_int = 5i32;
```

### `IPV6_2292PKTOPTIONS`
```rust
const IPV6_2292PKTOPTIONS: c_int = 6i32;
```

### `IPV6_CHECKSUM`
```rust
const IPV6_CHECKSUM: c_int = 7i32;
```

### `IPV6_2292HOPLIMIT`
```rust
const IPV6_2292HOPLIMIT: c_int = 8i32;
```

### `IPV6_NEXTHOP`
```rust
const IPV6_NEXTHOP: c_int = 9i32;
```

### `IPV6_AUTHHDR`
```rust
const IPV6_AUTHHDR: c_int = 10i32;
```

### `IPV6_UNICAST_HOPS`
```rust
const IPV6_UNICAST_HOPS: c_int = 16i32;
```

### `IPV6_MULTICAST_IF`
```rust
const IPV6_MULTICAST_IF: c_int = 17i32;
```

### `IPV6_MULTICAST_HOPS`
```rust
const IPV6_MULTICAST_HOPS: c_int = 18i32;
```

### `IPV6_MULTICAST_LOOP`
```rust
const IPV6_MULTICAST_LOOP: c_int = 19i32;
```

### `IPV6_ADD_MEMBERSHIP`
```rust
const IPV6_ADD_MEMBERSHIP: c_int = 20i32;
```

### `IPV6_DROP_MEMBERSHIP`
```rust
const IPV6_DROP_MEMBERSHIP: c_int = 21i32;
```

### `IPV6_ROUTER_ALERT`
```rust
const IPV6_ROUTER_ALERT: c_int = 22i32;
```

### `IPV6_MTU_DISCOVER`
```rust
const IPV6_MTU_DISCOVER: c_int = 23i32;
```

### `IPV6_MTU`
```rust
const IPV6_MTU: c_int = 24i32;
```

### `IPV6_RECVERR`
```rust
const IPV6_RECVERR: c_int = 25i32;
```

### `IPV6_V6ONLY`
```rust
const IPV6_V6ONLY: c_int = 26i32;
```

### `IPV6_JOIN_ANYCAST`
```rust
const IPV6_JOIN_ANYCAST: c_int = 27i32;
```

### `IPV6_LEAVE_ANYCAST`
```rust
const IPV6_LEAVE_ANYCAST: c_int = 28i32;
```

### `IPV6_IPSEC_POLICY`
```rust
const IPV6_IPSEC_POLICY: c_int = 34i32;
```

### `IPV6_XFRM_POLICY`
```rust
const IPV6_XFRM_POLICY: c_int = 35i32;
```

### `IPV6_HDRINCL`
```rust
const IPV6_HDRINCL: c_int = 36i32;
```

### `IPV6_RECVPKTINFO`
```rust
const IPV6_RECVPKTINFO: c_int = 49i32;
```

### `IPV6_PKTINFO`
```rust
const IPV6_PKTINFO: c_int = 50i32;
```

### `IPV6_RECVHOPLIMIT`
```rust
const IPV6_RECVHOPLIMIT: c_int = 51i32;
```

### `IPV6_HOPLIMIT`
```rust
const IPV6_HOPLIMIT: c_int = 52i32;
```

### `IPV6_RECVHOPOPTS`
```rust
const IPV6_RECVHOPOPTS: c_int = 53i32;
```

### `IPV6_HOPOPTS`
```rust
const IPV6_HOPOPTS: c_int = 54i32;
```

### `IPV6_RTHDRDSTOPTS`
```rust
const IPV6_RTHDRDSTOPTS: c_int = 55i32;
```

### `IPV6_RECVRTHDR`
```rust
const IPV6_RECVRTHDR: c_int = 56i32;
```

### `IPV6_RTHDR`
```rust
const IPV6_RTHDR: c_int = 57i32;
```

### `IPV6_RECVDSTOPTS`
```rust
const IPV6_RECVDSTOPTS: c_int = 58i32;
```

### `IPV6_DSTOPTS`
```rust
const IPV6_DSTOPTS: c_int = 59i32;
```

### `IPV6_RECVPATHMTU`
```rust
const IPV6_RECVPATHMTU: c_int = 60i32;
```

### `IPV6_PATHMTU`
```rust
const IPV6_PATHMTU: c_int = 61i32;
```

### `IPV6_DONTFRAG`
```rust
const IPV6_DONTFRAG: c_int = 62i32;
```

### `IPV6_RECVTCLASS`
```rust
const IPV6_RECVTCLASS: c_int = 66i32;
```

### `IPV6_TCLASS`
```rust
const IPV6_TCLASS: c_int = 67i32;
```

### `IPV6_AUTOFLOWLABEL`
```rust
const IPV6_AUTOFLOWLABEL: c_int = 70i32;
```

### `IPV6_ADDR_PREFERENCES`
```rust
const IPV6_ADDR_PREFERENCES: c_int = 72i32;
```

### `IPV6_MINHOPCOUNT`
```rust
const IPV6_MINHOPCOUNT: c_int = 73i32;
```

### `IPV6_ORIGDSTADDR`
```rust
const IPV6_ORIGDSTADDR: c_int = 74i32;
```

### `IPV6_RECVORIGDSTADDR`
```rust
const IPV6_RECVORIGDSTADDR: c_int = 74i32;
```

### `IPV6_TRANSPARENT`
```rust
const IPV6_TRANSPARENT: c_int = 75i32;
```

### `IPV6_UNICAST_IF`
```rust
const IPV6_UNICAST_IF: c_int = 76i32;
```

### `IPV6_PREFER_SRC_TMP`
```rust
const IPV6_PREFER_SRC_TMP: c_int = 1i32;
```

### `IPV6_PREFER_SRC_PUBLIC`
```rust
const IPV6_PREFER_SRC_PUBLIC: c_int = 2i32;
```

### `IPV6_PREFER_SRC_PUBTMP_DEFAULT`
```rust
const IPV6_PREFER_SRC_PUBTMP_DEFAULT: c_int = 256i32;
```

### `IPV6_PREFER_SRC_COA`
```rust
const IPV6_PREFER_SRC_COA: c_int = 4i32;
```

### `IPV6_PREFER_SRC_HOME`
```rust
const IPV6_PREFER_SRC_HOME: c_int = 1_024i32;
```

### `IPV6_PREFER_SRC_CGA`
```rust
const IPV6_PREFER_SRC_CGA: c_int = 8i32;
```

### `IPV6_PREFER_SRC_NONCGA`
```rust
const IPV6_PREFER_SRC_NONCGA: c_int = 2_048i32;
```

### `IPV6_PMTUDISC_DONT`
```rust
const IPV6_PMTUDISC_DONT: c_int = 0i32;
```

### `IPV6_PMTUDISC_WANT`
```rust
const IPV6_PMTUDISC_WANT: c_int = 1i32;
```

### `IPV6_PMTUDISC_DO`
```rust
const IPV6_PMTUDISC_DO: c_int = 2i32;
```

### `IPV6_PMTUDISC_PROBE`
```rust
const IPV6_PMTUDISC_PROBE: c_int = 3i32;
```

### `IPV6_PMTUDISC_INTERFACE`
```rust
const IPV6_PMTUDISC_INTERFACE: c_int = 4i32;
```

### `IPV6_PMTUDISC_OMIT`
```rust
const IPV6_PMTUDISC_OMIT: c_int = 5i32;
```

### `TCP_NODELAY`
```rust
const TCP_NODELAY: c_int = 1i32;
```

### `TCP_MAXSEG`
```rust
const TCP_MAXSEG: c_int = 2i32;
```

### `TCP_CORK`
```rust
const TCP_CORK: c_int = 3i32;
```

### `TCP_KEEPIDLE`
```rust
const TCP_KEEPIDLE: c_int = 4i32;
```

### `TCP_KEEPINTVL`
```rust
const TCP_KEEPINTVL: c_int = 5i32;
```

### `TCP_KEEPCNT`
```rust
const TCP_KEEPCNT: c_int = 6i32;
```

### `TCP_SYNCNT`
```rust
const TCP_SYNCNT: c_int = 7i32;
```

### `TCP_LINGER2`
```rust
const TCP_LINGER2: c_int = 8i32;
```

### `TCP_DEFER_ACCEPT`
```rust
const TCP_DEFER_ACCEPT: c_int = 9i32;
```

### `TCP_WINDOW_CLAMP`
```rust
const TCP_WINDOW_CLAMP: c_int = 10i32;
```

### `TCP_INFO`
```rust
const TCP_INFO: c_int = 11i32;
```

### `TCP_QUICKACK`
```rust
const TCP_QUICKACK: c_int = 12i32;
```

### `TCP_CONGESTION`
```rust
const TCP_CONGESTION: c_int = 13i32;
```

### `TCP_MD5SIG`
```rust
const TCP_MD5SIG: c_int = 14i32;
```

### `TCP_COOKIE_TRANSACTIONS`
```rust
const TCP_COOKIE_TRANSACTIONS: c_int = 15i32;
```

### `TCP_THIN_LINEAR_TIMEOUTS`
```rust
const TCP_THIN_LINEAR_TIMEOUTS: c_int = 16i32;
```

### `TCP_THIN_DUPACK`
```rust
const TCP_THIN_DUPACK: c_int = 17i32;
```

### `TCP_USER_TIMEOUT`
```rust
const TCP_USER_TIMEOUT: c_int = 18i32;
```

### `TCP_REPAIR`
```rust
const TCP_REPAIR: c_int = 19i32;
```

### `TCP_REPAIR_QUEUE`
```rust
const TCP_REPAIR_QUEUE: c_int = 20i32;
```

### `TCP_QUEUE_SEQ`
```rust
const TCP_QUEUE_SEQ: c_int = 21i32;
```

### `TCP_REPAIR_OPTIONS`
```rust
const TCP_REPAIR_OPTIONS: c_int = 22i32;
```

### `TCP_FASTOPEN`
```rust
const TCP_FASTOPEN: c_int = 23i32;
```

### `TCP_TIMESTAMP`
```rust
const TCP_TIMESTAMP: c_int = 24i32;
```

### `TCP_NOTSENT_LOWAT`
```rust
const TCP_NOTSENT_LOWAT: c_int = 25i32;
```

### `TCP_CC_INFO`
```rust
const TCP_CC_INFO: c_int = 26i32;
```

### `TCP_SAVE_SYN`
```rust
const TCP_SAVE_SYN: c_int = 27i32;
```

### `TCP_SAVED_SYN`
```rust
const TCP_SAVED_SYN: c_int = 28i32;
```

### `TCP_REPAIR_WINDOW`
```rust
const TCP_REPAIR_WINDOW: c_int = 29i32;
```

### `TCP_FASTOPEN_CONNECT`
```rust
const TCP_FASTOPEN_CONNECT: c_int = 30i32;
```

### `TCP_ULP`
```rust
const TCP_ULP: c_int = 31i32;
```

### `TCP_MD5SIG_EXT`
```rust
const TCP_MD5SIG_EXT: c_int = 32i32;
```

### `TCP_FASTOPEN_KEY`
```rust
const TCP_FASTOPEN_KEY: c_int = 33i32;
```

### `TCP_FASTOPEN_NO_COOKIE`
```rust
const TCP_FASTOPEN_NO_COOKIE: c_int = 34i32;
```

### `TCP_ZEROCOPY_RECEIVE`
```rust
const TCP_ZEROCOPY_RECEIVE: c_int = 35i32;
```

### `TCP_INQ`
```rust
const TCP_INQ: c_int = 36i32;
```

### `TCP_CM_INQ`
```rust
const TCP_CM_INQ: c_int = 36i32;
```

### `TCP_MD5SIG_MAXKEYLEN`
```rust
const TCP_MD5SIG_MAXKEYLEN: usize = 80usize;
```

### `SO_DEBUG`
```rust
const SO_DEBUG: c_int = 1i32;
```

### `SHUT_RD`
```rust
const SHUT_RD: c_int = 0i32;
```

### `SHUT_WR`
```rust
const SHUT_WR: c_int = 1i32;
```

### `SHUT_RDWR`
```rust
const SHUT_RDWR: c_int = 2i32;
```

### `LOCK_SH`
```rust
const LOCK_SH: c_int = 1i32;
```

### `LOCK_EX`
```rust
const LOCK_EX: c_int = 2i32;
```

### `LOCK_NB`
```rust
const LOCK_NB: c_int = 4i32;
```

### `LOCK_UN`
```rust
const LOCK_UN: c_int = 8i32;
```

### `SS_ONSTACK`
```rust
const SS_ONSTACK: c_int = 1i32;
```

### `SS_DISABLE`
```rust
const SS_DISABLE: c_int = 2i32;
```

### `NAME_MAX`
```rust
const NAME_MAX: c_int = 255i32;
```

### `PATH_MAX`
```rust
const PATH_MAX: c_int = 4_096i32;
```

### `UIO_MAXIOV`
```rust
const UIO_MAXIOV: c_int = 1_024i32;
```

### `FD_SETSIZE`
```rust
const FD_SETSIZE: usize = 1_024usize;
```

### `EPOLLIN`
```rust
const EPOLLIN: c_int = 1i32;
```

### `EPOLLPRI`
```rust
const EPOLLPRI: c_int = 2i32;
```

### `EPOLLOUT`
```rust
const EPOLLOUT: c_int = 4i32;
```

### `EPOLLERR`
```rust
const EPOLLERR: c_int = 8i32;
```

### `EPOLLHUP`
```rust
const EPOLLHUP: c_int = 16i32;
```

### `EPOLLRDNORM`
```rust
const EPOLLRDNORM: c_int = 64i32;
```

### `EPOLLRDBAND`
```rust
const EPOLLRDBAND: c_int = 128i32;
```

### `EPOLLWRNORM`
```rust
const EPOLLWRNORM: c_int = 256i32;
```

### `EPOLLWRBAND`
```rust
const EPOLLWRBAND: c_int = 512i32;
```

### `EPOLLMSG`
```rust
const EPOLLMSG: c_int = 1_024i32;
```

### `EPOLLRDHUP`
```rust
const EPOLLRDHUP: c_int = 8_192i32;
```

### `EPOLLEXCLUSIVE`
```rust
const EPOLLEXCLUSIVE: c_int = 268_435_456i32;
```

### `EPOLLWAKEUP`
```rust
const EPOLLWAKEUP: c_int = 536_870_912i32;
```

### `EPOLLONESHOT`
```rust
const EPOLLONESHOT: c_int = 1_073_741_824i32;
```

### `EPOLLET`
```rust
const EPOLLET: c_int = -2_147_483_648i32;
```

### `EPOLL_CTL_ADD`
```rust
const EPOLL_CTL_ADD: c_int = 1i32;
```

### `EPOLL_CTL_MOD`
```rust
const EPOLL_CTL_MOD: c_int = 3i32;
```

### `EPOLL_CTL_DEL`
```rust
const EPOLL_CTL_DEL: c_int = 2i32;
```

### `MNT_FORCE`
```rust
const MNT_FORCE: c_int = 1i32;
```

### `MNT_DETACH`
```rust
const MNT_DETACH: c_int = 2i32;
```

### `MNT_EXPIRE`
```rust
const MNT_EXPIRE: c_int = 4i32;
```

### `UMOUNT_NOFOLLOW`
```rust
const UMOUNT_NOFOLLOW: c_int = 8i32;
```

### `Q_GETFMT`
```rust
const Q_GETFMT: c_int = 8_388_612i32;
```

### `Q_GETINFO`
```rust
const Q_GETINFO: c_int = 8_388_613i32;
```

### `Q_SETINFO`
```rust
const Q_SETINFO: c_int = 8_388_614i32;
```

### `QIF_BLIMITS`
```rust
const QIF_BLIMITS: u32 = 1u32;
```

### `QIF_SPACE`
```rust
const QIF_SPACE: u32 = 2u32;
```

### `QIF_ILIMITS`
```rust
const QIF_ILIMITS: u32 = 4u32;
```

### `QIF_INODES`
```rust
const QIF_INODES: u32 = 8u32;
```

### `QIF_BTIME`
```rust
const QIF_BTIME: u32 = 16u32;
```

### `QIF_ITIME`
```rust
const QIF_ITIME: u32 = 32u32;
```

### `QIF_LIMITS`
```rust
const QIF_LIMITS: u32 = 5u32;
```

### `QIF_USAGE`
```rust
const QIF_USAGE: u32 = 10u32;
```

### `QIF_TIMES`
```rust
const QIF_TIMES: u32 = 48u32;
```

### `QIF_ALL`
```rust
const QIF_ALL: u32 = 63u32;
```

### `Q_SYNC`
```rust
const Q_SYNC: c_int = 8_388_609i32;
```

### `Q_QUOTAON`
```rust
const Q_QUOTAON: c_int = 8_388_610i32;
```

### `Q_QUOTAOFF`
```rust
const Q_QUOTAOFF: c_int = 8_388_611i32;
```

### `Q_GETQUOTA`
```rust
const Q_GETQUOTA: c_int = 8_388_615i32;
```

### `Q_SETQUOTA`
```rust
const Q_SETQUOTA: c_int = 8_388_616i32;
```

### `TCIOFF`
```rust
const TCIOFF: c_int = 2i32;
```

### `TCION`
```rust
const TCION: c_int = 3i32;
```

### `TCOOFF`
```rust
const TCOOFF: c_int = 0i32;
```

### `TCOON`
```rust
const TCOON: c_int = 1i32;
```

### `TCIFLUSH`
```rust
const TCIFLUSH: c_int = 0i32;
```

### `TCOFLUSH`
```rust
const TCOFLUSH: c_int = 1i32;
```

### `TCIOFLUSH`
```rust
const TCIOFLUSH: c_int = 2i32;
```

### `NL0`
```rust
const NL0: crate::tcflag_t = 0u32;
```

### `NL1`
```rust
const NL1: crate::tcflag_t = 256u32;
```

### `TAB0`
```rust
const TAB0: crate::tcflag_t = 0u32;
```

### `CR0`
```rust
const CR0: crate::tcflag_t = 0u32;
```

### `FF0`
```rust
const FF0: crate::tcflag_t = 0u32;
```

### `BS0`
```rust
const BS0: crate::tcflag_t = 0u32;
```

### `VT0`
```rust
const VT0: crate::tcflag_t = 0u32;
```

### `VERASE`
```rust
const VERASE: usize = 2usize;
```

### `VKILL`
```rust
const VKILL: usize = 3usize;
```

### `VINTR`
```rust
const VINTR: usize = 0usize;
```

### `VQUIT`
```rust
const VQUIT: usize = 1usize;
```

### `VLNEXT`
```rust
const VLNEXT: usize = 15usize;
```

### `IGNBRK`
```rust
const IGNBRK: crate::tcflag_t = 1u32;
```

### `BRKINT`
```rust
const BRKINT: crate::tcflag_t = 2u32;
```

### `IGNPAR`
```rust
const IGNPAR: crate::tcflag_t = 4u32;
```

### `PARMRK`
```rust
const PARMRK: crate::tcflag_t = 8u32;
```

### `INPCK`
```rust
const INPCK: crate::tcflag_t = 16u32;
```

### `ISTRIP`
```rust
const ISTRIP: crate::tcflag_t = 32u32;
```

### `INLCR`
```rust
const INLCR: crate::tcflag_t = 64u32;
```

### `IGNCR`
```rust
const IGNCR: crate::tcflag_t = 128u32;
```

### `ICRNL`
```rust
const ICRNL: crate::tcflag_t = 256u32;
```

### `IXANY`
```rust
const IXANY: crate::tcflag_t = 2_048u32;
```

### `IMAXBEL`
```rust
const IMAXBEL: crate::tcflag_t = 8_192u32;
```

### `OPOST`
```rust
const OPOST: crate::tcflag_t = 1u32;
```

### `CS5`
```rust
const CS5: crate::tcflag_t = 0u32;
```

### `CRTSCTS`
```rust
const CRTSCTS: crate::tcflag_t = 2_147_483_648u32;
```

### `ECHO`
```rust
const ECHO: crate::tcflag_t = 8u32;
```

### `OCRNL`
```rust
const OCRNL: crate::tcflag_t = 8u32;
```

### `ONOCR`
```rust
const ONOCR: crate::tcflag_t = 16u32;
```

### `ONLRET`
```rust
const ONLRET: crate::tcflag_t = 32u32;
```

### `OFILL`
```rust
const OFILL: crate::tcflag_t = 64u32;
```

### `OFDEL`
```rust
const OFDEL: crate::tcflag_t = 128u32;
```

### `CLONE_VM`
```rust
const CLONE_VM: c_int = 256i32;
```

### `CLONE_FS`
```rust
const CLONE_FS: c_int = 512i32;
```

### `CLONE_FILES`
```rust
const CLONE_FILES: c_int = 1_024i32;
```

### `CLONE_SIGHAND`
```rust
const CLONE_SIGHAND: c_int = 2_048i32;
```

### `CLONE_PTRACE`
```rust
const CLONE_PTRACE: c_int = 8_192i32;
```

### `CLONE_VFORK`
```rust
const CLONE_VFORK: c_int = 16_384i32;
```

### `CLONE_PARENT`
```rust
const CLONE_PARENT: c_int = 32_768i32;
```

### `CLONE_THREAD`
```rust
const CLONE_THREAD: c_int = 65_536i32;
```

### `CLONE_NEWNS`
```rust
const CLONE_NEWNS: c_int = 131_072i32;
```

### `CLONE_SYSVSEM`
```rust
const CLONE_SYSVSEM: c_int = 262_144i32;
```

### `CLONE_SETTLS`
```rust
const CLONE_SETTLS: c_int = 524_288i32;
```

### `CLONE_PARENT_SETTID`
```rust
const CLONE_PARENT_SETTID: c_int = 1_048_576i32;
```

### `CLONE_CHILD_CLEARTID`
```rust
const CLONE_CHILD_CLEARTID: c_int = 2_097_152i32;
```

### `CLONE_DETACHED`
```rust
const CLONE_DETACHED: c_int = 4_194_304i32;
```

### `CLONE_UNTRACED`
```rust
const CLONE_UNTRACED: c_int = 8_388_608i32;
```

### `CLONE_CHILD_SETTID`
```rust
const CLONE_CHILD_SETTID: c_int = 16_777_216i32;
```

### `CLONE_NEWCGROUP`
```rust
const CLONE_NEWCGROUP: c_int = 33_554_432i32;
```

### `CLONE_NEWUTS`
```rust
const CLONE_NEWUTS: c_int = 67_108_864i32;
```

### `CLONE_NEWIPC`
```rust
const CLONE_NEWIPC: c_int = 134_217_728i32;
```

### `CLONE_NEWUSER`
```rust
const CLONE_NEWUSER: c_int = 268_435_456i32;
```

### `CLONE_NEWPID`
```rust
const CLONE_NEWPID: c_int = 536_870_912i32;
```

### `CLONE_NEWNET`
```rust
const CLONE_NEWNET: c_int = 1_073_741_824i32;
```

### `CLONE_IO`
```rust
const CLONE_IO: c_int = -2_147_483_648i32;
```

### `WNOHANG`
```rust
const WNOHANG: c_int = 1i32;
```

### `WUNTRACED`
```rust
const WUNTRACED: c_int = 2i32;
```

### `WSTOPPED`
```rust
const WSTOPPED: c_int = 2i32;
```

### `WEXITED`
```rust
const WEXITED: c_int = 4i32;
```

### `WCONTINUED`
```rust
const WCONTINUED: c_int = 8i32;
```

### `WNOWAIT`
```rust
const WNOWAIT: c_int = 16_777_216i32;
```

### `ADDR_NO_RANDOMIZE`
```rust
const ADDR_NO_RANDOMIZE: c_int = 262_144i32;
```

### `MMAP_PAGE_ZERO`
```rust
const MMAP_PAGE_ZERO: c_int = 1_048_576i32;
```

### `ADDR_COMPAT_LAYOUT`
```rust
const ADDR_COMPAT_LAYOUT: c_int = 2_097_152i32;
```

### `READ_IMPLIES_EXEC`
```rust
const READ_IMPLIES_EXEC: c_int = 4_194_304i32;
```

### `ADDR_LIMIT_32BIT`
```rust
const ADDR_LIMIT_32BIT: c_int = 8_388_608i32;
```

### `SHORT_INODE`
```rust
const SHORT_INODE: c_int = 16_777_216i32;
```

### `WHOLE_SECONDS`
```rust
const WHOLE_SECONDS: c_int = 33_554_432i32;
```

### `STICKY_TIMEOUTS`
```rust
const STICKY_TIMEOUTS: c_int = 67_108_864i32;
```

### `ADDR_LIMIT_3GB`
```rust
const ADDR_LIMIT_3GB: c_int = 134_217_728i32;
```

### `PTRACE_O_TRACESYSGOOD`
```rust
const PTRACE_O_TRACESYSGOOD: c_int = 1i32;
```

### `PTRACE_O_TRACEFORK`
```rust
const PTRACE_O_TRACEFORK: c_int = 2i32;
```

### `PTRACE_O_TRACEVFORK`
```rust
const PTRACE_O_TRACEVFORK: c_int = 4i32;
```

### `PTRACE_O_TRACECLONE`
```rust
const PTRACE_O_TRACECLONE: c_int = 8i32;
```

### `PTRACE_O_TRACEEXEC`
```rust
const PTRACE_O_TRACEEXEC: c_int = 16i32;
```

### `PTRACE_O_TRACEVFORKDONE`
```rust
const PTRACE_O_TRACEVFORKDONE: c_int = 32i32;
```

### `PTRACE_O_TRACEEXIT`
```rust
const PTRACE_O_TRACEEXIT: c_int = 64i32;
```

### `PTRACE_O_TRACESECCOMP`
```rust
const PTRACE_O_TRACESECCOMP: c_int = 128i32;
```

### `PTRACE_O_SUSPEND_SECCOMP`
```rust
const PTRACE_O_SUSPEND_SECCOMP: c_int = 2_097_152i32;
```

### `PTRACE_O_EXITKILL`
```rust
const PTRACE_O_EXITKILL: c_int = 1_048_576i32;
```

### `PTRACE_O_MASK`
```rust
const PTRACE_O_MASK: c_int = 3_145_983i32;
```

### `PTRACE_EVENT_FORK`
```rust
const PTRACE_EVENT_FORK: c_int = 1i32;
```

### `PTRACE_EVENT_VFORK`
```rust
const PTRACE_EVENT_VFORK: c_int = 2i32;
```

### `PTRACE_EVENT_CLONE`
```rust
const PTRACE_EVENT_CLONE: c_int = 3i32;
```

### `PTRACE_EVENT_EXEC`
```rust
const PTRACE_EVENT_EXEC: c_int = 4i32;
```

### `PTRACE_EVENT_VFORK_DONE`
```rust
const PTRACE_EVENT_VFORK_DONE: c_int = 5i32;
```

### `PTRACE_EVENT_EXIT`
```rust
const PTRACE_EVENT_EXIT: c_int = 6i32;
```

### `PTRACE_EVENT_SECCOMP`
```rust
const PTRACE_EVENT_SECCOMP: c_int = 7i32;
```

### `__WNOTHREAD`
```rust
const __WNOTHREAD: c_int = 536_870_912i32;
```

### `__WALL`
```rust
const __WALL: c_int = 1_073_741_824i32;
```

### `__WCLONE`
```rust
const __WCLONE: c_int = -2_147_483_648i32;
```

### `SPLICE_F_MOVE`
```rust
const SPLICE_F_MOVE: c_uint = 1u32;
```

### `SPLICE_F_NONBLOCK`
```rust
const SPLICE_F_NONBLOCK: c_uint = 2u32;
```

### `SPLICE_F_MORE`
```rust
const SPLICE_F_MORE: c_uint = 4u32;
```

### `SPLICE_F_GIFT`
```rust
const SPLICE_F_GIFT: c_uint = 8u32;
```

### `RTLD_LOCAL`
```rust
const RTLD_LOCAL: c_int = 0i32;
```

### `RTLD_LAZY`
```rust
const RTLD_LAZY: c_int = 1i32;
```

### `POSIX_FADV_NORMAL`
```rust
const POSIX_FADV_NORMAL: c_int = 0i32;
```

### `POSIX_FADV_RANDOM`
```rust
const POSIX_FADV_RANDOM: c_int = 1i32;
```

### `POSIX_FADV_SEQUENTIAL`
```rust
const POSIX_FADV_SEQUENTIAL: c_int = 2i32;
```

### `POSIX_FADV_WILLNEED`
```rust
const POSIX_FADV_WILLNEED: c_int = 3i32;
```

### `AT_FDCWD`
```rust
const AT_FDCWD: c_int = -100i32;
```

### `AT_SYMLINK_NOFOLLOW`
```rust
const AT_SYMLINK_NOFOLLOW: c_int = 256i32;
```

### `AT_REMOVEDIR`
```rust
const AT_REMOVEDIR: c_int = 512i32;
```

### `AT_SYMLINK_FOLLOW`
```rust
const AT_SYMLINK_FOLLOW: c_int = 1_024i32;
```

### `AT_NO_AUTOMOUNT`
```rust
const AT_NO_AUTOMOUNT: c_int = 2_048i32;
```

### `AT_EMPTY_PATH`
```rust
const AT_EMPTY_PATH: c_int = 4_096i32;
```

### `AT_RECURSIVE`
```rust
const AT_RECURSIVE: c_int = 32_768i32;
```

### `LOG_CRON`
```rust
const LOG_CRON: c_int = 72i32;
```

### `LOG_AUTHPRIV`
```rust
const LOG_AUTHPRIV: c_int = 80i32;
```

### `LOG_FTP`
```rust
const LOG_FTP: c_int = 88i32;
```

### `LOG_PERROR`
```rust
const LOG_PERROR: c_int = 32i32;
```

### `PIPE_BUF`
```rust
const PIPE_BUF: usize = 4_096usize;
```

### `SI_LOAD_SHIFT`
```rust
const SI_LOAD_SHIFT: c_uint = 16u32;
```

### `SI_USER`
```rust
const SI_USER: c_int = 0i32;
```

### `SI_KERNEL`
```rust
const SI_KERNEL: c_int = 128i32;
```

### `SI_QUEUE`
```rust
const SI_QUEUE: c_int = -1i32;
```

### `SI_TIMER`
```rust
const SI_TIMER: c_int = -2i32;
```

### `SI_MESGQ`
```rust
const SI_MESGQ: c_int = -3i32;
```

### `SI_ASYNCIO`
```rust
const SI_ASYNCIO: c_int = -4i32;
```

### `SI_SIGIO`
```rust
const SI_SIGIO: c_int = -5i32;
```

### `SI_TKILL`
```rust
const SI_TKILL: c_int = -6i32;
```

### `SI_ASYNCNL`
```rust
const SI_ASYNCNL: c_int = -60i32;
```

### `BUS_ADRALN`
```rust
const BUS_ADRALN: c_int = 1i32;
```

### `BUS_ADRERR`
```rust
const BUS_ADRERR: c_int = 2i32;
```

### `BUS_OBJERR`
```rust
const BUS_OBJERR: c_int = 3i32;
```

### `BUS_MCEERR_AR`
```rust
const BUS_MCEERR_AR: c_int = 4i32;
```

### `BUS_MCEERR_AO`
```rust
const BUS_MCEERR_AO: c_int = 5i32;
```

### `TRAP_BRKPT`
```rust
const TRAP_BRKPT: c_int = 1i32;
```

### `TRAP_TRACE`
```rust
const TRAP_TRACE: c_int = 2i32;
```

### `TRAP_BRANCH`
```rust
const TRAP_BRANCH: c_int = 3i32;
```

### `TRAP_HWBKPT`
```rust
const TRAP_HWBKPT: c_int = 4i32;
```

### `TRAP_UNK`
```rust
const TRAP_UNK: c_int = 5i32;
```

### `CLD_EXITED`
```rust
const CLD_EXITED: c_int = 1i32;
```

### `CLD_KILLED`
```rust
const CLD_KILLED: c_int = 2i32;
```

### `CLD_DUMPED`
```rust
const CLD_DUMPED: c_int = 3i32;
```

### `CLD_TRAPPED`
```rust
const CLD_TRAPPED: c_int = 4i32;
```

### `CLD_STOPPED`
```rust
const CLD_STOPPED: c_int = 5i32;
```

### `CLD_CONTINUED`
```rust
const CLD_CONTINUED: c_int = 6i32;
```

### `SIGEV_SIGNAL`
```rust
const SIGEV_SIGNAL: c_int = 0i32;
```

### `SIGEV_NONE`
```rust
const SIGEV_NONE: c_int = 1i32;
```

### `SIGEV_THREAD`
```rust
const SIGEV_THREAD: c_int = 2i32;
```

### `P_ALL`
```rust
const P_ALL: idtype_t = 0u32;
```

### `P_PID`
```rust
const P_PID: idtype_t = 1u32;
```

### `P_PGID`
```rust
const P_PGID: idtype_t = 2u32;
```

### `P_PIDFD`
```rust
const P_PIDFD: idtype_t = 3u32;
```

### `UTIME_OMIT`
```rust
const UTIME_OMIT: c_long = 1_073_741_822i64;
```

### `UTIME_NOW`
```rust
const UTIME_NOW: c_long = 1_073_741_823i64;
```

### `POLLIN`
```rust
const POLLIN: c_short = 1i16;
```

### `POLLPRI`
```rust
const POLLPRI: c_short = 2i16;
```

### `POLLOUT`
```rust
const POLLOUT: c_short = 4i16;
```

### `POLLERR`
```rust
const POLLERR: c_short = 8i16;
```

### `POLLHUP`
```rust
const POLLHUP: c_short = 16i16;
```

### `POLLNVAL`
```rust
const POLLNVAL: c_short = 32i16;
```

### `POLLRDNORM`
```rust
const POLLRDNORM: c_short = 64i16;
```

### `POLLRDBAND`
```rust
const POLLRDBAND: c_short = 128i16;
```

### `POLLRDHUP`
```rust
const POLLRDHUP: c_short = 8_192i16;
```

### `IPTOS_LOWDELAY`
```rust
const IPTOS_LOWDELAY: u8 = 16u8;
```

### `IPTOS_THROUGHPUT`
```rust
const IPTOS_THROUGHPUT: u8 = 8u8;
```

### `IPTOS_RELIABILITY`
```rust
const IPTOS_RELIABILITY: u8 = 4u8;
```

### `IPTOS_MINCOST`
```rust
const IPTOS_MINCOST: u8 = 2u8;
```

### `IPTOS_PREC_NETCONTROL`
```rust
const IPTOS_PREC_NETCONTROL: u8 = 224u8;
```

### `IPTOS_PREC_INTERNETCONTROL`
```rust
const IPTOS_PREC_INTERNETCONTROL: u8 = 192u8;
```

### `IPTOS_PREC_CRITIC_ECP`
```rust
const IPTOS_PREC_CRITIC_ECP: u8 = 160u8;
```

### `IPTOS_PREC_FLASHOVERRIDE`
```rust
const IPTOS_PREC_FLASHOVERRIDE: u8 = 128u8;
```

### `IPTOS_PREC_FLASH`
```rust
const IPTOS_PREC_FLASH: u8 = 96u8;
```

### `IPTOS_PREC_IMMEDIATE`
```rust
const IPTOS_PREC_IMMEDIATE: u8 = 64u8;
```

### `IPTOS_PREC_PRIORITY`
```rust
const IPTOS_PREC_PRIORITY: u8 = 32u8;
```

### `IPTOS_PREC_ROUTINE`
```rust
const IPTOS_PREC_ROUTINE: u8 = 0u8;
```

### `IPTOS_ECN_MASK`
```rust
const IPTOS_ECN_MASK: u8 = 3u8;
```

### `IPTOS_ECN_ECT1`
```rust
const IPTOS_ECN_ECT1: u8 = 1u8;
```

### `IPTOS_ECN_ECT0`
```rust
const IPTOS_ECN_ECT0: u8 = 2u8;
```

### `IPTOS_ECN_CE`
```rust
const IPTOS_ECN_CE: u8 = 3u8;
```

### `IPOPT_COPY`
```rust
const IPOPT_COPY: u8 = 128u8;
```

### `IPOPT_CLASS_MASK`
```rust
const IPOPT_CLASS_MASK: u8 = 96u8;
```

### `IPOPT_NUMBER_MASK`
```rust
const IPOPT_NUMBER_MASK: u8 = 31u8;
```

### `IPOPT_CONTROL`
```rust
const IPOPT_CONTROL: u8 = 0u8;
```

### `IPOPT_RESERVED1`
```rust
const IPOPT_RESERVED1: u8 = 32u8;
```

### `IPOPT_MEASUREMENT`
```rust
const IPOPT_MEASUREMENT: u8 = 64u8;
```

### `IPOPT_RESERVED2`
```rust
const IPOPT_RESERVED2: u8 = 96u8;
```

### `IPOPT_END`
```rust
const IPOPT_END: u8 = 0u8;
```

### `IPOPT_NOOP`
```rust
const IPOPT_NOOP: u8 = 1u8;
```

### `IPOPT_SEC`
```rust
const IPOPT_SEC: u8 = 130u8;
```

### `IPOPT_LSRR`
```rust
const IPOPT_LSRR: u8 = 131u8;
```

### `IPOPT_TIMESTAMP`
```rust
const IPOPT_TIMESTAMP: u8 = 68u8;
```

### `IPOPT_RR`
```rust
const IPOPT_RR: u8 = 7u8;
```

### `IPOPT_SID`
```rust
const IPOPT_SID: u8 = 136u8;
```

### `IPOPT_SSRR`
```rust
const IPOPT_SSRR: u8 = 137u8;
```

### `IPOPT_RA`
```rust
const IPOPT_RA: u8 = 148u8;
```

### `IPVERSION`
```rust
const IPVERSION: u8 = 4u8;
```

### `MAXTTL`
```rust
const MAXTTL: u8 = 255u8;
```

### `IPDEFTTL`
```rust
const IPDEFTTL: u8 = 64u8;
```

### `IPOPT_OPTVAL`
```rust
const IPOPT_OPTVAL: u8 = 0u8;
```

### `IPOPT_OLEN`
```rust
const IPOPT_OLEN: u8 = 1u8;
```

### `IPOPT_OFFSET`
```rust
const IPOPT_OFFSET: u8 = 2u8;
```

### `IPOPT_MINOFF`
```rust
const IPOPT_MINOFF: u8 = 4u8;
```

### `MAX_IPOPTLEN`
```rust
const MAX_IPOPTLEN: u8 = 40u8;
```

### `IPOPT_NOP`
```rust
const IPOPT_NOP: u8 = 1u8;
```

### `IPOPT_EOL`
```rust
const IPOPT_EOL: u8 = 0u8;
```

### `IPOPT_TS`
```rust
const IPOPT_TS: u8 = 68u8;
```

### `IPOPT_TS_TSONLY`
```rust
const IPOPT_TS_TSONLY: u8 = 0u8;
```

### `IPOPT_TS_TSANDADDR`
```rust
const IPOPT_TS_TSANDADDR: u8 = 1u8;
```

### `IPOPT_TS_PRESPEC`
```rust
const IPOPT_TS_PRESPEC: u8 = 3u8;
```

### `ARPOP_RREQUEST`
```rust
const ARPOP_RREQUEST: u16 = 3u16;
```

### `ARPOP_RREPLY`
```rust
const ARPOP_RREPLY: u16 = 4u16;
```

### `ARPOP_InREQUEST`
```rust
const ARPOP_InREQUEST: u16 = 8u16;
```

### `ARPOP_InREPLY`
```rust
const ARPOP_InREPLY: u16 = 9u16;
```

### `ARPOP_NAK`
```rust
const ARPOP_NAK: u16 = 10u16;
```

### `ATF_NETMASK`
```rust
const ATF_NETMASK: c_int = 32i32;
```

### `ATF_DONTPUB`
```rust
const ATF_DONTPUB: c_int = 64i32;
```

### `ARPHRD_NETROM`
```rust
const ARPHRD_NETROM: u16 = 0u16;
```

### `ARPHRD_ETHER`
```rust
const ARPHRD_ETHER: u16 = 1u16;
```

### `ARPHRD_EETHER`
```rust
const ARPHRD_EETHER: u16 = 2u16;
```

### `ARPHRD_AX25`
```rust
const ARPHRD_AX25: u16 = 3u16;
```

### `ARPHRD_PRONET`
```rust
const ARPHRD_PRONET: u16 = 4u16;
```

### `ARPHRD_CHAOS`
```rust
const ARPHRD_CHAOS: u16 = 5u16;
```

### `ARPHRD_IEEE802`
```rust
const ARPHRD_IEEE802: u16 = 6u16;
```

### `ARPHRD_ARCNET`
```rust
const ARPHRD_ARCNET: u16 = 7u16;
```

### `ARPHRD_APPLETLK`
```rust
const ARPHRD_APPLETLK: u16 = 8u16;
```

### `ARPHRD_DLCI`
```rust
const ARPHRD_DLCI: u16 = 15u16;
```

### `ARPHRD_ATM`
```rust
const ARPHRD_ATM: u16 = 19u16;
```

### `ARPHRD_METRICOM`
```rust
const ARPHRD_METRICOM: u16 = 23u16;
```

### `ARPHRD_IEEE1394`
```rust
const ARPHRD_IEEE1394: u16 = 24u16;
```

### `ARPHRD_EUI64`
```rust
const ARPHRD_EUI64: u16 = 27u16;
```

### `ARPHRD_INFINIBAND`
```rust
const ARPHRD_INFINIBAND: u16 = 32u16;
```

### `ARPHRD_SLIP`
```rust
const ARPHRD_SLIP: u16 = 256u16;
```

### `ARPHRD_CSLIP`
```rust
const ARPHRD_CSLIP: u16 = 257u16;
```

### `ARPHRD_SLIP6`
```rust
const ARPHRD_SLIP6: u16 = 258u16;
```

### `ARPHRD_CSLIP6`
```rust
const ARPHRD_CSLIP6: u16 = 259u16;
```

### `ARPHRD_RSRVD`
```rust
const ARPHRD_RSRVD: u16 = 260u16;
```

### `ARPHRD_ADAPT`
```rust
const ARPHRD_ADAPT: u16 = 264u16;
```

### `ARPHRD_ROSE`
```rust
const ARPHRD_ROSE: u16 = 270u16;
```

### `ARPHRD_X25`
```rust
const ARPHRD_X25: u16 = 271u16;
```

### `ARPHRD_HWX25`
```rust
const ARPHRD_HWX25: u16 = 272u16;
```

### `ARPHRD_CAN`
```rust
const ARPHRD_CAN: u16 = 280u16;
```

### `ARPHRD_PPP`
```rust
const ARPHRD_PPP: u16 = 512u16;
```

### `ARPHRD_CISCO`
```rust
const ARPHRD_CISCO: u16 = 513u16;
```

### `ARPHRD_HDLC`
```rust
const ARPHRD_HDLC: u16 = 513u16;
```

### `ARPHRD_LAPB`
```rust
const ARPHRD_LAPB: u16 = 516u16;
```

### `ARPHRD_DDCMP`
```rust
const ARPHRD_DDCMP: u16 = 517u16;
```

### `ARPHRD_RAWHDLC`
```rust
const ARPHRD_RAWHDLC: u16 = 518u16;
```

### `ARPHRD_TUNNEL`
```rust
const ARPHRD_TUNNEL: u16 = 768u16;
```

### `ARPHRD_TUNNEL6`
```rust
const ARPHRD_TUNNEL6: u16 = 769u16;
```

### `ARPHRD_FRAD`
```rust
const ARPHRD_FRAD: u16 = 770u16;
```

### `ARPHRD_SKIP`
```rust
const ARPHRD_SKIP: u16 = 771u16;
```

### `ARPHRD_LOOPBACK`
```rust
const ARPHRD_LOOPBACK: u16 = 772u16;
```

### `ARPHRD_LOCALTLK`
```rust
const ARPHRD_LOCALTLK: u16 = 773u16;
```

### `ARPHRD_FDDI`
```rust
const ARPHRD_FDDI: u16 = 774u16;
```

### `ARPHRD_BIF`
```rust
const ARPHRD_BIF: u16 = 775u16;
```

### `ARPHRD_SIT`
```rust
const ARPHRD_SIT: u16 = 776u16;
```

### `ARPHRD_IPDDP`
```rust
const ARPHRD_IPDDP: u16 = 777u16;
```

### `ARPHRD_IPGRE`
```rust
const ARPHRD_IPGRE: u16 = 778u16;
```

### `ARPHRD_PIMREG`
```rust
const ARPHRD_PIMREG: u16 = 779u16;
```

### `ARPHRD_HIPPI`
```rust
const ARPHRD_HIPPI: u16 = 780u16;
```

### `ARPHRD_ASH`
```rust
const ARPHRD_ASH: u16 = 781u16;
```

### `ARPHRD_ECONET`
```rust
const ARPHRD_ECONET: u16 = 782u16;
```

### `ARPHRD_IRDA`
```rust
const ARPHRD_IRDA: u16 = 783u16;
```

### `ARPHRD_FCPP`
```rust
const ARPHRD_FCPP: u16 = 784u16;
```

### `ARPHRD_FCAL`
```rust
const ARPHRD_FCAL: u16 = 785u16;
```

### `ARPHRD_FCPL`
```rust
const ARPHRD_FCPL: u16 = 786u16;
```

### `ARPHRD_FCFABRIC`
```rust
const ARPHRD_FCFABRIC: u16 = 787u16;
```

### `ARPHRD_IEEE802_TR`
```rust
const ARPHRD_IEEE802_TR: u16 = 800u16;
```

### `ARPHRD_IEEE80211`
```rust
const ARPHRD_IEEE80211: u16 = 801u16;
```

### `ARPHRD_IEEE80211_PRISM`
```rust
const ARPHRD_IEEE80211_PRISM: u16 = 802u16;
```

### `ARPHRD_IEEE80211_RADIOTAP`
```rust
const ARPHRD_IEEE80211_RADIOTAP: u16 = 803u16;
```

### `ARPHRD_IEEE802154`
```rust
const ARPHRD_IEEE802154: u16 = 804u16;
```

### `ARPHRD_VOID`
```rust
const ARPHRD_VOID: u16 = 65_535u16;
```

### `ARPHRD_NONE`
```rust
const ARPHRD_NONE: u16 = 65_534u16;
```

### `IFF_TUN`
```rust
const IFF_TUN: c_int = 1i32;
```

### `IFF_TAP`
```rust
const IFF_TAP: c_int = 2i32;
```

### `IFF_NAPI`
```rust
const IFF_NAPI: c_int = 16i32;
```

### `IFF_NAPI_FRAGS`
```rust
const IFF_NAPI_FRAGS: c_int = 32i32;
```

### `IFF_NO_CARRIER`
```rust
const IFF_NO_CARRIER: c_int = 64i32;
```

### `IFF_NO_PI`
```rust
const IFF_NO_PI: c_int = 4_096i32;
```

### `TUN_READQ_SIZE`
```rust
const TUN_READQ_SIZE: c_short = 500i16;
```

### `TUN_TUN_DEV`
```rust
const TUN_TUN_DEV: c_short = 1i16;
```

### `TUN_TAP_DEV`
```rust
const TUN_TAP_DEV: c_short = 2i16;
```

### `TUN_TYPE_MASK`
```rust
const TUN_TYPE_MASK: c_short = 15i16;
```

### `IFF_ONE_QUEUE`
```rust
const IFF_ONE_QUEUE: c_int = 8_192i32;
```

### `IFF_VNET_HDR`
```rust
const IFF_VNET_HDR: c_int = 16_384i32;
```

### `IFF_TUN_EXCL`
```rust
const IFF_TUN_EXCL: c_int = 32_768i32;
```

### `IFF_MULTI_QUEUE`
```rust
const IFF_MULTI_QUEUE: c_int = 256i32;
```

### `IFF_ATTACH_QUEUE`
```rust
const IFF_ATTACH_QUEUE: c_int = 512i32;
```

### `IFF_DETACH_QUEUE`
```rust
const IFF_DETACH_QUEUE: c_int = 1_024i32;
```

### `IFF_PERSIST`
```rust
const IFF_PERSIST: c_int = 2_048i32;
```

### `IFF_NOFILTER`
```rust
const IFF_NOFILTER: c_int = 4_096i32;
```

### `TUN_TX_TIMESTAMP`
```rust
const TUN_TX_TIMESTAMP: c_int = 1i32;
```

### `TUN_F_CSUM`
```rust
const TUN_F_CSUM: c_uint = 1u32;
```

### `TUN_F_TSO4`
```rust
const TUN_F_TSO4: c_uint = 2u32;
```

### `TUN_F_TSO6`
```rust
const TUN_F_TSO6: c_uint = 4u32;
```

### `TUN_F_TSO_ECN`
```rust
const TUN_F_TSO_ECN: c_uint = 8u32;
```

### `TUN_F_UFO`
```rust
const TUN_F_UFO: c_uint = 16u32;
```

### `TUN_F_USO4`
```rust
const TUN_F_USO4: c_uint = 32u32;
```

### `TUN_F_USO6`
```rust
const TUN_F_USO6: c_uint = 64u32;
```

### `TUN_PKT_STRIP`
```rust
const TUN_PKT_STRIP: c_int = 1i32;
```

### `TUN_FLT_ALLMULTI`
```rust
const TUN_FLT_ALLMULTI: c_int = 1i32;
```

### `T_TYPE`
```rust
const T_TYPE: u32 = 84u32;
```

### `TUNSETNOCSUM`
```rust
const TUNSETNOCSUM: c_ulong = 1_074_025_672u64;
```

### `TUNSETDEBUG`
```rust
const TUNSETDEBUG: c_ulong = 1_074_025_673u64;
```

### `TUNSETIFF`
```rust
const TUNSETIFF: c_ulong = 1_074_025_674u64;
```

### `TUNSETPERSIST`
```rust
const TUNSETPERSIST: c_ulong = 1_074_025_675u64;
```

### `TUNSETOWNER`
```rust
const TUNSETOWNER: c_ulong = 1_074_025_676u64;
```

### `TUNSETLINK`
```rust
const TUNSETLINK: c_ulong = 1_074_025_677u64;
```

### `TUNSETGROUP`
```rust
const TUNSETGROUP: c_ulong = 1_074_025_678u64;
```

### `TUNGETFEATURES`
```rust
const TUNGETFEATURES: c_ulong = 2_147_767_503u64;
```

### `TUNSETOFFLOAD`
```rust
const TUNSETOFFLOAD: c_ulong = 1_074_025_680u64;
```

### `TUNSETTXFILTER`
```rust
const TUNSETTXFILTER: c_ulong = 1_074_025_681u64;
```

### `TUNGETIFF`
```rust
const TUNGETIFF: c_ulong = 2_147_767_506u64;
```

### `TUNGETSNDBUF`
```rust
const TUNGETSNDBUF: c_ulong = 2_147_767_507u64;
```

### `TUNSETSNDBUF`
```rust
const TUNSETSNDBUF: c_ulong = 1_074_025_684u64;
```

### `TUNATTACHFILTER`
```rust
const TUNATTACHFILTER: c_ulong = 1_074_812_117u64;
```

### `TUNDETACHFILTER`
```rust
const TUNDETACHFILTER: c_ulong = 1_074_812_118u64;
```

### `TUNGETVNETHDRSZ`
```rust
const TUNGETVNETHDRSZ: c_ulong = 2_147_767_511u64;
```

### `TUNSETVNETHDRSZ`
```rust
const TUNSETVNETHDRSZ: c_ulong = 1_074_025_688u64;
```

### `TUNSETQUEUE`
```rust
const TUNSETQUEUE: c_ulong = 1_074_025_689u64;
```

### `TUNSETIFINDEX`
```rust
const TUNSETIFINDEX: c_ulong = 1_074_025_690u64;
```

### `TUNGETFILTER`
```rust
const TUNGETFILTER: c_ulong = 2_148_553_947u64;
```

### `TUNSETVNETLE`
```rust
const TUNSETVNETLE: c_ulong = 1_074_025_692u64;
```

### `TUNGETVNETLE`
```rust
const TUNGETVNETLE: c_ulong = 2_147_767_517u64;
```

### `TUNSETVNETBE`
```rust
const TUNSETVNETBE: c_ulong = 1_074_025_694u64;
```

### `TUNGETVNETBE`
```rust
const TUNGETVNETBE: c_ulong = 2_147_767_519u64;
```

### `TUNSETSTEERINGEBPF`
```rust
const TUNSETSTEERINGEBPF: c_ulong = 2_147_767_520u64;
```

### `TUNSETFILTEREBPF`
```rust
const TUNSETFILTEREBPF: c_ulong = 2_147_767_521u64;
```

### `TUNSETCARRIER`
```rust
const TUNSETCARRIER: c_ulong = 1_074_025_698u64;
```

### `TUNGETDEVNETNS`
```rust
const TUNGETDEVNETNS: c_ulong = 21_731u64;
```

### `FS_IOC_GETFLAGS`
```rust
const FS_IOC_GETFLAGS: c_ulong = 2_148_034_049u64;
```

### `FS_IOC_SETFLAGS`
```rust
const FS_IOC_SETFLAGS: c_ulong = 1_074_292_226u64;
```

### `FS_IOC_GETVERSION`
```rust
const FS_IOC_GETVERSION: c_ulong = 2_148_038_145u64;
```

### `FS_IOC_SETVERSION`
```rust
const FS_IOC_SETVERSION: c_ulong = 1_074_296_322u64;
```

### `FS_IOC32_GETFLAGS`
```rust
const FS_IOC32_GETFLAGS: c_ulong = 2_147_771_905u64;
```

### `FS_IOC32_SETFLAGS`
```rust
const FS_IOC32_SETFLAGS: c_ulong = 1_074_030_082u64;
```

### `FS_IOC32_GETVERSION`
```rust
const FS_IOC32_GETVERSION: c_ulong = 2_147_776_001u64;
```

### `FS_IOC32_SETVERSION`
```rust
const FS_IOC32_SETVERSION: c_ulong = 1_074_034_178u64;
```

### `FICLONE`
```rust
const FICLONE: c_ulong = 1_074_041_865u64;
```

### `FICLONERANGE`
```rust
const FICLONERANGE: c_ulong = 1_075_876_877u64;
```

### `ADFS_SUPER_MAGIC`
```rust
const ADFS_SUPER_MAGIC: c_long = 44_533i64;
```

### `AFFS_SUPER_MAGIC`
```rust
const AFFS_SUPER_MAGIC: c_long = 44_543i64;
```

### `AFS_SUPER_MAGIC`
```rust
const AFS_SUPER_MAGIC: c_long = 1_397_113_167i64;
```

### `AUTOFS_SUPER_MAGIC`
```rust
const AUTOFS_SUPER_MAGIC: c_long = 391i64;
```

### `BPF_FS_MAGIC`
```rust
const BPF_FS_MAGIC: c_long = 3_405_662_737i64;
```

### `BTRFS_SUPER_MAGIC`
```rust
const BTRFS_SUPER_MAGIC: c_long = 2_435_016_766i64;
```

### `CGROUP2_SUPER_MAGIC`
```rust
const CGROUP2_SUPER_MAGIC: c_long = 1_667_723_888i64;
```

### `CGROUP_SUPER_MAGIC`
```rust
const CGROUP_SUPER_MAGIC: c_long = 2_613_483i64;
```

### `CODA_SUPER_MAGIC`
```rust
const CODA_SUPER_MAGIC: c_long = 1_937_076_805i64;
```

### `CRAMFS_MAGIC`
```rust
const CRAMFS_MAGIC: c_long = 684_539_205i64;
```

### `DEBUGFS_MAGIC`
```rust
const DEBUGFS_MAGIC: c_long = 1_684_170_528i64;
```

### `DEVPTS_SUPER_MAGIC`
```rust
const DEVPTS_SUPER_MAGIC: c_long = 7_377i64;
```

### `ECRYPTFS_SUPER_MAGIC`
```rust
const ECRYPTFS_SUPER_MAGIC: c_long = 61_791i64;
```

### `EFS_SUPER_MAGIC`
```rust
const EFS_SUPER_MAGIC: c_long = 4_278_867i64;
```

### `EXT2_SUPER_MAGIC`
```rust
const EXT2_SUPER_MAGIC: c_long = 61_267i64;
```

### `EXT3_SUPER_MAGIC`
```rust
const EXT3_SUPER_MAGIC: c_long = 61_267i64;
```

### `EXT4_SUPER_MAGIC`
```rust
const EXT4_SUPER_MAGIC: c_long = 61_267i64;
```

### `F2FS_SUPER_MAGIC`
```rust
const F2FS_SUPER_MAGIC: c_long = 4_076_150_800i64;
```

### `FUSE_SUPER_MAGIC`
```rust
const FUSE_SUPER_MAGIC: c_long = 1_702_057_286i64;
```

### `FUTEXFS_SUPER_MAGIC`
```rust
const FUTEXFS_SUPER_MAGIC: c_long = 195_894_762i64;
```

### `HOSTFS_SUPER_MAGIC`
```rust
const HOSTFS_SUPER_MAGIC: c_long = 12_648_430i64;
```

### `HPFS_SUPER_MAGIC`
```rust
const HPFS_SUPER_MAGIC: c_long = 4_187_351_113i64;
```

### `HUGETLBFS_MAGIC`
```rust
const HUGETLBFS_MAGIC: c_long = 2_508_478_710i64;
```

### `ISOFS_SUPER_MAGIC`
```rust
const ISOFS_SUPER_MAGIC: c_long = 38_496i64;
```

### `JFFS2_SUPER_MAGIC`
```rust
const JFFS2_SUPER_MAGIC: c_long = 29_366i64;
```

### `MINIX2_SUPER_MAGIC2`
```rust
const MINIX2_SUPER_MAGIC2: c_long = 9_336i64;
```

### `MINIX2_SUPER_MAGIC`
```rust
const MINIX2_SUPER_MAGIC: c_long = 9_320i64;
```

### `MINIX3_SUPER_MAGIC`
```rust
const MINIX3_SUPER_MAGIC: c_long = 19_802i64;
```

### `MINIX_SUPER_MAGIC2`
```rust
const MINIX_SUPER_MAGIC2: c_long = 5_007i64;
```

### `MINIX_SUPER_MAGIC`
```rust
const MINIX_SUPER_MAGIC: c_long = 4_991i64;
```

### `MSDOS_SUPER_MAGIC`
```rust
const MSDOS_SUPER_MAGIC: c_long = 19_780i64;
```

### `NCP_SUPER_MAGIC`
```rust
const NCP_SUPER_MAGIC: c_long = 22_092i64;
```

### `NFS_SUPER_MAGIC`
```rust
const NFS_SUPER_MAGIC: c_long = 26_985i64;
```

### `NILFS_SUPER_MAGIC`
```rust
const NILFS_SUPER_MAGIC: c_long = 13_364i64;
```

### `OCFS2_SUPER_MAGIC`
```rust
const OCFS2_SUPER_MAGIC: c_long = 1_952_539_503i64;
```

### `OPENPROM_SUPER_MAGIC`
```rust
const OPENPROM_SUPER_MAGIC: c_long = 40_865i64;
```

### `OVERLAYFS_SUPER_MAGIC`
```rust
const OVERLAYFS_SUPER_MAGIC: c_long = 2_035_054_128i64;
```

### `PROC_SUPER_MAGIC`
```rust
const PROC_SUPER_MAGIC: c_long = 40_864i64;
```

### `QNX4_SUPER_MAGIC`
```rust
const QNX4_SUPER_MAGIC: c_long = 47i64;
```

### `QNX6_SUPER_MAGIC`
```rust
const QNX6_SUPER_MAGIC: c_long = 1_746_473_250i64;
```

### `RDTGROUP_SUPER_MAGIC`
```rust
const RDTGROUP_SUPER_MAGIC: c_long = 124_082_209i64;
```

### `REISERFS_SUPER_MAGIC`
```rust
const REISERFS_SUPER_MAGIC: c_long = 1_382_369_651i64;
```

### `SECURITYFS_MAGIC`
```rust
const SECURITYFS_MAGIC: c_long = 1_935_894_131i64;
```

### `SELINUX_MAGIC`
```rust
const SELINUX_MAGIC: c_long = 4_185_718_668i64;
```

### `SMACK_MAGIC`
```rust
const SMACK_MAGIC: c_long = 1_128_357_203i64;
```

### `SMB_SUPER_MAGIC`
```rust
const SMB_SUPER_MAGIC: c_long = 20_859i64;
```

### `SYSFS_MAGIC`
```rust
const SYSFS_MAGIC: c_long = 1_650_812_274i64;
```

### `TMPFS_MAGIC`
```rust
const TMPFS_MAGIC: c_long = 16_914_836i64;
```

### `TRACEFS_MAGIC`
```rust
const TRACEFS_MAGIC: c_long = 1_953_653_091i64;
```

### `UDF_SUPER_MAGIC`
```rust
const UDF_SUPER_MAGIC: c_long = 352_400_198i64;
```

### `USBDEVICE_SUPER_MAGIC`
```rust
const USBDEVICE_SUPER_MAGIC: c_long = 40_866i64;
```

### `XENFS_SUPER_MAGIC`
```rust
const XENFS_SUPER_MAGIC: c_long = 2_881_100_148i64;
```

### `NSFS_MAGIC`
```rust
const NSFS_MAGIC: c_long = 1_853_056_627i64;
```

### `AT_STATX_SYNC_TYPE`
```rust
const AT_STATX_SYNC_TYPE: c_int = 24_576i32;
```

### `AT_STATX_SYNC_AS_STAT`
```rust
const AT_STATX_SYNC_AS_STAT: c_int = 0i32;
```

### `AT_STATX_FORCE_SYNC`
```rust
const AT_STATX_FORCE_SYNC: c_int = 8_192i32;
```

### `AT_STATX_DONT_SYNC`
```rust
const AT_STATX_DONT_SYNC: c_int = 16_384i32;
```

### `STATX_TYPE`
```rust
const STATX_TYPE: c_uint = 1u32;
```

### `STATX_MODE`
```rust
const STATX_MODE: c_uint = 2u32;
```

### `STATX_NLINK`
```rust
const STATX_NLINK: c_uint = 4u32;
```

### `STATX_UID`
```rust
const STATX_UID: c_uint = 8u32;
```

### `STATX_GID`
```rust
const STATX_GID: c_uint = 16u32;
```

### `STATX_ATIME`
```rust
const STATX_ATIME: c_uint = 32u32;
```

### `STATX_MTIME`
```rust
const STATX_MTIME: c_uint = 64u32;
```

### `STATX_CTIME`
```rust
const STATX_CTIME: c_uint = 128u32;
```

### `STATX_INO`
```rust
const STATX_INO: c_uint = 256u32;
```

### `STATX_SIZE`
```rust
const STATX_SIZE: c_uint = 512u32;
```

### `STATX_BLOCKS`
```rust
const STATX_BLOCKS: c_uint = 1_024u32;
```

### `STATX_BASIC_STATS`
```rust
const STATX_BASIC_STATS: c_uint = 2_047u32;
```

### `STATX_BTIME`
```rust
const STATX_BTIME: c_uint = 2_048u32;
```

### `STATX_ALL`
```rust
const STATX_ALL: c_uint = 4_095u32;
```

### `STATX_MNT_ID`
```rust
const STATX_MNT_ID: c_uint = 4_096u32;
```

### `STATX_DIOALIGN`
```rust
const STATX_DIOALIGN: c_uint = 8_192u32;
```

### `STATX__RESERVED`
```rust
const STATX__RESERVED: c_int = -2_147_483_648i32;
```

### `STATX_ATTR_COMPRESSED`
```rust
const STATX_ATTR_COMPRESSED: c_int = 4i32;
```

### `STATX_ATTR_IMMUTABLE`
```rust
const STATX_ATTR_IMMUTABLE: c_int = 16i32;
```

### `STATX_ATTR_APPEND`
```rust
const STATX_ATTR_APPEND: c_int = 32i32;
```

### `STATX_ATTR_NODUMP`
```rust
const STATX_ATTR_NODUMP: c_int = 64i32;
```

### `STATX_ATTR_ENCRYPTED`
```rust
const STATX_ATTR_ENCRYPTED: c_int = 2_048i32;
```

### `STATX_ATTR_AUTOMOUNT`
```rust
const STATX_ATTR_AUTOMOUNT: c_int = 4_096i32;
```

### `STATX_ATTR_MOUNT_ROOT`
```rust
const STATX_ATTR_MOUNT_ROOT: c_int = 8_192i32;
```

### `STATX_ATTR_VERITY`
```rust
const STATX_ATTR_VERITY: c_int = 1_048_576i32;
```

### `STATX_ATTR_DAX`
```rust
const STATX_ATTR_DAX: c_int = 2_097_152i32;
```

### `_IOC_NRBITS`
```rust
const _IOC_NRBITS: u32 = 8u32;
```

### `_IOC_TYPEBITS`
```rust
const _IOC_TYPEBITS: u32 = 8u32;
```

### `_IOC_SIZEBITS`
```rust
const _IOC_SIZEBITS: u32 = 14u32;
```

### `_IOC_DIRBITS`
```rust
const _IOC_DIRBITS: u32 = 2u32;
```

### `_IOC_NONE`
```rust
const _IOC_NONE: u32 = 0u32;
```

### `_IOC_WRITE`
```rust
const _IOC_WRITE: u32 = 1u32;
```

### `_IOC_READ`
```rust
const _IOC_READ: u32 = 2u32;
```

### `_IOC_NRMASK`
```rust
const _IOC_NRMASK: u32 = 255u32;
```

### `_IOC_TYPEMASK`
```rust
const _IOC_TYPEMASK: u32 = 255u32;
```

### `_IOC_SIZEMASK`
```rust
const _IOC_SIZEMASK: u32 = 16_383u32;
```

### `_IOC_DIRMASK`
```rust
const _IOC_DIRMASK: u32 = 3u32;
```

### `_IOC_NRSHIFT`
```rust
const _IOC_NRSHIFT: u32 = 0u32;
```

### `_IOC_TYPESHIFT`
```rust
const _IOC_TYPESHIFT: u32 = 8u32;
```

### `_IOC_SIZESHIFT`
```rust
const _IOC_SIZESHIFT: u32 = 16u32;
```

### `_IOC_DIRSHIFT`
```rust
const _IOC_DIRSHIFT: u32 = 30u32;
```

