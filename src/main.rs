//main.rs
#![no_std]
#![no_main]

use core::panic::PanicInfo;
mod vga_buffer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
	use core::fmt::Write;
	// vga_buffer::WRITER.lock().write_str("HAHAHAHHA Dio\nZA WARUDDOOOOO !!!\n\n\n\nChipi chipi chapa chapa\n").unwrap();
	write!(vga_buffer::WRITER.lock(), "{}", "Texte plutot basique HAHAH").unwrap();

	loop {}
}