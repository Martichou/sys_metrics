use crate::memory::Swap;

use std::io::Error;

/// Return the [Swap] struct.
///
/// [Swap]: ../memory/struct.Swap.html
pub fn get_swap() -> Result<Swap, Error> {
    let mut name: [i32; 2] = [libc::CTL_VM, libc::VM_SWAPUSAGE];
    let mut swap_info = std::mem::MaybeUninit::<libc::xsw_usage>::uninit();
    let mut length = std::mem::size_of::<libc::xsw_usage>();
    if unsafe {
        libc::sysctl(
            name.as_mut_ptr(),
            2,
            swap_info.as_mut_ptr() as *mut libc::c_void,
            &mut length,
            std::ptr::null_mut(),
            0,
        )
    } != 0
    {
        return Err(Error::last_os_error());
    }
    let swap_info = unsafe { swap_info.assume_init() };

    // Compute the values from the swap_info and divide by 1024 for MB
    Ok(Swap {
        total: swap_info.xsu_total / (1024 * 1024),
        free: swap_info.xsu_avail / (1024 * 1024),
        used: swap_info.xsu_used / (1024 * 1024),
    })
}

// The [2, 124] values were got by reverse checking the sysctl call.
// Like so: do the sysctl call using vm.compressor_mode instead of ptr::null_mut() and [0, 3] as the name.
// [0, 3] is "magic and undocumented" as per https://github.com/st3fan/osx-10.9/blob/master/Libc-997.1.1/gen/FreeBSD/sysctlbyname.c
// Which in terms give us the read name (code (2-124)).

/// Determine if the system uses Swap.
///
/// Check the value of vm.compressor_mode and return true if the return is 4, else otherwise.
/// See: https://tr23.net/2014/02/04/memory-compression-settings-in-osx-10-9/
pub fn has_swap() -> Result<bool, Error> {
    let mut name: [i32; 2] = [libc::CTL_VM, 124];
    let mut value = std::mem::MaybeUninit::<i32>::uninit();
    let mut length = std::mem::size_of::<i32>();
    if unsafe {
        libc::sysctl(
            name.as_mut_ptr(),
            2,
            value.as_mut_ptr() as *mut libc::c_void,
            &mut length,
            std::ptr::null_mut(),
            0,
        )
    } != 0
    {
        return Err(Error::last_os_error());
    }
    let value = unsafe { value.assume_init() };

    Ok(value == 4)
}
