use std::time::Duration;

#[derive(Clone, Copy)]
pub struct CharacterPosition {
    pub x: f32,
    pub y: f32,
}

pub struct CharacterControl {
    position: CharacterPosition,
}

impl CharacterControl {
    pub fn new() -> Self {
        CharacterControl {
            position: CharacterPosition { x: 0.0, y: 0.0 },
        }
    }

    pub fn move_to(&mut self, x: f32, y: f32, duration: Duration) {
        // TODO: Implement smooth movement logic here
        self.position.x = x;
        self.position.y = y;
    }

    pub fn get_current_position(&self) -> CharacterPosition {
        self.position
    }
}
