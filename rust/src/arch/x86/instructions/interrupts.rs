
use core::arch::asm;

/// Cause a breakpoint exception by invoking the `int3` instruction.
pub fn int3() {
	unsafe {
		asm!("int3", options(nomem, nostack));
	}
}
