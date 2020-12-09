// https://supportcenter.checkpoint.com/supportcenter/portal?eventSubmit_doGoviewsolutiondetails=&solutionid=sk65143
// read from /proc/stat and capture the 10 column with many information
//	1st column : user = normal processes executing in user mode
//	2nd column : nice = niced processes executing in user mode
//	3rd column : system = processes executing in kernel mode
//	4th column : idle = twiddling thumbs
//	5th column : iowait = waiting for I/O to complete
//	6th column : irq = servicing interrupts
//	7th column : softirq = servicing softirqs
//	8th column : steal = ticks spent executing other virtual hosts
//	9th column : guest = time spent running a virtual CPU for guest operating systems under the control of the Kernel
//	10th column: guest_nice = time spent running a niced guest (virtual CPU for guest operating systems under the control of the Linux kernel)
