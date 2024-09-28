use crate::core::learning::LearningModule;
use crate::core::error::SeokjinError;
use reqwest;
use whatlang;
use chrono::{DateTime, Utc, Local};
use serde_json::Value;

pub struct InteractionModule {
    learning_module: LearningModule,
    web_client: reqwest::Client,
}

impl InteractionModule {
    pub fn new() -> Self {
        InteractionModule {
            learning_module: LearningModule::new(),
            web_client: reqwest::Client::new(),
        }
    }

    pub fn generate_response(&mut self, input: &str) -> Result<String, SeokjinError> {
        let language = self.detect_language(input);
        let corrected_input = self.correct_language(input, &language);
        
        let response = if let Some(learned_response) = self.learning_module.get_learned_response(&corrected_input) {
            learned_response.clone()
        } else {
            self.process_input(&corrected_input)?
        };

        self.learning_module.learn_from_interaction(&corrected_input, &response);
        Ok(response)
    }

    fn process_input(&self, input: &str) -> Result<String, SeokjinError> {
        match input.to_lowercase().as_str() {
            "what time is it?" => self.get_current_time(),
            "what's the weather like?" => self.get_weather(),
            "tell me a joke" => self.tell_joke(),
            "set a reminder" => self.set_reminder(),
            "play music" => self.play_music(),
            _ => self.general_response(input),
        }
    }

    fn get_current_time(&self) -> Result<String, SeokjinError> {
        let now: DateTime<Local> = Local::now();
        Ok(format!("The current time is {}", now.format("%I:%M %p")))
    }

    fn get_weather(&self) -> Result<String, SeokjinError> {
        // This is a placeholder. In a real implementation, you'd call a weather API.
        Ok("I'm sorry, I don't have access to real-time weather data at the moment.".to_string())
    }

    fn tell_joke(&self) -> Result<String, SeokjinError> {
        // This is a placeholder. In a real implementation, you might have a database of jokes.
        Ok("Why don't scientists trust atoms? Because they make up everything!".to_string())
    }

    fn set_reminder(&self) -> Result<String, SeokjinError> {
        // This is a placeholder. In a real implementation, you'd integrate with the device's reminder system.
        Ok("I'm sorry, I can't set reminders at the moment. This feature will be available in a future update.".to_string())
    }

    fn play_music(&self) -> Result<String, SeokjinError> {
        // This is a placeholder. In a real implementation, you'd integrate with a music player.
        Ok("I'm sorry, I can't play music at the moment. This feature will be available in a future update.".to_string())
    }

    fn general_response(&self, input: &str) -> Result<String, SeokjinError> {
        Ok(format!("I'm not sure how to respond to '{}'. Can you please rephrase or ask me something else?", input))
    }

    fn detect_language(&self, text: &str) -> whatlang::Lang {
        whatlang::detect(text).map(|info| info.lang()).unwrap_or(whatlang::Lang::Eng)
    }

    fn correct_language(&self, text: &str, detected_lang: &whatlang::Lang) -> String {
        if *detected_lang != whatlang::Lang::Eng {
            format!("I noticed you might not be a native English speaker. Here's a correction: {}", text)
        } else {
            text.to_string()
        }
    }
}
