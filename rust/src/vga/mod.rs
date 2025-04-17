
pub mod screen;

use self::screen::Screen;

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

#[derive(Debug)]
pub struct VGA {
	display: core::sync::atomic::AtomicUsize,
	screens: [spin::Mutex<Screen>; 8],
}

impl VGA {
	const ADDR: usize = 0x000b8000;

	pub fn new() -> Self {
		Self {
			display: core::sync::atomic::AtomicUsize::new(1),
			screens: core::array::from_fn(|i| spin::Mutex::new(Screen::new(Self::ADDR + i * Screen::SIZE))),
		}
	}

	pub fn get_screen(&self, index: usize) -> &spin::Mutex<Screen> {
		if index == self.display.load(core::sync::atomic::Ordering::Relaxed) {
			&self.screens[0]
		}
		else {
			&self.screens[index]
		}
	}

	pub fn set_display(&self, index: usize) -> () {
		if 0 < index && index < self.screens.len() {
			let old_index = self.display.load(core::sync::atomic::Ordering::Relaxed);
			if index != old_index {
				let mut screen_0 = self.screens[0].lock();
				// Backup currently displayed screen
				self.screens[old_index].lock().copy_from(&screen_0);
				// Display previously backuped screen
				screen_0.copy_from(&self.screens[index].lock());
				self.display.store(index, core::sync::atomic::Ordering::Relaxed);
			}
		}
		else {
			panic!("set_display: index must belong to 1..8");
		}
	}
}

