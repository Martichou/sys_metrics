use crate::to_str;

use libc::{c_char, c_short, c_void, pid_t, read};
use std::fs::File;
use std::io::Error;
use std::mem;
use std::os::unix::prelude::*;

const UT_LINESIZE: usize = 32;
const UT_NAMESIZE: usize = 32;
const UT_HOSTSIZE: usize = 256;

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
    pub ut_line: [c_char; UT_LINESIZE],
    pub ut_id: [c_char; 4],
    pub ut_user: [c_char; UT_NAMESIZE],
    pub ut_host: [c_char; UT_HOSTSIZE],
    pub ut_exit: exit_status,
    pub ut_session: i32,
    pub ut_tv: ut_tv,
    pub ut_addr_v6: [i32; 4],
    pub __glibc_reserved: [c_char; 20],
}

impl Default for exit_status {
    fn default() -> exit_status {
        exit_status {
            e_termination: 0,
            e_exit: 0,
        }
    }
}

impl Default for ut_tv {
    fn default() -> ut_tv {
        ut_tv {
            tv_sec: 0,
            tv_usec: 0,
        }
    }
}

impl Default for utmp {
    fn default() -> utmp {
        utmp {
            ut_type: 0,
            ut_pid: 0,
            ut_line: [0; UT_LINESIZE],
            ut_id: [0; 4],
            ut_user: [0; UT_NAMESIZE],
            ut_host: [0; UT_HOSTSIZE],
            ut_exit: Default::default(),
            ut_session: 0,
            ut_tv: Default::default(),
            ut_addr_v6: [0; 4],
            __glibc_reserved: [0; 20],
        }
    }
}

/// Get the currently logged users.
///
/// On linux it will get them from `/var/run/utmp`. It will use the C's UTMP Struct and the unsafe read C's function.
///
/// On macOS it will use unsafes call to multiple OSX specific functions [setutxent, getutxent] (the struct is UTMPX for the inner usage).
pub fn get_users() -> Result<Vec<String>, Error> {
    let mut users: Vec<String> = Vec::new();
    let utmp_file = File::open("/var/run/utmp")?;
    let mut utmp_struct: utmp = Default::default();
    let buffer: *mut c_void = &mut utmp_struct as *mut _ as *mut c_void;

    unsafe {
        while read(utmp_file.as_raw_fd(), buffer, mem::size_of::<utmp>()) != 0 {
            let cbuffer = &*(buffer as *mut utmp) as &utmp;
            let cuser = &*(&cbuffer.ut_user as *const [i8]);

            if cuser[0] != 0 && cbuffer.ut_type == 7 {
                let csuser = to_str(cuser.as_ptr()).trim_matches('\0').to_owned();
                users.push(csuser);
            }
        }
    }

    Ok(users)
}
