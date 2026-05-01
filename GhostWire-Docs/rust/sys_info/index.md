Crate sys\_info Copy item path

<span class="sub-heading"><a href="../src/sys_info/lib.rs.html#1-941" class="src">Source</a>
</span>

Expand description

\#Introduction This crate focuses on geting system information.

For now it supports Linux, Mac OS X and Windows. And now it can get
information of kernel/cpu/memory/disk/load/hostname and so on.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.DiskInfo.html" class="struct"
title="struct sys_info::DiskInfo">DiskInfo</a>  
Disk information.

<a href="struct.LinuxOSReleaseInfo.html" class="struct"
title="struct sys_info::LinuxOSReleaseInfo">LinuxOSReleaseInfo</a>  
The os release info of Linux.

<a href="struct.LoadAvg.html" class="struct"
title="struct sys_info::LoadAvg">LoadAvg</a>  
System load average value.

<a href="struct.MemInfo.html" class="struct"
title="struct sys_info::MemInfo">MemInfo</a>  
System memory information.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Error.html" class="enum"
title="enum sys_info::Error">Error</a>  
Error types

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.boottime.html" class="fn"
title="fn sys_info::boottime">boottime</a>  
Get system boottime

<a href="fn.cpu_num.html" class="fn"
title="fn sys_info::cpu_num">cpu_num</a>  
Get cpu num quantity.

<a href="fn.cpu_speed.html" class="fn"
title="fn sys_info::cpu_speed">cpu_speed</a>  
Get cpu speed.

<a href="fn.disk_info.html" class="fn"
title="fn sys_info::disk_info">disk_info</a>  
Get disk information.

<a href="fn.hostname.html" class="fn"
title="fn sys_info::hostname">hostname</a>  
Get hostname.

<a href="fn.linux_os_release.html" class="fn"
title="fn sys_info::linux_os_release">linux_os_release</a>  
Get the os release note of Linux

<a href="fn.loadavg.html" class="fn"
title="fn sys_info::loadavg">loadavg</a>  
Get system load average value.

<a href="fn.mem_info.html" class="fn"
title="fn sys_info::mem_info">mem_info</a>  
Get memory information.

<a href="fn.os_release.html" class="fn"
title="fn sys_info::os_release">os_release</a>  
Get operation system release version.

<a href="fn.os_type.html" class="fn"
title="fn sys_info::os_type">os_type</a>  
Get operation system type.

<a href="fn.proc_total.html" class="fn"
title="fn sys_info::proc_total">proc_total</a>  
Get current processes quantity.
