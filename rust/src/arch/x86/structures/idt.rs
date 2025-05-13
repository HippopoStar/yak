
use core::arch::asm;
use core::fmt::{self, Debug};
use core::marker::PhantomData;
use core::ops::Deref;
use bit_field::BitField;
use volatile::Volatile;

/// A struct describing a pointer to a descriptor table (GDT / IDT).
/// This is in a format suitable for giving to 'lgdt' or 'lidt'.
#[derive(Debug, Clone, Copy)]
#[repr(C, packed(2))]
pub struct DescriptorTablePointer {
	/// Size of the DT in bytes - 1.
	pub limit: u16,
	/// Pointer to the memory region containing the DT.
	pub base: u32,
}

/// Load an IDT.
///
/// Use the
/// [`InterruptDescriptorTable`](crate::structures::idt::InterruptDescriptorTable) struct for a high-level
/// interface to loading an IDT.
///
/// ## Safety
///
/// This function is unsafe because the caller must ensure that the given
/// `DescriptorTablePointer` points to a valid IDT and that loading this
/// IDT is safe.
#[inline]
pub unsafe fn lidt(idt: &DescriptorTablePointer) {
	unsafe {
		asm!("lidt [{}]", in(reg) idt, options(readonly, nostack, preserves_flags));
	}
}

/// Get the address of the current IDT.
#[inline]
pub fn sidt() -> DescriptorTablePointer {
	let mut idt: DescriptorTablePointer = DescriptorTablePointer {
		limit: 0,
		base: 0,
	};
	unsafe {
		asm!("sidt [{}]", in(reg) &mut idt, options(nostack, preserves_flags));
	}
	idt
}

#[inline]
fn get_reg_code_segment() -> u16 {
	let segment: u16;
	unsafe {
		asm!("mov {0:x}, cs", out(reg) segment, options(nomem, nostack, preserves_flags));
	}
	segment
}

#[derive(Clone, Debug)]
#[repr(C)]
#[repr(align(16))]
pub struct InterruptDescriptorTable {
	pub divide_error: Entry<HandlerFunc>,
	pub debug: Entry<HandlerFunc>,
	pub non_maskable_interrupt: Entry<HandlerFunc>,
	pub breakpoint: Entry<HandlerFunc>,
	pub overflow: Entry<HandlerFunc>,
	pub bound_range_exceeded: Entry<HandlerFunc>,
	pub invalid_opcode: Entry<HandlerFunc>,
	pub device_not_available: Entry<HandlerFunc>,
	pub double_fault: Entry<DivergingHandlerFuncWithErrCode>,
	coprocessor_segment_overrun: Entry<HandlerFunc>,
	pub invalid_tss: Entry<HandlerFuncWithErrCode>,
	pub segment_not_present: Entry<HandlerFuncWithErrCode>,
	pub stack_segment_fault: Entry<HandlerFuncWithErrCode>,
	pub general_protection_fault: Entry<HandlerFuncWithErrCode>,
	pub page_fault: Entry<PageFaultHandlerFunc>,
	reserved_1: Entry<HandlerFunc>,
	pub x87_floating_point: Entry<HandlerFunc>,
	pub alignment_check: Entry<HandlerFuncWithErrCode>,
	pub machine_check: Entry<DivergingHandlerFunc>,
	pub simd_floating_point: Entry<HandlerFunc>,
	pub virtualization: Entry<HandlerFunc>,
	pub cp_protection_exception: Entry<HandlerFuncWithErrCode>,
	reserved_2: [Entry<HandlerFunc>; 6],
	pub hv_injection_exception: Entry<HandlerFunc>,
	pub vmm_communication_exception: Entry<HandlerFuncWithErrCode>,
	pub security_exception: Entry<HandlerFuncWithErrCode>,
	reserved_3: Entry<HandlerFunc>,
	interrupts: [Entry<HandlerFunc>; 256 - 32],
}

