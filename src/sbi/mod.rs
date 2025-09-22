use core::{arch::asm, num::NonZeroIsize};

pub mod base;
pub mod legacy;

#[repr(isize)]
pub enum SbiErrorType {
    Failed = -1,
    NotSupported = -2,
    InvalidParam = -3,
    Denied = -4,
    InvalidAddress = -5,
    AlreadyAvailable = -6,
    AlreadyStarted = -7,
    AlreadyStopped = -8,
    NoSHMEM = -9,
    InvalidState = -10,
    BadRange = -11,
    Timeout = -12,
    IO = -13,
    DeniedLocked = -14,
}

#[derive(Debug)]
pub struct SbiError(Option<NonZeroIsize>);

impl SbiError {
    pub fn new(error: isize) -> Self {
        Self(NonZeroIsize::new(error))
    }
}

pub type SbiResult<T> = Result<T, SbiError>;

#[repr(C)]
pub struct HartMask {
    base: usize,
    mask: usize,
}

/// Zero argument call to sbi
pub unsafe fn call_sbi0(extension_id: usize, function_id: usize) -> Result<usize, SbiError> {
    let error: isize;
    let value: usize;
    unsafe {
        asm!("ecall",
            in("a7") extension_id,
            in("a6") function_id,
            lateout("a0") error,
            lateout("a1") value);
    }
    match error {
        0 => Ok(value),
        _ => Err(SbiError::new(error)),
    }
}

/// One argument call to sbi
pub unsafe fn call_sbi1(
    extension_id: usize,
    function_id: usize,
    arg0: usize,
) -> Result<usize, SbiError> {
    let error: isize;
    let value: usize;
    unsafe {
        asm!("ecall",
            in("a7") extension_id,
            in("a6") function_id,
            inlateout("a0") arg0 => error,
            lateout("a1") value);
    }
    match error {
        0 => Ok(value),
        _ => Err(SbiError::new(error)),
    }
}

/// Two argument call to sbi
pub unsafe fn call_sbi2(
    extension_id: usize,
    function_id: usize,
    arg0: usize,
    arg1: usize,
) -> Result<usize, SbiError> {
    let error: isize;
    let value: usize;
    unsafe {
        asm!("ecall",
            in("a7") extension_id,
            in("a6") function_id,
            inlateout("a0") arg0 => error,
            inlateout("a1") arg1 => value);
    }
    match error {
        0 => Ok(value),
        _ => Err(SbiError::new(error)),
    }
}

/// Three argument call to sbi
pub unsafe fn call_sbi3(
    extension_id: usize,
    function_id: usize,
    arg0: usize,
    arg1: usize,
    arg2: usize,
) -> Result<usize, SbiError> {
    let error: isize;
    let value: usize;
    unsafe {
        asm!("ecall",
            in("a7") extension_id,
            in("a6") function_id,
            inlateout("a0") arg0 => error,
            inlateout("a1") arg1 => value,
            in("a2") arg2,
        );
    }
    match error {
        0 => Ok(value),
        _ => Err(SbiError::new(error)),
    }
}

/// Four argument call to sbi
pub unsafe fn call_sbi4(
    extension_id: usize,
    function_id: usize,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
) -> Result<usize, SbiError> {
    let error: isize;
    let value: usize;
    unsafe {
        asm!("ecall",
            in("a7") extension_id,
            in("a6") function_id,
            inlateout("a0") arg0 => error,
            inlateout("a1") arg1 => value,
            in("a2") arg2,
            in("a3") arg3
        );
    }
    match error {
        0 => Ok(value),
        _ => Err(SbiError::new(error)),
    }
}

/// Five argument call to sbi
pub unsafe fn call_sbi5(
    extension_id: usize,
    function_id: usize,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
) -> Result<usize, SbiError> {
    let error: isize;
    let value: usize;
    unsafe {
        asm!("ecall",
            in("a7") extension_id,
            in("a6") function_id,
            inlateout("a0") arg0 => error,
            inlateout("a1") arg1 => value,
            in("a2") arg2,
            in("a3") arg3,
            in("a4") arg4,
        );
    }
    match error {
        0 => Ok(value),
        _ => Err(SbiError::new(error)),
    }
}

/// Six argument call to sbi
pub unsafe fn call_sbi6(
    extension_id: usize,
    function_id: usize,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
) -> Result<usize, SbiError> {
    let error: isize;
    let value: usize;
    unsafe {
        asm!("ecall",
            in("a7") extension_id,
            in("a6") function_id,
            inlateout("a0") arg0 => error,
            inlateout("a1") arg1 => value,
            in("a2") arg2,
            in("a3") arg3,
            in("a4") arg4,
            in("a5") arg5
        );
    }
    match error {
        0 => Ok(value),
        _ => Err(SbiError::new(error)),
    }
}
