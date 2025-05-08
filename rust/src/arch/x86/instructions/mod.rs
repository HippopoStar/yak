
use core::arch::asm;

/// Halts the CPU until the next interrupt arrives.
pub fn hlt() {
	unsafe {
		asm!("hlt", options(nomem, nostack, preserves_flags));
	}
}
