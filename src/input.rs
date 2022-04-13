use std::collections::HashMap;

use sfml::window::{mouse::Button, Event, Key};

#[derive(PartialEq, Eq, Hash, Clone)]
enum HashableButton {
    Left,
    Right,
    Middle,
    XButton1,
    XButton2,
}

impl From<Button> for HashableButton {
    fn from(b: Button) -> Self {
        match b {
            Button::LEFT => HashableButton::Left,
            Button::RIGHT => HashableButton::Right,
            Button::MIDDLE => HashableButton::Middle,
            Button::X_BUTTON_1 => HashableButton::XButton1,
            Button::X_BUTTON_2 => HashableButton::XButton2,
            _ => HashableButton::Left,
        }
    }
}

#[derive(Clone)]
pub struct InputState {
    key_state: HashMap<Key, bool>,
    key_state_delta: HashMap<Key, bool>,
    mouse_state: HashMap<HashableButton, bool>,
    mouse_state_delta: HashMap<HashableButton, bool>,
    mouse_position: (f32, f32),
    mouse_wheel_state: f32,
    mouse_wheel_state_delta: f32,
}

impl InputState {
    pub fn new() -> Self {
        InputState {
            key_state: HashMap::new(),
            key_state_delta: HashMap::new(),
            mouse_state: HashMap::new(),
            mouse_state_delta: HashMap::new(),
            mouse_position: (0., 0.),
            mouse_wheel_state: 0.,
            mouse_wheel_state_delta: 0.,
        }
    }

    pub fn clear(&mut self) {
        self.key_state_delta.clear();
        self.mouse_state_delta.clear();
        self.mouse_wheel_state_delta = 0.;
    }

    pub fn handle_event(&mut self, event: Event) {
        match event {
            Event::KeyPressed { code, .. } => {
                self.key_state.insert(code, true);
                self.key_state_delta.insert(code, true);
            }
            Event::KeyReleased { code, .. } => {
                self.key_state.insert(code, false);
                self.key_state_delta.insert(code, false);
            }
            Event::MouseButtonPressed { button, .. } => {
                self.mouse_state.insert(button.into(), true);
                self.mouse_state_delta.insert(button.into(), true);
            }
            Event::MouseButtonReleased { button, .. } => {
                self.mouse_state.insert(button.into(), false);
                self.mouse_state_delta.insert(button.into(), false);
            }
            Event::MouseWheelScrolled {
                wheel: _, delta, ..
            } => {
                self.mouse_wheel_state += delta;
                self.mouse_wheel_state_delta = delta;
            }
            Event::MouseMoved { x, y } => {
                self.mouse_position = (x as f32, y as f32)
            }
            _ => {}
        };
    }

    pub fn key_pressed(&self, key: Key) -> bool {
        match self.key_state.get(&key) {
            Some(true) => true,
            _ => false,
        }
    }

    pub fn key_just_pressed(&self, key: Key) -> bool {
        match self.key_state_delta.get(&key) {
            Some(true) => true,
            _ => false,
        }
    }

    pub fn mouse_pressed(&self, key: Button) -> bool {
        match self.mouse_state.get(&key.into()) {
            Some(true) => true,
            _ => false,
        }
    }

    pub fn mouse_just_pressed(&self, key: Button) -> bool {
        match self.mouse_state_delta.get(&key.into()) {
            Some(true) => true,
            _ => false,
        }
    }

    pub fn mouse_position(&self) -> (f32, f32) {
        self.mouse_position
    }

    pub fn mouse_wheel(&self) -> f32 {
        self.mouse_wheel_state
    }

    pub fn mouse_wheel_delta(&self) -> f32 {
        self.mouse_wheel_state_delta
    }
}
