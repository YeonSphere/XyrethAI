use std::time::{Duration, Instant};

pub struct CharacterAnimation {
    current_animation: Option<(String, Instant, Duration)>,
}

impl CharacterAnimation {
    pub fn new() -> Self {
        CharacterAnimation {
            current_animation: None,
        }
    }

    pub fn start_animation(&mut self, animation_name: &str, duration: Duration) {
        self.current_animation = Some((animation_name.to_string(), Instant::now(), duration));
    }

    pub fn stop_animation(&mut self) {
        self.current_animation = None;
    }

    pub fn update(&mut self) {
        if let Some((_, start_time, duration)) = self.current_animation {
            if start_time.elapsed() >= duration {
                self.stop_animation();
            }
        }
    }

    pub fn get_current_animation(&self) -> Option<&str> {
        self.current_animation.as_ref().map(|(name, _, _)| name.as_str())
    }
}
