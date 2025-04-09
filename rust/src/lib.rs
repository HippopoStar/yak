
#![no_std]

mod vga;

use core::fmt::Write;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
	loop {}
}

fn print_rainbow_42(vga: &mut vga::VGA) -> () {
	let str_42 = "
        :::      ::::::::
      :+:      :+:    :+:
    +:+ +:+         +:+
  +#+  +:+       +#+
+#+#+#+#+#+   +#+
     #+#    #+#
    ###   ########.fr";

	vga.set_color(vga::Color::Black);
	for line in str_42.lines() {
		writeln!(vga, "{}", line).unwrap();
		vga.set_next_rainbow_color();
	}
}

#[no_mangle]
pub extern "C" fn rust_main() {
	// ATTENTION: we have a very small stack and no guard page

	let mut vga = vga::VGA::new();
	print_rainbow_42(&mut vga);

	let vga_writer: &mut dyn core::fmt::Write = &mut vga;
	write!(vga_writer, "$> ").unwrap();

	loop {}
}

