use std::time::{Duration, Instant};

pub struct UIInteraction {
    pub current_chat_bubble: Option<(String, Instant, Duration)>,
}

impl UIInteraction {
    pub fn new() -> Self {
        UIInteraction {
            current_chat_bubble: None,
        }
    }

    pub fn display_chat_bubble(&mut self, text: &str, duration: Duration) {
        self.current_chat_bubble = Some((text.to_string(), Instant::now(), duration));
    }

    pub fn hide_chat_bubble(&mut self) {
        self.current_chat_bubble = None;
    }

    pub fn update(&mut self) {
        if let Some((_, start_time, duration)) = self.current_chat_bubble {
            if start_time.elapsed() >= duration {
                self.hide_chat_bubble();
            }
        }
    }

    pub fn get_character_state(&self) -> CharacterState {
        CharacterState {
            is_speaking: self.current_chat_bubble.is_some(),
        }
    }
}

pub struct CharacterState {
    pub is_speaking: bool,
}
