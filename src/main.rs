#![no_std]
#![no_main]

use cobalt_core::entrypoint;
use bootloader::BootInfo;
entrypoint!(kmain);

pub fn kmain(_info: &'static mut BootInfo) -> ! {
    loop {};
}



