#![no_std]
#![deny(missing_docs)]

//!
//!

extern crate alloc;

use alloc::string::String;

mod helper;

/// Contains Information about the current system
#[derive(Clone, Debug, Hash, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct Uname {
    /// The Operating System Name.
    pub sysname: String,
    /// The Kernel Name
    pub kernel_name: String,
    /// The Node name or local system name
    pub nodename: String,
    /// The release version of the kernel
    pub kernel_release: String,
    /// The specific build version of the kernel
    pub kernel_version: String,
    /// The current Host Machine Architecture
    pub machine: String,
    /// The Processor Name
    /// If known, this corresponds to a specific string that can be used with `-march`-like and `-mtune`-like flags in compilers to correspond closely with both feature support and timing information.
    /// Otherwise, it is the same as [`Uname::machine`]
    pub processor: String,
    /// The Hardware Platform.
    ///
    /// This is the specific version of the [`Uname::machine`]. If not known for a given architecture, it is set to be the same as [`Uname::machine`]
    /// On x86_64, this corresponds to the microarchitecture level supported (according to cpuid).
    /// On x86-32 this is one of i386, i486, i586, i686, or i786 (depending on the cpu implemented).
    /// Note that the default implementation for x86-32 (used other than on windows or lilium) will never report i386,
    /// as the default implementation requires the processor to support the cpuid instruction
    ///
    ///
    /// This may be extended to support other architectures in the future
    pub hardware_platform: String,
}

impl Uname {
    /// Constructs a new, blank [`Uname`] instance, with all fields set to empty strings.
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
        target_os = "lilium" => {
            mod lilium;
            pub use lilium::*;
        }
        target_family = "windows" => {
            mod windows;
            pub use windows::*;
        }
        target_family = "unix" => {
            mod unix;
            pub use unix::*;
        }
    }

    #[cfg(target_arch = "x86_64")]
    mod x86_64;

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    mod x86;
}

/// A Generic Error type.
pub type Error = error_repr::Error<error::ErrorKind>;

/// Types for error codes
pub mod error;

/// Determines the complete name of the system.
///
///
/// # Implementation
///
/// This function is implemented in various ways depending on the host OS.
/// On unix-like systems, it calls the `uname(2)` function for most fields.
/// On Lilium, this uses `GetSystemInfo` with the `SysInfoRequestKernelVendor`, `SysInfoRequestOsVersion`, `SysInfoRequestArchInfo`, and `SysInfoRequestProcessorName` to gather information.
///
/// On Windows this uses the following system calls:
/// * `GetComputerNameExW`,
/// * `GetSystemInfoEx`,
/// * `RtlGetVersion`
///
/// # Errors
/// Returns an [`Error`] if computing the system name fails for some reason (this shouldn't happen except on unsupported targets)
///
pub fn uname() -> Result<Uname, Error> {
    let mut uname = Uname::new();
    imp::populate_uname(&mut uname).map_err(|v| Error::from_raw_os_error(v))?;

    Ok(uname)
}
