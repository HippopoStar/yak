
struct Cursor {
	line: usize,
	column: usize,
}

pub struct VGABuffer {
	cursor: Cursor,
}

impl VGABuffer {
	const ADDR: *mut [u8; VGABuffer::SIZE] = 0x000b8000 as *mut [u8; VGABuffer::SIZE];
	const HEIGHT: usize = 25;
	const WIDTH: usize = 80;
	const SIZE: usize = VGABuffer::HEIGHT * VGABuffer::WIDTH * 2;

	pub fn new() -> Self {
		Self {
			cursor: Cursor {
				line: 0,
				column: 0,
			},
		}
	}

	fn shift_upward(&mut self) -> () {
		self.cursor.line = VGABuffer::HEIGHT - 1;
		self.cursor.column = 0;
		unsafe {
			let underneath_addr = &mut ((*VGABuffer::ADDR)[(VGABuffer::WIDTH * 2)..]);
			for n in 0..((VGABuffer::HEIGHT - 1) * VGABuffer::WIDTH * 2) {
				(*VGABuffer::ADDR)[n] = underneath_addr[n];
			}
			for n in 0..(VGABuffer::WIDTH * 2) {
				(*VGABuffer::ADDR)[((VGABuffer::HEIGHT - 1) * VGABuffer::WIDTH * 2) + n] = b'\0';
			}
		}
	}

	fn write_new_line(&mut self) -> () {
		self.cursor.column = 0;
		self.cursor.line += 1;
		if VGABuffer::HEIGHT == self.cursor.line {
			self.shift_upward();
		}
	}

	fn write_byte(&mut self, c: u8) -> () {
		unsafe {
			(*VGABuffer::ADDR)[((self.cursor.line * VGABuffer::WIDTH) + self.cursor.column) * 2] = c;
		}
		self.cursor.column += 1;
		if VGABuffer::WIDTH == self.cursor.column {
			self.write_new_line();
		}
	}
}

impl core::fmt::Write for VGABuffer {
	fn write_str(&mut self, s: &str) -> core::fmt::Result {
		for c in s.chars() {
			if c.is_ascii_graphic() {
				self.write_byte(c as u8);
			}
			else if '\n' == c {
				self.write_new_line();
			}
			else if c.is_whitespace() {
				self.write_byte(b' ');
			}
			else {
				self.write_byte(b'.');
			}
		}
		Ok(())
	}
}

