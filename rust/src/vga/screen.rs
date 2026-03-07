
use super::Color;

const HISTORY_CAPACITY: usize = 5;

/// Invariants
/// 0 <= self.cursor.row < Self::HEIGHT
/// 0 <= self.cursor.column < Self::WIDTH
/// for each row, &self.buff[self.cursor.row][0..self.cursor.column] does not contain any b'\0'
/// last column of the last row (either in self.buff or in self.history) == b'\0'

// ############################################################################
// #                              CELL                                        #
// ############################################################################

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

// ############################################################################
// #                              CURSOR                                      #
// ############################################################################

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
struct Cursor {
	row: usize,
	column: usize,
}

// ############################################################################
// #                              ROW                                         #
// ############################################################################

// Fundamental Types -> Type Aliases
// Structs -> Tuple-Like Structs
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
struct Row<const W: usize>([Cell; W]);

impl<const W: usize> Row<W> {
	// https://doc.rust-lang.org/core/primitive.slice.html#method.copy_from_slice
	// volatile copy? (not needed in the context of History)
	fn copy_from(&mut self, rhs: &Self) -> () {
		self.0.copy_from_slice(&rhs.0);
	}
	fn initialize(&mut self) -> () {
		self.0.fill(Cell::default());
	}
	// https://doc.rust-lang.org/core/primitive.slice.html#method.rotate_left
	fn left_shift(&mut self, column: usize, amplitude: usize) -> () {
		if column + amplitude < W {
			self.0[column..].rotate_left(amplitude);
		}
	}
	// https://doc.rust-lang.org/core/primitive.slice.html#method.rotate_right
	fn right_shift(&mut self, column: usize, amplitude: usize) -> () {
		if column + amplitude < W {
			self.0[column..].rotate_right(amplitude);
		}
	}
}

impl<const W: usize> Default for Row<W> {
	fn default() -> Self {
		Self {
			0: [Cell::default(); W],
		}
	}
}

// Operator Overloading -> Index and IndexMut
impl<const W: usize> core::ops::Index<usize> for Row<W> {
	type Output = Cell;
	fn index(&self, idx: usize) -> &Cell {
		return &self.0[idx]
	}
}

impl<const W: usize> core::ops::IndexMut<usize> for Row<W> {
	fn index_mut(&mut self, idx: usize) -> &mut Cell {
		return &mut self.0[idx]
	}
}

// ############################################################################
// #                              HISTORY                                     #
// ############################################################################

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
	circular_buffer: [Row<{Screen::WIDTH}>; N],
}

// | oldest upward -- (head)
// |
// | newest upward
// | newest downward -- (pivot)
// |
// | oldest downward
// -- (length)

impl<const N: usize> History<N> {
	fn new() -> Self {
		Self {
			head: 0,
			length: 0,
			pivot: 0,
			circular_buffer: [Row::<{Screen::WIDTH}>::default(); N],
		}
	}

	fn get_upward_length(&self) -> usize {
		self.pivot
	}

	fn get_downward_length(&self) -> usize {
		self.length - self.pivot
	}

	// This method swaps the content of 'upper_row' with the content of the newest downward row, if any
	fn push_upper_row(&mut self, upper_row: &mut Row<{Screen::WIDTH}>) -> () {
		if 0 < N {
			let dest_row: &mut Row<{Screen::WIDTH}> = &mut self.circular_buffer[(self.head + self.pivot) % N];
			core::mem::swap(dest_row, upper_row);
			if self.pivot < self.length {
				self.pivot += 1;
			}
			else if self.length < N {
				// No need to initialize upper_row (dest_row was already empty)
				self.length += 1;
				self.pivot += 1;
			}
			else {
				upper_row.initialize();
				self.head = (self.head + 1) % N;
			}
		}
	}

	// If there is a upward history
	fn pop_upper_row(&mut self, lower_row: &mut Row<{Screen::WIDTH}>) -> () {
		if 0 < self.pivot {
			self.pivot -=1;
			let src_row: &mut Row<{Screen::WIDTH}> = &mut self.circular_buffer[(self.head + self.pivot) % N];
			core::mem::swap(lower_row, src_row);
		}
	}