impl InterruptDescriptorTable {
	/// Creates a new IDT filled with non-present entries.
	pub fn new() -> Self {
		Self {
			divide_error: Entry::missing(),
			debug: Entry::missing(),
			non_maskable_interrupt: Entry::missing(),
			breakpoint: Entry::missing(),
			overflow: Entry::missing(),
			bound_range_exceeded: Entry::missing(),
			invalid_opcode: Entry::missing(),
			device_not_available: Entry::missing(),
			double_fault: Entry::missing(),
			coprocessor_segment_overrun: Entry::missing(),
			invalid_tss: Entry::missing(),
			segment_not_present: Entry::missing(),
			stack_segment_fault: Entry::missing(),
			general_protection_fault: Entry::missing(),
			page_fault: Entry::missing(),
			reserved_1: Entry::missing(),
			x87_floating_point: Entry::missing(),
			alignment_check: Entry::missing(),
			machine_check: Entry::missing(),
			simd_floating_point: Entry::missing(),
			virtualization: Entry::missing(),
			cp_protection_exception: Entry::missing(),
			reserved_2: [Entry::missing(); 6],
			hv_injection_exception: Entry::missing(),
			vmm_communication_exception: Entry::missing(),
			security_exception: Entry::missing(),
			reserved_3: Entry::missing(),
			interrupts: [Entry::missing(); 256 - 32],
		}
	}

	/// Resets all entries of this IDT in place.
	pub fn reset(&mut self) -> () {
		*self = Self::new();
	}

	/// Loads the IDT in the CPU using the `lidt` command.
	pub fn load(&'static self) -> () {
		unsafe { self.load_unsafe() }
	}

	/// Loads the IDT in the CPU using the `lidt` command.
	///
	/// # Safety
	///
	/// As long as it is the active IDT, you must ensure that:
	///
	/// - `self` is never destroyed.
	/// - `self` always stays at the same memory location. It is recommended to wrap it in
	///   a `Box`.
	pub unsafe fn load_unsafe(&self) {
		{ // DEBUG
			use core::fmt::Write;
			writeln!(crate::vga::_VGA.get_screen(1), "interrupts::IDT.pointer(): {:?}", &self.pointer()).unwrap();
		}
		unsafe {
			lidt(&self.pointer());
		}
		{ // DEBUG
			use core::fmt::Write;
			let idt: crate::arch::x86::structures::idt::DescriptorTablePointer = crate::arch::x86::structures::idt::sidt();
			writeln!(crate::vga::_VGA.get_screen(1), "crate::arch::x86::structures::idt::sidt(): {:?}", idt).unwrap();
		}
	}

	/// Creates the descriptor pointer for this table. This pointer can only be
	/// safely used if the table is never modified or destroyed while in use.
	fn pointer(&self) -> DescriptorTablePointer {
		DescriptorTablePointer {
			limit: (core::mem::size_of::<Self>() - 1) as u16,
			base: self as *const _ as u32,
		}
	}
}

impl Default for InterruptDescriptorTable {
	fn default() -> Self {
		Self::new()
	}
}

/// An Interrupt Descriptor Table entry.
///
/// The generic parameter is some [`HandlerFuncType`], depending on the interrupt vector.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Entry<F> {
	pointer_low: u16,
	options: EntryOptions,
	pointer_high: u16,
	phantom: PhantomData<F>,
}

impl<T> Debug for Entry<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Entry")
			.field("handler_addr", &format_args!("{:#x}", self.handler_addr()))
			.field("options", &self.options)
			.finish()
	}
}

impl<T> PartialEq for Entry<T> {
	fn eq(&self, other: &Self) -> bool {
		self.pointer_low == other.pointer_low
			&& self.options == other.options
			&& self.pointer_high == other.pointer_high
	}
}

/// A handler function for an interrupt or an exception without error code.
pub type HandlerFunc = extern "x86-interrupt" fn(InterruptStackFrame);

/// A handler function for an exception that pushes an error code.
pub type HandlerFuncWithErrCode = extern "x86-interrupt" fn(InterruptStackFrame, error_code: u32);

/// A page fault handler function that pushes a page fault error code.
pub type PageFaultHandlerFunc = extern "x86-interrupt" fn(InterruptStackFrame, error_code: PageFaultErrorCode);

/// A handler function that must not return, e.g. for a machine check exception.
pub type DivergingHandlerFunc = extern "x86-interrupt" fn(InterruptStackFrame) -> !;

/// A handler function with an error code that must not return, e.g. for a double fault exception.
pub type DivergingHandlerFuncWithErrCode = extern "x86-interrupt" fn(InterruptStackFrame, error_code: u32) -> !;

/// A general handler function for an interrupt or an exception with the interrupt/exceptions's index and an optional error code.
pub type GeneralHandlerFunc = fn(InterruptStackFrame, index: u8, error_code: Option<u32>);

