use winit::event::{KeyEvent, ElementState};
use winit::keyboard::{Key, NamedKey, SmolStr};


pub enum PongKey {
    Space,
    ArrowUp,
    ArrowDown,
    K,
    J,
    Q,
    A,
    Enter,
}

const KEY_SPACE: u32        = 1 << 0;
const KEY_ARROW_UP: u32     = 1 << 1;
const KEY_ARROW_DOWN: u32   = 1 << 2;
const KEY_K: u32            = 1 << 3;
const KEY_J: u32            = 1 << 4;
const KEY_Q: u32            = 1 << 5;
const KEY_A: u32            = 1 << 6;
const KEY_ENTER: u32        = 1 << 7;

const MASK: u32             = 0xFFFF;

pub struct KeyMap {
    pub move_up: Vec<PongKey>,
    pub move_down: Vec<PongKey>,
}

pub struct InputController {
    key_state: u32,
}

impl InputController {
    pub fn new() -> Self {
        InputController {
            key_state: 0,
        }
    }

    pub fn handle_keyboard(&mut self, event: KeyEvent) {
        let mask = Self::mask_from_winit_key(event.logical_key);
        if mask != 0 {
            self.update_state(mask, event.state);
        }
    }

    pub fn is_key_pressed(&self, key: &PongKey) -> bool {
        let mask = Self::mask_from_pong_key(key);
        self.key_state & mask > 0
    }

    pub fn any_pressed(&self, keys: &[PongKey]) -> bool {
        for key in keys {
            if self.is_key_pressed(key) {
                return true;
            }
        }

        false
    }

    fn mask_from_winit_key(key: winit::keyboard::Key) -> u32 {
        match key {
            Key::Named(NamedKey::Space) => { KEY_SPACE },
            Key::Named(NamedKey::ArrowUp) => { KEY_ARROW_UP },
            Key::Named(NamedKey::ArrowDown) => { KEY_ARROW_DOWN },
            Key::Named(NamedKey::Enter) => { KEY_ENTER },
            Key::Character(character) => {
                if character == "k" || character == "K" {
                    KEY_K
                } else if character == "j" || character == "J" {
                    KEY_J
                } else if character == "q" || character == "Q" {
                    KEY_Q
                } else if character == "a" || character == "A" {
                    KEY_A
                } else {
                    0
                }
            },
            _ => { 0 }
        }
    }
    
    fn mask_from_pong_key(key: &PongKey) -> u32 {
        match key {
            PongKey::Space => { KEY_SPACE },
            PongKey::ArrowUp => { KEY_ARROW_UP },
            PongKey::ArrowDown => { KEY_ARROW_DOWN },
            PongKey::K => { KEY_K },
            PongKey::J => { KEY_J },
            PongKey::Q => { KEY_Q },
            PongKey::A => { KEY_A },
            PongKey::Enter => { KEY_ENTER },
        }
    }
    /*
     *
     * 0b000000 => No key pressed,
     * 0b000100 => KEY_ARROW_DOWN is pressed
     *      In order to get from 0b0000000 => 0b0000100, I can just OR self.key_state with
     *      KEY_ARROW_DOWN.
     *
     * 0b0100000 => q/Q key pressed,
     * 0b0100100 => q/Q still pressed, KEY_ARROW_DOWN also pressed
     *      In order to go from 0b0100000 => 0b0100100 I can just OR self.key_state with KEY_Q
     *      0b0100000 | 0b0000100 = 0b0100100
     *
     * 0b0000100 => q/Q release, KEY_ARROW_DOWN still pressed,
     *      In order to go from 0b0100100 => 0b0000100 I can just AND self.key_state with inverse of
     *      KEY_Q
     *      ~0b0100000 = 0b1011111;
     *      0b0100100 & 0b1011111 = 0b00000100;
     * */
    fn update_state(&mut self, key_mask: u32, state: ElementState) {
        // println!("KEY_STATE BEFORE: {:#8b}", self.key_state);
        match state {
            ElementState::Pressed => { self.key_state |= key_mask; },
            ElementState::Released => { self.key_state &= !key_mask; },
        }
        // println!("KEY_STATE AFTER: {:#8b}", self.key_state);
    }
}


