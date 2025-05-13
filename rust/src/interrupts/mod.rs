
use lazy_static::lazy_static;
use core::fmt::Write;
use crate::arch::x86::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static! {
	static ref IDT: InterruptDescriptorTable = {
		let mut idt = InterruptDescriptorTable::new();
		idt.breakpoint.set_handler_fn(breakpoint_handler);
		idt
	};
}

pub fn init_idt() -> () {
	IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) -> ()
{
	writeln!(crate::vga::_VGA.get_screen(1), "EXCEPTION: BREAKPOINT\n{:#?}", stack_frame).unwrap();
}
