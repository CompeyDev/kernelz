#![no_std]
#![no_main]

mod utils;
use utils::{exit, println};

#[no_mangle]
pub extern "C" fn _start() -> ! {
   println("Hello, world!"); 
   exit();
}
