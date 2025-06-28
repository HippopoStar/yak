
use lazy_static::lazy_static;
use core::fmt::Write;
use crate::arch::x86::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::PICS;
use crate::keyboard::Keyboard; 

lazy_static! {
	static ref IDT: InterruptDescriptorTable = {
		let mut idt = InterruptDescriptorTable::new();
		idt.breakpoint.set_handler_fn(breakpoint_handler);
	    idt.interrupts[0].set_handler_fn(timer_handler);
	    idt.interrupts[1].set_handler_fn(keyboard_handler);
		idt
	};
}

lazy_static! {
	pub static ref _KB: Keyboard = Keyboard::new();
}

pub fn init_idt() -> () {
	IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame)
{
	writeln!(crate::vga::_VGA.get_screen(1), "EXCEPTION: BREAKPOINT\n{:#?}", stack_frame).unwrap();
}

extern "x86-interrupt" fn timer_handler(stack_frame: InterruptStackFrame)
{
//	write!(crate::vga::_VGA.get_screen(2), ".").unwrap();
    unsafe {
        PICS.lock().notify_end_of_interrupt(32);
    }
}

extern "x86-interrupt" fn keyboard_handler(stack_frame: InterruptStackFrame)
{
    Keyboard::print_scancode();
}
