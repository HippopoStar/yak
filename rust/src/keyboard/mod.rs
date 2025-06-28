use core::fmt::Write;
use crate::PICS;
use crate::arch::x86::instructions::port::Port;
use core::fmt;

pub struct Keyboard {
    shift: bool, 
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
    pub fn print_scancode() -> () {

    const SCANCODES: &'static [Key] = &[
        Key {character: b'\0', character_uppercase: b'\0'},
        Key {character: b'`', character_uppercase: b'~'},
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
// 42 shift
    ];

    let mut port = Port::new(0x60);
    let mut scancode: u8 = unsafe { port.read() };

    if scancode == 14 {
        crate::vga::_VGA.get_screen(2).del_byte();    
    }
    else if scancode != 142{
//    if scancode & 0x80 == 0 {
        write!(crate::vga::_VGA.get_screen(2), "scancode {} ", scancode).unwrap();
       // write!(crate::vga::_VGA.get_screen(2), "{} ", SCANCODES[scancode as usize]).unwrap();
  //  }
    }

    unsafe {
        PICS.lock().notify_end_of_interrupt(33);
    }

    }
    pub fn new() -> Self {
        return Self {shift: false}
    }
}


