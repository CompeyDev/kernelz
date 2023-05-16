#![no_std]
#![no_main]

mod utils;
use utils::{exit, println};

// we quite literally want there to be a `_start` section 
// for the assembler, hence the no_mangle and the name
#[no_mangle]
pub extern fn _start() -> ! {
   println("Hello, world!"); 
   exit();
}