impl<F> Entry<F> {
	/// Creates a non-present IDT entry (but sets the must-be-one bits).
	pub const fn missing() -> Self {
		Entry {
			pointer_low: 0,
			options: EntryOptions::minimal(),
			pointer_high: 0,
			phantom: PhantomData,
		}
	}

	/// Sets the handler address for the IDT entry and sets the following defaults:
	///   - The code selector is the code segment currently active in the CPU
	///   - The present bit is set
	///   - Interrupts are disabled on handler invocation
	///   - The privilege level (DPL) is [`PrivilegeLevel::Ring0`]
	///   - No IST is configured (existing stack will be used)
	///
	/// The function returns a mutable reference to the entry's options that allows
	/// further customization.
	///
	/// # Safety
	///
	/// The caller must ensure that `addr` is the address of a valid interrupt handler function,
	/// and the signature of such a function is correct for the entry type.
	pub unsafe fn set_handler_addr(&mut self, addr: u32) -> &mut EntryOptions {

		self.pointer_low = addr as u16;
		self.pointer_high = (addr >> 16) as u16;

		self.options = EntryOptions::minimal();
		unsafe { self.options.set_code_selector(get_reg_code_segment()) };
		self.options.set_present(true);
		{ // DEBUG
			use core::fmt::Write;
			writeln!(crate::vga::_VGA.get_screen(1), "{:?}", &self.options).unwrap();
		}
		&mut self.options
	}

	/// Returns the virtual address of this IDT entry's handler function.
	pub fn handler_addr(&self) -> u32 {
		self.pointer_low as u32 | ((self.pointer_high as u32) << 16)
	}
}

impl<F: HandlerFuncType> Entry<F> {
	/// Sets the handler function for the IDT entry and sets the following defaults:
	///   - The code selector is the code segment currently active in the CPU
	///   - The present bit is set
	///   - Interrupts are disabled on handler invocation
	///   - The privilege level (DPL) is [`PrivilegeLevel::Ring0`]
	///   - No IST is configured (existing stack will be used)
	///
	/// The function returns a mutable reference to the entry's options that allows
	/// further customization.
	///
	/// This method is only usable with the `abi_x86_interrupt` feature enabled. Without it, the
	/// unsafe [`Entry::set_handler_addr`] method has to be used instead.
	pub fn set_handler_fn(&mut self, handler: F) -> &mut EntryOptions {
		{ // DEBUG
			use core::fmt::Write;
			writeln!(crate::vga::_VGA.get_screen(1), "&handler as *const _ as u32: {:#x}", &handler as *const _ as u32).unwrap();
		}
		unsafe { self.set_handler_addr(handler.to_virt_addr()) }
	}
}

/// A common trait for all handler functions usable in [`Entry`].
///
/// # Safety
///
/// Implementors have to ensure that `to_virt_addr` returns a valid address.
pub unsafe trait HandlerFuncType {
	/// Get the virtual address of the handler function.
	fn to_virt_addr(self) -> u32;
}

macro_rules! impl_handler_func_type {
	($f:ty) => {
		unsafe impl HandlerFuncType for $f {
			fn to_virt_addr(self) -> u32 {
				{ // DEBUG
					use core::fmt::Write;
					writeln!(crate::vga::_VGA.get_screen(1), "self as u32: {:#x}", self as u32).unwrap();
				}
				// Casting a function pointer to u32 is fine, if the pointer
				// width doesn't exeed 32 bits.
				self as u32
			}
		}
	};
}

impl_handler_func_type!(HandlerFunc);
impl_handler_func_type!(HandlerFuncWithErrCode);
impl_handler_func_type!(PageFaultHandlerFunc);
impl_handler_func_type!(DivergingHandlerFunc);
impl_handler_func_type!(DivergingHandlerFuncWithErrCode);

/// Represents the 4 non-offset bytes of an IDT entry.
#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub struct EntryOptions {
	cs: u16,
	bits: u16,
}

impl Debug for EntryOptions {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("EntryOptions")
			.field("code_selector", &self.cs)
			.field("bits", &format_args!("{:#b}", self.bits))
			.field("present", &self.present())
			.finish()
	}
}

impl EntryOptions {
	/// Creates a minimal options field with all the must-be-one bits set. This
	/// means the CS selector, IST, and DPL field are all 0.
	const fn minimal() -> Self {
		EntryOptions {
			cs: 0,
			bits: 0b0000_1110_0000_0000,
		}
	}

