
#![no_std]

mod vga;

use core::fmt::Write;
use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}

#[no_mangle]
pub extern "C" fn rust_main() {
	// ATTENTION: we have a very small stack and no guard page

	let hello = b"Hello World!";
	let color_byte = 0x2f; // white foreground, green background

	let mut hello_colored = [color_byte; 24];
	for (i, char_byte) in hello.into_iter().enumerate() {
		hello_colored[i*2] = *char_byte;
	}

	// write `Hello World!` to the center of the VGA text buffer
	let buffer_ptr = (0xb8000 + 1988) as *mut _;
	unsafe { *buffer_ptr = hello_colored };

	let str_42 = "
        :::      ::::::::
      :+:      :+:    :+:
    +:+ +:+         +:+
  +#+  +:+       +#+
+#+#+#+#+#+   +#+
     #+#    #+#
    ###   ########.fr";
	let mut vga = vga::VGABuffer::new();
	writeln!(&mut vga, "{}", &str_42).unwrap();
	for i in 0..17 {
		writeln!(vga, "Patatra {:02}", i).unwrap();
	}

	loop {}
}

