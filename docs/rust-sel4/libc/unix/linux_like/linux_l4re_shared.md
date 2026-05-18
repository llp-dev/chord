**libc > unix > linux_like > linux_l4re_shared**

# Module: unix::linux_like::linux_l4re_shared

## Contents

**Structs**

- [`Elf32_Ehdr`](#elf32_ehdr)
- [`Elf32_Phdr`](#elf32_phdr)
- [`Elf32_Shdr`](#elf32_shdr)
- [`Elf32_Sym`](#elf32_sym)
- [`Elf64_Ehdr`](#elf64_ehdr)
- [`Elf64_Phdr`](#elf64_phdr)
- [`Elf64_Shdr`](#elf64_shdr)
- [`Elf64_Sym`](#elf64_sym)
- [`__c_anonymous_elf32_rel`](#__c_anonymous_elf32_rel)
- [`__c_anonymous_elf32_rela`](#__c_anonymous_elf32_rela)
- [`__c_anonymous_elf64_rel`](#__c_anonymous_elf64_rel)
- [`__c_anonymous_elf64_rela`](#__c_anonymous_elf64_rela)
- [`__c_anonymous_ifru_map`](#__c_anonymous_ifru_map)
- [`arpd_request`](#arpd_request)
- [`cpu_set_t`](#cpu_set_t)
- [`dirent`](#dirent)
- [`dirent64`](#dirent64)
- [`dl_phdr_info`](#dl_phdr_info)
- [`fsid_t`](#fsid_t)
- [`glob_t`](#glob_t)
- [`ifconf`](#ifconf) - Structure used in SIOCGIFCONF request.  Used to retrieve interface configuration for
- [`ifreq`](#ifreq)
- [`in6_pktinfo`](#in6_pktinfo)
- [`itimerspec`](#itimerspec)
- [`mntent`](#mntent)
- [`option`](#option)
- [`packet_mreq`](#packet_mreq)
- [`passwd`](#passwd)
- [`regmatch_t`](#regmatch_t)
- [`rlimit64`](#rlimit64)
- [`sembuf`](#sembuf)
- [`spwd`](#spwd)
- [`ucred`](#ucred)

**Unions**

- [`__c_anonymous_ifc_ifcu`](#__c_anonymous_ifc_ifcu)
- [`__c_anonymous_ifr_ifru`](#__c_anonymous_ifr_ifru)

**Functions**

- [`CMSG_NXTHDR`](#cmsg_nxthdr)
- [`CPU_ALLOC_SIZE`](#cpu_alloc_size)
- [`CPU_CLR`](#cpu_clr)
- [`CPU_COUNT`](#cpu_count)
- [`CPU_COUNT_S`](#cpu_count_s)
- [`CPU_EQUAL`](#cpu_equal)
- [`CPU_ISSET`](#cpu_isset)
- [`CPU_SET`](#cpu_set)
- [`CPU_ZERO`](#cpu_zero)
- [`ELF32_R_INFO`](#elf32_r_info)
- [`ELF32_R_SYM`](#elf32_r_sym)
- [`ELF32_R_TYPE`](#elf32_r_type)
- [`ELF64_R_INFO`](#elf64_r_info)
- [`ELF64_R_SYM`](#elf64_r_sym)
- [`ELF64_R_TYPE`](#elf64_r_type)
- [`IPTOS_PREC`](#iptos_prec)
- [`IPTOS_TOS`](#iptos_tos)
- [`RT_ADDRCLASS`](#rt_addrclass)
- [`RT_LOCALADDR`](#rt_localaddr)
- [`RT_TOS`](#rt_tos)
- [`__errno_location`](#__errno_location)
- [`abs`](#abs)
- [`addmntent`](#addmntent)
- [`aio_cancel`](#aio_cancel)
- [`aio_error`](#aio_error)
- [`aio_fsync`](#aio_fsync)
- [`aio_read`](#aio_read)
- [`aio_return`](#aio_return)
- [`aio_suspend`](#aio_suspend)
- [`aio_write`](#aio_write)
- [`copy_file_range`](#copy_file_range)
- [`daemon`](#daemon)
- [`dl_iterate_phdr`](#dl_iterate_phdr)
- [`drand48`](#drand48)
- [`endgrent`](#endgrent)
- [`endmntent`](#endmntent)
- [`endpwent`](#endpwent)
- [`endspent`](#endspent)
- [`erand48`](#erand48)
- [`faccessat`](#faccessat)
- [`freopen64`](#freopen64)
- [`fseeko64`](#fseeko64)
- [`fsetpos64`](#fsetpos64)
- [`ftello64`](#ftello64)
- [`futimes`](#futimes)
- [`getgrent`](#getgrent)
- [`getgrgid`](#getgrgid)
- [`getgrgid_r`](#getgrgid_r)
- [`getgrnam`](#getgrnam)
- [`getgrnam_r`](#getgrnam_r)
- [`getloadavg`](#getloadavg)
- [`getmntent`](#getmntent)
- [`getnameinfo`](#getnameinfo)
- [`getopt_long`](#getopt_long)
- [`getpwent`](#getpwent)
- [`getspent`](#getspent)
- [`getspnam`](#getspnam)
- [`gettid`](#gettid)
- [`glob`](#glob)
- [`globfree`](#globfree)
- [`hasmntopt`](#hasmntopt)
- [`iconv`](#iconv)
- [`iconv_close`](#iconv_close)
- [`iconv_open`](#iconv_open)
- [`initgroups`](#initgroups)
- [`ioperm`](#ioperm)
- [`iopl`](#iopl)
- [`jrand48`](#jrand48)
- [`labs`](#labs)
- [`lio_listio`](#lio_listio)
- [`lrand48`](#lrand48)
- [`madvise`](#madvise)
- [`major`](#major)
- [`makedev`](#makedev)
- [`memmem`](#memmem)
- [`minor`](#minor)
- [`mount`](#mount)
- [`mprotect`](#mprotect)
- [`mremap`](#mremap)
- [`msync`](#msync)
- [`nl_langinfo`](#nl_langinfo)
- [`nl_langinfo_l`](#nl_langinfo_l)
- [`nrand48`](#nrand48)
- [`popen`](#popen)
- [`ppoll`](#ppoll)
- [`prctl`](#prctl)
- [`preadv`](#preadv)
- [`process_vm_readv`](#process_vm_readv)
- [`process_vm_writev`](#process_vm_writev)
- [`pwritev`](#pwritev)
- [`rand`](#rand)
- [`recvfrom`](#recvfrom)
- [`regcomp`](#regcomp)
- [`regerror`](#regerror)
- [`regexec`](#regexec)
- [`regfree`](#regfree)
- [`sched_get_priority_max`](#sched_get_priority_max)
- [`sched_get_priority_min`](#sched_get_priority_min)
- [`sched_getaffinity`](#sched_getaffinity)
- [`sched_getcpu`](#sched_getcpu)
- [`seekdir`](#seekdir)
- [`sem_close`](#sem_close)
- [`sem_getvalue`](#sem_getvalue)
- [`sem_open`](#sem_open)
- [`sem_timedwait`](#sem_timedwait)
- [`sem_unlink`](#sem_unlink)
- [`setgrent`](#setgrent)
- [`sethostname`](#sethostname)
- [`setmntent`](#setmntent)
- [`setpwent`](#setpwent)
- [`setspent`](#setspent)
- [`settimeofday`](#settimeofday)
- [`shmat`](#shmat)
- [`shmctl`](#shmctl)
- [`shmdt`](#shmdt)
- [`shmget`](#shmget)
- [`sigsuspend`](#sigsuspend)
- [`sigwait`](#sigwait)
- [`srand`](#srand)
- [`srand48`](#srand48)
- [`strerror_r`](#strerror_r)
- [`sysinfo`](#sysinfo)
- [`telldir`](#telldir)
- [`timer_create`](#timer_create)
- [`timer_delete`](#timer_delete)
- [`timer_getoverrun`](#timer_getoverrun)
- [`timer_gettime`](#timer_gettime)
- [`timer_settime`](#timer_settime)

**Constants**

- [`ABDAY_1`](#abday_1)
- [`ABDAY_2`](#abday_2)
- [`ABDAY_3`](#abday_3)
- [`ABDAY_4`](#abday_4)
- [`ABDAY_5`](#abday_5)
- [`ABDAY_6`](#abday_6)
- [`ABDAY_7`](#abday_7)
- [`ABMON_1`](#abmon_1)
- [`ABMON_10`](#abmon_10)
- [`ABMON_11`](#abmon_11)
- [`ABMON_12`](#abmon_12)
- [`ABMON_2`](#abmon_2)
- [`ABMON_3`](#abmon_3)
- [`ABMON_4`](#abmon_4)
- [`ABMON_5`](#abmon_5)
- [`ABMON_6`](#abmon_6)
- [`ABMON_7`](#abmon_7)
- [`ABMON_8`](#abmon_8)
- [`ABMON_9`](#abmon_9)
- [`AIO_ALLDONE`](#aio_alldone)
- [`AIO_CANCELED`](#aio_canceled)
- [`AIO_NOTCANCELED`](#aio_notcanceled)
- [`AI_ADDRCONFIG`](#ai_addrconfig)
- [`AI_ALL`](#ai_all)
- [`AI_CANONNAME`](#ai_canonname)
- [`AI_NUMERICHOST`](#ai_numerichost)
- [`AI_NUMERICSERV`](#ai_numericserv)
- [`AI_PASSIVE`](#ai_passive)
- [`AI_V4MAPPED`](#ai_v4mapped)
- [`ALT_DIGITS`](#alt_digits)
- [`AM_STR`](#am_str)
- [`ARPD_FLUSH`](#arpd_flush)
- [`ARPD_LOOKUP`](#arpd_lookup)
- [`ARPD_UPDATE`](#arpd_update)
- [`ATF_MAGIC`](#atf_magic)
- [`AT_BASE`](#at_base)
- [`AT_BASE_PLATFORM`](#at_base_platform)
- [`AT_CLKTCK`](#at_clktck)
- [`AT_EACCESS`](#at_eaccess)
- [`AT_EGID`](#at_egid)
- [`AT_ENTRY`](#at_entry)
- [`AT_EUID`](#at_euid)
- [`AT_EXECFD`](#at_execfd)
- [`AT_EXECFN`](#at_execfn)
- [`AT_FLAGS`](#at_flags)
- [`AT_GID`](#at_gid)
- [`AT_HWCAP`](#at_hwcap)
- [`AT_HWCAP2`](#at_hwcap2)
- [`AT_HWCAP3`](#at_hwcap3)
- [`AT_HWCAP4`](#at_hwcap4)
- [`AT_IGNORE`](#at_ignore)
- [`AT_MINSIGSTKSZ`](#at_minsigstksz)
- [`AT_NOTELF`](#at_notelf)
- [`AT_NULL`](#at_null)
- [`AT_PAGESZ`](#at_pagesz)
- [`AT_PHDR`](#at_phdr)
- [`AT_PHENT`](#at_phent)
- [`AT_PHNUM`](#at_phnum)
- [`AT_PLATFORM`](#at_platform)
- [`AT_RANDOM`](#at_random)
- [`AT_SECURE`](#at_secure)
- [`AT_SYSINFO_EHDR`](#at_sysinfo_ehdr)
- [`AT_UID`](#at_uid)
- [`CMSPAR`](#cmspar)
- [`CODESET`](#codeset)
- [`CRNCYSTR`](#crncystr)
- [`CSIGNAL`](#csignal)
- [`DAY_1`](#day_1)
- [`DAY_2`](#day_2)
- [`DAY_3`](#day_3)
- [`DAY_4`](#day_4)
- [`DAY_5`](#day_5)
- [`DAY_6`](#day_6)
- [`DAY_7`](#day_7)
- [`D_FMT`](#d_fmt)
- [`D_T_FMT`](#d_t_fmt)
- [`E2BIG`](#e2big)
- [`EACCES`](#eacces)
- [`EAGAIN`](#eagain)
- [`EAI_AGAIN`](#eai_again)
- [`EAI_BADFLAGS`](#eai_badflags)
- [`EAI_FAIL`](#eai_fail)
- [`EAI_FAMILY`](#eai_family)
- [`EAI_MEMORY`](#eai_memory)
- [`EAI_NODATA`](#eai_nodata)
- [`EAI_NONAME`](#eai_noname)
- [`EAI_OVERFLOW`](#eai_overflow)
- [`EAI_SERVICE`](#eai_service)
- [`EAI_SOCKTYPE`](#eai_socktype)
- [`EAI_SYSTEM`](#eai_system)
- [`EBADF`](#ebadf)
- [`EBUSY`](#ebusy)
- [`ECHILD`](#echild)
- [`EDOM`](#edom)
- [`EEXIST`](#eexist)
- [`EFAULT`](#efault)
- [`EFBIG`](#efbig)
- [`EINTR`](#eintr)
- [`EINVAL`](#einval)
- [`EIO`](#eio)
- [`EISDIR`](#eisdir)
- [`EI_ABIVERSION`](#ei_abiversion)
- [`EI_CLASS`](#ei_class)
- [`EI_DATA`](#ei_data)
- [`EI_MAG0`](#ei_mag0)
- [`EI_MAG1`](#ei_mag1)
- [`EI_MAG2`](#ei_mag2)
- [`EI_MAG3`](#ei_mag3)
- [`EI_NIDENT`](#ei_nident)
- [`EI_OSABI`](#ei_osabi)
- [`EI_PAD`](#ei_pad)
- [`EI_VERSION`](#ei_version)
- [`ELFCLASS32`](#elfclass32)
- [`ELFCLASS64`](#elfclass64)
- [`ELFCLASSNONE`](#elfclassnone)
- [`ELFCLASSNUM`](#elfclassnum)
- [`ELFDATA2LSB`](#elfdata2lsb)
- [`ELFDATA2MSB`](#elfdata2msb)
- [`ELFDATANONE`](#elfdatanone)
- [`ELFDATANUM`](#elfdatanum)
- [`ELFMAG0`](#elfmag0)
- [`ELFMAG1`](#elfmag1)
- [`ELFMAG2`](#elfmag2)
- [`ELFMAG3`](#elfmag3)
- [`ELFOSABI_AIX`](#elfosabi_aix)
- [`ELFOSABI_ARM`](#elfosabi_arm)
- [`ELFOSABI_FREEBSD`](#elfosabi_freebsd)
- [`ELFOSABI_GNU`](#elfosabi_gnu)
- [`ELFOSABI_HPUX`](#elfosabi_hpux)
- [`ELFOSABI_IRIX`](#elfosabi_irix)
- [`ELFOSABI_LINUX`](#elfosabi_linux)
- [`ELFOSABI_MODESTO`](#elfosabi_modesto)
- [`ELFOSABI_NETBSD`](#elfosabi_netbsd)
- [`ELFOSABI_NONE`](#elfosabi_none)
- [`ELFOSABI_OPENBSD`](#elfosabi_openbsd)
- [`ELFOSABI_SOLARIS`](#elfosabi_solaris)
- [`ELFOSABI_STANDALONE`](#elfosabi_standalone)
- [`ELFOSABI_SYSV`](#elfosabi_sysv)
- [`ELFOSABI_TRU64`](#elfosabi_tru64)
- [`EMFILE`](#emfile)
- [`EMLINK`](#emlink)
- [`EM_386`](#em_386)
- [`EM_68HC05`](#em_68hc05)
- [`EM_68HC08`](#em_68hc08)
- [`EM_68HC11`](#em_68hc11)
- [`EM_68HC12`](#em_68hc12)
- [`EM_68HC16`](#em_68hc16)
- [`EM_68K`](#em_68k)
- [`EM_860`](#em_860)
- [`EM_88K`](#em_88k)
- [`EM_960`](#em_960)
- [`EM_AARCH64`](#em_aarch64)
- [`EM_ALPHA`](#em_alpha)
- [`EM_ARC`](#em_arc)
- [`EM_ARC_A5`](#em_arc_a5)
- [`EM_ARM`](#em_arm)
- [`EM_AVR`](#em_avr)
- [`EM_COLDFIRE`](#em_coldfire)
- [`EM_CRIS`](#em_cris)
- [`EM_D10V`](#em_d10v)
- [`EM_D30V`](#em_d30v)
- [`EM_FAKE_ALPHA`](#em_fake_alpha)
- [`EM_FIREPATH`](#em_firepath)
- [`EM_FR20`](#em_fr20)
- [`EM_FR30`](#em_fr30)
- [`EM_FX66`](#em_fx66)
- [`EM_H8S`](#em_h8s)
- [`EM_H8_300`](#em_h8_300)
- [`EM_H8_300H`](#em_h8_300h)
- [`EM_H8_500`](#em_h8_500)
- [`EM_HUANY`](#em_huany)
- [`EM_IA_64`](#em_ia_64)
- [`EM_JAVELIN`](#em_javelin)
- [`EM_M32`](#em_m32)
- [`EM_M32R`](#em_m32r)
- [`EM_ME16`](#em_me16)
- [`EM_MIPS`](#em_mips)
- [`EM_MIPS_RS3_LE`](#em_mips_rs3_le)
- [`EM_MIPS_X`](#em_mips_x)
- [`EM_MMA`](#em_mma)
- [`EM_MMIX`](#em_mmix)
- [`EM_MN10200`](#em_mn10200)
- [`EM_MN10300`](#em_mn10300)
- [`EM_NCPU`](#em_ncpu)
- [`EM_NDR1`](#em_ndr1)
- [`EM_NONE`](#em_none)
- [`EM_OPENRISC`](#em_openrisc)
- [`EM_PARISC`](#em_parisc)
- [`EM_PCP`](#em_pcp)
- [`EM_PDSP`](#em_pdsp)
- [`EM_PJ`](#em_pj)
- [`EM_PPC`](#em_ppc)
- [`EM_PPC64`](#em_ppc64)
- [`EM_PRISM`](#em_prism)
- [`EM_RCE`](#em_rce)
- [`EM_RH32`](#em_rh32)
- [`EM_RISCV`](#em_riscv)
- [`EM_S370`](#em_s370)
- [`EM_S390`](#em_s390)
- [`EM_SH`](#em_sh)
- [`EM_SPARC`](#em_sparc)
- [`EM_SPARC32PLUS`](#em_sparc32plus)
- [`EM_SPARCV9`](#em_sparcv9)
- [`EM_ST100`](#em_st100)
- [`EM_ST19`](#em_st19)
- [`EM_ST7`](#em_st7)
- [`EM_ST9PLUS`](#em_st9plus)
- [`EM_STARCORE`](#em_starcore)
- [`EM_SVX`](#em_svx)
- [`EM_TILEGX`](#em_tilegx)
- [`EM_TILEPRO`](#em_tilepro)
- [`EM_TINYJ`](#em_tinyj)
- [`EM_TRICORE`](#em_tricore)
- [`EM_V800`](#em_v800)
- [`EM_V850`](#em_v850)
- [`EM_VAX`](#em_vax)
- [`EM_VPP500`](#em_vpp500)
- [`EM_X86_64`](#em_x86_64)
- [`EM_XTENSA`](#em_xtensa)
- [`EM_ZSP`](#em_zsp)
- [`ENFILE`](#enfile)
- [`ENODEV`](#enodev)
- [`ENOENT`](#enoent)
- [`ENOEXEC`](#enoexec)
- [`ENOMEM`](#enomem)
- [`ENOSPC`](#enospc)
- [`ENOTBLK`](#enotblk)
- [`ENOTDIR`](#enotdir)
- [`ENOTTY`](#enotty)
- [`ENXIO`](#enxio)
- [`EPERM`](#eperm)
- [`EPIPE`](#epipe)
- [`ERA`](#era)
- [`ERANGE`](#erange)
- [`ERA_D_FMT`](#era_d_fmt)
- [`ERA_D_T_FMT`](#era_d_t_fmt)
- [`ERA_T_FMT`](#era_t_fmt)
- [`EROFS`](#erofs)
- [`ESPIPE`](#espipe)
- [`ESRCH`](#esrch)
- [`ETXTBSY`](#etxtbsy)
- [`ET_CORE`](#et_core)
- [`ET_DYN`](#et_dyn)
- [`ET_EXEC`](#et_exec)
- [`ET_HIOS`](#et_hios)
- [`ET_HIPROC`](#et_hiproc)
- [`ET_LOOS`](#et_loos)
- [`ET_LOPROC`](#et_loproc)
- [`ET_NONE`](#et_none)
- [`ET_NUM`](#et_num)
- [`ET_REL`](#et_rel)
- [`EV_CURRENT`](#ev_current)
- [`EV_NONE`](#ev_none)
- [`EV_NUM`](#ev_num)
- [`EWOULDBLOCK`](#ewouldblock)
- [`EXDEV`](#exdev)
- [`F_LOCK`](#f_lock)
- [`F_TEST`](#f_test)
- [`F_TLOCK`](#f_tlock)
- [`F_ULOCK`](#f_ulock)
- [`GLOB_ABORTED`](#glob_aborted)
- [`GLOB_APPEND`](#glob_append)
- [`GLOB_DOOFFS`](#glob_dooffs)
- [`GLOB_ERR`](#glob_err)
- [`GLOB_MARK`](#glob_mark)
- [`GLOB_NOCHECK`](#glob_nocheck)
- [`GLOB_NOESCAPE`](#glob_noescape)
- [`GLOB_NOMATCH`](#glob_nomatch)
- [`GLOB_NOSORT`](#glob_nosort)
- [`GLOB_NOSPACE`](#glob_nospace)
- [`IPC_CREAT`](#ipc_creat)
- [`IPC_EXCL`](#ipc_excl)
- [`IPC_INFO`](#ipc_info)
- [`IPC_NOWAIT`](#ipc_nowait)
- [`IPC_PRIVATE`](#ipc_private)
- [`IPC_RMID`](#ipc_rmid)
- [`IPC_SET`](#ipc_set)
- [`IPC_STAT`](#ipc_stat)
- [`IPPROTO_MAX`](#ipproto_max)
- [`IPTOS_ECN_NOT_ECT`](#iptos_ecn_not_ect)
- [`IPTOS_PREC_MASK`](#iptos_prec_mask)
- [`IPTOS_TOS_MASK`](#iptos_tos_mask)
- [`IPV6_MULTICAST_ALL`](#ipv6_multicast_all)
- [`IPV6_ROUTER_ALERT_ISOLATE`](#ipv6_router_alert_isolate)
- [`IPV6_RTHDR_LOOSE`](#ipv6_rthdr_loose)
- [`IPV6_RTHDR_STRICT`](#ipv6_rthdr_strict)
- [`ITIMER_PROF`](#itimer_prof)
- [`ITIMER_REAL`](#itimer_real)
- [`ITIMER_VIRTUAL`](#itimer_virtual)
- [`IUTF8`](#iutf8)
- [`LIO_NOP`](#lio_nop)
- [`LIO_NOWAIT`](#lio_nowait)
- [`LIO_READ`](#lio_read)
- [`LIO_WAIT`](#lio_wait)
- [`LIO_WRITE`](#lio_write)
- [`LOG_NFACILITIES`](#log_nfacilities)
- [`L_tmpnam`](#l_tmpnam)
- [`MAP_FIXED_NOREPLACE`](#map_fixed_noreplace)
- [`MAX_ADDR_LEN`](#max_addr_len)
- [`MFD_ALLOW_SEALING`](#mfd_allow_sealing)
- [`MFD_CLOEXEC`](#mfd_cloexec)
- [`MFD_EXEC`](#mfd_exec)
- [`MFD_HUGETLB`](#mfd_hugetlb)
- [`MFD_HUGE_16GB`](#mfd_huge_16gb)
- [`MFD_HUGE_16MB`](#mfd_huge_16mb)
- [`MFD_HUGE_1GB`](#mfd_huge_1gb)
- [`MFD_HUGE_1MB`](#mfd_huge_1mb)
- [`MFD_HUGE_256MB`](#mfd_huge_256mb)
- [`MFD_HUGE_2GB`](#mfd_huge_2gb)
- [`MFD_HUGE_2MB`](#mfd_huge_2mb)
- [`MFD_HUGE_32MB`](#mfd_huge_32mb)
- [`MFD_HUGE_512KB`](#mfd_huge_512kb)
- [`MFD_HUGE_512MB`](#mfd_huge_512mb)
- [`MFD_HUGE_64KB`](#mfd_huge_64kb)
- [`MFD_HUGE_8MB`](#mfd_huge_8mb)
- [`MFD_HUGE_MASK`](#mfd_huge_mask)
- [`MFD_HUGE_SHIFT`](#mfd_huge_shift)
- [`MFD_NOEXEC_SEAL`](#mfd_noexec_seal)
- [`MLOCK_ONFAULT`](#mlock_onfault)
- [`MON_1`](#mon_1)
- [`MON_10`](#mon_10)
- [`MON_11`](#mon_11)
- [`MON_12`](#mon_12)
- [`MON_2`](#mon_2)
- [`MON_3`](#mon_3)
- [`MON_4`](#mon_4)
- [`MON_5`](#mon_5)
- [`MON_6`](#mon_6)
- [`MON_7`](#mon_7)
- [`MON_8`](#mon_8)
- [`MON_9`](#mon_9)
- [`MSG_COPY`](#msg_copy)
- [`MS_NOUSER`](#ms_nouser)
- [`NI_DGRAM`](#ni_dgram)
- [`NI_IDN`](#ni_idn)
- [`NI_NAMEREQD`](#ni_namereqd)
- [`NI_NOFQDN`](#ni_nofqdn)
- [`NI_NUMERICHOST`](#ni_numerichost)
- [`NI_NUMERICSERV`](#ni_numericserv)
- [`NOEXPR`](#noexpr)
- [`NOSTR`](#nostr)
- [`NT_ASRS`](#nt_asrs)
- [`NT_AUXV`](#nt_auxv)
- [`NT_FPREGSET`](#nt_fpregset)
- [`NT_GWINDOWS`](#nt_gwindows)
- [`NT_LWPSINFO`](#nt_lwpsinfo)
- [`NT_LWPSTATUS`](#nt_lwpstatus)
- [`NT_PLATFORM`](#nt_platform)
- [`NT_PRCRED`](#nt_prcred)
- [`NT_PRFPREG`](#nt_prfpreg)
- [`NT_PRFPXREG`](#nt_prfpxreg)
- [`NT_PRPSINFO`](#nt_prpsinfo)
- [`NT_PRSTATUS`](#nt_prstatus)
- [`NT_PRXREG`](#nt_prxreg)
- [`NT_PSINFO`](#nt_psinfo)
- [`NT_PSTATUS`](#nt_pstatus)
- [`NT_TASKSTRUCT`](#nt_taskstruct)
- [`NT_UTSNAME`](#nt_utsname)
- [`PACKET_ADD_MEMBERSHIP`](#packet_add_membership)
- [`PACKET_AUXDATA`](#packet_auxdata)
- [`PACKET_BROADCAST`](#packet_broadcast)
- [`PACKET_COPY_THRESH`](#packet_copy_thresh)
- [`PACKET_DROP_MEMBERSHIP`](#packet_drop_membership)
- [`PACKET_HDRLEN`](#packet_hdrlen)
- [`PACKET_HOST`](#packet_host)
- [`PACKET_KERNEL`](#packet_kernel)
- [`PACKET_LOOPBACK`](#packet_loopback)
- [`PACKET_LOSS`](#packet_loss)
- [`PACKET_MR_ALLMULTI`](#packet_mr_allmulti)
- [`PACKET_MR_MULTICAST`](#packet_mr_multicast)
- [`PACKET_MR_PROMISC`](#packet_mr_promisc)
- [`PACKET_MR_UNICAST`](#packet_mr_unicast)
- [`PACKET_MULTICAST`](#packet_multicast)
- [`PACKET_ORIGDEV`](#packet_origdev)
- [`PACKET_OTHERHOST`](#packet_otherhost)
- [`PACKET_OUTGOING`](#packet_outgoing)
- [`PACKET_RECV_OUTPUT`](#packet_recv_output)
- [`PACKET_RESERVE`](#packet_reserve)
- [`PACKET_RX_RING`](#packet_rx_ring)
- [`PACKET_STATISTICS`](#packet_statistics)
- [`PACKET_TIMESTAMP`](#packet_timestamp)
- [`PACKET_TX_RING`](#packet_tx_ring)
- [`PACKET_TX_TIMESTAMP`](#packet_tx_timestamp)
- [`PACKET_USER`](#packet_user)
- [`PACKET_VERSION`](#packet_version)
- [`PACKET_VNET_HDR`](#packet_vnet_hdr)
- [`PF_MASKOS`](#pf_maskos)
- [`PF_MASKPROC`](#pf_maskproc)
- [`PF_R`](#pf_r)
- [`PF_W`](#pf_w)
- [`PF_X`](#pf_x)
- [`PM_STR`](#pm_str)
- [`POSIX_MADV_DONTNEED`](#posix_madv_dontneed)
- [`POSIX_MADV_NORMAL`](#posix_madv_normal)
- [`POSIX_MADV_RANDOM`](#posix_madv_random)
- [`POSIX_MADV_SEQUENTIAL`](#posix_madv_sequential)
- [`POSIX_MADV_WILLNEED`](#posix_madv_willneed)
- [`PR_CAPBSET_DROP`](#pr_capbset_drop)
- [`PR_CAPBSET_READ`](#pr_capbset_read)
- [`PR_CAP_AMBIENT`](#pr_cap_ambient)
- [`PR_CAP_AMBIENT_CLEAR_ALL`](#pr_cap_ambient_clear_all)
- [`PR_CAP_AMBIENT_IS_SET`](#pr_cap_ambient_is_set)
- [`PR_CAP_AMBIENT_LOWER`](#pr_cap_ambient_lower)
- [`PR_CAP_AMBIENT_RAISE`](#pr_cap_ambient_raise)
- [`PR_ENDIAN_BIG`](#pr_endian_big)
- [`PR_ENDIAN_LITTLE`](#pr_endian_little)
- [`PR_ENDIAN_PPC_LITTLE`](#pr_endian_ppc_little)
- [`PR_FPEMU_NOPRINT`](#pr_fpemu_noprint)
- [`PR_FPEMU_SIGFPE`](#pr_fpemu_sigfpe)
- [`PR_FP_EXC_ASYNC`](#pr_fp_exc_async)
- [`PR_FP_EXC_DISABLED`](#pr_fp_exc_disabled)
- [`PR_FP_EXC_DIV`](#pr_fp_exc_div)
- [`PR_FP_EXC_INV`](#pr_fp_exc_inv)
- [`PR_FP_EXC_NONRECOV`](#pr_fp_exc_nonrecov)
- [`PR_FP_EXC_OVF`](#pr_fp_exc_ovf)
- [`PR_FP_EXC_PRECISE`](#pr_fp_exc_precise)
- [`PR_FP_EXC_RES`](#pr_fp_exc_res)
- [`PR_FP_EXC_SW_ENABLE`](#pr_fp_exc_sw_enable)
- [`PR_FP_EXC_UND`](#pr_fp_exc_und)
- [`PR_FP_MODE_FR`](#pr_fp_mode_fr)
- [`PR_FP_MODE_FRE`](#pr_fp_mode_fre)
- [`PR_GET_CHILD_SUBREAPER`](#pr_get_child_subreaper)
- [`PR_GET_DUMPABLE`](#pr_get_dumpable)
- [`PR_GET_ENDIAN`](#pr_get_endian)
- [`PR_GET_FPEMU`](#pr_get_fpemu)
- [`PR_GET_FPEXC`](#pr_get_fpexc)
- [`PR_GET_FP_MODE`](#pr_get_fp_mode)
- [`PR_GET_KEEPCAPS`](#pr_get_keepcaps)
- [`PR_GET_NAME`](#pr_get_name)
- [`PR_GET_NO_NEW_PRIVS`](#pr_get_no_new_privs)
- [`PR_GET_PDEATHSIG`](#pr_get_pdeathsig)
- [`PR_GET_SECCOMP`](#pr_get_seccomp)
- [`PR_GET_SECUREBITS`](#pr_get_securebits)
- [`PR_GET_THP_DISABLE`](#pr_get_thp_disable)
- [`PR_GET_TID_ADDRESS`](#pr_get_tid_address)
- [`PR_GET_TIMERSLACK`](#pr_get_timerslack)
- [`PR_GET_TIMING`](#pr_get_timing)
- [`PR_GET_TSC`](#pr_get_tsc)
- [`PR_GET_UNALIGN`](#pr_get_unalign)
- [`PR_MCE_KILL`](#pr_mce_kill)
- [`PR_MCE_KILL_CLEAR`](#pr_mce_kill_clear)
- [`PR_MCE_KILL_DEFAULT`](#pr_mce_kill_default)
- [`PR_MCE_KILL_EARLY`](#pr_mce_kill_early)
- [`PR_MCE_KILL_GET`](#pr_mce_kill_get)
- [`PR_MCE_KILL_LATE`](#pr_mce_kill_late)
- [`PR_MCE_KILL_SET`](#pr_mce_kill_set)
- [`PR_MPX_DISABLE_MANAGEMENT`](#pr_mpx_disable_management)
- [`PR_MPX_ENABLE_MANAGEMENT`](#pr_mpx_enable_management)
- [`PR_SCHED_CORE`](#pr_sched_core)
- [`PR_SCHED_CORE_CREATE`](#pr_sched_core_create)
- [`PR_SCHED_CORE_GET`](#pr_sched_core_get)
- [`PR_SCHED_CORE_MAX`](#pr_sched_core_max)
- [`PR_SCHED_CORE_SCOPE_PROCESS_GROUP`](#pr_sched_core_scope_process_group)
- [`PR_SCHED_CORE_SCOPE_THREAD`](#pr_sched_core_scope_thread)
- [`PR_SCHED_CORE_SCOPE_THREAD_GROUP`](#pr_sched_core_scope_thread_group)
- [`PR_SCHED_CORE_SHARE_FROM`](#pr_sched_core_share_from)
- [`PR_SCHED_CORE_SHARE_TO`](#pr_sched_core_share_to)
- [`PR_SET_CHILD_SUBREAPER`](#pr_set_child_subreaper)
- [`PR_SET_DUMPABLE`](#pr_set_dumpable)
- [`PR_SET_ENDIAN`](#pr_set_endian)
- [`PR_SET_FPEMU`](#pr_set_fpemu)
- [`PR_SET_FPEXC`](#pr_set_fpexc)
- [`PR_SET_FP_MODE`](#pr_set_fp_mode)
- [`PR_SET_KEEPCAPS`](#pr_set_keepcaps)
- [`PR_SET_MM`](#pr_set_mm)
- [`PR_SET_MM_ARG_END`](#pr_set_mm_arg_end)
- [`PR_SET_MM_ARG_START`](#pr_set_mm_arg_start)
- [`PR_SET_MM_AUXV`](#pr_set_mm_auxv)
- [`PR_SET_MM_BRK`](#pr_set_mm_brk)
- [`PR_SET_MM_END_CODE`](#pr_set_mm_end_code)
- [`PR_SET_MM_END_DATA`](#pr_set_mm_end_data)
- [`PR_SET_MM_ENV_END`](#pr_set_mm_env_end)
- [`PR_SET_MM_ENV_START`](#pr_set_mm_env_start)
- [`PR_SET_MM_EXE_FILE`](#pr_set_mm_exe_file)
- [`PR_SET_MM_MAP`](#pr_set_mm_map)
- [`PR_SET_MM_MAP_SIZE`](#pr_set_mm_map_size)
- [`PR_SET_MM_START_BRK`](#pr_set_mm_start_brk)
- [`PR_SET_MM_START_CODE`](#pr_set_mm_start_code)
- [`PR_SET_MM_START_DATA`](#pr_set_mm_start_data)
- [`PR_SET_MM_START_STACK`](#pr_set_mm_start_stack)
- [`PR_SET_NAME`](#pr_set_name)
- [`PR_SET_NO_NEW_PRIVS`](#pr_set_no_new_privs)
- [`PR_SET_PDEATHSIG`](#pr_set_pdeathsig)
- [`PR_SET_PTRACER`](#pr_set_ptracer)
- [`PR_SET_PTRACER_ANY`](#pr_set_ptracer_any)
- [`PR_SET_SECCOMP`](#pr_set_seccomp)
- [`PR_SET_SECUREBITS`](#pr_set_securebits)
- [`PR_SET_THP_DISABLE`](#pr_set_thp_disable)
- [`PR_SET_TIMERSLACK`](#pr_set_timerslack)
- [`PR_SET_TIMING`](#pr_set_timing)
- [`PR_SET_TSC`](#pr_set_tsc)
- [`PR_SET_UNALIGN`](#pr_set_unalign)
- [`PR_SET_VMA`](#pr_set_vma)
- [`PR_SET_VMA_ANON_NAME`](#pr_set_vma_anon_name)
- [`PR_TASK_PERF_EVENTS_DISABLE`](#pr_task_perf_events_disable)
- [`PR_TASK_PERF_EVENTS_ENABLE`](#pr_task_perf_events_enable)
- [`PR_TIMING_STATISTICAL`](#pr_timing_statistical)
- [`PR_TIMING_TIMESTAMP`](#pr_timing_timestamp)
- [`PR_TSC_ENABLE`](#pr_tsc_enable)
- [`PR_TSC_SIGSEGV`](#pr_tsc_sigsegv)
- [`PR_UNALIGN_NOPRINT`](#pr_unalign_noprint)
- [`PR_UNALIGN_SIGBUS`](#pr_unalign_sigbus)
- [`PTHREAD_BARRIER_SERIAL_THREAD`](#pthread_barrier_serial_thread)
- [`PTHREAD_EXPLICIT_SCHED`](#pthread_explicit_sched)
- [`PTHREAD_INHERIT_SCHED`](#pthread_inherit_sched)
- [`PTHREAD_MUTEX_DEFAULT`](#pthread_mutex_default)
- [`PTHREAD_MUTEX_ERRORCHECK`](#pthread_mutex_errorcheck)
- [`PTHREAD_MUTEX_NORMAL`](#pthread_mutex_normal)
- [`PTHREAD_MUTEX_RECURSIVE`](#pthread_mutex_recursive)
- [`PTHREAD_MUTEX_ROBUST`](#pthread_mutex_robust)
- [`PTHREAD_MUTEX_STALLED`](#pthread_mutex_stalled)
- [`PTHREAD_ONCE_INIT`](#pthread_once_init)
- [`PTHREAD_PRIO_INHERIT`](#pthread_prio_inherit)
- [`PTHREAD_PRIO_NONE`](#pthread_prio_none)
- [`PTHREAD_PRIO_PROTECT`](#pthread_prio_protect)
- [`PTHREAD_PROCESS_PRIVATE`](#pthread_process_private)
- [`PTHREAD_PROCESS_SHARED`](#pthread_process_shared)
- [`PTRACE_EVENT_STOP`](#ptrace_event_stop)
- [`PT_DYNAMIC`](#pt_dynamic)
- [`PT_GNU_EH_FRAME`](#pt_gnu_eh_frame)
- [`PT_GNU_RELRO`](#pt_gnu_relro)
- [`PT_GNU_STACK`](#pt_gnu_stack)
- [`PT_HIOS`](#pt_hios)
- [`PT_HIPROC`](#pt_hiproc)
- [`PT_HISUNW`](#pt_hisunw)
- [`PT_INTERP`](#pt_interp)
- [`PT_LOAD`](#pt_load)
- [`PT_LOOS`](#pt_loos)
- [`PT_LOPROC`](#pt_loproc)
- [`PT_LOSUNW`](#pt_losunw)
- [`PT_NOTE`](#pt_note)
- [`PT_NULL`](#pt_null)
- [`PT_NUM`](#pt_num)
- [`PT_PHDR`](#pt_phdr)
- [`PT_SHLIB`](#pt_shlib)
- [`PT_SUNWBSS`](#pt_sunwbss)
- [`PT_SUNWSTACK`](#pt_sunwstack)
- [`PT_TLS`](#pt_tls)
- [`RADIXCHAR`](#radixchar)
- [`REG_BADBR`](#reg_badbr)
- [`REG_BADPAT`](#reg_badpat)
- [`REG_BADRPT`](#reg_badrpt)
- [`REG_EBRACE`](#reg_ebrace)
- [`REG_EBRACK`](#reg_ebrack)
- [`REG_ECOLLATE`](#reg_ecollate)
- [`REG_ECTYPE`](#reg_ectype)
- [`REG_EESCAPE`](#reg_eescape)
- [`REG_ENOSYS`](#reg_enosys)
- [`REG_EPAREN`](#reg_eparen)
- [`REG_ERANGE`](#reg_erange)
- [`REG_ESPACE`](#reg_espace)
- [`REG_ESUBREG`](#reg_esubreg)
- [`REG_EXTENDED`](#reg_extended)
- [`REG_ICASE`](#reg_icase)
- [`REG_NEWLINE`](#reg_newline)
- [`REG_NOMATCH`](#reg_nomatch)
- [`REG_NOSUB`](#reg_nosub)
- [`REG_NOTBOL`](#reg_notbol)
- [`REG_NOTEOL`](#reg_noteol)
- [`RLIM_SAVED_CUR`](#rlim_saved_cur)
- [`RLIM_SAVED_MAX`](#rlim_saved_max)
- [`RTCF_DIRECTSRC`](#rtcf_directsrc)
- [`RTCF_DOREDIRECT`](#rtcf_doredirect)
- [`RTCF_LOG`](#rtcf_log)
- [`RTCF_MASQ`](#rtcf_masq)
- [`RTCF_NAT`](#rtcf_nat)
- [`RTCF_VALVE`](#rtcf_valve)
- [`RTF_ADDRCLASSMASK`](#rtf_addrclassmask)
- [`RTF_ADDRCONF`](#rtf_addrconf)
- [`RTF_ALLONLINK`](#rtf_allonlink)
- [`RTF_BROADCAST`](#rtf_broadcast)
- [`RTF_CACHE`](#rtf_cache)
- [`RTF_DEFAULT`](#rtf_default)
- [`RTF_DYNAMIC`](#rtf_dynamic)
- [`RTF_FLOW`](#rtf_flow)
- [`RTF_GATEWAY`](#rtf_gateway)
- [`RTF_HOST`](#rtf_host)
- [`RTF_INTERFACE`](#rtf_interface)
- [`RTF_IRTT`](#rtf_irtt)
- [`RTF_LINKRT`](#rtf_linkrt)
- [`RTF_LOCAL`](#rtf_local)
- [`RTF_MODIFIED`](#rtf_modified)
- [`RTF_MSS`](#rtf_mss)
- [`RTF_MTU`](#rtf_mtu)
- [`RTF_MULTICAST`](#rtf_multicast)
- [`RTF_NAT`](#rtf_nat)
- [`RTF_NOFORWARD`](#rtf_noforward)
- [`RTF_NONEXTHOP`](#rtf_nonexthop)
- [`RTF_NOPMTUDISC`](#rtf_nopmtudisc)
- [`RTF_POLICY`](#rtf_policy)
- [`RTF_REINSTATE`](#rtf_reinstate)
- [`RTF_REJECT`](#rtf_reject)
- [`RTF_STATIC`](#rtf_static)
- [`RTF_THROW`](#rtf_throw)
- [`RTF_UP`](#rtf_up)
- [`RTF_WINDOW`](#rtf_window)
- [`RTF_XRESOLVE`](#rtf_xresolve)
- [`RTLD_DEFAULT`](#rtld_default)
- [`RTLD_NEXT`](#rtld_next)
- [`RTLD_NODELETE`](#rtld_nodelete)
- [`RTLD_NOW`](#rtld_now)
- [`RT_CLASS_DEFAULT`](#rt_class_default)
- [`RT_CLASS_LOCAL`](#rt_class_local)
- [`RT_CLASS_MAIN`](#rt_class_main)
- [`RT_CLASS_MAX`](#rt_class_max)
- [`RT_CLASS_UNSPEC`](#rt_class_unspec)
- [`RUSAGE_CHILDREN`](#rusage_children)
- [`RUSAGE_THREAD`](#rusage_thread)
- [`SCHED_BATCH`](#sched_batch)
- [`SCHED_DEADLINE`](#sched_deadline)
- [`SCHED_FIFO`](#sched_fifo)
- [`SCHED_IDLE`](#sched_idle)
- [`SCHED_NORMAL`](#sched_normal)
- [`SCHED_OTHER`](#sched_other)
- [`SCHED_RESET_ON_FORK`](#sched_reset_on_fork)
- [`SCHED_RR`](#sched_rr)
- [`SELFMAG`](#selfmag)
- [`SEM_FAILED`](#sem_failed)
- [`SHM_EXEC`](#shm_exec)
- [`SHM_HUGETLB`](#shm_hugetlb)
- [`SHM_LOCK`](#shm_lock)
- [`SHM_NORESERVE`](#shm_noreserve)
- [`SHM_R`](#shm_r)
- [`SHM_RDONLY`](#shm_rdonly)
- [`SHM_REMAP`](#shm_remap)
- [`SHM_RND`](#shm_rnd)
- [`SHM_UNLOCK`](#shm_unlock)
- [`SHM_W`](#shm_w)
- [`SIOCADDMULTI`](#siocaddmulti)
- [`SIOCADDRT`](#siocaddrt)
- [`SIOCDARP`](#siocdarp)
- [`SIOCDELMULTI`](#siocdelmulti)
- [`SIOCDELRT`](#siocdelrt)
- [`SIOCDIFADDR`](#siocdifaddr)
- [`SIOCDRARP`](#siocdrarp)
- [`SIOCETHTOOL`](#siocethtool)
- [`SIOCGARP`](#siocgarp)
- [`SIOCGIFADDR`](#siocgifaddr)
- [`SIOCGIFBR`](#siocgifbr)
- [`SIOCGIFBRDADDR`](#siocgifbrdaddr)
- [`SIOCGIFCONF`](#siocgifconf)
- [`SIOCGIFCOUNT`](#siocgifcount)
- [`SIOCGIFDSTADDR`](#siocgifdstaddr)
- [`SIOCGIFENCAP`](#siocgifencap)
- [`SIOCGIFFLAGS`](#siocgifflags)
- [`SIOCGIFHWADDR`](#siocgifhwaddr)
- [`SIOCGIFINDEX`](#siocgifindex)
- [`SIOCGIFMAP`](#siocgifmap)
- [`SIOCGIFMEM`](#siocgifmem)
- [`SIOCGIFMETRIC`](#siocgifmetric)
- [`SIOCGIFMTU`](#siocgifmtu)
- [`SIOCGIFNAME`](#siocgifname)
- [`SIOCGIFNETMASK`](#siocgifnetmask)
- [`SIOCGIFPFLAGS`](#siocgifpflags)
- [`SIOCGIFSLAVE`](#siocgifslave)
- [`SIOCGIFTXQLEN`](#siocgiftxqlen)
- [`SIOCGMIIPHY`](#siocgmiiphy)
- [`SIOCGMIIREG`](#siocgmiireg)
- [`SIOCGRARP`](#siocgrarp)
- [`SIOCGSKNS`](#siocgskns)
- [`SIOCOUTQNSD`](#siocoutqnsd)
- [`SIOCSARP`](#siocsarp)
- [`SIOCSIFADDR`](#siocsifaddr)
- [`SIOCSIFBR`](#siocsifbr)
- [`SIOCSIFBRDADDR`](#siocsifbrdaddr)
- [`SIOCSIFDSTADDR`](#siocsifdstaddr)
- [`SIOCSIFENCAP`](#siocsifencap)
- [`SIOCSIFFLAGS`](#siocsifflags)
- [`SIOCSIFHWADDR`](#siocsifhwaddr)
- [`SIOCSIFHWBROADCAST`](#siocsifhwbroadcast)
- [`SIOCSIFLINK`](#siocsiflink)
- [`SIOCSIFMAP`](#siocsifmap)
- [`SIOCSIFMEM`](#siocsifmem)
- [`SIOCSIFMETRIC`](#siocsifmetric)
- [`SIOCSIFMTU`](#siocsifmtu)
- [`SIOCSIFNAME`](#siocsifname)
- [`SIOCSIFNETMASK`](#siocsifnetmask)
- [`SIOCSIFPFLAGS`](#siocsifpflags)
- [`SIOCSIFSLAVE`](#siocsifslave)
- [`SIOCSIFTXQLEN`](#siocsiftxqlen)
- [`SIOCSMIIREG`](#siocsmiireg)
- [`SIOCSRARP`](#siocsrarp)
- [`SIOCWANDEV`](#siocwandev)
- [`SIOGIFINDEX`](#siogifindex)
- [`ST_APPEND`](#st_append)
- [`ST_IMMUTABLE`](#st_immutable)
- [`ST_MANDLOCK`](#st_mandlock)
- [`ST_NOATIME`](#st_noatime)
- [`ST_NODEV`](#st_nodev)
- [`ST_NODIRATIME`](#st_nodiratime)
- [`ST_NOEXEC`](#st_noexec)
- [`ST_NOSUID`](#st_nosuid)
- [`ST_RDONLY`](#st_rdonly)
- [`ST_SYNCHRONOUS`](#st_synchronous)
- [`ST_WRITE`](#st_write)
- [`S_IEXEC`](#s_iexec)
- [`S_IREAD`](#s_iread)
- [`S_IWRITE`](#s_iwrite)
- [`THOUSEP`](#thousep)
- [`T_FMT`](#t_fmt)
- [`T_FMT_AMPM`](#t_fmt_ampm)
- [`UDP_CORK`](#udp_cork)
- [`UDP_ENCAP`](#udp_encap)
- [`UDP_GRO`](#udp_gro)
- [`UDP_NO_CHECK6_RX`](#udp_no_check6_rx)
- [`UDP_NO_CHECK6_TX`](#udp_no_check6_tx)
- [`UDP_SEGMENT`](#udp_segment)
- [`YESEXPR`](#yesexpr)
- [`YESSTR`](#yesstr)
- [`_CS_PATH`](#_cs_path)
- [`_CS_POSIX_V5_WIDTH_RESTRICTED_ENVS`](#_cs_posix_v5_width_restricted_envs)
- [`_CS_POSIX_V6_ILP32_OFF32_CFLAGS`](#_cs_posix_v6_ilp32_off32_cflags)
- [`_CS_POSIX_V6_ILP32_OFF32_LDFLAGS`](#_cs_posix_v6_ilp32_off32_ldflags)
- [`_CS_POSIX_V6_ILP32_OFF32_LIBS`](#_cs_posix_v6_ilp32_off32_libs)
- [`_CS_POSIX_V6_ILP32_OFF32_LINTFLAGS`](#_cs_posix_v6_ilp32_off32_lintflags)
- [`_CS_POSIX_V6_ILP32_OFFBIG_CFLAGS`](#_cs_posix_v6_ilp32_offbig_cflags)
- [`_CS_POSIX_V6_ILP32_OFFBIG_LDFLAGS`](#_cs_posix_v6_ilp32_offbig_ldflags)
- [`_CS_POSIX_V6_ILP32_OFFBIG_LIBS`](#_cs_posix_v6_ilp32_offbig_libs)
- [`_CS_POSIX_V6_ILP32_OFFBIG_LINTFLAGS`](#_cs_posix_v6_ilp32_offbig_lintflags)
- [`_CS_POSIX_V6_LP64_OFF64_CFLAGS`](#_cs_posix_v6_lp64_off64_cflags)
- [`_CS_POSIX_V6_LP64_OFF64_LDFLAGS`](#_cs_posix_v6_lp64_off64_ldflags)
- [`_CS_POSIX_V6_LP64_OFF64_LIBS`](#_cs_posix_v6_lp64_off64_libs)
- [`_CS_POSIX_V6_LP64_OFF64_LINTFLAGS`](#_cs_posix_v6_lp64_off64_lintflags)
- [`_CS_POSIX_V6_LPBIG_OFFBIG_CFLAGS`](#_cs_posix_v6_lpbig_offbig_cflags)
- [`_CS_POSIX_V6_LPBIG_OFFBIG_LDFLAGS`](#_cs_posix_v6_lpbig_offbig_ldflags)
- [`_CS_POSIX_V6_LPBIG_OFFBIG_LIBS`](#_cs_posix_v6_lpbig_offbig_libs)
- [`_CS_POSIX_V6_LPBIG_OFFBIG_LINTFLAGS`](#_cs_posix_v6_lpbig_offbig_lintflags)
- [`_CS_POSIX_V6_WIDTH_RESTRICTED_ENVS`](#_cs_posix_v6_width_restricted_envs)
- [`_CS_POSIX_V7_ILP32_OFF32_CFLAGS`](#_cs_posix_v7_ilp32_off32_cflags)
- [`_CS_POSIX_V7_ILP32_OFF32_LDFLAGS`](#_cs_posix_v7_ilp32_off32_ldflags)
- [`_CS_POSIX_V7_ILP32_OFF32_LIBS`](#_cs_posix_v7_ilp32_off32_libs)
- [`_CS_POSIX_V7_ILP32_OFF32_LINTFLAGS`](#_cs_posix_v7_ilp32_off32_lintflags)
- [`_CS_POSIX_V7_ILP32_OFFBIG_CFLAGS`](#_cs_posix_v7_ilp32_offbig_cflags)
- [`_CS_POSIX_V7_ILP32_OFFBIG_LDFLAGS`](#_cs_posix_v7_ilp32_offbig_ldflags)
- [`_CS_POSIX_V7_ILP32_OFFBIG_LIBS`](#_cs_posix_v7_ilp32_offbig_libs)
- [`_CS_POSIX_V7_ILP32_OFFBIG_LINTFLAGS`](#_cs_posix_v7_ilp32_offbig_lintflags)
- [`_CS_POSIX_V7_LP64_OFF64_CFLAGS`](#_cs_posix_v7_lp64_off64_cflags)
- [`_CS_POSIX_V7_LP64_OFF64_LDFLAGS`](#_cs_posix_v7_lp64_off64_ldflags)
- [`_CS_POSIX_V7_LP64_OFF64_LIBS`](#_cs_posix_v7_lp64_off64_libs)
- [`_CS_POSIX_V7_LP64_OFF64_LINTFLAGS`](#_cs_posix_v7_lp64_off64_lintflags)
- [`_CS_POSIX_V7_LPBIG_OFFBIG_CFLAGS`](#_cs_posix_v7_lpbig_offbig_cflags)
- [`_CS_POSIX_V7_LPBIG_OFFBIG_LDFLAGS`](#_cs_posix_v7_lpbig_offbig_ldflags)
- [`_CS_POSIX_V7_LPBIG_OFFBIG_LIBS`](#_cs_posix_v7_lpbig_offbig_libs)
- [`_CS_POSIX_V7_LPBIG_OFFBIG_LINTFLAGS`](#_cs_posix_v7_lpbig_offbig_lintflags)
- [`_CS_POSIX_V7_WIDTH_RESTRICTED_ENVS`](#_cs_posix_v7_width_restricted_envs)
- [`_PC_2_SYMLINKS`](#_pc_2_symlinks)
- [`_PC_ALLOC_SIZE_MIN`](#_pc_alloc_size_min)
- [`_PC_ASYNC_IO`](#_pc_async_io)
- [`_PC_CHOWN_RESTRICTED`](#_pc_chown_restricted)
- [`_PC_FILESIZEBITS`](#_pc_filesizebits)
- [`_PC_LINK_MAX`](#_pc_link_max)
- [`_PC_MAX_CANON`](#_pc_max_canon)
- [`_PC_MAX_INPUT`](#_pc_max_input)
- [`_PC_NAME_MAX`](#_pc_name_max)
- [`_PC_NO_TRUNC`](#_pc_no_trunc)
- [`_PC_PATH_MAX`](#_pc_path_max)
- [`_PC_PIPE_BUF`](#_pc_pipe_buf)
- [`_PC_PRIO_IO`](#_pc_prio_io)
- [`_PC_REC_INCR_XFER_SIZE`](#_pc_rec_incr_xfer_size)
- [`_PC_REC_MAX_XFER_SIZE`](#_pc_rec_max_xfer_size)
- [`_PC_REC_MIN_XFER_SIZE`](#_pc_rec_min_xfer_size)
- [`_PC_REC_XFER_ALIGN`](#_pc_rec_xfer_align)
- [`_PC_SOCK_MAXBUF`](#_pc_sock_maxbuf)
- [`_PC_SYMLINK_MAX`](#_pc_symlink_max)
- [`_PC_SYNC_IO`](#_pc_sync_io)
- [`_PC_VDISABLE`](#_pc_vdisable)
- [`_POSIX_VDISABLE`](#_posix_vdisable)
- [`_SC_2_CHAR_TERM`](#_sc_2_char_term)
- [`_SC_2_C_BIND`](#_sc_2_c_bind)
- [`_SC_2_C_DEV`](#_sc_2_c_dev)
- [`_SC_2_FORT_DEV`](#_sc_2_fort_dev)
- [`_SC_2_FORT_RUN`](#_sc_2_fort_run)
- [`_SC_2_LOCALEDEF`](#_sc_2_localedef)
- [`_SC_2_PBS`](#_sc_2_pbs)
- [`_SC_2_PBS_ACCOUNTING`](#_sc_2_pbs_accounting)
- [`_SC_2_PBS_CHECKPOINT`](#_sc_2_pbs_checkpoint)
- [`_SC_2_PBS_LOCATE`](#_sc_2_pbs_locate)
- [`_SC_2_PBS_MESSAGE`](#_sc_2_pbs_message)
- [`_SC_2_PBS_TRACK`](#_sc_2_pbs_track)
- [`_SC_2_SW_DEV`](#_sc_2_sw_dev)
- [`_SC_2_UPE`](#_sc_2_upe)
- [`_SC_2_VERSION`](#_sc_2_version)
- [`_SC_ADVISORY_INFO`](#_sc_advisory_info)
- [`_SC_AIO_LISTIO_MAX`](#_sc_aio_listio_max)
- [`_SC_AIO_MAX`](#_sc_aio_max)
- [`_SC_AIO_PRIO_DELTA_MAX`](#_sc_aio_prio_delta_max)
- [`_SC_ARG_MAX`](#_sc_arg_max)
- [`_SC_ASYNCHRONOUS_IO`](#_sc_asynchronous_io)
- [`_SC_ATEXIT_MAX`](#_sc_atexit_max)
- [`_SC_AVPHYS_PAGES`](#_sc_avphys_pages)
- [`_SC_BARRIERS`](#_sc_barriers)
- [`_SC_BC_BASE_MAX`](#_sc_bc_base_max)
- [`_SC_BC_DIM_MAX`](#_sc_bc_dim_max)
- [`_SC_BC_SCALE_MAX`](#_sc_bc_scale_max)
- [`_SC_BC_STRING_MAX`](#_sc_bc_string_max)
- [`_SC_CHILD_MAX`](#_sc_child_max)
- [`_SC_CLK_TCK`](#_sc_clk_tck)
- [`_SC_CLOCK_SELECTION`](#_sc_clock_selection)
- [`_SC_COLL_WEIGHTS_MAX`](#_sc_coll_weights_max)
- [`_SC_CPUTIME`](#_sc_cputime)
- [`_SC_DELAYTIMER_MAX`](#_sc_delaytimer_max)
- [`_SC_EXPR_NEST_MAX`](#_sc_expr_nest_max)
- [`_SC_FSYNC`](#_sc_fsync)
- [`_SC_GETGR_R_SIZE_MAX`](#_sc_getgr_r_size_max)
- [`_SC_GETPW_R_SIZE_MAX`](#_sc_getpw_r_size_max)
- [`_SC_HOST_NAME_MAX`](#_sc_host_name_max)
- [`_SC_IOV_MAX`](#_sc_iov_max)
- [`_SC_IPV6`](#_sc_ipv6)
- [`_SC_JOB_CONTROL`](#_sc_job_control)
- [`_SC_LINE_MAX`](#_sc_line_max)
- [`_SC_LOGIN_NAME_MAX`](#_sc_login_name_max)
- [`_SC_MAPPED_FILES`](#_sc_mapped_files)
- [`_SC_MEMLOCK`](#_sc_memlock)
- [`_SC_MEMLOCK_RANGE`](#_sc_memlock_range)
- [`_SC_MEMORY_PROTECTION`](#_sc_memory_protection)
- [`_SC_MESSAGE_PASSING`](#_sc_message_passing)
- [`_SC_MONOTONIC_CLOCK`](#_sc_monotonic_clock)
- [`_SC_MQ_OPEN_MAX`](#_sc_mq_open_max)
- [`_SC_MQ_PRIO_MAX`](#_sc_mq_prio_max)
- [`_SC_NGROUPS_MAX`](#_sc_ngroups_max)
- [`_SC_NPROCESSORS_CONF`](#_sc_nprocessors_conf)
- [`_SC_NPROCESSORS_ONLN`](#_sc_nprocessors_onln)
- [`_SC_NZERO`](#_sc_nzero)
- [`_SC_OPEN_MAX`](#_sc_open_max)
- [`_SC_PAGESIZE`](#_sc_pagesize)
- [`_SC_PAGE_SIZE`](#_sc_page_size)
- [`_SC_PASS_MAX`](#_sc_pass_max)
- [`_SC_PHYS_PAGES`](#_sc_phys_pages)
- [`_SC_PRIORITIZED_IO`](#_sc_prioritized_io)
- [`_SC_PRIORITY_SCHEDULING`](#_sc_priority_scheduling)
- [`_SC_RAW_SOCKETS`](#_sc_raw_sockets)
- [`_SC_READER_WRITER_LOCKS`](#_sc_reader_writer_locks)
- [`_SC_REALTIME_SIGNALS`](#_sc_realtime_signals)
- [`_SC_REGEXP`](#_sc_regexp)
- [`_SC_RE_DUP_MAX`](#_sc_re_dup_max)
- [`_SC_RTSIG_MAX`](#_sc_rtsig_max)
- [`_SC_SAVED_IDS`](#_sc_saved_ids)
- [`_SC_SEMAPHORES`](#_sc_semaphores)
- [`_SC_SEM_NSEMS_MAX`](#_sc_sem_nsems_max)
- [`_SC_SEM_VALUE_MAX`](#_sc_sem_value_max)
- [`_SC_SHARED_MEMORY_OBJECTS`](#_sc_shared_memory_objects)
- [`_SC_SHELL`](#_sc_shell)
- [`_SC_SIGQUEUE_MAX`](#_sc_sigqueue_max)
- [`_SC_SPAWN`](#_sc_spawn)
- [`_SC_SPIN_LOCKS`](#_sc_spin_locks)
- [`_SC_SPORADIC_SERVER`](#_sc_sporadic_server)
- [`_SC_SS_REPL_MAX`](#_sc_ss_repl_max)
- [`_SC_STREAMS`](#_sc_streams)
- [`_SC_STREAM_MAX`](#_sc_stream_max)
- [`_SC_SYMLOOP_MAX`](#_sc_symloop_max)
- [`_SC_SYNCHRONIZED_IO`](#_sc_synchronized_io)
- [`_SC_THREADS`](#_sc_threads)
- [`_SC_THREAD_ATTR_STACKADDR`](#_sc_thread_attr_stackaddr)
- [`_SC_THREAD_ATTR_STACKSIZE`](#_sc_thread_attr_stacksize)
- [`_SC_THREAD_CPUTIME`](#_sc_thread_cputime)
- [`_SC_THREAD_DESTRUCTOR_ITERATIONS`](#_sc_thread_destructor_iterations)
- [`_SC_THREAD_KEYS_MAX`](#_sc_thread_keys_max)
- [`_SC_THREAD_PRIORITY_SCHEDULING`](#_sc_thread_priority_scheduling)
- [`_SC_THREAD_PRIO_INHERIT`](#_sc_thread_prio_inherit)
- [`_SC_THREAD_PRIO_PROTECT`](#_sc_thread_prio_protect)
- [`_SC_THREAD_PROCESS_SHARED`](#_sc_thread_process_shared)
- [`_SC_THREAD_ROBUST_PRIO_INHERIT`](#_sc_thread_robust_prio_inherit)
- [`_SC_THREAD_ROBUST_PRIO_PROTECT`](#_sc_thread_robust_prio_protect)
- [`_SC_THREAD_SAFE_FUNCTIONS`](#_sc_thread_safe_functions)
- [`_SC_THREAD_SPORADIC_SERVER`](#_sc_thread_sporadic_server)
- [`_SC_THREAD_STACK_MIN`](#_sc_thread_stack_min)
- [`_SC_THREAD_THREADS_MAX`](#_sc_thread_threads_max)
- [`_SC_TIMEOUTS`](#_sc_timeouts)
- [`_SC_TIMERS`](#_sc_timers)
- [`_SC_TIMER_MAX`](#_sc_timer_max)
- [`_SC_TRACE`](#_sc_trace)
- [`_SC_TRACE_EVENT_FILTER`](#_sc_trace_event_filter)
- [`_SC_TRACE_EVENT_NAME_MAX`](#_sc_trace_event_name_max)
- [`_SC_TRACE_INHERIT`](#_sc_trace_inherit)
- [`_SC_TRACE_LOG`](#_sc_trace_log)
- [`_SC_TRACE_NAME_MAX`](#_sc_trace_name_max)
- [`_SC_TRACE_SYS_MAX`](#_sc_trace_sys_max)
- [`_SC_TRACE_USER_EVENT_MAX`](#_sc_trace_user_event_max)
- [`_SC_TTY_NAME_MAX`](#_sc_tty_name_max)
- [`_SC_TYPED_MEMORY_OBJECTS`](#_sc_typed_memory_objects)
- [`_SC_TZNAME_MAX`](#_sc_tzname_max)
- [`_SC_UIO_MAXIOV`](#_sc_uio_maxiov)
- [`_SC_V6_ILP32_OFF32`](#_sc_v6_ilp32_off32)
- [`_SC_V6_ILP32_OFFBIG`](#_sc_v6_ilp32_offbig)
- [`_SC_V6_LP64_OFF64`](#_sc_v6_lp64_off64)
- [`_SC_V6_LPBIG_OFFBIG`](#_sc_v6_lpbig_offbig)
- [`_SC_V7_ILP32_OFF32`](#_sc_v7_ilp32_off32)
- [`_SC_V7_ILP32_OFFBIG`](#_sc_v7_ilp32_offbig)
- [`_SC_V7_LP64_OFF64`](#_sc_v7_lp64_off64)
- [`_SC_V7_LPBIG_OFFBIG`](#_sc_v7_lpbig_offbig)
- [`_SC_VERSION`](#_sc_version)
- [`_SC_XBS5_ILP32_OFF32`](#_sc_xbs5_ilp32_off32)
- [`_SC_XBS5_ILP32_OFFBIG`](#_sc_xbs5_ilp32_offbig)
- [`_SC_XBS5_LP64_OFF64`](#_sc_xbs5_lp64_off64)
- [`_SC_XBS5_LPBIG_OFFBIG`](#_sc_xbs5_lpbig_offbig)
- [`_SC_XOPEN_CRYPT`](#_sc_xopen_crypt)
- [`_SC_XOPEN_ENH_I18N`](#_sc_xopen_enh_i18n)
- [`_SC_XOPEN_LEGACY`](#_sc_xopen_legacy)
- [`_SC_XOPEN_REALTIME`](#_sc_xopen_realtime)
- [`_SC_XOPEN_REALTIME_THREADS`](#_sc_xopen_realtime_threads)
- [`_SC_XOPEN_SHM`](#_sc_xopen_shm)
- [`_SC_XOPEN_STREAMS`](#_sc_xopen_streams)
- [`_SC_XOPEN_UNIX`](#_sc_xopen_unix)
- [`_SC_XOPEN_VERSION`](#_sc_xopen_version)
- [`_SC_XOPEN_XCU_VERSION`](#_sc_xopen_xcu_version)
- [`_SC_XOPEN_XPG2`](#_sc_xopen_xpg2)
- [`_SC_XOPEN_XPG3`](#_sc_xopen_xpg3)
- [`_SC_XOPEN_XPG4`](#_sc_xopen_xpg4)
- [`__SIZEOF_PTHREAD_COND_T`](#__sizeof_pthread_cond_t)

**Type Aliases**

- [`Elf32_Addr`](#elf32_addr)
- [`Elf32_Half`](#elf32_half)
- [`Elf32_Off`](#elf32_off)
- [`Elf32_Rel`](#elf32_rel)
- [`Elf32_Rela`](#elf32_rela)
- [`Elf32_Relr`](#elf32_relr)
- [`Elf32_Section`](#elf32_section)
- [`Elf32_Sword`](#elf32_sword)
- [`Elf32_Word`](#elf32_word)
- [`Elf32_Xword`](#elf32_xword)
- [`Elf64_Addr`](#elf64_addr)
- [`Elf64_Half`](#elf64_half)
- [`Elf64_Off`](#elf64_off)
- [`Elf64_Rel`](#elf64_rel)
- [`Elf64_Rela`](#elf64_rela)
- [`Elf64_Relr`](#elf64_relr)
- [`Elf64_Section`](#elf64_section)
- [`Elf64_Sword`](#elf64_sword)
- [`Elf64_Sxword`](#elf64_sxword)
- [`Elf64_Word`](#elf64_word)
- [`Elf64_Xword`](#elf64_xword)
- [`iconv_t`](#iconv_t)

---

## libc::unix::linux_like::linux_l4re_shared::ABDAY_1

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::ABDAY_2

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::ABDAY_3

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::ABDAY_4

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::ABDAY_5

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::ABDAY_6

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::ABDAY_7

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::ABMON_1

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::ABMON_10

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::ABMON_11

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::ABMON_12

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::ABMON_2

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::ABMON_3

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::ABMON_4

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::ABMON_5

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::ABMON_6

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::ABMON_7

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::ABMON_8

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::ABMON_9

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::AIO_ALLDONE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::AIO_CANCELED

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::AIO_NOTCANCELED

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::AI_ADDRCONFIG

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::AI_ALL

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::AI_CANONNAME

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::AI_NUMERICHOST

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::AI_NUMERICSERV

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::AI_PASSIVE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::AI_V4MAPPED

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::ALT_DIGITS

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::AM_STR

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::ARPD_FLUSH

*Constant*: `c_ushort`



## libc::unix::linux_like::linux_l4re_shared::ARPD_LOOKUP

*Constant*: `c_ushort`



## libc::unix::linux_like::linux_l4re_shared::ARPD_UPDATE

*Constant*: `c_ushort`



## libc::unix::linux_like::linux_l4re_shared::ATF_MAGIC

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::AT_BASE

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::AT_BASE_PLATFORM

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::AT_CLKTCK

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::AT_EACCESS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::AT_EGID

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::AT_ENTRY

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::AT_EUID

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::AT_EXECFD

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::AT_EXECFN

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::AT_FLAGS

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::AT_GID

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::AT_HWCAP

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::AT_HWCAP2

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::AT_HWCAP3

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::AT_HWCAP4

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::AT_IGNORE

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::AT_MINSIGSTKSZ

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::AT_NOTELF

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::AT_NULL

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::AT_PAGESZ

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::AT_PHDR

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::AT_PHENT

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::AT_PHNUM

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::AT_PLATFORM

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::AT_RANDOM

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::AT_SECURE

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::AT_SYSINFO_EHDR

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::AT_UID

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::CMSG_NXTHDR

*Function*

```rust
fn CMSG_NXTHDR(mhdr: *const crate::msghdr, cmsg: *const crate::cmsghdr) -> *mut crate::cmsghdr
```



## libc::unix::linux_like::linux_l4re_shared::CMSPAR

*Constant*: `crate::tcflag_t`



## libc::unix::linux_like::linux_l4re_shared::CODESET

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::CPU_ALLOC_SIZE

*Function*

```rust
fn CPU_ALLOC_SIZE(count: c_int) -> size_t
```



## libc::unix::linux_like::linux_l4re_shared::CPU_CLR

*Function*

```rust
fn CPU_CLR(cpu: usize, cpuset: & mut cpu_set_t)
```



## libc::unix::linux_like::linux_l4re_shared::CPU_COUNT

*Function*

```rust
fn CPU_COUNT(cpuset: &cpu_set_t) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::CPU_COUNT_S

*Function*

```rust
fn CPU_COUNT_S(size: usize, cpuset: &cpu_set_t) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::CPU_EQUAL

*Function*

```rust
fn CPU_EQUAL(set1: &cpu_set_t, set2: &cpu_set_t) -> bool
```



## libc::unix::linux_like::linux_l4re_shared::CPU_ISSET

*Function*

```rust
fn CPU_ISSET(cpu: usize, cpuset: &cpu_set_t) -> bool
```



## libc::unix::linux_like::linux_l4re_shared::CPU_SET

*Function*

```rust
fn CPU_SET(cpu: usize, cpuset: & mut cpu_set_t)
```



## libc::unix::linux_like::linux_l4re_shared::CPU_ZERO

*Function*

```rust
fn CPU_ZERO(cpuset: & mut cpu_set_t)
```



## libc::unix::linux_like::linux_l4re_shared::CRNCYSTR

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::CSIGNAL

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::DAY_1

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::DAY_2

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::DAY_3

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::DAY_4

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::DAY_5

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::DAY_6

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::DAY_7

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::D_FMT

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::D_T_FMT

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::E2BIG

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::EACCES

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::EAGAIN

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::EAI_AGAIN

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::EAI_BADFLAGS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::EAI_FAIL

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::EAI_FAMILY

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::EAI_MEMORY

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::EAI_NODATA

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::EAI_NONAME

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::EAI_OVERFLOW

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::EAI_SERVICE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::EAI_SOCKTYPE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::EAI_SYSTEM

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::EBADF

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::EBUSY

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::ECHILD

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::EDOM

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::EEXIST

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::EFAULT

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::EFBIG

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::EINTR

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::EINVAL

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::EIO

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::EISDIR

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::EI_ABIVERSION

*Constant*: `usize`



## libc::unix::linux_like::linux_l4re_shared::EI_CLASS

*Constant*: `usize`



## libc::unix::linux_like::linux_l4re_shared::EI_DATA

*Constant*: `usize`



## libc::unix::linux_like::linux_l4re_shared::EI_MAG0

*Constant*: `usize`



## libc::unix::linux_like::linux_l4re_shared::EI_MAG1

*Constant*: `usize`



## libc::unix::linux_like::linux_l4re_shared::EI_MAG2

*Constant*: `usize`



## libc::unix::linux_like::linux_l4re_shared::EI_MAG3

*Constant*: `usize`



## libc::unix::linux_like::linux_l4re_shared::EI_NIDENT

*Constant*: `usize`



## libc::unix::linux_like::linux_l4re_shared::EI_OSABI

*Constant*: `usize`



## libc::unix::linux_like::linux_l4re_shared::EI_PAD

*Constant*: `usize`



## libc::unix::linux_like::linux_l4re_shared::EI_VERSION

*Constant*: `usize`



## libc::unix::linux_like::linux_l4re_shared::ELF32_R_INFO

*Function*

```rust
fn ELF32_R_INFO(sym: Elf32_Word, t: Elf32_Word) -> Elf32_Word
```



## libc::unix::linux_like::linux_l4re_shared::ELF32_R_SYM

*Function*

```rust
fn ELF32_R_SYM(val: Elf32_Word) -> Elf32_Word
```



## libc::unix::linux_like::linux_l4re_shared::ELF32_R_TYPE

*Function*

```rust
fn ELF32_R_TYPE(val: Elf32_Word) -> Elf32_Word
```



## libc::unix::linux_like::linux_l4re_shared::ELF64_R_INFO

*Function*

```rust
fn ELF64_R_INFO(sym: Elf64_Xword, t: Elf64_Xword) -> Elf64_Xword
```



## libc::unix::linux_like::linux_l4re_shared::ELF64_R_SYM

*Function*

```rust
fn ELF64_R_SYM(val: Elf64_Xword) -> Elf64_Xword
```



## libc::unix::linux_like::linux_l4re_shared::ELF64_R_TYPE

*Function*

```rust
fn ELF64_R_TYPE(val: Elf64_Xword) -> Elf64_Xword
```



## libc::unix::linux_like::linux_l4re_shared::ELFCLASS32

*Constant*: `u8`



## libc::unix::linux_like::linux_l4re_shared::ELFCLASS64

*Constant*: `u8`



## libc::unix::linux_like::linux_l4re_shared::ELFCLASSNONE

*Constant*: `u8`



## libc::unix::linux_like::linux_l4re_shared::ELFCLASSNUM

*Constant*: `usize`



## libc::unix::linux_like::linux_l4re_shared::ELFDATA2LSB

*Constant*: `u8`



## libc::unix::linux_like::linux_l4re_shared::ELFDATA2MSB

*Constant*: `u8`



## libc::unix::linux_like::linux_l4re_shared::ELFDATANONE

*Constant*: `u8`



## libc::unix::linux_like::linux_l4re_shared::ELFDATANUM

*Constant*: `usize`



## libc::unix::linux_like::linux_l4re_shared::ELFMAG0

*Constant*: `u8`



## libc::unix::linux_like::linux_l4re_shared::ELFMAG1

*Constant*: `u8`



## libc::unix::linux_like::linux_l4re_shared::ELFMAG2

*Constant*: `u8`



## libc::unix::linux_like::linux_l4re_shared::ELFMAG3

*Constant*: `u8`



## libc::unix::linux_like::linux_l4re_shared::ELFOSABI_AIX

*Constant*: `u8`



## libc::unix::linux_like::linux_l4re_shared::ELFOSABI_ARM

*Constant*: `u8`



## libc::unix::linux_like::linux_l4re_shared::ELFOSABI_FREEBSD

*Constant*: `u8`



## libc::unix::linux_like::linux_l4re_shared::ELFOSABI_GNU

*Constant*: `u8`



## libc::unix::linux_like::linux_l4re_shared::ELFOSABI_HPUX

*Constant*: `u8`



## libc::unix::linux_like::linux_l4re_shared::ELFOSABI_IRIX

*Constant*: `u8`



## libc::unix::linux_like::linux_l4re_shared::ELFOSABI_LINUX

*Constant*: `u8`



## libc::unix::linux_like::linux_l4re_shared::ELFOSABI_MODESTO

*Constant*: `u8`



## libc::unix::linux_like::linux_l4re_shared::ELFOSABI_NETBSD

*Constant*: `u8`



## libc::unix::linux_like::linux_l4re_shared::ELFOSABI_NONE

*Constant*: `u8`



## libc::unix::linux_like::linux_l4re_shared::ELFOSABI_OPENBSD

*Constant*: `u8`



## libc::unix::linux_like::linux_l4re_shared::ELFOSABI_SOLARIS

*Constant*: `u8`



## libc::unix::linux_like::linux_l4re_shared::ELFOSABI_STANDALONE

*Constant*: `u8`



## libc::unix::linux_like::linux_l4re_shared::ELFOSABI_SYSV

*Constant*: `u8`



## libc::unix::linux_like::linux_l4re_shared::ELFOSABI_TRU64

*Constant*: `u8`



## libc::unix::linux_like::linux_l4re_shared::EMFILE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::EMLINK

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::EM_386

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_68HC05

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_68HC08

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_68HC11

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_68HC12

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_68HC16

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_68K

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_860

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_88K

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_960

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_AARCH64

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_ALPHA

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_ARC

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_ARC_A5

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_ARM

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_AVR

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_COLDFIRE

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_CRIS

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_D10V

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_D30V

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_FAKE_ALPHA

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_FIREPATH

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_FR20

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_FR30

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_FX66

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_H8S

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_H8_300

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_H8_300H

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_H8_500

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_HUANY

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_IA_64

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_JAVELIN

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_M32

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_M32R

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_ME16

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_MIPS

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_MIPS_RS3_LE

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_MIPS_X

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_MMA

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_MMIX

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_MN10200

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_MN10300

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_NCPU

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_NDR1

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_NONE

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_OPENRISC

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_PARISC

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_PCP

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_PDSP

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_PJ

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_PPC

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_PPC64

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_PRISM

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_RCE

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_RH32

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_RISCV

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_S370

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_S390

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_SH

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_SPARC

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_SPARC32PLUS

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_SPARCV9

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_ST100

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_ST19

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_ST7

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_ST9PLUS

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_STARCORE

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_SVX

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_TILEGX

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_TILEPRO

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_TINYJ

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_TRICORE

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_V800

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_V850

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_VAX

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_VPP500

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_X86_64

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_XTENSA

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EM_ZSP

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::ENFILE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::ENODEV

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::ENOENT

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::ENOEXEC

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::ENOMEM

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::ENOSPC

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::ENOTBLK

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::ENOTDIR

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::ENOTTY

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::ENXIO

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::EPERM

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::EPIPE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::ERA

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::ERANGE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::ERA_D_FMT

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::ERA_D_T_FMT

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::ERA_T_FMT

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::EROFS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::ESPIPE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::ESRCH

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::ETXTBSY

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::ET_CORE

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::ET_DYN

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::ET_EXEC

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::ET_HIOS

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::ET_HIPROC

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::ET_LOOS

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::ET_LOPROC

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::ET_NONE

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::ET_NUM

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::ET_REL

*Constant*: `u16`



## libc::unix::linux_like::linux_l4re_shared::EV_CURRENT

*Constant*: `u32`



## libc::unix::linux_like::linux_l4re_shared::EV_NONE

*Constant*: `u32`



## libc::unix::linux_like::linux_l4re_shared::EV_NUM

*Constant*: `u32`



## libc::unix::linux_like::linux_l4re_shared::EWOULDBLOCK

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::EXDEV

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::Elf32_Addr

*Type Alias*: `u32`



## libc::unix::linux_like::linux_l4re_shared::Elf32_Ehdr

*Struct*

**Fields:**
- `e_ident: [c_uchar; 16]`
- `e_type: Elf32_Half`
- `e_machine: Elf32_Half`
- `e_version: Elf32_Word`
- `e_entry: Elf32_Addr`
- `e_phoff: Elf32_Off`
- `e_shoff: Elf32_Off`
- `e_flags: Elf32_Word`
- `e_ehsize: Elf32_Half`
- `e_phentsize: Elf32_Half`
- `e_phnum: Elf32_Half`
- `e_shentsize: Elf32_Half`
- `e_shnum: Elf32_Half`
- `e_shstrndx: Elf32_Half`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Elf32_Ehdr`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux_l4re_shared::Elf32_Half

*Type Alias*: `u16`



## libc::unix::linux_like::linux_l4re_shared::Elf32_Off

*Type Alias*: `u32`



## libc::unix::linux_like::linux_l4re_shared::Elf32_Phdr

*Struct*

**Fields:**
- `p_type: Elf32_Word`
- `p_offset: Elf32_Off`
- `p_vaddr: Elf32_Addr`
- `p_paddr: Elf32_Addr`
- `p_filesz: Elf32_Word`
- `p_memsz: Elf32_Word`
- `p_flags: Elf32_Word`
- `p_align: Elf32_Word`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Elf32_Phdr`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux_l4re_shared::Elf32_Rel

*Type Alias*: `__c_anonymous_elf32_rel`



## libc::unix::linux_like::linux_l4re_shared::Elf32_Rela

*Type Alias*: `__c_anonymous_elf32_rela`



## libc::unix::linux_like::linux_l4re_shared::Elf32_Relr

*Type Alias*: `Elf32_Word`



## libc::unix::linux_like::linux_l4re_shared::Elf32_Section

*Type Alias*: `u16`



## libc::unix::linux_like::linux_l4re_shared::Elf32_Shdr

*Struct*

**Fields:**
- `sh_name: Elf32_Word`
- `sh_type: Elf32_Word`
- `sh_flags: Elf32_Word`
- `sh_addr: Elf32_Addr`
- `sh_offset: Elf32_Off`
- `sh_size: Elf32_Word`
- `sh_link: Elf32_Word`
- `sh_info: Elf32_Word`
- `sh_addralign: Elf32_Word`
- `sh_entsize: Elf32_Word`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Elf32_Shdr`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux_l4re_shared::Elf32_Sword

*Type Alias*: `i32`



## libc::unix::linux_like::linux_l4re_shared::Elf32_Sym

*Struct*

**Fields:**
- `st_name: Elf32_Word`
- `st_value: Elf32_Addr`
- `st_size: Elf32_Word`
- `st_info: c_uchar`
- `st_other: c_uchar`
- `st_shndx: Elf32_Section`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Elf32_Sym`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux_l4re_shared::Elf32_Word

*Type Alias*: `u32`



## libc::unix::linux_like::linux_l4re_shared::Elf32_Xword

*Type Alias*: `u64`



## libc::unix::linux_like::linux_l4re_shared::Elf64_Addr

*Type Alias*: `u64`



## libc::unix::linux_like::linux_l4re_shared::Elf64_Ehdr

*Struct*

**Fields:**
- `e_ident: [c_uchar; 16]`
- `e_type: Elf64_Half`
- `e_machine: Elf64_Half`
- `e_version: Elf64_Word`
- `e_entry: Elf64_Addr`
- `e_phoff: Elf64_Off`
- `e_shoff: Elf64_Off`
- `e_flags: Elf64_Word`
- `e_ehsize: Elf64_Half`
- `e_phentsize: Elf64_Half`
- `e_phnum: Elf64_Half`
- `e_shentsize: Elf64_Half`
- `e_shnum: Elf64_Half`
- `e_shstrndx: Elf64_Half`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Elf64_Ehdr`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux_l4re_shared::Elf64_Half

*Type Alias*: `u16`



## libc::unix::linux_like::linux_l4re_shared::Elf64_Off

*Type Alias*: `u64`



## libc::unix::linux_like::linux_l4re_shared::Elf64_Phdr

*Struct*

**Fields:**
- `p_type: Elf64_Word`
- `p_flags: Elf64_Word`
- `p_offset: Elf64_Off`
- `p_vaddr: Elf64_Addr`
- `p_paddr: Elf64_Addr`
- `p_filesz: Elf64_Xword`
- `p_memsz: Elf64_Xword`
- `p_align: Elf64_Xword`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Elf64_Phdr`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux_l4re_shared::Elf64_Rel

*Type Alias*: `__c_anonymous_elf64_rel`



## libc::unix::linux_like::linux_l4re_shared::Elf64_Rela

*Type Alias*: `__c_anonymous_elf64_rela`



## libc::unix::linux_like::linux_l4re_shared::Elf64_Relr

*Type Alias*: `Elf32_Xword`



## libc::unix::linux_like::linux_l4re_shared::Elf64_Section

*Type Alias*: `u16`



## libc::unix::linux_like::linux_l4re_shared::Elf64_Shdr

*Struct*

**Fields:**
- `sh_name: Elf64_Word`
- `sh_type: Elf64_Word`
- `sh_flags: Elf64_Xword`
- `sh_addr: Elf64_Addr`
- `sh_offset: Elf64_Off`
- `sh_size: Elf64_Xword`
- `sh_link: Elf64_Word`
- `sh_info: Elf64_Word`
- `sh_addralign: Elf64_Xword`
- `sh_entsize: Elf64_Xword`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Elf64_Shdr`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux_l4re_shared::Elf64_Sword

*Type Alias*: `i32`



## libc::unix::linux_like::linux_l4re_shared::Elf64_Sxword

*Type Alias*: `i64`



## libc::unix::linux_like::linux_l4re_shared::Elf64_Sym

*Struct*

**Fields:**
- `st_name: Elf64_Word`
- `st_info: c_uchar`
- `st_other: c_uchar`
- `st_shndx: Elf64_Section`
- `st_value: Elf64_Addr`
- `st_size: Elf64_Xword`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Elf64_Sym`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux_l4re_shared::Elf64_Word

*Type Alias*: `u32`



## libc::unix::linux_like::linux_l4re_shared::Elf64_Xword

*Type Alias*: `u64`



## libc::unix::linux_like::linux_l4re_shared::F_LOCK

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::F_TEST

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::F_TLOCK

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::F_ULOCK

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::GLOB_ABORTED

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::GLOB_APPEND

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::GLOB_DOOFFS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::GLOB_ERR

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::GLOB_MARK

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::GLOB_NOCHECK

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::GLOB_NOESCAPE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::GLOB_NOMATCH

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::GLOB_NOSORT

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::GLOB_NOSPACE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::IPC_CREAT

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::IPC_EXCL

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::IPC_INFO

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::IPC_NOWAIT

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::IPC_PRIVATE

*Constant*: `crate::key_t`



## libc::unix::linux_like::linux_l4re_shared::IPC_RMID

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::IPC_SET

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::IPC_STAT

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::IPPROTO_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::IPTOS_ECN_NOT_ECT

*Constant*: `u8`



## libc::unix::linux_like::linux_l4re_shared::IPTOS_PREC

*Function*

```rust
fn IPTOS_PREC(tos: u8) -> u8
```



## libc::unix::linux_like::linux_l4re_shared::IPTOS_PREC_MASK

*Constant*: `u8`



## libc::unix::linux_like::linux_l4re_shared::IPTOS_TOS

*Function*

```rust
fn IPTOS_TOS(tos: u8) -> u8
```



## libc::unix::linux_like::linux_l4re_shared::IPTOS_TOS_MASK

*Constant*: `u8`



## libc::unix::linux_like::linux_l4re_shared::IPV6_MULTICAST_ALL

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::IPV6_ROUTER_ALERT_ISOLATE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::IPV6_RTHDR_LOOSE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::IPV6_RTHDR_STRICT

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::ITIMER_PROF

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::ITIMER_REAL

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::ITIMER_VIRTUAL

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::IUTF8

*Constant*: `crate::tcflag_t`



## libc::unix::linux_like::linux_l4re_shared::LIO_NOP

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::LIO_NOWAIT

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::LIO_READ

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::LIO_WAIT

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::LIO_WRITE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::LOG_NFACILITIES

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::L_tmpnam

*Constant*: `c_uint`



## libc::unix::linux_like::linux_l4re_shared::MAP_FIXED_NOREPLACE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::MAX_ADDR_LEN

*Constant*: `usize`



## libc::unix::linux_like::linux_l4re_shared::MFD_ALLOW_SEALING

*Constant*: `c_uint`



## libc::unix::linux_like::linux_l4re_shared::MFD_CLOEXEC

*Constant*: `c_uint`



## libc::unix::linux_like::linux_l4re_shared::MFD_EXEC

*Constant*: `c_uint`



## libc::unix::linux_like::linux_l4re_shared::MFD_HUGETLB

*Constant*: `c_uint`



## libc::unix::linux_like::linux_l4re_shared::MFD_HUGE_16GB

*Constant*: `c_uint`



## libc::unix::linux_like::linux_l4re_shared::MFD_HUGE_16MB

*Constant*: `c_uint`



## libc::unix::linux_like::linux_l4re_shared::MFD_HUGE_1GB

*Constant*: `c_uint`



## libc::unix::linux_like::linux_l4re_shared::MFD_HUGE_1MB

*Constant*: `c_uint`



## libc::unix::linux_like::linux_l4re_shared::MFD_HUGE_256MB

*Constant*: `c_uint`



## libc::unix::linux_like::linux_l4re_shared::MFD_HUGE_2GB

*Constant*: `c_uint`



## libc::unix::linux_like::linux_l4re_shared::MFD_HUGE_2MB

*Constant*: `c_uint`



## libc::unix::linux_like::linux_l4re_shared::MFD_HUGE_32MB

*Constant*: `c_uint`



## libc::unix::linux_like::linux_l4re_shared::MFD_HUGE_512KB

*Constant*: `c_uint`



## libc::unix::linux_like::linux_l4re_shared::MFD_HUGE_512MB

*Constant*: `c_uint`



## libc::unix::linux_like::linux_l4re_shared::MFD_HUGE_64KB

*Constant*: `c_uint`



## libc::unix::linux_like::linux_l4re_shared::MFD_HUGE_8MB

*Constant*: `c_uint`



## libc::unix::linux_like::linux_l4re_shared::MFD_HUGE_MASK

*Constant*: `c_uint`



## libc::unix::linux_like::linux_l4re_shared::MFD_HUGE_SHIFT

*Constant*: `c_uint`



## libc::unix::linux_like::linux_l4re_shared::MFD_NOEXEC_SEAL

*Constant*: `c_uint`



## libc::unix::linux_like::linux_l4re_shared::MLOCK_ONFAULT

*Constant*: `c_uint`



## libc::unix::linux_like::linux_l4re_shared::MON_1

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::MON_10

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::MON_11

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::MON_12

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::MON_2

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::MON_3

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::MON_4

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::MON_5

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::MON_6

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::MON_7

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::MON_8

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::MON_9

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::MSG_COPY

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::MS_NOUSER

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::NI_DGRAM

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::NI_IDN

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::NI_NAMEREQD

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::NI_NOFQDN

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::NI_NUMERICHOST

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::NI_NUMERICSERV

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::NOEXPR

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::NOSTR

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::NT_ASRS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::NT_AUXV

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::NT_FPREGSET

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::NT_GWINDOWS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::NT_LWPSINFO

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::NT_LWPSTATUS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::NT_PLATFORM

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::NT_PRCRED

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::NT_PRFPREG

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::NT_PRFPXREG

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::NT_PRPSINFO

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::NT_PRSTATUS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::NT_PRXREG

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::NT_PSINFO

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::NT_PSTATUS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::NT_TASKSTRUCT

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::NT_UTSNAME

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PACKET_ADD_MEMBERSHIP

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PACKET_AUXDATA

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PACKET_BROADCAST

*Constant*: `c_uchar`



## libc::unix::linux_like::linux_l4re_shared::PACKET_COPY_THRESH

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PACKET_DROP_MEMBERSHIP

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PACKET_HDRLEN

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PACKET_HOST

*Constant*: `c_uchar`



## libc::unix::linux_like::linux_l4re_shared::PACKET_KERNEL

*Constant*: `c_uchar`



## libc::unix::linux_like::linux_l4re_shared::PACKET_LOOPBACK

*Constant*: `c_uchar`



## libc::unix::linux_like::linux_l4re_shared::PACKET_LOSS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PACKET_MR_ALLMULTI

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PACKET_MR_MULTICAST

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PACKET_MR_PROMISC

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PACKET_MR_UNICAST

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PACKET_MULTICAST

*Constant*: `c_uchar`



## libc::unix::linux_like::linux_l4re_shared::PACKET_ORIGDEV

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PACKET_OTHERHOST

*Constant*: `c_uchar`



## libc::unix::linux_like::linux_l4re_shared::PACKET_OUTGOING

*Constant*: `c_uchar`



## libc::unix::linux_like::linux_l4re_shared::PACKET_RECV_OUTPUT

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PACKET_RESERVE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PACKET_RX_RING

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PACKET_STATISTICS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PACKET_TIMESTAMP

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PACKET_TX_RING

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PACKET_TX_TIMESTAMP

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PACKET_USER

*Constant*: `c_uchar`



## libc::unix::linux_like::linux_l4re_shared::PACKET_VERSION

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PACKET_VNET_HDR

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PF_MASKOS

*Constant*: `u32`



## libc::unix::linux_like::linux_l4re_shared::PF_MASKPROC

*Constant*: `u32`



## libc::unix::linux_like::linux_l4re_shared::PF_R

*Constant*: `u32`



## libc::unix::linux_like::linux_l4re_shared::PF_W

*Constant*: `u32`



## libc::unix::linux_like::linux_l4re_shared::PF_X

*Constant*: `u32`



## libc::unix::linux_like::linux_l4re_shared::PM_STR

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::POSIX_MADV_DONTNEED

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::POSIX_MADV_NORMAL

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::POSIX_MADV_RANDOM

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::POSIX_MADV_SEQUENTIAL

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::POSIX_MADV_WILLNEED

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_CAPBSET_DROP

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_CAPBSET_READ

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_CAP_AMBIENT

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_CAP_AMBIENT_CLEAR_ALL

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_CAP_AMBIENT_IS_SET

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_CAP_AMBIENT_LOWER

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_CAP_AMBIENT_RAISE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_ENDIAN_BIG

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_ENDIAN_LITTLE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_ENDIAN_PPC_LITTLE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_FPEMU_NOPRINT

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_FPEMU_SIGFPE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_FP_EXC_ASYNC

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_FP_EXC_DISABLED

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_FP_EXC_DIV

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_FP_EXC_INV

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_FP_EXC_NONRECOV

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_FP_EXC_OVF

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_FP_EXC_PRECISE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_FP_EXC_RES

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_FP_EXC_SW_ENABLE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_FP_EXC_UND

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_FP_MODE_FR

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_FP_MODE_FRE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_GET_CHILD_SUBREAPER

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_GET_DUMPABLE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_GET_ENDIAN

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_GET_FPEMU

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_GET_FPEXC

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_GET_FP_MODE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_GET_KEEPCAPS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_GET_NAME

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_GET_NO_NEW_PRIVS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_GET_PDEATHSIG

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_GET_SECCOMP

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_GET_SECUREBITS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_GET_THP_DISABLE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_GET_TID_ADDRESS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_GET_TIMERSLACK

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_GET_TIMING

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_GET_TSC

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_GET_UNALIGN

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_MCE_KILL

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_MCE_KILL_CLEAR

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_MCE_KILL_DEFAULT

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_MCE_KILL_EARLY

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_MCE_KILL_GET

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_MCE_KILL_LATE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_MCE_KILL_SET

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_MPX_DISABLE_MANAGEMENT

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_MPX_ENABLE_MANAGEMENT

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_SCHED_CORE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_SCHED_CORE_CREATE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_SCHED_CORE_GET

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_SCHED_CORE_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_SCHED_CORE_SCOPE_PROCESS_GROUP

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_SCHED_CORE_SCOPE_THREAD

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_SCHED_CORE_SCOPE_THREAD_GROUP

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_SCHED_CORE_SHARE_FROM

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_SCHED_CORE_SHARE_TO

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_SET_CHILD_SUBREAPER

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_SET_DUMPABLE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_SET_ENDIAN

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_SET_FPEMU

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_SET_FPEXC

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_SET_FP_MODE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_SET_KEEPCAPS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_SET_MM

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_SET_MM_ARG_END

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_SET_MM_ARG_START

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_SET_MM_AUXV

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_SET_MM_BRK

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_SET_MM_END_CODE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_SET_MM_END_DATA

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_SET_MM_ENV_END

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_SET_MM_ENV_START

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_SET_MM_EXE_FILE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_SET_MM_MAP

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_SET_MM_MAP_SIZE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_SET_MM_START_BRK

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_SET_MM_START_CODE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_SET_MM_START_DATA

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_SET_MM_START_STACK

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_SET_NAME

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_SET_NO_NEW_PRIVS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_SET_PDEATHSIG

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_SET_PTRACER

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_SET_PTRACER_ANY

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::PR_SET_SECCOMP

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_SET_SECUREBITS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_SET_THP_DISABLE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_SET_TIMERSLACK

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_SET_TIMING

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_SET_TSC

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_SET_UNALIGN

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_SET_VMA

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_SET_VMA_ANON_NAME

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_TASK_PERF_EVENTS_DISABLE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_TASK_PERF_EVENTS_ENABLE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_TIMING_STATISTICAL

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_TIMING_TIMESTAMP

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_TSC_ENABLE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_TSC_SIGSEGV

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_UNALIGN_NOPRINT

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PR_UNALIGN_SIGBUS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PTHREAD_BARRIER_SERIAL_THREAD

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PTHREAD_EXPLICIT_SCHED

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PTHREAD_INHERIT_SCHED

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PTHREAD_MUTEX_DEFAULT

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PTHREAD_MUTEX_ERRORCHECK

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PTHREAD_MUTEX_NORMAL

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PTHREAD_MUTEX_RECURSIVE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PTHREAD_MUTEX_ROBUST

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PTHREAD_MUTEX_STALLED

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PTHREAD_ONCE_INIT

*Constant*: `crate::pthread_once_t`



## libc::unix::linux_like::linux_l4re_shared::PTHREAD_PRIO_INHERIT

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PTHREAD_PRIO_NONE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PTHREAD_PRIO_PROTECT

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PTHREAD_PROCESS_PRIVATE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PTHREAD_PROCESS_SHARED

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PTRACE_EVENT_STOP

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::PT_DYNAMIC

*Constant*: `u32`



## libc::unix::linux_like::linux_l4re_shared::PT_GNU_EH_FRAME

*Constant*: `u32`



## libc::unix::linux_like::linux_l4re_shared::PT_GNU_RELRO

*Constant*: `u32`



## libc::unix::linux_like::linux_l4re_shared::PT_GNU_STACK

*Constant*: `u32`



## libc::unix::linux_like::linux_l4re_shared::PT_HIOS

*Constant*: `u32`



## libc::unix::linux_like::linux_l4re_shared::PT_HIPROC

*Constant*: `u32`



## libc::unix::linux_like::linux_l4re_shared::PT_HISUNW

*Constant*: `u32`



## libc::unix::linux_like::linux_l4re_shared::PT_INTERP

*Constant*: `u32`



## libc::unix::linux_like::linux_l4re_shared::PT_LOAD

*Constant*: `u32`



## libc::unix::linux_like::linux_l4re_shared::PT_LOOS

*Constant*: `u32`



## libc::unix::linux_like::linux_l4re_shared::PT_LOPROC

*Constant*: `u32`



## libc::unix::linux_like::linux_l4re_shared::PT_LOSUNW

*Constant*: `u32`



## libc::unix::linux_like::linux_l4re_shared::PT_NOTE

*Constant*: `u32`



## libc::unix::linux_like::linux_l4re_shared::PT_NULL

*Constant*: `u32`



## libc::unix::linux_like::linux_l4re_shared::PT_NUM

*Constant*: `u32`



## libc::unix::linux_like::linux_l4re_shared::PT_PHDR

*Constant*: `u32`



## libc::unix::linux_like::linux_l4re_shared::PT_SHLIB

*Constant*: `u32`



## libc::unix::linux_like::linux_l4re_shared::PT_SUNWBSS

*Constant*: `u32`



## libc::unix::linux_like::linux_l4re_shared::PT_SUNWSTACK

*Constant*: `u32`



## libc::unix::linux_like::linux_l4re_shared::PT_TLS

*Constant*: `u32`



## libc::unix::linux_like::linux_l4re_shared::RADIXCHAR

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::REG_BADBR

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::REG_BADPAT

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::REG_BADRPT

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::REG_EBRACE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::REG_EBRACK

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::REG_ECOLLATE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::REG_ECTYPE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::REG_EESCAPE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::REG_ENOSYS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::REG_EPAREN

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::REG_ERANGE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::REG_ESPACE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::REG_ESUBREG

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::REG_EXTENDED

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::REG_ICASE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::REG_NEWLINE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::REG_NOMATCH

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::REG_NOSUB

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::REG_NOTBOL

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::REG_NOTEOL

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::RLIM_SAVED_CUR

*Constant*: `crate::rlim_t`



## libc::unix::linux_like::linux_l4re_shared::RLIM_SAVED_MAX

*Constant*: `crate::rlim_t`



## libc::unix::linux_like::linux_l4re_shared::RTCF_DIRECTSRC

*Constant*: `u32`



## libc::unix::linux_like::linux_l4re_shared::RTCF_DOREDIRECT

*Constant*: `u32`



## libc::unix::linux_like::linux_l4re_shared::RTCF_LOG

*Constant*: `u32`



## libc::unix::linux_like::linux_l4re_shared::RTCF_MASQ

*Constant*: `u32`



## libc::unix::linux_like::linux_l4re_shared::RTCF_NAT

*Constant*: `u32`



## libc::unix::linux_like::linux_l4re_shared::RTCF_VALVE

*Constant*: `u32`



## libc::unix::linux_like::linux_l4re_shared::RTF_ADDRCLASSMASK

*Constant*: `u32`



## libc::unix::linux_like::linux_l4re_shared::RTF_ADDRCONF

*Constant*: `u32`



## libc::unix::linux_like::linux_l4re_shared::RTF_ALLONLINK

*Constant*: `u32`



## libc::unix::linux_like::linux_l4re_shared::RTF_BROADCAST

*Constant*: `u32`



## libc::unix::linux_like::linux_l4re_shared::RTF_CACHE

*Constant*: `u32`



## libc::unix::linux_like::linux_l4re_shared::RTF_DEFAULT

*Constant*: `u32`



## libc::unix::linux_like::linux_l4re_shared::RTF_DYNAMIC

*Constant*: `c_ushort`



## libc::unix::linux_like::linux_l4re_shared::RTF_FLOW

*Constant*: `u32`



## libc::unix::linux_like::linux_l4re_shared::RTF_GATEWAY

*Constant*: `c_ushort`



## libc::unix::linux_like::linux_l4re_shared::RTF_HOST

*Constant*: `c_ushort`



## libc::unix::linux_like::linux_l4re_shared::RTF_INTERFACE

*Constant*: `u32`



## libc::unix::linux_like::linux_l4re_shared::RTF_IRTT

*Constant*: `c_ushort`



## libc::unix::linux_like::linux_l4re_shared::RTF_LINKRT

*Constant*: `u32`



## libc::unix::linux_like::linux_l4re_shared::RTF_LOCAL

*Constant*: `u32`



## libc::unix::linux_like::linux_l4re_shared::RTF_MODIFIED

*Constant*: `c_ushort`



## libc::unix::linux_like::linux_l4re_shared::RTF_MSS

*Constant*: `c_ushort`



## libc::unix::linux_like::linux_l4re_shared::RTF_MTU

*Constant*: `c_ushort`



## libc::unix::linux_like::linux_l4re_shared::RTF_MULTICAST

*Constant*: `u32`



## libc::unix::linux_like::linux_l4re_shared::RTF_NAT

*Constant*: `u32`



## libc::unix::linux_like::linux_l4re_shared::RTF_NOFORWARD

*Constant*: `c_ushort`



## libc::unix::linux_like::linux_l4re_shared::RTF_NONEXTHOP

*Constant*: `u32`



## libc::unix::linux_like::linux_l4re_shared::RTF_NOPMTUDISC

*Constant*: `c_ushort`



## libc::unix::linux_like::linux_l4re_shared::RTF_POLICY

*Constant*: `u32`



## libc::unix::linux_like::linux_l4re_shared::RTF_REINSTATE

*Constant*: `c_ushort`



## libc::unix::linux_like::linux_l4re_shared::RTF_REJECT

*Constant*: `c_ushort`



## libc::unix::linux_like::linux_l4re_shared::RTF_STATIC

*Constant*: `c_ushort`



## libc::unix::linux_like::linux_l4re_shared::RTF_THROW

*Constant*: `c_ushort`



## libc::unix::linux_like::linux_l4re_shared::RTF_UP

*Constant*: `c_ushort`



## libc::unix::linux_like::linux_l4re_shared::RTF_WINDOW

*Constant*: `c_ushort`



## libc::unix::linux_like::linux_l4re_shared::RTF_XRESOLVE

*Constant*: `c_ushort`



## libc::unix::linux_like::linux_l4re_shared::RTLD_DEFAULT

*Constant*: `*mut c_void`



## libc::unix::linux_like::linux_l4re_shared::RTLD_NEXT

*Constant*: `*mut c_void`



## libc::unix::linux_like::linux_l4re_shared::RTLD_NODELETE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::RTLD_NOW

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::RT_ADDRCLASS

*Function*

```rust
fn RT_ADDRCLASS(flags: u32) -> u32
```



## libc::unix::linux_like::linux_l4re_shared::RT_CLASS_DEFAULT

*Constant*: `u8`



## libc::unix::linux_like::linux_l4re_shared::RT_CLASS_LOCAL

*Constant*: `u8`



## libc::unix::linux_like::linux_l4re_shared::RT_CLASS_MAIN

*Constant*: `u8`



## libc::unix::linux_like::linux_l4re_shared::RT_CLASS_MAX

*Constant*: `u8`



## libc::unix::linux_like::linux_l4re_shared::RT_CLASS_UNSPEC

*Constant*: `u8`



## libc::unix::linux_like::linux_l4re_shared::RT_LOCALADDR

*Function*

```rust
fn RT_LOCALADDR(flags: u32) -> bool
```



## libc::unix::linux_like::linux_l4re_shared::RT_TOS

*Function*

```rust
fn RT_TOS(tos: u8) -> u8
```



## libc::unix::linux_like::linux_l4re_shared::RUSAGE_CHILDREN

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::RUSAGE_THREAD

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::SCHED_BATCH

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::SCHED_DEADLINE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::SCHED_FIFO

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::SCHED_IDLE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::SCHED_NORMAL

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::SCHED_OTHER

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::SCHED_RESET_ON_FORK

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::SCHED_RR

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::SELFMAG

*Constant*: `usize`



## libc::unix::linux_like::linux_l4re_shared::SEM_FAILED

*Constant*: `*mut crate::sem_t`



## libc::unix::linux_like::linux_l4re_shared::SHM_EXEC

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::SHM_HUGETLB

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::SHM_LOCK

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::SHM_NORESERVE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::SHM_R

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::SHM_RDONLY

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::SHM_REMAP

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::SHM_RND

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::SHM_UNLOCK

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::SHM_W

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::SIOCADDMULTI

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCADDRT

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCDARP

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCDELMULTI

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCDELRT

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCDIFADDR

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCDRARP

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCETHTOOL

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCGARP

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCGIFADDR

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCGIFBR

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCGIFBRDADDR

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCGIFCONF

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCGIFCOUNT

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCGIFDSTADDR

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCGIFENCAP

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCGIFFLAGS

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCGIFHWADDR

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCGIFINDEX

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCGIFMAP

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCGIFMEM

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCGIFMETRIC

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCGIFMTU

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCGIFNAME

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCGIFNETMASK

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCGIFPFLAGS

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCGIFSLAVE

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCGIFTXQLEN

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCGMIIPHY

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCGMIIREG

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCGRARP

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCGSKNS

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCOUTQNSD

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCSARP

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCSIFADDR

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCSIFBR

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCSIFBRDADDR

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCSIFDSTADDR

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCSIFENCAP

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCSIFFLAGS

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCSIFHWADDR

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCSIFHWBROADCAST

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCSIFLINK

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCSIFMAP

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCSIFMEM

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCSIFMETRIC

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCSIFMTU

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCSIFNAME

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCSIFNETMASK

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCSIFPFLAGS

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCSIFSLAVE

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCSIFTXQLEN

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCSMIIREG

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCSRARP

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOCWANDEV

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::SIOGIFINDEX

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::ST_APPEND

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::ST_IMMUTABLE

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::ST_MANDLOCK

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::ST_NOATIME

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::ST_NODEV

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::ST_NODIRATIME

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::ST_NOEXEC

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::ST_NOSUID

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::ST_RDONLY

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::ST_SYNCHRONOUS

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::ST_WRITE

*Constant*: `c_ulong`



## libc::unix::linux_like::linux_l4re_shared::S_IEXEC

*Constant*: `crate::mode_t`



## libc::unix::linux_like::linux_l4re_shared::S_IREAD

*Constant*: `crate::mode_t`



## libc::unix::linux_like::linux_l4re_shared::S_IWRITE

*Constant*: `crate::mode_t`



## libc::unix::linux_like::linux_l4re_shared::THOUSEP

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::T_FMT

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::T_FMT_AMPM

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::UDP_CORK

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::UDP_ENCAP

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::UDP_GRO

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::UDP_NO_CHECK6_RX

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::UDP_NO_CHECK6_TX

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::UDP_SEGMENT

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::YESEXPR

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::YESSTR

*Constant*: `crate::nl_item`



## libc::unix::linux_like::linux_l4re_shared::_CS_PATH

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_CS_POSIX_V5_WIDTH_RESTRICTED_ENVS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_CS_POSIX_V6_ILP32_OFF32_CFLAGS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_CS_POSIX_V6_ILP32_OFF32_LDFLAGS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_CS_POSIX_V6_ILP32_OFF32_LIBS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_CS_POSIX_V6_ILP32_OFF32_LINTFLAGS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_CS_POSIX_V6_ILP32_OFFBIG_CFLAGS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_CS_POSIX_V6_ILP32_OFFBIG_LDFLAGS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_CS_POSIX_V6_ILP32_OFFBIG_LIBS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_CS_POSIX_V6_ILP32_OFFBIG_LINTFLAGS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_CS_POSIX_V6_LP64_OFF64_CFLAGS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_CS_POSIX_V6_LP64_OFF64_LDFLAGS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_CS_POSIX_V6_LP64_OFF64_LIBS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_CS_POSIX_V6_LP64_OFF64_LINTFLAGS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_CS_POSIX_V6_LPBIG_OFFBIG_CFLAGS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_CS_POSIX_V6_LPBIG_OFFBIG_LDFLAGS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_CS_POSIX_V6_LPBIG_OFFBIG_LIBS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_CS_POSIX_V6_LPBIG_OFFBIG_LINTFLAGS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_CS_POSIX_V6_WIDTH_RESTRICTED_ENVS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_CS_POSIX_V7_ILP32_OFF32_CFLAGS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_CS_POSIX_V7_ILP32_OFF32_LDFLAGS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_CS_POSIX_V7_ILP32_OFF32_LIBS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_CS_POSIX_V7_ILP32_OFF32_LINTFLAGS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_CS_POSIX_V7_ILP32_OFFBIG_CFLAGS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_CS_POSIX_V7_ILP32_OFFBIG_LDFLAGS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_CS_POSIX_V7_ILP32_OFFBIG_LIBS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_CS_POSIX_V7_ILP32_OFFBIG_LINTFLAGS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_CS_POSIX_V7_LP64_OFF64_CFLAGS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_CS_POSIX_V7_LP64_OFF64_LDFLAGS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_CS_POSIX_V7_LP64_OFF64_LIBS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_CS_POSIX_V7_LP64_OFF64_LINTFLAGS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_CS_POSIX_V7_LPBIG_OFFBIG_CFLAGS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_CS_POSIX_V7_LPBIG_OFFBIG_LDFLAGS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_CS_POSIX_V7_LPBIG_OFFBIG_LIBS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_CS_POSIX_V7_LPBIG_OFFBIG_LINTFLAGS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_CS_POSIX_V7_WIDTH_RESTRICTED_ENVS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_PC_2_SYMLINKS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_PC_ALLOC_SIZE_MIN

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_PC_ASYNC_IO

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_PC_CHOWN_RESTRICTED

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_PC_FILESIZEBITS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_PC_LINK_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_PC_MAX_CANON

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_PC_MAX_INPUT

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_PC_NAME_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_PC_NO_TRUNC

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_PC_PATH_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_PC_PIPE_BUF

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_PC_PRIO_IO

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_PC_REC_INCR_XFER_SIZE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_PC_REC_MAX_XFER_SIZE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_PC_REC_MIN_XFER_SIZE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_PC_REC_XFER_ALIGN

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_PC_SOCK_MAXBUF

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_PC_SYMLINK_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_PC_SYNC_IO

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_PC_VDISABLE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_POSIX_VDISABLE

*Constant*: `crate::cc_t`



## libc::unix::linux_like::linux_l4re_shared::_SC_2_CHAR_TERM

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_2_C_BIND

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_2_C_DEV

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_2_FORT_DEV

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_2_FORT_RUN

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_2_LOCALEDEF

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_2_PBS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_2_PBS_ACCOUNTING

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_2_PBS_CHECKPOINT

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_2_PBS_LOCATE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_2_PBS_MESSAGE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_2_PBS_TRACK

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_2_SW_DEV

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_2_UPE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_2_VERSION

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_ADVISORY_INFO

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_AIO_LISTIO_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_AIO_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_AIO_PRIO_DELTA_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_ARG_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_ASYNCHRONOUS_IO

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_ATEXIT_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_AVPHYS_PAGES

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_BARRIERS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_BC_BASE_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_BC_DIM_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_BC_SCALE_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_BC_STRING_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_CHILD_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_CLK_TCK

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_CLOCK_SELECTION

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_COLL_WEIGHTS_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_CPUTIME

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_DELAYTIMER_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_EXPR_NEST_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_FSYNC

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_GETGR_R_SIZE_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_GETPW_R_SIZE_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_HOST_NAME_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_IOV_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_IPV6

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_JOB_CONTROL

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_LINE_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_LOGIN_NAME_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_MAPPED_FILES

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_MEMLOCK

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_MEMLOCK_RANGE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_MEMORY_PROTECTION

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_MESSAGE_PASSING

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_MONOTONIC_CLOCK

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_MQ_OPEN_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_MQ_PRIO_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_NGROUPS_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_NPROCESSORS_CONF

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_NPROCESSORS_ONLN

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_NZERO

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_OPEN_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_PAGESIZE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_PAGE_SIZE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_PASS_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_PHYS_PAGES

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_PRIORITIZED_IO

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_PRIORITY_SCHEDULING

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_RAW_SOCKETS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_READER_WRITER_LOCKS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_REALTIME_SIGNALS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_REGEXP

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_RE_DUP_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_RTSIG_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_SAVED_IDS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_SEMAPHORES

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_SEM_NSEMS_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_SEM_VALUE_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_SHARED_MEMORY_OBJECTS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_SHELL

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_SIGQUEUE_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_SPAWN

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_SPIN_LOCKS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_SPORADIC_SERVER

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_SS_REPL_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_STREAMS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_STREAM_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_SYMLOOP_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_SYNCHRONIZED_IO

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_THREADS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_THREAD_ATTR_STACKADDR

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_THREAD_ATTR_STACKSIZE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_THREAD_CPUTIME

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_THREAD_DESTRUCTOR_ITERATIONS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_THREAD_KEYS_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_THREAD_PRIORITY_SCHEDULING

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_THREAD_PRIO_INHERIT

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_THREAD_PRIO_PROTECT

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_THREAD_PROCESS_SHARED

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_THREAD_ROBUST_PRIO_INHERIT

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_THREAD_ROBUST_PRIO_PROTECT

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_THREAD_SAFE_FUNCTIONS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_THREAD_SPORADIC_SERVER

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_THREAD_STACK_MIN

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_THREAD_THREADS_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_TIMEOUTS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_TIMERS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_TIMER_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_TRACE

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_TRACE_EVENT_FILTER

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_TRACE_EVENT_NAME_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_TRACE_INHERIT

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_TRACE_LOG

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_TRACE_NAME_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_TRACE_SYS_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_TRACE_USER_EVENT_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_TTY_NAME_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_TYPED_MEMORY_OBJECTS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_TZNAME_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_UIO_MAXIOV

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_V6_ILP32_OFF32

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_V6_ILP32_OFFBIG

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_V6_LP64_OFF64

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_V6_LPBIG_OFFBIG

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_V7_ILP32_OFF32

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_V7_ILP32_OFFBIG

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_V7_LP64_OFF64

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_V7_LPBIG_OFFBIG

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_VERSION

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_XBS5_ILP32_OFF32

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_XBS5_ILP32_OFFBIG

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_XBS5_LP64_OFF64

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_XBS5_LPBIG_OFFBIG

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_XOPEN_CRYPT

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_XOPEN_ENH_I18N

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_XOPEN_LEGACY

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_XOPEN_REALTIME

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_XOPEN_REALTIME_THREADS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_XOPEN_SHM

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_XOPEN_STREAMS

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_XOPEN_UNIX

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_XOPEN_VERSION

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_XOPEN_XCU_VERSION

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_XOPEN_XPG2

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_XOPEN_XPG3

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::_SC_XOPEN_XPG4

*Constant*: `c_int`



## libc::unix::linux_like::linux_l4re_shared::__SIZEOF_PTHREAD_COND_T

*Constant*: `usize`



## libc::unix::linux_like::linux_l4re_shared::__c_anonymous_elf32_rel

*Struct*

**Fields:**
- `r_offset: Elf32_Addr`
- `r_info: Elf32_Word`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> __c_anonymous_elf32_rel`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux_l4re_shared::__c_anonymous_elf32_rela

*Struct*

**Fields:**
- `r_offset: crate::Elf32_Addr`
- `r_info: crate::Elf32_Word`
- `r_addend: crate::Elf32_Sword`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> __c_anonymous_elf32_rela`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux_l4re_shared::__c_anonymous_elf64_rel

*Struct*

**Fields:**
- `r_offset: Elf64_Addr`
- `r_info: Elf64_Xword`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> __c_anonymous_elf64_rel`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux_l4re_shared::__c_anonymous_elf64_rela

*Struct*

**Fields:**
- `r_offset: crate::Elf64_Addr`
- `r_info: crate::Elf64_Xword`
- `r_addend: crate::Elf64_Sxword`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> __c_anonymous_elf64_rela`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux_l4re_shared::__c_anonymous_ifc_ifcu

*Union*

**Fields:**
- `ifcu_buf: *mut c_char`
- `ifcu_req: *mut crate::ifreq`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> __c_anonymous_ifc_ifcu`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`



## libc::unix::linux_like::linux_l4re_shared::__c_anonymous_ifr_ifru

*Union*

**Fields:**
- `ifru_addr: crate::sockaddr`
- `ifru_dstaddr: crate::sockaddr`
- `ifru_broadaddr: crate::sockaddr`
- `ifru_netmask: crate::sockaddr`
- `ifru_hwaddr: crate::sockaddr`
- `ifru_flags: c_short`
- `ifru_ifindex: c_int`
- `ifru_metric: c_int`
- `ifru_mtu: c_int`
- `ifru_map: __c_anonymous_ifru_map`
- `ifru_slave: [c_char; 16]`
- `ifru_newname: [c_char; 16]`
- `ifru_data: *mut c_char`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> __c_anonymous_ifr_ifru`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`



## libc::unix::linux_like::linux_l4re_shared::__c_anonymous_ifru_map

*Struct*

**Fields:**
- `mem_start: c_ulong`
- `mem_end: c_ulong`
- `base_addr: c_ushort`
- `irq: c_uchar`
- `dma: c_uchar`
- `port: c_uchar`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> __c_anonymous_ifru_map`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux_l4re_shared::__errno_location

*Function*

```rust
fn __errno_location() -> *mut c_int
```



## libc::unix::linux_like::linux_l4re_shared::abs

*Function*

```rust
fn abs(i: c_int) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::addmntent

*Function*

```rust
fn addmntent(stream: *mut crate::FILE, mnt: *const crate::mntent) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::aio_cancel

*Function*

```rust
fn aio_cancel(fd: c_int, aiocbp: *mut crate::aiocb) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::aio_error

*Function*

```rust
fn aio_error(aiocbp: *const crate::aiocb) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::aio_fsync

*Function*

```rust
fn aio_fsync(op: c_int, aiocbp: *mut crate::aiocb) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::aio_read

*Function*

```rust
fn aio_read(aiocbp: *mut crate::aiocb) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::aio_return

*Function*

```rust
fn aio_return(aiocbp: *mut crate::aiocb) -> ssize_t
```



## libc::unix::linux_like::linux_l4re_shared::aio_suspend

*Function*

```rust
fn aio_suspend(aiocb_list: *const *const crate::aiocb, nitems: c_int, timeout: *const crate::timespec) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::aio_write

*Function*

```rust
fn aio_write(aiocbp: *mut crate::aiocb) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::arpd_request

*Struct*

**Fields:**
- `req: c_ushort`
- `ip: u32`
- `dev: c_ulong`
- `stamp: c_ulong`
- `updated: c_ulong`
- `ha: [c_uchar; 7]`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> arpd_request`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux_l4re_shared::copy_file_range

*Function*

```rust
fn copy_file_range(fd_in: c_int, off_in: *mut crate::off64_t, fd_out: c_int, off_out: *mut crate::off64_t, len: size_t, flags: c_uint) -> ssize_t
```



## libc::unix::linux_like::linux_l4re_shared::cpu_set_t

*Struct*

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> cpu_set_t`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux_l4re_shared::daemon

*Function*

```rust
fn daemon(nochdir: c_int, noclose: c_int) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::dirent

*Struct*

**Fields:**
- `d_ino: crate::ino_t`
- `d_off: crate::off_t`
- `d_reclen: c_ushort`
- `d_type: c_uchar`
- `d_name: [c_char; 256]`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> dirent`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux_l4re_shared::dirent64

*Struct*

**Fields:**
- `d_ino: crate::ino64_t`
- `d_off: crate::off64_t`
- `d_reclen: c_ushort`
- `d_type: c_uchar`
- `d_name: [c_char; 256]`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> dirent64`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux_l4re_shared::dl_iterate_phdr

*Function*

```rust
fn dl_iterate_phdr(callback: Option<fn(...)>, data: *mut c_void) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::dl_phdr_info

*Struct*

**Fields:**
- `dlpi_addr: Elf64_Addr`
- `dlpi_name: *const c_char`
- `dlpi_phdr: *const Elf64_Phdr`
- `dlpi_phnum: Elf64_Half`
- `dlpi_adds: c_ulonglong`
- `dlpi_subs: c_ulonglong`
- `dlpi_tls_modid: size_t`
- `dlpi_tls_data: *mut c_void`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> dl_phdr_info`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux_l4re_shared::drand48

*Function*

```rust
fn drand48() -> c_double
```



## libc::unix::linux_like::linux_l4re_shared::endgrent

*Function*

```rust
fn endgrent()
```



## libc::unix::linux_like::linux_l4re_shared::endmntent

*Function*

```rust
fn endmntent(streamp: *mut crate::FILE) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::endpwent

*Function*

```rust
fn endpwent()
```



## libc::unix::linux_like::linux_l4re_shared::endspent

*Function*

```rust
fn endspent()
```



## libc::unix::linux_like::linux_l4re_shared::erand48

*Function*

```rust
fn erand48(xseed: *mut c_ushort) -> c_double
```



## libc::unix::linux_like::linux_l4re_shared::faccessat

*Function*

```rust
fn faccessat(dirfd: c_int, pathname: *const c_char, mode: c_int, flags: c_int) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::freopen64

*Function*

```rust
fn freopen64(filename: *const c_char, mode: *const c_char, file: *mut crate::FILE) -> *mut crate::FILE
```



## libc::unix::linux_like::linux_l4re_shared::fseeko64

*Function*

```rust
fn fseeko64(stream: *mut crate::FILE, offset: crate::off64_t, whence: c_int) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::fsetpos64

*Function*

```rust
fn fsetpos64(stream: *mut crate::FILE, ptr: *const crate::fpos64_t) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::fsid_t

*Struct*

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> fsid_t`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux_l4re_shared::ftello64

*Function*

```rust
fn ftello64(stream: *mut crate::FILE) -> crate::off64_t
```



## libc::unix::linux_like::linux_l4re_shared::futimes

*Function*

```rust
fn futimes(fd: c_int, times: *const crate::timeval) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::getgrent

*Function*

```rust
fn getgrent() -> *mut crate::group
```



## libc::unix::linux_like::linux_l4re_shared::getgrgid

*Function*

```rust
fn getgrgid(gid: crate::gid_t) -> *mut crate::group
```



## libc::unix::linux_like::linux_l4re_shared::getgrgid_r

*Function*

```rust
fn getgrgid_r(gid: crate::gid_t, grp: *mut crate::group, buf: *mut c_char, buflen: size_t, result: *mut *mut crate::group) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::getgrnam

*Function*

```rust
fn getgrnam(name: *const c_char) -> *mut crate::group
```



## libc::unix::linux_like::linux_l4re_shared::getgrnam_r

*Function*

```rust
fn getgrnam_r(name: *const c_char, grp: *mut crate::group, buf: *mut c_char, buflen: size_t, result: *mut *mut crate::group) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::getloadavg

*Function*

```rust
fn getloadavg(loadavg: *mut c_double, nelem: c_int) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::getmntent

*Function*

```rust
fn getmntent(stream: *mut crate::FILE) -> *mut crate::mntent
```



## libc::unix::linux_like::linux_l4re_shared::getnameinfo

*Function*

```rust
fn getnameinfo(sa: *const crate::sockaddr, salen: crate::socklen_t, host: *mut c_char, hostlen: crate::socklen_t, serv: *mut c_char, servlen: crate::socklen_t, flags: c_int) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::getopt_long

*Function*

```rust
fn getopt_long(argc: c_int, argv: *const *mut c_char, optstring: *const c_char, longopts: *const option, longindex: *mut c_int) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::getpwent

*Function*

```rust
fn getpwent() -> *mut passwd
```



## libc::unix::linux_like::linux_l4re_shared::getspent

*Function*

```rust
fn getspent() -> *mut spwd
```



## libc::unix::linux_like::linux_l4re_shared::getspnam

*Function*

```rust
fn getspnam(name: *const c_char) -> *mut spwd
```



## libc::unix::linux_like::linux_l4re_shared::gettid

*Function*

```rust
fn gettid() -> crate::pid_t
```



## libc::unix::linux_like::linux_l4re_shared::glob

*Function*

```rust
fn glob(pattern: *const c_char, flags: c_int, errfunc: Option<fn(...)>, pglob: *mut crate::glob_t) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::glob_t

*Struct*

**Fields:**
- `gl_pathc: size_t`
- `gl_pathv: *mut *mut c_char`
- `gl_offs: size_t`
- `gl_flags: c_int`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> glob_t`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux_l4re_shared::globfree

*Function*

```rust
fn globfree(pglob: *mut crate::glob_t)
```



## libc::unix::linux_like::linux_l4re_shared::hasmntopt

*Function*

```rust
fn hasmntopt(mnt: *const crate::mntent, opt: *const c_char) -> *mut c_char
```



## libc::unix::linux_like::linux_l4re_shared::iconv

*Function*

```rust
fn iconv(cd: iconv_t, inbuf: *mut *mut c_char, inbytesleft: *mut size_t, outbuf: *mut *mut c_char, outbytesleft: *mut size_t) -> size_t
```



## libc::unix::linux_like::linux_l4re_shared::iconv_close

*Function*

```rust
fn iconv_close(cd: iconv_t) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::iconv_open

*Function*

```rust
fn iconv_open(tocode: *const c_char, fromcode: *const c_char) -> iconv_t
```



## libc::unix::linux_like::linux_l4re_shared::iconv_t

*Type Alias*: `*mut c_void`



## libc::unix::linux_like::linux_l4re_shared::ifconf

*Struct*

Structure used in SIOCGIFCONF request.  Used to retrieve interface configuration for
machine (useful for programs which must know all networks accessible).

**Fields:**
- `ifc_len: c_int` - Size of buffer
- `ifc_ifcu: __c_anonymous_ifc_ifcu`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ifconf`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux_l4re_shared::ifreq

*Struct*

**Fields:**
- `ifr_name: [c_char; 16]` - interface name, e.g. "en0"
- `ifr_ifru: __c_anonymous_ifr_ifru`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ifreq`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux_l4re_shared::in6_pktinfo

*Struct*

**Fields:**
- `ipi6_addr: crate::in6_addr`
- `ipi6_ifindex: c_uint`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> in6_pktinfo`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux_l4re_shared::initgroups

*Function*

```rust
fn initgroups(user: *const c_char, group: crate::gid_t) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::ioperm

*Function*

```rust
fn ioperm(from: c_ulong, num: c_ulong, turn_on: c_int) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::iopl

*Function*

```rust
fn iopl(level: c_int) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::itimerspec

*Struct*

**Fields:**
- `it_interval: crate::timespec`
- `it_value: crate::timespec`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> itimerspec`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux_l4re_shared::jrand48

*Function*

```rust
fn jrand48(xseed: *mut c_ushort) -> c_long
```



## libc::unix::linux_like::linux_l4re_shared::labs

*Function*

```rust
fn labs(i: c_long) -> c_long
```



## libc::unix::linux_like::linux_l4re_shared::lio_listio

*Function*

```rust
fn lio_listio(mode: c_int, aiocb_list: *const *mut crate::aiocb, nitems: c_int, sevp: *mut crate::sigevent) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::lrand48

*Function*

```rust
fn lrand48() -> c_long
```



## libc::unix::linux_like::linux_l4re_shared::madvise

*Function*

```rust
fn madvise(addr: *mut c_void, len: size_t, advice: c_int) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::major

*Function*

```rust
fn major(dev: crate::dev_t) -> c_uint
```



## libc::unix::linux_like::linux_l4re_shared::makedev

*Function*

```rust
fn makedev(major: c_uint, minor: c_uint) -> crate::dev_t
```



## libc::unix::linux_like::linux_l4re_shared::memmem

*Function*

```rust
fn memmem(haystack: *const c_void, haystacklen: size_t, needle: *const c_void, needlelen: size_t) -> *mut c_void
```



## libc::unix::linux_like::linux_l4re_shared::minor

*Function*

```rust
fn minor(dev: crate::dev_t) -> c_uint
```



## libc::unix::linux_like::linux_l4re_shared::mntent

*Struct*

**Fields:**
- `mnt_fsname: *mut c_char`
- `mnt_dir: *mut c_char`
- `mnt_type: *mut c_char`
- `mnt_opts: *mut c_char`
- `mnt_freq: c_int`
- `mnt_passno: c_int`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> mntent`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux_l4re_shared::mount

*Function*

```rust
fn mount(src: *const c_char, target: *const c_char, fstype: *const c_char, flags: c_ulong, data: *const c_void) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::mprotect

*Function*

```rust
fn mprotect(addr: *mut c_void, len: size_t, prot: c_int) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::mremap

*Function*

```rust
fn mremap(addr: *mut c_void, len: size_t, new_len: size_t, flags: c_int) -> *mut c_void
```



## libc::unix::linux_like::linux_l4re_shared::msync

*Function*

```rust
fn msync(addr: *mut c_void, len: size_t, flags: c_int) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::nl_langinfo

*Function*

```rust
fn nl_langinfo(item: crate::nl_item) -> *mut c_char
```



## libc::unix::linux_like::linux_l4re_shared::nl_langinfo_l

*Function*

```rust
fn nl_langinfo_l(item: crate::nl_item, locale: crate::locale_t) -> *mut c_char
```



## libc::unix::linux_like::linux_l4re_shared::nrand48

*Function*

```rust
fn nrand48(xseed: *mut c_ushort) -> c_long
```



## libc::unix::linux_like::linux_l4re_shared::option

*Struct*

**Fields:**
- `name: *const c_char`
- `has_arg: c_int`
- `flag: *mut c_int`
- `val: c_int`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> option`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux_l4re_shared::packet_mreq

*Struct*

**Fields:**
- `mr_ifindex: c_int`
- `mr_type: c_ushort`
- `mr_alen: c_ushort`
- `mr_address: [c_uchar; 8]`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> packet_mreq`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux_l4re_shared::passwd

*Struct*

**Fields:**
- `pw_name: *mut c_char`
- `pw_passwd: *mut c_char`
- `pw_uid: crate::uid_t`
- `pw_gid: crate::gid_t`
- `pw_gecos: *mut c_char`
- `pw_dir: *mut c_char`
- `pw_shell: *mut c_char`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> passwd`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux_l4re_shared::popen

*Function*

```rust
fn popen(command: *const c_char, mode: *const c_char) -> *mut crate::FILE
```



## libc::unix::linux_like::linux_l4re_shared::ppoll

*Function*

```rust
fn ppoll(fds: *mut crate::pollfd, nfds: crate::nfds_t, timeout: *const crate::timespec, sigmask: *const crate::sigset_t) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::prctl

*Function*

```rust
fn prctl(option: c_int) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::preadv

*Function*

```rust
fn preadv(fd: c_int, iov: *const crate::iovec, iovcnt: c_int, offset: crate::off_t) -> ssize_t
```



## libc::unix::linux_like::linux_l4re_shared::process_vm_readv

*Function*

```rust
fn process_vm_readv(pid: crate::pid_t, local_iov: *const crate::iovec, liovcnt: c_ulong, remote_iov: *const crate::iovec, riovcnt: c_ulong, flags: c_ulong) -> isize
```



## libc::unix::linux_like::linux_l4re_shared::process_vm_writev

*Function*

```rust
fn process_vm_writev(pid: crate::pid_t, local_iov: *const crate::iovec, liovcnt: c_ulong, remote_iov: *const crate::iovec, riovcnt: c_ulong, flags: c_ulong) -> isize
```



## libc::unix::linux_like::linux_l4re_shared::pwritev

*Function*

```rust
fn pwritev(fd: c_int, iov: *const crate::iovec, iovcnt: c_int, offset: crate::off_t) -> ssize_t
```



## libc::unix::linux_like::linux_l4re_shared::rand

*Function*

```rust
fn rand() -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::recvfrom

*Function*

```rust
fn recvfrom(socket: c_int, buf: *mut c_void, len: size_t, flags: c_int, addr: *mut crate::sockaddr, addrlen: *mut crate::socklen_t) -> ssize_t
```



## libc::unix::linux_like::linux_l4re_shared::regcomp

*Function*

```rust
fn regcomp(preg: *mut crate::regex_t, pattern: *const c_char, cflags: c_int) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::regerror

*Function*

```rust
fn regerror(errcode: c_int, preg: *const crate::regex_t, errbuf: *mut c_char, errbuf_size: size_t) -> size_t
```



## libc::unix::linux_like::linux_l4re_shared::regexec

*Function*

```rust
fn regexec(preg: *const crate::regex_t, input: *const c_char, nmatch: size_t, pmatch: *mut regmatch_t, eflags: c_int) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::regfree

*Function*

```rust
fn regfree(preg: *mut crate::regex_t)
```



## libc::unix::linux_like::linux_l4re_shared::regmatch_t

*Struct*

**Fields:**
- `rm_so: crate::regoff_t`
- `rm_eo: crate::regoff_t`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> regmatch_t`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux_l4re_shared::rlimit64

*Struct*

**Fields:**
- `rlim_cur: crate::rlim64_t`
- `rlim_max: crate::rlim64_t`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> rlimit64`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux_l4re_shared::sched_get_priority_max

*Function*

```rust
fn sched_get_priority_max(policy: c_int) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::sched_get_priority_min

*Function*

```rust
fn sched_get_priority_min(policy: c_int) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::sched_getaffinity

*Function*

```rust
fn sched_getaffinity(pid: crate::pid_t, cpusetsize: size_t, cpuset: *mut cpu_set_t) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::sched_getcpu

*Function*

```rust
fn sched_getcpu() -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::seekdir

*Function*

```rust
fn seekdir(dirp: *mut crate::DIR, loc: c_long)
```



## libc::unix::linux_like::linux_l4re_shared::sem_close

*Function*

```rust
fn sem_close(sem: *mut crate::sem_t) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::sem_getvalue

*Function*

```rust
fn sem_getvalue(sem: *mut crate::sem_t, sval: *mut c_int) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::sem_open

*Function*

```rust
fn sem_open(name: *const c_char, oflag: c_int) -> *mut crate::sem_t
```



## libc::unix::linux_like::linux_l4re_shared::sem_timedwait

*Function*

```rust
fn sem_timedwait(sem: *mut crate::sem_t, abstime: *const crate::timespec) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::sem_unlink

*Function*

```rust
fn sem_unlink(name: *const c_char) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::sembuf

*Struct*

**Fields:**
- `sem_num: c_ushort`
- `sem_op: c_short`
- `sem_flg: c_short`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> sembuf`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux_l4re_shared::setgrent

*Function*

```rust
fn setgrent()
```



## libc::unix::linux_like::linux_l4re_shared::sethostname

*Function*

```rust
fn sethostname(name: *const c_char, len: size_t) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::setmntent

*Function*

```rust
fn setmntent(filename: *const c_char, ty: *const c_char) -> *mut crate::FILE
```



## libc::unix::linux_like::linux_l4re_shared::setpwent

*Function*

```rust
fn setpwent()
```



## libc::unix::linux_like::linux_l4re_shared::setspent

*Function*

```rust
fn setspent()
```



## libc::unix::linux_like::linux_l4re_shared::settimeofday

*Function*

```rust
fn settimeofday(tv: *const crate::timeval, tz: *const crate::timezone) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::shmat

*Function*

```rust
fn shmat(shmid: c_int, shmaddr: *const c_void, shmflg: c_int) -> *mut c_void
```



## libc::unix::linux_like::linux_l4re_shared::shmctl

*Function*

```rust
fn shmctl(shmid: c_int, cmd: c_int, buf: *mut crate::shmid_ds) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::shmdt

*Function*

```rust
fn shmdt(shmaddr: *const c_void) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::shmget

*Function*

```rust
fn shmget(key: crate::key_t, size: size_t, shmflg: c_int) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::sigsuspend

*Function*

```rust
fn sigsuspend(mask: *const crate::sigset_t) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::sigwait

*Function*

```rust
fn sigwait(set: *const crate::sigset_t, sig: *mut c_int) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::spwd

*Struct*

**Fields:**
- `sp_namp: *mut c_char`
- `sp_pwdp: *mut c_char`
- `sp_lstchg: c_long`
- `sp_min: c_long`
- `sp_max: c_long`
- `sp_warn: c_long`
- `sp_inact: c_long`
- `sp_expire: c_long`
- `sp_flag: c_ulong`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> spwd`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux_l4re_shared::srand

*Function*

```rust
fn srand(seed: c_uint)
```



## libc::unix::linux_like::linux_l4re_shared::srand48

*Function*

```rust
fn srand48(seed: c_long)
```



## libc::unix::linux_like::linux_l4re_shared::strerror_r

*Function*

```rust
fn strerror_r(errnum: c_int, buf: *mut c_char, buflen: size_t) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::sysinfo

*Function*

```rust
fn sysinfo(info: *mut crate::sysinfo) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::telldir

*Function*

```rust
fn telldir(dirp: *mut crate::DIR) -> c_long
```



## libc::unix::linux_like::linux_l4re_shared::timer_create

*Function*

```rust
fn timer_create(clockid: crate::clockid_t, sevp: *mut crate::sigevent, timerid: *mut crate::timer_t) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::timer_delete

*Function*

```rust
fn timer_delete(timerid: crate::timer_t) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::timer_getoverrun

*Function*

```rust
fn timer_getoverrun(timerid: crate::timer_t) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::timer_gettime

*Function*

```rust
fn timer_gettime(timerid: crate::timer_t, curr_value: *mut crate::itimerspec) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::timer_settime

*Function*

```rust
fn timer_settime(timerid: crate::timer_t, flags: c_int, new_value: *const crate::itimerspec, old_value: *mut crate::itimerspec) -> c_int
```



## libc::unix::linux_like::linux_l4re_shared::ucred

*Struct*

**Fields:**
- `pid: crate::pid_t`
- `uid: crate::uid_t`
- `gid: crate::gid_t`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ucred`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



