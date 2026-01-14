use alloc::string::String;
use core::ffi::CStr;

pub fn bytes_to_string<const N: usize>(name: &[i8; N]) -> String {
    CStr::from_bytes_until_nul(bytemuck::bytes_of(name))
        .unwrap()
        .to_string_lossy()
        .into()
}
