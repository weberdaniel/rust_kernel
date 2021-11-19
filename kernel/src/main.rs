// main.rs

// we do not include the standard library, since we want to build a bare
// metal binar for an operating system, so we may not depend on threads,
// networking libraries or other things included in libc
#![no_std]

// though one might think main is the entry point, it is not true.
// Most languages have a runtime system, which e.g. provides garbage
// collection. This runtime is called before main usually. Typically
// rust start point is crt0 ("C runtime zero"). Then this C runtime
// invokes entry point of rust runtime. Then it calls the main function.
// We must therefore overwrite the crt0 entry point.
#![no_main] 

// the panic handler is removed when not including the standard library
// so we must implement it on our own
use core::panic::PanicInfo;

// this function loops forever and never returns, so it returns the "never" 
// type denoted by an exclamation mark
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  loop { }
}

// stack unwinding is not possible within a bare metal executable, it 
// requires some complex operating system capabilities, so we must
// remove these capabilities. This is donein the Cargo.toml file

static HELLO: &[u8] = b"Hello World!";

mod vga_buffer;

// implement the starting point

#[no_mangle]
pub extern "C" fn _start() -> ! {
  // this is the place where the vga buffer is located
  //let vga_buffer = 0xb8000 as *mut u8;

  // for each character ..
  //for (i, &byte) in HELLO.iter().enumerate() {
    //unsafe {
      // ... we do an unsafe write to the vga buffer
      //*vga_buffer.offset(i as isize * 2) = byte;
      //*vga_buffer.offset(i as isize * 2 + 1) = 0xb
      // ... though this could be done better later.
    //}
  //}

  // no longer needed
  //vga_buffer::print_something();
  
  // no longer needed
  // use core::fmt::Write;
  // vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
  // write!(vga_buffer::WRITER.lock(), 
         // ", some numbers:  {} {}", 42, 1.337).unwrap();
         
  println!("Hello World{}", "!"); 

  loop { }
}

fn main() { }
