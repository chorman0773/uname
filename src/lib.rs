#![no_std]

extern crate alloc;

use alloc::string::String;

mod helper;

#[derive(Clone, Debug, Hash, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct Uname {
    pub sysname: String,
    pub kernel_name: String,
    pub nodename: String,
    pub kernel_release: String,
    pub kernel_version: String,
    pub machine: String,
    pub processor: String,
    pub hardware_platform: String,
}

impl Uname {
    pub const fn new() -> Uname {
        Self {
            sysname: String::new(),
            kernel_name: String::new(),
            nodename: String::new(),
            kernel_release: String::new(),
            kernel_version: String::new(),
            machine: String::new(),
            processor: String::new(),
            hardware_platform: String::new(),
        }
    }
}

mod imp {
    use alloc::string::ToString;

    use crate::Uname;

    pub fn populate_os_name(x: &mut Uname) {
        x.sysname = core::env!("TARGET_OS").to_string();
    }

    pub fn populate_hardware_platform(x: &mut Uname) {
        cfg_match::cfg_match! {
            target_arch = "x86_64" => x86_64::populate_hardware_platform(x),
            target_arch = "x86" => x86::populate_hardware_platform(x),
            _ => ({
                x.hardware_platform = x.machine.clone();
            })
        }
    }

    pub fn populate_processor(x: &mut Uname) {
        cfg_match::cfg_match! {
            any(target_arch = "x86_64", target_arch = "x86") => x86::populate_processor(x),
            _ => ({
                x.processor = x.machine.clone();
            })
        }
    }

    cfg_match::cfg_match! {
        target_family = "unix" => {
            mod unix;
            pub use unix::*;
        };
        target_os = "lilium" => {
            mod lilium;
            pub use lilium::*;
        };
    }

    #[cfg(target_arch = "x86_64")]
    mod x86_64;

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    mod x86;
}

#[derive(Debug)]
pub struct Error(());

pub fn uname() -> Result<Uname, Error> {
    let mut uname = Uname::new();
    imp::populate_uname(&mut uname).map_err(|_| Error(()))?;

    Ok(uname)
}
