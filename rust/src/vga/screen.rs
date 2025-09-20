
use super::Color;

const HISTORY_CAPACITY: usize = 20;

/// Invariants
/// 0 <= self.cursor.row < Self::HEIGHT
/// 0 <= self.cursor.column < Self::WIDTH
/// for each row, &self.buff[self.cursor.row][0..self.cursor.column] does not contain any b'\0'
/// last column of the last row (either in self.buff or in self.history) == b'\0'

// ===== Cell =====

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
struct Cell(u8, Color);

impl Cell {
	// Traits and Generics -> Fully Qualified Method Calls
	fn volatile_copy(dst: &mut Self, src: &Self) -> () {
		unsafe { (dst as *mut Self).write_volatile((src as *const Self).read_volatile()) };
	}

	fn new_from(other: &Cell) -> Self {
		Self(other.0, other.1) // volatile_copy?
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

fn right_shift_row(row: &mut [Cell; Screen::WIDTH], column: usize) {
	if column < Screen::WIDTH {
		let last_cell = *(row.last().unwrap());
		let mut it_columns = row.iter_mut().rev().take(Screen::WIDTH - column).peekable();
		while let Some(current) = it_columns.next() {
			if let Some(leftward) = it_columns.peek() {
				if leftward != &current {
					Cell::volatile_copy(current, leftward);
				}
			}
			else {
				Cell::volatile_copy(current, &last_cell);
			}
		}
	}
}

fn left_shift_row(row: &mut [Cell; Screen::WIDTH], column: usize) {
	if column < Screen::WIDTH {
		let first_cell = *(row.get(column).unwrap());
		let mut it_columns = row.iter_mut().skip(column).peekable();
		while let Some(current) = it_columns.next() {
			if let Some(rightward) = it_columns.peek() {
				if rightward != &current {
					Cell::volatile_copy(current, rightward);
				}
			}
			else {
				Cell::volatile_copy(current, &first_cell);
			}
		}
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

// | oldest up -- (head)
// |
// | newest up
// | newest down -- (pivot)
// |
// | oldest down
// -- (length)

impl<const N: usize> History<N> {
	fn new() -> Self {
		Self {
			head: 0,
			length: 0,
			pivot: 0,
			circular_buffer: [[Cell::default(); Screen::WIDTH]; N],
		}
	}

	fn get_head_length(&self) -> usize {
		self.pivot
	}

	fn get_tail_length(&self) -> usize {
		self.length - self.pivot
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

	fn shift_rightward(&mut self, mut above_end_of_line: Cell) {
		let mut idx = self.pivot;
		while b'\0' != above_end_of_line.0 && idx < self.length {
			right_shift_row(&mut self.circular_buffer[(self.head + idx) % N], 0);
			above_end_of_line = core::mem::replace(&mut self.circular_buffer[(self.head + idx) % N][0], above_end_of_line);
			idx += 1;
		}
		if b'\0' != above_end_of_line.0 {
			if self.length < N {
				copy_row(&mut self.circular_buffer[(self.head + self.length) % N], &[Cell::default(); Screen::WIDTH]);
				above_end_of_line = core::mem::replace(&mut self.circular_buffer[(self.head + self.length) % N][0], above_end_of_line);
				self.length += 1;
			}
			else if 0 < self.pivot {
				self.pivot -= 1;
				copy_row(&mut self.circular_buffer[self.head % N], &[Cell::default(); Screen::WIDTH]);
				above_end_of_line = core::mem::replace(&mut self.circular_buffer[self.head % N][0], above_end_of_line);
				self.head = (self.head + 1) % N;
				// according to the 2 above lines, "above_end_of_line" should now be Cell::default()
			}
		}
	}
}

// ===== Screen =====

#[derive(Debug)]
pub(super) struct Screen {
	cursor: Cursor,
	color: Color,
	history: History<HISTORY_CAPACITY>,
	buff: &'static mut [[Cell; Self::WIDTH]; Self::HEIGHT],
	input_mode: bool,
}

impl Screen {
	const HEIGHT: usize = 25;
	const WIDTH: usize = 80;
	pub const LENGTH: usize = Self::HEIGHT * Self::WIDTH;
	pub const SIZE: usize = Self::LENGTH * core::mem::size_of::<Cell>();

	pub fn new(addr: usize) -> Self {
		let mut instance = Self {
			cursor: Cursor {
				row: 0,
				column: 0,
			},
			color: Color::default(),
			history: History::new(),
			buff: unsafe { &mut (*(addr as *mut _)) },
			input_mode: false,
		};
		instance.clear();
		instance
	}

	pub fn set_color(&mut self, color: Color) -> () {
		self.color = color;
	}

	pub fn set_input_mode(&mut self, input_mode: bool) {
		self.input_mode = input_mode;
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
			_ => self.color,
		}
	}

	pub fn print_rainbow_42(&mut self) -> () {
		const STR_42: &'static str = "
        :::      ::::::::
      :+:      :+:    :+:
    +:+ +:+         +:+
  +#+  +:+       +#+
+#+#+#+#+#+   +#+
     #+#    #+#
    ###   ########.fr";

		use core::fmt::Write;
		STR_42.lines().for_each(|l| {
			self.write_str(l);
			self.write_str("\n");
			self.set_next_rainbow_color();
		});
	}

	fn clear(&mut self) -> () {
		let default_cell = &Cell::default();
		let mut it_rows = self.buff.iter_mut();
		for row in it_rows {
			let mut it_columns = row.iter_mut();
			for column in it_columns {
				Cell::volatile_copy(column, default_cell);
			}
		}
	}

	fn shift_upward(&mut self) -> () {
	//	if 0 < self.history.get_tail_length() {
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
	//	}
	}

	fn shift_downward(&mut self) -> () {
		if 0 < self.history.get_head_length() {
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
		self.shift_rightward(self.cursor.row, self.cursor.column);
		Cell::volatile_copy(&mut self.buff[self.cursor.row][self.cursor.column], &Cell(c, self.color));
		self.cursor.column += 1;
		if Self::WIDTH == self.cursor.column {
			self.write_new_line();
		}
	}

	// TODO: detect a '\n' and act accordingly
	// Collections -> Vec<T> -> Splitting
	// https://doc.rust-lang.org/stable/core/primitive.slice.html#method.chunks_mut
	fn shift_leftward(&mut self, mut row: usize, column: usize) -> () {
		// TODO: fix invariants
		let mut current_end_of_line = Cell::new_from(&self.buff[row][Self::WIDTH - 1]);
		left_shift_row(&mut self.buff[row], column);
		while b'\0' != current_end_of_line.0 && row + 1 < Self::HEIGHT {
			let below_start_of_line = Cell::new_from(&self.buff[row + 1][0]);
			Cell::volatile_copy(&mut self.buff[row][Self::WIDTH - 1], &below_start_of_line);
			row += 1;
			Cell::volatile_copy(&mut current_end_of_line, &self.buff[row][Self::WIDTH - 1]);
			left_shift_row(&mut self.buff[row], column);
		}
		if b'\0' == current_end_of_line.0 {
			Cell::volatile_copy(&mut self.buff[row][Self::WIDTH - 1], &Cell::default());
		}
		else {
			// TODO: fix this
		}



		// let mut it_rows = self.buff.iter_mut().skip(row).peekable();
		// while let Some(above) = it_rows.next() {
		// 	let mut it_columns = above.iter_mut().skip(column).peekable();
		// 	column = 0;
		// 	while let Some(current) = it_columns.next() {
		// 		let mut next: &Cell = &Cell::default();
		// 		// Same row, rightward character
		// 		if let Some(rightward) = it_columns.peek() {
		// 			next = rightward;
		// 		}
		// 		// Underneath row, first character
		// 		else if let Some(below) = it_rows.peek() {
		// 			next = below.first().unwrap();
		// 		}
		// 		// References -> Working with References -> Comparing References
		// 		if current != next {
		// 			Cell::volatile_copy(current, next);
		// 		}
		// 	}
		// }
	}

	// Iterators -> Implementing Your Own Iterators
	// .rev().take(Self::HEIGHT - row).peekable()
	// !(b'\0' == leftward.0)
	fn shift_rightward(&mut self, mut row: usize, column: usize) -> () {
		right_shift_row(&mut self.buff[row], column);
		let mut above_end_of_line = Cell::new_from(&self.buff[row][column]);
		// Cell::volatile_copy(&mut self.buff[row][column], Cell::default());
		while b'\0' != above_end_of_line.0 && row + 1 < Self::HEIGHT {
			row += 1;
			right_shift_row(&mut self.buff[row], 0);
			above_end_of_line = core::mem::replace(&mut self.buff[row][0], above_end_of_line);
		}
		if b'\0' != above_end_of_line.0 {
			if 0 < HISTORY_CAPACITY - self.history.get_tail_length() {
				self.history.shift_rightward(above_end_of_line);
			}
			else {
				self.shift_upward();
				if 0 < self.cursor.row {
					self.cursor.row -= 1;
				}
				else {
					self.left_align_cursor();
				}
				self.history.shift_rightward(above_end_of_line);
			}
		}
	}

	fn suppr_byte(&mut self) -> () {
		self.shift_leftward(self.cursor.row, self.cursor.column);
	}

	fn del_byte(&mut self) -> () {
		self.move_cursor_left();
		self.shift_leftward(self.cursor.row, self.cursor.column);
	}

	fn left_align_cursor(&mut self) -> () {
		while 0 < self.cursor.column && b'\0' == self.buff[self.cursor.row][self.cursor.column - 1].0 {
			self.cursor.column -= 1;
		}
	}

	fn move_cursor_up(&mut self) -> () {
		if 0 < self.cursor.row {
			self.cursor.row -= 1;
		}
		else if 0 < self.history.get_head_length() {
			self.shift_downward();
		}
		self.left_align_cursor();
	}

	fn move_cursor_down(&mut self) -> () {
		if self.cursor.row + 1 < Self::HEIGHT {
			self.cursor.row += 1;
		}
		else if 0 < self.history.get_tail_length() {
			self.shift_upward();
		}
		self.left_align_cursor();
	}

	fn move_cursor_right(&mut self) -> () {
		if self.cursor.column + 1 < Self::WIDTH {
			if b'\0' == self.buff[self.cursor.row][self.cursor.column].0 {
				if self.cursor.row + 1 < Self::HEIGHT || 0 < self.history.get_tail_length() {
					self.cursor.column = 0;
					self.move_cursor_down();
				}
			}
			else {
				self.cursor.column += 1;
			}
		}
		else if self.cursor.row + 1 < Self::HEIGHT {
			self.cursor.row += 1;
			self.cursor.column = 0;
		}
		else if 0 < self.history.get_tail_length() {
			self.shift_upward();
		}
		self.left_align_cursor();
	}

	fn move_cursor_left(&mut self) -> () {
		if 0 < self.cursor.column {
			self.cursor.column -= 1;
		}
		else if 0 < self.cursor.row {
			self.cursor.row -= 1;
			self.cursor.column = Self::WIDTH - 1;
		}
		else if 0 < self.history.get_head_length() {
			self.shift_downward();
			self.cursor.column = Self::WIDTH - 1;
		}
		self.left_align_cursor();
	}
}

// TODO: 2 modes, default & insert (replacing characters in place)

impl core::fmt::Write for Screen {
	fn write_str(&mut self, s: &str) -> core::fmt::Result {
		self.buff[self.cursor.row][self.cursor.column].1 = Color::default();
		if self.input_mode {
			for c in s.bytes() {
				if c.is_ascii_graphic() {
					self.write_byte(c);
				}
				else if b'\n' == c {
					self.write_new_line();
				}
				else if b'\x08' == c {
					// backspace
					self.del_byte();
				}
				else if b'\x7f' == c {
					// del
					self.suppr_byte();
				}
				else if b'\x18' == c {
					// arrow up
					self.move_cursor_up();
				}
				else if b'\x19' == c {
					// arrow down
					self.move_cursor_down();
				}
				else if b'\x1a' == c {
					// arrow right
					self.move_cursor_right();
				}
				else if b'\x1b' == c {
					// arrow left
					self.move_cursor_left();
				}
				else if b'\x1e' == c {
					// scroll up
					if 0 < self.history.get_head_length() {
						self.shift_downward();
						if self.cursor.row + 1 < Self::HEIGHT {
							self.cursor.row += 1;
						}
						self.left_align_cursor();
					}
				}
				else if b'\x1f' == c {
					// scroll down
					if 0 < self.history.get_tail_length() {
						self.shift_upward();
						if 0 < self.cursor.row {
							self.cursor.row -= 1;
						}
						self.left_align_cursor();
					}
				}
				else if c.is_ascii_whitespace() {
					self.write_byte(b' ');
				}
				else {
					// https://en.wikipedia.org/wiki/Code_page_437
					self.write_byte(b'\xfe');
				}
			}
		}
		else {
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
					self.write_byte(b'\xfe');
				}
			}
		}
		self.buff[self.cursor.row][self.cursor.column].1 = Color::Black_on_White;
		Ok(())
	}
}

