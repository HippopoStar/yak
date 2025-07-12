
use core::arch::asm;
use bitflags::bitflags;

// http://css.csail.mit.edu/6.858/2013/readings/i386.pdf -> EFLAGS Register

bitflags! {
	/// The EFLAGS register. All bit patterns are valid representations for this type.
	#[repr(transparent)]
	#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
	pub struct RFlags: u32 {
		/// Enable the virtual-8086 mode.
		const VIRTUAL_8086_MODE = 1 << 17;
		/// Allows to restart an instruction following an instruction breakpoint.
		const RESUME_FLAG = 1 << 16;
		/// Used by `iret` in hardware task switch mode to determine if current task is nested.
		const NESTED_TASK = 1 << 14;
		/// The high bit of the I/O Privilege Level field.
		///
		/// Specifies the privilege level required for executing I/O address-space instructions.
		const IOPL_HIGH = 1 << 13;
		/// The low bit of the I/O Privilege Level field.
		///
		/// Specifies the privilege level required for executing I/O address-space instructions.
		const IOPL_LOW = 1 << 12;
		/// Set by hardware to indicate that the sign bit of the result of the last signed integer
		/// operation differs from the source operands.
		const OVERFLOW_FLAG = 1 << 11;
		/// Determines the order in which strings are processed.
		const DIRECTION_FLAG = 1 << 10;
		/// Enable interrupts.
		const INTERRUPT_FLAG = 1 << 9;
		/// Enable single-step mode for debugging.
		const TRAP_FLAG = 1 << 8;
		/// Set by hardware if last arithmetic operation resulted in a negative value.
		const SIGN_FLAG = 1 << 7;
		/// Set by hardware if last arithmetic operation resulted in a zero value.
		const ZERO_FLAG = 1 << 6;
		/// Set by hardware if last arithmetic operation generated a carry ouf of bit 3 of the
		/// result.
		const AUXILIARY_CARRY_FLAG = 1 << 4;
		/// Set by hardware if last result has an even number of 1 bits (only for some operations).
		const PARITY_FLAG = 1 << 2;
		/// Set by hardware if last arithmetic operation generated a carry out of the
		/// most-significant bit of the result.
		const CARRY_FLAG = 1;
	}
}

/// Returns the current value of the RFLAGS register.
///
/// Drops any unknown bits.
#[inline]
pub fn read() -> RFlags {
	RFlags::from_bits_truncate(read_raw())
}

/// Returns the raw current value of the RFLAGS register.
#[inline]
pub fn read_raw() -> u32 {
	let r: u32;

	unsafe {
		asm!("pushfd; pop {}", out(reg) r, options(nomem, preserves_flags));
	}

	r
}

/// Writes the RFLAGS register, preserves reserved bits.
///
/// ## Safety
///
/// Unsafe because undefined becavior can occur if certain flags are modified. For example,
/// the `DF` flag must be unset in all Rust code. Also, modifying `CF`, `PF`, or any other
/// flags also used by Rust/LLVM can result in undefined behavior too.
#[inline]
pub unsafe fn write(flags: RFlags) {
	let old_value = read_raw();
	let reserved = old_value & !(RFlags::all().bits());
	let new_value = reserved | flags.bits();

	unsafe {
		write_raw(new_value);
	}
}

/// Writes the RFLAGS register.
///
/// Does not preserve any bits, including reserved bits.
///
///
/// ## Safety
///
/// Unsafe because undefined becavior can occur if certain flags are modified. For example,
/// the `DF` flag must be unset in all Rust code. Also, modifying `CF`, `PF`, or any other
/// flags also used by Rust/LLVM can result in undefined behavior too.
#[inline]
pub unsafe fn write_raw(val: u32) {
	// HACK: we mark this function as preserves_flags to prevent Rust from restoring
	// saved flags after the "popf" below. See above note on safety.
	unsafe {
		asm!("push {}; popfd", in(reg) val, options(nomem, preserves_flags));
	}
}
