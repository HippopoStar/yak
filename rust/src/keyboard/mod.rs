
use crate::{vga_print, vga_input};
use crate::arch::x86::instructions::port::Port;
use core::sync::atomic::{AtomicBool, Ordering};
use core::fmt;

pub static _KB: Keyboard = Keyboard::new();

#[derive(Default)]
pub struct Keyboard {
    shift: core::sync::atomic::AtomicBool,
    extended: core::sync::atomic::AtomicBool,
    ctrl: core::sync::atomic::AtomicBool,
}

pub struct Key {
    character :u8,
    character_uppercase: u8,
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.character as char)
    }
}

impl Keyboard {
    const SCANCODES: &'static [Key] = &[
        Key {character: b'\0', character_uppercase: b'\0'},
        Key {character: b'!', character_uppercase: b'!'}, // 1 esc
        Key {character: b'1', character_uppercase: b'!'},
        Key {character: b'2', character_uppercase: b'@'},
        Key {character: b'3', character_uppercase: b'#'},
        Key {character: b'4', character_uppercase: b'$'},
        Key {character: b'5', character_uppercase: b'%'},
        Key {character: b'6', character_uppercase: b'^'},
        Key {character: b'7', character_uppercase: b'&'},
        Key {character: b'8', character_uppercase: b'*'},
        Key {character: b'9', character_uppercase: b'('},
        Key {character: b'0', character_uppercase: b')'}, // 11
        Key {character: b'-', character_uppercase: b'_'},
        Key {character: b'=', character_uppercase: b'+'},
        Key {character: b'!', character_uppercase: b'!'}, // 14 backspace
        Key {character: b'\t', character_uppercase: b'\t'}, // 15
        Key {character: b'q', character_uppercase: b'Q'},
        Key {character: b'w', character_uppercase: b'W'},
        Key {character: b'e', character_uppercase: b'E'},
        Key {character: b'r', character_uppercase: b'R'},
        Key {character: b't', character_uppercase: b'T'},
        Key {character: b'y', character_uppercase: b'Y'},
        Key {character: b'u', character_uppercase: b'U'},
        Key {character: b'i', character_uppercase: b'I'},
        Key {character: b'o', character_uppercase: b'O'},
        Key {character: b'p', character_uppercase: b'P'},
        Key {character: b'[', character_uppercase: b'{'},
        Key {character: b']', character_uppercase: b'}'},
        Key {character: b'\n', character_uppercase: b'\n'}, // 28 enter
        Key {character: b'!', character_uppercase: b'!'}, // 29 left ctrl
        Key {character: b'a', character_uppercase: b'A'}, // 30
        Key {character: b's', character_uppercase: b'S'},
        Key {character: b'd', character_uppercase: b'D'},
        Key {character: b'f', character_uppercase: b'F'},
        Key {character: b'g', character_uppercase: b'G'},
        Key {character: b'h', character_uppercase: b'H'}, // 35
        Key {character: b'j', character_uppercase: b'J'},
        Key {character: b'k', character_uppercase: b'K'},
        Key {character: b'l', character_uppercase: b'L'},
        Key {character: b';', character_uppercase: b':'},
        Key {character: b'\'', character_uppercase: b'\"'}, // 40
        Key {character: b'`', character_uppercase: b'~'},
        Key {character: b'!', character_uppercase: b'!'}, // 42 lshift
        Key {character: b'\\', character_uppercase: b'|'},
        Key {character: b'z', character_uppercase: b'Z'},
        Key {character: b'x', character_uppercase: b'X'},
        Key {character: b'c', character_uppercase: b'C'}, // 45
        Key {character: b'v', character_uppercase: b'V'},
        Key {character: b'b', character_uppercase: b'B'},
        Key {character: b'n', character_uppercase: b'N'},
        Key {character: b'm', character_uppercase: b'M'}, // 50
        Key {character: b',', character_uppercase: b'<'},
        Key {character: b'.', character_uppercase: b'>'},
        Key {character: b'/', character_uppercase: b'?'},
        Key {character: b'!', character_uppercase: b'!'}, // right shift
        Key {character: b'*', character_uppercase: b'*'}, // 55 * keypad
        Key {character: b'!', character_uppercase: b'!'}, // right shift
        Key {character: b' ', character_uppercase: b' '}, // space
        Key {character: b'!', character_uppercase: b'!'}, // capslock
        Key {character: b'!', character_uppercase: b'!'}, // f1
        Key {character: b'!', character_uppercase: b'!'}, // 60 f2
        Key {character: b'!', character_uppercase: b'!'}, // f3
        Key {character: b'!', character_uppercase: b'!'}, // f4
        Key {character: b'!', character_uppercase: b'!'}, // f5
        Key {character: b'!', character_uppercase: b'!'}, // f6
        Key {character: b'!', character_uppercase: b'!'}, // 65 f7
        Key {character: b'!', character_uppercase: b'!'}, // f8
        Key {character: b'!', character_uppercase: b'!'}, // f9
        Key {character: b'!', character_uppercase: b'!'}, // f10
        Key {character: b'!', character_uppercase: b'!'}, // numlock
        Key {character: b'!', character_uppercase: b'!'}, // 70 scrolllock
        Key {character: b'7', character_uppercase: b'7'}, // keypad 7
        Key {character: b'8', character_uppercase: b'8'}, // keypad 8
        Key {character: b'9', character_uppercase: b'9'}, // keypad 9
        Key {character: b'-', character_uppercase: b'-'}, // keypad -
        Key {character: b'4', character_uppercase: b'4'}, // 75 keypad 4
        Key {character: b'5', character_uppercase: b'5'}, // keypad 5
        Key {character: b'6', character_uppercase: b'6'}, // keypad 6
        Key {character: b'+', character_uppercase: b'+'}, // keypad +
        Key {character: b'1', character_uppercase: b'1'}, // keypad 1
        Key {character: b'2', character_uppercase: b'2'}, // 80 keypad 2
        Key {character: b'3', character_uppercase: b'3'}, // keypad 3
        Key {character: b'0', character_uppercase: b'0'}, // keypad 0
        Key {character: b'.', character_uppercase: b'.'}, // keypad .
        Key {character: b'!', character_uppercase: b'!'}, // F11
    ];

    pub const fn new() -> Self {
        Self {
            shift: AtomicBool::new(false),
            extended: AtomicBool::new(false),
            ctrl: AtomicBool::new(false),
        }
    }

    pub fn print_scancode(&self) -> () {
        let mut port = Port::new(0x60);
        let scancode: u8 = unsafe { port.read() };
        let _real_scancode: u8 = scancode & 0x7f;
        let _is_pressed: bool = (scancode & 0x80) == 0;

        //vga_print!("SCANCODE {:#x} ", scancode).unwrap();
        if self.extended.load(Ordering::Relaxed) == false {
            if scancode == 0xE0 {
                self.extended.store(true, Ordering::Relaxed);
                //vga_print!("EXTENDED-BYTE ").unwrap();
            }
            else if scancode == 42 || scancode == 56 {
                self.shift.store(true, Ordering::Relaxed);
            }
            else if scancode == 29 {
                self.ctrl.store(true, Ordering::Relaxed);
            }
            else if (scancode & 0x80) == 0 {
                if scancode > 84 {
                    vga_print!("UNHANDLED SCANCODE {:#x} !", scancode).unwrap();
                }
                else if scancode == 14 {
                    vga_input!("\x08").unwrap(); // BackSpace (ASCII 0x08)
                }
                else {
                    if self.ctrl.load(Ordering::Relaxed) == true && self.shift.load(Ordering::Relaxed) == false && scancode == 25 {
                        crate::vga::print_rainbow_42();
                        self.ctrl.store(false, Ordering::Relaxed);
                    }
                    else if 59 <= scancode && scancode <= 66 {
                        let value = scancode - 59;
                        crate::vga::_VGA.set_display(value as usize);
                    }
                    else if self.shift.load(Ordering::Relaxed) == false {
                        vga_input!("{}", Self::SCANCODES[scancode as usize].character as char).unwrap();
                    }
                    else {
                        vga_input!("{}", Self::SCANCODES[scancode as usize].character_uppercase as char).unwrap();
                        self.shift.store(false, Ordering::Relaxed);
                    }
                }
            }
        }
        else {
            if (scancode & 0x80) == 0 {
                if scancode == 0x48 { // arrow up
                    vga_input!("{}", b'\x18' as char).unwrap();
                }
                else if scancode == 0x49 { // pg up
                    vga_input!("{}", b'\x1e' as char).unwrap();
                }
                else if scancode == 0x4b { // arrow left
                    vga_input!("{}", b'\x1b' as char).unwrap();
                }
                else if scancode == 0x4d { // arrow right
                    vga_input!("{}", b'\x1a' as char).unwrap();
                }
                else if scancode == 0x50 { // arrow down
                    vga_input!("{}", b'\x19' as char).unwrap();
                }
                else if scancode == 0x51 { // pg down
                    vga_input!("{}", b'\x1f' as char).unwrap();
                }
                else if scancode == 0x53 { // del
                    vga_input!("{}", b'\x7f' as char).unwrap();
                }
                else {
                    vga_print!("EXTENDED scancode {:#x} pressed", scancode).unwrap();
                }
            }
            else {
                //vga_print!("Extended scancode {:#x} released", scancode).unwrap();
            }
            self.extended.store(false, Ordering::Relaxed);
        }
    }
}

