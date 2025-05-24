use winit::event::{KeyEvent, ElementState};
use winit::keyboard::{Key, NamedKey, SmolStr};
use nalgebra_glm as glm;



pub enum KeyKind {
    Space,
    ArrowUp,
    ArrowDown,
    K,
    J,
    Q,
    A,
    Enter,
    ArrowLeft,
    ArrowRight,
    W,
    S,
    D,
}




const KEY_SPACE: u32        = 1 << 0;
const KEY_ARROW_UP: u32     = 1 << 1;
const KEY_ARROW_DOWN: u32   = 1 << 2;
const KEY_K: u32            = 1 << 3;
const KEY_J: u32            = 1 << 4;
const KEY_Q: u32            = 1 << 5;
const KEY_A: u32            = 1 << 6;
const KEY_ENTER: u32        = 1 << 7;
const KEY_W: u32            = 1 << 8;
const KEY_S: u32            = 1 << 9;
const KEY_D: u32            = 1 << 10;
const KEY_ARROW_LEFT: u32   = 1 << 11;
const KEY_ARROW_RIGHT: u32  = 1 << 12;



fn mask_from_winit_key(key: winit::keyboard::Key) -> u32 {
    match key {
        Key::Named(NamedKey::Space) => { KEY_SPACE },
        Key::Named(NamedKey::ArrowUp) => { KEY_ARROW_UP },
        Key::Named(NamedKey::ArrowDown) => { KEY_ARROW_DOWN },
        Key::Named(NamedKey::ArrowLeft) => { KEY_ARROW_LEFT },
        Key::Named(NamedKey::ArrowRight) => { KEY_ARROW_RIGHT },
        Key::Named(NamedKey::Enter) => { KEY_ENTER },
        Key::Character(character) => {
            if character == "k" || character == "K" {
                KEY_K
            } else if character == "j" {
                KEY_J
            } else if character == "q" {
                KEY_Q
            } else if character == "a" {
                KEY_A
            } else if character == "w" {
                KEY_W
            } else if character == "s" {
                KEY_S
            } else if character == "d" {
                KEY_D
            } else {
                0
            }
        },
        _ => { 0 }
    }
}

fn mask_from_key_kind(key: &KeyKind) -> u32 {
    match key {
        KeyKind::Space => { KEY_SPACE },
        KeyKind::ArrowUp => { KEY_ARROW_UP },
        KeyKind::ArrowDown => { KEY_ARROW_DOWN },
        KeyKind::K => { KEY_K },
        KeyKind::J => { KEY_J },
        KeyKind::Q => { KEY_Q },
        KeyKind::A => { KEY_A },
        KeyKind::Enter => { KEY_ENTER },
        KeyKind::ArrowLeft => { KEY_ARROW_LEFT },
        KeyKind::ArrowRight => { KEY_ARROW_RIGHT },
        KeyKind::W => { KEY_W },
        KeyKind::S => { KEY_S },
        KeyKind::D => { KEY_D },
    }
}




pub struct KeyMap {
    pub move_up: Vec<KeyKind>,
    pub move_down: Vec<KeyKind>,
}




pub struct InputController {
    key_state: u32,
    cursor_pos: glm::Vec2,
}




pub struct InputState {
    key_state: u32,
    cursor_pos: glm::Vec2,
}




impl InputController {
    pub fn new() -> Self {
        InputController {
            key_state: 0,
            cursor_pos: glm::Vec2::new(0.0, 0.0),
        }
    }

    pub fn state(&self) -> InputState {
        InputState::new(self.key_state, self.cursor_pos.clone())
    }

    pub fn handle_cursor(&mut self, x: f32, y: f32) {
        self.cursor_pos.x = x;
        self.cursor_pos.y = y;
    }

    pub fn handle_keyboard(&mut self, event: KeyEvent) {
        let mask = mask_from_winit_key(event.logical_key);
        if mask != 0 {
            self.update_state(mask, event.state);
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




impl InputState {
    fn new(key_state: u32, cursor_pos: glm::Vec2) -> Self {
        InputState {
            key_state,
            cursor_pos,
        }
    }

    pub fn is_key_pressed(&self, key: &KeyKind) -> bool {
        let mask = mask_from_key_kind(key);
        self.key_state & mask > 0
    }

    pub fn any_pressed(&self, keys: &[KeyKind]) -> bool {
        for key in keys {
            if self.is_key_pressed(key) {
                return true;
            }
        }

        false
    }
}

