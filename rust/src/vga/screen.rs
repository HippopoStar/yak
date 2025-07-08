
use super::Color;

// ===== Cell =====

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
struct Cell(u8, Color);

impl Cell {
	// Traits and Generics -> Fully Qualified Method Calls
	fn volatile_copy(dst: &mut Self, src: &Self) -> () {
		unsafe { (dst as *mut Self).write_volatile((src as *const Self).read_volatile()) };
	}
}

// ===== Cursor =====

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Cursor {
	row: usize,
	column: usize,
}

// ===== History =====

fn copy_row(dst: &mut [Cell; Screen::WIDTH], src: &[Cell; Screen::WIDTH]) -> () {
	for column in 0..Screen::WIDTH {
		Cell::volatile_copy(&mut dst[column], &src[column]);
	}
}

// // Fundamental Types -> Type Aliases
// // Structs -> Tuple-Like Structs
// #[repr(transparent)]
// struct Row([Cell; Screen::WIDTH]);

// impl Default for Row {
// 	fn default() -> Self {
// 		Self {
// 			0: [Cell::default(); Screen::WIDTH],
// 		}
// 	}
// }

// https://doc.rust-lang.org/stable/core/ptr/fn.copy_nonoverlapping.html
// https://doc.rust-lang.org/stable/core/mem/fn.replace.html
// Collections -> VecDeque<T>
// https://doc.rust-lang.org/stable/alloc/collections/vec_deque/index.html
// https://doc.rust-lang.org/stable/std/collections/vec_deque/index.html
// Structs -> Generic Structs with Constant Parameters
#[derive(Debug)]
struct History<const N: usize> {
	head: usize,
	length: usize,
	pivot: usize,
	circular_buffer: [[Cell; Screen::WIDTH]; N],
}

impl<const N: usize> History<N> {
	fn new() -> Self {
		Self {
			head: 0,
			length: 0,
			pivot: 0,
			circular_buffer: [[Cell::default(); Screen::WIDTH]; N],
		}
	}

	fn get_pivot(&self) -> usize {
		self.pivot
	}

	// TODO: does not need 'volatile' copy used by 'copy_row'

	fn push_upper_row(&mut self, upper_row: &[Cell; Screen::WIDTH], lower_row: &mut [Cell; Screen::WIDTH]) -> () {
		if 0 < N {
			let target_row: &mut [Cell; Screen::WIDTH] = &mut self.circular_buffer[(self.head + self.pivot) % N];
			if self.pivot < self.length {
				copy_row(lower_row, target_row);
				self.pivot += 1;
			}
			else if self.length < N {
				self.length += 1;
				self.pivot += 1;
			}
			else {
				self.head = (self.head + 1) % N;
			}
			copy_row(target_row, upper_row);
		}
	}

	fn pop_upper_row(&mut self, upper_row: &mut [Cell; Screen::WIDTH], lower_row: &[Cell; Screen::WIDTH]) -> () {
		if 0 < self.pivot {
			self.pivot -=1;
			let source_row: &mut [Cell; Screen::WIDTH] = &mut self.circular_buffer[(self.head + self.pivot) % N];
			copy_row(upper_row, source_row);
			copy_row(source_row, lower_row);
		}
	}
}

// ===== Screen =====

#[derive(Debug)]
pub struct Screen {
	cursor: Cursor,
	color: Color,
	history: History<5>,
	buff: &'static mut [[Cell; Self::WIDTH]; Self::HEIGHT],
}

impl Screen {
	const HEIGHT: usize = 25;
	const WIDTH: usize = 80;
	pub(super) const SIZE: usize = Self::HEIGHT * Self::WIDTH * core::mem::size_of::<Cell>();

	pub fn new(addr: usize) -> Self {
		Self {
			cursor: Cursor {
				row: 0,
				column: 0,
			},
			color: Color::default(),
			history: History::new(),
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

	// pub fn copy_from(&mut self, other: &Self) -> () {
	// 	self.cursor = other.cursor;
	// 	self.color = other.color;
	// 	for row in 0..Self::HEIGHT {
	// 		copy_row(&mut self.buff[row], &other.buff[row]);
	// 	}
	// }

	pub fn shift_upward(&mut self) -> () {
		let lower_row = &mut [Cell::default(); Screen::WIDTH];
		self.history.push_upper_row(self.buff.first().unwrap(), lower_row);

		let mut it = self.buff.iter_mut().peekable();
		while let Some(above) = it.next() {
			if let Some(below) = it.peek() {
				copy_row(above, below);
			}
			else {
				copy_row(above, lower_row);
			}
		}
	}

	pub fn shift_downward(&mut self) -> () {
		if 0 < self.history.get_pivot() {
			let upper_row = &mut [Cell::default(); Screen::WIDTH];
			self.history.pop_upper_row(upper_row, self.buff.last().unwrap());

			let mut it = self.buff.iter_mut().rev().peekable();
			while let Some(above) = it.next() {
				if let Some(below) = it.peek() {
					copy_row(above, below);
				}
				else {
					copy_row(above, upper_row);
				}
			}
		}
	}

	fn write_new_line(&mut self) -> () {
		self.cursor.column = 0;
		if self.cursor.row + 1 < Self::HEIGHT {
			self.cursor.row += 1;
		}
		else {
			self.shift_upward();
		}
	}

	fn write_byte(&mut self, c: u8) -> () {
		Cell::volatile_copy(&mut self.buff[self.cursor.row][self.cursor.column], &Cell(c, self.color));
		self.cursor.column += 1;
		if Self::WIDTH == self.cursor.column {
			self.write_new_line();
		}
	}

	// TODO: detect a '\n' and act accordingly
	// Collections -> Vec<T> -> Splitting
	// https://doc.rust-lang.org/stable/core/primitive.slice.html#method.chunks_mut
	fn shift_leftward(&mut self, row: usize, mut column: usize) -> () {
		let mut it_rows = self.buff.iter_mut().skip(row).peekable();
		while let Some(above) = it_rows.next() {
			let mut it_columns = above.iter_mut().skip(column).peekable();
			column = 0;
			while let Some(current) = it_columns.next() {
				let mut next: &Cell = &Cell::default();
				if let Some(rightward) = it_columns.peek() {
					next = &rightward;
				}
				else if let Some(below) = it_rows.peek() {
					next = &below[0];
				}
				// References -> Working with References -> Comparing References
				if current != next {
					Cell::volatile_copy(current, next);
				}
			}
		}
	}

	pub fn suppr_byte(&mut self) -> () {
		self.shift_leftward(self.cursor.row, self.cursor.column);
	}

	pub fn del_byte(&mut self) -> () {
		if 0 == self.cursor.column {
			if 0 == self.cursor.row {
				// retrieve last row in history buffer
				self.shift_downward();
			}
			else {
				self.cursor.row -= 1;
			}
			self.cursor.column = Self::WIDTH;
		}
		// multiple '\0' should be considered as 1
		// no 'do {} while ();' loop in Rust
		self.cursor.column -= 1;
		while 0 < self.cursor.column && Cell::default() == self.buff[self.cursor.row][self.cursor.column - 1] {
			self.cursor.column -= 1;
		}
		self.shift_leftward(self.cursor.row, self.cursor.column);
	}
}

// TODO: 2 modes, default & insert (replacing characters in place)

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
				// https://en.wikipedia.org/wiki/Code_page_437
				self.write_byte(0xfe);
			}
		}
		Ok(())
	}
}

