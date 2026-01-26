use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use lilium_sys::sys::{
    error::INSUFFICIENT_LENGTH,
    info::{self, GetSystemInfo, arch_info},
    kstr::{KSlice, KStrPtr},
    option::OPTION_FLAG_IGNORE,
};

use crate::Uname;

pub fn populate_uname(v: &mut Uname) -> Result<(), isize> {
    let mut os_vendor = String::with_capacity(32);
    let mut kernel_vendor = String::with_capacity(32);
    let mut computer_name = String::with_capacity(32);

    let mut requests = [
        info::SysInfoRequest {
            os_version: info::SysInfoRequestOsVersion::INIT,
        },
        info::SysInfoRequest {
            kernel_vendor: info::SysInfoRequestKernelVendor::INIT,
        },
        info::SysInfoRequest {
            computer_name: info::SysInfoRequestComputerName::INIT,
        },
        info::SysInfoRequest {
            arch_info: info::SysInfoRequestArchInfo::INIT,
        },
    ];

    unsafe { requests[2].head.flags |= OPTION_FLAG_IGNORE };

    let os_version_string = unsafe { &raw mut (requests[0].os_version.osvendor_name) };
    let kernel_version_string = unsafe { &raw mut requests[1].kernel_vendor.kvendor_name };
    let computer_name_string = unsafe { &raw mut requests[2].computer_name.sys_display_name };
    let computer_name_flags_ignore = unsafe { &raw const requests[2].head.flags };

    let strings_array = [
        (&mut os_vendor, os_version_string, core::ptr::null()),
        (&mut kernel_vendor, kernel_version_string, core::ptr::null()),
        (
            &mut computer_name,
            computer_name_string,
            computer_name_flags_ignore,
        ),
    ];
    let mut dirty = true;
    while dirty {
        dirty = false;
        for (string, sptr, _) in strings_array {
            let sptr = unsafe { &mut *sptr };
            sptr.len = string.capacity();
            sptr.str_ptr = string.as_mut_ptr();
        }

        let res = unsafe { GetSystemInfo(KSlice::from_slice_mut(&mut requests)) };

        if res == 0 {
            break;
        } else if res == INSUFFICIENT_LENGTH {
            for (string, sptr) in strings_array {
                if string.capacity() < unsafe { (*sptr).len } {
                    string.reserve(unsafe { (*sptr).len });
                    dirty = true;
                }
            }
        } else {
            return Err(res);
        }
    }

    for (string, sptr, ignore) in strings_array {
        if let Some(flags) = unsafe { ignore.as_ref() }
            && (flags & OPTION_FLAG_IGNORE) != 0
        {
            continue;
        }
        unsafe { string.as_mut_vec().set_len((*sptr).len) }
    }

    v.sysname = os_vendor;
    v.kernel_name = "Lilium".to_string();
    v.nodename = computer_name;

    let osinfo = unsafe { requests[0].os_version };

    let kvendor = unsafe { requests[1].kernel_vendor };

    let arch_info = unsafe { requests[3].arch_info };

    v.kernel_release = alloc::format!(
        "{}.{} ({} {}.{})",
        osinfo.os_major,
        osinfo.os_minor,
        kernel_vendor,
        kvendor.kernel_major,
        kvendor.kernel_minor
    );

    v.kernel_version = alloc::format!(
        "{} {}.{}-{}",
        kernel_vendor,
        kvendor.kernel_major,
        kvendor.kernel_minor,
        kvendor.build_id
    );

    match arch_info.arch_type {
        arch_info::ARCH_TYPE_X86_64 => {
            v.machine = "x86_64".to_string();
            if arch_info.arch_version > 1 {
                v.hardware_platform = alloc::format!("x86_64-v{}", arch_info.arch_version);
            } else {
                v.hardware_platform = "x86_64".to_string();
            }
        }
        arch_info::ARCH_TYPE_X86_IA_32 => {
            v.machine = alloc::format!("i{}86", arch_info.arch_version);
            v.hardware_platform = v.machine.clone();
        }
        arch_info::ARCH_TYPE_CLEVER_ISA => {
            v.machine = "clever".to_string();
            v.hardware_platform = alloc::format!("clever1.{}", arch_info.arch_version);
        }
        arch_info::ARCH_TYPE_ARM32 => {
            v.machine = "arm".to_string();
            super::populate_hardware_platform(v); // TODO: We'll check how to format arm versions
        }
        arch_info::ARCH_TYPE_AARCH64 => {
            v.machine = "aarch64".to_string();
            super::populate_hardware_platform(v); // TODO: Same here
        }
        arch_info::ARCH_TYPE_RISCV32 => {
            v.machine = "riscv32".to_string();
            v.hardware_platform = alloc::format!("rva{}u32", arch_info.arch_version);
        }
        arch_info::ARCH_TYPE_RISCV64 => {
            v.machine = "riscv64".to_string();
            v.hardware_platform = alloc::format!("rva{}u64", arch_info.arch_version)
        }
        _ => {
            v.machine = "**UNKNOWN ARCH**".to_string();
            v.hardware_platform =
                alloc::format!("**UNKNOWN ARCH (Version {})**", arch_info.arch_version);
        }
    }
    super::populate_processor(v);

    Ok(())
}