	// The intent of this method is to right shift downward history
	// to insert the screen latest cell in the event of a writing on Screen
	// The caller is responsible for ensuring there is enough room in History
	fn shift_rightward(&mut self, mut above_end_of_line: Cell) {
		let mut idx = self.pivot;
		// While the last cell of a row is not '\0',
		// rotate the underneath row 1 cell to the right and replace
		// the first cell of that row with the aforementioned cell
		while b'\0' != above_end_of_line.0 && idx < self.length {
			self.circular_buffer[(self.head + idx) % N].right_shift(0, 1);
			above_end_of_line = core::mem::replace(&mut self.circular_buffer[(self.head + idx) % N][0], above_end_of_line);
			idx += 1;
		}
		if b'\0' != above_end_of_line.0 {
			// If there is still room available in the history,
			// add a new row with 1 cell
			if self.length < N {
				self.circular_buffer[(self.head + self.length) % N].initialize();
				above_end_of_line = core::mem::replace(&mut self.circular_buffer[(self.head + self.length) % N][0], above_end_of_line);
				self.length += 1;
			}
			// Otherwise, erase oldest upward history to make room
			else if 0 < self.pivot {
				self.pivot -= 1;
				self.circular_buffer[self.head % N].initialize();
				above_end_of_line = core::mem::replace(&mut self.circular_buffer[self.head % N][0], above_end_of_line);
				self.head = (self.head + 1) % N;
			}
		}
	}
}

// ############################################################################
// #                              SCREEN                                      #
// ############################################################################

#[derive(Debug)]
pub(super) struct Screen {
	cursor: Cursor,
	color: Color,
	history: History<HISTORY_CAPACITY>,
	buff: &'static mut [Row<{Self::WIDTH}>; Self::HEIGHT],
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
		let it_rows = self.buff.iter_mut();
		for row in it_rows {
			row.initialize();
		}
	}

	fn shift_upward(&mut self) -> () {
		self.history.push_upper_row(&mut self.buff.first_mut().unwrap());
		self.buff.rotate_left(1);
	}

	fn shift_downward(&mut self) -> () {
		if 0 < self.history.get_upward_length() {
			self.history.pop_upper_row(&mut self.buff.last_mut().unwrap());
			self.buff.rotate_right(1);
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
		self.buff[row].left_shift(column, 1);
		while b'\0' != current_end_of_line.0 && row + 1 < Self::HEIGHT {
			let below_start_of_line = Cell::new_from(&self.buff[row + 1][0]);
			Cell::volatile_copy(&mut self.buff[row][Self::WIDTH - 1], &below_start_of_line);
			row += 1;
			Cell::volatile_copy(&mut current_end_of_line, &self.buff[row][Self::WIDTH - 1]);
			self.buff[row].left_shift(column, 1);
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
		self.buff[row].right_shift(column, 1);
		let mut above_end_of_line = Cell::new_from(&self.buff[row][column]);
		// Cell::volatile_copy(&mut self.buff[row][column], Cell::default());
		while b'\0' != above_end_of_line.0 && row + 1 < Self::HEIGHT {
			row += 1;
			self.buff[row].right_shift(0, 1);
			above_end_of_line = core::mem::replace(&mut self.buff[row][0], above_end_of_line);
		}
		if b'\0' != above_end_of_line.0 {
			if 0 < HISTORY_CAPACITY - self.history.get_downward_length() {
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
		else if 0 < self.history.get_upward_length() {
			self.shift_downward();
		}
		self.left_align_cursor();
	}

	fn move_cursor_down(&mut self) -> () {
		if self.cursor.row + 1 < Self::HEIGHT {
			self.cursor.row += 1;
		}
		else if 0 < self.history.get_downward_length() {
			self.shift_upward();
		}
		self.left_align_cursor();
	}

	fn move_cursor_right(&mut self) -> () {
		if self.cursor.column + 1 < Self::WIDTH {
			if b'\0' == self.buff[self.cursor.row][self.cursor.column].0 {
				if self.cursor.row + 1 < Self::HEIGHT || 0 < self.history.get_downward_length() {
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
		else if 0 < self.history.get_downward_length() {
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
		else if 0 < self.history.get_upward_length() {
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
					if 0 < self.history.get_upward_length() {
						self.shift_downward();
						if self.cursor.row + 1 < Self::HEIGHT {
							self.cursor.row += 1;
						}
						self.left_align_cursor();
					}
				}
				else if b'\x1f' == c {
					// scroll down
					if 0 < self.history.get_downward_length() {
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

