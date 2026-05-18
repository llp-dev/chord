**libc > unix**

# Module: unix

## Contents

**Structs**

- [`group`](#group)
- [`hostent`](#hostent)
- [`in6_addr`](#in6_addr)
- [`iovec`](#iovec)
- [`ipv6_mreq`](#ipv6_mreq)
- [`itimerval`](#itimerval)
- [`linger`](#linger)
- [`pollfd`](#pollfd)
- [`protoent`](#protoent)
- [`rlimit`](#rlimit)
- [`rusage`](#rusage)
- [`servent`](#servent)
- [`sigval`](#sigval)
- [`timeval`](#timeval)
- [`tms`](#tms)
- [`utimbuf`](#utimbuf)
- [`winsize`](#winsize)

**Enums**

- [`DIR`](#dir)
- [`FILE`](#file)

**Functions**

- [`_exit`](#_exit)
- [`abort`](#abort)
- [`accept`](#accept)
- [`access`](#access)
- [`adjtime`](#adjtime)
- [`alarm`](#alarm)
- [`aligned_alloc`](#aligned_alloc)
- [`atexit`](#atexit)
- [`atof`](#atof)
- [`atoi`](#atoi)
- [`atol`](#atol)
- [`atoll`](#atoll)
- [`bsearch`](#bsearch)
- [`calloc`](#calloc)
- [`cfgetispeed`](#cfgetispeed)
- [`cfgetospeed`](#cfgetospeed)
- [`cfmakeraw`](#cfmakeraw)
- [`cfsetispeed`](#cfsetispeed)
- [`cfsetospeed`](#cfsetospeed)
- [`cfsetspeed`](#cfsetspeed)
- [`chdir`](#chdir)
- [`chmod`](#chmod)
- [`chown`](#chown)
- [`chroot`](#chroot)
- [`clearerr`](#clearerr)
- [`close`](#close)
- [`closedir`](#closedir)
- [`closelog`](#closelog)
- [`confstr`](#confstr)
- [`connect`](#connect)
- [`creat`](#creat)
- [`difftime`](#difftime)
- [`dladdr`](#dladdr)
- [`dlclose`](#dlclose)
- [`dlerror`](#dlerror)
- [`dlopen`](#dlopen)
- [`dlsym`](#dlsym)
- [`dup`](#dup)
- [`dup2`](#dup2)
- [`endservent`](#endservent)
- [`execl`](#execl)
- [`execle`](#execle)
- [`execlp`](#execlp)
- [`execv`](#execv)
- [`execve`](#execve)
- [`execvp`](#execvp)
- [`exit`](#exit)
- [`fchdir`](#fchdir)
- [`fchmod`](#fchmod)
- [`fchmodat`](#fchmodat)
- [`fchown`](#fchown)
- [`fchownat`](#fchownat)
- [`fclose`](#fclose)
- [`fcntl`](#fcntl)
- [`fdopen`](#fdopen)
- [`fdopendir`](#fdopendir)
- [`feof`](#feof)
- [`ferror`](#ferror)
- [`fflush`](#fflush)
- [`fgetc`](#fgetc)
- [`fgetpos`](#fgetpos)
- [`fgets`](#fgets)
- [`fileno`](#fileno)
- [`flock`](#flock)
- [`fmemopen`](#fmemopen)
- [`fnmatch`](#fnmatch)
- [`fopen`](#fopen)
- [`fork`](#fork)
- [`fpathconf`](#fpathconf)
- [`fprintf`](#fprintf)
- [`fputc`](#fputc)
- [`fputs`](#fputs)
- [`fread`](#fread)
- [`free`](#free)
- [`freeaddrinfo`](#freeaddrinfo)
- [`freopen`](#freopen)
- [`fscanf`](#fscanf)
- [`fseek`](#fseek)
- [`fseeko`](#fseeko)
- [`fsetpos`](#fsetpos)
- [`fstat`](#fstat)
- [`fstatat`](#fstatat)
- [`fstatvfs`](#fstatvfs)
- [`fsync`](#fsync)
- [`ftell`](#ftell)
- [`ftello`](#ftello)
- [`ftruncate`](#ftruncate)
- [`fwrite`](#fwrite)
- [`gai_strerror`](#gai_strerror)
- [`getaddrinfo`](#getaddrinfo)
- [`getchar`](#getchar)
- [`getchar_unlocked`](#getchar_unlocked)
- [`getcwd`](#getcwd)
- [`getegid`](#getegid)
- [`getenv`](#getenv)
- [`geteuid`](#geteuid)
- [`getgid`](#getgid)
- [`getgroups`](#getgroups)
- [`gethostname`](#gethostname)
- [`getline`](#getline)
- [`getlogin`](#getlogin)
- [`getopt`](#getopt)
- [`getpeername`](#getpeername)
- [`getpgid`](#getpgid)
- [`getpgrp`](#getpgrp)
- [`getpid`](#getpid)
- [`getppid`](#getppid)
- [`getprotobyname`](#getprotobyname)
- [`getprotobynumber`](#getprotobynumber)
- [`getpwnam`](#getpwnam)
- [`getpwuid`](#getpwuid)
- [`getrusage`](#getrusage)
- [`getservbyname`](#getservbyname)
- [`getservbyport`](#getservbyport)
- [`getservent`](#getservent)
- [`getsid`](#getsid)
- [`getsockname`](#getsockname)
- [`getsockopt`](#getsockopt)
- [`getuid`](#getuid)
- [`gmtime`](#gmtime)
- [`gmtime_r`](#gmtime_r)
- [`grantpt`](#grantpt)
- [`hstrerror`](#hstrerror)
- [`htonl`](#htonl)
- [`htons`](#htons)
- [`if_indextoname`](#if_indextoname)
- [`if_nametoindex`](#if_nametoindex)
- [`isalnum`](#isalnum)
- [`isalpha`](#isalpha)
- [`isatty`](#isatty)
- [`isblank`](#isblank)
- [`iscntrl`](#iscntrl)
- [`isdigit`](#isdigit)
- [`isgraph`](#isgraph)
- [`islower`](#islower)
- [`isprint`](#isprint)
- [`ispunct`](#ispunct)
- [`isspace`](#isspace)
- [`isupper`](#isupper)
- [`isxdigit`](#isxdigit)
- [`kill`](#kill)
- [`killpg`](#killpg)
- [`lchown`](#lchown)
- [`link`](#link)
- [`linkat`](#linkat)
- [`listen`](#listen)
- [`localeconv`](#localeconv)
- [`localtime`](#localtime)
- [`localtime_r`](#localtime_r)
- [`lockf`](#lockf)
- [`lseek`](#lseek)
- [`lstat`](#lstat)
- [`malloc`](#malloc)
- [`memccpy`](#memccpy)
- [`memchr`](#memchr)
- [`memcmp`](#memcmp)
- [`memcpy`](#memcpy)
- [`memmove`](#memmove)
- [`memset`](#memset)
- [`mkdir`](#mkdir)
- [`mkdirat`](#mkdirat)
- [`mkdtemp`](#mkdtemp)
- [`mkfifo`](#mkfifo)
- [`mknod`](#mknod)
- [`mkstemp`](#mkstemp)
- [`mktime`](#mktime)
- [`mlock`](#mlock)
- [`mlockall`](#mlockall)
- [`mmap`](#mmap)
- [`munlock`](#munlock)
- [`munlockall`](#munlockall)
- [`munmap`](#munmap)
- [`nanosleep`](#nanosleep)
- [`nice`](#nice)
- [`ntohl`](#ntohl)
- [`ntohs`](#ntohs)
- [`open`](#open)
- [`open_memstream`](#open_memstream)
- [`open_wmemstream`](#open_wmemstream)
- [`openat`](#openat)
- [`opendir`](#opendir)
- [`openlog`](#openlog)
- [`pathconf`](#pathconf)
- [`pause`](#pause)
- [`pclose`](#pclose)
- [`perror`](#perror)
- [`pipe`](#pipe)
- [`poll`](#poll)
- [`posix_memalign`](#posix_memalign)
- [`posix_openpt`](#posix_openpt)
- [`pread`](#pread)
- [`printf`](#printf)
- [`pselect`](#pselect)
- [`pthread_attr_destroy`](#pthread_attr_destroy)
- [`pthread_attr_getstacksize`](#pthread_attr_getstacksize)
- [`pthread_attr_init`](#pthread_attr_init)
- [`pthread_attr_setdetachstate`](#pthread_attr_setdetachstate)
- [`pthread_attr_setstacksize`](#pthread_attr_setstacksize)
- [`pthread_cond_broadcast`](#pthread_cond_broadcast)
- [`pthread_cond_destroy`](#pthread_cond_destroy)
- [`pthread_cond_init`](#pthread_cond_init)
- [`pthread_cond_signal`](#pthread_cond_signal)
- [`pthread_cond_timedwait`](#pthread_cond_timedwait)
- [`pthread_cond_wait`](#pthread_cond_wait)
- [`pthread_condattr_destroy`](#pthread_condattr_destroy)
- [`pthread_condattr_init`](#pthread_condattr_init)
- [`pthread_detach`](#pthread_detach)
- [`pthread_equal`](#pthread_equal)
- [`pthread_exit`](#pthread_exit)
- [`pthread_getspecific`](#pthread_getspecific)
- [`pthread_join`](#pthread_join)
- [`pthread_key_create`](#pthread_key_create)
- [`pthread_key_delete`](#pthread_key_delete)
- [`pthread_mutex_destroy`](#pthread_mutex_destroy)
- [`pthread_mutex_init`](#pthread_mutex_init)
- [`pthread_mutex_lock`](#pthread_mutex_lock)
- [`pthread_mutex_trylock`](#pthread_mutex_trylock)
- [`pthread_mutex_unlock`](#pthread_mutex_unlock)
- [`pthread_mutexattr_destroy`](#pthread_mutexattr_destroy)
- [`pthread_mutexattr_init`](#pthread_mutexattr_init)
- [`pthread_mutexattr_settype`](#pthread_mutexattr_settype)
- [`pthread_rwlock_destroy`](#pthread_rwlock_destroy)
- [`pthread_rwlock_init`](#pthread_rwlock_init)
- [`pthread_rwlock_rdlock`](#pthread_rwlock_rdlock)
- [`pthread_rwlock_tryrdlock`](#pthread_rwlock_tryrdlock)
- [`pthread_rwlock_trywrlock`](#pthread_rwlock_trywrlock)
- [`pthread_rwlock_unlock`](#pthread_rwlock_unlock)
- [`pthread_rwlock_wrlock`](#pthread_rwlock_wrlock)
- [`pthread_rwlockattr_destroy`](#pthread_rwlockattr_destroy)
- [`pthread_rwlockattr_init`](#pthread_rwlockattr_init)
- [`pthread_self`](#pthread_self)
- [`pthread_setspecific`](#pthread_setspecific)
- [`ptsname`](#ptsname)
- [`putchar`](#putchar)
- [`putchar_unlocked`](#putchar_unlocked)
- [`putenv`](#putenv)
- [`puts`](#puts)
- [`pwrite`](#pwrite)
- [`qsort`](#qsort)
- [`raise`](#raise)
- [`read`](#read)
- [`readdir`](#readdir)
- [`readdir_r`](#readdir_r) - The 64-bit libc on Solaris and illumos only has readdir_r. If a
- [`readlink`](#readlink)
- [`readlinkat`](#readlinkat)
- [`realloc`](#realloc)
- [`realpath`](#realpath)
- [`recv`](#recv)
- [`remove`](#remove)
- [`rename`](#rename)
- [`renameat`](#renameat)
- [`res_init`](#res_init)
- [`rewind`](#rewind)
- [`rewinddir`](#rewinddir)
- [`rmdir`](#rmdir)
- [`scanf`](#scanf)
- [`sched_yield`](#sched_yield)
- [`select`](#select)
- [`sem_post`](#sem_post)
- [`sem_trywait`](#sem_trywait)
- [`sem_wait`](#sem_wait)
- [`send`](#send)
- [`sendto`](#sendto)
- [`setbuf`](#setbuf)
- [`setegid`](#setegid)
- [`setenv`](#setenv)
- [`seteuid`](#seteuid)
- [`setgid`](#setgid)
- [`setlocale`](#setlocale)
- [`setlogmask`](#setlogmask)
- [`setpgid`](#setpgid)
- [`setregid`](#setregid)
- [`setreuid`](#setreuid)
- [`setservent`](#setservent)
- [`setsid`](#setsid)
- [`setsockopt`](#setsockopt)
- [`setuid`](#setuid)
- [`setvbuf`](#setvbuf)
- [`shutdown`](#shutdown)
- [`sigaction`](#sigaction)
- [`sigaddset`](#sigaddset)
- [`sigdelset`](#sigdelset)
- [`sigemptyset`](#sigemptyset)
- [`sigfillset`](#sigfillset)
- [`sigismember`](#sigismember)
- [`signal`](#signal)
- [`sigpending`](#sigpending)
- [`sigprocmask`](#sigprocmask)
- [`sigqueue`](#sigqueue)
- [`sleep`](#sleep)
- [`snprintf`](#snprintf)
- [`socket`](#socket)
- [`socketpair`](#socketpair)
- [`sprintf`](#sprintf)
- [`sscanf`](#sscanf)
- [`stat`](#stat)
- [`statvfs`](#statvfs)
- [`stpcpy`](#stpcpy)
- [`stpncpy`](#stpncpy)
- [`strcasecmp`](#strcasecmp)
- [`strcasestr`](#strcasestr)
- [`strcat`](#strcat)
- [`strchr`](#strchr)
- [`strcmp`](#strcmp)
- [`strcoll`](#strcoll)
- [`strcpy`](#strcpy)
- [`strcspn`](#strcspn)
- [`strdup`](#strdup)
- [`strerror`](#strerror)
- [`strlen`](#strlen)
- [`strncasecmp`](#strncasecmp)
- [`strncat`](#strncat)
- [`strncmp`](#strncmp)
- [`strncpy`](#strncpy)
- [`strndup`](#strndup)
- [`strnlen`](#strnlen)
- [`strpbrk`](#strpbrk)
- [`strrchr`](#strrchr)
- [`strsignal`](#strsignal)
- [`strspn`](#strspn)
- [`strstr`](#strstr)
- [`strtod`](#strtod)
- [`strtof`](#strtof)
- [`strtok`](#strtok)
- [`strtok_r`](#strtok_r)
- [`strtol`](#strtol)
- [`strtoll`](#strtoll)
- [`strtoul`](#strtoul)
- [`strtoull`](#strtoull)
- [`strxfrm`](#strxfrm)
- [`symlink`](#symlink)
- [`symlinkat`](#symlinkat)
- [`sysconf`](#sysconf)
- [`syslog`](#syslog)
- [`system`](#system)
- [`tcdrain`](#tcdrain)
- [`tcflow`](#tcflow)
- [`tcflush`](#tcflush)
- [`tcgetattr`](#tcgetattr)
- [`tcgetpgrp`](#tcgetpgrp)
- [`tcgetsid`](#tcgetsid)
- [`tcsendbreak`](#tcsendbreak)
- [`tcsetattr`](#tcsetattr)
- [`tcsetpgrp`](#tcsetpgrp)
- [`time`](#time)
- [`timegm`](#timegm)
- [`times`](#times)
- [`tmpfile`](#tmpfile)
- [`tmpnam`](#tmpnam)
- [`tolower`](#tolower)
- [`toupper`](#toupper)
- [`truncate`](#truncate)
- [`ttyname`](#ttyname)
- [`ttyname_r`](#ttyname_r)
- [`umask`](#umask)
- [`ungetc`](#ungetc)
- [`unlink`](#unlink)
- [`unlinkat`](#unlinkat)
- [`unlockpt`](#unlockpt)
- [`unsetenv`](#unsetenv)
- [`usleep`](#usleep)
- [`utime`](#utime)
- [`utimes`](#utimes)
- [`wait`](#wait)
- [`waitpid`](#waitpid)
- [`wcslen`](#wcslen)
- [`wcstombs`](#wcstombs)
- [`wmemchr`](#wmemchr)
- [`write`](#write)

**Statics**

- [`in6addr_any`](#in6addr_any)
- [`in6addr_loopback`](#in6addr_loopback)

**Constants**

- [`ARPOP_REPLY`](#arpop_reply)
- [`ARPOP_REQUEST`](#arpop_request)
- [`ATF_COM`](#atf_com)
- [`ATF_PERM`](#atf_perm)
- [`ATF_PUBL`](#atf_publ)
- [`ATF_USETRAILERS`](#atf_usetrailers)
- [`DT_BLK`](#dt_blk)
- [`DT_CHR`](#dt_chr)
- [`DT_DIR`](#dt_dir)
- [`DT_FIFO`](#dt_fifo)
- [`DT_LNK`](#dt_lnk)
- [`DT_REG`](#dt_reg)
- [`DT_SOCK`](#dt_sock)
- [`DT_UNKNOWN`](#dt_unknown)
- [`FD_CLOEXEC`](#fd_cloexec)
- [`FNM_CASEFOLD`](#fnm_casefold)
- [`FNM_NOESCAPE`](#fnm_noescape)
- [`FNM_NOMATCH`](#fnm_nomatch)
- [`FNM_PATHNAME`](#fnm_pathname)
- [`FNM_PERIOD`](#fnm_period)
- [`GRPQUOTA`](#grpquota)
- [`IFNAMSIZ`](#ifnamsiz)
- [`IF_NAMESIZE`](#if_namesize)
- [`IN6ADDR_ANY_INIT`](#in6addr_any_init)
- [`IN6ADDR_LOOPBACK_INIT`](#in6addr_loopback_init)
- [`INADDR_ANY`](#inaddr_any)
- [`INADDR_BROADCAST`](#inaddr_broadcast)
- [`INADDR_LOOPBACK`](#inaddr_loopback)
- [`INADDR_NONE`](#inaddr_none)
- [`INT_MAX`](#int_max)
- [`INT_MIN`](#int_min)
- [`IPPROTO_ICMP`](#ipproto_icmp)
- [`IPPROTO_ICMPV6`](#ipproto_icmpv6)
- [`IPPROTO_IP`](#ipproto_ip)
- [`IPPROTO_IPV6`](#ipproto_ipv6)
- [`IPPROTO_TCP`](#ipproto_tcp)
- [`IPPROTO_UDP`](#ipproto_udp)
- [`LOG_ALERT`](#log_alert)
- [`LOG_AUTH`](#log_auth)
- [`LOG_CONS`](#log_cons)
- [`LOG_CRIT`](#log_crit)
- [`LOG_DAEMON`](#log_daemon)
- [`LOG_DEBUG`](#log_debug)
- [`LOG_EMERG`](#log_emerg)
- [`LOG_ERR`](#log_err)
- [`LOG_FACMASK`](#log_facmask)
- [`LOG_INFO`](#log_info)
- [`LOG_KERN`](#log_kern)
- [`LOG_LOCAL0`](#log_local0)
- [`LOG_LOCAL1`](#log_local1)
- [`LOG_LOCAL2`](#log_local2)
- [`LOG_LOCAL3`](#log_local3)
- [`LOG_LOCAL4`](#log_local4)
- [`LOG_LOCAL5`](#log_local5)
- [`LOG_LOCAL6`](#log_local6)
- [`LOG_LOCAL7`](#log_local7)
- [`LOG_LPR`](#log_lpr)
- [`LOG_MAIL`](#log_mail)
- [`LOG_NDELAY`](#log_ndelay)
- [`LOG_NEWS`](#log_news)
- [`LOG_NOTICE`](#log_notice)
- [`LOG_NOWAIT`](#log_nowait)
- [`LOG_ODELAY`](#log_odelay)
- [`LOG_PID`](#log_pid)
- [`LOG_PRIMASK`](#log_primask)
- [`LOG_SYSLOG`](#log_syslog)
- [`LOG_USER`](#log_user)
- [`LOG_UUCP`](#log_uucp)
- [`LOG_WARNING`](#log_warning)
- [`PRIO_MAX`](#prio_max)
- [`PRIO_MIN`](#prio_min)
- [`SIGIOT`](#sigiot)
- [`SIG_DFL`](#sig_dfl)
- [`SIG_ERR`](#sig_err)
- [`SIG_IGN`](#sig_ign)
- [`S_ISGID`](#s_isgid)
- [`S_ISUID`](#s_isuid)
- [`S_ISVTX`](#s_isvtx)
- [`USRQUOTA`](#usrquota)

**Type Aliases**

- [`cc_t`](#cc_t)
- [`gid_t`](#gid_t)
- [`in_addr_t`](#in_addr_t)
- [`in_port_t`](#in_port_t)
- [`intmax_t`](#intmax_t)
- [`intptr_t`](#intptr_t)
- [`locale_t`](#locale_t)
- [`pid_t`](#pid_t)
- [`ptrdiff_t`](#ptrdiff_t)
- [`sighandler_t`](#sighandler_t)
- [`size_t`](#size_t)
- [`ssize_t`](#ssize_t)
- [`uid_t`](#uid_t)
- [`uintmax_t`](#uintmax_t)
- [`uintptr_t`](#uintptr_t)

---

## libc::unix::ARPOP_REPLY

*Constant*: `u16`



## libc::unix::ARPOP_REQUEST

*Constant*: `u16`



## libc::unix::ATF_COM

*Constant*: `c_int`



## libc::unix::ATF_PERM

*Constant*: `c_int`



## libc::unix::ATF_PUBL

*Constant*: `c_int`



## libc::unix::ATF_USETRAILERS

*Constant*: `c_int`



## libc::unix::DIR

*Enum*

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> DIR`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::DT_BLK

*Constant*: `u8`



## libc::unix::DT_CHR

*Constant*: `u8`



## libc::unix::DT_DIR

*Constant*: `u8`



## libc::unix::DT_FIFO

*Constant*: `u8`



## libc::unix::DT_LNK

*Constant*: `u8`



## libc::unix::DT_REG

*Constant*: `u8`



## libc::unix::DT_SOCK

*Constant*: `u8`



## libc::unix::DT_UNKNOWN

*Constant*: `u8`



## libc::unix::FD_CLOEXEC

*Constant*: `c_int`



## libc::unix::FILE

*Enum*

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> FILE`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::FNM_CASEFOLD

*Constant*: `c_int`



## libc::unix::FNM_NOESCAPE

*Constant*: `c_int`



## libc::unix::FNM_NOMATCH

*Constant*: `c_int`



## libc::unix::FNM_PATHNAME

*Constant*: `c_int`



## libc::unix::FNM_PERIOD

*Constant*: `c_int`



## libc::unix::GRPQUOTA

*Constant*: `c_int`



## libc::unix::IFNAMSIZ

*Constant*: `size_t`



## libc::unix::IF_NAMESIZE

*Constant*: `size_t`



## libc::unix::IN6ADDR_ANY_INIT

*Constant*: `in6_addr`



## libc::unix::IN6ADDR_LOOPBACK_INIT

*Constant*: `in6_addr`



## libc::unix::INADDR_ANY

*Constant*: `in_addr_t`



## libc::unix::INADDR_BROADCAST

*Constant*: `in_addr_t`



## libc::unix::INADDR_LOOPBACK

*Constant*: `in_addr_t`



## libc::unix::INADDR_NONE

*Constant*: `in_addr_t`



## libc::unix::INT_MAX

*Constant*: `c_int`



## libc::unix::INT_MIN

*Constant*: `c_int`



## libc::unix::IPPROTO_ICMP

*Constant*: `c_int`



## libc::unix::IPPROTO_ICMPV6

*Constant*: `c_int`



## libc::unix::IPPROTO_IP

*Constant*: `c_int`



## libc::unix::IPPROTO_IPV6

*Constant*: `c_int`



## libc::unix::IPPROTO_TCP

*Constant*: `c_int`



## libc::unix::IPPROTO_UDP

*Constant*: `c_int`



## libc::unix::LOG_ALERT

*Constant*: `c_int`



## libc::unix::LOG_AUTH

*Constant*: `c_int`



## libc::unix::LOG_CONS

*Constant*: `c_int`



## libc::unix::LOG_CRIT

*Constant*: `c_int`



## libc::unix::LOG_DAEMON

*Constant*: `c_int`



## libc::unix::LOG_DEBUG

*Constant*: `c_int`



## libc::unix::LOG_EMERG

*Constant*: `c_int`



## libc::unix::LOG_ERR

*Constant*: `c_int`



## libc::unix::LOG_FACMASK

*Constant*: `c_int`



## libc::unix::LOG_INFO

*Constant*: `c_int`



## libc::unix::LOG_KERN

*Constant*: `c_int`



## libc::unix::LOG_LOCAL0

*Constant*: `c_int`



## libc::unix::LOG_LOCAL1

*Constant*: `c_int`



## libc::unix::LOG_LOCAL2

*Constant*: `c_int`



## libc::unix::LOG_LOCAL3

*Constant*: `c_int`



## libc::unix::LOG_LOCAL4

*Constant*: `c_int`



## libc::unix::LOG_LOCAL5

*Constant*: `c_int`



## libc::unix::LOG_LOCAL6

*Constant*: `c_int`



## libc::unix::LOG_LOCAL7

*Constant*: `c_int`



## libc::unix::LOG_LPR

*Constant*: `c_int`



## libc::unix::LOG_MAIL

*Constant*: `c_int`



## libc::unix::LOG_NDELAY

*Constant*: `c_int`



## libc::unix::LOG_NEWS

*Constant*: `c_int`



## libc::unix::LOG_NOTICE

*Constant*: `c_int`



## libc::unix::LOG_NOWAIT

*Constant*: `c_int`



## libc::unix::LOG_ODELAY

*Constant*: `c_int`



## libc::unix::LOG_PID

*Constant*: `c_int`



## libc::unix::LOG_PRIMASK

*Constant*: `c_int`



## libc::unix::LOG_SYSLOG

*Constant*: `c_int`



## libc::unix::LOG_USER

*Constant*: `c_int`



## libc::unix::LOG_UUCP

*Constant*: `c_int`



## libc::unix::LOG_WARNING

*Constant*: `c_int`



## libc::unix::PRIO_MAX

*Constant*: `c_int`



## libc::unix::PRIO_MIN

*Constant*: `c_int`



## libc::unix::SIGIOT

*Constant*: `c_int`



## libc::unix::SIG_DFL

*Constant*: `sighandler_t`



## libc::unix::SIG_ERR

*Constant*: `sighandler_t`



## libc::unix::SIG_IGN

*Constant*: `sighandler_t`



## libc::unix::S_ISGID

*Constant*: `mode_t`



## libc::unix::S_ISUID

*Constant*: `mode_t`



## libc::unix::S_ISVTX

*Constant*: `mode_t`



## libc::unix::USRQUOTA

*Constant*: `c_int`



## libc::unix::_exit

*Function*

```rust
fn _exit(status: c_int) -> never
```



## libc::unix::abort

*Function*

```rust
fn abort() -> never
```



## libc::unix::accept

*Function*

```rust
fn accept(socket: c_int, address: *mut sockaddr, address_len: *mut socklen_t) -> c_int
```



## libc::unix::access

*Function*

```rust
fn access(path: *const c_char, amode: c_int) -> c_int
```



## libc::unix::adjtime

*Function*

```rust
fn adjtime(delta: *const timeval, olddelta: *mut timeval) -> c_int
```



## libc::unix::alarm

*Function*

```rust
fn alarm(seconds: c_uint) -> c_uint
```



## libc::unix::aligned_alloc

*Function*

```rust
fn aligned_alloc(alignment: size_t, size: size_t) -> *mut c_void
```



## libc::unix::atexit

*Function*

```rust
fn atexit(cb: fn(...)) -> c_int
```



## libc::unix::atof

*Function*

```rust
fn atof(s: *const c_char) -> c_double
```



## libc::unix::atoi

*Function*

```rust
fn atoi(s: *const c_char) -> c_int
```



## libc::unix::atol

*Function*

```rust
fn atol(s: *const c_char) -> c_long
```



## libc::unix::atoll

*Function*

```rust
fn atoll(s: *const c_char) -> c_longlong
```



## libc::unix::bsearch

*Function*

```rust
fn bsearch(key: *const c_void, base: *const c_void, num: size_t, size: size_t, compar: Option<fn(...)>) -> *mut c_void
```



## libc::unix::calloc

*Function*

```rust
fn calloc(nobj: size_t, size: size_t) -> *mut c_void
```



## libc::unix::cc_t

*Type Alias*: `c_uchar`



## libc::unix::cfgetispeed

*Function*

```rust
fn cfgetispeed(termios: *const crate::termios) -> crate::speed_t
```



## libc::unix::cfgetospeed

*Function*

```rust
fn cfgetospeed(termios: *const crate::termios) -> crate::speed_t
```



## libc::unix::cfmakeraw

*Function*

```rust
fn cfmakeraw(termios: *mut crate::termios)
```



## libc::unix::cfsetispeed

*Function*

```rust
fn cfsetispeed(termios: *mut crate::termios, speed: crate::speed_t) -> c_int
```



## libc::unix::cfsetospeed

*Function*

```rust
fn cfsetospeed(termios: *mut crate::termios, speed: crate::speed_t) -> c_int
```



## libc::unix::cfsetspeed

*Function*

```rust
fn cfsetspeed(termios: *mut crate::termios, speed: crate::speed_t) -> c_int
```



## libc::unix::chdir

*Function*

```rust
fn chdir(dir: *const c_char) -> c_int
```



## libc::unix::chmod

*Function*

```rust
fn chmod(path: *const c_char, mode: mode_t) -> c_int
```



## libc::unix::chown

*Function*

```rust
fn chown(path: *const c_char, uid: uid_t, gid: gid_t) -> c_int
```



## libc::unix::chroot

*Function*

```rust
fn chroot(name: *const c_char) -> c_int
```



## libc::unix::clearerr

*Function*

```rust
fn clearerr(stream: *mut FILE)
```



## libc::unix::close

*Function*

```rust
fn close(fd: c_int) -> c_int
```



## libc::unix::closedir

*Function*

```rust
fn closedir(dirp: *mut crate::DIR) -> c_int
```



## libc::unix::closelog

*Function*

```rust
fn closelog()
```



## libc::unix::confstr

*Function*

```rust
fn confstr(name: c_int, buf: *mut c_char, len: size_t) -> size_t
```



## libc::unix::connect

*Function*

```rust
fn connect(socket: c_int, address: *const sockaddr, len: socklen_t) -> c_int
```



## libc::unix::creat

*Function*

```rust
fn creat(path: *const c_char, mode: mode_t) -> c_int
```



## libc::unix::difftime

*Function*

```rust
fn difftime(time1: time_t, time0: time_t) -> c_double
```



## libc::unix::dladdr

*Function*

```rust
fn dladdr(addr: *const c_void, info: *mut Dl_info) -> c_int
```



## libc::unix::dlclose

*Function*

```rust
fn dlclose(handle: *mut c_void) -> c_int
```



## libc::unix::dlerror

*Function*

```rust
fn dlerror() -> *mut c_char
```



## libc::unix::dlopen

*Function*

```rust
fn dlopen(filename: *const c_char, flag: c_int) -> *mut c_void
```



## libc::unix::dlsym

*Function*

```rust
fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void
```



## libc::unix::dup

*Function*

```rust
fn dup(fd: c_int) -> c_int
```



## libc::unix::dup2

*Function*

```rust
fn dup2(src: c_int, dst: c_int) -> c_int
```



## libc::unix::endservent

*Function*

```rust
fn endservent()
```



## libc::unix::execl

*Function*

```rust
fn execl(path: *const c_char, arg0: *const c_char) -> c_int
```



## libc::unix::execle

*Function*

```rust
fn execle(path: *const c_char, arg0: *const c_char) -> c_int
```



## libc::unix::execlp

*Function*

```rust
fn execlp(file: *const c_char, arg0: *const c_char) -> c_int
```



## libc::unix::execv

*Function*

```rust
fn execv(prog: *const c_char, argv: *const *const c_char) -> c_int
```



## libc::unix::execve

*Function*

```rust
fn execve(prog: *const c_char, argv: *const *const c_char, envp: *const *const c_char) -> c_int
```



## libc::unix::execvp

*Function*

```rust
fn execvp(c: *const c_char, argv: *const *const c_char) -> c_int
```



## libc::unix::exit

*Function*

```rust
fn exit(status: c_int) -> never
```



## libc::unix::fchdir

*Function*

```rust
fn fchdir(dirfd: c_int) -> c_int
```



## libc::unix::fchmod

*Function*

```rust
fn fchmod(fd: c_int, mode: mode_t) -> c_int
```



## libc::unix::fchmodat

*Function*

```rust
fn fchmodat(dirfd: c_int, pathname: *const c_char, mode: mode_t, flags: c_int) -> c_int
```



## libc::unix::fchown

*Function*

```rust
fn fchown(fd: c_int, owner: crate::uid_t, group: crate::gid_t) -> c_int
```



## libc::unix::fchownat

*Function*

```rust
fn fchownat(dirfd: c_int, pathname: *const c_char, owner: crate::uid_t, group: crate::gid_t, flags: c_int) -> c_int
```



## libc::unix::fclose

*Function*

```rust
fn fclose(file: *mut FILE) -> c_int
```



## libc::unix::fcntl

*Function*

```rust
fn fcntl(fd: c_int, cmd: c_int) -> c_int
```



## libc::unix::fdopen

*Function*

```rust
fn fdopen(fd: c_int, mode: *const c_char) -> *mut crate::FILE
```



## libc::unix::fdopendir

*Function*

```rust
fn fdopendir(fd: c_int) -> *mut crate::DIR
```



## libc::unix::feof

*Function*

```rust
fn feof(stream: *mut FILE) -> c_int
```



## libc::unix::ferror

*Function*

```rust
fn ferror(stream: *mut FILE) -> c_int
```



## libc::unix::fflush

*Function*

```rust
fn fflush(file: *mut FILE) -> c_int
```



## libc::unix::fgetc

*Function*

```rust
fn fgetc(stream: *mut FILE) -> c_int
```



## libc::unix::fgetpos

*Function*

```rust
fn fgetpos(stream: *mut FILE, ptr: *mut fpos_t) -> c_int
```



## libc::unix::fgets

*Function*

```rust
fn fgets(buf: *mut c_char, n: c_int, stream: *mut FILE) -> *mut c_char
```



## libc::unix::fileno

*Function*

```rust
fn fileno(stream: *mut crate::FILE) -> c_int
```



## libc::unix::flock

*Function*

```rust
fn flock(fd: c_int, operation: c_int) -> c_int
```



## libc::unix::fmemopen

*Function*

```rust
fn fmemopen(buf: *mut c_void, size: size_t, mode: *const c_char) -> *mut FILE
```



## libc::unix::fnmatch

*Function*

```rust
fn fnmatch(pattern: *const c_char, name: *const c_char, flags: c_int) -> c_int
```



## libc::unix::fopen

*Function*

```rust
fn fopen(filename: *const c_char, mode: *const c_char) -> *mut FILE
```



## libc::unix::fork

*Function*

```rust
fn fork() -> pid_t
```



## libc::unix::fpathconf

*Function*

```rust
fn fpathconf(filedes: c_int, name: c_int) -> c_long
```



## libc::unix::fprintf

*Function*

```rust
fn fprintf(stream: *mut crate::FILE, format: *const c_char) -> c_int
```



## libc::unix::fputc

*Function*

```rust
fn fputc(c: c_int, stream: *mut FILE) -> c_int
```



## libc::unix::fputs

*Function*

```rust
fn fputs(s: *const c_char, stream: *mut FILE) -> c_int
```



## libc::unix::fread

*Function*

```rust
fn fread(ptr: *mut c_void, size: size_t, nobj: size_t, stream: *mut FILE) -> size_t
```



## libc::unix::free

*Function*

```rust
fn free(p: *mut c_void)
```



## libc::unix::freeaddrinfo

*Function*

```rust
fn freeaddrinfo(res: *mut addrinfo)
```



## libc::unix::freopen

*Function*

```rust
fn freopen(filename: *const c_char, mode: *const c_char, file: *mut FILE) -> *mut FILE
```



## libc::unix::fscanf

*Function*

```rust
fn fscanf(stream: *mut crate::FILE, format: *const c_char) -> c_int
```



## libc::unix::fseek

*Function*

```rust
fn fseek(stream: *mut FILE, offset: c_long, whence: c_int) -> c_int
```



## libc::unix::fseeko

*Function*

```rust
fn fseeko(stream: *mut crate::FILE, offset: off_t, whence: c_int) -> c_int
```



## libc::unix::fsetpos

*Function*

```rust
fn fsetpos(stream: *mut FILE, ptr: *const fpos_t) -> c_int
```



## libc::unix::fstat

*Function*

```rust
fn fstat(fildes: c_int, buf: *mut stat) -> c_int
```



## libc::unix::fstatat

*Function*

```rust
fn fstatat(dirfd: c_int, pathname: *const c_char, buf: *mut stat, flags: c_int) -> c_int
```



## libc::unix::fstatvfs

*Function*

```rust
fn fstatvfs(fd: c_int, buf: *mut crate::statvfs) -> c_int
```



## libc::unix::fsync

*Function*

```rust
fn fsync(fd: c_int) -> c_int
```



## libc::unix::ftell

*Function*

```rust
fn ftell(stream: *mut FILE) -> c_long
```



## libc::unix::ftello

*Function*

```rust
fn ftello(stream: *mut crate::FILE) -> off_t
```



## libc::unix::ftruncate

*Function*

```rust
fn ftruncate(fd: c_int, length: off_t) -> c_int
```



## libc::unix::fwrite

*Function*

```rust
fn fwrite(ptr: *const c_void, size: size_t, nobj: size_t, stream: *mut FILE) -> size_t
```



## libc::unix::gai_strerror

*Function*

```rust
fn gai_strerror(errcode: c_int) -> *const c_char
```



## libc::unix::getaddrinfo

*Function*

```rust
fn getaddrinfo(node: *const c_char, service: *const c_char, hints: *const addrinfo, res: *mut *mut addrinfo) -> c_int
```



## libc::unix::getchar

*Function*

```rust
fn getchar() -> c_int
```



## libc::unix::getchar_unlocked

*Function*

```rust
fn getchar_unlocked() -> c_int
```



## libc::unix::getcwd

*Function*

```rust
fn getcwd(buf: *mut c_char, size: size_t) -> *mut c_char
```



## libc::unix::getegid

*Function*

```rust
fn getegid() -> gid_t
```



## libc::unix::getenv

*Function*

```rust
fn getenv(s: *const c_char) -> *mut c_char
```



## libc::unix::geteuid

*Function*

```rust
fn geteuid() -> uid_t
```



## libc::unix::getgid

*Function*

```rust
fn getgid() -> gid_t
```



## libc::unix::getgroups

*Function*

```rust
fn getgroups(ngroups_max: c_int, groups: *mut gid_t) -> c_int
```



## libc::unix::gethostname

*Function*

```rust
fn gethostname(name: *mut c_char, len: size_t) -> c_int
```



## libc::unix::getline

*Function*

```rust
fn getline(lineptr: *mut *mut c_char, n: *mut size_t, stream: *mut FILE) -> ssize_t
```



## libc::unix::getlogin

*Function*

```rust
fn getlogin() -> *mut c_char
```



## libc::unix::getopt

*Function*

```rust
fn getopt(argc: c_int, argv: *const *mut c_char, optstr: *const c_char) -> c_int
```



## libc::unix::getpeername

*Function*

```rust
fn getpeername(socket: c_int, address: *mut sockaddr, address_len: *mut socklen_t) -> c_int
```



## libc::unix::getpgid

*Function*

```rust
fn getpgid(pid: pid_t) -> pid_t
```



## libc::unix::getpgrp

*Function*

```rust
fn getpgrp() -> pid_t
```



## libc::unix::getpid

*Function*

```rust
fn getpid() -> pid_t
```



## libc::unix::getppid

*Function*

```rust
fn getppid() -> pid_t
```



## libc::unix::getprotobyname

*Function*

```rust
fn getprotobyname(name: *const c_char) -> *mut protoent
```



## libc::unix::getprotobynumber

*Function*

```rust
fn getprotobynumber(proto: c_int) -> *mut protoent
```



## libc::unix::getpwnam

*Function*

```rust
fn getpwnam(name: *const c_char) -> *mut passwd
```



## libc::unix::getpwuid

*Function*

```rust
fn getpwuid(uid: crate::uid_t) -> *mut passwd
```



## libc::unix::getrusage

*Function*

```rust
fn getrusage(resource: c_int, usage: *mut rusage) -> c_int
```



## libc::unix::getservbyname

*Function*

```rust
fn getservbyname(name: *const c_char, proto: *const c_char) -> *mut servent
```



## libc::unix::getservbyport

*Function*

```rust
fn getservbyport(port: c_int, proto: *const c_char) -> *mut servent
```



## libc::unix::getservent

*Function*

```rust
fn getservent() -> *mut servent
```



## libc::unix::getsid

*Function*

```rust
fn getsid(pid: pid_t) -> pid_t
```



## libc::unix::getsockname

*Function*

```rust
fn getsockname(socket: c_int, address: *mut sockaddr, address_len: *mut socklen_t) -> c_int
```



## libc::unix::getsockopt

*Function*

```rust
fn getsockopt(sockfd: c_int, level: c_int, optname: c_int, optval: *mut c_void, optlen: *mut crate::socklen_t) -> c_int
```



## libc::unix::getuid

*Function*

```rust
fn getuid() -> uid_t
```



## libc::unix::gid_t

*Type Alias*: `u32`



## libc::unix::gmtime

*Function*

```rust
fn gmtime(time_p: *const time_t) -> *mut tm
```



## libc::unix::gmtime_r

*Function*

```rust
fn gmtime_r(time_p: *const time_t, result: *mut tm) -> *mut tm
```



## libc::unix::grantpt

*Function*

```rust
fn grantpt(fd: c_int) -> c_int
```



## libc::unix::group

*Struct*

**Fields:**
- `gr_name: *mut c_char`
- `gr_passwd: *mut c_char`
- `gr_gid: crate::gid_t`
- `gr_mem: *mut *mut c_char`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> group`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::hostent

*Struct*

**Fields:**
- `h_name: *mut c_char`
- `h_aliases: *mut *mut c_char`
- `h_addrtype: c_int`
- `h_length: c_int`
- `h_addr_list: *mut *mut c_char`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> hostent`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::hstrerror

*Function*

```rust
fn hstrerror(errcode: c_int) -> *const c_char
```



## libc::unix::htonl

*Function*

```rust
fn htonl(hostlong: u32) -> u32
```



## libc::unix::htons

*Function*

```rust
fn htons(hostshort: u16) -> u16
```



## libc::unix::if_indextoname

*Function*

```rust
fn if_indextoname(ifindex: c_uint, ifname: *mut c_char) -> *mut c_char
```



## libc::unix::if_nametoindex

*Function*

```rust
fn if_nametoindex(ifname: *const c_char) -> c_uint
```



## libc::unix::in6_addr

*Struct*

**Fields:**
- `s6_addr: [u8; 16]`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> in6_addr`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::in6addr_any

*Static*

```rust
unsafe static in6addr_any: in6_addr
```



## libc::unix::in6addr_loopback

*Static*

```rust
unsafe static in6addr_loopback: in6_addr
```



## libc::unix::in_addr_t

*Type Alias*: `u32`



## libc::unix::in_port_t

*Type Alias*: `u16`



## libc::unix::intmax_t

*Type Alias*: `i64`



## libc::unix::intptr_t

*Type Alias*: `isize`



## libc::unix::iovec

*Struct*

**Fields:**
- `iov_base: *mut c_void`
- `iov_len: size_t`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> iovec`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::ipv6_mreq

*Struct*

**Fields:**
- `ipv6mr_multiaddr: in6_addr`
- `ipv6mr_interface: c_uint`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ipv6_mreq`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::isalnum

*Function*

```rust
fn isalnum(c: c_int) -> c_int
```



## libc::unix::isalpha

*Function*

```rust
fn isalpha(c: c_int) -> c_int
```



## libc::unix::isatty

*Function*

```rust
fn isatty(fd: c_int) -> c_int
```



## libc::unix::isblank

*Function*

```rust
fn isblank(c: c_int) -> c_int
```



## libc::unix::iscntrl

*Function*

```rust
fn iscntrl(c: c_int) -> c_int
```



## libc::unix::isdigit

*Function*

```rust
fn isdigit(c: c_int) -> c_int
```



## libc::unix::isgraph

*Function*

```rust
fn isgraph(c: c_int) -> c_int
```



## libc::unix::islower

*Function*

```rust
fn islower(c: c_int) -> c_int
```



## libc::unix::isprint

*Function*

```rust
fn isprint(c: c_int) -> c_int
```



## libc::unix::ispunct

*Function*

```rust
fn ispunct(c: c_int) -> c_int
```



## libc::unix::isspace

*Function*

```rust
fn isspace(c: c_int) -> c_int
```



## libc::unix::isupper

*Function*

```rust
fn isupper(c: c_int) -> c_int
```



## libc::unix::isxdigit

*Function*

```rust
fn isxdigit(c: c_int) -> c_int
```



## libc::unix::itimerval

*Struct*

**Fields:**
- `it_interval: crate::timeval`
- `it_value: crate::timeval`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> itimerval`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::kill

*Function*

```rust
fn kill(pid: pid_t, sig: c_int) -> c_int
```



## libc::unix::killpg

*Function*

```rust
fn killpg(pgrp: pid_t, sig: c_int) -> c_int
```



## libc::unix::lchown

*Function*

```rust
fn lchown(path: *const c_char, uid: uid_t, gid: gid_t) -> c_int
```



## libc::unix::linger

*Struct*

**Fields:**
- `l_onoff: c_int`
- `l_linger: c_int`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> linger`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::link

*Function*

```rust
fn link(src: *const c_char, dst: *const c_char) -> c_int
```



## libc::unix::linkat

*Function*

```rust
fn linkat(olddirfd: c_int, oldpath: *const c_char, newdirfd: c_int, newpath: *const c_char, flags: c_int) -> c_int
```



## libc::unix::listen

*Function*

```rust
fn listen(socket: c_int, backlog: c_int) -> c_int
```



## libc::unix::locale_t

*Type Alias*: `*mut c_void`



## libc::unix::localeconv

*Function*

```rust
fn localeconv() -> *mut lconv
```



## libc::unix::localtime

*Function*

```rust
fn localtime(time_p: *const time_t) -> *mut tm
```



## libc::unix::localtime_r

*Function*

```rust
fn localtime_r(time_p: *const time_t, result: *mut tm) -> *mut tm
```



## libc::unix::lockf

*Function*

```rust
fn lockf(fd: c_int, cmd: c_int, len: off_t) -> c_int
```



## libc::unix::lseek

*Function*

```rust
fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t
```



## libc::unix::lstat

*Function*

```rust
fn lstat(path: *const c_char, buf: *mut stat) -> c_int
```



## libc::unix::malloc

*Function*

```rust
fn malloc(size: size_t) -> *mut c_void
```



## libc::unix::memccpy

*Function*

```rust
fn memccpy(dest: *mut c_void, src: *const c_void, c: c_int, n: size_t) -> *mut c_void
```



## libc::unix::memchr

*Function*

```rust
fn memchr(cx: *const c_void, c: c_int, n: size_t) -> *mut c_void
```



## libc::unix::memcmp

*Function*

```rust
fn memcmp(cx: *const c_void, ct: *const c_void, n: size_t) -> c_int
```



## libc::unix::memcpy

*Function*

```rust
fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void
```



## libc::unix::memmove

*Function*

```rust
fn memmove(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void
```



## libc::unix::memset

*Function*

```rust
fn memset(dest: *mut c_void, c: c_int, n: size_t) -> *mut c_void
```



## libc::unix::mkdir

*Function*

```rust
fn mkdir(path: *const c_char, mode: mode_t) -> c_int
```



## libc::unix::mkdirat

*Function*

```rust
fn mkdirat(dirfd: c_int, pathname: *const c_char, mode: mode_t) -> c_int
```



## libc::unix::mkdtemp

*Function*

```rust
fn mkdtemp(template: *mut c_char) -> *mut c_char
```



## libc::unix::mkfifo

*Function*

```rust
fn mkfifo(path: *const c_char, mode: mode_t) -> c_int
```



## libc::unix::mknod

*Function*

```rust
fn mknod(pathname: *const c_char, mode: mode_t, dev: crate::dev_t) -> c_int
```



## libc::unix::mkstemp

*Function*

```rust
fn mkstemp(template: *mut c_char) -> c_int
```



## libc::unix::mktime

*Function*

```rust
fn mktime(tm: *mut tm) -> time_t
```



## libc::unix::mlock

*Function*

```rust
fn mlock(addr: *const c_void, len: size_t) -> c_int
```



## libc::unix::mlockall

*Function*

```rust
fn mlockall(flags: c_int) -> c_int
```



## libc::unix::mmap

*Function*

```rust
fn mmap(addr: *mut c_void, len: size_t, prot: c_int, flags: c_int, fd: c_int, offset: off_t) -> *mut c_void
```



## libc::unix::munlock

*Function*

```rust
fn munlock(addr: *const c_void, len: size_t) -> c_int
```



## libc::unix::munlockall

*Function*

```rust
fn munlockall() -> c_int
```



## libc::unix::munmap

*Function*

```rust
fn munmap(addr: *mut c_void, len: size_t) -> c_int
```



## libc::unix::nanosleep

*Function*

```rust
fn nanosleep(rqtp: *const timespec, rmtp: *mut timespec) -> c_int
```



## libc::unix::nice

*Function*

```rust
fn nice(incr: c_int) -> c_int
```



## libc::unix::ntohl

*Function*

```rust
fn ntohl(netlong: u32) -> u32
```



## libc::unix::ntohs

*Function*

```rust
fn ntohs(netshort: u16) -> u16
```



## libc::unix::open

*Function*

```rust
fn open(path: *const c_char, oflag: c_int) -> c_int
```



## libc::unix::open_memstream

*Function*

```rust
fn open_memstream(ptr: *mut *mut c_char, sizeloc: *mut size_t) -> *mut FILE
```



## libc::unix::open_wmemstream

*Function*

```rust
fn open_wmemstream(ptr: *mut *mut wchar_t, sizeloc: *mut size_t) -> *mut FILE
```



## libc::unix::openat

*Function*

```rust
fn openat(dirfd: c_int, pathname: *const c_char, flags: c_int) -> c_int
```



## libc::unix::opendir

*Function*

```rust
fn opendir(dirname: *const c_char) -> *mut crate::DIR
```



## libc::unix::openlog

*Function*

```rust
fn openlog(ident: *const c_char, logopt: c_int, facility: c_int)
```



## libc::unix::pathconf

*Function*

```rust
fn pathconf(path: *const c_char, name: c_int) -> c_long
```



## libc::unix::pause

*Function*

```rust
fn pause() -> c_int
```



## libc::unix::pclose

*Function*

```rust
fn pclose(stream: *mut crate::FILE) -> c_int
```



## libc::unix::perror

*Function*

```rust
fn perror(s: *const c_char)
```



## libc::unix::pid_t

*Type Alias*: `i32`



## libc::unix::pipe

*Function*

```rust
fn pipe(fds: *mut c_int) -> c_int
```



## libc::unix::poll

*Function*

```rust
fn poll(fds: *mut pollfd, nfds: nfds_t, timeout: c_int) -> c_int
```



## libc::unix::pollfd

*Struct*

**Fields:**
- `fd: c_int`
- `events: c_short`
- `revents: c_short`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> pollfd`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::posix_memalign

*Function*

```rust
fn posix_memalign(memptr: *mut *mut c_void, align: size_t, size: size_t) -> c_int
```



## libc::unix::posix_openpt

*Function*

```rust
fn posix_openpt(flags: c_int) -> c_int
```



## libc::unix::pread

*Function*

```rust
fn pread(fd: c_int, buf: *mut c_void, count: size_t, offset: off_t) -> ssize_t
```



## libc::unix::printf

*Function*

```rust
fn printf(format: *const c_char) -> c_int
```



## libc::unix::protoent

*Struct*

**Fields:**
- `p_name: *mut c_char`
- `p_aliases: *mut *mut c_char`
- `p_proto: c_int`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> protoent`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::pselect

*Function*

```rust
fn pselect(nfds: c_int, readfds: *mut fd_set, writefds: *mut fd_set, errorfds: *mut fd_set, timeout: *const timespec, sigmask: *const sigset_t) -> c_int
```



## libc::unix::pthread_attr_destroy

*Function*

```rust
fn pthread_attr_destroy(attr: *mut crate::pthread_attr_t) -> c_int
```



## libc::unix::pthread_attr_getstacksize

*Function*

```rust
fn pthread_attr_getstacksize(attr: *const crate::pthread_attr_t, stacksize: *mut size_t) -> c_int
```



## libc::unix::pthread_attr_init

*Function*

```rust
fn pthread_attr_init(attr: *mut crate::pthread_attr_t) -> c_int
```



## libc::unix::pthread_attr_setdetachstate

*Function*

```rust
fn pthread_attr_setdetachstate(attr: *mut crate::pthread_attr_t, state: c_int) -> c_int
```



## libc::unix::pthread_attr_setstacksize

*Function*

```rust
fn pthread_attr_setstacksize(attr: *mut crate::pthread_attr_t, stack_size: size_t) -> c_int
```



## libc::unix::pthread_cond_broadcast

*Function*

```rust
fn pthread_cond_broadcast(cond: *mut crate::pthread_cond_t) -> c_int
```



## libc::unix::pthread_cond_destroy

*Function*

```rust
fn pthread_cond_destroy(cond: *mut crate::pthread_cond_t) -> c_int
```



## libc::unix::pthread_cond_init

*Function*

```rust
fn pthread_cond_init(cond: *mut crate::pthread_cond_t, attr: *const crate::pthread_condattr_t) -> c_int
```



## libc::unix::pthread_cond_signal

*Function*

```rust
fn pthread_cond_signal(cond: *mut crate::pthread_cond_t) -> c_int
```



## libc::unix::pthread_cond_timedwait

*Function*

```rust
fn pthread_cond_timedwait(cond: *mut crate::pthread_cond_t, lock: *mut crate::pthread_mutex_t, abstime: *const crate::timespec) -> c_int
```



## libc::unix::pthread_cond_wait

*Function*

```rust
fn pthread_cond_wait(cond: *mut crate::pthread_cond_t, lock: *mut crate::pthread_mutex_t) -> c_int
```



## libc::unix::pthread_condattr_destroy

*Function*

```rust
fn pthread_condattr_destroy(attr: *mut crate::pthread_condattr_t) -> c_int
```



## libc::unix::pthread_condattr_init

*Function*

```rust
fn pthread_condattr_init(attr: *mut crate::pthread_condattr_t) -> c_int
```



## libc::unix::pthread_detach

*Function*

```rust
fn pthread_detach(thread: crate::pthread_t) -> c_int
```



## libc::unix::pthread_equal

*Function*

```rust
fn pthread_equal(t1: crate::pthread_t, t2: crate::pthread_t) -> c_int
```



## libc::unix::pthread_exit

*Function*

```rust
fn pthread_exit(value: *mut c_void) -> never
```



## libc::unix::pthread_getspecific

*Function*

```rust
fn pthread_getspecific(key: crate::pthread_key_t) -> *mut c_void
```



## libc::unix::pthread_join

*Function*

```rust
fn pthread_join(native: crate::pthread_t, value: *mut *mut c_void) -> c_int
```



## libc::unix::pthread_key_create

*Function*

```rust
fn pthread_key_create(key: *mut crate::pthread_key_t, dtor: Option<fn(...)>) -> c_int
```



## libc::unix::pthread_key_delete

*Function*

```rust
fn pthread_key_delete(key: crate::pthread_key_t) -> c_int
```



## libc::unix::pthread_mutex_destroy

*Function*

```rust
fn pthread_mutex_destroy(lock: *mut crate::pthread_mutex_t) -> c_int
```



## libc::unix::pthread_mutex_init

*Function*

```rust
fn pthread_mutex_init(lock: *mut crate::pthread_mutex_t, attr: *const crate::pthread_mutexattr_t) -> c_int
```



## libc::unix::pthread_mutex_lock

*Function*

```rust
fn pthread_mutex_lock(lock: *mut crate::pthread_mutex_t) -> c_int
```



## libc::unix::pthread_mutex_trylock

*Function*

```rust
fn pthread_mutex_trylock(lock: *mut crate::pthread_mutex_t) -> c_int
```



## libc::unix::pthread_mutex_unlock

*Function*

```rust
fn pthread_mutex_unlock(lock: *mut crate::pthread_mutex_t) -> c_int
```



## libc::unix::pthread_mutexattr_destroy

*Function*

```rust
fn pthread_mutexattr_destroy(attr: *mut crate::pthread_mutexattr_t) -> c_int
```



## libc::unix::pthread_mutexattr_init

*Function*

```rust
fn pthread_mutexattr_init(attr: *mut crate::pthread_mutexattr_t) -> c_int
```



## libc::unix::pthread_mutexattr_settype

*Function*

```rust
fn pthread_mutexattr_settype(attr: *mut crate::pthread_mutexattr_t, _type: c_int) -> c_int
```



## libc::unix::pthread_rwlock_destroy

*Function*

```rust
fn pthread_rwlock_destroy(lock: *mut crate::pthread_rwlock_t) -> c_int
```



## libc::unix::pthread_rwlock_init

*Function*

```rust
fn pthread_rwlock_init(lock: *mut crate::pthread_rwlock_t, attr: *const crate::pthread_rwlockattr_t) -> c_int
```



## libc::unix::pthread_rwlock_rdlock

*Function*

```rust
fn pthread_rwlock_rdlock(lock: *mut crate::pthread_rwlock_t) -> c_int
```



## libc::unix::pthread_rwlock_tryrdlock

*Function*

```rust
fn pthread_rwlock_tryrdlock(lock: *mut crate::pthread_rwlock_t) -> c_int
```



## libc::unix::pthread_rwlock_trywrlock

*Function*

```rust
fn pthread_rwlock_trywrlock(lock: *mut crate::pthread_rwlock_t) -> c_int
```



## libc::unix::pthread_rwlock_unlock

*Function*

```rust
fn pthread_rwlock_unlock(lock: *mut crate::pthread_rwlock_t) -> c_int
```



## libc::unix::pthread_rwlock_wrlock

*Function*

```rust
fn pthread_rwlock_wrlock(lock: *mut crate::pthread_rwlock_t) -> c_int
```



## libc::unix::pthread_rwlockattr_destroy

*Function*

```rust
fn pthread_rwlockattr_destroy(attr: *mut crate::pthread_rwlockattr_t) -> c_int
```



## libc::unix::pthread_rwlockattr_init

*Function*

```rust
fn pthread_rwlockattr_init(attr: *mut crate::pthread_rwlockattr_t) -> c_int
```



## libc::unix::pthread_self

*Function*

```rust
fn pthread_self() -> crate::pthread_t
```



## libc::unix::pthread_setspecific

*Function*

```rust
fn pthread_setspecific(key: crate::pthread_key_t, value: *const c_void) -> c_int
```



## libc::unix::ptrdiff_t

*Type Alias*: `isize`



## libc::unix::ptsname

*Function*

```rust
fn ptsname(fd: c_int) -> *mut c_char
```



## libc::unix::putchar

*Function*

```rust
fn putchar(c: c_int) -> c_int
```



## libc::unix::putchar_unlocked

*Function*

```rust
fn putchar_unlocked(c: c_int) -> c_int
```



## libc::unix::putenv

*Function*

```rust
fn putenv(string: *mut c_char) -> c_int
```



## libc::unix::puts

*Function*

```rust
fn puts(s: *const c_char) -> c_int
```



## libc::unix::pwrite

*Function*

```rust
fn pwrite(fd: c_int, buf: *const c_void, count: size_t, offset: off_t) -> ssize_t
```



## libc::unix::qsort

*Function*

```rust
fn qsort(base: *mut c_void, num: size_t, size: size_t, compar: Option<fn(...)>)
```



## libc::unix::raise

*Function*

```rust
fn raise(signum: c_int) -> c_int
```



## libc::unix::read

*Function*

```rust
fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t
```



## libc::unix::readdir

*Function*

```rust
fn readdir(dirp: *mut crate::DIR) -> *mut crate::dirent
```



## libc::unix::readdir_r

*Function*

The 64-bit libc on Solaris and illumos only has readdir_r. If a
32-bit Solaris or illumos target is ever created, it should use
__posix_readdir_r. See libc(3LIB) on Solaris or illumos:
https://illumos.org/man/3lib/libc
https://docs.oracle.com/cd/E36784_01/html/E36873/libc-3lib.html
https://www.unix.com/man-page/opensolaris/3LIB/libc/

```rust
fn readdir_r(dirp: *mut crate::DIR, entry: *mut crate::dirent, result: *mut *mut crate::dirent) -> c_int
```



## libc::unix::readlink

*Function*

```rust
fn readlink(path: *const c_char, buf: *mut c_char, bufsz: size_t) -> ssize_t
```



## libc::unix::readlinkat

*Function*

```rust
fn readlinkat(dirfd: c_int, pathname: *const c_char, buf: *mut c_char, bufsiz: size_t) -> ssize_t
```



## libc::unix::realloc

*Function*

```rust
fn realloc(p: *mut c_void, size: size_t) -> *mut c_void
```



## libc::unix::realpath

*Function*

```rust
fn realpath(pathname: *const c_char, resolved: *mut c_char) -> *mut c_char
```



## libc::unix::recv

*Function*

```rust
fn recv(socket: c_int, buf: *mut c_void, len: size_t, flags: c_int) -> ssize_t
```



## libc::unix::remove

*Function*

```rust
fn remove(filename: *const c_char) -> c_int
```



## libc::unix::rename

*Function*

```rust
fn rename(oldname: *const c_char, newname: *const c_char) -> c_int
```



## libc::unix::renameat

*Function*

```rust
fn renameat(olddirfd: c_int, oldpath: *const c_char, newdirfd: c_int, newpath: *const c_char) -> c_int
```



## libc::unix::res_init

*Function*

```rust
fn res_init() -> c_int
```



## libc::unix::rewind

*Function*

```rust
fn rewind(stream: *mut FILE)
```



## libc::unix::rewinddir

*Function*

```rust
fn rewinddir(dirp: *mut crate::DIR)
```



## libc::unix::rlimit

*Struct*

**Fields:**
- `rlim_cur: rlim_t`
- `rlim_max: rlim_t`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> rlimit`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::rmdir

*Function*

```rust
fn rmdir(path: *const c_char) -> c_int
```



## libc::unix::rusage

*Struct*

**Fields:**
- `ru_utime: timeval`
- `ru_stime: timeval`
- `ru_maxrss: c_long`
- `ru_ixrss: c_long`
- `ru_idrss: c_long`
- `ru_isrss: c_long`
- `ru_minflt: c_long`
- `ru_majflt: c_long`
- `ru_nswap: c_long`
- `ru_inblock: c_long`
- `ru_oublock: c_long`
- `ru_msgsnd: c_long`
- `ru_msgrcv: c_long`
- `ru_nsignals: c_long`
- `ru_nvcsw: c_long`
- `ru_nivcsw: c_long`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> rusage`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::scanf

*Function*

```rust
fn scanf(format: *const c_char) -> c_int
```



## libc::unix::sched_yield

*Function*

```rust
fn sched_yield() -> c_int
```



## libc::unix::select

*Function*

```rust
fn select(nfds: c_int, readfds: *mut fd_set, writefds: *mut fd_set, errorfds: *mut fd_set, timeout: *mut timeval) -> c_int
```



## libc::unix::sem_post

*Function*

```rust
fn sem_post(sem: *mut sem_t) -> c_int
```



## libc::unix::sem_trywait

*Function*

```rust
fn sem_trywait(sem: *mut sem_t) -> c_int
```



## libc::unix::sem_wait

*Function*

```rust
fn sem_wait(sem: *mut sem_t) -> c_int
```



## libc::unix::send

*Function*

```rust
fn send(socket: c_int, buf: *const c_void, len: size_t, flags: c_int) -> ssize_t
```



## libc::unix::sendto

*Function*

```rust
fn sendto(socket: c_int, buf: *const c_void, len: size_t, flags: c_int, addr: *const sockaddr, addrlen: socklen_t) -> ssize_t
```



## libc::unix::servent

*Struct*

**Fields:**
- `s_name: *mut c_char`
- `s_aliases: *mut *mut c_char`
- `s_port: c_int`
- `s_proto: *mut c_char`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> servent`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::setbuf

*Function*

```rust
fn setbuf(stream: *mut FILE, buf: *mut c_char)
```



## libc::unix::setegid

*Function*

```rust
fn setegid(gid: gid_t) -> c_int
```



## libc::unix::setenv

*Function*

```rust
fn setenv(name: *const c_char, val: *const c_char, overwrite: c_int) -> c_int
```



## libc::unix::seteuid

*Function*

```rust
fn seteuid(uid: uid_t) -> c_int
```



## libc::unix::setgid

*Function*

```rust
fn setgid(gid: gid_t) -> c_int
```



## libc::unix::setlocale

*Function*

```rust
fn setlocale(category: c_int, locale: *const c_char) -> *mut c_char
```



## libc::unix::setlogmask

*Function*

```rust
fn setlogmask(maskpri: c_int) -> c_int
```



## libc::unix::setpgid

*Function*

```rust
fn setpgid(pid: pid_t, pgid: pid_t) -> c_int
```



## libc::unix::setregid

*Function*

```rust
fn setregid(rgid: gid_t, egid: gid_t) -> c_int
```



## libc::unix::setreuid

*Function*

```rust
fn setreuid(ruid: uid_t, euid: uid_t) -> c_int
```



## libc::unix::setservent

*Function*

```rust
fn setservent(stayopen: c_int)
```



## libc::unix::setsid

*Function*

```rust
fn setsid() -> pid_t
```



## libc::unix::setsockopt

*Function*

```rust
fn setsockopt(socket: c_int, level: c_int, name: c_int, value: *const c_void, option_len: socklen_t) -> c_int
```



## libc::unix::setuid

*Function*

```rust
fn setuid(uid: uid_t) -> c_int
```



## libc::unix::setvbuf

*Function*

```rust
fn setvbuf(stream: *mut FILE, buffer: *mut c_char, mode: c_int, size: size_t) -> c_int
```



## libc::unix::shutdown

*Function*

```rust
fn shutdown(socket: c_int, how: c_int) -> c_int
```



## libc::unix::sigaction

*Function*

```rust
fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int
```



## libc::unix::sigaddset

*Function*

```rust
fn sigaddset(set: *mut sigset_t, signum: c_int) -> c_int
```



## libc::unix::sigdelset

*Function*

```rust
fn sigdelset(set: *mut sigset_t, signum: c_int) -> c_int
```



## libc::unix::sigemptyset

*Function*

```rust
fn sigemptyset(set: *mut sigset_t) -> c_int
```



## libc::unix::sigfillset

*Function*

```rust
fn sigfillset(set: *mut sigset_t) -> c_int
```



## libc::unix::sighandler_t

*Type Alias*: `size_t`



## libc::unix::sigismember

*Function*

```rust
fn sigismember(set: *const sigset_t, signum: c_int) -> c_int
```



## libc::unix::signal

*Function*

```rust
fn signal(signum: c_int, handler: sighandler_t) -> sighandler_t
```



## libc::unix::sigpending

*Function*

```rust
fn sigpending(set: *mut sigset_t) -> c_int
```



## libc::unix::sigprocmask

*Function*

```rust
fn sigprocmask(how: c_int, set: *const sigset_t, oldset: *mut sigset_t) -> c_int
```



## libc::unix::sigqueue

*Function*

```rust
fn sigqueue(pid: pid_t, sig: c_int, value: crate::sigval) -> c_int
```



## libc::unix::sigval

*Struct*

**Fields:**
- `sival_ptr: *mut c_void`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> sigval`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::size_t

*Type Alias*: `usize`



## libc::unix::sleep

*Function*

```rust
fn sleep(secs: c_uint) -> c_uint
```



## libc::unix::snprintf

*Function*

```rust
fn snprintf(s: *mut c_char, n: size_t, format: *const c_char) -> c_int
```



## libc::unix::socket

*Function*

```rust
fn socket(domain: c_int, ty: c_int, protocol: c_int) -> c_int
```



## libc::unix::socketpair

*Function*

```rust
fn socketpair(domain: c_int, type_: c_int, protocol: c_int, socket_vector: *mut c_int) -> c_int
```



## libc::unix::sprintf

*Function*

```rust
fn sprintf(s: *mut c_char, format: *const c_char) -> c_int
```



## libc::unix::sscanf

*Function*

```rust
fn sscanf(s: *const c_char, format: *const c_char) -> c_int
```



## libc::unix::ssize_t

*Type Alias*: `isize`



## libc::unix::stat

*Function*

```rust
fn stat(path: *const c_char, buf: *mut stat) -> c_int
```



## libc::unix::statvfs

*Function*

```rust
fn statvfs(path: *const c_char, buf: *mut crate::statvfs) -> c_int
```



## libc::unix::stpcpy

*Function*

```rust
fn stpcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char
```



## libc::unix::stpncpy

*Function*

```rust
fn stpncpy(dst: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char
```



## libc::unix::strcasecmp

*Function*

```rust
fn strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int
```



## libc::unix::strcasestr

*Function*

```rust
fn strcasestr(cs: *const c_char, ct: *const c_char) -> *mut c_char
```



## libc::unix::strcat

*Function*

```rust
fn strcat(s: *mut c_char, ct: *const c_char) -> *mut c_char
```



## libc::unix::strchr

*Function*

```rust
fn strchr(cs: *const c_char, c: c_int) -> *mut c_char
```



## libc::unix::strcmp

*Function*

```rust
fn strcmp(cs: *const c_char, ct: *const c_char) -> c_int
```



## libc::unix::strcoll

*Function*

```rust
fn strcoll(cs: *const c_char, ct: *const c_char) -> c_int
```



## libc::unix::strcpy

*Function*

```rust
fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char
```



## libc::unix::strcspn

*Function*

```rust
fn strcspn(cs: *const c_char, ct: *const c_char) -> size_t
```



## libc::unix::strdup

*Function*

```rust
fn strdup(cs: *const c_char) -> *mut c_char
```



## libc::unix::strerror

*Function*

```rust
fn strerror(n: c_int) -> *mut c_char
```



## libc::unix::strlen

*Function*

```rust
fn strlen(cs: *const c_char) -> size_t
```



## libc::unix::strncasecmp

*Function*

```rust
fn strncasecmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int
```



## libc::unix::strncat

*Function*

```rust
fn strncat(s: *mut c_char, ct: *const c_char, n: size_t) -> *mut c_char
```



## libc::unix::strncmp

*Function*

```rust
fn strncmp(cs: *const c_char, ct: *const c_char, n: size_t) -> c_int
```



## libc::unix::strncpy

*Function*

```rust
fn strncpy(dst: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char
```



## libc::unix::strndup

*Function*

```rust
fn strndup(cs: *const c_char, n: size_t) -> *mut c_char
```



## libc::unix::strnlen

*Function*

```rust
fn strnlen(cs: *const c_char, maxlen: size_t) -> size_t
```



## libc::unix::strpbrk

*Function*

```rust
fn strpbrk(cs: *const c_char, ct: *const c_char) -> *mut c_char
```



## libc::unix::strrchr

*Function*

```rust
fn strrchr(cs: *const c_char, c: c_int) -> *mut c_char
```



## libc::unix::strsignal

*Function*

```rust
fn strsignal(sig: c_int) -> *mut c_char
```



## libc::unix::strspn

*Function*

```rust
fn strspn(cs: *const c_char, ct: *const c_char) -> size_t
```



## libc::unix::strstr

*Function*

```rust
fn strstr(cs: *const c_char, ct: *const c_char) -> *mut c_char
```



## libc::unix::strtod

*Function*

```rust
fn strtod(s: *const c_char, endp: *mut *mut c_char) -> c_double
```



## libc::unix::strtof

*Function*

```rust
fn strtof(s: *const c_char, endp: *mut *mut c_char) -> c_float
```



## libc::unix::strtok

*Function*

```rust
fn strtok(s: *mut c_char, t: *const c_char) -> *mut c_char
```



## libc::unix::strtok_r

*Function*

```rust
fn strtok_r(s: *mut c_char, t: *const c_char, p: *mut *mut c_char) -> *mut c_char
```



## libc::unix::strtol

*Function*

```rust
fn strtol(s: *const c_char, endp: *mut *mut c_char, base: c_int) -> c_long
```



## libc::unix::strtoll

*Function*

```rust
fn strtoll(s: *const c_char, endp: *mut *mut c_char, base: c_int) -> c_longlong
```



## libc::unix::strtoul

*Function*

```rust
fn strtoul(s: *const c_char, endp: *mut *mut c_char, base: c_int) -> c_ulong
```



## libc::unix::strtoull

*Function*

```rust
fn strtoull(s: *const c_char, endp: *mut *mut c_char, base: c_int) -> c_ulonglong
```



## libc::unix::strxfrm

*Function*

```rust
fn strxfrm(s: *mut c_char, ct: *const c_char, n: size_t) -> size_t
```



## libc::unix::symlink

*Function*

```rust
fn symlink(path1: *const c_char, path2: *const c_char) -> c_int
```



## libc::unix::symlinkat

*Function*

```rust
fn symlinkat(target: *const c_char, newdirfd: c_int, linkpath: *const c_char) -> c_int
```



## libc::unix::sysconf

*Function*

```rust
fn sysconf(name: c_int) -> c_long
```



## libc::unix::syslog

*Function*

```rust
fn syslog(priority: c_int, message: *const c_char)
```



## libc::unix::system

*Function*

```rust
fn system(s: *const c_char) -> c_int
```



## libc::unix::tcdrain

*Function*

```rust
fn tcdrain(fd: c_int) -> c_int
```



## libc::unix::tcflow

*Function*

```rust
fn tcflow(fd: c_int, action: c_int) -> c_int
```



## libc::unix::tcflush

*Function*

```rust
fn tcflush(fd: c_int, action: c_int) -> c_int
```



## libc::unix::tcgetattr

*Function*

```rust
fn tcgetattr(fd: c_int, termios: *mut crate::termios) -> c_int
```



## libc::unix::tcgetpgrp

*Function*

```rust
fn tcgetpgrp(fd: c_int) -> pid_t
```



## libc::unix::tcgetsid

*Function*

```rust
fn tcgetsid(fd: c_int) -> crate::pid_t
```



## libc::unix::tcsendbreak

*Function*

```rust
fn tcsendbreak(fd: c_int, duration: c_int) -> c_int
```



## libc::unix::tcsetattr

*Function*

```rust
fn tcsetattr(fd: c_int, optional_actions: c_int, termios: *const crate::termios) -> c_int
```



## libc::unix::tcsetpgrp

*Function*

```rust
fn tcsetpgrp(fd: c_int, pgrp: crate::pid_t) -> c_int
```



## libc::unix::time

*Function*

```rust
fn time(time: *mut time_t) -> time_t
```



## libc::unix::timegm

*Function*

```rust
fn timegm(tm: *mut crate::tm) -> time_t
```



## libc::unix::times

*Function*

```rust
fn times(buf: *mut crate::tms) -> crate::clock_t
```



## libc::unix::timeval

*Struct*

**Fields:**
- `tv_sec: time_t`
- `tv_usec: crate::suseconds_t`

**Traits:** Copy

**Trait Implementations:**

- **Default**
  - `fn default() -> timeval`
- **Clone**
  - `fn clone(self: &Self) -> timeval`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::tmpfile

*Function*

```rust
fn tmpfile() -> *mut FILE
```



## libc::unix::tmpnam

*Function*

```rust
fn tmpnam(ptr: *mut c_char) -> *mut c_char
```



## libc::unix::tms

*Struct*

**Fields:**
- `tms_utime: crate::clock_t`
- `tms_stime: crate::clock_t`
- `tms_cutime: crate::clock_t`
- `tms_cstime: crate::clock_t`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> tms`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::tolower

*Function*

```rust
fn tolower(c: c_int) -> c_int
```



## libc::unix::toupper

*Function*

```rust
fn toupper(c: c_int) -> c_int
```



## libc::unix::truncate

*Function*

```rust
fn truncate(path: *const c_char, length: off_t) -> c_int
```



## libc::unix::ttyname

*Function*

```rust
fn ttyname(fd: c_int) -> *mut c_char
```



## libc::unix::ttyname_r

*Function*

```rust
fn ttyname_r(fd: c_int, buf: *mut c_char, buflen: size_t) -> c_int
```



## libc::unix::uid_t

*Type Alias*: `u32`



## libc::unix::uintmax_t

*Type Alias*: `u64`



## libc::unix::uintptr_t

*Type Alias*: `usize`



## libc::unix::umask

*Function*

```rust
fn umask(mask: mode_t) -> mode_t
```



## libc::unix::ungetc

*Function*

```rust
fn ungetc(c: c_int, stream: *mut FILE) -> c_int
```



## libc::unix::unlink

*Function*

```rust
fn unlink(c: *const c_char) -> c_int
```



## libc::unix::unlinkat

*Function*

```rust
fn unlinkat(dirfd: c_int, pathname: *const c_char, flags: c_int) -> c_int
```



## libc::unix::unlockpt

*Function*

```rust
fn unlockpt(fd: c_int) -> c_int
```



## libc::unix::unsetenv

*Function*

```rust
fn unsetenv(name: *const c_char) -> c_int
```



## libc::unix::usleep

*Function*

```rust
fn usleep(secs: c_uint) -> c_int
```



## libc::unix::utimbuf

*Struct*

**Fields:**
- `actime: time_t`
- `modtime: time_t`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> utimbuf`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::utime

*Function*

```rust
fn utime(file: *const c_char, buf: *const utimbuf) -> c_int
```



## libc::unix::utimes

*Function*

```rust
fn utimes(filename: *const c_char, times: *const crate::timeval) -> c_int
```



## libc::unix::wait

*Function*

```rust
fn wait(status: *mut c_int) -> pid_t
```



## libc::unix::waitpid

*Function*

```rust
fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t
```



## libc::unix::wcslen

*Function*

```rust
fn wcslen(buf: *const wchar_t) -> size_t
```



## libc::unix::wcstombs

*Function*

```rust
fn wcstombs(dest: *mut c_char, src: *const wchar_t, n: size_t) -> size_t
```



## libc::unix::winsize

*Struct*

**Fields:**
- `ws_row: c_ushort`
- `ws_col: c_ushort`
- `ws_xpixel: c_ushort`
- `ws_ypixel: c_ushort`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> winsize`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::wmemchr

*Function*

```rust
fn wmemchr(cx: *const wchar_t, c: wchar_t, n: size_t) -> *mut wchar_t
```



## libc::unix::write

*Function*

```rust
fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t
```



