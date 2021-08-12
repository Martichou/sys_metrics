use crate::virt::{err_not_found, Virtualization};

use std::{
    fs::File,
    io::{Error, Read},
};

pub fn detect_vm_dmi() -> Result<Virtualization, Error> {
    let probing = [
        "/sys/class/dmi/id/product_name",
        "/sys/class/dmi/id/sys_vendor",
        "/sys/class/dmi/id/board_vendor",
        "/sys/class/dmi/id/bios_vendor",
    ];

    for path in probing {
        let mut file = File::open(path)?;
        let mut buf = [0u8; 3];
        file.read_exact(&mut buf)?;

        match buf {
            [88, 101, 110] => return Ok(Virtualization::Xen),
            [75, 86, 77] => return Ok(Virtualization::Kvm),
            [81, 69, 77] => return Ok(Virtualization::Qemu),
            [86, 77, 119] | [86, 77, 87] => return Ok(Virtualization::Vmware),
            [105, 110, 110] => return Ok(Virtualization::Oracle),
            [66, 111, 99] => return Ok(Virtualization::Bochs),
            [80, 97, 114] => return Ok(Virtualization::Parallels),
            [66, 72, 89] => return Ok(Virtualization::Bhyve),
            _ => continue,
        }
    }

    Err(err_not_found())
}
