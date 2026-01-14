use core::mem;

use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use crate::Uname;

use windows::Win32::{
    Foundation::ERROR_MORE_DATA,
    System::{
        self,
        SystemInformation::{
            OSVERSIONINFOEXW, PROCESSOR_ARCHITECTURE_ALPHA, PROCESSOR_ARCHITECTURE_ALPHA64,
            PROCESSOR_ARCHITECTURE_AMD64, PROCESSOR_ARCHITECTURE_ARM, PROCESSOR_ARCHITECTURE_ARM64,
            PROCESSOR_ARCHITECTURE_INTEL,
        },
    },
};

use windows::Wdk::System::SystemServices::RtlGetVersion;

pub fn populate_uname(x: &mut Uname) -> Result<(), ()> {
    let mut vstr = Vec::with_capacity(System::WindowsProgramming::MAX_COMPUTERNAME_LENGTH as usize);

    let mut len = vstr.capacity() as u32;
    while let Err(e) = unsafe {
        System::SystemInformation::GetComputerNameExW(
            System::SystemInformation::ComputerNameNetBIOS,
            Some(windows::core::PWSTR::from_raw(vstr.as_mut_ptr())),
            &mut len,
        )
    } {
        if e == ERROR_MORE_DATA.into() {
            vstr.reserve(len as usize);
        } else {
            return Err(());
        }
    }

    unsafe {
        vstr.set_len(len as usize - 1);
    }

    super::populate_os_name(x);
    x.kernel_name = "Windows NT".to_string();

    x.nodename = String::from_utf16_lossy(&vstr);

    let mut sysinfo = unsafe { mem::zeroed() };

    unsafe { System::SystemInformation::GetSystemInfo(&mut sysinfo) };

    let mut osinfo: OSVERSIONINFOEXW = unsafe { mem::zeroed() };
    osinfo.dwOSVersionInfoSize = core::mem::size_of::<OSVERSIONINFOEXW>() as u32;

    if let Err(_e) = unsafe { RtlGetVersion((&raw mut osinfo).cast()) }.ok() {
        return Err(());
    }

    let mach = match unsafe { sysinfo.Anonymous.Anonymous.wProcessorArchitecture } {
        PROCESSOR_ARCHITECTURE_AMD64 => "x86_64",
        PROCESSOR_ARCHITECTURE_ARM => "arm",
        PROCESSOR_ARCHITECTURE_ARM64 => "aarch64",
        PROCESSOR_ARCHITECTURE_ALPHA => "alpha",
        PROCESSOR_ARCHITECTURE_ALPHA64 => "alpha64",
        PROCESSOR_ARCHITECTURE_INTEL => match sysinfo.wProcessorLevel {
            ..3 => unreachable!("Surely"),
            3 => "i386",
            4 => "i486",
            5 => "i586",
            6.. => "i686",
        },
        _ => "**UNKNOWN ARCHITECTURE**",
    };

    x.machine = mach.to_string();

    super::populate_hardware_platform(x);
    super::populate_processor(x);

    x.kernel_release = match (
        osinfo.dwMajorVersion,
        osinfo.dwMinorVersion,
        osinfo.dwBuildNumber,
        osinfo.wProductType,
    ) {
        (5, 0, _, _) => "Windows 2000".to_string(),
        (5, 1, _, _) => "Windows XP".to_string(),
        (5, 2, _, 1) => "Windows XP Professional".to_string(),
        (5, 2, _, _) if (osinfo.wSuiteMask & 0x00008000) != 0 => "Windows Home Server".to_string(),
        (5, 2, _, _) => "Windows Server 2003".to_string(),
        (6, 0, _, 1) => "Windows Vista".to_string(),
        (6, 0, _, _) => "Windows Server 2008".to_string(),
        (6, 1, _, 1) => "Windows 7".to_string(),
        (6, 1, _, _) => "Windows Server 2008 R2".to_string(),
        (6, 2, _, 1) => "Windows 8".to_string(),
        (6, 2, _, _) => "Windows Server 2012".to_string(),
        (6, 3, _, 1) => "Windows 8.1".to_string(),
        (6, 3, _, _) => "Windows Server 2012 R2".to_string(),
        (6, 4, _, _) => "Windows 10 Technical Preview".to_string(),
        (10, 0, ..22000, 1) => "Windows 10".to_string(),
        (10, 0, _, 1) => "Windows 11".to_string(),
        (10, 0, ..17763, _) => "Windows Server 2016".to_string(),
        (10, 0, ..20348, _) => "Windows Server 2019".to_string(),
        (10, 0, ..26100, _) => "Windows Server 2022".to_string(),
        (10, 0, _, _) => "Windows Server 2025".to_string(),
        (major, minor, _, _) => alloc::format!("Unknown Kernel Release {major}.{minor}"),
    };

    let sp_end = osinfo.szCSDVersion.iter().take_while(|&&v| v != 0).count();
    let sp = String::from_utf16_lossy(&osinfo.szCSDVersion[..sp_end]);

    if !sp.is_empty() {
        x.kernel_version = alloc::format!(
            "NT {}.{} (Build {}) {}",
            osinfo.dwMajorVersion,
            osinfo.dwMinorVersion,
            osinfo.dwBuildNumber,
            sp
        );
    } else {
        x.kernel_version = alloc::format!(
            "NT {}.{} (Build {})",
            osinfo.dwMajorVersion,
            osinfo.dwMinorVersion,
            osinfo.dwBuildNumber
        )
    }

    Ok(())
}
