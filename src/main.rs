#![no_std]
#![no_main]

extern crate panic_halt;

use riscv_rt::entry;
use core::panic::PanicInfo;

pub fn square(num: i32) -> i32 {
    num * num
}

#[entry]
fn main() -> ! {
    loop {}
}
