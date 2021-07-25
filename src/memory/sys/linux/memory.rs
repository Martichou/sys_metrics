use crate::memory::Memory;

use std::{
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind},
};

/// Return the [Memory] struct.
///
/// It will get the info from `/proc/meminfo`.
///
/// Note that `used` is computed from Total, Free, Buffers and Cached (which is Cached + SReclaimable).
///
/// [Memory]: ../memory/struct.Memory.html
pub fn get_memory() -> Result<Memory, Error> {
    let file = File::open("/proc/meminfo")?;
    let mut file = BufReader::with_capacity(2048, file);

    let mut matched_lines = 0u8;
    let mut memory = Memory::default();
    let mut line = String::with_capacity(64);
    while file.read_line(&mut line)? != 0 {
        // We only need 6 values which can be detected by their 4first bytes
        let first_bytes = &line.as_bytes()[..4];
        match first_bytes {
            b"MemT" | b"MemF" | b"Buff" | b"Cach" | b"Shme" | b"SRec" => {}
            _ => {
                line.clear();
                continue;
            }
        }

        // Split the line at the : separator
        let mut parts = line.splitn(2, ':');
        // Check if if the value we search is the splitted one
        // if so, return a pointer to the memory zone we'll modify.
        let field = match parts.next() {
            Some("MemTotal") => &mut memory.total,
            Some("MemFree") => &mut memory.free,
            Some("Buffers") => &mut memory.buffers,
            Some("Cached") => &mut memory.cached,
            Some("SReclaimable") => &mut memory.cached,
            Some("Shmem") => &mut memory.shared,
            _ => {
                line.clear();
                continue;
            }
        };

        // Get the value part
        match parts.next() {
            Some(value) => {
                // Increment the field we previously got (pointer)
                *field += {
                    // Trim to only get the bytes value
                    let kbytes = match value.trim_start().splitn(2, ' ').next() {
                        Some(kkbytes) => kkbytes.parse::<u64>().unwrap(),
                        None => {
                            line.clear();
                            continue;
                        }
                    };
                    matched_lines += 1;
                    kbytes / 1024
                }
            }

            None => {
                line.clear();
                continue;
            }
        }

        // If we've found all our information, we can return.
        if matched_lines == 6 {
            return Ok(memory.set_used());
        }

        line.clear();
    }

    Err(Error::new(
        ErrorKind::Other,
        "Couldn't get the memory information",
    ))
}
