use crate::to_str;

use core_foundation_sys::{
    dictionary::{CFDictionaryGetValueIfPresent, CFDictionaryRef},
    number::{CFBooleanGetValue, CFBooleanRef, CFNumberGetValue, CFNumberRef},
    string::{CFStringGetCString, CFStringRef},
};
use io_kit_sys::*;
use libc::c_void;
use std::io::Error;
use std::io::ErrorKind;

pub(crate) trait KeyVal {
    unsafe fn get_dict(&self, key: &'static str) -> Result<CFDictionaryRef, Error>;
    unsafe fn get_i64(&self, key: &'static str) -> Result<i64, Error>;
    unsafe fn get_string(&self, key: &'static str) -> Result<String, Error>;
    unsafe fn get_bool(&self, key: &'static str) -> Result<bool, Error>;
}

impl KeyVal for CFDictionaryRef {
    unsafe fn get_dict(&self, key: &'static str) -> Result<CFDictionaryRef, Error> {
        let mut stats_dict = std::mem::MaybeUninit::<CFDictionaryRef>::uninit();
        if CFDictionaryGetValueIfPresent(
            self.as_ref().unwrap(),
            CFSTR(key.as_bytes().as_ptr() as *mut i8) as *mut c_void,
            &mut stats_dict as *mut _ as *mut *const c_void,
        ) == 0
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!("get_dict: {} not found in the dict", key),
            ));
        }

        Ok(stats_dict.assume_init())
    }

    unsafe fn get_i64(&self, key: &'static str) -> Result<i64, Error> {
        let mut nbr = 0i64;
        let mut _nbr = std::mem::MaybeUninit::<CFNumberRef>::uninit();
        if CFDictionaryGetValueIfPresent(
            self.as_ref().unwrap(),
            CFSTR(key.as_bytes().as_ptr() as *mut i8) as *mut c_void,
            _nbr.as_mut_ptr() as *mut *const c_void,
        ) == 0
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!("get_i64: {} not found in the stats_dict", key),
            ));
        }
        let number = _nbr.assume_init();
        CFNumberGetValue(number, 4, &mut nbr as *mut _ as *mut c_void);

        Ok(nbr)
    }

    unsafe fn get_string(&self, key: &'static str) -> Result<String, Error> {
        let mut str_ref = std::mem::MaybeUninit::<CFStringRef>::uninit();
        if CFDictionaryGetValueIfPresent(
            self.as_ref().unwrap(),
            CFSTR(key.as_bytes().as_ptr() as *mut i8) as *mut c_void,
            str_ref.as_mut_ptr() as *mut *const c_void,
        ) == 0
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!("get_string: {} not found in the parent_dict", key),
            ));
        }
        let str_ref = str_ref.assume_init();
        // Max 64 char long
        let mut name = [0i8; 64];
        // I forgot why 134217984, sorry... (but seems to work)
        if CFStringGetCString(str_ref, name.as_mut_ptr(), 64, 134217984) == 0 {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "Cannot get the buffer filled to transform the key({}) to String",
                    key
                ),
            ));
        }

        Ok(to_str(name.as_ptr()).to_owned())
    }

    unsafe fn get_bool(&self, key: &'static str) -> Result<bool, Error> {
        let mut _ref = std::mem::MaybeUninit::<CFBooleanRef>::uninit();
        if CFDictionaryGetValueIfPresent(
            self.as_ref().unwrap(),
            CFSTR(key.as_bytes().as_ptr() as *mut i8) as *mut c_void,
            _ref.as_mut_ptr() as *mut *const c_void,
        ) == 0
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!("get_bool: {} not found in the parent_dict", key),
            ));
        }

        Ok(CFBooleanGetValue(_ref.assume_init()))
    }
}
