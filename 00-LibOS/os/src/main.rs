#![no_main]
#![no_std]

#[macro_use]
mod console;

mod panic;
mod sbi;

mod logger;

use core::arch::global_asm;
use log::{debug, error, info, trace, warn, LevelFilter};

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    logger::init(LevelFilter::Trace).unwrap();

    error!(".error");
    warn!(".warn");
    info!(".info");
    debug!(".debug");
    trace!(".trace");

    println!("Hello, world!");
    panic!("Shutdown machine!");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}
