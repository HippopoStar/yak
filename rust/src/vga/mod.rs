
pub mod screen;

use core::fmt::Write;
use self::screen::Screen;
use crate::arch::x86::instructions::{interrupts, port::Port};

lazy_static::lazy_static! {
	pub static ref _VGA: VGA = VGA::new();
}

// ===== Macros =====

// https://os.phil-opp.com/vga-text-mode/#a-println-macro
// https://os.phil-opp.com/hardware-interrupts/#deadlocks

// https://doc.rust-lang.org/stable/core/macro.writeln.html
// https://doc.rust-lang.org/stable/std/macro.println.html

// ===== print! =====

#[macro_export]
macro_rules! vga_print {
	($($arg:tt)*) => {
		$crate::vga::_print(core::format_args!($($arg)*))
	};
}

#[macro_export]
macro_rules! vga_println {
	() => {
		$crate::vga_print!("\n")
	};
	($($arg:tt)*) => {
		$crate::vga_print!("{}\n", core::format_args!($($arg)*))
	};
}

/// Prints the given formatted string to the VGA text buffer
/// through the global `WRITER` instance.
#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) -> core::fmt::Result {
	let mut result: core::fmt::Result = Err(core::fmt::Error);
	interrupts::without_interrupts(|| {
		result = crate::vga::_VGA.get_current_screen().write_fmt(args);
	});
	result
}

// ===== write! =====

#[macro_export]
macro_rules! vga_write {
	($dst:expr, $($arg:tt)*) => {
		$crate::vga::_write($dst, core::format_args!($($arg)*))
	};
}

#[macro_export]
macro_rules! vga_writeln {
	($dst:expr $(,)?) => {
		$crate::vga_write!($dst, "\n")
	};
	($dst:expr, $($arg:tt)*) => {
		$crate::vga_write!($dst, "{}\n", core::format_args!($($arg)*))
	};
}

#[doc(hidden)]
pub fn _write(idx: usize, args: core::fmt::Arguments) -> core::fmt::Result {
	let mut result: core::fmt::Result = Err(core::fmt::Error);
	interrupts::without_interrupts(|| {
		if let Some(mut screen) = crate::vga::_VGA.get_screen(idx) {
			result = screen.write_fmt(args);
		}
	});
	result
}

// ===== Color =====

#[allow(dead_code)]
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
	#[default]
	Black = 0,
	Blue = 1,
	Green = 2,
	Cyan = 3,
	Red = 4,
	Magenta = 5,
	Yellow = 6,
	White = 7,
}

// ===== VGAPorts =====

// CRTC register selector
const VGA_CRTC_INDEX: u16 = 0x3D4;

// read or write to the selected register
const VGA_CRTC_DATA: u16 = 0x3D5;

struct VGAPorts {
	command: Port<u8>,
	data: Port<u8>,
}

// ===== VGA =====

//#[derive(Debug)]
pub struct VGA {
	display: core::sync::atomic::AtomicUsize,
	screens: [spin::Mutex<Screen>; Self::LENGTH],
	screen_offset: [usize; Self::LENGTH],
	ports: spin::Mutex<VGAPorts>,
}

impl VGA {
	const ADDR: usize = 0x000b8000;
	const LENGTH: usize = 8;

	pub fn new() -> Self {
		Self {
			display: core::sync::atomic::AtomicUsize::new(1),
			screens: core::array::from_fn(|i| spin::Mutex::new(Screen::new(Self::ADDR + i * Screen::SIZE))),
			screen_offset: core::array::from_fn(|i| i * Screen::LENGTH),
			ports: spin::Mutex::new(
				VGAPorts {
					command: Port::new(VGA_CRTC_INDEX),
					data: Port::new(VGA_CRTC_DATA),
				}
			),
		}
	}

	fn get_screen(&self, index: usize) -> Option<spin::MutexGuard<Screen>> {
		if index < Self::LENGTH {
			Some(self.screens[index].lock())
		}
		else {
			None
		}
	}

	pub fn get_current_screen(&self) -> spin::MutexGuard<Screen> {
		self.screens[self.display.load(core::sync::atomic::Ordering::Relaxed)].lock()
	}

	pub fn set_display(&self, index: usize) -> () {
		if index < Self::LENGTH {
			let old_index = self.display.swap(index, core::sync::atomic::Ordering::Relaxed);
			if index != old_index {
				let mut ports_guard = self.ports.lock(); // TODO: without_interrupts (unless only keyboard interrupts can trigger this method)
				unsafe {
					ports_guard.command.write(0x0c);
					ports_guard.data.write(((self.screen_offset[index] >> 8) & 0xFF) as u8);
					ports_guard.command.write(0x0d);
					ports_guard.data.write((self.screen_offset[index] & 0xFF) as u8);
				}
			}
		}
		else {
			panic!("set_display: index must belong to 0..7");
		}
	}
}

