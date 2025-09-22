use core::arch::asm;

use crate::sbi::HartMask;

pub fn set_timer(stime_value: u64) -> isize {
    let result: isize;
    unsafe {
        asm!("ecall",
            inout("a0") stime_value => result,
            in("a7") 0x00,
        )
    }

    result
}

pub fn console_getchar() -> Option<u8> {
    let result: isize;
    unsafe {
        asm!("ecall",
            out("a0") result,
            in("a7") 0x02
        )
    }

    match result {
        -1 => None,
        _ => Some((result & 0xFF) as u8),
    }
}

pub fn send_ipi(hart_mask: *mut HartMask) -> Result<(), isize> {
    let result: isize;
    unsafe {
        asm!("ecall",
            inout("a0") hart_mask => result,
            in("a7") 0x04
        )
    }

    match result {
        0 => Ok(()),
        e => Err(e),
    }
}

pub fn remote_fence_i(hart_mask: *mut HartMask) -> Result<(), isize> {
    let result: isize;
    unsafe {
        asm!("ecall",
            inout("a0") hart_mask => result,
            in("a7") 0x05
        )
    }

    match result {
        0 => Ok(()),
        e => Err(e),
    }
}

pub fn remote_sfence_vma(hart_mask: *mut HartMask, start: usize, size: usize) -> Result<(), isize> {
    let result: isize;
    unsafe {
        asm!("ecall",
            inout("a0") hart_mask => result,
            in("a1") start,
            in("a2") size,
            in("a7") 0x06
        )
    }

    match result {
        0 => Ok(()),
        e => Err(e),
    }
}

pub fn remote_sfence_vma_asid(
    hart_mask: *mut HartMask,
    start: usize,
    size: usize,
    asid: usize,
) -> Result<(), isize> {
    let result: isize;
    unsafe {
        asm!("ecall",
            inout("a0") hart_mask => result,
            in("a1") start,
            in("a2") size,
            in("a3") asid,
            in("a7") 0x07
        )
    }

    match result {
        0 => Ok(()),
        e => Err(e),
    }
}

pub fn shutdown() -> ! {
    // this already doesn't return
    unsafe {
        asm!("ecall", in("a7") 0x08);
    }
    loop {}
}
