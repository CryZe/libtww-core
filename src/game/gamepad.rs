#[repr(C)]
struct JUTGamePad {
    _unknown: [u8; 4],
    buttons_down: u16,
    buttons_pressed: u16,
}

extern "C" {
    #[link_name = "JUTGamePad::mPadButton"]
    static mut GAMEPAD: JUTGamePad;
}

bitflags::bitflags! {
    pub struct Buttons: u16 {
        const DPAD_LEFT = 0x0001;
        const DPAD_RIGHT = 0x0002;
        const DPAD_DOWN = 0x0004;
        const DPAD_UP = 0x0008;
        const Z = 0x0010;
        const R = 0x0020;
        const L = 0x0040;
        const A = 0x0100;
        const B = 0x0200;
        const X = 0x0400;
        const Y = 0x0800;
        const START = 0x1000;
    }
}

pub const DPAD_LEFT: Buttons = Buttons::DPAD_LEFT;
pub const DPAD_RIGHT: Buttons = Buttons::DPAD_RIGHT;
pub const DPAD_DOWN: Buttons = Buttons::DPAD_DOWN;
pub const DPAD_UP: Buttons = Buttons::DPAD_UP;
pub const Z: Buttons = Buttons::Z;
pub const R: Buttons = Buttons::R;
pub const L: Buttons = Buttons::L;
pub const A: Buttons = Buttons::A;
pub const B: Buttons = Buttons::B;
pub const X: Buttons = Buttons::X;
pub const Y: Buttons = Buttons::Y;
pub const START: Buttons = Buttons::START;

pub fn buttons_down() -> Buttons {
    unsafe { Buttons::from_bits_truncate(GAMEPAD.buttons_down) }
}

pub fn buttons_pressed() -> Buttons {
    unsafe { Buttons::from_bits_truncate(GAMEPAD.buttons_pressed) }
}

pub fn set_buttons_down(buttons: Buttons) {
    unsafe {
        GAMEPAD.buttons_down = buttons.bits();
    }
}

pub fn set_buttons_pressed(buttons: Buttons) {
    unsafe {
        GAMEPAD.buttons_pressed = buttons.bits();
    }
}

pub fn is_down(buttons: Buttons) -> bool {
    buttons_down().contains(buttons)
}

pub fn is_pressed(buttons: Buttons) -> bool {
    buttons_pressed().contains(buttons)
}

pub fn is_any_down(buttons: Buttons) -> bool {
    buttons_down().intersects(buttons)
}

pub fn is_any_pressed(buttons: Buttons) -> bool {
    buttons_pressed().intersects(buttons)
}
