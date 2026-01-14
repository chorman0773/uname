use crate::{Uname, helper::bytes_to_string};

pub fn populate_uname(v: &mut Uname) -> Result<(), ()> {
    let mut name: libc::utsname = unsafe { core::mem::zeroed() };

    if unsafe { libc::uname(&mut name) } < 0 {
        return Err(());
    }

    v.kernel_name = bytes_to_string(&name.sysname);
    v.kernel_release = bytes_to_string(&name.release);
    v.kernel_version = bytes_to_string(&name.version);
    v.machine = bytes_to_string(&name.machine);
    v.nodename = bytes_to_string(&name.nodename);

    super::populate_os_name(v);
    super::populate_processor(v);
    super::populate_hardware_platform(v);

    Ok(())
}
