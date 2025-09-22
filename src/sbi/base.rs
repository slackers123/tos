use crate::sbi::{SbiResult, call_sbi0, call_sbi1};

pub fn get_spec_version() -> SbiResult<(usize, usize)> {
    unsafe { call_sbi0(0x10, 0) }.map(|v| {
        let major = v >> 24;
        let minor = v & 0xFFFFFF;
        (major, minor)
    })
}

pub fn get_impl_id() -> SbiResult<usize> {
    unsafe { call_sbi0(0x10, 1) }
}

pub fn get_impl_version() -> SbiResult<usize> {
    unsafe { call_sbi0(0x10, 2) }
}

pub fn probe_extension(extension_id: usize) -> SbiResult<bool> {
    unsafe { call_sbi1(0x10, 3, extension_id) }.map(|v| v != 0)
}

pub fn get_mvendroid() -> SbiResult<usize> {
    unsafe { call_sbi0(0x10, 4) }
}

pub fn get_marchid() -> SbiResult<usize> {
    unsafe { call_sbi0(0x10, 5) }
}

pub fn get_mimpid() -> SbiResult<usize> {
    unsafe { call_sbi0(0x10, 6) }
}
