
#![no_std]
#![feature(abi_x86_interrupt)]

pub mod arch;
mod interrupts;
mod vga;
mod keyboard;

// https://os.phil-opp.com/hardware-interrupts/#the-hlt-instruction
pub fn hlt_loop() -> ! {
	loop {
		arch::x86::instructions::hlt();
	}
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
	// Meant to be the only occurrence in which screen 0 is allowed
	let _result: core::fmt::Result = vga_writeln!(0, "\n\n{}", info); // TODO: write on serial port
	hlt_loop();
}

// fn print_rainbow_42(screen_index: usize) -> () {
// 	let str_42 = "
//         :::      ::::::::
//       :+:      :+:    :+:
//     +:+ +:+         +:+
//   +#+  +:+       +#+
// +#+#+#+#+#+   +#+
//      #+#    #+#
//     ###   ########.fr";

// 	let mut screen: spin::MutexGuard<vga::screen::Screen> = vga::_VGA.get_screen(screen_index);
// 	screen.set_color(vga::Color::Black);
// 	for line in str_42.lines() {
// 		writeln!(screen, "{}", line).unwrap();
// 		screen.set_next_rainbow_color();
// 	}
// }

use core::arch::asm;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
struct DescriptorTablePointer {
    limit: u16,
    base: u64,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
struct SegmentDescriptor {
    limit_low: u16,
    base_low: u16,
    base_mid: u8,
    access: u8,
    flags_limit: u8,
    base_high: u8,
}

impl SegmentDescriptor {
    fn base(&self) -> u32 {
        (self.base_low as u32)
            | ((self.base_mid as u32) << 16)
            | ((self.base_high as u32) << 24)
    }

    fn limit(&self) -> u32 {
        (self.limit_low as u32) | (((self.flags_limit & 0x0F) as u32) << 16)
    }

    fn flags(&self) -> u8 {
        (self.flags_limit >> 4) & 0x0F
    }

    const fn empty() -> Self {
        Self {limit_low: 0, base_low:0, base_mid:0, access:0, flags_limit:0, base_high: 0}
    }
    const fn flat(access: u8) -> Self {
        Self {limit_low: 0xFFFF, base_low:0, base_mid:0, access:access, flags_limit:0xCF, base_high: 0}
    }
}

pub fn dump_gdt() {
    let mut gdtr = DescriptorTablePointer { limit: 0, base: 0 };

    unsafe {
        asm!(
            "sgdt [{}]",
            in(reg) &mut gdtr,
            options(nostack, preserves_flags)
        );
    }

    let gdt_size = (gdtr.limit as usize) + 1;
    let num_entries = gdt_size / core::mem::size_of::<SegmentDescriptor>();
	vga::_VGA.set_display(7);
    vga_println!("gdtr.limit: {} gdtr.base {}", gdtr.limit as usize, gdtr.base as usize);
    vga_println!("size: {} num_entries {}", gdt_size, num_entries);

    let gdt_ptr = gdtr.base as *const SegmentDescriptor;
    for i in 0..num_entries {
        let desc = unsafe { *gdt_ptr.add(i) };
		vga_println!(
            "GDT[{}]: base=0x{:08X}, limit=0x{:X}, access=0x{:02X}, flags=0x{:02X}",
            i,
            desc.base(),
            desc.limit(),
            desc.access,
            desc.flags(),
        );
    }
}

pub fn init_gdt() {
    let gdtr = DescriptorTablePointer { limit: 7 * core::mem::size_of::<SegmentDescriptor>() as u16 - 1, base: 0x800 };
    let dest_ptr = 0x800 as *mut SegmentDescriptor;
    let mut gdt = [SegmentDescriptor::empty(); 7];

    gdt[1] = SegmentDescriptor::flat(0x9A);
    gdt[2] = SegmentDescriptor::flat(0x92);
    gdt[3] = SegmentDescriptor::flat(0x92);
    gdt[4] = SegmentDescriptor::flat(0xFA);
    gdt[5] = SegmentDescriptor::flat(0xF2);
    gdt[6] = SegmentDescriptor::flat(0xF2);

    unsafe{
        core::ptr::copy(gdt.as_ptr(), dest_ptr, 7);
        asm!("lgdt [{}]",
             "push 0x8", // push new code segment_descriptor offset on the stack
             "lea {tmp}, [2f]", // get the address of label 2: (2f == label 2 + look fowrard)
             "push {tmp}", // put that address on the stack
             "retf", // pops the 2: address value into EIP and kernel code segment descriptor offset (0x8) in CS register which makes it reload its code cache
             "2:",
             "mov ds, ax", // replace all registers address with the address of our data segment
             "mov es, ax",
             "mov fs, ax",
             "mov gs, ax",
             "mov ss, bx", // replace the stack register by the address of our stack segment
             in(reg) &gdtr,
             tmp = out(reg) _,
             in("ax") 0x10u16,
             in("bx") 0x18u16,
             options(readonly, nostack, preserves_flags)
             );
    }
}


fn init() {
	init_gdt();
	interrupts::init_idt();
	unsafe { interrupts::_PICS.lock().initialize() };

	arch::x86::instructions::interrupts::enable();
}

#[no_mangle]
pub extern "C" fn rust_main(n: u32) {
	// ATTENTION: we have a very small stack and no guard page

	vga::_VGA.set_display(7);
	vga_println!("\n{}", n).unwrap();
	dump_gdt();
	init();
	dump_gdt();
	// print_rainbow_42(7);
	vga_print!("$> ").unwrap();
	vga_print!("\nThe END").unwrap();

	vga::_VGA.set_display(6);
	vga_print!(
		"\
		aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
		bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb\
		cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc\
		dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd\
		eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee\
		ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
		gggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg\
		hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh\
		iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii\
		jjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjj\
		kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk\
		llllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllll\
		mmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm\
		nnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnn\
		oooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo\
		pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp\
		qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq\
		rrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrr\
		ssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssss\
		tttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttt\
		uuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuu\
		vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv\
		wwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwww\
		xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\
		yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy\
		zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz\
		"
	).unwrap();
	// for _ in 0..10 {
	// 	vga::_VGA.get_current_screen().shift_upward();
	// }
	// for _ in 0..10 {
	// 	vga::_VGA.get_current_screen().shift_downward();
	// }
	arch::x86::instructions::interrupts::int3();

	hlt_loop();
}

