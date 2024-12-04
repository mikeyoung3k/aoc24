#![no_main]
#![no_std]
//#![feature(alloc)]
#![feature(alloc_error_handler)]
#![feature(extract_if)]
#![feature(unsigned_signed_diff)]


const BUF_SIZE: usize = 1024;

use panic_halt as _;

use cortex_m_rt::entry;
use cortex_m::{asm,iprintln, Peripherals};

use core::alloc::GlobalAlloc;
use core::alloc::Layout;
use alloc_cortex_m::CortexMHeap;



#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

const HEAP_SIZE: usize = 1024*40; // in bytes

mod day1;
mod day2;
mod day3;
mod day4;

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    asm::bkpt();

    loop {}
}


#[entry]
fn main() -> ! {
    // let heap_start: usize = 0x20001000; // Leave 4K for stack
    // let heap_size: usize = 0xB000;// 44K heap
    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) }
    let mut p = Peripherals::take().unwrap();
    let mut stim = &mut p.ITM.stim[0];

    iprintln!(stim, "Hello, world!");

    // day1::run(&mut stim);
    // day2::run(&mut stim);
    // day3::run(&mut stim);
    day4::run(&mut stim);

    loop {}
}

