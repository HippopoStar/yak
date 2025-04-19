
#![no_std]

mod vga;

use core::fmt::Write;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
	// Meant to be the only occurrence in which screen 0 is allowed
	let _result: core::fmt::Result = writeln!(vga::_VGA.get_screen(0), "\n\n{}", info);
	loop {}
}

fn print_rainbow_42(screen_index: usize) -> () {
	let str_42 = "
        :::      ::::::::
      :+:      :+:    :+:
    +:+ +:+         +:+
  +#+  +:+       +#+
+#+#+#+#+#+   +#+
     #+#    #+#
    ###   ########.fr";

	let mut screen: spin::MutexGuard<vga::screen::Screen> = vga::_VGA.get_screen(screen_index);
	screen.set_color(vga::Color::Black);
	for line in str_42.lines() {
		writeln!(screen, "{}", line).unwrap();
		screen.set_next_rainbow_color();
	}
}

#[no_mangle]
pub extern "C" fn rust_main() {
	// ATTENTION: we have a very small stack and no guard page

	print_rainbow_42(2);

	write!(vga::_VGA.get_screen(2), "$> ").unwrap();
	vga::_VGA.set_display(2);

	loop {}
}

