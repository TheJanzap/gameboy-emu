#[derive(Default, Copy, Clone)]
pub(crate) struct InterruptFlags {
    vblank: bool,
    lcd: bool,
    timer: bool,
    serial: bool,
    joypad: bool,
}

const VBLANK_BYTE_POSITION: u8 = 0;
const LCD_BYTE_POSITION: u8 = 1;
const TIMER_BYTE_POSITION: u8 = 2;
const SERIAL_BYTE_POSITION: u8 = 3;
const JOYPAD_BYTE_POSITION: u8 = 4;

impl From<u8> for InterruptFlags {
    fn from(byte: u8) -> InterruptFlags {
        let vblank = ((byte >> VBLANK_BYTE_POSITION) & 0b1) != 0;
        let lcd = ((byte >> LCD_BYTE_POSITION) & 0b1) != 0;
        let timer = ((byte >> TIMER_BYTE_POSITION) & 0b1) != 0;
        let serial = ((byte >> SERIAL_BYTE_POSITION) & 0b1) != 0;
        let joypad = ((byte >> JOYPAD_BYTE_POSITION) & 0b1) != 0;

        InterruptFlags {
            vblank,
            lcd,
            timer,
            serial,
            joypad,
        }
    }
}

impl From<InterruptFlags> for u8 {
    fn from(flags: InterruptFlags) -> u8 {
        (if flags.vblank { 1 } else { 0 }) << VBLANK_BYTE_POSITION
            | (if flags.lcd { 1 } else { 0 }) << LCD_BYTE_POSITION
            | (if flags.timer { 1 } else { 0 }) << TIMER_BYTE_POSITION
            | (if flags.serial { 1 } else { 0 }) << SERIAL_BYTE_POSITION
            | (if flags.joypad { 1 } else { 0 }) << JOYPAD_BYTE_POSITION
    }
}
