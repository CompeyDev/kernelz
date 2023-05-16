use core::panic::PanicInfo;

pub fn println(msg: &'static str) {
    let msg_bytes: &[u8] = msg.as_bytes();

    let vga_buf = 0xb;
    let vga_buf_ptr = 0xb8000 as *mut u8;

    for (pos, &byte) in msg_bytes.iter().enumerate() {
        unsafe {
            *vga_buf_ptr.offset(pos as isize * 2) = byte; // set the byte of char
            *vga_buf_ptr.offset(pos as isize * 2 + 1) = vga_buf; // set the next byte to vga buf val
        };
    };
}


#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

pub fn exit() -> ! {
    loop {}
}
