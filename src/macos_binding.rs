use libc::statfs;
use mach::{
    kern_return::kern_return_t,
    mach_types::{host_name_port_t, host_t},
    message::mach_msg_type_number_t,
    vm_types::{integer_t, natural_t},
};

#[doc(hidden)]
#[repr(C)]
#[derive(Debug)]
pub struct vmmeter {
    pub v_swtch: libc::c_uint,
    pub v_trap: libc::c_uint,
    pub v_syscall: libc::c_uint,
    pub v_intr: libc::c_uint,
    pub v_soft: libc::c_uint,
    pub v_faults: libc::c_uint,

    pub v_lookups: libc::c_uint,
    pub v_hits: libc::c_uint,
    pub v_vm_faults: libc::c_uint,
    pub v_cow_faults: libc::c_uint,
    pub v_swpin: libc::c_uint,
    pub v_swpout: libc::c_uint,
    pub v_pswpin: libc::c_uint,
    pub v_pswpout: libc::c_uint,
    pub v_pageins: libc::c_uint,
    pub v_pageouts: libc::c_uint,
    pub v_pgpgin: libc::c_uint,
    pub v_pgpgout: libc::c_uint,
    pub v_intrans: libc::c_uint,
    pub v_reactivated: libc::c_uint,
    pub v_rev: libc::c_uint,
    pub v_scan: libc::c_uint,
    pub v_dfree: libc::c_uint,
    pub v_pfree: libc::c_uint,
    pub v_zfod: libc::c_uint,
    pub v_nzfod: libc::c_uint,

    pub v_page_size: libc::c_uint,
    pub v_kernel_pages: libc::c_uint,
    pub v_free_target: libc::c_uint,
    pub v_free_min: libc::c_uint,
    pub v_free_count: libc::c_uint,
    pub v_wire_count: libc::c_uint,
    pub v_active_count: libc::c_uint,
    pub v_inactive_target: libc::c_uint,
    pub v_inactive_count: libc::c_uint,
}

#[doc(hidden)]
#[cfg(target_os = "macos")]
#[repr(C)]
pub struct vm_statistics64 {
    pub free_count: natural_t,
    pub active_count: natural_t,
    pub inactive_count: natural_t,
    pub wire_count: natural_t,
    pub zero_fill_count: u64,
    pub reactivations: u64,
    pub pageins: u64,
    pub pageouts: u64,
    pub faults: u64,
    pub cow_faults: u64,
    pub lookups: u64,
    pub hits: u64,
    pub purges: u64,
    pub purgeable_count: natural_t,
    pub speculative_count: natural_t,
    pub decompressions: u64,
    pub compressions: u64,
    pub swapins: u64,
    pub swapouts: u64,
    pub compressor_page_count: natural_t,
    pub throttled_count: natural_t,
    pub external_page_count: natural_t,
    pub internal_page_count: natural_t,
    pub total_uncompressed_pages_in_compressor: u64,
}

/// https://developer.apple.com/documentation/kernel/host_flavor_t?language=objc
#[allow(non_camel_case_types)]
pub type host_flavor_t = integer_t;
/// https://developer.apple.com/documentation/kernel/host_info_t?language=objc
#[allow(non_camel_case_types)]
pub type host_info_t = *mut integer_t;
/// https://developer.apple.com/documentation/kernel/host_info64_t?language=objc
#[allow(non_camel_case_types)]
pub type host_info64_t = *mut integer_t;

extern "C" {
    pub fn mach_host_self() -> host_name_port_t;

    pub fn host_statistics(
        host_priv: host_t,
        flavor: host_flavor_t,
        host_info_out: host_info_t,
        host_info_outCnt: *const mach_msg_type_number_t,
    ) -> kern_return_t;

    pub fn host_statistics64(
        host_priv: host_t,
        flavor: integer_t,
        host_info_out: *mut integer_t,
        host_info_outCnt: *const mach_msg_type_number_t,
    ) -> kern_return_t;

    pub fn getfsstat64(buf: *mut statfs, bufsize: libc::c_int, flags: libc::c_int) -> libc::c_int;
}
