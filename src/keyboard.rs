use crate::chip_8::{Key, KeyboardInterface};
use sdl2::{keyboard::Scancode, EventPump};
use std::{cell::RefCell, rc::Rc};

pub struct Keyboard {
    event_pump: Rc<RefCell<EventPump>>,
}

impl Keyboard {
    pub fn new(event_pump: Rc<RefCell<EventPump>>) -> Self {
        Keyboard { event_pump }
    }

    fn key_to_scancode(key: Key) -> Scancode {
        match key.value() {
            0x0 => Scancode::X,
            0x1 => Scancode::Num1,
            0x2 => Scancode::Num2,
            0x3 => Scancode::Num3,
            0x4 => Scancode::Q,
            0x5 => Scancode::W,
            0x6 => Scancode::E,
            0x7 => Scancode::A,
            0x8 => Scancode::S,
            0x9 => Scancode::D,
            0xA => Scancode::Z,
            0xB => Scancode::C,
            0xC => Scancode::Num4,
            0xD => Scancode::R,
            0xE => Scancode::F,
            0xF => Scancode::V,
            _ => panic!(), // would have panicked in Key::new()
        }
    }

    fn scancode_to_key(scancode: Scancode) -> Option<Key> {
        match scancode {
            Scancode::Num1 => Some(Key::new(0x1)),
            Scancode::Num2 => Some(Key::new(0x2)),
            Scancode::Num3 => Some(Key::new(0x3)),
            Scancode::Num4 => Some(Key::new(0xC)),
            Scancode::Q => Some(Key::new(0x4)),
            Scancode::W => Some(Key::new(0x5)),
            Scancode::E => Some(Key::new(0x6)),
            Scancode::R => Some(Key::new(0xD)),
            Scancode::A => Some(Key::new(0x7)),
            Scancode::S => Some(Key::new(0x8)),
            Scancode::D => Some(Key::new(0x9)),
            Scancode::F => Some(Key::new(0xE)),
            Scancode::Z => Some(Key::new(0xA)),
            Scancode::X => Some(Key::new(0x0)),
            Scancode::C => Some(Key::new(0xB)),
            Scancode::V => Some(Key::new(0xF)),
            _ => None,
        }
    }
}

impl KeyboardInterface for Keyboard {
    fn is_key_pressed(&self, key: Key) -> bool {
        let event_pump = self.event_pump.borrow();
        let kbd_state = event_pump.keyboard_state();
        let scancode = Keyboard::key_to_scancode(key);
        kbd_state.is_scancode_pressed(scancode)
    }

    fn get_pressed_key(&self) -> Option<Key> {
        let event_pump = self.event_pump.borrow();
        let kbd_state = event_pump.keyboard_state();

        for scancode in kbd_state.pressed_scancodes() {
            if let Some(key) = Keyboard::scancode_to_key(scancode) {
                return Some(key);
            }
        }

        None
    }
}
