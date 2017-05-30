use std::collections::HashMap;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

enum KeyState {
    Press,
    Release
}

pub struct KeyHandler {
    incoming: Vec<(KeyState, Keycode)>,
    last_press: Option<(Keycode, u64)>,
    keys: HashMap<Keycode, u64>
}

impl KeyHandler {
    pub fn new() -> KeyHandler {
        KeyHandler {
            incoming: vec![],
            last_press: None,
            keys: HashMap::new()
        }
    }

    pub fn handle_event(&mut self, event: &Event) {
        match event {
            Event::KeyDown {keycode: Some(key), ..} => {
                self.incoming.push((KeyState::Press, key));
            },
            Event::KeyUp {keycode: Some(key), ..} => {
                self.incoming.push((KeyState::Release, key));
            },
            _ => ()
        }
    }

    pub fn think(&mut self, tick: u64) {
        // Digest each of the incoming items;
        for &(ref state, ref key) in self.incoming.iter() {
            match *state {
                KeyState::Press => {
                    self.last_press = Some((*key, tick));
                    self.keys.insert(*key, tick);
                },
                KeyState::Release => {
                    self.keys.remove(key);
                }
            }
        };
        self.incoming.clear();
    }

    pub fn last_key(&mut self) -> Option<(Key, u64)> {
        match self.last_press {
            Some((key, timestamp)) => {
                match self.keys.get(&key) {
                    Some(_) => {
                        Some((key, timestamp))
                    },
                    None => {
                        // Clear ourself out if we're not currently pressed
                        self.last_press = None;
                        Some((key, timestamp))
                    }
                }
            },
            None => None
        }
    }

    pub fn is_pressed(&self, code: Keycode) -> bool {
        self.keys.get(&key).is_some()
    }

    pub fn time_pressed(&self, code: Keycode) -> Option<u64> {
        self.keys.get(&key)
    }
}
