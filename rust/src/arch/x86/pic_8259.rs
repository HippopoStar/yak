
use super::instructions::port::Port;

/// Command sent to begin PIC initialization.
const CMD_INIT: u8 = 0x11;

/// Command sent to acknowledge an interrupt.
const CMD_END_OF_INTERRUPT: u8 = 0x20;

// The mode in which we want to run our PICs.
const MODE_8086: u8 = 0x01;

/// An individual PIC chip.  This is not exported, because we always access
/// it through `Pics` below.
struct Pic {
	/// The base offset to which our interrupts are mapped.
	offset: u8,

	/// The processor I/O port on which we send commands.
	command: Port<u8>,

	/// The processor I/O port on which we send and receive data.
	data: Port<u8>,
}

impl Pic {
	/// Are we in charge of handling the specified interrupt?
	/// (Each PIC handles 8 interrupts.)
	fn handles_interrupt(&self, interrupt_id: u8) -> bool {
		self.offset <= interrupt_id && interrupt_id < self.offset + 8
	}

	/// Notify us that an interrupt has been handled and that we're ready
	/// for more.
	unsafe fn end_of_interrupt(&mut self) -> () {
		self.command.write(CMD_END_OF_INTERRUPT);
	}

	/// Reads the interrupt mask of this PIC.
	unsafe fn read_mask(&mut self) -> u8 {
		self.data.read()
	}

	/// Writes the interrupt mask of this PIC.
	unsafe fn write_mask(&mut self, mask: u8) -> () {
		self.data.write(mask)
	}
}

/// A pair of chained PICs.  This is the standard setup on x86.
pub struct ChainedPics {
	pics: [Pic; 2],
}

impl ChainedPics {
	/// Create a new interface for the standard PIC1 and PIC2,
	/// specifying the desired interrupt offsets.
	pub const unsafe fn new(offset1: u8, offset2: u8) -> Self {
		Self {
			pics: [
				Pic {
					offset: offset1,
					command: Port::new(0x20),
					data: Port::new(0x21),
				},
				Pic {
					offset: offset2,
					command: Port::new(0xA0),
					data: Port::new(0xA1),
				},
			],
		}
	}

	/// Create a new `ChainedPics` interface that will map the PICs contiguously starting at the given interrupt offset.
	///
	/// This is a convenience function that maps the PIC1 and PIC2 to a
	/// contiguous set of interrupts. This function is equivalent to
	/// `Self::new(primary_offset, primary_offset + 8)`.
	pub const unsafe fn new_contiguous(primary_offset: u8) -> Self {
		Self::new(primary_offset, primary_offset + 8)
	}

	/// Initialize both our PICs.  We initialize them together, at the same
	/// time, because it's traditional to do so, and because I/O operations
	/// might not be instantaneous on older processors.
	pub unsafe fn initialize(&mut self) -> () {
		// We need to add a delay between writes to our PICs, especially on
		// older motherboards.  But we don't necessarily have any kind of
		// timers yet, because most of them require interrupts.  Various
		// older versions of Linux and other PC operating systems have
		// worked around this by writing garbage data to port 0x80, which
		// allegedly takes long enough to make everything work on most
		// hardware.  Here, `wait` is a closure.
		let mut wait_port: Port<u8> = Port::new(0x80);
		let mut wait = || wait_port.write(0);

		// Save our original interrupt masks, because I'm too lazy to
		// figure out reasonable values. We'll restore these when we're
		// done.
		let saved_masks = self.read_masks();

		// Tell each PIC that we're going to send it a three-byte
		// initialization sequence on its data port.
		self.pics[0].command.write(CMD_INIT);
		wait();
		self.pics[1].command.write(CMD_INIT);
		wait();

		// Byte 1: Set up our base offsets.
		self.pics[0].data.write(self.pics[0].offset);
		wait();
		self.pics[1].data.write(self.pics[1].offset);
		wait();

		// Byte 2: Configure chaining between PIC1 and PIC2.
		self.pics[0].data.write(4);
		wait();
		self.pics[1].data.write(2);
		wait();

		// Byte 3: Set our mode.
		self.pics[0].data.write(MODE_8086);
		wait();
		self.pics[1].data.write(MODE_8086);
		wait();

		// Restore our saved masks.
		self.write_masks(saved_masks[0], saved_masks[1])
	}

	/// Reads the interrupt masks of both PICs.
	pub unsafe fn read_masks(&mut self) -> [u8; 2] {
		[self.pics[0].read_mask(), self.pics[1].read_mask()]
	}

	/// Writes the interrupt masks of both PICs.
	pub unsafe fn write_masks(&mut self, mask1: u8, mask2: u8) -> () {
		self.pics[0].write_mask(mask1);
		self.pics[1].write_mask(mask2);
	}

	/// Disables both PICs by masking all interrupts.
	pub unsafe fn disable(&mut self) -> () {
		self.write_masks(u8::MAX, u8::MAX)
	}

	/// Do we handle this interrupt?
	pub fn handles_interrupt(&self, interrupt_id: u8) -> bool {
		self.pics.iter().any(|p| p.handles_interrupt(interrupt_id))
	}

	/// Figure out which (if any) PICs in our chain need to know about this
	/// interrupt.  This is tricky, because all interrupts from `pics[1]`
	/// get chained through `pics[0]`.
	pub unsafe fn notify_end_of_interrupt(&mut self, interrupt_id: u8) {
		if self.handles_interrupt(interrupt_id) {
			if self.pics[1].handles_interrupt(interrupt_id) {
				self.pics[1].end_of_interrupt();
			}
			self.pics[0].end_of_interrupt();
		}
	}
}
