use crate::memory::Swap;

use std::io::Error;

/// Return the [Swap] struct.
///
/// It will get the info from syscall to sysinfo.
///
/// [Swap]: ../memory/struct.Swap.html
pub fn get_swap() -> Result<Swap, Error> {
    let mut name: [i32; 2] = [2, 5];
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

    // Compute the values from the swap_info and divide by 1024 for kb
    Ok(Swap {
        total: swap_info.xsu_total / 1024,
        free: swap_info.xsu_avail / 1024,
        used: swap_info.xsu_used / 1024,
    })
}
