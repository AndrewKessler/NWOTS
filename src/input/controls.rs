use std::collections::HashSet;

use winit::{
    event::ElementState,
    keyboard::KeyCode,
};

pub struct InputState {
    pub keys: HashSet<KeyCode>,
}

impl InputState {

    pub fn new() -> Self {

        Self {
            keys: HashSet::new(),
        }
    }

    pub fn set_key(
        &mut self,
        key: KeyCode,
        state: ElementState,
    ) {

        match state {

            ElementState::Pressed => {

                self.keys.insert(key);
            }

            ElementState::Released => {

                self.keys.remove(&key);
            }
        }
    }

    pub fn is_down(
        &self,
        key: KeyCode,
    ) -> bool {

        self.keys.contains(&key)
    }
}