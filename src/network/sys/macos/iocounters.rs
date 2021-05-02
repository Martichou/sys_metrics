// Based on https://github.com/heim-rs/heim/blob/master/heim-net/src/sys/macos/bindings.rs
use crate::binding::{if_msghdr2, if_msghdr_partial};
use crate::network::IoCounters;

use libc::sysctl;
use std::io::Error;
use std::mem;
use std::ptr;

#[derive(Debug)]
struct Routes {
    position: usize,
    data: Vec<u8>,
}

impl Iterator for Routes {
    type Item = if_msghdr2;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.position == self.data.len() {
                return None;
            }

            let data_ptr = unsafe { self.data.as_ptr().add(self.position) };

            // In order not to read uninitialized memory (leading to heap-buffer-overflow),
            // which might happen if the whole `libc::if_msghdr` struct would be used here,
            // we are going to read as small as possible bytes amount
            // and see if that would be enough to determine the `ifm_type`
            assert!(
                self.position + mem::size_of::<if_msghdr_partial>() < self.data.len(),
                "Not enough data to read the `if_msghdr` header, need at least {} bytes, got {}",
                mem::size_of::<if_msghdr_partial>(),
                self.data.len() - self.position,
            );

            let hdr = unsafe {
                let mut maybe_hdr = mem::MaybeUninit::<if_msghdr_partial>::uninit();
                ptr::copy_nonoverlapping(
                    data_ptr,
                    maybe_hdr.as_mut_ptr() as *mut u8,
                    mem::size_of::<if_msghdr_partial>(),
                );
                maybe_hdr.assume_init()
            };
            debug_assert!(hdr.ifm_msglen as usize <= self.data.len() + self.position);

            self.position += hdr.ifm_msglen as usize;

            if libc::c_int::from(hdr.ifm_type) == libc::RTM_IFINFO2 {
                let hdr = unsafe {
                    let mut maybe_hdr = mem::MaybeUninit::<if_msghdr2>::uninit();
                    ptr::copy_nonoverlapping(
                        data_ptr,
                        maybe_hdr.as_mut_ptr() as *mut u8,
                        mem::size_of::<if_msghdr2>(),
                    );
                    maybe_hdr.assume_init()
                };

                // Just in case to be sure that copying worked properly
                debug_assert!(libc::c_int::from(hdr.ifm_type) == libc::RTM_IFINFO2);

                return Some(hdr);
            } else {
                continue;
            }
        }
    }
}

fn net_pf_route() -> Result<Routes, Error> {
    let mut name: [libc::c_int; 6] = [libc::CTL_NET, libc::PF_ROUTE, 0, 0, libc::NET_RT_IFLIST2, 0];
    let mut length: libc::size_t = 0;

    if unsafe {
        sysctl(
            name.as_mut_ptr(),
            6,
            ptr::null_mut(),
            &mut length,
            ptr::null_mut(),
            0,
        )
    } < 0
    {
        return Err(Error::last_os_error());
    }

    let mut data: Vec<u8> = Vec::with_capacity(length);
    if unsafe {
        sysctl(
            name.as_mut_ptr(),
            6,
            data.as_mut_ptr() as *mut libc::c_void,
            &mut length,
            ptr::null_mut(),
            0,
        )
    } < 0
    {
        return Err(Error::last_os_error());
    }
    // Why is this needed ?
    // -> Seems like as we use a call to sysctl, the call will fill the buffer
    //    with various values at various index. The data.len() result as 0.
    //    As we know the values has been set, we override the len().
    unsafe { data.set_len(length) };
    Ok(Routes { position: 0, data })
}

/// Return the [IoCounters] struct.
///
/// [IoCounters]: ../network/struct.IoCounters.html
pub fn get_net_iocounters() -> Result<Vec<IoCounters>, Error> {
    net_pf_route()?
        .into_iter()
        .map(|msg: if_msghdr2| {
            let mut name: [u8; libc::IF_NAMESIZE] = [0; libc::IF_NAMESIZE];
            if unsafe {
                libc::if_indextoname(msg.ifm_index.into(), name.as_mut_ptr() as *mut libc::c_char)
            }
            .is_null()
            {
                return Err(Error::last_os_error());
            }
            let first_nul = name.iter().position(|c| *c == b'\0').unwrap_or(0);
            let name = String::from_utf8_lossy(&name[..first_nul]).to_string();

            Ok(IoCounters {
                interface: name,
                rx_bytes: msg.ifm_data.ifi_ibytes as u64,
                rx_packets: msg.ifm_data.ifi_ipackets as u64,
                rx_errs: msg.ifm_data.ifi_ierrors as u64,
                rx_drop: msg.ifm_data.ifi_iqdrops as u64,
                tx_bytes: msg.ifm_data.ifi_obytes as u64,
                tx_packets: msg.ifm_data.ifi_opackets as u64,
                tx_errs: msg.ifm_data.ifi_oerrors as u64,
                tx_drop: msg.ifm_snd_drops as u64, // Not sure about this one, can't find enough doc
            })
        })
        .collect()
}
