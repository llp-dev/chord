**libc > unix > linux_like > linux > gnu**

# Module: unix::linux_like::linux::gnu

## Contents

**Structs**

- [`Elf32_Chdr`](#elf32_chdr)
- [`Elf64_Chdr`](#elf64_chdr)
- [`__c_anonymous_ptrace_syscall_info_entry`](#__c_anonymous_ptrace_syscall_info_entry)
- [`__c_anonymous_ptrace_syscall_info_exit`](#__c_anonymous_ptrace_syscall_info_exit)
- [`__c_anonymous_ptrace_syscall_info_seccomp`](#__c_anonymous_ptrace_syscall_info_seccomp)
- [`__exit_status`](#__exit_status)
- [`__timeval`](#__timeval)
- [`aiocb`](#aiocb)
- [`cmsghdr`](#cmsghdr)
- [`fanotify_event_info_error`](#fanotify_event_info_error)
- [`fanotify_event_info_pidfd`](#fanotify_event_info_pidfd)
- [`fpos64_t`](#fpos64_t)
- [`fpos_t`](#fpos_t)
- [`glob64_t`](#glob64_t)
- [`iocb`](#iocb)
- [`mallinfo`](#mallinfo)
- [`mallinfo2`](#mallinfo2)
- [`mbstate_t`](#mbstate_t)
- [`msghdr`](#msghdr)
- [`ntptimeval`](#ntptimeval)
- [`ptrace_peeksiginfo_args`](#ptrace_peeksiginfo_args)
- [`ptrace_sud_config`](#ptrace_sud_config)
- [`ptrace_syscall_info`](#ptrace_syscall_info)
- [`regex_t`](#regex_t)
- [`sem_t`](#sem_t)
- [`seminfo`](#seminfo)
- [`tcp_info`](#tcp_info)
- [`termios`](#termios)
- [`timespec`](#timespec)
- [`utmpx`](#utmpx)

**Unions**

- [`__c_anonymous_ptrace_syscall_info_data`](#__c_anonymous_ptrace_syscall_info_data)

**Functions**

- [`adjtimex`](#adjtimex)
- [`asctime_r`](#asctime_r)
- [`backtrace`](#backtrace)
- [`backtrace_symbols`](#backtrace_symbols)
- [`backtrace_symbols_fd`](#backtrace_symbols_fd)
- [`clock_adjtime`](#clock_adjtime)
- [`close_range`](#close_range)
- [`ctermid`](#ctermid)
- [`ctime_r`](#ctime_r)
- [`dirname`](#dirname)
- [`dladdr1`](#dladdr1)
- [`dlinfo`](#dlinfo)
- [`dlmopen`](#dlmopen)
- [`dlvsym`](#dlvsym)
- [`eaccess`](#eaccess)
- [`endutxent`](#endutxent)
- [`epoll_pwait2`](#epoll_pwait2)
- [`euidaccess`](#euidaccess)
- [`execveat`](#execveat)
- [`explicit_bzero`](#explicit_bzero)
- [`fanotify_mark`](#fanotify_mark)
- [`fgetgrent_r`](#fgetgrent_r)
- [`fgetpwent_r`](#fgetpwent_r)
- [`fgetspent_r`](#fgetspent_r)
- [`getauxval`](#getauxval)
- [`getentropy`](#getentropy)
- [`getgrent_r`](#getgrent_r)
- [`getmntent_r`](#getmntent_r)
- [`getpriority`](#getpriority)
- [`getpt`](#getpt)
- [`getpwent_r`](#getpwent_r)
- [`getrandom`](#getrandom)
- [`getrlimit`](#getrlimit)
- [`getrlimit64`](#getrlimit64)
- [`getspent_r`](#getspent_r)
- [`gettimeofday`](#gettimeofday)
- [`getutxent`](#getutxent)
- [`getutxid`](#getutxid)
- [`getutxline`](#getutxline)
- [`glob64`](#glob64)
- [`globfree64`](#globfree64)
- [`gnu_basename`](#gnu_basename) - GNU version of `basename(3)`, defined in `string.h`.
- [`gnu_get_libc_release`](#gnu_get_libc_release)
- [`gnu_get_libc_version`](#gnu_get_libc_version)
- [`mallinfo`](#mallinfo)
- [`mallinfo2`](#mallinfo2)
- [`malloc_info`](#malloc_info)
- [`malloc_stats`](#malloc_stats)
- [`malloc_trim`](#malloc_trim)
- [`malloc_usable_size`](#malloc_usable_size)
- [`mallopt`](#mallopt)
- [`memfd_create`](#memfd_create)
- [`mempcpy`](#mempcpy)
- [`mlock2`](#mlock2)
- [`mq_notify`](#mq_notify)
- [`ntp_adjtime`](#ntp_adjtime)
- [`ntp_gettime`](#ntp_gettime)
- [`posix_basename`](#posix_basename) - POSIX version of `basename(3)`, defined in `libgen.h`.
- [`posix_spawn_file_actions_addchdir_np`](#posix_spawn_file_actions_addchdir_np)
- [`posix_spawn_file_actions_addclosefrom_np`](#posix_spawn_file_actions_addclosefrom_np)
- [`posix_spawn_file_actions_addfchdir_np`](#posix_spawn_file_actions_addfchdir_np)
- [`posix_spawn_file_actions_addtcsetpgrp_np`](#posix_spawn_file_actions_addtcsetpgrp_np)
- [`preadv2`](#preadv2)
- [`preadv64v2`](#preadv64v2)
- [`prlimit`](#prlimit)
- [`prlimit64`](#prlimit64)
- [`pthread_attr_getaffinity_np`](#pthread_attr_getaffinity_np)
- [`pthread_attr_setaffinity_np`](#pthread_attr_setaffinity_np)
- [`pthread_rwlockattr_getkind_np`](#pthread_rwlockattr_getkind_np)
- [`pthread_rwlockattr_setkind_np`](#pthread_rwlockattr_setkind_np)
- [`pthread_sigqueue`](#pthread_sigqueue)
- [`pthread_timedjoin_np`](#pthread_timedjoin_np)
- [`pthread_tryjoin_np`](#pthread_tryjoin_np)
- [`ptrace`](#ptrace)
- [`putgrent`](#putgrent)
- [`putpwent`](#putpwent)
- [`pututxline`](#pututxline)
- [`pwritev2`](#pwritev2)
- [`pwritev64v2`](#pwritev64v2)
- [`qsort_r`](#qsort_r)
- [`reallocarray`](#reallocarray)
- [`recvmmsg`](#recvmmsg)
- [`renameat2`](#renameat2)
- [`sendmmsg`](#sendmmsg)
- [`sethostid`](#sethostid)
- [`setpriority`](#setpriority)
- [`setrlimit`](#setrlimit)
- [`setrlimit64`](#setrlimit64)
- [`setutxent`](#setutxent)
- [`sgetspent_r`](#sgetspent_r)
- [`tgkill`](#tgkill)
- [`utmpname`](#utmpname)
- [`utmpxname`](#utmpxname)

**Constants**

- [`ACCOUNTING`](#accounting)
- [`ADJ_ESTERROR`](#adj_esterror)
- [`ADJ_FREQUENCY`](#adj_frequency)
- [`ADJ_MAXERROR`](#adj_maxerror)
- [`ADJ_MICRO`](#adj_micro)
- [`ADJ_NANO`](#adj_nano)
- [`ADJ_OFFSET`](#adj_offset)
- [`ADJ_OFFSET_SINGLESHOT`](#adj_offset_singleshot)
- [`ADJ_OFFSET_SS_READ`](#adj_offset_ss_read)
- [`ADJ_SETOFFSET`](#adj_setoffset)
- [`ADJ_STATUS`](#adj_status)
- [`ADJ_TAI`](#adj_tai)
- [`ADJ_TICK`](#adj_tick)
- [`ADJ_TIMECONST`](#adj_timeconst)
- [`AF_IB`](#af_ib)
- [`AF_MPLS`](#af_mpls)
- [`AF_NFC`](#af_nfc)
- [`AF_VSOCK`](#af_vsock)
- [`AF_XDP`](#af_xdp)
- [`BINDERFS_SUPER_MAGIC`](#binderfs_super_magic)
- [`BOOT_TIME`](#boot_time)
- [`BUFSIZ`](#bufsiz)
- [`CLONE_CLEAR_SIGHAND`](#clone_clear_sighand)
- [`CLONE_INTO_CGROUP`](#clone_into_cgroup)
- [`CLONE_NEWTIME`](#clone_newtime)
- [`CPU_SETSIZE`](#cpu_setsize)
- [`DEAD_PROCESS`](#dead_process)
- [`ELFOSABI_ARM_AEABI`](#elfosabi_arm_aeabi)
- [`EMPTY`](#empty)
- [`ENOTSUP`](#enotsup)
- [`FDPIC_FUNCPTRS`](#fdpic_funcptrs)
- [`FILENAME_MAX`](#filename_max)
- [`FOPEN_MAX`](#fopen_max)
- [`GENL_ID_PMCRAID`](#genl_id_pmcraid)
- [`GENL_ID_VFS_DQUOT`](#genl_id_vfs_dquot)
- [`GENL_UNS_ADMIN_PERM`](#genl_uns_admin_perm)
- [`GLOB_ALTDIRFUNC`](#glob_altdirfunc)
- [`GLOB_BRACE`](#glob_brace)
- [`GLOB_NOMAGIC`](#glob_nomagic)
- [`GLOB_ONLYDIR`](#glob_onlydir)
- [`GLOB_PERIOD`](#glob_period)
- [`GLOB_TILDE`](#glob_tilde)
- [`GLOB_TILDE_CHECK`](#glob_tilde_check)
- [`HUGETLB_FLAG_ENCODE_16GB`](#hugetlb_flag_encode_16gb)
- [`HUGETLB_FLAG_ENCODE_16MB`](#hugetlb_flag_encode_16mb)
- [`HUGETLB_FLAG_ENCODE_1GB`](#hugetlb_flag_encode_1gb)
- [`HUGETLB_FLAG_ENCODE_1MB`](#hugetlb_flag_encode_1mb)
- [`HUGETLB_FLAG_ENCODE_256MB`](#hugetlb_flag_encode_256mb)
- [`HUGETLB_FLAG_ENCODE_2GB`](#hugetlb_flag_encode_2gb)
- [`HUGETLB_FLAG_ENCODE_2MB`](#hugetlb_flag_encode_2mb)
- [`HUGETLB_FLAG_ENCODE_32MB`](#hugetlb_flag_encode_32mb)
- [`HUGETLB_FLAG_ENCODE_512KB`](#hugetlb_flag_encode_512kb)
- [`HUGETLB_FLAG_ENCODE_512MB`](#hugetlb_flag_encode_512mb)
- [`HUGETLB_FLAG_ENCODE_64KB`](#hugetlb_flag_encode_64kb)
- [`HUGETLB_FLAG_ENCODE_8MB`](#hugetlb_flag_encode_8mb)
- [`HUGETLB_FLAG_ENCODE_MASK`](#hugetlb_flag_encode_mask)
- [`HUGETLB_FLAG_ENCODE_SHIFT`](#hugetlb_flag_encode_shift)
- [`INIT_PROCESS`](#init_process)
- [`LC_ADDRESS`](#lc_address)
- [`LC_ADDRESS_MASK`](#lc_address_mask)
- [`LC_ALL_MASK`](#lc_all_mask)
- [`LC_IDENTIFICATION`](#lc_identification)
- [`LC_IDENTIFICATION_MASK`](#lc_identification_mask)
- [`LC_MEASUREMENT`](#lc_measurement)
- [`LC_MEASUREMENT_MASK`](#lc_measurement_mask)
- [`LC_NAME`](#lc_name)
- [`LC_NAME_MASK`](#lc_name_mask)
- [`LC_PAPER`](#lc_paper)
- [`LC_PAPER_MASK`](#lc_paper_mask)
- [`LC_TELEPHONE`](#lc_telephone)
- [`LC_TELEPHONE_MASK`](#lc_telephone_mask)
- [`LM_ID_BASE`](#lm_id_base)
- [`LM_ID_NEWLM`](#lm_id_newlm)
- [`LOGIN_PROCESS`](#login_process)
- [`MADV_COLLAPSE`](#madv_collapse)
- [`MAP_HUGE_16GB`](#map_huge_16gb)
- [`MAP_HUGE_16MB`](#map_huge_16mb)
- [`MAP_HUGE_1GB`](#map_huge_1gb)
- [`MAP_HUGE_1MB`](#map_huge_1mb)
- [`MAP_HUGE_256MB`](#map_huge_256mb)
- [`MAP_HUGE_2GB`](#map_huge_2gb)
- [`MAP_HUGE_2MB`](#map_huge_2mb)
- [`MAP_HUGE_32MB`](#map_huge_32mb)
- [`MAP_HUGE_512KB`](#map_huge_512kb)
- [`MAP_HUGE_512MB`](#map_huge_512mb)
- [`MAP_HUGE_64KB`](#map_huge_64kb)
- [`MAP_HUGE_8MB`](#map_huge_8mb)
- [`MAP_HUGE_MASK`](#map_huge_mask)
- [`MAP_HUGE_SHIFT`](#map_huge_shift)
- [`MAXTC`](#maxtc)
- [`MOD_CLKA`](#mod_clka)
- [`MOD_CLKB`](#mod_clkb)
- [`MOD_ESTERROR`](#mod_esterror)
- [`MOD_FREQUENCY`](#mod_frequency)
- [`MOD_MAXERROR`](#mod_maxerror)
- [`MOD_MICRO`](#mod_micro)
- [`MOD_NANO`](#mod_nano)
- [`MOD_OFFSET`](#mod_offset)
- [`MOD_STATUS`](#mod_status)
- [`MOD_TAI`](#mod_tai)
- [`MOD_TIMECONST`](#mod_timeconst)
- [`MOVE_MOUNT_BENEATH`](#move_mount_beneath)
- [`MOVE_MOUNT_F_AUTOMOUNTS`](#move_mount_f_automounts)
- [`MOVE_MOUNT_F_EMPTY_PATH`](#move_mount_f_empty_path)
- [`MOVE_MOUNT_F_SYMLINKS`](#move_mount_f_symlinks)
- [`MOVE_MOUNT_SET_GROUP`](#move_mount_set_group)
- [`MOVE_MOUNT_T_AUTOMOUNTS`](#move_mount_t_automounts)
- [`MOVE_MOUNT_T_EMPTY_PATH`](#move_mount_t_empty_path)
- [`MOVE_MOUNT_T_SYMLINKS`](#move_mount_t_symlinks)
- [`MSG_TRYHARD`](#msg_tryhard)
- [`MS_RMT_MASK`](#ms_rmt_mask)
- [`M_ARENA_MAX`](#m_arena_max)
- [`M_ARENA_TEST`](#m_arena_test)
- [`M_CHECK_ACTION`](#m_check_action)
- [`M_GRAIN`](#m_grain)
- [`M_KEEP`](#m_keep)
- [`M_MMAP_MAX`](#m_mmap_max)
- [`M_MMAP_THRESHOLD`](#m_mmap_threshold)
- [`M_MXFAST`](#m_mxfast)
- [`M_NLBLKS`](#m_nlblks)
- [`M_PERTURB`](#m_perturb)
- [`M_TOP_PAD`](#m_top_pad)
- [`M_TRIM_THRESHOLD`](#m_trim_threshold)
- [`NDA_LINK_NETNSID`](#nda_link_netnsid)
- [`NDA_MASTER`](#nda_master)
- [`NDA_SRC_VNI`](#nda_src_vni)
- [`NEW_TIME`](#new_time)
- [`NI_MAXHOST`](#ni_maxhost)
- [`NTF_EXT_LEARNED`](#ntf_ext_learned)
- [`NTF_OFFLOADED`](#ntf_offloaded)
- [`NTP_API`](#ntp_api)
- [`OLD_TIME`](#old_time)
- [`O_ACCMODE`](#o_accmode)
- [`PF_IB`](#pf_ib)
- [`PF_MPLS`](#pf_mpls)
- [`PF_NFC`](#pf_nfc)
- [`PF_VSOCK`](#pf_vsock)
- [`PF_XDP`](#pf_xdp)
- [`PRIO_PGRP`](#prio_pgrp)
- [`PRIO_PROCESS`](#prio_process)
- [`PRIO_USER`](#prio_user)
- [`PTHREAD_MUTEX_ADAPTIVE_NP`](#pthread_mutex_adaptive_np)
- [`PTHREAD_STACK_MIN`](#pthread_stack_min)
- [`PTRACE_ATTACH`](#ptrace_attach)
- [`PTRACE_CONT`](#ptrace_cont)
- [`PTRACE_GETEVENTMSG`](#ptrace_geteventmsg)
- [`PTRACE_GETREGSET`](#ptrace_getregset)
- [`PTRACE_GETSIGINFO`](#ptrace_getsiginfo)
- [`PTRACE_GETSIGMASK`](#ptrace_getsigmask)
- [`PTRACE_GET_SYSCALL_INFO`](#ptrace_get_syscall_info)
- [`PTRACE_GET_SYSCALL_USER_DISPATCH_CONFIG`](#ptrace_get_syscall_user_dispatch_config)
- [`PTRACE_INTERRUPT`](#ptrace_interrupt)
- [`PTRACE_KILL`](#ptrace_kill)
- [`PTRACE_LISTEN`](#ptrace_listen)
- [`PTRACE_PEEKDATA`](#ptrace_peekdata)
- [`PTRACE_PEEKSIGINFO`](#ptrace_peeksiginfo)
- [`PTRACE_PEEKTEXT`](#ptrace_peektext)
- [`PTRACE_PEEKUSER`](#ptrace_peekuser)
- [`PTRACE_POKEDATA`](#ptrace_pokedata)
- [`PTRACE_POKETEXT`](#ptrace_poketext)
- [`PTRACE_POKEUSER`](#ptrace_pokeuser)
- [`PTRACE_SEIZE`](#ptrace_seize)
- [`PTRACE_SETOPTIONS`](#ptrace_setoptions)
- [`PTRACE_SETREGSET`](#ptrace_setregset)
- [`PTRACE_SETSIGINFO`](#ptrace_setsiginfo)
- [`PTRACE_SETSIGMASK`](#ptrace_setsigmask)
- [`PTRACE_SET_SYSCALL_INFO`](#ptrace_set_syscall_info)
- [`PTRACE_SET_SYSCALL_USER_DISPATCH_CONFIG`](#ptrace_set_syscall_user_dispatch_config)
- [`PTRACE_SINGLESTEP`](#ptrace_singlestep)
- [`PTRACE_SYSCALL`](#ptrace_syscall)
- [`PTRACE_SYSCALL_INFO_ENTRY`](#ptrace_syscall_info_entry)
- [`PTRACE_SYSCALL_INFO_EXIT`](#ptrace_syscall_info_exit)
- [`PTRACE_SYSCALL_INFO_NONE`](#ptrace_syscall_info_none)
- [`PTRACE_SYSCALL_INFO_SECCOMP`](#ptrace_syscall_info_seccomp)
- [`PTRACE_TRACEME`](#ptrace_traceme)
- [`REG_EEND`](#reg_eend)
- [`REG_ERPAREN`](#reg_erparen)
- [`REG_ESIZE`](#reg_esize)
- [`REG_STARTEND`](#reg_startend)
- [`RTA_ENCAP`](#rta_encap)
- [`RTA_ENCAP_TYPE`](#rta_encap_type)
- [`RTA_EXPIRES`](#rta_expires)
- [`RTA_NEWDST`](#rta_newdst)
- [`RTA_PAD`](#rta_pad)
- [`RTA_PREF`](#rta_pref)
- [`RTA_TTL_PROPAGATE`](#rta_ttl_propagate)
- [`RTA_UID`](#rta_uid)
- [`RTA_VIA`](#rta_via)
- [`RTLD_DI_CONFIGADDR`](#rtld_di_configaddr)
- [`RTLD_DI_LINKMAP`](#rtld_di_linkmap)
- [`RTLD_DI_LMID`](#rtld_di_lmid)
- [`RTLD_DI_ORIGIN`](#rtld_di_origin)
- [`RTLD_DI_PROFILENAME`](#rtld_di_profilename)
- [`RTLD_DI_PROFILEOUT`](#rtld_di_profileout)
- [`RTLD_DI_SERINFO`](#rtld_di_serinfo)
- [`RTLD_DI_SERINFOSIZE`](#rtld_di_serinfosize)
- [`RTLD_DI_TLS_DATA`](#rtld_di_tls_data)
- [`RTLD_DI_TLS_MODID`](#rtld_di_tls_modid)
- [`RTM_DELNETCONF`](#rtm_delnetconf)
- [`RTM_F_FIB_MATCH`](#rtm_f_fib_match)
- [`RTM_F_LOOKUP_TABLE`](#rtm_f_lookup_table)
- [`RTM_GETSTATS`](#rtm_getstats)
- [`RTM_NEWCACHEREPORT`](#rtm_newcachereport)
- [`RTM_NEWSTATS`](#rtm_newstats)
- [`RUN_LVL`](#run_lvl)
- [`SIGEV_THREAD_ID`](#sigev_thread_id)
- [`SOCK_DCCP`](#sock_dccp)
- [`SOCK_NONBLOCK`](#sock_nonblock)
- [`SOCK_PACKET`](#sock_packet)
- [`SOCK_SEQPACKET`](#sock_seqpacket)
- [`SOL_CAIF`](#sol_caif)
- [`SOL_IUCV`](#sol_iucv)
- [`SOL_NFC`](#sol_nfc)
- [`SOL_PNPIPE`](#sol_pnpipe)
- [`SOL_PPPOL2TP`](#sol_pppol2tp)
- [`SOL_RDS`](#sol_rds)
- [`SOL_RXRPC`](#sol_rxrpc)
- [`SOMAXCONN`](#somaxconn)
- [`STA_CLK`](#sta_clk)
- [`STA_CLOCKERR`](#sta_clockerr)
- [`STA_DEL`](#sta_del)
- [`STA_FLL`](#sta_fll)
- [`STA_FREQHOLD`](#sta_freqhold)
- [`STA_INS`](#sta_ins)
- [`STA_MODE`](#sta_mode)
- [`STA_NANO`](#sta_nano)
- [`STA_PLL`](#sta_pll)
- [`STA_PPSERROR`](#sta_ppserror)
- [`STA_PPSFREQ`](#sta_ppsfreq)
- [`STA_PPSJITTER`](#sta_ppsjitter)
- [`STA_PPSSIGNAL`](#sta_ppssignal)
- [`STA_PPSTIME`](#sta_ppstime)
- [`STA_PPSWANDER`](#sta_ppswander)
- [`STA_RONLY`](#sta_ronly)
- [`STA_UNSYNC`](#sta_unsync)
- [`ST_RELATIME`](#st_relatime)
- [`TCA_CHAIN`](#tca_chain)
- [`TCA_DUMP_INVISIBLE`](#tca_dump_invisible)
- [`TCA_HW_OFFLOAD`](#tca_hw_offload)
- [`TCA_PAD`](#tca_pad)
- [`TIME_BAD`](#time_bad)
- [`TIME_DEL`](#time_del)
- [`TIME_ERROR`](#time_error)
- [`TIME_INS`](#time_ins)
- [`TIME_OK`](#time_ok)
- [`TIME_OOP`](#time_oop)
- [`TIME_WAIT`](#time_wait)
- [`TMP_MAX`](#tmp_max)
- [`UNAME26`](#uname26)
- [`USER_PROCESS`](#user_process)
- [`XFS_SUPER_MAGIC`](#xfs_super_magic)
- [`_CS_GNU_LIBC_VERSION`](#_cs_gnu_libc_version)
- [`_CS_GNU_LIBPTHREAD_VERSION`](#_cs_gnu_libpthread_version)
- [`_CS_V6_ENV`](#_cs_v6_env)
- [`_CS_V7_ENV`](#_cs_v7_env)
- [`_SC_2_C_VERSION`](#_sc_2_c_version)
- [`_SC_BASE`](#_sc_base)
- [`_SC_CHARCLASS_NAME_MAX`](#_sc_charclass_name_max)
- [`_SC_CHAR_BIT`](#_sc_char_bit)
- [`_SC_CHAR_MAX`](#_sc_char_max)
- [`_SC_CHAR_MIN`](#_sc_char_min)
- [`_SC_C_LANG_SUPPORT`](#_sc_c_lang_support)
- [`_SC_C_LANG_SUPPORT_R`](#_sc_c_lang_support_r)
- [`_SC_DEVICE_IO`](#_sc_device_io)
- [`_SC_DEVICE_SPECIFIC`](#_sc_device_specific)
- [`_SC_DEVICE_SPECIFIC_R`](#_sc_device_specific_r)
- [`_SC_EQUIV_CLASS_MAX`](#_sc_equiv_class_max)
- [`_SC_FD_MGMT`](#_sc_fd_mgmt)
- [`_SC_FIFO`](#_sc_fifo)
- [`_SC_FILE_ATTRIBUTES`](#_sc_file_attributes)
- [`_SC_FILE_LOCKING`](#_sc_file_locking)
- [`_SC_FILE_SYSTEM`](#_sc_file_system)
- [`_SC_INT_MAX`](#_sc_int_max)
- [`_SC_INT_MIN`](#_sc_int_min)
- [`_SC_LEVEL1_DCACHE_ASSOC`](#_sc_level1_dcache_assoc)
- [`_SC_LEVEL1_DCACHE_LINESIZE`](#_sc_level1_dcache_linesize)
- [`_SC_LEVEL1_DCACHE_SIZE`](#_sc_level1_dcache_size)
- [`_SC_LEVEL1_ICACHE_ASSOC`](#_sc_level1_icache_assoc)
- [`_SC_LEVEL1_ICACHE_LINESIZE`](#_sc_level1_icache_linesize)
- [`_SC_LEVEL1_ICACHE_SIZE`](#_sc_level1_icache_size)
- [`_SC_LEVEL2_CACHE_ASSOC`](#_sc_level2_cache_assoc)
- [`_SC_LEVEL2_CACHE_LINESIZE`](#_sc_level2_cache_linesize)
- [`_SC_LEVEL2_CACHE_SIZE`](#_sc_level2_cache_size)
- [`_SC_LEVEL3_CACHE_ASSOC`](#_sc_level3_cache_assoc)
- [`_SC_LEVEL3_CACHE_LINESIZE`](#_sc_level3_cache_linesize)
- [`_SC_LEVEL3_CACHE_SIZE`](#_sc_level3_cache_size)
- [`_SC_LEVEL4_CACHE_ASSOC`](#_sc_level4_cache_assoc)
- [`_SC_LEVEL4_CACHE_LINESIZE`](#_sc_level4_cache_linesize)
- [`_SC_LEVEL4_CACHE_SIZE`](#_sc_level4_cache_size)
- [`_SC_LONG_BIT`](#_sc_long_bit)
- [`_SC_MB_LEN_MAX`](#_sc_mb_len_max)
- [`_SC_MULTI_PROCESS`](#_sc_multi_process)
- [`_SC_NETWORKING`](#_sc_networking)
- [`_SC_NL_ARGMAX`](#_sc_nl_argmax)
- [`_SC_NL_LANGMAX`](#_sc_nl_langmax)
- [`_SC_NL_MSGMAX`](#_sc_nl_msgmax)
- [`_SC_NL_NMAX`](#_sc_nl_nmax)
- [`_SC_NL_SETMAX`](#_sc_nl_setmax)
- [`_SC_NL_TEXTMAX`](#_sc_nl_textmax)
- [`_SC_PII`](#_sc_pii)
- [`_SC_PII_INTERNET`](#_sc_pii_internet)
- [`_SC_PII_INTERNET_DGRAM`](#_sc_pii_internet_dgram)
- [`_SC_PII_INTERNET_STREAM`](#_sc_pii_internet_stream)
- [`_SC_PII_OSI`](#_sc_pii_osi)
- [`_SC_PII_OSI_CLTS`](#_sc_pii_osi_clts)
- [`_SC_PII_OSI_COTS`](#_sc_pii_osi_cots)
- [`_SC_PII_OSI_M`](#_sc_pii_osi_m)
- [`_SC_PII_SOCKET`](#_sc_pii_socket)
- [`_SC_PII_XTI`](#_sc_pii_xti)
- [`_SC_PIPE`](#_sc_pipe)
- [`_SC_POLL`](#_sc_poll)
- [`_SC_REGEX_VERSION`](#_sc_regex_version)
- [`_SC_SCHAR_MAX`](#_sc_schar_max)
- [`_SC_SCHAR_MIN`](#_sc_schar_min)
- [`_SC_SELECT`](#_sc_select)
- [`_SC_SHRT_MAX`](#_sc_shrt_max)
- [`_SC_SHRT_MIN`](#_sc_shrt_min)
- [`_SC_SIGNALS`](#_sc_signals)
- [`_SC_SINGLE_PROCESS`](#_sc_single_process)
- [`_SC_SSIZE_MAX`](#_sc_ssize_max)
- [`_SC_SYSTEM_DATABASE`](#_sc_system_database)
- [`_SC_SYSTEM_DATABASE_R`](#_sc_system_database_r)
- [`_SC_T_IOV_MAX`](#_sc_t_iov_max)
- [`_SC_UCHAR_MAX`](#_sc_uchar_max)
- [`_SC_UINT_MAX`](#_sc_uint_max)
- [`_SC_ULONG_MAX`](#_sc_ulong_max)
- [`_SC_USER_GROUPS`](#_sc_user_groups)
- [`_SC_USER_GROUPS_R`](#_sc_user_groups_r)
- [`_SC_USHRT_MAX`](#_sc_ushrt_max)
- [`_SC_WORD_BIT`](#_sc_word_bit)
- [`__UT_HOSTSIZE`](#__ut_hostsize)
- [`__UT_LINESIZE`](#__ut_linesize)
- [`__UT_NAMESIZE`](#__ut_namesize)

**Type Aliases**

- [`Lmid_t`](#lmid_t)
- [`__kernel_rwf_t`](#__kernel_rwf_t)
- [`__priority_which_t`](#__priority_which_t)
- [`__rlimit_resource_t`](#__rlimit_resource_t)
- [`pthread_t`](#pthread_t)
- [`regoff_t`](#regoff_t)

---

## libc::unix::linux_like::linux::gnu::ACCOUNTING

*Constant*: `c_short`



## libc::unix::linux_like::linux::gnu::ADJ_ESTERROR

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::ADJ_FREQUENCY

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::ADJ_MAXERROR

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::ADJ_MICRO

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::ADJ_NANO

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::ADJ_OFFSET

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::ADJ_OFFSET_SINGLESHOT

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::ADJ_OFFSET_SS_READ

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::ADJ_SETOFFSET

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::ADJ_STATUS

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::ADJ_TAI

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::ADJ_TICK

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::ADJ_TIMECONST

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::AF_IB

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::AF_MPLS

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::AF_NFC

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::AF_VSOCK

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::AF_XDP

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::BINDERFS_SUPER_MAGIC

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::BOOT_TIME

*Constant*: `c_short`



## libc::unix::linux_like::linux::gnu::BUFSIZ

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::CLONE_CLEAR_SIGHAND

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::CLONE_INTO_CGROUP

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::CLONE_NEWTIME

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::CPU_SETSIZE

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::DEAD_PROCESS

*Constant*: `c_short`



## libc::unix::linux_like::linux::gnu::ELFOSABI_ARM_AEABI

*Constant*: `u8`



## libc::unix::linux_like::linux::gnu::EMPTY

*Constant*: `c_short`



## libc::unix::linux_like::linux::gnu::ENOTSUP

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::Elf32_Chdr

*Struct*

**Fields:**
- `ch_type: crate::Elf32_Word`
- `ch_size: crate::Elf32_Word`
- `ch_addralign: crate::Elf32_Word`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Elf32_Chdr`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::gnu::Elf64_Chdr

*Struct*

**Fields:**
- `ch_type: crate::Elf64_Word`
- `ch_reserved: crate::Elf64_Word`
- `ch_size: crate::Elf64_Xword`
- `ch_addralign: crate::Elf64_Xword`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Elf64_Chdr`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::gnu::FDPIC_FUNCPTRS

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::FILENAME_MAX

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::FOPEN_MAX

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::GENL_ID_PMCRAID

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::GENL_ID_VFS_DQUOT

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::GENL_UNS_ADMIN_PERM

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::GLOB_ALTDIRFUNC

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::GLOB_BRACE

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::GLOB_NOMAGIC

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::GLOB_ONLYDIR

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::GLOB_PERIOD

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::GLOB_TILDE

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::GLOB_TILDE_CHECK

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::HUGETLB_FLAG_ENCODE_16GB

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::HUGETLB_FLAG_ENCODE_16MB

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::HUGETLB_FLAG_ENCODE_1GB

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::HUGETLB_FLAG_ENCODE_1MB

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::HUGETLB_FLAG_ENCODE_256MB

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::HUGETLB_FLAG_ENCODE_2GB

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::HUGETLB_FLAG_ENCODE_2MB

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::HUGETLB_FLAG_ENCODE_32MB

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::HUGETLB_FLAG_ENCODE_512KB

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::HUGETLB_FLAG_ENCODE_512MB

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::HUGETLB_FLAG_ENCODE_64KB

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::HUGETLB_FLAG_ENCODE_8MB

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::HUGETLB_FLAG_ENCODE_MASK

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::HUGETLB_FLAG_ENCODE_SHIFT

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::INIT_PROCESS

*Constant*: `c_short`



## libc::unix::linux_like::linux::gnu::LC_ADDRESS

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::LC_ADDRESS_MASK

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::LC_ALL_MASK

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::LC_IDENTIFICATION

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::LC_IDENTIFICATION_MASK

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::LC_MEASUREMENT

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::LC_MEASUREMENT_MASK

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::LC_NAME

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::LC_NAME_MASK

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::LC_PAPER

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::LC_PAPER_MASK

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::LC_TELEPHONE

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::LC_TELEPHONE_MASK

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::LM_ID_BASE

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::LM_ID_NEWLM

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::LOGIN_PROCESS

*Constant*: `c_short`



## libc::unix::linux_like::linux::gnu::Lmid_t

*Type Alias*: `c_long`



## libc::unix::linux_like::linux::gnu::MADV_COLLAPSE

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::MAP_HUGE_16GB

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::MAP_HUGE_16MB

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::MAP_HUGE_1GB

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::MAP_HUGE_1MB

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::MAP_HUGE_256MB

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::MAP_HUGE_2GB

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::MAP_HUGE_2MB

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::MAP_HUGE_32MB

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::MAP_HUGE_512KB

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::MAP_HUGE_512MB

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::MAP_HUGE_64KB

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::MAP_HUGE_8MB

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::MAP_HUGE_MASK

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::MAP_HUGE_SHIFT

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::MAXTC

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::MOD_CLKA

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::MOD_CLKB

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::MOD_ESTERROR

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::MOD_FREQUENCY

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::MOD_MAXERROR

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::MOD_MICRO

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::MOD_NANO

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::MOD_OFFSET

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::MOD_STATUS

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::MOD_TAI

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::MOD_TIMECONST

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::MOVE_MOUNT_BENEATH

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::MOVE_MOUNT_F_AUTOMOUNTS

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::MOVE_MOUNT_F_EMPTY_PATH

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::MOVE_MOUNT_F_SYMLINKS

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::MOVE_MOUNT_SET_GROUP

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::MOVE_MOUNT_T_AUTOMOUNTS

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::MOVE_MOUNT_T_EMPTY_PATH

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::MOVE_MOUNT_T_SYMLINKS

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::MSG_TRYHARD

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::MS_RMT_MASK

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::gnu::M_ARENA_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::M_ARENA_TEST

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::M_CHECK_ACTION

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::M_GRAIN

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::M_KEEP

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::M_MMAP_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::M_MMAP_THRESHOLD

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::M_MXFAST

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::M_NLBLKS

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::M_PERTURB

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::M_TOP_PAD

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::M_TRIM_THRESHOLD

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::NDA_LINK_NETNSID

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::gnu::NDA_MASTER

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::gnu::NDA_SRC_VNI

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::gnu::NEW_TIME

*Constant*: `c_short`



## libc::unix::linux_like::linux::gnu::NI_MAXHOST

*Constant*: `crate::socklen_t`



## libc::unix::linux_like::linux::gnu::NTF_EXT_LEARNED

*Constant*: `u8`



## libc::unix::linux_like::linux::gnu::NTF_OFFLOADED

*Constant*: `u8`



## libc::unix::linux_like::linux::gnu::NTP_API

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::OLD_TIME

*Constant*: `c_short`



## libc::unix::linux_like::linux::gnu::O_ACCMODE

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::PF_IB

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::PF_MPLS

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::PF_NFC

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::PF_VSOCK

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::PF_XDP

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::PRIO_PGRP

*Constant*: `crate::__priority_which_t`



## libc::unix::linux_like::linux::gnu::PRIO_PROCESS

*Constant*: `crate::__priority_which_t`



## libc::unix::linux_like::linux::gnu::PRIO_USER

*Constant*: `crate::__priority_which_t`



## libc::unix::linux_like::linux::gnu::PTHREAD_MUTEX_ADAPTIVE_NP

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::PTHREAD_STACK_MIN

*Constant*: `size_t`



## libc::unix::linux_like::linux::gnu::PTRACE_ATTACH

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::PTRACE_CONT

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::PTRACE_GETEVENTMSG

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::PTRACE_GETREGSET

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::PTRACE_GETSIGINFO

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::PTRACE_GETSIGMASK

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::PTRACE_GET_SYSCALL_INFO

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::PTRACE_GET_SYSCALL_USER_DISPATCH_CONFIG

*Constant*: `crate::__u8`



## libc::unix::linux_like::linux::gnu::PTRACE_INTERRUPT

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::PTRACE_KILL

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::PTRACE_LISTEN

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::PTRACE_PEEKDATA

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::PTRACE_PEEKSIGINFO

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::PTRACE_PEEKTEXT

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::PTRACE_PEEKUSER

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::PTRACE_POKEDATA

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::PTRACE_POKETEXT

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::PTRACE_POKEUSER

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::PTRACE_SEIZE

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::PTRACE_SETOPTIONS

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::PTRACE_SETREGSET

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::PTRACE_SETSIGINFO

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::PTRACE_SETSIGMASK

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::PTRACE_SET_SYSCALL_INFO

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::PTRACE_SET_SYSCALL_USER_DISPATCH_CONFIG

*Constant*: `crate::__u8`



## libc::unix::linux_like::linux::gnu::PTRACE_SINGLESTEP

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::PTRACE_SYSCALL

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::PTRACE_SYSCALL_INFO_ENTRY

*Constant*: `crate::__u8`



## libc::unix::linux_like::linux::gnu::PTRACE_SYSCALL_INFO_EXIT

*Constant*: `crate::__u8`



## libc::unix::linux_like::linux::gnu::PTRACE_SYSCALL_INFO_NONE

*Constant*: `crate::__u8`



## libc::unix::linux_like::linux::gnu::PTRACE_SYSCALL_INFO_SECCOMP

*Constant*: `crate::__u8`



## libc::unix::linux_like::linux::gnu::PTRACE_TRACEME

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::REG_EEND

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::REG_ERPAREN

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::REG_ESIZE

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::REG_STARTEND

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::RTA_ENCAP

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::gnu::RTA_ENCAP_TYPE

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::gnu::RTA_EXPIRES

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::gnu::RTA_NEWDST

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::gnu::RTA_PAD

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::gnu::RTA_PREF

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::gnu::RTA_TTL_PROPAGATE

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::gnu::RTA_UID

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::gnu::RTA_VIA

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::gnu::RTLD_DI_CONFIGADDR

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::RTLD_DI_LINKMAP

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::RTLD_DI_LMID

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::RTLD_DI_ORIGIN

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::RTLD_DI_PROFILENAME

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::RTLD_DI_PROFILEOUT

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::RTLD_DI_SERINFO

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::RTLD_DI_SERINFOSIZE

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::RTLD_DI_TLS_DATA

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::RTLD_DI_TLS_MODID

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::RTM_DELNETCONF

*Constant*: `u16`



## libc::unix::linux_like::linux::gnu::RTM_F_FIB_MATCH

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::RTM_F_LOOKUP_TABLE

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::RTM_GETSTATS

*Constant*: `u16`



## libc::unix::linux_like::linux::gnu::RTM_NEWCACHEREPORT

*Constant*: `u16`



## libc::unix::linux_like::linux::gnu::RTM_NEWSTATS

*Constant*: `u16`



## libc::unix::linux_like::linux::gnu::RUN_LVL

*Constant*: `c_short`



## libc::unix::linux_like::linux::gnu::SIGEV_THREAD_ID

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::SOCK_DCCP

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::SOCK_NONBLOCK

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::SOCK_PACKET

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::SOCK_SEQPACKET

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::SOL_CAIF

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::SOL_IUCV

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::SOL_NFC

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::SOL_PNPIPE

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::SOL_PPPOL2TP

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::SOL_RDS

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::SOL_RXRPC

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::SOMAXCONN

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::STA_CLK

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::STA_CLOCKERR

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::STA_DEL

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::STA_FLL

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::STA_FREQHOLD

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::STA_INS

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::STA_MODE

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::STA_NANO

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::STA_PLL

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::STA_PPSERROR

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::STA_PPSFREQ

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::STA_PPSJITTER

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::STA_PPSSIGNAL

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::STA_PPSTIME

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::STA_PPSWANDER

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::STA_RONLY

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::STA_UNSYNC

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::ST_RELATIME

*Constant*: `c_ulong`



## libc::unix::linux_like::linux::gnu::TCA_CHAIN

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::gnu::TCA_DUMP_INVISIBLE

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::gnu::TCA_HW_OFFLOAD

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::gnu::TCA_PAD

*Constant*: `c_ushort`



## libc::unix::linux_like::linux::gnu::TIME_BAD

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::TIME_DEL

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::TIME_ERROR

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::TIME_INS

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::TIME_OK

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::TIME_OOP

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::TIME_WAIT

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::TMP_MAX

*Constant*: `c_uint`



## libc::unix::linux_like::linux::gnu::UNAME26

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::USER_PROCESS

*Constant*: `c_short`



## libc::unix::linux_like::linux::gnu::XFS_SUPER_MAGIC

*Constant*: `c_long`



## libc::unix::linux_like::linux::gnu::_CS_GNU_LIBC_VERSION

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_CS_GNU_LIBPTHREAD_VERSION

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_CS_V6_ENV

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_CS_V7_ENV

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_2_C_VERSION

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_BASE

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_CHARCLASS_NAME_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_CHAR_BIT

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_CHAR_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_CHAR_MIN

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_C_LANG_SUPPORT

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_C_LANG_SUPPORT_R

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_DEVICE_IO

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_DEVICE_SPECIFIC

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_DEVICE_SPECIFIC_R

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_EQUIV_CLASS_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_FD_MGMT

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_FIFO

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_FILE_ATTRIBUTES

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_FILE_LOCKING

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_FILE_SYSTEM

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_INT_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_INT_MIN

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_LEVEL1_DCACHE_ASSOC

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_LEVEL1_DCACHE_LINESIZE

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_LEVEL1_DCACHE_SIZE

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_LEVEL1_ICACHE_ASSOC

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_LEVEL1_ICACHE_LINESIZE

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_LEVEL1_ICACHE_SIZE

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_LEVEL2_CACHE_ASSOC

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_LEVEL2_CACHE_LINESIZE

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_LEVEL2_CACHE_SIZE

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_LEVEL3_CACHE_ASSOC

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_LEVEL3_CACHE_LINESIZE

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_LEVEL3_CACHE_SIZE

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_LEVEL4_CACHE_ASSOC

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_LEVEL4_CACHE_LINESIZE

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_LEVEL4_CACHE_SIZE

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_LONG_BIT

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_MB_LEN_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_MULTI_PROCESS

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_NETWORKING

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_NL_ARGMAX

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_NL_LANGMAX

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_NL_MSGMAX

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_NL_NMAX

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_NL_SETMAX

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_NL_TEXTMAX

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_PII

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_PII_INTERNET

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_PII_INTERNET_DGRAM

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_PII_INTERNET_STREAM

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_PII_OSI

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_PII_OSI_CLTS

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_PII_OSI_COTS

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_PII_OSI_M

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_PII_SOCKET

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_PII_XTI

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_PIPE

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_POLL

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_REGEX_VERSION

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_SCHAR_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_SCHAR_MIN

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_SELECT

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_SHRT_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_SHRT_MIN

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_SIGNALS

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_SINGLE_PROCESS

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_SSIZE_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_SYSTEM_DATABASE

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_SYSTEM_DATABASE_R

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_T_IOV_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_UCHAR_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_UINT_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_ULONG_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_USER_GROUPS

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_USER_GROUPS_R

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_USHRT_MAX

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::_SC_WORD_BIT

*Constant*: `c_int`



## libc::unix::linux_like::linux::gnu::__UT_HOSTSIZE

*Constant*: `usize`



## libc::unix::linux_like::linux::gnu::__UT_LINESIZE

*Constant*: `usize`



## libc::unix::linux_like::linux::gnu::__UT_NAMESIZE

*Constant*: `usize`



## libc::unix::linux_like::linux::gnu::__c_anonymous_ptrace_syscall_info_data

*Union*

**Fields:**
- `entry: __c_anonymous_ptrace_syscall_info_entry`
- `exit: __c_anonymous_ptrace_syscall_info_exit`
- `seccomp: __c_anonymous_ptrace_syscall_info_seccomp`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> __c_anonymous_ptrace_syscall_info_data`
- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> ::core::fmt::Result`



## libc::unix::linux_like::linux::gnu::__c_anonymous_ptrace_syscall_info_entry

*Struct*

**Fields:**
- `nr: crate::__u64`
- `args: [crate::__u64; 6]`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> __c_anonymous_ptrace_syscall_info_entry`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::gnu::__c_anonymous_ptrace_syscall_info_exit

*Struct*

**Fields:**
- `sval: crate::__s64`
- `is_error: crate::__u8`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> __c_anonymous_ptrace_syscall_info_exit`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::gnu::__c_anonymous_ptrace_syscall_info_seccomp

*Struct*

**Fields:**
- `nr: crate::__u64`
- `args: [crate::__u64; 6]`
- `ret_data: crate::__u32`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> __c_anonymous_ptrace_syscall_info_seccomp`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::gnu::__exit_status

*Struct*

**Fields:**
- `e_termination: c_short`
- `e_exit: c_short`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> __exit_status`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::gnu::__kernel_rwf_t

*Type Alias*: `c_int`



## libc::unix::linux_like::linux::gnu::__priority_which_t

*Type Alias*: `c_uint`



## libc::unix::linux_like::linux::gnu::__rlimit_resource_t

*Type Alias*: `c_uint`



## libc::unix::linux_like::linux::gnu::__timeval

*Struct*

**Fields:**
- `tv_sec: i32`
- `tv_usec: i32`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> __timeval`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::gnu::adjtimex

*Function*

```rust
fn adjtimex(buf: *mut timex) -> c_int
```



## libc::unix::linux_like::linux::gnu::aiocb

*Struct*

**Fields:**
- `aio_fildes: c_int`
- `aio_lio_opcode: c_int`
- `aio_reqprio: c_int`
- `aio_buf: *mut c_void`
- `aio_nbytes: size_t`
- `aio_sigevent: crate::sigevent`
- `aio_offset: off_t`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> aiocb`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::gnu::asctime_r

*Function*

```rust
fn asctime_r(tm: *const crate::tm, buf: *mut c_char) -> *mut c_char
```



## libc::unix::linux_like::linux::gnu::backtrace

*Function*

```rust
fn backtrace(buf: *mut *mut c_void, sz: c_int) -> c_int
```



## libc::unix::linux_like::linux::gnu::backtrace_symbols

*Function*

```rust
fn backtrace_symbols(buffer: *const *mut c_void, len: c_int) -> *mut *mut c_char
```



## libc::unix::linux_like::linux::gnu::backtrace_symbols_fd

*Function*

```rust
fn backtrace_symbols_fd(buffer: *const *mut c_void, len: c_int, fd: c_int)
```



## libc::unix::linux_like::linux::gnu::clock_adjtime

*Function*

```rust
fn clock_adjtime(clk_id: crate::clockid_t, buf: *mut crate::timex) -> c_int
```



## libc::unix::linux_like::linux::gnu::close_range

*Function*

```rust
fn close_range(first: c_uint, last: c_uint, flags: c_int) -> c_int
```



## libc::unix::linux_like::linux::gnu::cmsghdr

*Struct*

**Fields:**
- `cmsg_len: size_t`
- `cmsg_level: c_int`
- `cmsg_type: c_int`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> cmsghdr`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::gnu::ctermid

*Function*

```rust
fn ctermid(s: *mut c_char) -> *mut c_char
```



## libc::unix::linux_like::linux::gnu::ctime_r

*Function*

```rust
fn ctime_r(timep: *const time_t, buf: *mut c_char) -> *mut c_char
```



## libc::unix::linux_like::linux::gnu::dirname

*Function*

```rust
fn dirname(path: *mut c_char) -> *mut c_char
```



## libc::unix::linux_like::linux::gnu::dladdr1

*Function*

```rust
fn dladdr1(addr: *const c_void, info: *mut crate::Dl_info, extra_info: *mut *mut c_void, flags: c_int) -> c_int
```



## libc::unix::linux_like::linux::gnu::dlinfo

*Function*

```rust
fn dlinfo(handle: *mut c_void, request: c_int, info: *mut c_void) -> c_int
```



## libc::unix::linux_like::linux::gnu::dlmopen

*Function*

```rust
fn dlmopen(lmid: Lmid_t, filename: *const c_char, flag: c_int) -> *mut c_void
```



## libc::unix::linux_like::linux::gnu::dlvsym

*Function*

```rust
fn dlvsym(handle: *mut c_void, symbol: *const c_char, version: *const c_char) -> *mut c_void
```



## libc::unix::linux_like::linux::gnu::eaccess

*Function*

```rust
fn eaccess(pathname: *const c_char, mode: c_int) -> c_int
```



## libc::unix::linux_like::linux::gnu::endutxent

*Function*

```rust
fn endutxent()
```



## libc::unix::linux_like::linux::gnu::epoll_pwait2

*Function*

```rust
fn epoll_pwait2(epfd: c_int, events: *mut crate::epoll_event, maxevents: c_int, timeout: *const crate::timespec, sigmask: *const crate::sigset_t) -> c_int
```



## libc::unix::linux_like::linux::gnu::euidaccess

*Function*

```rust
fn euidaccess(pathname: *const c_char, mode: c_int) -> c_int
```



## libc::unix::linux_like::linux::gnu::execveat

*Function*

```rust
fn execveat(dirfd: c_int, pathname: *const c_char, argv: *const *mut c_char, envp: *const *mut c_char, flags: c_int) -> c_int
```



## libc::unix::linux_like::linux::gnu::explicit_bzero

*Function*

```rust
fn explicit_bzero(s: *mut c_void, len: size_t)
```



## libc::unix::linux_like::linux::gnu::fanotify_event_info_error

*Struct*

**Fields:**
- `hdr: crate::fanotify_event_info_header`
- `error: crate::__s32`
- `error_count: crate::__u32`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> fanotify_event_info_error`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::gnu::fanotify_event_info_pidfd

*Struct*

**Fields:**
- `hdr: crate::fanotify_event_info_header`
- `pidfd: crate::__s32`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> fanotify_event_info_pidfd`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::gnu::fanotify_mark

*Function*

```rust
fn fanotify_mark(fd: c_int, flags: c_uint, mask: u64, dirfd: c_int, path: *const c_char) -> c_int
```



## libc::unix::linux_like::linux::gnu::fgetgrent_r

*Function*

```rust
fn fgetgrent_r(stream: *mut crate::FILE, grp: *mut crate::group, buf: *mut c_char, buflen: size_t, result: *mut *mut crate::group) -> c_int
```



## libc::unix::linux_like::linux::gnu::fgetpwent_r

*Function*

```rust
fn fgetpwent_r(stream: *mut crate::FILE, pwd: *mut crate::passwd, buf: *mut c_char, buflen: size_t, result: *mut *mut crate::passwd) -> c_int
```



## libc::unix::linux_like::linux::gnu::fgetspent_r

*Function*

```rust
fn fgetspent_r(fp: *mut crate::FILE, spbuf: *mut crate::spwd, buf: *mut c_char, buflen: size_t, spbufp: *mut *mut crate::spwd) -> c_int
```



## libc::unix::linux_like::linux::gnu::fpos64_t

*Struct*

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> fpos64_t`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::gnu::fpos_t

*Struct*

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> fpos_t`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::gnu::getauxval

*Function*

```rust
fn getauxval(type_: c_ulong) -> c_ulong
```



## libc::unix::linux_like::linux::gnu::getentropy

*Function*

```rust
fn getentropy(buf: *mut c_void, buflen: size_t) -> c_int
```



## libc::unix::linux_like::linux::gnu::getgrent_r

*Function*

```rust
fn getgrent_r(grp: *mut crate::group, buf: *mut c_char, buflen: size_t, result: *mut *mut crate::group) -> c_int
```



## libc::unix::linux_like::linux::gnu::getmntent_r

*Function*

```rust
fn getmntent_r(stream: *mut crate::FILE, mntbuf: *mut crate::mntent, buf: *mut c_char, buflen: c_int) -> *mut crate::mntent
```



## libc::unix::linux_like::linux::gnu::getpriority

*Function*

```rust
fn getpriority(which: crate::__priority_which_t, who: crate::id_t) -> c_int
```



## libc::unix::linux_like::linux::gnu::getpt

*Function*

```rust
fn getpt() -> c_int
```



## libc::unix::linux_like::linux::gnu::getpwent_r

*Function*

```rust
fn getpwent_r(pwd: *mut crate::passwd, buf: *mut c_char, buflen: size_t, result: *mut *mut crate::passwd) -> c_int
```



## libc::unix::linux_like::linux::gnu::getrandom

*Function*

```rust
fn getrandom(buf: *mut c_void, buflen: size_t, flags: c_uint) -> ssize_t
```



## libc::unix::linux_like::linux::gnu::getrlimit

*Function*

```rust
fn getrlimit(resource: crate::__rlimit_resource_t, rlim: *mut crate::rlimit) -> c_int
```



## libc::unix::linux_like::linux::gnu::getrlimit64

*Function*

```rust
fn getrlimit64(resource: crate::__rlimit_resource_t, rlim: *mut crate::rlimit64) -> c_int
```



## libc::unix::linux_like::linux::gnu::getspent_r

*Function*

```rust
fn getspent_r(spbuf: *mut crate::spwd, buf: *mut c_char, buflen: size_t, spbufp: *mut *mut crate::spwd) -> c_int
```



## libc::unix::linux_like::linux::gnu::gettimeofday

*Function*

```rust
fn gettimeofday(tp: *mut crate::timeval, tz: *mut crate::timezone) -> c_int
```



## libc::unix::linux_like::linux::gnu::getutxent

*Function*

```rust
fn getutxent() -> *mut utmpx
```



## libc::unix::linux_like::linux::gnu::getutxid

*Function*

```rust
fn getutxid(ut: *const utmpx) -> *mut utmpx
```



## libc::unix::linux_like::linux::gnu::getutxline

*Function*

```rust
fn getutxline(ut: *const utmpx) -> *mut utmpx
```



## libc::unix::linux_like::linux::gnu::glob64

*Function*

```rust
fn glob64(pattern: *const c_char, flags: c_int, errfunc: Option<fn(...)>, pglob: *mut glob64_t) -> c_int
```



## libc::unix::linux_like::linux::gnu::glob64_t

*Struct*

**Fields:**
- `gl_pathc: size_t`
- `gl_pathv: *mut *mut c_char`
- `gl_offs: size_t`
- `gl_flags: c_int`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> glob64_t`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::gnu::globfree64

*Function*

```rust
fn globfree64(pglob: *mut glob64_t)
```



## libc::unix::linux_like::linux::gnu::gnu_basename

*Function*

GNU version of `basename(3)`, defined in `string.h`.

```rust
fn gnu_basename(path: *const c_char) -> *mut c_char
```



## libc::unix::linux_like::linux::gnu::gnu_get_libc_release

*Function*

```rust
fn gnu_get_libc_release() -> *const c_char
```



## libc::unix::linux_like::linux::gnu::gnu_get_libc_version

*Function*

```rust
fn gnu_get_libc_version() -> *const c_char
```



## libc::unix::linux_like::linux::gnu::iocb

*Struct*

**Fields:**
- `aio_data: crate::__u64`
- `aio_key: crate::__u32`
- `aio_rw_flags: crate::__kernel_rwf_t`
- `aio_lio_opcode: crate::__u16`
- `aio_reqprio: crate::__s16`
- `aio_fildes: crate::__u32`
- `aio_buf: crate::__u64`
- `aio_nbytes: crate::__u64`
- `aio_offset: crate::__s64`
- `aio_flags: crate::__u32`
- `aio_resfd: crate::__u32`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> iocb`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::gnu::mallinfo

*Function*

```rust
fn mallinfo() -> crate::mallinfo
```



## libc::unix::linux_like::linux::gnu::mallinfo

*Struct*

**Fields:**
- `arena: c_int`
- `ordblks: c_int`
- `smblks: c_int`
- `hblks: c_int`
- `hblkhd: c_int`
- `usmblks: c_int`
- `fsmblks: c_int`
- `uordblks: c_int`
- `fordblks: c_int`
- `keepcost: c_int`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> mallinfo`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::gnu::mallinfo2

*Struct*

**Fields:**
- `arena: size_t`
- `ordblks: size_t`
- `smblks: size_t`
- `hblks: size_t`
- `hblkhd: size_t`
- `usmblks: size_t`
- `fsmblks: size_t`
- `uordblks: size_t`
- `fordblks: size_t`
- `keepcost: size_t`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> mallinfo2`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::gnu::mallinfo2

*Function*

```rust
fn mallinfo2() -> crate::mallinfo2
```



## libc::unix::linux_like::linux::gnu::malloc_info

*Function*

```rust
fn malloc_info(options: c_int, stream: *mut crate::FILE) -> c_int
```



## libc::unix::linux_like::linux::gnu::malloc_stats

*Function*

```rust
fn malloc_stats()
```



## libc::unix::linux_like::linux::gnu::malloc_trim

*Function*

```rust
fn malloc_trim(__pad: size_t) -> c_int
```



## libc::unix::linux_like::linux::gnu::malloc_usable_size

*Function*

```rust
fn malloc_usable_size(ptr: *mut c_void) -> size_t
```



## libc::unix::linux_like::linux::gnu::mallopt

*Function*

```rust
fn mallopt(param: c_int, value: c_int) -> c_int
```



## libc::unix::linux_like::linux::gnu::mbstate_t

*Struct*

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> mbstate_t`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::gnu::memfd_create

*Function*

```rust
fn memfd_create(name: *const c_char, flags: c_uint) -> c_int
```



## libc::unix::linux_like::linux::gnu::mempcpy

*Function*

```rust
fn mempcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void
```



## libc::unix::linux_like::linux::gnu::mlock2

*Function*

```rust
fn mlock2(addr: *const c_void, len: size_t, flags: c_uint) -> c_int
```



## libc::unix::linux_like::linux::gnu::mq_notify

*Function*

```rust
fn mq_notify(mqdes: crate::mqd_t, sevp: *const crate::sigevent) -> c_int
```



## libc::unix::linux_like::linux::gnu::msghdr

*Struct*

**Fields:**
- `msg_name: *mut c_void`
- `msg_namelen: crate::socklen_t`
- `msg_iov: *mut crate::iovec`
- `msg_iovlen: size_t`
- `msg_control: *mut c_void`
- `msg_controllen: size_t`
- `msg_flags: c_int`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> msghdr`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::gnu::ntp_adjtime

*Function*

```rust
fn ntp_adjtime(buf: *mut timex) -> c_int
```



## libc::unix::linux_like::linux::gnu::ntp_gettime

*Function*

```rust
fn ntp_gettime(buf: *mut ntptimeval) -> c_int
```



## libc::unix::linux_like::linux::gnu::ntptimeval

*Struct*

**Fields:**
- `time: crate::timeval`
- `maxerror: c_long`
- `esterror: c_long`
- `tai: c_long`
- `__glibc_reserved1: c_long`
- `__glibc_reserved2: c_long`
- `__glibc_reserved3: c_long`
- `__glibc_reserved4: c_long`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ntptimeval`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::gnu::posix_basename

*Function*

POSIX version of `basename(3)`, defined in `libgen.h`.

```rust
fn posix_basename(path: *mut c_char) -> *mut c_char
```



## libc::unix::linux_like::linux::gnu::posix_spawn_file_actions_addchdir_np

*Function*

```rust
fn posix_spawn_file_actions_addchdir_np(actions: *mut crate::posix_spawn_file_actions_t, path: *const c_char) -> c_int
```



## libc::unix::linux_like::linux::gnu::posix_spawn_file_actions_addclosefrom_np

*Function*

```rust
fn posix_spawn_file_actions_addclosefrom_np(actions: *mut crate::posix_spawn_file_actions_t, from: c_int) -> c_int
```



## libc::unix::linux_like::linux::gnu::posix_spawn_file_actions_addfchdir_np

*Function*

```rust
fn posix_spawn_file_actions_addfchdir_np(actions: *mut crate::posix_spawn_file_actions_t, fd: c_int) -> c_int
```



## libc::unix::linux_like::linux::gnu::posix_spawn_file_actions_addtcsetpgrp_np

*Function*

```rust
fn posix_spawn_file_actions_addtcsetpgrp_np(actions: *mut crate::posix_spawn_file_actions_t, tcfd: c_int) -> c_int
```



## libc::unix::linux_like::linux::gnu::preadv2

*Function*

```rust
fn preadv2(fd: c_int, iov: *const crate::iovec, iovcnt: c_int, offset: off_t, flags: c_int) -> ssize_t
```



## libc::unix::linux_like::linux::gnu::preadv64v2

*Function*

```rust
fn preadv64v2(fd: c_int, iov: *const crate::iovec, iovcnt: c_int, offset: off64_t, flags: c_int) -> ssize_t
```



## libc::unix::linux_like::linux::gnu::prlimit

*Function*

```rust
fn prlimit(pid: crate::pid_t, resource: crate::__rlimit_resource_t, new_limit: *const crate::rlimit, old_limit: *mut crate::rlimit) -> c_int
```



## libc::unix::linux_like::linux::gnu::prlimit64

*Function*

```rust
fn prlimit64(pid: crate::pid_t, resource: crate::__rlimit_resource_t, new_limit: *const crate::rlimit64, old_limit: *mut crate::rlimit64) -> c_int
```



## libc::unix::linux_like::linux::gnu::pthread_attr_getaffinity_np

*Function*

```rust
fn pthread_attr_getaffinity_np(attr: *const crate::pthread_attr_t, cpusetsize: size_t, cpuset: *mut crate::cpu_set_t) -> c_int
```



## libc::unix::linux_like::linux::gnu::pthread_attr_setaffinity_np

*Function*

```rust
fn pthread_attr_setaffinity_np(attr: *mut crate::pthread_attr_t, cpusetsize: size_t, cpuset: *const crate::cpu_set_t) -> c_int
```



## libc::unix::linux_like::linux::gnu::pthread_rwlockattr_getkind_np

*Function*

```rust
fn pthread_rwlockattr_getkind_np(attr: *const crate::pthread_rwlockattr_t, val: *mut c_int) -> c_int
```



## libc::unix::linux_like::linux::gnu::pthread_rwlockattr_setkind_np

*Function*

```rust
fn pthread_rwlockattr_setkind_np(attr: *mut crate::pthread_rwlockattr_t, val: c_int) -> c_int
```



## libc::unix::linux_like::linux::gnu::pthread_sigqueue

*Function*

```rust
fn pthread_sigqueue(thread: crate::pthread_t, sig: c_int, value: crate::sigval) -> c_int
```



## libc::unix::linux_like::linux::gnu::pthread_t

*Type Alias*: `c_ulong`



## libc::unix::linux_like::linux::gnu::pthread_timedjoin_np

*Function*

```rust
fn pthread_timedjoin_np(thread: crate::pthread_t, retval: *mut *mut c_void, abstime: *const crate::timespec) -> c_int
```



## libc::unix::linux_like::linux::gnu::pthread_tryjoin_np

*Function*

```rust
fn pthread_tryjoin_np(thread: crate::pthread_t, retval: *mut *mut c_void) -> c_int
```



## libc::unix::linux_like::linux::gnu::ptrace

*Function*

```rust
fn ptrace(request: c_uint) -> c_long
```



## libc::unix::linux_like::linux::gnu::ptrace_peeksiginfo_args

*Struct*

**Fields:**
- `off: crate::__u64`
- `flags: crate::__u32`
- `nr: crate::__s32`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ptrace_peeksiginfo_args`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::gnu::ptrace_sud_config

*Struct*

**Fields:**
- `mode: crate::__u64`
- `selector: crate::__u64`
- `offset: crate::__u64`
- `len: crate::__u64`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ptrace_sud_config`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::gnu::ptrace_syscall_info

*Struct*

**Fields:**
- `op: crate::__u8`
- `pad: [crate::__u8; 3]`
- `arch: crate::__u32`
- `instruction_pointer: crate::__u64`
- `stack_pointer: crate::__u64`
- `u: __c_anonymous_ptrace_syscall_info_data`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ptrace_syscall_info`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::gnu::putgrent

*Function*

```rust
fn putgrent(grp: *const crate::group, stream: *mut crate::FILE) -> c_int
```



## libc::unix::linux_like::linux::gnu::putpwent

*Function*

```rust
fn putpwent(p: *const crate::passwd, stream: *mut crate::FILE) -> c_int
```



## libc::unix::linux_like::linux::gnu::pututxline

*Function*

```rust
fn pututxline(ut: *const utmpx) -> *mut utmpx
```



## libc::unix::linux_like::linux::gnu::pwritev2

*Function*

```rust
fn pwritev2(fd: c_int, iov: *const crate::iovec, iovcnt: c_int, offset: off_t, flags: c_int) -> ssize_t
```



## libc::unix::linux_like::linux::gnu::pwritev64v2

*Function*

```rust
fn pwritev64v2(fd: c_int, iov: *const crate::iovec, iovcnt: c_int, offset: off64_t, flags: c_int) -> ssize_t
```



## libc::unix::linux_like::linux::gnu::qsort_r

*Function*

```rust
fn qsort_r(base: *mut c_void, num: size_t, size: size_t, compar: Option<fn(...)>, arg: *mut c_void)
```



## libc::unix::linux_like::linux::gnu::reallocarray

*Function*

```rust
fn reallocarray(ptr: *mut c_void, nmemb: size_t, size: size_t) -> *mut c_void
```



## libc::unix::linux_like::linux::gnu::recvmmsg

*Function*

```rust
fn recvmmsg(sockfd: c_int, msgvec: *mut crate::mmsghdr, vlen: c_uint, flags: c_int, timeout: *mut crate::timespec) -> c_int
```



## libc::unix::linux_like::linux::gnu::regex_t

*Struct*

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> regex_t`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::gnu::regoff_t

*Type Alias*: `c_int`



## libc::unix::linux_like::linux::gnu::renameat2

*Function*

```rust
fn renameat2(olddirfd: c_int, oldpath: *const c_char, newdirfd: c_int, newpath: *const c_char, flags: c_uint) -> c_int
```



## libc::unix::linux_like::linux::gnu::sem_t

*Struct*

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> sem_t`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::gnu::seminfo

*Struct*

**Fields:**
- `semmap: c_int`
- `semmni: c_int`
- `semmns: c_int`
- `semmnu: c_int`
- `semmsl: c_int`
- `semopm: c_int`
- `semume: c_int`
- `semusz: c_int`
- `semvmx: c_int`
- `semaem: c_int`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> seminfo`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::gnu::sendmmsg

*Function*

```rust
fn sendmmsg(sockfd: c_int, msgvec: *mut crate::mmsghdr, vlen: c_uint, flags: c_int) -> c_int
```



## libc::unix::linux_like::linux::gnu::sethostid

*Function*

```rust
fn sethostid(hostid: c_long) -> c_int
```



## libc::unix::linux_like::linux::gnu::setpriority

*Function*

```rust
fn setpriority(which: crate::__priority_which_t, who: crate::id_t, prio: c_int) -> c_int
```



## libc::unix::linux_like::linux::gnu::setrlimit

*Function*

```rust
fn setrlimit(resource: crate::__rlimit_resource_t, rlim: *const crate::rlimit) -> c_int
```



## libc::unix::linux_like::linux::gnu::setrlimit64

*Function*

```rust
fn setrlimit64(resource: crate::__rlimit_resource_t, rlim: *const crate::rlimit64) -> c_int
```



## libc::unix::linux_like::linux::gnu::setutxent

*Function*

```rust
fn setutxent()
```



## libc::unix::linux_like::linux::gnu::sgetspent_r

*Function*

```rust
fn sgetspent_r(s: *const c_char, spbuf: *mut crate::spwd, buf: *mut c_char, buflen: size_t, spbufp: *mut *mut crate::spwd) -> c_int
```



## libc::unix::linux_like::linux::gnu::tcp_info

*Struct*

**Fields:**
- `tcpi_state: u8`
- `tcpi_ca_state: u8`
- `tcpi_retransmits: u8`
- `tcpi_probes: u8`
- `tcpi_backoff: u8`
- `tcpi_options: u8`
- `tcpi_snd_rcv_wscale: u8` - This contains the bitfields `tcpi_snd_wscale` and `tcpi_rcv_wscale`.
- `tcpi_rto: u32`
- `tcpi_ato: u32`
- `tcpi_snd_mss: u32`
- `tcpi_rcv_mss: u32`
- `tcpi_unacked: u32`
- `tcpi_sacked: u32`
- `tcpi_lost: u32`
- `tcpi_retrans: u32`
- `tcpi_fackets: u32`
- `tcpi_last_data_sent: u32`
- `tcpi_last_ack_sent: u32`
- `tcpi_last_data_recv: u32`
- `tcpi_last_ack_recv: u32`
- `tcpi_pmtu: u32`
- `tcpi_rcv_ssthresh: u32`
- `tcpi_rtt: u32`
- `tcpi_rttvar: u32`
- `tcpi_snd_ssthresh: u32`
- `tcpi_snd_cwnd: u32`
- `tcpi_advmss: u32`
- `tcpi_reordering: u32`
- `tcpi_rcv_rtt: u32`
- `tcpi_rcv_space: u32`
- `tcpi_total_retrans: u32`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> tcp_info`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::gnu::termios

*Struct*

**Fields:**
- `c_iflag: crate::tcflag_t`
- `c_oflag: crate::tcflag_t`
- `c_cflag: crate::tcflag_t`
- `c_lflag: crate::tcflag_t`
- `c_line: crate::cc_t`
- `c_cc: [crate::cc_t; 32]`
- `c_ispeed: crate::speed_t`
- `c_ospeed: crate::speed_t`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> termios`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::gnu::tgkill

*Function*

```rust
fn tgkill(tgid: crate::pid_t, tid: crate::pid_t, sig: c_int) -> c_int
```



## libc::unix::linux_like::linux::gnu::timespec

*Struct*

**Fields:**
- `tv_sec: time_t`
- `tv_nsec: c_long`

**Traits:** Copy

**Trait Implementations:**

- **Default**
  - `fn default() -> timespec`
- **Clone**
  - `fn clone(self: &Self) -> timespec`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::gnu::utmpname

*Function*

```rust
fn utmpname(file: *const c_char) -> c_int
```



## libc::unix::linux_like::linux::gnu::utmpx

*Struct*

**Fields:**
- `ut_type: c_short`
- `ut_pid: crate::pid_t`
- `ut_line: [c_char; 32]`
- `ut_id: [c_char; 4]`
- `ut_user: [c_char; 32]`
- `ut_host: [c_char; 256]`
- `ut_exit: __exit_status`
- `ut_session: i32`
- `ut_tv: __timeval`
- `ut_addr_v6: [i32; 4]`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> utmpx`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## libc::unix::linux_like::linux::gnu::utmpxname

*Function*

```rust
fn utmpxname(file: *const c_char) -> c_int
```



