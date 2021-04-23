use crate::to_str;

use libc::{c_char, c_short, c_void, pid_t, read};
use std::fs::File;
use std::io::Error;
use std::mem;
use std::os::unix::prelude::*;

#[doc(hidden)]
#[repr(C)]
#[derive(Debug)]
pub struct exit_status {
    pub e_termination: c_short,
    pub e_exit: c_short,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug)]
pub struct ut_tv {
    pub tv_sec: i32,
    pub tv_usec: i32,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug)]
pub struct utmp {
    pub ut_type: c_short,
    pub ut_pid: pid_t,
    pub ut_line: [c_char; 32],
    pub ut_id: [c_char; 4],
    pub ut_user: [c_char; 32],
    pub ut_host: [c_char; 256],
    pub ut_exit: exit_status,
    pub ut_session: i32,
    pub ut_tv: ut_tv,
    pub ut_addr_v6: [i32; 4],
    pub __glibc_reserved: [c_char; 20],
}

/// Get the currently logged users.
///
/// On linux it will get them from `/var/run/utmp`. It will use the C's UTMP Struct and the unsafe read C's function.
pub fn get_users() -> Result<Vec<String>, Error> {
    let mut users: Vec<String> = Vec::new();
    let utmp_file = File::open("/var/run/utmp")?;
    let buffer = std::mem::MaybeUninit::<utmp>::uninit().as_mut_ptr();

    while unsafe {
        read(
            utmp_file.as_raw_fd(),
            buffer as *mut c_void,
            mem::size_of::<utmp>(),
        )
    } != 0
    {
        let cbuffer = unsafe { &*(buffer as *mut utmp) as &utmp };
        let cuser = unsafe { &*(&cbuffer.ut_user as *const [i8]) };

        if cuser[0] != 0 && cbuffer.ut_type == 7 {
            let csuser = to_str(cuser.as_ptr()).trim_matches('\0').to_owned();
            users.push(csuser);
        }
    }

    Ok(users)
}
