use seokjin_ai::{SeokjinAI, AIConfig, InteractionModule, LearningModule, PlatformModule};
use std::collections::HashMap;
use crate::core::ai_engine::SeokjinAI;
use crate::core::error::SeokjinError;
use crate::core::benchmarking::Benchmark;
use crate::core::data_preprocessing::preprocess_data;
use crate::core::hyperparameter_tuning::{HyperParameters, random_search};
use crate::core::character_control::CharacterControl;
use crate::core::ui_interactions::UIInteraction;
use std::time::Duration;

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_ai() -> SeokjinAI {
        let config = AIConfig {
            model_name: "TestModel".to_string(),
            model_version: "1.0".to_string(),
            wake_word: "Hey Seokjin".to_string(),
        };
        SeokjinAI::new(config)
    }

    #[test]
    fn test_ai_initialization() {
        let ai = create_test_ai();
        assert_eq!(ai.config.model_name, "TestModel");
        assert_eq!(ai.config.model_version, "1.0");
        assert_eq!(ai.config.wake_word, "Hey Seokjin");
    }

    #[test]
    fn test_process_input_with_wake_word() {
        let mut ai = create_test_ai();
        let result = ai.process_input("Hey Seokjin, what time is it?");
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.contains("current time"));
    }

    #[test]
    fn test_process_input_without_wake_word() {
        let mut ai = create_test_ai();
        let result = ai.process_input("What's the weather like?");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), SeokjinError::InvalidInput(_)));
    }

    #[test]
    fn test_file_indexing() {
        let mut ai = create_test_ai();
        let result = ai.index_files("./test_data");
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.contains("Indexed"));
    }

    #[test]
    fn test_open_app() {
        let ai = create_test_ai();
        let result = ai.open_app("TestApp");
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.contains("open TestApp"));
    }

    #[test]
    fn test_get_battery_level() {
        let ai = create_test_ai();
        let result = ai.get_battery_level();
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.contains("Battery level"));
    }

    #[test]
    fn test_learning_module() {
        let mut learning_module = LearningModule::new();
        learning_module.learn_from_interaction("Hello", "Hi there!");
        let response = learning_module.get_learned_response("Hello");
        assert_eq!(response, Some(&"Hi there!".to_string()));
    }

    #[test]
    fn test_interaction_module() {
        let mut interaction_module = InteractionModule::new();
        let result = interaction_module.generate_response("Tell me a joke");
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.contains("joke") || response.contains("Joke"));
    }

    #[test]
    fn test_platform_module() {
        let platform_module = PlatformModule::new();
        let result = platform_module.open_app("TestApp");
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.contains("open TestApp") || response.contains("not supported"));
    }

    #[test]
    fn test_file_indexing_non_existent() {
        let mut ai = create_test_ai();
        let result = ai.index_files("non_existent_directory");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), SeokjinError::FileNotFound(_)));
    }

    #[test]
    fn test_benchmarking() {
        let _benchmark = Benchmark::new("Test Benchmark");
        // Simulate some work
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    #[test]
    fn test_data_preprocessing() {
        let processed_data = preprocess_data("test_data.txt").unwrap();
        assert!(!processed_data.is_empty());
    }

    #[test]
    fn test_model_serialization() {
        let ai = create_test_ai();
        ai.save_model("test_model.json").unwrap();
        let loaded_ai = SeokjinAI::load_model("test_model.json").unwrap();
        assert_eq!(ai.config.model_name, loaded_ai.config.model_name);
    }

    #[test]
    fn test_hyperparameter_tuning() {
        let hyperparameters = random_search(10);
        assert_eq!(hyperparameters.len(), 10);
    }

    #[test]
    fn test_character_control() {
        let mut character = CharacterControl::new();
        character.move_to(100.0, 100.0, Duration::from_secs(1));
        let state = character.get_current_state();
        assert_eq!(state.position.x, 100.0);
        assert_eq!(state.position.y, 100.0);
    }

    #[test]
    fn test_ui_interaction() {
        let mut ui = UIInteraction::new();
        ui.display_chat_bubble("Hello, I'm Seokjin!", Duration::from_secs(3));
        assert!(ui.current_chat_bubble.is_some());
        assert!(ui.get_character_state().is_speaking);
    }

    #[test]
    fn test_sensitive_information_handling() {
        let mut ai = create_test_ai();
        let result = ai.process_input("My password is 12345");
        assert!(matches!(result, Err(SeokjinError::SensitiveInformation(_))));
    }

    #[test]
    fn test_continuous_learning() {
        let mut ai = create_test_ai();
        let input = "What's the capital of France?";
        let first_response = ai.process_input(input).unwrap();
        let second_response = ai.process_input(input).unwrap();
        assert_eq!(first_response, second_response);
    }

    #[test]
    fn test_learn_from_web() {
        let mut ai = create_test_ai();
        let result = ai.learn_from_web("https://example.com");
        assert!(result.is_ok());
        // You might want to add more specific assertions here, depending on what you expect
        // the AI to learn from the example.com website
    }
}
