
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

fn init() {
	interrupts::init_idt();
	unsafe { interrupts::_PICS.lock().initialize() };

	arch::x86::instructions::interrupts::enable();
}

pub struct BootInfoHeader {
    total_size :u32,
    reserved :u32,
}

pub struct TagHeader {
    tag_type :u32,
    tag_size :u32,
}

#[derive(Copy, Clone)]
pub struct MmapEntry {
    base_addr :u64,
    length :u64,
    entryType :u32,
    reserved :u32,
}

#[derive(Default, Copy, Clone)]
pub struct MmapInfo {
    tag_type :u32,
    tag_size :u32,
    entry_size :u32,
    entry_version: u32,
}

#[derive(Default, Copy, Clone)]
pub struct ElfSections {
    tag_type :u32,
    tag_size :u32,
    num :u32, // number of section headers
    entsize :u32, // size of each section header
    shndx :u32, // index of section name string table
}

#[derive(Default, Copy, Clone)]
pub struct ElfSectionHeader {
    sh_name: u32,      // Offset into section header string table
    sh_type: u32,      // Section type
    sh_flags: u32,     // Section flags
    sh_addr: u32,      // Virtual address in memory
    sh_offset: u32,    // Offset in file/image
    sh_size: u32,      // Size of section in bytes
    sh_link: u32,      // Link to another section
    sh_info: u32,      // Extra info
    sh_addralign: u32, // Alignment
    sh_entsize: u32,   // Size of entries if section holds a table
}

#[derive(Default)]
pub struct BootInfo {
    mmapInfo :MmapInfo,
    elfSections :ElfSections,
}

pub fn parse_bootinfo(bootinfo_addr :u32, magic: u32) {
    if magic != 0x36d76289 {
        panic!("\nmagic number is not 0x36d76289: {:#x}", magic);
    }

    let bootinfoHeader = unsafe {& (*((bootinfo_addr as usize) as *const BootInfoHeader))};
    let mut tagHeaderAddress = bootinfo_addr + core::mem::size_of::<BootInfoHeader>() as u32;
    let mut tagHeader = unsafe {& (*(tagHeaderAddress as *const TagHeader))};
    let mut bootInfo = BootInfo::default();

	vga_println!("bootinfo size {}", bootinfoHeader.total_size).unwrap();
	vga_println!("tagHeader type {} tagHeader size {}", tagHeader.tag_type, tagHeader.tag_size).unwrap();
    while tagHeader.tag_type != 0 && tagHeader.tag_size != 8 {
        if tagHeader.tag_type == 6 { // memory map
            bootInfo.mmapInfo = unsafe {*(tagHeaderAddress as *const MmapInfo)};
            vga_println!("mmap type (should be 6): {}", bootInfo.mmapInfo.tag_type).unwrap();
            vga_println!("mmap size: {}", bootInfo.mmapInfo.tag_size).unwrap();
            if bootInfo.mmapInfo.entry_size != 24 {
                panic!("Can't handle mmap entries != 24: {}", bootInfo.mmapInfo.entry_size);
            }
            let entries_number = (bootInfo.mmapInfo.tag_size as usize - core::mem::size_of::<MmapInfo>()) / 24;
            vga_println!("mmap entries number {}", entries_number).unwrap();
            let mut entry_address = tagHeaderAddress + core::mem::size_of::<MmapInfo>() as u32;
            for index in 0..entries_number {
                if index != 0 {
                    entry_address += core::mem::size_of::<MmapEntry>() as u32;
                }
                let entry = unsafe {& (*(entry_address as *const MmapEntry))};
                vga_println!("<addr : {:#x} length: {} type: {} reserved {}>", entry.base_addr, entry.length, entry.entryType, entry.reserved).unwrap();
            }

        }
        else if tagHeader.tag_type == 9 { // elf
            bootInfo.elfSections = unsafe {*(tagHeaderAddress as *const ElfSections)};
            vga_println!("elf sections type (should be 9): {}", bootInfo.elfSections.tag_type).unwrap();
            vga_println!("elf size: {}", bootInfo.elfSections.tag_size).unwrap();
            vga_println!("elf num: {}", bootInfo.elfSections.num).unwrap();
            vga_println!("elf entsize: {}", bootInfo.elfSections.entsize).unwrap();
            if bootInfo.elfSections.entsize != 40 {
                panic!("Can't handle elf sections != 40: {}", bootInfo.elfSections.entsize);
            }
            vga_println!("elf shndx: {} {:#x}", bootInfo.elfSections.shndx, bootInfo.elfSections.shndx).unwrap();
            let mut section_address = tagHeaderAddress + core::mem::size_of::<ElfSections>() as u32;
            for index in 0..bootInfo.elfSections.num {
                if index != 0 {
                    section_address += bootInfo.elfSections.entsize as u32;
                }
                let section = unsafe {& (*(section_address as *const ElfSectionHeader))};
                if index < 7 {
                vga_println!("section {}  sh_name: {} sh_type {} sh_flags {:#b}", index, section.sh_name, section.sh_type, section.sh_flags).unwrap();
                vga_println!("  sh_addr: {:#x} sh_offset: {} sh_size {} sh_link {}", section.sh_addr, section.sh_offset, section.sh_size, section.sh_link).unwrap();
                vga_println!("  sh_info: {} sh_addralign {} sh_entsize {}", section.sh_info, section.sh_addralign, section.sh_entsize).unwrap();
                }
            }
        }
        if tagHeader.tag_size % 8 != 0 {
            tagHeaderAddress += ((tagHeader.tag_size + 8) / 8) * 8;
        }
        else {
            tagHeaderAddress += tagHeader.tag_size;
        }
        tagHeader = unsafe {& (*(tagHeaderAddress as *const TagHeader))};
	    //vga_println!("tagHeader type {} tagHeader size {}", tagHeader.tag_type, tagHeader.tag_size).unwrap();
    }
}

#[no_mangle]
pub extern "C" fn rust_main(n: u32, bootinfo_addr: u32, magic: u32) {
	// ATTENTION: we have a very small stack and no guard page

	vga::_VGA.set_display(7);
	vga_println!("\n{}", n).unwrap();
    parse_bootinfo(bootinfo_addr, magic);

        /*
    unsafe {
        for index in 238..240 {
            if index == 238 {
	            vga_println!("type: {}", value).unwrap();
            }
            else if index == 239 {
                vga_println!("size: {}", value).unwrap();
            }
            else if index == 2 {
	            vga_println!("base addr high: {:#x}", value).unwrap();
            }
            else if index == 3 {
	            vga_println!("length low: {}", value).unwrap();
            }
            else if index == 4 {
	            vga_println!("length high: {}", value).unwrap();
            }
            else if index == 5 {
	            vga_println!("type: {:b}", value).unwrap();
            }

        }
    }
*/
	init();

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

