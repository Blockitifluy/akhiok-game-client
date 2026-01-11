//! Handles the use of Inputs

// TODO: Mouse support later
use std::collections::HashMap;

use beryllium::events::SDL_Keycode as Keycode;

/// The status of a key on a keyboard
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PressedStatus {
    /// The key has just been pressed
    Down,
    /// The key has just been released
    Released,
    /// The key is pressed (Start of holding a key)
    Pressed,
    /// Isn't in any of the states
    None,
}

#[derive(Debug)]
struct KeyStatus {
    pressed_status: PressedStatus,
}

/// Handles key inputs
#[derive(Debug)]
pub struct InputService {
    global_key_status: HashMap<Keycode, KeyStatus>,
    has_changed: bool,
}

impl InputService {
    /// Removes all Keys marked as `Released`, convert Keys marked as `Pressed` to `Down`.
    pub fn mark_cleanup(&mut self) {
        if !self.has_changed {
            return;
        }

        self.has_changed = false;
        self.global_key_status.retain(|_, status| {
            if status.pressed_status == PressedStatus::Released {
                false
            } else {
                if status.pressed_status == PressedStatus::Pressed {
                    status.pressed_status = PressedStatus::Down;
                }
                true
            }
        })
    }

    /// Adds or mutates a new entry inside of InputService.
    /// # Arguements
    /// - `keycode`: the keycode
    /// - `pressed`: if the button has been pressed
    pub fn provide_input(&mut self, keycode: Keycode, pressed: bool) {
        if let Some(key_status) = self.global_key_status.get_mut(&keycode) {
            if pressed {
                eprintln!("pressed status is set to down, but the entry exists");
            }

            key_status.pressed_status = PressedStatus::Released;
            self.has_changed = true;
            return;
        }

        let key_status = KeyStatus {
            pressed_status: PressedStatus::Pressed,
        };
        self.global_key_status.insert(keycode, key_status);
        self.has_changed = true;
    }

    /// Has the `keycode` been pressed?
    /// # Arguements
    /// - `keycode`: the keycode being checked
    /// # Returns
    /// Has the keycode just been pressed
    pub fn is_key_pressed(&self, keycode: Keycode) -> bool {
        let Some(status) = self.global_key_status.get(&keycode) else {
            return false;
        };

        status.pressed_status == PressedStatus::Pressed
    }

    /// Is the `keycode` released?
    /// # Arguements
    /// - `keycode`: the keycode being checked
    /// # Returns
    /// Has the keycode been released
    pub fn is_key_released(&self, keycode: Keycode) -> bool {
        let Some(status) = self.global_key_status.get(&keycode) else {
            return false;
        };

        status.pressed_status == PressedStatus::Released
    }

    /// Is the `keycode` down?
    /// # Arguements
    /// - `keycode`: the keycode being checked
    /// # Returns
    /// Is the keycode down
    pub fn is_key_down(&self, keycode: Keycode) -> bool {
        let Some(status) = self.global_key_status.get(&keycode) else {
            return false;
        };

        status.pressed_status == PressedStatus::Down
    }

    /// Is the `keycode`, either: `Down`, `Released`, `Pressed`?
    /// # Arguements
    /// - `keycode`: the keycode being checked
    /// # Returns
    /// Is the keycode active
    pub fn is_key_active(&self, keycode: Keycode) -> bool {
        self.global_key_status.contains_key(&keycode)
    }
    /// Gets the status of the `keycode`.
    /// # Arguements
    /// - `keycode`: the keycode being checked
    /// # Returns
    /// The status of the key, returns `PressedStatus::None`, if inactive
    pub fn get_key_status(&self, keycode: Keycode) -> PressedStatus {
        if let Some(status) = self.global_key_status.get(&keycode) {
            status.pressed_status
        } else {
            PressedStatus::None
        }
    }

    /// Gets the keycodes, that are pressed.
    /// # Returns
    /// A vector of keycodes that are pressed.
    pub fn get_keys_pressed(&self) -> Vec<Keycode> {
        self.global_key_status
            .iter()
            .filter(|(_, s)| s.pressed_status == PressedStatus::Pressed)
            .map(|(k, _)| *k)
            .collect()
    }

    /// Gets the keycodes, that have been released.
    /// # Returns
    /// A vector of keycodes that have been released
    pub fn get_keys_released(&self) -> Vec<Keycode> {
        self.global_key_status
            .iter()
            .filter(|(_, s)| s.pressed_status == PressedStatus::Released)
            .map(|(k, _)| *k)
            .collect()
    }

    /// Gets the keycode, that are down.
    /// # Returns
    /// A vector of keycodes that are down
    pub fn get_keys_down(&self) -> Vec<Keycode> {
        self.global_key_status
            .iter()
            .filter(|(_, s)| s.pressed_status == PressedStatus::Down)
            .map(|(k, _)| *k)
            .collect()
    }

    /// Gets the keycodes, that are either: `Down`, `Released` or `Pressed`.
    /// # Returns
    /// A vector of active keycodes
    pub fn get_keys_active(&self) -> Vec<Keycode> {
        self.global_key_status.keys().copied().collect()
    }
}

impl Default for InputService {
    fn default() -> Self {
        Self {
            global_key_status: HashMap::with_capacity(64),
            has_changed: false,
        }
    }
}
