
#![no_std]
#![feature(abi_x86_interrupt)]

pub mod arch;
mod interrupts;
mod vga;
mod keyboard;

// https://os.phil-opp.com/hardware-interrupts/#the-hlt-instruction
pub fn hlt_loop() -> ! {
	loop {
		arch::x86::instructions::hlt();
	}
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
	// Meant to be the only occurrence in which screen 0 is allowed
	let _result: core::fmt::Result = kwriteln!(0, "\n\n{}", info); // TODO: write on serial port
	hlt_loop();
}

// fn print_rainbow_42(screen_index: usize) -> () {
// 	let str_42 = "
//         :::      ::::::::
//       :+:      :+:    :+:
//     +:+ +:+         +:+
//   +#+  +:+       +#+
// +#+#+#+#+#+   +#+
//      #+#    #+#
//     ###   ########.fr";

// 	let mut screen: spin::MutexGuard<vga::screen::Screen> = vga::_VGA.get_screen(screen_index);
// 	screen.set_color(vga::Color::Black);
// 	for line in str_42.lines() {
// 		writeln!(screen, "{}", line).unwrap();
// 		screen.set_next_rainbow_color();
// 	}
// }

fn init() {
	interrupts::init_idt();
	unsafe { PICS.lock().initialize() }; // new

	use core::arch::asm;
	unsafe {
		asm!("sti", options(preserves_flags, nostack));
	}
}

use arch::x86::pic_8259::ChainedPics;
use spin;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> = spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

#[no_mangle]
pub extern "C" fn rust_main(n: u32) {
	// ATTENTION: we have a very small stack and no guard page

	vga::_VGA.set_display(7);
	kprintln!("\n{}", n).unwrap();
	init();

	// print_rainbow_42(7);

	kprint!("$> ").unwrap();
	kprint!("\nThe END").unwrap();

	vga::_VGA.set_display(6);
	kprint!(
		"\
		aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
		bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb\
		cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc\
		dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd\
		eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee\
		ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
		gggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg\
		hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh\
		iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii\
		jjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjj\
		kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk\
		llllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllll\
		mmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm\
		nnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnn\
		oooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo\
		pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp\
		qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq\
		rrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrr\
		ssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssss\
		tttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttt\
		uuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuu\
		vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv\
		wwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwww\
		xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\
		yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy\
		zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz\
		"
	).unwrap();
	arch::x86::instructions::interrupts::int3();

	// hlt_loop();
	loop {}
}

