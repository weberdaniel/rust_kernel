// normally compiler would warn us about unused enum
#[allow(dead_code)]
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
  chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
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
        self.buffer.chars[row][col] = ScreenChar {
          ascii_character: byte,
          color_code,
        };
        self.column_position += 1;
      }
    }
  }

  fn new_line(&mut self) { /*TODO*/}
