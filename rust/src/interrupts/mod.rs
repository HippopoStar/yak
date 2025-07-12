
use lazy_static::lazy_static;
use crate::arch::x86::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::arch::x86::pic_8259::ChainedPics;
use crate::keyboard;

lazy_static! {
	static ref IDT: InterruptDescriptorTable = {
		let mut idt = InterruptDescriptorTable::new();
		idt.breakpoint.set_handler_fn(breakpoint_handler);
		idt.interrupts[0].set_handler_fn(timer_handler);
		idt.interrupts[1].set_handler_fn(keyboard_handler);
		idt
	};
}

pub fn init_idt() -> () {
	IDT.load();
}

const PIC_1_OFFSET: u8 = 32;
const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static _PICS: spin::Mutex<ChainedPics> = spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });



extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame)
{
	crate::vga_writeln!(1, "EXCEPTION: BREAKPOINT\n{:#?}", stack_frame).unwrap(); // TODO: print on serial port
}

extern "x86-interrupt" fn timer_handler(_stack_frame: InterruptStackFrame)
{
	// crate::vga_write!(2, ".").unwrap();
	unsafe {
		_PICS.lock().notify_end_of_interrupt(32);
	}
}

extern "x86-interrupt" fn keyboard_handler(_stack_frame: InterruptStackFrame)
{
	keyboard::_KB.print_scancode();
	unsafe {
		_PICS.lock().notify_end_of_interrupt(33);
	}
}
