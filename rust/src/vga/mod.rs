
pub mod screen;

use self::screen::Screen;

use crate::arch::x86::instructions::port::Port;

lazy_static::lazy_static! {
	pub static ref _VGA: VGA = VGA::new();
}

// ===== Color =====

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
	Black = 0,
	Blue = 1,
	Green = 2,
	Cyan = 3,
	Red = 4,
	Magenta = 5,
	Yellow = 6,
	White = 7,
}

// ===== VGA =====

// CRTC register selector
const VGA_CRTC_INDEX: u16 = 0x3D4;

// read or write to the selected register
const VGA_CRTC_DATA: u16 = 0x3D5;

struct VGAPorts {
    command: Port<u8>,
    data: Port<u8>,
}

//#[derive(Debug)]
pub struct VGA {
	display: core::sync::atomic::AtomicUsize,
	screens: [spin::Mutex<Screen>; 8],
    screen_offset: [usize; 8],
    ports: spin::Mutex<VGAPorts>,
}

impl VGA {
	const ADDR: usize = 0x000b8000;

	pub fn new() -> Self {
		Self {
			display: core::sync::atomic::AtomicUsize::new(1),
			screens: core::array::from_fn(|i| spin::Mutex::new(Screen::new(Self::ADDR + i * Screen::SIZE))),
            ports: spin::Mutex::new(VGAPorts {
                command: Port::new(VGA_CRTC_INDEX),
                data: Port::new(VGA_CRTC_DATA),
            }),
			screen_offset: core::array::from_fn(|i| i * 2000),
		}
	}

	// Not thread safe:
	// It is quite unlikely, but self.display value might change by the time
	// screen 0 is locked.
	// Hopefully, since there is one less conditionnal structure in the meantime
	// than in the function that modifies self.display value, screen 0 should
	// happen to get locked earlier here
	pub fn get_screen(&self, index: usize) -> spin::MutexGuard<Screen> {
			self.screens[index].lock()
	}

    pub fn get_current_screen(&self) -> spin::MutexGuard<Screen> {
            self.screens[self.display.load(core::sync::atomic::Ordering::Relaxed)].lock()
    }

	pub fn set_display(&self, index: usize) -> () {
		if 0 <= index && index < 8 {
			let old_index = self.display.swap(index, core::sync::atomic::Ordering::Relaxed);
			if index != old_index {
                let mut ports_guard = self.ports.lock();
                unsafe {
                    ports_guard.command.write(0x0c);
                    ports_guard.data.write(((self.screen_offset[index] >> 8) & 0xFF) as u8);
                    ports_guard.command.write(0x0d);
                    ports_guard.data.write((self.screen_offset[index] & 0xFF) as u8);
                }
			}
		}
		else {
			panic!("set_display: index must belong to 1..8");
		}
	}
}

