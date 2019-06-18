#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

extern crate panic_halt;

use riscv_rt::entry;

#[entry]
fn main() -> ! {
    loop { }
}

#[cfg(test)]
fn test_runner(_tests: &[&dyn Fn()]) {
    //println!("Running {} tests", tests.len());
    loop { }
}