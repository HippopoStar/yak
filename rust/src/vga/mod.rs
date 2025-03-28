
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
struct Cell(u8, Color);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Cursor {
	line: usize,
	column: usize,
}

#[derive(Debug)]
pub struct VGA {
	cursor: Cursor,
	color: Color,
	buff: &'static mut [[Cell; Self::WIDTH]; Self::HEIGHT],
}

impl VGA {
	const ADDR: usize = 0x000b8000;
	const HEIGHT: usize = 25;
	const WIDTH: usize = 80;

	pub fn new() -> Self {
		Self {
			cursor: Cursor {
				line: 0,
				column: 0,
			},
			color: Color::Black,
			buff: unsafe { &mut (*(Self::ADDR as *mut _)) },
		}
	}

	fn shift_upward(&mut self) -> () {
		for i in 0..Self::HEIGHT - 1 {
			for j in 0..Self::WIDTH {
				self.buff[i][j] = self.buff[i+1][j];
				// let mut above: &mut Cell = &mut self.buff[i][j];
				// let below: &Cell = &self.buff[i + 1][j];
				// unsafe { (above as *mut Cell).write_volatile((below as *const Cell).read_volatile()) };
			}
		}
		for j in 0..Self::WIDTH {
			self.buff[Self::HEIGHT - 1][j] = Cell(0, Color::Black);
		}
	}

	fn write_new_line(&mut self) -> () {
		self.cursor.column = 0;
		self.cursor.line += 1;
		if Self::HEIGHT == self.cursor.line {
			self.shift_upward();
		}
	}

	fn write_byte(&mut self, c: u8) -> () {
		self.buff[self.cursor.line][self.cursor.column].0 = c;
		self.buff[self.cursor.line][self.cursor.column].1 = self.color;
		self.cursor.column += 1;
		if Self::WIDTH == self.cursor.column {
			self.write_new_line();
		}
	}
}

impl core::fmt::Write for VGA {
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

