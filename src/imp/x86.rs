#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::__cpuid;

#[cfg(target_arch = "x86")]
use core::arch::x86::__cpuid;

use alloc::string::ToString;

use crate::Uname;

struct Version {
    family: u32,
    model: u32,
    stepping: u8,
}

fn get_version_info() -> Version {
    let version_info = __cpuid(1).eax;

    let ext_family = (version_info >> 20) & 0xFF;

    let ext_model = (version_info >> 16) & 0xF;

    let raw_family = (version_info >> 8) & 0xF;

    let raw_model = (version_info >> 4) & 0xF;

    let model = if raw_family == 15 || raw_family == 6 {
        raw_model + (ext_model << 4)
    } else {
        raw_model
    };

    let family = if raw_family == 15 {
        raw_family + ext_family
    } else {
        raw_family
    };

    let stepping = (version_info & 0xF) as u8;

    Version {
        family,
        model,
        stepping,
    }
}

pub fn populate_processor(x: &mut Uname) {
    let name = __cpuid(0);

    let name = [name.ebx, name.edx, name.ecx];

    let name: [u8; 12] = bytemuck::cast(name);

    let Version {
        family,
        model,
        stepping,
    } = get_version_info();

    let proc = match (&name, family, model) {
        (b"GenuineIntel", 4, _) => "i486",
        (b"GenuineIntel", 5, 0..=2 | 7 | 9) => "pentium",
        (b"GenuineIntel", 5, 4 | 8) => "pentium-mmx",
        (b"GenuineIntel", 6, 1) => "pentiumpro",
        (b"GenuineIntel", 6, 3 | 5 | 6) => "pentium2",
        (b"GenuineIntel", 6, 7 | 8 | 10 | 11) => "pentium3",
        (b"GenuineIntel", 6, 9 | 13 | 21) => "pentium-m",
        (b"GenuineIntel", 6, 23 | 29) => "core2",
        (b"GenuineIntel", 6, 26 | 30 | 31 | 46) => "nehalem",
        (b"GenuineIntel", 6, 28 | 38) => "bonnel",
        (b"GenuineIntel", 6, 37 | 44 | 47) => "westmere",
        (b"GenuineIntel", 6, 42 | 45) => "sandybridge",
        (b"GenuineIntel", 6, 54 | 55 | 77 | 90 | 93) => "silvermont",
        (b"GenuineIntel", 6, 58 | 62) => "ivybridge",
        (b"GenuineIntel", 6, 60 | 63 | 69 | 70) => "haswell",
        (b"GenuineIntel", 6, 61 | 71 | 79 | 86) => "broadwell",
        (b"GenuineIntel", 6, 78 | 94 | 142 | 158) => "skylake",
        (b"GenuineIntel", 6, 85) if stepping == 7 => "cascadelake",
        (b"GenuineIntel", 6, 85) if stepping == 1 => "cooperlake",
        (b"GenuineIntel", 6, 85) => "skylake-avx512",
        (b"GenuineIntel", 6, 92 | 95) => "goldmont",
        (b"GenuineIntel", 6, 102) => "cannonlake",
        (b"GenuineIntel", 6, 122) => "goldmont-plus",
        (b"GenuineIntel", 6, 125 | 126 | 157) => "icelake-client",
        (b"GenuineIntel", 6, 106 | 108) => "icelake-server",
        (b"GenuineIntel", 6, 134 | 138 | 150 | 156) => "tremont",
        (b"GenuineIntel", 6, 140 | 141) => "tigerlake",
        (b"GenuineIntel", 6, 143) => "sapphirerapids",

        (b"GenuineIntel", 6, 151 | 154) => "alderlake",
        (b"GenuineIntel", 6, 167) => "rocketlake",

        (b"GenuineIntel", 6, 170 | 171 | 172) => "meteorlake",
        (b"GenuineIntel", 6, 173) => "graniterapids",
        (b"GenuineIntel", 6, 174) => "graniterapids-d",
        (b"GenuineIntel", 6, 175) => "sierraforest",
        (b"GenuineIntel", 6, 181 | 197) => "arrowlake",
        (b"GenuineIntel", 6, 183 | 186 | 190 | 160) => "raptorlake",
        (b"GenuineIntel", 6, 198) => "arrowlake-s",
        (b"GenuineIntel", 6, 188 | 189) => "lunarlake",
        (b"GenuineIntel", 6, 204) => "pantherlake",
        (b"GenuineIntel", 6, 207) => "emeraldrapids",
        (b"GenuineIntel", 6, 0xD5) => "wildcatlake",
        (b"GenuineIntel", 6, 221) => "clearwaterforest",

        (b"GenuineIntel", 0x0F, 0..=2) => "pentium4",
        (b"GenuineIntel", 0x0F, 3 | 4) => "prescott",
        (b"GenuineIntel", 0x0F, 6) => "pentium4m",
        (b"GenuineIntel", 18, 1) => "novalake",
        (b"GenuineIntel", 19, 1) => "diamondrapids",
        (b"AuthenticAMD", 4, _) => "i486",
        (b"AuthenticAMD", 5, 0..6) => "i586",
        (b"AuthenticAMD", 5, 6 | 7) => "k6",
        (b"AuthenticAMD", 5, 8) => "k6-2",
        (b"AuthenticAMD", 5, 9 | 13) => "k6-3",
        (b"AuthenticAMD", 6, 0..5) => "athlon",
        (b"AuthenticAMD", 6, _) => "athlon-4",
        // TODO: It may be important to divide this into athlon64, athlon-fx, and optron (plus sse3 counterparts)
        (b"AuthenticAMD", 15, ..32) => "k8",
        (b"AuthenticAMD", 15, _) => "k8-sse3",
        // Yes, there are more than just 10h in this list... but apparently there's nothing until Family 15h
        (b"AuthenticAMD", 16 | 18, _) => "amdfam10",
        (b"AuthenticAMD", 20, _) => "btver1",
        (b"AuthenticAMD", 21, 1) => "bdver1",
        (b"AuthenticAMD", 21, 2 | 16 | 19) => "bdver2",
        (b"AuthenticAMD", 21, 0x30..0x40) => "bdver3",
        (b"AuthenticAMD", 21, 0x60..0x80) => "bdver4",
        (b"AuthenticAMD", 22, _) => "btver2",
        (b"AuthenticAMD", 23, 0..0x30) => "znver1",
        (b"AuthenticAMD", 23, 0x30..) => "znver2",
        (b"AuthenticAMD", 25, ..0x10 | 0x20..0x60) => "znver3",
        (b"AuthenticAMD", 25, _) => "znver4",
        (b"AuthenticAMD", 26, ..0x50 | 0x60..0x80) => "znver5",
        (b"AuthenticAMD", 26, _) => "znver6",
        _ => {
            // Fallback: Either ix86 or x86_64, depending on the current architecture
            &x.machine
        }
    };

    x.processor = proc.to_string();
}

#[cfg(target_arch = "x86")]
pub fn populate_hardware_platform(x: &mut Uname) {
    let Version { family, model, .. } = get_version_info();

    match family {
        ..6 => x.hardware_platform = alloc::format!("i{family}86"),
        6 if model < 0x0F => {
            x.hardware_platform = "i686".to_string();
        }
        _ => {
            x.hardware_platform = "i786".to_string();
        }
    };
}
