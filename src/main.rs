#![no_std]
#![no_main]

extern crate panic_halt;

use riscv_rt::entry;

pub fn square(num: i32) -> i32 {
    num * num
}

#[entry]
fn main() -> ! {
    let a = 5;
    let b = square(a);
    loop {
        
    }
}
