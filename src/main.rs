<<<<<<< HEAD
#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod framebuffer; // Import the framebuffer module

// The entry point of the kernel
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut writer = framebuffer::FrameBufferWriter::new(framebuffer::ColorCode::new(0x0f, 0x00)); // white foreground, black background

    // Print a hello world message
    writer.write_string("Hello, Kernel World!\n");

    // Print a message with a newline and color change
    writer.set_color(framebuffer::ColorCode::new(0x04, 0x00)); // red foreground
    writer.write_string("This is a red text!\n");

    // Print a message with tab and newline
    writer.write_string("Here comes a tab:\tIndented Text\n");

    // Simulate screen scrolling (writing a lot of lines to scroll the screen)
    for _ in 0..30 {
        writer.write_string("This line will scroll the screen.\n");
    }

    loop {}
}

// Custom panic handler to prevent the kernel from crashing
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
=======
fn main() {
    println!("Hello, world!");
>>>>>>> 5a3b30d (My first commit)
}
