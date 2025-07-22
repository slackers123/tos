#![no_std]
#![no_main]

use core::{
    arch::{asm, naked_asm},
    fmt::Write,
};

pub mod uart;

unsafe extern "C" {
    static __bss: *mut u8;
    static __bss_end: *mut u8;
    static __stack_top: *mut u8;
}

#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn boot() {
    naked_asm!("la t0, __stack_top; mv sp, t0; j kernel_main");
}

pub unsafe fn memset(buf: *mut u8, c: u8, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        unsafe {
            *buf.add(i) = c;
        }
        i += 1;
    }

    return buf;
}

macro_rules! print
{
	($($args:tt)+) => ({
			use core::fmt::Write;
			let _ = write!(crate::uart::Uart::new(0x1000_0000), $($args)+);
	});
}

macro_rules! println
{
	() => ({
		print!("\r\n")
	});
	($fmt:expr) => ({
		print!(concat!($fmt, "\r\n"))
	});
	($fmt:expr, $($args:tt)+) => ({
		print!(concat!($fmt, "\r\n"), $($args)+)
	});
}

#[panic_handler]
pub fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    print!("Aborting: ");
    if let Some(p) = info.location() {
        println!("line {}, file {}: {}", p.line(), p.file(), info.message());
    } else {
        println!("no information available.");
    }
    abort();
}

#[unsafe(no_mangle)]
extern "C" fn abort() -> ! {
    loop {
        unsafe {
            asm!("wfi");
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    unsafe { memset(__bss, 0, __bss_end as usize - __bss as usize) };

    uart::Uart::new(0x1000_0000).init();

    println!("Hello, tos!");

    abort()
}
