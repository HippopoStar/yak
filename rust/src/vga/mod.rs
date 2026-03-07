
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
		$crate::vga::_write($crate::vga::_VGA.get_current_index(), core::format_args!($($arg)*))
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

// ===== input! =====

#[macro_export]
macro_rules! vga_input {
	($($arg:tt)*) => {
		$crate::vga::_input(core::format_args!($($arg)*))
	};
}

#[doc(hidden)]
pub fn _input(args: core::fmt::Arguments) -> core::fmt::Result {
	let mut result: core::fmt::Result = Err(core::fmt::Error);
	interrupts::without_interrupts(|| {
		if let Some(mut screen) = crate::vga::_VGA.get_screen(crate::vga::_VGA.get_current_index()) {
			screen.set_input_mode(true);
			result = screen.write_fmt(args);
			screen.set_input_mode(false);
		}
	});
	result
}

pub fn print_rainbow_42() -> () {
	interrupts::without_interrupts(|| {
		if let Some(mut screen) = crate::vga::_VGA.get_screen(crate::vga::_VGA.get_current_index()) {
			screen.print_rainbow_42();
		}
	});
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
	Black_on_White = 0x70,
	Blue_on_White = 0x71,
	Green_on_White = 0x72,
	Cyan_on_White = 0x73,
	Red_on_White = 0x74,
	Magenta_on_White = 0x75,
	Yellow_on_White = 0x76,
	White_on_White = 0x77,
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

	pub fn get_current_index(&self) -> usize {
		self.display.load(core::sync::atomic::Ordering::Relaxed)
	}

    pub fn clear_display(&self) -> () {
        if let Some(mut screen) = self.get_screen(self.get_current_index()) {
            screen.clear();
        }
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

