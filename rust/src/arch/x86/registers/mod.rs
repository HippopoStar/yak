
pub mod rflags;

pub fn get_stack_frame() -> (u32, u32) {
	let stack_ptr: u32;
	unsafe {
		core::arch::asm!(
			"mov {}, esp",
			out(reg) stack_ptr,
			options(nomem, preserves_flags)
		);
	}
	let base_ptr: u32;
	unsafe {
		core::arch::asm!(
			"mov {}, ebp",
			out(reg) base_ptr,
			options(nomem, preserves_flags)
		);
	}
	(stack_ptr, base_ptr)
}
