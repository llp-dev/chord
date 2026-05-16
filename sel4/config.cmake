# Generic configuration options
set(KernelVerificationBuild OFF CACHE BOOL "")
set(KernelIsMCS OFF CACHE BOOL "")
set(KernelRootCNodeSizeBits 12 CACHE STRING "")
set(KernelRetypeFanOutLimit 256 CACHE STRING "")
set(KernelMaxNumBootinfoUntypedCaps 230 CACHE STRING "")

# Scheduling configuration options

# Debug configuration options
set(KernelDebugBuild ON CACHE BOOL "")
set(KernelPrinting ON CACHE BOOL "")

# Performance analysis and profiling configuration options

# Target hardware architecture/platform options
set(KernelPlatform pc99 CACHE STRING "")
set(KernelSel4Arch x86_64 CACHE STRING "")
set(KernelMaxNumNodes 1 CACHE STRING "")

# x86
set(KernelVTX ON CACHE BOOL "")
set(KernelHugePage ON CACHE BOOL "")
set(KernelSupportPCID ON CACHE BOOL "")
set(KernelMultiboot1Header OFF CACHE BOOL "")
set(KernelMultiboot2Header ON CACHE BOOL "")
