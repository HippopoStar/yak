
use core::arch::asm;
use core::marker::PhantomData;

// ===== inb/inw/inl/outb/outw/outl =====

unsafe fn inb(port: u16) -> u8 {
	let value: u8;
	unsafe {
		asm!("in al, dx", out("al") value, in("dx") port, options(nomem, nostack, preserves_flags));
	}
	value
}

unsafe fn inw(port: u16) -> u16 {
	let value: u16;
	unsafe {
		asm!("in ax, dx", out("ax") value, in("dx") port, options(nomem, nostack, preserves_flags));
	}
	value
}

unsafe fn inl(port: u16) -> u32 {
	let value: u32;
	unsafe {
		asm!("in eax, dx", out("eax") value, in("dx") port, options(nomem, nostack, preserves_flags));
	}
	value
}

unsafe fn outb(value: u8, port: u16) -> () {
	unsafe {
		asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack, preserves_flags));
	}
}

unsafe fn outw(value: u16, port: u16) -> () {
	unsafe {
		asm!("out dx, ax", in("dx") port, in("ax") value, options(nomem, nostack, preserves_flags));
	}
}

unsafe fn outl(value: u32, port: u16) -> () {
	unsafe {
		asm!("out dx, eax", in("dx") port, in("eax") value, options(nomem, nostack, preserves_flags));
	}
}

// ===== PortRead/PortWrite =====

pub trait PortRead {
	unsafe fn read_from_port(port: u16) -> Self;
}

pub trait PortWrite {
	unsafe fn write_to_port(port: u16, value: Self);
}

impl PortRead for u8 {
	unsafe fn read_from_port(port: u16) -> Self {
		inb(port)
	}
}

impl PortRead for u16 {
	unsafe fn read_from_port(port: u16) -> Self {
		inw(port)
	}
}

impl PortRead for u32 {
	unsafe fn read_from_port(port: u16) -> Self {
		inl(port)
	}
}

impl PortWrite for u8 {
	unsafe fn write_to_port(port: u16, value: Self) -> () {
		outb(value, port);
	}
}

impl PortWrite for u16 {
	unsafe fn write_to_port(port: u16, value: Self) -> () {
		outw(value, port);
	}
}

impl PortWrite for u32 {
	unsafe fn write_to_port(port: u16, value: Self) -> () {
		outl(value, port)
	}
}

// ===== PortReadAccess/PortWriteAccess =====

pub trait PortAccess {}
pub trait PortReadAccess: PortAccess {}
pub trait PortWriteAccess: PortAccess {}

// Unit-Like Structs p.213
pub struct ReadOnlyAccess;
pub struct WriteOnlyAccess;
pub struct ReadWriteAccess;

impl PortAccess for ReadOnlyAccess {}
impl PortReadAccess for ReadOnlyAccess {}

impl PortAccess for WriteOnlyAccess {}
impl PortWriteAccess for WriteOnlyAccess {}

impl PortAccess for ReadWriteAccess {}
impl PortReadAccess for ReadWriteAccess {}
impl PortWriteAccess for ReadWriteAccess {}

// ===== Port/PortReadOnly/PortWriteOnly =====

// PhantomData p.644
pub struct PortGeneric<T, A> {
	port: u16,
	phantom: PhantomData<(T, A)>,
}

// Type Aliases p.78

/// A read-only I/O port
pub type PortReadOnly<T> = PortGeneric<T, ReadOnlyAccess>;

/// A write-only I/O port
pub type PortWriteOnly<T> = PortGeneric<T, WriteOnlyAccess>;

/// A read-write I/O port
pub type Port<T> = PortGeneric<T, ReadWriteAccess>;

impl<T, A> PortGeneric<T, A> {
	pub const fn new(port: u16) -> Self {
		Self {
			port,
			phantom: PhantomData,
		}
	}
}

impl<T: PortRead, A: PortReadAccess> PortGeneric<T, A> {
	pub unsafe fn read(&mut self) -> T {
		unsafe { T::read_from_port(self.port) }
	}
}

impl<T: PortWrite, A: PortWriteAccess> PortGeneric<T, A> {
	pub unsafe fn write(&mut self, value: T) -> () {
		unsafe { T::write_to_port(self.port, value) }
	}
}
