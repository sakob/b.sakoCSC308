use core::ptr;
use volatile::Volatile;
use core::sync::atomic::{AtomicUsize, Ordering};
use core::cell::UnsafeCell;

#[repr(C)]
pub struct ScreenChar {
    ascii_character: u8,
    color_code: u8,
}

impl ScreenChar {
    pub fn new(ascii_character: char, color_code: u8) -> Self {
        ScreenChar {
            ascii_character: ascii_character as u8,
            color_code,
        }
    }

    pub fn to_u16(&self) -> u16 {
        (self.color_code as u16) << 8 | (self.ascii_character as u16)
    }
}

// Wrap in UnsafeCell to allow interior mutability safely
struct VgaBuffer {
    buffer: UnsafeCell<*mut Volatile<u16>>,
}

unsafe impl Sync for VgaBuffer {} // Manually implement Sync since UnsafeCell is not Sync

static VGA_BUFFER: VgaBuffer = VgaBuffer {
    buffer: UnsafeCell::new(0xb8000 as *mut _),
};

pub struct FrameBufferWriter {
    color_code: ColorCode,
    column_position: usize,
}

impl FrameBufferWriter {
    pub fn new(color_code: ColorCode) -> Self {
        FrameBufferWriter {
            color_code,
            column_position: 0,
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        let screen_char = ScreenChar::new(byte as char, self.color_code.0);
        let screen_char_u16 = screen_char.to_u16();

        unsafe {
            let vga_ptr = *VGA_BUFFER.buffer.get(); // Get mutable pointer from UnsafeCell
            (*vga_ptr).write(screen_char_u16);
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
    }

    pub fn set_color(&mut self, color: ColorCode) {
        self.color_code = color;
    }
}

pub struct ColorCode(u8);

impl ColorCode {
    pub fn new(fg: u8, bg: u8) -> Self {
        ColorCode(fg | (bg << 4))
    }
}