	/// Set the code segment that will be used by this interrupt.
	///
	/// ## Safety
	/// This function is unsafe because the caller must ensure that the passed
	/// segment selector points to a valid, long-mode code segment.
	pub unsafe fn set_code_selector(&mut self, cs: u16) -> &mut Self {
		self.cs = cs;
		self
	}

	/// Set or reset the preset bit.
	pub fn set_present(&mut self, present: bool) -> &mut Self {
		self.bits.set_bit(15, present);
		self
	}

	fn present(&self) -> bool {
		self.bits.get_bit(15)
	}
}

/// Wrapper type for the interrupt stack frame pushed by the CPU.
///
/// This type derefs to an [`InterruptStackFrameValue`], which allows reading the actual values.
///
/// This wrapper type ensures that no accidental modification of the interrupt stack frame
/// occurs, which can cause undefined behavior (see the [`as_mut`](InterruptStackFrame::as_mut)
/// method for more information).
#[repr(transparent)]
pub struct InterruptStackFrame(InterruptStackFrameValue);

impl InterruptStackFrame {
	/// Creates a new interrupt stack frame with the given values.
	pub fn new(
		instruction_pointer: u32,
		code_segment: u16,
		cpu_flags: u32,
		stack_pointer: u32,
		stack_segment: u16,
	) -> Self {
		Self(InterruptStackFrameValue::new(
			instruction_pointer,
			code_segment,
			cpu_flags,
			stack_pointer,
			stack_segment,
		))
	}

	/// Gives mutable access to the contents of the interrupt stack frame.
	///
	/// The `Volatile` wrapper is used because LLVM optimizations remove non-volatile
	/// modifications of the interrupt stack frame.
	///
	/// ## Safety
	///
	/// This function is unsafe since modifying the content of the interrupt stack frame
	/// can easily lead to undefined behavior. For example, by writing an invalid value to
	/// the instruction pointer field, the CPU can jump to arbitrary code at the end of the
	/// interrupt.
	///
	/// Also, it is not fully clear yet whether modifications of the interrupt stack frame are
	/// officially supported by LLVM's x86 interrupt calling convention.
	pub unsafe fn as_mut(&mut self) -> Volatile<&mut InterruptStackFrameValue> {
		Volatile::new(&mut self.0)
	}
}

impl Deref for InterruptStackFrame {
	type Target = InterruptStackFrameValue;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl Debug for InterruptStackFrame {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.0.fmt(f)
	}
}

/// Represents the interrupt stack frame pushed by the CPU on interrupt or exception entry.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct InterruptStackFrameValue {
	/// This value points to the instruction that should be executed when the interrupt
	/// handler returns. For most interrupts, this value points to the instruction immediately
	/// following the last executed instruction. However, for some exceptions (e.g., page faults),
	/// this value points to the faulting instruction, so that the instruction is restarted on
	/// return. See the documentation of the [`InterruptDescriptorTable`] fields for more details.
	pub instruction_pointer: u32,
	/// The code segment selector at the time of the interrupt.
	pub code_segment: u16,
	_reserved1: [u8; 2],
	/// The flags register before the interrupt handler was invoked.
	pub cpu_flags: u32,
	/// The stack pointer at the time of the interrupt.
	pub stack_pointer: u32,
	/// The stack segment descriptor at the time of the interrupt (often zero in 64-bit mode).
	pub stack_segment: u16,
	_reserved2: [u8; 2],
}

impl InterruptStackFrameValue {
	/// Creates a new interrupt stack frame with the given values.
	pub fn new(
		instruction_pointer: u32,
		code_segment: u16,
		cpu_flags: u32,
		stack_pointer: u32,
		stack_segment: u16,
	) -> Self {
		Self {
			instruction_pointer,
			code_segment,
			_reserved1: Default::default(),
			cpu_flags,
			stack_pointer,
			stack_segment,
			_reserved2: Default::default(),
		}
	}
}

impl Debug for InterruptStackFrameValue {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut s = f.debug_struct("InterruptStackFrame");
		s.field("instruction_pointer", &self.instruction_pointer);
		s.field("code_segment", &self.code_segment);
		s.field("cpu_flags", &self.cpu_flags);
		s.field("stack_pointer", &self.stack_pointer);
		s.field("stack_segment", &self.stack_segment);
		s.finish()
	}
}

/// Describes an page fault error code.
///
/// This structure is defined by the following manual sections:
///   * AMD Volume 2: 8.4.2
///   * Intel Volume 3A: 4.7
#[repr(transparent)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
pub struct PageFaultErrorCode(u32);
