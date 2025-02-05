#![no_std]
#![no_main]

mod writer;
use writer::FrameBufferWriter;

use bootloader_api::config::Mapping;
use x86_64::instructions::hlt;
use core::fmt::Write;

// Kernel Memory Management.
pub static BOOTLOADER_CONFIG: bootloader_api::BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config.kernel_stack_size = 100 * 1024; // 100 KiB
    config
};

bootloader_api::entry_point!(my_entry_point, config = &BOOTLOADER_CONFIG);

static mut FRAME_BUFFER_WRITER: Option<FrameBufferWriter> = None;

struct Cursor {
    x: usize,
    y: usize,
}

static mut CURSOR: Cursor = Cursor { x: 0, y: 0 };

macro_rules! print {
    ($fmt:expr) => {
        unsafe {
            if let Some(ref mut writer) = FRAME_BUFFER_WRITER {
                write!(writer, "{}", $fmt).unwrap(); 
                CURSOR.x += 1; // Move cursor to the right after each character
            }
        }
    };
}

// Move text right function
fn move_text_right(steps: usize) {
    unsafe {
        if let Some(ref mut writer) = FRAME_BUFFER_WRITER {
            CURSOR.x += steps;
            writer.set_cursor_position(CURSOR.x, CURSOR.y);
        }
    }
}

// Move text left function
fn move_text_left(steps: usize) {
    unsafe {
        if let Some(ref mut writer) = FRAME_BUFFER_WRITER {
            if CURSOR.x >= steps {
                CURSOR.x -= steps;
            } else {
                CURSOR.x = 0;
            }
            writer.set_cursor_position(CURSOR.x, CURSOR.y);
        }
    }
}

fn my_entry_point(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    let frame_buffer_info = boot_info.framebuffer.as_mut().unwrap().info();
    let buffer = boot_info.framebuffer.as_mut().unwrap().buffer_mut();
    let frame_buffer_writer = FrameBufferWriter::new(buffer, frame_buffer_info);

    unsafe {
        FRAME_BUFFER_WRITER = Some(frame_buffer_writer);
    }

    print!("Hi, George!");
    print!("\nThis is Blessing's project.");
    print!("\n\\cRed text\\r \tIndented Text");

    let mut cursor_visible = true;
    loop {
        // Move text dynamically (for testing)
        move_text_right(2); // Move text right by 2 steps
        for _ in 0..500_000 {
            x86_64::instructions::hlt();
        }

        move_text_left(2); // Move text back to the left
        for _ in 0..500_000 {
            x86_64::instructions::hlt();
        }

        // Toggle cursor visibility
        cursor_visible = !cursor_visible;
        unsafe {
            if let Some(ref mut writer) = FRAME_BUFFER_WRITER {
                if cursor_visible {
                    writer.draw_cursor();
                } else {
                    writer.clear_cursor();
                }
            }
        }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        hlt();
    }
}
