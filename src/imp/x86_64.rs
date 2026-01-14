use crate::Uname;

const V2_LEAF_1_ECX: u32 = (1 << 0) | (1 << 9) | (1 << 13) | (1 << 19) | (1 << 20) | (1 << 23);
const V2_EXT_LEAF_1_ECX: u32 = 1 << 0;

const V3_LEAF_1_ECX: u32 = (1 << 12) | (1 << 22) | (1 << 26) | (1 << 28) | (1 << 29);
const V3_LEAF_7_0_EBX: u32 = (1 << 3) | (1 << 5) | (1 << 8);
const V3_EXT_LEAF_1_ECX: u32 = 1 << 5;

const V4_LEAF_7_0_EBX: u32 = (1 << 16) | (1 << 17) | (1 << 28) | (1 << 30) | (1 << 31);

const VERSION_CHECK_ARRAY: &[[u32; 3]] = &[
    [0, 0, 0],
    [V2_LEAF_1_ECX, V2_EXT_LEAF_1_ECX, 0],
    [V3_LEAF_1_ECX, V3_EXT_LEAF_1_ECX, V3_LEAF_7_0_EBX],
    [0, 0, V4_LEAF_7_0_EBX],
];

pub fn populate_hardware_platform(x: &mut Uname) {
    let leaf_1 = core::arch::x86_64::__cpuid(0x01);
    let ext_leaf_1 = core::arch::x86_64::__cpuid(0x8000_0001);
    let leaf_7_0 = core::arch::x86_64::__cpuid_count(0x07, 0x00);

    let mut version = VERSION_CHECK_ARRAY.len();

    for (v, &check) in core::iter::zip(0.., VERSION_CHECK_ARRAY) {
        if !core::iter::zip([leaf_1.ecx, ext_leaf_1.ecx, leaf_7_0.ebx], check)
            .all(|(actual, expected)| (actual & expected) == expected)
        {
            version = v;
            break;
        }
    }

    x.hardware_platform = alloc::format!("x86_64-v{version}");
}
