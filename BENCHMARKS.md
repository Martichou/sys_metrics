Comparing `sys_metrics` to other crates
-------------

This page provides some benchmark comparisons of `sys_metrics` against other crates.

Benchmarks
-------------
The benchmarks were performed on an i7-8750H running Windows 11 WSL2.
As a result this does only reflect performance on Linux.

Note that I only listed the benchmarks that were comparable, some benchmarks from heim, sys_metrics, ... are not present in the others. 

At the moment [sysinfo](https://github.com/GuillaumeGomez/sysinfo) is not part of this benchmark comparison. If someone would like to help me, I'm having a little trouble figuring out what to compare with what in sysinfo (because of the refresh).

|                | [sys_metrics](https://github.com/Martichou/sys_metrics)      | [heim](https://github.com/heim-rs/heim)   | [rust_psutil](https://github.com/rust-psutil/rust-psutil) |
|-|:-:|:-:|:-:|
| cpu_logical_count | **12.714 ns** | 228.28 ns | 865.69 ns |
| cpu_physical_count | **149.42 us** | 225.26 us | 177.73 us |
| cpu_frequency | **11.837 us** | 136.79 us | x |
| cpu_stats | **11.045 us** | 74.028 us | x |
| cpu_times | **9.5326 us** | 198.55 us | 15.485 us |
|||||
| disk_io | **56.766 us** | 263.63 us | 124.44 us |
| disk_io_physical | **119.05 us** | 2.2577 ms | x |
| disk_partitions | 1.1598 ms | 238.11 us | **95.982 us** |
| disk_partitions_physical | **50.970 us** | 343.04 us | 96.378 us |
|||||
| host_info | 854.26 ns | 887.65 ns | **501.43 ns** |
| logged_users | **1.2781 us** | 5.0848 us | x |
|||||
| memory | **7.0961 us** | 79.720 us | 25.918 us |
| swap | **409.02 ns** | 102.94 us | 94.424 us |
|||||
| net_io | **14.387 us** | 206.43 us | x |
| net_io_physical | **409.02 ns** | 102.94 us | x |
