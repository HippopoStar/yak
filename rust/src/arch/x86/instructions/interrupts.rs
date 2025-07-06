
use core::arch::asm;
use crate::arch::x86::registers;

/// Returns whether interrupts are enabled.
#[inline]
pub fn are_enabled() -> bool {
	registers::rflags::read().contains(registers::rflags::RFlags::INTERRUPT_FLAG)
}

/// Enable interrupts.
///
/// This is a wrapper around the `sti` instruction.
#[inline]
pub fn enable() {
	// Omit `nomem` to imitate a lock release. Otherwise, the compiler
	// is free to move reads and writes through this asm block.
	unsafe {
		asm!("sti", options(preserves_flags, nostack));
	}
}

/// Disable interrupts.
///
/// This is a wrapper around the `cli` instruction.
#[inline]
pub fn disable() {
	// Omit `nomem` to imitate a lock acquire. Otherwise, the compiler
	// is free to move reads and writes through this asm block.
	unsafe {
		asm!("cli", options(preserves_flags, nostack));
	}
}

/// Run a closure with disabled interrupts.
///
/// Run the given closure, disabling interrupts before running it (if they aren't already disabled).
/// Afterwards, interrupts are enabling again if they were enabled before.
///
/// If you have other `enable` and `disable` calls _within_ the closure, things may not work as expected.
///
/// # Examples
///
/// ```ignore
/// // interrupts are enabled
/// without_interrupts(|| {
///     // interrupts are disabled
///     without_interrupts(|| {
///         // interrupts are disabled
///     });
///     // interrupts are still disabled
/// });
/// // interrupts are enabled again
/// ```
#[inline]
pub fn without_interrupts<F, R>(f: F) -> R
where
	F: FnOnce() -> R,
{
	// true if the interrupt flag is set (i.e. interrupts are enabled)
	let saved_intpt_flag = are_enabled();

	// if interrupts are enabled, disable them for now
	if saved_intpt_flag {
		disable();
	}

	// do `f` while interrupts are disabled
	let ret = f();

	// re-enable interrupts if they were previously enabled
	if saved_intpt_flag {
		enable();
	}

	// return the result of `f` to the caller
	ret
}

/// Cause a breakpoint exception by invoking the `int3` instruction.
pub fn int3() {
	unsafe {
		asm!("int3", options(nomem, nostack));
	}
}
