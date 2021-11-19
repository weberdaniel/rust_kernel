// normally compiler would warn us about unused enum
#[allow(dead_code)]

use volatile::Volatile;

// use this for the static WRITER implementation, this mutex does not need
// an operating system as a basis
use spin::Mutex;

// use the lazy evaluation fo a static variable, thus initializing it not
// at compile time, but at the time of first use. This enables to cast
// raw pointers into references, which is not possible at compile time.
use lazy_static::lazy_static;

// with this we enable copy semantics for the type and ake it printable
// and comparable
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// normally we would use unsigned 4bit u4, but rust does not support it,
// so we use u8 instead
#[repr(u8)]
pub enum Color {
  Black = 0,
  Blue = 1,
  Green = 2,
  Cyan = 3,
  Red = 4,
  Magenta = 5,
  Brown = 6,
  LightGray = 7,
  DarkGray = 8,
  LightBlue = 9,
  LightGreen = 10,
  LightCyan = 11,
  LightRed = 12,
  Pink = 13,
  Yellow = 14,
  White = 15,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
// the ColorCode struct contains the full color byte, containing
// foreground and background color. To ensure the struct has the
// exact same data layout as u8, we use repr(transparent) attribute.
struct ColorCode(u8);

impl ColorCode {
  fn new(foreground: Color, background: Color) -> ColorCode {
    ColorCode((background as u8) << 4 | (foreground as u8))
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// the field ordering in default structs is undefined in Rust, so we 
// need repr(C) attribute. 
#[repr(C)]
struct ScreenChar {
  ascii_character: u8,
  color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

// For the Buffer struct we use repr(transparent) again to ensure it
// has sae memory layout as its single field
#[repr(transparent)]
struct Buffer {
  chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

// to actually write to the screen. A reference to the VGA buffer is
// stored in buffer. We need an explicit lifetime here to tell the 
// compiler how long the reference is valid. The 'static lifetime 
// tells the compiler that the lifetime is valid for the whole program
// run time.
pub struct Writer {
  column_position: usize,
  color_code: ColorCode,
  buffer: &'static mut Buffer
}


impl Writer {
  pub fn write_byte(&mut self, byte: u8) {
    match byte {
      b'\n' => self.new_line(),
      byte => {
        if self.column_position >= BUFFER_WIDTH {
            self.new_line();
        }
        let row = BUFFER_HEIGHT -1;
        let col = self.column_position;
        let color_code = self.color_code;
        self.buffer.chars[row][col].write(ScreenChar {
          ascii_character: byte,
          color_code,
        });
        self.column_position += 1;
      }
    }
  }

  // rust strings are utf-8 by default, so they might contain
  // bytes that are not supported by VGA text buffer.
  pub fn write_string(&mut self, s: &str) {
    for byte in s.bytes() {
      match byte {
        // printable ASCII byte or newline
        0x20..=0x7e | b'\n' => self.write_byte(byte),
        // not part of printable ASCII range
        _ => self.write_byte(0xfe),
      }
    }
  }

  fn new_line(&mut self) { 
    for row in 1..BUFFER_HEIGHT {
      for col in 0..BUFFER_WIDTH {
        let character = self.buffer.chars[row][col].read();
        self.buffer.chars[row - 1][col].write(character);
      }
    }
    self.clear_row(BUFFER_HEIGHT - 1);
    self.column_position = 0;
  }

  fn clear_row(&mut self, row: usize) { 
    let blank = ScreenChar {
      ascii_character: b' ',
      color_code: self.color_code,
    };
    for col in 0..BUFFER_WIDTH {
      self.buffer.chars[row][col].write(blank);
    }
  }
}

lazy_static! {
  pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
    column_position: 0,
    color_code: ColorCode::new(Color::Yellow, Color::Black),
    buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
  });
}

use core::fmt;
use core::fmt::Write;

impl fmt::Write for Writer {
  fn write_str(&mut self, s: &str) -> fmt::Result {
    self.write_string(s);
    Ok(())
  }
}

// this function creates a writer that points to the VGA buffer
// at 0x8000. First cast integer to a mutable raw pointer, then
// convert it to mutable reference by dereferencing it with *
// and immediately borrow it again trhough &mut. This conversion
// requires an unsafe block. 

pub fn print_something() {
  let mut writer = Writer {
    column_position: 0,
    color_code: ColorCode::new(Color::Yellow, Color::Black),
    buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
  };

  writer.write_byte(b'H');
  writer.write_string("ello ");
  writer.write_string("Wörld!");
  write!(writer, "The numbers are {} and {}", 42, 1.0/3.0).unwrap();
}
