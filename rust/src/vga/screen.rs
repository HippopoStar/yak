
use super::Color;

// ===== Cell =====

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
struct Cell(u8, Color);

impl Cell {
	fn volatile_copy(dst: &mut Self, src: &Self) -> () {
		unsafe { (dst as *mut Self).write_volatile((src as *const Self).read_volatile()) };
	}
}

// ===== Cursor =====

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Cursor {
	line: usize,
	column: usize,
}

// ===== Screen =====

#[derive(Debug)]
pub struct Screen {
	cursor: Cursor,
	color: Color,
	buff: &'static mut [[Cell; Self::WIDTH]; Self::HEIGHT],
}

impl Screen {
	const HEIGHT: usize = 25;
	const WIDTH: usize = 80;
	pub(super) const SIZE: usize = Self::HEIGHT * Self::WIDTH * core::mem::size_of::<Cell>();

	pub fn new(addr: usize) -> Self {
		Self {
			cursor: Cursor {
				line: 0,
				column: 0,
			},
			color: Color::Black,
			buff: unsafe { &mut (*(addr as *mut _)) },
		}
	}

	pub fn set_color(&mut self, color: Color) -> () {
		self.color = color;
	}

	pub fn set_next_rainbow_color(&mut self) -> () {
		self.color = match self.color {
			Color::Black => Color::White,
			Color::White => Color::Red,
			Color::Red => Color::Yellow,
			Color::Yellow => Color::Green,
			Color::Green => Color::Cyan,
			Color::Cyan => Color::Blue,
			Color::Blue => Color::Magenta,
			Color::Magenta => Color::Black,
		}
	}

	pub fn copy_from(&mut self, other: &Self) -> () {
		self.cursor = other.cursor;
		self.color = other.color;
		for row in 0..Self::HEIGHT {
			for column in 0..Self::WIDTH {
				Cell::volatile_copy(&mut self.buff[row][column], &other.buff[row][column]);
			}
		}
	}

	fn shift_upward(&mut self) -> () {
		let mut it = self.buff.iter_mut().peekable();
		while let Some(above) = it.next() {
			if let Some(below) = it.peek() {
				for column in 0..Self::WIDTH {
					Cell::volatile_copy(&mut above[column], &below[column]);
				}
			}
			else {
				for column in 0..Self::WIDTH {
					Cell::volatile_copy(&mut above[column], &Cell(b'\0', Color::Black));
				}
			}
		}
	}

	fn write_new_line(&mut self) -> () {
		self.cursor.column = 0;
		if Self::HEIGHT == self.cursor.line + 1 {
			self.shift_upward();
		}
		else {
			self.cursor.line += 1;
		}
	}

	fn write_byte(&mut self, c: u8) -> () {
		Cell::volatile_copy(&mut self.buff[self.cursor.line][self.cursor.column], &Cell(c, self.color));
		self.cursor.column += 1;
		if Self::WIDTH == self.cursor.column {
			self.write_new_line();
		}
	}

pub	fn del_byte(&mut self) -> () {
		self.cursor.column -= 1;
		Cell::volatile_copy(&mut self.buff[self.cursor.line][self.cursor.column], &Cell(b'\0', Color::Black));
        // penser au cas ou la colonne est deja a 0
	}

}

impl core::fmt::Write for Screen {
	fn write_str(&mut self, s: &str) -> core::fmt::Result {
		for c in s.bytes() {
			if c.is_ascii_graphic() {
				self.write_byte(c);
			}
			else if b'\n' == c {
				self.write_new_line();
			}
			else if c.is_ascii_whitespace() {
				self.write_byte(c);
			}
			else {
				self.write_byte(0x04); // diamond symbol
			}
		}
		Ok(())
	}
}

